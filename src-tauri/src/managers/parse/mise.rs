//! Pure mise parsers (SPEC §5.5, §7.1).
//!
//! - `mise ls --json` — inventory, keyed by tool; the ACTIVE installed version
//!   is chosen when a tool has several.
//! - `mise outdated --json` — `{}` is clean; the populated shape is UNVERIFIED,
//!   parsed best-effort from the labeled synthetic fixture; on any structural
//!   mismatch it returns `ParseFailed`, which the adapter turns into the wired
//!   text recovery.
//! - `mise outdated` text — the verified recovery form; `current == latest`
//!   rows (e.g. `rust stable stable stable`) are NOT outdated and are dropped.

use std::collections::BTreeMap;

use serde::Deserialize;

use super::{excerpt, make_id};
use crate::error::PmError;
use crate::ipc::{Package, PackageKind, PackageMeta};

#[derive(Deserialize)]
struct MiseInstall {
    version: String,
    #[serde(default)]
    requested_version: Option<String>,
    #[serde(default)]
    source: Option<MiseSource>,
    #[serde(default)]
    active: bool,
}

#[derive(Deserialize)]
struct MiseSource {
    #[serde(default)]
    path: Option<String>,
}

/// `mise ls --json` — one row per tool (BTreeMap → deterministic name order),
/// using the active install if present, else the last listed.
pub fn parse_ls_json(stdout: &str) -> Result<Vec<Package>, PmError> {
    let map: BTreeMap<String, Vec<MiseInstall>> =
        serde_json::from_str(stdout).map_err(|e| PmError::ParseFailed {
            what: format!("mise ls --json: {e}"),
            excerpt: excerpt(stdout),
        })?;

    let mut out = Vec::with_capacity(map.len());
    for (name, installs) in map {
        let Some(chosen) = installs
            .iter()
            .find(|i| i.active)
            .or_else(|| installs.last())
        else {
            continue; // empty install list for a tool — skip
        };
        let meta = build_meta(
            chosen.requested_version.clone(),
            chosen.source.as_ref().and_then(|s| s.path.clone()),
        );
        out.push(Package {
            id: make_id(PackageKind::Tool, &name),
            name: name.clone(),
            kind: PackageKind::Tool,
            installed: Some(chosen.version.clone()),
            latest: Some(chosen.version.clone()),
            outdated: false,
            pinned: false,
            meta,
        });
    }
    Ok(out)
}

#[derive(Deserialize)]
struct MiseOutdatedEntry {
    #[serde(default)]
    requested: Option<String>,
    current: String,
    latest: String,
    #[serde(default)]
    source: Option<MiseSource>,
}

/// `mise outdated --json` — `{}` (or empty) is clean. Populated shape is
/// UNVERIFIED (synthetic-tested); `current == latest` rows are dropped.
pub fn parse_outdated_json(stdout: &str) -> Result<Vec<Package>, PmError> {
    let trimmed = stdout.trim();
    if trimmed.is_empty() || trimmed == "{}" {
        return Ok(Vec::new());
    }
    let map: BTreeMap<String, MiseOutdatedEntry> =
        serde_json::from_str(trimmed).map_err(|e| PmError::ParseFailed {
            what: format!("mise outdated --json: {e}"),
            excerpt: excerpt(stdout),
        })?;

    let mut out = Vec::new();
    for (name, e) in map {
        if e.current == e.latest {
            continue; // not actually outdated
        }
        let meta = build_meta(e.requested, e.source.and_then(|s| s.path));
        out.push(outdated_pkg(&name, &e.current, &e.latest, meta));
    }
    Ok(out)
}

/// `mise outdated` text: whitespace columns `tool requested current latest
/// source`. No header in the fixture; a leading `Tool …` header is skipped if
/// present. `current == latest` rows are dropped.
pub fn parse_outdated_text(stdout: &str) -> Result<Vec<Package>, PmError> {
    let mut out = Vec::new();
    for (i, raw) in stdout.lines().enumerate() {
        let line = raw.trim();
        if line.is_empty() {
            continue;
        }
        if i == 0 && line.starts_with("Tool") {
            continue;
        }
        let cols: Vec<&str> = line.split_whitespace().collect();
        if cols.len() < 4 {
            return Err(PmError::ParseFailed {
                what: "mise outdated (text)".into(),
                excerpt: excerpt(stdout),
            });
        }
        let (tool, requested, current, latest) = (cols[0], cols[1], cols[2], cols[3]);
        if current == latest {
            continue;
        }
        let source = cols.get(4..).map(|s| s.join(" ")).filter(|s| !s.is_empty());
        let meta = build_meta(Some(requested.to_string()), source);
        out.push(outdated_pkg(tool, current, latest, meta));
    }
    Ok(out)
}

