import type { Page } from "@playwright/test";

export interface TauriIpcCall {
  command: string;
  args: Record<string, unknown>;
}

interface RuntimeResult {
  outcome: "resolve" | "reject";
  values: unknown[];
  revision: number;
}

interface SetRuntimeResult {
  kind: "set";
  command: string;
  result: RuntimeResult;
}

interface ReplaceRuntimeResults {
  kind: "replace";
  results: Array<[string, RuntimeResult]>;
  revision: number;
}

type RuntimeInstruction = SetRuntimeResult | ReplaceRuntimeResults;

interface BrowserRuntimeController {
  apply(instruction: RuntimeInstruction): void;
  calls(): TauriIpcCall[];
  emit(event: string, payload: unknown): void;
  listenerCount(event: string): number;
}

interface FakeTauriWindow extends Window {
  __PACK_MANAGER_TEST_IPC__?: BrowserRuntimeController;
  __PACK_MANAGER_TEST_IPC_PENDING__?: RuntimeInstruction[];
  __TAURI_EVENT_PLUGIN_INTERNALS__?: {
    unregisterListener(event: string, eventId: number): void;
  };
  __TAURI_INTERNALS__?: {
    callbacks: Map<number, (data: unknown) => void>;
    invoke(command: string, args?: Record<string, unknown>): Promise<unknown>;
    runCallback(id: number, data: unknown): void;
    transformCallback(
      callback?: (data: unknown) => void,
      once?: boolean,
    ): number;
    unregisterCallback(id: number): void;
  };
}

function applyRuntimeInstruction(instruction: RuntimeInstruction): void {
  const root = window as FakeTauriWindow;
  if (root.__PACK_MANAGER_TEST_IPC__) {
    root.__PACK_MANAGER_TEST_IPC__.apply(instruction);
  } else {
    (root.__PACK_MANAGER_TEST_IPC_PENDING__ ??= []).push(instruction);
  }
}

/** Installed before application code on every navigation. */
function installFakeTauriRuntime(
  initialResults: Array<[string, RuntimeResult]>,
): void {
  const root = window as FakeTauriWindow;
  if (root.__PACK_MANAGER_TEST_IPC__) return;

  const responses = new Map<string, RuntimeResult>();
  const calls: TauriIpcCall[] = [];
  const callbacks = new Map<number, (data: unknown) => void>();
  const onceCallbacks = new Set<number>();
  const listeners = new Map<string, Map<number, number>>();
  let nextCallbackId = 1;
  let nextEventId = 1;
  let resetRevision = 0;

  const clone = <T>(value: T): T => structuredClone(value);

  const setResult = (command: string, result: RuntimeResult): void => {
    const current = responses.get(command);
    if (
      result.revision < resetRevision ||
      (current && current.revision > result.revision)
    )
      return;
    responses.set(command, clone(result));
  };

  const apply = (instruction: RuntimeInstruction): void => {
    if (instruction.kind === "set") {
      setResult(instruction.command, instruction.result);
      return;
    }

    if (instruction.revision < resetRevision) return;
    resetRevision = instruction.revision;
    for (const [command, result] of responses) {
      if (result.revision < resetRevision) responses.delete(command);
    }
    for (const [command, result] of instruction.results)
      setResult(command, result);
    calls.length = 0;
    callbacks.clear();
    onceCallbacks.clear();
    listeners.clear();
  };

  const unregisterCallback = (id: number): void => {
    callbacks.delete(id);
    onceCallbacks.delete(id);
  };

  const removeListener = (event: string, eventId: number): void => {
    const eventListeners = listeners.get(event);
    const callbackId = eventListeners?.get(eventId);
    if (callbackId !== undefined) unregisterCallback(callbackId);
    eventListeners?.delete(eventId);
    if (eventListeners?.size === 0) listeners.delete(event);
  };

  const controller: BrowserRuntimeController = {
    apply,
    calls: () => clone(calls),
    emit: (event, payload) => {
      for (const [eventId, callbackId] of listeners.get(event) ?? []) {
        const callback = callbacks.get(callbackId);
        callback?.({ event, id: eventId, payload: clone(payload) });
        if (onceCallbacks.has(callbackId)) unregisterCallback(callbackId);
      }
    },
    listenerCount: (event) => listeners.get(event)?.size ?? 0,
  };

  root.__PACK_MANAGER_TEST_IPC__ = controller;
  root.__TAURI_EVENT_PLUGIN_INTERNALS__ = {
    unregisterListener: removeListener,
  };
  root.__TAURI_INTERNALS__ = {
    callbacks,
    invoke: async (command, args = {}) => {
      calls.push({ command, args: clone(args) });

      if (command === "plugin:event|listen") {
        const event = String(args.event);
        const callbackId = Number(args.handler);
        const eventId = nextEventId++;
        const eventListeners =
          listeners.get(event) ?? new Map<number, number>();
        eventListeners.set(eventId, callbackId);
        listeners.set(event, eventListeners);
        return eventId;
      }

      if (command === "plugin:event|unlisten") {
        const eventId = Number(args.eventId ?? args.id);
        removeListener(String(args.event), eventId);
        return null;
      }

      const configured = responses.get(command);
      if (!configured) {
        throw new Error(
          `fake Tauri IPC: no response configured for command "${command}"`,
        );
      }

      const value =
        configured.values.length > 1
          ? configured.values.shift()
          : configured.values[0];
      if (configured.outcome === "reject") throw clone(value);
      return clone(value);
    },
    runCallback: (id, data) => callbacks.get(id)?.(data),
    transformCallback: (callback, once = false) => {
      const id = nextCallbackId++;
      if (callback) callbacks.set(id, callback);
      if (once) onceCallbacks.add(id);
      return id;
    },
    unregisterCallback,
  };

  for (const [command, result] of initialResults) setResult(command, result);
  for (const instruction of root.__PACK_MANAGER_TEST_IPC_PENDING__ ?? [])
    apply(instruction);
  delete root.__PACK_MANAGER_TEST_IPC_PENDING__;
}

