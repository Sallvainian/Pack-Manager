//! Detection & managed-by classification (SPEC §5.3) — implemented by U4.
//!
//! `classify_managed_by` is a pure ordered-rule function whose FIRST rule
//! checks the RAW resolved path against mise's directories BEFORE any
//! canonicalization — mise shims ARE symlinks to the mise binary itself, so
//! canonicalize-first would misclassify uv/npm as brew-managed and misroute
//! their self-updates (DECISIONS D3, invariant #2).
//!
//! Detection orchestration resolves each adapter's binary on the constructed
//! search path (`which_in` via `ToolEnv`), falls back to fixed candidate
//! paths, probes `--version` (10s, via `CommandRunner`), classifies, and
//! resolves the self-update route with the SPEC §5.3 precedence:
//! in-band override → delegated-if-detected → native → unavailable.

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::Duration;

use crate::ipc::{
    DetectionReport, ManagedBy, ManagerId, ManagerInfo, ManagerStatus, Package, SelfUpdateRoute,
};
use crate::managers::brew::BrewAdapter;
use crate::managers::mas::MasAdapter;
use crate::managers::mise::MiseAdapter;
use crate::managers::npm::NpmAdapter;
use crate::managers::rustup::RustupAdapter;
use crate::managers::uv::UvAdapter;
use crate::managers::ManagerAdapter;
use crate::paths::ToolEnv;
use crate::process::{CmdPurpose, CommandRunner, CommandSpec, Timeout};

/// `--version` probe timeout (SPEC F1).
pub const VERSION_PROBE_TIMEOUT: Duration = Duration::from_secs(10);

/// Per-manager detection outcome. Absence is a normal state, never an error.
#[derive(Debug, Clone, PartialEq)]
pub enum DetectStatus {
    Absent {
        reason: String,
    },
    Present {
        binary_path: PathBuf,
        canonical_path: PathBuf,
        version: Option<String>,
        managed_by: ManagedBy,
        evidence: String,
    },
}

/// All six adapters in `ManagerId::ALL` order (SPEC F1).
pub fn all_adapters() -> Vec<Arc<dyn ManagerAdapter>> {
    vec![
        Arc::new(BrewAdapter),
        Arc::new(MiseAdapter),
        Arc::new(NpmAdapter),
        Arc::new(UvAdapter),
        Arc::new(RustupAdapter),
        Arc::new(MasAdapter),
    ]
}

/// The adapter for one manager id.
pub fn adapter_for(id: ManagerId) -> Arc<dyn ManagerAdapter> {
    match id {
        ManagerId::Brew => Arc::new(BrewAdapter),
        ManagerId::Mise => Arc::new(MiseAdapter),
        ManagerId::Npm => Arc::new(NpmAdapter),
        ManagerId::Uv => Arc::new(UvAdapter),
        ManagerId::Rustup => Arc::new(RustupAdapter),
        ManagerId::Mas => Arc::new(MasAdapter),
    }
}

/// Abbreviates `home` to `~` for evidence strings.
fn abbrev_home(path: &Path, home: &Path) -> String {
    match path.strip_prefix(home) {
        Ok(rest) => format!("~/{}", rest.display()),
        Err(_) => path.display().to_string(),
    }
}

/// PURE ordered-rule classification (SPEC §5.3). Returns the owning manager
/// and a human-readable evidence string.
///
/// 1. RAW resolved path under `{home}/.local/share/mise/shims/` or
///    `{home}/.local/share/mise/installs/` → `Mise`. Checked BEFORE
///    canonicalization (the shim location IS the evidence).
/// 2. Else canonicalize (best-effort; a non-existent path stays raw); under
///    `/opt/homebrew/`, `/usr/local/Cellar/`, or `/usr/local/Homebrew/` →
///    `Brew`.
/// 3. Under `{home}/.cargo/bin/` → `Rustup`.
/// 4. (Applied by [`classify_for_manager`]: the classified owner IS the
///    manager being classified → `Standalone`.)
/// 5. Else `Standalone`.
pub fn classify_managed_by(resolved: &Path, home: &Path) -> (ManagedBy, String) {
    let shims = home.join(".local/share/mise/shims");
    let installs = home.join(".local/share/mise/installs");
    if resolved.starts_with(&shims) || resolved.starts_with(&installs) {
        return (
            ManagedBy::Mise,
            format!("resolved at {}", abbrev_home(resolved, home)),
        );
    }

    let canonical = resolved
        .canonicalize()
        .unwrap_or_else(|_| resolved.to_path_buf());

    let brew_roots = ["/opt/homebrew", "/usr/local/Cellar", "/usr/local/Homebrew"];
    if brew_roots.iter().any(|r| canonical.starts_with(r)) {
        let evidence = if canonical == resolved {
            format!(
                "resolved at {} — under Homebrew's tree",
                abbrev_home(resolved, home)
            )
        } else {
            format!(
                "resolved at {} → {} — under Homebrew's tree",
                abbrev_home(resolved, home),
                abbrev_home(&canonical, home)
            )
        };
        return (ManagedBy::Brew, evidence);
    }

    if canonical.starts_with(home.join(".cargo/bin")) {
        return (
            ManagedBy::Rustup,
            format!(
                "resolved at {} — under ~/.cargo/bin",
                abbrev_home(resolved, home)
            ),
        );
    }

    (
        ManagedBy::Standalone,
        format!("resolved at {}", abbrev_home(resolved, home)),
    )
}

