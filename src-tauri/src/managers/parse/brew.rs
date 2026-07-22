//! Pure brew parsers (SPEC §5.5, §7.1).
//!
//! - `brew outdated --json=v2` — skip leading junk until the first `{` line,
//!   then parse `formulae[]` and (unverified) `casks[]`.
//! - `brew list --versions` / `brew list --cask --versions` — inventory.
//! - `brew outdated --greedy` text — the wired recovery parser for the
//!   unverified cask JSON shape (`name (installed) != latest`).
//! - greedy-only casks = greedy set MINUS plain set (never an in-JSON heuristic
//!   — fixture casks carry concrete versions, DECISIONS D7).

use std::collections::HashSet;
use std::sync::LazyLock;

use regex::Regex;
use serde::Deserialize;

use super::{excerpt, make_id};
use crate::error::PmError;
use crate::ipc::{Package, PackageKind, PackageMeta};

/// Result of parsing a `brew outdated --json=v2[ --greedy]` document.
#[derive(Debug, Clone, PartialEq)]
pub struct BrewOutdated {
    pub formulae: Vec<Package>,
    pub casks: Vec<Package>,
}

#[derive(Deserialize)]
struct BrewOutdatedDoc {
    #[serde(default)]
    formulae: Vec<BrewFormula>,
    #[serde(default)]
    casks: Vec<BrewCask>,
}

#[derive(Deserialize)]
struct BrewFormula {
    name: String,
    #[serde(default)]
    installed_versions: Vec<String>,
    #[serde(default)]
    current_version: Option<String>,
    #[serde(default)]
    pinned: bool,
    #[serde(default)]
    pinned_version: Option<String>,
}

/// Cask JSON shape is UNVERIFIED (both captured fixtures have `"casks": []`).
/// Deserialize leniently: unknown fields are ignored by default, and both
/// singular/plural version keys are accepted. The greedy TEXT parser is the
/// authoritative wired recovery until a populated cask JSON is captured.
#[derive(Deserialize)]
struct BrewCask {
    name: String,
    #[serde(default)]
    installed_versions: Vec<String>,
    #[serde(default)]
    installed: Option<String>,
    #[serde(default)]
    current_version: Option<String>,
}

/// `name (installed) != latest`
static GREEDY_TEXT_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^(?P<name>\S+)\s+\((?P<installed>[^)]*)\)\s*!=\s*(?P<latest>.+?)\s*$").unwrap()
});

/// Parse `brew outdated --json=v2`. Skips leading non-JSON lines (the fixture's
/// first line is `✔︎ JSON API packages.arm64_golden_gate.jws.json`) until the
/// first line whose trim starts with `{`. Text where JSON was expected →
/// `ParseFailed` (never a panic).
pub fn parse_outdated_json(stdout: &str) -> Result<BrewOutdated, PmError> {
    let Some(start) = stdout.lines().position(|l| l.trim_start().starts_with('{')) else {
        return Err(PmError::ParseFailed {
            what: "brew outdated --json=v2".into(),
            excerpt: excerpt(stdout),
        });
    };
    let json_text: String = stdout.lines().skip(start).collect::<Vec<_>>().join("\n");
    let doc: BrewOutdatedDoc =
        serde_json::from_str(&json_text).map_err(|e| PmError::ParseFailed {
            what: format!("brew outdated --json=v2: {e}"),
            excerpt: excerpt(stdout),
        })?;

    let formulae = doc
        .formulae
        .into_iter()
        .map(|f| Package {
            id: make_id(PackageKind::Formula, &f.name),
            name: f.name,
            kind: PackageKind::Formula,
            installed: f.installed_versions.last().cloned(),
            latest: f.current_version,
            outdated: true,
            pinned: f.pinned,
            meta: f.pinned_version.map(|pv| PackageMeta {
                pinned_version: Some(pv),
                ..PackageMeta::default()
            }),
        })
        .collect();

    let casks = doc
        .casks
        .into_iter()
        .map(|c| Package {
            id: make_id(PackageKind::Cask, &c.name),
            name: c.name,
            kind: PackageKind::Cask,
            installed: c.installed_versions.last().cloned().or(c.installed),
            latest: c.current_version,
            outdated: true,
            pinned: false,
            meta: None,
        })
        .collect();

    Ok(BrewOutdated { formulae, casks })
}

