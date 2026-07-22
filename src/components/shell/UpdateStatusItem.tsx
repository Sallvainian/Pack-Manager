/**
 * UpdateStatusItem — the bottom-left in-app-update indicator (DECISIONS D25).
 *
 * Invisible until it has something to say: it appears as a progress readout
 * while an update downloads, then turns into the button that installs it and
 * relaunches. Failed checks are NOT shown here — a manual check already toasts
 * (`onAppUpdate` in `lib/ipc/events.ts`) and an automatic one that fails
 * because the laptop is offline must not leave a permanent warning in the
 * chrome.
 */
import { installAppUpdate, logFrontendEvent } from "../../lib/ipc/client";
import { describeError } from "../../lib/errors";
import { downloadProgress, useAppUpdateStore } from "../../store/appUpdate";
import { activeOps, useOperationsStore } from "../../store/operations";
import { useUiStore } from "../../store/ui";
import { Spinner } from "../primitives/Spinner";

/** Compact control sized for the 28px status bar. */
const CHIP =
  "inline-flex items-center gap-1.5 rounded-control px-2 h-5 text-[11px] font-medium " +
  "focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-accent";

export function UpdateStatusItem() {
  const status = useAppUpdateStore((s) => s.status);
  const ops = useOperationsStore();
  const openDialog = useUiStore((s) => s.openDialog);

  const state = status?.state;
  if (!state) return null;

  function install() {
    // Restarting kills every child process, so an upgrade in flight gets the
    // same guard as a quit (SPEC §4.10 QuitGuardDialog).
    const running = activeOps(ops).map((o) => o.opId);
    if (running.length > 0) {
      openDialog({ kind: "quitGuard", opIds: running, reason: "update" });
      return;
    }
    // Resolves only on failure — on success the process restarts.
    void installAppUpdate().catch(
      (e) => void logFrontendEvent("error", `update install failed: ${describeError(e)}`),
    );
  }

  if (state.kind === "downloading") {
    const progress = downloadProgress(status);
    return (
      <span className="flex items-center gap-1.5 text-text-muted" data-testid="update-downloading">
        <Spinner size={10} label="Downloading update" />
        {progress === null
          ? `Downloading update ${state.version}…`
          : `Downloading update ${state.version} · ${Math.round(progress * 100)}%`}
      </span>
    );
  }

  if (state.kind === "readyToInstall") {
    return (
      <button
        type="button"
        onClick={install}
        title={state.notes ?? undefined}
        className={`${CHIP} bg-accent text-white hover:bg-accent-hover`}
      >
        ↑ Restart to update {state.version}
      </button>
    );
  }

  if (state.kind === "manualInstallRequired") {
    return (
      <span
        title={state.reason}
        className={`${CHIP} bg-warning/12 text-warning`}
        data-testid="update-manual-install"
      >
        ⚠ Update {state.version} needs a manual install
      </span>
    );
  }

  // idle / checking / upToDate / error — nothing worth a permanent chip.
  return null;
}
