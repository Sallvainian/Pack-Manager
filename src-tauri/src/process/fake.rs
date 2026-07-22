//! `FakeRunner` (SPEC §5.6): canned outputs keyed by (program basename, args)
//! via `.on(...)`, scripted line streams via `.on_streaming(...).emits(...)
//! .gate(...)`, `tokio::sync::Notify` gates for deterministic ordering, call
//! recording, and panic-on-unmatched.
//!
//! Streaming rules additionally honor `Timeout::{Absolute, Stall}` against
//! their scripted timeline (delays and gate waits) under tokio's clock, so
//! stall/hard-cap plumbing is testable with paused time and zero sleeps.
#![cfg(any(test, feature = "test-util"))]

use std::collections::VecDeque;
use std::future::Future;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use async_trait::async_trait;
use tokio::sync::Notify;
use tokio::time::Instant;
use tokio_util::sync::CancellationToken;

use crate::error::PmError;
use crate::ipc::StreamKind;
use crate::process::runner::{phase_of, CommandRunner, StallNotify};
use crate::process::{CmdPurpose, CommandOutput, CommandSpec, LineSink, LogLine, Timeout};

/// Reads a committed fixture from `dev/fixtures/` (panics loudly if missing).
pub fn fixture(name: &str) -> String {
    let path = format!("{}/../dev/fixtures/{name}", env!("CARGO_MANIFEST_DIR"));
    std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("FakeRunner: missing fixture {path}: {e}"))
}

/// One recorded invocation (start/end use tokio's clock so paused-time tests
/// can assert non-overlap intervals).
#[derive(Debug, Clone)]
pub struct RecordedCall {
    pub program: PathBuf,
    pub basename: String,
    pub args: Vec<String>,
    pub env: Vec<(String, String)>,
    pub purpose: CmdPurpose,
    pub streaming: bool,
    pub started_at: Instant,
    pub finished_at: Option<Instant>,
}

#[derive(Debug, Clone)]
enum BufferedResponse {
    Output(CommandOutput),
    Error(PmError),
}

#[derive(Debug)]
struct BufferedRule {
    basename: String,
    args: Vec<String>,
    responses: VecDeque<BufferedResponse>,
    /// When set, the fake command blocks until notified (or cancelled/timed
    /// out) before returning — deterministic ordering for buffered-op tests.
    gate: Option<Arc<Notify>>,
}

#[derive(Debug, Clone)]
struct TimedLine {
    delay_before: Duration,
    stream: StreamKind,
    line: String,
}

#[derive(Clone)]
struct StreamingRule {
    basename: String,
    args: Vec<String>,
    lines: Vec<TimedLine>,
    gate: Option<Arc<Notify>>,
    result: StreamResult,
}

#[derive(Debug, Clone)]
enum StreamResult {
    Exit(i32),
    Error(PmError),
}

/// Test double for [`CommandRunner`]. Buffered responses pop in order (the
/// last one repeats); streaming rules are reusable. Any invocation with no
/// matching rule panics with the offending spec and the registered rules.
#[derive(Default)]
pub struct FakeRunner {
    buffered: Mutex<Vec<BufferedRule>>,
    streaming: Mutex<Vec<StreamingRule>>,
    calls: Mutex<Vec<RecordedCall>>,
    stall_notify: Mutex<Option<StallNotify>>,
}

impl FakeRunner {
    pub fn new() -> Self {
        Self::default()
    }

