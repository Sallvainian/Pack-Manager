//! ALL serde IPC payload types (SPEC §5.9). Every serialized field here is part
//! of the wire contract and mirrored in `src/lib/ipc/types.ts`; backend-only
//! `#[serde(skip)]` fields are intentionally absent there. Drift is guarded by
//! `ipc_contract_matches_committed_fixtures` (Rust) and
//! `ipc_types_accept_contract_fixtures` (Vitest) over `dev/fixtures/ipc/*.json`.
//!
//! Conventions: struct fields serialize as lower camel case. Enum values use
//! their explicitly declared wire casing (usually lowercase or lower camel
//! case; `ErrorCode` is snake case). Fields optional in TS (`?`) use
//! `skip_serializing_if = "Option::is_none"`; fields typed `T | null` serialize
//! `null` explicitly.

use serde::{Deserialize, Serialize};

pub use crate::error::{ErrorCode, IpcError};
pub use crate::settings::{LogLevel, Settings};

// ---------------------------------------------------------------------------
// Core string enums
// ---------------------------------------------------------------------------

/// `'brew'|'mise'|'npm'|'uv'|'rustup'|'mas'`
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ManagerId {
    Brew,
    Mise,
    Npm,
    Uv,
    Rustup,
    Mas,
}

impl ManagerId {
    pub const ALL: [ManagerId; 6] = [
        ManagerId::Brew,
        ManagerId::Mise,
        ManagerId::Npm,
        ManagerId::Uv,
        ManagerId::Rustup,
        ManagerId::Mas,
    ];

    pub fn as_str(&self) -> &'static str {
        match self {
            ManagerId::Brew => "brew",
            ManagerId::Mise => "mise",
            ManagerId::Npm => "npm",
            ManagerId::Uv => "uv",
            ManagerId::Rustup => "rustup",
            ManagerId::Mas => "mas",
        }
    }
}

impl std::fmt::Display for ManagerId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

/// `'brew'|'mise'|'rustup'|'standalone'`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ManagedBy {
    Brew,
    Mise,
    Rustup,
    Standalone,
}

/// `'formula'|'cask'|'caskGreedy'|'tool'|'globalPackage'|'toolchain'|'app'`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum PackageKind {
    Formula,
    Cask,
    CaskGreedy,
    Tool,
    GlobalPackage,
    Toolchain,
    App,
}

/// `'refresh'|'upgrade'|'selfUpdate'|'healthFix'` (wire form; the scheduler's
/// data-carrying `queue::OpKind` maps onto this for IPC).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum OpKind {
    Refresh,
    Upgrade,
    SelfUpdate,
    HealthFix,
}

/// `'queued'|'running'|'succeeded'|'failed'|'cancelled'|'timedOut'|'interrupted'`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum OpStatus {
    Queued,
    Running,
    Succeeded,
    Failed,
    Cancelled,
    TimedOut,
    Interrupted,
}

/// `'loginShell'|'staticFallback'|'merged'` — how the search PATH was built.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum PathSource {
    LoginShell,
    StaticFallback,
    Merged,
}

/// `'present'|'absent'`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ManagerStatus {
    Present,
    Absent,
}

/// `'warning'|'error'`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum HealthSeverity {
    Warning,
    Error,
}

/// `'out'|'err'`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum StreamKind {
    Out,
    Err,
}

/// `'pinned'|'greedyCask'|'rustDedup'|'alreadyRunning'`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ExcludeReason {
    Pinned,
    GreedyCask,
    RustDedup,
    AlreadyRunning,
}

