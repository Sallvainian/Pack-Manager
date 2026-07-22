//! Diagnostics zip export (SPEC F9) — implemented by U5.
//!
//! `export_diagnostics` builds
//! `~/Desktop/Pack-Manager-diagnostics-<YYYYMMDD-HHmmss>.zip` containing:
//! `report.json` (app/OS version, arch, resolved search path + source, full
//! DetectionReport with evidence, settings, log filter), the last 3 app-log
//! files, the last 25 transcripts, and `operations.jsonl`. Only env vars we
//! set are ever logged — never the inherited environment.

use std::io::Write;
use std::path::{Path, PathBuf};

use serde::Serialize;
use time::OffsetDateTime;
use zip::write::SimpleFileOptions;
use zip::ZipWriter;

use crate::error::PmError;
use crate::ipc::{DetectionReport, EnvInfo};
use crate::settings::Settings;

pub const APP_LOGS_INCLUDED: usize = 3;
pub const TRANSCRIPTS_INCLUDED: usize = 25;

/// `report.json` body (camelCase like the wire types).
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DiagnosticsReport {
    pub app_version: String,
    pub os: String,
    pub arch: String,
    /// Resolved search path + source (Environment Report).
    pub env: EnvInfo,
    /// Full detection report with evidence strings; `None` before detection.
    pub detection: Option<DetectionReport>,
    pub settings: Settings,
    /// The active tracing filter directive.
    pub log_directive: String,
}

fn internal(detail: impl std::fmt::Display) -> PmError {
    PmError::Internal {
        detail: detail.to_string(),
    }
}

/// Files in `dir` whose name passes `keep`, sorted name-descending (both log
/// kinds carry sortable date prefixes/suffixes), truncated to `limit`.
///
/// Only REGULAR files qualify: `DirEntry::file_type()` does not follow
/// symlinks. The log/operations dirs are user-writable and this bundle is
/// exported for sharing — a symlink dropped there (e.g. by a package's
/// post-install script) pointing at `~/.ssh/id_rsa` must never have its
/// target's bytes embedded in the zip.
fn newest_files(dir: &Path, keep: impl Fn(&str) -> bool, limit: usize) -> Vec<(String, PathBuf)> {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return Vec::new();
    };
    let mut files: Vec<(String, PathBuf)> = entries
        .flatten()
        .filter_map(|e| {
            let name = e.file_name().to_string_lossy().into_owned();
            let is_regular = e.file_type().map(|t| t.is_file()).unwrap_or(false);
            (is_regular && keep(&name)).then(|| (name, e.path()))
        })
        .collect();
    files.sort_by(|a, b| b.0.cmp(&a.0));
    files.truncate(limit);
    files
}

/// `true` only for a REGULAR file at `path` itself (symlinks excluded —
/// `Path::is_file` would follow them).
fn is_regular_file(path: &Path) -> bool {
    std::fs::symlink_metadata(path)
        .map(|m| m.is_file())
        .unwrap_or(false)
}

/// Streams one source file into the zip. Belt-and-braces re-check that the
/// path is a regular file (not a symlink) at read time, and `io::copy`
/// instead of read-all so multi-hundred-MB transcripts never have to fit in
/// memory on the 16GB target machine.
fn add_file(
    zip: &mut ZipWriter<std::fs::File>,
    entry_name: &str,
    path: &Path,
) -> Result<(), PmError> {
    if !is_regular_file(path) {
        tracing::warn!(path = %path.display(), "diagnostics: skipping non-regular file");
        return Ok(());
    }
    let mut file = std::fs::File::open(path)?;
    zip.start_file(entry_name, SimpleFileOptions::default())
        .map_err(internal)?;
    std::io::copy(&mut file, zip)?;
    Ok(())
}

/// Zip file name for a timestamp: `Pack-Manager-diagnostics-YYYYMMDD-HHmmss.zip`.
pub fn zip_file_name(at: OffsetDateTime) -> String {
    format!(
        "Pack-Manager-diagnostics-{:04}{:02}{:02}-{:02}{:02}{:02}.zip",
        at.year(),
        u8::from(at.month()),
        at.day(),
        at.hour(),
        at.minute(),
        at.second(),
    )
}

