/**
 * PackageTable — the package list (SPEC §4.8). Virtualized beyond 100 rows via
 * @tanstack/react-virtual; rendered plainly below that so small snapshots (and
 * tests) exercise a simple path. The header checkbox is tri-state over the
 * *visible selectable* rows — never greedy casks, never pinned (those are not in
 * `orderedSelectable`). Greedy casks live in a collapsed, opt-in section.
 */
import { useRef, useState } from "react";
import { useVirtualizer } from "@tanstack/react-virtual";
import type { ManagerId, Package } from "../../lib/ipc/types";
import { PackageRow } from "./PackageRow";
import type { RowOpState } from "./StatusBadge";

/** Above this row count the body virtualizes (SPEC §4.8). */
const VIRTUALIZE_ABOVE = 100;

export interface PackageTableProps {
  managerId: ManagerId;
  /** Main rows (non-greedy), already filtered by search + outdated-only. */
  rows: Package[];
  /** Greedy-only casks, shown in the collapsed self-updating section. */
  greedyRows: Package[];
  selection: Set<string>;
  /** Selectable ids among `rows`, in display order (drives the header tri-state). */
  orderedSelectable: string[];
  opStateById: Record<string, RowOpState>;
  /** Cross-manager join target row to ring (SPEC §4.7). */
  highlightId?: string;
  onToggleSelect: (packageId: string) => void;
  onRangeSelect: (packageId: string) => void;
  onToggleAll: () => void;
  onUpgrade: (pkg: Package) => void;
}

export function PackageTable({
  managerId,
  rows,
  greedyRows,
  selection,
  orderedSelectable,
  opStateById,
  highlightId,
  onToggleSelect,
  onRangeSelect,
  onToggleAll,
  onUpgrade,
}: PackageTableProps) {
  const [expanded, setExpanded] = useState<Set<string>>(() => new Set());
  const [greedyOpen, setGreedyOpen] = useState(false);
  const scrollRef = useRef<HTMLDivElement>(null);

  function toggleExpand(id: string) {
    setExpanded((prev) => {
      const next = new Set(prev);
      if (next.has(id)) next.delete(id);
      else next.add(id);
      return next;
    });
  }

  const allSelected =
    orderedSelectable.length > 0 && orderedSelectable.every((id) => selection.has(id));
  const someSelected = orderedSelectable.some((id) => selection.has(id));
  const indeterminate = someSelected && !allSelected;

  function renderRow(pkg: Package) {
    return (
      <PackageRow
        key={pkg.id}
        pkg={pkg}
        managerId={managerId}
        selected={selection.has(pkg.id)}
        selectable={orderedSelectable.includes(pkg.id)}
        opState={opStateById[pkg.id]}
        highlighted={pkg.id === highlightId}
        expanded={expanded.has(pkg.id)}
        onToggleExpand={() => toggleExpand(pkg.id)}
        onToggleSelect={onToggleSelect}
        onRangeSelect={onRangeSelect}
        onUpgrade={onUpgrade}
      />
    );
  }

  return (
    <div role="table" aria-label={`${managerId} packages`} className="rounded-card border border-border">
      {/* Header */}
      <div
        role="row"
        className="flex items-center gap-2 rounded-t-card border-b border-border bg-bg-surface px-3 py-2 text-[11px] font-medium uppercase tracking-wide text-text-muted"
      >
        <div className="flex w-9 shrink-0 justify-center">
          <input
            type="checkbox"
            checked={allSelected}
            ref={(el) => {
              if (el) el.indeterminate = indeterminate && !allSelected;
            }}
            disabled={orderedSelectable.length === 0}
            onChange={onToggleAll}
            aria-label="Select all visible packages"
            className={[
              "h-4 w-4 shrink-0 cursor-pointer rounded-[4px] border border-border-strong bg-bg-raised accent-accent",
              "focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-accent focus-visible:ring-offset-1 focus-visible:ring-offset-bg-surface",
              "disabled:cursor-not-allowed disabled:opacity-40",
            ].join(" ")}
          />
        </div>
        <div className="min-w-0 flex-1">Name</div>
        <div className="w-[120px] shrink-0">Installed</div>
        <div className="w-[150px] shrink-0">Latest</div>
        <div className="w-[110px] shrink-0">Status</div>
        <div className="w-[90px] shrink-0" />
      </div>

      {/* Body */}
      {rows.length > VIRTUALIZE_ABOVE ? (
        <VirtualBody scrollRef={scrollRef} rows={rows} renderRow={renderRow} />
      ) : (
        <div>{rows.map(renderRow)}</div>
      )}

      {/* Self-updating (greedy) casks — collapsed, excluded from select-all. */}
      {greedyRows.length > 0 && (
        <div className="border-t border-border">
          <button
            type="button"
            onClick={() => setGreedyOpen((o) => !o)}
            aria-expanded={greedyOpen}
            className="flex w-full items-center gap-2 px-3 py-2 text-left text-[13px] text-text-secondary hover:bg-bg-raised focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-accent"
          >
            <span aria-hidden="true" className="w-3 text-[10px] text-text-muted">
              {greedyOpen ? "▼" : "▶"}
            </span>
            Self-updating casks ({greedyRows.length})
          </button>
          {greedyOpen && (
            <div>
              <div className="px-3 pb-2 text-[12px] text-text-muted">
                These casks update themselves and are excluded from Upgrade All unless opted in.
              </div>
              {greedyRows.map(renderRow)}
            </div>
          )}
        </div>
      )}
    </div>
  );
}

interface VirtualBodyProps {
  scrollRef: React.RefObject<HTMLDivElement | null>;
  rows: Package[];
  renderRow: (pkg: Package) => React.ReactNode;
}

/** Windowed body for large snapshots (SPEC §4.8). */
function VirtualBody({ scrollRef, rows, renderRow }: VirtualBodyProps) {
  const virtualizer = useVirtualizer({
    count: rows.length,
    getScrollElement: () => scrollRef.current,
    estimateSize: () => 44,
    overscan: 12,
  });

  return (
    <div ref={scrollRef} className="max-h-[60vh] overflow-auto">
      <div style={{ height: virtualizer.getTotalSize(), position: "relative", width: "100%" }}>
        {virtualizer.getVirtualItems().map((item) => (
          <div
            key={rows[item.index].id}
            data-index={item.index}
            ref={virtualizer.measureElement}
            style={{ position: "absolute", top: 0, left: 0, width: "100%", transform: `translateY(${item.start}px)` }}
          >
            {renderRow(rows[item.index])}
          </div>
        ))}
      </div>
    </div>
  );
}
