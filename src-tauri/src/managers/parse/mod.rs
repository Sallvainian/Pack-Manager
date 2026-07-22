//! Pure, fixture-grounded parsers (SPEC §5.5) and the adapter-level merge
//! helpers (SPEC §5.4 snapshot assembly). Everything here is a pure function:
//! bytes in, packages/health out. Parsers never log and never touch the clock,
//! the filesystem, or a process (SPEC §6.3 hygiene). The `refreshedAt`
//! timestamp and the `ManagerSnapshot` wrapper are added by the adapters (U4).
//!
//! Package ids are `${kind}:${name}` where `kind` is the wire (camelCase) kind
//! name and `name` is verbatim (split on the FIRST ':' only, so
//! `tool:npm:prettier` round-trips — SPEC §5.9).

pub mod brew;
pub mod mas;
pub mod mise;
pub mod npm;
pub mod rustup;
pub mod uv;

use std::collections::{HashMap, HashSet};

use crate::ipc::{Package, PackageKind, PackageMeta, SelfStatus};

/// First 500 chars of the offending output, for `PmError::ParseFailed`
/// excerpts (SPEC §5.10). Char-bounded so we never split a UTF-8 sequence.
pub(crate) fn excerpt(s: &str) -> String {
    s.chars().take(500).collect()
}

/// Wire-form (camelCase) kind prefix used in package ids.
pub(crate) fn kind_prefix(kind: PackageKind) -> &'static str {
    match kind {
        PackageKind::Formula => "formula",
        PackageKind::Cask => "cask",
        PackageKind::CaskGreedy => "caskGreedy",
        PackageKind::Tool => "tool",
        PackageKind::GlobalPackage => "globalPackage",
        PackageKind::Toolchain => "toolchain",
        PackageKind::App => "app",
    }
}

/// `${kind}:${name}` package id.
pub(crate) fn make_id(kind: PackageKind, name: &str) -> String {
    format!("{}:{}", kind_prefix(kind), name)
}

/// Merge a `PackageMeta` overlay onto a base; overlay wins per field, base
/// fills the gaps. `None`/`None` collapses to `None`.
fn merge_meta(base: Option<PackageMeta>, overlay: Option<PackageMeta>) -> Option<PackageMeta> {
    match (base, overlay) {
        (None, o) => o,
        (b, None) => b,
        (Some(b), Some(o)) => Some(PackageMeta {
            executables: o.executables.or(b.executables),
            requested: o.requested.or(b.requested),
            source: o.source.or(b.source),
            wanted: o.wanted.or(b.wanted),
            depended_by: o.depended_by.or(b.depended_by),
            pinned_version: o.pinned_version.or(b.pinned_version),
        }),
    }
}

/// Snapshot assembly (SPEC §5.4): inventory rows arrive with
/// `latest = installed`, `outdated = false`; the outdated overlay patches
/// `latest` / `outdated` / `pinned` / `meta` on rows with a matching id;
/// overlay-only rows are appended in overlay order. Deterministic.
pub fn merge_inventory_overlay(inventory: Vec<Package>, overlay: Vec<Package>) -> Vec<Package> {
    let inv_ids: HashSet<String> = inventory.iter().map(|p| p.id.clone()).collect();

    let mut overlay_order: Vec<String> = Vec::with_capacity(overlay.len());
    let mut overlay_map: HashMap<String, Package> = HashMap::with_capacity(overlay.len());
    for p in overlay {
        overlay_order.push(p.id.clone());
        overlay_map.insert(p.id.clone(), p);
    }

    let mut out: Vec<Package> = Vec::with_capacity(inv_ids.len() + overlay_order.len());
    for mut inv in inventory {
        if let Some(ov) = overlay_map.get(&inv.id) {
            inv.latest = ov.latest.clone();
            inv.outdated = ov.outdated;
            inv.pinned = inv.pinned || ov.pinned;
            inv.meta = merge_meta(inv.meta.take(), ov.meta.clone());
        }
        out.push(inv);
    }
    for id in overlay_order {
        if !inv_ids.contains(&id) {
            if let Some(ov) = overlay_map.remove(&id) {
                out.push(ov);
            }
        }
    }
    out
}

/// Hoist the manager's own package (matched by `name`) out of the list into a
/// `SelfStatus`. The manager's own row never appears in the package table; it
/// lives only on the SelfUpdateCard (SPEC §5.4, F6). First match wins.
pub fn extract_self(packages: Vec<Package>, self_name: &str) -> (Vec<Package>, Option<SelfStatus>) {
    let mut self_status: Option<SelfStatus> = None;
    let mut remaining: Vec<Package> = Vec::with_capacity(packages.len());
    for p in packages {
        if self_status.is_none() && p.name == self_name {
            self_status = Some(SelfStatus {
                installed: p.installed.clone(),
                latest: p.latest.clone(),
                update_available: p.outdated,
            });
            continue;
        }
        remaining.push(p);
    }
    (remaining, self_status)
}

