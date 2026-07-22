//! `CommandRunner` — the single seam for every child process (SPEC §5.6, §7).
//!
//! `RealRunner`: `tokio::process::Command` with `.process_group(0)`,
//! `.stdin(Stdio::null())`, line readers with `\r` split, ANSI stripping and
//! an unterminated-notice split (D26), 512KiB caps, stall watchdog, absolute
//! timeout, and SIGTERM → 5s grace → SIGKILL via `nix::killpg`.

use std::process::Stdio;
use std::sync::{Arc, OnceLock};
use std::time::Duration;

use async_trait::async_trait;
use nix::sys::signal::{killpg, Signal};
use nix::unistd::Pid;
use regex::Regex;
use tokio::io::{AsyncBufReadExt, AsyncRead, BufReader};
use tokio::sync::mpsc;
use tokio::time::Instant;
use tokio_util::sync::CancellationToken;

use crate::error::PmError;
use crate::ipc::StreamKind;
use crate::process::{CommandOutput, CommandSpec, LineSink, LogLine, Timeout};

#[async_trait]
pub trait CommandRunner: Send + Sync {
    /// Buffered execution (refresh/detection). `cancel` triggers the same
    /// SIGTERM → 5s grace → SIGKILL escalation as streaming execution —
    /// refreshes are cancellable ops too (SPEC F7).
    async fn run(
        &self,
        spec: &CommandSpec,
        cancel: CancellationToken,
    ) -> Result<CommandOutput, PmError>;

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

/// Callback invoked when the stall watchdog fires (argument = seconds of
/// silence so far). The op continues; only the hard cap kills. Attribution to
/// an opId is the caller's job (construct one runner per operation, or wrap).
pub type StallNotify = Arc<dyn Fn(u64) + Send + Sync>;

/// Per-stream retention cap for [`CommandOutput`] (transcripts keep the full
/// output; this is the in-memory copy handed to parsers).
pub const STREAM_CAP_BYTES: usize = 512 * 1024;

/// Grace between SIGTERM and SIGKILL (SPEC F7).
pub const TERM_GRACE: Duration = Duration::from_secs(5);

/// After the direct child exits, how long to keep waiting for pipe EOF before
/// completing anyway. Descendants inherit the stdout/stderr write ends
/// (`.process_group(0)` isolates signals, not fds), so a lingering helper can
/// hold the pipes open long after the command itself finished — the op must
/// complete on the child's exit, not the helper's.
pub const POST_EXIT_EOF_GRACE: Duration = Duration::from_secs(2);

fn ansi_re() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| {
        // CSI sequences, OSC sequences, then two-byte ESC codes.
        Regex::new(r"\x1b(?:\[[0-9;?]*[ -/]*[@-~]|\][^\x07\x1b]*(?:\x07|\x1b\\)?|[@-Z\\-_])")
            .expect("static ANSI regex compiles")
    })
}

/// Strips ANSI escape sequences (colors, cursor movement, OSC titles).
pub fn strip_ansi(s: &str) -> String {
    ansi_re().replace_all(s, "").into_owned()
}

/// Notices that a producer prints with NO line terminator at all — not `\n`,
/// not `\r` — so the next line of real output concatenates onto them and both
/// arrive in a single `read_until(b'\n')` buffer.
///
/// Splitting after one of these is the ONLY place Pack-Manager inserts a break
/// the child never printed, so the list stays closed and literal: each entry is
/// a verbatim string from a captured transcript, never a pattern. See D26.
///
/// - `Update progress cannot be displayed` — `mas` 7.0.0 emits this per app
///   during `mas upgrade` when stdout is not a TTY and it cannot draw its
///   progress bar. Captured at `operations/…_mas_upgrade.log`, which shows
///   `displayed==> U` with no byte between.
const UNTERMINATED_NOTICES: &[&str] = &["Update progress cannot be displayed"];

