//! Snapshot registry & cross-manager self-version join (SPEC §5.8) —
//! implemented by U4.
//!
//! `Registry` holds the merged snapshot per manager behind an `RwLock`. After
//! any snapshot update it runs the cross-manager join: for each manager with a
//! Routed self-update, look up its row in the executor's snapshot (mise's
//! latest ← brew's `formula:mise`; uv's latest ← mise's `tool:uv`) and patch
//! the subject's `selfStatus`. [`Registry::upsert`] returns every managerId
//! whose snapshot changed so the caller can emit `snapshot:updated` for each.

use std::collections::HashMap;
use std::sync::RwLock;

use crate::ipc::{DetectionReport, ManagerId, ManagerSnapshot, SelfStatus, SelfUpdateRoute};

/// RFC3339 "now" for `ManagerSnapshot.refreshedAt` (shared by the adapters —
/// parsers stay pure; the timestamp is added at snapshot-assembly time).
pub fn now_rfc3339() -> String {
    time::OffsetDateTime::now_utc()
        .format(&time::format_description::well_known::Rfc3339)
        .unwrap_or_default()
}

/// The verbatim name inside a `${kind}:${name}` package id (split on the
/// FIRST ':' only, so `tool:npm:prettier` → `npm:prettier` — SPEC §5.9).
pub fn pkg_name(id: &str) -> &str {
    id.split_once(':').map(|(_, name)| name).unwrap_or(id)
}

/// The id under which `subject`'s own package row appears in `executor`'s
/// snapshot: brew carries formulae (`formula:mise`), mise carries tools
/// (`tool:uv`), npm global packages, rustup toolchains, mas apps.
fn join_key(executor: ManagerId, subject: ManagerId) -> String {
    let kind = match executor {
        ManagerId::Brew => "formula",
        ManagerId::Mise | ManagerId::Uv => "tool",
        ManagerId::Npm => "globalPackage",
        ManagerId::Rustup => "toolchain",
        ManagerId::Mas => "app",
    };
    format!("{kind}:{}", subject.as_str())
}

/// Snapshot store + cross-manager self-version join.
#[derive(Debug, Default)]
pub struct Registry {
    snapshots: RwLock<HashMap<ManagerId, ManagerSnapshot>>,
    /// `(subject, executor)` pairs for managers with a Routed self-update.
    routed: RwLock<Vec<(ManagerId, ManagerId)>>,
}

impl Registry {
    pub fn new() -> Self {
        Self::default()
    }

    /// Replaces the routed-pair set from a detection report (re-detect safe).
    pub fn set_routes_from(&self, report: &DetectionReport) {
        let pairs = report
            .managers
            .iter()
            .filter_map(|m| match &m.self_update {
                SelfUpdateRoute::Routed { executor, .. } => Some((m.id, *executor)),
                _ => None,
            })
            .collect();
        self.set_routed_pairs(pairs);
    }

    /// Replaces the routed-pair set directly (tests, U5 wiring).
    pub fn set_routed_pairs(&self, pairs: Vec<(ManagerId, ManagerId)>) {
        *self.routed.write().expect("registry poisoned") = pairs;
    }

    /// Inserts/replaces one manager's snapshot, then runs the cross-manager
    /// join (SPEC §5.8). Returns every managerId whose stored snapshot changed
    /// — the upserted one first, then any join-patched subjects — so the
    /// caller emits `snapshot:updated` for each.
    pub fn upsert(&self, snapshot: ManagerSnapshot) -> Vec<ManagerId> {
        let upserted = snapshot.manager_id;
        let mut changed = vec![upserted];
        let mut snaps = self.snapshots.write().expect("registry poisoned");
        snaps.insert(upserted, snapshot);

        let pairs = self.routed.read().expect("registry poisoned").clone();
        for (subject, executor) in pairs {
            let key = join_key(executor, subject);
            let row = match snaps.get(&executor) {
                Some(exec_snap) => match exec_snap.packages.iter().find(|p| p.id == key) {
                    Some(row) => (row.installed.clone(), row.latest.clone(), row.outdated),
                    None => continue,
                },
                None => continue,
            };
            let Some(sub_snap) = snaps.get_mut(&subject) else {
                continue;
            };
            let (installed, latest, outdated) = row;
            let patched = SelfStatus {
                // The subject's own installed version (when it already knows
                // one) wins; the executor's row fills the gap.
                installed: sub_snap
                    .self_status
                    .as_ref()
                    .and_then(|s| s.installed.clone())
                    .or(installed),
                latest,
                update_available: outdated,
            };
            if sub_snap.self_status.as_ref() != Some(&patched) {
                sub_snap.self_status = Some(patched);
                if !changed.contains(&subject) {
                    changed.push(subject);
                }
            }
        }
        changed
    }

