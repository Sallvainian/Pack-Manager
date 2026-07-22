/**
 * events.ts — the single event subscription (SPEC §5.9, §5.11).
 *
 * `subscribeEvents()` is called once from App mount; it wires the five backend
 * events to the stores and returns an unlisten. The individual `on*` handlers
 * are exported for direct unit testing (no bridge mock required).
 *
 * A failed *refresh* additionally sets the manager's error and marks its
 * snapshot stale — that is per-manager refresh isolation (SPEC §F2), a data
 * mapping that belongs to the dispatcher. Completion toasts and drawer
 * auto-open (SPEC §4.9/§4.10) are U8 flows: they observe the operations store
 * and the ui-store toast/drawer APIs this unit provides, rather than living
 * here.
 */
import { useManagersStore } from "../../store/managers";
import { useOperationsStore } from "../../store/operations";
import { usePackagesStore } from "../../store/packages";
import { useUiStore } from "../../store/ui";
import { listen, type UnlistenFn } from "./bridge";
import {
  EVENT_DETECTION_UPDATED,
  EVENT_OP_OUTPUT,
  EVENT_OP_STALLED,
  EVENT_OP_STATUS,
  EVENT_SNAPSHOT_UPDATED,
  type DetectionReport,
  type OpOutputEvent,
  type OpStalledEvent,
  type OpStatusEvent,
  type SnapshotUpdatedEvent,
} from "./types";

const FAILURE: ReadonlySet<string> = new Set(["failed", "cancelled", "timedOut", "interrupted"]);

export function onDetection(report: DetectionReport): void {
  useManagersStore.getState().setDetection(report);
}

export function onSnapshot(evt: SnapshotUpdatedEvent): void {
  usePackagesStore.getState().setSnapshot(evt.managerId, evt.snapshot);
  // A fresh snapshot clears any prior refresh error for that manager.
  useManagersStore.getState().setManagerError(evt.managerId, null);
}

export function onStatus(evt: OpStatusEvent): void {
  useOperationsStore.getState().applyStatus(evt);

  // Per-manager refresh isolation: a failed refresh surfaces on the manager and
  // its prior snapshot is retained but marked stale (SPEC §F2).
  if (evt.kind === "refresh" && FAILURE.has(evt.status) && evt.error) {
    useManagersStore.getState().setManagerError(evt.subject, evt.error);
    usePackagesStore.getState().markStale(evt.subject);
  }
}

export function onOutput(evt: OpOutputEvent): void {
  useOperationsStore.getState().appendOutput(evt.opId, evt.batch);
}

export function onStalled(evt: OpStalledEvent): void {
  useUiStore
    .getState()
    .openDialog({ kind: "stalled", opId: evt.opId, silentForSecs: evt.silentForSecs });
}

/** Subscribe to all backend events. Returns a single unlisten for teardown. */
export async function subscribeEvents(): Promise<UnlistenFn> {
  const unlisteners = await Promise.all([
    listen<DetectionReport>(EVENT_DETECTION_UPDATED, (e) => onDetection(e.payload)),
    listen<SnapshotUpdatedEvent>(EVENT_SNAPSHOT_UPDATED, (e) => onSnapshot(e.payload)),
    listen<OpStatusEvent>(EVENT_OP_STATUS, (e) => onStatus(e.payload)),
    listen<OpOutputEvent>(EVENT_OP_OUTPUT, (e) => onOutput(e.payload)),
    listen<OpStalledEvent>(EVENT_OP_STALLED, (e) => onStalled(e.payload)),
  ]);
  return () => unlisteners.forEach((u) => u());
}
