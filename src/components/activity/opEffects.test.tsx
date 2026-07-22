import "../../test/setup";
import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import { act, fireEvent, render, screen } from "@testing-library/react";

vi.mock("../../lib/ipc/bridge", () => import("../../test/fakeIpc"));

import { useOperationEffects } from "./useOperationEffects";
import { ToastHost } from "../shell/ToastHost";
import type { IpcError, OpKind, OpStatus, OpStatusEvent } from "../../lib/ipc/types";
import { resetStores, useManagersStore, useOperationsStore, useUiStore } from "../../store";
import { defaultSettings, detectionReport, upgradeRecord } from "../../test/fixtures";

function Effects() {
  useOperationEffects();
  return null;
}

function evt(
  opId: string,
  status: OpStatus,
  over: Partial<OpStatusEvent> = {},
): OpStatusEvent {
  return {
    opId,
    kind: "upgrade",
    executor: "npm",
    subject: "npm",
    status,
    commandLine: "npm install -g typescript@latest",
    logPath: "/tmp/op.log",
    ...over,
  };
}

/** Drive an op through queued -> running (a genuine start transition). */
function start(opId: string, kind: OpKind, subject: OpStatusEvent["subject"]) {
  act(() =>
    useOperationsStore.getState().applyStatus(evt(opId, "queued", { kind, subject, executor: subject })),
  );
  act(() =>
    useOperationsStore.getState().applyStatus(evt(opId, "running", { kind, subject, executor: subject })),
  );
}

beforeEach(() => {
  resetStores();
  useManagersStore.getState().setDetection(detectionReport);
});

describe("drawer_auto_open_honors_setting", () => {
  it("opens+focuses on a mutating start only when enabled, and never for refreshes", () => {
    useUiStore.getState().setSettings({ ...defaultSettings, autoOpenDrawer: true });
    render(<Effects />);

    // Upgrade start with the setting on -> drawer opens, op focused.
    start("opA", "upgrade", "npm");
    expect(useUiStore.getState().drawerOpen).toBe(true);
    expect(useUiStore.getState().focusedOpId).toBe("opA");

    // Reset, disable the setting: a new mutating start must NOT open the drawer.
    act(() => useUiStore.getState().setDrawerOpen(false));
    act(() => useUiStore.getState().setSettings({ ...defaultSettings, autoOpenDrawer: false }));
    start("opB", "selfUpdate", "mise");
    expect(useUiStore.getState().drawerOpen).toBe(false);

    // Re-enable: a refresh start still never opens the drawer.
    act(() => useUiStore.getState().setSettings({ ...defaultSettings, autoOpenDrawer: true }));
    start("opC", "refresh", "brew");
    expect(useUiStore.getState().drawerOpen).toBe(false);
  });
});

describe("failure_toast_persists_and_view_log_focuses_op", () => {
  const error: IpcError = {
    code: "non_zero_exit",
    message: "npm exited 1",
    logPath: "/tmp/op.log",
  };

  beforeEach(() => vi.useFakeTimers());
  afterEach(() => vi.useRealTimers());

  it("raises a persistent failure toast whose View log focuses the op + opens the drawer", () => {
    // autoOpenDrawer off so the drawer state is driven only by the toast action.
    useUiStore.getState().setSettings({ ...defaultSettings, autoOpenDrawer: false });
    render(
      <>
        <Effects />
        <ToastHost />
      </>,
    );

    start("op-fail", "upgrade", "npm");
    act(() =>
      useOperationsStore
        .getState()
        .applyStatus(evt("op-fail", "failed", { exitCode: 1, error, finishedAt: "2026-07-22T14:05:00Z" })),
    );

    expect(screen.getByText(/upgrade failed \(exit 1\)/i)).toBeInTheDocument();
    // Persistent: still present after the 4s success auto-dismiss window.
    act(() => vi.advanceTimersByTime(5000));
    expect(screen.getByText(/upgrade failed \(exit 1\)/i)).toBeInTheDocument();

    fireEvent.click(screen.getByRole("button", { name: "View log" }));
    expect(useUiStore.getState().focusedOpId).toBe("op-fail");
    expect(useUiStore.getState().drawerOpen).toBe(true);
  });
});

describe("success and cancel toasts", () => {
  it("raises an auto-dismissing success toast for a completed upgrade, but none for a refresh", () => {
    useUiStore.getState().setSettings({ ...defaultSettings, autoOpenDrawer: false });
    // Hydrate the op with its packageIds (op:status events don't carry them, so a
    // record hydrated from get_state/list is where the count comes from).
    useOperationsStore.getState().setRecords([
      {
        ...upgradeRecord,
        opId: "op-ok",
        status: "queued",
        subject: "npm",
        executor: "npm",
        kind: "upgrade",
        packageIds: ["globalPackage:typescript"],
      },
    ]);
    render(<Effects />);

    start("op-ok", "upgrade", "npm");
    act(() => useOperationsStore.getState().applyStatus(evt("op-ok", "succeeded", { exitCode: 0 })));
    const toasts = useUiStore.getState().toasts;
    expect(toasts.some((t) => t.kind === "success" && /1 package upgraded/.test(t.message))).toBe(true);

    // A refresh completing raises no toast (its result shows on the manager card).
    const before = useUiStore.getState().toasts.length;
    start("op-rf", "refresh", "brew");
    act(() =>
      useOperationsStore.getState().applyStatus(evt("op-rf", "succeeded", { kind: "refresh", subject: "brew", executor: "brew" })),
    );
    expect(useUiStore.getState().toasts.length).toBe(before);
  });
});
