/**
 * useOperationEffects — the toast-completion and drawer-auto-open flows
 * (SPEC §4.9/§4.10). Mounted once (from the always-present ActivityDrawer) it
 * subscribes to the operations store and reacts to genuine *status transitions*:
 *
 * - An upgrade/self-update/health-fix entering `running` opens the drawer and
 *   focuses that op — only when `settings.autoOpenDrawer` is on, and NEVER for a
 *   refresh (SPEC §4.9).
 * - A terminal transition raises a toast (SPEC §4.10): success auto-dismisses;
 *   failure/timeout persists and carries "View log" (the toast's `opId` lets
 *   ToastHost focus the op); cancellation is a brief info toast.
 *
 * First sightings (records hydrated from `get_state`, or an op's initial `queued`
 * event) never fire anything — only a change with a known prior status does — so
 * rehydration is silent and the drawer never pops on launch.
 */
import { useEffect } from "react";
import type { OpStatus } from "../../lib/ipc/types";
import { managerInfo, useManagersStore } from "../../store/managers";
import { useOperationsStore, type OpView } from "../../store/operations";
import { useUiStore } from "../../store/ui";
import { isTerminal, kindWord } from "./opDisplay";

function subjectName(op: OpView): string {
  const detection = useManagersStore.getState().detection;
  return managerInfo(detection, op.subject)?.displayName ?? op.subject;
}

function successMessage(op: OpView, name: string): string {
  switch (op.kind) {
    case "upgrade": {
      const n = op.packageIds.length;
      return n > 0
        ? `${name}: ${n} package${n === 1 ? "" : "s"} upgraded`
        : `${name}: upgrade complete`;
    }
    case "selfUpdate":
      return `${name} updated`;
    case "healthFix":
      return `${name}: fix applied`;
    case "refresh":
      return `${name}: refresh complete`;
  }
}

function handleTransition(op: OpView, prevStatus: OpStatus): void {
  const ui = useUiStore.getState();
  const mutating = op.kind !== "refresh";
  const name = subjectName(op);

  // Drawer auto-open when a mutating op starts (SPEC §4.9). Never for refreshes.
  if (op.status === "running" && prevStatus !== "running") {
    if (mutating && ui.settings?.autoOpenDrawer) {
      ui.setFocusedOp(op.opId);
      ui.setDrawerOpen(true);
    }
    return;
  }

  if (!isTerminal(op.status)) return;

  switch (op.status) {
    case "succeeded":
      // Refresh success stays quiet (its result shows on the manager card).
      if (mutating) ui.pushToast({ kind: "success", message: successMessage(op, name) });
      break;
    case "failed":
    case "timedOut": {
      const exit = op.exitCode != null ? ` (exit ${op.exitCode})` : "";
      ui.pushToast({
        kind: "error",
        message: `${name}: ${kindWord(op.kind)} failed${exit}`,
        opId: op.opId,
        persistent: true,
      });
      break;
    }
    case "cancelled":
      if (mutating) {
        ui.pushToast({ kind: "info", message: `${name}: ${kindWord(op.kind)} cancelled` });
      }
      break;
    // interrupted: only ever a first-sighting (start-without-finish) → no toast.
  }
}

export function useOperationEffects(): void {
  useEffect(() => {
    return useOperationsStore.subscribe((state, prev) => {
      for (const id of state.order) {
        const op = state.byId[id];
        if (!op) continue;
        const before = prev.byId[id]?.status;
        // Only act on transitions of already-known ops; first sightings are silent.
        if (before === undefined || before === op.status) continue;
        handleTransition(op, before);
      }
    });
  }, []);
}