    /// Clones one manager's stored snapshot.
    pub fn get(&self, id: ManagerId) -> Option<ManagerSnapshot> {
        self.snapshots
            .read()
            .expect("registry poisoned")
            .get(&id)
            .cloned()
    }

    /// All stored snapshots in `ManagerId::ALL` order (deterministic).
    pub fn all(&self) -> Vec<ManagerSnapshot> {
        let snaps = self.snapshots.read().expect("registry poisoned");
        ManagerId::ALL
            .iter()
            .filter_map(|id| snaps.get(id).cloned())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ipc::{Package, PackageKind};

    fn snap(id: ManagerId, packages: Vec<Package>) -> ManagerSnapshot {
        ManagerSnapshot {
            manager_id: id,
            refreshed_at: "2026-07-22T14:00:00Z".into(),
            packages,
            self_status: None,
            health: vec![],
        }
    }

    fn row(
        kind: PackageKind,
        prefix: &str,
        name: &str,
        installed: &str,
        latest: &str,
        outdated: bool,
    ) -> Package {
        Package {
            id: format!("{prefix}:{name}"),
            name: name.into(),
            kind,
            installed: Some(installed.into()),
            latest: Some(latest.into()),
            outdated,
            pinned: false,
            meta: None,
        }
    }

    #[test]
    fn cross_join_brew_formula_mise_patches_mise_self_status() {
        let reg = Registry::new();
        reg.set_routed_pairs(vec![(ManagerId::Mise, ManagerId::Brew)]);

        reg.upsert(snap(ManagerId::Mise, vec![]));
        let changed = reg.upsert(snap(
            ManagerId::Brew,
            vec![row(
                PackageKind::Formula,
                "formula",
                "mise",
                "2026.1.0",
                "2026.2.0",
                true,
            )],
        ));
        assert_eq!(changed, vec![ManagerId::Brew, ManagerId::Mise]);

        let mise = reg.get(ManagerId::Mise).unwrap();
        let s = mise.self_status.expect("patched by join");
        assert_eq!(s.installed.as_deref(), Some("2026.1.0"));
        assert_eq!(s.latest.as_deref(), Some("2026.2.0"));
        assert!(s.update_available);

        // The mise row STAYS in brew's table (only a manager's own row is
        // hoisted out of its own table).
        let brew = reg.get(ManagerId::Brew).unwrap();
        assert!(brew.packages.iter().any(|p| p.id == "formula:mise"));
    }

    #[test]
    fn cross_join_uv_from_mise_tool_uv_fixture_values() {
        let reg = Registry::new();
        reg.set_routed_pairs(vec![(ManagerId::Uv, ManagerId::Mise)]);

        // Fixture row: `uv latest 0.11.26 0.11.30` (mise_outdated_text_2026-07-21.txt).
        reg.upsert(snap(
            ManagerId::Mise,
            vec![row(
                PackageKind::Tool,
                "tool",
                "uv",
                "0.11.26",
                "0.11.30",
                true,
            )],
        ));
        // uv's snapshot arrives AFTER the executor's — the join re-runs on the
        // subject's own upsert.
        let changed = reg.upsert(snap(ManagerId::Uv, vec![]));
        assert_eq!(changed, vec![ManagerId::Uv]);

        let uv = reg.get(ManagerId::Uv).unwrap();
        let s = uv.self_status.expect("patched by join");
        assert_eq!(s.installed.as_deref(), Some("0.11.26"));
        assert_eq!(s.latest.as_deref(), Some("0.11.30"));
        assert!(s.update_available);
    }

    #[test]
    fn join_preserves_subjects_own_installed_version() {
        let reg = Registry::new();
        reg.set_routed_pairs(vec![(ManagerId::Uv, ManagerId::Mise)]);

        let mut uv_snap = snap(ManagerId::Uv, vec![]);
        uv_snap.self_status = Some(SelfStatus {
            installed: Some("0.11.26".into()),
            latest: None,
            update_available: false,
        });
        reg.upsert(uv_snap);
        reg.upsert(snap(
            ManagerId::Mise,
            // Executor reports a different installed string; the subject's own wins.
            vec![row(
                PackageKind::Tool,
                "tool",
                "uv",
                "0.11.25",
                "0.11.30",
                true,
            )],
        ));

        let s = reg.get(ManagerId::Uv).unwrap().self_status.unwrap();
        assert_eq!(s.installed.as_deref(), Some("0.11.26"));
        assert_eq!(s.latest.as_deref(), Some("0.11.30"));
    }

    #[test]
    fn upsert_without_matching_row_or_route_changes_only_itself() {
        let reg = Registry::new();
        reg.set_routed_pairs(vec![(ManagerId::Mise, ManagerId::Brew)]);
        let changed = reg.upsert(snap(ManagerId::Brew, vec![]));
        assert_eq!(changed, vec![ManagerId::Brew]);
        assert!(reg.get(ManagerId::Mise).is_none());
    }

    #[test]
    fn set_routes_from_extracts_routed_pairs() {
        use crate::ipc::{
            EnvInfo, ManagedBy, ManagerInfo, ManagerStatus, PathSource, SelfUpdateRoute,
        };
        let report = DetectionReport {
            managers: vec![
                ManagerInfo {
                    id: ManagerId::Uv,
                    display_name: "uv".into(),
                    status: ManagerStatus::Present,
                    binary_path: None,
                    canonical_path: None,
                    version: None,
                    managed_by: ManagedBy::Mise,
                    evidence: None,
                    self_update: SelfUpdateRoute::Routed {
                        executor: ManagerId::Mise,
                        command_preview: "mise upgrade uv".into(),
                        why: "uv is managed by mise".into(),
                    },
                    install_hint: None,
                },
                ManagerInfo {
                    id: ManagerId::Rustup,
                    display_name: "rustup".into(),
                    status: ManagerStatus::Present,
                    binary_path: None,
                    canonical_path: None,
                    version: None,
                    managed_by: ManagedBy::Standalone,
                    evidence: None,
                    self_update: SelfUpdateRoute::InBand {
                        command_preview: "rustup self update".into(),
                        note: None,
                    },
                    install_hint: None,
                },
            ],
            env: EnvInfo {
                path: String::new(),
                entries: vec![],
                source: PathSource::StaticFallback,
                home: String::new(),
            },
        };
        let reg = Registry::new();
        reg.set_routes_from(&report);
        assert_eq!(
            *reg.routed.read().unwrap(),
            vec![(ManagerId::Uv, ManagerId::Mise)]
        );
    }

    #[test]
    fn pkg_name_splits_on_first_colon_only() {
        assert_eq!(pkg_name("tool:npm:prettier"), "npm:prettier");
        assert_eq!(pkg_name("formula:dolt"), "dolt");
        assert_eq!(pkg_name("no-colon"), "no-colon");
    }

    #[test]
    fn all_returns_manager_id_order() {
        let reg = Registry::new();
        reg.upsert(snap(ManagerId::Rustup, vec![]));
        reg.upsert(snap(ManagerId::Brew, vec![]));
        let ids: Vec<ManagerId> = reg.all().iter().map(|s| s.manager_id).collect();
        assert_eq!(ids, vec![ManagerId::Brew, ManagerId::Rustup]);
    }
}
