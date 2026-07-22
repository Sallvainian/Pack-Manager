import { revealLogsDir } from "../../lib/ipc/client";
import { activeOps, useOperationsStore } from "../../store/operations";
import { usePackagesStore } from "../../store/packages";
import { useManagersStore } from "../../store/managers";
import { useUiStore } from "../../store/ui";
import { UpdateStatusItem } from "./UpdateStatusItem";

export function StatusBar() {
  const ops = useOperationsStore();
  const snapshots = usePackagesStore((s) => s.snapshots);
  const detection = useManagersStore((s) => s.detection);
  const navigate = useUiStore((s) => s.navigate);

  const running = activeOps(ops);
  const runningCount = running.filter((o) => o.status === "running").length;
  const queuedCount = running.filter((o) => o.status === "queued").length;

  const firstUnhealthy = detection?.managers.find(
    (m) => (snapshots[m.id]?.health.length ?? 0) > 0,
  );

  const refreshTimes = Object.values(snapshots)
    .map((s) => s?.refreshedAt)
    .filter((x): x is string => !!x)
    .sort();
  const lastRefresh = refreshTimes.length ? refreshTimes[refreshTimes.length - 1] : undefined;

  return (
    <footer className="flex h-7 shrink-0 items-center gap-4 border-t border-border bg-bg-surface px-4 text-[11px] text-text-muted">
      <UpdateStatusItem />
      <span>{lastRefresh ? `Last refresh ${lastRefresh}` : "No refresh yet"}</span>

      {firstUnhealthy && (
        <button
          type="button"
          onClick={() => navigate({ kind: "manager", managerId: firstUnhealthy.id })}
          className="flex items-center gap-1 text-warning hover:underline"
        >
          ⚠ Health issue in {firstUnhealthy.displayName}
        </button>
      )}

      <span className="ml-auto">
        {runningCount > 0 || queuedCount > 0
          ? `${runningCount} running · ${queuedCount} queued`
          : "Idle"}
      </span>

      <button
        type="button"
        aria-label="Open logs folder"
        title="Open logs folder"
        onClick={() => void revealLogsDir()}
        className="hover:text-text-secondary"
      >
        🗂
      </button>
      <button
        type="button"
        aria-label="Settings"
        title="Settings"
        onClick={() => navigate({ kind: "settings" })}
        className="hover:text-text-secondary"
      >
        ⚙
      </button>
    </footer>
  );
}
