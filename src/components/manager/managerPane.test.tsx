import "../../test/setup";
import { beforeEach, describe, expect, it, vi } from "vitest";
import { fireEvent, render, screen } from "@testing-library/react";

vi.mock("../../lib/ipc/bridge", () => import("../../test/fakeIpc"));

import { ManagerPane } from "./ManagerPane";
import { resetStores, useManagersStore, usePackagesStore, useUiStore } from "../../store";
import * as fakeIpc from "../../test/fakeIpc";
import {
  brewSnapshot,
  defaultSettings,
  detectionReport,
  miseSnapshot,
  npmSnapshot,
  uvSnapshot,
} from "../../test/fixtures";

beforeEach(() => {
  fakeIpc.reset();
  resetStores();
  useManagersStore.getState().setDetection(detectionReport);
  useUiStore.getState().setSettings(defaultSettings);
});

describe("selection_shift_range_tri_state_and_toolbar_count", () => {
  it("shift-click selects the inclusive range and drives the toolbar + tri-state", () => {
    usePackagesStore.getState().setSnapshot("npm", npmSnapshot);
    render(<ManagerPane managerId="npm" />);

    fireEvent.click(screen.getByLabelText("Select @google/gemini-cli"));
    fireEvent.click(screen.getByLabelText("Select dmux"), { shiftKey: true });

    // gemini-cli (anchor) .. dmux inclusive → 3 rows.
    expect(screen.getByText("3 selected")).toBeInTheDocument();
    expect(usePackagesStore.getState().selection["npm"]?.size).toBe(3);

    const header = screen.getByLabelText("Select all visible packages") as HTMLInputElement;
    expect(header.indeterminate).toBe(true);
    expect(header.checked).toBe(false);
  });
});

describe("select_all_respects_filter_and_never_greedy_or_pinned", () => {
  it("select-all picks only visible, outdated, non-pinned, non-greedy rows", () => {
    usePackagesStore.getState().setSnapshot("brew", brewSnapshot);
    render(<ManagerPane managerId="brew" />);

    // Pinned rows are not selectable.
    expect((screen.getByLabelText("Select deno") as HTMLInputElement).disabled).toBe(true);

    fireEvent.click(screen.getByLabelText("Select all visible packages"));

    const selected = usePackagesStore.getState().selection["brew"] ?? new Set<string>();
    expect([...selected]).toEqual(["formula:dolt"]);
    expect(selected.has("formula:deno")).toBe(false); // pinned
    expect(selected.has("caskGreedy:openusage")).toBe(false); // greedy
  });
});

describe("outdated_only_toggle_default_behavior", () => {
  it("defaults to outdated-only and reveals up-to-date rows when toggled off", () => {
    usePackagesStore.getState().setSnapshot("mise", miseSnapshot);
    render(<ManagerPane managerId="mise" />);

    // `rust stable stable stable` is up-to-date → hidden by default.
    expect(screen.queryByTestId("row-tool:rust")).not.toBeInTheDocument();
    expect(screen.getByTestId("row-tool:deno")).toBeInTheDocument();

    fireEvent.click(screen.getByLabelText("Outdated only"));
    expect(screen.getByTestId("row-tool:rust")).toBeInTheDocument();
  });
});

describe("search_filters_names_and_executables", () => {
  it("matches both package names and their executables", async () => {
    usePackagesStore.getState().setSnapshot("uv", uvSnapshot);
    usePackagesStore.getState().setOutdatedOnly("uv", false);
    render(<ManagerPane managerId="uv" />);

    // Both visible with outdated-only off.
    expect(screen.getByTestId("row-tool:ruff")).toBeInTheDocument();
    expect(screen.getByTestId("row-tool:serena-agent")).toBeInTheDocument();

    // "hooks" appears only in serena-agent's executables, not its name.
    fireEvent.change(screen.getByLabelText("Search packages"), { target: { value: "hooks" } });
    await vi.waitFor(() => {
      expect(screen.getByTestId("row-tool:serena-agent")).toBeInTheDocument();
      expect(screen.queryByTestId("row-tool:ruff")).not.toBeInTheDocument();
    });

    // Name match.
    fireEvent.change(screen.getByLabelText("Search packages"), { target: { value: "ruff" } });
    await vi.waitFor(() => {
      expect(screen.getByTestId("row-tool:ruff")).toBeInTheDocument();
      expect(screen.queryByTestId("row-tool:serena-agent")).not.toBeInTheDocument();
    });
  });
});
