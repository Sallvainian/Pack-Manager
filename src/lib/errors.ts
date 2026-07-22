/**
 * errors.ts — user-facing copy for the error taxonomy (SPEC §5.10).
 *
 * The backend already sets `IpcError.message` to a user-facing string, but the
 * frontend owns the *presentation*: a short taxonomy-mapped title for
 * `ErrorState`/`ManagerCard`, and a sensible fallback message per code when the
 * backend copy is missing. Every string states what happened and the next
 * action.
 */
import { isIpcError, type ErrorCode, type IpcError } from "./ipc/types";

/** Taxonomy-mapped short titles (SPEC §4.8 "taxonomy-mapped title"). */
export const ERROR_TITLES: Record<ErrorCode, string> = {
  tool_not_found: "Tool not found",
  spawn_failed: "Couldn't start command",
  timeout: "Timed out",
  non_zero_exit: "Command failed",
  brew_lock_busy: "Homebrew is busy",
  parse_failed: "Couldn't read output",
  cancelled: "Cancelled",
  self_update_unavailable: "Self-update unavailable",
  env_capture_failed: "Environment probe failed",
  io: "I/O error",
  internal: "Internal error",
};

/**
 * Fallback message per code (what happened + next action). Used only when the
 * backend `IpcError.message` is empty.
 */
export const ERROR_FALLBACKS: Record<ErrorCode, string> = {
  tool_not_found:
    "The command's binary wasn't found on the search path. Open Settings → Environment Report to check the resolved PATH.",
  spawn_failed: "The command could not be started. Check the log and retry.",
  timeout: "The command ran past its timeout. Check your network and retry.",
  non_zero_exit: "The command exited with an error. View the log for details.",
  brew_lock_busy: "Homebrew is busy in another terminal. Retry when it finishes.",
  parse_failed:
    "The command's output couldn't be parsed. The previous data is still shown; retry to try again.",
  cancelled: "The operation was cancelled.",
  self_update_unavailable: "This manager can't update itself in Pack-Manager.",
  env_capture_failed:
    "The login-shell PATH probe failed; a static fallback path is in use. See the Environment Report.",
  io: "A file-system error occurred. Check the log and retry.",
  internal: "Something went wrong. Check the log and retry.",
};

/** Copy shown when a network-dependent op degrades offline. */
export const OFFLINE_HINT = "You may be offline — this manager needs the network to check for updates.";

export interface ErrorCopy {
  title: string;
  message: string;
  /** Optional extra hint (e.g. offline). */
  hint?: string;
}

/** Map an `IpcError` to display copy. Prefers the backend message. */
export function errorCopy(err: IpcError): ErrorCopy {
  const title = ERROR_TITLES[err.code] ?? "Error";
  const message = err.message && err.message.trim().length > 0 ? err.message : ERROR_FALLBACKS[err.code];
  const hint = err.code === "timeout" ? OFFLINE_HINT : undefined;
  return { title, message, hint };
}

/**
 * Serialize a caught unknown value for logging. Tauri rejects a failed
 * `invoke()` with the serialized `IpcError` — a plain object, which
 * `String(e)` collapses to "[object Object]", destroying the diagnostic
 * payload. This preserves IpcError fields, Error stacks, and falls back to
 * JSON for anything else.
 */
export function describeError(e: unknown): string {
  if (isIpcError(e)) {
    const parts = [`${e.code}: ${e.message}`];
    if (e.detail) parts.push(`detail: ${e.detail}`);
    if (e.managerId) parts.push(`manager: ${e.managerId}`);
    if (e.opId) parts.push(`opId: ${e.opId}`);
    if (e.logPath) parts.push(`log: ${e.logPath}`);
    return parts.join(" — ");
  }
  if (e instanceof Error) {
    return e.stack ?? `${e.name}: ${e.message}`;
  }
  try {
    return JSON.stringify(e) ?? String(e);
  } catch {
    return String(e);
  }
}

/**
 * Up to `n` trailing lines of an error's `detail` (typically the stderr tail),
 * for the compact card/banner display.
 */
export function stderrTail(err: IpcError, n: number): string[] {
  if (!err.detail) return [];
  const lines = err.detail.split("\n").filter((l) => l.length > 0);
  return lines.slice(Math.max(0, lines.length - n));
}
