/**
 * STUB — replaced by U7 (Frontend package UX). Sequential handoff, not concurrent.
 *
 * U7 builds the full pane: SelfUpdateCard, HealthBanner, PackageToolbar,
 * virtualized PackageTable, SelectionToolbar, and the plan flow (SPEC §4.8).
 * This placeholder renders the header from detection so the shell is navigable.
 */
import type { ManagerId } from "../../lib/ipc/types";
import { managerInfo, useManagersStore } from "../../store/managers";
import { EmptyState } from "../primitives/EmptyState";

export interface ManagerPaneProps {
  managerId: ManagerId;
}

export function ManagerPane({ managerId }: ManagerPaneProps) {
  const detection = useManagersStore((s) => s.detection);
  const info = managerInfo(detection, managerId);

  return (
    <div className="flex h-full flex-col">
      <header className="flex items-center gap-3 border-b border-border px-6 py-4">
        <h1 className="text-[20px] font-semibold text-text-primary">
          {info?.displayName ?? managerId}
        </h1>
        {info?.version && (
          <span className="font-mono text-[12px] text-text-secondary">{info.version}</span>
        )}
      </header>
      <div className="flex-1 overflow-auto p-6">
        <EmptyState title="Package view" description="This pane is under construction." />
      </div>
    </div>
  );
}
