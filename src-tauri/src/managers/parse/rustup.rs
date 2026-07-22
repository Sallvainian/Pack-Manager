//! Pure rustup parsers (SPEC §5.5, §7.1).
//!
//! - `rustup check` — one line per toolchain plus a `rustup` self line. The
//!   `\s*:\s*` in the regex is load-bearing: real output mixes `up to date:`
//!   and `up to date :` in a single run. Commit hashes/dates after the version
//!   are ignored.
//! - `rustup toolchain list` — `name [(…)]` per line; the parenthetical
//!   (`(active, default)` on this machine) is ignored.

use std::sync::LazyLock;

use regex::Regex;

use super::{excerpt, make_id};
use crate::error::PmError;
use crate::ipc::{Package, PackageKind, SelfStatus};

/// Toolchains + the rustup self line parsed from `rustup check`.
#[derive(Debug, Clone, PartialEq)]
pub struct RustupCheck {
    pub packages: Vec<Package>,
    pub self_status: Option<SelfStatus>,
}

static CHECK_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"^(?P<name>\S+)\s+-\s+(?:Update available\s*:\s*(?P<from>\S+).*?->\s*(?P<to>\S+)|up to date\s*:\s*(?P<cur>\S+))",
    )
    .unwrap()
});

pub fn parse_check(stdout: &str) -> Result<RustupCheck, PmError> {
    let mut packages = Vec::new();
    let mut self_status = None;
    let mut matched_any = false;

    for raw in stdout.lines() {
        let line = raw.trim();
        if line.is_empty() {
            continue;
        }
        let Some(c) = CHECK_RE.captures(line) else {
            continue; // tolerate unexpected lines (matched_any guards a total miss)
        };
        matched_any = true;
        let name = c["name"].to_string();
        let (installed, latest, outdated) = if let Some(to) = c.name("to") {
            (
                c.name("from").map(|m| m.as_str().to_string()),
                Some(to.as_str().to_string()),
                true,
            )
        } else {
            let cur = c.name("cur").map(|m| m.as_str().to_string());
            (cur.clone(), cur, false)
        };

        if name == "rustup" {
            self_status = Some(SelfStatus {
                installed,
                latest,
                update_available: outdated,
            });
        } else {
            packages.push(Package {
                id: make_id(PackageKind::Toolchain, &name),
                name,
                kind: PackageKind::Toolchain,
                installed,
                latest,
                outdated,
                pinned: false,
                meta: None,
            });
        }
    }

    if !matched_any {
        return Err(PmError::ParseFailed {
            what: "rustup check".into(),
            excerpt: excerpt(stdout),
        });
    }
    Ok(RustupCheck {
        packages,
        self_status,
    })
}

/// `rustup toolchain list` — `name [(…)]` per line. Only the toolchain name is
/// taken; versions come from `rustup check` at merge time.
pub fn parse_toolchain_list(stdout: &str) -> Vec<Package> {
    let mut out = Vec::new();
    for raw in stdout.lines() {
        let line = raw.trim();
        if line.is_empty() {
            continue;
        }
        let Some(name) = line.split_whitespace().next() else {
            continue;
        };
        out.push(Package {
            id: make_id(PackageKind::Toolchain, name),
            name: name.to_string(),
            kind: PackageKind::Toolchain,
            installed: None,
            latest: None,
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
    fn rustup_check_outdated_yields_toolchain_and_self() {
        let check = parse_check(&read_fixture("rustup_check_2026-07-21.txt")).expect("parse");
        assert_eq!(check.packages.len(), 1);
        let tc = &check.packages[0];
        assert_eq!(tc.name, "stable-aarch64-apple-darwin");
        assert_eq!(tc.id, "toolchain:stable-aarch64-apple-darwin");
        assert_eq!(tc.installed.as_deref(), Some("1.94.0"));
        assert_eq!(tc.latest.as_deref(), Some("1.97.1"));
        assert!(tc.outdated);

        let s = check.self_status.expect("rustup self line");
        assert_eq!(s.installed.as_deref(), Some("1.28.2"));
        assert_eq!(s.latest.as_deref(), Some("1.29.0"));
        assert!(s.update_available);
    }

    #[test]
    fn rustup_check_tolerates_both_colon_spacings() {
        // `up to date:` (toolchain) and `up to date :` (rustup) in one file.
        let check = parse_check(&read_fixture("rustup_check.txt")).expect("parse");
        assert_eq!(check.packages.len(), 1);
        let tc = &check.packages[0];
        assert_eq!(tc.installed.as_deref(), Some("1.97.1"));
        assert_eq!(tc.latest.as_deref(), Some("1.97.1"));
        assert!(!tc.outdated);

        let s = check.self_status.expect("rustup self line");
        assert_eq!(s.installed.as_deref(), Some("1.29.0"));
        assert_eq!(s.latest.as_deref(), Some("1.29.0"));
        assert!(!s.update_available);
    }

    #[test]
    fn rustup_toolchain_list_parses_names() {
        let toolchains =
            parse_toolchain_list(&read_fixture("rustup_toolchain_list_2026-07-22.txt"));
        assert_eq!(toolchains.len(), 1);
        assert_eq!(toolchains[0].name, "stable-aarch64-apple-darwin");
        assert_eq!(toolchains[0].kind, PackageKind::Toolchain);
    }

    #[test]
    fn rustup_check_garbage_is_parse_failed_not_panic() {
        let err = parse_check("zsh: command not found: rustup\n").unwrap_err();
        assert!(matches!(err, PmError::ParseFailed { .. }));
    }
}
