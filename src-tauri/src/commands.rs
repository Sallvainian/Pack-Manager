//! IPC command handlers (SPEC §5.9) against `state::AppState`. Handlers stay
//! thin: the logic lives in `queue.rs` /
//! `journal.rs` / `diagnostics.rs`, all unit-tested there.

use std::collections::BTreeSet;
use std::path::PathBuf;

use serde::Deserialize;
use tauri::State;

use crate::detect::{adapter_for, DetectStatus, DetectionOutcome};
use crate::error::{IpcError, PmError};
use crate::ipc::{
    AppState as AppStateWire, AppUpdateStatus, DetectionReport, DiagnosticsResult, ErrorCode,
    ManagerId, OpIds, OpRef, OperationDetail, OperationRecord, PlanRequest, SelfUpdateRoute,
    UpdateCheckTrigger, UpgradePlan,
};
use crate::paths::ToolEnv;
use crate::queue::{self, PlanSources};
use crate::settings::{Settings, SettingsPatch};
use crate::state::{AppState, IssuedPlan};

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

fn plan_stale(detail: impl Into<String>) -> IpcError {
    IpcError::from_code(
        ErrorCode::PlanStale,
        "The available updates changed. Review the refreshed plan and confirm again.",
    )
    .with_detail(detail)
}

/// Point-in-time inputs for canonical plan validation and subsequent
/// submission construction. Synchronous guards are released before any queue
/// submission is awaited.
struct CurrentPlanState {
    outcome: DetectionOutcome,
    snapshots: Vec<crate::ipc::ManagerSnapshot>,
    records: Vec<OperationRecord>,
    settings: Settings,
    env: ToolEnv,
}

/// Caller must hold `state.plan_coordinator`; every writer follows the same
/// coordinator-first lock order, so these separately stored values represent
/// one canonical epoch rather than a mixed-time collection.
fn current_plan_state(state: &AppState) -> Result<CurrentPlanState, IpcError> {
    let outcome = state
        .detection
        .read()
        .expect("detection poisoned")
        .as_ref()
        .cloned()
        .ok_or_else(detection_not_ready)?;
    Ok(CurrentPlanState {
        outcome,
        snapshots: state.registry.all(),
        records: state.queue.records(),
        settings: state.settings.read().expect("settings poisoned").clone(),
        env: state.tool_env.read().expect("tool_env poisoned").clone(),
    })
}

