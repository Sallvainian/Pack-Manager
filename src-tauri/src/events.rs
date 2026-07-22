//! Event surface (SPEC §5.9 events table, plus `appUpdate:status` per
//! DECISIONS D25): `EventSink` trait, the event payload structs, `VecSink` for
//! tests, the batching emitter (`op:output` flushed every 50ms / 64 lines /
//! 8KiB, whichever first), and the Tauri-backed sink. Core logic emits through
//! `EventSink` and never touches `tauri::AppHandle`.

use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;

use serde::{Deserialize, Serialize};
use tokio::sync::mpsc;
use tokio::time::Instant;

use crate::error::IpcError;
use crate::ipc::{
    AppUpdateStatus, DetectionReport, LogLine, ManagerId, ManagerSnapshot, OpKind, OpStatus,
};

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
pub const EVENT_APP_UPDATE_STATUS: &str = "appUpdate:status";

/// One of the six events, name + typed payload.
#[derive(Debug, Clone, PartialEq)]
pub enum AppEvent {
    DetectionUpdated(DetectionReport),
    SnapshotUpdated(SnapshotUpdatedEvent),
    OpStatus(OpStatusEvent),
    OpOutput(OpOutputEvent),
    OpStalled(OpStalledEvent),
    /// Pack-Manager updating itself (DECISIONS D25) — unrelated to `op:*`,
    /// which only ever describe package-manager operations.
    AppUpdateStatus(AppUpdateStatus),
}

