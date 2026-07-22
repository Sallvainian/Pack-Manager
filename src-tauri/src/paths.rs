//! ToolEnv & PATH resolution (SPEC §5.2) — the #1 failure mode.
//!
//! Finder-launched apps get `PATH=/usr/bin:/bin:/usr/sbin:/sbin`. The ToolEnv
//! is built once at startup (and rebuilt on Re-detect) from:
//! 1. a static list (order matters; mise shims first so uv/npm/node resolve to
//!    shims),
//! 2. a best-effort, non-fatal login-shell probe (`$SHELL -l -c` with sentinel
//!    markers, 5s timeout, 64KiB output cap),
//! 3. a merge: static list first, probe entries appended deduped.
//!
//! Probe failure → `source: staticFallback`, WARN logged, surfaced in the
//! Environment Report. Tools resolve to ABSOLUTE paths at detection
//! (`which::which_in`); children are spawned by absolute path, never PATH
//! lookup, with a constructed environment (never inherited).

use std::path::{Path, PathBuf};
use std::process::Stdio;
use std::time::Duration;

use nix::sys::signal::{killpg, Signal};
use nix::unistd::Pid;
use tokio::io::AsyncReadExt;

use crate::error::PmError;
use crate::ipc::{EnvInfo, PathSource};

/// Sentinel emitted before the probed `$PATH` (survives profile noise).
pub const PROBE_SENTINEL_START: &str = "__PM_S__";
/// Sentinel emitted after the probed `$PATH`.
pub const PROBE_SENTINEL_END: &str = "__PM_E__";
/// The `-c` script per SPEC §5.2: sentinels around `$PATH`. `-i` is
/// deliberately NOT used (interactive rc files can block on TTY).
pub const PROBE_SCRIPT: &str = "echo __PM_S__; printf %s \"$PATH\"; echo; echo __PM_E__";
/// Login-shell probe timeout (best-effort, non-fatal).
pub const PROBE_TIMEOUT: Duration = Duration::from_secs(5);
/// Cap on probe stdout — profiles can be arbitrarily noisy.
pub const PROBE_OUTPUT_CAP: u64 = 64 * 1024;

/// The static search list (SPEC §5.2 step 1). Order matters: mise shims first
/// so uv/npm/node resolve to shims, never to brew-installed binaries.
pub fn static_entries(home: &Path) -> Vec<PathBuf> {
    vec![
        home.join(".local/share/mise/shims"),
        PathBuf::from("/opt/homebrew/bin"),
        PathBuf::from("/opt/homebrew/sbin"),
        home.join(".cargo/bin"),
        home.join(".local/bin"),
        PathBuf::from("/usr/local/bin"),
        PathBuf::from("/usr/bin"),
        PathBuf::from("/bin"),
        PathBuf::from("/usr/sbin"),
        PathBuf::from("/sbin"),
    ]
}

/// Extracts the probed `$PATH` from raw login-shell stdout: the content between
/// the LAST start-sentinel line and the FIRST end-sentinel line after it.
/// Profile noise before/after the sentinels is ignored by construction.
pub fn extract_probe_path(stdout: &str) -> Option<String> {
    let lines: Vec<&str> = stdout.lines().collect();
    let start = lines
        .iter()
        .rposition(|l| l.trim() == PROBE_SENTINEL_START)?;
    let end = lines[start + 1..]
        .iter()
        .position(|l| l.trim() == PROBE_SENTINEL_END)
        .map(|i| i + start + 1)?;
    let between = &lines[start + 1..end];
    if between.is_empty() {
        return None;
    }
    // Between our two echo commands nothing else runs, so this is normally a
    // single line holding `$PATH`.
    let value = between.concat();
    if value.is_empty() {
        None
    } else {
        Some(value)
    }
}

/// Splits a `PATH` string into entries, dropping empty segments.
pub fn split_path(path: &str) -> Vec<PathBuf> {
    path.split(':')
        .filter(|s| !s.is_empty())
        .map(PathBuf::from)
        .collect()
}

/// Merge per SPEC §5.2 step 3: `base` (static list) first, `extra` (probe
/// entries) appended, deduped preserving first-occurrence order.
pub fn merge_entries(base: Vec<PathBuf>, extra: impl IntoIterator<Item = PathBuf>) -> Vec<PathBuf> {
    let mut merged: Vec<PathBuf> = Vec::new();
    for entry in base.into_iter().chain(extra) {
        if !merged.contains(&entry) {
            merged.push(entry);
        }
    }
    merged
}