/// `brew list --versions` includes casks as well as formulae; the cask list is
/// authoritative for kind, so drop any formula row whose name also appears in
/// the cask list before merging (SPEC §5.4).
pub fn dedupe_formulae_against_casks(formulae: Vec<Package>, casks: &[Package]) -> Vec<Package> {
    let cask_names: HashSet<&str> = casks.iter().map(|c| c.name.as_str()).collect();
    formulae
        .into_iter()
        .filter(|f| !cask_names.contains(f.name.as_str()))
        .collect()
}

// ---------------------------------------------------------------------------
// Shared test helpers.
// ---------------------------------------------------------------------------

#[cfg(test)]
pub(crate) fn read_fixture(name: &str) -> String {
    let path =
        std::path::Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/../dev/fixtures")).join(name);
    std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("read fixture {}: {e}", path.display()))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn inv(kind: PackageKind, name: &str, ver: &str) -> Package {
        Package {
            id: make_id(kind, name),
            name: name.to_string(),
            kind,
            installed: Some(ver.to_string()),
            latest: Some(ver.to_string()),
            outdated: false,
            pinned: false,
            meta: None,
        }
    }

    fn overlay_row(kind: PackageKind, name: &str, installed: &str, latest: &str) -> Package {
        Package {
            id: make_id(kind, name),
            name: name.to_string(),
            kind,
            installed: Some(installed.to_string()),
            latest: Some(latest.to_string()),
            outdated: true,
            pinned: false,
            meta: None,
        }
    }

    #[test]
    fn merge_patches_inventory_and_appends_overlay_only() {
        let inventory = vec![
            inv(PackageKind::Tool, "deno", "2.9.0"),
            inv(PackageKind::Tool, "ruby", "4.0.5"),
        ];
        let overlay = vec![
            overlay_row(PackageKind::Tool, "deno", "2.9.0", "2.9.3"),
            overlay_row(PackageKind::Tool, "newtool", "1.0", "1.1"),
        ];
        let merged = merge_inventory_overlay(inventory, overlay);
        assert_eq!(merged.len(), 3);

        let deno = &merged[0];
        assert_eq!(deno.name, "deno");
        assert!(deno.outdated);
        assert_eq!(deno.latest.as_deref(), Some("2.9.3"));

        let ruby = &merged[1];
        assert_eq!(ruby.name, "ruby");
        assert!(!ruby.outdated, "unmatched inventory row stays not-outdated");
        assert_eq!(ruby.latest.as_deref(), Some("4.0.5"));

        let newtool = &merged[2];
        assert_eq!(newtool.name, "newtool", "overlay-only row appended last");
        assert!(newtool.outdated);
    }

    #[test]
    fn extract_self_hoists_named_row_into_self_status() {
        let packages = vec![
            overlay_row(PackageKind::GlobalPackage, "typescript", "6.0.3", "7.0.2"),
            overlay_row(PackageKind::GlobalPackage, "npm", "11.16.0", "12.0.1"),
        ];
        let (rest, self_status) = extract_self(packages, "npm");
        assert_eq!(rest.len(), 1);
        assert_eq!(rest[0].name, "typescript");
        let s = self_status.expect("npm hoisted");
        assert_eq!(s.installed.as_deref(), Some("11.16.0"));
        assert_eq!(s.latest.as_deref(), Some("12.0.1"));
        assert!(s.update_available);
    }

    #[test]
    fn dedupe_removes_casks_from_formula_inventory() {
        let formulae = vec![
            inv(PackageKind::Formula, "abseil", "20260107.1"),
            inv(PackageKind::Formula, "ghostty", "1.3.1"), // also a cask
            inv(PackageKind::Formula, "openusage", "0.7.6"), // also a cask
        ];
        let casks = vec![
            inv(PackageKind::Cask, "ghostty", "1.3.1"),
            inv(PackageKind::Cask, "openusage", "0.7.6"),
        ];
        let deduped = dedupe_formulae_against_casks(formulae, &casks);
        assert_eq!(deduped.len(), 1);
        assert_eq!(deduped[0].name, "abseil");
    }

    #[test]
    fn package_ids_use_camel_case_kind_prefix() {
        assert_eq!(
            make_id(PackageKind::GlobalPackage, "typescript"),
            "globalPackage:typescript"
        );
        assert_eq!(
            make_id(PackageKind::CaskGreedy, "openusage"),
            "caskGreedy:openusage"
        );
        assert_eq!(
            make_id(PackageKind::Tool, "npm:prettier"),
            "tool:npm:prettier"
        );
    }
}
