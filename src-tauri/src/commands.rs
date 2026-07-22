//! All 17 IPC command handlers (SPEC §5.9). U1 registers every command with
//! placeholder bodies (defaults where trivially constructible, `internal`
//! errors otherwise); U5 completes them against `state::AppState`.

use serde::Deserialize;

use crate::error::IpcError;
use crate::ipc::{
    AppState, DetectionReport, DiagnosticsResult, EnvInfo, ManagerId, OpIds, OpRef,
    OperationDetail, OperationRecord, PathSource, PlanRequest, UpgradePlan,
};
use crate::settings::{Settings, SettingsPatch};

fn not_implemented(what: &str) -> IpcError {
    IpcError::internal(format!("{what} is not implemented yet (U5)"))
}

fn placeholder_detection_report() -> DetectionReport {
    DetectionReport {
        managers: vec![],
        env: EnvInfo {
            path: String::new(),
            entries: vec![],
            source: PathSource::StaticFallback,
            home: String::new(),
        },
    }
}

// ---------------------------------------------------------------------------
// Argument wrappers (TS side invokes with `{ args: { … } }`)
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ManagerArgs {
    pub manager_id: ManagerId,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExecutePlanArgs {
    pub plan: UpgradePlan,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HealthFixArgs {
    pub manager_id: ManagerId,
    pub issue_id: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OpArgs {
    pub op_id: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListOperationsArgs {
    pub limit: u32,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetSettingsArgs {
    pub patch: SettingsPatch,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LogFrontendEventArgs {
    pub level: FrontendLogLevel,
    pub message: String,
}

#[derive(Debug, Clone, Copy, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum FrontendLogLevel {
    Warn,
    Error,
}

// ---------------------------------------------------------------------------
// Commands
// ---------------------------------------------------------------------------

/// Also serves as Re-detect; rebuilds ToolEnv.
#[tauri::command]
pub async fn detect_managers() -> Result<DetectionReport, IpcError> {
    Err(not_implemented("detect_managers"))
}

/// Rehydration on mount / dev reload.
#[tauri::command]
pub async fn get_state() -> Result<AppState, IpcError> {
    Ok(AppState {
        detection: placeholder_detection_report(),
        snapshots: vec![],
        operations: vec![],
        settings: Settings::default(),
    })
}

/// Coalesces duplicates.
#[tauri::command]
pub async fn refresh_manager(args: ManagerArgs) -> Result<OpRef, IpcError> {
    let _ = args;
    Err(not_implemented("refresh_manager"))
}

#[tauri::command]
pub async fn refresh_all() -> Result<OpIds, IpcError> {
    Err(not_implemented("refresh_all"))
}

/// PURE preview — the trust device.
#[tauri::command]
pub async fn build_upgrade_plan(args: PlanRequest) -> Result<UpgradePlan, IpcError> {
    let _ = args;
    Err(not_implemented("build_upgrade_plan"))
}

#[tauri::command]
pub async fn execute_plan(args: ExecutePlanArgs) -> Result<OpIds, IpcError> {
    let _ = args;
    Err(not_implemented("execute_plan"))
}

/// Errors with code `self_update_unavailable` when there is no route.
#[tauri::command]
pub async fn self_update_manager(args: ManagerArgs) -> Result<OpRef, IpcError> {
    let _ = args;
    Err(not_implemented("self_update_manager"))
}

#[tauri::command]
pub async fn run_health_fix(args: HealthFixArgs) -> Result<OpRef, IpcError> {
    let _ = args;
    Err(not_implemented("run_health_fix"))
}

#[tauri::command]
pub async fn cancel_operation(args: OpArgs) -> Result<(), IpcError> {
    let _ = args;
    Err(not_implemented("cancel_operation"))
}

/// Record + ring-buffer replay.
#[tauri::command]
pub async fn get_operation(args: OpArgs) -> Result<OperationDetail, IpcError> {
    let _ = args;
    Err(not_implemented("get_operation"))
}

/// Session + journal (Interrupted).
#[tauri::command]
pub async fn list_operations(args: ListOperationsArgs) -> Result<Vec<OperationRecord>, IpcError> {
    let _ = args;
    Ok(vec![])
}

#[tauri::command]
pub async fn get_settings() -> Result<Settings, IpcError> {
    Ok(Settings::default())
}

/// Placeholder: applies the patch to defaults without persisting. U5 wires
/// load/save through managed state (settings.rs already implements both).
#[tauri::command]
pub async fn set_settings(args: SetSettingsArgs) -> Result<Settings, IpcError> {
    let mut settings = Settings::default();
    settings.apply_patch(&args.patch);
    Ok(settings)
}

/// Via tauri-plugin-opener.
#[tauri::command]
pub async fn reveal_operation_log(args: OpArgs) -> Result<(), IpcError> {
    let _ = args;
    Err(not_implemented("reveal_operation_log"))
}

#[tauri::command]
pub async fn reveal_logs_dir() -> Result<(), IpcError> {
    Err(not_implemented("reveal_logs_dir"))
}

#[tauri::command]
pub async fn export_diagnostics() -> Result<DiagnosticsResult, IpcError> {
    Err(not_implemented("export_diagnostics"))
}

#[tauri::command]
pub async fn log_frontend_event(args: LogFrontendEventArgs) -> Result<(), IpcError> {
    // U5 routes this through tracing (target `pack_manager_lib::frontend`).
    let _ = args;
    Ok(())
}