/// SPEC §5.3 rule 4 helper: the tree a manager calls its own. When a
/// manager's binary classifies to ITSELF as owner (brew under
/// `/opt/homebrew`, rustup under `~/.cargo/bin`), it is standalone — a
/// manager does not manage itself.
fn own_tree_label(id: ManagerId, owner: ManagedBy) -> Option<&'static str> {
    match (id, owner) {
        (ManagerId::Brew, ManagedBy::Brew) => Some("Homebrew's own tree"),
        (ManagerId::Mise, ManagedBy::Mise) => Some("mise's own tree"),
        (ManagerId::Rustup, ManagedBy::Rustup) => Some("rustup's own tree"),
        _ => None,
    }
}

/// [`classify_managed_by`] plus SPEC §5.3 rule 4: when the classified owner IS
/// the manager being classified (brew under `/opt/homebrew`, rustup under
/// `~/.cargo/bin`), it is `Standalone` — a manager does not manage itself.
pub fn classify_for_manager(id: ManagerId, resolved: &Path, home: &Path) -> (ManagedBy, String) {
    let (owner, evidence) = classify_managed_by(resolved, home);
    match own_tree_label(id, owner) {
        Some(tree) => (
            ManagedBy::Standalone,
            format!("resolved at {} — {}", abbrev_home(resolved, home), tree),
        ),
        None => (owner, evidence),
    }
}

/// Self-update route precedence (SPEC §5.3): the adapter applies the in-band
/// override and its native/delegated rules; this wrapper enforces
/// "delegated-if-DETECTED" — a route to an absent executor falls through to
/// the adapter's native (standalone) route.
pub fn resolve_route(
    adapter: &dyn ManagerAdapter,
    managed_by: ManagedBy,
    own_outdated_row: Option<&Package>,
    present: &BTreeSet<ManagerId>,
) -> SelfUpdateRoute {
    match adapter.self_update_route(managed_by, own_outdated_row) {
        SelfUpdateRoute::Routed { executor, .. } if !present.contains(&executor) => {
            adapter.self_update_route(ManagedBy::Standalone, own_outdated_row)
        }
        route => route,
    }
}

/// Detection outcome: the wire report plus the internal per-manager statuses
/// (`refresh_plan` takes `&DetectStatus`).
#[derive(Debug, Clone, PartialEq)]
pub struct DetectionOutcome {
    pub report: DetectionReport,
    pub statuses: BTreeMap<ManagerId, DetectStatus>,
}

impl DetectionOutcome {
    /// Present managers as a set (route resolution, refresh fan-out).
    pub fn present(&self) -> BTreeSet<ManagerId> {
        self.statuses
            .iter()
            .filter(|(_, s)| matches!(s, DetectStatus::Present { .. }))
            .map(|(id, _)| *id)
            .collect()
    }
}

/// Install hint for an absent manager (SPEC F1 names only mas's hint).
fn install_hint(id: ManagerId) -> Option<String> {
    match id {
        ManagerId::Mas => Some("brew install mas".into()),
        _ => None,
    }
}

