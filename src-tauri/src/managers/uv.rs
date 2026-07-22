//! uv adapter — implemented by U4 (SPEC §5.4 command surface, §5.5 parsing
//! via `parse::uv`).
//!
//! Refresh: `uv tool list` (60s; `warning:` lines from either stream become
//! `HealthIssue`s) → `uv tool list --outdated` (120s; empty stdout = clean per
//! the 0-byte capture; unknown suffixes degrade to `latest: null` so the UI
//! shows "update available", never a fabricated delta).

use std::time::Duration;

use crate::detect::DetectStatus;
use crate::error::PmError;
use crate::ipc::{ManagedBy, ManagerId, ManagerSnapshot, Package, SelfUpdateRoute};
use crate::managers::parse::{self, uv as parse_uv};
use crate::managers::{ExitClass, ManagerAdapter, PackageId, PlanOptions, PlannedCommand, Timeout};
use crate::process::CommandOutput;
use crate::registry::{now_rfc3339, pkg_name};
use crate::settings::Settings;

pub struct UvAdapter;

#[async_trait::async_trait]
impl ManagerAdapter for UvAdapter {
    fn id(&self) -> ManagerId {
        ManagerId::Uv
    }

    fn display_name(&self) -> &'static str {
        "uv"
    }

    fn binary_name(&self) -> &'static str {
        "uv"
    }

    fn detection_candidates(&self) -> &'static [&'static str] {
        &[
            "~/.local/share/mise/shims/uv",
            "~/.local/bin/uv",
            "/opt/homebrew/bin/uv",
        ]
    }

    fn refresh_plan(&self, det: &DetectStatus, _settings: &Settings) -> Vec<PlannedCommand> {
        if matches!(det, DetectStatus::Absent { .. }) {
            return vec![];
        }
        vec![
            PlannedCommand {
                label: "uv tool list",
                argv: vec!["tool".into(), "list".into()],
                timeout: Timeout::Absolute(Duration::from_secs(60)),
                extra_env: vec![],
                phase_label: None,
            },
            PlannedCommand {
                label: "uv tool list --outdated",
                argv: vec!["tool".into(), "list".into(), "--outdated".into()],
                timeout: Timeout::Absolute(Duration::from_secs(120)),
                extra_env: vec![],
                phase_label: None,
            },
        ]
    }

    fn parse_refresh(&self, outputs: &[CommandOutput]) -> Result<ManagerSnapshot, PmError> {
        let [list, outdated] = outputs else {
            return Err(PmError::Internal {
                detail: format!("uv parse_refresh expected 2 outputs, got {}", outputs.len()),
            });
        };
        let parsed = parse_uv::parse_tool_list(&list.stdout, &list.stderr);
        if !outdated.stdout.trim().is_empty() {
            // The populated `--outdated` format is under-verified (0-byte
            // capture) — WARN until it is captured (SPEC §6.3).
            tracing::warn!(
                bytes = outdated.stdout.len(),
                "uv tool list --outdated returned output; format is under-verified"
            );
        }
        let overlay = parse_uv::parse_tool_list_outdated(&outdated.stdout);
        Ok(ManagerSnapshot {
            manager_id: ManagerId::Uv,
            refreshed_at: now_rfc3339(),
            packages: parse::merge_inventory_overlay(parsed.packages, overlay),
            // uv's selfStatus arrives via the cross-manager join (mise's
            // `tool:uv` row — SPEC §5.8).
            self_status: None,
            health: parsed.health,
        })
    }

    fn recovery_plan(&self, _failed: &PlannedCommand) -> Option<PlannedCommand> {
        None // SPEC §5.4: uv has no recovery command.
    }

    fn parse_recovery(
        &self,
        failed: &PlannedCommand,
        _out: &CommandOutput,
    ) -> Result<ManagerSnapshot, PmError> {
        Err(PmError::Internal {
            detail: format!("uv has no recovery for `{}`", failed.label),
        })
    }

    fn upgrade_plan(&self, pkgs: &[PackageId], _opts: &PlanOptions) -> Vec<PlannedCommand> {
        if pkgs.is_empty() {
            return vec![];
        }
        let mut argv = vec!["tool".to_string(), "upgrade".to_string()];
        argv.extend(pkgs.iter().map(|id| pkg_name(id).to_string()));
        vec![PlannedCommand {
            label: "uv tool upgrade",
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
        if own_outdated_row.is_some() {
            return SelfUpdateRoute::InBand {
                command_preview: "uv self update".into(),
                note: None,
            };
        }
        match managed_by {
            ManagedBy::Mise => SelfUpdateRoute::Routed {
                executor: ManagerId::Mise,
                command_preview: "mise upgrade uv".into(),
                why: "uv is managed by mise".into(),
            },
            ManagedBy::Brew => SelfUpdateRoute::Routed {
                executor: ManagerId::Brew,
                command_preview: "brew upgrade uv".into(),
                why: "uv is managed by Homebrew".into(),
            },
            _ => SelfUpdateRoute::InBand {
                command_preview: "uv self update".into(),
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
    use crate::ipc::HealthSeverity;
    use crate::process::fake::FakeRunner;
    use crate::process::{CmdPurpose, CommandRunner, CommandSpec};
    use std::path::PathBuf;

    fn present() -> DetectStatus {
        DetectStatus::Present {
            binary_path: PathBuf::from("/Users/testuser/.local/share/mise/shims/uv"),
            canonical_path: PathBuf::from("/opt/homebrew/bin/mise"),
            version: Some("0.11.26".into()),
            managed_by: ManagedBy::Mise,
            evidence: "resolved at ~/.local/share/mise/shims/uv".into(),
        }
    }

    fn spec_for(cmd: &PlannedCommand) -> CommandSpec {
        CommandSpec {
            program: PathBuf::from("/Users/testuser/.local/share/mise/shims/uv"),
            args: cmd.argv.clone(),
            env: vec![],
            timeout: cmd.timeout,
            purpose: CmdPurpose::Refresh,
        }
    }

    #[tokio::test]
    async fn warning_line_becomes_health_issue_and_report_is_ok() {
        let fake = FakeRunner::new();
        // Fixture line 1: warning: Tool `aider-chat` environment not found …
        fake.on("uv", &["tool", "list"])
            .fixture("uv_tool_list_2026-07-21.txt");
        fake.on("uv", &["tool", "list", "--outdated"])
            .fixture("uv_tool_list_outdated.txt"); // 0-byte = clean

        let adapter = UvAdapter;
        let plan = adapter.refresh_plan(&present(), &Settings::default());
        let mut outputs = Vec::new();
        for cmd in &plan {
            outputs.push(fake.run(&spec_for(cmd)).await.unwrap());
        }
        let snapshot = adapter
            .parse_refresh(&outputs)
            .expect("report Ok despite warning");
        assert_eq!(snapshot.packages.len(), 12, "12 tools despite the warning");
        assert!(snapshot.packages.iter().all(|p| !p.outdated));

        assert_eq!(snapshot.health.len(), 1);
        let h = &snapshot.health[0];
        assert_eq!(h.id, "uv:aider-chat");
        assert_eq!(h.severity, HealthSeverity::Warning);
        assert_eq!(
            h.fix_command.as_deref(),
            Some("uv tool install aider-chat --reinstall")
        );
    }

    #[test]
    fn unknown_outdated_suffix_keeps_latest_null() {
        let outputs = vec![
            CommandOutput {
                exit_code: Some(0),
                stdout: "ruff v0.15.20\n- ruff\n".into(),
                stderr: String::new(),
                duration: Duration::ZERO,
            },
            CommandOutput {
                exit_code: Some(0),
                stdout: "ruff 0.15.20 (some unrecognised note)\n".into(),
                stderr: String::new(),
                duration: Duration::ZERO,
            },
        ];
        let snapshot = UvAdapter.parse_refresh(&outputs).expect("snapshot");
        let ruff = snapshot.packages.iter().find(|p| p.name == "ruff").unwrap();
        assert!(ruff.outdated, "the manager's verdict is authoritative");
        assert!(
            ruff.latest.is_none(),
            "unknown suffix degrades to null latest — UI shows \"update available\""
        );
    }

    #[test]
    fn upgrade_plan_is_one_tool_upgrade_command() {
        let plan = UvAdapter.upgrade_plan(
            &["tool:ruff".into(), "tool:serena-agent".into()],
            &PlanOptions::default(),
        );
        assert_eq!(plan.len(), 1);
        assert_eq!(
            plan[0].argv,
            vec!["tool", "upgrade", "ruff", "serena-agent"]
        );
    }

    #[test]
    fn routes_follow_managed_by_dynamically() {
        match UvAdapter.self_update_route(ManagedBy::Mise, None) {
            SelfUpdateRoute::Routed {
                executor,
                command_preview,
                ..
            } => {
                assert_eq!(executor, ManagerId::Mise);
                assert_eq!(command_preview, "mise upgrade uv");
            }
            other => panic!("expected Routed, got {other:?}"),
        }
        // The counterfactual: standalone uv self-updates in band.
        match UvAdapter.self_update_route(ManagedBy::Standalone, None) {
            SelfUpdateRoute::InBand {
                command_preview, ..
            } => assert_eq!(command_preview, "uv self update"),
            other => panic!("expected InBand, got {other:?}"),
        }
    }

    #[test]
    fn no_recovery_wired() {
        let plan = UvAdapter.refresh_plan(&present(), &Settings::default());
        assert!(UvAdapter.recovery_plan(&plan[1]).is_none());
    }
}
