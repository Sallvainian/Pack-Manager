import "../../test/setup";
import { beforeEach, describe, expect, it, vi } from "vitest";
import { fireEvent, render } from "@testing-library/react";

vi.mock("../../lib/ipc/bridge", () => import("../../test/fakeIpc"));

import { useKeyboard } from "../../hooks/useKeyboard";
import {
  resetStores,
  useManagersStore,
  usePackagesStore,
  useUiStore,
} from "../../store";
import * as fakeIpc from "../../test/fakeIpc";
import { defaultSettings, detectionReport, npmSnapshot } from "../../test/fixtures";

function KB() {
  useKeyboard();
  return null;
}

beforeEach(() => {
  fakeIpc.reset();
  resetStores();
  useManagersStore.getState().setDetection(detectionReport);
  useUiStore.getState().setSettings(defaultSettings);
  usePackagesStore.getState().setSnapshot("npm", npmSnapshot);
  useUiStore.getState().navigate({ kind: "manager", managerId: "npm" });
  fakeIpc.respond("refresh_manager", () => ({ opId: "r1" }));
  fakeIpc.respond("refresh_all", () => ({ opIds: [] }));
  fakeIpc.respond("build_upgrade_plan", () => ({
    planId: "p",
    groups: [],
    excluded: [],
    notes: [],
    warnings: [],
  }));
});

describe("keyboard_map_dispatches_actions", () => {
  it("Cmd+L toggles the drawer", () => {
    render(<KB />);
    expect(useUiStore.getState().drawerOpen).toBe(false);
    fireEvent.keyDown(document.body, { key: "l", metaKey: true });
    expect(useUiStore.getState().drawerOpen).toBe(true);
    fireEvent.keyDown(document.body, { key: "l", metaKey: true });
    expect(useUiStore.getState().drawerOpen).toBe(false);
  });

  it("Cmd+A selects every visible selectable row for the current manager", () => {
    render(<KB />);
    fireEvent.keyDown(document.body, { key: "a", metaKey: true });
    const sel = [...(usePackagesStore.getState().selection.npm ?? [])].sort();
    expect(sel).toEqual(
      [
        "globalPackage:@google/gemini-cli",
        "globalPackage:@just-every/code",
        "globalPackage:dmux",
        "globalPackage:typescript",
      ].sort(),
    );
  });

  it("Cmd+R refreshes the current manager", async () => {
    render(<KB />);
    fireEvent.keyDown(document.body, { key: "r", metaKey: true });
    await vi.waitFor(() => expect(fakeIpc.called("refresh_manager")).toBe(true));
    expect(fakeIpc.callsFor("refresh_manager")[0].args).toEqual({ args: { managerId: "npm" } });
  });

  it("Cmd+Shift+R refreshes all", async () => {
    render(<KB />);
    fireEvent.keyDown(document.body, { key: "R", metaKey: true, shiftKey: true });
    await vi.waitFor(() => expect(fakeIpc.called("refresh_all")).toBe(true));
  });

  it("Esc clears a non-empty selection before closing the drawer", () => {
    render(<KB />);
    fireEvent.keyDown(document.body, { key: "a", metaKey: true });
    expect((usePackagesStore.getState().selection.npm ?? new Set()).size).toBeGreaterThan(0);
    fireEvent.keyDown(document.body, { key: "Escape" });
    expect((usePackagesStore.getState().selection.npm ?? new Set()).size).toBe(0);
  });

  it("Cmd+Shift+U opens the plan sheet for everything", async () => {
    render(<KB />);
    fireEvent.keyDown(document.body, { key: "U", metaKey: true, shiftKey: true });
    await vi.waitFor(() => expect(fakeIpc.called("build_upgrade_plan")).toBe(true));
    const req = fakeIpc.callsFor("build_upgrade_plan")[0].args as {
      args: { selection: unknown };
    };
    expect(req.args.selection).toBeNull();
    await vi.waitFor(() => expect(useUiStore.getState().dialog.kind).toBe("upgradePlan"));
  });

  it("ignores shortcuts while typing in an input (except Escape)", () => {
    render(
      <>
        <KB />
        <input aria-label="probe" />
      </>,
    );
    const input = document.querySelector('input[aria-label="probe"]') as HTMLInputElement;
    input.focus();
    fireEvent.keyDown(input, { key: "l", metaKey: true });
    expect(useUiStore.getState().drawerOpen).toBe(false);
  });
});
