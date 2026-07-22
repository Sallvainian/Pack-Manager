/**
 * STUB — replaced by U8 (Frontend operations UX). Sequential handoff.
 *
 * U8 wires DialogHost to UpgradePlanSheet (built by U7), StalledOperationDialog,
 * and QuitGuardDialog off `ui.dialog.kind` (SPEC §4.10). This placeholder reads
 * the dialog state but renders nothing yet, so opening a dialog is a no-op crash-
 * free until U8 lands.
 */
import { useUiStore } from "../../store/ui";

export function DialogHost() {
  const dialog = useUiStore((s) => s.dialog);
  if (dialog.kind === "none") return null;
  // Real dialog rendering arrives with U8; nothing to show yet.
  return null;
}
