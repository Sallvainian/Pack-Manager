/**
 * ManagerPane — the per-manager view (SPEC §4.8). Replaces the U6 stub.
 *
 * Assembles: header (name · ManagedByChip · version · refreshed · Refresh),
 * SelfUpdateCard, HealthBanner, PackageToolbar, virtualized PackageTable, and the
 * SelectionToolbar. Owns the plan flow: bulk actions build a plan and open the
 * Upgrade Plan Sheet; a row's Upgrade button executes a single-package plan
 * immediately (SPEC §F4/§F5/§D6). Pane states: absent, loading, error (+ stale
 * snapshot), clean, populated.
 */
import { useMemo } from "react";
import {
  buildUpgradePlan,
  detectManagers,
  executePlan,
  refreshManager,
  revealOperationLog,
} from "../../lib/ipc/client";
import type { ManagerId, Package, PlanSelection } from "../../lib/ipc/types";
import { managerInfo, useManagersStore } from "../../store/managers";
import { activeOps, useOperationsStore } from "../../store/operations";
import { isSelectable, usePackagesStore } from "../../store/packages";
import { useUiStore } from "../../store/ui";
import { Button } from "../primitives/Button";
import { CopyableCommand } from "../primitives/CopyableCommand";
import { EmptyState } from "../primitives/EmptyState";
import { ErrorState } from "../primitives/ErrorState";
import { SkeletonRows } from "../primitives/SkeletonRows";
import { ManagedByChip } from "../dashboard/ManagedByChip";
import { HealthBanner } from "./HealthBanner";
import { PackageTable } from "./PackageTable";
import { PackageToolbar } from "./PackageToolbar";
import { SelectionToolbar } from "./SelectionToolbar";
import { SelfUpdateCard } from "./SelfUpdateCard";
import type { RowOpState } from "./StatusBadge";

export interface ManagerPaneProps {
  managerId: ManagerId;
}

/** Coarse "refreshed 2m ago" meta (SPEC §4.8). */
function formatRefreshed(iso: string): string {
  const then = Date.parse(iso);
  if (Number.isNaN(then)) return "";
  const secs = Math.max(0, Math.round((Date.now() - then) / 1000));
  if (secs < 45) return "just now";
  const mins = Math.round(secs / 60);
  if (mins < 60) return `${mins}m ago`;
  const hrs = Math.round(mins / 60);
  if (hrs < 24) return `${hrs}h ago`;
  return `${Math.round(hrs / 24)}d ago`;
}

