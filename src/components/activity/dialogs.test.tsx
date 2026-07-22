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
  fakeIpc.respond("cancel_operation", () => undefined);
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
  it("lists the running ops and cancels each on quit", async () => {
    act(() => useUiStore.getState().openDialog({ kind: "quitGuard", opIds: [OP_ID] }));
    render(<DialogHost />);

    const dialog = screen.getByRole("alertdialog");
    expect(dialog.textContent).toContain("Operations still running");
    expect(dialog.textContent).toContain("Upgrade · Homebrew");

    fireEvent.click(screen.getByRole("button", { name: "Cancel operations and quit" }));
    await vi.waitFor(() => expect(fakeIpc.called("cancel_operation")).toBe(true));
    expect(useUiStore.getState().dialog.kind).toBe("none");
  });
});
