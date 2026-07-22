//! Pure mas parsers (SPEC §5.5, §7.1, DECISIONS D23).
//!
//! mas is ABSENT on this machine (`zsh: command not found: mas`), so these are
//! grounded only in labeled `*_synthetic` fixtures and mas's documented text
//! format. Every parse failure is `ParseFailed`-with-excerpt, never a panic —
//! a defensive test documents that the shell "command not found" line never
//! reaches a parser (detection gates mas).
//!
//! NOTE: unlike every other manager, the mas package id segment is the numeric
//! App Store id (`app:497799835`), not the display name — `mas upgrade` takes
//! the numeric id, so the routing key must survive in the id. `name` is the
//! human-readable app name.

use std::sync::LazyLock;

use regex::Regex;

use super::excerpt;
use crate::error::PmError;
use crate::ipc::{Package, PackageKind};

/// `123456 App Name (1.2.3)`
static LIST_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^(?P<id>\d+)\s+(?P<name>.+?)\s+\((?P<ver>\S+)\)\s*$").unwrap());

/// `123456 App Name (1.2.2 -> 1.2.3)`
static OUTDATED_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^(?P<id>\d+)\s+(?P<name>.+?)\s+\((?P<from>\S+)\s*->\s*(?P<to>\S+)\)\s*$").unwrap()
});

/// `mas list` — installed apps (inventory).
pub fn parse_list(stdout: &str) -> Result<Vec<Package>, PmError> {
    let mut out = Vec::new();
    for raw in stdout.lines() {
        let line = raw.trim();
        if line.is_empty() {
            continue;
        }
        let Some(c) = LIST_RE.captures(line) else {
            return Err(PmError::ParseFailed {
                what: "mas list".into(),
                excerpt: excerpt(stdout),
            });
        };
        let ver = c["ver"].to_string();
        out.push(app_pkg(&c["id"], &c["name"], Some(ver.clone()), Some(ver), false));
    }
    Ok(out)
}

/// `mas outdated` — apps with an update available.
pub fn parse_outdated(stdout: &str) -> Result<Vec<Package>, PmError> {
    let mut out = Vec::new();
    for raw in stdout.lines() {
        let line = raw.trim();
        if line.is_empty() {
            continue;
        }
        let Some(c) = OUTDATED_RE.captures(line) else {
            return Err(PmError::ParseFailed {
                what: "mas outdated".into(),
                excerpt: excerpt(stdout),
            });
        };
        out.push(app_pkg(
            &c["id"],
            &c["name"],
            Some(c["from"].to_string()),
            Some(c["to"].to_string()),
            true,
        ));
    }
    Ok(out)
}

fn app_pkg(
    app_id: &str,
    name: &str,
    installed: Option<String>,
    latest: Option<String>,
    outdated: bool,
) -> Package {
    Package {
        id: format!("{}:{}", super::kind_prefix(PackageKind::App), app_id),
        name: name.to_string(),
        kind: PackageKind::App,
        installed,
        latest,
        outdated,
        pinned: false,
        meta: None,
    }
}

#[cfg(test)]
mod tests {
    use super::super::read_fixture;
    use super::*;

    #[test]
    fn mas_outdated_synthetic_parses() {
        let rows = parse_outdated(&read_fixture("mas_outdated_synthetic.txt")).expect("parse");
        assert_eq!(rows.len(), 2);

        let xcode = &rows[0];
        assert_eq!(xcode.id, "app:497799835");
        assert_eq!(xcode.name, "Xcode");
        assert_eq!(xcode.installed.as_deref(), Some("16.1"));
        assert_eq!(xcode.latest.as_deref(), Some("16.2"));
        assert!(xcode.outdated);

        // App name with spaces is handled.
        let rdp = &rows[1];
        assert_eq!(rdp.name, "Microsoft Remote Desktop");
        assert_eq!(rdp.installed.as_deref(), Some("10.9.4"));
        assert_eq!(rdp.latest.as_deref(), Some("10.9.5"));
    }

    #[test]
    fn mas_list_synthetic_parses() {
        let rows = parse_list(&read_fixture("mas_list_synthetic.txt")).expect("parse");
        assert_eq!(rows.len(), 3);
        let xcode = &rows[0];
        assert_eq!(xcode.name, "Xcode");
        assert_eq!(xcode.installed.as_deref(), Some("16.2"));
        assert!(!xcode.outdated);
    }

    #[test]
    fn mas_shell_error_never_reaches_parser() {
        // Detection gates mas (`zsh: command not found: mas`); if that string
        // ever reached a parser it degrades to ParseFailed, never a panic.
        let err = parse_outdated(&read_fixture("mas_outdated.txt")).unwrap_err();
        match err {
            PmError::ParseFailed { excerpt, .. } => {
                assert!(excerpt.contains("command not found"))
            }
            other => panic!("expected ParseFailed, got {other:?}"),
        }
    }
}