/// Resolves one adapter's binary: `which_in` on OUR search path first, then
/// the fixed candidate paths (`~/` expanded against `env.home`).
///
/// **Shim-vs-own-tree preference**: mise's shims proxy tools mise manages
/// through the mise binary — including `rustup`/`cargo` shims created for
/// mise's rust toolchain. When the `which` hit for a MANAGER's binary is such
/// a shim but the manager has its own real installation in its own tree (a
/// fixed candidate that classifies to the manager itself — SPEC §5.3 rule 4),
/// the real binary wins: SPEC §2's machine facts pin rustup standalone at
/// `~/.cargo/bin` while mise's rust tool shims it. npm/uv exist nowhere
/// outside mise's tree, so their shim resolution (managed by mise) stands —
/// this preference can never reroute a manager to ANOTHER manager's tree.
fn resolve_binary(env: &ToolEnv, adapter: &dyn ManagerAdapter) -> Option<PathBuf> {
    let expand = |candidate: &str| match candidate.strip_prefix("~/") {
        Some(rest) => env.home.join(rest),
        None => PathBuf::from(candidate),
    };
    if let Some(found) = env.which(adapter.binary_name()) {
        let shims = env.home.join(".local/share/mise/shims");
        if found.starts_with(&shims) {
            for candidate in adapter.detection_candidates() {
                let path = expand(candidate);
                if path.is_file() {
                    let (owner, _) = classify_managed_by(&path, &env.home);
                    if own_tree_label(adapter.id(), owner).is_some() {
                        tracing::info!(
                            manager = %adapter.id(),
                            shim = %found.display(),
                            preferred = %path.display(),
                            "which hit a mise shim; preferring the manager's own-tree binary"
                        );
                        return Some(path);
                    }
                }
            }
        }
        return Some(found);
    }
    for candidate in adapter.detection_candidates() {
        let path = expand(candidate);
        if path.is_file() {
            return Some(path);
        }
    }
    None
}

/// First plausible version token of a `--version` probe: the first
/// whitespace-separated token of the first line that starts with an ASCII
/// digit (`Homebrew 4.5.2` → `4.5.2`; `11.16.0` → `11.16.0`), else the whole
/// first line verbatim.
fn extract_version(stdout: &str) -> Option<String> {
    let first = stdout.lines().next()?.trim();
    if first.is_empty() {
        return None;
    }
    first
        .split_whitespace()
        .find(|t| t.chars().next().is_some_and(|c| c.is_ascii_digit()))
        .map(str::to_string)
        .or_else(|| Some(first.to_string()))
}

