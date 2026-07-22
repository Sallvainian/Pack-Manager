/**
 * STUB — replaced by U8 (Frontend operations UX). Sequential handoff.
 *
 * U8 builds OperationList/OperationRow + virtualized LiveLogView with batch
 * append, pin/unpin, `\r` collapse, and the 5000-line cap banner (SPEC §4.9).
 * This placeholder renders the collapsed summary bar so the layout is complete.
 */
import { activeOps, useOperationsStore } from "../../store/operations";
import { useUiStore } from "../../store/ui";

export function ActivityDrawer() {
  const ops = useOperationsStore();
  const open = useUiStore((s) => s.drawerOpen);
  const toggle = useUiStore((s) => s.toggleDrawer);

  const active = activeOps(ops);
  const running = active.filter((o) => o.status === "running").length;
  const queued = active.filter((o) => o.status === "queued").length;
  const summary =
    running > 0 || queued > 0 ? `${running} running · ${queued} queued` : "Idle";

  return (
    <div className="shrink-0 border-t border-border bg-bg-surface">
      <button
        type="button"
        onClick={toggle}
        aria-expanded={open}
        className="flex h-8 w-full items-center gap-2 px-4 text-left text-[12px] text-text-secondary hover:bg-bg-raised"
      >
        <span className={["h-1.5 w-1.5 rounded-full", running > 0 ? "bg-accent animate-pulse" : "bg-text-muted"].join(" ")} />
        <span>{summary}</span>
        <span className="ml-auto">{open ? "▾" : "▴"}</span>
      </button>
      {open && (
        <div className="h-40 overflow-auto border-t border-border bg-bg-inset p-4 text-[12px] text-text-muted">
          Activity view is under construction.
        </div>
      )}
    </div>
  );
}
