/**
 * useKeyboard — the global keyboard map (SPEC §4.11). Mounted once (from the
 * always-present ActivityDrawer), it installs a single window `keydown` listener
 * and dispatches store actions / IPC calls off the current store state read fresh
 * on each event (no stale closures).
 *
 * Map: Cmd+R refresh current (Dashboard: all) · Cmd+Shift+R refresh all ·
 * Cmd+U upgrade selected (plan sheet) · Cmd+Shift+U Update Everything (sheet) ·
 * Cmd+A select all visible selectable rows · Esc clear selection / close sheet /
 * close drawer · Cmd+L toggle drawer · Cmd+F focus search · Cmd+1..9 sidebar jump.
 *
 * Cmd is matched via metaKey or ctrlKey so the map also works under non-mac test
 * runners. Shortcuts are suppressed while focus is in an editable field (except
 * Escape, which always resolves the current overlay/selection).
 */
import { useEffect } from "react";
import { buildUpgradePlan, refreshAll, refreshManager } from "../lib/ipc/client";
import type { ManagerId, PlanRequest } from "../lib/ipc/types";
import { presentManagers, useManagersStore } from "../store/managers";
import { isSelectable, usePackagesStore } from "../store/packages";
import { useUiStore, type ActiveView } from "../store/ui";

function isEditable(target: EventTarget | null): boolean {
  if (!(target instanceof HTMLElement)) return false;
  const tag = target.tagName;
  return (
    tag === "INPUT" ||
    tag === "TEXTAREA" ||
    tag === "SELECT" ||
    target.isContentEditable
  );
}

/** Visible + selectable package ids for a manager (mirrors ManagerPane filters). */
function visibleSelectableIds(managerId: ManagerId): string[] {
  const pkgs = usePackagesStore.getState();
  const snap = pkgs.snapshots[managerId];
  if (!snap) return [];
  const search = (pkgs.search[managerId] ?? "").trim().toLowerCase();
  const anyOutdated = snap.packages.some((p) => p.outdated && p.kind !== "caskGreedy");
  const outdatedOnly = pkgs.outdatedOnly[managerId] ?? anyOutdated;
  return snap.packages
    .filter((p) => p.kind !== "caskGreedy")
    .filter(
      (p) =>
        !search ||
        p.name.toLowerCase().includes(search) ||
        (p.meta?.executables ?? []).some((e) => e.toLowerCase().includes(search)),
    )
    .filter((p) => !outdatedOnly || p.outdated)
    .filter(isSelectable)
    .map((p) => p.id);
}

async function openPlan(req: PlanRequest): Promise<void> {
  try {
    const plan = await buildUpgradePlan(req);
    useUiStore.getState().openDialog({ kind: "upgradePlan", plan });
  } catch {
    // Build failure surfaces via its own error path; the shortcut is a no-op.
  }
}

function handleEscape(): void {
  const ui = useUiStore.getState();
  if (ui.dialog.kind !== "none") {
    ui.closeDialog();
    return;
  }
  if (ui.view.kind === "manager") {
    const pkgs = usePackagesStore.getState();
    const sel = pkgs.selection[ui.view.managerId];
    if (sel && sel.size > 0) {
      pkgs.clearSelection(ui.view.managerId);
      return;
    }
  }
  if (ui.drawerOpen) ui.setDrawerOpen(false);
}

function refreshCurrent(): void {
  const ui = useUiStore.getState();
  if (ui.view.kind === "manager") void refreshManager(ui.view.managerId);
  else void refreshAll();
}

function selectAllVisible(): void {
  const ui = useUiStore.getState();
  if (ui.view.kind !== "manager") return;
  usePackagesStore.getState().setSelection(ui.view.managerId, visibleSelectableIds(ui.view.managerId));
}

function upgradeSelected(): void {
  const ui = useUiStore.getState();
  if (ui.view.kind !== "manager") return;
  const managerId = ui.view.managerId;
  const sel = [...(usePackagesStore.getState().selection[managerId] ?? [])];
  if (sel.length === 0) return;
  void openPlan({
    selection: sel.map((packageId) => ({ managerId, packageId })),
    includeSelfUpdates: true,
    includeGreedyCasks: ui.settings?.includeGreedyByDefault ?? false,
  });
}

function updateEverything(): void {
  const ui = useUiStore.getState();
  void openPlan({
    selection: null,
    includeSelfUpdates: true,
    includeGreedyCasks: ui.settings?.includeGreedyByDefault ?? false,
  });
}

function focusSearch(): void {
  const el = document.querySelector<HTMLInputElement>(
    'input[type="search"], input[aria-label*="Search" i], input[placeholder*="Search" i]',
  );
  el?.focus();
}

function sidebarJump(n: number): void {
  const detection = useManagersStore.getState().detection;
  const targets: ActiveView[] = [
    { kind: "dashboard" },
    ...presentManagers(detection).map((m) => ({ kind: "manager" as const, managerId: m.id })),
    { kind: "history" },
    { kind: "settings" },
  ];
  const target = targets[n - 1];
  if (target) useUiStore.getState().navigate(target);
}

export function useKeyboard(): void {
  useEffect(() => {
    function onKey(e: KeyboardEvent) {
      // Escape resolves overlays/selection even from within inputs.
      if (e.key === "Escape") {
        handleEscape();
        return;
      }
      if (isEditable(e.target)) return;

      const meta = e.metaKey || e.ctrlKey;
      if (!meta) return;

      const key = e.key;
      const lower = key.toLowerCase();
      switch (lower) {
        case "r":
          e.preventDefault();
          if (e.shiftKey) void refreshAll();
          else refreshCurrent();
          break;
        case "u":
          e.preventDefault();
          if (e.shiftKey) updateEverything();
          else upgradeSelected();
          break;
        case "a":
          e.preventDefault();
          selectAllVisible();
          break;
        case "l":
          e.preventDefault();
          useUiStore.getState().toggleDrawer();
          break;
        case "f":
          e.preventDefault();
          focusSearch();
          break;
        default:
          if (/^[1-9]$/.test(key)) {
            e.preventDefault();
            sidebarJump(Number(key));
          }
      }
    }

    window.addEventListener("keydown", onKey);
    return () => window.removeEventListener("keydown", onKey);
  }, []);
}
