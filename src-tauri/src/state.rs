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
use crate::queue::{self, Queue, QueueDeps, RefreshFactory};
use crate::registry::Registry;
use crate::settings::Settings;

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
        let queue = Queue::new(QueueDeps {
            runner: runner.clone(),
            sink: sink.clone(),
            registry: registry.clone(),
            journal,
            ops_dir: logging::operations_dir(),
            refresh_factory: Some(factory),
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
    pub fn shutdown(&self) {
        let running = self.queue.running().len();
        if running > 0 {
            tracing::info!(running, "app exit: cancelling running operations");
        }
        self.queue.cancel_all();
    }
}
