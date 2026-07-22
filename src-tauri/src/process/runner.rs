//! `CommandRunner` — the single seam for every child process (SPEC §5.6, §7).
//!
//! U2 adds `RealRunner` here: `tokio::process::Command` with
//! `.process_group(0)`, `.stdin(Stdio::null())`, line readers with `\r` split
//! and ANSI stripping, 512KiB caps, stall watchdog, absolute timeout, and
//! SIGTERM → 5s grace → SIGKILL via `nix::killpg`.

use async_trait::async_trait;
use tokio_util::sync::CancellationToken;

use crate::error::PmError;
use crate::process::{CommandOutput, CommandSpec, LineSink};

#[async_trait]
pub trait CommandRunner: Send + Sync {
    /// Buffered execution (refresh/detection).
    async fn run(&self, spec: &CommandSpec) -> Result<CommandOutput, PmError>;

    /// Streaming execution (upgrades/self-updates): every line goes to `sink`
    /// as it arrives; `cancel` triggers SIGTERM → 5s grace → SIGKILL on the
    /// process group.
    async fn run_streaming(
        &self,
        spec: &CommandSpec,
        sink: LineSink,
        cancel: CancellationToken,
    ) -> Result<CommandOutput, PmError>;
}
