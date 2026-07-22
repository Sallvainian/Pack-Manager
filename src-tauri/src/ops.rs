//! Operation model + transcript writer (SPEC §5.7 model, §6.2 format) —
//! implemented by U5.
//!
//! Transcripts are written incrementally from spawn and line-flushed, so they
//! survive crashes. Write errors after creation are swallowed (a full disk
//! drops transcript lines, never hangs an upgrade — SPEC §6.3 hygiene).

use std::collections::BTreeSet;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::time::Duration;

use time::OffsetDateTime;
use tokio_util::sync::CancellationToken;
use uuid::Uuid;

use crate::error::PmError;
use crate::ipc::{self, LogLine, ManagerId, OpStatus, StreamKind};
use crate::process::CommandSpec;

// ---------------------------------------------------------------------------
// Operation model (SPEC §5.7)
// ---------------------------------------------------------------------------

/// Data-carrying op kind; maps onto the wire `ipc::OpKind` via [`OpKind::wire`].
#[derive(Debug, Clone, PartialEq)]
pub enum OpKind {
    Refresh,
    Upgrade { package_ids: Vec<String> },
    SelfUpdate,
    HealthFix { issue_id: String },
}

impl OpKind {
    pub fn wire(&self) -> ipc::OpKind {
        match self {
            OpKind::Refresh => ipc::OpKind::Refresh,
            OpKind::Upgrade { .. } => ipc::OpKind::Upgrade,
            OpKind::SelfUpdate => ipc::OpKind::SelfUpdate,
            OpKind::HealthFix { .. } => ipc::OpKind::HealthFix,
        }
    }

    pub fn package_ids(&self) -> Vec<String> {
        match self {
            OpKind::Upgrade { package_ids } => package_ids.clone(),
            _ => Vec::new(),
        }
    }
}

/// One queued unit of work (SPEC §5.7). Ids are UUIDv7 (time-sortable).
pub struct Operation {
    pub id: Uuid,
    pub kind: OpKind,
    pub executor: ManagerId,
    pub subject: ManagerId,
    pub locks: BTreeSet<ManagerId>,
    pub specs: Vec<CommandSpec>,
    pub cancel: CancellationToken,
    pub log_path: PathBuf,
}

/// Wire (camelCase) name of an op kind — used in transcript headers and
/// transcript file names.
pub fn wire_kind_str(kind: ipc::OpKind) -> &'static str {
    match kind {
        ipc::OpKind::Refresh => "refresh",
        ipc::OpKind::Upgrade => "upgrade",
        ipc::OpKind::SelfUpdate => "selfUpdate",
        ipc::OpKind::HealthFix => "healthFix",
    }
}

/// Footer status string (SPEC §6.2: cancelled/timed-out footers record the
/// signal path).
pub fn footer_status_str(status: OpStatus) -> &'static str {
    match status {
        OpStatus::Succeeded => "succeeded",
        OpStatus::Failed => "failed",
        OpStatus::Cancelled => "cancelled (SIGTERM→exit)",
        OpStatus::TimedOut => "timed_out (SIGKILL after 5s grace)",
        OpStatus::Interrupted => "interrupted",
        OpStatus::Queued => "queued",
        OpStatus::Running => "running",
    }
}

/// `program arg1 arg2 …` for one spec (absolute program path — the transcript
/// header is the ground truth of what was spawned).
pub fn spec_line(spec: &CommandSpec) -> String {
    let mut s = spec.program.display().to_string();
    for a in &spec.args {
        s.push(' ');
        s.push_str(a);
    }
    s
}

/// The op's command line: serial specs joined with ` && `.
pub fn command_line_of(specs: &[CommandSpec]) -> String {
    specs.iter().map(spec_line).collect::<Vec<_>>().join(" && ")
}

/// Transcript file name (SPEC §6.1):
/// `<YYYY-MM-DDTHH-mm-ss>_<opId8>_<manager>_<kind>.log`
pub fn transcript_file_name(
    at: OffsetDateTime,
    op_id: &str,
    subject: ManagerId,
    kind: ipc::OpKind,
) -> String {
    let id8 = op_id.get(..8).unwrap_or(op_id);
    format!(
        "{:04}-{:02}-{:02}T{:02}-{:02}-{:02}_{}_{}_{}.log",
        at.year(),
        u8::from(at.month()),
        at.day(),
        at.hour(),
        at.minute(),
        at.second(),
        id8,
        subject.as_str(),
        wire_kind_str(kind),
    )
}