/// Parametrized export (seam for tests). Returns the zip path.
pub fn export_to(
    dest_dir: &Path,
    report: &DiagnosticsReport,
    logs_dir: &Path,
    ops_dir: &Path,
    journal_path: &Path,
    at: OffsetDateTime,
) -> Result<PathBuf, PmError> {
    std::fs::create_dir_all(dest_dir)?;
    let zip_path = dest_dir.join(zip_file_name(at));
    let file = std::fs::File::create(&zip_path)?;
    let mut zip = ZipWriter::new(file);

    let mut body = serde_json::to_string_pretty(report).map_err(internal)?;
    body.push('\n');
    zip.start_file("report.json", SimpleFileOptions::default())
        .map_err(internal)?;
    zip.write_all(body.as_bytes())?;

    for (name, path) in newest_files(
        logs_dir,
        |n| n.starts_with(crate::logging::APP_LOG_PREFIX),
        APP_LOGS_INCLUDED,
    ) {
        add_file(&mut zip, &format!("logs/{name}"), &path)?;
    }
    for (name, path) in newest_files(ops_dir, |n| n.ends_with(".log"), TRANSCRIPTS_INCLUDED) {
        add_file(&mut zip, &format!("operations/{name}"), &path)?;
    }
    if is_regular_file(journal_path) {
        add_file(&mut zip, "operations.jsonl", journal_path)?;
    }

    zip.finish().map_err(internal)?;
    tracing::info!(zip = %zip_path.display(), "diagnostics exported");
    Ok(zip_path)
}

