//! Pure uv parsers (SPEC §5.5, §7.1).
//!
//! - `uv tool list` — tool lines `^name vVERSION$`; `- exe` lines accumulate
//!   into `meta.executables`; `warning:` lines (from EITHER stream) become
//!   `HealthIssue`s. A parenthetical command is only made runnable when it
//!   passes the strict backend parser and allowlist.
//! - `uv tool list --outdated` — parent lines use
//!   `name vINSTALLED [latest: LATEST]`; `- exe` child lines are ignored
//!   because the inventory parser already owns executable metadata. Empty
//!   output is clean, and unknown parent suffixes degrade to `latest: null`
//!   (UI shows "update available", never a fabricated delta).

use std::sync::LazyLock;

use regex::Regex;

use super::make_id;
use crate::ipc::{HealthIssue, HealthSeverity, ManagerId, Package, PackageKind, PackageMeta};

/// Tool + health issues parsed from `uv tool list`.
#[derive(Debug, Clone, PartialEq)]
pub struct UvToolList {
    pub packages: Vec<Package>,
    pub health: Vec<HealthIssue>,
}

/// `warning: Tool `NAME` …`
static WARN_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^warning:\s+Tool\s+`(?P<name>[^`]+)`").unwrap());

/// Exact optional reinstall suggestion appended to a uv tool warning.
static REINSTALL_SUGGESTION_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\(run `(?P<fix>[^`]+)` to reinstall\)$").unwrap());

/// Python distribution names are composed of ASCII letters/digits plus
/// `.`, `_`, and `-`, and start/end with an alphanumeric character. Keeping
/// this allowlist local to the trusted parser prevents a warning from
/// smuggling arbitrary argv into a runnable health fix.
static SAFE_TOOL_NAME_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^[A-Za-z0-9](?:[A-Za-z0-9._-]*[A-Za-z0-9])?$").unwrap());

/// `^name vVERSION$`
static TOOL_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^(?P<name>\S+)\s+v(?P<ver>\S+)$").unwrap());

/// Verified populated `uv tool list --outdated` parent row.
static OUTDATED_TOOL_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^(?P<name>\S+)\s+v(?P<installed>\S+)\s+\[latest:\s*(?P<latest>[^\]\s]+)\]$")
        .unwrap()
});

/// Previously supported parent-row shape. Keep it fully anchored so an
/// embedded `(vX available)` fragment cannot promote an unrelated line.
static OUTDATED_LEGACY_TOOL_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^(?P<name>\S+)\s+v?(?P<installed>[0-9]\S*)\s+\(v?(?P<latest>\S+)\s+available\)$")
        .unwrap()
});

pub fn parse_tool_list(stdout: &str, stderr: &str) -> UvToolList {
    UvToolList {
        packages: extract_tools(stdout),
        health: extract_health(stdout, stderr),
    }
}

fn extract_health(stdout: &str, stderr: &str) -> Vec<HealthIssue> {
    let mut out = Vec::new();
    for line in stdout.lines().chain(stderr.lines()) {
        let l = line.trim();
        if let Some(c) = WARN_RE.captures(l) {
            let name = c["name"].to_string();
            let suggested_fix = REINSTALL_SUGGESTION_RE
                .captures(l)
                .map(|captures| captures["fix"].to_string());
            let canonical_args = vec![
                "tool".to_string(),
                "install".to_string(),
                name.clone(),
                "--reinstall".to_string(),
            ];
            let canonical_fix = crate::ipc::command_preview("uv", &canonical_args);
            let fixable = SAFE_TOOL_NAME_RE.is_match(&name)
                && suggested_fix.as_deref() == Some(canonical_fix.as_str());
            out.push(HealthIssue {
                id: format!("uv:{name}"),
                manager_id: ManagerId::Uv,
                severity: HealthSeverity::Warning,
                title: format!("Tool `{name}` environment is broken."),
                detail: l.to_string(),
                // The full warning remains in `detail`, but an altered
                // suggestion is neither runnable nor presented as a copyable
                // fix command.
                fix_command: fixable.then_some(canonical_fix),
                fix_args: fixable.then_some(canonical_args),
                fixable,
            });
        }
    }
    out
}

