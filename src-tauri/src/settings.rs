//! Settings (SPEC F11): struct + defaults, load/save at
//! `~/Library/Application Support/Pack-Manager/settings.json`, patch merge for
//! `set_settings({ patch })`. Paths are parameters for testability; the default
//! path is derived from `dirs::config_dir()`.

use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::error::PmError;

pub const APP_DIR_NAME: &str = "Pack-Manager";
pub const SETTINGS_FILE_NAME: &str = "settings.json";

/// `'error'|'warn'|'info'|'debug'|'trace'`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LogLevel {
    Error,
    Warn,
    Info,
    Debug,
    Trace,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct Settings {
    pub run_brew_update_on_refresh: bool,
    pub auto_refresh_on_launch: bool,
    pub stall_after_secs: u64,
    pub upgrade_hard_cap_mins: u64,
    pub log_level: LogLevel,
    pub auto_open_drawer: bool,
    pub include_greedy_by_default: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            run_brew_update_on_refresh: true,
            auto_refresh_on_launch: true,
            stall_after_secs: 120,
            upgrade_hard_cap_mins: 30,
            log_level: LogLevel::Debug, // 'debug' for our own crate (SPEC F11)
            auto_open_drawer: true,
            include_greedy_by_default: false,
        }
    }
}

/// Mirror of TS `Partial<Settings>` — every field optional; only present fields
/// are applied by [`Settings::apply_patch`].
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SettingsPatch {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_brew_update_on_refresh: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_refresh_on_launch: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stall_after_secs: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upgrade_hard_cap_mins: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_level: Option<LogLevel>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_open_drawer: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_greedy_by_default: Option<bool>,
}

impl Settings {
    /// `~/Library/Application Support/Pack-Manager`
    pub fn app_support_dir() -> PathBuf {
        dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join(APP_DIR_NAME)
    }

    /// `~/Library/Application Support/Pack-Manager/settings.json`
    pub fn default_path() -> PathBuf {
        Self::app_support_dir().join(SETTINGS_FILE_NAME)
    }

    /// Loads settings from `path`. Missing file, unreadable file, or corrupt
    /// JSON all yield defaults (never fatal); a present-but-unparseable file
    /// is WARN-logged so a silent revert of every preference is at least
    /// visible. A partial file fills missing fields from defaults via
    /// `#[serde(default)]`.
    pub fn load_from(path: &Path) -> Settings {
        let (settings, corrupt) = Self::load_from_reporting(path);
        if let Some(detail) = corrupt {
            tracing::warn!(
                path = %path.display(),
                %detail,
                "settings.json is corrupt; falling back to defaults"
            );
        }
        settings
    }

    /// Like [`Settings::load_from`], but returns the corruption detail to the
    /// caller instead of only logging it — the startup path loads settings
    /// BEFORE the logging subscriber exists, so it re-logs after init.
    pub fn load_from_reporting(path: &Path) -> (Settings, Option<String>) {
        match std::fs::read_to_string(path) {
            Ok(raw) => match serde_json::from_str(&raw) {
                Ok(settings) => (settings, None),
                Err(e) => (Settings::default(), Some(e.to_string())),
            },
            // Missing file is the normal first-launch state, not corruption.
            Err(_) => (Settings::default(), None),
        }
    }

    /// Saves pretty-printed JSON (+ trailing newline), creating parent dirs.
    /// Atomic: temp file + fsync + rename, so a crash or full disk mid-write
    /// can never leave a truncated settings.json (which would silently revert
    /// every preference to defaults on the next launch).
    pub fn save_to(&self, path: &Path) -> Result<(), PmError> {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let mut rendered = serde_json::to_string_pretty(self).map_err(|e| PmError::Internal {
            detail: format!("settings serialize: {e}"),
        })?;
        rendered.push('\n');
        let file_name = path
            .file_name()
            .map(|n| n.to_string_lossy().into_owned())
            .unwrap_or_else(|| SETTINGS_FILE_NAME.to_string());
        let tmp = path.with_file_name(format!("{file_name}.tmp"));
        let result = (|| -> std::io::Result<()> {
            use std::io::Write as _;
            let mut f = std::fs::File::create(&tmp)?;
            f.write_all(rendered.as_bytes())?;
            f.sync_all()?;
            std::fs::rename(&tmp, path)
        })();
        if let Err(e) = result {
            let _ = std::fs::remove_file(&tmp);
            return Err(e.into());
        }
        Ok(())
    }

