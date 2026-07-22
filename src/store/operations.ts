/**
 * operations store — operation records (by id + ordered) and per-op live-log
 * ring buffer (SPEC §5.11).
 *
 * The ring buffer caps each op at {@link LOG_CAP} lines; older lines are dropped
 * and counted in `overflow` so the LiveLogView (U8) can show the "earlier output
 * in log file" banner.
 */
import { create } from "zustand";
import type {
  LogLine,
  OperationRecord,
  OpStatusEvent,
} from "../lib/ipc/types";

/** In-memory cap per operation (SPEC §4.9 / §5.11). */
export const LOG_CAP = 5000;

/** A stored operation: the record plus transient UI fields from op:status. */
export interface OpView extends OperationRecord {
  phaseLabel?: string;
  queuePosition?: number;
}

export interface OpLog {
  lines: LogLine[];
  /** Count of lines dropped from the front of the ring buffer. */
  overflow: number;
}

export interface OperationsState {
  byId: Record<string, OpView>;
  /** opIds most-recent-first. */
  order: string[];
  logs: Record<string, OpLog>;

  setRecords: (records: OperationRecord[]) => void;
  applyStatus: (evt: OpStatusEvent) => void;
  appendOutput: (opId: string, batch: LogLine[]) => void;
  setLog: (opId: string, lines: LogLine[], truncated: boolean) => void;
  reset: () => void;
}

function initial(): Pick<OperationsState, "byId" | "order" | "logs"> {
  return { byId: {}, order: [], logs: {} };
}

/** Newest-first order: unknown ids prepended, known ids keep their slot. */
function withOrder(order: string[], opId: string): string[] {
  if (order.includes(opId)) return order;
  return [opId, ...order];
}

export const useOperationsStore = create<OperationsState>((set) => ({
  ...initial(),

  setRecords: (records) =>
    set(() => {
      const byId: Record<string, OpView> = {};
      const order: string[] = [];
      for (const r of records) {
        byId[r.opId] = r;
        order.push(r.opId);
      }
      return { byId, order };
    }),

  applyStatus: (evt) =>
    set((s) => {
      const prev = s.byId[evt.opId];
      // Fields the event does not carry (packageIds, queuedAt) are preserved
      // from the prior record; a first sighting seeds them from the event.
      const base: OpView = prev ?? {
        opId: evt.opId,
        kind: evt.kind,
        executor: evt.executor,
        subject: evt.subject,
        status: evt.status,
        commandLine: evt.commandLine,
        packageIds: [],
        queuedAt: evt.startedAt ?? new Date().toISOString(),
        startedAt: null,
        finishedAt: null,
        exitCode: null,
        error: null,
        logPath: evt.logPath,
      };
      const merged: OpView = {
        ...base,
        opId: evt.opId,
        kind: evt.kind,
        executor: evt.executor,
        subject: evt.subject,
        status: evt.status,
        commandLine: evt.commandLine,
        logPath: evt.logPath,
        phaseLabel: evt.phaseLabel,
        queuePosition: evt.queuePosition,
      };
      if (evt.startedAt !== undefined) merged.startedAt = evt.startedAt;
      if (evt.finishedAt !== undefined) merged.finishedAt = evt.finishedAt;
      if (evt.exitCode !== undefined) merged.exitCode = evt.exitCode;
      if (evt.error !== undefined) merged.error = evt.error;
      return {
        byId: { ...s.byId, [evt.opId]: merged },
        order: withOrder(s.order, evt.opId),
      };
    }),

  appendOutput: (opId, batch) =>
    set((s) => {
      const cur = s.logs[opId] ?? { lines: [], overflow: 0 };
      let lines = batch.length ? cur.lines.concat(batch) : cur.lines;
      let overflow = cur.overflow;
      if (lines.length > LOG_CAP) {
        const drop = lines.length - LOG_CAP;
        overflow += drop;
        lines = lines.slice(drop);
      }
      return { logs: { ...s.logs, [opId]: { lines, overflow } } };
    }),

  setLog: (opId, lines, truncated) =>
    set((s) => {
      const capped = lines.length > LOG_CAP ? lines.slice(lines.length - LOG_CAP) : lines;
      const overflow = truncated || lines.length > LOG_CAP ? lines.length - capped.length : 0;
      return { logs: { ...s.logs, [opId]: { lines: capped, overflow } } };
    }),

  reset: () => set(initial()),
}));

/** Ops that are queued or running, newest-first. */
export function activeOps(state: OperationsState): OpView[] {
  return state.order
    .map((id) => state.byId[id])
    .filter((o): o is OpView => !!o && (o.status === "queued" || o.status === "running"));
}