fn extract_tools(stdout: &str) -> Vec<Package> {
    let mut packages = Vec::new();
    let mut current: Option<(Package, Vec<String>)> = None;

    for raw in stdout.lines() {
        let t = raw.trim();
        if t.is_empty() || t.starts_with("warning:") {
            continue;
        }
        if let Some(exe) = t.strip_prefix("- ") {
            if let Some((_, exes)) = current.as_mut() {
                exes.push(exe.trim().to_string());
            }
            continue;
        }
        if let Some(c) = TOOL_RE.captures(t) {
            if let Some((pkg, exes)) = current.take() {
                packages.push(finalize(pkg, exes));
            }
            let name = c["name"].to_string();
            let ver = c["ver"].to_string();
            let pkg = Package {
                id: make_id(PackageKind::Tool, &name),
                name,
                kind: PackageKind::Tool,
                installed: Some(ver.clone()),
                latest: Some(ver),
                outdated: false,
                pinned: false,
                meta: None,
            };
            current = Some((pkg, Vec::new()));
        }
        // Any other line is ignored leniently (never a panic).
    }
    if let Some((pkg, exes)) = current.take() {
        packages.push(finalize(pkg, exes));
    }
    packages
}

fn finalize(mut pkg: Package, exes: Vec<String>) -> Package {
    if !exes.is_empty() {
        pkg.meta = Some(PackageMeta {
            executables: Some(exes),
            ..PackageMeta::default()
        });
    }
    pkg
}

