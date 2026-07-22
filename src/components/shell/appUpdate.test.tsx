import "../../test/setup";
import { beforeEach, describe, expect, it, vi } from "vitest";
import { fireEvent, render, screen, waitFor } from "@testing-library/react";

vi.mock("../../lib/ipc/bridge", () => import("../../test/fakeIpc"));

import { UpdateStatusItem } from "./UpdateStatusItem";
import { DialogHost } from "../dialogs/DialogHost";
import { onAppUpdate } from "../../lib/ipc/events";
import type { AppUpdateState, AppUpdateStatus, OpStatusEvent } from "../../lib/ipc/types";
import {
  resetStores,
  useAppUpdateStore,
  useManagersStore,
  useOperationsStore,
  useUiStore,
} from "../../store";
import * as fakeIpc from "../../test/fakeIpc";
import { defaultSettings, detectionReport } from "../../test/fixtures";

beforeEach(() => {
  fakeIpc.reset();
  resetStores();
  useManagersStore.getState().setDetection(detectionReport);
  useUiStore.getState().setSettings(defaultSettings);
});

function status(
  state: AppUpdateState,
  lastTrigger: AppUpdateStatus["lastTrigger"] = "automatic",
): AppUpdateStatus {
  return { currentVersion: "0.1.1", state, lastTrigger };
}

function npmUpgradeRunning(): OpStatusEvent {
  return {
    opId: "op-npm-upgrade",
    kind: "upgrade",
    executor: "npm",
    subject: "npm",
    status: "running",
    commandLine: "/Users/sallvain/.local/share/mise/shims/npm install -g typescript@latest",
    logPath: "/tmp/npm.log",
  };
}

describe("update_status_item_hidden_when_idle_and_up_to_date", () => {
  it("renders nothing before a check, while checking, or when already current", () => {
    const { container, rerender } = render(<UpdateStatusItem />);
    expect(container).toBeEmptyDOMElement();

    for (const state of [
      { kind: "idle" } as const,
      { kind: "checking" } as const,
      { kind: "upToDate" } as const,
    ]) {
      useAppUpdateStore.getState().setStatus(status(state));
      rerender(<UpdateStatusItem />);
      expect(container, `${state.kind} must stay invisible`).toBeEmptyDOMElement();
    }
  });

  it("keeps a failed check out of the chrome — offline must not leave a permanent warning", () => {
    useAppUpdateStore
      .getState()
      .setStatus(status({ kind: "error", message: "network unreachable" }));
    const { container } = render(<UpdateStatusItem />);
    expect(container).toBeEmptyDOMElement();
  });
});

describe("downloading_state_renders_progress", () => {
  it("shows a percentage when the server sent a content length", () => {
    useAppUpdateStore
      .getState()
      .setStatus(status({ kind: "downloading", version: "0.2.0", received: 512, total: 2048 }));
    render(<UpdateStatusItem />);
    expect(screen.getByTestId("update-downloading").textContent).toContain("0.2.0 · 25%");
  });

  it("degrades to an indeterminate label rather than fabricating a percentage", () => {
    useAppUpdateStore
      .getState()
      .setStatus(status({ kind: "downloading", version: "0.2.0", received: 512, total: null }));
    render(<UpdateStatusItem />);
    const text = screen.getByTestId("update-downloading").textContent ?? "";
    expect(text).toContain("Downloading update 0.2.0");
    expect(text).not.toContain("%");
  });
});

describe("ready_state_renders_restart_button_and_invokes_install", () => {
  it("installs immediately when nothing is running", async () => {
    fakeIpc.respond("install_app_update", () => undefined);
    useAppUpdateStore
      .getState()
      .setStatus(status({ kind: "readyToInstall", version: "0.2.0", notes: "Adds auto-update" }));
    render(<UpdateStatusItem />);

    const button = screen.getByRole("button", { name: /Restart to update 0\.2\.0/ });
    fireEvent.click(button);
    await waitFor(() => expect(fakeIpc.called("install_app_update")).toBe(true));
    expect(useUiStore.getState().dialog.kind).toBe("none");
  });
});

describe("restart_with_running_ops_opens_quit_guard", () => {
  it("confirms before killing in-flight operations, then installs on confirm", async () => {
    fakeIpc.respond("install_app_update", () => undefined);
    fakeIpc.respond("cancel_operation", () => undefined);
    useOperationsStore.getState().applyStatus(npmUpgradeRunning());
    useAppUpdateStore
      .getState()
      .setStatus(status({ kind: "readyToInstall", version: "0.2.0", notes: null }));
    render(
      <>
        <UpdateStatusItem />
        <DialogHost />
      </>,
    );

    fireEvent.click(screen.getByRole("button", { name: /Restart to update/ }));
    expect(fakeIpc.called("install_app_update")).toBe(false);

    const dialog = screen.getByRole("alertdialog");
    expect(dialog.textContent).toContain("Restarting to update");
    fireEvent.click(screen.getByRole("button", { name: "Cancel operations and restart" }));

    expect(fakeIpc.calls.filter((c) => c.cmd === "cancel_operation")).toHaveLength(1);
    await waitFor(() => expect(fakeIpc.called("install_app_update")).toBe(true));
  });
});

describe("manual_install_required_is_surfaced_instead_of_prompting_for_a_password", () => {
  it("explains why, rather than silently doing nothing", () => {
    useAppUpdateStore.getState().setStatus(
      status({
        kind: "manualInstallRequired",
        version: "0.2.0",
        reason: "/Volumes/Pack-Manager is not writable — move Pack-Manager to /Applications",
      }),
    );
    render(<UpdateStatusItem />);
    const chip = screen.getByTestId("update-manual-install");
    expect(chip.textContent).toContain("0.2.0 needs a manual install");
    expect(chip.getAttribute("title")).toContain("move Pack-Manager to /Applications");
  });
});

describe("manual_check_up_to_date_pushes_info_toast_automatic_does_not", () => {
  it("stays silent for the 6h timer and speaks up for a menu-bar check", () => {
    onAppUpdate(status({ kind: "upToDate" }, "automatic"));
    expect(useUiStore.getState().toasts).toHaveLength(0);

    onAppUpdate(status({ kind: "upToDate" }, "manual"));
    const toasts = useUiStore.getState().toasts;
    expect(toasts).toHaveLength(1);
    expect(toasts[0].message).toBe("Pack-Manager 0.1.1 is up to date");
    expect(toasts[0].kind).toBe("info");
  });

  it("toasts a manual failure persistently but never an automatic one", () => {
    onAppUpdate(status({ kind: "error", message: "network unreachable" }, "automatic"));
    expect(useUiStore.getState().toasts).toHaveLength(0);

    onAppUpdate(status({ kind: "error", message: "network unreachable" }, "manual"));
    const toasts = useUiStore.getState().toasts;
    expect(toasts).toHaveLength(1);
    expect(toasts[0].kind).toBe("error");
    expect(toasts[0].persistent).toBe(true);
    expect(toasts[0].message).toContain("network unreachable");
  });

  it("does not re-toast when an identical manual result arrives twice", () => {
    onAppUpdate(status({ kind: "upToDate" }, "manual"));
    onAppUpdate(status({ kind: "upToDate" }, "manual"));
    expect(useUiStore.getState().toasts).toHaveLength(1);
  });
});