/// Breaks a piece after any [`UNTERMINATED_NOTICES`] entry that has more output
/// glued to it. A notice sitting at the very end of the piece was terminated
/// normally and is left alone.
fn split_unterminated_notices(piece: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut rest = piece;
    'scan: loop {
        for notice in UNTERMINATED_NOTICES {
            let Some(pos) = rest.find(notice) else {
                continue;
            };
            let end = pos + notice.len();
            if end >= rest.len() {
                continue; // properly terminated — nothing glued on
            }
            out.push(rest[..end].to_string());
            rest = &rest[end..];
            continue 'scan;
        }
        break;
    }
    if !rest.is_empty() || out.is_empty() {
        out.push(rest.to_string());
    }
    out
}

/// Turns one raw `read_until(b'\n')` buffer into displayable line(s):
/// lossy UTF-8, trailing `\n` stripped, ANSI stripped, split on `\r`
/// (progress repaints become their own lines), then split after any known
/// unterminated notice. An entirely-empty raw line yields one empty line
/// (blank output lines are legitimate).
pub fn split_output_line(raw: &[u8]) -> Vec<String> {
    let mut text = String::from_utf8_lossy(raw).into_owned();
    if text.ends_with('\n') {
        text.pop();
    }
    let stripped = strip_ansi(&text);
    let pieces: Vec<String> = stripped
        .split('\r')
        .filter(|p| !p.is_empty())
        .flat_map(split_unterminated_notices)
        .collect();
    if pieces.is_empty() {
        vec![String::new()]
    } else {
        pieces
    }
}

fn now_ms() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_millis() as u64)
        .unwrap_or(0)
}

/// Short human phase string for Timeout errors, e.g. `"brew update"`.
pub(crate) fn phase_of(spec: &CommandSpec) -> String {
    let program = spec
        .program
        .file_name()
        .map(|n| n.to_string_lossy().into_owned())
        .unwrap_or_else(|| spec.program.display().to_string());
    let mut phase = program;
    for arg in &spec.args {
        if phase.len() + arg.len() + 1 > 80 {
            phase.push('…');
            break;
        }
        phase.push(' ');
        phase.push_str(arg);
    }
    phase
}

fn append_capped(buf: &mut String, line: &str) {
    if buf.len().saturating_add(line.len()) + 1 > STREAM_CAP_BYTES {
        return; // cap reached — transcripts still carry the full output
    }
    buf.push_str(line);
    buf.push('\n');
}

fn spawn_reader<R>(
    reader: R,
    stream: StreamKind,
    tx: mpsc::UnboundedSender<LogLine>,
) -> tokio::task::JoinHandle<()>
where
    R: AsyncRead + Unpin + Send + 'static,
{
    tokio::spawn(async move {
        let mut reader = BufReader::new(reader);
        let mut buf: Vec<u8> = Vec::new();
        loop {
            buf.clear();
            match reader.read_until(b'\n', &mut buf).await {
                Ok(0) | Err(_) => break,
                Ok(_) => {
                    for piece in split_output_line(&buf) {
                        if tx
                            .send(LogLine {
                                stream,
                                line: piece,
                                ts_ms: now_ms(),
                            })
                            .is_err()
                        {
                            return;
                        }
                    }
                }
            }
        }
    })
}

enum LoopEnd {
    Completed,
    Cancelled,
    TimedOut,
}

/// The real process executor. Cheap to construct — callers that need stall
/// attribution create one per operation via [`RealRunner::with_stall_notify`].
#[derive(Default)]
pub struct RealRunner {
    stall_notify: Option<StallNotify>,
    term_grace: Option<Duration>,
    eof_grace: Option<Duration>,
}

impl RealRunner {
    pub fn new() -> Self {
        Self::default()
    }

    /// Attach a stall-watchdog callback (fires on `Timeout::Stall` silence
    /// thresholds; re-arms on output and after each firing).
    pub fn with_stall_notify(mut self, notify: StallNotify) -> Self {
        self.stall_notify = Some(notify);
        self
    }

