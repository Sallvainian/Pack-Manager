//! npm adapter — implemented by U4 (SPEC §5.4 command surface, §5.5 parsing
//! via `parse::npm`).
//!
//! Refresh: `npm ls -g --depth=0 --json` (60s) → `npm outdated -g --json`
//! (120s). **Exit 1 on the JSON commands is expected** (npm exits 1 whenever
//! outdated packages exist — verified capture); JSON validity is judged by the
//! parser so exit-1-with-garbage surfaces as `ParseFailed`, never
//! `NonZeroExit` (SPEC §5.4, §7.3). The `npm` row is hoisted into
//! `selfStatus`; the in-band self-update override rides on it (DECISIONS D5).

use std::time::Duration;

use crate::detect::DetectStatus;
use crate::error::PmError;
use crate::ipc::{ManagedBy, ManagerId, ManagerSnapshot, Package, SelfUpdateRoute};
use crate::managers::parse::{self, npm as parse_npm};
use crate::managers::{ExitClass, ManagerAdapter, PackageId, PlanOptions, PlannedCommand, Timeout};
use crate::process::CommandOutput;
use crate::registry::{now_rfc3339, pkg_name};
use crate::settings::Settings;

/// Permanent copy on npm's SelfUpdateCard (SPEC F6, DECISIONS D21).
pub const NPM_MISE_RESET_NOTE: &str = "npm and all global packages live inside the mise-managed \
                                       node — upgrading node via mise resets them.";

pub struct NpmAdapter;

#[async_trait::async_trait]
impl ManagerAdapter for NpmAdapter {
    fn id(&self) -> ManagerId {
        ManagerId::Npm
    }

