//! Event surface (SPEC §5.9 events table): `EventSink` trait, the five event
//! payload structs, and `VecSink` for tests. Core logic emits through
//! `EventSink` and never touches `tauri::AppHandle`.
//!
//! U2 completes this file with the batching emitter (`op:output` flushed every
//! 50ms / 64 lines / 8KiB, whichever first) and the Tauri-backed sink.

use serde::{Deserialize, Serialize};

use crate::error::IpcError;
use crate::ipc::{DetectionReport, LogLine, ManagerId, ManagerSnapshot, OpKind, OpStatus};

// ---------------------------------------------------------------------------
// Event payloads (wire contract; mirrored in src/lib/ipc/types.ts)
// ---------------------------------------------------------------------------

/// Payload of `snapshot:updated` (health rides in the snapshot).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SnapshotUpdatedEvent {
    pub manager_id: ManagerId,
    pub snapshot: ManagerSnapshot,
}

/// Payload of `op:status` — emitted on enqueue (queued), start, phase change, finish.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OpStatusEvent {
    pub op_id: String,
    pub kind: OpKind,
    pub executor: ManagerId,
    pub subject: ManagerId,
    pub status: OpStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queue_position: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phase_label: Option<String>,
    pub command_line: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exit_code: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<IpcError>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finished_at: Option<String>,
    pub log_path: String,
}

/// Payload of `op:output` — batched ≤50ms / 64 lines / 8KiB.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OpOutputEvent {
    pub op_id: String,
    pub batch: Vec<LogLine>,
}

/// Payload of `op:stalled`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OpStalledEvent {
    pub op_id: String,
    pub silent_for_secs: u64,
}

// ---------------------------------------------------------------------------
// AppEvent + EventSink
// ---------------------------------------------------------------------------

pub const EVENT_DETECTION_UPDATED: &str = "detection:updated";
pub const EVENT_SNAPSHOT_UPDATED: &str = "snapshot:updated";
pub const EVENT_OP_STATUS: &str = "op:status";
pub const EVENT_OP_OUTPUT: &str = "op:output";
pub const EVENT_OP_STALLED: &str = "op:stalled";

/// One of the five events, name + typed payload.
#[derive(Debug, Clone, PartialEq)]
pub enum AppEvent {
    DetectionUpdated(DetectionReport),
    SnapshotUpdated(SnapshotUpdatedEvent),
    OpStatus(OpStatusEvent),
    OpOutput(OpOutputEvent),
    OpStalled(OpStalledEvent),
}

impl AppEvent {
    pub fn name(&self) -> &'static str {
        match self {
            AppEvent::DetectionUpdated(_) => EVENT_DETECTION_UPDATED,
            AppEvent::SnapshotUpdated(_) => EVENT_SNAPSHOT_UPDATED,
            AppEvent::OpStatus(_) => EVENT_OP_STATUS,
            AppEvent::OpOutput(_) => EVENT_OP_OUTPUT,
            AppEvent::OpStalled(_) => EVENT_OP_STALLED,
        }
    }

    pub fn payload_json(&self) -> serde_json::Value {
        match self {
            AppEvent::DetectionUpdated(p) => serde_json::to_value(p),
            AppEvent::SnapshotUpdated(p) => serde_json::to_value(p),
            AppEvent::OpStatus(p) => serde_json::to_value(p),
            AppEvent::OpOutput(p) => serde_json::to_value(p),
            AppEvent::OpStalled(p) => serde_json::to_value(p),
        }
        .expect("event payloads are plain data and always serialize")
    }
}

/// The seam between core logic and the window. The Tauri-backed implementation
/// (U2) forwards to `AppHandle::emit`; tests use [`VecSink`].
pub trait EventSink: Send + Sync {
    fn emit(&self, event: AppEvent);
}

/// Test sink: records every emitted event in order.
#[derive(Debug, Default)]
pub struct VecSink {
    events: std::sync::Mutex<Vec<AppEvent>>,
}

impl VecSink {
    pub fn new() -> Self {
        Self::default()
    }

    /// Clones the events recorded so far.
    pub fn events(&self) -> Vec<AppEvent> {
        self.events.lock().expect("VecSink poisoned").clone()
    }

    /// Drains and returns the events recorded so far.
    pub fn take(&self) -> Vec<AppEvent> {
        std::mem::take(&mut *self.events.lock().expect("VecSink poisoned"))
    }
}

impl EventSink for VecSink {
    fn emit(&self, event: AppEvent) {
        self.events.lock().expect("VecSink poisoned").push(event);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vec_sink_records_in_order_and_names_are_stable() {
        let sink = VecSink::new();
        sink.emit(AppEvent::OpStalled(OpStalledEvent {
            op_id: "op-1".into(),
            silent_for_secs: 120,
        }));
        sink.emit(AppEvent::OpOutput(OpOutputEvent {
            op_id: "op-1".into(),
            batch: vec![],
        }));
        let events = sink.take();
        assert_eq!(events.len(), 2);
        assert_eq!(events[0].name(), "op:stalled");
        assert_eq!(events[1].name(), "op:output");
        assert!(sink.events().is_empty());
    }

    #[test]
    fn payload_json_uses_camel_case_fields() {
        let ev = AppEvent::OpStalled(OpStalledEvent {
            op_id: "op-1".into(),
            silent_for_secs: 120,
        });
        let v = ev.payload_json();
        assert_eq!(v["opId"], "op-1");
        assert_eq!(v["silentForSecs"], 120);
    }
}
