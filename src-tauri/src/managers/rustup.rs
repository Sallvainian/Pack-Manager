//! rustup adapter — implemented by U4 (SPEC §5.4 command surface, §5.5 parsing
//! via `parse::rustup`).
//!
//! Refresh: `rustup toolchain list` (30s) → `rustup check` (120s). `rustup
//! check` is authoritative for versions and carries the `rustup` self line;
//! toolchains listed but missing from `check` are appended with unknown
//! versions. The merge is done directly here because check rows carry
//! `installed` values inventory lacks.

use std::collections::HashSet;
use std::time::Duration;

use crate::detect::DetectStatus;
use crate::error::PmError;
use crate::ipc::{ManagedBy, ManagerId, ManagerSnapshot, Package, SelfUpdateRoute};
use crate::managers::parse::rustup as parse_rustup;
use crate::managers::{ExitClass, ManagerAdapter, PackageId, PlanOptions, PlannedCommand, Timeout};
use crate::process::CommandOutput;
use crate::registry::{now_rfc3339, pkg_name};
use crate::settings::Settings;

pub struct RustupAdapter;

#[async_trait::async_trait]
impl ManagerAdapter for RustupAdapter {
    fn id(&self) -> ManagerId {
        ManagerId::Rustup
    }

    fn display_name(&self) -> &'static str {
        "rustup"
    }

    fn binary_name(&self) -> &'static str {
        "rustup"
    }

    fn detection_candidates(&self) -> &'static [&'static str] {
        &["~/.cargo/bin/rustup", "/opt/homebrew/bin/rustup"]
    }

    fn refresh_plan(&self, det: &DetectStatus, _settings: &Settings) -> Vec<PlannedCommand> {
        if matches!(det, DetectStatus::Absent { .. }) {
            return vec![];
        }
        vec![
            PlannedCommand {
                label: "rustup toolchain list",
                argv: vec!["toolchain".into(), "list".into()],
                timeout: Timeout::Absolute(Duration::from_secs(30)),
                extra_env: vec![],
                phase_label: None,
            },
            PlannedCommand {
                label: "rustup check",
                argv: vec!["check".into()],
                timeout: Timeout::Absolute(Duration::from_secs(120)),
                extra_env: vec![],
                phase_label: None,
            },
        ]
    }

    fn parse_refresh(&self, outputs: &[CommandOutput]) -> Result<ManagerSnapshot, PmError> {
        let [list, check_out] = outputs else {
            return Err(PmError::Internal {
                detail: format!(
                    "rustup parse_refresh expected 2 outputs, got {}",
                    outputs.len()
                ),
            });
        };
        let inventory = parse_rustup::parse_toolchain_list(&list.stdout);
        let check = parse_rustup::parse_check(&check_out.stdout)?;

        // check rows are authoritative (they carry versions); inventory-only
        // toolchains are appended with unknown versions.
        let known: HashSet<&str> = check.packages.iter().map(|p| p.name.as_str()).collect();
        let mut packages = check.packages.clone();
        for t in inventory {
            if !known.contains(t.name.as_str()) {
                packages.push(t);
            }
        }
        Ok(ManagerSnapshot {
            manager_id: ManagerId::Rustup,
            refreshed_at: now_rfc3339(),
            packages,
            // The `rustup` line of `rustup check` (SPEC §5.8).
            self_status: check.self_status,
            health: vec![],
        })
    }

    fn recovery_plan(&self, _failed: &PlannedCommand) -> Option<PlannedCommand> {
        None // SPEC §5.4: rustup has no recovery command.
    }

    fn parse_recovery(
        &self,
        failed: &PlannedCommand,
        _out: &CommandOutput,
    ) -> Result<ManagerSnapshot, PmError> {
        Err(PmError::Internal {
            detail: format!("rustup has no recovery for `{}`", failed.label),
        })
    }

    fn upgrade_plan(&self, pkgs: &[PackageId], _opts: &PlanOptions) -> Vec<PlannedCommand> {
        if pkgs.is_empty() {
            return vec![];
        }
        let mut argv = vec!["update".to_string()];
        argv.extend(pkgs.iter().map(|id| pkg_name(id).to_string()));
        vec![PlannedCommand {
            label: "rustup update",
            argv,
            timeout: super::brew::default_upgrade_timeout(),
            extra_env: vec![],
            phase_label: None,
        }]
    }

    fn self_update_route(
        &self,
        managed_by: ManagedBy,
        own_outdated_row: Option<&Package>,
    ) -> SelfUpdateRoute {
        // rustup reports itself in `rustup check` — in-band either way.
        if own_outdated_row.is_some() {
            return SelfUpdateRoute::InBand {
                command_preview: "rustup self update".into(),
                note: None,
            };
        }
        match managed_by {
            ManagedBy::Brew => SelfUpdateRoute::Routed {
                executor: ManagerId::Brew,
                command_preview: "brew upgrade rustup".into(),
                why: "rustup is managed by Homebrew".into(),
            },
            _ => SelfUpdateRoute::InBand {
                command_preview: "rustup self update".into(),
                note: None,
            },
        }
    }

    fn classify_exit(&self, _cmd: &PlannedCommand, out: &CommandOutput) -> ExitClass {
        match out.exit_code {
            Some(0) => ExitClass::Success,
            _ => ExitClass::Failure,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::process::fake::fixture;
    use std::path::PathBuf;

    fn present() -> DetectStatus {
        DetectStatus::Present {
            binary_path: PathBuf::from("/Users/testuser/.cargo/bin/rustup"),
            canonical_path: PathBuf::from("/Users/testuser/.cargo/bin/rustup"),
            version: Some("1.28.2".into()),
            managed_by: ManagedBy::Standalone,
            evidence: "resolved at ~/.cargo/bin/rustup — rustup's own tree".into(),
        }
    }

    fn out(stdout: &str) -> CommandOutput {
        CommandOutput {
            exit_code: Some(0),
            stdout: stdout.to_string(),
            stderr: String::new(),
            duration: Duration::ZERO,
        }
    }

    #[test]
    fn refresh_yields_toolchain_rows_and_rustup_self_status() {
        let outputs = vec![
            out(&fixture("rustup_toolchain_list_2026-07-22.txt")),
            out(&fixture("rustup_check_2026-07-21.txt")),
        ];
        let snapshot = RustupAdapter.parse_refresh(&outputs).expect("snapshot");
        assert_eq!(snapshot.packages.len(), 1);
        let tc = &snapshot.packages[0];
        assert_eq!(tc.id, "toolchain:stable-aarch64-apple-darwin");
        assert_eq!(tc.installed.as_deref(), Some("1.94.0"));
        assert_eq!(tc.latest.as_deref(), Some("1.97.1"));
        assert!(tc.outdated);

        // `rustup - Update available : 1.28.2 -> 1.29.0` → selfStatus.
        let s = snapshot.self_status.expect("rustup self line");
        assert_eq!(s.installed.as_deref(), Some("1.28.2"));
        assert_eq!(s.latest.as_deref(), Some("1.29.0"));
        assert!(s.update_available);
    }

    #[test]
    fn clean_check_with_both_colon_spacings_is_up_to_date() {
        let outputs = vec![
            out(&fixture("rustup_toolchain_list_2026-07-22.txt")),
            out(&fixture("rustup_check.txt")),
        ];
        let snapshot = RustupAdapter.parse_refresh(&outputs).expect("snapshot");
        assert!(snapshot.packages.iter().all(|p| !p.outdated));
        let s = snapshot.self_status.expect("self");
        assert!(!s.update_available);
    }

    #[test]
    fn inventory_only_toolchain_is_appended_with_unknown_versions() {
        let outputs = vec![
            out("stable-aarch64-apple-darwin (active, default)\nnightly-aarch64-apple-darwin\n"),
            out(&fixture("rustup_check_2026-07-21.txt")),
        ];
        let snapshot = RustupAdapter.parse_refresh(&outputs).expect("snapshot");
        assert_eq!(snapshot.packages.len(), 2);
        let nightly = snapshot
            .packages
            .iter()
            .find(|p| p.name == "nightly-aarch64-apple-darwin")
            .expect("appended");
        assert!(nightly.installed.is_none());
        assert!(!nightly.outdated);
    }

    #[test]
    fn upgrade_plan_is_rustup_update_with_toolchains() {
        let plan = RustupAdapter.upgrade_plan(
            &["toolchain:stable-aarch64-apple-darwin".into()],
            &PlanOptions::default(),
        );
        assert_eq!(plan.len(), 1);
        assert_eq!(plan[0].argv, vec!["update", "stable-aarch64-apple-darwin"]);
    }

    #[test]
    fn route_is_in_band_self_update() {
        match RustupAdapter.self_update_route(ManagedBy::Standalone, None) {
            SelfUpdateRoute::InBand {
                command_preview, ..
            } => assert_eq!(command_preview, "rustup self update"),
            other => panic!("expected InBand, got {other:?}"),
        }
    }

    #[test]
    fn refresh_plan_is_toolchain_list_then_check() {
        let plan = RustupAdapter.refresh_plan(&present(), &Settings::default());
        assert_eq!(plan.len(), 2);
        assert_eq!(plan[0].argv, vec!["toolchain", "list"]);
        assert_eq!(plan[1].argv, vec!["check"]);
    }

    #[test]
    fn absent_rustup_plans_nothing() {
        let det = DetectStatus::Absent {
            reason: "not found".into(),
        };
        assert!(RustupAdapter
            .refresh_plan(&det, &Settings::default())
            .is_empty());
    }
}
