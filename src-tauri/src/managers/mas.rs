//! mas adapter — implemented by U4 (SPEC §5.4 command surface, §5.5 parsing
//! via `parse::mas`).
//!
//! mas is ABSENT on this machine (verified `zsh: command not found: mas`), so
//! this adapter ships fully implemented but labeled UNVERIFIED (DECISIONS
//! D23): parsers are synthetic-fixture-tested, failures degrade to
//! `ParseFailed`-with-excerpt, and installing mas later requires zero code
//! changes. Absence is a normal state — an absent mas plans NOTHING, so the
//! runner is never invoked for it.
//!
//! Package ids carry the numeric App Store id (`app:497799835`) because
//! `mas upgrade` takes numeric ids; `name` is the display name.

use std::time::Duration;

use crate::detect::DetectStatus;
use crate::error::PmError;
use crate::ipc::{ManagedBy, ManagerId, ManagerSnapshot, Package, SelfUpdateRoute};
use crate::managers::parse::{self, mas as parse_mas};
use crate::managers::{ExitClass, ManagerAdapter, PackageId, PlanOptions, PlannedCommand, Timeout};
use crate::process::CommandOutput;
use crate::registry::{now_rfc3339, pkg_name};
use crate::settings::Settings;

pub struct MasAdapter;

#[async_trait::async_trait]
impl ManagerAdapter for MasAdapter {
    fn id(&self) -> ManagerId {
        ManagerId::Mas
    }

