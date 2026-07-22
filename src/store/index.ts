/**
 * store/index.ts — barrel re-exports and cross-store derived selectors.
 *
 * Per-manager phase is derived from operation records + last error rather than
 * stored (SPEC §5.9 "Frontend derives per-manager phase from op records").
 */
import type { ManagerId } from "../lib/ipc/types";
import { useManagersStore } from "./managers";
import { useOperationsStore, type OperationsState, type OpView } from "./operations";
import { outdatedCount, usePackagesStore } from "./packages";
import { useUiStore } from "./ui";

export * from "./managers";
export * from "./packages";
export * from "./operations";
export * from "./ui";

/** Per-manager phase used for status dots and busy affordances (SPEC §4.5). */
export type ManagerPhase = "idle" | "refreshing" | "busy" | "error";

function opTouchesManager(op: OpView, id: ManagerId): boolean {
  return op.executor === id || op.subject === id;
}

/**
 * Derive a manager's phase from operations + its last error:
 * error > refreshing (a live Refresh op) > busy (a live non-refresh op) > idle.
 */
export function deriveManagerPhase(
  id: ManagerId,
  ops: OperationsState,
  hasError: boolean,
): ManagerPhase {
  if (hasError) return "error";
  const live = ops.order
    .map((oid) => ops.byId[oid])
    .filter((o): o is OpView => !!o && (o.status === "queued" || o.status === "running"))
    .filter((o) => opTouchesManager(o, id));
  if (live.some((o) => o.kind === "refresh")) return "refreshing";
  if (live.length > 0) return "busy";
  return "idle";
}

/** Hook: a manager's current phase. */
export function useManagerPhase(id: ManagerId): ManagerPhase {
  const ops = useOperationsStore();
  const hasError = useManagersStore((s) => !!s.errors[id]);
  return deriveManagerPhase(id, ops, hasError);
}

/** Total actionable outdated count across all snapshots (greedy casks excluded). */
export function useTotalOutdated(): number {
  return usePackagesStore((s) =>
    Object.values(s.snapshots).reduce((sum, snap) => sum + outdatedCount(snap), 0),
  );
}

/** Reset every store — used by the test harness between cases. */
export function resetStores(): void {
  useManagersStore.getState().reset();
  usePackagesStore.getState().reset();
  useOperationsStore.getState().reset();
  useUiStore.getState().reset();
}
