/**
 * QuitGuardDialog — shown when a quit is requested while operations are still
 * running (SPEC §4.10). It lists the in-flight operations and offers to cancel
 * them all (SIGTERM→SIGKILL per op) or keep them running.
 *
 * An update restart takes the same path (`reason: "update"`, DECISIONS D25):
 * relaunching kills exactly the same child processes, so it deserves exactly
 * the same confirmation — only the wording and the follow-up action differ.
 *
 * The window-close trigger (and the actual quit once operations are cancelled) is
 * host wiring outside this unit; this dialog owns the presentation, the per-op
 * cancellation, and dismissal.
 */
import { cancelOperation, installAppUpdate, logFrontendEvent } from "../../lib/ipc/client";
import { describeError } from "../../lib/errors";
import type { ManagerId } from "../../lib/ipc/types";
import { managerInfo, useManagersStore } from "../../store/managers";
import { useOperationsStore } from "../../store/operations";
import { useUiStore } from "../../store/ui";
import { Button } from "../primitives/Button";
import { opTitle } from "../activity/opDisplay";

export interface QuitGuardDialogProps {
  opIds: string[];
  /** `"quit"` (default) closes the app; `"update"` installs and relaunches. */
  reason?: "quit" | "update";
}

export function QuitGuardDialog({ opIds, reason = "quit" }: QuitGuardDialogProps) {
  const byId = useOperationsStore((s) => s.byId);
  const detection = useManagersStore((s) => s.detection);
  const closeDialog = useUiStore((s) => s.closeDialog);

  const resolveName = (id: ManagerId) => managerInfo(detection, id)?.displayName ?? id;
  const ops = opIds.map((id) => byId[id]).filter((o): o is NonNullable<typeof o> => !!o);

  const updating = reason === "update";

  function cancelAll() {
    for (const id of opIds) void cancelOperation(id);
    closeDialog();
    if (updating) {
      // Resolves only on failure — on success the process restarts.
      void installAppUpdate().catch(
        (e) => void logFrontendEvent("error", `update install failed: ${describeError(e)}`),
      );
    }
  }

  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center bg-black/50 p-4" onClick={closeDialog}>
      <div
        role="alertdialog"
        aria-modal="true"
        aria-label="Operations still running"
        onClick={(e) => e.stopPropagation()}
        className="flex w-[440px] max-w-full flex-col gap-3 rounded-card border border-border-strong bg-bg-overlay p-5 shadow-2xl"
      >
        <h2 className="text-[15px] font-semibold text-text-primary">Operations still running</h2>
        <p className="text-[13px] text-text-secondary">
          {updating ? "Restarting to update" : "Quitting"} now will cancel {ops.length} running
          operation{ops.length === 1 ? "" : "s"}:
        </p>
        <ul className="flex flex-col gap-1 rounded-control border border-border bg-bg-inset px-3 py-2">
          {ops.map((op) => (
            <li key={op.opId} className="font-mono text-[12px] text-text-secondary">
              {opTitle(op, resolveName)}
            </li>
          ))}
        </ul>
        <div className="mt-1 flex items-center justify-end gap-2">
          <Button variant="ghost" size="md" onClick={closeDialog}>
            Keep running
          </Button>
          <Button variant="danger" size="md" onClick={cancelAll}>
            {updating ? "Cancel operations and restart" : "Cancel operations and quit"}
          </Button>
        </div>
      </div>
    </div>
  );
}
