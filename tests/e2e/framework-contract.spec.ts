import type { Page } from "@playwright/test";

import { test, expect } from "../support/fixtures";

interface ContractEventState {
  payloads: unknown[];
  unlisten?: () => Promise<void>;
}

interface ContractWindow extends Window {
  __PACK_MANAGER_CONTRACT_EVENT__?: ContractEventState;
}

async function invokeThroughBridge(
  page: Page,
  command: string,
  args: Record<string, unknown> = {},
): Promise<unknown> {
  return page.evaluate(
    async ({ commandName, commandArgs }) => {
      const bridgeModulePath = "/src/lib/ipc/bridge.ts";
      const { invoke } = (await import(bridgeModulePath)) as {
        invoke<T>(command: string, args?: Record<string, unknown>): Promise<T>;
      };
      return invoke(commandName, commandArgs);
    },
    { commandName: command, commandArgs: args },
  );
}

async function invokeErrorThroughBridge(
  page: Page,
  command: string,
): Promise<unknown> {
  return page.evaluate(async (commandName) => {
    const bridgeModulePath = "/src/lib/ipc/bridge.ts";
    const { invoke } = (await import(bridgeModulePath)) as {
      invoke<T>(command: string): Promise<T>;
    };
    try {
      await invoke(commandName);
      return null;
    } catch (error) {
      return error instanceof Error ? error.message : error;
    }
  }, command);
}

test.describe("Playwright framework contract", () => {
  test("drives the fake Tauri runtime through the real API wrappers", async ({
    page,
    factories,
    tauriIpc,
  }, testInfo) => {
    const seedIdentity = testInfo.titlePath.join("::");
    let expectedSeed = 2_166_136_261;
    for (let index = 0; index < seedIdentity.length; index += 1) {
      expectedSeed ^= seedIdentity.charCodeAt(index);
      expectedSeed = Math.imul(expectedSeed, 16_777_619);
    }
    expect(factories.seed).toBe(expectedSeed >>> 0);

    await page.goto("/");
    await tauriIpc.reset();

    await test.step("invoke records arguments and consumes response sequences", async () => {
      await tauriIpc.respond("contract_call", { accepted: true });
      await expect(
        invokeThroughBridge(page, "contract_call", { packageId: "npm:test" }),
      ).resolves.toEqual({ accepted: true });
      await expect(tauriIpc.callsFor("contract_call")).resolves.toEqual([
        {
          command: "contract_call",
          args: { packageId: "npm:test" },
        },
      ]);

      await tauriIpc.respondSequence("contract_sequence", ["first", "second"]);
      await expect(
        invokeThroughBridge(page, "contract_sequence"),
      ).resolves.toBe("first");
      await expect(
        invokeThroughBridge(page, "contract_sequence"),
      ).resolves.toBe("second");
    });

    await test.step("invoke surfaces configured and unknown-command failures", async () => {
      const rejection = { code: "contract_rejected", message: "expected" };
      await tauriIpc.reject("contract_rejection", rejection);
      await expect(
        invokeErrorThroughBridge(page, "contract_rejection"),
      ).resolves.toEqual(rejection);
      await expect(
        invokeErrorThroughBridge(page, "contract_unknown"),
      ).resolves.toContain(
        'fake Tauri IPC: no response configured for command "contract_unknown"',
      );
    });

    await test.step("listen receives events and unlisten releases the callback", async () => {
      await page.evaluate(async () => {
        const bridgeModulePath = "/src/lib/ipc/bridge.ts";
        const { listen } = (await import(bridgeModulePath)) as {
          listen<T>(
            event: string,
            handler: (event: { payload: T }) => void,
          ): Promise<() => Promise<void>>;
        };
        const state: ContractEventState = { payloads: [] };
        state.unlisten = await listen("contract:event", (event) => {
          state.payloads.push(event.payload);
        });
        (window as ContractWindow).__PACK_MANAGER_CONTRACT_EVENT__ = state;
      });

      await expect(tauriIpc.listenerCount("contract:event")).resolves.toBe(1);
      await tauriIpc.emit("contract:event", { revision: 1 });
      await expect(
        page.evaluate(
          () =>
            (window as ContractWindow).__PACK_MANAGER_CONTRACT_EVENT__
              ?.payloads,
        ),
      ).resolves.toEqual([{ revision: 1 }]);

      await page.evaluate(async () => {
        await (
          window as ContractWindow
        ).__PACK_MANAGER_CONTRACT_EVENT__?.unlisten?.();
      });
      await expect(tauriIpc.listenerCount("contract:event")).resolves.toBe(0);
    });

    await test.step("responses survive reload and reset restores defaults", async () => {
      await tauriIpc.respond("contract_reload", "persisted");
      await page.reload();
      await expect(invokeThroughBridge(page, "contract_reload")).resolves.toBe(
        "persisted",
      );

      await tauriIpc.reset();
      await expect(tauriIpc.calls()).resolves.toEqual([]);
      await page.reload();
      await expect(
        invokeErrorThroughBridge(page, "contract_reload"),
      ).resolves.toContain(
        'fake Tauri IPC: no response configured for command "contract_reload"',
      );
      await expect(invokeThroughBridge(page, "get_state")).resolves.toEqual(
        expect.objectContaining({
          operations: [],
          snapshots: expect.any(Array),
        }),
      );
    });
  });

  test("blocks outbound requests before they leave the browser", async ({
    page,
    networkIsolation,
  }) => {
    await page.goto("/");
    const externalUrl = "https://example.invalid/pack-manager-contract";

    await expect(
      page.evaluate(async (url) => {
        try {
          await fetch(url);
          return "resolved";
        } catch {
          return "blocked";
        }
      }, externalUrl),
    ).resolves.toBe("blocked");
    expect(networkIsolation.blockedRequestUrls).toContain(externalUrl);
  });
});
