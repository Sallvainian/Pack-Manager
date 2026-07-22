/**
 * opDisplay — pure presentation helpers shared by the ActivityDrawer, the
 * operation list, and the toast/effect flows (SPEC §4.9/§4.10). No React, no
 * store access: everything is derived from an OperationRecord-shaped value plus a
 * name resolver, so it is trivially unit-testable and reused across views.
 */
import type { ChipTone } from "../primitives/Chip";
import type { ManagerId, OpKind, OpStatus } from "../../lib/ipc/types";
import type { OpView } from "../../store/operations";

/** Terminal (finished) operation statuses. */
export const TERMINAL_STATUSES: ReadonlySet<OpStatus> = new Set([
  "succeeded",
  "failed",
  "cancelled",
  "timedOut",
  "interrupted",
]);

export function isTerminal(status: OpStatus): boolean {
  return TERMINAL_STATUSES.has(status);
}

/** Resolves a manager id to its display name (falls back to the id). */
export type NameResolver = (id: ManagerId) => string;

export interface StatusMeta {
  label: string;
  tone: ChipTone;
  /** Running ops pulse their status dot (SPEC §4.9). */
  pulse: boolean;
}

/** Status pill label + tone; every state carries a text label (SPEC §4.11 a11y). */
export function statusMeta(status: OpStatus): StatusMeta {
  switch (status) {
    case "queued":
      return { label: "Queued", tone: "neutral", pulse: false };
    case "running":
      return { label: "Running", tone: "accent", pulse: true };
    case "succeeded":
      return { label: "Succeeded", tone: "success", pulse: false };
    case "failed":
      return { label: "Failed", tone: "danger", pulse: false };
    case "cancelled":
      return { label: "Cancelled", tone: "neutral", pulse: false };
    case "timedOut":
      return { label: "Timed out", tone: "danger", pulse: false };
    case "interrupted":
      return { label: "Interrupted", tone: "neutral", pulse: false };
  }
}

/** The verb form of an op kind, for toast/history copy. */
export function kindWord(kind: OpKind): string {
  switch (kind) {
    case "refresh":
      return "refresh";
    case "upgrade":
      return "upgrade";
    case "selfUpdate":
      return "self-update";
    case "healthFix":
      return "fix";
  }
}

/**
 * A one-line operation title (SPEC §4.9: "Upgrade 3 · npm",
 * "Self-update: mise · via brew").
 */
export function opTitle(op: OpView, name: NameResolver): string {
  const subject = name(op.subject);
  switch (op.kind) {
    case "refresh":
      return `Refresh · ${subject}`;
    case "upgrade": {
      const n = op.packageIds.length;
      return n > 0 ? `Upgrade ${n} · ${subject}` : `Upgrade · ${subject}`;
    }
    case "selfUpdate":
      return op.executor !== op.subject
        ? `Self-update: ${subject} · via ${name(op.executor)}`
        : `Self-update: ${subject}`;
    case "healthFix":
      return `Fix · ${subject}`;
  }
}

/** Human duration between two epoch-ms instants (SPEC §4.9 duration ticker). */
export function formatDuration(ms: number): string {
  const secs = Math.max(0, Math.floor(ms / 1000));
  if (secs < 60) return `${secs}s`;
  const mins = Math.floor(secs / 60);
  const rem = secs % 60;
  if (mins < 60) return rem ? `${mins}m ${rem}s` : `${mins}m`;
  const hrs = Math.floor(mins / 60);
  const remMin = mins % 60;
  return remMin ? `${hrs}h ${remMin}m` : `${hrs}h`;
}

/**
 * Elapsed/total duration label for an op. Running ops measure to `now`; finished
 * ops measure start→finish. Returns "" when the op has not started.
 */
export function durationLabel(op: OpView, now: number): string {
  if (!op.startedAt) return "";
  const start = Date.parse(op.startedAt);
  if (Number.isNaN(start)) return "";
  const end = op.finishedAt ? Date.parse(op.finishedAt) : now;
  if (Number.isNaN(end)) return "";
  return formatDuration(end - start);
}

/**
 * Collapse an in-place carriage-return repaint to its final segment (SPEC §4.9:
 * "`\r` repaints collapse in place"). A progress line like "50%\r100%" renders as
 * "100%"; lines without a `\r` are returned unchanged.
 */
export function collapseCarriageReturns(line: string): string {
  const i = line.lastIndexOf("\r");
  return i >= 0 ? line.slice(i + 1) : line;
}
