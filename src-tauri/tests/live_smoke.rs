//! Live smoke tests (SPEC §7.3, IMPL_PLAN U9) — ALL `#[ignore]`.
//!
//! These run REAL commands against THIS machine (macOS, brew/mise/npm/uv/
//! rustup present, mas absent) and are developer-run only:
//!
//! ```sh
//! cd src-tauri && cargo test -- --ignored
//! ```
//!
//! They are never part of the default suite or CI — the default suites stay
//! offline and deterministic via the `CommandRunner` seam.

use std::time::Duration;

use pack_manager_lib::detect::detect_all;
use pack_manager_lib::ipc::{ManagedBy, ManagerId, ManagerStatus, SelfUpdateRoute};
use pack_manager_lib::managers::parse::brew::parse_outdated_json;
use pack_manager_lib::managers::Timeout;
use pack_manager_lib::paths::ToolEnv;
use pack_manager_lib::process::runner::RealRunner;
use pack_manager_lib::process::{CmdPurpose, CommandRunner, CommandSpec};

/// SPEC F1 acceptance on this machine: brew/mise/npm/uv/rustup present, mas
/// absent; managedBy: brew→standalone, mise→brew, npm→mise, uv→mise,
/// rustup→standalone; each with an evidence string; routes derived.
#[tokio::test]
#[ignore]
async fn live_detection_classifies_this_machine() {
    let env = ToolEnv::build().await;
    let runner = RealRunner::new();
    let outcome = detect_all(&env, &runner).await;

    let info = |id: ManagerId| {
        outcome
            .report
            .managers
            .iter()
            .find(|m| m.id == id)
            .unwrap_or_else(|| panic!("report always carries all six managers ({id})"))
    };

    // Present set (machine fact: mas is the only absent manager).
    for id in [
        ManagerId::Brew,
        ManagerId::Mise,
        ManagerId::Npm,
        ManagerId::Uv,
        ManagerId::Rustup,
    ] {
        let m = info(id);
        assert_eq!(
            m.status,
            ManagerStatus::Present,
            "{id} must be present on this machine"
        );
        assert!(
            m.version.is_some(),
            "{id} --version probe must yield a version"
        );
        assert!(
            m.evidence.as_deref().is_some_and(|e| !e.is_empty()),
            "{id} must carry an evidence string"
        );
        assert!(
            m.binary_path.is_some() && m.canonical_path.is_some(),
            "{id} must carry resolved paths"
        );
    }

    // Classifications (SPEC F1 acceptance, derived — never hardcoded).
    assert_eq!(info(ManagerId::Brew).managed_by, ManagedBy::Standalone);
    assert_eq!(info(ManagerId::Mise).managed_by, ManagedBy::Brew);
    assert_eq!(info(ManagerId::Npm).managed_by, ManagedBy::Mise);
    assert_eq!(info(ManagerId::Uv).managed_by, ManagedBy::Mise);
    assert_eq!(info(ManagerId::Rustup).managed_by, ManagedBy::Standalone);

    // The regression the raw-path rule exists for: npm/uv resolve at mise's
    // shims (which ARE symlinks to the mise binary — canonicalize-first would
    // say brew).
    for id in [ManagerId::Npm, ManagerId::Uv] {
        let evidence = info(id).evidence.as_deref().unwrap();
        assert!(
            evidence.contains(".local/share/mise/"),
            "{id} evidence must point at mise's tree: {evidence}"
        );
    }

    // Routes (SPEC F6; npm has no outdated listing at detection time, so it
    // delegates to mise until a refresh reveals the in-band override).
    match &info(ManagerId::Mise).self_update {
        SelfUpdateRoute::Routed {
            executor,
            command_preview,
            ..
        } => {
            assert_eq!(*executor, ManagerId::Brew);
            assert_eq!(command_preview, "brew upgrade mise");
        }
        other => panic!("mise must route via brew, got {other:?}"),
    }
    match &info(ManagerId::Uv).self_update {
        SelfUpdateRoute::Routed {
            executor,
            command_preview,
            ..
        } => {
            assert_eq!(*executor, ManagerId::Mise);
            assert_eq!(command_preview, "mise upgrade uv");
        }
        other => panic!("uv must route via mise, got {other:?}"),
    }
    match &info(ManagerId::Rustup).self_update {
        SelfUpdateRoute::InBand {
            command_preview, ..
        } => assert_eq!(command_preview, "rustup self update"),
        other => panic!("rustup must self-update in-band, got {other:?}"),
    }

    // mas: absent is a normal state with an install hint (SPEC F1).
    let mas = info(ManagerId::Mas);
    assert_eq!(
        mas.status,
        ManagerStatus::Absent,
        "machine fact: mas is not installed"
    );
    assert_eq!(mas.install_hint.as_deref(), Some("brew install mas"));
    assert!(matches!(
        mas.self_update,
        SelfUpdateRoute::Unavailable { .. }
    ));
}

/// Real `brew outdated --json=v2` round-trip: the live output parses with the
/// production parser (junk-line tolerant) and the resulting wire `Package`s
/// survive an IPC serialize→deserialize cycle byte-identically.
#[tokio::test]
#[ignore]
async fn live_brew_outdated_json_round_trips() {
    let env = ToolEnv::build().await;
    let brew = env
        .require("brew")
        .expect("machine fact: brew is installed");

    // Mirror the adapter's spec: constructed child env + NO_AUTO_UPDATE (only
    // the explicit `brew update` spec omits it) + the SPEC §5.4 120s timeout.
    let mut child_env = env.child_env();
    child_env.push(("HOMEBREW_NO_AUTO_UPDATE".into(), "1".into()));
    let spec = CommandSpec {
        program: brew,
        args: vec!["outdated".into(), "--json=v2".into()],
        env: child_env,
        timeout: Timeout::Absolute(Duration::from_secs(120)),
        purpose: CmdPurpose::Refresh,
    };

    let runner = RealRunner::new();
    let out = runner
        .run(&spec, tokio_util::sync::CancellationToken::new())
        .await
        .expect("brew outdated must spawn");
    assert_eq!(
        out.exit_code,
        Some(0),
        "brew outdated --json=v2 exits 0; stderr: {}",
        out.stderr
    );

    let parsed = parse_outdated_json(&out.stdout).expect("live brew JSON must parse");
    for pkg in parsed.formulae.iter().chain(parsed.casks.iter()) {
        assert!(!pkg.name.is_empty());
        assert!(pkg.outdated, "everything brew reports outdated is outdated");
        assert!(
            pkg.installed.is_some(),
            "brew always reports installed_versions: {}",
            pkg.name
        );
    }

    // IPC round-trip on the real data (wire types, camelCase serde).
    let all: Vec<_> = parsed
        .formulae
        .iter()
        .chain(parsed.casks.iter())
        .cloned()
        .collect();
    let json = serde_json::to_string(&all).expect("wire Package serializes");
    let back: Vec<pack_manager_lib::ipc::Package> =
        serde_json::from_str(&json).expect("wire Package deserializes");
    assert_eq!(all, back, "IPC round-trip must be lossless");

    println!(
        "live brew outdated: {} formulae, {} casks",
        parsed.formulae.len(),
        parsed.casks.len()
    );
}
