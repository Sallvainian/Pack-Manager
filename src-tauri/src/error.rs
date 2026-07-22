//! Error taxonomy (SPEC §5.10) and its IPC serialization.
//!
//! `PmError` is the internal error type; it crosses the IPC boundary as
//! `IpcError { code, message, detail?, managerId?, opId?, logPath? }`.
//! Rules enforced elsewhere: `ExpectedNonZero` never becomes `NonZeroExit`;
//! `ParseFailed` on refresh keeps the previous snapshot; `BrewLockBusy` is a
//! distinct user-facing state with NO automatic retry. `logPath` is always
//! populated for op-scoped errors — "View log" never dangles.

use serde::{Deserialize, Serialize};

use crate::ipc::ManagerId;

#[derive(thiserror::Error, Debug, Clone, PartialEq)]
pub enum PmError {
    #[error("tool `{tool}` not found (searched {searched:?})")]
    ToolNotFound { tool: String, searched: Vec<String> },

    #[error("failed to spawn `{program}`: {detail}")]
    SpawnFailed { program: String, detail: String },

    #[error("timed out after {after_secs}s ({phase})")]
    Timeout { after_secs: u64, phase: String },

    /// Last 20 lines of stderr; consulted AFTER `classify_exit`.
    #[error("command exited with code {code}")]
    NonZeroExit { code: i32, stderr_tail: String },

    /// stderr matches "Another active Homebrew process".
    #[error("Homebrew is busy in another terminal")]
    BrewLockBusy { detail: String },

    /// `excerpt` = first 500 chars of the offending output.
    #[error("failed to parse {what}")]
    ParseFailed { what: String, excerpt: String },

    #[error("operation cancelled")]
    Cancelled,

    #[error("self-update unavailable: {reason}")]
    SelfUpdateUnavailable { reason: String },

    /// Login-shell probe failed (non-fatal, reported in Environment Report).
    #[error("environment capture failed: {detail}")]
    EnvCaptureFailed { detail: String },

    #[error("I/O error: {detail}")]
    Io { detail: String },

    #[error("internal error: {detail}")]
    Internal { detail: String },
}

impl From<std::io::Error> for PmError {
    fn from(e: std::io::Error) -> Self {
        PmError::Io {
            detail: e.to_string(),
        }
    }
}

/// Wire codes, snake_case:
/// `tool_not_found | spawn_failed | timeout | non_zero_exit | brew_lock_busy |
///  parse_failed | cancelled | self_update_unavailable | env_capture_failed | io | internal`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ErrorCode {
    ToolNotFound,
    SpawnFailed,
    Timeout,
    NonZeroExit,
    BrewLockBusy,
    ParseFailed,
    Cancelled,
    SelfUpdateUnavailable,
    EnvCaptureFailed,
    Io,
    Internal,
}

/// The serialized error shape all 17 commands return in their `Err` arm and
/// that rides inside `OperationRecord.error` / `op:status`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IpcError {
    pub code: ErrorCode,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manager_id: Option<ManagerId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub op_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_path: Option<String>,
}

impl IpcError {
    pub fn from_code(code: ErrorCode, message: impl Into<String>) -> Self {
        IpcError {
            code,
            message: message.into(),
            detail: None,
            manager_id: None,
            op_id: None,
            log_path: None,
        }
    }

    pub fn internal(message: impl Into<String>) -> Self {
        Self::from_code(ErrorCode::Internal, message)
    }

    pub fn with_detail(mut self, detail: impl Into<String>) -> Self {
        self.detail = Some(detail.into());
        self
    }

    pub fn with_manager(mut self, manager_id: ManagerId) -> Self {
        self.manager_id = Some(manager_id);
        self
    }

    pub fn with_op(mut self, op_id: impl Into<String>) -> Self {
        self.op_id = Some(op_id.into());
        self
    }

    pub fn with_log_path(mut self, log_path: impl Into<String>) -> Self {
        self.log_path = Some(log_path.into());
        self
    }
}

impl std::fmt::Display for IpcError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for IpcError {}

