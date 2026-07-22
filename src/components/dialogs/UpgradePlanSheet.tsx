/**
 * UpgradePlanSheet — the trust device (SPEC §4.10, §F4, §D6). Previews the EXACT
 * commands the backend will spawn (never a bare `brew upgrade`), the two toggles
 * (self-updates on / greedy off), the excluded list with reasons, and any
 * warnings. Confirm calls `execute_plan` with the currently-displayed, ready
 * plan. Toggling invalidates that readiness until `build_upgrade_plan` returns,
 * so a pending or failed rebuild can never execute an older preview.
 *
 * Mounted by DialogHost (U8) for `ui.dialog.kind === "upgradePlan"`, receiving
 * that dialog's `plan`.
 */
import { useEffect, useRef, useState } from "react";
import { errorCopy } from "../../lib/errors";
import { buildUpgradePlan, executePlan } from "../../lib/ipc/client";
import {
  isIpcError,
  type ExcludeReason,
  type ManagerId,
  type PlanRequest,
  type UpgradePlan,
} from "../../lib/ipc/types";
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

/** Best-effort display name for a packageId (`${kind}:${name}`, split on first ':'). */
function packageName(packageId: string): string {
  const i = packageId.indexOf(":");
  return i >= 0 ? packageId.slice(i + 1) : packageId;
}

export interface UpgradePlanSheetProps {
  plan: UpgradePlan;
}

type PlanReadiness = "ready" | "pending" | "failed";

