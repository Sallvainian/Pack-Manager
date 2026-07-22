//! Lock-set operation scheduler (SPEC §5.7) — implemented by U5.
//!
//! A single tokio task owns `pending`/`held`/`running`; lock acquisition is
//! atomic inside that task, so no deadlock is possible and no ordered
//! acquisition is needed. FIFO with skip-ahead, disabled past any op that has
//! waited longer than the aging guard (120s). Global `Semaphore(4)`.
//! Duplicate `refresh_manager` submissions coalesce to the existing opId.
//!
//! Lock rules (DECISIONS D4): base `{executor}`; routed self-updates add the
//! subject; npm/uv ops add `Mise` when the executor binary is mise-managed.
//!
//! Also home of the pure plan builder (`build_upgrade_plan`) and the
//! submission builders that bind adapter `PlannedCommand`s to absolute
//! programs + constructed env (`CommandSpec`).

use std::collections::{BTreeMap, BTreeSet, HashMap, VecDeque};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::time::Duration;

use tokio::sync::{mpsc, oneshot, OwnedSemaphorePermit, Semaphore};
use tokio::time::Instant;
use tokio_util::sync::CancellationToken;
use uuid::Uuid;

use crate::detect::{adapter_for, DetectStatus};
use crate::error::{IpcError, PmError};
use crate::events::{
    AppEvent, BatchingEmitter, EventSink, OpStalledEvent, OpStatusEvent, SnapshotUpdatedEvent,
};
use crate::ipc::{
    self, DetectionReport, ExcludeReason, ExcludedPackage, LogLine, ManagedBy, ManagerId,
    ManagerSnapshot, OpStatus, OperationRecord, PlanCommand, PlanGroup, PlanRequest, StreamKind,
    UpgradePlan,
};
use crate::journal::{FinishRecord, Journal, StartRecord};
use crate::managers::{
    brew, ExitClass, ManagerAdapter, PlanOptions, PlannedCommand, Timeout as CmdTimeout,
};
use crate::ops::{self, OpKind, Operation, Transcript, TranscriptHeader};
use crate::paths::ToolEnv;
use crate::process::{CmdPurpose, CommandOutput, CommandRunner, CommandSpec, LineSink};
use crate::registry::{now_rfc3339, Registry};
use crate::settings::Settings;
use crate::state::PlanCoordinator;

/// Global concurrency cap (16GB machine — SPEC §5.7).
pub const MAX_CONCURRENCY: usize = 4;
/// Skip-ahead is disabled past any op that has waited this long.
pub const AGING_GUARD: Duration = Duration::from_secs(120);
/// In-memory log ring buffer cap per op (SPEC §5.9 OperationDetail).
pub const RING_CAP: usize = 5000;

// ---------------------------------------------------------------------------
// Lock sets & command binding
// ---------------------------------------------------------------------------

/// The op's lock set (SPEC §5.7): `{executor}` ∪ `{subject}` (routed
/// self-updates hold both), plus `Mise` for npm/uv executors whose binary is
/// mise-managed (shared-tree guard).
pub fn lock_set(
    executor: ManagerId,
    subject: ManagerId,
    executor_managed_by: ManagedBy,
) -> BTreeSet<ManagerId> {
    let mut locks = BTreeSet::new();
    locks.insert(executor);
    locks.insert(subject);
    if matches!(executor, ManagerId::Npm | ManagerId::Uv) && executor_managed_by == ManagedBy::Mise
    {
        locks.insert(ManagerId::Mise);
    }
    locks
}

/// A `PlannedCommand` bound to an absolute program + constructed env.
#[derive(Debug, Clone)]
pub struct BoundCommand {
    pub planned: PlannedCommand,
    pub spec: CommandSpec,
}

/// Binds adapter plans to specs. `Timeout::Stall` values are re-bound to the
/// live settings (adapters emit F11 defaults); `extra_env` is appended to the
/// constructed base env.
pub fn bind_commands(
    planned: Vec<PlannedCommand>,
    program: &Path,
    base_env: &[(String, String)],
    settings: &Settings,
    purpose: CmdPurpose,
) -> Vec<BoundCommand> {
    planned
        .into_iter()
        .map(|p| {
            let mut env = base_env.to_vec();
            env.extend(p.extra_env.iter().cloned());
            let timeout = match p.timeout {
                CmdTimeout::Absolute(d) => CmdTimeout::Absolute(d),
                CmdTimeout::Stall { .. } => CmdTimeout::Stall {
                    silence: Duration::from_secs(settings.stall_after_secs),
                    hard_cap: Duration::from_secs(settings.upgrade_hard_cap_mins * 60),
                },
            };
            BoundCommand {
                spec: CommandSpec {
                    program: program.to_path_buf(),
                    args: p.argv.clone(),
                    env,
                    timeout,
                    purpose,
                },
                planned: p,
            }
        })
        .collect()
}

// ---------------------------------------------------------------------------
// Op submissions (built by commands.rs helpers below, consumed by the queue)
// ---------------------------------------------------------------------------

pub struct OpSubmission {
    pub kind: OpKind,
    pub executor: ManagerId,
    pub subject: ManagerId,
    pub locks: BTreeSet<ManagerId>,
    pub commands: Vec<BoundCommand>,
    /// The EXECUTOR's adapter (classify_exit; parse/recovery for refreshes).
    pub adapter: Arc<dyn ManagerAdapter>,
    /// Constructed child env WITHOUT per-command extra_env (recovery binding).
    pub base_env: Vec<(String, String)>,
    pub program: PathBuf,
}

fn present_parts(det: &DetectStatus) -> Option<(&PathBuf, ManagedBy)> {
    match det {
        DetectStatus::Present {
            binary_path,
            managed_by,
            ..
        } => Some((binary_path, *managed_by)),
        DetectStatus::Absent { .. } => None,
    }
}

/// Refresh submission for one present manager. `None` when absent (an absent
/// manager plans nothing — the runner is never invoked).
pub fn make_refresh_submission(
    id: ManagerId,
    det: &DetectStatus,
    settings: &Settings,
    env: &ToolEnv,
) -> Option<OpSubmission> {
    let (binary_path, managed_by) = present_parts(det)?;
    let adapter = adapter_for(id);
    let planned = adapter.refresh_plan(det, settings);
    if planned.is_empty() {
        return None;
    }
    let base_env = env.child_env();
    let commands = bind_commands(
        planned,
        binary_path,
        &base_env,
        settings,
        CmdPurpose::Refresh,
    );
    Some(OpSubmission {
        kind: OpKind::Refresh,
        executor: id,
        subject: id,
        locks: lock_set(id, id, managed_by),
        commands,
        adapter,
        base_env,
        program: binary_path.clone(),
    })
}

/// Upgrade submission: exact argv from the executor's pure `upgrade_plan`.
pub fn make_upgrade_submission(
    executor: ManagerId,
    package_ids: &[String],
    include_greedy_casks: bool,
    det: &DetectStatus,
    settings: &Settings,
    env: &ToolEnv,
) -> Result<OpSubmission, PmError> {
    let (binary_path, managed_by) = present_parts(det).ok_or_else(|| PmError::ToolNotFound {
        tool: executor.as_str().to_string(),
        searched: env
            .entries
            .iter()
            .map(|p| p.to_string_lossy().into_owned())
            .collect(),
    })?;
    let adapter = adapter_for(executor);
    let opts = PlanOptions {
        include_self_updates: false,
        include_greedy_casks,
    };
    let ids: Vec<String> = package_ids.to_vec();
    let planned = adapter.upgrade_plan(&ids, &opts);
    if planned.is_empty() {
        return Err(PmError::Internal {
            detail: format!("{executor}: empty upgrade plan for {ids:?}"),
        });
    }
    let base_env = env.child_env();
    let commands = bind_commands(
        planned,
        binary_path,
        &base_env,
        settings,
        CmdPurpose::Upgrade,
    );
    Ok(OpSubmission {
        kind: OpKind::Upgrade { package_ids: ids },
        executor,
        subject: executor,
        locks: lock_set(executor, executor, managed_by),
        commands,
        adapter,
        base_env,
        program: binary_path.clone(),
    })
}

fn stall_default() -> CmdTimeout {
    CmdTimeout::Stall {
        silence: Duration::from_secs(120),
        hard_cap: Duration::from_secs(1800),
    }
}

/// Self-update submission from a resolved route. `ViaRefresh` is the caller's
/// job (enqueue a refresh); `Unavailable` errors with the route's reason.
pub fn make_self_update_submission(
    subject: ManagerId,
    route: &ipc::SelfUpdateRoute,
    statuses: &BTreeMap<ManagerId, DetectStatus>,
    settings: &Settings,
    env: &ToolEnv,
) -> Result<OpSubmission, PmError> {
    let (executor, preview) = match route {
        ipc::SelfUpdateRoute::InBand {
            command_preview, ..
        } => (subject, command_preview.clone()),
        ipc::SelfUpdateRoute::Routed {
            executor,
            command_preview,
            ..
        } => (*executor, command_preview.clone()),
        ipc::SelfUpdateRoute::ViaRefresh { .. } => {
            return Err(PmError::Internal {
                detail: "ViaRefresh self-update is enqueued as a refresh by the caller".into(),
            })
        }
        ipc::SelfUpdateRoute::Unavailable { reason } => {
            return Err(PmError::SelfUpdateUnavailable {
                reason: reason.clone(),
            })
        }
    };
    let args = route
        .command_args()
        .filter(|args| !args.is_empty())
        .ok_or_else(|| PmError::Internal {
            detail: format!("self-update route for {subject} has no trusted argv"),
        })?;
    let expected_preview = ipc::command_preview(adapter_for(executor).binary_name(), args);
    if preview != expected_preview {
        return Err(PmError::Internal {
            detail: format!(
                "self-update preview/argv mismatch for {subject}: expected `{expected_preview}`"
            ),
        });
    }
    let det = statuses
        .get(&executor)
        .ok_or_else(|| PmError::SelfUpdateUnavailable {
            reason: format!("executor {executor} is not detected"),
        })?;
    let (binary_path, managed_by) =
        present_parts(det).ok_or_else(|| PmError::SelfUpdateUnavailable {
            reason: format!("executor {executor} is not installed"),
        })?;
    let extra_env = if executor == ManagerId::Brew {
        vec![("HOMEBREW_NO_AUTO_UPDATE".to_string(), "1".to_string())]
    } else {
        vec![]
    };
    let planned = vec![PlannedCommand {
        label: "self-update",
        argv: args.to_vec(),
        timeout: stall_default(),
        extra_env,
        phase_label: None,
    }];
    let base_env = env.child_env();
    let commands = bind_commands(
        planned,
        binary_path,
        &base_env,
        settings,
        CmdPurpose::SelfUpdate,
    );
    Ok(OpSubmission {
        kind: OpKind::SelfUpdate,
        executor,
        subject,
        locks: lock_set(executor, subject, managed_by),
        commands,
        adapter: adapter_for(executor),
        base_env,
        program: binary_path.clone(),
    })
}

/// Health-fix submission (F13): runs the issue's fix command on the manager's
/// own lane.
pub fn make_health_fix_submission(
    manager: ManagerId,
    issue: &ipc::HealthIssue,
    det: &DetectStatus,
    settings: &Settings,
    env: &ToolEnv,
) -> Result<OpSubmission, PmError> {
    let (binary_path, managed_by) = present_parts(det).ok_or_else(|| PmError::ToolNotFound {
        tool: manager.as_str().to_string(),
        searched: vec![],
    })?;
    if issue.manager_id != manager || !issue.fixable {
        return Err(PmError::Internal {
            detail: format!("health issue {} is not runnable for {manager}", issue.id),
        });
    }
    let fix = issue
        .fix_command
        .as_ref()
        .ok_or_else(|| PmError::Internal {
            detail: format!("health issue {} has no fix command", issue.id),
        })?;
    let args = issue
        .fix_args
        .as_deref()
        .filter(|args| !args.is_empty())
        .ok_or_else(|| PmError::Internal {
            detail: format!("health issue {} has no trusted fix argv", issue.id),
        })?;
    let expected_fix = ipc::command_preview(adapter_for(manager).binary_name(), args);
    if fix != &expected_fix {
        return Err(PmError::Internal {
            detail: format!(
                "health issue {} preview/argv mismatch: expected `{expected_fix}`",
                issue.id
            ),
        });
    }
    let planned = vec![PlannedCommand {
        label: "health fix",
        argv: args.to_vec(),
        timeout: stall_default(),
        extra_env: vec![],
        phase_label: None,
    }];
    let base_env = env.child_env();
    let commands = bind_commands(
        planned,
        binary_path,
        &base_env,
        settings,
        CmdPurpose::HealthFix,
    );
    Ok(OpSubmission {
        kind: OpKind::HealthFix {
            issue_id: issue.id.clone(),
        },
        executor: manager,
        subject: manager,
        locks: lock_set(manager, manager, managed_by),
        commands,
        adapter: adapter_for(manager),
        base_env,
        program: binary_path.clone(),
    })
}

// ---------------------------------------------------------------------------
// Plan builder (SPEC §5.7 — PURE; the trust device)
// ---------------------------------------------------------------------------

pub const RUST_DEDUP_NOTE: &str = "rust toolchains are handled by rustup in this plan";
/// Defensive IPC bounds: comfortably above a legitimate full-machine plan,
/// while preventing an untrusted frontend from making canonicalization or
/// cache storage scale without limit. At both ceilings the raw package-id
/// payload is at most 1 MiB per request; even the request's intentional second
/// copy inside the cached plan stays bounded across the 64-entry cache.
pub const MAX_PLAN_SELECTIONS: usize = 2_048;
pub const MAX_PLAN_PACKAGE_ID_BYTES: usize = 512;

/// Validates and canonicalizes the untrusted plan intent before it is used or
/// cached. Exact duplicate selections are removed first-seen-order, so they
/// cannot duplicate argv or interfere with cross-manager deduplication rules.
pub fn canonicalize_plan_request(mut request: PlanRequest) -> Result<PlanRequest, PmError> {
    let Some(selection) = request.selection.take() else {
        return Ok(request);
    };
    if selection.len() > MAX_PLAN_SELECTIONS {
        return Err(PmError::Internal {
            detail: format!(
                "plan selection contains {} entries; maximum is {MAX_PLAN_SELECTIONS}",
                selection.len()
            ),
        });
    }

    let mut seen = BTreeSet::new();
    let mut canonical = Vec::with_capacity(selection.len());
    for item in selection {
        if item.package_id.len() > MAX_PLAN_PACKAGE_ID_BYTES {
            return Err(PmError::Internal {
                detail: format!(
                    "plan package id for {} is {} bytes; maximum is {MAX_PLAN_PACKAGE_ID_BYTES}",
                    item.manager_id,
                    item.package_id.len()
                ),
            });
        }
        let key = (item.manager_id, item.package_id.clone());
        if seen.insert(key) {
            canonical.push(item);
        }
    }
    request.selection = Some(canonical);
    Ok(request)
}

/// Pure inputs for [`build_upgrade_plan`].
pub struct PlanSources<'a> {
    pub report: &'a DetectionReport,
    pub snapshots: &'a [ManagerSnapshot],
    /// Package ids already part of a queued/running upgrade (`alreadyRunning`).
    pub busy: &'a BTreeSet<(ManagerId, String)>,
    /// Managers whose last check errored ("list may be stale").
    pub stale: &'a BTreeSet<ManagerId>,
}

/// `alreadyRunning` exclusions source: package ids of queued/running upgrades.
pub fn busy_package_ids(records: &[OperationRecord]) -> BTreeSet<(ManagerId, String)> {
    records
        .iter()
        .filter(|r| {
            r.kind == ipc::OpKind::Upgrade
                && matches!(r.status, OpStatus::Queued | OpStatus::Running)
        })
        .flat_map(|r| r.package_ids.iter().map(move |id| (r.subject, id.clone())))
        .collect()
}

/// Staleness source: the most recent refresh per subject failed or timed out.
pub fn stale_managers(records: &[OperationRecord]) -> BTreeSet<ManagerId> {
    let mut last: BTreeMap<ManagerId, OpStatus> = BTreeMap::new();
    for r in records {
        if r.kind == ipc::OpKind::Refresh {
            last.insert(r.subject, r.status);
        }
    }
    last.into_iter()
        .filter(|(_, s)| matches!(s, OpStatus::Failed | OpStatus::TimedOut))
        .map(|(id, _)| id)
        .collect()
}