export function ManagerPane({ managerId }: ManagerPaneProps) {
  const detection = useManagersStore((s) => s.detection);
  const error = useManagersStore((s) => s.errors[managerId]);
  const snapshot = usePackagesStore((s) => s.snapshots[managerId]);
  const stale = usePackagesStore((s) => !!s.stale[managerId]);
  const selectionSet = usePackagesStore((s) => s.selection[managerId]);
  const searchRaw = usePackagesStore((s) => s.search[managerId] ?? "");
  const storedOutdatedOnly = usePackagesStore((s) => s.outdatedOnly[managerId]);
  const toggleSelect = usePackagesStore((s) => s.toggleSelect);
  const selectRange = usePackagesStore((s) => s.selectRange);
  const setSelection = usePackagesStore((s) => s.setSelection);
  const clearSelection = usePackagesStore((s) => s.clearSelection);
  const setOutdatedOnly = usePackagesStore((s) => s.setOutdatedOnly);
  const opsState = useOperationsStore();
  const settings = useUiStore((s) => s.settings);
  const openDialog = useUiStore((s) => s.openDialog);
  const highlight = useUiStore((s) => s.highlight);

  const info = managerInfo(detection, managerId);
  const selection = useMemo(() => selectionSet ?? new Set<string>(), [selectionSet]);
  const search = searchRaw.trim().toLowerCase();

  // Per-package live op state (running wins over queued), this manager only.
  const opStateById = useMemo(() => {
    const map: Record<string, RowOpState> = {};
    for (const op of activeOps(opsState)) {
      if (op.subject !== managerId) continue;
      const state: RowOpState | null =
        op.status === "running" ? "upgrading" : op.status === "queued" ? "queued" : null;
      if (!state) continue;
      for (const pid of op.packageIds) {
        if (state === "upgrading" || !map[pid]) map[pid] = state;
      }
    }
    return map;
  }, [opsState, managerId]);

  const packages = snapshot?.packages ?? [];
  const anyOutdated = packages.some((p) => p.outdated && p.kind !== "caskGreedy");
  const outdatedOnly = storedOutdatedOnly ?? anyOutdated;

  function matchesSearch(pkg: Package): boolean {
    if (!search) return true;
    if (pkg.name.toLowerCase().includes(search)) return true;
    return (pkg.meta?.executables ?? []).some((e) => e.toLowerCase().includes(search));
  }

  const mainPackages = packages.filter((p) => p.kind !== "caskGreedy");
  const greedyPackages = packages.filter((p) => p.kind === "caskGreedy");
  const visibleMain = mainPackages.filter(
    (p) => matchesSearch(p) && (!outdatedOnly || p.outdated),
  );
  const visibleGreedy = greedyPackages.filter(matchesSearch);
  const orderedSelectable = visibleMain.filter(isSelectable).map((p) => p.id);

  const installedCount = mainPackages.length;
  const outdatedTotal = mainPackages.filter((p) => p.outdated).length;
  const selectionCount = selection.size;
  const highlightId = highlight?.managerId === managerId ? highlight.packageId : undefined;

  // --- selection handlers --------------------------------------------------
  function onToggleAll() {
    const next = new Set(selection);
    const all = orderedSelectable.length > 0 && orderedSelectable.every((id) => selection.has(id));
    if (all) orderedSelectable.forEach((id) => next.delete(id));
    else orderedSelectable.forEach((id) => next.add(id));
    setSelection(managerId, next);
  }

  // --- plan flow -----------------------------------------------------------
  async function openPlan(sel: PlanSelection[]) {
    const plan = await buildUpgradePlan({
      selection: sel,
      includeSelfUpdates: true,
      includeGreedyCasks: settings?.includeGreedyByDefault ?? false,
    });
    openDialog({ kind: "upgradePlan", plan });
  }

  function upgradeSelected() {
    const sel = [...selection].map((packageId) => ({ managerId, packageId }));
    if (sel.length > 0) void openPlan(sel);
  }

  function upgradeAll() {
    const sel = mainPackages
      .filter(isSelectable)
      .map((p) => ({ managerId, packageId: p.id }));
    if (sel.length > 0) void openPlan(sel);
  }

  async function upgradeRow(pkg: Package) {
    // Single-package plan executes immediately — no sheet (SPEC §F5).
    const plan = await buildUpgradePlan({
      selection: [{ managerId, packageId: pkg.id }],
      includeSelfUpdates: false,
      includeGreedyCasks: false,
    });
    await executePlan(plan);
  }

  // --- header --------------------------------------------------------------
  const header = (
    <header className="flex items-center gap-3 border-b border-border px-6 py-4">
      <h1 className="text-[20px] font-semibold text-text-primary">{info?.displayName ?? managerId}</h1>
      {info && <ManagedByChip info={info} />}
      {info?.version && (
        <span className="font-mono text-[12px] text-text-secondary">{info.version}</span>
      )}
      {snapshot && (
        <span className="text-[11px] uppercase tracking-wide text-text-muted">
          Refreshed {formatRefreshed(snapshot.refreshedAt)}
        </span>
      )}
      <div className="ml-auto">
        <Button variant="secondary" size="sm" onClick={() => void refreshManager(managerId)}>
          Refresh
        </Button>
      </div>
    </header>
  );

  // --- absent (mas) --------------------------------------------------------
  if (info && info.status === "absent") {
    return (
      <div className="flex h-full flex-col">
        {header}
        <div className="flex-1 overflow-auto p-6">
          <EmptyState
            title={`${info.displayName} is not installed`}
            description="Install it to manage these packages here."
            action={
              <div className="flex items-center gap-2">
                {info.installHint && <CopyableCommand command={info.installHint} label="Install" />}
                <Button variant="secondary" size="sm" onClick={() => void detectManagers()}>
                  Re-detect
                </Button>
              </div>
            }
          />
        </div>
      </div>
    );
  }

  const showClean =
    !!snapshot && visibleMain.length === 0 && visibleGreedy.length === 0 && !search;

  return (
    <div className="flex h-full flex-col">
      {header}
      <div className="flex flex-1 flex-col gap-4 overflow-auto p-6">
        <SelfUpdateCard managerId={managerId} />

        {snapshot && snapshot.health.length > 0 && (
          <HealthBanner managerId={managerId} issues={snapshot.health} />
        )}

        {error && (
          <ErrorState
            error={error}
            onRetry={() => void refreshManager(managerId)}
            onViewLog={error.opId ? () => void revealOperationLog(error.opId!) : undefined}
          />
        )}

        {/* Loading: header already shown from detection; skeleton rows below. */}
        {!snapshot && !error && <SkeletonRows rows={8} rowHeight={20} />}

        {snapshot && (
          <>
            {stale && (
              <div className="text-[11px] text-text-muted">
                Showing last successful data from {snapshot.refreshedAt}
              </div>
            )}

            <PackageToolbar
              managerId={managerId}
              installedCount={installedCount}
              outdatedCount={outdatedTotal}
              selectionCount={selectionCount}
              onUpgradeSelected={upgradeSelected}
              onUpgradeAll={upgradeAll}
            />

            {showClean ? (
              <EmptyState
                tone="success"
                icon={<span aria-hidden="true">✓</span>}
                title="Everything up to date"
                description={`${installedCount} installed · 0 outdated`}
              />
            ) : visibleMain.length === 0 && visibleGreedy.length === 0 ? (
              <EmptyState
                title="No matching packages"
                description={
                  outdatedOnly && !search ? (
                    <button
                      type="button"
                      onClick={() => setOutdatedOnly(managerId, false)}
                      className="text-accent hover:underline focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-accent"
                    >
                      Show all packages
                    </button>
                  ) : (
                    "Try a different search."
                  )
                }
              />
            ) : (
              <PackageTable
                managerId={managerId}
                rows={visibleMain}
                greedyRows={visibleGreedy}
                selection={selection}
                orderedSelectable={orderedSelectable}
                opStateById={opStateById}
                highlightId={highlightId}
                onToggleSelect={(id) => toggleSelect(managerId, id)}
                onRangeSelect={(id) => selectRange(managerId, id, orderedSelectable)}
                onToggleAll={onToggleAll}
                onUpgrade={(pkg) => void upgradeRow(pkg)}
              />
            )}

            <SelectionToolbar
              count={selectionCount}
              onUpgradeSelected={upgradeSelected}
              onClear={() => clearSelection(managerId)}
            />
          </>
        )}
      </div>
    </div>
  );
}
