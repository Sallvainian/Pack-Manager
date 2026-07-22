/**
 * HistoryView — session + journal operation history (SPEC §4.9, §F8). Replaces
 * the U6 stub.
 *
 * Reads the operations store (hydrated from `get_state`, which already folds in
 * the crash journal — start-without-finish records surface as `Interrupted`).
 * Filter bar (manager / status / search), a table (time · manager · kind ·
 * summary · duration · status · exit), an expandable row detail that lazily
 * fetches the transcript tail (`get_operation`) and reveals the log file, and an
 * Export-diagnostics footer.
 */
import { useState } from "react";
import { exportDiagnostics, getOperation, revealOperationLog } from "../../lib/ipc/client";
import type {
  LogLine,
  ManagerId,
  OperationRecord,
  OpStatus,
} from "../../lib/ipc/types";
import { MANAGER_IDS, OP_STATUSES } from "../../lib/ipc/types";
import { managerInfo, useManagersStore } from "../../store/managers";
import { useOperationsStore, type OpView } from "../../store/operations";
import { useUiStore } from "../../store/ui";
import { Button } from "../primitives/Button";
import { Chip } from "../primitives/Chip";
import { EmptyState } from "../primitives/EmptyState";
import { durationLabel, kindWord, opTitle, statusMeta, type NameResolver } from "../activity/opDisplay";

type ManagerFilter = ManagerId | "all";
type StatusFilter = OpStatus | "all";

function shortTime(iso: string | null): string {
  if (!iso) return "—";
  const t = Date.parse(iso);
  if (Number.isNaN(t)) return iso;
  const d = new Date(t);
  const pad = (n: number) => String(n).padStart(2, "0");
  return `${d.getFullYear()}-${pad(d.getMonth() + 1)}-${pad(d.getDate())} ${pad(d.getHours())}:${pad(
    d.getMinutes(),
  )}`;
}

export function HistoryView() {
  const order = useOperationsStore((s) => s.order);
  const byId = useOperationsStore((s) => s.byId);
  const detection = useManagersStore((s) => s.detection);
  const pushToast = useUiStore((s) => s.pushToast);

  const [managerFilter, setManagerFilter] = useState<ManagerFilter>("all");
  const [statusFilter, setStatusFilter] = useState<StatusFilter>("all");
  const [search, setSearch] = useState("");
  const [expanded, setExpanded] = useState<string | null>(null);

  const resolveName: NameResolver = (id) => managerInfo(detection, id)?.displayName ?? id;
  const q = search.trim().toLowerCase();

  const records: OpView[] = order
    .map((id) => byId[id])
    .filter((o): o is OpView => !!o)
    .filter((o) => managerFilter === "all" || o.subject === managerFilter || o.executor === managerFilter)
    .filter((o) => statusFilter === "all" || o.status === statusFilter)
    .filter(
      (o) =>
        !q ||
        o.commandLine.toLowerCase().includes(q) ||
        opTitle(o, resolveName).toLowerCase().includes(q),
    );

  async function exportZip() {
    try {
      const { zipPath } = await exportDiagnostics();
      pushToast({ kind: "success", message: `Diagnostics saved to ${zipPath}` });
    } catch {
      pushToast({ kind: "error", message: "Couldn't export diagnostics — check the log." });
    }
  }

  return (
    <div className="flex h-full flex-col">
      <header className="flex items-center gap-3 border-b border-border px-6 py-4">
        <h1 className="text-[20px] font-semibold text-text-primary">History</h1>
      </header>

      {/* Filter bar */}
      <div className="flex flex-wrap items-center gap-2 border-b border-border px-6 py-3">
        <select
          aria-label="Filter by manager"
          value={managerFilter}
          onChange={(e) => setManagerFilter(e.target.value as ManagerFilter)}
          className="h-8 rounded-control border border-border bg-bg-raised px-2 text-[13px] text-text-primary"
        >
          <option value="all">All managers</option>
          {MANAGER_IDS.map((id) => (
            <option key={id} value={id}>
              {resolveName(id)}
            </option>
          ))}
        </select>
        <select
          aria-label="Filter by status"
          value={statusFilter}
          onChange={(e) => setStatusFilter(e.target.value as StatusFilter)}
          className="h-8 rounded-control border border-border bg-bg-raised px-2 text-[13px] text-text-primary"
        >
          <option value="all">All statuses</option>
          {OP_STATUSES.map((s) => (
            <option key={s} value={s}>
              {statusMeta(s).label}
            </option>
          ))}
        </select>
        <input
          type="search"
          aria-label="Search history"
          placeholder="Search command or summary"
          value={search}
          onChange={(e) => setSearch(e.target.value)}
          className="h-8 w-60 rounded-control border border-border bg-bg-raised px-2.5 text-[13px] text-text-primary placeholder:text-text-muted"
        />
      </div>

      {/* Table */}
      <div className="flex-1 overflow-auto px-6 py-4">
        {records.length === 0 ? (
          <EmptyState title="No operations" description="Nothing matches the current filters." />
        ) : (
          <div role="table" aria-label="Operation history" className="rounded-card border border-border">
            <div
              role="row"
              className="flex items-center gap-3 border-b border-border bg-bg-surface px-3 py-2 text-[11px] font-medium uppercase tracking-wide text-text-muted"
            >
              <div className="w-[130px] shrink-0">Time</div>
              <div className="w-[90px] shrink-0">Manager</div>
              <div className="w-[80px] shrink-0">Kind</div>
              <div className="min-w-0 flex-1">Summary</div>
              <div className="w-[70px] shrink-0 text-right">Duration</div>
              <div className="w-[110px] shrink-0">Status</div>
              <div className="w-[60px] shrink-0 text-right">Exit</div>
            </div>
            {records.map((op) => (
              <HistoryRow
                key={op.opId}
                op={op}
                resolveName={resolveName}
                expanded={expanded === op.opId}
                onToggle={() => setExpanded((cur) => (cur === op.opId ? null : op.opId))}
              />
            ))}
          </div>
        )}
      </div>

      {/* Footer */}
      <footer className="flex items-center justify-end border-t border-border px-6 py-3">
        <Button variant="secondary" size="sm" onClick={() => void exportZip()}>
          Export diagnostics
        </Button>
      </footer>
    </div>
  );
}

