//! Pure npm parsers (SPEC §5.5, §7.1).
//!
//! - `npm ls -g --depth=0 --json` — inventory (`{ dependencies: { pkg: {
//!   version } } }`).
//! - `npm outdated -g --json` — `{}` is clean; populated shape UNVERIFIED,
//!   parsed from the labeled synthetic fixture.
//! - `npm outdated -g` text — the verified recovery form.
//!
//! The `npm` row is NOT hoisted here; callers run `super::extract_self(rows,
//! "npm")` so inventory and outdated share one hoist path (SPEC §5.4).

use std::collections::BTreeMap;

use serde::Deserialize;

use super::{excerpt, make_id};
use crate::error::PmError;
use crate::ipc::{Package, PackageKind, PackageMeta};

#[derive(Deserialize)]
struct NpmLsDoc {
    #[serde(default)]
    dependencies: BTreeMap<String, NpmLsDep>,
}

#[derive(Deserialize)]
struct NpmLsDep {
    #[serde(default)]
    version: Option<String>,
}

/// `npm ls -g --depth=0 --json` inventory (BTreeMap → deterministic order).
/// Includes the `npm` self row; the caller hoists it via `extract_self`.
pub fn parse_ls_json(stdout: &str) -> Result<Vec<Package>, PmError> {
    let doc: NpmLsDoc = serde_json::from_str(stdout).map_err(|e| PmError::ParseFailed {
        what: format!("npm ls -g --json: {e}"),
        excerpt: excerpt(stdout),
    })?;
    let out = doc
        .dependencies
        .into_iter()
        .map(|(name, dep)| Package {
            id: make_id(PackageKind::GlobalPackage, &name),
            name,
            kind: PackageKind::GlobalPackage,
            installed: dep.version.clone(),
            latest: dep.version,
            outdated: false,
            pinned: false,
            meta: None,
        })
        .collect();
    Ok(out)
}

#[derive(Deserialize)]
struct NpmOutdatedEntry {
    #[serde(default)]
    current: Option<String>,
    #[serde(default)]
    wanted: Option<String>,
    #[serde(default)]
    latest: Option<String>,
    #[serde(default)]
    location: Option<String>,
    #[serde(default)]
    dependent: Option<String>,
}

/// `npm outdated -g --json` — `{}`/empty is clean. Populated shape UNVERIFIED
/// (synthetic-tested). Display uses `latest`; `wanted`/`location`/`dependent`
/// ride in meta.
pub fn parse_outdated_json(stdout: &str) -> Result<Vec<Package>, PmError> {
    let trimmed = stdout.trim();
    if trimmed.is_empty() || trimmed == "{}" {
        return Ok(Vec::new());
    }
    let map: BTreeMap<String, NpmOutdatedEntry> =
        serde_json::from_str(trimmed).map_err(|e| PmError::ParseFailed {
            what: format!("npm outdated -g --json: {e}"),
            excerpt: excerpt(stdout),
        })?;
    let out = map
        .into_iter()
        .map(|(name, e)| {
            npm_pkg(
                &name,
                e.current,
                e.latest,
                e.wanted,
                e.location,
                e.dependent,
            )
        })
        .collect();
    Ok(out)
}

/// `npm outdated -g` text: header `Package Current Wanted Latest Location
/// Depended by`, then rows. Header is skipped if present.
pub fn parse_outdated_text(stdout: &str) -> Result<Vec<Package>, PmError> {
    let mut out = Vec::new();
    for (i, raw) in stdout.lines().enumerate() {
        let line = raw.trim();
        if line.is_empty() {
            continue;
        }
        if i == 0 && line.starts_with("Package") {
            continue;
        }
        let cols: Vec<&str> = line.split_whitespace().collect();
        if cols.len() < 4 {
            return Err(PmError::ParseFailed {
                what: "npm outdated -g (text)".into(),
                excerpt: excerpt(stdout),
            });
        }
        let name = cols[0];
        let current = Some(cols[1].to_string());
        let wanted = Some(cols[2].to_string());
        let latest = Some(cols[3].to_string());
        let location = cols.get(4).map(|s| s.to_string());
        let dependent = cols.get(5..).map(|s| s.join(" ")).filter(|s| !s.is_empty());
        out.push(npm_pkg(name, current, latest, wanted, location, dependent));
    }
    Ok(out)
}

