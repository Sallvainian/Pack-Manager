/**
 * UpgradePlanSheet — the trust device (SPEC §4.10, §F4, §D6). Previews the EXACT
 * commands the backend will spawn (never a bare `brew upgrade`), the two toggles
 * (self-updates on / greedy off), the excluded list with reasons, and any
 * warnings. Confirm calls `execute_plan` with the currently-displayed plan;
 * toggling rebuilds the plan through `build_upgrade_plan` so what is shown is
 * always what will run.
 *
 * Mounted by DialogHost (U8) for `ui.dialog.kind === "upgradePlan"`, receiving
 * that dialog's `plan`.
 */
import { useEffect, useState } from "react";
import { buildUpgradePlan, executePlan } from "../../lib/ipc/client";
import type { ExcludeReason, ManagerId, PlanSelection, UpgradePlan } from "../../lib/ipc/types";
import { managerInfo, useManagersStore } from "../../store/managers";
import { usePackagesStore } from "../../store/packages";
import { useUiStore } from "../../store/ui";
import { Button } from "../primitives/Button";

const REASON_LABEL: Record<ExcludeReason, string> = {
  pinned: "pinned",
  greedyCask: "self-updating cask",
  rustDedup: "handled by rustup",
  alreadyRunning: "already running",
};

/** Rebuild the full candidate selection from a plan (groups + everything excluded),
 * so a toggle change re-filters against the same set the backend last considered. */
function reconstructSelection(plan: UpgradePlan): PlanSelection[] {
  const selection: PlanSelection[] = [];
  for (const g of plan.groups) {
    for (const packageId of g.packageIds) selection.push({ managerId: g.subject, packageId });
  }
  for (const e of plan.excluded) {
    selection.push({ managerId: e.managerId, packageId: e.packageId });
  }
  return selection;
}

/** Best-effort display name for a packageId (`${kind}:${name}`, split on first ':'). */
function packageName(packageId: string): string {
  const i = packageId.indexOf(":");
  return i >= 0 ? packageId.slice(i + 1) : packageId;
}

export interface UpgradePlanSheetProps {
  plan: UpgradePlan;
}