    /// Registers a buffered rule for `run` keyed by (program basename, args).
    /// Chain response builders; each call queues one response.
    pub fn on(&self, program: &str, args: &[&str]) -> OnBuffered<'_> {
        let mut rules = self.buffered.lock().unwrap();
        rules.push(BufferedRule {
            basename: program.to_string(),
            args: args.iter().map(|s| s.to_string()).collect(),
            responses: VecDeque::new(),
            gate: None,
        });
        let index = rules.len() - 1;
        OnBuffered {
            runner: self,
            index,
        }
    }

    /// Registers a streaming rule for `run_streaming`.
    pub fn on_streaming(&self, program: &str, args: &[&str]) -> OnStreaming<'_> {
        let mut rules = self.streaming.lock().unwrap();
        rules.push(StreamingRule {
            basename: program.to_string(),
            args: args.iter().map(|s| s.to_string()).collect(),
            lines: Vec::new(),
            gate: None,
            result: StreamResult::Exit(0),
        });
        let index = rules.len() - 1;
        OnStreaming {
            runner: self,
            index,
        }
    }

    /// Mirrors `RealRunner::with_stall_notify` (settable after construction
    /// because fakes are usually shared as `Arc`).
    pub fn set_stall_notify(&self, notify: StallNotify) {
        *self.stall_notify.lock().unwrap() = Some(notify);
    }

    /// Every invocation so far, in order.
    pub fn calls(&self) -> Vec<RecordedCall> {
        self.calls.lock().unwrap().clone()
    }

    fn record_start(&self, spec: &CommandSpec, streaming: bool) -> usize {
        let mut calls = self.calls.lock().unwrap();
        calls.push(RecordedCall {
            program: spec.program.clone(),
            basename: basename_of(spec),
            args: spec.args.clone(),
            env: spec.env.clone(),
            purpose: spec.purpose,
            streaming,
            started_at: Instant::now(),
            finished_at: None,
        });
        calls.len() - 1
    }

    fn record_end(&self, index: usize) {
        self.calls.lock().unwrap()[index].finished_at = Some(Instant::now());
    }

    fn registered_summary(&self) -> String {
        let buffered = self.buffered.lock().unwrap();
        let streaming = self.streaming.lock().unwrap();
        let mut out = String::new();
        for r in buffered.iter() {
            out.push_str(&format!("  buffered:  {} {:?}\n", r.basename, r.args));
        }
        for r in streaming.iter() {
            out.push_str(&format!("  streaming: {} {:?}\n", r.basename, r.args));
        }
        if out.is_empty() {
            out.push_str("  (none)\n");
        }
        out
    }

    fn take_buffered_response(
        &self,
        spec: &CommandSpec,
    ) -> Option<(BufferedResponse, Option<Arc<Notify>>)> {
        let basename = basename_of(spec);
        let mut rules = self.buffered.lock().unwrap();
        let rule = rules
            .iter_mut()
            .find(|r| r.basename == basename && r.args == spec.args)?;
        if rule.responses.is_empty() {
            panic!(
                "FakeRunner: rule `{} {:?}` was registered without a response",
                basename, spec.args
            );
        }
        let response = if rule.responses.len() > 1 {
            rule.responses.pop_front()
        } else {
            rule.responses.front().cloned()
        };
        response.map(|r| (r, rule.gate.clone()))
    }

    fn find_streaming_rule(&self, spec: &CommandSpec) -> Option<StreamingRule> {
        let basename = basename_of(spec);
        self.streaming
            .lock()
            .unwrap()
            .iter()
            .find(|r| r.basename == basename && r.args == spec.args)
            .cloned()
    }
}

fn basename_of(spec: &CommandSpec) -> String {
    spec.program
        .file_name()
        .map(|n| n.to_string_lossy().into_owned())
        .unwrap_or_else(|| spec.program.display().to_string())
}

/// Builder handle for buffered rules.
pub struct OnBuffered<'a> {
    runner: &'a FakeRunner,
    index: usize,
}

impl OnBuffered<'_> {
    fn push(self, response: BufferedResponse) -> Self {
        self.runner.buffered.lock().unwrap()[self.index]
            .responses
            .push_back(response);
        self
    }

    /// Full canned output.
    pub fn output(self, out: CommandOutput) -> Self {
        self.push(BufferedResponse::Output(out))
    }

    /// Exit 0 with the given stdout.
    pub fn ok(self, stdout: &str) -> Self {
        self.exit(0, stdout, "")
    }

    /// Arbitrary exit code + streams.
    pub fn exit(self, code: i32, stdout: &str, stderr: &str) -> Self {
        self.output(CommandOutput {
            exit_code: Some(code),
            stdout: stdout.to_string(),
            stderr: stderr.to_string(),
            duration: Duration::ZERO,
        })
    }

    /// Exit 0 with stdout loaded from `dev/fixtures/<name>`.
    pub fn fixture(self, name: &str) -> Self {
        let body = fixture(name);
        self.ok(&body)
    }

    /// Fixture stdout with a specific exit code (npm's exit-1-with-JSON).
    pub fn fixture_with_exit(self, name: &str, code: i32) -> Self {
        let body = fixture(name);
        self.exit(code, &body, "")
    }

    /// The invocation fails at the runner level (spawn failure, timeout, …).
    pub fn fail(self, err: PmError) -> Self {
        self.push(BufferedResponse::Error(err))
    }

    /// The fake command blocks until the gate is notified (deterministic
    /// ordering for buffered-op tests); cancel/timeout interrupt the wait
    /// like the real runner's SIGTERM/deadline would.
    pub fn gate(self, gate: Arc<Notify>) -> Self {
        self.runner.buffered.lock().unwrap()[self.index].gate = Some(gate);
        self
    }
}