fn outdated_pkg(name: &str, current: &str, latest: &str, meta: Option<PackageMeta>) -> Package {
    Package {
        id: make_id(PackageKind::Tool, name),
        name: name.to_string(),
        kind: PackageKind::Tool,
        installed: Some(current.to_string()),
        latest: Some(latest.to_string()),
        outdated: true,
        pinned: false,
        meta,
    }
}

fn build_meta(requested: Option<String>, source: Option<String>) -> Option<PackageMeta> {
    if requested.is_none() && source.is_none() {
        return None;
    }
    Some(PackageMeta {
        requested,
        source,
        ..PackageMeta::default()
    })
}

#[cfg(test)]
mod tests {
    use super::super::read_fixture;
    use super::*;

    #[test]
    fn mise_outdated_json_empty_object_means_clean() {
        assert!(parse_outdated_json(&read_fixture("mise_outdated.json"))
            .unwrap()
            .is_empty());
        assert!(parse_outdated_json("{}").unwrap().is_empty());
        assert!(parse_outdated_json("").unwrap().is_empty());
    }

    #[test]
    fn mise_outdated_text_parses_seven_rows_six_outdated() {
        let rows =
            parse_outdated_text(&read_fixture("mise_outdated_text_2026-07-21.txt")).expect("parse");
        // 7 rows in the fixture; `rust stable stable stable` (current==latest)
        // is dropped → 6 outdated.
        assert_eq!(rows.len(), 6);
        assert!(rows.iter().all(|p| p.outdated));
        assert!(!rows.iter().any(|p| p.name == "rust"), "rust dropped");

        let uv = rows.iter().find(|p| p.name == "uv").unwrap();
        assert_eq!(uv.installed.as_deref(), Some("0.11.26"));
        assert_eq!(uv.latest.as_deref(), Some("0.11.30"));

        // `npm:prettier` kept verbatim; source path in meta.
        let prettier = rows.iter().find(|p| p.name == "npm:prettier").unwrap();
        assert_eq!(prettier.id, "tool:npm:prettier");
        assert_eq!(
            prettier.meta.as_ref().unwrap().source.as_deref(),
            Some("~/.config/mise/config.toml")
        );
    }

    #[test]
    fn mise_outdated_json_populated_synthetic() {
        // Synthetic JSON, values copied verbatim from the text capture → same
        // 6 outdated rows (rust dropped).
        let rows =
            parse_outdated_json(&read_fixture("mise_outdated_synthetic.json")).expect("parse");
        assert_eq!(rows.len(), 6);
        assert!(!rows.iter().any(|p| p.name == "rust"));
        let uv = rows.iter().find(|p| p.name == "uv").unwrap();
        assert_eq!(uv.installed.as_deref(), Some("0.11.26"));
        assert_eq!(uv.latest.as_deref(), Some("0.11.30"));
        let prettier = rows.iter().find(|p| p.name == "npm:prettier").unwrap();
        assert_eq!(prettier.id, "tool:npm:prettier");
    }

    #[test]
    fn mise_ls_json_parses_eleven_tools_active_version() {
        let tools = parse_ls_json(&read_fixture("mise_ls_2026-07-22.json")).expect("parse");
        assert_eq!(tools.len(), 11);

        // node has two installs; the active one (24.18.0) is chosen.
        let node = tools.iter().find(|p| p.name == "node").unwrap();
        assert_eq!(node.installed.as_deref(), Some("24.18.0"));
        assert!(!node.outdated);
        assert_eq!(node.meta.as_ref().unwrap().requested.as_deref(), Some("24"));

        // npm:prettier kept verbatim.
        let prettier = tools.iter().find(|p| p.name == "npm:prettier").unwrap();
        assert_eq!(prettier.id, "tool:npm:prettier");
        assert_eq!(prettier.installed.as_deref(), Some("3.9.5"));
    }

    #[test]
    fn mise_ls_json_garbage_is_parse_failed_not_panic() {
        let err = parse_ls_json("not json at all").unwrap_err();
        assert!(matches!(err, PmError::ParseFailed { .. }));
    }
}
