/**
 * SelfUpdateCard — the manager's own update state and route (SPEC §4.8 row 2,
 * §F6, §D21). Shows installed → latest (cross-manager join), the route in plain
 * language with the exact command, and an Update button that is:
 *   - "Updating…" while a self-update op for this manager runs,
 *   - accompanied by a "Queued behind <executor>" chip when the executor lane is
 *     busy (routed updates run on the executor's queue),
 *   - disabled with a reason when the route is unavailable or its executor is
 *     absent.
 * npm's permanent mise-reset note (route.note) is rendered here at the point of
 * action.
 */
import { selfUpdateManager } from "../../lib/ipc/client";
import type { ManagerId } from "../../lib/ipc/types";
import { managerInfo, useManagersStore } from "../../store/managers";
import { activeOps, useOperationsStore } from "../../store/operations";
import { usePackagesStore } from "../../store/packages";
import { Button } from "../primitives/Button";
import { Chip } from "../primitives/Chip";
import { Spinner } from "../primitives/Spinner";
import { VersionDelta } from "./VersionDelta";

export interface SelfUpdateCardProps {
  managerId: ManagerId;
}

export function SelfUpdateCard({ managerId }: SelfUpdateCardProps) {
  const detection = useManagersStore((s) => s.detection);
  const snapshot = usePackagesStore((s) => s.snapshots[managerId]);
  const opsState = useOperationsStore();

  const info = managerInfo(detection, managerId);
  if (!info) return null;

  const route = info.selfUpdate;
  const self = snapshot?.selfStatus;
  const live = activeOps(opsState);

  const selfUpdating = live.some(
    (o) => o.subject === managerId && o.kind === "selfUpdate" && o.status === "running",
  );

  const executorId = route.kind === "routed" ? route.executor : null;
  const executorInfo = executorId ? managerInfo(detection, executorId) : undefined;
  const executorName = executorInfo?.displayName ?? executorId ?? "";
  const executorPresent = !executorId || executorInfo?.status === "present";
  const executorBusy = executorId ? live.some((o) => o.executor === executorId) : false;

  const disabledReason =
    route.kind === "unavailable"
      ? route.reason
      : route.kind === "routed" && !executorPresent
        ? `${executorName} is not installed`
        : null;

  const disabled = selfUpdating || disabledReason != null;
  const updateKnown = !!self?.updateAvailable;

  return (
    <section
      aria-label={`${info.displayName} self-update`}
      className="rounded-card border border-border bg-bg-surface p-4"
    >
      <div className="flex items-start gap-3">
        <div className="min-w-0 flex-1">
          <div className="text-[11px] font-medium uppercase tracking-wide text-text-muted">Manager</div>
          <div className="mt-0.5 flex items-center gap-2">
            <span className="text-[15px] font-semibold text-text-primary">{info.displayName}</span>
            {self ? (
              <VersionDelta
                installed={self.installed}
                latest={self.latest}
                outdated={self.updateAvailable}
              />
            ) : (
              <span className="font-mono text-[12px] text-text-secondary">{info.version ?? "—"}</span>
            )}
          </div>

          {/* Route copy (SPEC §F6). */}
          <div className="mt-2 space-y-1 text-[13px] text-text-secondary">
            {route.kind === "routed" && (
              <p>
                {route.why} — this runs{" "}
                <code className="font-mono text-[12px] text-text-primary">{route.commandPreview}</code> on
                the {executorName} queue.
              </p>
            )}
            {route.kind === "inBand" && (
              <p>
                Runs <code className="font-mono text-[12px] text-text-primary">{route.commandPreview}</code>.
              </p>
            )}
            {route.kind === "viaRefresh" && <p>{route.note}</p>}
            {route.kind === "unavailable" && <p className="text-text-muted">{route.reason}</p>}
          </div>

          {/* Permanent note (npm mise-reset warning, §D21). */}
          {route.kind === "inBand" && route.note && (
            <p className="mt-2 rounded-control border border-warning/30 bg-warning/12 px-2.5 py-1.5 text-[12px] text-warning">
              {route.note}
            </p>
          )}
        </div>

        <div className="flex shrink-0 flex-col items-end gap-1.5">
          {disabledReason ? (
            <Button variant="secondary" size="sm" disabled title={disabledReason}>
              Update {info.displayName}
            </Button>
          ) : (
            <Button
              variant={updateKnown ? "primary" : "secondary"}
              size="sm"
              disabled={disabled}
              onClick={() => void selfUpdateManager(managerId)}
            >
              {selfUpdating ? (
                <>
                  <Spinner size={12} /> Updating…
                </>
              ) : (
                `Update ${info.displayName}`
              )}
            </Button>
          )}
          {executorBusy && !selfUpdating && route.kind === "routed" && (
            <Chip tone="neutral">Queued behind {executorName}</Chip>
          )}
          {disabledReason && <span className="text-[11px] text-text-muted">{disabledReason}</span>}
        </div>
      </div>
    </section>
  );
}
