/**
 * OperationList — the drawer's left column (SPEC §4.9, 280px). Lists session
 * operations newest-first and wires row selection (drives the LiveLogView) and
 * per-op cancellation.
 */
import { cancelOperation } from "../../lib/ipc/client";
import type { ManagerId } from "../../lib/ipc/types";
import { managerInfo, useManagersStore } from "../../store/managers";
import { useOperationsStore } from "../../store/operations";
import { useUiStore } from "../../store/ui";
import { EmptyState } from "../primitives/EmptyState";
import { OperationRow } from "./OperationRow";

export function OperationList() {
  const order = useOperationsStore((s) => s.order);
  const byId = useOperationsStore((s) => s.byId);
  const detection = useManagersStore((s) => s.detection);
  const focusedOpId = useUiStore((s) => s.focusedOpId);
  const setFocusedOp = useUiStore((s) => s.setFocusedOp);

  const resolveName = (id: ManagerId) => managerInfo(detection, id)?.displayName ?? id;
  const ops = order.map((id) => byId[id]).filter((o): o is NonNullable<typeof o> => !!o);
  const effectiveFocus = focusedOpId ?? ops[0]?.opId ?? null;

  if (ops.length === 0) {
    return (
      <div className="flex w-[280px] shrink-0 items-center justify-center border-r border-border bg-bg-surface">
        <EmptyState title="No operations yet" description="Refreshes and upgrades appear here." />
      </div>
    );
  }

  return (
    <div
      role="list"
      aria-label="Operations"
      className="w-[280px] shrink-0 overflow-auto border-r border-border bg-bg-surface"
    >
      {ops.map((op) => (
        <OperationRow
          key={op.opId}
          op={op}
          focused={op.opId === effectiveFocus}
          resolveName={resolveName}
          onSelect={() => setFocusedOp(op.opId)}
          onCancel={() => void cancelOperation(op.opId)}
        />
      ))}
    </div>
  );
}
