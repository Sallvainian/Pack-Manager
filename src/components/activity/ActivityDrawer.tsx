/**
 * ActivityDrawer — the operations surface (SPEC §4.9). Replaces the U6 stub.
 *
 * Collapsed: a 32px bar summarising running/queued ops. Expanded: OperationList
 * (280px) beside the LiveLogView for the focused op, sized to `drawerHeight` and
 * drag-resizable (25–60%).
 *
 * Because the drawer is always mounted in the shell, it is also the mount point
 * for the two app-level effects this unit owns: the global keyboard map
 * (`useKeyboard`) and the toast/auto-open flows (`useOperationEffects`).
 */
import { useKeyboard } from "../../hooks/useKeyboard";
import { useOperationsStore, type OperationsState } from "../../store/operations";
import { useUiStore } from "../../store/ui";
import { LiveLogView } from "./LiveLogView";
import { OperationList } from "./OperationList";
import { useOperationEffects } from "./useOperationEffects";

/** Count ops in a given status — a primitive selector, so `op:output` batches
 * (which touch only `logs`) never re-render the drawer or cascade to LiveLogView. */
function countByStatus(s: OperationsState, status: string): number {
  let n = 0;
  for (const id of s.order) if (s.byId[id]?.status === status) n += 1;
  return n;
}

export function ActivityDrawer() {
  useKeyboard();
  useOperationEffects();

  const open = useUiStore((s) => s.drawerOpen);
  const height = useUiStore((s) => s.drawerHeight);
  const setHeight = useUiStore((s) => s.setDrawerHeight);
  const toggle = useUiStore((s) => s.toggleDrawer);
  const focusedOpId = useUiStore((s) => s.focusedOpId);

  const running = useOperationsStore((s) => countByStatus(s, "running"));
  const queued = useOperationsStore((s) => countByStatus(s, "queued"));
  const firstOpId = useOperationsStore((s) => s.order[0] ?? null);

  const summary =
    running > 0 || queued > 0
      ? `${running} running${queued > 0 ? ` · ${queued} queued` : ""}`
      : "Idle";

  const effectiveFocus = focusedOpId ?? firstOpId;

  function onResizePointerDown(e: React.PointerEvent) {
    e.preventDefault();
    const parent = (e.currentTarget as HTMLElement).parentElement?.parentElement;
    if (!parent) return;
    const rect = parent.getBoundingClientRect();
    function onMove(ev: PointerEvent) {
      const fraction = (rect.bottom - ev.clientY) / rect.height;
      setHeight(fraction);
    }
    function onUp() {
      window.removeEventListener("pointermove", onMove);
      window.removeEventListener("pointerup", onUp);
    }
    window.addEventListener("pointermove", onMove);
    window.addEventListener("pointerup", onUp);
  }

  return (
    <div
      className="flex shrink-0 flex-col border-t border-border bg-bg-surface"
      style={open ? { height: `${Math.round(height * 100)}%` } : undefined}
    >
      {open && (
        <div
          role="separator"
          aria-label="Resize activity drawer"
          onPointerDown={onResizePointerDown}
          className="h-1 shrink-0 cursor-row-resize bg-transparent hover:bg-border-strong"
        />
      )}
      <button
        type="button"
        onClick={toggle}
        aria-expanded={open}
        aria-label={open ? "Collapse activity drawer" : "Expand activity drawer"}
        className="flex h-8 shrink-0 items-center gap-2 px-4 text-left text-[12px] text-text-secondary hover:bg-bg-raised"
      >
        <span
          className={[
            "h-1.5 w-1.5 rounded-full",
            running > 0 ? "bg-accent animate-pulse" : "bg-text-muted",
          ].join(" ")}
        />
        <span>{summary}</span>
        <span className="ml-auto">{open ? "▾" : "▴"}</span>
      </button>

      {open && (
        <div className="flex min-h-0 flex-1 border-t border-border">
          <OperationList />
          {effectiveFocus ? (
            // Keyed by op: switching focus remounts the log fresh (re-pinned to
            // the tail) instead of carrying the previous op's pin/scroll state.
            <LiveLogView key={effectiveFocus} opId={effectiveFocus} />
          ) : (
            <div className="flex min-w-0 flex-1 items-center justify-center bg-bg-inset text-[12px] text-text-muted">
              Select an operation to view its output.
            </div>
          )}
        </div>
      )}
    </div>
  );
}
