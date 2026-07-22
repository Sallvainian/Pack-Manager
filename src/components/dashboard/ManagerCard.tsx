import { useState } from "react";
import {
  buildUpgradePlan,
  detectManagers,
  refreshManager,
  revealOperationLog,
  selfUpdateManager,
} from "../../lib/ipc/client";
import { errorCopy, stderrTail } from "../../lib/errors";
import type { ManagerInfo, PlanSelection } from "../../lib/ipc/types";
import { useManagerPhase } from "../../store";
import { useManagersStore } from "../../store/managers";
import { isSelectable, outdatedCount, usePackagesStore } from "../../store/packages";
import { useUiStore } from "../../store/ui";
import { Button } from "../primitives/Button";
import { CopyableCommand } from "../primitives/CopyableCommand";
import { ManagedByChip } from "./ManagedByChip";

function ManagerGlyph({ name }: { name: string }) {
  return (
    <span
      aria-hidden="true"
      className="flex h-8 w-8 shrink-0 items-center justify-center rounded-control bg-bg-raised text-[13px] font-semibold text-text-secondary"
    >
      {name.slice(0, 1).toUpperCase()}
    </span>
  );
}

export interface ManagerCardProps {
  info: ManagerInfo;
}

export function ManagerCard({ info }: ManagerCardProps) {
  const snapshot = usePackagesStore((s) => s.snapshots[info.id]);
  const stale = usePackagesStore((s) => !!s.stale[info.id]);
  const error = useManagersStore((s) => s.errors[info.id]);
  const phase = useManagerPhase(info.id);
  const navigate = useUiStore((s) => s.navigate);
  const openDialog = useUiStore((s) => s.openDialog);
  const settings = useUiStore((s) => s.settings);
  const [menuOpen, setMenuOpen] = useState(false);

  const busy = phase === "refreshing" || phase === "busy";
  const outdated = outdatedCount(snapshot);

  function view() {
    navigate({ kind: "manager", managerId: info.id });
  }

  async function upgradeAll() {
    if (!snapshot) return;
    const selection: PlanSelection[] = snapshot.packages
      .filter(isSelectable)
      .map((p) => ({ managerId: info.id, packageId: p.id }));
    const plan = await buildUpgradePlan({
      selection,
      includeSelfUpdates: true,
      includeGreedyCasks: settings?.includeGreedyByDefault ?? false,
    });
    openDialog({ kind: "upgradePlan", plan });
  }

  // --- Absent -------------------------------------------------------------
  if (info.status === "absent") {
    return (
      <section
        aria-label={info.displayName}
        className="rounded-card border border-border bg-bg-surface p-4 opacity-60"
      >
        <header className="flex items-center gap-3">
          <ManagerGlyph name={info.displayName} />
          <div className="text-[15px] font-semibold text-text-primary">{info.displayName}</div>
          <span className="ml-auto text-[11px] uppercase tracking-wide text-text-muted">
            Not installed
          </span>
        </header>
        {info.installHint && (
          <div className="mt-3">
            <CopyableCommand command={info.installHint} label="Install" />
          </div>
        )}
      </section>
    );
  }

  return (
    <section
      aria-label={info.displayName}
      className="relative overflow-hidden rounded-card border border-border bg-bg-surface p-4"
    >
      {busy && (
        <div
          data-testid="busy-bar"
          className="absolute inset-x-0 top-0 h-0.5 animate-pulse bg-accent"
        />
      )}

      <header className="flex items-center gap-3">
        <ManagerGlyph name={info.displayName} />
        <button
          type="button"
          onClick={view}
          className="text-left text-[15px] font-semibold text-text-primary hover:text-accent focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-accent"
        >
          {info.displayName}
        </button>
        <ManagedByChip info={info} />
        <div className="relative ml-auto">
          <Button
            variant="ghost"
            size="sm"
            aria-label={`${info.displayName} actions`}
            onClick={() => setMenuOpen((o) => !o)}
          >
            ⋯
          </Button>
          {menuOpen && (
            <div
              role="menu"
              className="absolute right-0 z-20 mt-1 w-44 rounded-control border border-border-strong bg-bg-overlay py-1 text-[13px] shadow-lg"
              onMouseLeave={() => setMenuOpen(false)}
            >
              <MenuItem onClick={() => { view(); setMenuOpen(false); }}>View packages</MenuItem>
              {info.selfUpdate.kind !== "unavailable" && (
                <MenuItem
                  onClick={() => {
                    void selfUpdateManager(info.id);
                    setMenuOpen(false);
                  }}
                >
                  Self-update
                </MenuItem>
              )}
              <MenuItem
                onClick={() => {
                  void detectManagers();
                  setMenuOpen(false);
                }}
              >
                Re-detect
              </MenuItem>
            </div>
          )}
        </div>
      </header>

      <div className="mt-2 truncate font-mono text-[12px] text-text-secondary" title={info.binaryPath}>
        {info.version ?? "—"}
        {info.binaryPath ? ` · ${info.binaryPath}` : ""}
      </div>

      {error ? (
        <div className="mt-3 rounded-control border-l-4 border-l-danger bg-danger/8 px-3 py-2">
          <div className="text-[13px] font-medium text-text-primary">{errorCopy(error).title}</div>
          <div className="text-[12px] text-text-secondary">{errorCopy(error).message}</div>
          {stderrTail(error, 2).map((line, i) => (
            <pre key={i} className="mt-1 overflow-auto font-mono text-[11px] text-text-muted">
              {line}
            </pre>
          ))}
          <div className="mt-2 flex items-center gap-2">
            <Button variant="secondary" size="sm" onClick={() => void refreshManager(info.id)}>
              Retry
            </Button>
            {error.opId && (
              <Button variant="ghost" size="sm" onClick={() => void revealOperationLog(error.opId!)}>
                View log
              </Button>
            )}
          </div>
          {stale && snapshot && (
            <div className="mt-2 text-[11px] text-text-muted">
              Showing last successful data from {snapshot.refreshedAt}
            </div>
          )}
        </div>
      ) : !snapshot ? (
        <div className="mt-4 flex items-baseline gap-2">
          <div className="h-4 w-6 animate-pulse rounded bg-bg-raised" data-testid="numeral-skeleton" />
          <span className="text-[12px] text-text-muted">Refreshing…</span>
        </div>
      ) : (
        <>
          <div className="mt-4 flex items-baseline gap-1.5">
            {outdated > 0 ? (
              <>
                {/* Same size/weight as the "Up to date" line below: the two are
                    peer states of one card, so neither should outweigh the other
                    (and neither should outweigh the page's 20px h1). */}
                <span className="text-[15px] font-semibold text-warning">{outdated}</span>
                <span className="text-[12px] text-text-secondary">
                  update{outdated === 1 ? "" : "s"} available
                </span>
              </>
            ) : (
              <span className="flex items-center gap-1.5 text-[15px] font-semibold text-success">
                ✓ Up to date
              </span>
            )}
          </div>

          {snapshot.selfStatus?.updateAvailable && (
            <div className="mt-2 font-mono text-[12px] text-text-secondary">
              {info.displayName}: {snapshot.selfStatus.installed ?? "—"} →{" "}
              <span className="text-accent">{snapshot.selfStatus.latest ?? "?"}</span>
            </div>
          )}

          <footer className="mt-4 flex items-center gap-2">
            <Button variant="ghost" size="sm" onClick={() => void refreshManager(info.id)}>
              Refresh
            </Button>
            {outdated > 0 && (
              <Button variant="secondary" size="sm" onClick={() => void upgradeAll()}>
                Upgrade all ({outdated})
              </Button>
            )}
          </footer>
        </>
      )}
    </section>
  );
}

function MenuItem({ children, onClick }: { children: React.ReactNode; onClick: () => void }) {
  return (
    <button
      type="button"
      role="menuitem"
      onClick={onClick}
      className="block w-full px-3 py-1.5 text-left text-text-secondary hover:bg-bg-raised hover:text-text-primary"
    >
      {children}
    </button>
  );
}