    /// Test affordance: shorten the SIGTERM→SIGKILL grace (default 5s).
    pub fn with_term_grace(mut self, grace: Duration) -> Self {
        self.term_grace = Some(grace);
        self
    }

    /// Test affordance: shorten the post-exit EOF grace (default 2s).
    pub fn with_post_exit_eof_grace(mut self, grace: Duration) -> Self {
        self.eof_grace = Some(grace);
        self
    }

    fn grace(&self) -> Duration {
        self.term_grace.unwrap_or(TERM_GRACE)
    }

    fn post_exit_eof_grace(&self) -> Duration {
        self.eof_grace.unwrap_or(POST_EXIT_EOF_GRACE)
    }

    async fn kill_group(&self, pgid: Pid, child: &mut tokio::process::Child, already_exited: bool) {
        if pgid.as_raw() > 0 {
            let _ = killpg(pgid, Signal::SIGTERM);
        }
        if already_exited {
            return;
        }
        match tokio::time::timeout(self.grace(), child.wait()).await {
            Ok(_) => {}
            Err(_) => {
                if pgid.as_raw() > 0 {
                    let _ = killpg(pgid, Signal::SIGKILL);
                }
                let _ = child.wait().await;
            }
        }
    }
}

#[async_trait]
impl CommandRunner for RealRunner {
    async fn run(
        &self,
        spec: &CommandSpec,
        cancel: CancellationToken,
    ) -> Result<CommandOutput, PmError> {
        // The op's REAL cancel token is threaded through so buffered
        // refreshes honor Cancel / cancel_all like streaming ops do.
        self.run_streaming(spec, Arc::new(|_| {}), cancel).await
    }

