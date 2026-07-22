/**
 * OperationRow — one entry in the drawer's OperationList (SPEC §4.9): status
 * pill, title, live duration ticker, queue position, and a Cancel icon-button
 * (queued/running only). Presentational: the op comes in as a prop and the
 * store-wiring (focus, cancel) lives in OperationList.
 */
import { useEffect, useState } from "react";
import type { OpView } from "../../store/operations";
import { Chip } from "../primitives/Chip";
import { durationLabel, opTitle, statusMeta, type NameResolver } from "./opDisplay";

export interface OperationRowProps {
  op: OpView;
  focused: boolean;
  resolveName: NameResolver;
  onSelect: () => void;
  onCancel: () => void;
}

export function OperationRow({ op, focused, resolveName, onSelect, onCancel }: OperationRowProps) {
  // Live duration ticker while running; static once finished (SPEC §4.9).
  const [now, setNow] = useState(() => Date.now());
  useEffect(() => {
    if (op.status !== "running") return;
    const t = window.setInterval(() => setNow(Date.now()), 1000);
    return () => window.clearInterval(t);
  }, [op.status]);

  const meta = statusMeta(op.status);
  const active = op.status === "queued" || op.status === "running";
  const duration = durationLabel(op, now);
  const queueAhead = op.status === "queued" ? op.queuePosition ?? 0 : 0;

  return (
    <div
      role="listitem"
      onClick={onSelect}
      className={[
        "flex cursor-pointer flex-col gap-1 border-l-2 px-3 py-2 hover:bg-bg-raised",
        focused ? "border-l-accent bg-bg-raised" : "border-l-transparent",
      ].join(" ")}
    >
      <div className="flex items-center gap-2">
        <Chip tone={meta.tone} className={meta.pulse ? "animate-pulse" : ""}>
          {meta.label}
        </Chip>
        <span className="min-w-0 flex-1 truncate text-[13px] text-text-primary">
          {opTitle(op, resolveName)}
        </span>
        {active && (
          <button
            type="button"
            aria-label={`Cancel ${opTitle(op, resolveName)}`}
            title="Cancel operation"
            onClick={(e) => {
              e.stopPropagation();
              onCancel();
            }}
            className="shrink-0 rounded-control px-1.5 text-[13px] text-text-muted hover:text-danger focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-accent"
          >
            ✕
          </button>
        )}
      </div>
      <div className="flex items-center gap-2 text-[11px] text-text-muted">
        {duration && <span className="tabular-nums">{duration}</span>}
        {op.status === "queued" && (
          <span>{queueAhead > 0 ? `Waiting · ${queueAhead} ahead` : "Waiting"}</span>
        )}
        {op.phaseLabel && op.status === "running" && <span>{op.phaseLabel}</span>}
      </div>
    </div>
  );
}