/// `uv tool list --outdated`. Empty = clean. Executable child rows are not
/// packages and are ignored. Unknown parent suffixes degrade to `latest: null`.
pub fn parse_tool_list_outdated(stdout: &str) -> Vec<Package> {
    let mut out = Vec::new();
    for raw in stdout.lines() {
        let line = raw.trim();
        if line.is_empty() || line.starts_with("warning:") {
            continue;
        }
        if line
            .split_whitespace()
            .next()
            .is_some_and(|token| token == "-")
        {
            continue;
        }
        if let Some(c) = OUTDATED_TOOL_RE.captures(line) {
            if !SAFE_TOOL_NAME_RE.is_match(&c["name"]) {
                continue;
            }
            out.push(Package {
                id: make_id(PackageKind::Tool, &c["name"]),
                name: c["name"].to_string(),
                kind: PackageKind::Tool,
                installed: Some(c["installed"].to_string()),
                latest: Some(c["latest"].to_string()),
                outdated: true,
                pinned: false,
                meta: None,
            });
            continue;
        }
        if let Some(c) = OUTDATED_LEGACY_TOOL_RE.captures(line) {
            if !SAFE_TOOL_NAME_RE.is_match(&c["name"]) {
                continue;
            }
            out.push(Package {
                id: make_id(PackageKind::Tool, &c["name"]),
                name: c["name"].to_string(),
                kind: PackageKind::Tool,
                installed: Some(c["installed"].to_string()),
                latest: Some(c["latest"].to_string()),
                outdated: true,
                pinned: false,
                meta: None,
            });
            continue;
        }
        let mut tokens = line.split_whitespace();
        let Some(name) = tokens.next() else {
            continue;
        };
        if !SAFE_TOOL_NAME_RE.is_match(name) {
            continue;
        }
        let Some(version_token) = tokens.next() else {
            continue;
        };
        let installed = version_token.strip_prefix('v').unwrap_or(version_token);
        if !installed.starts_with(|character: char| character.is_ascii_digit()) {
            continue;
        }
        if tokens
            .next()
            .is_some_and(|suffix| !suffix.starts_with('(') && !suffix.starts_with('['))
        {
            continue;
        }
        out.push(Package {
            id: make_id(PackageKind::Tool, name),
            name: name.to_string(),
            kind: PackageKind::Tool,
            installed: Some(installed.to_string()),
            latest: None,
            outdated: true,
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
    fn uv_tool_list_extracts_broken_env_warning_and_fix_command() {
        let parsed = parse_tool_list(&read_fixture("uv_tool_list_2026-07-21.txt"), "");
        assert_eq!(
            parsed.packages.len(),
            12,
            "12 tools despite the warning line"
        );

        assert_eq!(parsed.health.len(), 1);
        let h = &parsed.health[0];
        assert_eq!(h.id, "uv:aider-chat");
        assert_eq!(h.manager_id, ManagerId::Uv);
        assert_eq!(h.severity, HealthSeverity::Warning);
        assert_eq!(h.title, "Tool `aider-chat` environment is broken.");
        assert_eq!(
            h.fix_command.as_deref(),
            Some("uv tool install aider-chat --reinstall")
        );
        assert_eq!(
            h.fix_args.as_deref(),
            Some(
                ["tool", "install", "aider-chat", "--reinstall"]
                    .map(str::to_string)
                    .as_slice()
            )
        );
        assert!(h.fixable);
    }

    #[test]
    fn altered_uv_reinstall_suggestion_remains_visible_but_is_not_runnable() {
        let parsed = parse_tool_list(
            "",
            "warning: Tool `aider-chat` environment not found (run `uv tool install aider-chat --reinstall --index-url https://attacker.invalid` to reinstall)\n",
        );
        assert_eq!(parsed.health.len(), 1);
        let issue = &parsed.health[0];
        assert!(issue.detail.contains("attacker.invalid"));
        assert!(issue.fix_command.is_none());
        assert!(!issue.fixable);
        assert!(issue.fix_args.is_none());

        let unsafe_name = parse_tool_list(
            "warning: Tool `aider chat` environment not found (run `uv tool install aider chat --reinstall` to reinstall)\n",
            "",
        );
        assert_eq!(unsafe_name.health.len(), 1);
        assert!(!unsafe_name.health[0].fixable);
        assert!(unsafe_name.health[0].fix_args.is_none());
    }

    #[test]
    fn uv_warning_without_reinstall_suggestion_remains_visible() {
        let warning = "warning: Tool `aider-chat` environment not found";
        let parsed = parse_tool_list("", warning);

        assert_eq!(parsed.health.len(), 1);
        let issue = &parsed.health[0];
        assert_eq!(issue.id, "uv:aider-chat");
        assert_eq!(issue.detail, warning);
        assert!(issue.fix_command.is_none());
        assert!(issue.fix_args.is_none());
        assert!(!issue.fixable);
    }

    #[test]
    fn uv_warning_with_malformed_reinstall_suggestion_remains_visible() {
        let warning = "warning: Tool `aider-chat` environment not found (run `uv tool install aider-chat --reinstall` to reinstall";
        let parsed = parse_tool_list(warning, "");

        assert_eq!(parsed.health.len(), 1);
        let issue = &parsed.health[0];
        assert_eq!(issue.detail, warning);
        assert!(issue.fix_command.is_none());
        assert!(issue.fix_args.is_none());
        assert!(!issue.fixable);
    }

    #[test]
    fn uv_tool_list_clean_collects_executables() {
        let parsed = parse_tool_list(&read_fixture("uv_tool_list.txt"), "");
        assert_eq!(parsed.packages.len(), 12);
        assert!(parsed.health.is_empty());

        let cct = parsed
            .packages
            .iter()
            .find(|p| p.name == "claude-code-tools")
            .unwrap();
        assert_eq!(
            cct.meta
                .as_ref()
                .unwrap()
                .executables
                .as_ref()
                .unwrap()
                .len(),
            17
        );

        // Non-semver version kept verbatim.
        let serena = parsed
            .packages
            .iter()
            .find(|p| p.name == "serena-agent")
            .unwrap();
        assert_eq!(serena.installed.as_deref(), Some("1.6.2.dev0"));
        assert!(!serena.outdated);
    }

    #[test]
    fn uv_outdated_empty_output_is_clean_not_error() {
        assert!(parse_tool_list_outdated(&read_fixture("uv_tool_list_outdated.txt")).is_empty());
        assert!(parse_tool_list_outdated("").is_empty());
    }

    #[test]
    fn uv_outdated_populated_capture_ignores_executables_and_extracts_versions() {
        let rows = parse_tool_list_outdated(&read_fixture("uv_tool_list_outdated_2026-07-23.txt"));
        assert_eq!(rows.len(), 1);

        let cct = &rows[0];
        assert_eq!(cct.id, "tool:claude-code-tools");
        assert_eq!(cct.name, "claude-code-tools");
        assert_eq!(cct.installed.as_deref(), Some("1.19.0"));
        assert_eq!(cct.latest.as_deref(), Some("1.19.2"));
        assert!(cct.outdated);
        assert!(!rows.iter().any(|package| package.id == "tool:-"));
    }

    #[test]
    fn uv_outdated_unknown_suffix_degrades_to_null_latest() {
        // Unknown suffix (no `(… available)`): latest degrades to null.
        let rows = parse_tool_list_outdated("ruff 0.15.20 (some unrecognised note)\n");
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].name, "ruff");
        assert!(rows[0].latest.is_none(), "unknown suffix → null latest");
        assert!(rows[0].outdated);

        // A recognisable `(vX available)` suffix IS captured as latest.
        let known = parse_tool_list_outdated("ruff 0.15.20 (v0.15.22 available)\n");
        assert_eq!(known[0].latest.as_deref(), Some("0.15.22"));
    }

    #[test]
    fn uv_outdated_ignores_alternate_whitespace_children_and_unrelated_lines() {
        let rows = parse_tool_list_outdated(
            "-\tvoice-type\nstatus message\ncache 2026 unavailable\nruff 0.15.20 note (v9 available)\n",
        );
        assert!(rows.is_empty());
    }
}