/// Export against the real locations: Desktop, `~/Library/Logs/Pack-Manager`,
/// its `operations/` subdir, and the Application Support journal.
pub fn export_default(report: &DiagnosticsReport) -> Result<PathBuf, PmError> {
    let desktop = dirs::desktop_dir().ok_or_else(|| internal("no Desktop directory"))?;
    export_to(
        &desktop,
        report,
        &crate::logging::logs_dir(),
        &crate::logging::operations_dir(),
        &Settings::app_support_dir().join("operations.jsonl"),
        OffsetDateTime::now_utc(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ipc::PathSource;
    use time::macros::datetime;

    fn report() -> DiagnosticsReport {
        DiagnosticsReport {
            app_version: "0.1.0".into(),
            os: std::env::consts::OS.into(),
            arch: std::env::consts::ARCH.into(),
            env: EnvInfo {
                path: "/a:/b".into(),
                entries: vec!["/a".into(), "/b".into()],
                source: PathSource::Merged,
                home: "/Users/testuser".into(),
            },
            detection: None,
            settings: Settings::default(),
            log_directive: "info,pack_manager_lib=debug".into(),
        }
    }

    #[test]
    fn export_bundles_report_logs_transcripts_and_journal() {
        let dir = tempfile::tempdir().unwrap();
        let logs = dir.path().join("logs");
        let ops = dir.path().join("ops");
        std::fs::create_dir_all(&logs).unwrap();
        std::fs::create_dir_all(&ops).unwrap();

        // 5 app logs → only the newest 3 ship.
        for day in 18..23 {
            std::fs::write(
                logs.join(format!("pack-manager.log.2026-07-{day}")),
                "log line\n",
            )
            .unwrap();
        }
        std::fs::write(logs.join("unrelated.txt"), "nope").unwrap();
        // 30 transcripts → only the newest 25 ship.
        for i in 0..30 {
            std::fs::write(
                ops.join(format!(
                    "2026-07-22T10-{:02}-00_op{i:02}_npm_upgrade.log",
                    i
                )),
                "transcript\n",
            )
            .unwrap();
        }
        let journal = dir.path().join("operations.jsonl");
        std::fs::write(&journal, "{}\n").unwrap();

        let dest = dir.path().join("desktop");
        let zip_path = export_to(
            &dest,
            &report(),
            &logs,
            &ops,
            &journal,
            datetime!(2026-07-22 14:03:11 UTC),
        )
        .unwrap();
        assert_eq!(
            zip_path.file_name().unwrap().to_str().unwrap(),
            "Pack-Manager-diagnostics-20260722-140311.zip"
        );

        let mut archive = zip::ZipArchive::new(std::fs::File::open(&zip_path).unwrap()).unwrap();
        let names: Vec<String> = archive.file_names().map(str::to_string).collect();
        assert!(names.contains(&"report.json".to_string()));
        assert!(names.contains(&"operations.jsonl".to_string()));
        let log_names: Vec<&String> = names.iter().filter(|n| n.starts_with("logs/")).collect();
        assert_eq!(log_names.len(), 3, "last 3 app logs");
        assert!(names.contains(&"logs/pack-manager.log.2026-07-22".to_string()));
        assert!(
            !names.contains(&"logs/pack-manager.log.2026-07-18".to_string()),
            "older logs dropped"
        );
        assert!(!names.iter().any(|n| n.contains("unrelated")));
        let transcript_names: Vec<&String> = names
            .iter()
            .filter(|n| n.starts_with("operations/"))
            .collect();
        assert_eq!(transcript_names.len(), 25, "last 25 transcripts");
        assert!(names.contains(&"operations/2026-07-22T10-29-00_op29_npm_upgrade.log".to_string()));
        assert!(!names.contains(&"operations/2026-07-22T10-00-00_op00_npm_upgrade.log".to_string()));

        // report.json round-trips as JSON with the expected fields.
        use std::io::Read;
        let mut body = String::new();
        archive
            .by_name("report.json")
            .unwrap()
            .read_to_string(&mut body)
            .unwrap();
        let v: serde_json::Value = serde_json::from_str(&body).unwrap();
        assert_eq!(v["appVersion"], "0.1.0");
        assert_eq!(v["env"]["source"], "merged");
        assert_eq!(v["logDirective"], "info,pack_manager_lib=debug");
    }

    /// Regression: a symlink dropped into the log/operations dirs (named to
    /// pass the filename filters) must NOT have its target's bytes embedded
    /// in the shareable zip — previously `is_file()`/`fs::read` followed it
    /// and exfiltrated arbitrary files (e.g. a private key) into the bundle.
    #[test]
    fn export_never_follows_symlinks_into_the_bundle() {
        let dir = tempfile::tempdir().unwrap();
        let logs = dir.path().join("logs");
        let ops = dir.path().join("ops");
        std::fs::create_dir_all(&logs).unwrap();
        std::fs::create_dir_all(&ops).unwrap();

        // The "secret" an attacker wants exfiltrated.
        let secret = dir.path().join("id_rsa");
        std::fs::write(&secret, "-----BEGIN OPENSSH PRIVATE KEY-----\nhunter2\n").unwrap();

        // One legitimate transcript, plus symlinks named to pass every filter.
        std::fs::write(
            ops.join("2026-07-22T10-00-00_op00_npm_upgrade.log"),
            "real\n",
        )
        .unwrap();
        std::os::unix::fs::symlink(&secret, ops.join("2026-07-22T23-59-59_evil_x_x.log")).unwrap();
        std::os::unix::fs::symlink(&secret, logs.join("pack-manager.log.2026-07-23")).unwrap();
        // Even the journal path itself could be swapped for a symlink.
        let journal = dir.path().join("operations.jsonl");
        std::os::unix::fs::symlink(&secret, &journal).unwrap();

        let zip_path = export_to(
            &dir.path().join("desktop"),
            &report(),
            &logs,
            &ops,
            &journal,
            datetime!(2026-07-22 14:03:11 UTC),
        )
        .unwrap();

        let mut archive = zip::ZipArchive::new(std::fs::File::open(&zip_path).unwrap()).unwrap();
        let names: Vec<String> = archive.file_names().map(str::to_string).collect();
        assert!(
            names.contains(&"operations/2026-07-22T10-00-00_op00_npm_upgrade.log".to_string()),
            "real transcript still ships"
        );
        assert!(
            !names.iter().any(|n| n.contains("evil")),
            "symlinked transcript excluded: {names:?}"
        );
        assert!(
            !names.contains(&"logs/pack-manager.log.2026-07-23".to_string()),
            "symlinked app log excluded"
        );
        assert!(
            !names.contains(&"operations.jsonl".to_string()),
            "symlinked journal excluded"
        );
        // No entry anywhere carries the secret's bytes.
        use std::io::Read;
        for i in 0..archive.len() {
            let mut f = archive.by_index(i).unwrap();
            let name = f.name().to_string();
            let mut body = String::new();
            f.read_to_string(&mut body).unwrap();
            assert!(!body.contains("hunter2"), "secret bytes leaked into {name}");
        }
    }

    /// The streaming copy must be byte-faithful (no truncation/duplication).
    #[test]
    fn export_streams_file_bodies_byte_identically() {
        let dir = tempfile::tempdir().unwrap();
        let logs = dir.path().join("logs");
        let ops = dir.path().join("ops");
        std::fs::create_dir_all(&logs).unwrap();
        std::fs::create_dir_all(&ops).unwrap();
        let body = "line\n".repeat(10_000);
        std::fs::write(ops.join("2026-07-22T10-00-00_op00_npm_upgrade.log"), &body).unwrap();

        let zip_path = export_to(
            &dir.path().join("desktop"),
            &report(),
            &logs,
            &ops,
            &dir.path().join("no-journal.jsonl"),
            datetime!(2026-07-22 14:03:11 UTC),
        )
        .unwrap();
        let mut archive = zip::ZipArchive::new(std::fs::File::open(&zip_path).unwrap()).unwrap();
        use std::io::Read;
        let mut round_tripped = String::new();
        archive
            .by_name("operations/2026-07-22T10-00-00_op00_npm_upgrade.log")
            .unwrap()
            .read_to_string(&mut round_tripped)
            .unwrap();
        assert_eq!(round_tripped, body);
    }

    #[test]
    fn export_tolerates_missing_sources() {
        let dir = tempfile::tempdir().unwrap();
        let zip_path = export_to(
            &dir.path().join("desktop"),
            &report(),
            &dir.path().join("no-logs"),
            &dir.path().join("no-ops"),
            &dir.path().join("no-journal.jsonl"),
            datetime!(2026-07-22 14:03:11 UTC),
        )
        .unwrap();
        let archive = zip::ZipArchive::new(std::fs::File::open(&zip_path).unwrap()).unwrap();
        let names: Vec<String> = archive.file_names().map(str::to_string).collect();
        assert_eq!(names, vec!["report.json".to_string()]);
    }
}