// ---------------------------------------------------------------------------
// Transcript formatting (pure — golden-format tested)
// ---------------------------------------------------------------------------

/// Everything the header needs; assembled by the queue at op start.
#[derive(Debug, Clone)]
pub struct TranscriptHeader {
    pub op_id: String,
    pub kind: ipc::OpKind,
    pub executor: ManagerId,
    pub subject: ManagerId,
    pub queued_at: String,
    pub started_at: String,
    pub command_line: String,
    pub cwd: String,
    pub path: String,
    /// Pre-joined `K=V K=V …` of the constructed env (PATH excluded — it has
    /// its own line). Only env we set is ever logged.
    pub env_set: String,
    /// e.g. `stall 120s / hard cap 1800s` or `absolute 600s`.
    pub timeout: String,
    pub pgid: i32,
}

/// `K=V` pairs joined by single spaces, PATH excluded.
pub fn env_set_string(env: &[(String, String)]) -> String {
    env.iter()
        .filter(|(k, _)| k != "PATH")
        .map(|(k, v)| format!("{k}={v}"))
        .collect::<Vec<_>>()
        .join(" ")
}

/// Human form of a [`crate::managers::Timeout`].
pub fn timeout_string(timeout: &crate::managers::Timeout) -> String {
    match timeout {
        crate::managers::Timeout::Absolute(d) => format!("absolute {}s", d.as_secs()),
        crate::managers::Timeout::Stall { silence, hard_cap } => format!(
            "stall {}s / hard cap {}s",
            silence.as_secs(),
            hard_cap.as_secs()
        ),
    }
}

pub fn format_header(h: &TranscriptHeader) -> String {
    format!(
        "=== Pack-Manager operation ===\n\
         op_id:      {}   kind: {}   executor: {}   subject: {}\n\
         queued_at:  {}   started_at: {}\n\
         command:    {}\n\
         cwd:        {}\n\
         PATH:       {}\n\
         env_set:    {}\n\
         timeout:    {}   pgid: {}\n\
         === output ===\n",
        h.op_id,
        wire_kind_str(h.kind),
        h.executor,
        h.subject,
        h.queued_at,
        h.started_at,
        h.command_line,
        h.cwd,
        h.path,
        h.env_set,
        h.timeout,
        h.pgid,
    )
}

/// `HH:MM:SS.mmm` (UTC) from unix milliseconds.
pub fn format_ts_ms(ts_ms: u64) -> String {
    let secs = (ts_ms / 1000) as i64;
    let millis = (ts_ms % 1000) as u16;
    match OffsetDateTime::from_unix_timestamp(secs) {
        Ok(t) => format!(
            "{:02}:{:02}:{:02}.{:03}",
            t.hour(),
            t.minute(),
            t.second(),
            millis
        ),
        Err(_) => format!("00:00:00.{millis:03}"),
    }
}

pub fn format_line(line: &LogLine) -> String {
    let stream = match line.stream {
        StreamKind::Out => "out",
        StreamKind::Err => "err",
    };
    format!("{} [{}] {}\n", format_ts_ms(line.ts_ms), stream, line.line)
}

/// Marker between the serial specs of a multi-command op (smallest-behavior
/// addition — the SPEC §6.2 sample shows a single-command transcript).
pub fn format_marker(command: &str) -> String {
    format!("--- command: {command} ---\n")
}

pub fn format_footer(
    status: OpStatus,
    exit_code: Option<i32>,
    duration: Duration,
    finished_at: &str,
) -> String {
    let exit = exit_code
        .map(|c| c.to_string())
        .unwrap_or_else(|| "-".to_string());
    format!(
        "=== result ===\nstatus: {}   exit_code: {}   duration: {:.1}s   finished_at: {}\n",
        footer_status_str(status),
        exit,
        duration.as_secs_f64(),
        finished_at,
    )
}

