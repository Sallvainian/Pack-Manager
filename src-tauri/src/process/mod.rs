//! Process execution seam (SPEC §5.6). This module holds the shared data
//! types; the `CommandRunner` trait lives in [`runner`]; `RealRunner` and
//! `FakeRunner` are implemented by U2.

pub mod fake;
pub mod runner;

use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;

pub use crate::ipc::{LogLine, StreamKind};
pub use crate::managers::Timeout;
pub use runner::CommandRunner;

/// Drives logging/UI treatment of a spawned command.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CmdPurpose {
    Detection,
    Refresh,
    Upgrade,
    SelfUpdate,
    HealthFix,
}

/// A fully-bound command: absolute program path + constructed env (children
/// never inherit our environment; see SPEC §5.2).
#[derive(Debug, Clone, PartialEq)]
pub struct CommandSpec {
    /// Absolute path — children are spawned by absolute path, never PATH lookup.
    pub program: PathBuf,
    pub args: Vec<String>,
    /// The FULL constructed environment for the child.
    pub env: Vec<(String, String)>,
    pub timeout: Timeout,
    pub purpose: CmdPurpose,
}

/// Buffered result of a finished command. `stdout`/`stderr` retention is
/// capped at 512KiB per stream by the runner.
#[derive(Debug, Clone, PartialEq)]
pub struct CommandOutput {
    /// `None` when the process was killed by a signal.
    pub exit_code: Option<i32>,
    pub stdout: String,
    pub stderr: String,
    pub duration: Duration,
}

/// Callback receiving each parsed output line during streaming execution.
pub type LineSink = Arc<dyn Fn(LogLine) + Send + Sync>;