// ---------------------------------------------------------------------------
// Detection
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DetectionReport {
    pub managers: Vec<ManagerInfo>,
    pub env: EnvInfo,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EnvInfo {
    pub path: String,
    pub entries: Vec<String>,
    pub source: PathSource,
    pub home: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ManagerInfo {
    pub id: ManagerId,
    pub display_name: String,
    pub status: ManagerStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub binary_path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub canonical_path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    pub managed_by: ManagedBy,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evidence: Option<String>,
    pub self_update: SelfUpdateRoute,
    /// Absent managers only, e.g. `"brew install mas"`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub install_hint: Option<String>,
}

/// Discriminated union on `kind`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(
    tag = "kind",
    rename_all = "camelCase",
    rename_all_fields = "camelCase"
)]
pub enum SelfUpdateRoute {
    /// rustup; npm (note = mise-reset warning).
    InBand {
        command_preview: String,
        /// Trusted argv excluding the executable. This is deliberately never
        /// serialized: the frontend may display the preview, but it can never
        /// supply executable arguments back to the backend.
        #[serde(skip)]
        command_args: Vec<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        note: Option<String>,
    },
    Routed {
        executor: ManagerId,
        command_preview: String,
        /// Trusted argv excluding the executable (backend-only).
        #[serde(skip)]
        command_args: Vec<String>,
        why: String,
    },
    /// brew.
    ViaRefresh {
        note: String,
    },
    Unavailable {
        reason: String,
    },
}

impl SelfUpdateRoute {
    /// Builds an in-band route with its display preview and executable argv
    /// from the same trusted values. `program` is a display name such as
    /// `npm`; the actual spawn still uses the detected absolute path.
    pub fn in_band(program: &str, command_args: Vec<String>, note: Option<String>) -> Self {
        Self::InBand {
            command_preview: command_preview(program, &command_args),
            command_args,
            note,
        }
    }

    /// Builds a routed route with its preview and argv from one trusted
    /// source. The executor's detected absolute path is bound at submission.
    pub fn routed(
        executor: ManagerId,
        program: &str,
        command_args: Vec<String>,
        why: impl Into<String>,
    ) -> Self {
        Self::Routed {
            executor,
            command_preview: command_preview(program, &command_args),
            command_args,
            why: why.into(),
        }
    }

    /// Returns trusted backend-only argv for executable routes.
    pub(crate) fn command_args(&self) -> Option<&[String]> {
        match self {
            Self::InBand { command_args, .. } | Self::Routed { command_args, .. } => {
                Some(command_args)
            }
            Self::ViaRefresh { .. } | Self::Unavailable { .. } => None,
        }
    }
}

/// Renders the stable, human-visible command form used throughout IPC. It is
/// intentionally a renderer only; executable argv always remains structured.
pub fn command_preview(program: &str, args: &[String]) -> String {
    let mut preview = program.to_string();
    for arg in args {
        preview.push(' ');
        preview.push_str(arg);
    }
    preview
}