/// PURE plan preview (SPEC F4, §5.7): expands the selection (or all outdated
/// when `null`), excludes pinned/greedy/already-running with reasons, applies
/// the rust-dedup rule with a visible note, previews exact argv per manager,
/// and appends self-update groups when opted in.
pub fn build_upgrade_plan(req: &PlanRequest, src: &PlanSources) -> UpgradePlan {
    let snap_by: BTreeMap<ManagerId, &ManagerSnapshot> =
        src.snapshots.iter().map(|s| (s.manager_id, s)).collect();
    let managed_by_of = |id: ManagerId| -> ManagedBy {
        src.report
            .managers
            .iter()
            .find(|m| m.id == id)
            .map(|m| m.managed_by)
            .unwrap_or(ManagedBy::Standalone)
    };

    // 1. Candidates in a stable order.
    let mut candidates: Vec<(ManagerId, &ipc::Package)> = Vec::new();
    match &req.selection {
        None => {
            for id in ManagerId::ALL {
                if let Some(snap) = snap_by.get(&id) {
                    for p in snap.packages.iter().filter(|p| p.outdated) {
                        candidates.push((id, p));
                    }
                }
            }
        }
        Some(selection) => {
            for sel in selection {
                if let Some(snap) = snap_by.get(&sel.manager_id) {
                    if let Some(p) = snap.packages.iter().find(|p| p.id == sel.package_id) {
                        candidates.push((sel.manager_id, p));
                    }
                }
            }
        }
    }

    // 2. Exclusions.
    let mut included: BTreeMap<ManagerId, Vec<String>> = BTreeMap::new();
    let mut excluded: Vec<ExcludedPackage> = Vec::new();
    for (id, p) in candidates {
        if p.pinned {
            excluded.push(ExcludedPackage {
                manager_id: id,
                package_id: p.id.clone(),
                reason: ExcludeReason::Pinned,
            });
            continue;
        }
        if p.kind == ipc::PackageKind::CaskGreedy && !req.include_greedy_casks {
            excluded.push(ExcludedPackage {
                manager_id: id,
                package_id: p.id.clone(),
                reason: ExcludeReason::GreedyCask,
            });
            continue;
        }
        if src.busy.contains(&(id, p.id.clone())) {
            excluded.push(ExcludedPackage {
                manager_id: id,
                package_id: p.id.clone(),
                reason: ExcludeReason::AlreadyRunning,
            });
            continue;
        }
        if !p.outdated {
            continue; // up-to-date rows are never upgradable
        }
        included.entry(id).or_default().push(p.id.clone());
    }

    // 3. rust-dedup (DECISIONS D10): one plan never races two upgrades of the
    // same toolchain.
    let mut notes: Vec<String> = Vec::new();
    let rustup_present = included
        .get(&ManagerId::Rustup)
        .is_some_and(|v| !v.is_empty());
    if rustup_present {
        if let Some(mise_ids) = included.get_mut(&ManagerId::Mise) {
            if let Some(pos) = mise_ids.iter().position(|id| id == "tool:rust") {
                mise_ids.remove(pos);
                excluded.push(ExcludedPackage {
                    manager_id: ManagerId::Mise,
                    package_id: "tool:rust".into(),
                    reason: ExcludeReason::RustDedup,
                });
                notes.push(RUST_DEDUP_NOTE.to_string());
            }
        }
    }

    // 4. Package groups in ManagerId order.
    let mut groups: Vec<PlanGroup> = Vec::new();
    for id in ManagerId::ALL {
        let Some(ids) = included.get(&id) else {
            continue;
        };
        if ids.is_empty() {
            continue;
        }
        let adapter = adapter_for(id);
        let opts = PlanOptions {
            include_self_updates: req.include_self_updates,
            include_greedy_casks: req.include_greedy_casks,
        };
        let commands: Vec<PlanCommand> = adapter
            .upgrade_plan(ids, &opts)
            .into_iter()
            .map(|p| PlanCommand {
                argv_preview: ipc::command_preview(adapter.binary_name(), &p.argv),
                label: p.label.to_string(),
            })
            .collect();
        groups.push(PlanGroup {
            subject: id,
            executor: id,
            locks: lock_set(id, id, managed_by_of(id)).into_iter().collect(),
            commands,
            package_ids: ids.clone(),
            self_update: false,
        });
    }

    // 5. Self-update groups (toggle default on — SPEC F4).
    if req.include_self_updates {
        for m in &src.report.managers {
            let update_known = snap_by
                .get(&m.id)
                .and_then(|s| s.self_status.as_ref())
                .is_some_and(|s| s.update_available);
            if !update_known {
                continue;
            }
            match &m.self_update {
                ipc::SelfUpdateRoute::InBand {
                    command_preview, ..
                } => groups.push(PlanGroup {
                    subject: m.id,
                    executor: m.id,
                    locks: lock_set(m.id, m.id, managed_by_of(m.id))
                        .into_iter()
                        .collect(),
                    commands: vec![PlanCommand {
                        argv_preview: command_preview.clone(),
                        label: format!("Self-update {}", m.display_name),
                    }],
                    package_ids: vec![],
                    self_update: true,
                }),
                ipc::SelfUpdateRoute::Routed {
                    executor,
                    command_preview,
                    ..
                } => groups.push(PlanGroup {
                    subject: m.id,
                    executor: *executor,
                    locks: lock_set(*executor, m.id, managed_by_of(*executor))
                        .into_iter()
                        .collect(),
                    commands: vec![PlanCommand {
                        argv_preview: command_preview.clone(),
                        label: format!(
                            "Self-update {} via {}",
                            m.display_name,
                            adapter_for(*executor).display_name()
                        ),
                    }],
                    package_ids: vec![],
                    self_update: true,
                }),
                // brew updates via refresh; unavailable routes have no command.
                _ => {}
            }
        }
    }

    // 6. Staleness warnings for managers participating in the plan.
    let mut warnings: Vec<String> = Vec::new();
    for id in ManagerId::ALL {
        if src.stale.contains(&id) && groups.iter().any(|g| g.subject == id || g.executor == id) {
            warnings.push(format!("{id}: list may be stale — last check errored"));
        }
    }

    UpgradePlan {
        plan_id: Uuid::now_v7().to_string(),
        request: req.clone(),
        groups,
        excluded,
        notes,
        warnings,
    }
}

// ---------------------------------------------------------------------------
// Scheduler
// ---------------------------------------------------------------------------

pub type RefreshFactory = Arc<dyn Fn(ManagerId) -> Option<OpSubmission> + Send + Sync>;

/// Invoked with every freshly parsed refresh snapshot BEFORE it is published:
/// SPEC §5.3 routes are "re-checked each refresh" with the manager's own
/// listing (`detect::recheck_route_from_snapshot` behind AppState's locks).
pub type RouteRecheck = Arc<dyn Fn(&ManagerSnapshot) + Send + Sync>;

pub struct QueueDeps {
    pub runner: Arc<dyn CommandRunner>,
    pub sink: Arc<dyn EventSink>,
    pub registry: Arc<Registry>,
    pub journal: Arc<Journal>,
    /// Transcript directory (SPEC §6.1 `…/Logs/Pack-Manager/operations`).
    pub ops_dir: PathBuf,
    /// Builds the auto re-refresh submission after successful upgrades.
    pub refresh_factory: Option<RefreshFactory>,
    /// Re-checks the subject's self-update route from a fresh snapshot.
    pub route_recheck: Option<RouteRecheck>,
    /// Shared canonical-state epoch used by plan issue/validation and queue
    /// admission. Scheduler mutations take this before touching records.
    pub plan_coordinator: Arc<Mutex<PlanCoordinator>>,
    pub max_concurrency: usize,
    pub aging_guard: Duration,
}

struct Ring {
    lines: VecDeque<LogLine>,
    overflow: u64,
}

#[derive(Default)]
struct RecordStore {
    order: Vec<String>,
    by_id: HashMap<String, OperationRecord>,
}

#[derive(Default)]
struct Shared {
    records: Mutex<RecordStore>,
    buffers: Mutex<HashMap<String, Ring>>,
    tokens: Mutex<HashMap<String, CancellationToken>>,
}

impl Shared {
    fn push_line(&self, op_id: &str, line: LogLine) {
        let mut buffers = self.buffers.lock().expect("buffers poisoned");
        let ring = buffers.entry(op_id.to_string()).or_insert_with(|| Ring {
            lines: VecDeque::new(),
            overflow: 0,
        });
        if ring.lines.len() >= RING_CAP {
            ring.lines.pop_front();
            ring.overflow += 1;
        }
        ring.lines.push_back(line);
    }

    fn update_record(&self, op_id: &str, f: impl FnOnce(&mut OperationRecord)) {
        let mut records = self.records.lock().expect("records poisoned");
        if let Some(r) = records.by_id.get_mut(op_id) {
            f(r);
        }
    }
}

enum Msg {
    Submit {
        sub: Box<OpSubmission>,
        reply: Option<oneshot::Sender<String>>,
    },
    SubmitPlanBatch {
        subs: Vec<OpSubmission>,
        expected_revision: u64,
        reply: oneshot::Sender<Result<Vec<String>, PlanBatchError>>,
    },
    Cancel {
        op_id: String,
    },
    Finished {
        op_id: String,
        status: OpStatus,
        exit_code: Option<i32>,
        error: Option<PmError>,
    },
}

/// Fail-closed outcomes from the scheduler's atomic plan admission check.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlanBatchError {
    RevisionChanged,
    ActiveRefresh,
    MutatingOperationConflict,
    StateUpdateInProgress,
    SchedulerGone,
}

/// Handle to the scheduler task.
pub struct Queue {
    tx: mpsc::UnboundedSender<Msg>,
    shared: Arc<Shared>,
}

impl Queue {
    /// Spawns the scheduler task. Must be called inside a tokio runtime.
    pub fn new(deps: QueueDeps) -> Arc<Queue> {
        let shared = Arc::new(Shared::default());
        let (tx, rx) = mpsc::unbounded_channel();
        let emitter = Arc::new(BatchingEmitter::new(deps.sink.clone()));
        tokio::spawn(scheduler_task(
            rx,
            tx.clone(),
            deps,
            shared.clone(),
            emitter,
        ));
        Arc::new(Queue { tx, shared })
    }

    /// Enqueues an op; resolves to its opId (the EXISTING opId when a
    /// duplicate refresh coalesces).
    pub async fn submit(&self, sub: OpSubmission) -> Result<String, PmError> {
        let (rtx, rrx) = oneshot::channel();
        self.tx
            .send(Msg::Submit {
                sub: Box::new(sub),
                reply: Some(rtx),
            })
            .map_err(|_| PmError::Internal {
                detail: "scheduler is gone".into(),
            })?;
        rrx.await.map_err(|_| PmError::Internal {
            detail: "scheduler dropped the submission".into(),
        })
    }

    /// Atomically checks the canonical revision and enqueues every operation
    /// in a reviewed plan, or enqueues none. A successful batch advances the
    /// revision exactly once, invalidating all other prebuilt capabilities.
    pub async fn submit_plan_batch(
        &self,
        subs: Vec<OpSubmission>,
        expected_revision: u64,
    ) -> Result<Vec<String>, PlanBatchError> {
        let (rtx, rrx) = oneshot::channel();
        self.tx
            .send(Msg::SubmitPlanBatch {
                subs,
                expected_revision,
                reply: rtx,
            })
            .map_err(|_| PlanBatchError::SchedulerGone)?;
        rrx.await.map_err(|_| PlanBatchError::SchedulerGone)?
    }

    /// Cancels a running (SIGTERM→5s→SIGKILL) or queued op. Unknown/finished
    /// ids are a no-op.
    pub fn cancel(&self, op_id: &str) {
        let _ = self.tx.send(Msg::Cancel {
            op_id: op_id.to_string(),
        });
    }

    /// Session records in submission order.
    pub fn records(&self) -> Vec<OperationRecord> {
        let store = self.shared.records.lock().expect("records poisoned");
        store
            .order
            .iter()
            .filter_map(|id| store.by_id.get(id).cloned())
            .collect()
    }

    pub fn record(&self, op_id: &str) -> Option<OperationRecord> {
        self.shared
            .records
            .lock()
            .expect("records poisoned")
            .by_id
            .get(op_id)
            .cloned()
    }

    /// Ring-buffer replay for one op: `(lines, truncated)`.
    pub fn lines(&self, op_id: &str) -> (Vec<LogLine>, bool) {
        let buffers = self.shared.buffers.lock().expect("buffers poisoned");
        match buffers.get(op_id) {
            Some(ring) => (ring.lines.iter().cloned().collect(), ring.overflow > 0),
            None => (Vec::new(), false),
        }
    }

    /// Currently running ops (quit guard).
    pub fn running(&self) -> Vec<OperationRecord> {
        self.records()
            .into_iter()
            .filter(|r| r.status == OpStatus::Running)
            .collect()
    }

    /// Cancels every running op (quit-guard kill hook). Only flips the
    /// tokens — the runner tasks perform the SIGTERM→grace→SIGKILL work, so
    /// the quit path must ALSO await [`Queue::wait_until_idle`] before the
    /// process exits or the kill tasks may never be polled.
    pub fn cancel_all(&self) {
        for token in self.shared.tokens.lock().expect("tokens poisoned").values() {
            token.cancel();
        }
    }

    /// Waits (bounded) until no op is `Running`. The quit guard calls this
    /// after [`Queue::cancel_all`] so the runner tasks' SIGTERM → grace →
    /// SIGKILL escalation demonstrably completes before the process exits —
    /// children never outlive the app (SPEC F7). Returns `false` when the
    /// timeout elapsed with ops still running.
    pub async fn wait_until_idle(&self, timeout: Duration) -> bool {
        let deadline = Instant::now() + timeout;
        loop {
            if self.running().is_empty() {
                return true;
            }
            if Instant::now() >= deadline {
                return false;
            }
            tokio::time::sleep(Duration::from_millis(25)).await;
        }
    }
}

struct Pending {
    op: Operation,
    planned: Vec<PlannedCommand>,
    adapter: Arc<dyn ManagerAdapter>,
    base_env: Vec<(String, String)>,
    enqueued_at: Instant,
}

struct RunningInfo {
    locks: BTreeSet<ManagerId>,
    _permit: OwnedSemaphorePermit,
    kind: ipc::OpKind,
    executor: ManagerId,
    subject: ManagerId,
}

struct Sched {
    tx: mpsc::UnboundedSender<Msg>,
    deps: QueueDeps,
    shared: Arc<Shared>,
    emitter: Arc<BatchingEmitter>,
    pending: VecDeque<Pending>,
    held: BTreeSet<ManagerId>,
    running: HashMap<String, RunningInfo>,
    semaphore: Arc<Semaphore>,
    /// Queued/running refresh per subject → opId (coalescing).
    active_refresh: HashMap<ManagerId, String>,
}

async fn scheduler_task(
    mut rx: mpsc::UnboundedReceiver<Msg>,
    tx: mpsc::UnboundedSender<Msg>,
    deps: QueueDeps,
    shared: Arc<Shared>,
    emitter: Arc<BatchingEmitter>,
) {
    let semaphore = Arc::new(Semaphore::new(deps.max_concurrency.max(1)));
    let mut sched = Sched {
        tx,
        deps,
        shared,
        emitter,
        pending: VecDeque::new(),
        held: BTreeSet::new(),
        running: HashMap::new(),
        semaphore,
        active_refresh: HashMap::new(),
    };
    while let Some(msg) = rx.recv().await {
        match msg {
            Msg::Submit { sub, reply } => sched.handle_submit(*sub, reply),
            Msg::SubmitPlanBatch {
                subs,
                expected_revision,
                reply,
            } => sched.handle_plan_batch(subs, expected_revision, reply),
            Msg::Cancel { op_id } => sched.handle_cancel(&op_id),
            Msg::Finished {
                op_id,
                status,
                exit_code,
                error,
            } => sched.handle_finished(&op_id, status, exit_code, error),
        }
        sched.try_start_all_coordinated();
    }
}

impl Sched {
    fn status_event(&self, r: &OperationRecord, queue_position: Option<u32>) -> OpStatusEvent {
        OpStatusEvent {
            op_id: r.op_id.clone(),
            kind: r.kind,
            executor: r.executor,
            subject: r.subject,
            status: r.status,
            queue_position,
            phase_label: None,
            command_line: r.command_line.clone(),
            exit_code: r.exit_code,
            error: r.error.clone(),
            started_at: r.started_at.clone(),
            finished_at: r.finished_at.clone(),
            log_path: r.log_path.clone(),
        }
    }

    fn handle_submit(&mut self, sub: OpSubmission, reply: Option<oneshot::Sender<String>>) {
        let coordinator = self.deps.plan_coordinator.clone();
        let mut coordinator = coordinator.lock().expect("plan coordinator poisoned");
        let (op_id, created) = self.enqueue_submission(sub);
        if created {
            coordinator.bump_revision();
        }
        if let Some(reply) = reply {
            let _ = reply.send(op_id);
        }
    }

