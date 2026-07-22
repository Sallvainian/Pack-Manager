/**
 * client.ts — typed wrappers for every IPC command (SPEC §5.9, plus the
 * in-app update trio per DECISIONS D25).
 *
 * Each Rust command takes a single parameter named `args` (see
 * `src-tauri/src/commands.rs`), so Tauri expects the invoke payload under the
 * `args` key: `invoke("refresh_manager", { args: { managerId } })`. These
 * wrappers are the only callers of {@link invoke}; UI code never touches the
 * bridge directly.
 */
import { invoke } from "./bridge";
import type {
  AppState,
  AppUpdateStatus,
  DetectionReport,
  DiagnosticsResult,
  ManagerId,
  OperationDetail,
  OperationRecord,
  OpIds,
  OpRef,
  PlanRequest,
  Settings,
  UpgradePlan,
} from "./types";

/** Detect installed managers; also serves as Re-detect (rebuilds ToolEnv). */
export function detectManagers(): Promise<DetectionReport> {
  return invoke<DetectionReport>("detect_managers");
}

/** Full state for rehydration on mount / dev reload. */
export function getState(): Promise<AppState> {
  return invoke<AppState>("get_state");
}

/** Refresh one manager; coalesces with an in-flight refresh for the same manager. */
export function refreshManager(managerId: ManagerId): Promise<OpRef> {
  return invoke<OpRef>("refresh_manager", { args: { managerId } });
}

/** Fan out one refresh op per present manager. */
export function refreshAll(): Promise<OpIds> {
  return invoke<OpIds>("refresh_all");
}

/** Issues the trust-device preview shown in the Upgrade Plan Sheet. */
export function buildUpgradePlan(args: PlanRequest): Promise<UpgradePlan> {
  return invoke<UpgradePlan>("build_upgrade_plan", { args });
}

/** Re-validates and enqueues the previewed plan. */
export function executePlan(plan: UpgradePlan): Promise<OpIds> {
  return invoke<OpIds>("execute_plan", { args: { plan } });
}

/** Route the manager's own update (errors `self_update_unavailable`). */
export function selfUpdateManager(managerId: ManagerId): Promise<OpRef> {
  return invoke<OpRef>("self_update_manager", { args: { managerId } });
}

/** Enqueue a health fix (e.g. `uv tool install <name> --reinstall`). */
export function runHealthFix(managerId: ManagerId, issueId: string): Promise<OpRef> {
  return invoke<OpRef>("run_health_fix", { args: { managerId, issueId } });
}

/** Cancel a running/queued operation (SIGTERM → 5s → SIGKILL). */
export function cancelOperation(opId: string): Promise<void> {
  return invoke<void>("cancel_operation", { args: { opId } });
}

/** Record + ring-buffer replay for one operation. */
export function getOperation(opId: string): Promise<OperationDetail> {
  return invoke<OperationDetail>("get_operation", { args: { opId } });
}

/** Session + journal records (Interrupted included). */
export function listOperations(limit: number): Promise<OperationRecord[]> {
  return invoke<OperationRecord[]>("list_operations", { args: { limit } });
}

export function getSettings(): Promise<Settings> {
  return invoke<Settings>("get_settings");
}

/** Patch-merge settings; returns the merged result. */
export function setSettings(patch: Partial<Settings>): Promise<Settings> {
  return invoke<Settings>("set_settings", { args: { patch } });
}

/** Reveal an operation's transcript in Finder (tauri-plugin-opener). */
export function revealOperationLog(opId: string): Promise<void> {
  return invoke<void>("reveal_operation_log", { args: { opId } });
}

/** Open the logs folder in Finder. */
export function revealLogsDir(): Promise<void> {
  return invoke<void>("reveal_logs_dir");
}

/** Build the diagnostics zip on the Desktop. */
export function exportDiagnostics(): Promise<DiagnosticsResult> {
  return invoke<DiagnosticsResult>("export_diagnostics");
}

/** Forward a frontend warning/error into the backend structured log. */
export function logFrontendEvent(level: "warn" | "error", message: string): Promise<void> {
  return invoke<void>("log_frontend_event", { args: { level, message } });
}

// ---------------------------------------------------------------------------
// In-app update (DECISIONS D25)
// ---------------------------------------------------------------------------

/** Current update state, for rehydration on mount. */
export function getAppUpdateState(): Promise<AppUpdateStatus> {
  return invoke<AppUpdateStatus>("get_app_update_state");
}

/**
 * Manual check. Resolves as soon as the check is under way — the outcome
 * arrives on `appUpdate:status`, so a slow network never blocks the caller.
 */
export function checkForAppUpdate(): Promise<void> {
  return invoke<void>("check_for_app_update");
}

/**
 * Install the downloaded update and relaunch. Never resolves on success (the
 * process restarts); cancel running operations before calling it.
 */
export function installAppUpdate(): Promise<void> {
  return invoke<void>("install_app_update");
}
