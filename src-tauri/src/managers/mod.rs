//! Manager adapter contract (SPEC §5.4): pure plans, generic execution.
//! Adapter implementations (`brew.rs` … `mas.rs`) arrive in U4; pure parsers in
//! `parse/` arrive in U3.

pub mod brew;
pub mod mas;
pub mod mise;
pub mod npm;
pub mod parse;
pub mod rustup;
pub mod uv;

use std::time::Duration;

use async_trait::async_trait;

pub use crate::ipc::{ManagedBy, ManagerId, SelfUpdateRoute};

use crate::detect::DetectStatus;
use crate::error::PmError;
use crate::ipc::{ManagerSnapshot, Package};
use crate::process::CommandOutput;
use crate::settings::Settings;

/// Package ids are `${kind}:${name}` strings (split on FIRST ':' only).
pub type PackageId = String;

/// Options threaded from a `PlanRequest` into per-adapter upgrade planning.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct PlanOptions {
    pub include_self_updates: bool,
    pub include_greedy_casks: bool,
}

/// One command inside an operation's serial spec list, before it is bound to
/// an absolute program path + constructed env (that binding happens in U5).
#[derive(Debug, Clone, PartialEq)]
pub struct PlannedCommand {
    pub label: &'static str,
    pub argv: Vec<String>,
    pub timeout: Timeout,
    pub extra_env: Vec<(String, String)>,
    /// UI phase label, e.g. "Updating Homebrew metadata…".
    pub phase_label: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Timeout {
    Absolute(Duration),
    Stall {
        silence: Duration,
        hard_cap: Duration,
    },
}

/// How a command's exit should be interpreted (SPEC: `npm outdated -g --json`
/// exits 1 with parseable JSON = success; `ExpectedNonZero` never becomes
/// `NonZeroExit`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExitClass {
    Success,
    ExpectedNonZero,
    Failure,
}

#[async_trait]
pub trait ManagerAdapter: Send + Sync {
    fn id(&self) -> ManagerId;
    fn display_name(&self) -> &'static str;
    fn binary_name(&self) -> &'static str;
    /// Fixed-path fallbacks probed when PATH resolution misses.
    fn detection_candidates(&self) -> &'static [&'static str];

    /// PURE: ordered refresh commands (run serially inside one Refresh op).
    fn refresh_plan(&self, det: &DetectStatus, settings: &Settings) -> Vec<PlannedCommand>;

    /// PURE, fixture-tested: outputs in refresh_plan order -> snapshot.
    fn parse_refresh(&self, outputs: &[CommandOutput]) -> Result<ManagerSnapshot, PmError>;

    /// PURE: one recovery command when a given spec's output failed to parse
    /// (mise/npm/brew-casks text fallback).
    fn recovery_plan(&self, failed: &PlannedCommand) -> Option<PlannedCommand>;

    /// PURE: rebuilds the WHOLE snapshot from the recovery command's output.
    /// `refresh_outputs` are the already-captured outputs of the normal
    /// refresh plan (in `refresh_plan` order) — the inventory parsed from them
    /// must be merged with the recovered overlay, or every up-to-date package
    /// would vanish from the table whenever recovery fires.
    fn parse_recovery(
        &self,
        failed: &PlannedCommand,
        refresh_outputs: &[CommandOutput],
        out: &CommandOutput,
    ) -> Result<ManagerSnapshot, PmError>;

    /// PURE: exact argv for upgrading the given package ids.
    fn upgrade_plan(&self, pkgs: &[PackageId], opts: &PlanOptions) -> Vec<PlannedCommand>;

    fn self_update_route(
        &self,
        managed_by: ManagedBy,
        own_outdated_row: Option<&Package>,
    ) -> SelfUpdateRoute;

    /// Non-zero exits that still mean success for a command.
    fn classify_exit(&self, cmd: &PlannedCommand, out: &CommandOutput) -> ExitClass;
}