    async fn run_streaming(
        &self,
        spec: &CommandSpec,
        sink: LineSink,
        cancel: CancellationToken,
    ) -> Result<CommandOutput, PmError> {
        let started = Instant::now();
        let mut cmd = tokio::process::Command::new(&spec.program);
        cmd.args(&spec.args)
            .env_clear()
            .envs(spec.env.iter().cloned())
            .stdin(Stdio::null()) // no sudo, no password entry, ever
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .process_group(0);
        let mut child = cmd.spawn().map_err(|e| PmError::SpawnFailed {
            program: spec.program.display().to_string(),
            detail: e.to_string(),
        })?;
        // process_group(0) makes the child the leader of a fresh group whose
        // pgid equals its pid.
        let pgid = Pid::from_raw(child.id().map(|id| id as i32).unwrap_or(0));

        let (tx, mut rx) = mpsc::unbounded_channel::<LogLine>();
        let stdout = child.stdout.take().expect("stdout piped");
        let stderr = child.stderr.take().expect("stderr piped");
        let out_reader = spawn_reader(stdout, StreamKind::Out, tx.clone());
        let err_reader = spawn_reader(stderr, StreamKind::Err, tx);
        // Both reader tasks own a sender; `rx` closes when both hit EOF.

        let (overall, stall_silence) = match spec.timeout {
            Timeout::Absolute(d) => (d, None),
            Timeout::Stall { silence, hard_cap } => (hard_cap, Some(silence)),
        };
        let overall_deadline = started + overall;
        let mut stall_deadline = stall_silence.map(|s| started + s);
        let mut last_output = started;

        let mut stdout_buf = String::new();
        let mut stderr_buf = String::new();
        let mut streams_done = false;
        let mut exit_status: Option<std::process::ExitStatus> = None;
        // Armed when the child exits with the pipes still open: a descendant
        // holding the inherited write ends must not stall the op until the
        // overall deadline turns a finished command into TimedOut.
        let mut eof_deadline: Option<Instant> = None;

        let end = loop {
            if streams_done && exit_status.is_some() {
                break LoopEnd::Completed;
            }
            tokio::select! {
                maybe = rx.recv(), if !streams_done => {
                    match maybe {
                        Some(line) => {
                            last_output = Instant::now();
                            if let Some(s) = stall_silence {
                                stall_deadline = Some(last_output + s); // re-arms on output
                            }
                            match line.stream {
                                StreamKind::Out => append_capped(&mut stdout_buf, &line.line),
                                StreamKind::Err => append_capped(&mut stderr_buf, &line.line),
                            }
                            sink(line);
                        }
                        None => streams_done = true,
                    }
                }
                res = child.wait(), if exit_status.is_none() => {
                    match res {
                        Ok(status) => {
                            exit_status = Some(status);
                            eof_deadline = Some(Instant::now() + self.post_exit_eof_grace());
                        }
                        Err(e) => {
                            return Err(PmError::Io { detail: format!("wait failed: {e}") });
                        }
                    }
                }
                _ = cancel.cancelled() => break LoopEnd::Cancelled,
                _ = tokio::time::sleep_until(overall_deadline) => break LoopEnd::TimedOut,
                _ = tokio::time::sleep_until(eof_deadline.unwrap_or(overall_deadline)),
                    if eof_deadline.is_some() && !streams_done => {
                    // Child reaped but EOF never arrived (an fd-inheriting
                    // descendant keeps the pipes open). Drain what is already
                    // queued and complete on the child's own exit.
                    while let Ok(line) = rx.try_recv() {
                        match line.stream {
                            StreamKind::Out => append_capped(&mut stdout_buf, &line.line),
                            StreamKind::Err => append_capped(&mut stderr_buf, &line.line),
                        }
                        sink(line);
                    }
                    break LoopEnd::Completed;
                }
                _ = tokio::time::sleep_until(stall_deadline.unwrap_or(overall_deadline)),
                    if stall_deadline.is_some() => {
                    let silent = Instant::now().duration_since(last_output);
                    if let Some(notify) = &self.stall_notify {
                        notify(silent.as_secs());
                    }
                    if let Some(s) = stall_silence {
                        // Fire again after another full threshold of silence.
                        stall_deadline = Some(Instant::now() + s);
                    }
                }
            }
        };
        // Readers may still be parked on a pipe a descendant holds open; they
        // are no longer needed on any exit path.
        out_reader.abort();
        err_reader.abort();

        match end {
            LoopEnd::Completed => Ok(CommandOutput {
                exit_code: exit_status.and_then(|s| s.code()),
                stdout: stdout_buf,
                stderr: stderr_buf,
                duration: started.elapsed(),
            }),
            LoopEnd::Cancelled => {
                self.kill_group(pgid, &mut child, exit_status.is_some())
                    .await;
                Err(PmError::Cancelled)
            }
            LoopEnd::TimedOut => {
                self.kill_group(pgid, &mut child, exit_status.is_some())
                    .await;
                Err(PmError::Timeout {
                    after_secs: overall.as_secs(),
                    phase: phase_of(spec),
                })
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::process::CmdPurpose;
    use std::path::PathBuf;
    use std::sync::Mutex;

    // ---------------- pure helpers (default, deterministic) ----------------

    #[test]
    fn strip_ansi_removes_csi_and_osc_sequences() {
        assert_eq!(
            strip_ansi("\x1b[1;32m==>\x1b[0m Upgrading"),
            "==> Upgrading"
        );
        assert_eq!(strip_ansi("\x1b]0;title\x07plain"), "plain");
        assert_eq!(strip_ansi("no escapes"), "no escapes");
        assert_eq!(strip_ansi("\x1b[2K\x1b[1Gprogress 50%"), "progress 50%");
    }

    #[test]
    fn split_output_line_splits_carriage_return_repaints() {
        assert_eq!(split_output_line(b"plain line\n"), vec!["plain line"]);
        assert_eq!(
            split_output_line(b"progress 1\rprogress 2\rprogress 3\n"),
            vec!["progress 1", "progress 2", "progress 3"]
        );
        // CRLF line endings collapse to the payload.
        assert_eq!(split_output_line(b"windowsy\r\n"), vec!["windowsy"]);
        // A blank line stays one blank line.
        assert_eq!(split_output_line(b"\n"), vec![""]);
        assert_eq!(split_output_line(b"\r\n"), vec![""]);
    }

    #[test]
    fn split_output_line_breaks_after_an_unterminated_notice() {
        // Verbatim from a `mas upgrade` transcript: mas prints the notice with
        // no terminator, so `==> Updated …` glues onto it and both land in one
        // `read_until(b'\n')` buffer.
        assert_eq!(
            split_output_line(
                b"Update progress cannot be displayed==> Updated Canary Mail App (5.21.0) in /Applications/Canary Mail.app\n"
            ),
            vec![
                "Update progress cannot be displayed",
                "==> Updated Canary Mail App (5.21.0) in /Applications/Canary Mail.app",
            ]
        );
    }

    #[test]
    fn split_output_line_leaves_a_terminated_notice_alone() {
        // Same text, but mas terminated it properly — no synthetic break.
        assert_eq!(
            split_output_line(b"Update progress cannot be displayed\n"),
            vec!["Update progress cannot be displayed"]
        );
        // Repeated notices in one buffer each end a line. The break goes AFTER
        // the notice, so text preceding one stays on its line — which is what
        // the real case wants (the notice comes first, `==> Updated …` glues
        // on behind it).
        assert_eq!(
            split_output_line(
                b"Update progress cannot be displayedaUpdate progress cannot be displayedb\n"
            ),
            vec![
                "Update progress cannot be displayed",
                "aUpdate progress cannot be displayed",
                "b",
            ]
        );
        // Output that merely mentions the notice mid-line still splits — the
        // list is literal by design, and a false positive costs one line break.
        assert_eq!(
            split_output_line(b"prefix Update progress cannot be displayed suffix\n"),
            vec!["prefix Update progress cannot be displayed", " suffix"]
        );
    }

    #[test]
    fn split_output_line_is_lossy_on_invalid_utf8() {
        let raw = b"ok \xff\xfe bytes\n";
        let lines = split_output_line(raw);
        assert_eq!(lines.len(), 1);
        assert!(lines[0].starts_with("ok "));
    }

    #[test]
    fn append_capped_stops_at_cap() {
        let mut buf = String::new();
        let chunk = "x".repeat(1024);
        for _ in 0..600 {
            append_capped(&mut buf, &chunk);
        }
        assert!(buf.len() <= STREAM_CAP_BYTES);
        assert!(
            buf.len() > STREAM_CAP_BYTES - 2048,
            "should fill close to cap"
        );
    }

    #[test]
    fn phase_of_is_program_basename_plus_args() {
        let spec = sh_spec("echo hi", Timeout::Absolute(Duration::from_secs(1)));
        assert_eq!(phase_of(&spec), "sh -c echo hi");
    }

    // ---------------- real-process tests (#[ignore], developer-run) --------

    fn sh_spec(script: &str, timeout: Timeout) -> CommandSpec {
        CommandSpec {
            program: PathBuf::from("/bin/sh"),
            args: vec!["-c".into(), script.into()],
            env: vec![("PATH".into(), "/usr/bin:/bin".into())],
            timeout,
            purpose: CmdPurpose::Refresh,
        }
    }

    fn collecting_sink() -> (LineSink, Arc<Mutex<Vec<LogLine>>>) {
        let lines: Arc<Mutex<Vec<LogLine>>> = Arc::new(Mutex::new(Vec::new()));
        let clone = lines.clone();
        (
            Arc::new(move |line: LogLine| clone.lock().unwrap().push(line)),
            lines,
        )
    }

    #[tokio::test(flavor = "multi_thread")]
    #[ignore]
    async fn real_spawns_captures_streams_and_exit_code() {
        let runner = RealRunner::new();
        let spec = sh_spec(
            "printf 'a\\nb\\n'; printf 'oops\\n' >&2; exit 3",
            Timeout::Absolute(Duration::from_secs(5)),
        );
        let (sink, lines) = collecting_sink();
        let out = runner
            .run_streaming(&spec, sink, CancellationToken::new())
            .await
            .unwrap();
        assert_eq!(out.exit_code, Some(3));
        assert_eq!(out.stdout, "a\nb\n");
        assert_eq!(out.stderr, "oops\n");
        let lines = lines.lock().unwrap();
        let outs: Vec<_> = lines
            .iter()
            .filter(|l| l.stream == StreamKind::Out)
            .map(|l| l.line.clone())
            .collect();
        assert_eq!(outs, vec!["a", "b"]);
    }

    #[tokio::test(flavor = "multi_thread")]
    #[ignore]
    async fn real_splits_carriage_return_progress() {
        let runner = RealRunner::new();
        let spec = sh_spec(
            "printf 'p1\\rp2\\r\\n'",
            Timeout::Absolute(Duration::from_secs(5)),
        );
        let (sink, lines) = collecting_sink();
        runner
            .run_streaming(&spec, sink, CancellationToken::new())
            .await
            .unwrap();
        let outs: Vec<_> = lines
            .lock()
            .unwrap()
            .iter()
            .map(|l| l.line.clone())
            .collect();
        assert_eq!(outs, vec!["p1", "p2"]);
    }

    #[tokio::test(flavor = "multi_thread")]
    #[ignore]
    async fn real_cancel_sigterm_kills_group() {
        let runner = RealRunner::new();
        let spec = sh_spec("sleep 30", Timeout::Absolute(Duration::from_secs(60)));
        let cancel = CancellationToken::new();
        let canceller = cancel.clone();
        tokio::spawn(async move {
            tokio::time::sleep(Duration::from_millis(200)).await;
            canceller.cancel();
        });
        let started = std::time::Instant::now();
        let err = runner
            .run_streaming(&spec, Arc::new(|_| {}), cancel)
            .await
            .unwrap_err();
        assert_eq!(err, PmError::Cancelled);
        assert!(
            started.elapsed() < Duration::from_secs(5),
            "SIGTERM path is fast"
        );
    }

    /// Default-run (NOT `#[ignore]`): this is the only test guarding the
    /// SIGTERM→grace→SIGKILL escalation in `kill_group` — the queue-level
    /// cancel/timeout tests only assert the status→footer mapping through the
    /// FakeRunner. Fast (300ms grace) and deterministic: /bin/sh only, no
    /// network, no package managers.
    #[tokio::test(flavor = "multi_thread")]
    async fn real_cancel_escalates_to_sigkill_when_term_ignored() {
        let runner = RealRunner::new().with_term_grace(Duration::from_millis(300));
        let spec = sh_spec(
            "trap '' TERM; sleep 30",
            Timeout::Absolute(Duration::from_secs(60)),
        );
        let cancel = CancellationToken::new();
        let canceller = cancel.clone();
        tokio::spawn(async move {
            tokio::time::sleep(Duration::from_millis(200)).await;
            canceller.cancel();
        });
        let started = std::time::Instant::now();
        let err = runner
            .run_streaming(&spec, Arc::new(|_| {}), cancel)
            .await
            .unwrap_err();
        assert_eq!(err, PmError::Cancelled);
        assert!(
            started.elapsed() < Duration::from_secs(5),
            "SIGKILL after short grace, not the default 5s"
        );
    }

    /// Default-run: a leader that exits while a backgrounded descendant holds
    /// the inherited stdout pipe must complete on the child's exit (bounded
    /// EOF grace), not stall until the absolute timeout kills it as TimedOut.
    #[tokio::test(flavor = "multi_thread")]
    async fn real_completes_when_descendant_holds_pipes_open_after_exit() {
        let runner = RealRunner::new().with_post_exit_eof_grace(Duration::from_millis(300));
        // `sleep 15` inherits the stdout write end and keeps it open far past
        // the 5s absolute timeout; the leader exits immediately with code 7.
        let spec = sh_spec(
            "sleep 15 & echo done; exit 7",
            Timeout::Absolute(Duration::from_secs(5)),
        );
        let (sink, lines) = collecting_sink();
        let started = std::time::Instant::now();
        let out = runner
            .run_streaming(&spec, sink, CancellationToken::new())
            .await
            .expect("must complete on child exit, not TimedOut on pipe EOF");
        assert_eq!(out.exit_code, Some(7));
        assert_eq!(out.stdout, "done\n");
        assert_eq!(lines.lock().unwrap().len(), 1);
        assert!(
            started.elapsed() < Duration::from_secs(5),
            "completed within the EOF grace, not at the timeout"
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    #[ignore]
    async fn real_absolute_timeout_kills_and_reports() {
        let runner = RealRunner::new();
        let spec = sh_spec("sleep 30", Timeout::Absolute(Duration::from_millis(300)));
        let err = runner
            .run(&spec, CancellationToken::new())
            .await
            .unwrap_err();
        match err {
            PmError::Timeout { after_secs, phase } => {
                assert_eq!(after_secs, 0); // 300ms rounds down
                assert!(phase.starts_with("sh -c"));
            }
            other => panic!("expected Timeout, got {other:?}"),
        }
    }

    #[tokio::test(flavor = "multi_thread")]
    #[ignore]
    async fn real_stall_fires_at_threshold_and_rearms_on_output() {
        let fired: Arc<Mutex<Vec<u64>>> = Arc::new(Mutex::new(Vec::new()));
        let fired_clone = fired.clone();
        let runner = RealRunner::new()
            .with_stall_notify(Arc::new(move |secs| fired_clone.lock().unwrap().push(secs)));
        // Quiet 700ms, one line, quiet 700ms, exit — with a 500ms stall
        // threshold this fires once per silence episode (twice total).
        let spec = sh_spec(
            "sleep 0.7; echo tick; sleep 0.7",
            Timeout::Stall {
                silence: Duration::from_millis(500),
                hard_cap: Duration::from_secs(10),
            },
        );
        let (sink, lines) = collecting_sink();
        let out = runner
            .run_streaming(&spec, sink, CancellationToken::new())
            .await
            .unwrap();
        assert_eq!(out.exit_code, Some(0));
        assert_eq!(lines.lock().unwrap().len(), 1);
        let firings = fired.lock().unwrap().len();
        assert_eq!(
            firings, 2,
            "one firing per silence episode, re-armed by output"
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    #[ignore]
    async fn real_stall_hard_cap_times_out() {
        let runner = RealRunner::new();
        let spec = sh_spec(
            "sleep 30",
            Timeout::Stall {
                silence: Duration::from_secs(10),
                hard_cap: Duration::from_millis(400),
            },
        );
        let err = runner
            .run(&spec, CancellationToken::new())
            .await
            .unwrap_err();
        assert!(matches!(err, PmError::Timeout { .. }));
    }

    #[tokio::test(flavor = "multi_thread")]
    #[ignore]
    async fn real_child_env_is_exactly_the_spec_env() {
        let runner = RealRunner::new();
        let mut spec = sh_spec(
            "printf '%s\\n' \"$PM_PROBE\" \"$HOME\"",
            Timeout::Absolute(Duration::from_secs(5)),
        );
        spec.env = vec![
            ("PATH".into(), "/usr/bin:/bin".into()),
            ("PM_PROBE".into(), "constructed".into()),
            ("HOME".into(), "/tmp/pm-home".into()),
        ];
        let out = runner.run(&spec, CancellationToken::new()).await.unwrap();
        assert_eq!(out.stdout, "constructed\n/tmp/pm-home\n");
    }
}