/// Recovery parser for the greedy cask text form (`name (installed) != latest`).
/// Empty output = no greedy casks (clean). A line that does not match →
/// `ParseFailed` (protects against a wrong fixture reaching this parser).
pub fn parse_greedy_text(stdout: &str) -> Result<Vec<Package>, PmError> {
    let mut out = Vec::new();
    for line in stdout.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let Some(c) = GREEDY_TEXT_RE.captures(line) else {
            return Err(PmError::ParseFailed {
                what: "brew outdated --greedy (text)".into(),
                excerpt: excerpt(stdout),
            });
        };
        let name = c["name"].to_string();
        out.push(Package {
            id: make_id(PackageKind::Cask, &name),
            name,
            kind: PackageKind::Cask,
            installed: Some(c["installed"].to_string()),
            latest: Some(c["latest"].to_string()),
            outdated: true,
            pinned: false,
            meta: None,
        });
    }
    Ok(out)
}

/// Greedy-only casks = casks present in the greedy result but NOT in the plain
/// result (SPEC §5.4). Matched by name; the result is re-kinded `CaskGreedy`.
pub fn greedy_only(plain_casks: &[Package], greedy_casks: &[Package]) -> Vec<Package> {
    let plain: HashSet<&str> = plain_casks.iter().map(|p| p.name.as_str()).collect();
    greedy_casks
        .iter()
        .filter(|g| !plain.contains(g.name.as_str()))
        .map(|g| Package {
            id: make_id(PackageKind::CaskGreedy, &g.name),
            kind: PackageKind::CaskGreedy,
            ..g.clone()
        })
        .collect()
}

/// `brew list --versions` — one `name ver1 [ver2…]` per line; the last version
/// token wins. NOTE: this command also lists casks; dedupe against the cask
/// list with `super::dedupe_formulae_against_casks` before merging.
pub fn parse_list_versions(stdout: &str) -> Vec<Package> {
    parse_versions_lines(stdout, PackageKind::Formula)
}

/// `brew list --cask --versions` — one `name ver1 [ver2…]` per line.
pub fn parse_cask_versions(stdout: &str) -> Vec<Package> {
    parse_versions_lines(stdout, PackageKind::Cask)
}

fn parse_versions_lines(stdout: &str, kind: PackageKind) -> Vec<Package> {
    let mut out = Vec::new();
    for line in stdout.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let mut tokens = line.split_whitespace();
        let Some(name) = tokens.next() else {
            continue;
        };
        // Version tokens may contain commas (e.g. ngrok `3.39.9,6pfVfGALLzX,a`);
        // "last version wins" (SPEC §5.5).
        let installed = tokens.last().map(str::to_string);
        out.push(Package {
            id: make_id(kind, name),
            name: name.to_string(),
            kind,
            installed: installed.clone(),
            latest: installed,
            outdated: false,
            pinned: false,
            meta: None,
        });
    }
    out
}

#[cfg(test)]
mod tests {
    use super::super::read_fixture;
    use super::*;

    #[test]
    fn brew_outdated_json_skips_leading_junk_line() {
        let out = parse_outdated_json(&read_fixture("brew_outdated.json")).expect("parse");
        assert_eq!(out.casks.len(), 0, "casks empty");
        assert_eq!(out.formulae.len(), 1);
        let dolt = &out.formulae[0];
        assert_eq!(dolt.name, "dolt");
        assert_eq!(dolt.id, "formula:dolt");
        assert_eq!(dolt.installed.as_deref(), Some("2.2.1"));
        assert_eq!(dolt.latest.as_deref(), Some("2.2.2"));
        assert!(dolt.outdated);
        assert!(!dolt.pinned);
    }

