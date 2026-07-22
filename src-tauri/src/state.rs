//! Managed application state — completed by U5 (SPEC §5.12 startup sequence).
//!
//! `AppState` is `Clone` (all shared pieces are `Arc`s) so the async startup
//! task and the Tauri managed-state cell can hold the same instance.

use std::collections::{HashMap, VecDeque};
use std::path::PathBuf;
use std::sync::{Arc, Mutex, RwLock};

use crate::app_update::AppUpdater;
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

/// Maximum number of unconsumed upgrade-plan capabilities retained in one
/// app session. Oldest-first eviction keeps memory bounded and makes an
/// evicted `planId` fail closed just like an unknown or replayed one.
pub const ISSUED_PLAN_LIMIT: usize = 64;

#[derive(Debug, Clone)]
pub struct IssuedPlan {
    pub request: crate::ipc::PlanRequest,
    pub plan: crate::ipc::UpgradePlan,
    /// Canonical-state revision against which this capability was built.
    pub revision: u64,
}

/// Session-local, one-use plan capabilities.
#[derive(Debug)]
pub struct IssuedPlanStore {
    limit: usize,
    order: VecDeque<String>,
    plans: HashMap<String, IssuedPlan>,
}

/// One transaction boundary for every input that can change an upgrade plan.
///
/// Queue record mutations, refresh publication, redetection, settings, plan
/// issuance, and atomic plan submission all take this mutex. The revision is
/// deliberately monotonic for the process lifetime: even if an operation
/// finishes before another prebuilt plan is validated, the earlier batch's
/// enqueue permanently invalidated that second capability.
#[derive(Debug, Default)]
pub struct PlanCoordinator {
    revision: u64,
    state_updates_in_progress: u32,
    issued_plans: IssuedPlanStore,
}

impl PlanCoordinator {
    pub fn revision(&self) -> u64 {
        self.revision
    }

    pub fn bump_revision(&mut self) {
        self.revision = self
            .revision
            .checked_add(1)
            .expect("plan revision exhausted");
    }

    pub fn begin_state_update(&mut self) {
        self.state_updates_in_progress = self
            .state_updates_in_progress
            .checked_add(1)
            .expect("state update counter exhausted");
        self.bump_revision();
    }

    pub fn finish_state_update(&mut self) {
        self.state_updates_in_progress = self
            .state_updates_in_progress
            .checked_sub(1)
            .expect("state update finished without a matching begin");
        self.bump_revision();
    }

    pub fn state_update_in_progress(&self) -> bool {
        self.state_updates_in_progress != 0
    }

    pub fn insert_plan(&mut self, issued: IssuedPlan) {
        self.issued_plans.insert(issued);
    }

    pub fn take_plan(&mut self, plan_id: &str) -> Option<IssuedPlan> {
        self.issued_plans.take(plan_id)
    }
}

/// Cancellation-safe ownership of one in-progress canonical state update.
/// The lease holds no mutex while async detection runs. Successful callers
/// publish through [`StateUpdateLease::complete`]; dropping a pending future
/// clears the in-progress barrier and advances the revision so old plans still
/// fail closed without permanently wedging future execution.
struct StateUpdateLease {
    coordinator: Arc<Mutex<PlanCoordinator>>,
    active: bool,
}

impl StateUpdateLease {
    fn begin(coordinator: Arc<Mutex<PlanCoordinator>>) -> Self {
        coordinator
            .lock()
            .expect("plan coordinator poisoned")
            .begin_state_update();
        Self {
            coordinator,
            active: true,
        }
    }

    fn complete(mut self, publish: impl FnOnce()) {
        let coordinator = self.coordinator.clone();
        let mut coordinator = coordinator.lock().expect("plan coordinator poisoned");
        publish();
        coordinator.finish_state_update();
        self.active = false;
    }
}

impl Drop for StateUpdateLease {
    fn drop(&mut self) {
        if self.active {
            self.coordinator
                .lock()
                .expect("plan coordinator poisoned")
                .finish_state_update();
            self.active = false;
        }
    }
}

impl Default for IssuedPlanStore {
    fn default() -> Self {
        Self::new(ISSUED_PLAN_LIMIT)
    }
}

impl IssuedPlanStore {
    pub fn new(limit: usize) -> Self {
        Self {
            limit,
            order: VecDeque::new(),
            plans: HashMap::new(),
        }
    }