/// Runs the login-shell PATH probe with the default `$SHELL` (fallback
/// `/bin/zsh`), 5s timeout and 64KiB output cap. Best-effort: every failure
/// mode returns `EnvCaptureFailed` (non-fatal, reported).
pub async fn probe_login_shell_path() -> Result<Vec<PathBuf>, PmError> {
    let shell = std::env::var("SHELL").unwrap_or_else(|_| "/bin/zsh".to_string());
    probe_login_shell_path_with(Path::new(&shell), PROBE_TIMEOUT).await
}

/// Probe against a specific shell binary with a specific timeout (seam for the
/// `#[ignore]` real-process test).
pub async fn probe_login_shell_path_with(
    shell: &Path,
    timeout: Duration,
) -> Result<Vec<PathBuf>, PmError> {
    let mut cmd = tokio::process::Command::new(shell);
    cmd.args(["-l", "-c", PROBE_SCRIPT])
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        // Own process group, like the op runner: profiles can background
        // helpers, and start_kill alone would only signal the shell itself,
        // orphaning those descendants to launchd on every launch/Re-detect.
        .process_group(0)
        .kill_on_drop(true);
    let mut child = cmd.spawn().map_err(|e| PmError::EnvCaptureFailed {
        detail: format!("failed to spawn {}: {e}", shell.display()),
    })?;
    // process_group(0): the child leads a fresh group whose pgid is its pid.
    let pgid = Pid::from_raw(child.id().map(|id| id as i32).unwrap_or(0));
    let kill_probe = |child: &mut tokio::process::Child| {
        if pgid.as_raw() > 0 {
            let _ = killpg(pgid, Signal::SIGKILL);
        }
        let _ = child.start_kill();
    };
    let mut stdout = child.stdout.take().expect("stdout piped");

    let read = async {
        let mut out = Vec::new();
        let mut limited = (&mut stdout).take(PROBE_OUTPUT_CAP);
        limited.read_to_end(&mut out).await.map(|_| out)
    };
    let bytes = match tokio::time::timeout(timeout, read).await {
        Ok(Ok(bytes)) => bytes,
        Ok(Err(e)) => {
            kill_probe(&mut child);
            return Err(PmError::EnvCaptureFailed {
                detail: format!("probe read failed: {e}"),
            });
        }
        Err(_) => {
            kill_probe(&mut child);
            return Err(PmError::EnvCaptureFailed {
                detail: format!("login-shell probe timed out after {}s", timeout.as_secs()),
            });
        }
    };
    // Reap (or kill a shell still chattering past the cap) without blocking
    // startup; kill_on_drop covers the pathological case. killpg reaps any
    // profile-spawned descendants along with the shell.
    kill_probe(&mut child);
    let _ = tokio::time::timeout(Duration::from_millis(500), child.wait()).await;

    let text = String::from_utf8_lossy(&bytes);
    let path = extract_probe_path(&text).ok_or_else(|| PmError::EnvCaptureFailed {
        detail: "sentinels not found in login-shell output".to_string(),
    })?;
    let entries = split_path(&path);
    if entries.is_empty() {
        return Err(PmError::EnvCaptureFailed {
            detail: "login-shell PATH was empty".to_string(),
        });
    }
    Ok(entries)
}

/// The constructed search environment (SPEC §5.2 step 4), kept in AppState.
#[derive(Debug, Clone, PartialEq)]
pub struct ToolEnv {
    /// `entries` joined with `:` — the PATH children receive.
    pub path: String,
    pub entries: Vec<PathBuf>,
    pub home: PathBuf,
    pub source: PathSource,
    /// Filesystem root that ABSOLUTE fixed-path detection candidates
    /// (`ManagerAdapter::detection_candidates`) resolve under. Always `/` in
    /// production; hermetic detection tests re-root it into a tempdir so a
    /// manager installed on the host machine (e.g. a real
    /// `/opt/homebrew/bin/mas`) can never leak into a sandboxed detection.
    pub candidate_root: PathBuf,
}

impl ToolEnv {
    /// Builds the ToolEnv with the real home dir and the real login-shell
    /// probe. INFO-logs the result (the single record that answers "it can't
    /// find brew"); WARNs on probe failure.
    pub async fn build() -> ToolEnv {
        let home = dirs::home_dir().unwrap_or_else(|| PathBuf::from("/"));
        let probe = probe_login_shell_path().await;
        ToolEnv::from_probe(home, probe)
    }

