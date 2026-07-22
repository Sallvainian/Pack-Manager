/**
 * StalledOperationDialog — the no-output handoff (SPEC §F7/§D14). Raised by the
 * `op:stalled` event. Pack-Manager never enters passwords, so a silent command is
 * likely waiting for input it cannot provide: the dialog offers to keep waiting
 * (dismiss — the watchdog re-arms on the next line of output), copy the command
 * to run it in a real terminal, or cancel the operation.
 */
import { cancelOperation } from "../../lib/ipc/client";
import { useOperationsStore } from "../../store/operations";
import { useUiStore } from "../../store/ui";
import { Button } from "../primitives/Button";
import { CopyableCommand } from "../primitives/CopyableCommand";

export interface StalledOperationDialogProps {
  opId: string;
  silentForSecs: number;
}

/** "2m" when a whole number of minutes, otherwise "120s". */
function formatSilence(secs: number): string {
  if (secs >= 60 && secs % 60 === 0) return `${secs / 60}m`;
  if (secs >= 60) return `${Math.round((secs / 60) * 10) / 10}m`;
  return `${secs}s`;
}

export function StalledOperationDialog({ opId, silentForSecs }: StalledOperationDialogProps) {
  const command = useOperationsStore((s) => s.byId[opId]?.commandLine);
  const closeDialog = useUiStore((s) => s.closeDialog);

  function cancel() {
    void cancelOperation(opId);
    closeDialog();
  }

  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center bg-black/50 p-4" onClick={closeDialog}>
      <div
        role="alertdialog"
        aria-modal="true"
        aria-label="Operation stalled"
        onClick={(e) => e.stopPropagation()}
        className="flex w-[440px] max-w-full flex-col gap-3 rounded-card border border-border-strong bg-bg-overlay p-5 shadow-2xl"
      >
        <h2 className="flex items-center gap-2 text-[15px] font-semibold text-text-primary">
          <span aria-hidden="true" className="text-warning">
            ⚠
          </span>
          No output for {formatSilence(silentForSecs)}
        </h2>
        <p className="text-[13px] leading-relaxed text-text-secondary">
          This command may be waiting for input Pack-Manager cannot provide — it never enters
          passwords. You can run it yourself in a terminal, keep waiting, or cancel.
        </p>
        {command && <CopyableCommand command={command} className="w-full" />}
        <div className="mt-1 flex items-center justify-end gap-2">
          <Button variant="ghost" size="md" onClick={closeDialog}>
            Keep waiting
          </Button>
          <Button variant="danger" size="md" onClick={cancel}>
            Cancel operation
          </Button>
        </div>
      </div>
    </div>
  );
}