export function UpgradePlanSheet({ plan: initialPlan }: UpgradePlanSheetProps) {
  const detection = useManagersStore((s) => s.detection);
  const settings = useUiStore((s) => s.settings);
  const closeDialog = useUiStore((s) => s.closeDialog);
  const clearSelection = usePackagesStore((s) => s.clearSelection);

  const [plan, setPlan] = useState<UpgradePlan>(initialPlan);
  const [includeSelfUpdates, setIncludeSelfUpdates] = useState(true);
  const [includeGreedyCasks, setIncludeGreedyCasks] = useState(
    settings?.includeGreedyByDefault ?? false,
  );
  const [submitting, setSubmitting] = useState(false);

  // Local Escape-to-close (SPEC §4.11); harmless alongside U8's global handler.
  useEffect(() => {
    function onKey(e: KeyboardEvent) {
      if (e.key === "Escape") closeDialog();
    }
    window.addEventListener("keydown", onKey);
    return () => window.removeEventListener("keydown", onKey);
  }, [closeDialog]);

  function displayName(id: ManagerId): string {
    return managerInfo(detection, id)?.displayName ?? id;
  }

  async function applyToggles(nextSelf: boolean, nextGreedy: boolean) {
    setIncludeSelfUpdates(nextSelf);
    setIncludeGreedyCasks(nextGreedy);
    try {
      const next = await buildUpgradePlan({
        selection: reconstructSelection(plan),
        includeSelfUpdates: nextSelf,
        includeGreedyCasks: nextGreedy,
      });
      setPlan(next);
    } catch {
      // Keep the current preview; the toggles reflect the requested intent.
    }
  }

  async function confirm() {
    setSubmitting(true);
    try {
      await executePlan(plan);
      // Selection clears after enqueue (SPEC §F5).
      for (const g of plan.groups) clearSelection(g.subject);
      closeDialog();
    } catch {
      setSubmitting(false);
    }
  }

  const totalPackages = plan.groups.reduce((n, g) => n + g.packageIds.length, 0);
  const selfUpdateCount = plan.groups.filter((g) => g.selfUpdate).length;
  const greedyExcludedCount = plan.excluded.filter((e) => e.reason === "greedyCask").length;
  const hasCommands = plan.groups.some((g) => g.commands.length > 0);

  return (
    <div
      className="fixed inset-0 z-50 flex items-center justify-center bg-black/50 p-4"
      onClick={closeDialog}
    >
      <div
        role="dialog"
        aria-modal="true"
        aria-label="Upgrade plan"
        onClick={(e) => e.stopPropagation()}
        className="flex max-h-[85vh] w-[560px] max-w-full flex-col overflow-hidden rounded-card border border-border-strong bg-bg-overlay shadow-2xl"
      >
        <header className="border-b border-border px-5 py-3">
          <h2 className="text-[15px] font-semibold text-text-primary">
            Upgrade {totalPackages} package{totalPackages === 1 ? "" : "s"}
          </h2>
        </header>

        <div className="flex-1 overflow-auto px-5 py-4">
          {/* Toggles */}
          <div className="mb-4 flex flex-col gap-2">
            <label className="flex cursor-pointer items-center gap-2 text-[13px] text-text-secondary">
              <input
                type="checkbox"
                checked={includeSelfUpdates}
                onChange={(e) => void applyToggles(e.target.checked, includeGreedyCasks)}
                aria-label="Include manager self-updates"
                className="h-4 w-4 rounded-[4px] border border-border-strong bg-bg-raised accent-accent"
              />
              Include manager self-updates{selfUpdateCount > 0 ? ` (${selfUpdateCount})` : ""}
            </label>
            <label className="flex cursor-pointer items-center gap-2 text-[13px] text-text-secondary">
              <input
                type="checkbox"
                checked={includeGreedyCasks}
                onChange={(e) => void applyToggles(includeSelfUpdates, e.target.checked)}
                aria-label="Include self-updating casks"
                className="h-4 w-4 rounded-[4px] border border-border-strong bg-bg-raised accent-accent"
              />
              Include self-updating casks{greedyExcludedCount > 0 ? ` (${greedyExcludedCount})` : ""}
            </label>
          </div>

          {/* Per-manager command previews */}
          <div className="flex flex-col gap-3">
            {plan.groups.map((g) => (
              <section key={`${g.subject}-${g.executor}`} aria-label={`${g.subject} commands`}>
                <div className="mb-1 flex items-center gap-2 text-[13px] font-medium text-text-primary">
                  {displayName(g.subject)}
                  {g.executor !== g.subject && (
                    <span className="text-[11px] uppercase tracking-wide text-text-muted">
                      via {displayName(g.executor)}
                    </span>
                  )}
                </div>
                <div className="flex flex-col gap-1">
                  {g.commands.map((cmd, i) => (
                    <pre
                      key={i}
                      className="overflow-x-auto rounded-control border border-border bg-bg-inset px-3 py-2 font-mono text-[12px] text-text-secondary"
                    >
                      {cmd.argvPreview}
                    </pre>
                  ))}
                </div>
              </section>
            ))}
          </div>

          {/* Excluded */}
          {plan.excluded.length > 0 && (
            <div className="mt-4">
              <div className="text-[11px] font-medium uppercase tracking-wide text-text-muted">
                Excluded
              </div>
              <ul className="mt-1 flex flex-col gap-0.5">
                {plan.excluded.map((e) => (
                  <li key={`${e.managerId}:${e.packageId}`} className="text-[12px] text-text-secondary">
                    <span className="font-mono">{packageName(e.packageId)}</span>
                    <span className="text-text-muted"> — {REASON_LABEL[e.reason]}</span>
                  </li>
                ))}
              </ul>
            </div>
          )}

          {/* Warnings */}
          {plan.warnings.length > 0 && (
            <div className="mt-4 rounded-control border border-warning/30 bg-warning/12 px-3 py-2">
              {plan.warnings.map((w, i) => (
                <div key={i} className="text-[12px] text-warning">
                  {w}
                </div>
              ))}
            </div>
          )}

          {/* Notes */}
          {plan.notes.length > 0 && (
            <ul className="mt-3 flex flex-col gap-0.5">
              {plan.notes.map((n, i) => (
                <li key={i} className="text-[12px] text-text-muted">
                  {n}
                </li>
              ))}
            </ul>
          )}
        </div>

        <footer className="flex items-center justify-between gap-3 border-t border-border px-5 py-3">
          <span className="text-[11px] text-text-muted">
            Managers run in parallel; each manager runs one command at a time.
          </span>
          <div className="flex items-center gap-2">
            <Button variant="ghost" size="md" onClick={closeDialog}>
              Cancel
            </Button>
            <Button variant="primary" size="md" disabled={submitting || !hasCommands} onClick={() => void confirm()}>
              Upgrade
            </Button>
          </div>
        </footer>
      </div>
    </div>
  );
}