impl AppEvent {
    pub fn name(&self) -> &'static str {
        match self {
            AppEvent::DetectionUpdated(_) => EVENT_DETECTION_UPDATED,
            AppEvent::SnapshotUpdated(_) => EVENT_SNAPSHOT_UPDATED,
            AppEvent::OpStatus(_) => EVENT_OP_STATUS,
            AppEvent::OpOutput(_) => EVENT_OP_OUTPUT,
            AppEvent::OpStalled(_) => EVENT_OP_STALLED,
            AppEvent::AppUpdateStatus(_) => EVENT_APP_UPDATE_STATUS,
        }
    }

    pub fn payload_json(&self) -> serde_json::Value {
        match self {
            AppEvent::DetectionUpdated(p) => serde_json::to_value(p),
            AppEvent::SnapshotUpdated(p) => serde_json::to_value(p),
            AppEvent::OpStatus(p) => serde_json::to_value(p),
            AppEvent::OpOutput(p) => serde_json::to_value(p),
            AppEvent::OpStalled(p) => serde_json::to_value(p),
            AppEvent::AppUpdateStatus(p) => serde_json::to_value(p),
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

/// The Tauri-backed sink: forwards every event to the window via
/// `AppHandle::emit`. Constructed once in `lib.rs` (U5); everything else
/// depends only on `EventSink`.
pub struct TauriSink {
    handle: tauri::AppHandle,
}

impl TauriSink {
    pub fn new(handle: tauri::AppHandle) -> Self {
        Self { handle }
    }
}

impl EventSink for TauriSink {
    fn emit(&self, event: AppEvent) {
        use tauri::Emitter as _;
        if let Err(e) = self.handle.emit(event.name(), event.payload_json()) {
            tracing::warn!(event = event.name(), error = %e, "event emit failed");
        }
    }
}

// ---------------------------------------------------------------------------
// Batching emitter for `op:output`
// ---------------------------------------------------------------------------

/// Flush when a batch reaches this many lines…
pub const BATCH_MAX_LINES: usize = 64;
/// …or this many payload bytes (force-flush)…
pub const BATCH_MAX_BYTES: usize = 8 * 1024;
/// …or this much time since the batch's first line — whichever first.
pub const BATCH_MAX_DELAY: Duration = Duration::from_millis(50);

enum BatchMsg {
    Line { op_id: String, line: LogLine },
    FlushOp { op_id: String },
}

/// Batches per-op output lines into `op:output` events (≤50ms / 64 lines /
/// 8KiB, whichever first — SPEC §5.9). Requires a tokio runtime; dropping the
/// emitter flushes every remaining buffer.
pub struct BatchingEmitter {
    tx: mpsc::UnboundedSender<BatchMsg>,
}

impl BatchingEmitter {
    pub fn new(sink: Arc<dyn EventSink>) -> Self {
        let (tx, rx) = mpsc::unbounded_channel();
        tokio::spawn(batch_task(rx, sink));
        Self { tx }
    }

    /// Queues one line for `op_id`.
    pub fn push(&self, op_id: &str, line: LogLine) {
        let _ = self.tx.send(BatchMsg::Line {
            op_id: op_id.to_string(),
            line,
        });
    }

    /// Flushes `op_id`'s buffer immediately (terminal op state — the last
    /// lines must not wait out the 50ms timer).
    pub fn flush_op(&self, op_id: &str) {
        let _ = self.tx.send(BatchMsg::FlushOp {
            op_id: op_id.to_string(),
        });
    }

    /// A `LineSink` bound to one op — the adapter between `CommandRunner`
    /// streaming and this emitter.
    pub fn line_sink(&self, op_id: impl Into<String>) -> crate::process::LineSink {
        let tx = self.tx.clone();
        let op_id = op_id.into();
        Arc::new(move |line: LogLine| {
            let _ = tx.send(BatchMsg::Line {
                op_id: op_id.clone(),
                line,
            });
        })
    }
}

struct Batch {
    lines: Vec<LogLine>,
    bytes: usize,
    deadline: Instant,
}

fn flush_one(buffers: &mut HashMap<String, Batch>, op_id: &str, sink: &Arc<dyn EventSink>) {
    if let Some(batch) = buffers.remove(op_id) {
        if !batch.lines.is_empty() {
            sink.emit(AppEvent::OpOutput(OpOutputEvent {
                op_id: op_id.to_string(),
                batch: batch.lines,
            }));
        }
    }
}

async fn batch_task(mut rx: mpsc::UnboundedReceiver<BatchMsg>, sink: Arc<dyn EventSink>) {
    let mut buffers: HashMap<String, Batch> = HashMap::new();
    loop {
        let next_deadline = buffers.values().map(|b| b.deadline).min();
        tokio::select! {
            msg = rx.recv() => match msg {
                Some(BatchMsg::Line { op_id, line }) => {
                    let batch = buffers.entry(op_id.clone()).or_insert_with(|| Batch {
                        lines: Vec::new(),
                        bytes: 0,
                        deadline: Instant::now() + BATCH_MAX_DELAY,
                    });
                    batch.bytes += line.line.len();
                    batch.lines.push(line);
                    if batch.lines.len() >= BATCH_MAX_LINES || batch.bytes >= BATCH_MAX_BYTES {
                        flush_one(&mut buffers, &op_id, &sink);
                    }
                }
                Some(BatchMsg::FlushOp { op_id }) => flush_one(&mut buffers, &op_id, &sink),
                None => {
                    // Emitter dropped: flush everything and end.
                    let ids: Vec<String> = buffers.keys().cloned().collect();
                    for id in ids {
                        flush_one(&mut buffers, &id, &sink);
                    }
                    return;
                }
            },
            _ = tokio::time::sleep_until(
                next_deadline.unwrap_or_else(|| Instant::now() + Duration::from_secs(3600))
            ), if next_deadline.is_some() => {
                let now = Instant::now();
                let due: Vec<String> = buffers
                    .iter()
                    .filter(|(_, b)| b.deadline <= now)
                    .map(|(id, _)| id.clone())
                    .collect();
                for id in due {
                    flush_one(&mut buffers, &id, &sink);
                }
            }
        }
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

    // ---------------- batching emitter (paused time, zero sleeps) ----------

    use crate::ipc::StreamKind;

    fn line(text: &str) -> LogLine {
        LogLine {
            stream: StreamKind::Out,
            line: text.to_string(),
            ts_ms: 0,
        }
    }

    fn output_batches(events: &[AppEvent]) -> Vec<(String, usize)> {
        events
            .iter()
            .filter_map(|e| match e {
                AppEvent::OpOutput(o) => Some((o.op_id.clone(), o.batch.len())),
                _ => None,
            })
            .collect()
    }

    // SPEC §7.3 names this `500_lines_flush_in_le_64_line_batches_at_ge_50ms`;
    // Rust identifiers cannot start with a digit, hence the `batch_` prefix.
    #[tokio::test(start_paused = true)]
    async fn batch_500_lines_flush_in_le_64_line_batches_at_ge_50ms() {
        let sink = Arc::new(VecSink::new());
        let emitter = BatchingEmitter::new(sink.clone());
        for i in 0..500 {
            emitter.push("op-1", line(&format!("line {i}")));
        }

        // Let the task drain without reaching the 50ms timer: seven full
        // 64-line batches flush on the line cap alone.
        tokio::time::sleep(Duration::from_millis(1)).await;
        let batches = output_batches(&sink.events());
        assert_eq!(batches.len(), 7);
        assert!(batches.iter().all(|(_, n)| *n == 64));

        // The 52-line remainder flushes only once the 50ms timer fires.
        tokio::time::sleep(Duration::from_millis(60)).await;
        let batches = output_batches(&sink.events());
        assert_eq!(batches.len(), 8);
        assert!(batches.iter().all(|(_, n)| *n <= 64));
        assert_eq!(batches.iter().map(|(_, n)| n).sum::<usize>(), 500);
        assert_eq!(batches[7].1, 52);
    }

    #[tokio::test(start_paused = true)]
    async fn batch_force_flushes_at_8kib() {
        let sink = Arc::new(VecSink::new());
        let emitter = BatchingEmitter::new(sink.clone());
        let kib = "x".repeat(1024);
        for _ in 0..8 {
            emitter.push("op-1", line(&kib));
        }
        tokio::time::sleep(Duration::from_millis(1)).await;
        let batches = output_batches(&sink.events());
        assert_eq!(
            batches,
            vec![("op-1".to_string(), 8)],
            "flushed before the 50ms timer"
        );
    }

    #[tokio::test(start_paused = true)]
    async fn batch_keeps_ops_separate_and_timer_flushes_each() {
        let sink = Arc::new(VecSink::new());
        let emitter = BatchingEmitter::new(sink.clone());
        emitter.push("op-a", line("a1"));
        emitter.push("op-b", line("b1"));
        emitter.push("op-a", line("a2"));
        tokio::time::sleep(Duration::from_millis(60)).await;
        let mut batches = output_batches(&sink.events());
        batches.sort();
        assert_eq!(
            batches,
            vec![("op-a".to_string(), 2), ("op-b".to_string(), 1)]
        );
    }

    #[tokio::test(start_paused = true)]
    async fn batch_flush_op_flushes_immediately() {
        let sink = Arc::new(VecSink::new());
        let emitter = BatchingEmitter::new(sink.clone());
        emitter.push("op-1", line("final line"));
        emitter.flush_op("op-1");
        tokio::time::sleep(Duration::from_millis(1)).await;
        assert_eq!(
            output_batches(&sink.events()),
            vec![("op-1".to_string(), 1)]
        );
    }

    #[tokio::test(start_paused = true)]
    async fn batch_drop_flushes_remaining_and_line_sink_feeds_batches() {
        let sink = Arc::new(VecSink::new());
        let emitter = BatchingEmitter::new(sink.clone());
        let line_sink = emitter.line_sink("op-1");
        line_sink(line("via sink"));
        // The channel closes once the emitter AND every line_sink are gone.
        drop(line_sink);
        drop(emitter);
        tokio::time::sleep(Duration::from_millis(1)).await;
        assert_eq!(
            output_batches(&sink.events()),
            vec![("op-1".to_string(), 1)]
        );
    }
}
