//! Logging init (SPEC §6): tracing JSON daily-rolling app log + dev stderr
//! layer, `PACK_MANAGER_LOG` > Settings logLevel > default filter precedence,
//! live reload handle, and startup prune (app logs >14d; transcripts >90d /
//! newest 200).
//!
//! All writers are non-blocking — a full disk drops log lines, never hangs an
//! upgrade.

use std::path::{Path, PathBuf};

use time::{Date, Month, OffsetDateTime};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::registry::Registry;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{fmt, reload, EnvFilter};

use crate::settings::{LogLevel, Settings};

/// EnvFilter override variable (EnvFilter syntax), highest precedence.
pub const ENV_VAR: &str = "PACK_MANAGER_LOG";
/// Default directive (SPEC §6.3).
pub const DEFAULT_DIRECTIVE: &str = "info,pack_manager_lib=debug";
/// App-log file prefix; `tracing-appender` daily rolling appends `.<YYYY-MM-DD>`.
pub const APP_LOG_PREFIX: &str = "pack-manager.log";

pub const APP_LOG_RETENTION_DAYS: i64 = 14;
pub const TRANSCRIPT_RETENTION_DAYS: i64 = 90;
pub const TRANSCRIPT_MAX_FILES: usize = 200;

/// `~/Library/Logs/Pack-Manager`
pub fn logs_dir() -> PathBuf {
    dirs::home_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("Library/Logs")
        .join("Pack-Manager")
}

/// `~/Library/Logs/Pack-Manager/operations`
pub fn operations_dir() -> PathBuf {
    logs_dir().join("operations")
}

fn level_str(level: LogLevel) -> &'static str {
    match level {
        LogLevel::Error => "error",
        LogLevel::Warn => "warn",
        LogLevel::Info => "info",
        LogLevel::Debug => "debug",
        LogLevel::Trace => "trace",
    }
}

/// Settings level → directive: externals stay at `info`, our crate gets the
/// chosen level (F11: logLevel is "for own crate").
pub fn directive_for_level(level: LogLevel) -> String {
    format!("info,pack_manager_lib={}", level_str(level))
}

/// Precedence: env `PACK_MANAGER_LOG` (when set and non-empty) > Settings
/// logLevel > default. Returns `(directive, env_override_active)`.
pub fn resolve_directive(env_value: Option<&str>, settings_level: LogLevel) -> (String, bool) {
    match env_value {
        Some(v) if !v.trim().is_empty() => (v.trim().to_string(), true),
        _ => (directive_for_level(settings_level), false),
    }
}

fn parse_filter(directive: &str) -> EnvFilter {
    EnvFilter::try_new(directive).unwrap_or_else(|_| {
        EnvFilter::new(DEFAULT_DIRECTIVE) // invalid syntax degrades to default
    })
}

type FilterHandle = reload::Handle<EnvFilter, Registry>;

/// Keeps the reload handle and the non-blocking worker guard alive for the app
/// lifetime (U5 stores it; dropping it flushes the writer).
pub struct LoggingHandle {
    handle: FilterHandle,
    env_override: bool,
    _guard: tracing_appender::non_blocking::WorkerGuard,
}

impl LoggingHandle {
    /// Applies a Settings logLevel change live. Returns `false` (no-op) while
    /// the `PACK_MANAGER_LOG` env override is active or if reload fails.
    pub fn apply_settings_level(&self, level: LogLevel) -> bool {
        if self.env_override {
            return false;
        }
        self.handle
            .reload(parse_filter(&directive_for_level(level)))
            .is_ok()
    }

    pub fn env_override_active(&self) -> bool {
        self.env_override
    }
}

/// Initializes tracing with the real logs dir and real env var. Call FIRST in
/// startup (SPEC §5.12), then [`prune_at_startup`].
pub fn init(settings: &Settings) -> LoggingHandle {
    let env_value = std::env::var(ENV_VAR).ok();
    init_at(settings, &logs_dir(), env_value.as_deref())
}

/// Parametrized init (seam for tests): explicit dir + env override value.
pub fn init_at(settings: &Settings, dir: &Path, env_value: Option<&str>) -> LoggingHandle {
    let _ = std::fs::create_dir_all(dir);
    let (directive, env_override) = resolve_directive(env_value, settings.log_level);

    let (filter_layer, handle) = reload::Layer::new(parse_filter(&directive));
    let appender = tracing_appender::rolling::daily(dir, APP_LOG_PREFIX);
    let (writer, guard) = tracing_appender::non_blocking(appender);
    let file_layer = fmt::layer().json().with_ansi(false).with_writer(writer);
    // Dev console: pretty stderr in debug builds only (SPEC §6.1).
    let dev_layer = if cfg!(debug_assertions) {
        Some(fmt::layer().pretty().with_writer(std::io::stderr))
    } else {
        None
    };

    // `try_init` so repeated calls (tests) are harmless; the first wins.
    let _ = tracing_subscriber::registry()
        .with(filter_layer)
        .with(file_layer)
        .with(dev_layer)
        .try_init();

    tracing::info!(
        directive = %directive,
        env_override,
        dir = %dir.display(),
        "logging initialized"
    );
    LoggingHandle {
        handle,
        env_override,
        _guard: guard,
    }
}

