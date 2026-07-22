import type { ManagerId, ManagerInfo } from "../../lib/ipc/types";
import { managerInfo, useManagersStore } from "../../store/managers";
import { useUiStore } from "../../store/ui";
import { Chip } from "../primitives/Chip";
import { Tooltip } from "../primitives/Tooltip";

/** Row id under which a subject manager appears in its executor's package list. */
function joinRowId(executor: ManagerId, subject: ManagerId): string {
  switch (executor) {
    case "brew":
      return `formula:${subject}`;
    case "rustup":
      return `toolchain:${subject}`;
    default:
      return `tool:${subject}`;
  }
}

export interface ManagedByChipProps {
  info: ManagerInfo;
}

/**
 * Shows who manages this manager. `via <executor>` chips are clickable: they
 * navigate to the executor's pane and highlight the subject's row (the
 * cross-manager join, SPEC §4.7). `standalone` chips are inert.
 */
export function ManagedByChip({ info }: ManagedByChipProps) {
  const detection = useManagersStore((s) => s.detection);
  const navigate = useUiStore((s) => s.navigate);
  const setHighlight = useUiStore((s) => s.setHighlight);

  const executorId =
    info.managedBy === "standalone" || info.managedBy === info.id
      ? null
      : (info.managedBy as ManagerId);
  const executor = executorId ? managerInfo(detection, executorId) : undefined;
  const evidence = info.evidence ?? "";

  if (!executorId || !executor || executor.status !== "present") {
    return (
      <Tooltip content={evidence || "Not managed by another tool"}>
        <Chip tone="neutral">standalone</Chip>
      </Tooltip>
    );
  }

  function onClick() {
    navigate({ kind: "manager", managerId: executorId! });
    setHighlight({ managerId: executorId!, packageId: joinRowId(executorId!, info.id) });
  }

  return (
    <Tooltip content={evidence || `managed by ${executor.displayName}`}>
      <Chip tone="accent" onClick={onClick} title={`Go to ${executor.displayName}`}>
        via {executor.displayName}
      </Chip>
    </Tooltip>
  );
}
