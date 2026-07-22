/**
 * STUB — replaced by U8 (Frontend operations UX). Sequential handoff.
 *
 * U8 builds the filter bar, records table, row detail (transcript tail + Reveal
 * in Finder), Interrupted rendering, and the Export diagnostics footer
 * (SPEC §4.9). This placeholder renders the header so the shell is navigable.
 */
import { EmptyState } from "../primitives/EmptyState";

export function HistoryView() {
  return (
    <div className="flex h-full flex-col">
      <header className="flex items-center gap-3 border-b border-border px-6 py-4">
        <h1 className="text-[20px] font-semibold text-text-primary">History</h1>
      </header>
      <div className="flex-1 overflow-auto p-6">
        <EmptyState title="Operation history" description="This view is under construction." />
      </div>
    </div>
  );
}