/**
 * Controller for the browser-local Tauri double. It never invokes a native
 * command, process, filesystem path, or remote endpoint.
 */
export class FakeTauriIpc {
  readonly #page: Page;
  readonly #defaults: Map<string, unknown>;
  #revision = 0;

  constructor(page: Page, defaults: Record<string, unknown>) {
    this.#page = page;
    this.#defaults = new Map(Object.entries(defaults));
  }

  async install(): Promise<void> {
    await this.#page.addInitScript(
      installFakeTauriRuntime,
      this.#defaultResults(),
    );
  }

  async respond(command: string, value: unknown): Promise<void> {
    await this.#set(command, { outcome: "resolve", values: [value] });
  }

  async respondSequence(
    command: string,
    values: readonly unknown[],
  ): Promise<void> {
    if (values.length === 0)
      throw new Error("respondSequence requires at least one value");
    await this.#set(command, { outcome: "resolve", values: [...values] });
  }

  async reject(command: string, error: unknown): Promise<void> {
    await this.#set(command, { outcome: "reject", values: [error] });
  }

  async emit(event: string, payload: unknown): Promise<void> {
    await this.#page.evaluate(
      ({ eventName, eventPayload }) => {
        const runtime = (window as FakeTauriWindow).__PACK_MANAGER_TEST_IPC__;
        if (!runtime)
          throw new Error(
            "fake Tauri IPC is not installed; navigate to the app first",
          );
        runtime.emit(eventName, eventPayload);
      },
      { eventName: event, eventPayload: payload },
    );
  }

  async calls(): Promise<TauriIpcCall[]> {
    return this.#page.evaluate(
      () =>
        (window as FakeTauriWindow).__PACK_MANAGER_TEST_IPC__?.calls() ?? [],
    );
  }

  async callsFor(command: string): Promise<TauriIpcCall[]> {
    return (await this.calls()).filter((call) => call.command === command);
  }

  async listenerCount(event: string): Promise<number> {
    return this.#page.evaluate(
      (eventName) =>
        (window as FakeTauriWindow).__PACK_MANAGER_TEST_IPC__?.listenerCount(
          eventName,
        ) ?? 0,
      event,
    );
  }

  async reset(): Promise<void> {
    const revision = ++this.#revision;
    await this.#apply({
      kind: "replace",
      results: this.#defaultResults(revision),
      revision,
    });
  }

  async cleanup(): Promise<void> {
    if (this.#page.isClosed()) return;
    try {
      await this.reset();
    } catch {
      // Browser teardown may race fixture teardown; a closed context is clean.
    }
  }

  async #set(
    command: string,
    result: Omit<RuntimeResult, "revision">,
  ): Promise<void> {
    const instruction: SetRuntimeResult = {
      kind: "set",
      command,
      result: { ...result, revision: ++this.#revision },
    };
    await this.#apply(instruction);
  }

  async #apply(instruction: RuntimeInstruction): Promise<void> {
    await this.#page.addInitScript(applyRuntimeInstruction, instruction);
    await this.#page.evaluate(applyRuntimeInstruction, instruction);
  }

  #defaultResults(revision = 0): Array<[string, RuntimeResult]> {
    return [...this.#defaults].map(([command, value]) => [
      command,
      { outcome: "resolve", values: [value], revision },
    ]);
  }
}