// ---------------------------------------------------------------------------
// Startup prune
// ---------------------------------------------------------------------------

/// Parses a strict `YYYY-MM-DD`.
fn parse_ymd(s: &str) -> Option<Date> {
    let b = s.as_bytes();
    if b.len() != 10 || b[4] != b'-' || b[7] != b'-' {
        return None;
    }
    let y: i32 = s[0..4].parse().ok()?;
    let m: u8 = s[5..7].parse().ok()?;
    let d: u8 = s[8..10].parse().ok()?;
    Date::from_calendar_date(y, Month::try_from(m).ok()?, d).ok()
}

/// Date from `pack-manager.log.<YYYY-MM-DD>`; `None` for anything else.
fn app_log_date(file_name: &str) -> Option<Date> {
    let suffix = file_name.strip_prefix(APP_LOG_PREFIX)?.strip_prefix('.')?;
    parse_ymd(suffix)
}

/// Date from `<YYYY-MM-DDTHH-mm-ss>_<opId8>_<manager>_<kind>.log`.
fn transcript_date(file_name: &str) -> Option<Date> {
    if !file_name.ends_with(".log") || file_name.len() < 11 || file_name.as_bytes()[10] != b'T' {
        return None;
    }
    parse_ymd(&file_name[0..10])
}

/// Removes app logs older than [`APP_LOG_RETENTION_DAYS`]. Only files whose
/// name matches the rolling pattern are ever touched. Returns removed count.
pub fn prune_app_logs(dir: &Path, today: Date) -> usize {
    let Some(cutoff) = today.checked_sub(time::Duration::days(APP_LOG_RETENTION_DAYS)) else {
        return 0;
    };
    let Ok(entries) = std::fs::read_dir(dir) else {
        return 0;
    };
    let mut removed = 0;
    for entry in entries.flatten() {
        let name = entry.file_name().to_string_lossy().into_owned();
        if let Some(date) = app_log_date(&name) {
            if date < cutoff && std::fs::remove_file(entry.path()).is_ok() {
                removed += 1;
            }
        }
    }
    removed
}

/// Removes transcripts older than [`TRANSCRIPT_RETENTION_DAYS`], then trims to
/// the newest [`TRANSCRIPT_MAX_FILES`] (SPEC §6.1). Only files matching the
/// transcript naming pattern are ever touched. Returns removed count.
pub fn prune_transcripts(dir: &Path, today: Date) -> usize {
    let Some(cutoff) = today.checked_sub(time::Duration::days(TRANSCRIPT_RETENTION_DAYS)) else {
        return 0;
    };
    let Ok(entries) = std::fs::read_dir(dir) else {
        return 0;
    };
    let mut removed = 0;
    let mut kept: Vec<(String, PathBuf)> = Vec::new();
    for entry in entries.flatten() {
        let name = entry.file_name().to_string_lossy().into_owned();
        if let Some(date) = transcript_date(&name) {
            if date < cutoff {
                if std::fs::remove_file(entry.path()).is_ok() {
                    removed += 1;
                }
            } else {
                kept.push((name, entry.path()));
            }
        }
    }
    // Timestamp prefixes sort lexicographically — newest first after reverse.
    kept.sort_by(|a, b| b.0.cmp(&a.0));
    for (_, path) in kept.into_iter().skip(TRANSCRIPT_MAX_FILES) {
        if std::fs::remove_file(path).is_ok() {
            removed += 1;
        }
    }
    removed
}