    /// Pure assembly from a probe outcome: `Ok` entries are merged after the
    /// static list (`source: merged`); `Err` falls back to the static list
    /// alone (`source: staticFallback`).
    pub fn from_probe(home: PathBuf, probe: Result<Vec<PathBuf>, PmError>) -> ToolEnv {
        let base = static_entries(&home);
        let (entries, source) = match probe {
            Ok(extra) => (merge_entries(base, extra), PathSource::Merged),
            Err(e) => {
                tracing::warn!(error = %e, "login-shell PATH probe failed; using static fallback");
                (base, PathSource::StaticFallback)
            }
        };
        let env = ToolEnv::from_entries(home, entries, source);
        tracing::info!(
            path = %env.path,
            source = ?env.source,
            home = %env.home.display(),
            "ToolEnv constructed"
        );
        env
    }

    /// Direct constructor (tests, U4 detection tests).
    pub fn from_entries(home: PathBuf, entries: Vec<PathBuf>, source: PathSource) -> ToolEnv {
        let path = entries
            .iter()
            .map(|p| p.to_string_lossy().into_owned())
            .collect::<Vec<_>>()
            .join(":");
        ToolEnv {
            path,
            entries,
            home,
            source,
            candidate_root: PathBuf::from("/"),
        }
    }

    /// Test seam: re-roots the fixed-path candidate fallback (detection) into
    /// a sandbox directory. Production code never calls this.
    #[cfg(any(test, feature = "test-util"))]
    pub fn with_candidate_root(mut self, root: impl Into<PathBuf>) -> ToolEnv {
        self.candidate_root = root.into();
        self
    }

    /// Resolves `binary` to an absolute path on OUR search path (never the
    /// inherited PATH). `None` when not found.
    pub fn which(&self, binary: &str) -> Option<PathBuf> {
        which::which_in(binary, Some(&self.path), &self.home).ok()
    }

    /// Like [`Self::which`] but errors with the searched entries
    /// (`PmError::ToolNotFound`).
    pub fn require(&self, binary: &str) -> Result<PathBuf, PmError> {
        self.which(binary).ok_or_else(|| PmError::ToolNotFound {
            tool: binary.to_string(),
            searched: self
                .entries
                .iter()
                .map(|p| p.to_string_lossy().into_owned())
                .collect(),
        })
    }

    /// The constructed child environment (SPEC §5.2 step 6) — children NEVER
    /// inherit ours. `HOMEBREW_NO_AUTO_UPDATE=1` is deliberately absent here:
    /// the brew adapter adds it per command (every brew command EXCEPT the
    /// explicit `brew update` spec) via `PlannedCommand.extra_env`.
    pub fn child_env(&self) -> Vec<(String, String)> {
        let mut env: Vec<(String, String)> = vec![
            ("PATH".into(), self.path.clone()),
            ("HOME".into(), self.home.to_string_lossy().into_owned()),
        ];
        for key in ["USER", "LOGNAME", "TMPDIR"] {
            if let Ok(v) = std::env::var(key) {
                env.push((key.into(), v));
            }
        }
        env.extend(
            [
                ("LANG", "en_US.UTF-8"),
                ("NO_COLOR", "1"),
                ("TERM", "dumb"),
                ("GIT_TERMINAL_PROMPT", "0"),
                ("HOMEBREW_COLOR", "0"),
                ("HOMEBREW_NO_EMOJI", "1"),
                ("HOMEBREW_NO_ENV_HINTS", "1"),
                ("HOMEBREW_NO_INSTALL_CLEANUP", "1"),
            ]
            .into_iter()
            .map(|(k, v)| (k.to_string(), v.to_string())),
        );
        env
    }

