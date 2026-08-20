/**
 * Frontend-only delivery coverage for Story 6-6. The browser-local Tauri
 * double proves the event/dialog/command limb, not native menu/window events.
 */
import type { OperationRecord } from "../../src/lib/ipc/types";
import { test, expect } from "../support/fixtures";
import { givenPackManagerState, openPackManager } from "../support/helpers/pack-manager";
import {
  QUIT_COMMANDS,
  createQuitGuard,
  whenQuitIsRequested,
} from "../support/helpers/quit-guard";

test.describe("quit guard", () => {
  test("[P0] cancels every active operation and confirms the quit", async ({
    page,
    factories,
    tauriIpc,
  }) => {
    const running: OperationRecord = factories.createOperationRecord({
      opId: "op-running-brew",
      kind: "upgrade",
      executor: "brew",
      subject: "brew",
      status: "running",
      commandLine: "/opt/homebrew/bin/brew upgrade dolt",
      packageIds: ["formula:dolt"],
      startedAt: "2026-07-22T12:00:05.000Z",
    });
    const queued: OperationRecord = factories.createOperationRecord({
      opId: "op-queued-npm",
      kind: "upgrade",
      executor: "npm",
      subject: "npm",
      status: "queued",
      commandLine: "npm install -g typescript@latest",
      packageIds: ["globalPackage:typescript"],
    });
    const guard = createQuitGuard(page);

    await givenPackManagerState(
      tauriIpc,
      factories.createAppState({ operations: [running, queued] }),
    );
    await tauriIpc.respond(QUIT_COMMANDS.cancelOperation, null);
    await tauriIpc.respond(QUIT_COMMANDS.confirmQuit, null);
    await openPackManager(page);

    await whenQuitIsRequested(tauriIpc, [running.opId, queued.opId]);
    await expect(guard.dialog).toBeVisible();
    await expect(guard.heading).toBeVisible();
    await expect(guard.operations()).toHaveCount(2);
    await expect(guard.operation("Upgrade 1 · Homebrew")).toBeVisible();
    await expect(guard.operation("Upgrade 1 · npm")).toBeVisible();
    await expect(
      guard.dialog.getByText(/Quitting now will cancel 2 running operations/),
    ).toBeVisible();
    await expect(page.getByRole("button", { name: /undo|roll ?back|restore/i })).toHaveCount(0);

    await guard.cancelAndQuit.click();
    await expect
      .poll(() => tauriIpc.callsFor(QUIT_COMMANDS.cancelOperation))
      .toEqual([
        {
          command: QUIT_COMMANDS.cancelOperation,
          args: { args: { opId: running.opId } },
        },
        {
          command: QUIT_COMMANDS.cancelOperation,
          args: { args: { opId: queued.opId } },
        },
      ]);
    await expect
      .poll(() => tauriIpc.callsFor(QUIT_COMMANDS.confirmQuit))
      .toEqual([{ command: QUIT_COMMANDS.confirmQuit, args: {} }]);
    await expect(guard.dialog).toBeHidden();
  });

  test("[P1] Keep running is inert", async ({ page, factories, tauriIpc }) => {
    const running = factories.createOperationRecord({
      opId: "op-running-uv",
      kind: "upgrade",
      executor: "uv",
      subject: "uv",
      status: "running",
      commandLine: "uv tool upgrade ruff",
      packageIds: ["tool:ruff"],
      startedAt: "2026-07-22T12:00:05.000Z",
    });
    const guard = createQuitGuard(page);

    await givenPackManagerState(
      tauriIpc,
      factories.createAppState({ operations: [running] }),
    );
    await tauriIpc.respond(QUIT_COMMANDS.cancelOperation, null);
    await tauriIpc.respond(QUIT_COMMANDS.confirmQuit, null);
    await openPackManager(page);
    await whenQuitIsRequested(tauriIpc, [running.opId]);

    await guard.keepRunning.click();
    await expect(guard.dialog).toBeHidden();
    expect(await tauriIpc.callsFor(QUIT_COMMANDS.cancelOperation)).toEqual([]);
    expect(await tauriIpc.callsFor(QUIT_COMMANDS.confirmQuit)).toEqual([]);
    await expect(page.getByRole("heading", { name: "Packages", level: 1 })).toBeVisible();
  });

  test("[P2] queued-only work is presented as active", async ({
    page,
    factories,
    tauriIpc,
  }) => {
    const queued = factories.createOperationRecord({
      opId: "op-queued-only",
      kind: "upgrade",
      executor: "mise",
      subject: "mise",
      status: "queued",
      commandLine: "mise upgrade node",
      packageIds: ["tool:node"],
    });
    const guard = createQuitGuard(page);

    await givenPackManagerState(
      tauriIpc,
      factories.createAppState({ operations: [queued] }),
    );
    await tauriIpc.respond(QUIT_COMMANDS.cancelOperation, null);
    await tauriIpc.respond(QUIT_COMMANDS.confirmQuit, null);
    await openPackManager(page);
    await whenQuitIsRequested(tauriIpc, [queued.opId]);

    await expect(guard.dialog).toBeVisible();
    await expect(guard.operations()).toHaveCount(1);
    await expect(guard.operation("Upgrade 1 · mise")).toBeVisible();
    await expect(
      guard.dialog.getByText(/Quitting now will cancel 1 running operation(?!s)/),
    ).toBeVisible();
  });
});