/// Builder handle for streaming rules.
pub struct OnStreaming<'a> {
    runner: &'a FakeRunner,
    index: usize,
}

impl OnStreaming<'_> {
    fn with_rule(self, f: impl FnOnce(&mut StreamingRule)) -> Self {
        f(&mut self.runner.streaming.lock().unwrap()[self.index]);
        self
    }

    /// Appends lines emitted immediately (no scripted delay).
    pub fn emits(self, lines: &[(StreamKind, &str)]) -> Self {
        self.with_rule(|r| {
            r.lines.extend(lines.iter().map(|(stream, line)| TimedLine {
                delay_before: Duration::ZERO,
                stream: *stream,
                line: line.to_string(),
            }));
        })
    }

    /// Appends one line emitted after a scripted delay (paused-time friendly).
    pub fn emits_after(self, delay: Duration, stream: StreamKind, line: &str) -> Self {
        self.with_rule(|r| {
            r.lines.push(TimedLine {
                delay_before: delay,
                stream,
                line: line.to_string(),
            });
        })
    }

    /// After emitting all lines, the fake command blocks until the gate is
    /// notified (deterministic ordering for scheduler tests).
    pub fn gate(self, gate: Arc<Notify>) -> Self {
        self.with_rule(|r| r.gate = Some(gate))
    }

    /// Exit code for the completed stream (default 0).
    pub fn exit(self, code: i32) -> Self {
        self.with_rule(|r| r.result = StreamResult::Exit(code))
    }

    /// The invocation fails at the runner level after the scripted lines.
    pub fn fail(self, err: PmError) -> Self {
        self.with_rule(|r| r.result = StreamResult::Error(err))
    }
}

struct Watchdog {
    overall: Duration,
    overall_deadline: Instant,
    silence: Option<Duration>,
    stall_deadline: Option<Instant>,
    last_output: Instant,
}

impl Watchdog {
    fn new(timeout: &Timeout, start: Instant) -> Self {
        let (overall, silence) = match *timeout {
            Timeout::Absolute(d) => (d, None),
            Timeout::Stall { silence, hard_cap } => (hard_cap, Some(silence)),
        };
        Watchdog {
            overall,
            overall_deadline: start + overall,
            silence,
            stall_deadline: silence.map(|s| start + s),
            last_output: start,
        }
    }

    fn on_output(&mut self) {
        self.last_output = Instant::now();
        if let Some(s) = self.silence {
            self.stall_deadline = Some(self.last_output + s);
        }
    }
}

enum WaitEnd {
    Ready,
    Cancelled,
    TimedOut,
}

async fn guarded_wait<F: Future<Output = ()>>(
    fut: F,
    cancel: &CancellationToken,
    wd: &mut Watchdog,
    notify: &Option<StallNotify>,
) -> WaitEnd {
    tokio::pin!(fut);
    loop {
        tokio::select! {
            _ = &mut fut => return WaitEnd::Ready,
            _ = cancel.cancelled() => return WaitEnd::Cancelled,
            _ = tokio::time::sleep_until(wd.overall_deadline) => return WaitEnd::TimedOut,
            _ = tokio::time::sleep_until(wd.stall_deadline.unwrap_or(wd.overall_deadline)),
                if wd.stall_deadline.is_some() => {
                let silent = Instant::now().duration_since(wd.last_output);
                if let Some(n) = notify {
                    n(silent.as_secs());
                }
                if let Some(s) = wd.silence {
                    wd.stall_deadline = Some(Instant::now() + s);
                }
            }
        }
    }
}

fn now_ms() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_millis() as u64)
        .unwrap_or(0)
}