    fn handle_plan_batch(
        &mut self,
        subs: Vec<OpSubmission>,
        expected_revision: u64,
        reply: oneshot::Sender<Result<Vec<String>, PlanBatchError>>,
    ) {
        let coordinator = self.deps.plan_coordinator.clone();
        let mut coordinator = coordinator.lock().expect("plan coordinator poisoned");
        if coordinator.revision() != expected_revision {
            let _ = reply.send(Err(PlanBatchError::RevisionChanged));
            return;
        }
        if coordinator.state_update_in_progress() {
            let _ = reply.send(Err(PlanBatchError::StateUpdateInProgress));
            return;
        }
        if !self.active_refresh.is_empty() {
            let _ = reply.send(Err(PlanBatchError::ActiveRefresh));
            return;
        }
        if self.has_existing_mutating_lock_conflict(&subs) {
            let _ = reply.send(Err(PlanBatchError::MutatingOperationConflict));
            return;
        }

        // enqueue_submission is infallible after construction, so no record
        // or event is published until every admission predicate above has
        // passed. The scheduler task owns all queue internals, making this
        // whole loop one atomic message relative to every other submission.
        let mut op_ids = Vec::with_capacity(subs.len());
        for sub in subs {
            let (op_id, _) = self.enqueue_submission(sub);
            op_ids.push(op_id);
        }
        coordinator.bump_revision();
        let _ = reply.send(Ok(op_ids));
    }

    /// Incoming groups may overlap one another—the scheduler serializes those
    /// within the accepted batch. They may not queue behind an earlier direct
    /// mutation, because that mutation's completion/auto-refresh must publish
    /// a new canonical revision before another reviewed plan can be accepted.
    fn has_existing_mutating_lock_conflict(&self, subs: &[OpSubmission]) -> bool {
        let incoming: BTreeSet<ManagerId> = subs
            .iter()
            .flat_map(|sub| sub.locks.iter().copied())
            .collect();
        let mutating = |kind: ipc::OpKind| {
            matches!(
                kind,
                ipc::OpKind::Upgrade | ipc::OpKind::SelfUpdate | ipc::OpKind::HealthFix
            )
        };

        self.pending.iter().any(|pending| {
            mutating(pending.op.kind.wire()) && !incoming.is_disjoint(&pending.op.locks)
        }) || self
            .running
            .values()
            .any(|running| mutating(running.kind) && !incoming.is_disjoint(&running.locks))
    }

    /// Inserts one already-constructed submission. Returns `(op_id, created)`;
    /// a duplicate refresh coalesces and therefore does not mutate state.
    fn enqueue_submission(&mut self, sub: OpSubmission) -> (String, bool) {
        // Duplicate refresh coalesces to the existing opId (SPEC §5.7).
        if matches!(sub.kind, OpKind::Refresh) {
            if let Some(existing) = self.active_refresh.get(&sub.subject) {
                tracing::debug!(subject = %sub.subject, op = %existing, "refresh coalesced");
                return (existing.clone(), false);
            }
        }

        let id = Uuid::now_v7();
        let op_id = id.to_string();
        let queued_at = now_rfc3339();
        let file_name = ops::transcript_file_name(
            time::OffsetDateTime::now_utc(),
            &op_id,
            sub.subject,
            sub.kind.wire(),
        );
        let log_path = self.deps.ops_dir.join(file_name);
        let specs: Vec<CommandSpec> = sub.commands.iter().map(|c| c.spec.clone()).collect();
        let planned: Vec<PlannedCommand> = sub.commands.iter().map(|c| c.planned.clone()).collect();
        let command_line = ops::command_line_of(&specs);

        let record = OperationRecord {
            op_id: op_id.clone(),
            kind: sub.kind.wire(),
            executor: sub.executor,
            subject: sub.subject,
            status: OpStatus::Queued,
            command_line,
            package_ids: sub.kind.package_ids(),
            queued_at,
            started_at: None,
            finished_at: None,
            exit_code: None,
            error: None,
            log_path: log_path.to_string_lossy().into_owned(),
        };
        {
            let mut store = self.shared.records.lock().expect("records poisoned");
            store.order.push(op_id.clone());
            store.by_id.insert(op_id.clone(), record.clone());
        }
        if matches!(sub.kind, OpKind::Refresh) {
            self.active_refresh.insert(sub.subject, op_id.clone());
        }
        let position = self.pending.len() as u32;
        self.deps.sink.emit(AppEvent::OpStatus(
            self.status_event(&record, Some(position)),
        ));
        tracing::info!(
            op = %op_id,
            kind = ?record.kind,
            executor = %record.executor,
            subject = %record.subject,
            "op queued"
        );

        self.pending.push_back(Pending {
            op: Operation {
                id,
                kind: sub.kind,
                executor: sub.executor,
                subject: sub.subject,
                locks: sub.locks,
                specs,
                cancel: CancellationToken::new(),
                log_path,
            },
            planned,
            adapter: sub.adapter,
            base_env: sub.base_env,
            enqueued_at: Instant::now(),
        });
        (op_id, true)
    }

    fn handle_cancel(&mut self, op_id: &str) {
        let coordinator = self.deps.plan_coordinator.clone();
        let mut coordinator = coordinator.lock().expect("plan coordinator poisoned");
        // Running: cancel the token; the runner SIGTERMs the group and the op
        // task reports Cancelled through the normal Finished path.
        if let Some(token) = self
            .shared
            .tokens
            .lock()
            .expect("tokens poisoned")
            .get(op_id)
        {
            token.cancel();
            return;
        }
        // Queued: remove and finalize as Cancelled (it never started — no
        // journal lines, no transcript).
        if let Some(pos) = self
            .pending
            .iter()
            .position(|p| p.op.id.to_string() == op_id)
        {
            let pending = self.pending.remove(pos).expect("position just found");
            if matches!(pending.op.kind, OpKind::Refresh) {
                self.remove_active_refresh(pending.op.subject, op_id);
            }
            self.shared.update_record(op_id, |r| {
                r.status = OpStatus::Cancelled;
                r.finished_at = Some(now_rfc3339());
            });
            if let Some(r) = self
                .shared
                .records
                .lock()
                .expect("records poisoned")
                .by_id
                .get(op_id)
                .cloned()
            {
                self.deps
                    .sink
                    .emit(AppEvent::OpStatus(self.status_event(&r, None)));
            }
            coordinator.bump_revision();
        }
    }

    fn remove_active_refresh(&mut self, subject: ManagerId, op_id: &str) {
        if self
            .active_refresh
            .get(&subject)
            .is_some_and(|id| id == op_id)
        {
            self.active_refresh.remove(&subject);
        }
    }

    fn handle_finished(
        &mut self,
        op_id: &str,
        status: OpStatus,
        exit_code: Option<i32>,
        error: Option<PmError>,
    ) {
        let coordinator = self.deps.plan_coordinator.clone();
        let mut coordinator = coordinator.lock().expect("plan coordinator poisoned");
        let Some(info) = self.running.remove(op_id) else {
            return;
        };
        for lock in &info.locks {
            self.held.remove(lock);
        }
        self.shared
            .tokens
            .lock()
            .expect("tokens poisoned")
            .remove(op_id);
        if info.kind == ipc::OpKind::Refresh {
            self.remove_active_refresh(info.subject, op_id);
        }

        let finished_at = now_rfc3339();
        let log_path = self
            .shared
            .records
            .lock()
            .expect("records poisoned")
            .by_id
            .get(op_id)
            .map(|r| r.log_path.clone())
            .unwrap_or_default();
        let ipc_error = error.map(|e| {
            IpcError::from(e)
                .with_manager(info.subject)
                .with_op(op_id)
                .with_log_path(&log_path)
        });
        self.shared.update_record(op_id, |r| {
            r.status = status;
            r.exit_code = exit_code;
            r.finished_at = Some(finished_at.clone());
            r.error = ipc_error.clone();
        });
        self.deps.journal.record_finish(&FinishRecord {
            op_id: op_id.to_string(),
            outcome: status,
            exit_code,
            finished_at,
        });
        if let Some(r) = self
            .shared
            .records
            .lock()
            .expect("records poisoned")
            .by_id
            .get(op_id)
            .cloned()
        {
            tracing::info!(op = %op_id, status = ?status, exit = ?exit_code, "op finished");
            self.deps
                .sink
                .emit(AppEvent::OpStatus(self.status_event(&r, None)));
        }

        // Successful upgrades auto-enqueue a refresh of the affected managers.
        if status == OpStatus::Succeeded
            && matches!(
                info.kind,
                ipc::OpKind::Upgrade | ipc::OpKind::SelfUpdate | ipc::OpKind::HealthFix
            )
        {
            if let Some(factory) = self.deps.refresh_factory.clone() {
                let mut targets = vec![info.subject];
                if info.executor != info.subject {
                    targets.push(info.executor);
                }
                for target in targets {
                    if let Some(sub) = factory(target) {
                        self.enqueue_submission(sub);
                    }
                }
            }
        }
        coordinator.bump_revision();
    }

    fn try_start_all_coordinated(&mut self) {
        let coordinator = self.deps.plan_coordinator.clone();
        let _coordinator = coordinator.lock().expect("plan coordinator poisoned");
        self.try_start_all();
    }

    fn try_start_all(&mut self) {
        let mut i = 0;
        while i < self.pending.len() {
            let can_start = self.pending[i].op.locks.is_disjoint(&self.held);
            if can_start {
                match self.semaphore.clone().try_acquire_owned() {
                    Ok(permit) => {
                        let pending = self.pending.remove(i).expect("index in bounds");
                        self.start(pending, permit);
                        continue; // re-check the same index
                    }
                    Err(_) => break, // concurrency cap reached
                }
            }
            // Blocked: the aging guard disables skip-ahead past an op that has
            // waited too long (starvation guard — SPEC §5.7).
            if self.pending[i].enqueued_at.elapsed() >= self.deps.aging_guard {
                tracing::debug!(
                    op = %self.pending[i].op.id,
                    "aging guard: skip-ahead disabled past this op"
                );
                break;
            }
            i += 1;
        }
    }

    fn start(&mut self, pending: Pending, permit: OwnedSemaphorePermit) {
        let op_id = pending.op.id.to_string();
        let started_at = now_rfc3339();
        for lock in &pending.op.locks {
            self.held.insert(*lock);
        }
        self.shared
            .tokens
            .lock()
            .expect("tokens poisoned")
            .insert(op_id.clone(), pending.op.cancel.clone());
        self.running.insert(
            op_id.clone(),
            RunningInfo {
                locks: pending.op.locks.clone(),
                _permit: permit,
                kind: pending.op.kind.wire(),
                executor: pending.op.executor,
                subject: pending.op.subject,
            },
        );
        self.shared.update_record(&op_id, |r| {
            r.status = OpStatus::Running;
            r.started_at = Some(started_at.clone());
        });
        let record = self
            .shared
            .records
            .lock()
            .expect("records poisoned")
            .by_id
            .get(&op_id)
            .cloned()
            .expect("record exists");
        self.deps.journal.record_start(&StartRecord {
            op_id: op_id.clone(),
            kind: record.kind,
            executor: record.executor,
            subject: record.subject,
            command_line: record.command_line.clone(),
            // The CommandRunner seam does not expose the child's pgid;
            // informational only — never signaled on startup (DECISIONS D12).
            pgid: 0,
            started_at: started_at.clone(),
            log_path: record.log_path.clone(),
        });
        tracing::info!(op = %op_id, command = %record.command_line, "op started");
        self.deps
            .sink
            .emit(AppEvent::OpStatus(self.status_event(&record, None)));

        let args = RunArgs {
            op: pending.op,
            planned: pending.planned,
            adapter: pending.adapter,
            base_env: pending.base_env,
            record,
            runner: self.deps.runner.clone(),
            sink: self.deps.sink.clone(),
            emitter: self.emitter.clone(),
            registry: self.deps.registry.clone(),
            route_recheck: self.deps.route_recheck.clone(),
            plan_coordinator: self.deps.plan_coordinator.clone(),
            shared: self.shared.clone(),
            tx: self.tx.clone(),
        };
        tokio::spawn(run_operation(args));
    }
}

// ---------------------------------------------------------------------------
// Op execution
// ---------------------------------------------------------------------------

struct RunArgs {
    op: Operation,
    planned: Vec<PlannedCommand>,
    adapter: Arc<dyn ManagerAdapter>,
    base_env: Vec<(String, String)>,
    record: OperationRecord,
    runner: Arc<dyn CommandRunner>,
    sink: Arc<dyn EventSink>,
    emitter: Arc<BatchingEmitter>,
    registry: Arc<Registry>,
    route_recheck: Option<RouteRecheck>,
    plan_coordinator: Arc<Mutex<PlanCoordinator>>,
    shared: Arc<Shared>,
    tx: mpsc::UnboundedSender<Msg>,
}

fn now_ms() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_millis() as u64)
        .unwrap_or(0)
}

fn stderr_tail(stderr: &str, lines: usize) -> String {
    let all: Vec<&str> = stderr.lines().collect();
    let start = all.len().saturating_sub(lines);
    all[start..].join("\n")
}

/// Feeds one line to the ring buffer, the transcript, and the batching
/// emitter (`op:output`).
fn deliver_line(
    shared: &Shared,
    transcript: &Mutex<Transcript>,
    emitter: &BatchingEmitter,
    op_id: &str,
    line: LogLine,
) {
    shared.push_line(op_id, line.clone());
    transcript.lock().expect("transcript poisoned").line(&line);
    emitter.push(op_id, line);
}

/// The queue-level stall watchdog: emits `op:stalled` after `silence` without
/// output, re-arms on output and after each firing. The op continues; only the
/// hard cap (enforced by the runner) kills.
fn spawn_stall_watchdog(
    op_id: String,
    silence: Duration,
    last_output: Arc<Mutex<Instant>>,
    sink: Arc<dyn EventSink>,
) -> tokio::task::JoinHandle<()> {
    tokio::spawn(async move {
        let mut next = *last_output.lock().expect("watchdog poisoned") + silence;
        loop {
            tokio::time::sleep_until(next).await;
            let lo = *last_output.lock().expect("watchdog poisoned");
            let rearmed = lo + silence;
            if rearmed > Instant::now() {
                next = rearmed; // output arrived — re-arm
            } else {
                let silent = Instant::now().duration_since(lo);
                tracing::warn!(op = %op_id, silent_secs = silent.as_secs(), "op stalled");
                sink.emit(AppEvent::OpStalled(OpStalledEvent {
                    op_id: op_id.clone(),
                    silent_for_secs: silent.as_secs(),
                }));
                next = Instant::now() + silence;
            }
        }
    })
}

enum SpecEnd {
    Ok(CommandOutput),
    Terminal {
        status: OpStatus,
        error: Option<PmError>,
        exit_code: Option<i32>,
    },
}