    #[test]
    fn brew_outdated_clean_json_parses_without_junk() {
        // brew_outdated_greedy.json starts directly with `{` (no junk line).
        let out = parse_outdated_json(&read_fixture("brew_outdated_greedy.json")).expect("parse");
        assert_eq!(out.formulae.len(), 1);
        assert_eq!(out.formulae[0].name, "dolt");
        assert_eq!(out.casks.len(), 0);
    }

    #[test]
    fn brew_greedy_only_is_set_difference() {
        // Plain casks (from JSON) are empty; greedy casks come from the text
        // recovery fixture → all three are greedy-only.
        let plain = parse_outdated_json(&read_fixture("brew_outdated.json"))
            .expect("plain")
            .casks;
        let greedy = parse_greedy_text(&read_fixture("brew_outdated_greedy_text_2026-07-21.txt"))
            .expect("greedy text");
        assert_eq!(greedy.len(), 3);

        let only = greedy_only(&plain, &greedy);
        assert_eq!(only.len(), 3);
        assert!(only.iter().all(|c| c.kind == PackageKind::CaskGreedy));

        let by_name = |n: &str| only.iter().find(|c| c.name == n).unwrap();
        let openusage = by_name("openusage");
        assert_eq!(openusage.installed.as_deref(), Some("0.6.20"));
        assert_eq!(openusage.latest.as_deref(), Some("0.7.6"));
        assert_eq!(openusage.id, "caskGreedy:openusage");

        let sync = by_name("syncthing-app");
        assert_eq!(sync.installed.as_deref(), Some("2.0.14-1"));
        assert_eq!(sync.latest.as_deref(), Some("2.1.2-1"));

        let trans = by_name("transmission");
        assert_eq!(trans.installed.as_deref(), Some("4.1.1"));
        assert_eq!(trans.latest.as_deref(), Some("4.1.3"));
    }

    #[test]
    fn brew_text_where_json_expected_is_parse_failed_with_excerpt() {
        // Feeding the greedy text (no `{` line) to the JSON parser must fail
        // gracefully, never panic.
        let err = parse_outdated_json(&read_fixture("brew_outdated_greedy_text_2026-07-21.txt"))
            .unwrap_err();
        match err {
            PmError::ParseFailed { excerpt, .. } => assert!(excerpt.contains("openusage")),
            other => panic!("expected ParseFailed, got {other:?}"),
        }
    }

    #[test]
    fn brew_list_versions_parses_inventory_and_last_version_wins() {
        let formulae = parse_list_versions(&read_fixture("brew_list_versions_2026-07-22.txt"));
        // 258 lines in the capture (includes the 15 casks brew list emits).
        assert_eq!(formulae.len(), 258);
        let abseil = formulae.iter().find(|p| p.name == "abseil").unwrap();
        assert_eq!(abseil.installed.as_deref(), Some("20260107.1"));
        assert_eq!(abseil.latest.as_deref(), Some("20260107.1"));
        assert!(!abseil.outdated);
        assert_eq!(abseil.kind, PackageKind::Formula);

        // Last version wins on a multi-version line.
        let multi = parse_list_versions("foo 1.0 1.2\n");
        assert_eq!(multi[0].installed.as_deref(), Some("1.2"));
    }

    #[test]
    fn brew_cask_versions_parses_comma_version() {
        let casks = parse_cask_versions(&read_fixture("brew_list_cask_versions_2026-07-22.txt"));
        assert_eq!(casks.len(), 15);
        let ngrok = casks.iter().find(|p| p.name == "ngrok").unwrap();
        assert_eq!(ngrok.installed.as_deref(), Some("3.39.9,6pfVfGALLzX,a"));
        assert_eq!(ngrok.kind, PackageKind::Cask);
    }

    #[test]
    fn brew_formula_inventory_deduped_against_casks_is_243() {
        let formulae = parse_list_versions(&read_fixture("brew_list_versions_2026-07-22.txt"));
        let casks = parse_cask_versions(&read_fixture("brew_list_cask_versions_2026-07-22.txt"));
        let deduped = super::super::dedupe_formulae_against_casks(formulae, &casks);
        assert_eq!(deduped.len(), 243, "258 total − 15 casks");
        assert!(deduped.iter().all(|f| f.name != "ghostty"));
    }
}