    fn display_name(&self) -> &'static str {
        "mas"
    }

    fn binary_name(&self) -> &'static str {
        "mas"
    }

    fn detection_candidates(&self) -> &'static [&'static str] {
        &["/opt/homebrew/bin/mas", "/usr/local/bin/mas"]
    }

    fn refresh_plan(&self, det: &DetectStatus, _settings: &Settings) -> Vec<PlannedCommand> {
        if matches!(det, DetectStatus::Absent { .. }) {
            return vec![]; // absent mas NEVER reaches the runner
        }
        vec![
            PlannedCommand {
                label: "mas list",
                argv: vec!["list".into()],
                timeout: Timeout::Absolute(Duration::from_secs(60)),
                extra_env: vec![],
                phase_label: None,
            },
            PlannedCommand {
                label: "mas outdated",
                argv: vec!["outdated".into()],
                timeout: Timeout::Absolute(Duration::from_secs(120)),
                extra_env: vec![],
                phase_label: None,
            },
        ]
    }

    fn parse_refresh(&self, outputs: &[CommandOutput]) -> Result<ManagerSnapshot, PmError> {
        let [list, outdated] = outputs else {
            return Err(PmError::Internal {
                detail: format!(
                    "mas parse_refresh expected 2 outputs, got {}",
                    outputs.len()
                ),
            });
        };
        let inventory = parse_mas::parse_list(&list.stdout)?;
        let overlay = parse_mas::parse_outdated(&outdated.stdout)?;
        Ok(ManagerSnapshot {
            manager_id: ManagerId::Mas,
            refreshed_at: now_rfc3339(),
            packages: parse::merge_inventory_overlay(inventory, overlay),
            self_status: None,
            health: vec![],
        })
    }

    fn recovery_plan(&self, _failed: &PlannedCommand) -> Option<PlannedCommand> {
        None // SPEC §5.4: no recovery (UNVERIFIED live; labeled).
    }

    fn parse_recovery(
        &self,
        failed: &PlannedCommand,
        _refresh_outputs: &[CommandOutput],
        _out: &CommandOutput,
    ) -> Result<ManagerSnapshot, PmError> {
        Err(PmError::Internal {
            detail: format!("mas has no recovery for `{}`", failed.label),
        })
    }

    fn upgrade_plan(&self, pkgs: &[PackageId], _opts: &PlanOptions) -> Vec<PlannedCommand> {
        if pkgs.is_empty() {
            return vec![];
        }
        let mut argv = vec!["upgrade".to_string()];
        // `app:497799835` → the numeric App Store id `mas upgrade` expects.
        argv.extend(pkgs.iter().map(|id| pkg_name(id).to_string()));
        vec![PlannedCommand {
            label: "mas upgrade",
            argv,
            timeout: super::brew::default_upgrade_timeout(),
            extra_env: vec![],
            phase_label: None,
        }]
    }

    fn self_update_route(
        &self,
        managed_by: ManagedBy,
        _own_outdated_row: Option<&Package>,
    ) -> SelfUpdateRoute {
        // SPEC §5.3 native rule: Unavailable unless brew-managed.
        match managed_by {
            ManagedBy::Brew => SelfUpdateRoute::Routed {
                executor: ManagerId::Brew,
                command_preview: "brew upgrade mas".into(),
                why: "mas is managed by Homebrew".into(),
            },
            _ => SelfUpdateRoute::Unavailable {
                reason: "mas has no self-update mechanism".into(),
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

    fn out(stdout: &str) -> CommandOutput {
        CommandOutput {
            exit_code: Some(0),
            stdout: stdout.to_string(),
            stderr: String::new(),
            duration: Duration::ZERO,
        }
    }

    #[test]
    fn absent_mas_plans_nothing_so_runner_is_never_called() {
        let det = DetectStatus::Absent {
            reason: "`mas` not found on the search path".into(),
        };
        assert!(MasAdapter
            .refresh_plan(&det, &Settings::default())
            .is_empty());
    }

    #[test]
    fn refresh_merges_list_and_outdated() {
        // Both fixtures come from the SAME live refresh, so the overlay is
        // exercised against a self-consistent pair: list reports 5.20.0
        // installed, outdated reports 5.20.0 -> 5.21.0.
        let outputs = vec![
            out(&fixture("mas_list_2026-07-22.txt")),
            out(&fixture("mas_outdated_2026-07-22.txt")),
        ];
        let snapshot = MasAdapter.parse_refresh(&outputs).expect("snapshot");
        assert_eq!(snapshot.packages.len(), 12);

        let canary = snapshot
            .packages
            .iter()
            .find(|p| p.name == "Canary Mail")
            .expect("canary row");
        assert_eq!(canary.id, "app:1236045954");
        assert_eq!(canary.installed.as_deref(), Some("5.20.0"));
        assert!(canary.outdated);
        assert_eq!(canary.latest.as_deref(), Some("5.21.0"));

        // Only the 3 rows in the outdated overlay flip; the other 9 stay clean.
        assert_eq!(snapshot.packages.iter().filter(|p| p.outdated).count(), 3);
        let xcode = snapshot
            .packages
            .iter()
            .find(|p| p.name == "Xcode")
            .expect("xcode row");
        assert!(!xcode.outdated);
        // `parse_list` mirrors installed into latest, so an untouched row
        // reports its own version rather than a null the UI would have to
        // render as "update available".
        assert_eq!(xcode.latest.as_deref(), Some("26.6"));
        assert_eq!(xcode.installed.as_deref(), Some("26.6"));
    }

    #[test]
    fn shell_error_degrades_to_parse_failed_never_a_crash() {
        // Defense in depth: detection gates mas, but if the shell error ever
        // reached parse_refresh it is ParseFailed, not a panic.
        let outputs = vec![out(&fixture("mas_outdated.txt")), out("")];
        let err = MasAdapter.parse_refresh(&outputs).unwrap_err();
        assert!(matches!(err, PmError::ParseFailed { .. }));
    }

    #[test]
    fn upgrade_plan_uses_numeric_ids() {
        let plan = MasAdapter.upgrade_plan(
            &["app:497799835".into(), "app:1295203466".into()],
            &PlanOptions::default(),
        );
        assert_eq!(plan.len(), 1);
        assert_eq!(plan[0].argv, vec!["upgrade", "497799835", "1295203466"]);
    }

    #[test]
    fn route_is_unavailable_unless_brew_managed() {
        match MasAdapter.self_update_route(ManagedBy::Standalone, None) {
            SelfUpdateRoute::Unavailable { .. } => {}
            other => panic!("expected Unavailable, got {other:?}"),
        }
        match MasAdapter.self_update_route(ManagedBy::Brew, None) {
            SelfUpdateRoute::Routed {
                executor,
                command_preview,
                ..
            } => {
                assert_eq!(executor, ManagerId::Brew);
                assert_eq!(command_preview, "brew upgrade mas");
            }
            other => panic!("expected Routed, got {other:?}"),
        }
    }
}