export function UpgradePlanSheet({ plan: initialPlan }: UpgradePlanSheetProps) {
  const detection = useManagersStore((s) => s.detection);
  const closeDialog = useUiStore((s) => s.closeDialog);
  const clearSelection = usePackagesStore((s) => s.clearSelection);

  const [plan, setPlan] = useState<UpgradePlan>(initialPlan);
  const [includeSelfUpdates, setIncludeSelfUpdates] = useState(
    initialPlan.request.includeSelfUpdates,
  );
  const [includeGreedyCasks, setIncludeGreedyCasks] = useState(
    initialPlan.request.includeGreedyCasks,
  );
  const mounted = useRef(true);
  const rebuildSequence = useRef(0);
  const [planReadiness, setPlanReadiness] = useState<PlanReadiness>("ready");
  const [submitting, setSubmitting] = useState(false);
  const [reviewMessage, setReviewMessage] = useState<string | null>(null);

  // Local Escape-to-close (SPEC §4.11); harmless alongside U8's global handler.
  useEffect(() => {
    function onKey(e: KeyboardEvent) {
      if (e.key === "Escape") closeDialog();
    }
    window.addEventListener("keydown", onKey);
    return () => window.removeEventListener("keydown", onKey);
  }, [closeDialog]);

  // Ignore every late continuation after the sheet has been dismissed. This
  // covers both an in-flight rebuild and execute_plan rejecting as stale after
  // Cancel has already unmounted the dialog.
  useEffect(() => {
    mounted.current = true;
    return () => {
      mounted.current = false;
      rebuildSequence.current += 1;
    };
  }, []);

  function displayName(id: ManagerId): string {
    return managerInfo(detection, id)?.displayName ?? id;
  }

  async function rebuildPlan(
    request: PlanRequest,
    successMessage: string | null,
    failureMessage: string,
  ) {
    if (!mounted.current) return;

    const sequence = ++rebuildSequence.current;
    setPlanReadiness("pending");
    setReviewMessage(null);

    try {
      const next = await buildUpgradePlan(request);
      if (!mounted.current || sequence !== rebuildSequence.current) return;

      setPlan(next);
      setIncludeSelfUpdates(next.request.includeSelfUpdates);
      setIncludeGreedyCasks(next.request.includeGreedyCasks);
      setPlanReadiness("ready");
      setReviewMessage(successMessage);
    } catch (error) {
      if (!mounted.current || sequence !== rebuildSequence.current) return;

      setPlanReadiness("failed");
      setReviewMessage(
        isIpcError(error) ? errorCopy(error).message : failureMessage,
      );
    }
  }

  function applyToggles(nextSelf: boolean, nextGreedy: boolean) {
    if (submitting) return;

    setIncludeSelfUpdates(nextSelf);
    setIncludeGreedyCasks(nextGreedy);
    void rebuildPlan(
      {
        selection: plan.request.selection,
        includeSelfUpdates: nextSelf,
        includeGreedyCasks: nextGreedy,
      },
      null,
      "The plan could not be refreshed. Retry before upgrading.",
    );
  }

  function retryRefresh() {
    if (submitting || planReadiness === "pending") return;

    void rebuildPlan(
      {
        selection: plan.request.selection,
        includeSelfUpdates,
        includeGreedyCasks,
      },
      "The plan was refreshed. Review it before upgrading.",
      "The plan could not be refreshed. Retry before upgrading.",
    );
  }

  async function confirm() {
    if (submitting || planReadiness !== "ready") return;

    setSubmitting(true);
    try {
      await executePlan(plan);
      if (!mounted.current) return;

      // Selection clears after enqueue (SPEC §F5).
      for (const g of plan.groups) clearSelection(g.subject);
      closeDialog();
    } catch (error) {
      if (!mounted.current) return;

      if (isIpcError(error) && error.code === "plan_stale") {
        const staleMessage = errorCopy(error).message;
        setSubmitting(false);
        await rebuildPlan(
          plan.request,
          staleMessage,
          "The plan changed, but the current plan could not be loaded. Retry before upgrading.",
        );
        return;
      } else {
        // The one-use capability may have been consumed even if enqueueing
        // failed. Require a fresh plan instead of retrying it speculatively.
        setPlanReadiness("failed");
        setReviewMessage(
          isIpcError(error)
            ? errorCopy(error).message
            : "The upgrade could not be queued. Review the plan and retry.",
        );
      }
      setSubmitting(false);
    }
  }

  const totalPackages = plan.groups.reduce((n, g) => n + g.packageIds.length, 0);
  const selfUpdateCount = plan.groups.filter((g) => g.selfUpdate).length;
  const greedyExcludedCount = plan.excluded.filter((e) => e.reason === "greedyCask").length;
  const hasCommands = plan.groups.some((g) => g.commands.length > 0);
  const controlsBusy = submitting || planReadiness === "pending";

  return (
    <div
      className="fixed inset-0 z-50 flex items-center justify-center bg-black/50 p-4"
      onClick={closeDialog}
    >
      <div
        role="dialog"
        aria-modal="true"
        aria-label="Upgrade plan"
        aria-busy={controlsBusy}
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
                disabled={controlsBusy}
                onChange={(e) => applyToggles(e.target.checked, includeGreedyCasks)}
                aria-label="Include manager self-updates"
                className="h-4 w-4 rounded-[4px] border border-border-strong bg-bg-raised accent-accent"
              />
              Include manager self-updates{selfUpdateCount > 0 ? ` (${selfUpdateCount})` : ""}
            </label>
            <label className="flex cursor-pointer items-center gap-2 text-[13px] text-text-secondary">
              <input
                type="checkbox"
                checked={includeGreedyCasks}
                disabled={controlsBusy}
                onChange={(e) => applyToggles(includeSelfUpdates, e.target.checked)}
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

          {/* Review required after a stale plan or rebuild failure */}
          {reviewMessage && (
            <div
              role="alert"
              className="mt-4 rounded-control border border-warning/30 bg-warning/12 px-3 py-2 text-[12px] text-warning"
            >
              <div>{reviewMessage}</div>
              {planReadiness === "failed" && (
                <Button
                  variant="secondary"
                  size="sm"
                  className="mt-2"
                  onClick={retryRefresh}
                >
                  Refresh plan
                </Button>
              )}
            </div>
          )}

          {planReadiness === "pending" && (
            <div role="status" className="mt-4 text-[12px] text-text-muted">
              Refreshing plan…
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
            <Button
              variant="primary"
              size="md"
              disabled={submitting || planReadiness !== "ready" || !hasCommands}
              onClick={() => void confirm()}
            >
              Upgrade
            </Button>
          </div>
        </footer>
      </div>
    </div>
  );
}
