/**
 * QuitGuardDialog — shown when a quit is requested while operations are still
 * running (SPEC §4.10). It lists the in-flight operations and offers to cancel
 * them all (SIGTERM→SIGKILL per op) or keep them running.
 *
 * An update restart takes the same path (`reason: "update"`, DECISIONS D25):
 * relaunching kills exactly the same child processes, so it deserves exactly
 * the same confirmation — only the wording and the follow-up action differ.
 *
 * Native quit triggers are wired in Rust; this dialog owns the presentation,
 * confirmed destructive action, per-op quit cancellation, and dismissal.
 */
import {
  cancelOperation,
  confirmAppUpdate,
  confirmQuit,
  logFrontendEvent,
} from "../../lib/ipc/client";
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
  const pushToast = useUiStore((s) => s.pushToast);

  const resolveName = (id: ManagerId) => managerInfo(detection, id)?.displayName ?? id;
  const entries = opIds.map((id) => ({ id, op: byId[id] }));

  const updating = reason === "update";

  function cancelAll() {
    closeDialog();
    if (updating) {
      // The backend first reserves admission and attempts the fallible install.
      // Only success commits cancellation, drains, and restarts, so a failed
      // install does not destroy the work the user had in flight.
      void confirmAppUpdate().catch((e) => {
        const detail = describeError(e);
        pushToast({
          kind: "error",
          message: `Update restart failed: ${detail}`,
          persistent: true,
        });
        void logFrontendEvent("error", `update install failed: ${detail}`).catch(() => undefined);
      });
    } else {
      const cancellations = opIds.map(async (id) => {
        try {
          await cancelOperation(id);
        } catch (e) {
          // Persist the failure before a confirmed quit can end the process,
          // but logger failure must never prevent the explicit exit choice.
          await logFrontendEvent(
            "error",
            `operation cancel failed: ${describeError(e)}`,
          ).catch(() => undefined);
        }
      });
      // Wait for every cancellation request to settle, but a failed request
      // must not strand the app in a half-confirmed quit. The backend shutdown
      // hook repeats cancellation and performs the bounded process drain.
      void Promise.all(cancellations)
        .then(() => confirmQuit())
        .catch((e) => {
          void logFrontendEvent("error", `confirmed quit failed: ${describeError(e)}`).catch(
            () => undefined,
          );
        });
    }
  }

  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center bg-black/50 p-4" onClick={closeDialog}>
      <div
        role="alertdialog"
        aria-modal="true"
        aria-label="Operations are active"
        onClick={(e) => e.stopPropagation()}
        className="flex w-[440px] max-w-full flex-col gap-3 rounded-card border border-border-strong bg-bg-overlay p-5 shadow-2xl"
      >
        <h2 className="text-[15px] font-semibold text-text-primary">Operations are active</h2>
        <p className="text-[13px] text-text-secondary">
          {updating ? "Restarting to update" : "Quitting"} now will cancel {entries.length} active
          operation{entries.length === 1 ? "" : "s"}:
        </p>
        <ul className="flex flex-col gap-1 rounded-control border border-border bg-bg-inset px-3 py-2">
          {entries.map(({ id, op }) => (
            <li key={id} className="font-mono text-[12px] text-text-secondary">
              {op ? opTitle(op, resolveName) : `Operation · ${id}`}
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
