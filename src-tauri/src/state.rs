//! Managed application state — completed by U5 (SPEC §5.12 startup sequence).
//!
//! `AppState` is `Clone` (all shared pieces are `Arc`s) so the async startup
//! task and the Tauri managed-state cell can hold the same instance.

use std::path::PathBuf;
use std::sync::{Arc, Mutex, RwLock};

use crate::detect::{self, DetectionOutcome};
use crate::events::{AppEvent, EventSink, TauriSink};
use crate::journal::{self, Journal};
use crate::logging::{self, LoggingHandle};
use crate::paths::{static_entries, ToolEnv};
use crate::process::runner::RealRunner;
use crate::process::CommandRunner;
use crate::queue::{self, Queue, QueueDeps, RefreshFactory, RouteRecheck};
use crate::registry::Registry;
use crate::settings::Settings;

/// Quit-guard bound: SIGTERM → 5s grace → SIGKILL plus scheduling headroom.
/// The exit path blocks at most this long for running ops to finalize.
const SHUTDOWN_GRACE: std::time::Duration = std::time::Duration::from_secs(7);

#[derive(Clone)]
pub struct AppState {
    pub settings: Arc<RwLock<Settings>>,
    pub settings_path: Arc<PathBuf>,
    pub tool_env: Arc<RwLock<ToolEnv>>,
    pub detection: Arc<RwLock<Option<DetectionOutcome>>>,
    pub registry: Arc<Registry>,
    pub queue: Arc<Queue>,
    /// Records loaded from the journal at startup (previous sessions;
    /// start-without-finish already marked Interrupted).
    pub journal_records: Arc<RwLock<Vec<crate::ipc::OperationRecord>>>,
    pub runner: Arc<dyn CommandRunner>,
    pub sink: Arc<dyn EventSink>,
    pub logging: Arc<Mutex<Option<LoggingHandle>>>,
}