    /// Applies every `Some` field of the patch.
    pub fn apply_patch(&mut self, patch: &SettingsPatch) {
        if let Some(v) = patch.run_brew_update_on_refresh {
            self.run_brew_update_on_refresh = v;
        }
        if let Some(v) = patch.auto_refresh_on_launch {
            self.auto_refresh_on_launch = v;
        }
        if let Some(v) = patch.stall_after_secs {
            self.stall_after_secs = v;
        }
        if let Some(v) = patch.upgrade_hard_cap_mins {
            self.upgrade_hard_cap_mins = v;
        }
        if let Some(v) = patch.log_level {
            self.log_level = v;
        }
        if let Some(v) = patch.auto_open_drawer {
            self.auto_open_drawer = v;
        }
        if let Some(v) = patch.include_greedy_by_default {
            self.include_greedy_by_default = v;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defaults_match_spec_f11() {
        let s = Settings::default();
        assert!(s.run_brew_update_on_refresh);
        assert!(s.auto_refresh_on_launch);
        assert_eq!(s.stall_after_secs, 120);
        assert_eq!(s.upgrade_hard_cap_mins, 30);
        assert_eq!(s.log_level, LogLevel::Debug);
        assert!(s.auto_open_drawer);
        assert!(!s.include_greedy_by_default);
    }

    #[test]
    fn save_then_load_round_trips() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("nested").join(SETTINGS_FILE_NAME);
        let s = Settings {
            stall_after_secs: 60,
            log_level: LogLevel::Trace,
            ..Settings::default()
        };
        s.save_to(&path).unwrap();
        assert_eq!(Settings::load_from(&path), s);
    }

    #[test]
    fn load_missing_or_corrupt_file_yields_defaults() {
        let dir = tempfile::tempdir().unwrap();
        let missing = dir.path().join("nope.json");
        assert_eq!(Settings::load_from(&missing), Settings::default());

        let corrupt = dir.path().join("corrupt.json");
        std::fs::write(&corrupt, "{ not json").unwrap();
        assert_eq!(Settings::load_from(&corrupt), Settings::default());
    }

    /// Regression: corruption must be DISTINGUISHABLE from a missing file so
    /// the caller can log it — silently defaulting hid the loss of every
    /// persisted preference.
    #[test]
    fn load_from_reporting_flags_corruption_but_not_missing_files() {
        let dir = tempfile::tempdir().unwrap();

        let (s, corrupt) = Settings::load_from_reporting(&dir.path().join("nope.json"));
        assert_eq!(s, Settings::default());
        assert!(corrupt.is_none(), "missing file is normal first-launch");

        let path = dir.path().join("corrupt.json");
        std::fs::write(&path, "{ not json").unwrap();
        let (s, corrupt) = Settings::load_from_reporting(&path);
        assert_eq!(s, Settings::default());
        assert!(
            corrupt.is_some(),
            "present-but-unparseable must be reported"
        );
    }

    /// Regression: save is atomic — when the rewrite cannot complete (here a
    /// read-only dir stands in for ENOSPC / crash mid-write), the existing
    /// settings.json must survive byte-identically instead of being truncated
    /// in place and silently reverting every preference on the next launch.
    #[test]
    fn failed_save_leaves_existing_settings_intact() {
        use std::os::unix::fs::PermissionsExt;
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join(SETTINGS_FILE_NAME);
        let original = Settings {
            stall_after_secs: 60,
            ..Settings::default()
        };
        original.save_to(&path).unwrap();
        let before = std::fs::read_to_string(&path).unwrap();
        assert!(
            !dir.path().join("settings.json.tmp").exists(),
            "temp file renamed away on success"
        );

        std::fs::set_permissions(dir.path(), std::fs::Permissions::from_mode(0o555)).unwrap();
        let result = Settings::default().save_to(&path);
        std::fs::set_permissions(dir.path(), std::fs::Permissions::from_mode(0o755)).unwrap();

        assert!(result.is_err(), "save must fail, not truncate in place");
        assert_eq!(std::fs::read_to_string(&path).unwrap(), before);
        assert_eq!(Settings::load_from(&path), original);
    }

    #[test]
    fn load_partial_file_fills_missing_fields_from_defaults() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("partial.json");
        std::fs::write(&path, r#"{ "stallAfterSecs": 45 }"#).unwrap();
        let s = Settings::load_from(&path);
        assert_eq!(s.stall_after_secs, 45);
        assert_eq!(s.upgrade_hard_cap_mins, 30);
        assert!(s.auto_open_drawer);
    }

    #[test]
    fn apply_patch_touches_only_present_fields() {
        let mut s = Settings::default();
        let patch = SettingsPatch {
            auto_open_drawer: Some(false),
            upgrade_hard_cap_mins: Some(45),
            ..SettingsPatch::default()
        };
        s.apply_patch(&patch);
        assert!(!s.auto_open_drawer);
        assert_eq!(s.upgrade_hard_cap_mins, 45);
        // Everything else untouched.
        assert!(s.run_brew_update_on_refresh);
        assert_eq!(s.stall_after_secs, 120);
        assert_eq!(s.log_level, LogLevel::Debug);
    }

    #[test]
    fn patch_deserializes_camel_case_partial_json() {
        let patch: SettingsPatch =
            serde_json::from_str(r#"{ "includeGreedyByDefault": true }"#).unwrap();
        assert_eq!(patch.include_greedy_by_default, Some(true));
        assert_eq!(patch.stall_after_secs, None);
    }
}