/// Startup prune against the real directories; INFO-logs the stats.
pub fn prune_at_startup() -> (usize, usize) {
    let today = OffsetDateTime::now_utc().date();
    let app_logs_removed = prune_app_logs(&logs_dir(), today);
    let transcripts_removed = prune_transcripts(&operations_dir(), today);
    tracing::info!(app_logs_removed, transcripts_removed, "startup log prune");
    (app_logs_removed, transcripts_removed)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn d(y: i32, m: u8, day: u8) -> Date {
        Date::from_calendar_date(y, Month::try_from(m).unwrap(), day).unwrap()
    }

    #[test]
    fn directive_for_level_targets_own_crate() {
        assert_eq!(directive_for_level(LogLevel::Debug), DEFAULT_DIRECTIVE);
        assert_eq!(
            directive_for_level(LogLevel::Trace),
            "info,pack_manager_lib=trace"
        );
        assert_eq!(
            directive_for_level(LogLevel::Warn),
            "info,pack_manager_lib=warn"
        );
    }

    #[test]
    fn resolve_directive_env_beats_settings_beats_default() {
        // env wins
        let (dir, over) = resolve_directive(Some("trace"), LogLevel::Warn);
        assert_eq!(dir, "trace");
        assert!(over);
        // empty env is ignored
        let (dir, over) = resolve_directive(Some("  "), LogLevel::Warn);
        assert_eq!(dir, "info,pack_manager_lib=warn");
        assert!(!over);
        // no env → settings level
        let (dir, over) = resolve_directive(None, LogLevel::Debug);
        assert_eq!(dir, DEFAULT_DIRECTIVE);
        assert!(!over);
    }

    #[test]
    fn filename_date_parsers_are_strict() {
        assert_eq!(
            app_log_date("pack-manager.log.2026-07-22"),
            Some(d(2026, 7, 22))
        );
        assert_eq!(app_log_date("pack-manager.log"), None);
        assert_eq!(app_log_date("pack-manager.log.garbage"), None);
        assert_eq!(app_log_date("other.log.2026-07-22"), None);

        assert_eq!(
            transcript_date("2026-07-22T14-03-11_01981f2e_npm_upgrade.log"),
            Some(d(2026, 7, 22))
        );
        assert_eq!(transcript_date("2026-07-22_missing_t.log"), None);
        assert_eq!(transcript_date("notes.txt"), None);
        assert_eq!(transcript_date("2026-13-99T00-00-00_x_y_z.log"), None);
    }

    #[test]
    fn prune_app_logs_removes_only_old_matching_files() {
        let dir = tempfile::tempdir().unwrap();
        let touch = |name: &str| std::fs::write(dir.path().join(name), "x").unwrap();
        touch("pack-manager.log.2026-07-01"); // 21 days old → pruned
        touch("pack-manager.log.2026-07-21"); // fresh → kept
        touch("unrelated.txt"); // never touched
        touch("pack-manager.log.not-a-date"); // never touched

        let removed = prune_app_logs(dir.path(), d(2026, 7, 22));
        assert_eq!(removed, 1);
        assert!(!dir.path().join("pack-manager.log.2026-07-01").exists());
        assert!(dir.path().join("pack-manager.log.2026-07-21").exists());
        assert!(dir.path().join("unrelated.txt").exists());
        assert!(dir.path().join("pack-manager.log.not-a-date").exists());
    }

    #[test]
    fn prune_transcripts_by_age_then_newest_200() {
        let dir = tempfile::tempdir().unwrap();
        let touch = |name: &str| std::fs::write(dir.path().join(name), "x").unwrap();

        // One ancient transcript (>90 days before 2026-07-22 → cutoff 2026-04-23).
        touch("2026-04-01T10-00-00_deadbeef_brew_refresh.log");
        // 205 recent transcripts across two days.
        for i in 0..205 {
            let (day, idx) = if i < 100 { (20, i) } else { (21, i - 100) };
            touch(&format!(
                "2026-07-{day}T10-{:02}-{:02}_op{i:03}_npm_upgrade.log",
                idx / 60,
                idx % 60
            ));
        }
        // A non-matching file survives everything.
        touch("README.txt");

        let removed = prune_transcripts(dir.path(), d(2026, 7, 22));
        assert_eq!(removed, 1 + 5, "1 by age, 5 over the 200-file cap");
        let remaining: Vec<String> = std::fs::read_dir(dir.path())
            .unwrap()
            .flatten()
            .map(|e| e.file_name().to_string_lossy().into_owned())
            .collect();
        assert_eq!(remaining.len(), 200 + 1); // 200 transcripts + README.txt
        assert!(remaining.contains(&"README.txt".to_string()));
        // The oldest recent ones (earliest on 07-20) were the trimmed ones.
        assert!(!remaining.contains(&"2026-07-20T10-00-00_op000_npm_upgrade.log".to_string()));
        assert!(remaining.contains(&"2026-07-21T10-00-00_op100_npm_upgrade.log".to_string()));
    }

    #[test]
    fn init_at_writes_json_log_and_reload_respects_env_override() {
        let dir = tempfile::tempdir().unwrap();
        // env override active → settings changes are refused.
        let handle = init_at(&Settings::default(), dir.path(), Some("debug"));
        assert!(handle.env_override_active());
        assert!(!handle.apply_settings_level(LogLevel::Trace));

        tracing::info!(marker = "pm-logging-smoke", "smoke line");
        drop(handle); // WorkerGuard flush

        let mut found = false;
        for entry in std::fs::read_dir(dir.path()).unwrap().flatten() {
            let name = entry.file_name().to_string_lossy().into_owned();
            if name.starts_with(APP_LOG_PREFIX) {
                let body = std::fs::read_to_string(entry.path()).unwrap();
                if body.contains("pm-logging-smoke") {
                    found = true;
                }
            }
        }
        assert!(found, "JSON log file written under the rolling prefix");
    }
}