fn canonical_plan(request: &PlanRequest, current: &CurrentPlanState) -> UpgradePlan {
    let busy = queue::busy_package_ids(&current.records);
    let stale = queue::stale_managers(&current.records);
    queue::build_upgrade_plan(
        request,
        &PlanSources {
            report: &current.outcome.report,
            snapshots: &current.snapshots,
            busy: &busy,
            stale: &stale,
        },
    )
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
    let outcome = state.redetect(ToolEnv::build().await).await;
    Ok(outcome.report)
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

/// Refresh submission from the CACHED detection outcome. `None` when detection
/// has not completed yet or the manager is currently marked absent.
fn refresh_submission_from_cache(state: &AppState, id: ManagerId) -> Option<queue::OpSubmission> {
    let det = state.detection.read().expect("detection poisoned");
    let outcome = det.as_ref()?;
    let status = outcome.statuses.get(&id)?;
    let settings = state.settings.read().expect("settings poisoned").clone();
    let env = state.tool_env.read().expect("tool_env poisoned").clone();
    queue::make_refresh_submission(id, status, &settings, &env)
}

/// Coalesces duplicates (same-manager refresh → existing opId). A manager the
/// cached detection marks absent is re-probed first (rebuild ToolEnv +
/// detect_all — same path as Re-detect) instead of erroring: the ManagerPane
/// header's Refresh button is reachable while a pane shows the absent state,
/// and the manager may have been installed since the last detection.
#[tauri::command]
pub async fn refresh_manager(
    state: State<'_, AppState>,
    args: ManagerArgs,
) -> Result<OpRef, IpcError> {
    if let Some(sub) = refresh_submission_from_cache(&state, args.manager_id) {
        let op_id = state.queue.submit(sub).await.map_err(IpcError::from)?;
        return Ok(OpRef { op_id });
    }
    refresh_manager_after_redetect(&state, args.manager_id, ToolEnv::build().await).await
}

/// The absent-manager re-probe path of [`refresh_manager`], with the ToolEnv
/// injected (`ToolEnv::build` probes the real login shell — this seam keeps
/// the path deterministic under test). Re-detects, then submits the refresh
/// when the manager turned out present; errors `tool_not_found` otherwise.
pub async fn refresh_manager_after_redetect(
    state: &AppState,
    id: ManagerId,
    env: ToolEnv,
) -> Result<OpRef, IpcError> {
    let outcome = state.redetect(env).await;
    let sub = {
        let settings = state.settings.read().expect("settings poisoned").clone();
        let env = state.tool_env.read().expect("tool_env poisoned").clone();
        outcome
            .statuses
            .get(&id)
            .and_then(|status| queue::make_refresh_submission(id, status, &settings, &env))
    };
    let Some(sub) = sub else {
        return Err(IpcError::from(PmError::ToolNotFound {
            tool: id.as_str().to_string(),
            searched: vec![],
        })
        .with_manager(id));
    };
    let op_id = state.queue.submit(sub).await.map_err(IpcError::from)?;
    Ok(OpRef { op_id })
}

/// Fan out one refresh op per present manager (SPEC F2). Re-runs detection
/// FIRST (same path as Re-detect: rebuild ToolEnv + detect_all, store, emit
/// `detection:updated`) so the fan-out reflects the CURRENT machine state — a
/// manager installed mid-session is detected AND refreshed by the same
/// Refresh All click, and one that disappeared stops being refreshed.
#[tauri::command]
pub async fn refresh_all(state: State<'_, AppState>) -> Result<OpIds, IpcError> {
    refresh_all_with_env(&state, ToolEnv::build().await).await
}

/// [`refresh_all`] with the ToolEnv injected (`ToolEnv::build` probes the real
/// login shell — this seam keeps the pathway deterministic under test).
/// `detection:updated` is emitted (inside [`AppState::redetect`]) BEFORE the
/// refresh submissions are built from the fresh statuses.
pub async fn refresh_all_with_env(state: &AppState, env: ToolEnv) -> Result<OpIds, IpcError> {
    let outcome = state.redetect(env).await;
    let subs: Vec<_> = {
        let settings = state.settings.read().expect("settings poisoned").clone();
        let env = state.tool_env.read().expect("tool_env poisoned").clone();
        outcome
            .statuses
            .iter()
            .filter_map(|(id, status)| queue::make_refresh_submission(*id, status, &settings, &env))
            .collect()
    };
    let mut op_ids = Vec::with_capacity(subs.len());
    for sub in subs {
        op_ids.push(state.queue.submit(sub).await.map_err(IpcError::from)?);
    }
    Ok(OpIds { op_ids })
}

/// Issues a backend capability for the pure plan preview (SPEC F4).
#[tauri::command]
pub async fn build_upgrade_plan(
    state: State<'_, AppState>,
    args: PlanRequest,
) -> Result<UpgradePlan, IpcError> {
    issue_upgrade_plan(&state, args).await
}

/// Core issuance path with no Tauri dependency (deterministic tests).
pub async fn issue_upgrade_plan(
    state: &AppState,
    request: PlanRequest,
) -> Result<UpgradePlan, IpcError> {
    let request = queue::canonicalize_plan_request(request).map_err(IpcError::from)?;
    let mut coordinator = state
        .plan_coordinator
        .lock()
        .expect("plan coordinator poisoned");
    let current = current_plan_state(state)?;
    let plan = canonical_plan(&request, &current);
    let revision = coordinator.revision();
    coordinator.insert_plan(IssuedPlan {
        request,
        plan: plan.clone(),
        revision,
    });
    Ok(plan)
}

/// Re-validates and enqueues the previewed plan. Commands are re-derived from
/// the same pure adapters that built the preview — byte-identical argv by
/// construction; nothing runs that was not shown.
#[tauri::command]
pub async fn execute_plan(
    state: State<'_, AppState>,
    args: ExecutePlanArgs,
) -> Result<OpIds, IpcError> {
    execute_issued_plan(&state, args.plan).await
}

fn submission_matches_group(sub: &queue::OpSubmission, group: &crate::ipc::PlanGroup) -> bool {
    let kind_matches = if group.self_update {
        matches!(sub.kind, crate::ops::OpKind::SelfUpdate)
    } else {
        matches!(sub.kind, crate::ops::OpKind::Upgrade { .. })
            && sub.kind.package_ids() == group.package_ids
    };
    let previews: Vec<String> = sub
        .commands
        .iter()
        .map(|command| {
            crate::ipc::command_preview(adapter_for(sub.executor).binary_name(), &command.spec.args)
        })
        .collect();
    let displayed: Vec<&str> = group
        .commands
        .iter()
        .map(|command| command.argv_preview.as_str())
        .collect();
    kind_matches
        && sub.executor == group.executor
        && sub.subject == group.subject
        && sub.locks.iter().copied().collect::<Vec<_>>() == group.locks
        && previews.iter().map(String::as_str).collect::<Vec<_>>() == displayed
}

/// Consumes and re-validates a backend-issued plan against one canonical
/// revision, then asks the scheduler to atomically re-check that revision and
/// enqueue the complete batch. No synchronous guard crosses an await.
pub async fn execute_issued_plan(
    state: &AppState,
    submitted: UpgradePlan,
) -> Result<OpIds, IpcError> {
    let (subs, expected_revision) = {
        let mut coordinator = state
            .plan_coordinator
            .lock()
            .expect("plan coordinator poisoned");
        let issued = coordinator
            .take_plan(&submitted.plan_id)
            .ok_or_else(|| plan_stale("unknown, evicted, or already-used planId"))?;
        if submitted != issued.plan || issued.plan.request != issued.request {
            return Err(plan_stale("submitted plan differs from the issued plan"));
        }
        if coordinator.state_update_in_progress() {
            return Err(plan_stale("canonical state is currently being refreshed"));
        }
        if issued.revision != coordinator.revision() {
            return Err(plan_stale("canonical state revision changed after preview"));
        }

        let current = current_plan_state(state)
            .map_err(|_| plan_stale("current detection state is unavailable"))?;
        if current.records.iter().any(|record| {
            record.kind == crate::ipc::OpKind::Refresh
                && matches!(
                    record.status,
                    crate::ipc::OpStatus::Queued | crate::ipc::OpStatus::Running
                )
        }) {
            return Err(plan_stale("a package-manager refresh is active"));
        }
        let mut fresh = canonical_plan(&issued.request, &current);
        // UUID generation is the only nondeterministic plan field. Normalizing
        // it lets exact equality cover every authenticated preview field.
        fresh.plan_id.clone_from(&issued.plan.plan_id);
        if fresh != issued.plan {
            return Err(plan_stale("package-manager state changed after preview"));
        }

        let mut subs = Vec::new();
        for group in &fresh.groups {
            if group.self_update {
                let info = current
                    .outcome
                    .report
                    .managers
                    .iter()
                    .find(|m| m.id == group.subject)
                    .ok_or_else(|| plan_stale("self-update subject is no longer detected"))?;
                match &info.self_update {
                    SelfUpdateRoute::ViaRefresh { .. } => {
                        return Err(plan_stale("self-update route changed to refresh"));
                    }
                    SelfUpdateRoute::Unavailable { .. } => {
                        return Err(plan_stale("self-update route became unavailable"));
                    }
                    route => {
                        let sub = queue::make_self_update_submission(
                            group.subject,
                            route,
                            &current.outcome.statuses,
                            &current.settings,
                            &current.env,
                        )
                        .map_err(IpcError::from)?;
                        if !submission_matches_group(&sub, group) {
                            return Err(plan_stale(
                                "re-derived self-update command differs from preview",
                            ));
                        }
                        subs.push(sub);
                    }
                }
            } else {
                let status = current
                    .outcome
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
                    &current.settings,
                    &current.env,
                )
                .map_err(IpcError::from)?;
                if !submission_matches_group(&sub, group) {
                    return Err(plan_stale(
                        "re-derived upgrade command differs from preview",
                    ));
                }
                subs.push(sub);
            }
        }
        (subs, coordinator.revision())
    };
    let op_ids = state
        .queue
        .submit_plan_batch(subs, expected_revision)
        .await
        .map_err(|error| match error {
            queue::PlanBatchError::SchedulerGone => IpcError::internal("scheduler is unavailable"),
            queue::PlanBatchError::RevisionChanged => {
                plan_stale("canonical state revision changed before queue admission")
            }
            queue::PlanBatchError::ActiveRefresh => {
                plan_stale("a package-manager refresh became active before queue admission")
            }
            queue::PlanBatchError::MutatingOperationConflict => plan_stale(
                "an earlier package-manager mutation conflicts with this plan's lock set",
            ),
            queue::PlanBatchError::StateUpdateInProgress => {
                plan_stale("canonical state update began before queue admission")
            }
        })?;
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
                let status = outcome.statuses.get(&args.manager_id).cloned().unwrap_or(
                    DetectStatus::Absent {
                        reason: "not detected".into(),
                    },
                );
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
        let snapshot = state
            .registry
            .get(args.manager_id)
            .ok_or_else(|| IpcError::internal(format!("no snapshot for {}", args.manager_id)))?;
        let issue = snapshot
            .health
            .iter()
            .find(|h| h.id == args.issue_id)
            .ok_or_else(|| IpcError::internal(format!("unknown health issue {}", args.issue_id)))?
            .clone();
        let det = state.detection.read().expect("detection poisoned");
        let outcome = det.as_ref().ok_or_else(detection_not_ready)?;
        let status =
            outcome
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
    let merged = set_settings_core(&state, &args.patch)?;
    if args.patch.log_level.is_some() {
        if let Some(handle) = state.logging.lock().expect("logging poisoned").as_ref() {
            let applied = handle.apply_settings_level(merged.log_level);
            tracing::info!(applied, level = ?merged.log_level, "log level change");
        }
    }
    tracing::info!("settings updated");
    Ok(merged)
}

