import "../../test/setup";
import { beforeEach, describe, expect, it, vi } from "vitest";
import { act, fireEvent, render, screen } from "@testing-library/react";

vi.mock("../../lib/ipc/bridge", () => import("../../test/fakeIpc"));

import { DialogHost } from "../dialogs/DialogHost";
import type { OpStatusEvent } from "../../lib/ipc/types";
import { resetStores, useManagersStore, useOperationsStore, useUiStore } from "../../store";
import * as fakeIpc from "../../test/fakeIpc";
import { detectionReport } from "../../test/fixtures";

const OP_ID = "01981f2e-6a3b-7c40-9d5e-1f2a3b4c5d6e";
const QUEUED_OP_ID = "01981f2e-6a3b-7c40-9d5e-1f2a3b4c5d6f";

function runningOp(): OpStatusEvent {
  return {
    opId: OP_ID,
    kind: "upgrade",
    executor: "brew",
    subject: "brew",
    status: "running",
    commandLine: "/opt/homebrew/bin/brew upgrade dolt",
    logPath: "/tmp/op.log",
  };
}

beforeEach(() => {
  fakeIpc.reset();
  resetStores();
  useManagersStore.getState().setDetection(detectionReport);
  useOperationsStore.getState().applyStatus(runningOp());
  useOperationsStore.getState().applyStatus({
    ...runningOp(),
    opId: QUEUED_OP_ID,
    executor: "npm",
    subject: "npm",
    status: "queued",
    commandLine: "npm install -g typescript@latest",
  });
  fakeIpc.respond("cancel_operation", () => undefined);
  fakeIpc.respond("confirm_quit", () => undefined);
  fakeIpc.respond("log_frontend_event", () => undefined);
});

describe("stall_dialog_keep_waiting_vs_cancel", () => {
  it("renders the no-password handoff copy; Keep waiting dismisses, Cancel cancels", async () => {
    act(() =>
      useUiStore.getState().openDialog({ kind: "stalled", opId: OP_ID, silentForSecs: 120 }),
    );
    render(<DialogHost />);

    const dialog = screen.getByRole("alertdialog");
    expect(dialog.textContent).toContain("No output for 2m");
    expect(dialog.textContent).toContain("never enters passwords");
    expect(screen.getByText("/opt/homebrew/bin/brew upgrade dolt")).toBeInTheDocument();

    // Keep waiting -> dialog closes, no cancel issued.
    fireEvent.click(screen.getByRole("button", { name: "Keep waiting" }));
    expect(useUiStore.getState().dialog.kind).toBe("none");
    expect(fakeIpc.called("cancel_operation")).toBe(false);

    // Re-open and cancel -> cancel_operation for this op, dialog closes.
    act(() =>
      useUiStore.getState().openDialog({ kind: "stalled", opId: OP_ID, silentForSecs: 120 }),
    );
    fireEvent.click(screen.getByRole("button", { name: "Cancel operation" }));
    await vi.waitFor(() => expect(fakeIpc.called("cancel_operation")).toBe(true));
    expect(fakeIpc.callsFor("cancel_operation")[0].args).toEqual({ args: { opId: OP_ID } });
    expect(useUiStore.getState().dialog.kind).toBe("none");
  });
});

