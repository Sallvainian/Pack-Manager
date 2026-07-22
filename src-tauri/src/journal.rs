//! Crash-safe journal `operations.jsonl` (SPEC §5.7, F8, DECISIONS D12) —
//! implemented by U5.
//!
//! One line at op start, one at finish, flushed each write. Start-without-
//! finish renders `Interrupted` on the next launch. Recorded pgids are NEVER
//! signaled on startup (pid reuse). Compacted to the newest 1000 operations at
//! startup. This one file is both the crash journal and the History source.

use std::collections::HashMap;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::sync::Mutex;

use serde::{Deserialize, Serialize};

use crate::ipc::{ManagerId, OpKind, OpStatus, OperationRecord};

/// Operations kept by startup compaction.
pub const COMPACT_KEEP: usize = 1000;

/// Journal start line (SPEC §5.7). `logPath` is an addition beyond the SPEC's
/// listed fields so interrupted ops keep a working "Reveal in Finder"
/// (`OperationRecord.logPath` is non-optional wire-side).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StartRecord {
    pub op_id: String,
    pub kind: OpKind,
    pub executor: ManagerId,
    pub subject: ManagerId,
    pub command_line: String,
    /// Informational only — never signaled on startup (pid reuse). `0` when
    /// the runner seam does not expose the child's pgid.
    pub pgid: i32,
    pub started_at: String,
    #[serde(default)]
    pub log_path: String,
}

/// Journal finish line (SPEC §5.7).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FinishRecord {
    pub op_id: String,
    pub outcome: OpStatus,
    pub exit_code: Option<i32>,
    pub finished_at: String,
}

#[derive(Deserialize)]
#[serde(untagged)]
enum Line {
    Start(StartRecord),
    Finish(FinishRecord),
}

/// Append-only, flushed-per-write journal handle.
pub struct Journal {
    path: PathBuf,
    file: Mutex<Option<std::fs::File>>,
}

impl Journal {
    pub fn new(path: PathBuf) -> Journal {
        Journal {
            path,
            file: Mutex::new(None),
        }
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    /// Serializes one record as a JSON line, appends, flushes. IO errors are
    /// logged and dropped — journal failures never fail an operation.
    fn append<T: Serialize>(&self, record: &T) {
        let mut guard = self.file.lock().expect("journal poisoned");
        if guard.is_none() {
            if let Some(parent) = self.path.parent() {
                let _ = std::fs::create_dir_all(parent);
            }
            match std::fs::OpenOptions::new()
                .create(true)
                .append(true)
                .open(&self.path)
            {
                Ok(f) => *guard = Some(f),
                Err(e) => {
                    tracing::error!(path = %self.path.display(), error = %e, "journal open failed");
                    return;
                }
            }
        }
        if let Some(file) = guard.as_mut() {
            match serde_json::to_string(record) {
                Ok(mut line) => {
                    line.push('\n');
                    if let Err(e) = file.write_all(line.as_bytes()).and_then(|_| file.flush()) {
                        tracing::error!(error = %e, "journal write failed");
                    }
                }
                Err(e) => tracing::error!(error = %e, "journal serialize failed"),
            }
        }
    }

    pub fn record_start(&self, record: &StartRecord) {
        self.append(record);
    }

    pub fn record_finish(&self, record: &FinishRecord) {
        self.append(record);
    }
}

/// Parses the journal leniently (garbage lines skipped, finish-without-start
/// ignored). Returns `(start, finish)` pairs in file order.
pub fn load_entries(path: &Path) -> Vec<(StartRecord, Option<FinishRecord>)> {
    let Ok(body) = std::fs::read_to_string(path) else {
        return Vec::new();
    };
    let mut entries: Vec<(StartRecord, Option<FinishRecord>)> = Vec::new();
    let mut index: HashMap<String, usize> = HashMap::new();
    for line in body.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        match serde_json::from_str::<Line>(line) {
            Ok(Line::Start(s)) => {
                index.insert(s.op_id.clone(), entries.len());
                entries.push((s, None));
            }
            Ok(Line::Finish(f)) => {
                if let Some(&i) = index.get(&f.op_id) {
                    entries[i].1 = Some(f);
                }
            }
            Err(_) => { /* lenient: skip garbage lines */ }
        }
    }
    entries
}

/// Journal entries as wire `OperationRecord`s: finish outcome when present,
/// otherwise `Interrupted` (start-without-finish — SPEC F8).
pub fn load_records(path: &Path) -> Vec<OperationRecord> {
    load_entries(path)
        .into_iter()
        .map(|(start, finish)| {
            let (status, exit_code, finished_at) = match finish {
                Some(f) => (f.outcome, f.exit_code, Some(f.finished_at)),
                None => (OpStatus::Interrupted, None, None),
            };
            OperationRecord {
                op_id: start.op_id,
                kind: start.kind,
                executor: start.executor,
                subject: start.subject,
                status,
                command_line: start.command_line,
                package_ids: Vec::new(),
                queued_at: start.started_at.clone(),
                started_at: Some(start.started_at),
                finished_at,
                exit_code,
                error: None,
                log_path: start.log_path,
            }
        })
        .collect()
}