#[allow(clippy::too_many_arguments)]
fn npm_pkg(
    name: &str,
    current: Option<String>,
    latest: Option<String>,
    wanted: Option<String>,
    location: Option<String>,
    dependent: Option<String>,
) -> Package {
    let meta = if wanted.is_none() && location.is_none() && dependent.is_none() {
        None
    } else {
        Some(PackageMeta {
            wanted,
            source: location,
            depended_by: dependent,
            ..PackageMeta::default()
        })
    };
    Package {
        id: make_id(PackageKind::GlobalPackage, name),
        name: name.to_string(),
        kind: PackageKind::GlobalPackage,
        installed: current,
        latest,
        outdated: true,
        pinned: false,
        meta,
    }
}

#[cfg(test)]
mod tests {
    use super::super::{extract_self, read_fixture};
    use super::*;

    #[test]
    fn npm_outdated_json_empty_object_means_clean() {
        assert!(parse_outdated_json(&read_fixture("npm_outdated_g.json"))
            .unwrap()
            .is_empty());
        assert!(parse_outdated_json("{}").unwrap().is_empty());
    }

    #[test]
    fn npm_outdated_text_parses_five_rows_hoists_npm_self() {
        let rows = parse_outdated_text(&read_fixture("npm_outdated_g_text_2026-07-21.txt"))
            .expect("parse");
        assert_eq!(rows.len(), 5);

        let (pkgs, self_status) = extract_self(rows, "npm");
        assert_eq!(pkgs.len(), 4, "npm hoisted to self");
        let s = self_status.expect("npm self");
        assert_eq!(s.installed.as_deref(), Some("11.16.0"));
        assert_eq!(s.latest.as_deref(), Some("12.0.1"));
        assert!(s.update_available);

        let ts = pkgs.iter().find(|p| p.name == "typescript").unwrap();
        assert_eq!(ts.installed.as_deref(), Some("6.0.3"));
        assert_eq!(ts.latest.as_deref(), Some("7.0.2"));
        let meta = ts.meta.as_ref().unwrap();
        assert_eq!(meta.wanted.as_deref(), Some("7.0.2"));
        assert_eq!(meta.source.as_deref(), Some("node_modules/typescript"));
        assert_eq!(meta.depended_by.as_deref(), Some("global"));
    }

    #[test]
    fn npm_outdated_json_populated_synthetic() {
        // Values copied verbatim from the text capture → identical rows.
        let rows =
            parse_outdated_json(&read_fixture("npm_outdated_g_synthetic.json")).expect("parse");
        assert_eq!(rows.len(), 5);
        let (pkgs, self_status) = extract_self(rows, "npm");
        assert_eq!(pkgs.len(), 4);
        let s = self_status.expect("npm self");
        assert_eq!(s.installed.as_deref(), Some("11.16.0"));
        assert_eq!(s.latest.as_deref(), Some("12.0.1"));

        let ts = pkgs.iter().find(|p| p.name == "typescript").unwrap();
        assert_eq!(ts.installed.as_deref(), Some("6.0.3"));
        assert_eq!(ts.latest.as_deref(), Some("7.0.2"));
        assert_eq!(ts.meta.as_ref().unwrap().wanted.as_deref(), Some("7.0.2"));
    }

    #[test]
    fn npm_ls_json_parses_fifteen_global_deps() {
        let deps = parse_ls_json(&read_fixture("npm_ls_g_2026-07-22.json")).expect("parse");
        assert_eq!(deps.len(), 15);
        let ts = deps.iter().find(|p| p.name == "typescript").unwrap();
        assert_eq!(ts.installed.as_deref(), Some("7.0.2"));
        assert!(!ts.outdated);
        assert_eq!(ts.kind, PackageKind::GlobalPackage);
        // npm is present in inventory; hoist leaves 14.
        assert!(deps.iter().any(|p| p.name == "npm"));
        let (rest, self_status) = extract_self(deps, "npm");
        assert_eq!(rest.len(), 14);
        assert_eq!(self_status.unwrap().installed.as_deref(), Some("12.0.1"));
    }
}