#[async_trait]
impl CommandRunner for FakeRunner {
    async fn run(
        &self,
        spec: &CommandSpec,
        cancel: CancellationToken,
    ) -> Result<CommandOutput, PmError> {
        let started = Instant::now();
        let index = self.record_start(spec, false);
        let (response, gate) = self.take_buffered_response(spec).unwrap_or_else(|| {
            panic!(
                "FakeRunner: no buffered rule for `{} {:?}`\nregistered rules:\n{}",
                basename_of(spec),
                spec.args,
                self.registered_summary()
            )
        });
        // Mirror the real runner: a cancelled token wins over completion.
        if cancel.is_cancelled() {
            self.record_end(index);
            return Err(PmError::Cancelled);
        }
        if let Some(gate) = gate {
            let notify = self.stall_notify.lock().unwrap().clone();
            let mut wd = Watchdog::new(&spec.timeout, started);
            match guarded_wait(gate.notified(), &cancel, &mut wd, &notify).await {
                WaitEnd::Ready => {}
                WaitEnd::Cancelled => {
                    self.record_end(index);
                    return Err(PmError::Cancelled);
                }
                WaitEnd::TimedOut => {
                    self.record_end(index);
                    return Err(PmError::Timeout {
                        after_secs: wd.overall.as_secs(),
                        phase: phase_of(spec),
                    });
                }
            }
        }
        self.record_end(index);
        match response {
            BufferedResponse::Output(out) => Ok(out),
            BufferedResponse::Error(err) => Err(err),
        }
    }