    pub fn insert(&mut self, issued: IssuedPlan) {
        let plan_id = issued.plan.plan_id.clone();
        if self.limit == 0 {
            return;
        }
        if self.plans.remove(&plan_id).is_some() {
            self.order.retain(|id| id != &plan_id);
        }
        while self.plans.len() >= self.limit {
            if let Some(oldest) = self.order.pop_front() {
                self.plans.remove(&oldest);
            } else {
                break;
            }
        }
        self.order.push_back(plan_id.clone());
        self.plans.insert(plan_id, issued);
    }

    /// Atomically consumes a capability. Failed validation never restores it,
    /// so tampering and stale state cannot be retried with the same plan.
    pub fn take(&mut self, plan_id: &str) -> Option<IssuedPlan> {
        let issued = self.plans.remove(plan_id)?;
        self.order.retain(|id| id != plan_id);
        Some(issued)
    }

    #[cfg(test)]
    fn len(&self) -> usize {
        self.plans.len()
    }
}

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
    /// Pack-Manager updating itself (DECISIONS D25) — deliberately outside the
    /// queue: it holds no manager lock and is not an `Operation`.
    pub app_update: Arc<AppUpdater>,
    /// Coherent canonical-state revision plus bounded, one-use plan
    /// capabilities. Queue batch admission shares this exact mutex.
    pub plan_coordinator: Arc<Mutex<PlanCoordinator>>,
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
        let plan_coordinator = Arc::new(Mutex::new(PlanCoordinator::default()));

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
            plan_coordinator: plan_coordinator.clone(),
            max_concurrency: queue::MAX_CONCURRENCY,
            aging_guard: queue::AGING_GUARD,
        });

        let app_update = Arc::new(AppUpdater::new(env!("CARGO_PKG_VERSION"), sink.clone()));

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
            app_update,
            plan_coordinator,
        }
    }

    /// Async startup (SPEC §5.12): build the probed ToolEnv, run detection,
    /// store routes, emit `detection:updated`. The window is already showing;
    /// the frontend renders skeletons until this lands.
    pub async fn startup(&self) {
        self.redetect(ToolEnv::build().await).await;
    }

    /// Re-runs detection against `env` and publishes the outcome: stores the
    /// ToolEnv, rebinds the registry's routes, stores the outcome, emits
    /// `detection:updated`. The single detection path shared by startup,
    /// `detect_managers` (Re-detect) and `refresh_all` (SPEC F2 — Refresh All
    /// re-detects first so a manager installed or removed mid-session is
    /// picked up by the same click). Idempotent: re-running with an unchanged
    /// machine state stores and emits an equivalent report.
    ///
    /// No lock guard is held across an await: a cancellation-safe lease marks
    /// the update, detection runs, then the coordinator is reacquired briefly
    /// for coherent publication.
    pub async fn redetect(&self, env: ToolEnv) -> DetectionOutcome {
        // Reserve a new epoch before the async probe. Plans issued while the
        // machine is being re-detected may still be previewed, but execution
        // fails closed until the coherent publication below completes.
        let update = StateUpdateLease::begin(self.plan_coordinator.clone());
        let outcome = detect::detect_all(&env, self.runner.as_ref()).await;
        let report = outcome.report.clone();
        update.complete(|| {
            *self.tool_env.write().expect("tool_env poisoned") = env;
            self.registry.set_routes_from(&outcome.report);
            *self.detection.write().expect("detection poisoned") = Some(outcome.clone());
        });
        self.sink.emit(AppEvent::DetectionUpdated(report));
        outcome
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ipc::{PlanRequest, UpgradePlan};

    fn issued(id: &str) -> IssuedPlan {
        let request = PlanRequest {
            selection: None,
            include_self_updates: false,
            include_greedy_casks: false,
        };
        IssuedPlan {
            request: request.clone(),
            revision: 0,
            plan: UpgradePlan {
                plan_id: id.into(),
                request,
                groups: vec![],
                excluded: vec![],
                notes: vec![],
                warnings: vec![],
            },
        }
    }

    #[test]
    fn issued_plan_store_is_bounded_oldest_first_and_one_use() {
        let mut store = IssuedPlanStore::new(2);
        store.insert(issued("one"));
        store.insert(issued("two"));
        store.insert(issued("three"));

        assert_eq!(store.len(), 2);
        assert!(store.take("one").is_none(), "oldest capability is evicted");
        assert!(store.take("two").is_some());
        assert!(store.take("two").is_none(), "capability cannot be replayed");
        assert!(store.take("three").is_some());
        assert_eq!(store.len(), 0);
    }
}