impl From<PmError> for IpcError {
    fn from(e: PmError) -> Self {
        let message = e.to_string();
        match e {
            PmError::ToolNotFound { searched, .. } => {
                Self::from_code(ErrorCode::ToolNotFound, message)
                    .with_detail(format!("searched: {}", searched.join(", ")))
            }
            PmError::SpawnFailed { detail, .. } => {
                Self::from_code(ErrorCode::SpawnFailed, message).with_detail(detail)
            }
            PmError::Timeout { .. } => Self::from_code(ErrorCode::Timeout, message),
            PmError::NonZeroExit { stderr_tail, .. } => {
                Self::from_code(ErrorCode::NonZeroExit, message).with_detail(stderr_tail)
            }
            PmError::BrewLockBusy { detail } => Self::from_code(
                ErrorCode::BrewLockBusy,
                "Homebrew is busy in another terminal. Retry when it finishes.",
            )
            .with_detail(detail),
            PmError::ParseFailed { excerpt, .. } => {
                Self::from_code(ErrorCode::ParseFailed, message).with_detail(excerpt)
            }
            PmError::Cancelled => Self::from_code(ErrorCode::Cancelled, message),
            PmError::SelfUpdateUnavailable { reason } => {
                Self::from_code(ErrorCode::SelfUpdateUnavailable, message).with_detail(reason)
            }
            PmError::EnvCaptureFailed { detail } => {
                Self::from_code(ErrorCode::EnvCaptureFailed, message).with_detail(detail)
            }
            PmError::Io { detail } => Self::from_code(ErrorCode::Io, message).with_detail(detail),
            PmError::Internal { detail } => {
                Self::from_code(ErrorCode::Internal, message).with_detail(detail)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_pm_error_variant_maps_to_its_wire_code() {
        let cases: Vec<(PmError, ErrorCode)> = vec![
            (
                PmError::ToolNotFound {
                    tool: "brew".into(),
                    searched: vec!["/usr/bin".into()],
                },
                ErrorCode::ToolNotFound,
            ),
            (
                PmError::SpawnFailed {
                    program: "/opt/homebrew/bin/brew".into(),
                    detail: "ENOENT".into(),
                },
                ErrorCode::SpawnFailed,
            ),
            (
                PmError::Timeout {
                    after_secs: 600,
                    phase: "brew update".into(),
                },
                ErrorCode::Timeout,
            ),
            (
                PmError::NonZeroExit {
                    code: 1,
                    stderr_tail: "boom".into(),
                },
                ErrorCode::NonZeroExit,
            ),
            (
                PmError::BrewLockBusy {
                    detail: "Another active Homebrew update process is already in progress.".into(),
                },
                ErrorCode::BrewLockBusy,
            ),
            (
                PmError::ParseFailed {
                    what: "brew outdated --json=v2".into(),
                    excerpt: "not json".into(),
                },
                ErrorCode::ParseFailed,
            ),
            (PmError::Cancelled, ErrorCode::Cancelled),
            (
                PmError::SelfUpdateUnavailable {
                    reason: "mas is not installed".into(),
                },
                ErrorCode::SelfUpdateUnavailable,
            ),
            (
                PmError::EnvCaptureFailed {
                    detail: "probe timed out".into(),
                },
                ErrorCode::EnvCaptureFailed,
            ),
            (
                PmError::Io {
                    detail: "disk full".into(),
                },
                ErrorCode::Io,
            ),
            (
                PmError::Internal {
                    detail: "unreachable".into(),
                },
                ErrorCode::Internal,
            ),
        ];
        for (err, code) in cases {
            let ipc: IpcError = err.into();
            assert_eq!(ipc.code, code);
            assert!(!ipc.message.is_empty());
        }
    }

    #[test]
    fn brew_lock_busy_message_is_the_actionable_copy() {
        let ipc: IpcError = PmError::BrewLockBusy {
            detail: "lock held".into(),
        }
        .into();
        assert_eq!(
            ipc.message,
            "Homebrew is busy in another terminal. Retry when it finishes."
        );
    }
}