    async fn run_streaming(
        &self,
        spec: &CommandSpec,
        sink: LineSink,
        cancel: CancellationToken,
    ) -> Result<CommandOutput, PmError> {
        let started = Instant::now();
        let index = self.record_start(spec, true);
        let rule = self.find_streaming_rule(spec).unwrap_or_else(|| {
            panic!(
                "FakeRunner: no streaming rule for `{} {:?}`\nregistered rules:\n{}",
                basename_of(spec),
                spec.args,
                self.registered_summary()
            )
        });
        let notify = self.stall_notify.lock().unwrap().clone();
        let mut wd = Watchdog::new(&spec.timeout, started);
        let mut stdout = String::new();
        let mut stderr = String::new();

        for tl in &rule.lines {
            if !tl.delay_before.is_zero() {
                match guarded_wait(
                    tokio::time::sleep(tl.delay_before),
                    &cancel,
                    &mut wd,
                    &notify,
                )
                .await
                {
                    WaitEnd::Ready => {}
                    WaitEnd::Cancelled => {
                        self.record_end(index);
                        return Err(PmError::Cancelled);
                    }
                    WaitEnd::TimedOut => {
                        self.record_end(index);
                        return Err(PmError::Timeout {
                            after_secs: wd.overall.as_secs(),
                            phase: phase_of(spec),
                        });
                    }
                }
            }
            wd.on_output();
            match tl.stream {
                StreamKind::Out => {
                    stdout.push_str(&tl.line);
                    stdout.push('\n');
                }
                StreamKind::Err => {
                    stderr.push_str(&tl.line);
                    stderr.push('\n');
                }
            }
            sink(LogLine {
                stream: tl.stream,
                line: tl.line.clone(),
                ts_ms: now_ms(),
            });
        }

        if let Some(gate) = &rule.gate {
            let gate = gate.clone();
            match guarded_wait(gate.notified(), &cancel, &mut wd, &notify).await {
                WaitEnd::Ready => {}
                WaitEnd::Cancelled => {
                    self.record_end(index);
                    return Err(PmError::Cancelled);
                }
                WaitEnd::TimedOut => {
                    self.record_end(index);
                    return Err(PmError::Timeout {
                        after_secs: wd.overall.as_secs(),
                        phase: phase_of(spec),
                    });
                }
            }
        }

        self.record_end(index);
        match rule.result {
            StreamResult::Exit(code) => Ok(CommandOutput {
                exit_code: Some(code),
                stdout,
                stderr,
                duration: started.elapsed(),
            }),
            StreamResult::Error(err) => Err(err),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};

    fn spec(program: &str, args: &[&str], timeout: Timeout) -> CommandSpec {
        CommandSpec {
            program: PathBuf::from(program),
            args: args.iter().map(|s| s.to_string()).collect(),
            env: vec![("PATH".into(), "/fake".into())],
            timeout,
            purpose: CmdPurpose::Refresh,
        }
    }

    fn abs5() -> Timeout {
        Timeout::Absolute(Duration::from_secs(5))
    }

    #[tokio::test]
    async fn fake_matches_on_basename_and_args_and_records_calls() {
        let fake = FakeRunner::new();
        fake.on("brew", &["update"]).ok("Already up-to-date.\n");
        let out = fake
            .run(
                &spec("/opt/homebrew/bin/brew", &["update"], abs5()),
                CancellationToken::new(),
            )
            .await
            .unwrap();
        assert_eq!(out.exit_code, Some(0));
        assert_eq!(out.stdout, "Already up-to-date.\n");

        let calls = fake.calls();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].basename, "brew");
        assert_eq!(calls[0].args, vec!["update"]);
        assert_eq!(
            calls[0].env,
            vec![("PATH".to_string(), "/fake".to_string())]
        );
        assert!(!calls[0].streaming);
        assert!(calls[0].finished_at.is_some());
    }

    #[tokio::test]
    async fn fake_pops_successive_responses_then_repeats_last() {
        let fake = FakeRunner::new();
        fake.on("npm", &["outdated", "-g", "--json"])
            .exit(1, "{}", "")
            .ok("{}");
        let s = spec("/x/npm", &["outdated", "-g", "--json"], abs5());
        let tok = CancellationToken::new;
        assert_eq!(fake.run(&s, tok()).await.unwrap().exit_code, Some(1));
        assert_eq!(fake.run(&s, tok()).await.unwrap().exit_code, Some(0));
        assert_eq!(
            fake.run(&s, tok()).await.unwrap().exit_code,
            Some(0),
            "last repeats"
        );
    }

    #[tokio::test]
    async fn fake_fail_returns_the_scripted_error() {
        let fake = FakeRunner::new();
        fake.on("brew", &["outdated", "--json=v2"])
            .fail(PmError::Timeout {
                after_secs: 120,
                phase: "brew outdated --json=v2".into(),
            });
        let err = fake
            .run(
                &spec("/opt/homebrew/bin/brew", &["outdated", "--json=v2"], abs5()),
                CancellationToken::new(),
            )
            .await
            .unwrap_err();
        assert!(matches!(
            err,
            PmError::Timeout {
                after_secs: 120,
                ..
            }
        ));
    }

    #[tokio::test]
    #[should_panic(expected = "no buffered rule")]
    async fn fake_panics_on_unmatched() {
        let fake = FakeRunner::new();
        fake.on("brew", &["update"]).ok("");
        let _ = fake
            .run(
                &spec("/x/mise", &["ls", "--json"], abs5()),
                CancellationToken::new(),
            )
            .await;
    }

    #[tokio::test]
    async fn fake_fixture_loads_from_dev_fixtures() {
        let fake = FakeRunner::new();
        fake.on("brew", &["outdated", "--json=v2"])
            .fixture("brew_outdated.json");
        let out = fake
            .run(
                &spec("/opt/homebrew/bin/brew", &["outdated", "--json=v2"], abs5()),
                CancellationToken::new(),
            )
            .await
            .unwrap();
        assert!(out.stdout.contains("formulae"), "real fixture body loaded");
    }

    /// Regression pin for the buffered-cancel seam: a gated buffered command
    /// (refresh-shaped) must return `Cancelled` when the op's token fires —
    /// previously buffered runs ignored cancellation entirely.
    #[tokio::test(start_paused = true)]
    async fn fake_buffered_gate_cancel_returns_cancelled() {
        let fake = Arc::new(FakeRunner::new());
        let gate = Arc::new(Notify::new());
        fake.on("brew", &["update"]).ok("").gate(gate);

        let cancel = CancellationToken::new();
        let runner = fake.clone();
        let token = cancel.clone();
        let handle = tokio::spawn(async move {
            runner
                .run(&spec("/opt/homebrew/bin/brew", &["update"], abs5()), token)
                .await
        });
        tokio::task::yield_now().await;
        assert!(!handle.is_finished(), "held by the gate");
        cancel.cancel();
        assert_eq!(handle.await.unwrap().unwrap_err(), PmError::Cancelled);
    }

    #[tokio::test(start_paused = true)]
    async fn fake_streaming_emits_lines_and_waits_gate() {
        let fake = Arc::new(FakeRunner::new());
        let gate = Arc::new(Notify::new());
        fake.on_streaming("npm", &["install", "-g", "typescript@latest"])
            .emits(&[
                (StreamKind::Out, "added 1 package in 4s"),
                (StreamKind::Err, "npm warn deprecated"),
            ])
            .gate(gate.clone())
            .exit(0);

        let seen = Arc::new(Mutex::new(Vec::<LogLine>::new()));
        let sink_seen = seen.clone();
        let sink: LineSink = Arc::new(move |l| sink_seen.lock().unwrap().push(l));

        let runner = fake.clone();
        let handle = tokio::spawn(async move {
            runner
                .run_streaming(
                    &spec("/x/npm", &["install", "-g", "typescript@latest"], abs5()),
                    sink,
                    CancellationToken::new(),
                )
                .await
        });

        // Let the fake emit and park on the gate.
        tokio::task::yield_now().await;
        assert_eq!(
            seen.lock().unwrap().len(),
            2,
            "lines stream before the gate"
        );
        assert!(!handle.is_finished(), "held by the gate");

        gate.notify_one();
        let out = handle.await.unwrap().unwrap();
        assert_eq!(out.exit_code, Some(0));
        assert_eq!(out.stdout, "added 1 package in 4s\n");
        assert_eq!(out.stderr, "npm warn deprecated\n");
        let calls = fake.calls();
        assert!(calls[0].streaming);
        assert!(calls[0].finished_at.is_some());
    }

    #[tokio::test(start_paused = true)]
    async fn fake_streaming_cancel_while_gated_returns_cancelled() {
        let fake = Arc::new(FakeRunner::new());
        let gate = Arc::new(Notify::new());
        fake.on_streaming("brew", &["upgrade", "dolt"])
            .emits(&[(StreamKind::Out, "==> Upgrading dolt")])
            .gate(gate);

        let cancel = CancellationToken::new();
        let runner = fake.clone();
        let token = cancel.clone();
        let handle = tokio::spawn(async move {
            runner
                .run_streaming(
                    &spec("/opt/homebrew/bin/brew", &["upgrade", "dolt"], abs5()),
                    Arc::new(|_| {}),
                    token,
                )
                .await
        });
        tokio::task::yield_now().await;
        cancel.cancel();
        assert_eq!(handle.await.unwrap().unwrap_err(), PmError::Cancelled);
    }

    #[tokio::test(start_paused = true)]
    async fn fake_streaming_hard_cap_times_out_under_paused_time() {
        let fake = FakeRunner::new();
        fake.on_streaming("brew", &["upgrade", "dolt"]).emits_after(
            Duration::from_secs(3600),
            StreamKind::Out,
            "never emitted",
        );
        let err = fake
            .run_streaming(
                &spec(
                    "/opt/homebrew/bin/brew",
                    &["upgrade", "dolt"],
                    Timeout::Stall {
                        silence: Duration::from_secs(120),
                        hard_cap: Duration::from_secs(1800),
                    },
                ),
                Arc::new(|_| {}),
                CancellationToken::new(),
            )
            .await
            .unwrap_err();
        match err {
            PmError::Timeout { after_secs, .. } => assert_eq!(after_secs, 1800),
            other => panic!("expected Timeout, got {other:?}"),
        }
    }

    #[tokio::test(start_paused = true)]
    async fn fake_streaming_stall_notify_fires_and_rearms_on_output() {
        let fake = FakeRunner::new();
        let fired = Arc::new(AtomicUsize::new(0));
        let fired_clone = fired.clone();
        fake.set_stall_notify(Arc::new(move |_secs| {
            fired_clone.fetch_add(1, Ordering::SeqCst);
        }));
        // 130s of silence (one stall firing at 120s), a line (re-arm), then
        // done — no second firing because the command ends.
        fake.on_streaming("mise", &["upgrade", "uv"])
            .emits_after(
                Duration::from_secs(130),
                StreamKind::Out,
                "mise uv upgraded",
            )
            .exit(0);
        let out = fake
            .run_streaming(
                &spec(
                    "/opt/homebrew/bin/mise",
                    &["upgrade", "uv"],
                    Timeout::Stall {
                        silence: Duration::from_secs(120),
                        hard_cap: Duration::from_secs(1800),
                    },
                ),
                Arc::new(|_| {}),
                CancellationToken::new(),
            )
            .await
            .unwrap();
        assert_eq!(out.exit_code, Some(0));
        assert_eq!(
            fired.load(Ordering::SeqCst),
            1,
            "fired once at the threshold"
        );
    }
}
