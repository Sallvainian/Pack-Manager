import "../../test/setup";
import { Profiler } from "react";
import { beforeEach, describe, expect, it, vi } from "vitest";
import { act, fireEvent, render, screen } from "@testing-library/react";

vi.mock("../../lib/ipc/bridge", () => import("../../test/fakeIpc"));

import { ActivityDrawer } from "./ActivityDrawer";
import { LiveLogView } from "./LiveLogView";
import { OperationRow } from "./OperationRow";
import type { LogLine, OpStatusEvent } from "../../lib/ipc/types";
import { LOG_CAP, useOperationsStore } from "../../store/operations";
import { resetStores, useManagersStore, useUiStore } from "../../store";
import * as fakeIpc from "../../test/fakeIpc";
import { detectionReport } from "../../test/fixtures";

beforeEach(() => {
  fakeIpc.reset();
  resetStores();
});

function evt(over: Partial<OpStatusEvent> & Pick<OpStatusEvent, "opId">): OpStatusEvent {
  return {
    kind: "upgrade",
    executor: "npm",
    subject: "npm",
    status: "running",
    commandLine: "/Users/sallvain/.local/share/mise/shims/npm install -g typescript@latest",
    logPath: "/tmp/op.log",
    ...over,
  };
}

function line(text: string, stream: LogLine["stream"] = "out", ts = 1): LogLine {
  return { stream, line: text, tsMs: ts };
}

describe("log_view_appends_batches_pins_unpins_with_jump_chip", () => {
  it("renders appended batches, and unpin surfaces the jump chip which re-pins", () => {
    useOperationsStore.getState().applyStatus(evt({ opId: "op1" }));
    render(<LiveLogView opId="op1" />);

    // Batch append.
    act(() =>
      useOperationsStore.getState().appendOutput("op1", [
        line("added 1 package"),
        line("resolved deps"),
        line("npm warn deprecated foo", "err"),
      ]),
    );
    expect(screen.getByText("added 1 package")).toBeInTheDocument();
    expect(screen.getByText("npm warn deprecated foo")).toBeInTheDocument();

    // Pinned to tail: no jump chip yet.
    expect(screen.queryByRole("button", { name: /Jump to latest/ })).toBeNull();

    // Scroll away from the bottom -> unpinned -> jump chip appears.
    const logEl = screen.getByRole("log");
    Object.defineProperty(logEl, "scrollHeight", { configurable: true, value: 1000 });
    Object.defineProperty(logEl, "clientHeight", { configurable: true, value: 200 });
    logEl.scrollTop = 0;
    fireEvent.scroll(logEl);

    const jump = screen.getByRole("button", { name: /Jump to latest/ });
    expect(jump).toBeInTheDocument();

    // Clicking it re-pins -> chip removed.
    fireEvent.click(jump);
    expect(screen.queryByRole("button", { name: /Jump to latest/ })).toBeNull();
  });

  it("collapses a carriage-return progress repaint to its final segment", () => {
    useOperationsStore.getState().applyStatus(evt({ opId: "op1" }));
    render(<LiveLogView opId="op1" />);
    act(() => useOperationsStore.getState().appendOutput("op1", [line("50%\r100%")]));
    expect(screen.getByText("100%")).toBeInTheDocument();
    expect(screen.queryByText("50%\r100%")).toBeNull();
  });

  it("shows the ring-buffer cap banner when older lines overflowed to the file", () => {
    useOperationsStore.getState().applyStatus(evt({ opId: "op1" }));
    // Inject an overflow count directly (the store enforces the real 5000 cap).
    act(() =>
      useOperationsStore.setState((s) => ({
        logs: { ...s.logs, op1: { lines: [line("tail line")], overflow: 7 } },
      })),
    );
    render(<LiveLogView opId="op1" />);
    expect(screen.getByText(new RegExp(`7 earlier lines.*${LOG_CAP}`))).toBeInTheDocument();
  });
});

describe("log_view_single_render_per_batch", () => {
  it("commits exactly once per appended batch", () => {
    useOperationsStore.getState().applyStatus(evt({ opId: "op1" }));
    let commits = 0;
    render(
      <Profiler id="ll" onRender={() => (commits += 1)}>
        <LiveLogView opId="op1" />
      </Profiler>,
    );
    const base = commits;

    act(() => useOperationsStore.getState().appendOutput("op1", [line("a"), line("b")]));
    expect(commits).toBe(base + 1);

    act(() => useOperationsStore.getState().appendOutput("op1", [line("c")]));
    expect(commits).toBe(base + 2);
  });
});

describe("activity_drawer_summary_and_expanded_panes", () => {
  it("summarises running ops in the bar and shows the list + log when expanded", () => {
    useManagersStore.getState().setDetection(detectionReport);
    useOperationsStore.getState().applyStatus(evt({ opId: "opd", status: "running" }));
    useUiStore.getState().setFocusedOp("opd");
    useUiStore.getState().setDrawerOpen(true);

    render(<ActivityDrawer />);

    expect(screen.getByText("1 running")).toBeInTheDocument();
    // OperationList row (title) + LiveLogView command header both render.
    expect(screen.getByRole("list", { name: "Operations" })).toBeInTheDocument();
    expect(screen.getByText("Upgrade · npm")).toBeInTheDocument();
    expect(
      screen.getByText("/Users/sallvain/.local/share/mise/shims/npm install -g typescript@latest"),
    ).toBeInTheDocument();
  });
});

describe("cancel_flips_pill_on_event", () => {
  function Row({ opId }: { opId: string }) {
    const op = useOperationsStore((s) => s.byId[opId]);
    if (!op) return null;
    return (
      <OperationRow
        op={op}
        focused={false}
        resolveName={(id) => id}
        onSelect={() => {}}
        onCancel={() => {}}
      />
    );
  }

  it("re-renders the status pill Running -> Cancelled when the op:status event lands", () => {
    useOperationsStore.getState().applyStatus(evt({ opId: "opc", status: "running" }));
    render(<Row opId="opc" />);
    expect(screen.getByText("Running")).toBeInTheDocument();

    act(() =>
      useOperationsStore.getState().applyStatus(
        evt({ opId: "opc", status: "cancelled", finishedAt: "2026-07-22T14:04:00Z" }),
      ),
    );
    expect(screen.getByText("Cancelled")).toBeInTheDocument();
    expect(screen.queryByText("Running")).toBeNull();
  });
});