    /// Wire form for the Environment Report.
    pub fn env_info(&self) -> EnvInfo {
        EnvInfo {
            path: self.path.clone(),
            entries: self
                .entries
                .iter()
                .map(|p| p.to_string_lossy().into_owned())
                .collect(),
            source: self.source,
            home: self.home.to_string_lossy().into_owned(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn home() -> PathBuf {
        PathBuf::from("/Users/testuser")
    }

    #[test]
    fn sentinel_extraction_ignores_profile_noise() {
        let out = "Last login: Tue Jul 22 on ttys001\n\
                   🚀 welcome banner from ~/.zprofile\n\
                   __PM_S__\n\
                   /a:/b:/c\n\
                   __PM_E__\n\
                   goodbye from ~/.zlogout\n";
        assert_eq!(extract_probe_path(out), Some("/a:/b:/c".to_string()));
    }

    #[test]
    fn sentinel_extraction_uses_last_start_marker() {
        // A profile that echoes the start sentinel itself must not confuse us.
        let out = "__PM_S__\nnoise pretending to be a path\n__PM_S__\n/real:/path\n__PM_E__\n";
        assert_eq!(extract_probe_path(out), Some("/real:/path".to_string()));
    }

    #[test]
    fn sentinel_extraction_missing_markers_is_none() {
        assert_eq!(extract_probe_path("no sentinels here\n"), None);
        assert_eq!(extract_probe_path("__PM_S__\n/half/open\n"), None);
        assert_eq!(extract_probe_path(""), None);
    }

    #[test]
    fn split_path_drops_empty_segments() {
        assert_eq!(
            split_path("/a::/b:"),
            vec![PathBuf::from("/a"), PathBuf::from("/b")]
        );
    }

    #[test]
    fn probe_failure_falls_back_to_static_with_shims_before_homebrew() {
        let env = ToolEnv::from_probe(
            home(),
            Err(PmError::EnvCaptureFailed {
                detail: "probe timed out".into(),
            }),
        );
        assert_eq!(env.source, PathSource::StaticFallback);
        // Full static order per SPEC §5.2 — shims FIRST so uv/npm resolve to
        // mise shims, never to brew binaries.
        let expected: Vec<PathBuf> = vec![
            "/Users/testuser/.local/share/mise/shims".into(),
            "/opt/homebrew/bin".into(),
            "/opt/homebrew/sbin".into(),
            "/Users/testuser/.cargo/bin".into(),
            "/Users/testuser/.local/bin".into(),
            "/usr/local/bin".into(),
            "/usr/bin".into(),
            "/bin".into(),
            "/usr/sbin".into(),
            "/sbin".into(),
        ];
        assert_eq!(env.entries, expected);
        assert!(env
            .path
            .starts_with("/Users/testuser/.local/share/mise/shims:/opt/homebrew/bin:"));
        assert!(env.path.ends_with(":/usr/sbin:/sbin"));
    }

    #[test]
    fn merge_dedupes_preserving_order() {
        let base = static_entries(&home());
        let probe: Vec<PathBuf> = vec![
            "/usr/bin".into(),          // dup of a static entry — not re-added
            "/custom/bin".into(),       // new — appended
            "/opt/homebrew/bin".into(), // dup — not re-added
            "/custom/bin".into(),       // dup within probe — appended once
        ];
        let merged = merge_entries(base.clone(), probe);
        assert_eq!(&merged[..base.len()], &base[..]);
        assert_eq!(merged[base.len()], PathBuf::from("/custom/bin"));
        assert_eq!(merged.len(), base.len() + 1);
    }

    #[test]
    fn probe_success_merges_with_source_merged() {
        let env = ToolEnv::from_probe(home(), Ok(vec![PathBuf::from("/custom/bin")]));
        assert_eq!(env.source, PathSource::Merged);
        assert_eq!(env.entries[0], home().join(".local/share/mise/shims"));
        assert_eq!(
            env.entries.last(),
            Some(&PathBuf::from("/custom/bin")),
            "probe entries append after the static list"
        );
    }

    #[test]
    fn child_env_is_constructed_not_inherited() {
        let env = ToolEnv::from_probe(home(), Ok(vec![]));
        let child = env.child_env();
        let get = |k: &str| {
            child
                .iter()
                .find(|(key, _)| key == k)
                .map(|(_, v)| v.as_str())
        };
        assert_eq!(get("PATH"), Some(env.path.as_str()));
        assert_eq!(get("HOME"), Some("/Users/testuser"));
        assert_eq!(get("LANG"), Some("en_US.UTF-8"));
        assert_eq!(get("NO_COLOR"), Some("1"));
        assert_eq!(get("TERM"), Some("dumb"));
        assert_eq!(get("GIT_TERMINAL_PROMPT"), Some("0"));
        assert_eq!(get("HOMEBREW_COLOR"), Some("0"));
        assert_eq!(get("HOMEBREW_NO_EMOJI"), Some("1"));
        assert_eq!(get("HOMEBREW_NO_ENV_HINTS"), Some("1"));
        assert_eq!(get("HOMEBREW_NO_INSTALL_CLEANUP"), Some("1"));
        // Added per brew command by the adapter, NOT here (brew update must
        // not carry it).
        assert_eq!(get("HOMEBREW_NO_AUTO_UPDATE"), None);
    }

    #[test]
    fn which_resolves_only_on_our_entries_and_require_reports_searched() {
        use std::os::unix::fs::PermissionsExt;
        let dir = tempfile::tempdir().unwrap();
        let tool = dir.path().join("pm-test-tool");
        std::fs::write(&tool, "#!/bin/sh\nexit 0\n").unwrap();
        std::fs::set_permissions(&tool, std::fs::Permissions::from_mode(0o755)).unwrap();

        let env = ToolEnv::from_entries(
            home(),
            vec![dir.path().to_path_buf()],
            PathSource::StaticFallback,
        );
        assert_eq!(env.which("pm-test-tool"), Some(tool));
        let err = env.require("pm-definitely-missing").unwrap_err();
        match err {
            PmError::ToolNotFound { tool, searched } => {
                assert_eq!(tool, "pm-definitely-missing");
                assert_eq!(searched, vec![dir.path().to_string_lossy().into_owned()]);
            }
            other => panic!("expected ToolNotFound, got {other:?}"),
        }
    }

    #[test]
    fn env_info_mirrors_the_wire_shape() {
        let env = ToolEnv::from_probe(home(), Ok(vec![PathBuf::from("/custom/bin")]));
        let info = env.env_info();
        assert_eq!(info.path, env.path);
        assert_eq!(info.source, PathSource::Merged);
        assert_eq!(info.home, "/Users/testuser");
        assert_eq!(info.entries.len(), env.entries.len());
    }

    /// Regression (default-run, /bin/sh only): the probe runs in its OWN
    /// process group and kills the WHOLE group on timeout — a login profile
    /// that backgrounds a child must not leak it to launchd on every
    /// launch/Re-detect. Previously only the direct shell was killed.
    #[tokio::test(flavor = "multi_thread")]
    async fn probe_timeout_kills_profile_spawned_descendants() {
        use std::os::unix::fs::PermissionsExt;
        let dir = tempfile::tempdir().unwrap();
        let pid_file = dir.path().join("bg.pid");
        // Stand-in for a noisy login shell: backgrounds a long-lived child
        // (recording its pid), then hangs without printing sentinels.
        let shell = dir.path().join("noisy-shell.sh");
        std::fs::write(
            &shell,
            format!(
                "#!/bin/sh\nsleep 30 &\necho $! > {}\nwait\n",
                pid_file.display()
            ),
        )
        .unwrap();
        std::fs::set_permissions(&shell, std::fs::Permissions::from_mode(0o755)).unwrap();

        let err = probe_login_shell_path_with(&shell, Duration::from_millis(300))
            .await
            .unwrap_err();
        assert!(matches!(err, PmError::EnvCaptureFailed { .. }));

        let pid: i32 = std::fs::read_to_string(&pid_file)
            .expect("script wrote the background pid")
            .trim()
            .parse()
            .unwrap();
        // The group SIGKILL reaps the backgrounded descendant too; once init
        // re-parents and reaps it, `kill(pid, 0)` reports ESRCH.
        let deadline = std::time::Instant::now() + Duration::from_secs(5);
        loop {
            if nix::sys::signal::kill(nix::unistd::Pid::from_raw(pid), None).is_err() {
                break; // gone — the whole group was killed
            }
            assert!(
                std::time::Instant::now() < deadline,
                "backgrounded descendant (pid {pid}) survived the probe kill"
            );
            tokio::time::sleep(Duration::from_millis(25)).await;
        }
    }

    /// Real login-shell probe on this machine — developer-run only.
    #[tokio::test]
    #[ignore]
    async fn real_login_shell_probe_returns_entries() {
        let entries = probe_login_shell_path().await.expect("probe should work");
        assert!(!entries.is_empty());
        // Every real login shell on macOS ends up with /usr/bin somewhere.
        assert!(entries.contains(&PathBuf::from("/usr/bin")));
    }

    /// Sentinel probe against a shell whose "profile" is noisy — real process.
    #[tokio::test]
    #[ignore]
    async fn real_probe_times_out_against_sleeping_shell() {
        // /bin/sh -l -c with a script that never prints sentinels *and* hangs:
        // use a wrapper that ignores our script. Easiest hang: /bin/sleep is
        // not a shell, so `-l -c script` is ignored and it sleeps.
        let err = probe_login_shell_path_with(Path::new("/bin/sleep"), Duration::from_millis(300))
            .await
            .unwrap_err();
        match err {
            PmError::EnvCaptureFailed { .. } => {}
            other => panic!("expected EnvCaptureFailed, got {other:?}"),
        }
    }
}