// ---------------------------------------------------------------------------
// Packages & snapshots
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Package {
    /// `${kind}:${name}`, name verbatim; split on FIRST ':' only ("tool:npm:prettier").
    pub id: String,
    pub name: String,
    pub kind: PackageKind,
    /// Verbatim; `null` = unknown (no fabricated deltas).
    pub installed: Option<String>,
    pub latest: Option<String>,
    /// The manager's verdict — authoritative.
    pub outdated: bool,
    pub pinned: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta: Option<PackageMeta>,
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PackageMeta {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub executables: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wanted: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub depended_by: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pinned_version: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ManagerSnapshot {
    pub manager_id: ManagerId,
    /// RFC3339.
    pub refreshed_at: String,
    /// Excludes the manager's own self row.
    pub packages: Vec<Package>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub self_status: Option<SelfStatus>,
    pub health: Vec<HealthIssue>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SelfStatus {
    pub installed: Option<String>,
    pub latest: Option<String>,
    pub update_available: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HealthIssue {
    pub id: String,
    pub manager_id: ManagerId,
    pub severity: HealthSeverity,
    pub title: String,
    pub detail: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fix_command: Option<String>,
    /// Trusted argv excluding the executable. Never crosses IPC.
    #[serde(skip)]
    pub fix_args: Option<Vec<String>>,
    pub fixable: bool,
}

// ---------------------------------------------------------------------------
// Plans
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlanRequest {
    /// `null` = all outdated, all managers.
    pub selection: Option<Vec<PlanSelection>>,
    pub include_self_updates: bool,
    pub include_greedy_casks: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlanSelection {
    pub manager_id: ManagerId,
    pub package_id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpgradePlan {
    pub plan_id: String,
    /// Echoed solely so the UI can rebuild the same intent after a stale-plan
    /// rejection. Execution uses the backend-cached request, never this copy.
    pub request: PlanRequest,
    pub groups: Vec<PlanGroup>,
    pub excluded: Vec<ExcludedPackage>,
    pub notes: Vec<String>,
    pub warnings: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlanGroup {
    pub subject: ManagerId,
    pub executor: ManagerId,
    pub locks: Vec<ManagerId>,
    pub commands: Vec<PlanCommand>,
    pub package_ids: Vec<String>,
    pub self_update: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlanCommand {
    pub argv_preview: String,
    pub label: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExcludedPackage {
    pub manager_id: ManagerId,
    pub package_id: String,
    pub reason: ExcludeReason,
}

// ---------------------------------------------------------------------------
// Operations
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OpRef {
    pub op_id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OpIds {
    pub op_ids: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OperationRecord {
    pub op_id: String,
    pub kind: OpKind,
    pub executor: ManagerId,
    pub subject: ManagerId,
    pub status: OpStatus,
    pub command_line: String,
    pub package_ids: Vec<String>,
    pub queued_at: String,
    pub started_at: Option<String>,
    pub finished_at: Option<String>,
    pub exit_code: Option<i32>,
    pub error: Option<IpcError>,
    pub log_path: String,
}

/// Record + ring-buffer replay (cap 5000).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OperationDetail {
    pub record: OperationRecord,
    pub lines: Vec<LogLine>,
    pub truncated: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LogLine {
    pub stream: StreamKind,
    pub line: String,
    pub ts_ms: u64,
}

// ---------------------------------------------------------------------------
// App state & diagnostics
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppState {
    pub detection: DetectionReport,
    pub snapshots: Vec<ManagerSnapshot>,
    pub operations: Vec<OperationRecord>,
    pub settings: Settings,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DiagnosticsResult {
    pub zip_path: String,
}

// ---------------------------------------------------------------------------
// In-app update (Pack-Manager updating itself, not a managed package)
// ---------------------------------------------------------------------------

/// Why a check ran. Manual checks (menu bar → "Check for Updates…", Settings →
/// "Check now") report "you're up to date" and failures to the user; automatic
/// ones stay silent so a flaky network never nags on a timer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum UpdateCheckTrigger {
    Manual,
    Automatic,
}

/// Discriminated union on `kind`, same shape convention as [`SelfUpdateRoute`].
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(
    tag = "kind",
    rename_all = "camelCase",
    rename_all_fields = "camelCase"
)]
pub enum AppUpdateState {
    /// Nothing has been checked yet this session.
    Idle,
    Checking,
    UpToDate,
    /// Downloading happens automatically once an update is found; installing
    /// never does.
    Downloading {
        version: String,
        received: u64,
        total: Option<u64>,
    },
    /// Downloaded and verified; waiting for the user to click Restart.
    ReadyToInstall {
        version: String,
        notes: Option<String>,
    },
    /// Downloaded, but the app bundle's directory is not writable, so the
    /// plugin's install path would raise an admin-password prompt. Pack-Manager
    /// never asks for a password (SPEC §1 invariant 4), so we stop here and
    /// tell the user to move the app or install the DMG by hand.
    ManualInstallRequired {
        version: String,
        reason: String,
    },
    Error {
        message: String,
    },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppUpdateStatus {
    /// `env!("CARGO_PKG_VERSION")` — the running build.
    pub current_version: String,
    pub state: AppUpdateState,
    /// Trigger of the check that produced `state`; `null` before the first one.
    pub last_trigger: Option<UpdateCheckTrigger>,
}

// ---------------------------------------------------------------------------
// Contract test (SPEC §7.4) — byte-equality against dev/fixtures/ipc/*.json.
// Regenerate with `PM_UPDATE_CONTRACT=1 cargo test ipc_contract`.
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::events::{OpOutputEvent, OpStalledEvent, OpStatusEvent, SnapshotUpdatedEvent};
    use serde::de::DeserializeOwned;
    use std::path::PathBuf;

    fn fixtures_dir() -> PathBuf {
        PathBuf::from(concat!(env!("CARGO_MANIFEST_DIR"), "/../dev/fixtures/ipc"))
    }

    /// Serializes `value` pretty + trailing newline and compares byte-for-byte
    /// with the committed fixture; also proves the fixture deserializes back.
    fn check<T: serde::Serialize + DeserializeOwned>(name: &str, value: &T) {
        let path = fixtures_dir().join(name);
        let mut rendered = serde_json::to_string_pretty(value).expect("serialize");
        rendered.push('\n');
        if std::env::var("PM_UPDATE_CONTRACT").is_ok() {
            std::fs::create_dir_all(fixtures_dir()).expect("mkdir fixtures/ipc");
            std::fs::write(&path, &rendered).expect("write fixture");
            return;
        }
        let committed = std::fs::read_to_string(&path)
            .unwrap_or_else(|e| panic!("missing committed fixture {}: {e}", path.display()));
        assert_eq!(
            committed, rendered,
            "IPC contract drift for {name} — if the Rust change is intended, regenerate with \
             PM_UPDATE_CONTRACT=1 and update src/lib/ipc/types.ts to match"
        );
        // The committed bytes must also round-trip through Deserialize.
        let _: T = serde_json::from_str(&committed)
            .unwrap_or_else(|e| panic!("fixture {name} does not deserialize: {e}"));
    }

    fn sample_detection_report() -> DetectionReport {
        DetectionReport {
            managers: vec![
                ManagerInfo {
                    id: ManagerId::Brew,
                    display_name: "Homebrew".into(),
                    status: ManagerStatus::Present,
                    binary_path: Some("/opt/homebrew/bin/brew".into()),
                    canonical_path: Some("/opt/homebrew/bin/brew".into()),
                    version: Some("4.5.2".into()),
                    managed_by: ManagedBy::Standalone,
                    evidence: Some("resolved at /opt/homebrew/bin/brew — Homebrew's own tree".into()),
                    self_update: SelfUpdateRoute::ViaRefresh {
                        note: "brew update runs as part of every refresh".into(),
                    },
                    install_hint: None,
                },
                ManagerInfo {
                    id: ManagerId::Npm,
                    display_name: "npm".into(),
                    status: ManagerStatus::Present,
                    binary_path: Some("/Users/sallvain/.local/share/mise/shims/npm".into()),
                    canonical_path: Some("/opt/homebrew/bin/mise".into()),
                    version: Some("11.16.0".into()),
                    managed_by: ManagedBy::Mise,
                    evidence: Some("resolved at ~/.local/share/mise/shims/npm".into()),
                    self_update: SelfUpdateRoute::InBand {
                        command_preview: "npm install -g npm@latest".into(),
                        command_args: vec![
                            "install".into(),
                            "-g".into(),
                            "npm@latest".into(),
                        ],
                        note: Some(
                            "npm and all global packages live inside the mise-managed node — \
                             upgrading node via mise resets them."
                                .into(),
                        ),
                    },
                    install_hint: None,
                },
                ManagerInfo {
                    id: ManagerId::Uv,
                    display_name: "uv".into(),
                    status: ManagerStatus::Present,
                    binary_path: Some("/Users/sallvain/.local/share/mise/shims/uv".into()),
                    canonical_path: Some("/opt/homebrew/bin/mise".into()),
                    version: Some("0.11.26".into()),
                    managed_by: ManagedBy::Mise,
                    evidence: Some("resolved at ~/.local/share/mise/shims/uv".into()),
                    self_update: SelfUpdateRoute::Routed {
                        executor: ManagerId::Mise,
                        command_preview: "mise upgrade uv".into(),
                        command_args: vec!["upgrade".into(), "uv".into()],
                        why: "uv is managed by mise".into(),
                    },
                    install_hint: None,
                },
                ManagerInfo {
                    id: ManagerId::Mas,
                    display_name: "mas".into(),
                    status: ManagerStatus::Absent,
                    binary_path: None,
                    canonical_path: None,
                    version: None,
                    managed_by: ManagedBy::Standalone,
                    evidence: None,
                    self_update: SelfUpdateRoute::Unavailable {
                        reason: "mas is not installed".into(),
                    },
                    install_hint: Some("brew install mas".into()),
                },
            ],
            env: EnvInfo {
                path: "/Users/sallvain/.local/share/mise/shims:/opt/homebrew/bin:/opt/homebrew/sbin:/Users/sallvain/.cargo/bin:/usr/local/bin:/usr/bin:/bin:/usr/sbin:/sbin".into(),
                entries: vec![
                    "/Users/sallvain/.local/share/mise/shims".into(),
                    "/opt/homebrew/bin".into(),
                    "/opt/homebrew/sbin".into(),
                    "/Users/sallvain/.cargo/bin".into(),
                    "/usr/local/bin".into(),
                    "/usr/bin".into(),
                    "/bin".into(),
                    "/usr/sbin".into(),
                    "/sbin".into(),
                ],
                source: PathSource::Merged,
                home: "/Users/sallvain".into(),
            },
        }
    }

    fn sample_uv_snapshot() -> ManagerSnapshot {
        ManagerSnapshot {
            manager_id: ManagerId::Uv,
            refreshed_at: "2026-07-22T14:05:00Z".into(),
            packages: vec![
                Package {
                    id: "tool:ruff".into(),
                    name: "ruff".into(),
                    kind: PackageKind::Tool,
                    installed: Some("0.15.20".into()),
                    // uv outdated format under-verified → latest stays null,
                    // UI shows "update available" (no fabricated delta).
                    latest: None,
                    outdated: true,
                    pinned: false,
                    meta: Some(PackageMeta {
                        executables: Some(vec!["ruff".into()]),
                        ..PackageMeta::default()
                    }),
                },
                Package {
                    id: "tool:serena-agent".into(),
                    name: "serena-agent".into(),
                    kind: PackageKind::Tool,
                    installed: Some("1.6.2.dev0".into()),
                    latest: Some("1.6.2.dev0".into()),
                    outdated: false,
                    pinned: false,
                    meta: Some(PackageMeta {
                        executables: Some(vec![
                            "serena".into(),
                            "serena-agent".into(),
                            "index-project".into(),
                        ]),
                        ..PackageMeta::default()
                    }),
                },
            ],
            self_status: Some(SelfStatus {
                installed: Some("0.11.26".into()),
                latest: Some("0.11.30".into()),
                update_available: true,
            }),
            health: vec![HealthIssue {
                id: "uv:aider-chat".into(),
                manager_id: ManagerId::Uv,
                severity: HealthSeverity::Warning,
                title: "Tool `aider-chat` environment is broken.".into(),
                detail: "warning: Tool `aider-chat` environment not found (run `uv tool install \
                         aider-chat --reinstall` to reinstall)"
                    .into(),
                fix_command: Some("uv tool install aider-chat --reinstall".into()),
                fix_args: Some(vec![
                    "tool".into(),
                    "install".into(),
                    "aider-chat".into(),
                    "--reinstall".into(),
                ]),
                fixable: true,
            }],
        }
    }

    fn sample_npm_snapshot() -> ManagerSnapshot {
        ManagerSnapshot {
            manager_id: ManagerId::Npm,
            refreshed_at: "2026-07-22T14:03:11Z".into(),
            packages: vec![Package {
                id: "globalPackage:typescript".into(),
                name: "typescript".into(),
                kind: PackageKind::GlobalPackage,
                installed: Some("6.0.3".into()),
                latest: Some("7.0.2".into()),
                outdated: true,
                pinned: false,
                meta: Some(PackageMeta {
                    wanted: Some("7.0.2".into()),
                    depended_by: Some("global".into()),
                    ..PackageMeta::default()
                }),
            }],
            self_status: Some(SelfStatus {
                installed: Some("11.16.0".into()),
                latest: Some("12.0.1".into()),
                update_available: true,
            }),
            health: vec![],
        }
    }

    const OP_ID: &str = "01981f2e-6a3b-7c40-9d5e-1f2a3b4c5d6e";
    const LOG_PATH: &str = "/Users/sallvain/Library/Logs/Pack-Manager/operations/2026-07-22T14-03-11_01981f2e_npm_upgrade.log";

    fn sample_operation_record() -> OperationRecord {
        OperationRecord {
            op_id: OP_ID.into(),
            kind: OpKind::Upgrade,
            executor: ManagerId::Npm,
            subject: ManagerId::Npm,
            status: OpStatus::Succeeded,
            command_line:
                "/Users/sallvain/.local/share/mise/shims/npm install -g typescript@latest".into(),
            package_ids: vec!["globalPackage:typescript".into()],
            queued_at: "2026-07-22T14:03:11Z".into(),
            started_at: Some("2026-07-22T14:03:11Z".into()),
            finished_at: Some("2026-07-22T14:03:16Z".into()),
            exit_code: Some(0),
            error: None,
            log_path: LOG_PATH.into(),
        }
    }

    #[test]
    fn ipc_contract_matches_committed_fixtures() {
        check("detection_report.json", &sample_detection_report());
        check("manager_snapshot.json", &sample_uv_snapshot());

        check(
            "plan_request.json",
            &PlanRequest {
                selection: Some(vec![
                    PlanSelection {
                        manager_id: ManagerId::Brew,
                        package_id: "formula:dolt".into(),
                    },
                    PlanSelection {
                        manager_id: ManagerId::Npm,
                        package_id: "globalPackage:typescript".into(),
                    },
                ]),
                include_self_updates: true,
                include_greedy_casks: false,
            },
        );

        check(
            "upgrade_plan.json",
            &UpgradePlan {
                plan_id: "01981f2e-0000-7000-8000-5f8cff3fb96b".into(),
                request: PlanRequest {
                    selection: Some(vec![
                        PlanSelection {
                            manager_id: ManagerId::Brew,
                            package_id: "formula:dolt".into(),
                        },
                        PlanSelection {
                            manager_id: ManagerId::Npm,
                            package_id: "globalPackage:typescript".into(),
                        },
                    ]),
                    include_self_updates: true,
                    include_greedy_casks: false,
                },
                groups: vec![
                    PlanGroup {
                        subject: ManagerId::Brew,
                        executor: ManagerId::Brew,
                        locks: vec![ManagerId::Brew],
                        commands: vec![PlanCommand {
                            argv_preview: "brew upgrade dolt".into(),
                            label: "Upgrade 1 formula".into(),
                        }],
                        package_ids: vec!["formula:dolt".into()],
                        self_update: false,
                    },
                    PlanGroup {
                        subject: ManagerId::Mise,
                        executor: ManagerId::Brew,
                        locks: vec![ManagerId::Brew, ManagerId::Mise],
                        commands: vec![PlanCommand {
                            argv_preview: "brew upgrade mise".into(),
                            label: "Self-update mise via Homebrew".into(),
                        }],
                        package_ids: vec![],
                        self_update: true,
                    },
                ],
                excluded: vec![
                    ExcludedPackage {
                        manager_id: ManagerId::Brew,
                        package_id: "formula:deno".into(),
                        reason: ExcludeReason::Pinned,
                    },
                    ExcludedPackage {
                        manager_id: ManagerId::Mise,
                        package_id: "tool:rust".into(),
                        reason: ExcludeReason::RustDedup,
                    },
                ],
                notes: vec!["rust toolchains are handled by rustup in this plan".into()],
                warnings: vec!["mise: list may be stale — last check errored".into()],
            },
        );

        check(
            "op_ref.json",
            &OpRef {
                op_id: OP_ID.into(),
            },
        );
        check("operation_record.json", &sample_operation_record());

        check(
            "operation_detail.json",
            &OperationDetail {
                record: OperationRecord {
                    status: OpStatus::Failed,
                    exit_code: Some(1),
                    error: Some(
                        IpcError::from_code(
                            ErrorCode::BrewLockBusy,
                            "Homebrew is busy in another terminal. Retry when it finishes.",
                        )
                        .with_detail(
                            "Error: Another active Homebrew update process is already in progress.",
                        )
                        .with_manager(ManagerId::Brew)
                        .with_op(OP_ID)
                        .with_log_path(LOG_PATH),
                    ),
                    ..sample_operation_record()
                },
                lines: vec![
                    LogLine {
                        stream: StreamKind::Out,
                        line: "==> Upgrading dolt".into(),
                        ts_ms: 1753192991104,
                    },
                    LogLine {
                        stream: StreamKind::Err,
                        line:
                            "Error: Another active Homebrew update process is already in progress."
                                .into(),
                        ts_ms: 1753192991371,
                    },
                ],
                truncated: false,
            },
        );

        check(
            "app_state.json",
            &AppState {
                detection: sample_detection_report(),
                snapshots: vec![sample_npm_snapshot()],
                operations: vec![OperationRecord {
                    status: OpStatus::Queued,
                    started_at: None,
                    finished_at: None,
                    exit_code: None,
                    error: None,
                    ..sample_operation_record()
                }],
                settings: Settings::default(),
            },
        );

        check("settings.json", &Settings::default());

        // The in-app updater's richest state: a discriminated-union variant
        // carrying data, so drift in the `kind` tag or the field casing fails
        // here rather than in the status bar.
        check(
            "event_app_update_status.json",
            &AppUpdateStatus {
                current_version: "0.1.1".into(),
                state: AppUpdateState::ReadyToInstall {
                    version: "0.2.0".into(),
                    notes: Some("Adds in-app updates.".into()),
                },
                last_trigger: Some(UpdateCheckTrigger::Automatic),
            },
        );

        check(
            "ipc_error.json",
            &IpcError::from_code(
                ErrorCode::Timeout,
                "Homebrew refresh timed out after 600s. Check your network and retry.",
            )
            .with_detail("phase: brew update")
            .with_manager(ManagerId::Brew)
            .with_op(OP_ID)
            .with_log_path(LOG_PATH),
        );

        check(
            "event_snapshot_updated.json",
            &SnapshotUpdatedEvent {
                manager_id: ManagerId::Uv,
                snapshot: sample_uv_snapshot(),
            },
        );

        check(
            "event_op_status.json",
            &OpStatusEvent {
                op_id: OP_ID.into(),
                kind: OpKind::Refresh,
                executor: ManagerId::Brew,
                subject: ManagerId::Brew,
                status: OpStatus::Running,
                queue_position: None,
                phase_label: Some("Updating Homebrew metadata…".into()),
                command_line: "/opt/homebrew/bin/brew update".into(),
                exit_code: None,
                error: None,
                started_at: Some("2026-07-22T14:03:11Z".into()),
                finished_at: None,
                log_path: LOG_PATH.into(),
            },
        );

        check(
            "event_op_output.json",
            &OpOutputEvent {
                op_id: OP_ID.into(),
                batch: vec![
                    LogLine {
                        stream: StreamKind::Out,
                        line: "added 1 package in 4s".into(),
                        ts_ms: 1753192992104,
                    },
                    LogLine {
                        stream: StreamKind::Err,
                        line: "npm warn deprecated inflight@1.0.6".into(),
                        ts_ms: 1753192992371,
                    },
                ],
            },
        );

        check(
            "event_op_stalled.json",
            &OpStalledEvent {
                op_id: OP_ID.into(),
                silent_for_secs: 120,
            },
        );
    }
}
