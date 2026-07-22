/**
 * packages store — merged snapshots by manager, plus per-manager selection and
 * filter state (SPEC §5.11).
 *
 * The store keeps *primitive* selection ops (toggle, range, set, clear). The
 * exclusion policy — pinned rows and greedy casks are never in select-all — is
 * applied by the toolbar/table (U7), which passes only the selectable ids into
 * {@link PackagesState.setSelection} / {@link PackagesState.selectRange}.
 */
import { create } from "zustand";
import type { ManagerId, ManagerSnapshot, Package } from "../lib/ipc/types";

export interface PackagesState {
  snapshots: Partial<Record<ManagerId, ManagerSnapshot>>;
  /** A manager whose last refresh failed keeps a prior snapshot, marked stale. */
  stale: Partial<Record<ManagerId, boolean>>;
  selection: Partial<Record<ManagerId, Set<string>>>;
  /** Shift-range anchor per manager. */
  anchor: Partial<Record<ManagerId, string | null>>;
  search: Partial<Record<ManagerId, string>>;
  /** undefined = use the default (ON when any row is outdated). */
  outdatedOnly: Partial<Record<ManagerId, boolean>>;

  setSnapshot: (id: ManagerId, snapshot: ManagerSnapshot) => void;
  markStale: (id: ManagerId) => void;
  toggleSelect: (id: ManagerId, packageId: string) => void;
  setSelection: (id: ManagerId, ids: Iterable<string>) => void;
  /** Select the inclusive range from the current anchor to `target`. */
  selectRange: (id: ManagerId, target: string, orderedSelectable: string[]) => void;
  clearSelection: (id: ManagerId) => void;
  setAnchor: (id: ManagerId, packageId: string | null) => void;
  setSearch: (id: ManagerId, search: string) => void;
  setOutdatedOnly: (id: ManagerId, value: boolean) => void;
  reset: () => void;
}

function initial(): Pick<
  PackagesState,
  "snapshots" | "stale" | "selection" | "anchor" | "search" | "outdatedOnly"
> {
  return { snapshots: {}, stale: {}, selection: {}, anchor: {}, search: {}, outdatedOnly: {} };
}

export const usePackagesStore = create<PackagesState>((set) => ({
  ...initial(),

  setSnapshot: (id, snapshot) =>
    set((s) => {
      const stale = { ...s.stale };
      delete stale[id];
      // Drop selected ids that no longer exist in the fresh snapshot.
      const present = new Set(snapshot.packages.map((p) => p.id));
      const prev = s.selection[id];
      const selection = { ...s.selection };
      if (prev) selection[id] = new Set([...prev].filter((pid) => present.has(pid)));
      // Default the outdated-only toggle ON when anything is outdated.
      const outdatedOnly = { ...s.outdatedOnly };
      if (outdatedOnly[id] === undefined) {
        outdatedOnly[id] = snapshot.packages.some((p) => p.outdated);
      }
      return {
        snapshots: { ...s.snapshots, [id]: snapshot },
        stale,
        selection,
        outdatedOnly,
      };
    }),

  markStale: (id) => set((s) => ({ stale: { ...s.stale, [id]: true } })),

  toggleSelect: (id, packageId) =>
    set((s) => {
      const next = new Set(s.selection[id] ?? []);
      if (next.has(packageId)) next.delete(packageId);
      else next.add(packageId);
      return {
        selection: { ...s.selection, [id]: next },
        anchor: { ...s.anchor, [id]: packageId },
      };
    }),

  setSelection: (id, ids) =>
    set((s) => ({ selection: { ...s.selection, [id]: new Set(ids) } })),

  selectRange: (id, target, orderedSelectable) =>
    set((s) => {
      const anchor = s.anchor[id];
      if (!anchor || !orderedSelectable.includes(anchor)) {
        // No usable anchor: behave like a single selection.
        return {
          selection: { ...s.selection, [id]: new Set([target]) },
          anchor: { ...s.anchor, [id]: target },
        };
      }
      const a = orderedSelectable.indexOf(anchor);
      const b = orderedSelectable.indexOf(target);
      if (b < 0) return {};
      const [lo, hi] = a <= b ? [a, b] : [b, a];
      const next = new Set(s.selection[id] ?? []);
      for (let i = lo; i <= hi; i++) next.add(orderedSelectable[i]);
      return { selection: { ...s.selection, [id]: next } };
    }),

  clearSelection: (id) =>
    set((s) => ({
      selection: { ...s.selection, [id]: new Set() },
      anchor: { ...s.anchor, [id]: null },
    })),

  setAnchor: (id, packageId) => set((s) => ({ anchor: { ...s.anchor, [id]: packageId } })),
  setSearch: (id, search) => set((s) => ({ search: { ...s.search, [id]: search } })),
  setOutdatedOnly: (id, value) =>
    set((s) => ({ outdatedOnly: { ...s.outdatedOnly, [id]: value } })),

  reset: () => set(initial()),
}));

/** Outdated count for a manager (the manager's verdict; greedy casks excluded). */
export function outdatedCount(snapshot: ManagerSnapshot | undefined): number {
  if (!snapshot) return 0;
  return snapshot.packages.filter((p) => p.outdated && p.kind !== "caskGreedy").length;
}

/** Selectable = outdated, not pinned, not a greedy cask (SPEC §F3/§F5). */
export function isSelectable(pkg: Package): boolean {
  return pkg.outdated && !pkg.pinned && pkg.kind !== "caskGreedy";
}

/** Current selection size for a manager. */
export function selectionCount(state: PackagesState, id: ManagerId): number {
  return state.selection[id]?.size ?? 0;
}