/// Detects all six managers (SPEC F1): resolve → probe `--version` (10s) →
/// classify → route (with the detected set). Also serves Re-detect; the
/// caller rebuilds the `ToolEnv` first.
pub async fn detect_all(env: &ToolEnv, runner: &dyn CommandRunner) -> DetectionOutcome {
    let adapters = all_adapters();
    let mut statuses: BTreeMap<ManagerId, DetectStatus> = BTreeMap::new();

    for adapter in &adapters {
        let id = adapter.id();
        let Some(resolved) = resolve_binary(env, adapter.as_ref()) else {
            let reason = format!("`{}` not found on the search path", adapter.binary_name());
            tracing::info!(manager = %id, %reason, "detection: absent");
            statuses.insert(id, DetectStatus::Absent { reason });
            continue;
        };

        let spec = CommandSpec {
            program: resolved.clone(),
            args: vec!["--version".into()],
            env: env.child_env(),
            timeout: Timeout::Absolute(VERSION_PROBE_TIMEOUT),
            purpose: CmdPurpose::Detection,
        };
        let version = match runner.run(&spec).await {
            Ok(out) if out.exit_code == Some(0) => extract_version(&out.stdout),
            Ok(out) => {
                tracing::warn!(manager = %id, exit = ?out.exit_code, "version probe non-zero exit");
                None
            }
            Err(e) => {
                tracing::warn!(manager = %id, error = %e, "version probe failed");
                None
            }
        };

        let (managed_by, evidence) = classify_for_manager(id, &resolved, &env.home);
        let canonical_path = resolved.canonicalize().unwrap_or_else(|_| resolved.clone());
        tracing::info!(
            manager = %id,
            path = %resolved.display(),
            canonical = %canonical_path.display(),
            managed_by = ?managed_by,
            %evidence,
            "detection: present"
        );
        statuses.insert(
            id,
            DetectStatus::Present {
                binary_path: resolved,
                canonical_path,
                version,
                managed_by,
                evidence,
            },
        );
    }

    // Routes are resolved AFTER all managers are known (delegated-if-detected).
    let present: BTreeSet<ManagerId> = statuses
        .iter()
        .filter(|(_, s)| matches!(s, DetectStatus::Present { .. }))
        .map(|(id, _)| *id)
        .collect();

    let mut managers = Vec::with_capacity(adapters.len());
    for adapter in &adapters {
        let id = adapter.id();
        let info = match statuses.get(&id) {
            Some(DetectStatus::Present {
                binary_path,
                canonical_path,
                version,
                managed_by,
                evidence,
            }) => {
                // No outdated data exists at detection time; routes are
                // re-checked each refresh with the manager's own listing.
                let route = resolve_route(adapter.as_ref(), *managed_by, None, &present);
                tracing::info!(manager = %id, route = ?route, "detection: route resolved");
                ManagerInfo {
                    id,
                    display_name: adapter.display_name().to_string(),
                    status: ManagerStatus::Present,
                    binary_path: Some(binary_path.to_string_lossy().into_owned()),
                    canonical_path: Some(canonical_path.to_string_lossy().into_owned()),
                    version: version.clone(),
                    managed_by: *managed_by,
                    evidence: Some(evidence.clone()),
                    self_update: route,
                    install_hint: None,
                }
            }
            _ => ManagerInfo {
                id,
                display_name: adapter.display_name().to_string(),
                status: ManagerStatus::Absent,
                binary_path: None,
                canonical_path: None,
                version: None,
                managed_by: ManagedBy::Standalone,
                evidence: None,
                self_update: SelfUpdateRoute::Unavailable {
                    reason: format!("{} is not installed", adapter.binary_name()),
                },
                install_hint: install_hint(id),
            },
        };
        managers.push(info);
    }

    DetectionOutcome {
        report: DetectionReport {
            managers,
            env: env.env_info(),
        },
        statuses,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ipc::PackageKind;
    use crate::process::fake::FakeRunner;

    fn home() -> PathBuf {
        PathBuf::from("/Users/testuser")
    }

    fn present_all() -> BTreeSet<ManagerId> {
        ManagerId::ALL.into_iter().collect()
    }

    // -----------------------------------------------------------------------
    // classify_managed_by (SPEC §7.2)
    // -----------------------------------------------------------------------

    /// THE regression test (DECISIONS D3): a mise shim that is a REAL symlink
    /// to a binary OUTSIDE mise's tree must classify as Mise from the RAW
    /// path — canonicalize-first would follow the symlink and misroute.
    #[test]
    fn classify_mise_shim_path_is_mise_managed_without_canonicalizing() {
        use std::os::unix::fs::PermissionsExt;
        let tmp = tempfile::tempdir().unwrap();
        let home = tmp.path().join("home");
        let shims = home.join(".local/share/mise/shims");
        std::fs::create_dir_all(&shims).unwrap();

        // The "brew-installed mise" the shim actually points at.
        let brewish = tmp.path().join("brewbin");
        std::fs::create_dir_all(&brewish).unwrap();
        let mise_bin = brewish.join("mise");
        std::fs::write(&mise_bin, "#!/bin/sh\nexit 0\n").unwrap();
        std::fs::set_permissions(&mise_bin, std::fs::Permissions::from_mode(0o755)).unwrap();
        let shim = shims.join("uv");
        std::os::unix::fs::symlink(&mise_bin, &shim).unwrap();

        // Sanity: canonicalizing WOULD leave mise's directory entirely.
        let canonical = shim.canonicalize().unwrap();
        assert!(!canonical.starts_with(&shims), "shim resolves elsewhere");

        let (owner, evidence) = classify_managed_by(&shim, &home);
        assert_eq!(owner, ManagedBy::Mise);
        assert!(
            evidence.contains("~/.local/share/mise/shims/uv"),
            "the shim location is the evidence: {evidence}"
        );
    }

    #[test]
    fn classify_mise_installs_dir_is_mise_managed() {
        let (owner, _) = classify_managed_by(
            &home().join(".local/share/mise/installs/node/24.18.0/bin/node"),
            &home(),
        );
        assert_eq!(owner, ManagedBy::Mise);
    }

    #[test]
    fn classify_opt_homebrew_canonical_is_brew() {
        let (owner, evidence) = classify_managed_by(Path::new("/opt/homebrew/bin/mise"), &home());
        assert_eq!(owner, ManagedBy::Brew);
        assert!(evidence.contains("/opt/homebrew/bin/mise"), "{evidence}");
    }

    #[test]
    fn classify_cargo_bin_is_rustup() {
        let (owner, evidence) = classify_managed_by(&home().join(".cargo/bin/cargo"), &home());
        assert_eq!(owner, ManagedBy::Rustup);
        assert!(evidence.contains("~/.cargo/bin/cargo"), "{evidence}");
    }

    #[test]
    fn classify_brew_itself_is_standalone() {
        let (owner, evidence) = classify_for_manager(
            ManagerId::Brew,
            Path::new("/opt/homebrew/bin/brew"),
            &home(),
        );
        assert_eq!(owner, ManagedBy::Standalone);
        assert!(evidence.contains("Homebrew's own tree"), "{evidence}");

        // rustup under ~/.cargo/bin is likewise its own tree.
        let (owner, _) = classify_for_manager(
            ManagerId::Rustup,
            &home().join(".cargo/bin/rustup"),
            &home(),
        );
        assert_eq!(owner, ManagedBy::Standalone);
    }

    #[test]
    fn classify_unknown_is_standalone() {
        let (owner, evidence) = classify_managed_by(Path::new("/usr/bin/python3"), &home());
        assert_eq!(owner, ManagedBy::Standalone);
        assert!(evidence.contains("/usr/bin/python3"));
    }

    /// The counterfactual proving routing is derived, never hardcoded: a
    /// standalone uv in `~/.local/bin` classifies Standalone and routes
    /// in-band (`uv self update`), NOT via mise.
    #[test]
    fn classify_standalone_uv_in_local_bin_routes_in_band() {
        let (owner, _) =
            classify_for_manager(ManagerId::Uv, &home().join(".local/bin/uv"), &home());
        assert_eq!(owner, ManagedBy::Standalone);
        let route = resolve_route(&UvAdapter, owner, None, &present_all());
        match route {
            SelfUpdateRoute::InBand {
                command_preview, ..
            } => assert_eq!(command_preview, "uv self update"),
            other => panic!("expected InBand, got {other:?}"),
        }
    }

    // -----------------------------------------------------------------------
    // Route precedence (SPEC §7.2)
    // -----------------------------------------------------------------------

    #[test]
    fn npm_in_band_override_wins_over_delegation() {
        let own = Package {
            id: "globalPackage:npm".into(),
            name: "npm".into(),
            kind: PackageKind::GlobalPackage,
            installed: Some("11.16.0".into()),
            latest: Some("12.0.1".into()),
            outdated: true,
            pinned: false,
            meta: None,
        };
        let route = resolve_route(&NpmAdapter, ManagedBy::Mise, Some(&own), &present_all());
        match route {
            SelfUpdateRoute::InBand {
                command_preview, ..
            } => assert_eq!(command_preview, "npm install -g npm@latest"),
            other => panic!("in-band override must win, got {other:?}"),
        }
    }

    #[test]
    fn mise_routes_via_brew_when_brew_detected() {
        let route = resolve_route(&MiseAdapter, ManagedBy::Brew, None, &present_all());
        match route {
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
    }

    #[test]
    fn mise_falls_through_to_in_band_when_brew_absent() {
        let present: BTreeSet<ManagerId> = [ManagerId::Mise, ManagerId::Npm, ManagerId::Uv]
            .into_iter()
            .collect();
        let route = resolve_route(&MiseAdapter, ManagedBy::Brew, None, &present);
        match route {
            SelfUpdateRoute::InBand {
                command_preview, ..
            } => assert_eq!(command_preview, "mise self-update"),
            other => panic!("expected native fall-through, got {other:?}"),
        }
    }

    // -----------------------------------------------------------------------
    // Detection orchestration (FakeRunner; real files on a temp ToolEnv)
    // -----------------------------------------------------------------------

    fn make_exec(path: &Path) {
        use std::os::unix::fs::PermissionsExt;
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        std::fs::write(path, "#!/bin/sh\nexit 0\n").unwrap();
        std::fs::set_permissions(path, std::fs::Permissions::from_mode(0o755)).unwrap();
    }

    #[tokio::test]
    async fn detect_all_classifies_routes_and_never_probes_absent_mas() {
        use crate::ipc::PathSource;

        let tmp = tempfile::tempdir().unwrap();
        let home = tmp.path().join("home");
        let shims = home.join(".local/share/mise/shims");
        let bin = tmp.path().join("bin");
        let cargo_bin = home.join(".cargo/bin");

        make_exec(&bin.join("brew"));
        make_exec(&bin.join("mise"));
        make_exec(&cargo_bin.join("rustup"));
        // npm/uv are REAL symlinks to the mise binary, as on the target machine.
        std::fs::create_dir_all(&shims).unwrap();
        std::os::unix::fs::symlink(bin.join("mise"), shims.join("npm")).unwrap();
        std::os::unix::fs::symlink(bin.join("mise"), shims.join("uv")).unwrap();
        // mas: nowhere.

        let env = ToolEnv::from_entries(
            home.clone(),
            vec![shims.clone(), bin.clone(), cargo_bin.clone()],
            PathSource::StaticFallback,
        );

        let fake = FakeRunner::new();
        fake.on("brew", &["--version"]).ok("Homebrew 4.5.2\n");
        fake.on("mise", &["--version"]).ok("2026.1.5 macos-arm64\n");
        fake.on("npm", &["--version"]).ok("11.16.0\n");
        fake.on("uv", &["--version"])
            .ok("uv 0.11.26 (abc123 2026-06-01)\n");
        fake.on("rustup", &["--version"])
            .ok("rustup 1.29.0 (2026-05-01)\n");
        // Defensive rule: if a machine WITH mas ever runs this suite, the
        // machine-fact assertion below fails cleanly instead of panicking.
        fake.on("mas", &["--version"]).ok("1.9.0\n");

        let outcome = detect_all(&env, &fake).await;

        // mas never reached the runner.
        assert!(
            fake.calls().iter().all(|c| c.basename != "mas"),
            "absent mas must never be probed"
        );

        let by_id = |id: ManagerId| -> &ManagerInfo {
            outcome.report.managers.iter().find(|m| m.id == id).unwrap()
        };

        let brew = by_id(ManagerId::Brew);
        assert_eq!(brew.status, ManagerStatus::Present);
        assert_eq!(brew.version.as_deref(), Some("4.5.2"));

        let npm = by_id(ManagerId::Npm);
        assert_eq!(npm.managed_by, ManagedBy::Mise);
        assert_eq!(npm.version.as_deref(), Some("11.16.0"));
        assert!(npm.evidence.as_deref().unwrap().contains("mise/shims/npm"));
        // No outdated data at detection time → delegated to the detected mise.
        match &npm.self_update {
            SelfUpdateRoute::Routed { executor, .. } => assert_eq!(*executor, ManagerId::Mise),
            other => panic!("expected Routed, got {other:?}"),
        }

        let uv = by_id(ManagerId::Uv);
        assert_eq!(uv.managed_by, ManagedBy::Mise);
        assert_eq!(uv.version.as_deref(), Some("0.11.26"));
        match &uv.self_update {
            SelfUpdateRoute::Routed {
                executor,
                command_preview,
                ..
            } => {
                assert_eq!(*executor, ManagerId::Mise);
                assert_eq!(command_preview, "mise upgrade uv");
            }
            other => panic!("expected Routed, got {other:?}"),
        }

        let rustup = by_id(ManagerId::Rustup);
        assert_eq!(
            rustup.managed_by,
            ManagedBy::Standalone,
            "own tree = standalone"
        );
        match &rustup.self_update {
            SelfUpdateRoute::InBand {
                command_preview, ..
            } => assert_eq!(command_preview, "rustup self update"),
            other => panic!("expected InBand, got {other:?}"),
        }

        let mas = by_id(ManagerId::Mas);
        assert_eq!(mas.status, ManagerStatus::Absent);
        assert_eq!(mas.install_hint.as_deref(), Some("brew install mas"));
        assert!(matches!(
            mas.self_update,
            SelfUpdateRoute::Unavailable { .. }
        ));
        assert!(matches!(
            outcome.statuses[&ManagerId::Mas],
            DetectStatus::Absent { .. }
        ));

        // Present set drives refresh fan-out.
        let present = outcome.present();
        assert_eq!(present.len(), 5);
        assert!(!present.contains(&ManagerId::Mas));
    }

    /// Machine fact (SPEC §2): mise's rust toolchain SHIMS `rustup`, while the
    /// real standalone rustup lives at `~/.cargo/bin/rustup`. Even with the
    /// shim first on the search path, rustup must resolve to its own tree and
    /// classify Standalone (SPEC F1: rustup→standalone) — while npm/uv, which
    /// exist nowhere outside mise's tree, keep their shim resolution.
    #[tokio::test]
    async fn rustup_prefers_own_tree_binary_over_mise_shim() {
        use crate::ipc::PathSource;

        let tmp = tempfile::tempdir().unwrap();
        // Canonicalize the root: on macOS `/var/folders/…` canonicalizes to
        // `/private/var/folders/…`, and the own-tree rule compares the
        // CANONICAL binary path against `{home}/.cargo/bin`.
        let root = tmp.path().canonicalize().unwrap();
        let home = root.join("home");
        let shims = home.join(".local/share/mise/shims");
        let bin = root.join("bin");
        let cargo_bin = home.join(".cargo/bin");

        make_exec(&bin.join("mise"));
        make_exec(&cargo_bin.join("rustup"));
        std::fs::create_dir_all(&shims).unwrap();
        // As on the target machine: rustup/npm/uv shims all point at mise.
        for shim in ["rustup", "npm", "uv"] {
            std::os::unix::fs::symlink(bin.join("mise"), shims.join(shim)).unwrap();
        }

        // Shims FIRST — the SPEC §5.2 static ordering.
        let env = ToolEnv::from_entries(
            home.clone(),
            vec![shims.clone(), bin.clone(), cargo_bin.clone()],
            PathSource::StaticFallback,
        );

        let fake = FakeRunner::new();
        for tool in ["brew", "mise", "npm", "uv", "rustup", "mas"] {
            fake.on(tool, &["--version"]).ok("1.0.0\n");
        }

        let outcome = detect_all(&env, &fake).await;
        let by_id = |id: ManagerId| -> &ManagerInfo {
            outcome.report.managers.iter().find(|m| m.id == id).unwrap()
        };

        let rustup = by_id(ManagerId::Rustup);
        assert_eq!(rustup.status, ManagerStatus::Present);
        assert_eq!(
            rustup.managed_by,
            ManagedBy::Standalone,
            "own-tree binary must win over the mise shim"
        );
        assert_eq!(
            rustup.binary_path.as_deref(),
            Some(cargo_bin.join("rustup").to_str().unwrap()),
            "spawns must use the real rustup, not the shim proxy"
        );
        assert!(
            rustup.evidence.as_deref().unwrap().contains("own tree"),
            "{:?}",
            rustup.evidence
        );

        // npm/uv have no own tree — shim resolution (managed by mise) stands.
        for id in [ManagerId::Npm, ManagerId::Uv] {
            assert_eq!(by_id(id).managed_by, ManagedBy::Mise, "{id} stays mise");
        }
    }

    #[tokio::test]
    async fn probe_failure_keeps_manager_present_without_version() {
        use crate::error::PmError;
        use crate::ipc::PathSource;

        let tmp = tempfile::tempdir().unwrap();
        let home = tmp.path().join("home");
        let bin = tmp.path().join("bin");
        make_exec(&bin.join("brew"));

        let env = ToolEnv::from_entries(home, vec![bin], PathSource::StaticFallback);
        let fake = FakeRunner::new();
        fake.on("brew", &["--version"]).fail(PmError::Timeout {
            after_secs: 10,
            phase: "brew --version".into(),
        });
        // Fixed-path detection candidates may exist on the machine running
        // this suite (e.g. /opt/homebrew/bin/mise here); give every other
        // manager a permissive probe so only brew's outcome is asserted.
        for tool in ["mise", "npm", "uv", "rustup", "mas"] {
            fake.on(tool, &["--version"]).ok("1.0.0\n");
        }

        let outcome = detect_all(&env, &fake).await;
        let brew = outcome
            .report
            .managers
            .iter()
            .find(|m| m.id == ManagerId::Brew)
            .unwrap();
        assert_eq!(brew.status, ManagerStatus::Present, "binary exists");
        assert_eq!(brew.version, None);
    }

    #[test]
    fn extract_version_takes_first_numeric_token() {
        assert_eq!(
            extract_version("Homebrew 4.5.2\n").as_deref(),
            Some("4.5.2")
        );
        assert_eq!(extract_version("11.16.0\n").as_deref(), Some("11.16.0"));
        assert_eq!(
            extract_version("uv 0.11.26 (abc123 2026-06-01)\n").as_deref(),
            Some("0.11.26")
        );
        assert_eq!(
            extract_version("rustup 1.29.0 (2026-05-01)\n").as_deref(),
            Some("1.29.0")
        );
        assert_eq!(extract_version("stable\n").as_deref(), Some("stable"));
        assert_eq!(extract_version(""), None);
    }
}
