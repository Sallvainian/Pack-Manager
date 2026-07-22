/**
 * test/fakeIpc.ts — the mock for `lib/ipc/bridge` (SPEC §7.5).
 *
 * Test files install it with:
 *   `vi.mock("../lib/ipc/bridge", () => import("../test/fakeIpc"));`
 * The mocked bridge then exposes `invoke`/`listen` backed by the mutable state
 * below, which the same test controls through `respond`, `emit`, `calls`, and
 * `reset` (imported directly — Vitest resolves both imports to this one module,
 * so they share state).
 */
export interface RecordedCall {
  cmd: string;
  args: unknown;
}

export type Responder = (args: unknown) => unknown | Promise<unknown>;

/** Every command invocation, in order — assert against this in tests. */
export const calls: RecordedCall[] = [];

const responders = new Map<string, Responder>();
const listeners = new Map<string, Set<(e: { payload: unknown }) => void>>();
const listenFailures = new Map<string, unknown>();

/** Mocked `bridge.invoke`. */
export async function invoke<T>(cmd: string, args?: Record<string, unknown>): Promise<T> {
  calls.push({ cmd, args });
  const responder = responders.get(cmd);
  if (!responder) {
    throw new Error(`fakeIpc: no responder registered for command "${cmd}"`);
  }
  return (await responder(args)) as T;
}

/** Mocked `bridge.listen`. */
export async function listen<T>(
  event: string,
  handler: (e: { payload: T }) => void,
): Promise<() => void> {
  if (listenFailures.has(event)) {
    throw listenFailures.get(event);
  }
  let set = listeners.get(event);
  if (!set) {
    set = new Set();
    listeners.set(event, set);
  }
  const wrapped = handler as (e: { payload: unknown }) => void;
  set.add(wrapped);
  return () => {
    set?.delete(wrapped);
  };
}

/** Register a handler (or static value) for a command. */
export function respond(cmd: string, responder: Responder | (() => unknown)): void {
  responders.set(cmd, responder as Responder);
}

/** Deliver an event to every current listener. */
export function emit(event: string, payload: unknown): void {
  listeners.get(event)?.forEach((cb) => cb({ payload }));
}

/** Convenience: was `cmd` invoked at all? */
export function called(cmd: string): boolean {
  return calls.some((c) => c.cmd === cmd);
}

/** Convenience: the recorded calls for one command. */
export function callsFor(cmd: string): RecordedCall[] {
  return calls.filter((c) => c.cmd === cmd);
}

/** Make the next `listen(event, …)` calls reject with `error`. */
export function failListen(event: string, error: unknown): void {
  listenFailures.set(event, error);
}

/** Count of live listeners for an event (leak assertions). */
export function listenerCount(event: string): number {
  return listeners.get(event)?.size ?? 0;
}

/** Clear all responders, listeners, and recorded calls between tests. */
export function reset(): void {
  responders.clear();
  listeners.clear();
  listenFailures.clear();
  calls.length = 0;
}
