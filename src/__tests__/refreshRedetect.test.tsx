/**
 * Refresh All re-detects managers mid-session (SPEC F1/F2 — the "brew install
 * mas, press Refresh All" regression). Models the fixed backend contract:
 * `refresh_all` re-runs detection FIRST, emits `detection:updated` with the
 * fresh report, THEN fans out refresh ops over the fresh present set — so a
 * manager installed mid-session transitions absent → present from a single
 * Refresh All click, and its refresh op rides the same fan-out.
 */
import "../test/setup";
import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import { fireEvent, render, within } from "@testing-library/react";

vi.mock("../lib/ipc/bridge", () => import("../test/fakeIpc"));

import { DashboardView } from "../components/dashboard/DashboardView";
import { Sidebar } from "../components/shell/Sidebar";
import { resetLaunchRefresh, subscribeEvents } from "../lib/ipc/events";
import {
  EVENT_DETECTION_UPDATED,
  EVENT_OP_STATUS,
  EVENT_SNAPSHOT_UPDATED,
} from "../lib/ipc/types";
import { resetStores, useManagersStore, useOperationsStore } from "../store";
import * as fakeIpc from "../test/fakeIpc";
import { cleanSnapshot, detectionReport, detectionReportMasPresent } from "../test/fixtures";

let unlisten: (() => void) | undefined;

beforeEach(() => {
  fakeIpc.reset();
  resetStores();
  resetLaunchRefresh();
  useManagersStore.getState().setDetection(detectionReport);
});

afterEach(() => {
  unlisten?.();
  unlisten = undefined;
});

describe("refresh_all_re_detects_managers_mid_session", () => {
  it("transitions mas absent → present on the dashboard and sidebar from one Refresh All", async () => {
    // Fake backend with the FIXED refresh_all contract: detection:updated
    // (mas now present) is emitted before the fan-out op ids resolve, and the
    // fan-out includes the mas refresh op.
    fakeIpc.respond("refresh_all", () => {
      fakeIpc.emit(EVENT_DETECTION_UPDATED, detectionReportMasPresent);
      fakeIpc.emit(EVENT_OP_STATUS, {
        opId: "op-mas-refresh",
        kind: "refresh",
        executor: "mas",
        subject: "mas",
        status: "queued",
        commandLine: "/opt/homebrew/bin/mas list",
        logPath: "/tmp/mas-refresh.log",
      });
      return { opIds: ["op-brew-refresh", "op-mas-refresh"] };
    });

    unlisten = await subscribeEvents();
    const dashboard = render(<DashboardView />);
    const sidebar = render(<Sidebar />);

    // Before: muted absent card; sidebar lists mas under NOT INSTALLED.
    const masCardBefore = within(dashboard.container).getByLabelText("mas");
    expect(within(masCardBefore).getByText("Not installed")).toBeInTheDocument();
    expect(within(sidebar.container).getByText(/Not installed \(1\)/)).toBeInTheDocument();

    // One Refresh All click — the sidebar is the only place it lives.
    fireEvent.click(within(sidebar.container).getByRole("button", { name: "Refresh All" }));
    await vi.waitFor(() => expect(fakeIpc.called("refresh_all")).toBe(true));
    expect(fakeIpc.callsFor("refresh_all")).toHaveLength(1);

    // Card leaves the absent state: no "Not installed", refreshing skeleton
    // until the snapshot from the fan-out lands.
    await vi.waitFor(() => {
      const card = within(dashboard.container).getByLabelText("mas");
      expect(within(card).queryByText("Not installed")).not.toBeInTheDocument();
      expect(within(card).getByText(/Refreshing/)).toBeInTheDocument();
    });

    // Sidebar: mas moved out of NOT INSTALLED into the present list.
    expect(within(sidebar.container).queryByText(/Not installed \(/)).not.toBeInTheDocument();
    expect(within(sidebar.container).getByText("mas")).toBeInTheDocument();

    // The fan-out included mas: its refresh op landed in the operations store.
    const masOp = useOperationsStore.getState().byId["op-mas-refresh"];
    expect(masOp).toBeDefined();
    expect(masOp.kind).toBe("refresh");
    expect(masOp.subject).toBe("mas");

    // The fan-out refresh completes → snapshot arrives → the card fills.
    fakeIpc.emit(EVENT_SNAPSHOT_UPDATED, { managerId: "mas", snapshot: cleanSnapshot("mas") });
    await vi.waitFor(() => {
      const card = within(dashboard.container).getByLabelText("mas");
      expect(within(card).getByText(/Up to date/)).toBeInTheDocument();
    });
  });

  it("renders a manager absent again when a re-detect reports it gone", async () => {
    // Inverse transition: present → absent (manager uninstalled mid-session).
    useManagersStore.getState().setDetection(detectionReportMasPresent);
    unlisten = await subscribeEvents();
    const dashboard = render(<DashboardView />);
    const sidebar = render(<Sidebar />);

    expect(
      within(within(dashboard.container).getByLabelText("mas")).queryByText("Not installed"),
    ).not.toBeInTheDocument();

    fakeIpc.emit(EVENT_DETECTION_UPDATED, detectionReport);

    await vi.waitFor(() => {
      const card = within(dashboard.container).getByLabelText("mas");
      expect(within(card).getByText("Not installed")).toBeInTheDocument();
    });
    expect(within(sidebar.container).getByText(/Not installed \(1\)/)).toBeInTheDocument();
  });
});
