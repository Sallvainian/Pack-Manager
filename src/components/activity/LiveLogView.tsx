/**
 * LiveLogView — the drawer's right pane (SPEC §4.9): a virtualized, batch-append
 * log for one operation.
 *
 * - Header strip: the exact command (mono, copyable) + "Reveal log file".
 * - Body: `bg-inset`, mono 12/1.6; stdout in `text-secondary`, stderr with an
 *   amber 2px left gutter; `\r` progress repaints collapse to their final segment.
 * - Auto-scroll is pinned to the tail; scrolling up unpins and shows a
 *   "Jump to latest ↓" chip that re-pins.
 * - The ring buffer caps at 5000 lines/op (store-enforced); when older lines were
 *   dropped a pinned banner reports how many now live only in the log file.
 *
 * Rendering is store-selector-scoped to `logs[opId]` so a single `op:output`
 * batch produces exactly one re-render (SPEC §4.9 / U8 acceptance).
 */
import { useEffect, useRef, useState } from "react";
import { useVirtualizer } from "@tanstack/react-virtual";
import { revealOperationLog } from "../../lib/ipc/client";
import type { LogLine } from "../../lib/ipc/types";
import { LOG_CAP, useOperationsStore } from "../../store/operations";
import { collapseCarriageReturns } from "./opDisplay";

/** Above this line count the body virtualizes (keeps small logs/tests simple). */
const VIRTUALIZE_ABOVE = 200;
/** Distance from the bottom (px) still counted as "pinned to latest". */
const PIN_THRESHOLD = 24;

export interface LiveLogViewProps {
  opId: string;
}

export function LiveLogView({ opId }: LiveLogViewProps) {
  const log = useOperationsStore((s) => s.logs[opId]);
  const commandLine = useOperationsStore((s) => s.byId[opId]?.commandLine);

  const lines = log?.lines ?? [];
  const overflow = log?.overflow ?? 0;

  const [pinned, setPinned] = useState(true);
  const scrollRef = useRef<HTMLDivElement>(null);
  const endRef = useRef<HTMLDivElement>(null);

  // Keep the tail in view while pinned. Reads length so appends re-run it; does
  // NOT set state, so an append is still one commit.
  useEffect(() => {
    if (pinned) endRef.current?.scrollIntoView({ block: "end" });
  }, [lines.length, pinned]);

  function onScroll() {
    const el = scrollRef.current;
    if (!el) return;
    const distance = el.scrollHeight - el.scrollTop - el.clientHeight;
    setPinned(distance <= PIN_THRESHOLD);
  }

  function jumpToLatest() {
    setPinned(true);
    endRef.current?.scrollIntoView({ block: "end" });
  }

  const virtualize = lines.length > VIRTUALIZE_ABOVE;

  return (
    <div className="relative flex min-w-0 flex-1 flex-col bg-bg-inset">
      {/* Command header */}
      <div className="flex items-center gap-2 border-b border-border bg-bg-surface px-3 py-2">
        <code className="min-w-0 flex-1 truncate font-mono text-[12px] text-text-secondary">
          {commandLine ?? "—"}
        </code>
        {commandLine && (
          <button
            type="button"
            title="Copy command"
            aria-label="Copy command"
            onClick={() => void navigator.clipboard?.writeText(commandLine)}
            className="shrink-0 text-[11px] text-text-muted hover:text-accent focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-accent"
          >
            Copy
          </button>
        )}
        <button
          type="button"
          onClick={() => void revealOperationLog(opId)}
          className="shrink-0 text-[11px] text-text-muted hover:text-accent focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-accent"
        >
          Reveal log file
        </button>
      </div>

      {/* Body */}
      <div
        ref={scrollRef}
        role="log"
        aria-label="Operation output"
        onScroll={onScroll}
        className="min-h-0 flex-1 overflow-auto px-3 py-2 font-mono text-[12px] leading-[1.6]"
      >
        {overflow > 0 && (
          <div className="mb-1 rounded-control bg-bg-raised px-2 py-1 text-[11px] text-text-muted">
            {overflow} earlier line{overflow === 1 ? "" : "s"} are in the log file (buffer capped at{" "}
            {LOG_CAP}).
          </div>
        )}
        {lines.length === 0 ? (
          <div className="text-text-muted">No output yet.</div>
        ) : virtualize ? (
          <VirtualLog scrollRef={scrollRef} lines={lines} />
        ) : (
          lines.map((line, i) => <LogRow key={i} line={line} />)
        )}
        <div ref={endRef} />
      </div>

      {/* Jump-to-latest chip (shown once auto-scroll is unpinned). */}
      {!pinned && (
        <button
          type="button"
          onClick={jumpToLatest}
          className="absolute bottom-3 left-1/2 -translate-x-1/2 rounded-full border border-border-strong bg-bg-overlay px-3 py-1 text-[11px] text-text-primary shadow-lg hover:brightness-125 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-accent"
        >
          Jump to latest ↓
        </button>
      )}
    </div>
  );
}

function LogRow({ line }: { line: LogLine }) {
  const text = collapseCarriageReturns(line.line);
  if (line.stream === "err") {
    return (
      <div className="whitespace-pre-wrap break-words border-l-2 border-warning pl-2 text-text-secondary">
        {text}
      </div>
    );
  }
  return <div className="whitespace-pre-wrap break-words text-text-secondary">{text}</div>;
}

interface VirtualLogProps {
  scrollRef: React.RefObject<HTMLDivElement | null>;
  lines: LogLine[];
}

/** Windowed body for long logs (SPEC §4.9 virtualized). */
function VirtualLog({ scrollRef, lines }: VirtualLogProps) {
  const virtualizer = useVirtualizer({
    count: lines.length,
    getScrollElement: () => scrollRef.current,
    estimateSize: () => 20,
    overscan: 24,
  });

  return (
    <div style={{ height: virtualizer.getTotalSize(), position: "relative", width: "100%" }}>
      {virtualizer.getVirtualItems().map((item) => (
        <div
          key={item.index}
          data-index={item.index}
          ref={virtualizer.measureElement}
          style={{ position: "absolute", top: 0, left: 0, width: "100%", transform: `translateY(${item.start}px)` }}
        >
          <LogRow line={lines[item.index]} />
        </div>
      ))}
    </div>
  );
}
