//! mise adapter — implemented by U4 (SPEC §5.4 command surface, §5.5 parsing
//! via `parse::mise`).
//!
//! Refresh: `mise ls --json` (60s) → `mise outdated --json` (120s; `{}` =
//! clean, verified). The populated outdated-JSON shape is UNVERIFIED, so the
//! fixture-tested `mise outdated` TEXT parser is wired as recovery
//! (DECISIONS D8).

use std::time::Duration;

use crate::detect::DetectStatus;
use crate::error::PmError;
use crate::ipc::{ManagedBy, ManagerId, ManagerSnapshot, Package, SelfUpdateRoute};
use crate::managers::parse::{self, mise as parse_mise};
use crate::managers::{ExitClass, ManagerAdapter, PackageId, PlanOptions, PlannedCommand, Timeout};
use crate::process::CommandOutput;
use crate::registry::{now_rfc3339, pkg_name};
use crate::settings::Settings;

pub struct MiseAdapter;

#[async_trait::async_trait]
impl ManagerAdapter for MiseAdapter {
    fn id(&self) -> ManagerId {
        ManagerId::Mise
    }

    fn display_name(&self) -> &'static str {
        "mise"
    }

    fn binary_name(&self) -> &'static str {
        "mise"
    }

    fn detection_candidates(&self) -> &'static [&'static str] {
        &["/opt/homebrew/bin/mise", "~/.local/bin/mise"]
    }

    fn refresh_plan(&self, det: &DetectStatus, _settings: &Settings) -> Vec<PlannedCommand> {
        if matches!(det, DetectStatus::Absent { .. }) {
            return vec![];
        }
        vec![
            PlannedCommand {
                label: "mise ls --json",
                argv: vec!["ls".into(), "--json".into()],
                timeout: Timeout::Absolute(Duration::from_secs(60)),
                extra_env: vec![],
                phase_label: None,
            },
            PlannedCommand {
                label: "mise outdated --json",
                argv: vec!["outdated".into(), "--json".into()],
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
                    "mise parse_refresh expected 2 outputs, got {}",
                    outputs.len()
                ),
            });
        };
        let inventory = parse_mise::parse_ls_json(&ls.stdout)?;
        let overlay = parse_mise::parse_outdated_json(&outdated.stdout)?;
        Ok(self.snapshot(parse::merge_inventory_overlay(inventory, overlay)))
    }

    fn recovery_plan(&self, failed: &PlannedCommand) -> Option<PlannedCommand> {
        // Outdated JSON parse failure → verified `mise outdated` text form.
        if failed.argv == ["outdated", "--json"] {
            return Some(PlannedCommand {
                label: "mise outdated",
                argv: vec!["outdated".into()],
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
        refresh_outputs: &[CommandOutput],
        out: &CommandOutput,
    ) -> Result<ManagerSnapshot, PmError> {
        // `mise ls --json` already succeeded — merge its full inventory with
        // the recovered text overlay instead of shrinking the snapshot to the
        // outdated rows alone (the up-to-date tools must not vanish).
        let [ls, _outdated] = refresh_outputs else {
            return Err(PmError::Internal {
                detail: format!(
                    "mise parse_recovery expected 2 refresh outputs, got {}",
                    refresh_outputs.len()
                ),
            });
        };
        let inventory = parse_mise::parse_ls_json(&ls.stdout)?;
        let overlay = parse_mise::parse_outdated_text(&out.stdout)?;
        Ok(self.snapshot(parse::merge_inventory_overlay(inventory, overlay)))
    }

    fn upgrade_plan(&self, pkgs: &[PackageId], _opts: &PlanOptions) -> Vec<PlannedCommand> {
        if pkgs.is_empty() {
            return vec![];
        }
        let mut argv = vec!["upgrade".to_string()];
        // Names verbatim, incl. `npm:prettier` (split ids on FIRST ':' only).
        argv.extend(pkgs.iter().map(|id| pkg_name(id).to_string()));
        vec![PlannedCommand {
            label: "mise upgrade",
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
        // In-band override first (SPEC §5.3 precedence).
        if own_outdated_row.is_some() {
            return SelfUpdateRoute::InBand {
                command_preview: "mise self-update".into(),
                note: None,
            };
        }
        match managed_by {
            ManagedBy::Brew => SelfUpdateRoute::Routed {
                executor: ManagerId::Brew,
                command_preview: "brew upgrade mise".into(),
                why: "mise is managed by Homebrew".into(),
            },
            _ => SelfUpdateRoute::InBand {
                command_preview: "mise self-update".into(),
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

impl MiseAdapter {
    fn snapshot(&self, packages: Vec<Package>) -> ManagerSnapshot {
        ManagerSnapshot {
            manager_id: ManagerId::Mise,
            refreshed_at: now_rfc3339(),
            packages,
            // mise's selfStatus arrives via the cross-manager join (brew's
            // `formula:mise` row — SPEC §5.8).
            self_status: None,
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
            binary_path: PathBuf::from("/opt/homebrew/bin/mise"),
            canonical_path: PathBuf::from("/opt/homebrew/bin/mise"),
            version: Some("2026.1.5".into()),
            managed_by: ManagedBy::Brew,
            evidence: "resolved at /opt/homebrew/bin/mise — under Homebrew's tree".into(),
        }
    }

    fn spec_for(cmd: &PlannedCommand) -> CommandSpec {
        CommandSpec {
            program: PathBuf::from("/opt/homebrew/bin/mise"),
            args: cmd.argv.clone(),
            env: vec![],
            timeout: cmd.timeout,
            purpose: CmdPurpose::Refresh,
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

    #[tokio::test]
    async fn refresh_merges_inventory_with_clean_outdated() {
        let fake = FakeRunner::new();
        fake.on("mise", &["ls", "--json"])
            .fixture("mise_ls_2026-07-22.json");
        fake.on("mise", &["outdated", "--json"])
            .fixture("mise_outdated.json"); // `{}`

        let adapter = MiseAdapter;
        let plan = adapter.refresh_plan(&present(), &Settings::default());
        assert_eq!(plan.len(), 2);
        let mut outputs = Vec::new();
        for cmd in &plan {
            outputs.push(
                fake.run(&spec_for(cmd), tokio_util::sync::CancellationToken::new())
                    .await
                    .unwrap(),
            );
        }
        let snapshot = adapter.parse_refresh(&outputs).expect("snapshot");
        assert_eq!(snapshot.packages.len(), 11, "11 tools in mise_ls fixture");
        assert!(
            snapshot.packages.iter().all(|p| !p.outdated),
            "{{}} = clean"
        );
    }

    #[test]
    fn refresh_overlay_marks_outdated_rows() {
        // Synthetic outdated JSON, values verbatim from the 2026-07-21 text
        // capture; the 2026-07-22 inventory already carries uv 0.11.30. Merge
        // semantics: inventory's `installed` wins, the overlay patches
        // `latest`/`outdated` (the manager's verdict is authoritative).
        let outputs = vec![
            out(&fixture("mise_ls_2026-07-22.json")),
            out(&fixture("mise_outdated_synthetic.json")),
        ];
        let snapshot = MiseAdapter.parse_refresh(&outputs).expect("snapshot");
        let uv = snapshot.packages.iter().find(|p| p.name == "uv").unwrap();
        assert!(uv.outdated);
        assert_eq!(uv.installed.as_deref(), Some("0.11.30"), "inventory value");
        assert_eq!(uv.latest.as_deref(), Some("0.11.30"));
        // `rust stable stable stable` is NOT outdated (current == latest).
        let rust = snapshot.packages.iter().find(|p| p.name == "rust").unwrap();
        assert!(!rust.outdated);
    }

    #[tokio::test]
    async fn recovery_fallback_runs_text_parser_on_bad_outdated_json() {
        let adapter = MiseAdapter;
        let plan = adapter.refresh_plan(&present(), &Settings::default());

        // Outdated returned garbage where JSON was expected.
        let outputs = vec![
            out(&fixture("mise_ls_2026-07-22.json")),
            out("Tool  Requested  Current  Latest\nnot json at all"),
        ];
        let err = adapter.parse_refresh(&outputs).unwrap_err();
        assert!(matches!(err, PmError::ParseFailed { .. }));

        // The wired recovery is the text command for the outdated spec only.
        assert!(
            adapter.recovery_plan(&plan[0]).is_none(),
            "ls has no recovery"
        );
        let recovery = adapter.recovery_plan(&plan[1]).expect("recovery");
        assert_eq!(recovery.argv, vec!["outdated"]);

        let fake = FakeRunner::new();
        fake.on("mise", &["outdated"])
            .fixture("mise_outdated_text_2026-07-21.txt");
        let rec_out = fake
            .run(
                &spec_for(&recovery),
                tokio_util::sync::CancellationToken::new(),
            )
            .await
            .unwrap();
        // Regression: recovery merges the already-captured `mise ls --json`
        // inventory (11 tools) with the text overlay — the up-to-date tools
        // (bun/go/node/python/rust) must not vanish when recovery fires.
        let snapshot = adapter
            .parse_recovery(&recovery, &outputs, &rec_out)
            .expect("snapshot");
        assert_eq!(snapshot.packages.len(), 11, "full inventory survives");
        assert_eq!(
            snapshot.packages.iter().filter(|p| p.outdated).count(),
            6,
            "7 overlay rows, rust dropped (current == latest)"
        );
        assert!(
            snapshot
                .packages
                .iter()
                .any(|p| p.name == "node" && !p.outdated),
            "up-to-date inventory row retained"
        );
        let prettier = snapshot
            .packages
            .iter()
            .find(|p| p.name == "npm:prettier")
            .expect("verbatim npm:prettier");
        assert_eq!(prettier.id, "tool:npm:prettier");
        assert!(prettier.outdated);
    }

    #[test]
    fn upgrade_plan_is_one_command_with_verbatim_names() {
        let pkgs: Vec<PackageId> = vec![
            "tool:deno".into(),
            "tool:ruby".into(),
            "tool:fnox".into(),
            "tool:ruff".into(),
            "tool:npm:prettier".into(),
            "tool:uv".into(),
        ];
        let plan = MiseAdapter.upgrade_plan(&pkgs, &PlanOptions::default());
        assert_eq!(plan.len(), 1);
        assert_eq!(
            plan[0].argv,
            vec![
                "upgrade",
                "deno",
                "ruby",
                "fnox",
                "ruff",
                "npm:prettier",
                "uv"
            ]
        );
        assert!(matches!(plan[0].timeout, Timeout::Stall { .. }));
    }

    #[test]
    fn route_is_brew_when_brew_managed_else_in_band() {
        let adapter = MiseAdapter;
        match adapter.self_update_route(ManagedBy::Brew, None) {
            SelfUpdateRoute::Routed {
                executor,
                command_preview,
                ..
            } => {
                assert_eq!(executor, ManagerId::Brew);
                assert_eq!(command_preview, "brew upgrade mise");
            }
            other => panic!("expected Routed, got {other:?}"),
        }
        match adapter.self_update_route(ManagedBy::Standalone, None) {
            SelfUpdateRoute::InBand {
                command_preview, ..
            } => assert_eq!(command_preview, "mise self-update"),
            other => panic!("expected InBand, got {other:?}"),
        }
    }

    #[test]
    fn absent_mise_plans_nothing() {
        let det = DetectStatus::Absent {
            reason: "not found".into(),
        };
        assert!(MiseAdapter
            .refresh_plan(&det, &Settings::default())
            .is_empty());
    }
}