describe("quit_guard_lists_ops_and_cancels_all", () => {
  it("lists every active op, cancels each, then confirms the quit", async () => {
    act(() =>
      useUiStore.getState().openDialog({ kind: "quitGuard", opIds: [OP_ID, QUEUED_OP_ID] }),
    );
    render(<DialogHost />);

    const dialog = screen.getByRole("alertdialog");
    expect(dialog.textContent).toContain("Operations still running");
    expect(dialog.textContent).toContain("Upgrade · Homebrew");
    expect(dialog.textContent).toContain("Upgrade · npm");
    expect(screen.getAllByRole("button").map((button) => button.textContent?.trim()).sort()).toEqual([
      "Cancel operations and quit",
      "Keep running",
    ]);
    expect(dialog.textContent).not.toMatch(/undo|roll ?back|restore/i);

    fireEvent.click(screen.getByRole("button", { name: "Cancel operations and quit" }));
    await vi.waitFor(() => expect(fakeIpc.called("confirm_quit")).toBe(true));
    expect(fakeIpc.callsFor("cancel_operation").map((call) => call.args)).toEqual([
      { args: { opId: OP_ID } },
      { args: { opId: QUEUED_OP_ID } },
    ]);
    expect(fakeIpc.callsFor("confirm_quit")).toHaveLength(1);
    expect(useUiStore.getState().dialog.kind).toBe("none");
  });

  it("keeps every operation running when the user declines the quit", () => {
    act(() =>
      useUiStore.getState().openDialog({ kind: "quitGuard", opIds: [OP_ID, QUEUED_OP_ID] }),
    );
    render(<DialogHost />);

    fireEvent.click(screen.getByRole("button", { name: "Keep running" }));

    expect(useUiStore.getState().dialog.kind).toBe("none");
    expect(fakeIpc.called("cancel_operation")).toBe(false);
    expect(fakeIpc.called("confirm_quit")).toBe(false);
    expect(useOperationsStore.getState().byId[OP_ID].status).toBe("running");
    expect(useOperationsStore.getState().byId[QUEUED_OP_ID].status).toBe("queued");
  });

  it("keeps a refused operation visible when its status event has not arrived", () => {
    const missingOpId = "op-missing-from-local-store";
    act(() => useUiStore.getState().openDialog({ kind: "quitGuard", opIds: [missingOpId] }));
    render(<DialogHost />);

    const dialog = screen.getByRole("alertdialog");
    expect(dialog.textContent).toContain("cancel 1 running operation");
    expect(dialog.textContent).toContain(`Operation · ${missingOpId}`);
  });

  it("waits for every cancellation request before confirming the quit", async () => {
    const resolveCancel: Array<() => void> = [];
    fakeIpc.respond(
      "cancel_operation",
      () => new Promise<void>((resolve) => resolveCancel.push(resolve)),
    );
    act(() =>
      useUiStore.getState().openDialog({ kind: "quitGuard", opIds: [OP_ID, QUEUED_OP_ID] }),
    );
    render(<DialogHost />);

    fireEvent.click(screen.getByRole("button", { name: "Cancel operations and quit" }));
    await vi.waitFor(() => expect(resolveCancel).toHaveLength(2));
    expect(fakeIpc.called("confirm_quit")).toBe(false);

    resolveCancel[0]();
    await Promise.resolve();
    expect(fakeIpc.called("confirm_quit")).toBe(false);

    resolveCancel[1]();
    await vi.waitFor(() => expect(fakeIpc.called("confirm_quit")).toBe(true));
    expect(fakeIpc.callsFor("confirm_quit")).toHaveLength(1);
  });

  it("logs a failed cancellation and still confirms the quit", async () => {
    fakeIpc.respond("cancel_operation", () => {
      throw { code: "internal", message: "Could not cancel the operation." };
    });
    act(() => useUiStore.getState().openDialog({ kind: "quitGuard", opIds: [OP_ID] }));
    render(<DialogHost />);

    fireEvent.click(screen.getByRole("button", { name: "Cancel operations and quit" }));

    await vi.waitFor(() => expect(fakeIpc.called("confirm_quit")).toBe(true));
    const logged = fakeIpc.callsFor("log_frontend_event")[0].args as {
      args: { level: string; message: string };
    };
    expect(logged.args.level).toBe("error");
    expect(logged.args.message).toContain("internal");
    expect(logged.args.message).not.toContain("[object Object]");
  });

  it("contains a failed fallback logger after a failed confirmed quit", async () => {
    fakeIpc.respond("confirm_quit", () => {
      throw { code: "internal", message: "exit refused", detail: "app.exit(0) rejected" };
    });
    fakeIpc.respond("log_frontend_event", () => {
      throw new Error("logging transport unavailable");
    });
    act(() => useUiStore.getState().openDialog({ kind: "quitGuard", opIds: [OP_ID] }));
    render(<DialogHost />);

    fireEvent.click(screen.getByRole("button", { name: "Cancel operations and quit" }));

    await vi.waitFor(() => expect(fakeIpc.called("log_frontend_event")).toBe(true));
    const logged = fakeIpc.callsFor("log_frontend_event")[0].args as {
      args: { message: string };
    };
    expect(logged.args.message).toContain("exit refused");
    expect(logged.args.message).not.toContain("[object Object]");
    await new Promise((resolve) => setTimeout(resolve, 0));
  });
});
