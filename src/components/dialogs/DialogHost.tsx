/**
 * DialogHost — the single mount point for modal surfaces, routed off
 * `ui.dialog.kind` (SPEC §4.10). Replaces the U6 stub. One dialog is shown at a
 * time; `none` renders nothing.
 */
import { useUiStore } from "../../store/ui";
import { UpgradePlanSheet } from "./UpgradePlanSheet";
import { StalledOperationDialog } from "./StalledOperationDialog";
import { QuitGuardDialog } from "./QuitGuardDialog";

export function DialogHost() {
  const dialog = useUiStore((s) => s.dialog);

  switch (dialog.kind) {
    case "upgradePlan":
      return <UpgradePlanSheet plan={dialog.plan} />;
    case "stalled":
      return <StalledOperationDialog opId={dialog.opId} silentForSecs={dialog.silentForSecs} />;
    case "quitGuard":
      return <QuitGuardDialog opIds={dialog.opIds} />;
    case "none":
      return null;
  }
}
