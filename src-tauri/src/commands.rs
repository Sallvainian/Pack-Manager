//! All 17 IPC command handlers (SPEC §5.9) — completed by U5 against
//! `state::AppState`. Handlers stay thin: the logic lives in `queue.rs` /
//! `journal.rs` / `diagnostics.rs`, all unit-tested there.

use std::collections::BTreeSet;
use std::path::PathBuf;

use serde::Deserialize;
use tauri::State;

use crate::detect::DetectStatus;
use crate::error::{IpcError, PmError};
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
                let status = outcome.statuses.get(&group.executor).cloned().unwrap_or(
                    DetectStatus::Absent {
                        reason: format!("{} was not detected", group.executor),
                    },
                );
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Mutex, RwLock};

    use crate::events::{AppEvent, VecSink};
    use crate::ipc::{ManagerStatus, OpKind, PathSource};
    use crate::journal::Journal;
    use crate::process::fake::FakeRunner;
    use crate::queue::{Queue, QueueDeps};
    use crate::registry::Registry;

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
        let queue = Queue::new(QueueDeps {
            runner: fake.clone(),
            sink: sink.clone(),
            registry: registry.clone(),
            journal: Arc::new(Journal::new(dir.path().join("operations.jsonl"))),
            ops_dir: dir.path().join("ops"),
            refresh_factory: None,
            route_recheck: None,
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
        fake.on("mas", &["--version"]).ok("1.9.0\n");
        fake.on("mas", &["list"]).fixture("mas_list_synthetic.txt");
        fake.on("mas", &["outdated"])
            .fixture("mas_outdated_synthetic.txt");
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
}