async fn run_operation(args: RunArgs) {
    let RunArgs {
        op,
        planned,
        adapter,
        base_env,
        record,
        runner,
        sink,
        emitter,
        registry,
        route_recheck,
        plan_coordinator,
        shared,
        tx,
    } = args;
    let op_id = op.id.to_string();
    let started = Instant::now();
    let is_refresh = matches!(op.kind, OpKind::Refresh);

    let transcript = match Transcript::create(&op.log_path) {
        Ok(t) => Arc::new(Mutex::new(t)),
        Err(e) => {
            tracing::error!(op = %op_id, error = %e, "transcript create failed");
            let _ = tx.send(Msg::Finished {
                op_id,
                status: OpStatus::Failed,
                exit_code: None,
                error: Some(e),
            });
            return;
        }
    };

    // Header (SPEC §6.2) from the first spec's constructed env.
    {
        let first = op.specs.first();
        let env = first.map(|s| s.env.as_slice()).unwrap_or(&[]);
        let cwd = env
            .iter()
            .find(|(k, _)| k == "HOME")
            .map(|(_, v)| v.clone())
            .unwrap_or_else(|| "-".to_string());
        let path = env
            .iter()
            .find(|(k, _)| k == "PATH")
            .map(|(_, v)| v.clone())
            .unwrap_or_default();
        let header = TranscriptHeader {
            op_id: op_id.clone(),
            kind: record.kind,
            executor: record.executor,
            subject: record.subject,
            queued_at: record.queued_at.clone(),
            started_at: record.started_at.clone().unwrap_or_default(),
            command_line: record.command_line.clone(),
            cwd,
            path,
            env_set: ops::env_set_string(env),
            timeout: first
                .map(|s| ops::timeout_string(&s.timeout))
                .unwrap_or_default(),
            pgid: 0, // the runner seam does not expose the child's pgid
        };
        transcript
            .lock()
            .expect("transcript poisoned")
            .header(&header);
    }

    let mut outputs: Vec<CommandOutput> = Vec::new();
    let mut end: Option<(OpStatus, Option<PmError>, Option<i32>)> = None;

    for (i, spec) in op.specs.iter().enumerate() {
        let cmd_planned = &planned[i];
        if let Some(label) = &cmd_planned.phase_label {
            sink.emit(AppEvent::OpStatus(OpStatusEvent {
                op_id: op_id.clone(),
                kind: record.kind,
                executor: record.executor,
                subject: record.subject,
                status: OpStatus::Running,
                queue_position: None,
                phase_label: Some(label.clone()),
                command_line: record.command_line.clone(),
                exit_code: None,
                error: None,
                started_at: record.started_at.clone(),
                finished_at: None,
                log_path: record.log_path.clone(),
            }));
        }
        if i > 0 {
            transcript
                .lock()
                .expect("transcript poisoned")
                .marker(&ops::spec_line(spec));
        }

        let result = run_one_spec(
            spec,
            is_refresh,
            &op_id,
            &op.cancel,
            runner.as_ref(),
            &shared,
            &transcript,
            &emitter,
            &sink,
        )
        .await;

        match result {
            SpecEnd::Ok(out) => {
                let class = adapter.classify_exit(cmd_planned, &out);
                tracing::debug!(op = %op_id, label = cmd_planned.label, class = ?class, exit = ?out.exit_code, "classify_exit");
                match class {
                    ExitClass::Success | ExitClass::ExpectedNonZero => outputs.push(out),
                    ExitClass::Failure => {
                        // A Failure classification aborts remaining specs.
                        let err = if op.executor == ManagerId::Brew {
                            brew::classify_brew_failure(&out)
                        } else {
                            None
                        }
                        .unwrap_or_else(|| PmError::NonZeroExit {
                            code: out.exit_code.unwrap_or(-1),
                            stderr_tail: stderr_tail(&out.stderr, 20),
                        });
                        end = Some((OpStatus::Failed, Some(err), out.exit_code));
                        break;
                    }
                }
            }
            SpecEnd::Terminal {
                status,
                error,
                exit_code,
            } => {
                end = Some((status, error, exit_code));
                break;
            }
        }
    }

    // Refresh parse + recovery + registry upsert.
    if end.is_none() && is_refresh {
        match adapter.parse_refresh(&outputs) {
            Ok(snapshot) => {
                // SPEC §5.3: routes are re-checked each refresh with the
                // manager's own listing — BEFORE the upsert so the join
                // uses fresh routed pairs.
                publish_refresh_snapshot(
                    &registry,
                    &sink,
                    route_recheck.as_ref(),
                    &plan_coordinator,
                    snapshot,
                )
            }
            Err(parse_err) => {
                tracing::error!(op = %op_id, error = %parse_err, "refresh parse failed");
                let recovery = planned
                    .iter()
                    .find_map(|p| adapter.recovery_plan(p).map(|r| (p.clone(), r)));
                match recovery {
                    Some((failed_planned, rec)) => {
                        let mut env = base_env.clone();
                        env.extend(rec.extra_env.iter().cloned());
                        let rec_spec = CommandSpec {
                            program: op.specs[0].program.clone(),
                            args: rec.argv.clone(),
                            env,
                            timeout: rec.timeout,
                            purpose: CmdPurpose::Refresh,
                        };
                        transcript
                            .lock()
                            .expect("transcript poisoned")
                            .marker(&ops::spec_line(&rec_spec));
                        let rec_end = run_one_spec(
                            &rec_spec,
                            true,
                            &op_id,
                            &op.cancel,
                            runner.as_ref(),
                            &shared,
                            &transcript,
                            &emitter,
                            &sink,
                        )
                        .await;
                        match rec_end {
                            SpecEnd::Ok(out) => {
                                // Recovery output is judged by the parser (npm's
                                // text form exits 1 when outdated), except a
                                // brew lock which is its own named state.
                                let lock_busy = (op.executor == ManagerId::Brew
                                    && out.exit_code != Some(0))
                                .then(|| brew::classify_brew_failure(&out))
                                .flatten();
                                if let Some(err) = lock_busy {
                                    end = Some((OpStatus::Failed, Some(err), out.exit_code));
                                } else {
                                    // The captured refresh outputs ride along
                                    // so recovery can rebuild the inventory
                                    // and merge (not replace) the snapshot.
                                    match adapter.parse_recovery(&failed_planned, &outputs, &out) {
                                        Ok(snapshot) => {
                                            outputs.push(out);
                                            publish_refresh_snapshot(
                                                &registry,
                                                &sink,
                                                route_recheck.as_ref(),
                                                &plan_coordinator,
                                                snapshot,
                                            );
                                        }
                                        Err(rec_err) => {
                                            end = Some((
                                                OpStatus::Failed,
                                                Some(rec_err),
                                                out.exit_code,
                                            ));
                                        }
                                    }
                                }
                            }
                            SpecEnd::Terminal {
                                status,
                                error,
                                exit_code,
                            } => end = Some((status, error, exit_code)),
                        }
                    }
                    None => {
                        end = Some((OpStatus::Failed, Some(parse_err), None));
                    }
                }
            }
        }
    }

    let (status, error, exit_code) = end.unwrap_or_else(|| {
        (
            OpStatus::Succeeded,
            None,
            outputs.last().and_then(|o| o.exit_code),
        )
    });

    let finished_at = now_rfc3339();
    transcript.lock().expect("transcript poisoned").footer(
        status,
        exit_code,
        started.elapsed(),
        &finished_at,
    );
    emitter.flush_op(&op_id);
    let _ = tx.send(Msg::Finished {
        op_id,
        status,
        exit_code,
        error,
    });
}

fn publish_snapshot(registry: &Registry, sink: &Arc<dyn EventSink>, snapshot: ManagerSnapshot) {
    for id in registry.upsert(snapshot) {
        if let Some(snap) = registry.get(id) {
            sink.emit(AppEvent::SnapshotUpdated(SnapshotUpdatedEvent {
                manager_id: id,
                snapshot: snap,
            }));
        }
    }
}

/// Publishes route re-evaluation and the joined snapshot as one canonical
/// epoch. No plan can observe one without the other, and every plan issued
/// before this refresh becomes stale before the operation reports finished.
fn publish_refresh_snapshot(
    registry: &Registry,
    sink: &Arc<dyn EventSink>,
    route_recheck: Option<&RouteRecheck>,
    plan_coordinator: &Arc<Mutex<PlanCoordinator>>,
    snapshot: ManagerSnapshot,
) {
    let mut coordinator = plan_coordinator.lock().expect("plan coordinator poisoned");
    if let Some(recheck) = route_recheck {
        recheck(&snapshot);
    }
    publish_snapshot(registry, sink, snapshot);
    coordinator.bump_revision();
}

#[allow(clippy::too_many_arguments)]
async fn run_one_spec(
    spec: &CommandSpec,
    buffered: bool,
    op_id: &str,
    cancel: &CancellationToken,
    runner: &dyn CommandRunner,
    shared: &Arc<Shared>,
    transcript: &Arc<Mutex<Transcript>>,
    emitter: &Arc<BatchingEmitter>,
    sink: &Arc<dyn EventSink>,
) -> SpecEnd {
    let result = if buffered {
        // The op's REAL cancel token rides into the buffered run too — a
        // refresh is cancellable (Cancel button, quit-time cancel_all) and
        // the runner SIGTERMs the group exactly like the streaming path.
        let res = runner.run(spec, cancel.clone()).await;
        if let Ok(out) = &res {
            // Buffered commands (refresh/detection) land in the transcript,
            // ring buffer, and drawer after completion.
            for line in out.stdout.lines() {
                deliver_line(
                    shared,
                    transcript,
                    emitter,
                    op_id,
                    LogLine {
                        stream: StreamKind::Out,
                        line: line.to_string(),
                        ts_ms: now_ms(),
                    },
                );
            }
            for line in out.stderr.lines() {
                deliver_line(
                    shared,
                    transcript,
                    emitter,
                    op_id,
                    LogLine {
                        stream: StreamKind::Err,
                        line: line.to_string(),
                        ts_ms: now_ms(),
                    },
                );
            }
        }
        res
    } else {
        let last_output = Arc::new(Mutex::new(Instant::now()));
        let watchdog = match spec.timeout {
            CmdTimeout::Stall { silence, .. } => Some(spawn_stall_watchdog(
                op_id.to_string(),
                silence,
                last_output.clone(),
                sink.clone(),
            )),
            CmdTimeout::Absolute(_) => None,
        };
        let line_sink: LineSink = {
            let shared = shared.clone();
            let transcript = transcript.clone();
            let emitter = emitter.clone();
            let op_id = op_id.to_string();
            let last_output = last_output.clone();
            Arc::new(move |line: LogLine| {
                *last_output.lock().expect("watchdog poisoned") = Instant::now();
                deliver_line(&shared, &transcript, &emitter, &op_id, line);
            })
        };
        let res = runner.run_streaming(spec, line_sink, cancel.clone()).await;
        if let Some(w) = watchdog {
            w.abort();
        }
        res
    };

    match result {
        Ok(out) => SpecEnd::Ok(out),
        Err(PmError::Cancelled) => SpecEnd::Terminal {
            status: OpStatus::Cancelled,
            error: Some(PmError::Cancelled),
            exit_code: None,
        },
        Err(e @ PmError::Timeout { .. }) => SpecEnd::Terminal {
            status: OpStatus::TimedOut,
            error: Some(e),
            exit_code: None,
        },
        Err(e) => SpecEnd::Terminal {
            status: OpStatus::Failed,
            error: Some(e),
            exit_code: None,
        },
    }
}