interface HistoryRowProps {
  op: OperationRecord;
  resolveName: NameResolver;
  expanded: boolean;
  onToggle: () => void;
}

function HistoryRow({ op, resolveName, expanded, onToggle }: HistoryRowProps) {
  const meta = statusMeta(op.status);
  const [tail, setTail] = useState<LogLine[] | null>(null);
  const [loading, setLoading] = useState(false);

  async function loadDetail() {
    if (tail || loading) return;
    setLoading(true);
    try {
      const detail = await getOperation(op.opId);
      setTail(detail.lines.slice(-40));
    } catch {
      setTail([]);
    } finally {
      setLoading(false);
    }
  }

  function toggle() {
    onToggle();
    if (!expanded) void loadDetail();
  }

  return (
    <div className="border-b border-border last:border-b-0">
      <div
        role="row"
        onClick={toggle}
        className="flex cursor-pointer items-center gap-3 px-3 py-2 text-[13px] hover:bg-bg-raised"
      >
        <div className="w-[130px] shrink-0 tabular-nums text-text-secondary">
          {shortTime(op.startedAt ?? op.queuedAt)}
        </div>
        <div className="w-[90px] shrink-0 text-text-primary">{resolveName(op.subject)}</div>
        <div className="w-[80px] shrink-0 text-text-secondary">{kindWord(op.kind)}</div>
        <div className="min-w-0 flex-1 truncate font-mono text-[12px] text-text-secondary">
          {op.commandLine}
        </div>
        <div className="w-[70px] shrink-0 text-right tabular-nums text-text-muted">
          {durationLabel(op as OpView, Date.now()) || "—"}
        </div>
        <div className="w-[110px] shrink-0">
          <Chip tone={meta.tone}>{meta.label}</Chip>
        </div>
        <div className="w-[60px] shrink-0 text-right tabular-nums text-text-muted">
          {op.exitCode ?? "—"}
        </div>
      </div>

      {expanded && (
        <div className="flex flex-col gap-2 bg-bg-inset px-3 py-3">
          <div>
            <div className="text-[11px] font-medium uppercase tracking-wide text-text-muted">Command</div>
            <code className="mt-0.5 block overflow-x-auto whitespace-pre font-mono text-[12px] text-text-secondary">
              {op.commandLine}
            </code>
          </div>
          <div>
            <div className="text-[11px] font-medium uppercase tracking-wide text-text-muted">
              Transcript tail
            </div>
            {loading ? (
              <div className="text-[12px] text-text-muted">Loading…</div>
            ) : tail && tail.length > 0 ? (
              <pre className="mt-0.5 max-h-48 overflow-auto rounded-control bg-bg-base px-2 py-1.5 font-mono text-[12px] leading-[1.6] text-text-secondary">
                {tail.map((l) => l.line).join("\n")}
              </pre>
            ) : (
              <div className="text-[12px] text-text-muted">No transcript lines.</div>
            )}
          </div>
          <div>
            <Button variant="ghost" size="sm" onClick={() => void revealOperationLog(op.opId)}>
              Reveal in Finder
            </Button>
          </div>
        </div>
      )}
    </div>
  );
}
