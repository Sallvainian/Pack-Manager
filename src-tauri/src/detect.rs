//! Detection & managed-by classification (SPEC §5.3). U4 implements
//! `classify_managed_by` (raw-path mise rule BEFORE canonicalize — THE
//! regression trap, see DECISIONS D3), detection orchestration, and route
//! precedence. U1 provides only the shared `DetectStatus` shape the
//! `ManagerAdapter` trait needs.

use std::path::PathBuf;

use crate::ipc::ManagedBy;

/// Per-manager detection outcome. Absence is a normal state, never an error.
#[derive(Debug, Clone, PartialEq)]
pub enum DetectStatus {
    Absent {
        reason: String,
    },
    Present {
        binary_path: PathBuf,
        canonical_path: PathBuf,
        version: Option<String>,
        managed_by: ManagedBy,
        evidence: String,
    },
}
