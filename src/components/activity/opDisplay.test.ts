import { describe, expect, it } from "vitest";

import type { OpView } from "../../store/operations";
import { interruptedRecord, upgradeRecord } from "../../test/fixtures";
import { durationLabel, TERMINAL_STATUSES } from "./opDisplay";

const START = Date.parse("2026-07-22T13:00:00Z");

function op(over: Partial<OpView>): OpView {
  return { ...(upgradeRecord as OpView), ...over };
}

describe("durationLabel", () => {
  it("returns '' when the op has not started", () => {
    expect(durationLabel(op({ startedAt: null, finishedAt: null }), START)).toBe("");
  });

  it("ticks against `now` while running", () => {
    const running = op({ status: "running", startedAt: "2026-07-22T13:00:00Z", finishedAt: null });
    expect(durationLabel(running, START + 5_000)).toBe("5s");
    // A later `now` grows the label — the live ticker.
    expect(durationLabel(running, START + 63_000)).toBe("1m 3s");
  });

  it("freezes finished ops at start→finish regardless of `now`", () => {
    const finished = op({
      status: "succeeded",
      startedAt: "2026-07-22T13:00:00Z",
      finishedAt: "2026-07-22T13:00:42Z",
    });
    expect(durationLabel(finished, START + 999_999)).toBe("42s");
    expect(durationLabel(finished, START + 5_000_000)).toBe("42s");
  });

  it("returns '' (→ em-dash) for an interrupted op with no finishedAt — never a ticker", () => {
    // The journal-reconstructed shape: startedAt set, finishedAt null.
    expect(interruptedRecord.status).toBe("interrupted");
    expect(interruptedRecord.finishedAt).toBeNull();
    const a = durationLabel(interruptedRecord as OpView, START + 63_000);
    const b = durationLabel(interruptedRecord as OpView, START + 999_000);
    expect(a).toBe("");
    expect(b).toBe("");
  });

  it("returns '' for EVERY terminal status lacking a finishedAt", () => {
    for (const status of TERMINAL_STATUSES) {
      const terminal = op({ status, startedAt: "2026-07-22T13:00:00Z", finishedAt: null });
      expect(durationLabel(terminal, START + 63_000), status).toBe("");
    }
  });
});
