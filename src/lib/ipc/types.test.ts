/**
 * Contract drift guard, TS half (SPEC §7.4). The Rust half
 * (`ipc_contract_matches_committed_fixtures`) asserts serialization
 * byte-equality against the same committed fixtures; this test proves the TS
 * type guards accept them. Editing either side alone breaks its half.
 */
import { describe, expect, it } from "vitest";

import {
  isAppState,
  isAppUpdateStatus,
  isDetectionReport,
  isIpcError,
  isManagerSnapshot,
  isOpOutputEvent,
  isOpRef,
  isOpStalledEvent,
  isOpStatusEvent,
  isOperationDetail,
  isOperationRecord,
  isPlanRequest,
  isSettings,
  isSnapshotUpdatedEvent,
  isUpgradePlan,
} from "./types";

const GUARDS: Record<string, (v: unknown) => boolean> = {
  "app_state.json": isAppState,
  "event_app_update_status.json": isAppUpdateStatus,
  "detection_report.json": isDetectionReport,
  "event_op_output.json": isOpOutputEvent,
  "event_op_stalled.json": isOpStalledEvent,
  "event_op_status.json": isOpStatusEvent,
  "event_snapshot_updated.json": isSnapshotUpdatedEvent,
  "ipc_error.json": isIpcError,
  "manager_snapshot.json": isManagerSnapshot,
  "op_ref.json": isOpRef,
  "operation_detail.json": isOperationDetail,
  "operation_record.json": isOperationRecord,
  "plan_request.json": isPlanRequest,
  "settings.json": isSettings,
  "upgrade_plan.json": isUpgradePlan,
};

// Eagerly import every committed fixture; keys are the resolved paths.
const FIXTURES = import.meta.glob("../../../dev/fixtures/ipc/*.json", {
  eager: true,
  import: "default",
});

function baseName(path: string): string {
  return path.slice(path.lastIndexOf("/") + 1);
}

describe("ipc_types_accept_contract_fixtures", () => {
  it("covers exactly the committed fixture set", () => {
    const found = Object.keys(FIXTURES).map(baseName).sort();
    expect(found).toEqual(Object.keys(GUARDS).sort());
  });

  for (const [file, guard] of Object.entries(GUARDS)) {
    it(`${file} passes ${guard.name}`, () => {
      const entry = Object.entries(FIXTURES).find(([path]) => baseName(path) === file);
      expect(entry, `fixture ${file} is missing — regenerate with PM_UPDATE_CONTRACT=1`).toBeDefined();
      const payload: unknown = entry![1];
      expect(guard(payload), `${guard.name} rejected dev/fixtures/ipc/${file}`).toBe(true);
    });
  }
});