// ---------------------------------------------------------------------------
// Tests (SPEC §7.3 + plan builder + lock sets) — paused time, zero sleeps
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::events::VecSink;
    use crate::ipc::{
        EnvInfo, ErrorCode, HealthIssue, HealthSeverity, ManagerInfo, ManagerStatus, Package,
        PackageKind, PathSource, PlanSelection, SelfStatus, SelfUpdateRoute,
    };
    use crate::process::fake::FakeRunner;
    use tokio::sync::Notify;

    // ---------------- shared helpers ----------------

    struct Harness {
        queue: Arc<Queue>,
        fake: Arc<FakeRunner>,
        sink: Arc<VecSink>,
        registry: Arc<Registry>,
        plan_coordinator: Arc<Mutex<PlanCoordinator>>,
        journal_path: PathBuf,
        _dir: tempfile::TempDir,
    }

    fn harness() -> Harness {
        harness_with(None, None)
    }

    fn harness_with_factory(factory: Option<RefreshFactory>) -> Harness {
        harness_with(factory, None)
    }

    fn harness_with(factory: Option<RefreshFactory>, recheck: Option<RouteRecheck>) -> Harness {
        let dir = tempfile::tempdir().unwrap();
        let fake = Arc::new(FakeRunner::new());
        let sink = Arc::new(VecSink::new());
        let registry = Arc::new(Registry::new());
        let plan_coordinator = Arc::new(Mutex::new(PlanCoordinator::default()));
        let journal_path = dir.path().join("operations.jsonl");
        let deps = QueueDeps {
            runner: fake.clone(),
            sink: sink.clone(),
            registry: registry.clone(),
            journal: Arc::new(Journal::new(journal_path.clone())),
            ops_dir: dir.path().join("ops"),
            refresh_factory: factory,
            route_recheck: recheck,
            plan_coordinator: plan_coordinator.clone(),
            max_concurrency: MAX_CONCURRENCY,
            aging_guard: AGING_GUARD,
        };
        Harness {
            queue: Queue::new(deps),
            fake,
            sink,
            registry,
            plan_coordinator,
            journal_path,
            _dir: dir,
        }
    }

    /// Deterministic wait under paused time: 1ms auto-advancing polls.
    async fn wait_for(mut cond: impl FnMut() -> bool) {
        for _ in 0..20_000 {
            if cond() {
                return;
            }
            tokio::time::sleep(Duration::from_millis(1)).await;
        }
        panic!("condition not met within the polling budget");
    }

    fn abs1h() -> CmdTimeout {
        CmdTimeout::Absolute(Duration::from_secs(3600))
    }

    /// Streaming (upgrade-shaped) submission with explicit locks.
    fn streaming_sub(
        executor: ManagerId,
        subject: ManagerId,
        locks: &[ManagerId],
        program: &str,
        args: &[&str],
        timeout: CmdTimeout,
        kind: OpKind,
    ) -> OpSubmission {
        let argv: Vec<String> = args.iter().map(|s| s.to_string()).collect();
        let planned = PlannedCommand {
            label: "test",
            argv: argv.clone(),
            timeout,
            extra_env: vec![],
            phase_label: None,
        };
        let spec = CommandSpec {
            program: PathBuf::from(program),
            args: argv,
            env: vec![
                ("PATH".into(), "/fake".into()),
                ("HOME".into(), "/Users/testuser".into()),
            ],
            timeout,
            purpose: CmdPurpose::Upgrade,
        };
        OpSubmission {
            kind,
            executor,
            subject,
            locks: locks.iter().copied().collect(),
            commands: vec![BoundCommand { planned, spec }],
            adapter: adapter_for(executor),
            base_env: vec![
                ("PATH".into(), "/fake".into()),
                ("HOME".into(), "/Users/testuser".into()),
            ],
            program: PathBuf::from(program),
        }
    }

    fn upgrade_sub(
        executor: ManagerId,
        locks: &[ManagerId],
        program: &str,
        args: &[&str],
    ) -> OpSubmission {
        streaming_sub(
            executor,
            executor,
            locks,
            program,
            args,
            abs1h(),
            OpKind::Upgrade {
                package_ids: vec!["formula:test".into()],
            },
        )
    }

    fn test_env() -> ToolEnv {
        ToolEnv::from_entries(
            PathBuf::from("/Users/testuser"),
            vec![],
            PathSource::StaticFallback,
        )
    }

    fn present(_id: ManagerId, program: &str, managed_by: ManagedBy) -> DetectStatus {
        DetectStatus::Present {
            binary_path: PathBuf::from(program),
            canonical_path: PathBuf::from(program),
            version: Some("1.0.0".into()),
            managed_by,
            evidence: format!("resolved at {program}"),
        }
    }

    fn refresh_sub(id: ManagerId, program: &str, managed_by: ManagedBy) -> OpSubmission {
        make_refresh_submission(
            id,
            &present(id, program, managed_by),
            &Settings::default(),
            &test_env(),
        )
        .expect("present manager yields a refresh submission")
    }

    fn status_of(h: &Harness, op_id: &str) -> OpStatus {
        h.queue.record(op_id).expect("record exists").status
    }

    fn stalled_events(sink: &VecSink) -> Vec<u64> {
        sink.events()
            .into_iter()
            .filter_map(|e| match e {
                AppEvent::OpStalled(s) => Some(s.silent_for_secs),
                _ => None,
            })
            .collect()
    }

    // ---------------- scheduler (SPEC §7.3) ----------------

    #[tokio::test(start_paused = true)]
    async fn two_brew_ops_never_overlap_fifo() {
        let h = harness();
        let g1 = Arc::new(Notify::new());
        let g2 = Arc::new(Notify::new());
        h.fake
            .on_streaming("brew", &["upgrade", "dolt"])
            .gate(g1.clone());
        h.fake
            .on_streaming("brew", &["upgrade", "abseil"])
            .gate(g2.clone());

        let id1 = h
            .queue
            .submit(upgrade_sub(
                ManagerId::Brew,
                &[ManagerId::Brew],
                "/fake/brew",
                &["upgrade", "dolt"],
            ))
            .await
            .unwrap();
        let id2 = h
            .queue
            .submit(upgrade_sub(
                ManagerId::Brew,
                &[ManagerId::Brew],
                "/fake/brew",
                &["upgrade", "abseil"],
            ))
            .await
            .unwrap();

        wait_for(|| h.fake.calls().len() == 1).await;
        assert_eq!(h.fake.calls()[0].args, vec!["upgrade", "dolt"], "FIFO");
        assert_eq!(status_of(&h, &id2), OpStatus::Queued, "brew lock held");

        g1.notify_one();
        wait_for(|| status_of(&h, &id1) == OpStatus::Succeeded).await;
        wait_for(|| h.fake.calls().len() == 2).await;
        g2.notify_one();
        wait_for(|| status_of(&h, &id2) == OpStatus::Succeeded).await;

        // Call-log intervals: op1 finished before op2 started.
        let calls = h.fake.calls();
        assert!(
            calls[0].finished_at.expect("finished") <= calls[1].started_at,
            "brew ops must serialize"
        );
    }

    #[tokio::test(start_paused = true)]
    async fn brew_and_mise_run_concurrently() {
        let h = harness();
        let g1 = Arc::new(Notify::new());
        let g2 = Arc::new(Notify::new());
        h.fake
            .on_streaming("brew", &["upgrade", "dolt"])
            .gate(g1.clone());
        h.fake
            .on_streaming("mise", &["upgrade", "deno"])
            .gate(g2.clone());

        let id1 = h
            .queue
            .submit(upgrade_sub(
                ManagerId::Brew,
                &[ManagerId::Brew],
                "/fake/brew",
                &["upgrade", "dolt"],
            ))
            .await
            .unwrap();
        let id2 = h
            .queue
            .submit(upgrade_sub(
                ManagerId::Mise,
                &[ManagerId::Mise],
                "/fake/mise",
                &["upgrade", "deno"],
            ))
            .await
            .unwrap();

        // Both in flight at once (disjoint locks).
        wait_for(|| h.fake.calls().len() == 2).await;
        assert_eq!(status_of(&h, &id1), OpStatus::Running);
        assert_eq!(status_of(&h, &id2), OpStatus::Running);

        g1.notify_one();
        g2.notify_one();
        wait_for(|| {
            status_of(&h, &id1) == OpStatus::Succeeded && status_of(&h, &id2) == OpStatus::Succeeded
        })
        .await;
    }

    #[tokio::test(start_paused = true)]
    async fn routed_self_update_blocks_subject_lane() {
        let h = harness();
        let g1 = Arc::new(Notify::new());
        let g3 = Arc::new(Notify::new());
        // brew upgrade mise (routed self-update): locks {Brew, Mise}.
        h.fake
            .on_streaming("brew", &["upgrade", "mise"])
            .gate(g1.clone());
        h.fake.on_streaming("mise", &["upgrade", "deno"]).exit(0);
        h.fake
            .on_streaming("rustup", &["update", "stable"])
            .gate(g3.clone());

        let id1 = h
            .queue
            .submit(streaming_sub(
                ManagerId::Brew,
                ManagerId::Mise,
                &[ManagerId::Brew, ManagerId::Mise],
                "/fake/brew",
                &["upgrade", "mise"],
                abs1h(),
                OpKind::SelfUpdate,
            ))
            .await
            .unwrap();
        let id2 = h
            .queue
            .submit(upgrade_sub(
                ManagerId::Mise,
                &[ManagerId::Mise],
                "/fake/mise",
                &["upgrade", "deno"],
            ))
            .await
            .unwrap();
        let id3 = h
            .queue
            .submit(upgrade_sub(
                ManagerId::Rustup,
                &[ManagerId::Rustup],
                "/fake/rustup",
                &["update", "stable"],
            ))
            .await
            .unwrap();

        // Unrelated rustup starts; the queued mise upgrade waits while mise's
        // binary is being replaced.
        wait_for(|| h.fake.calls().len() == 2).await;
        let basenames: Vec<String> = h.fake.calls().iter().map(|c| c.basename.clone()).collect();
        assert_eq!(basenames, vec!["brew", "rustup"]);
        assert_eq!(status_of(&h, &id2), OpStatus::Queued);
        assert_eq!(status_of(&h, &id3), OpStatus::Running);

        g1.notify_one();
        wait_for(|| status_of(&h, &id1) == OpStatus::Succeeded).await;
        wait_for(|| status_of(&h, &id2) == OpStatus::Succeeded).await;
        g3.notify_one();
        wait_for(|| status_of(&h, &id3) == OpStatus::Succeeded).await;

        let calls = h.fake.calls();
        let brew = &calls[0];
        let mise = calls.iter().find(|c| c.basename == "mise").unwrap();
        assert!(brew.finished_at.unwrap() <= mise.started_at);
    }

    #[tokio::test(start_paused = true)]
    async fn npm_op_blocks_mise_when_mise_managed() {
        let h = harness();
        let g1 = Arc::new(Notify::new());
        h.fake
            .on_streaming("npm", &["install", "-g", "typescript@latest"])
            .gate(g1.clone());
        h.fake.on_streaming("mise", &["upgrade", "deno"]).exit(0);

        // npm is mise-managed on this machine → the npm op holds {Npm, Mise}.
        let locks = lock_set(ManagerId::Npm, ManagerId::Npm, ManagedBy::Mise);
        assert_eq!(
            locks.iter().copied().collect::<Vec<_>>(),
            vec![ManagerId::Mise, ManagerId::Npm]
        );
        let id1 = h
            .queue
            .submit(streaming_sub(
                ManagerId::Npm,
                ManagerId::Npm,
                &[ManagerId::Npm, ManagerId::Mise],
                "/fake/npm",
                &["install", "-g", "typescript@latest"],
                abs1h(),
                OpKind::Upgrade {
                    package_ids: vec!["globalPackage:typescript".into()],
                },
            ))
            .await
            .unwrap();
        let id2 = h
            .queue
            .submit(upgrade_sub(
                ManagerId::Mise,
                &[ManagerId::Mise],
                "/fake/mise",
                &["upgrade", "deno"],
            ))
            .await
            .unwrap();

        wait_for(|| h.fake.calls().len() == 1).await;
        assert_eq!(status_of(&h, &id2), OpStatus::Queued, "Mise lock guarded");

        g1.notify_one();
        wait_for(|| status_of(&h, &id1) == OpStatus::Succeeded).await;
        wait_for(|| status_of(&h, &id2) == OpStatus::Succeeded).await;
        let calls = h.fake.calls();
        assert!(calls[0].finished_at.unwrap() <= calls[1].started_at);
    }

    #[tokio::test(start_paused = true)]
    async fn skip_ahead_starts_unblocked_op() {
        let h = harness();
        let g1 = Arc::new(Notify::new());
        let g3 = Arc::new(Notify::new());
        h.fake
            .on_streaming("brew", &["upgrade", "dolt"])
            .gate(g1.clone());
        h.fake.on_streaming("brew", &["upgrade", "abseil"]).exit(0);
        h.fake
            .on_streaming("mise", &["upgrade", "deno"])
            .gate(g3.clone());

        let _id1 = h
            .queue
            .submit(upgrade_sub(
                ManagerId::Brew,
                &[ManagerId::Brew],
                "/fake/brew",
                &["upgrade", "dolt"],
            ))
            .await
            .unwrap();
        let id2 = h
            .queue
            .submit(upgrade_sub(
                ManagerId::Brew,
                &[ManagerId::Brew],
                "/fake/brew",
                &["upgrade", "abseil"],
            ))
            .await
            .unwrap();
        let id3 = h
            .queue
            .submit(upgrade_sub(
                ManagerId::Mise,
                &[ManagerId::Mise],
                "/fake/mise",
                &["upgrade", "deno"],
            ))
            .await
            .unwrap();

        // op3 skips ahead of the lock-blocked op2.
        wait_for(|| h.fake.calls().iter().any(|c| c.basename == "mise")).await;
        assert_eq!(status_of(&h, &id2), OpStatus::Queued);
        assert_eq!(status_of(&h, &id3), OpStatus::Running);

        g1.notify_one();
        g3.notify_one();
        wait_for(|| status_of(&h, &id2) == OpStatus::Succeeded).await;
    }

    #[tokio::test(start_paused = true)]
    async fn aging_guard_blocks_skip_ahead_after_120s() {
        let h = harness();
        let g1 = Arc::new(Notify::new());
        let g3 = Arc::new(Notify::new());
        h.fake
            .on_streaming("brew", &["upgrade", "dolt"])
            .gate(g1.clone());
        h.fake.on_streaming("brew", &["upgrade", "abseil"]).exit(0);
        h.fake
            .on_streaming("mise", &["upgrade", "deno"])
            .gate(g3.clone());

        let _id1 = h
            .queue
            .submit(upgrade_sub(
                ManagerId::Brew,
                &[ManagerId::Brew],
                "/fake/brew",
                &["upgrade", "dolt"],
            ))
            .await
            .unwrap();
        let id2 = h
            .queue
            .submit(upgrade_sub(
                ManagerId::Brew,
                &[ManagerId::Brew],
                "/fake/brew",
                &["upgrade", "abseil"],
            ))
            .await
            .unwrap();
        wait_for(|| h.fake.calls().len() == 1).await;

        // op2 ages past the guard while blocked.
        tokio::time::sleep(Duration::from_secs(121)).await;
        let id3 = h
            .queue
            .submit(upgrade_sub(
                ManagerId::Mise,
                &[ManagerId::Mise],
                "/fake/mise",
                &["upgrade", "deno"],
            ))
            .await
            .unwrap();
        // Give the scheduler time: the mise op must NOT start (no skip-ahead
        // past the aged op2).
        tokio::time::sleep(Duration::from_millis(50)).await;
        assert!(
            h.fake.calls().iter().all(|c| c.basename != "mise"),
            "skip-ahead is disabled past an op that waited >120s"
        );
        assert_eq!(status_of(&h, &id3), OpStatus::Queued);

        // Releasing op1 unblocks op2; op3 then starts too.
        g1.notify_one();
        wait_for(|| status_of(&h, &id2) == OpStatus::Succeeded).await;
        wait_for(|| h.fake.calls().iter().any(|c| c.basename == "mise")).await;
        g3.notify_one();
        wait_for(|| status_of(&h, &id3) == OpStatus::Succeeded).await;

        let calls = h.fake.calls();
        let brew1 = &calls[0];
        let mise = calls.iter().find(|c| c.basename == "mise").unwrap();
        assert!(brew1.finished_at.unwrap() <= mise.started_at);
    }

    #[tokio::test(start_paused = true)]
    async fn semaphore_caps_concurrency_at_4() {
        let h = harness();
        let gates: Vec<Arc<Notify>> = (0..5).map(|_| Arc::new(Notify::new())).collect();
        let managers = [
            (ManagerId::Brew, "/fake/brew", "brew"),
            (ManagerId::Mise, "/fake/mise", "mise"),
            (ManagerId::Npm, "/fake/npm", "npm"),
            (ManagerId::Uv, "/fake/uv", "uv"),
            (ManagerId::Rustup, "/fake/rustup", "rustup"),
        ];
        for (i, (_, _, basename)) in managers.iter().enumerate() {
            h.fake
                .on_streaming(basename, &["upgrade", "x"])
                .gate(gates[i].clone());
        }
        let mut ids = Vec::new();
        for (id, program, _) in managers {
            ids.push(
                h.queue
                    .submit(upgrade_sub(id, &[id], program, &["upgrade", "x"]))
                    .await
                    .unwrap(),
            );
        }

        // Disjoint locks, but the global semaphore caps at 4.
        wait_for(|| h.fake.calls().len() == 4).await;
        tokio::time::sleep(Duration::from_millis(50)).await;
        assert_eq!(h.fake.calls().len(), 4, "5th op waits for a permit");
        assert_eq!(status_of(&h, &ids[4]), OpStatus::Queued);

        gates[0].notify_one();
        wait_for(|| h.fake.calls().len() == 5).await;
        for gate in &gates[1..] {
            gate.notify_one();
        }
        for id in &ids {
            wait_for(|| status_of(&h, id) == OpStatus::Succeeded).await;
        }
    }

    #[tokio::test(start_paused = true)]
    async fn duplicate_refresh_coalesces_to_same_opid() {
        let h = harness();
        let gate = Arc::new(Notify::new());
        // A gated op holds the Mise lock so the refresh stays queued.
        h.fake
            .on_streaming("mise", &["upgrade", "deno"])
            .gate(gate.clone());
        h.fake
            .on("mise", &["ls", "--json"])
            .fixture("mise_ls_2026-07-22.json");
        h.fake
            .on("mise", &["outdated", "--json"])
            .fixture("mise_outdated.json");

        let blocker = h
            .queue
            .submit(upgrade_sub(
                ManagerId::Mise,
                &[ManagerId::Mise],
                "/fake/mise",
                &["upgrade", "deno"],
            ))
            .await
            .unwrap();
        wait_for(|| h.fake.calls().len() == 1).await;

        let r1 = h
            .queue
            .submit(refresh_sub(ManagerId::Mise, "/fake/mise", ManagedBy::Brew))
            .await
            .unwrap();
        let r2 = h
            .queue
            .submit(refresh_sub(ManagerId::Mise, "/fake/mise", ManagedBy::Brew))
            .await
            .unwrap();
        assert_eq!(r1, r2, "duplicate refresh coalesces to the same opId");
        assert_eq!(h.queue.records().len(), 2, "blocker + ONE refresh");

        gate.notify_one();
        wait_for(|| status_of(&h, &blocker) == OpStatus::Succeeded).await;
        wait_for(|| status_of(&h, &r1) == OpStatus::Succeeded).await;

        // After the refresh finishes, a new submission is a new op.
        let r3 = h
            .queue
            .submit(refresh_sub(ManagerId::Mise, "/fake/mise", ManagedBy::Brew))
            .await
            .unwrap();
        assert_ne!(r1, r3);
        wait_for(|| status_of(&h, &r3) == OpStatus::Succeeded).await;
    }

    #[tokio::test(start_paused = true)]
    async fn plan_batch_revision_mismatch_enqueues_all_or_none() {
        let h = harness();
        let current_revision = h
            .plan_coordinator
            .lock()
            .expect("plan coordinator poisoned")
            .revision();
        let subs = vec![
            upgrade_sub(
                ManagerId::Brew,
                &[ManagerId::Brew],
                "/fake/brew",
                &["upgrade", "dolt"],
            ),
            upgrade_sub(
                ManagerId::Mise,
                &[ManagerId::Mise],
                "/fake/mise",
                &["upgrade", "deno"],
            ),
        ];

        let error = h
            .queue
            .submit_plan_batch(subs, current_revision + 1)
            .await
            .unwrap_err();

        assert_eq!(error, PlanBatchError::RevisionChanged);
        assert!(h.queue.records().is_empty(), "no partial batch is visible");
        assert!(h.fake.calls().is_empty(), "no command was started");
    }

    #[tokio::test(start_paused = true)]
    async fn plan_batch_rejects_existing_mutation_with_intersecting_locks() {
        let h = harness();
        let gate = Arc::new(Notify::new());
        h.fake
            .on_streaming("brew", &["upgrade", "other"])
            .gate(gate.clone())
            .exit(0);
        let direct = h
            .queue
            .submit(upgrade_sub(
                ManagerId::Brew,
                &[ManagerId::Brew],
                "/fake/brew",
                &["upgrade", "other"],
            ))
            .await
            .unwrap();
        wait_for(|| h.fake.calls().len() == 1).await;
        let current_revision = h
            .plan_coordinator
            .lock()
            .expect("plan coordinator poisoned")
            .revision();

        let error = h
            .queue
            .submit_plan_batch(
                vec![upgrade_sub(
                    ManagerId::Brew,
                    &[ManagerId::Brew],
                    "/fake/brew",
                    &["upgrade", "dolt"],
                )],
                current_revision,
            )
            .await
            .unwrap_err();

        assert_eq!(error, PlanBatchError::MutatingOperationConflict);
        assert_eq!(h.queue.records().len(), 1, "plan batch was not queued");
        assert_eq!(h.fake.calls().len(), 1, "plan command never started");

        gate.notify_one();
        wait_for(|| status_of(&h, &direct) == OpStatus::Succeeded).await;
    }

    #[tokio::test(start_paused = true)]
    async fn plan_batch_rejects_running_self_update_with_intersecting_locks() {
        let h = harness();
        let gate = Arc::new(Notify::new());
        h.fake
            .on_streaming("brew", &["upgrade", "mise"])
            .gate(gate.clone())
            .exit(0);
        let direct = h
            .queue
            .submit(streaming_sub(
                ManagerId::Brew,
                ManagerId::Mise,
                &[ManagerId::Brew, ManagerId::Mise],
                "/fake/brew",
                &["upgrade", "mise"],
                abs1h(),
                OpKind::SelfUpdate,
            ))
            .await
            .unwrap();
        wait_for(|| h.fake.calls().len() == 1).await;
        assert_eq!(status_of(&h, &direct), OpStatus::Running);
        let current_revision = h
            .plan_coordinator
            .lock()
            .expect("plan coordinator poisoned")
            .revision();

        let error = h
            .queue
            .submit_plan_batch(
                vec![upgrade_sub(
                    ManagerId::Brew,
                    &[ManagerId::Brew],
                    "/fake/brew",
                    &["upgrade", "dolt"],
                )],
                current_revision,
            )
            .await
            .unwrap_err();

        assert_eq!(error, PlanBatchError::MutatingOperationConflict);
        assert_eq!(h.queue.records().len(), 1, "plan batch was not queued");
        assert_eq!(h.fake.calls().len(), 1, "plan command never started");

        gate.notify_one();
        wait_for(|| status_of(&h, &direct) == OpStatus::Succeeded).await;
    }

    #[tokio::test(start_paused = true)]
    async fn plan_batch_rejects_queued_health_fix_with_intersecting_locks() {
        let h = harness();
        let blocker_gate = Arc::new(Notify::new());
        h.fake
            .on_streaming("brew", &["upgrade", "other"])
            .gate(blocker_gate.clone())
            .exit(0);
        h.fake
            .on_streaming("uv", &["tool", "install", "aider-chat", "--reinstall"])
            .exit(0);

        let blocker = h
            .queue
            .submit(upgrade_sub(
                ManagerId::Brew,
                &[ManagerId::Brew],
                "/fake/brew",
                &["upgrade", "other"],
            ))
            .await
            .unwrap();
        wait_for(|| h.fake.calls().len() == 1).await;
        let health_fix = h
            .queue
            .submit(streaming_sub(
                ManagerId::Uv,
                ManagerId::Uv,
                &[ManagerId::Brew, ManagerId::Uv],
                "/fake/uv",
                &["tool", "install", "aider-chat", "--reinstall"],
                abs1h(),
                OpKind::HealthFix {
                    issue_id: "uv:aider-chat".into(),
                },
            ))
            .await
            .unwrap();
        assert_eq!(status_of(&h, &health_fix), OpStatus::Queued);
        let current_revision = h
            .plan_coordinator
            .lock()
            .expect("plan coordinator poisoned")
            .revision();

        let error = h
            .queue
            .submit_plan_batch(
                vec![upgrade_sub(
                    ManagerId::Uv,
                    &[ManagerId::Uv],
                    "/fake/uv",
                    &["tool", "upgrade", "ruff"],
                )],
                current_revision,
            )
            .await
            .unwrap_err();

        assert_eq!(error, PlanBatchError::MutatingOperationConflict);
        assert_eq!(h.queue.records().len(), 2, "plan batch was not queued");
        assert_eq!(h.fake.calls().len(), 1, "plan command never started");
        assert_eq!(status_of(&h, &health_fix), OpStatus::Queued);

        blocker_gate.notify_one();
        wait_for(|| status_of(&h, &blocker) == OpStatus::Succeeded).await;
        wait_for(|| status_of(&h, &health_fix) == OpStatus::Succeeded).await;
    }

    #[tokio::test(start_paused = true)]
    async fn npm_exit_1_with_json_is_success() {
        let h = harness();
        h.fake
            .on("npm", &["ls", "-g", "--depth=0", "--json"])
            .fixture("npm_ls_g_2026-07-22.json");
        // npm exits 1 whenever outdated packages exist — still a success.
        h.fake
            .on("npm", &["outdated", "-g", "--json"])
            .fixture_with_exit("npm_outdated_g_synthetic.json", 1);

        let id = h
            .queue
            .submit(refresh_sub(ManagerId::Npm, "/fake/npm", ManagedBy::Mise))
            .await
            .unwrap();
        wait_for(|| status_of(&h, &id) == OpStatus::Succeeded).await;

        let snap = h.registry.get(ManagerId::Npm).expect("snapshot upserted");
        assert_eq!(snap.packages.len(), 14, "npm self row hoisted");
        assert!(snap.self_status.expect("self").update_available);
        assert!(
            h.sink.events().iter().any(|e| matches!(
                e,
                AppEvent::SnapshotUpdated(s) if s.manager_id == ManagerId::Npm
            )),
            "snapshot:updated emitted"
        );
        // The exit-1 rides through as the op's exit code, not an error.
        let record = h.queue.record(&id).unwrap();
        assert_eq!(record.exit_code, Some(1));
        assert!(record.error.is_none());
    }

    #[tokio::test(start_paused = true)]
    async fn npm_exit_1_with_garbage_is_parse_failed() {
        let h = harness();
        h.fake
            .on("npm", &["ls", "-g", "--depth=0", "--json"])
            .fixture("npm_ls_g_2026-07-22.json");
        h.fake
            .on("npm", &["outdated", "-g", "--json"])
            .exit(1, "npm ERR! something exploded", "");
        // The wired text recovery also returns garbage → ParseFailed.
        h.fake
            .on("npm", &["outdated", "-g"])
            .exit(1, "npm ERR! exploded", "");

        let id = h
            .queue
            .submit(refresh_sub(ManagerId::Npm, "/fake/npm", ManagedBy::Mise))
            .await
            .unwrap();
        wait_for(|| status_of(&h, &id) == OpStatus::Failed).await;

        let record = h.queue.record(&id).unwrap();
        let err = record.error.expect("error present");
        assert_eq!(err.code, ErrorCode::ParseFailed);
        assert_eq!(err.log_path.as_deref(), Some(record.log_path.as_str()));
        assert!(
            h.registry.get(ManagerId::Npm).is_none(),
            "no snapshot upserted — previous snapshot retained"
        );
    }

    #[tokio::test(start_paused = true)]
    async fn brew_lock_stderr_maps_to_brew_lock_busy() {
        let h = harness();
        h.fake
            .on_streaming("brew", &["upgrade", "dolt"])
            .emits(&[(
                StreamKind::Err,
                "Error: Another active Homebrew update process is already in progress.",
            )])
            .exit(1);

        let id = h
            .queue
            .submit(upgrade_sub(
                ManagerId::Brew,
                &[ManagerId::Brew],
                "/fake/brew",
                &["upgrade", "dolt"],
            ))
            .await
            .unwrap();
        wait_for(|| status_of(&h, &id) == OpStatus::Failed).await;

        let err = h.queue.record(&id).unwrap().error.expect("error");
        assert_eq!(err.code, ErrorCode::BrewLockBusy);
        assert_eq!(
            err.message,
            "Homebrew is busy in another terminal. Retry when it finishes."
        );
    }

    #[tokio::test(start_paused = true)]
    async fn recovery_plan_runs_text_fallback_on_json_parse_failure() {
        let h = harness();
        h.fake
            .on("mise", &["ls", "--json"])
            .fixture("mise_ls_2026-07-22.json");
        // Garbage where JSON was expected (exit 0 — the parser is the judge).
        h.fake
            .on("mise", &["outdated", "--json"])
            .ok("Tool  Requested  Current  Latest\nnot json at all");
        h.fake
            .on("mise", &["outdated"])
            .fixture("mise_outdated_text_2026-07-21.txt");

        let id = h
            .queue
            .submit(refresh_sub(ManagerId::Mise, "/fake/mise", ManagedBy::Brew))
            .await
            .unwrap();
        wait_for(|| status_of(&h, &id) == OpStatus::Succeeded).await;

        assert!(
            h.fake.calls().iter().any(|c| c.args == vec!["outdated"]),
            "text fallback ran"
        );
        let snap = h.registry.get(ManagerId::Mise).expect("recovered snapshot");
        // Recovery merges the already-captured 11-tool inventory with the
        // 6-row text overlay (7 rows, rust dropped) — up-to-date tools must
        // not vanish from the table when recovery fires.
        assert_eq!(snap.packages.len(), 11, "full inventory survives recovery");
        assert_eq!(snap.packages.iter().filter(|p| p.outdated).count(), 6);
    }

    #[tokio::test(start_paused = true)]
    async fn failure_aborts_remaining_specs() {
        let h = harness();
        h.fake
            .on_streaming("brew", &["upgrade", "dolt"])
            .emits(&[(StreamKind::Err, "Error: no such formula")])
            .exit(1);
        h.fake
            .on_streaming("brew", &["upgrade", "--cask", "ghostty"])
            .exit(0);

        let mut sub = upgrade_sub(
            ManagerId::Brew,
            &[ManagerId::Brew],
            "/fake/brew",
            &["upgrade", "dolt"],
        );
        // Second serial spec that must never run.
        let second = streaming_sub(
            ManagerId::Brew,
            ManagerId::Brew,
            &[ManagerId::Brew],
            "/fake/brew",
            &["upgrade", "--cask", "ghostty"],
            abs1h(),
            OpKind::Upgrade {
                package_ids: vec![],
            },
        );
        sub.commands.extend(second.commands);

        let id = h.queue.submit(sub).await.unwrap();
        wait_for(|| status_of(&h, &id) == OpStatus::Failed).await;

        assert_eq!(h.fake.calls().len(), 1, "second spec aborted");
        let err = h.queue.record(&id).unwrap().error.expect("error");
        assert_eq!(err.code, ErrorCode::NonZeroExit);
        assert!(err.detail.as_deref().unwrap().contains("no such formula"));
    }

    #[tokio::test(start_paused = true)]
    async fn successful_upgrade_auto_enqueues_refresh() {
        let factory_env = test_env();
        let factory_settings = Settings::default();
        let factory: RefreshFactory = Arc::new(move |id: ManagerId| {
            make_refresh_submission(
                id,
                &present(id, "/fake/mise", ManagedBy::Brew),
                &factory_settings,
                &factory_env,
            )
        });
        let h = harness_with_factory(Some(factory));
        h.fake.on_streaming("mise", &["upgrade", "deno"]).exit(0);
        h.fake
            .on("mise", &["ls", "--json"])
            .fixture("mise_ls_2026-07-22.json");
        h.fake
            .on("mise", &["outdated", "--json"])
            .fixture("mise_outdated.json");

        let id = h
            .queue
            .submit(streaming_sub(
                ManagerId::Mise,
                ManagerId::Mise,
                &[ManagerId::Mise],
                "/fake/mise",
                &["upgrade", "deno"],
                abs1h(),
                OpKind::Upgrade {
                    package_ids: vec!["tool:deno".into()],
                },
            ))
            .await
            .unwrap();
        wait_for(|| status_of(&h, &id) == OpStatus::Succeeded).await;

        // The auto-enqueued refresh runs to completion and upserts a snapshot.
        wait_for(|| {
            h.queue
                .records()
                .iter()
                .any(|r| r.kind == ipc::OpKind::Refresh && r.status == OpStatus::Succeeded)
        })
        .await;
        let refresh = h
            .queue
            .records()
            .into_iter()
            .find(|r| r.kind == ipc::OpKind::Refresh)
            .unwrap();
        assert_eq!(refresh.subject, ManagerId::Mise);
        assert!(h.registry.get(ManagerId::Mise).is_some());
    }

    /// SPEC §7.3 name. Scope note: through the FakeRunner this verifies the
    /// cancel → Cancelled status flow and the finalized transcript/journal —
    /// the footer string is a pure status mapping (`footer_status_str`). The
    /// actual SIGTERM→grace→SIGKILL escalation is guarded by the DEFAULT-RUN
    /// real-process test `real_cancel_escalates_to_sigkill_when_term_ignored`
    /// in `process/runner.rs`.
    #[tokio::test(start_paused = true)]
    async fn cancel_sigterm_then_sigkill_marks_cancelled_finalizes_transcript() {
        let h = harness();
        let gate = Arc::new(Notify::new());
        h.fake
            .on_streaming("brew", &["upgrade", "dolt"])
            .emits(&[(StreamKind::Out, "==> Upgrading dolt")])
            .gate(gate);

        let id = h
            .queue
            .submit(upgrade_sub(
                ManagerId::Brew,
                &[ManagerId::Brew],
                "/fake/brew",
                &["upgrade", "dolt"],
            ))
            .await
            .unwrap();
        wait_for(|| status_of(&h, &id) == OpStatus::Running).await;

        h.queue.cancel(&id);
        wait_for(|| status_of(&h, &id) == OpStatus::Cancelled).await;

        let record = h.queue.record(&id).unwrap();
        assert_eq!(record.error.as_ref().unwrap().code, ErrorCode::Cancelled);
        // The transcript is finalized with the signal-path footer.
        let body = std::fs::read_to_string(&record.log_path).expect("transcript exists");
        assert!(body.contains("=== Pack-Manager operation ==="));
        assert!(body.contains("[out] ==> Upgrading dolt"));
        assert!(body.contains("=== result ==="));
        assert!(body.contains("status: cancelled (SIGTERM→exit)"));
        // Journal has both lines.
        let records = crate::journal::load_records(&h.journal_path);
        assert_eq!(records.len(), 1);
        assert_eq!(records[0].status, OpStatus::Cancelled);
    }

    /// Regression: cancelling a RUNNING refresh was a silent no-op — the
    /// buffered path ran `runner.run(spec)` with a throwaway token, so the
    /// op's real token (flipped by handle_cancel) never reached the child.
    /// The op must finalize Cancelled, not run to natural completion.
    #[tokio::test(start_paused = true)]
    async fn cancel_running_refresh_marks_cancelled() {
        let h = harness();
        let gate = Arc::new(Notify::new());
        // First refresh command parks on the gate — "in flight".
        h.fake
            .on("mise", &["ls", "--json"])
            .fixture("mise_ls_2026-07-22.json")
            .gate(gate);
        h.fake
            .on("mise", &["outdated", "--json"])
            .fixture("mise_outdated.json");

        let id = h
            .queue
            .submit(refresh_sub(ManagerId::Mise, "/fake/mise", ManagedBy::Brew))
            .await
            .unwrap();
        wait_for(|| status_of(&h, &id) == OpStatus::Running).await;

        h.queue.cancel(&id);
        wait_for(|| status_of(&h, &id) == OpStatus::Cancelled).await;

        let record = h.queue.record(&id).unwrap();
        assert_eq!(record.error.as_ref().unwrap().code, ErrorCode::Cancelled);
        assert!(
            h.registry.get(ManagerId::Mise).is_none(),
            "cancelled refresh publishes no snapshot"
        );
        // The journal finalizes Cancelled (start + finish lines).
        let records = crate::journal::load_records(&h.journal_path);
        assert_eq!(records.len(), 1);
        assert_eq!(records[0].status, OpStatus::Cancelled);
    }

    /// Regression for the quit guard: cancel_all only flips tokens; the exit
    /// path must be able to WAIT until the runner tasks finish the kill work.
    /// With a running (buffered) refresh, cancel_all + wait_until_idle must
    /// converge to no running ops — previously the refresh child was
    /// untouchable and kept running past app exit (SPEC F7).
    #[tokio::test(start_paused = true)]
    async fn cancel_all_then_wait_until_idle_reaps_running_refresh() {
        let h = harness();
        let gate = Arc::new(Notify::new());
        h.fake
            .on("npm", &["ls", "-g", "--depth=0", "--json"])
            .fixture("npm_ls_g_2026-07-22.json")
            .gate(gate);
        h.fake
            .on("npm", &["outdated", "-g", "--json"])
            .fixture_with_exit("npm_outdated_g_synthetic.json", 1);

        let id = h
            .queue
            .submit(refresh_sub(ManagerId::Npm, "/fake/npm", ManagedBy::Mise))
            .await
            .unwrap();
        wait_for(|| status_of(&h, &id) == OpStatus::Running).await;
        assert_eq!(h.queue.running().len(), 1);

        h.queue.cancel_all();
        assert!(
            h.queue.wait_until_idle(Duration::from_secs(7)).await,
            "running ops must reach a terminal state within the shutdown grace"
        );
        assert_eq!(status_of(&h, &id), OpStatus::Cancelled);
        assert!(h.queue.running().is_empty());
    }

    /// Regression: a non-zero `brew update` (offline, tap trouble) must NOT
    /// abort the refresh — the remaining commands read local data and the
    /// full snapshot is still obtainable.
    #[tokio::test(start_paused = true)]
    async fn brew_update_failure_degrades_to_local_snapshot() {
        let h = harness();
        h.fake
            .on("brew", &["update"])
            .exit(1, "", "Error: Fetching /opt/homebrew failed!\n");
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

        let id = h
            .queue
            .submit(refresh_sub(
                ManagerId::Brew,
                "/fake/brew",
                ManagedBy::Standalone,
            ))
            .await
            .unwrap();
        wait_for(|| status_of(&h, &id) == OpStatus::Succeeded).await;

        assert_eq!(h.fake.calls().len(), 5, "all refresh commands still ran");
        let snap = h.registry.get(ManagerId::Brew).expect("snapshot published");
        assert_eq!(snap.packages.len(), 258, "full local inventory retained");
        assert!(h.queue.record(&id).unwrap().error.is_none());
    }

    /// SPEC §5.3 wiring: every successfully parsed refresh snapshot flows
    /// through the route-recheck hook BEFORE it is published.
    #[tokio::test(start_paused = true)]
    async fn refresh_invokes_route_recheck_with_parsed_snapshot() {
        let seen: Arc<Mutex<Vec<ManagerSnapshot>>> = Arc::new(Mutex::new(Vec::new()));
        let seen_clone = seen.clone();
        let recheck: RouteRecheck = Arc::new(move |snapshot: &ManagerSnapshot| {
            seen_clone.lock().unwrap().push(snapshot.clone());
        });
        let h = harness_with(None, Some(recheck));
        h.fake
            .on("npm", &["ls", "-g", "--depth=0", "--json"])
            .fixture("npm_ls_g_2026-07-22.json");
        h.fake
            .on("npm", &["outdated", "-g", "--json"])
            .fixture_with_exit("npm_outdated_g_synthetic.json", 1);

        let id = h
            .queue
            .submit(refresh_sub(ManagerId::Npm, "/fake/npm", ManagedBy::Mise))
            .await
            .unwrap();
        wait_for(|| status_of(&h, &id) == OpStatus::Succeeded).await;

        let seen = seen.lock().unwrap();
        assert_eq!(seen.len(), 1);
        assert_eq!(seen[0].manager_id, ManagerId::Npm);
        // npm reports itself outdated in this fixture — exactly the input the
        // in-band override (D5) needs at recheck time.
        assert!(seen[0].self_status.as_ref().unwrap().update_available);
    }

    #[tokio::test(start_paused = true)]
    async fn stall_fires_at_threshold_and_rearms_on_output() {
        let h = harness();
        let gate = Arc::new(Notify::new());
        h.fake
            .on_streaming("brew", &["upgrade", "dolt"])
            .emits_after(Duration::from_secs(130), StreamKind::Out, "tick")
            .gate(gate.clone());

        let id = h
            .queue
            .submit(streaming_sub(
                ManagerId::Brew,
                ManagerId::Brew,
                &[ManagerId::Brew],
                "/fake/brew",
                &["upgrade", "dolt"],
                CmdTimeout::Stall {
                    silence: Duration::from_secs(120),
                    hard_cap: Duration::from_secs(1800),
                },
                OpKind::Upgrade {
                    package_ids: vec!["formula:dolt".into()],
                },
            ))
            .await
            .unwrap();

        // t=125s: one firing at the 120s threshold.
        tokio::time::sleep(Duration::from_secs(125)).await;
        assert_eq!(stalled_events(&h.sink), vec![120]);

        // Output at t=130 re-arms: the next firing lands at t≈250, not t=240.
        tokio::time::sleep(Duration::from_secs(115)).await; // t=240
        assert_eq!(stalled_events(&h.sink).len(), 1, "re-armed by output");
        tokio::time::sleep(Duration::from_secs(15)).await; // t=255
        assert_eq!(stalled_events(&h.sink).len(), 2, "fires after re-arm");

        gate.notify_one();
        wait_for(|| status_of(&h, &id) == OpStatus::Succeeded).await;
    }

    /// SPEC §7.3 name. Scope note: verifies the hard cap → TimedOut flow via
    /// the FakeRunner; the footer's "(SIGKILL after 5s grace)" text is the
    /// status mapping, and the real kill escalation is guarded by the
    /// default-run runner test (see the cancel test's note above).
    #[tokio::test(start_paused = true)]
    async fn hard_cap_times_out() {
        let h = harness();
        h.fake
            .on_streaming("brew", &["upgrade", "dolt"])
            .emits_after(Duration::from_secs(3600), StreamKind::Out, "never");

        let id = h
            .queue
            .submit(streaming_sub(
                ManagerId::Brew,
                ManagerId::Brew,
                &[ManagerId::Brew],
                "/fake/brew",
                &["upgrade", "dolt"],
                CmdTimeout::Stall {
                    silence: Duration::from_secs(1200),
                    hard_cap: Duration::from_secs(1800),
                },
                OpKind::Upgrade {
                    package_ids: vec!["formula:dolt".into()],
                },
            ))
            .await
            .unwrap();
        // Advance past the 1800s hard cap, then let the terminal state land.
        tokio::time::sleep(Duration::from_secs(1801)).await;
        wait_for(|| status_of(&h, &id) == OpStatus::TimedOut).await;

        let record = h.queue.record(&id).unwrap();
        assert_eq!(record.error.as_ref().unwrap().code, ErrorCode::Timeout);
        let body = std::fs::read_to_string(&record.log_path).unwrap();
        assert!(body.contains("status: timed_out (SIGKILL after 5s grace)"));
    }

    #[tokio::test(start_paused = true)]
    async fn queued_op_cancel_removes_it_without_journal_lines() {
        let h = harness();
        let gate = Arc::new(Notify::new());
        h.fake
            .on_streaming("brew", &["upgrade", "dolt"])
            .gate(gate.clone());
        h.fake.on_streaming("brew", &["upgrade", "abseil"]).exit(0);

        let id1 = h
            .queue
            .submit(upgrade_sub(
                ManagerId::Brew,
                &[ManagerId::Brew],
                "/fake/brew",
                &["upgrade", "dolt"],
            ))
            .await
            .unwrap();
        let id2 = h
            .queue
            .submit(upgrade_sub(
                ManagerId::Brew,
                &[ManagerId::Brew],
                "/fake/brew",
                &["upgrade", "abseil"],
            ))
            .await
            .unwrap();
        wait_for(|| status_of(&h, &id1) == OpStatus::Running).await;

        h.queue.cancel(&id2);
        wait_for(|| status_of(&h, &id2) == OpStatus::Cancelled).await;
        gate.notify_one();
        wait_for(|| status_of(&h, &id1) == OpStatus::Succeeded).await;

        assert_eq!(h.fake.calls().len(), 1, "cancelled queued op never ran");
        let journal = crate::journal::load_records(&h.journal_path);
        assert_eq!(journal.len(), 1, "only the op that started is journaled");
        assert_eq!(journal[0].op_id, id1);
    }

    #[tokio::test(start_paused = true)]
    async fn refresh_phase_labels_ride_op_status_events() {
        let h = harness();
        h.fake.on("brew", &["update"]).ok("Already up-to-date.\n");
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

        let id = h
            .queue
            .submit(refresh_sub(
                ManagerId::Brew,
                "/fake/brew",
                ManagedBy::Standalone,
            ))
            .await
            .unwrap();
        wait_for(|| status_of(&h, &id) == OpStatus::Succeeded).await;

        let labels: Vec<String> = h
            .sink
            .events()
            .into_iter()
            .filter_map(|e| match e {
                AppEvent::OpStatus(s) => s.phase_label,
                _ => None,
            })
            .collect();
        assert_eq!(
            labels,
            vec![
                "Updating Homebrew metadata…",
                "Listing installed…",
                "Listing installed…",
                "Checking outdated…",
                "Checking outdated…",
            ]
        );
        assert!(h.registry.get(ManagerId::Brew).is_some());
    }

    // ---------------- lock sets (SPEC §7.2) ----------------

    #[test]
    fn routed_self_update_holds_both_locks() {
        let locks = lock_set(ManagerId::Brew, ManagerId::Mise, ManagedBy::Standalone);
        assert_eq!(
            locks.into_iter().collect::<Vec<_>>(),
            vec![ManagerId::Brew, ManagerId::Mise]
        );
    }

    #[test]
    fn npm_upgrade_with_mise_managed_node_locks_npm_and_mise() {
        let locks = lock_set(ManagerId::Npm, ManagerId::Npm, ManagedBy::Mise);
        assert_eq!(
            locks.into_iter().collect::<Vec<_>>(),
            vec![ManagerId::Mise, ManagerId::Npm]
        );
        // uv likewise; standalone tools guard nothing extra.
        let uv = lock_set(ManagerId::Uv, ManagerId::Uv, ManagedBy::Mise);
        assert!(uv.contains(&ManagerId::Mise));
        let solo = lock_set(ManagerId::Rustup, ManagerId::Rustup, ManagedBy::Standalone);
        assert_eq!(solo.len(), 1);
    }

    // ---------------- plan builder ----------------

    fn pkg(
        kind: PackageKind,
        name: &str,
        installed: &str,
        latest: &str,
        outdated: bool,
    ) -> Package {
        Package {
            id: format!("{}:{name}", kind_str(kind)),
            name: name.into(),
            kind,
            installed: Some(installed.into()),
            latest: Some(latest.into()),
            outdated,
            pinned: false,
            meta: None,
        }
    }

    fn kind_str(kind: PackageKind) -> &'static str {
        match kind {
            PackageKind::Formula => "formula",
            PackageKind::Cask => "cask",
            PackageKind::CaskGreedy => "caskGreedy",
            PackageKind::Tool => "tool",
            PackageKind::GlobalPackage => "globalPackage",
            PackageKind::Toolchain => "toolchain",
            PackageKind::App => "app",
        }
    }

    fn snap(id: ManagerId, packages: Vec<Package>) -> ManagerSnapshot {
        ManagerSnapshot {
            manager_id: id,
            refreshed_at: "2026-07-22T14:00:00Z".into(),
            packages,
            self_status: None,
            health: vec![],
        }
    }

    fn info(
        id: ManagerId,
        name: &str,
        managed_by: ManagedBy,
        route: SelfUpdateRoute,
    ) -> ManagerInfo {
        ManagerInfo {
            id,
            display_name: name.into(),
            status: ManagerStatus::Present,
            binary_path: None,
            canonical_path: None,
            version: None,
            managed_by,
            evidence: None,
            self_update: route,
            install_hint: None,
        }
    }

    /// This machine's topology (SPEC F1/F6 routes).
    fn machine_report() -> DetectionReport {
        DetectionReport {
            managers: vec![
                info(
                    ManagerId::Brew,
                    "Homebrew",
                    ManagedBy::Standalone,
                    SelfUpdateRoute::ViaRefresh {
                        note: "brew update runs as part of every refresh".into(),
                    },
                ),
                info(
                    ManagerId::Mise,
                    "mise",
                    ManagedBy::Brew,
                    SelfUpdateRoute::Routed {
                        executor: ManagerId::Brew,
                        command_preview: "brew upgrade mise".into(),
                        command_args: vec!["upgrade".into(), "mise".into()],
                        why: "mise is managed by Homebrew".into(),
                    },
                ),
                info(
                    ManagerId::Npm,
                    "npm",
                    ManagedBy::Mise,
                    SelfUpdateRoute::InBand {
                        command_preview: "npm install -g npm@latest".into(),
                        command_args: vec!["install".into(), "-g".into(), "npm@latest".into()],
                        note: None,
                    },
                ),
                info(
                    ManagerId::Uv,
                    "uv",
                    ManagedBy::Mise,
                    SelfUpdateRoute::Routed {
                        executor: ManagerId::Mise,
                        command_preview: "mise upgrade uv".into(),
                        command_args: vec!["upgrade".into(), "uv".into()],
                        why: "uv is managed by mise".into(),
                    },
                ),
                info(
                    ManagerId::Rustup,
                    "rustup",
                    ManagedBy::Standalone,
                    SelfUpdateRoute::InBand {
                        command_preview: "rustup self update".into(),
                        command_args: vec!["self".into(), "update".into()],
                        note: None,
                    },
                ),
            ],
            env: EnvInfo {
                path: String::new(),
                entries: vec![],
                source: PathSource::StaticFallback,
                home: "/Users/testuser".into(),
            },
        }
    }

    fn machine_snapshots() -> Vec<ManagerSnapshot> {
        let mut deno_pinned = pkg(PackageKind::Formula, "deno", "2.9.0", "2.9.3", true);
        deno_pinned.pinned = true;
        vec![
            snap(
                ManagerId::Brew,
                vec![
                    pkg(PackageKind::Formula, "dolt", "2.2.1", "2.2.2", true),
                    deno_pinned,
                    pkg(
                        PackageKind::Formula,
                        "abseil",
                        "20260107.1",
                        "20260107.1",
                        false,
                    ),
                    pkg(
                        PackageKind::CaskGreedy,
                        "openusage",
                        "0.6.20",
                        "0.7.6",
                        true,
                    ),
                ],
            ),
            snap(
                ManagerId::Mise,
                vec![
                    pkg(PackageKind::Tool, "deno", "2.9.0", "2.9.3", true),
                    pkg(PackageKind::Tool, "ruby", "4.0.5", "4.0.8", true),
                    pkg(PackageKind::Tool, "fnox", "1.7.5", "1.9.1", true),
                    pkg(PackageKind::Tool, "ruff", "0.15.20", "0.16.4", true),
                    pkg(PackageKind::Tool, "npm:prettier", "3.8.0", "4.0.1", true),
                    pkg(PackageKind::Tool, "uv", "0.11.26", "0.11.30", true),
                    pkg(PackageKind::Tool, "rust", "stable", "stable", false),
                ],
            ),
            snap(
                ManagerId::Npm,
                vec![
                    pkg(
                        PackageKind::GlobalPackage,
                        "typescript",
                        "6.0.3",
                        "7.0.2",
                        true,
                    ),
                    pkg(PackageKind::GlobalPackage, "dmux", "1.2.0", "1.3.0", true),
                ],
            ),
            snap(
                ManagerId::Rustup,
                vec![pkg(
                    PackageKind::Toolchain,
                    "stable-aarch64-apple-darwin",
                    "1.94.0",
                    "1.97.1",
                    true,
                )],
            ),
        ]
    }

    fn empty_sources<'a>(
        report: &'a DetectionReport,
        snapshots: &'a [ManagerSnapshot],
        busy: &'a BTreeSet<(ManagerId, String)>,
        stale: &'a BTreeSet<ManagerId>,
    ) -> PlanSources<'a> {
        PlanSources {
            report,
            snapshots,
            busy,
            stale,
        }
    }

    #[test]
    fn plan_builder_previews_exact_argv_and_excludes_pinned_and_greedy() {
        let report = machine_report();
        let snapshots = machine_snapshots();
        let busy = BTreeSet::new();
        let stale = BTreeSet::new();
        let req = PlanRequest {
            selection: None,
            include_self_updates: false,
            include_greedy_casks: false,
        };
        let plan = build_upgrade_plan(&req, &empty_sources(&report, &snapshots, &busy, &stale));

        let previews: Vec<&str> = plan
            .groups
            .iter()
            .flat_map(|g| g.commands.iter().map(|c| c.argv_preview.as_str()))
            .collect();
        assert_eq!(
            previews,
            vec![
                "brew upgrade dolt",
                "mise upgrade deno ruby fnox ruff npm:prettier uv",
                "npm install -g typescript@latest dmux@latest",
                "rustup update stable-aarch64-apple-darwin",
            ]
        );

        // Exclusions with reasons: pinned deno, greedy openusage.
        assert!(plan
            .excluded
            .iter()
            .any(|e| e.package_id == "formula:deno" && e.reason == ExcludeReason::Pinned));
        assert!(plan.excluded.iter().any(
            |e| e.package_id == "caskGreedy:openusage" && e.reason == ExcludeReason::GreedyCask
        ));
        // The up-to-date abseil row is neither included nor excluded.
        assert!(!plan
            .groups
            .iter()
            .any(|g| g.package_ids.contains(&"formula:abseil".to_string())));

        // npm group carries the mise shared-tree lock.
        let npm = plan
            .groups
            .iter()
            .find(|g| g.subject == ManagerId::Npm)
            .unwrap();
        assert_eq!(npm.locks, vec![ManagerId::Mise, ManagerId::Npm]);
    }

    #[test]
    fn plan_builder_greedy_opt_in_includes_the_cask() {
        let report = machine_report();
        let snapshots = machine_snapshots();
        let busy = BTreeSet::new();
        let stale = BTreeSet::new();
        let req = PlanRequest {
            selection: None,
            include_self_updates: false,
            include_greedy_casks: true,
        };
        let plan = build_upgrade_plan(&req, &empty_sources(&report, &snapshots, &busy, &stale));
        let brew = plan
            .groups
            .iter()
            .find(|g| g.subject == ManagerId::Brew)
            .unwrap();
        assert!(brew
            .commands
            .iter()
            .any(|c| c.argv_preview == "brew upgrade --cask --greedy openusage"));
        assert!(!plan
            .excluded
            .iter()
            .any(|e| e.reason == ExcludeReason::GreedyCask));
    }

    #[test]
    fn plan_builder_rust_dedup_drops_mise_rust_with_note() {
        let report = machine_report();
        let mut snapshots = machine_snapshots();
        // Make mise's rust row outdated so it becomes a candidate.
        for s in &mut snapshots {
            if s.manager_id == ManagerId::Mise {
                for p in &mut s.packages {
                    if p.name == "rust" {
                        p.outdated = true;
                    }
                }
            }
        }
        let busy = BTreeSet::new();
        let stale = BTreeSet::new();
        let req = PlanRequest {
            selection: None,
            include_self_updates: false,
            include_greedy_casks: false,
        };
        let plan = build_upgrade_plan(&req, &empty_sources(&report, &snapshots, &busy, &stale));

        // One plan never contains both mise `tool:rust` and rustup toolchains.
        let mise = plan
            .groups
            .iter()
            .find(|g| g.subject == ManagerId::Mise)
            .unwrap();
        assert!(!mise.package_ids.contains(&"tool:rust".to_string()));
        assert!(plan
            .excluded
            .iter()
            .any(|e| e.package_id == "tool:rust" && e.reason == ExcludeReason::RustDedup));
        assert_eq!(plan.notes, vec![RUST_DEDUP_NOTE.to_string()]);

        // Without rustup targets in the selection, mise's rust row survives.
        let req = PlanRequest {
            selection: Some(vec![PlanSelection {
                manager_id: ManagerId::Mise,
                package_id: "tool:rust".into(),
            }]),
            include_self_updates: false,
            include_greedy_casks: false,
        };
        let plan = build_upgrade_plan(&req, &empty_sources(&report, &snapshots, &busy, &stale));
        assert!(plan
            .groups
            .iter()
            .any(|g| g.package_ids.contains(&"tool:rust".to_string())));
        assert!(plan.notes.is_empty());
    }

    #[test]
    fn canonical_request_deduplicates_before_rust_dedup_and_argv_planning() {
        let report = machine_report();
        let mut snapshots = machine_snapshots();
        for snapshot in &mut snapshots {
            if snapshot.manager_id == ManagerId::Mise {
                snapshot
                    .packages
                    .iter_mut()
                    .find(|package| package.id == "tool:rust")
                    .unwrap()
                    .outdated = true;
            }
        }
        let repeated = PlanRequest {
            selection: Some(vec![
                PlanSelection {
                    manager_id: ManagerId::Mise,
                    package_id: "tool:rust".into(),
                },
                PlanSelection {
                    manager_id: ManagerId::Mise,
                    package_id: "tool:rust".into(),
                },
                PlanSelection {
                    manager_id: ManagerId::Rustup,
                    package_id: "toolchain:stable-aarch64-apple-darwin".into(),
                },
                PlanSelection {
                    manager_id: ManagerId::Rustup,
                    package_id: "toolchain:stable-aarch64-apple-darwin".into(),
                },
                PlanSelection {
                    manager_id: ManagerId::Mise,
                    package_id: "tool:deno".into(),
                },
                PlanSelection {
                    manager_id: ManagerId::Mise,
                    package_id: "tool:deno".into(),
                },
            ]),
            include_self_updates: false,
            include_greedy_casks: false,
        };

        let canonical = canonicalize_plan_request(repeated).unwrap();
        assert_eq!(canonical.selection.as_ref().unwrap().len(), 3);
        let busy = BTreeSet::new();
        let stale = BTreeSet::new();
        let plan = build_upgrade_plan(
            &canonical,
            &empty_sources(&report, &snapshots, &busy, &stale),
        );

        assert_eq!(
            plan.excluded
                .iter()
                .filter(|excluded| excluded.package_id == "tool:rust")
                .count(),
            1
        );
        assert_eq!(plan.notes, vec![RUST_DEDUP_NOTE.to_string()]);
        let mise = plan
            .groups
            .iter()
            .find(|group| group.subject == ManagerId::Mise)
            .unwrap();
        assert_eq!(mise.package_ids, vec!["tool:deno"]);
        assert_eq!(mise.commands[0].argv_preview, "mise upgrade deno");
        let rustup = plan
            .groups
            .iter()
            .find(|group| group.subject == ManagerId::Rustup)
            .unwrap();
        assert_eq!(
            rustup.package_ids,
            vec!["toolchain:stable-aarch64-apple-darwin"]
        );
        assert_eq!(
            rustup.commands[0].argv_preview,
            "rustup update stable-aarch64-apple-darwin"
        );
    }

    #[test]
    fn canonical_request_rejects_selection_and_package_id_limits() {
        let too_many = PlanRequest {
            selection: Some(
                (0..=MAX_PLAN_SELECTIONS)
                    .map(|index| PlanSelection {
                        manager_id: ManagerId::Brew,
                        package_id: format!("formula:pkg-{index}"),
                    })
                    .collect(),
            ),
            include_self_updates: false,
            include_greedy_casks: false,
        };
        assert!(canonicalize_plan_request(too_many).is_err());

        let too_long = PlanRequest {
            selection: Some(vec![PlanSelection {
                manager_id: ManagerId::Brew,
                package_id: "x".repeat(MAX_PLAN_PACKAGE_ID_BYTES + 1),
            }]),
            include_self_updates: false,
            include_greedy_casks: false,
        };
        assert!(canonicalize_plan_request(too_long).is_err());
    }

    #[test]
    fn plan_builder_selection_seeds_exact_ids() {
        let report = machine_report();
        let snapshots = machine_snapshots();
        let busy = BTreeSet::new();
        let stale = BTreeSet::new();
        let req = PlanRequest {
            selection: Some(vec![
                PlanSelection {
                    manager_id: ManagerId::Npm,
                    package_id: "globalPackage:typescript".into(),
                },
                PlanSelection {
                    manager_id: ManagerId::Brew,
                    package_id: "formula:dolt".into(),
                },
            ]),
            include_self_updates: false,
            include_greedy_casks: false,
        };
        let plan = build_upgrade_plan(&req, &empty_sources(&report, &snapshots, &busy, &stale));
        assert_eq!(plan.groups.len(), 2);
        let previews: Vec<&str> = plan
            .groups
            .iter()
            .flat_map(|g| g.commands.iter().map(|c| c.argv_preview.as_str()))
            .collect();
        assert_eq!(
            previews,
            vec!["brew upgrade dolt", "npm install -g typescript@latest"]
        );
    }

    #[test]
    fn plan_builder_includes_routed_self_update_group_with_dual_locks() {
        let report = machine_report();
        let mut snapshots = machine_snapshots();
        // mise knows an update (cross-join would have patched selfStatus).
        for s in &mut snapshots {
            if s.manager_id == ManagerId::Mise {
                s.self_status = Some(SelfStatus {
                    installed: Some("2026.1.0".into()),
                    latest: Some("2026.2.0".into()),
                    update_available: true,
                });
            }
        }
        let busy = BTreeSet::new();
        let stale = BTreeSet::new();
        let req = PlanRequest {
            selection: Some(vec![]),
            include_self_updates: true,
            include_greedy_casks: false,
        };
        let plan = build_upgrade_plan(&req, &empty_sources(&report, &snapshots, &busy, &stale));

        let self_group = plan
            .groups
            .iter()
            .find(|g| g.self_update)
            .expect("self-update group");
        assert_eq!(self_group.subject, ManagerId::Mise);
        assert_eq!(self_group.executor, ManagerId::Brew);
        assert_eq!(self_group.locks, vec![ManagerId::Brew, ManagerId::Mise]);
        assert_eq!(self_group.commands[0].argv_preview, "brew upgrade mise");
        assert_eq!(
            self_group.commands[0].label,
            "Self-update mise via Homebrew"
        );

        // Toggle off → no self-update groups.
        let req_off = PlanRequest {
            include_self_updates: false,
            ..req
        };
        let plan = build_upgrade_plan(&req_off, &empty_sources(&report, &snapshots, &busy, &stale));
        assert!(plan.groups.iter().all(|g| !g.self_update));
    }

    #[test]
    fn plan_builder_warns_on_stale_manager_and_excludes_already_running() {
        let report = machine_report();
        let snapshots = machine_snapshots();
        let mut busy = BTreeSet::new();
        busy.insert((ManagerId::Npm, "globalPackage:typescript".to_string()));
        let mut stale = BTreeSet::new();
        stale.insert(ManagerId::Mise);
        let req = PlanRequest {
            selection: None,
            include_self_updates: false,
            include_greedy_casks: false,
        };
        let plan = build_upgrade_plan(&req, &empty_sources(&report, &snapshots, &busy, &stale));

        assert_eq!(
            plan.warnings,
            vec!["mise: list may be stale — last check errored".to_string()]
        );
        assert!(plan
            .excluded
            .iter()
            .any(|e| e.package_id == "globalPackage:typescript"
                && e.reason == ExcludeReason::AlreadyRunning));
        let npm = plan
            .groups
            .iter()
            .find(|g| g.subject == ManagerId::Npm)
            .unwrap();
        assert_eq!(npm.package_ids, vec!["globalPackage:dmux".to_string()]);
    }

    #[test]
    fn busy_and_stale_derivations_read_session_records() {
        let mk = |kind: ipc::OpKind, subject: ManagerId, status: OpStatus, ids: &[&str]| {
            OperationRecord {
                op_id: format!("{subject}-{kind:?}-{status:?}"),
                kind,
                executor: subject,
                subject,
                status,
                command_line: String::new(),
                package_ids: ids.iter().map(|s| s.to_string()).collect(),
                queued_at: "2026-07-22T14:00:00Z".into(),
                started_at: None,
                finished_at: None,
                exit_code: None,
                error: None,
                log_path: String::new(),
            }
        };
        let records = vec![
            mk(
                ipc::OpKind::Upgrade,
                ManagerId::Npm,
                OpStatus::Running,
                &["globalPackage:typescript"],
            ),
            mk(
                ipc::OpKind::Upgrade,
                ManagerId::Brew,
                OpStatus::Succeeded,
                &["formula:dolt"],
            ),
            mk(ipc::OpKind::Refresh, ManagerId::Mise, OpStatus::Failed, &[]),
            mk(
                ipc::OpKind::Refresh,
                ManagerId::Brew,
                OpStatus::Succeeded,
                &[],
            ),
        ];
        let busy = busy_package_ids(&records);
        assert!(busy.contains(&(ManagerId::Npm, "globalPackage:typescript".to_string())));
        assert!(!busy.contains(&(ManagerId::Brew, "formula:dolt".to_string())));
        let stale = stale_managers(&records);
        assert!(stale.contains(&ManagerId::Mise));
        assert!(!stale.contains(&ManagerId::Brew));
    }

    // ---------------- submission builders ----------------

    #[test]
    fn bind_commands_rebinds_stall_timeouts_to_settings() {
        let settings = Settings {
            stall_after_secs: 60,
            upgrade_hard_cap_mins: 10,
            ..Settings::default()
        };
        let planned = vec![
            PlannedCommand {
                label: "upgrade",
                argv: vec!["upgrade".into(), "dolt".into()],
                timeout: stall_default(),
                extra_env: vec![("HOMEBREW_NO_AUTO_UPDATE".into(), "1".into())],
                phase_label: None,
            },
            PlannedCommand {
                label: "list",
                argv: vec!["list".into()],
                timeout: CmdTimeout::Absolute(Duration::from_secs(60)),
                extra_env: vec![],
                phase_label: None,
            },
        ];
        let base = vec![("PATH".to_string(), "/fake".to_string())];
        let bound = bind_commands(
            planned,
            Path::new("/opt/homebrew/bin/brew"),
            &base,
            &settings,
            CmdPurpose::Upgrade,
        );
        assert_eq!(
            bound[0].spec.timeout,
            CmdTimeout::Stall {
                silence: Duration::from_secs(60),
                hard_cap: Duration::from_secs(600),
            }
        );
        assert_eq!(
            bound[1].spec.timeout,
            CmdTimeout::Absolute(Duration::from_secs(60))
        );
        assert_eq!(
            bound[0].spec.env,
            vec![
                ("PATH".to_string(), "/fake".to_string()),
                ("HOMEBREW_NO_AUTO_UPDATE".to_string(), "1".to_string()),
            ]
        );
        assert_eq!(
            bound[0].spec.program,
            PathBuf::from("/opt/homebrew/bin/brew")
        );
    }

    #[test]
    fn self_update_submission_binds_routed_and_in_band_routes() {
        let mut statuses = BTreeMap::new();
        statuses.insert(
            ManagerId::Brew,
            present(
                ManagerId::Brew,
                "/opt/homebrew/bin/brew",
                ManagedBy::Standalone,
            ),
        );
        statuses.insert(
            ManagerId::Npm,
            present(
                ManagerId::Npm,
                "/Users/testuser/.local/share/mise/shims/npm",
                ManagedBy::Mise,
            ),
        );
        let settings = Settings::default();
        let env = test_env();

        // Routed: mise via brew → brew's binary, dual locks, brew env guard.
        let routed = ipc::SelfUpdateRoute::Routed {
            executor: ManagerId::Brew,
            command_preview: "brew upgrade mise".into(),
            command_args: vec!["upgrade".into(), "mise".into()],
            why: "mise is managed by Homebrew".into(),
        };
        let sub = make_self_update_submission(ManagerId::Mise, &routed, &statuses, &settings, &env)
            .unwrap();
        assert_eq!(sub.executor, ManagerId::Brew);
        assert_eq!(sub.subject, ManagerId::Mise);
        assert_eq!(
            sub.locks.iter().copied().collect::<Vec<_>>(),
            vec![ManagerId::Brew, ManagerId::Mise]
        );
        assert_eq!(
            sub.commands[0].spec.program,
            PathBuf::from("/opt/homebrew/bin/brew")
        );
        assert_eq!(sub.commands[0].spec.args, vec!["upgrade", "mise"]);
        assert!(sub.commands[0]
            .spec
            .env
            .iter()
            .any(|(k, v)| k == "HOMEBREW_NO_AUTO_UPDATE" && v == "1"));

        // In-band: npm updates itself; mise-managed npm guards the Mise lock.
        let in_band = ipc::SelfUpdateRoute::InBand {
            command_preview: "npm install -g npm@latest".into(),
            command_args: vec!["install".into(), "-g".into(), "npm@latest".into()],
            note: None,
        };
        let sub = make_self_update_submission(ManagerId::Npm, &in_band, &statuses, &settings, &env)
            .unwrap();
        assert_eq!(sub.executor, ManagerId::Npm);
        assert_eq!(
            sub.commands[0].spec.args,
            vec!["install", "-g", "npm@latest"]
        );
        assert!(sub.locks.contains(&ManagerId::Mise));

        // Unavailable errors with the route's reason.
        let unavailable = ipc::SelfUpdateRoute::Unavailable {
            reason: "mas has no self-update mechanism".into(),
        };
        let err =
            make_self_update_submission(ManagerId::Mas, &unavailable, &statuses, &settings, &env)
                .err()
                .expect("unavailable route must error");
        assert!(matches!(err, PmError::SelfUpdateUnavailable { .. }));
    }

    #[test]
    fn self_update_submission_preserves_structured_argument_boundaries() {
        let mut statuses = BTreeMap::new();
        statuses.insert(
            ManagerId::Npm,
            present(
                ManagerId::Npm,
                "/Users/testuser/.local/share/mise/shims/npm",
                ManagedBy::Mise,
            ),
        );
        let route = SelfUpdateRoute::in_band(
            "npm",
            vec![
                "install".into(),
                "-g".into(),
                "package name;$(touch /tmp/never)".into(),
            ],
            None,
        );
        let sub = make_self_update_submission(
            ManagerId::Npm,
            &route,
            &statuses,
            &Settings::default(),
            &test_env(),
        )
        .unwrap();
        assert_eq!(
            sub.commands[0].spec.args,
            vec!["install", "-g", "package name;$(touch /tmp/never)"]
        );
    }

    #[test]
    fn self_update_submission_fails_closed_without_matching_backend_argv() {
        let mut statuses = BTreeMap::new();
        statuses.insert(
            ManagerId::Npm,
            present(
                ManagerId::Npm,
                "/Users/testuser/.local/share/mise/shims/npm",
                ManagedBy::Mise,
            ),
        );
        let route = SelfUpdateRoute::in_band(
            "npm",
            vec!["install".into(), "-g".into(), "npm@latest".into()],
            None,
        );
        let wire = serde_json::to_string(&route).unwrap();
        let deserialized: SelfUpdateRoute = serde_json::from_str(&wire).unwrap();
        let err = make_self_update_submission(
            ManagerId::Npm,
            &deserialized,
            &statuses,
            &Settings::default(),
            &test_env(),
        )
        .err()
        .expect("IPC-round-tripped route has no trusted argv");
        assert!(matches!(err, PmError::Internal { .. }));

        let mismatched = SelfUpdateRoute::InBand {
            command_preview: "npm install -g attacker@latest".into(),
            command_args: vec!["install".into(), "-g".into(), "npm@latest".into()],
            note: None,
        };
        let err = make_self_update_submission(
            ManagerId::Npm,
            &mismatched,
            &statuses,
            &Settings::default(),
            &test_env(),
        )
        .err()
        .expect("preview/argv mismatch must fail");
        assert!(matches!(err, PmError::Internal { .. }));
    }

    #[test]
    fn health_fix_submission_binds_the_fix_command() {
        let issue = HealthIssue {
            id: "uv:aider-chat".into(),
            manager_id: ManagerId::Uv,
            severity: HealthSeverity::Warning,
            title: "Tool `aider-chat` environment is broken.".into(),
            detail: "warning: …".into(),
            fix_command: Some("uv tool install aider-chat --reinstall".into()),
            fix_args: Some(vec![
                "tool".into(),
                "install".into(),
                "aider-chat".into(),
                "--reinstall".into(),
            ]),
            fixable: true,
        };
        let det = present(
            ManagerId::Uv,
            "/Users/testuser/.local/share/mise/shims/uv",
            ManagedBy::Mise,
        );
        let sub = make_health_fix_submission(
            ManagerId::Uv,
            &issue,
            &det,
            &Settings::default(),
            &test_env(),
        )
        .unwrap();
        assert_eq!(
            sub.commands[0].spec.args,
            vec!["tool", "install", "aider-chat", "--reinstall"]
        );
        assert!(sub.locks.contains(&ManagerId::Mise), "uv is mise-managed");
        assert!(
            matches!(sub.kind, OpKind::HealthFix { ref issue_id } if issue_id == "uv:aider-chat")
        );
    }

    #[test]
    fn health_fix_submission_fails_closed_for_untrusted_or_mismatched_argv() {
        let det = present(
            ManagerId::Uv,
            "/Users/testuser/.local/share/mise/shims/uv",
            ManagedBy::Mise,
        );
        let mut issue = HealthIssue {
            id: "uv:aider-chat".into(),
            manager_id: ManagerId::Uv,
            severity: HealthSeverity::Warning,
            title: "broken".into(),
            detail: "warning".into(),
            fix_command: Some("uv tool install aider-chat --reinstall".into()),
            fix_args: None,
            fixable: true,
        };
        let err = make_health_fix_submission(
            ManagerId::Uv,
            &issue,
            &det,
            &Settings::default(),
            &test_env(),
        )
        .err()
        .expect("missing trusted argv must fail");
        assert!(matches!(err, PmError::Internal { .. }));

        issue.fix_args = Some(vec![
            "tool".into(),
            "install".into(),
            "other-tool".into(),
            "--reinstall".into(),
        ]);
        let err = make_health_fix_submission(
            ManagerId::Uv,
            &issue,
            &det,
            &Settings::default(),
            &test_env(),
        )
        .err()
        .expect("preview/argv mismatch must fail");
        assert!(matches!(err, PmError::Internal { .. }));
    }

    #[test]
    fn refresh_submission_binds_plan_env_and_meta() {
        let sub = refresh_sub(
            ManagerId::Brew,
            "/opt/homebrew/bin/brew",
            ManagedBy::Standalone,
        );
        assert!(matches!(sub.kind, OpKind::Refresh));
        assert_eq!(sub.commands.len(), 5);
        assert_eq!(sub.commands[0].spec.args, vec!["update"]);
        // brew update must NOT carry HOMEBREW_NO_AUTO_UPDATE; the others must.
        assert!(!sub.commands[0]
            .spec
            .env
            .iter()
            .any(|(k, _)| k == "HOMEBREW_NO_AUTO_UPDATE"));
        assert!(sub.commands[1]
            .spec
            .env
            .iter()
            .any(|(k, v)| k == "HOMEBREW_NO_AUTO_UPDATE" && v == "1"));
    }
}
