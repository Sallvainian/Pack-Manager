import "../test/setup";
import { beforeEach, describe, expect, it, vi } from "vitest";
import { fireEvent, render, screen } from "@testing-library/react";

vi.mock("../lib/ipc/bridge", () => import("../test/fakeIpc"));

import { SettingsView } from "../components/settings/SettingsView";
import { resetStores, useManagersStore, useUiStore } from "../store";
import * as fakeIpc from "../test/fakeIpc";
import { defaultSettings, detectionReport } from "../test/fixtures";

beforeEach(() => {
  fakeIpc.reset();
  resetStores();
  useManagersStore.getState().setDetection(detectionReport);
  useUiStore.getState().setSettings(defaultSettings);
});

describe("settings form", () => {
  it("renders the preference controls", () => {
    render(<SettingsView />);
    expect(screen.getByLabelText("Auto-refresh on launch")).toBeInTheDocument();
    expect(screen.getByLabelText("Stall threshold (seconds)")).toBeInTheDocument();
    expect(screen.getByLabelText("Log level")).toBeInTheDocument();
  });

  it("persists a toggled setting via set_settings and applies the merged result", async () => {
    fakeIpc.respond("set_settings", (args) => {
      const patch = (args as { args: { patch: object } }).args.patch;
      return { ...defaultSettings, ...patch };
    });
    render(<SettingsView />);

    fireEvent.click(screen.getByLabelText("Auto-refresh on launch"));
    await vi.waitFor(() => expect(fakeIpc.called("set_settings")).toBe(true));
    expect(fakeIpc.callsFor("set_settings")[0].args).toEqual({
      args: { patch: { autoRefreshOnLaunch: false } },
    });
    await vi.waitFor(() =>
      expect(useUiStore.getState().settings?.autoRefreshOnLaunch).toBe(false),
    );
  });

  it("persists the log level via a select", async () => {
    fakeIpc.respond("set_settings", (args) => {
      const patch = (args as { args: { patch: object } }).args.patch;
      return { ...defaultSettings, ...patch };
    });
    render(<SettingsView />);
    fireEvent.change(screen.getByLabelText("Log level"), { target: { value: "trace" } });
    await vi.waitFor(() => expect(fakeIpc.called("set_settings")).toBe(true));
    expect(fakeIpc.callsFor("set_settings")[0].args).toEqual({
      args: { patch: { logLevel: "trace" } },
    });
  });
});

describe("environment report", () => {
  it("shows the PATH source and per-tool evidence", () => {
    render(<SettingsView />);
    expect(screen.getByText("merged")).toBeInTheDocument();
    expect(screen.getByText("resolved at ~/.local/share/mise/shims/uv")).toBeInTheDocument();
  });
});

describe("maintenance actions", () => {
  it("Re-detect invokes detect_managers", async () => {
    fakeIpc.respond("detect_managers", () => detectionReport);
    render(<SettingsView />);
    fireEvent.click(screen.getByRole("button", { name: "Re-detect" }));
    await vi.waitFor(() => expect(fakeIpc.called("detect_managers")).toBe(true));
  });

  it("Export diagnostics and Open Logs Folder invoke their commands", async () => {
    fakeIpc.respond("export_diagnostics", () => ({ zipPath: "/tmp/diag.zip" }));
    fakeIpc.respond("reveal_logs_dir", () => undefined);
    render(<SettingsView />);
    fireEvent.click(screen.getByRole("button", { name: "Export diagnostics" }));
    fireEvent.click(screen.getByRole("button", { name: "Open Logs Folder" }));
    await vi.waitFor(() => expect(fakeIpc.called("export_diagnostics")).toBe(true));
    expect(fakeIpc.called("reveal_logs_dir")).toBe(true);
  });
});
