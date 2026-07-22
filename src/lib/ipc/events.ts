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
import { useAppUpdateStore } from "../../store/appUpdate";
import { useManagersStore } from "../../store/managers";
import { useOperationsStore } from "../../store/operations";
import { usePackagesStore } from "../../store/packages";
import { useUiStore } from "../../store/ui";
import { describeError } from "../errors";
import { listen, type UnlistenFn } from "./bridge";
import { logFrontendEvent, refreshAll } from "./client";
import {
  EVENT_APP_UPDATE_STATUS,
  EVENT_DETECTION_UPDATED,
  EVENT_OP_OUTPUT,
  EVENT_OP_STALLED,
  EVENT_OP_STATUS,
  EVENT_SNAPSHOT_UPDATED,
  type AppUpdateStatus,
  type DetectionReport,
  type OpOutputEvent,
  type OpStalledEvent,
  type OpStatusEvent,
  type SnapshotUpdatedEvent,
} from "./types";

const FAILURE: ReadonlySet<string> = new Set(["failed", "cancelled", "timedOut", "interrupted"]);

// ---------------------------------------------------------------------------
// Launch refresh gating (SPEC §5.12 step 3)
//
// Backend detection runs asynchronously after the window shows. `refresh_all`
// re-runs detection itself before fanning out (SPEC F1/F2), so firing the
// launch refresh inline from bootstrap would race the startup detection with
// a second, concurrent detection pass. Instead, the launch refresh fires
// immediately only when a real detection is already hydrated, and otherwise
// arms a one-shot that the first real `detection:updated` consumes. The
// `detection:updated` that refresh_all itself emits arrives with the one-shot
// already consumed — no re-fire loop.
// ---------------------------------------------------------------------------

let launchRefreshArmed = false;

/** A real detection report — the pre-detection placeholder has no managers. */
function isRealDetection(report: DetectionReport | null): report is DetectionReport {
  return !!report && report.managers.length > 0;
}

function fireLaunchRefresh(): void {
  void refreshAll().catch(
    (e) => void logFrontendEvent("error", `launch refresh failed: ${describeError(e)}`),
  );
}

/**
 * Request the launch refresh: runs now when detection is ready, otherwise
 * defers until the first real `detection:updated` arrives (one-shot).
 */
export function scheduleLaunchRefresh(): void {
  if (isRealDetection(useManagersStore.getState().detection)) fireLaunchRefresh();
  else launchRefreshArmed = true;
}

/** Test hook: clear the launch-refresh one-shot between cases. */
export function resetLaunchRefresh(): void {
  launchRefreshArmed = false;
}

export function onDetection(report: DetectionReport): void {
  useManagersStore.getState().setDetection(report);
  if (launchRefreshArmed && isRealDetection(report)) {
    launchRefreshArmed = false;
    fireLaunchRefresh();
  }
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

/**
 * In-app update state (DECISIONS D25). Only *manual* checks speak up: an
 * automatic check that finds nothing, or fails because the laptop is offline,
 * must stay invisible. The StatusBar indicator reacts to every transition.
 */
export function onAppUpdate(status: AppUpdateStatus): void {
  const previous = useAppUpdateStore.getState().status;
  useAppUpdateStore.getState().setStatus(status);
  if (status.lastTrigger !== "manual") return;
  // Guard against re-toasting when a repeated payload arrives for the same
  // terminal state (e.g. a second check that changes nothing).
  if (previous?.state.kind === status.state.kind && previous.lastTrigger === "manual") return;

  const ui = useUiStore.getState();
  if (status.state.kind === "upToDate") {
    ui.pushToast({
      kind: "info",
      message: `Pack-Manager ${status.currentVersion} is up to date`,
    });
  } else if (status.state.kind === "error") {
    ui.pushToast({
      kind: "error",
      message: `Update check failed: ${status.state.message}`,
      persistent: true,
    });
  }
}

/** Subscribe to all backend events. Returns a single unlisten for teardown. */
export async function subscribeEvents(): Promise<UnlistenFn> {
  // allSettled (not all): if any listen() rejects, the ones that DID register
  // must still be unlistened — Promise.all would trap their unlisten fns
  // inside the rejected aggregate and leak the handlers for the process
  // lifetime.
  const results = await Promise.allSettled([
    listen<DetectionReport>(EVENT_DETECTION_UPDATED, (e) => onDetection(e.payload)),
    listen<SnapshotUpdatedEvent>(EVENT_SNAPSHOT_UPDATED, (e) => onSnapshot(e.payload)),
    listen<OpStatusEvent>(EVENT_OP_STATUS, (e) => onStatus(e.payload)),
    listen<OpOutputEvent>(EVENT_OP_OUTPUT, (e) => onOutput(e.payload)),
    listen<OpStalledEvent>(EVENT_OP_STALLED, (e) => onStalled(e.payload)),
    listen<AppUpdateStatus>(EVENT_APP_UPDATE_STATUS, (e) => onAppUpdate(e.payload)),
  ]);
  const unlisteners = results
    .filter((r): r is PromiseFulfilledResult<UnlistenFn> => r.status === "fulfilled")
    .map((r) => r.value);
  const rejected = results.find((r): r is PromiseRejectedResult => r.status === "rejected");
  if (rejected) {
    unlisteners.forEach((u) => u());
    throw rejected.reason;
  }
  return () => unlisteners.forEach((u) => u());
}
