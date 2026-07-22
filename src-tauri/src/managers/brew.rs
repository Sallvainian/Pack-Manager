//! brew adapter — implemented by U4 (SPEC §5.4 command surface, §5.5 parsing
//! via `parse::brew`).
//!
//! Refresh: `brew update` (if the setting is on; this is also brew's
//! self-update, phase-labeled) → `brew list --versions` → `brew list --cask
//! --versions` → `brew outdated --json=v2` → `brew outdated --json=v2
//! --greedy`. Greedy-only casks = greedy minus plain (set difference, never an
//! in-JSON heuristic — DECISIONS D7). `HOMEBREW_NO_AUTO_UPDATE=1` rides on
//! every brew command EXCEPT the explicit `brew update` spec (SPEC §5.2).

use std::collections::HashSet;
use std::time::Duration;

use crate::detect::DetectStatus;
use crate::error::PmError;
use crate::ipc::{ManagedBy, ManagerId, ManagerSnapshot, Package, PackageKind, SelfUpdateRoute};
use crate::managers::parse::{self, brew as parse_brew};
use crate::managers::{ExitClass, ManagerAdapter, PackageId, PlanOptions, PlannedCommand, Timeout};
use crate::process::CommandOutput;
use crate::registry::{now_rfc3339, pkg_name};
use crate::settings::Settings;

/// stderr signature of an externally-held Homebrew lock (DECISIONS D22).
pub const BREW_LOCK_SIGNATURE: &str = "Another active Homebrew";

/// Default stall/hard-cap pair for upgrade commands (SPEC §5.4: settings-driven
/// values are re-bound by the spec binder in U5; these are the F11 defaults).
pub(crate) fn default_upgrade_timeout() -> Timeout {
    Timeout::Stall {
        silence: Duration::from_secs(120),
        hard_cap: Duration::from_secs(30 * 60),
    }
}

fn no_auto_update() -> Vec<(String, String)> {
    vec![("HOMEBREW_NO_AUTO_UPDATE".into(), "1".into())]
}

fn argv(parts: &[&str]) -> Vec<String> {
    parts.iter().map(|s| s.to_string()).collect()
}

/// Maps a failed brew command's stderr to the distinct `BrewLockBusy` state
/// (SPEC §5.10; DECISIONS D22 — named, never auto-retried). The queue (U5)
/// consults this after a `Failure` classification; `None` means the generic
/// `NonZeroExit` path applies.
pub fn classify_brew_failure(out: &CommandOutput) -> Option<PmError> {
    if out.stderr.contains(BREW_LOCK_SIGNATURE) {
        let detail = out
            .stderr
            .lines()
            .find(|l| l.contains(BREW_LOCK_SIGNATURE))
            .unwrap_or(BREW_LOCK_SIGNATURE)
            .trim()
            .to_string();
        return Some(PmError::BrewLockBusy { detail });
    }
    None
}

pub struct BrewAdapter;

#[async_trait::async_trait]
impl ManagerAdapter for BrewAdapter {
    fn id(&self) -> ManagerId {
        ManagerId::Brew
    }

