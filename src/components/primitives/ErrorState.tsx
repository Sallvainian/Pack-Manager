import { errorCopy, stderrTail } from "../../lib/errors";
import type { IpcError } from "../../lib/ipc/types";
import { Button } from "./Button";

export interface ErrorStateProps {
  error: IpcError;
  /** Max stderr lines to show inline (SPEC §4.8: ≤10). */
  maxLines?: number;
  onRetry?: () => void;
  onViewLog?: () => void;
  className?: string;
}

export function ErrorState({
  error,
  maxLines = 10,
  onRetry,
  onViewLog,
  className = "",
}: ErrorStateProps) {
  const copy = errorCopy(error);
  const lines = stderrTail(error, maxLines);
  return (
    <div
      role="alert"
      className={[
        "rounded-card border border-danger/40 border-l-4 border-l-danger bg-danger/8 p-4",
        className,
      ].join(" ")}
    >
      <div className="text-[15px] font-semibold text-text-primary">{copy.title}</div>
      <div className="mt-1 text-[13px] text-text-secondary">{copy.message}</div>
      {copy.hint && <div className="mt-1 text-[12px] text-text-muted">{copy.hint}</div>}
      {lines.length > 0 && (
        <pre className="mt-2 max-h-40 overflow-auto rounded-control bg-bg-inset px-3 py-2 font-mono text-[12px] leading-relaxed text-text-secondary">
          {lines.join("\n")}
        </pre>
      )}
      {(onRetry || onViewLog) && (
        <div className="mt-3 flex items-center gap-2">
          {onRetry && (
            <Button variant="secondary" size="sm" onClick={onRetry}>
              Retry
            </Button>
          )}
          {onViewLog && error.logPath && (
            <Button variant="ghost" size="sm" onClick={onViewLog}>
              View log
            </Button>
          )}
        </div>
      )}
    </div>
  );
}
