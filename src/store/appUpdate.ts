/**
 * appUpdate store — Pack-Manager updating itself (DECISIONS D25).
 *
 * The state machine lives in Rust (`src-tauri/src/app_update.rs`); this slice
 * only holds the latest `appUpdate:status` payload so the StatusBar indicator
 * and Settings can render off it. Deliberately separate from `operations`,
 * which only ever tracks package-manager work.
 */
import { create } from "zustand";
import type { AppUpdateStatus } from "../lib/ipc/types";

export interface AppUpdateState {
  /** Null until `get_app_update_state` or the first event lands. */
  status: AppUpdateStatus | null;
  setStatus: (status: AppUpdateStatus) => void;
  reset: () => void;
}

export const useAppUpdateStore = create<AppUpdateState>((set) => ({
  status: null,
  setStatus: (status) => set({ status }),
  reset: () => set({ status: null }),
}));

/** True while an update is being fetched. */
export function isDownloading(status: AppUpdateStatus | null): boolean {
  return status?.state.kind === "downloading";
}

/**
 * Download progress as 0..1, or null when the server sent no content length
 * (an indeterminate bar is honest; a fabricated percentage is not).
 */
export function downloadProgress(status: AppUpdateStatus | null): number | null {
  const state = status?.state;
  if (state?.kind !== "downloading") return null;
  if (state.total === null || state.total <= 0) return null;
  return Math.min(1, state.received / state.total);
}

/** The version the user would get by restarting, when one is ready. */
export function pendingVersion(status: AppUpdateStatus | null): string | null {
  const state = status?.state;
  if (state?.kind === "readyToInstall") return state.version;
  if (state?.kind === "manualInstallRequired") return state.version;
  return null;
}
