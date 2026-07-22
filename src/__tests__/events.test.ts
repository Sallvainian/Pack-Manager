import { beforeEach, describe, expect, it, vi } from "vitest";

vi.mock("../lib/ipc/bridge", () => import("../test/fakeIpc"));

import { onOutput, onSnapshot, onStalled, onStatus, subscribeEvents } from "../lib/ipc/events";
import type { IpcError, OpStatusEvent } from "../lib/ipc/types";
import {
  EVENT_DETECTION_UPDATED,
  EVENT_OP_OUTPUT,
  EVENT_OP_STALLED,
  EVENT_OP_STATUS,
  EVENT_SNAPSHOT_UPDATED,
} from "../lib/ipc/types";
import {
  resetStores,
  useManagersStore,
  useOperationsStore,
  usePackagesStore,
  useUiStore,
} from "../store";
import * as fakeIpc from "../test/fakeIpc";
import { npmSnapshot } from "../test/fixtures";

function status(over: Partial<OpStatusEvent> & Pick<OpStatusEvent, "opId">): OpStatusEvent {
  return {
    kind: "upgrade",
    executor: "npm",
    subject: "npm",
    status: "running",
    commandLine: "npm install -g typescript@latest",
    logPath: "/tmp/op.log",
    ...over,
  };
}

const brewError: IpcError = {
  code: "timeout",
  message: "Homebrew refresh timed out after 600s. Check your network and retry.",
  detail: "phase: brew update",
  managerId: "brew",
  logPath: "/tmp/brew.log",
};

beforeEach(() => resetStores());

describe("onSnapshot", () => {
  it("stores the snapshot and clears any prior manager error", () => {
    useManagersStore.getState().setManagerError("npm", brewError);
    onSnapshot({ managerId: "npm", snapshot: npmSnapshot });
    expect(usePackagesStore.getState().snapshots.npm).toEqual(npmSnapshot);
    expect(useManagersStore.getState().errors.npm).toBeUndefined();
  });
});

describe("onStatus — refresh isolation (SPEC §F2)", () => {
  it("a failed refresh sets the manager error and marks its snapshot stale", () => {
    usePackagesStore.getState().setSnapshot("brew", { ...npmSnapshot, managerId: "brew" });
    onStatus(
      status({
        opId: "r1",
        kind: "refresh",
        executor: "brew",
        subject: "brew",
        status: "failed",
        error: brewError,
      }),
    );
    expect(useManagersStore.getState().errors.brew).toEqual(brewError);
    expect(usePackagesStore.getState().stale.brew).toBe(true);
    // The prior snapshot is retained, not blanked.
    expect(usePackagesStore.getState().snapshots.brew).toBeDefined();
  });
});

describe("onStatus — op record upsert", () => {
  it("upserts a record for a mutating op without touching manager errors", () => {
    onStatus(status({ opId: "u1", status: "running" }));
    expect(useOperationsStore.getState().byId["u1"].status).toBe("running");
    expect(useManagersStore.getState().errors.npm).toBeUndefined();
  });
});

describe("onOutput", () => {
  it("appends batches to the op ring buffer", () => {
    onOutput({ opId: "u5", batch: [{ stream: "out", line: "added 1 package", tsMs: 1 }] });
    expect(useOperationsStore.getState().logs["u5"].lines).toHaveLength(1);
  });
});

describe("onStalled", () => {
  it("opens the stalled dialog with the silent duration", () => {
    onStalled({ opId: "u6", silentForSecs: 120 });
    expect(useUiStore.getState().dialog).toEqual({ kind: "stalled", opId: "u6", silentForSecs: 120 });
  });
});

describe("subscribeEvents — no listener leak on partial failure", () => {
  const ALL_EVENTS = [
    EVENT_DETECTION_UPDATED,
    EVENT_SNAPSHOT_UPDATED,
    EVENT_OP_STATUS,
    EVENT_OP_OUTPUT,
    EVENT_OP_STALLED,
  ];

  beforeEach(() => fakeIpc.reset());

  it("registers all five listeners and tears them all down", async () => {
    const unlisten = await subscribeEvents();
    for (const evt of ALL_EVENTS) expect(fakeIpc.listenerCount(evt)).toBe(1);
    unlisten();
    for (const evt of ALL_EVENTS) expect(fakeIpc.listenerCount(evt)).toBe(0);
  });

  it("unlistens the already-registered listeners when one listen() rejects", async () => {
    fakeIpc.failListen(EVENT_OP_STATUS, new Error("listen refused"));

    await expect(subscribeEvents()).rejects.toThrow("listen refused");

    // The siblings that DID register must not leak for the process lifetime.
    for (const evt of ALL_EVENTS) expect(fakeIpc.listenerCount(evt)).toBe(0);
  });
});
