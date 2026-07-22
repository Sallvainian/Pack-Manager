/**
 * managers store — the DetectionReport plus per-manager last error.
 *
 * Per-manager *phase* (idle/refreshing/busy/error) is not stored here: it is
 * derived from operation records + last error (SPEC §5.9, §5.11). See
 * {@link deriveManagerPhase} in `store/index.ts`.
 */
import { create } from "zustand";
import type { DetectionReport, IpcError, ManagerId, ManagerInfo } from "../lib/ipc/types";

export interface ManagersState {
  detection: DetectionReport | null;
  /** True while a detect_managers call is in flight (sidebar shimmer). */
  detecting: boolean;
  /** Last error per manager (a failed refresh keeps the prior snapshot). */
  errors: Partial<Record<ManagerId, IpcError>>;
  setDetection: (report: DetectionReport) => void;
  setDetecting: (detecting: boolean) => void;
  setManagerError: (id: ManagerId, error: IpcError | null) => void;
  reset: () => void;
}

function initial(): Pick<ManagersState, "detection" | "detecting" | "errors"> {
  return { detection: null, detecting: false, errors: {} };
}

export const useManagersStore = create<ManagersState>((set) => ({
  ...initial(),
  setDetection: (report) => set({ detection: report, detecting: false }),
  setDetecting: (detecting) => set({ detecting }),
  setManagerError: (id, error) =>
    set((s) => {
      const errors = { ...s.errors };
      if (error) errors[id] = error;
      else delete errors[id];
      return { errors };
    }),
  reset: () => set(initial()),
}));

/** Look up one manager's info by id. */
export function managerInfo(
  report: DetectionReport | null,
  id: ManagerId,
): ManagerInfo | undefined {
  return report?.managers.find((m) => m.id === id);
}

/** Managers that resolved as present, in report order. */
export function presentManagers(report: DetectionReport | null): ManagerInfo[] {
  return (report?.managers ?? []).filter((m) => m.status === "present");
}

/** Managers that are not installed (rendered under the "Not installed" disclosure). */
export function absentManagers(report: DetectionReport | null): ManagerInfo[] {
  return (report?.managers ?? []).filter((m) => m.status === "absent");
}
