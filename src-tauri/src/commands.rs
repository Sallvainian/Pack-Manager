//! All 17 IPC command handlers (SPEC §5.9) — completed by U5 against
//! `state::AppState`. Handlers stay thin: the logic lives in `queue.rs` /
//! `journal.rs` / `diagnostics.rs`, all unit-tested there.

use std::collections::BTreeSet;
use std::path::PathBuf;

use serde::Deserialize;
use tauri::State;

use crate::detect::{self, DetectStatus};
use crate::error::{IpcError, PmError};
use crate::events::AppEvent;
use crate::ipc::{
    AppState as AppStateWire, DetectionReport, DiagnosticsResult, ManagerId, OpIds, OpRef,
    OperationDetail, OperationRecord, PlanRequest, SelfUpdateRoute, UpgradePlan,
};
use crate::paths::ToolEnv;
use crate::queue::{self, PlanSources};
use crate::settings::{Settings, SettingsPatch};
use crate::state::AppState;

fn placeholder_detection_report(env: &ToolEnv) -> DetectionReport {
    DetectionReport {
        managers: vec![],
        env: env.env_info(),
    }
}

/// Session + journal records, newest first (journal rows the session already
/// knows are dropped by opId).
fn merged_records(state: &AppState) -> Vec<OperationRecord> {
    let mut records = state.queue.records();
    let session_ids: BTreeSet<String> = records.iter().map(|r| r.op_id.clone()).collect();
    {
        let journal = state
            .journal_records
            .read()
            .expect("journal records poisoned");
        records.extend(
            journal
                .iter()
                .filter(|r| !session_ids.contains(&r.op_id))
                .cloned(),
        );
    }
    records.sort_by(|a, b| b.queued_at.cmp(&a.queued_at));
    records
}