    fn display_name(&self) -> &'static str {
        "npm"
    }

    fn binary_name(&self) -> &'static str {
        "npm"
    }

    fn detection_candidates(&self) -> &'static [&'static str] {
        &[
            "~/.local/share/mise/shims/npm",
            "/opt/homebrew/bin/npm",
            "/usr/local/bin/npm",
        ]
    }

    fn refresh_plan(&self, det: &DetectStatus, _settings: &Settings) -> Vec<PlannedCommand> {
        if matches!(det, DetectStatus::Absent { .. }) {
            return vec![];
        }
        vec![
            PlannedCommand {
                label: "npm ls -g",
                argv: ["ls", "-g", "--depth=0", "--json"]
                    .into_iter()
                    .map(String::from)
                    .collect(),
                timeout: Timeout::Absolute(Duration::from_secs(60)),
                extra_env: vec![],
                phase_label: None,
            },
            PlannedCommand {
                label: "npm outdated -g --json",
                argv: ["outdated", "-g", "--json"]
                    .into_iter()
                    .map(String::from)
                    .collect(),
                timeout: Timeout::Absolute(Duration::from_secs(120)),
                extra_env: vec![],
                phase_label: None,
            },
        ]
    }

    fn parse_refresh(&self, outputs: &[CommandOutput]) -> Result<ManagerSnapshot, PmError> {
        let [ls, outdated] = outputs else {
            return Err(PmError::Internal {
                detail: format!(
                    "npm parse_refresh expected 2 outputs, got {}",
                    outputs.len()
                ),
            });
        };
        let inventory = parse_npm::parse_ls_json(&ls.stdout)?;
        let overlay = parse_npm::parse_outdated_json(&outdated.stdout)?;
        let merged = parse::merge_inventory_overlay(inventory, overlay);
        Ok(self.snapshot(merged))
    }

    fn recovery_plan(&self, failed: &PlannedCommand) -> Option<PlannedCommand> {
        // Outdated JSON parse failure → verified `npm outdated -g` text form.
        if failed.argv == ["outdated", "-g", "--json"] {
            return Some(PlannedCommand {
                label: "npm outdated -g",
                argv: vec!["outdated".into(), "-g".into()],
                timeout: Timeout::Absolute(Duration::from_secs(120)),
                extra_env: vec![],
                phase_label: None,
            });
        }
        None
    }

    fn parse_recovery(
        &self,
        _failed: &PlannedCommand,
        out: &CommandOutput,
    ) -> Result<ManagerSnapshot, PmError> {
        let rows = parse_npm::parse_outdated_text(&out.stdout)?;
        Ok(self.snapshot(rows))
    }

    fn upgrade_plan(&self, pkgs: &[PackageId], _opts: &PlanOptions) -> Vec<PlannedCommand> {
        if pkgs.is_empty() {
            return vec![];
        }
        let mut argv = vec!["install".to_string(), "-g".to_string()];
        argv.extend(pkgs.iter().map(|id| format!("{}@latest", pkg_name(id))));
        vec![PlannedCommand {
            label: "npm install -g",
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
        let note = (managed_by == ManagedBy::Mise).then(|| NPM_MISE_RESET_NOTE.to_string());
        // In-band override: npm reports ITSELF in its own outdated listing
        // (fixture row `npm 11.16.0 12.0.1 12.0.1`) — without this rule npm
        // would misroute to mise (SPEC §5.3 rule 1, DECISIONS D5).
        if own_outdated_row.is_some() {
            return SelfUpdateRoute::InBand {
                command_preview: "npm install -g npm@latest".into(),
                note,
            };
        }
        match managed_by {
            ManagedBy::Mise => SelfUpdateRoute::Routed {
                executor: ManagerId::Mise,
                command_preview: "mise upgrade npm".into(),
                why: "npm is managed by mise".into(),
            },
            ManagedBy::Brew => SelfUpdateRoute::Routed {
                executor: ManagerId::Brew,
                command_preview: "brew upgrade npm".into(),
                why: "npm is managed by Homebrew".into(),
            },
            _ => SelfUpdateRoute::InBand {
                command_preview: "npm install -g npm@latest".into(),
                note,
            },
        }
    }

    fn classify_exit(&self, cmd: &PlannedCommand, out: &CommandOutput) -> ExitClass {
        match out.exit_code {
            Some(0) => ExitClass::Success,
            // npm's JSON commands exit 1 whenever outdated packages exist;
            // the parser decides whether the payload is usable (garbage →
            // ParseFailed, never NonZeroExit).
            Some(1) if cmd.argv.iter().any(|a| a == "--json") => ExitClass::ExpectedNonZero,
            _ => ExitClass::Failure,
        }
    }
}

impl NpmAdapter {
    fn snapshot(&self, merged: Vec<Package>) -> ManagerSnapshot {
        // The npm row lives only on the SelfUpdateCard (SPEC §5.4, F6).
        let (packages, self_status) = parse::extract_self(merged, "npm");
        ManagerSnapshot {
            manager_id: ManagerId::Npm,
            refreshed_at: now_rfc3339(),
            packages,
            self_status,
            health: vec![],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::process::fake::{fixture, FakeRunner};
    use crate::process::{CmdPurpose, CommandRunner, CommandSpec};
    use std::path::PathBuf;

    fn present() -> DetectStatus {
        DetectStatus::Present {
            binary_path: PathBuf::from("/Users/testuser/.local/share/mise/shims/npm"),
            canonical_path: PathBuf::from("/opt/homebrew/bin/mise"),
            version: Some("11.16.0".into()),
            managed_by: ManagedBy::Mise,
            evidence: "resolved at ~/.local/share/mise/shims/npm".into(),
        }
    }

    fn spec_for(cmd: &PlannedCommand) -> CommandSpec {
        CommandSpec {
            program: PathBuf::from("/Users/testuser/.local/share/mise/shims/npm"),
            args: cmd.argv.clone(),
            env: vec![],
            timeout: cmd.timeout,
            purpose: CmdPurpose::Refresh,
        }
    }

    fn out_with(code: i32, stdout: &str) -> CommandOutput {
        CommandOutput {
            exit_code: Some(code),
            stdout: stdout.to_string(),
            stderr: String::new(),
            duration: Duration::ZERO,
        }
    }

    #[tokio::test]
    async fn npm_exit_1_with_json_is_success() {
        let fake = FakeRunner::new();
        fake.on("npm", &["ls", "-g", "--depth=0", "--json"])
            .fixture("npm_ls_g_2026-07-22.json");
        // npm exits 1 when outdated packages exist — still a success.
        fake.on("npm", &["outdated", "-g", "--json"])
            .fixture_with_exit("npm_outdated_g_synthetic.json", 1);

        let adapter = NpmAdapter;
        let plan = adapter.refresh_plan(&present(), &Settings::default());
        let mut outputs = Vec::new();
        for cmd in &plan {
            let out = fake.run(&spec_for(cmd)).await.unwrap();
            let class = adapter.classify_exit(cmd, &out);
            assert_ne!(class, ExitClass::Failure, "{}: {:?}", cmd.label, class);
            outputs.push(out);
        }
        assert_eq!(
            adapter.classify_exit(&plan[1], &outputs[1]),
            ExitClass::ExpectedNonZero
        );

        let snapshot = adapter.parse_refresh(&outputs).expect("snapshot");
        // 15 inventory rows − npm self row = 14 packages.
        assert_eq!(snapshot.packages.len(), 14);
        assert!(!snapshot.packages.iter().any(|p| p.name == "npm"));
        // The 2026-07-22 inventory already carries npm 12.0.1; the synthetic
        // overlay (2026-07-21 values) still patches latest + the outdated
        // verdict. Inventory `installed` wins in the merge.
        let s = snapshot.self_status.expect("npm hoisted");
        assert_eq!(s.installed.as_deref(), Some("12.0.1"), "inventory value");
        assert_eq!(s.latest.as_deref(), Some("12.0.1"));
        assert!(s.update_available, "the overlay's verdict is authoritative");
        let ts = snapshot
            .packages
            .iter()
            .find(|p| p.name == "typescript")
            .unwrap();
        assert!(ts.outdated);
        assert_eq!(ts.latest.as_deref(), Some("7.0.2"));
    }

    #[test]
    fn npm_exit_1_with_garbage_is_parse_failed() {
        let adapter = NpmAdapter;
        let plan = adapter.refresh_plan(&present(), &Settings::default());
        let garbage = out_with(1, "npm ERR! something exploded");
        // Exit classification stays lenient for the JSON command…
        assert_eq!(
            adapter.classify_exit(&plan[1], &garbage),
            ExitClass::ExpectedNonZero
        );
        // …and the parser is the judge: garbage → ParseFailed.
        let outputs = vec![out_with(0, &fixture("npm_ls_g_2026-07-22.json")), garbage];
        let err = adapter.parse_refresh(&outputs).unwrap_err();
        assert!(matches!(err, PmError::ParseFailed { .. }));
    }

    #[test]
    fn non_json_command_exit_1_is_failure() {
        let adapter = NpmAdapter;
        let upgrade = &adapter.upgrade_plan(
            &["globalPackage:typescript".into()],
            &PlanOptions::default(),
        )[0];
        assert_eq!(
            adapter.classify_exit(upgrade, &out_with(1, "")),
            ExitClass::Failure
        );
        let plan = adapter.refresh_plan(&present(), &Settings::default());
        assert_eq!(
            adapter.classify_exit(&plan[1], &out_with(2, "{}")),
            ExitClass::Failure,
            "only exit 1 is the expected non-zero"
        );
    }

    #[test]
    fn recovery_runs_text_fallback_and_still_hoists_npm() {
        let adapter = NpmAdapter;
        let plan = adapter.refresh_plan(&present(), &Settings::default());
        assert!(adapter.recovery_plan(&plan[0]).is_none());
        let recovery = adapter.recovery_plan(&plan[1]).expect("recovery");
        assert_eq!(recovery.argv, vec!["outdated", "-g"]);

        let out = out_with(1, &fixture("npm_outdated_g_text_2026-07-21.txt"));
        let snapshot = adapter.parse_recovery(&recovery, &out).expect("snapshot");
        assert_eq!(snapshot.packages.len(), 4, "5 rows, npm hoisted");
        let s = snapshot.self_status.expect("npm self");
        assert_eq!(s.installed.as_deref(), Some("11.16.0"));
        assert_eq!(s.latest.as_deref(), Some("12.0.1"));
    }

    #[test]
    fn upgrade_plan_is_one_install_g_command_with_latest_suffix() {
        let pkgs: Vec<PackageId> = vec![
            "globalPackage:typescript".into(),
            "globalPackage:dmux".into(),
        ];
        let plan = NpmAdapter.upgrade_plan(&pkgs, &PlanOptions::default());
        assert_eq!(plan.len(), 1);
        assert_eq!(
            plan[0].argv,
            vec!["install", "-g", "typescript@latest", "dmux@latest"]
        );
    }

    #[test]
    fn in_band_override_wins_when_npm_reports_itself_outdated() {
        // Fixture row `npm 11.16.0 12.0.1 12.0.1` (npm_outdated_g_text_2026-07-21.txt).
        let own = Package {
            id: "globalPackage:npm".into(),
            name: "npm".into(),
            kind: crate::ipc::PackageKind::GlobalPackage,
            installed: Some("11.16.0".into()),
            latest: Some("12.0.1".into()),
            outdated: true,
            pinned: false,
            meta: None,
        };
        match NpmAdapter.self_update_route(ManagedBy::Mise, Some(&own)) {
            SelfUpdateRoute::InBand {
                command_preview,
                note,
            } => {
                assert_eq!(command_preview, "npm install -g npm@latest");
                assert_eq!(note.as_deref(), Some(NPM_MISE_RESET_NOTE));
            }
            other => panic!("expected InBand override, got {other:?}"),
        }
        // Without the own row the generic delegation applies.
        match NpmAdapter.self_update_route(ManagedBy::Mise, None) {
            SelfUpdateRoute::Routed { executor, .. } => assert_eq!(executor, ManagerId::Mise),
            other => panic!("expected Routed, got {other:?}"),
        }
    }
}
