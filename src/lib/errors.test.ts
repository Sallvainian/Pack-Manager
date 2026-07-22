import { describe, expect, it } from "vitest";

import { describeError, errorCopy } from "./errors";
import type { IpcError } from "./ipc/types";

describe("describeError", () => {
  it("preserves the IpcError payload instead of '[object Object]'", () => {
    // The exact rejection shape Tauri delivers for a failed invoke: the
    // serialized Err arm as a plain object (no custom toString).
    const e: IpcError = { code: "internal", message: "detection has not completed yet" };
    const s = describeError(e);
    expect(s).toBe("internal: detection has not completed yet");
    expect(s).not.toContain("[object Object]");
  });

  it("includes detail/manager/opId/logPath when present", () => {
    const e: IpcError = {
      code: "timeout",
      message: "Homebrew refresh timed out after 600s. Check your network and retry.",
      detail: "phase: brew update",
      managerId: "brew",
      opId: "op-1",
      logPath: "/tmp/brew.log",
    };
    const s = describeError(e);
    expect(s).toContain("timeout: Homebrew refresh timed out");
    expect(s).toContain("detail: phase: brew update");
    expect(s).toContain("manager: brew");
    expect(s).toContain("opId: op-1");
    expect(s).toContain("log: /tmp/brew.log");
  });

  it("keeps an Error's message (and stack when available)", () => {
    const s = describeError(new TypeError("boom"));
    expect(s).toContain("boom");
    expect(s).not.toBe("[object Object]");
  });

  it("JSON-serializes unknown plain values", () => {
    expect(describeError({ weird: 1 })).toBe('{"weird":1}');
    expect(describeError("plain string")).toBe('"plain string"');
  });

  it("falls back to String() for unserializable values", () => {
    expect(describeError(undefined)).toBe("undefined");
    const cyclic: Record<string, unknown> = {};
    cyclic.self = cyclic;
    // JSON.stringify throws on cycles; the fallback must not.
    expect(typeof describeError(cyclic)).toBe("string");
  });
});

describe("plan_stale copy", () => {
  it("tells the user to review and confirm the refreshed plan", () => {
    const copy = errorCopy({ code: "plan_stale", message: "" });
    expect(copy.title).toBe("Plan needs review");
    expect(copy.message).toContain("Review the refreshed plan");
    expect(copy.message).toContain("confirm again");
  });
});