fn detection_not_ready() -> IpcError {
    IpcError::internal("detection has not completed yet")
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

/// Also serves as Re-detect; rebuilds ToolEnv (SPEC §5.9).
#[tauri::command]
pub async fn detect_managers(state: State<'_, AppState>) -> Result<DetectionReport, IpcError> {
    let env = ToolEnv::build().await;
    let outcome = detect::detect_all(&env, state.runner.as_ref()).await;
    *state.tool_env.write().expect("tool_env poisoned") = env;
    state.registry.set_routes_from(&outcome.report);
    let report = outcome.report.clone();
    *state.detection.write().expect("detection poisoned") = Some(outcome);
    state.sink.emit(AppEvent::DetectionUpdated(report.clone()));
    Ok(report)
}

/// Rehydration on mount / dev reload.
#[tauri::command]
pub async fn get_state(state: State<'_, AppState>) -> Result<AppStateWire, IpcError> {
    let detection = {
        let det = state.detection.read().expect("detection poisoned");
        match det.as_ref() {
            Some(outcome) => outcome.report.clone(),
            None => {
                let env = state.tool_env.read().expect("tool_env poisoned");
                placeholder_detection_report(&env)
            }
        }
    };
    Ok(AppStateWire {
        detection,
        snapshots: state.registry.all(),
        operations: merged_records(&state),
        settings: state.settings.read().expect("settings poisoned").clone(),
    })
}

/// Coalesces duplicates (same-manager refresh → existing opId).
#[tauri::command]
pub async fn refresh_manager(
    state: State<'_, AppState>,
    args: ManagerArgs,
) -> Result<OpRef, IpcError> {
    let sub = {
        let det = state.detection.read().expect("detection poisoned");
        let outcome = det.as_ref().ok_or_else(detection_not_ready)?;
        let status = outcome
            .statuses
            .get(&args.manager_id)
            .cloned()
            .unwrap_or(DetectStatus::Absent {
                reason: format!("{} was not detected", args.manager_id),
            });
        let settings = state.settings.read().expect("settings poisoned").clone();
        let env = state.tool_env.read().expect("tool_env poisoned").clone();
        queue::make_refresh_submission(args.manager_id, &status, &settings, &env)
    };
    let Some(sub) = sub else {
        return Err(IpcError::from(PmError::ToolNotFound {
            tool: args.manager_id.as_str().to_string(),
            searched: vec![],
        })
        .with_manager(args.manager_id));
    };
    let op_id = state.queue.submit(sub).await.map_err(IpcError::from)?;
    Ok(OpRef { op_id })
}

/// Fan out one refresh op per present manager (SPEC F2).
#[tauri::command]
pub async fn refresh_all(state: State<'_, AppState>) -> Result<OpIds, IpcError> {
    let subs: Vec<_> = {
        let det = state.detection.read().expect("detection poisoned");
        let outcome = det.as_ref().ok_or_else(detection_not_ready)?;
        let settings = state.settings.read().expect("settings poisoned").clone();
        let env = state.tool_env.read().expect("tool_env poisoned").clone();
        outcome
            .statuses
            .iter()
            .filter_map(|(id, status)| {
                queue::make_refresh_submission(*id, status, &settings, &env)
            })
            .collect()
    };
    let mut op_ids = Vec::with_capacity(subs.len());
    for sub in subs {
        op_ids.push(state.queue.submit(sub).await.map_err(IpcError::from)?);
    }
    Ok(OpIds { op_ids })
}

/// PURE preview — the trust device (SPEC F4).
#[tauri::command]
pub async fn build_upgrade_plan(
    state: State<'_, AppState>,
    args: PlanRequest,
) -> Result<UpgradePlan, IpcError> {
    let det = state.detection.read().expect("detection poisoned");
    let outcome = det.as_ref().ok_or_else(detection_not_ready)?;
    let snapshots = state.registry.all();
    let records = state.queue.records();
    let busy = queue::busy_package_ids(&records);
    let stale = queue::stale_managers(&records);
    Ok(queue::build_upgrade_plan(
        &args,
        &PlanSources {
            report: &outcome.report,
            snapshots: &snapshots,
            busy: &busy,
            stale: &stale,
        },
    ))
}

/// Re-validates and enqueues the previewed plan. Commands are re-derived from
/// the same pure adapters that built the preview — byte-identical argv by
/// construction; nothing runs that was not shown.
#[tauri::command]
pub async fn execute_plan(
    state: State<'_, AppState>,
    args: ExecutePlanArgs,
) -> Result<OpIds, IpcError> {
    let mut subs = Vec::new();
    {
        let det = state.detection.read().expect("detection poisoned");
        let outcome = det.as_ref().ok_or_else(detection_not_ready)?;
        let settings = state.settings.read().expect("settings poisoned").clone();
        let env = state.tool_env.read().expect("tool_env poisoned").clone();
        for group in &args.plan.groups {
            if group.self_update {
                let info = outcome
                    .report
                    .managers
                    .iter()
                    .find(|m| m.id == group.subject);
                let Some(info) = info else { continue };
                match &info.self_update {
                    SelfUpdateRoute::ViaRefresh { .. } => {
                        if let Some(status) = outcome.statuses.get(&group.subject) {
                            if let Some(sub) = queue::make_refresh_submission(
                                group.subject,
                                status,
                                &settings,
                                &env,
                            ) {
                                subs.push(sub);
                            }
                        }
                    }
                    SelfUpdateRoute::Unavailable { reason } => {
                        tracing::warn!(
                            manager = %group.subject,
                            %reason,
                            "self-update no longer available; skipped"
                        );
                    }
                    route => {
                        let sub = queue::make_self_update_submission(
                            group.subject,
                            route,
                            &outcome.statuses,
                            &settings,
                            &env,
                        )
                        .map_err(IpcError::from)?;
                        subs.push(sub);
                    }
                }
            } else {
                let status = outcome
                    .statuses
                    .get(&group.executor)
                    .cloned()
                    .unwrap_or(DetectStatus::Absent {
                        reason: format!("{} was not detected", group.executor),
                    });
                // Greedy casks reached the group only when opted in.
                let include_greedy = group
                    .package_ids
                    .iter()
                    .any(|id| id.starts_with("caskGreedy:"));
                let sub = queue::make_upgrade_submission(
                    group.executor,
                    &group.package_ids,
                    include_greedy,
                    &status,
                    &settings,
                    &env,
                )
                .map_err(IpcError::from)?;
                subs.push(sub);
            }
        }
    }
    let mut op_ids = Vec::with_capacity(subs.len());
    for sub in subs {
        op_ids.push(state.queue.submit(sub).await.map_err(IpcError::from)?);
    }
    Ok(OpIds { op_ids })
}

/// Errors with code `self_update_unavailable` when there is no route.
#[tauri::command]
pub async fn self_update_manager(
    state: State<'_, AppState>,
    args: ManagerArgs,
) -> Result<OpRef, IpcError> {
    let sub = {
        let det = state.detection.read().expect("detection poisoned");
        let outcome = det.as_ref().ok_or_else(detection_not_ready)?;
        let info = outcome
            .report
            .managers
            .iter()
            .find(|m| m.id == args.manager_id)
            .ok_or_else(|| {
                IpcError::from(PmError::SelfUpdateUnavailable {
                    reason: format!("{} was not detected", args.manager_id),
                })
            })?;
        let settings = state.settings.read().expect("settings poisoned").clone();
        let env = state.tool_env.read().expect("tool_env poisoned").clone();
        match &info.self_update {
            // brew: `brew update` IS the self-update — enqueue a refresh.
            SelfUpdateRoute::ViaRefresh { .. } => {
                let status = outcome
                    .statuses
                    .get(&args.manager_id)
                    .cloned()
                    .unwrap_or(DetectStatus::Absent {
                        reason: "not detected".into(),
                    });
                queue::make_refresh_submission(args.manager_id, &status, &settings, &env)
                    .ok_or_else(|| {
                        IpcError::from(PmError::SelfUpdateUnavailable {
                            reason: format!("{} is not installed", args.manager_id),
                        })
                    })?
            }
            route => queue::make_self_update_submission(
                args.manager_id,
                route,
                &outcome.statuses,
                &settings,
                &env,
            )
            .map_err(IpcError::from)?,
        }
    };
    let op_id = state.queue.submit(sub).await.map_err(IpcError::from)?;
    Ok(OpRef { op_id })
}

/// Enqueues the issue's fix command on the manager's own lane (F13).
#[tauri::command]
pub async fn run_health_fix(
    state: State<'_, AppState>,
    args: HealthFixArgs,
) -> Result<OpRef, IpcError> {
    let sub = {
        let snapshot = state.registry.get(args.manager_id).ok_or_else(|| {
            IpcError::internal(format!("no snapshot for {}", args.manager_id))
        })?;
        let issue = snapshot
            .health
            .iter()
            .find(|h| h.id == args.issue_id)
            .ok_or_else(|| {
                IpcError::internal(format!("unknown health issue {}", args.issue_id))
            })?
            .clone();
        let det = state.detection.read().expect("detection poisoned");
        let outcome = det.as_ref().ok_or_else(detection_not_ready)?;
        let status = outcome
            .statuses
            .get(&args.manager_id)
            .cloned()
            .unwrap_or(DetectStatus::Absent {
                reason: "not detected".into(),
            });
        let settings = state.settings.read().expect("settings poisoned").clone();
        let env = state.tool_env.read().expect("tool_env poisoned").clone();
        queue::make_health_fix_submission(args.manager_id, &issue, &status, &settings, &env)
            .map_err(IpcError::from)?
    };
    let op_id = state.queue.submit(sub).await.map_err(IpcError::from)?;
    Ok(OpRef { op_id })
}

#[tauri::command]
pub async fn cancel_operation(state: State<'_, AppState>, args: OpArgs) -> Result<(), IpcError> {
    state.queue.cancel(&args.op_id);
    Ok(())
}

/// Record + ring-buffer replay (cap 5000).
#[tauri::command]
pub async fn get_operation(
    state: State<'_, AppState>,
    args: OpArgs,
) -> Result<OperationDetail, IpcError> {
    let record = state.queue.record(&args.op_id).or_else(|| {
        state
            .journal_records
            .read()
            .expect("journal records poisoned")
            .iter()
            .find(|r| r.op_id == args.op_id)
            .cloned()
    });
    let record =
        record.ok_or_else(|| IpcError::internal(format!("unknown operation {}", args.op_id)))?;
    let (lines, truncated) = state.queue.lines(&args.op_id);
    Ok(OperationDetail {
        record,
        lines,
        truncated,
    })
}

/// Session + journal (Interrupted), newest first.
#[tauri::command]
pub async fn list_operations(
    state: State<'_, AppState>,
    args: ListOperationsArgs,
) -> Result<Vec<OperationRecord>, IpcError> {
    let mut records = merged_records(&state);
    records.truncate(args.limit as usize);
    Ok(records)
}

#[tauri::command]
pub async fn get_settings(state: State<'_, AppState>) -> Result<Settings, IpcError> {
    Ok(state.settings.read().expect("settings poisoned").clone())
}

/// Patch-merge, persist, apply the log level live (SPEC §6.3).
#[tauri::command]
pub async fn set_settings(
    state: State<'_, AppState>,
    args: SetSettingsArgs,
) -> Result<Settings, IpcError> {
    let merged = {
        let mut settings = state.settings.write().expect("settings poisoned");
        settings.apply_patch(&args.patch);
        settings.clone()
    };
    merged
        .save_to(&state.settings_path)
        .map_err(IpcError::from)?;
    if args.patch.log_level.is_some() {
        if let Some(handle) = state.logging.lock().expect("logging poisoned").as_ref() {
            let applied = handle.apply_settings_level(merged.log_level);
            tracing::info!(applied, level = ?merged.log_level, "log level change");
        }
    }
    tracing::info!("settings updated");
    Ok(merged)
}

/// Reveal an operation's transcript in Finder (tauri-plugin-opener).
#[tauri::command]
pub async fn reveal_operation_log(
    state: State<'_, AppState>,
    args: OpArgs,
) -> Result<(), IpcError> {
    let record = state
        .queue
        .record(&args.op_id)
        .or_else(|| {
            state
                .journal_records
                .read()
                .expect("journal records poisoned")
                .iter()
                .find(|r| r.op_id == args.op_id)
                .cloned()
        })
        .ok_or_else(|| IpcError::internal(format!("unknown operation {}", args.op_id)))?;
    tauri_plugin_opener::reveal_item_in_dir(PathBuf::from(&record.log_path))
        .map_err(|e| IpcError::from(PmError::Io {
            detail: e.to_string(),
        }))
}

#[tauri::command]
pub async fn reveal_logs_dir() -> Result<(), IpcError> {
    tauri_plugin_opener::open_path(crate::logging::logs_dir(), None::<&str>).map_err(|e| {
        IpcError::from(PmError::Io {
            detail: e.to_string(),
        })
    })
}

#[tauri::command]
pub async fn export_diagnostics(
    state: State<'_, AppState>,
) -> Result<DiagnosticsResult, IpcError> {
    let report = {
        let env = state.tool_env.read().expect("tool_env poisoned").env_info();
        let detection = state
            .detection
            .read()
            .expect("detection poisoned")
            .as_ref()
            .map(|o| o.report.clone());
        let settings = state.settings.read().expect("settings poisoned").clone();
        let log_directive = crate::logging::resolve_directive(
            std::env::var(crate::logging::ENV_VAR).ok().as_deref(),
            settings.log_level,
        )
        .0;
        crate::diagnostics::DiagnosticsReport {
            app_version: env!("CARGO_PKG_VERSION").to_string(),
            os: std::env::consts::OS.to_string(),
            arch: std::env::consts::ARCH.to_string(),
            env,
            detection,
            settings,
            log_directive,
        }
    };
    let zip_path = crate::diagnostics::export_default(&report).map_err(IpcError::from)?;
    Ok(DiagnosticsResult {
        zip_path: zip_path.to_string_lossy().into_owned(),
    })
}

/// Forwards a frontend warning/error into the structured log (target
/// `pack_manager_lib::frontend` — SPEC §6.3).
#[tauri::command]
pub async fn log_frontend_event(args: LogFrontendEventArgs) -> Result<(), IpcError> {
    match args.level {
        FrontendLogLevel::Warn => {
            tracing::warn!(target: "pack_manager_lib::frontend", message = %args.message, "frontend event");
        }
        FrontendLogLevel::Error => {
            tracing::error!(target: "pack_manager_lib::frontend", message = %args.message, "frontend event");
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ipc::PathSource;

    /// The placeholder report used before detection completes carries the
    /// current ToolEnv so the Environment Report is never blank.
    #[test]
    fn placeholder_report_carries_env_info() {
        let env = ToolEnv::from_entries(
            PathBuf::from("/Users/testuser"),
            vec![PathBuf::from("/a")],
            PathSource::StaticFallback,
        );
        let report = placeholder_detection_report(&env);
        assert!(report.managers.is_empty());
        assert_eq!(report.env.entries, vec!["/a".to_string()]);
        assert_eq!(report.env.home, "/Users/testuser");
    }
}