    fn display_name(&self) -> &'static str {
        "Homebrew"
    }

    fn binary_name(&self) -> &'static str {
        "brew"
    }

    fn detection_candidates(&self) -> &'static [&'static str] {
        &["/opt/homebrew/bin/brew", "/usr/local/bin/brew"]
    }

    fn refresh_plan(&self, det: &DetectStatus, settings: &Settings) -> Vec<PlannedCommand> {
        if matches!(det, DetectStatus::Absent { .. }) {
            return vec![];
        }
        let mut plan = Vec::with_capacity(5);
        if settings.run_brew_update_on_refresh {
            plan.push(PlannedCommand {
                label: "brew update",
                argv: argv(&["update"]),
                timeout: Timeout::Absolute(Duration::from_secs(600)),
                // The ONE brew command without HOMEBREW_NO_AUTO_UPDATE.
                extra_env: vec![],
                phase_label: Some("Updating Homebrew metadata…".into()),
            });
        }
        plan.push(PlannedCommand {
            label: "brew list --versions",
            argv: argv(&["list", "--versions"]),
            timeout: Timeout::Absolute(Duration::from_secs(60)),
            extra_env: no_auto_update(),
            phase_label: Some("Listing installed…".into()),
        });
        plan.push(PlannedCommand {
            label: "brew list --cask --versions",
            argv: argv(&["list", "--cask", "--versions"]),
            timeout: Timeout::Absolute(Duration::from_secs(60)),
            extra_env: no_auto_update(),
            phase_label: Some("Listing installed…".into()),
        });
        plan.push(PlannedCommand {
            label: "brew outdated --json=v2",
            argv: argv(&["outdated", "--json=v2"]),
            timeout: Timeout::Absolute(Duration::from_secs(120)),
            extra_env: no_auto_update(),
            phase_label: Some("Checking outdated…".into()),
        });
        plan.push(PlannedCommand {
            label: "brew outdated --json=v2 --greedy",
            argv: argv(&["outdated", "--json=v2", "--greedy"]),
            timeout: Timeout::Absolute(Duration::from_secs(120)),
            extra_env: no_auto_update(),
            phase_label: Some("Checking outdated…".into()),
        });
        plan
    }

    fn parse_refresh(&self, outputs: &[CommandOutput]) -> Result<ManagerSnapshot, PmError> {
        // 5 outputs when `brew update` ran first (its output is not parsed),
        // 4 when the setting is off.
        let data = match outputs.len() {
            5 => &outputs[1..],
            4 => outputs,
            n => {
                return Err(PmError::Internal {
                    detail: format!("brew parse_refresh expected 4 or 5 outputs, got {n}"),
                })
            }
        };
        let formulae_inv = parse_brew::parse_list_versions(&data[0].stdout);
        let casks_inv = parse_brew::parse_cask_versions(&data[1].stdout);
        let formulae_inv = parse::dedupe_formulae_against_casks(formulae_inv, &casks_inv);
        let plain = parse_brew::parse_outdated_json(&data[2].stdout)?;
        let greedy = parse_brew::parse_outdated_json(&data[3].stdout)?;
        let greedy_only = parse_brew::greedy_only(&plain.casks, &greedy.casks);

        // Greedy-only casks are re-kinded in the inventory too, so the merged
        // table carries exactly one row per cask (`caskGreedy:<name>`).
        let greedy_names: HashSet<&str> = greedy_only.iter().map(|p| p.name.as_str()).collect();
        let casks_inv: Vec<Package> = casks_inv
            .into_iter()
            .map(|mut c| {
                if greedy_names.contains(c.name.as_str()) {
                    c.kind = PackageKind::CaskGreedy;
                    c.id = format!("caskGreedy:{}", c.name);
                }
                c
            })
            .collect();

        let mut inventory = formulae_inv;
        inventory.extend(casks_inv);
        let mut overlay = plain.formulae;
        overlay.extend(plain.casks);
        overlay.extend(greedy_only);

        Ok(ManagerSnapshot {
            manager_id: ManagerId::Brew,
            refreshed_at: now_rfc3339(),
            packages: parse::merge_inventory_overlay(inventory, overlay),
            // brew has no latest concept — self-update is ViaRefresh (§5.8).
            self_status: None,
            health: vec![],
        })
    }

    fn recovery_plan(&self, failed: &PlannedCommand) -> Option<PlannedCommand> {
        // Cask JSON shape is UNVERIFIED; the fixture-tested greedy TEXT parser
        // is the wired recovery for either outdated-JSON spec (SPEC §5.5).
        if failed.argv.first().map(String::as_str) == Some("outdated")
            && failed.argv.iter().any(|a| a.starts_with("--json"))
        {
            return Some(PlannedCommand {
                label: "brew outdated --greedy",
                argv: argv(&["outdated", "--greedy"]),
                timeout: Timeout::Absolute(Duration::from_secs(120)),
                extra_env: no_auto_update(),
                phase_label: Some("Checking outdated…".into()),
            });
        }
        None
    }

    fn parse_recovery(
        &self,
        _failed: &PlannedCommand,
        out: &CommandOutput,
    ) -> Result<ManagerSnapshot, PmError> {
        let casks = parse_brew::parse_greedy_text(&out.stdout)?;
        Ok(ManagerSnapshot {
            manager_id: ManagerId::Brew,
            refreshed_at: now_rfc3339(),
            packages: casks,
            self_status: None,
            health: vec![],
        })
    }

    fn upgrade_plan(&self, pkgs: &[PackageId], opts: &PlanOptions) -> Vec<PlannedCommand> {
        let mut formulae: Vec<&str> = Vec::new();
        let mut casks: Vec<&str> = Vec::new();
        let mut greedy: Vec<&str> = Vec::new();
        for id in pkgs {
            let name = pkg_name(id);
            if id.starts_with("formula:") {
                formulae.push(name);
            } else if id.starts_with("cask:") {
                casks.push(name);
            } else if id.starts_with("caskGreedy:") && opts.include_greedy_casks {
                greedy.push(name);
            }
        }
        let mut plan = Vec::new();
        if !formulae.is_empty() {
            let mut a = argv(&["upgrade"]);
            a.extend(formulae.iter().map(|s| s.to_string()));
            plan.push(PlannedCommand {
                label: "brew upgrade",
                argv: a,
                timeout: default_upgrade_timeout(),
                extra_env: no_auto_update(),
                phase_label: None,
            });
        }
        if !casks.is_empty() {
            let mut a = argv(&["upgrade", "--cask"]);
            a.extend(casks.iter().map(|s| s.to_string()));
            plan.push(PlannedCommand {
                label: "brew upgrade --cask",
                argv: a,
                timeout: default_upgrade_timeout(),
                extra_env: no_auto_update(),
                phase_label: None,
            });
        }
        if !greedy.is_empty() {
            let mut a = argv(&["upgrade", "--cask", "--greedy"]);
            a.extend(greedy.iter().map(|s| s.to_string()));
            plan.push(PlannedCommand {
                label: "brew upgrade --cask --greedy",
                argv: a,
                timeout: default_upgrade_timeout(),
                extra_env: no_auto_update(),
                phase_label: None,
            });
        }
        plan
    }

    fn self_update_route(
        &self,
        _managed_by: ManagedBy,
        _own_outdated_row: Option<&Package>,
    ) -> SelfUpdateRoute {
        // brew never lists itself; `brew update` (part of every refresh) IS
        // its self-update (SPEC §5.3 native rule).
        SelfUpdateRoute::ViaRefresh {
            note: "brew update runs as part of every refresh".into(),
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
    use crate::detect::DetectStatus;
    use crate::events::{AppEvent, EventSink, OpStatusEvent, VecSink};
    use crate::ipc::{OpKind, OpStatus};
    use crate::process::fake::FakeRunner;
    use crate::process::{CmdPurpose, CommandRunner, CommandSpec};
    use std::path::PathBuf;

    fn present() -> DetectStatus {
        DetectStatus::Present {
            binary_path: PathBuf::from("/opt/homebrew/bin/brew"),
            canonical_path: PathBuf::from("/opt/homebrew/bin/brew"),
            version: Some("4.5.2".into()),
            managed_by: ManagedBy::Standalone,
            evidence: "resolved at /opt/homebrew/bin/brew — Homebrew's own tree".into(),
        }
    }

    fn spec_for(cmd: &PlannedCommand) -> CommandSpec {
        CommandSpec {
            program: PathBuf::from("/opt/homebrew/bin/brew"),
            args: cmd.argv.clone(),
            env: cmd.extra_env.clone(),
            timeout: cmd.timeout,
            purpose: CmdPurpose::Refresh,
        }
    }

    fn phase_event(label: &str) -> AppEvent {
        AppEvent::OpStatus(OpStatusEvent {
            op_id: "test-op".into(),
            kind: OpKind::Refresh,
            executor: ManagerId::Brew,
            subject: ManagerId::Brew,
            status: OpStatus::Running,
            queue_position: None,
            phase_label: Some(label.to_string()),
            command_line: String::new(),
            exit_code: None,
            error: None,
            started_at: None,
            finished_at: None,
            log_path: String::new(),
        })
    }

    #[test]
    fn refresh_plan_starts_with_brew_update_and_gates_on_setting() {
        let adapter = BrewAdapter;
        let plan = adapter.refresh_plan(&present(), &Settings::default());
        assert_eq!(plan.len(), 5);
        assert_eq!(plan[0].argv, vec!["update"]);
        assert!(
            plan[0].extra_env.is_empty(),
            "brew update must NOT carry HOMEBREW_NO_AUTO_UPDATE"
        );
        for cmd in &plan[1..] {
            assert!(
                cmd.extra_env
                    .iter()
                    .any(|(k, v)| k == "HOMEBREW_NO_AUTO_UPDATE" && v == "1"),
                "{} must carry HOMEBREW_NO_AUTO_UPDATE=1",
                cmd.label
            );
        }

        let off = Settings {
            run_brew_update_on_refresh: false,
            ..Settings::default()
        };
        let plan = adapter.refresh_plan(&present(), &off);
        assert_eq!(plan.len(), 4);
        assert_eq!(plan[0].argv, vec!["list", "--versions"]);
    }

    #[tokio::test]
    async fn brew_refresh_sequence_emits_phase_labels_and_parses_snapshot() {
        let fake = FakeRunner::new();
        fake.on("brew", &["update"]).ok("Already up-to-date.\n");
        fake.on("brew", &["list", "--versions"])
            .fixture("brew_list_versions_2026-07-22.txt");
        fake.on("brew", &["list", "--cask", "--versions"])
            .fixture("brew_list_cask_versions_2026-07-22.txt");
        fake.on("brew", &["outdated", "--json=v2"])
            .fixture("brew_outdated.json");
        fake.on("brew", &["outdated", "--json=v2", "--greedy"])
            .fixture("brew_outdated_greedy.json");

        let adapter = BrewAdapter;
        let sink = VecSink::new();
        let plan = adapter.refresh_plan(&present(), &Settings::default());

        // Serial spec execution as the queue (U5) will run it: emit the phase
        // label, then run the spec.
        let mut outputs = Vec::new();
        for cmd in &plan {
            if let Some(label) = &cmd.phase_label {
                sink.emit(phase_event(label));
            }
            outputs.push(fake.run(&spec_for(cmd)).await.expect("fake output"));
        }

        // The two-phase brew story in order: update (self-update phase) runs
        // FIRST, then inventory, then outdated.
        let calls = fake.calls();
        assert_eq!(calls[0].args, vec!["update"]);
        assert_eq!(calls[1].args, vec!["list", "--versions"]);
        let labels: Vec<String> = sink
            .events()
            .into_iter()
            .filter_map(|e| match e {
                AppEvent::OpStatus(s) => s.phase_label,
                _ => None,
            })
            .collect();
        assert_eq!(
            labels,
            vec![
                "Updating Homebrew metadata…",
                "Listing installed…",
                "Listing installed…",
                "Checking outdated…",
                "Checking outdated…",
            ]
        );

        let snapshot = adapter.parse_refresh(&outputs).expect("snapshot");
        assert_eq!(snapshot.manager_id, ManagerId::Brew);
        // 243 deduped formulae + 15 casks (verified fixture counts).
        assert_eq!(snapshot.packages.len(), 258);
        let dolt = snapshot
            .packages
            .iter()
            .find(|p| p.id == "formula:dolt")
            .expect("dolt row");
        assert!(dolt.outdated);
        assert_eq!(dolt.installed.as_deref(), Some("2.2.1"));
        assert_eq!(dolt.latest.as_deref(), Some("2.2.2"));
        assert!(snapshot.self_status.is_none(), "brew self is ViaRefresh");
    }

    #[tokio::test]
    async fn refresh_timeout_propagates_as_error_for_an_error_snapshot() {
        let fake = FakeRunner::new();
        fake.on("brew", &["update"]).fail(PmError::Timeout {
            after_secs: 600,
            phase: "brew update".into(),
        });
        let adapter = BrewAdapter;
        let plan = adapter.refresh_plan(&present(), &Settings::default());
        let err = fake.run(&spec_for(&plan[0])).await.unwrap_err();
        assert!(
            matches!(
                err,
                PmError::Timeout {
                    after_secs: 600,
                    ..
                }
            ),
            "queue maps this to the manager's error card; previous snapshot is retained"
        );
    }

    #[test]
    fn greedy_only_casks_are_rekinded_in_the_merged_table() {
        // Values copied verbatim from brew_outdated_greedy_text_2026-07-21.txt:
        // `openusage (0.6.20) != 0.7.6`. openusage is in the cask inventory.
        let outputs = vec![
            CommandOutput {
                exit_code: Some(0),
                stdout: crate::process::fake::fixture("brew_list_versions_2026-07-22.txt"),
                stderr: String::new(),
                duration: Duration::ZERO,
            },
            CommandOutput {
                exit_code: Some(0),
                stdout: crate::process::fake::fixture("brew_list_cask_versions_2026-07-22.txt"),
                stderr: String::new(),
                duration: Duration::ZERO,
            },
            CommandOutput {
                exit_code: Some(0),
                stdout: r#"{"formulae": [], "casks": []}"#.into(),
                stderr: String::new(),
                duration: Duration::ZERO,
            },
            CommandOutput {
                exit_code: Some(0),
                stdout: r#"{"formulae": [], "casks": [{"name": "openusage", "installed_versions": ["0.6.20"], "current_version": "0.7.6"}]}"#.into(),
                stderr: String::new(),
                duration: Duration::ZERO,
            },
        ];
        let snapshot = BrewAdapter.parse_refresh(&outputs).expect("snapshot");
        let openusage = snapshot
            .packages
            .iter()
            .find(|p| p.name == "openusage")
            .expect("openusage row");
        assert_eq!(openusage.kind, PackageKind::CaskGreedy);
        assert_eq!(openusage.id, "caskGreedy:openusage");
        assert!(openusage.outdated);
        assert_eq!(openusage.latest.as_deref(), Some("0.7.6"));
        // Exactly one openusage row — the plain-cask inventory row was re-kinded,
        // not duplicated.
        assert_eq!(
            snapshot
                .packages
                .iter()
                .filter(|p| p.name == "openusage")
                .count(),
            1
        );
    }

    #[test]
    fn recovery_plan_wires_greedy_text_for_outdated_json_only() {
        let adapter = BrewAdapter;
        let plan = adapter.refresh_plan(&present(), &Settings::default());
        let outdated_json = &plan[3];
        let recovery = adapter.recovery_plan(outdated_json).expect("recovery");
        assert_eq!(recovery.argv, vec!["outdated", "--greedy"]);
        assert!(
            adapter.recovery_plan(&plan[0]).is_none(),
            "brew update: none"
        );
        assert!(adapter.recovery_plan(&plan[1]).is_none(), "list: none");

        let out = CommandOutput {
            exit_code: Some(0),
            stdout: crate::process::fake::fixture("brew_outdated_greedy_text_2026-07-21.txt"),
            stderr: String::new(),
            duration: Duration::ZERO,
        };
        let snapshot = adapter.parse_recovery(&recovery, &out).expect("snapshot");
        assert_eq!(snapshot.packages.len(), 3);
        assert!(snapshot.packages.iter().all(|p| p.outdated));
    }

    #[test]
    fn upgrade_plan_splits_by_kind_and_omits_empty() {
        let adapter = BrewAdapter;
        let pkgs: Vec<PackageId> = vec![
            "formula:dolt".into(),
            "cask:ghostty".into(),
            "caskGreedy:openusage".into(),
            "formula:abseil".into(),
        ];
        let opts = PlanOptions {
            include_self_updates: true,
            include_greedy_casks: false,
        };
        let plan = adapter.upgrade_plan(&pkgs, &opts);
        assert_eq!(plan.len(), 2, "greedy omitted when not opted in");
        assert_eq!(plan[0].argv, vec!["upgrade", "dolt", "abseil"]);
        assert_eq!(plan[1].argv, vec!["upgrade", "--cask", "ghostty"]);

        let opts = PlanOptions {
            include_self_updates: true,
            include_greedy_casks: true,
        };
        let plan = adapter.upgrade_plan(&pkgs, &opts);
        assert_eq!(plan.len(), 3);
        assert_eq!(
            plan[2].argv,
            vec!["upgrade", "--cask", "--greedy", "openusage"]
        );
        assert!(matches!(plan[0].timeout, Timeout::Stall { .. }));
    }

    #[test]
    fn brew_lock_stderr_maps_to_brew_lock_busy() {
        let out = CommandOutput {
            exit_code: Some(1),
            stdout: String::new(),
            stderr: "Error: Another active Homebrew update process is already in progress.\n"
                .into(),
            duration: Duration::ZERO,
        };
        match classify_brew_failure(&out) {
            Some(PmError::BrewLockBusy { detail }) => {
                assert!(detail.contains("Another active Homebrew"))
            }
            other => panic!("expected BrewLockBusy, got {other:?}"),
        }
        let plain = CommandOutput {
            exit_code: Some(1),
            stdout: String::new(),
            stderr: "Error: something else\n".into(),
            duration: Duration::ZERO,
        };
        assert!(classify_brew_failure(&plain).is_none());
    }

    #[test]
    fn absent_brew_plans_nothing() {
        let det = DetectStatus::Absent {
            reason: "not found".into(),
        };
        assert!(BrewAdapter
            .refresh_plan(&det, &Settings::default())
            .is_empty());
    }
}
