import "../../test/setup";
import { beforeEach, describe, expect, it, vi } from "vitest";
import { fireEvent, render, screen, within } from "@testing-library/react";

vi.mock("../../lib/ipc/bridge", () => import("../../test/fakeIpc"));

import { HistoryView } from "./HistoryView";
import { resetStores, useManagersStore, useOperationsStore, useUiStore } from "../../store";
import * as fakeIpc from "../../test/fakeIpc";
import { detectionReport, interruptedRecord, upgradeRecord } from "../../test/fixtures";

beforeEach(() => {
  fakeIpc.reset();
  resetStores();
  useManagersStore.getState().setDetection(detectionReport);
});

describe("interrupted_ops_render_in_history", () => {
  it("renders a start-without-finish record as Interrupted in the table", () => {
    useOperationsStore.getState().setRecords([interruptedRecord]);
    render(<HistoryView />);

    const table = screen.getByRole("table", { name: "Operation history" });
    // The status chip inside the table (not the filter <option>) reads Interrupted.
    expect(within(table).getByText("Interrupted")).toBeInTheDocument();
    expect(within(table).getByText("Homebrew")).toBeInTheDocument();
    expect(within(table).getByText("/opt/homebrew/bin/brew update")).toBeInTheDocument();
  });
});

describe("history filters and export", () => {
  it("filters by status and exports diagnostics", async () => {
    useOperationsStore.getState().setRecords([interruptedRecord, upgradeRecord]);
    fakeIpc.respond("export_diagnostics", () => ({ zipPath: "/Users/sallvain/Desktop/diag.zip" }));
    render(<HistoryView />);

    const table = screen.getByRole("table", { name: "Operation history" });
    // Both rows present initially.
    expect(within(table).getByText("Interrupted")).toBeInTheDocument();
    expect(within(table).getByText("Running")).toBeInTheDocument();

    // Narrow to interrupted only -> the running row disappears.
    fireEvent.change(screen.getByLabelText("Filter by status"), { target: { value: "interrupted" } });
    expect(within(screen.getByRole("table")).queryByText("Running")).toBeNull();
    expect(within(screen.getByRole("table")).getByText("Interrupted")).toBeInTheDocument();

    fireEvent.click(screen.getByRole("button", { name: "Export diagnostics" }));
    await vi.waitFor(() => expect(fakeIpc.called("export_diagnostics")).toBe(true));
    await vi.waitFor(() =>
      expect(useUiStore.getState().toasts.some((t) => /diag\.zip/.test(t.message))).toBe(true),
    );
  });

  it("expands a row to load the transcript tail", async () => {
    useOperationsStore.getState().setRecords([upgradeRecord]);
    fakeIpc.respond("get_operation", () => ({
      record: upgradeRecord,
      lines: [
        { stream: "out", line: "added 1 package in 4s", tsMs: 1 },
        { stream: "err", line: "npm warn deprecated foo", tsMs: 2 },
      ],
      truncated: false,
    }));
    render(<HistoryView />);

    const table = screen.getByRole("table", { name: "Operation history" });
    fireEvent.click(within(table).getByText(/npm install -g typescript@latest/));
    await vi.waitFor(() => expect(fakeIpc.called("get_operation")).toBe(true));
    await vi.waitFor(() => expect(screen.getByText(/added 1 package in 4s/)).toBeInTheDocument());
  });
});
