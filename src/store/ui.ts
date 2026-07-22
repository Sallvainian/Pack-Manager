/**
 * ui store — view routing (no router lib), the ActivityDrawer geometry, the
 * dialog stack, toasts, and the working copy of Settings (SPEC §4.4, §5.11).
 *
 * The dialog union is defined here (not in U7/U8) so those units can dispatch
 * and read dialog state without touching the store: UpgradePlanSheet (U7) and
 * StalledOperationDialog/QuitGuardDialog (U8) render off `dialog.kind`.
 */
import { create } from "zustand";
import type { ManagerId, Settings, UpgradePlan } from "../lib/ipc/types";

export type ActiveView =
  | { kind: "dashboard" }
  | { kind: "manager"; managerId: ManagerId }
  | { kind: "history" }
  | { kind: "settings" };

export type Dialog =
  | { kind: "none" }
  | { kind: "upgradePlan"; plan: UpgradePlan }
  | { kind: "stalled"; opId: string; silentForSecs: number }
  /**
   * Running operations block a quit — and equally an update restart, which
   * kills the same child processes (DECISIONS D25).
   */
  | { kind: "quitGuard"; opIds: string[]; reason?: "quit" | "update" };

export type ToastKind = "success" | "error" | "info";

export interface Toast {
  id: string;
  kind: ToastKind;
  message: string;
  /** When set, the toast offers "View log" which opens the drawer to this op. */
  opId?: string;
  /** Failure toasts persist until dismissed; success/info auto-dismiss. */
  persistent?: boolean;
}

/** Drawer height bounds as a fraction of the window (SPEC §4.9: drag 25–60%). */
export const DRAWER_MIN = 0.25;
export const DRAWER_MAX = 0.6;
export const DRAWER_DEFAULT = 0.4;

/** Cross-manager join navigation target: highlight a row in another manager. */
export interface RowHighlight {
  managerId: ManagerId;
  packageId: string;
}

export interface UiState {
  view: ActiveView;
  drawerOpen: boolean;
  drawerHeight: number;
  focusedOpId: string | null;
  dialog: Dialog;
  toasts: Toast[];
  /** Working copy of Settings, hydrated from get_state/get_settings. */
  settings: Settings | null;
  /** Set when a ManagedByChip navigates to the executor's pane. */
  highlight: RowHighlight | null;

  navigate: (view: ActiveView) => void;
  setDrawerOpen: (open: boolean) => void;
  toggleDrawer: () => void;
  setDrawerHeight: (fraction: number) => void;
  setFocusedOp: (opId: string | null) => void;
  openDialog: (dialog: Dialog) => void;
  closeDialog: () => void;
  pushToast: (toast: Omit<Toast, "id">) => string;
  dismissToast: (id: string) => void;
  setSettings: (settings: Settings) => void;
  setHighlight: (highlight: RowHighlight | null) => void;
  reset: () => void;
}

let toastSeq = 0;
function nextToastId(): string {
  toastSeq += 1;
  return `toast-${toastSeq}`;
}

function initial(): Pick<
  UiState,
  | "view"
  | "drawerOpen"
  | "drawerHeight"
  | "focusedOpId"
  | "dialog"
  | "toasts"
  | "settings"
  | "highlight"
> {
  return {
    view: { kind: "dashboard" },
    drawerOpen: false,
    drawerHeight: DRAWER_DEFAULT,
    focusedOpId: null,
    dialog: { kind: "none" },
    toasts: [],
    settings: null,
    highlight: null,
  };
}

export const useUiStore = create<UiState>((set) => ({
  ...initial(),

  navigate: (view) => set({ view }),
  setDrawerOpen: (open) => set({ drawerOpen: open }),
  toggleDrawer: () => set((s) => ({ drawerOpen: !s.drawerOpen })),
  setDrawerHeight: (fraction) =>
    set({ drawerHeight: Math.min(DRAWER_MAX, Math.max(DRAWER_MIN, fraction)) }),
  setFocusedOp: (opId) => set({ focusedOpId: opId }),
  openDialog: (dialog) => set({ dialog }),
  closeDialog: () => set({ dialog: { kind: "none" } }),
  pushToast: (toast) => {
    const id = nextToastId();
    set((s) => ({ toasts: [...s.toasts, { ...toast, id }] }));
    return id;
  },
  dismissToast: (id) => set((s) => ({ toasts: s.toasts.filter((t) => t.id !== id) })),
  setSettings: (settings) => set({ settings }),
  setHighlight: (highlight) => set({ highlight }),
  reset: () => set(initial()),
}));