// ---------------------------------------------------------------------------
// Transcript writer (line-flushed)
// ---------------------------------------------------------------------------

pub struct Transcript {
    path: PathBuf,
    file: std::fs::File,
}

impl Transcript {
    /// Creates the transcript file (and parent dirs).
    pub fn create(path: &Path) -> Result<Transcript, PmError> {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let file = std::fs::File::create(path)?;
        Ok(Transcript {
            path: path.to_path_buf(),
            file,
        })
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    /// Write + flush; errors are dropped (never hang or fail an op over logs).
    fn write_str(&mut self, s: &str) {
        let _ = self.file.write_all(s.as_bytes());
        let _ = self.file.flush();
    }

    pub fn header(&mut self, h: &TranscriptHeader) {
        self.write_str(&format_header(h));
    }

    pub fn line(&mut self, line: &LogLine) {
        self.write_str(&format_line(line));
    }

    pub fn marker(&mut self, command: &str) {
        self.write_str(&format_marker(command));
    }

    pub fn footer(
        &mut self,
        status: OpStatus,
        exit_code: Option<i32>,
        duration: Duration,
        finished_at: &str,
    ) {
        self.write_str(&format_footer(status, exit_code, duration, finished_at));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;
    use time::macros::datetime;

    fn npm_spec() -> CommandSpec {
        CommandSpec {
            program: PathBuf::from("/Users/sallvain/.local/share/mise/shims/npm"),
            args: vec!["install".into(), "-g".into(), "typescript@latest".into()],
            env: vec![
                (
                    "PATH".into(),
                    "/Users/sallvain/.local/share/mise/shims:/opt/homebrew/bin".into(),
                ),
                ("HOME".into(), "/Users/sallvain".into()),
                ("LANG".into(), "en_US.UTF-8".into()),
                ("NO_COLOR".into(), "1".into()),
                ("TERM".into(), "dumb".into()),
            ],
            timeout: crate::managers::Timeout::Stall {
                silence: Duration::from_secs(120),
                hard_cap: Duration::from_secs(1800),
            },
            purpose: crate::process::CmdPurpose::Upgrade,
        }
    }

    /// SPEC §6.2 golden-format test: the transcript byte layout is pinned.
    #[test]
    fn transcript_golden_format_matches_spec_6_2() {
        let spec = npm_spec();
        let header = TranscriptHeader {
            op_id: "01981f2e-6a3b-7c40-9d5e-1f2a3b4c5d6e".into(),
            kind: ipc::OpKind::Upgrade,
            executor: ManagerId::Npm,
            subject: ManagerId::Npm,
            queued_at: "2026-07-22T14:03:11Z".into(),
            started_at: "2026-07-22T14:03:11Z".into(),
            command_line: command_line_of(std::slice::from_ref(&spec)),
            cwd: "/Users/sallvain".into(),
            path: "/Users/sallvain/.local/share/mise/shims:/opt/homebrew/bin".into(),
            env_set: env_set_string(&spec.env),
            timeout: timeout_string(&spec.timeout),
            pgid: 0,
        };
        // 2026-07-22T14:03:12.104Z / .371Z in unix ms.
        let base_ms = datetime!(2026-07-22 14:03:12 UTC).unix_timestamp() as u64 * 1000;
        let mut text = format_header(&header);
        text.push_str(&format_line(&LogLine {
            stream: StreamKind::Out,
            line: "added 1 package in 4s".into(),
            ts_ms: base_ms + 104,
        }));
        text.push_str(&format_line(&LogLine {
            stream: StreamKind::Err,
            line: "npm warn deprecated inflight@1.0.6".into(),
            ts_ms: base_ms + 371,
        }));
        text.push_str(&format_footer(
            OpStatus::Succeeded,
            Some(0),
            Duration::from_millis(5200),
            "2026-07-22T14:03:16Z",
        ));

        let expected = "\
=== Pack-Manager operation ===
op_id:      01981f2e-6a3b-7c40-9d5e-1f2a3b4c5d6e   kind: upgrade   executor: npm   subject: npm
queued_at:  2026-07-22T14:03:11Z   started_at: 2026-07-22T14:03:11Z
command:    /Users/sallvain/.local/share/mise/shims/npm install -g typescript@latest
cwd:        /Users/sallvain
PATH:       /Users/sallvain/.local/share/mise/shims:/opt/homebrew/bin
env_set:    HOME=/Users/sallvain LANG=en_US.UTF-8 NO_COLOR=1 TERM=dumb
timeout:    stall 120s / hard cap 1800s   pgid: 0
=== output ===
14:03:12.104 [out] added 1 package in 4s
14:03:12.371 [err] npm warn deprecated inflight@1.0.6
=== result ===
status: succeeded   exit_code: 0   duration: 5.2s   finished_at: 2026-07-22T14:03:16Z
";
        assert_eq!(text, expected);
    }

    #[test]
    fn footer_records_signal_paths_for_cancel_and_timeout() {
        let cancelled = format_footer(
            OpStatus::Cancelled,
            None,
            Duration::from_secs(3),
            "2026-07-22T14:03:16Z",
        );
        assert!(cancelled.contains("status: cancelled (SIGTERM→exit)"));
        assert!(cancelled.contains("exit_code: -"));

        let timed_out = format_footer(
            OpStatus::TimedOut,
            None,
            Duration::from_secs(1800),
            "2026-07-22T14:33:11Z",
        );
        assert!(timed_out.contains("status: timed_out (SIGKILL after 5s grace)"));
    }

    #[test]
    fn transcript_file_name_matches_the_spec_pattern() {
        let at = datetime!(2026-07-22 14:03:11 UTC);
        assert_eq!(
            transcript_file_name(
                at,
                "01981f2e-6a3b-7c40-9d5e-1f2a3b4c5d6e",
                ManagerId::Npm,
                ipc::OpKind::Upgrade
            ),
            "2026-07-22T14-03-11_01981f2e_npm_upgrade.log"
        );
        assert_eq!(
            transcript_file_name(at, "short", ManagerId::Mise, ipc::OpKind::SelfUpdate),
            "2026-07-22T14-03-11_short_mise_selfUpdate.log"
        );
    }

    #[test]
    fn command_line_joins_serial_specs() {
        let one = npm_spec();
        let mut two = npm_spec();
        two.args = vec!["outdated".into(), "-g".into()];
        assert_eq!(
            command_line_of(&[one.clone(), two]),
            "/Users/sallvain/.local/share/mise/shims/npm install -g typescript@latest && \
             /Users/sallvain/.local/share/mise/shims/npm outdated -g"
        );
        assert_eq!(
            command_line_of(&[one]),
            "/Users/sallvain/.local/share/mise/shims/npm install -g typescript@latest"
        );
    }

    #[test]
    fn transcript_writer_is_line_flushed_on_disk() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("ops").join("t.log");
        let mut t = Transcript::create(&path).unwrap();
        t.line(&LogLine {
            stream: StreamKind::Out,
            line: "first".into(),
            ts_ms: 0,
        });
        // Visible on disk BEFORE the writer is dropped (line-flushed).
        let body = std::fs::read_to_string(&path).unwrap();
        assert!(body.contains("[out] first"));
    }

    #[test]
    fn op_kind_maps_to_wire_and_carries_package_ids() {
        let up = OpKind::Upgrade {
            package_ids: vec!["formula:dolt".into()],
        };
        assert_eq!(up.wire(), ipc::OpKind::Upgrade);
        assert_eq!(up.package_ids(), vec!["formula:dolt".to_string()]);
        assert_eq!(OpKind::Refresh.wire(), ipc::OpKind::Refresh);
        assert!(OpKind::SelfUpdate.package_ids().is_empty());
        assert_eq!(
            OpKind::HealthFix {
                issue_id: "uv:aider-chat".into()
            }
            .wire(),
            ipc::OpKind::HealthFix
        );
    }

    #[test]
    fn env_set_excludes_path() {
        let env = vec![
            ("PATH".to_string(), "/a:/b".to_string()),
            ("NO_COLOR".to_string(), "1".to_string()),
        ];
        assert_eq!(env_set_string(&env), "NO_COLOR=1");
    }
}