/// Startup compaction: keep the newest `keep` operations (file order = age;
/// UUIDv7 op ids are time-sortable but appearance order is authoritative) and
/// rewrite the file as start line + finish line per kept op.
///
/// The rewrite is ATOMIC: content goes to a sibling temp file (fsynced), then
/// `rename` replaces `operations.jsonl` in one step. This file is the crash
/// journal itself — a truncate-in-place rewrite interrupted by power loss /
/// kill / ENOSPC would destroy the entire history INCLUDING the interrupted
/// records the file exists to surface. A crash mid-compaction now leaves the
/// original journal untouched.
pub fn compact(path: &Path, keep: usize) -> std::io::Result<()> {
    if !path.exists() {
        return Ok(());
    }
    let entries = load_entries(path);
    if entries.len() <= keep {
        return Ok(());
    }
    let skip = entries.len() - keep;
    let mut out = String::new();
    for (start, finish) in entries.into_iter().skip(skip) {
        if let Ok(line) = serde_json::to_string(&start) {
            out.push_str(&line);
            out.push('\n');
        }
        if let Some(f) = finish {
            if let Ok(line) = serde_json::to_string(&f) {
                out.push_str(&line);
                out.push('\n');
            }
        }
    }
    write_atomic(path, &out)
}

/// Write-to-temp + fsync + rename in the target's own directory (rename is
/// atomic on the same filesystem). On failure the temp file is best-effort
/// removed and the original is left untouched.
fn write_atomic(path: &Path, content: &str) -> std::io::Result<()> {
    let file_name = path
        .file_name()
        .map(|n| n.to_string_lossy().into_owned())
        .unwrap_or_else(|| "journal".to_string());
    let tmp = path.with_file_name(format!("{file_name}.tmp"));
    let result = (|| {
        let mut f = std::fs::File::create(&tmp)?;
        f.write_all(content.as_bytes())?;
        f.sync_all()?;
        std::fs::rename(&tmp, path)
    })();
    if result.is_err() {
        let _ = std::fs::remove_file(&tmp);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    fn start(op_id: &str, at: &str) -> StartRecord {
        StartRecord {
            op_id: op_id.into(),
            kind: OpKind::Upgrade,
            executor: ManagerId::Npm,
            subject: ManagerId::Npm,
            command_line: "/x/npm install -g typescript@latest".into(),
            pgid: 0,
            started_at: at.into(),
            log_path: format!("/logs/{op_id}.log"),
        }
    }

    fn finish(op_id: &str, outcome: OpStatus, at: &str) -> FinishRecord {
        FinishRecord {
            op_id: op_id.into(),
            outcome,
            exit_code: Some(0),
            finished_at: at.into(),
        }
    }

    #[test]
    fn journal_start_finish_roundtrip() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("nested").join("operations.jsonl");
        let journal = Journal::new(path.clone());
        journal.record_start(&start("op-1", "2026-07-22T14:03:11Z"));
        journal.record_finish(&finish("op-1", OpStatus::Succeeded, "2026-07-22T14:03:16Z"));

        let records = load_records(&path);
        assert_eq!(records.len(), 1);
        let r = &records[0];
        assert_eq!(r.op_id, "op-1");
        assert_eq!(r.status, OpStatus::Succeeded);
        assert_eq!(r.kind, OpKind::Upgrade);
        assert_eq!(r.executor, ManagerId::Npm);
        assert_eq!(r.command_line, "/x/npm install -g typescript@latest");
        assert_eq!(r.started_at.as_deref(), Some("2026-07-22T14:03:11Z"));
        assert_eq!(r.finished_at.as_deref(), Some("2026-07-22T14:03:16Z"));
        assert_eq!(r.exit_code, Some(0));
        assert_eq!(r.log_path, "/logs/op-1.log");
    }

    #[test]
    fn start_only_record_surfaces_interrupted() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("operations.jsonl");
        let journal = Journal::new(path.clone());
        journal.record_start(&start("op-crash", "2026-07-22T14:03:11Z"));

        let records = load_records(&path);
        assert_eq!(records.len(), 1);
        assert_eq!(records[0].status, OpStatus::Interrupted);
        assert_eq!(records[0].finished_at, None);
        assert_eq!(records[0].exit_code, None);
    }

    #[test]
    fn journal_lines_use_camel_case_wire_fields() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("operations.jsonl");
        let journal = Journal::new(path.clone());
        journal.record_start(&start("op-1", "2026-07-22T14:03:11Z"));
        journal.record_finish(&finish("op-1", OpStatus::TimedOut, "2026-07-22T14:33:11Z"));

        let body = std::fs::read_to_string(&path).unwrap();
        let lines: Vec<&str> = body.lines().collect();
        assert_eq!(lines.len(), 2);
        assert!(lines[0].contains("\"opId\":\"op-1\""));
        assert!(lines[0].contains("\"commandLine\""));
        assert!(lines[0].contains("\"pgid\":0"));
        assert!(lines[1].contains("\"outcome\":\"timedOut\""));
    }

    #[test]
    fn garbage_lines_and_orphan_finishes_are_skipped() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("operations.jsonl");
        let mut body = String::new();
        body.push_str("not json at all\n");
        body.push_str(&serde_json::to_string(&start("op-1", "2026-07-22T14:00:00Z")).unwrap());
        body.push('\n');
        // Finish for an unknown op — ignored.
        body.push_str(
            &serde_json::to_string(&finish("op-zzz", OpStatus::Failed, "2026-07-22T14:01:00Z"))
                .unwrap(),
        );
        body.push('\n');
        std::fs::write(&path, body).unwrap();

        let records = load_records(&path);
        assert_eq!(records.len(), 1);
        assert_eq!(records[0].op_id, "op-1");
        assert_eq!(records[0].status, OpStatus::Interrupted);
    }

    #[test]
    fn compact_keeps_newest_operations_with_their_finishes() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("operations.jsonl");
        let journal = Journal::new(path.clone());
        for i in 0..1005 {
            let id = format!("op-{i:04}");
            journal.record_start(&start(&id, "2026-07-22T14:00:00Z"));
            if i % 2 == 0 {
                journal.record_finish(&finish(&id, OpStatus::Succeeded, "2026-07-22T14:00:05Z"));
            }
        }
        compact(&path, COMPACT_KEEP).unwrap();

        let entries = load_entries(&path);
        assert_eq!(entries.len(), COMPACT_KEEP);
        assert_eq!(entries[0].0.op_id, "op-0005", "oldest five dropped");
        assert_eq!(entries.last().unwrap().0.op_id, "op-1004");
        // Finishes stay attached through compaction.
        let (s, f) = &entries[1]; // op-0006, even → finished
        assert_eq!(s.op_id, "op-0006");
        assert!(f.is_some());
    }

    /// Regression: compaction must never modify the journal in place. When
    /// the rewrite cannot complete (here: the directory refuses new files, a
    /// stand-in for ENOSPC / crash mid-write), the ORIGINAL journal — with
    /// its interrupted records — must survive byte-identically. The old
    /// truncate-in-place `std::fs::write` would have rewritten (or truncated)
    /// it despite the failure injection.
    #[test]
    fn compact_failure_leaves_the_original_journal_intact() {
        use std::os::unix::fs::PermissionsExt;
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("operations.jsonl");
        let journal = Journal::new(path.clone());
        for i in 0..1005 {
            journal.record_start(&start(&format!("op-{i:04}"), "2026-07-22T14:00:00Z"));
        }
        drop(journal);
        let before = std::fs::read_to_string(&path).unwrap();

        // Read-only dir: the sibling temp file cannot be created, so the
        // atomic rewrite must fail WITHOUT touching operations.jsonl.
        std::fs::set_permissions(dir.path(), std::fs::Permissions::from_mode(0o555)).unwrap();
        let result = compact(&path, COMPACT_KEEP);
        std::fs::set_permissions(dir.path(), std::fs::Permissions::from_mode(0o755)).unwrap();

        assert!(result.is_err(), "rewrite must fail, not truncate in place");
        assert_eq!(
            std::fs::read_to_string(&path).unwrap(),
            before,
            "original journal survives a failed compaction byte-identically"
        );
        // Interrupted records are still all recoverable.
        assert_eq!(load_records(&path).len(), 1005);
    }

    #[test]
    fn compact_leaves_no_temp_file_behind() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("operations.jsonl");
        let journal = Journal::new(path.clone());
        for i in 0..1005 {
            journal.record_start(&start(&format!("op-{i:04}"), "2026-07-22T14:00:00Z"));
        }
        compact(&path, COMPACT_KEEP).unwrap();
        assert_eq!(load_entries(&path).len(), COMPACT_KEEP);
        assert!(
            !dir.path().join("operations.jsonl.tmp").exists(),
            "temp file renamed away"
        );
    }

    #[test]
    fn compact_is_a_noop_under_the_cap_and_for_missing_files() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("operations.jsonl");
        compact(&path, COMPACT_KEEP).unwrap(); // missing → Ok

        let journal = Journal::new(path.clone());
        journal.record_start(&start("op-1", "2026-07-22T14:00:00Z"));
        let before = std::fs::read_to_string(&path).unwrap();
        compact(&path, COMPACT_KEEP).unwrap();
        assert_eq!(std::fs::read_to_string(&path).unwrap(), before);
    }
}
