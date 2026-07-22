import "../test/setup";
import { beforeEach, describe, expect, it, vi } from "vitest";
import { act, render } from "@testing-library/react";

vi.mock("../lib/ipc/bridge", () => import("../test/fakeIpc"));

import App from "../App";
import { resetLaunchRefresh } from "../lib/ipc/events";
import type { AppState, IpcError } from "../lib/ipc/types";
import { EVENT_DETECTION_UPDATED } from "../lib/ipc/types";
import { resetStores } from "../store";
import * as fakeIpc from "../test/fakeIpc";
import { defaultSettings, detectionReport } from "../test/fixtures";

/** `get_state` before backend detection lands: the placeholder report. */
const placeholderState: AppState = {
  detection: { managers: [], env: detectionReport.env },
  snapshots: [],
  operations: [],
  settings: { ...defaultSettings, autoRefreshOnLaunch: true },
};

beforeEach(() => {
  fakeIpc.reset();
  resetStores();
  resetLaunchRefresh();
});

async function flush(): Promise<void> {
  await act(async () => {});
}

describe("launch_refresh_waits_for_detection", () => {
  it("defers refresh_all until the first real detection:updated instead of racing it", async () => {
    fakeIpc.respond("get_state", () => placeholderState);
    fakeIpc.respond("refresh_all", () => ({ opIds: [] }));
    fakeIpc.respond("log_frontend_event", () => undefined);

    render(<App />);
    await vi.waitFor(() => expect(fakeIpc.called("get_state")).toBe(true));
    await flush();

    // Detection has not completed: the launch refresh must NOT have fired
    // (firing here is the seeded bug — detection_not_ready killed bootstrap).
    expect(fakeIpc.called("refresh_all")).toBe(false);

    // Backend detection lands → the armed one-shot fires the launch refresh.
    act(() => fakeIpc.emit(EVENT_DETECTION_UPDATED, detectionReport));
    await vi.waitFor(() => expect(fakeIpc.called("refresh_all")).toBe(true));

    // One-shot: a later re-detect does not fire a second launch refresh.
    act(() => fakeIpc.emit(EVENT_DETECTION_UPDATED, detectionReport));
    await flush();
    expect(fakeIpc.callsFor("refresh_all")).toHaveLength(1);
  });
});

describe("bootstrap_failure_is_logged_and_recoverable", () => {
  it("logs the real IpcError payload (not '[object Object]') and still runs the launch refresh", async () => {
    const rejection: IpcError = { code: "internal", message: "detection has not completed yet" };
    // Tauri delivers a failed invoke as the serialized Err arm: a plain object.
    fakeIpc.respond("get_state", () => {
      throw rejection;
    });
    fakeIpc.respond("refresh_all", () => ({ opIds: [] }));
    fakeIpc.respond("log_frontend_event", () => undefined);

    render(<App />);
    await vi.waitFor(() => expect(fakeIpc.called("log_frontend_event")).toBe(true));

    const logged = fakeIpc.callsFor("log_frontend_event").map(
      (c) => (c.args as { args: { level: string; message: string } }).args.message,
    );
    const bootstrapLine = logged.find((m) => m.startsWith("bootstrap failed:"));
    expect(bootstrapLine).toBeDefined();
    expect(bootstrapLine).toContain("internal: detection has not completed yet");
    expect(bootstrapLine).not.toContain("[object Object]");

    // A failed hydration must not strand the dashboard: once detection lands,
    // the launch refresh still fires (autoRefreshOnLaunch defaults to true).
    await flush();
    expect(fakeIpc.called("refresh_all")).toBe(false);
    act(() => fakeIpc.emit(EVENT_DETECTION_UPDATED, detectionReport));
    await vi.waitFor(() => expect(fakeIpc.called("refresh_all")).toBe(true));
  });
});