impl AppState {
    /// Builds the full state graph (SPEC §5.12 step 2, sync part): settings →
    /// journal (compact + load, Interrupted marked) → registry → queue. The
    /// ToolEnv starts as the static fallback; [`AppState::startup`] replaces
    /// it with the probed merge and runs detection.
    ///
    /// Must run inside the tokio runtime (the queue and event batcher spawn
    /// tasks).
    pub async fn initialize(
        handle: tauri::AppHandle,
        settings: Settings,
        settings_path: PathBuf,
        logging_handle: Option<LoggingHandle>,
    ) -> AppState {
        let sink: Arc<dyn EventSink> = Arc::new(TauriSink::new(handle));
        let runner: Arc<dyn CommandRunner> = Arc::new(RealRunner::new());
        let registry = Arc::new(Registry::new());

        let journal_path = Settings::app_support_dir().join("operations.jsonl");
        if let Err(e) = journal::compact(&journal_path, journal::COMPACT_KEEP) {
            tracing::error!(error = %e, "journal compaction failed");
        }
        let loaded = journal::load_records(&journal_path);
        let interrupted = loaded
            .iter()
            .filter(|r| r.status == crate::ipc::OpStatus::Interrupted)
            .count();
        tracing::info!(
            records = loaded.len(),
            interrupted,
            "journal loaded (recorded pgids are never signaled)"
        );
        let journal = Arc::new(Journal::new(journal_path));

        let home = dirs::home_dir().unwrap_or_else(|| PathBuf::from("/"));
        let tool_env = Arc::new(RwLock::new(ToolEnv::from_entries(
            home.clone(),
            static_entries(&home),
            crate::ipc::PathSource::StaticFallback,
        )));
        let settings = Arc::new(RwLock::new(settings));
        let detection: Arc<RwLock<Option<DetectionOutcome>>> = Arc::new(RwLock::new(None));

        // Auto re-refresh factory: builds a refresh submission from the
        // CURRENT detection + settings + ToolEnv.
        let factory: RefreshFactory = {
            let detection = detection.clone();
            let settings = settings.clone();
            let tool_env = tool_env.clone();
            Arc::new(move |id| {
                let det = detection.read().expect("detection poisoned");
                let status = det.as_ref()?.statuses.get(&id)?.clone();
                let settings = settings.read().expect("settings poisoned").clone();
                let env = tool_env.read().expect("tool_env poisoned").clone();
                drop(det);
                queue::make_refresh_submission(id, &status, &settings, &env)
            })
        };
        // SPEC §5.3 "re-checked each refresh": every parsed refresh snapshot
        // re-resolves the subject's self-update route from its own listing
        // (the in-band override — npm outdated in its own list must yield
        // `npm install -g npm@latest`, never `mise upgrade npm`, D5).
        let route_recheck: RouteRecheck = {
            let detection = detection.clone();
            let registry = registry.clone();
            let sink = sink.clone();
            Arc::new(move |snapshot: &crate::ipc::ManagerSnapshot| {
                let report = {
                    let mut det = detection.write().expect("detection poisoned");
                    let Some(outcome) = det.as_mut() else { return };
                    if !detect::recheck_route_from_snapshot(outcome, snapshot) {
                        return;
                    }
                    registry.set_routes_from(&outcome.report);
                    outcome.report.clone()
                };
                sink.emit(AppEvent::DetectionUpdated(report));
            })
        };
        let queue = Queue::new(QueueDeps {
            runner: runner.clone(),
            sink: sink.clone(),
            registry: registry.clone(),
            journal,
            ops_dir: logging::operations_dir(),
            refresh_factory: Some(factory),
            route_recheck: Some(route_recheck),
            max_concurrency: queue::MAX_CONCURRENCY,
            aging_guard: queue::AGING_GUARD,
        });

        AppState {
            settings,
            settings_path: Arc::new(settings_path),
            tool_env,
            detection: detection.clone(),
            registry,
            queue,
            journal_records: Arc::new(RwLock::new(loaded)),
            runner,
            sink,
            logging: Arc::new(Mutex::new(logging_handle)),
        }
    }

    /// Async startup (SPEC §5.12): build the probed ToolEnv, run detection,
    /// store routes, emit `detection:updated`. The window is already showing;
    /// the frontend renders skeletons until this lands.
    pub async fn startup(&self) {
        let env = ToolEnv::build().await;
        let outcome = detect::detect_all(&env, self.runner.as_ref()).await;
        *self.tool_env.write().expect("tool_env poisoned") = env;
        self.registry.set_routes_from(&outcome.report);
        let report = outcome.report.clone();
        *self.detection.write().expect("detection poisoned") = Some(outcome);
        self.sink.emit(AppEvent::DetectionUpdated(report));
    }

    /// Quit-guard kill hook: cancel every running op (SIGTERM → 5s → SIGKILL
    /// on the process groups) so children never outlive the app.
    ///
    /// `cancel_all` only flips the tokens — the SIGTERM/SIGKILL work happens
    /// inside each op's runner task on the async runtime. This hook runs on
    /// the main thread during `RunEvent::Exit`, so it BLOCKS (bounded by
    /// [`SHUTDOWN_GRACE`]) until those tasks report terminal states;
    /// returning immediately would let the process exit before any signal is
    /// sent and reparent live children to launchd (SPEC F7 violation).
    pub fn shutdown(&self) {
        let running = self.queue.running().len();
        self.queue.cancel_all();
        if running == 0 {
            return;
        }
        tracing::info!(running, "app exit: cancelling running operations");
        let queue = self.queue.clone();
        let done =
            tauri::async_runtime::block_on(
                async move { queue.wait_until_idle(SHUTDOWN_GRACE).await },
            );
        if done {
            tracing::info!("app exit: all operations finalized");
        } else {
            tracing::warn!("app exit: shutdown grace elapsed with operations still running");
        }
    }
}
