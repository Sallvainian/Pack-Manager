import type { ManagerInfo } from "../../lib/ipc/types";
import { useManagerPhase } from "../../store";
import { outdatedCount, usePackagesStore } from "../../store/packages";
import { useUiStore } from "../../store/ui";

type Dot = "idle" | "active" | "error" | "health";

const DOT_CLASS: Record<Dot, string> = {
  idle: "bg-text-muted",
  active: "bg-accent animate-pulse",
  error: "bg-danger",
  health: "bg-warning",
};

const DOT_LABEL: Record<Dot, string> = {
  idle: "idle",
  active: "busy",
  error: "error",
  health: "has health issues",
};

export interface SidebarManagerItemProps {
  info: ManagerInfo;
}

export function SidebarManagerItem({ info }: SidebarManagerItemProps) {
  const snapshot = usePackagesStore((s) => s.snapshots[info.id]);
  const phase = useManagerPhase(info.id);
  const view = useUiStore((s) => s.view);
  const navigate = useUiStore((s) => s.navigate);

  const count = outdatedCount(snapshot);
  const healthCount = snapshot?.health.length ?? 0;
  const selected = view.kind === "manager" && view.managerId === info.id;

  let dot: Dot = "idle";
  if (phase === "error") dot = "error";
  else if (phase === "refreshing" || phase === "busy") dot = "active";
  else if (healthCount > 0) dot = "health";

  return (
    <button
      type="button"
      aria-current={selected ? "page" : undefined}
      onClick={() => navigate({ kind: "manager", managerId: info.id })}
      className={[
        "group relative flex w-full items-center gap-2.5 rounded-control px-2.5 py-1.5 text-left text-[13px]",
        "focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-accent",
        selected
          ? "bg-bg-raised text-text-primary before:absolute before:inset-y-1 before:left-0 before:w-0.5 before:rounded-full before:bg-accent"
          : "text-text-secondary hover:bg-bg-raised hover:text-text-primary",
      ].join(" ")}
    >
      <span
        aria-hidden="true"
        className="flex h-5 w-5 shrink-0 items-center justify-center rounded bg-bg-raised text-[11px] font-semibold text-text-muted"
      >
        {info.displayName.slice(0, 1).toUpperCase()}
      </span>
      <span className="flex-1 truncate">{info.displayName}</span>
      <span
        role="status"
        aria-label={`${info.displayName} ${DOT_LABEL[dot]}`}
        className={["h-1.5 w-1.5 shrink-0 rounded-full", DOT_CLASS[dot]].join(" ")}
      />
      {count > 0 && (
        <span className="min-w-[18px] rounded-full bg-warning/15 px-1.5 text-center text-[11px] font-medium text-warning">
          {count}
        </span>
      )}
    </button>
  );
}
