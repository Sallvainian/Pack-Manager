import { beforeEach, describe, expect, it } from "vitest";
import type { LogLine, OpStatusEvent } from "../lib/ipc/types";
import {
  deriveManagerPhase,
  LOG_CAP,
  outdatedCount,
  resetStores,
  useManagersStore,
  useOperationsStore,
  usePackagesStore,
} from "../store";
import { brewSnapshot, miseSnapshot, npmSnapshot } from "../test/fixtures";

function line(i: number): LogLine {
  return { stream: "out", line: `line ${i}`, tsMs: i };
}

function statusEvent(over: Partial<OpStatusEvent> & Pick<OpStatusEvent, "opId">): OpStatusEvent {
  return {
    kind: "refresh",
    executor: "brew",
    subject: "brew",
    status: "running",
    commandLine: "/opt/homebrew/bin/brew update",
    logPath: "/tmp/op.log",
    ...over,
  };
}

beforeEach(() => resetStores());

describe("operations ring buffer", () => {
  it("caps at LOG_CAP lines and counts the overflow", () => {
    const ops = useOperationsStore.getState();
    ops.appendOutput("op1", Array.from({ length: 3000 }, (_, i) => line(i)));
    ops.appendOutput("op1", Array.from({ length: 3000 }, (_, i) => line(3000 + i)));
    const log = useOperationsStore.getState().logs["op1"];
    expect(log.lines).toHaveLength(LOG_CAP);
    expect(log.overflow).toBe(6000 - LOG_CAP);
    // Oldest lines were dropped; the newest survive.
    expect(log.lines[log.lines.length - 1].line).toBe("line 5999");
    expect(log.lines[0].line).toBe(`line ${6000 - LOG_CAP}`);
  });
});

describe("operations applyStatus", () => {
  it("preserves queuedAt/packageIds across later status events", () => {
    const ops = useOperationsStore.getState();
    ops.setRecords([
      {
        opId: "op9",
        kind: "upgrade",
        executor: "npm",
        subject: "npm",
        status: "queued",
        commandLine: "npm install -g typescript@latest",
        packageIds: ["globalPackage:typescript"],
        queuedAt: "2026-07-22T14:00:00Z",
        startedAt: null,
        finishedAt: null,
        exitCode: null,
        error: null,
        logPath: "/tmp/op9.log",
      },
    ]);
    useOperationsStore
      .getState()
      .applyStatus(statusEvent({ opId: "op9", kind: "upgrade", executor: "npm", subject: "npm", status: "running", startedAt: "2026-07-22T14:00:05Z" }));
    const rec = useOperationsStore.getState().byId["op9"];
    expect(rec.status).toBe("running");
    expect(rec.queuedAt).toBe("2026-07-22T14:00:00Z");
    expect(rec.packageIds).toEqual(["globalPackage:typescript"]);
    expect(rec.startedAt).toBe("2026-07-22T14:00:05Z");
  });
});

describe("deriveManagerPhase", () => {
  it("is refreshing while a refresh op for the manager is live", () => {
    useOperationsStore.getState().applyStatus(statusEvent({ opId: "r1", subject: "brew", executor: "brew", status: "running" }));
    const phase = deriveManagerPhase("brew", useOperationsStore.getState(), false);
    expect(phase).toBe("refreshing");
  });

  it("is busy while a non-refresh op touches the manager", () => {
    useOperationsStore.getState().applyStatus(
      statusEvent({ opId: "u1", kind: "upgrade", subject: "npm", executor: "npm", status: "running" }),
    );
    expect(deriveManagerPhase("npm", useOperationsStore.getState(), false)).toBe("busy");
  });

  it("is error when the manager has a last error, regardless of ops", () => {
    expect(deriveManagerPhase("brew", useOperationsStore.getState(), true)).toBe("error");
  });

  it("is idle with no live ops and no error", () => {
    expect(deriveManagerPhase("uv", useOperationsStore.getState(), false)).toBe("idle");
  });
});

describe("packages selection", () => {
  it("toggles a package and sets the anchor", () => {
    const pkgs = usePackagesStore.getState();
    pkgs.toggleSelect("npm", "globalPackage:typescript");
    expect(usePackagesStore.getState().selection.npm?.has("globalPackage:typescript")).toBe(true);
    expect(usePackagesStore.getState().anchor.npm).toBe("globalPackage:typescript");
    usePackagesStore.getState().toggleSelect("npm", "globalPackage:typescript");
    expect(usePackagesStore.getState().selection.npm?.has("globalPackage:typescript")).toBe(false);
  });

  it("selects an inclusive range from the anchor", () => {
    const ordered = ["a", "b", "c", "d"];
    const pkgs = usePackagesStore.getState();
    pkgs.setAnchor("mise", "b");
    pkgs.selectRange("mise", "d", ordered);
    const sel = usePackagesStore.getState().selection.mise!;
    expect([...sel].sort()).toEqual(["b", "c", "d"]);
  });

  it("clears selection and anchor", () => {
    usePackagesStore.getState().toggleSelect("mise", "tool:deno");
    usePackagesStore.getState().clearSelection("mise");
    expect(usePackagesStore.getState().selection.mise?.size).toBe(0);
    expect(usePackagesStore.getState().anchor.mise).toBeNull();
  });
});

describe("packages setSnapshot", () => {
  it("clears stale and defaults outdatedOnly ON when anything is outdated", () => {
    const pkgs = usePackagesStore.getState();
    pkgs.markStale("mise");
    pkgs.setSnapshot("mise", miseSnapshot);
    expect(usePackagesStore.getState().stale.mise).toBeUndefined();
    expect(usePackagesStore.getState().outdatedOnly.mise).toBe(true);
  });

  it("drops selected ids that no longer exist after a refresh", () => {
    const pkgs = usePackagesStore.getState();
    pkgs.setSnapshot("npm", npmSnapshot);
    pkgs.toggleSelect("npm", "globalPackage:typescript");
    pkgs.toggleSelect("npm", "globalPackage:ghost");
    // Re-snapshot without the ghost id.
    usePackagesStore.getState().setSnapshot("npm", npmSnapshot);
    const sel = usePackagesStore.getState().selection.npm!;
    expect(sel.has("globalPackage:typescript")).toBe(true);
    expect(sel.has("globalPackage:ghost")).toBe(false);
  });
});

describe("outdated counting", () => {
  it("counts the manager's outdated verdict but excludes greedy casks", () => {
    // brewSnapshot: dolt (outdated) + deno (outdated, pinned) + jq (clean)
    //               + 3 greedy casks (outdated). Greedy excluded → 2.
    expect(outdatedCount(brewSnapshot)).toBe(2);
    expect(outdatedCount(miseSnapshot)).toBe(6);
    expect(outdatedCount(undefined)).toBe(0);
  });
});

describe("managers store errors", () => {
  it("sets and clears a per-manager error", () => {
    const err = { code: "timeout" as const, message: "timed out" };
    useManagersStore.getState().setManagerError("brew", err);
    expect(useManagersStore.getState().errors.brew).toEqual(err);
    useManagersStore.getState().setManagerError("brew", null);
    expect(useManagersStore.getState().errors.brew).toBeUndefined();
  });
});