/// Persist-then-publish settings transaction, separated from the Tauri
/// wrapper so revision and failure behavior can be proven directly.
fn set_settings_core(state: &AppState, patch: &SettingsPatch) -> Result<Settings, IpcError> {
    let mut coordinator = state
        .plan_coordinator
        .lock()
        .expect("plan coordinator poisoned");
    let mut merged = state.settings.read().expect("settings poisoned").clone();
    merged.apply_patch(patch);
    // Persist before publishing: a failed write leaves both the in-memory
    // settings and the canonical plan revision unchanged.
    merged
        .save_to(&state.settings_path)
        .map_err(IpcError::from)?;
    *state.settings.write().expect("settings poisoned") = merged.clone();
    coordinator.bump_revision();
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
    tauri_plugin_opener::reveal_item_in_dir(PathBuf::from(&record.log_path)).map_err(|e| {
        IpcError::from(PmError::Io {
            detail: e.to_string(),
        })
    })
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
pub async fn export_diagnostics(state: State<'_, AppState>) -> Result<DiagnosticsResult, IpcError> {
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

// ---------------------------------------------------------------------------
// In-app update (DECISIONS D25)
// ---------------------------------------------------------------------------

/// Current state, for rehydration on mount. Every later transition arrives on
/// `appUpdate:status`.
#[tauri::command]
pub async fn get_app_update_state(state: State<'_, AppState>) -> Result<AppUpdateStatus, IpcError> {
    Ok(state.app_update.status())
}

/// Manual check (menu bar → "Check for Updates…", Settings → "Check now").
/// Returns as soon as the check is under way; the outcome arrives as events, so
/// a slow network never blocks the click.
#[tauri::command]
pub async fn check_for_app_update(
    app: tauri::AppHandle,
    state: State<'_, AppState>,
) -> Result<(), IpcError> {
    let updater = state.app_update.clone();
    tauri::async_runtime::spawn(async move {
        let source = crate::app_update::TauriUpdateSource::new(app);
        updater
            .check_and_download(&source, UpdateCheckTrigger::Manual)
            .await;
    });
    Ok(())
}

/// Installs the downloaded update over the running bundle and relaunches.
///
/// The frontend is responsible for cancelling in-flight operations first (it
/// routes this through the quit guard); by the time this runs, restarting is
/// already the user's decision. `restart` does not return.
#[tauri::command]
pub async fn install_app_update(
    app: tauri::AppHandle,
    state: State<'_, AppState>,
) -> Result<(), IpcError> {
    state.app_update.install().map_err(|detail| {
        IpcError::from(PmError::Io {
            detail: format!("update install failed: {detail}"),
        })
    })?;
    // Same kill hook as a normal quit: children must never outlive the app.
    state.shutdown();
    // `restart` bare-spawns the new binary instead of going through
    // LaunchServices, so it comes up behind every other window unless it
    // activates itself. The spawned process inherits this env var and acts on
    // it at `RunEvent::Ready`. Set last: it must not outlive a failed install.
    //
    // This is an async command, so it runs on a runtime worker rather than the
    // main thread, and `set_var` can in principle race a concurrent `getenv`
    // elsewhere — the reason edition 2024 makes it `unsafe`. Tolerated here:
    // `shutdown()` has already run and `restart` never returns, so the process
    // is milliseconds from exec. On an edition bump this needs an `unsafe`
    // block plus that safety note; it fails to compile rather than silently
    // changing behaviour, so the bump itself is the reminder.
    std::env::set_var(crate::RELAUNCH_FOCUS_ENV, "1");
    tracing::info!("restarting into the updated build");
    app.restart();
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Mutex, RwLock};

    use crate::events::{AppEvent, VecSink};
    use crate::ipc::{
        ErrorCode, ManagedBy, ManagerInfo, ManagerSnapshot, ManagerStatus, OpKind, Package,
        PackageKind, PathSource, PlanSelection, SelfStatus,
    };
    use crate::journal::Journal;
    use crate::process::fake::FakeRunner;
    use crate::queue::{Queue, QueueDeps};
    use crate::registry::Registry;

    struct PendingDetectionRunner {
        started: Arc<tokio::sync::Notify>,
    }

    #[async_trait::async_trait]
    impl crate::process::CommandRunner for PendingDetectionRunner {
        async fn run(
            &self,
            _spec: &crate::process::CommandSpec,
            _cancel: tokio_util::sync::CancellationToken,
        ) -> Result<crate::process::CommandOutput, PmError> {
            self.started.notify_one();
            std::future::pending().await
        }

        async fn run_streaming(
            &self,
            _spec: &crate::process::CommandSpec,
            _sink: crate::process::LineSink,
            _cancel: tokio_util::sync::CancellationToken,
        ) -> Result<crate::process::CommandOutput, PmError> {
            std::future::pending().await
        }
    }

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

    // -----------------------------------------------------------------------
    // Refresh All re-detects first (SPEC F1/F2 — the "brew install mas, press
    // Refresh All" bug): the fan-out must be built from FRESH statuses, never
    // the cached detection outcome.
    // -----------------------------------------------------------------------

    struct Harness {
        state: AppState,
        fake: Arc<FakeRunner>,
        sink: Arc<VecSink>,
        /// Sandboxed ToolEnv: `entries = [bin]`, home + candidate root inside
        /// the tempdir — host-installed managers can never leak in.
        env: ToolEnv,
        bin: PathBuf,
        _dir: tempfile::TempDir,
    }

    fn make_exec(path: &std::path::Path) {
        use std::os::unix::fs::PermissionsExt;
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        std::fs::write(path, "#!/bin/sh\nexit 0\n").unwrap();
        std::fs::set_permissions(path, std::fs::Permissions::from_mode(0o755)).unwrap();
    }

    /// Full AppState over FakeRunner + VecSink (no Tauri, no real binaries).
    fn harness() -> Harness {
        let dir = tempfile::tempdir().unwrap();
        let bin = dir.path().join("bin");
        std::fs::create_dir_all(&bin).unwrap();
        let env = ToolEnv::from_entries(
            dir.path().join("home"),
            vec![bin.clone()],
            PathSource::StaticFallback,
        )
        .with_candidate_root(dir.path().join("candidates"));

        let fake = Arc::new(FakeRunner::new());
        let sink = Arc::new(VecSink::new());
        let registry = Arc::new(Registry::new());
        let plan_coordinator = Arc::new(Mutex::new(crate::state::PlanCoordinator::default()));
        let queue = Queue::new(QueueDeps {
            runner: fake.clone(),
            sink: sink.clone(),
            registry: registry.clone(),
            journal: Arc::new(Journal::new(dir.path().join("operations.jsonl"))),
            ops_dir: dir.path().join("ops"),
            refresh_factory: None,
            route_recheck: None,
            plan_coordinator: plan_coordinator.clone(),
            max_concurrency: queue::MAX_CONCURRENCY,
            aging_guard: queue::AGING_GUARD,
        });
        let state = AppState {
            settings: Arc::new(RwLock::new(Settings::default())),
            settings_path: Arc::new(dir.path().join("settings.json")),
            tool_env: Arc::new(RwLock::new(env.clone())),
            detection: Arc::new(RwLock::new(None)),
            registry,
            queue,
            journal_records: Arc::new(RwLock::new(Vec::new())),
            runner: fake.clone(),
            sink: sink.clone(),
            logging: Arc::new(Mutex::new(None)),
            app_update: Arc::new(crate::app_update::AppUpdater::new(
                "0.0.0-test",
                sink.clone(),
            )),
            plan_coordinator,
        };
        Harness {
            state,
            fake,
            sink,
            env,
            bin,
            _dir: dir,
        }
    }

    /// Registers probe + full refresh-plan rules for brew and mas so any op
    /// the fan-out enqueues completes against canned outputs.
    fn register_brew_and_mas_rules(fake: &FakeRunner) {
        fake.on("brew", &["--version"]).ok("Homebrew 4.5.2\n");
        fake.on("brew", &["update"]).ok("Already up-to-date.\n");
        fake.on("brew", &["list", "--versions"])
            .fixture("brew_list_versions_2026-07-22.txt");
        fake.on("brew", &["list", "--cask", "--versions"])
            .fixture("brew_list_cask_versions_2026-07-22.txt");
        fake.on("brew", &["outdated", "--json=v2"])
            .fixture("brew_outdated.json");
        fake.on("brew", &["outdated", "--json=v2", "--greedy"])
            .fixture("brew_outdated_greedy.json");
        fake.on("mas", &["--version"]).ok("7.0.0\n");
        fake.on("mas", &["list"]).fixture("mas_list_2026-07-22.txt");
        fake.on("mas", &["outdated"])
            .fixture("mas_outdated_2026-07-22.txt");
    }

    fn detection_updated_reports(sink: &VecSink) -> Vec<DetectionReport> {
        sink.events()
            .into_iter()
            .filter_map(|e| match e {
                AppEvent::DetectionUpdated(r) => Some(r),
                _ => None,
            })
            .collect()
    }

    fn manager_status(report: &DetectionReport, id: ManagerId) -> ManagerStatus {
        report
            .managers
            .iter()
            .find(|m| m.id == id)
            .expect("manager in report")
            .status
    }

    fn refresh_subjects(state: &AppState) -> Vec<ManagerId> {
        state
            .queue
            .records()
            .into_iter()
            .filter(|r| matches!(r.kind, OpKind::Refresh))
            .map(|r| r.subject)
            .collect()
    }

    /// THE regression (user repro): mas is absent at first detection, then
    /// installed mid-session. Refresh All must re-detect FIRST — storing and
    /// emitting the fresh DetectionReport — and include the newly present mas
    /// in the same fan-out. Pre-fix, `refresh_all` read the CACHED outcome and
    /// mas stayed "Not installed" with no refresh enqueued.
    #[tokio::test]
    async fn refresh_all_re_detects_and_includes_newly_installed_manager() {
        let h = harness();
        register_brew_and_mas_rules(&h.fake);
        make_exec(&h.bin.join("brew"));

        // First detection: brew present, mas absent (nothing outside the
        // sandbox is visible — candidate root is re-rooted).
        let first = h.state.redetect(h.env.clone()).await;
        assert!(matches!(
            first.statuses[&ManagerId::Mas],
            crate::detect::DetectStatus::Absent { .. }
        ));
        h.sink.take();

        // The user runs `brew install mas` in a terminal…
        make_exec(&h.bin.join("mas"));

        // …and presses Refresh All.
        let ids = refresh_all_with_env(&h.state, h.env.clone()).await.unwrap();

        // Fan-out covers the FRESH present set: brew AND the new mas.
        assert_eq!(ids.op_ids.len(), 2, "brew + newly detected mas");
        let subjects = refresh_subjects(&h.state);
        assert!(subjects.contains(&ManagerId::Brew), "{subjects:?}");
        assert!(subjects.contains(&ManagerId::Mas), "{subjects:?}");

        // The fresh outcome is STORED…
        {
            let det = h.state.detection.read().unwrap();
            assert!(matches!(
                det.as_ref().unwrap().statuses[&ManagerId::Mas],
                crate::detect::DetectStatus::Present { .. }
            ));
        }
        // …and EMITTED before any op event (redetect emits synchronously,
        // submissions follow).
        let events = h.sink.events();
        assert!(
            matches!(events.first(), Some(AppEvent::DetectionUpdated(_))),
            "detection:updated must precede the fan-out, got {:?}",
            events.first().map(AppEvent::name)
        );
        let reports = detection_updated_reports(&h.sink);
        assert_eq!(reports.len(), 1);
        assert_eq!(
            manager_status(&reports[0], ManagerId::Mas),
            ManagerStatus::Present
        );

        // Let the enqueued refreshes finish against the canned outputs.
        assert!(
            h.state
                .queue
                .wait_until_idle(std::time::Duration::from_secs(10))
                .await
        );
    }

    /// The inverse: a manager that disappeared mid-session stops being
    /// refreshed and the emitted report renders it absent.
    #[tokio::test]
    async fn refresh_all_re_detects_and_drops_removed_manager() {
        let h = harness();
        register_brew_and_mas_rules(&h.fake);
        make_exec(&h.bin.join("brew"));
        make_exec(&h.bin.join("mas"));

        let first = h.state.redetect(h.env.clone()).await;
        assert!(matches!(
            first.statuses[&ManagerId::Mas],
            crate::detect::DetectStatus::Present { .. }
        ));
        h.sink.take();

        std::fs::remove_file(h.bin.join("mas")).unwrap();

        let ids = refresh_all_with_env(&h.state, h.env.clone()).await.unwrap();

        assert_eq!(ids.op_ids.len(), 1, "only brew remains in the fan-out");
        let subjects = refresh_subjects(&h.state);
        assert!(subjects.contains(&ManagerId::Brew), "{subjects:?}");
        assert!(!subjects.contains(&ManagerId::Mas), "{subjects:?}");

        let reports = detection_updated_reports(&h.sink);
        assert_eq!(reports.len(), 1);
        assert_eq!(
            manager_status(&reports[0], ManagerId::Mas),
            ManagerStatus::Absent
        );
        {
            let det = h.state.detection.read().unwrap();
            assert!(matches!(
                det.as_ref().unwrap().statuses[&ManagerId::Mas],
                crate::detect::DetectStatus::Absent { .. }
            ));
        }
        assert!(
            h.state
                .queue
                .wait_until_idle(std::time::Duration::from_secs(10))
                .await
        );
    }

    /// Per-manager refresh of a manager the CACHE marks absent re-probes
    /// instead of erroring (the ManagerPane header's Refresh button stays
    /// reachable in the absent state); a manager still absent after the
    /// re-probe errors `tool_not_found` as before.
    #[tokio::test]
    async fn refresh_manager_absent_in_cache_re_probes_then_submits() {
        let h = harness();
        register_brew_and_mas_rules(&h.fake);
        make_exec(&h.bin.join("brew"));

        h.state.redetect(h.env.clone()).await;
        assert!(
            refresh_submission_from_cache(&h.state, ManagerId::Mas).is_none(),
            "cache says absent — the command would fall through to the re-probe"
        );

        // Still absent after the re-probe → the original error.
        let err = refresh_manager_after_redetect(&h.state, ManagerId::Mas, h.env.clone())
            .await
            .unwrap_err();
        assert_eq!(err.code, crate::ipc::ErrorCode::ToolNotFound);

        // Installed since → the re-probe detects it and submits the refresh.
        make_exec(&h.bin.join("mas"));
        let op = refresh_manager_after_redetect(&h.state, ManagerId::Mas, h.env.clone())
            .await
            .unwrap();
        let record = h.state.queue.record(&op.op_id).expect("record exists");
        assert!(matches!(record.kind, OpKind::Refresh));
        assert_eq!(record.subject, ManagerId::Mas);
        {
            let det = h.state.detection.read().unwrap();
            assert!(matches!(
                det.as_ref().unwrap().statuses[&ManagerId::Mas],
                crate::detect::DetectStatus::Present { .. }
            ));
        }
        assert!(
            h.state
                .queue
                .wait_until_idle(std::time::Duration::from_secs(10))
                .await
        );
    }

    // -----------------------------------------------------------------------
    // Backend-issued upgrade-plan capabilities and current-state validation.
    // -----------------------------------------------------------------------

    async fn prepare_brew_upgrade(h: &Harness) {
        make_exec(&h.bin.join("brew"));
        h.fake.on("brew", &["--version"]).ok("Homebrew 4.5.2\n");
        h.state.redetect(h.env.clone()).await;
        h.state.registry.upsert(ManagerSnapshot {
            manager_id: ManagerId::Brew,
            refreshed_at: "2026-07-22T14:00:00Z".into(),
            packages: vec![
                Package {
                    id: "formula:dolt".into(),
                    name: "dolt".into(),
                    kind: PackageKind::Formula,
                    installed: Some("2.2.1".into()),
                    latest: Some("2.2.2".into()),
                    outdated: true,
                    pinned: false,
                    meta: None,
                },
                Package {
                    id: "formula:deno".into(),
                    name: "deno".into(),
                    kind: PackageKind::Formula,
                    installed: Some("2.9.0".into()),
                    latest: Some("2.9.3".into()),
                    outdated: true,
                    pinned: true,
                    meta: None,
                },
            ],
            self_status: None,
            health: vec![],
        });
    }

    fn prepare_brew_upgrade_with_routed_mise_self_update(h: &Harness) {
        make_exec(&h.bin.join("brew"));
        make_exec(&h.bin.join("mise"));
        let report = DetectionReport {
            managers: vec![
                ManagerInfo {
                    id: ManagerId::Brew,
                    display_name: "Homebrew".into(),
                    status: ManagerStatus::Present,
                    binary_path: Some(h.bin.join("brew").to_string_lossy().into_owned()),
                    canonical_path: Some(h.bin.join("brew").to_string_lossy().into_owned()),
                    version: Some("4.5.2".into()),
                    managed_by: ManagedBy::Standalone,
                    evidence: Some("test standalone Homebrew".into()),
                    self_update: SelfUpdateRoute::ViaRefresh {
                        note: "brew update runs as part of every refresh".into(),
                    },
                    install_hint: None,
                },
                ManagerInfo {
                    id: ManagerId::Mise,
                    display_name: "mise".into(),
                    status: ManagerStatus::Present,
                    binary_path: Some(h.bin.join("mise").to_string_lossy().into_owned()),
                    canonical_path: Some(h.bin.join("mise").to_string_lossy().into_owned()),
                    version: Some("2026.1.0".into()),
                    managed_by: ManagedBy::Brew,
                    evidence: Some("test route via Homebrew".into()),
                    self_update: SelfUpdateRoute::routed(
                        ManagerId::Brew,
                        "brew",
                        vec!["upgrade".into(), "mise".into()],
                        "mise is managed by Homebrew",
                    ),
                    install_hint: None,
                },
            ],
            env: h.env.env_info(),
        };
        let mut statuses = std::collections::BTreeMap::new();
        statuses.insert(
            ManagerId::Brew,
            DetectStatus::Present {
                binary_path: h.bin.join("brew"),
                canonical_path: h.bin.join("brew"),
                version: Some("4.5.2".into()),
                managed_by: ManagedBy::Standalone,
                evidence: "test standalone Homebrew".into(),
            },
        );
        statuses.insert(
            ManagerId::Mise,
            DetectStatus::Present {
                binary_path: h.bin.join("mise"),
                canonical_path: h.bin.join("mise"),
                version: Some("2026.1.0".into()),
                managed_by: ManagedBy::Brew,
                evidence: "test route via Homebrew".into(),
            },
        );
        h.state.registry.set_routes_from(&report);
        *h.state.detection.write().unwrap() = Some(DetectionOutcome { report, statuses });
        h.state.registry.upsert(ManagerSnapshot {
            manager_id: ManagerId::Brew,
            refreshed_at: "2026-07-22T14:00:00Z".into(),
            packages: vec![Package {
                id: "formula:dolt".into(),
                name: "dolt".into(),
                kind: PackageKind::Formula,
                installed: Some("2.2.1".into()),
                latest: Some("2.2.2".into()),
                outdated: true,
                pinned: false,
                meta: None,
            }],
            self_status: None,
            health: vec![],
        });
        h.state.registry.upsert(ManagerSnapshot {
            manager_id: ManagerId::Mise,
            refreshed_at: "2026-07-22T14:00:00Z".into(),
            packages: vec![],
            self_status: Some(SelfStatus {
                installed: Some("2026.1.0".into()),
                latest: Some("2026.2.0".into()),
                update_available: true,
            }),
            health: vec![],
        });
    }

    fn dolt_request() -> PlanRequest {
        PlanRequest {
            selection: Some(vec![
                PlanSelection {
                    manager_id: ManagerId::Brew,
                    package_id: "formula:dolt".into(),
                },
                PlanSelection {
                    manager_id: ManagerId::Brew,
                    package_id: "formula:deno".into(),
                },
            ]),
            include_self_updates: false,
            include_greedy_casks: false,
        }
    }

    async fn assert_plan_rejected_without_submission(state: &AppState, plan: UpgradePlan) {
        let before = state.queue.records().len();
        let error = execute_issued_plan(state, plan).await.unwrap_err();
        assert_eq!(error.code, ErrorCode::PlanStale);
        assert_eq!(state.queue.records().len(), before);
    }

    #[tokio::test]
    async fn issued_plan_executes_once_and_replay_submits_nothing() {
        let h = harness();
        prepare_brew_upgrade(&h).await;
        h.fake.on_streaming("brew", &["upgrade", "dolt"]).exit(0);

        let plan = issue_upgrade_plan(&h.state, dolt_request()).await.unwrap();
        assert_eq!(plan.request, dolt_request());
        let wire = serde_json::to_string(&plan).unwrap();
        let round_tripped: UpgradePlan = serde_json::from_str(&wire).unwrap();
        assert_eq!(round_tripped, plan);
        let result = execute_issued_plan(&h.state, round_tripped).await.unwrap();
        assert_eq!(result.op_ids.len(), 1);
        assert_eq!(h.state.queue.records().len(), 1);

        assert_plan_rejected_without_submission(&h.state, plan).await;
        assert!(
            h.state
                .queue
                .wait_until_idle(std::time::Duration::from_secs(10))
                .await
        );
    }

    #[tokio::test]
    async fn round_tripped_multi_group_plan_executes_routed_self_update_structurally() {
        let h = harness();
        prepare_brew_upgrade_with_routed_mise_self_update(&h);
        h.fake.on_streaming("brew", &["upgrade", "dolt"]).exit(0);
        h.fake.on_streaming("brew", &["upgrade", "mise"]).exit(0);
        let request = PlanRequest {
            selection: Some(vec![PlanSelection {
                manager_id: ManagerId::Brew,
                package_id: "formula:dolt".into(),
            }]),
            include_self_updates: true,
            include_greedy_casks: false,
        };

        let issued = issue_upgrade_plan(&h.state, request).await.unwrap();
        assert_eq!(issued.groups.len(), 2);
        let upgrade = issued
            .groups
            .iter()
            .find(|group| !group.self_update)
            .unwrap();
        assert_eq!(upgrade.locks, vec![ManagerId::Brew]);
        let self_update = issued
            .groups
            .iter()
            .find(|group| group.self_update)
            .unwrap();
        assert_eq!(self_update.executor, ManagerId::Brew);
        assert_eq!(self_update.subject, ManagerId::Mise);
        assert_eq!(self_update.locks, vec![ManagerId::Brew, ManagerId::Mise]);

        let wire = serde_json::to_string(&issued).unwrap();
        let round_tripped: UpgradePlan = serde_json::from_str(&wire).unwrap();
        let result = execute_issued_plan(&h.state, round_tripped).await.unwrap();
        assert_eq!(result.op_ids.len(), 2);
        assert!(
            h.state
                .queue
                .wait_until_idle(std::time::Duration::from_secs(10))
                .await
        );

        let records = h.state.queue.records();
        assert_eq!(records.len(), 2);
        assert_eq!(records[0].op_id, result.op_ids[0]);
        assert_eq!(records[0].kind, OpKind::Upgrade);
        assert_eq!(records[0].executor, ManagerId::Brew);
        assert_eq!(records[0].subject, ManagerId::Brew);
        assert_eq!(records[1].op_id, result.op_ids[1]);
        assert_eq!(records[1].kind, OpKind::SelfUpdate);
        assert_eq!(records[1].executor, ManagerId::Brew);
        assert_eq!(records[1].subject, ManagerId::Mise);
        let calls = h.fake.calls();
        assert_eq!(calls.len(), 2);
        assert_eq!(calls[0].basename, "brew");
        assert_eq!(calls[0].args, vec!["upgrade", "dolt"]);
        assert_eq!(calls[0].purpose, crate::process::CmdPurpose::Upgrade);
        assert_eq!(calls[1].basename, "brew");
        assert_eq!(calls[1].args, vec!["upgrade", "mise"]);
        assert_eq!(calls[1].purpose, crate::process::CmdPurpose::SelfUpdate);
    }

    #[tokio::test]
    async fn later_group_missing_backend_argv_submits_no_partial_batch() {
        let h = harness();
        prepare_brew_upgrade_with_routed_mise_self_update(&h);
        let request = PlanRequest {
            selection: Some(vec![PlanSelection {
                manager_id: ManagerId::Brew,
                package_id: "formula:dolt".into(),
            }]),
            include_self_updates: true,
            include_greedy_casks: false,
        };
        let issued = issue_upgrade_plan(&h.state, request).await.unwrap();
        let wire = serde_json::to_string(&issued).unwrap();
        let round_tripped: UpgradePlan = serde_json::from_str(&wire).unwrap();

        // Deliberately bypass the coordinator in this test to model corrupted
        // backend-only route data without triggering the earlier revision
        // guard. The ordinary Brew group derives first; the later self-update
        // must still fail before the scheduler sees any group.
        {
            let mut detection = h.state.detection.write().unwrap();
            let mise = detection
                .as_mut()
                .unwrap()
                .report
                .managers
                .iter_mut()
                .find(|manager| manager.id == ManagerId::Mise)
                .unwrap();
            match &mut mise.self_update {
                SelfUpdateRoute::Routed { command_args, .. } => command_args.clear(),
                _ => panic!("expected routed mise self-update"),
            }
        }

        let error = execute_issued_plan(&h.state, round_tripped)
            .await
            .unwrap_err();
        assert_eq!(error.code, ErrorCode::Internal);
        assert!(h.state.queue.records().is_empty());
        assert!(h.sink.events().is_empty());
        assert!(h.fake.calls().is_empty());
    }

    #[tokio::test]
    async fn every_round_tripped_plan_section_is_authenticated() {
        let h = harness();
        prepare_brew_upgrade(&h).await;

        let mut plan = issue_upgrade_plan(&h.state, dolt_request()).await.unwrap();
        plan.plan_id = "unknown-plan-id".into();
        assert_plan_rejected_without_submission(&h.state, plan).await;

        let mut plan = issue_upgrade_plan(&h.state, dolt_request()).await.unwrap();
        plan.request.include_greedy_casks = true;
        assert_plan_rejected_without_submission(&h.state, plan).await;

        let mut plan = issue_upgrade_plan(&h.state, dolt_request()).await.unwrap();
        plan.groups[0].locks.push(ManagerId::Mise);
        assert_plan_rejected_without_submission(&h.state, plan).await;

        let mut plan = issue_upgrade_plan(&h.state, dolt_request()).await.unwrap();
        plan.groups[0].subject = ManagerId::Mise;
        assert_plan_rejected_without_submission(&h.state, plan).await;

        let mut plan = issue_upgrade_plan(&h.state, dolt_request()).await.unwrap();
        plan.groups[0].executor = ManagerId::Mise;
        assert_plan_rejected_without_submission(&h.state, plan).await;

        let mut plan = issue_upgrade_plan(&h.state, dolt_request()).await.unwrap();
        plan.groups[0].package_ids.push("formula:attacker".into());
        assert_plan_rejected_without_submission(&h.state, plan).await;

        let mut plan = issue_upgrade_plan(&h.state, dolt_request()).await.unwrap();
        plan.groups[0].commands[0].argv_preview = "brew upgrade attacker".into();
        assert_plan_rejected_without_submission(&h.state, plan).await;

        let mut plan = issue_upgrade_plan(&h.state, dolt_request()).await.unwrap();
        plan.groups[0].self_update = true;
        assert_plan_rejected_without_submission(&h.state, plan).await;

        let mut plan = issue_upgrade_plan(&h.state, dolt_request()).await.unwrap();
        plan.excluded.clear();
        assert_plan_rejected_without_submission(&h.state, plan).await;

        let mut plan = issue_upgrade_plan(&h.state, dolt_request()).await.unwrap();
        plan.notes.push("frontend-added note".into());
        assert_plan_rejected_without_submission(&h.state, plan).await;

        let mut plan = issue_upgrade_plan(&h.state, dolt_request()).await.unwrap();
        plan.warnings.push("frontend-added warning".into());
        assert_plan_rejected_without_submission(&h.state, plan).await;
    }

    #[tokio::test]
    async fn snapshot_drift_consumes_plan_and_submits_nothing() {
        let h = harness();
        prepare_brew_upgrade(&h).await;
        let plan = issue_upgrade_plan(&h.state, dolt_request()).await.unwrap();

        let mut current = h.state.registry.get(ManagerId::Brew).unwrap();
        current
            .packages
            .iter_mut()
            .find(|package| package.id == "formula:dolt")
            .unwrap()
            .outdated = false;
        h.state.registry.upsert(current);

        assert_plan_rejected_without_submission(&h.state, plan.clone()).await;
        assert_plan_rejected_without_submission(&h.state, plan).await;
    }

    #[tokio::test]
    async fn issued_plan_echoes_deduplicated_request_and_rejects_oversized_selection() {
        let h = harness();
        prepare_brew_upgrade(&h).await;
        let duplicate_request = PlanRequest {
            selection: Some(vec![
                PlanSelection {
                    manager_id: ManagerId::Brew,
                    package_id: "formula:dolt".into(),
                },
                PlanSelection {
                    manager_id: ManagerId::Brew,
                    package_id: "formula:dolt".into(),
                },
            ]),
            include_self_updates: false,
            include_greedy_casks: false,
        };
        let plan = issue_upgrade_plan(&h.state, duplicate_request)
            .await
            .unwrap();
        assert_eq!(plan.request.selection.as_ref().unwrap().len(), 1);
        assert_eq!(plan.groups[0].package_ids, vec!["formula:dolt"]);
        assert_eq!(plan.groups[0].commands[0].argv_preview, "brew upgrade dolt");

        let oversized = PlanRequest {
            selection: Some(
                (0..=queue::MAX_PLAN_SELECTIONS)
                    .map(|index| PlanSelection {
                        manager_id: ManagerId::Brew,
                        package_id: format!("formula:pkg-{index}"),
                    })
                    .collect(),
            ),
            include_self_updates: false,
            include_greedy_casks: false,
        };
        let error = issue_upgrade_plan(&h.state, oversized).await.unwrap_err();
        assert_eq!(error.code, ErrorCode::Internal);
        assert!(h.state.queue.records().is_empty());
    }

    #[tokio::test]
    async fn redetection_revision_drift_consumes_plan_and_submits_nothing() {
        let h = harness();
        prepare_brew_upgrade(&h).await;
        let plan = issue_upgrade_plan(&h.state, dolt_request()).await.unwrap();

        // Even an equivalent re-detection is a new canonical epoch: routes,
        // binary ownership, and ToolEnv were re-probed after review.
        h.state.redetect(h.env.clone()).await;

        assert_plan_rejected_without_submission(&h.state, plan).await;
    }

    #[tokio::test]
    async fn cancelled_pending_redetection_releases_revision_barrier() {
        let mut h = harness();
        prepare_brew_upgrade(&h).await;
        let before_cancel = issue_upgrade_plan(&h.state, dolt_request()).await.unwrap();

        let started = Arc::new(tokio::sync::Notify::new());
        h.state.runner = Arc::new(PendingDetectionRunner {
            started: started.clone(),
        });
        let task_state = h.state.clone();
        let env = h.env.clone();
        let redetect = tokio::spawn(async move { task_state.redetect(env).await });
        started.notified().await;
        assert!(h
            .state
            .plan_coordinator
            .lock()
            .unwrap()
            .state_update_in_progress());

        redetect.abort();
        assert!(redetect.await.unwrap_err().is_cancelled());
        assert!(
            !h.state
                .plan_coordinator
                .lock()
                .unwrap()
                .state_update_in_progress(),
            "dropping redetect must not leave execution blocked forever"
        );
        assert_plan_rejected_without_submission(&h.state, before_cancel).await;

        h.fake.on_streaming("brew", &["upgrade", "dolt"]).exit(0);
        let fresh = issue_upgrade_plan(&h.state, dolt_request()).await.unwrap();
        let result = execute_issued_plan(&h.state, fresh).await.unwrap();
        assert_eq!(result.op_ids.len(), 1);
    }

    #[tokio::test]
    async fn plan_issued_during_active_refresh_is_rejected_without_upgrade() {
        let h = harness();
        prepare_brew_upgrade(&h).await;
        let gate = Arc::new(tokio::sync::Notify::new());
        h.fake
            .on("brew", &["update"])
            .gate(gate.clone())
            .ok("Already up-to-date.\n");
        h.fake
            .on("brew", &["list", "--versions"])
            .fixture("brew_list_versions_2026-07-22.txt");
        h.fake
            .on("brew", &["list", "--cask", "--versions"])
            .fixture("brew_list_cask_versions_2026-07-22.txt");
        h.fake
            .on("brew", &["outdated", "--json=v2"])
            .fixture("brew_outdated.json");
        h.fake
            .on("brew", &["outdated", "--json=v2", "--greedy"])
            .fixture("brew_outdated_greedy.json");

        let before_refresh = issue_upgrade_plan(&h.state, dolt_request()).await.unwrap();
        let refresh = refresh_submission_from_cache(&h.state, ManagerId::Brew).unwrap();
        let refresh_id = h.state.queue.submit(refresh).await.unwrap();
        assert_plan_rejected_without_submission(&h.state, before_refresh).await;

        let during_refresh = issue_upgrade_plan(&h.state, dolt_request()).await.unwrap();
        let before = h.state.queue.records().len();

        let error = execute_issued_plan(&h.state, during_refresh)
            .await
            .unwrap_err();
        assert_eq!(error.code, ErrorCode::PlanStale);
        assert_eq!(h.state.queue.records().len(), before);
        assert_eq!(before, 1, "only the active refresh is queued/running");

        gate.notify_one();
        assert!(
            h.state
                .queue
                .wait_until_idle(std::time::Duration::from_secs(10))
                .await
        );
        assert_eq!(
            h.state.queue.record(&refresh_id).unwrap().status,
            crate::ipc::OpStatus::Succeeded
        );
    }

    #[tokio::test]
    async fn plan_cannot_queue_behind_earlier_direct_mutation_on_same_lock() {
        let h = harness();
        prepare_brew_upgrade(&h).await;
        let baseline_calls = h.fake.calls().len();
        let gate = Arc::new(tokio::sync::Notify::new());
        h.fake
            .on_streaming("brew", &["upgrade", "other"])
            .gate(gate.clone())
            .exit(0);
        let status =
            h.state.detection.read().unwrap().as_ref().unwrap().statuses[&ManagerId::Brew].clone();
        let direct = queue::make_upgrade_submission(
            ManagerId::Brew,
            &["formula:other".into()],
            false,
            &status,
            &h.state.settings.read().unwrap(),
            &h.env,
        )
        .unwrap();
        let direct_id = h.state.queue.submit(direct).await.unwrap();
        for _ in 0..100 {
            if h.fake.calls().len() == baseline_calls + 1 {
                break;
            }
            tokio::task::yield_now().await;
        }
        assert_eq!(h.fake.calls().len(), baseline_calls + 1);

        let plan = issue_upgrade_plan(&h.state, dolt_request()).await.unwrap();
        let error = execute_issued_plan(&h.state, plan).await.unwrap_err();
        assert_eq!(error.code, ErrorCode::PlanStale);
        assert_eq!(h.state.queue.records().len(), 1);
        assert_eq!(h.fake.calls().len(), baseline_calls + 1);

        gate.notify_one();
        assert!(
            h.state
                .queue
                .wait_until_idle(std::time::Duration::from_secs(10))
                .await
        );
        assert_eq!(
            h.state.queue.record(&direct_id).unwrap().status,
            crate::ipc::OpStatus::Succeeded
        );
    }

    #[tokio::test]
    async fn settings_change_after_issue_invalidates_plan_without_submission() {
        let h = harness();
        prepare_brew_upgrade(&h).await;
        let plan = issue_upgrade_plan(&h.state, dolt_request()).await.unwrap();
        let before_revision = h.state.plan_coordinator.lock().unwrap().revision();

        let updated = set_settings_core(
            &h.state,
            &SettingsPatch {
                stall_after_secs: Some(321),
                ..SettingsPatch::default()
            },
        )
        .unwrap();
        assert_eq!(updated.stall_after_secs, 321);
        assert_eq!(
            h.state.plan_coordinator.lock().unwrap().revision(),
            before_revision + 1
        );

        assert_plan_rejected_without_submission(&h.state, plan).await;
        assert!(h.state.queue.records().is_empty());
    }

    #[tokio::test]
    async fn failed_settings_persistence_changes_neither_memory_nor_revision() {
        let mut h = harness();
        let file_where_directory_is_required = h.bin.join("not-a-directory");
        std::fs::write(&file_where_directory_is_required, "block parent creation").unwrap();
        h.state.settings_path = Arc::new(file_where_directory_is_required.join("settings.json"));
        let before_settings = h.state.settings.read().unwrap().clone();
        let before_revision = h.state.plan_coordinator.lock().unwrap().revision();

        let error = set_settings_core(
            &h.state,
            &SettingsPatch {
                stall_after_secs: Some(321),
                ..SettingsPatch::default()
            },
        )
        .unwrap_err();

        assert_eq!(error.code, ErrorCode::Io);
        assert_eq!(*h.state.settings.read().unwrap(), before_settings);
        assert_eq!(
            h.state.plan_coordinator.lock().unwrap().revision(),
            before_revision
        );
    }

    #[tokio::test]
    async fn oldest_plan_is_stale_after_bounded_cache_eviction() {
        let h = harness();
        prepare_brew_upgrade(&h).await;
        let oldest = issue_upgrade_plan(&h.state, dolt_request()).await.unwrap();
        for _ in 0..crate::state::ISSUED_PLAN_LIMIT {
            issue_upgrade_plan(&h.state, dolt_request()).await.unwrap();
        }
        assert_plan_rejected_without_submission(&h.state, oldest).await;
    }

    #[tokio::test]
    async fn overlapping_prebuilt_plans_serialize_validation_through_enqueue() {
        let h = harness();
        prepare_brew_upgrade(&h).await;
        let gate = Arc::new(tokio::sync::Notify::new());
        h.fake
            .on_streaming("brew", &["upgrade", "dolt"])
            .gate(gate.clone())
            .exit(0);

        let first = issue_upgrade_plan(&h.state, dolt_request()).await.unwrap();
        let second = issue_upgrade_plan(&h.state, dolt_request()).await.unwrap();

        let first_state = h.state.clone();
        let second_state = h.state.clone();
        let first_task =
            tokio::spawn(async move { execute_issued_plan(&first_state, first).await });
        let second_task =
            tokio::spawn(async move { execute_issued_plan(&second_state, second).await });
        let (first_result, second_result) = tokio::join!(first_task, second_task);
        let results = [first_result.unwrap(), second_result.unwrap()];
        assert_eq!(results.iter().filter(|result| result.is_ok()).count(), 1);
        assert_eq!(
            results
                .iter()
                .filter(|result| matches!(result, Err(error) if error.code == ErrorCode::PlanStale))
                .count(),
            1
        );
        assert!(results
            .iter()
            .filter_map(|result| result.as_ref().ok())
            .all(|ids| ids.op_ids.len() == 1));
        assert_eq!(h.state.queue.records().len(), 1);

        gate.notify_one();
        assert!(
            h.state
                .queue
                .wait_until_idle(std::time::Duration::from_secs(10))
                .await
        );
    }

    #[tokio::test]
    async fn fast_terminal_first_plan_still_invalidates_second_prebuilt_plan() {
        let h = harness();
        prepare_brew_upgrade(&h).await;
        h.fake.on_streaming("brew", &["upgrade", "dolt"]).exit(0);

        let first = issue_upgrade_plan(&h.state, dolt_request()).await.unwrap();
        let second = issue_upgrade_plan(&h.state, dolt_request()).await.unwrap();

        let first_result = execute_issued_plan(&h.state, first).await.unwrap();
        assert_eq!(first_result.op_ids.len(), 1);
        assert!(
            h.state
                .queue
                .wait_until_idle(std::time::Duration::from_secs(10))
                .await
        );
        assert_eq!(
            h.state
                .queue
                .record(&first_result.op_ids[0])
                .unwrap()
                .status,
            crate::ipc::OpStatus::Succeeded
        );

        assert_plan_rejected_without_submission(&h.state, second).await;
        assert_eq!(h.state.queue.records().len(), 1);
    }
}
