---
title: 'Configure Dependabot version updates'
type: 'chore'
created: '2026-07-22'
status: 'done'
baseline_commit: '673dc717e16950939bdc6d311874bcaf46c5210c'
review_loop_iteration: 0
context: []
---

<frozen-after-approval reason="human-owned intent — do not modify unless human renegotiates">

## Intent

**Problem:** Pack-Manager has npm, Cargo, and GitHub Actions dependencies but no Dependabot configuration, so GitHub cannot routinely propose version updates for them.

**Approach:** Add one `.github/dependabot.yml` configuration that checks all three ecosystems weekly from their actual manifest locations and uses conventional commit prefixes compatible with the project's release-please rules.

## Boundaries & Constraints

**Always:** Use Dependabot configuration version 2. Configure `npm` at `/`, `cargo` at `/src-tauri`, and `github-actions` at `/`. Keep each ecosystem in its own update block with a weekly schedule. Use `chore(deps)` for npm and Cargo commits and `ci(deps)` for GitHub Actions commits so automated dependency updates do not independently trigger an application release. Preserve all unrelated working-tree changes from the other active Codex session.

**Ask First:** Adding update groups, dependency allow/ignore rules, custom labels, assignees, reviewers, private-registry credentials, automatic merging, or a non-weekly schedule.

**Never:** Edit dependency manifests, lockfiles, workflow files, application versions, release-please-owned files, or secrets as part of this change. Do not use `fix:` or `feat:` Dependabot prefixes because those would cause release-please to calculate an application release.

</frozen-after-approval>

## Code Map

- `.github/dependabot.yml` -- new Dependabot version-update configuration and the only product file changed.
- `package.json` and `package-lock.json` -- confirm the npm ecosystem lives at the repository root.
- `src-tauri/Cargo.toml` and `src-tauri/Cargo.lock` -- confirm the Cargo ecosystem lives under `/src-tauri`.
- `.github/workflows/` -- contains versioned GitHub Actions dependencies discovered from directory `/`.
- `.github/workflows/ci.yml` -- existing pull-request checks that will validate npm and Cargo update PRs.

## Tasks & Acceptance

**Execution:**
- [x] `.github/dependabot.yml` -- add separate weekly update entries for npm, Cargo, and GitHub Actions with release-safe conventional commit prefixes.

**Acceptance Criteria:**
- Given the configuration is present on the default branch, when GitHub reads it, then all three supported ecosystems have valid version-update entries using their correct directories.
- Given Dependabot creates an npm or Cargo version-update PR, when it generates the commit message, then the prefix is `chore(deps)` and does not request a release-please version bump.
- Given Dependabot creates a GitHub Actions version-update PR, when it generates the commit message, then the prefix is `ci(deps)` and does not request a release-please version bump.
- Given this change is reviewed, when its diff is inspected, then no manifest, lockfile, workflow, release, version, credential, or unrelated file has been changed.

## Spec Change Log

## Verification

**Commands:**
- `ruby -e 'require "yaml"; YAML.load_file(".github/dependabot.yml")'` -- expected: the file parses as valid YAML without error.
- `ruby -e 'File.foreach(".github/dependabot.yml").with_index(1) { |line, number| abort("trailing whitespace on line #{number}") if line.chomp.match?(/[ \t]+\z/) }'` -- expected: no trailing whitespace errors.
- `ruby -ryaml -e 'config = YAML.load_file(".github/dependabot.yml"); expected = [["npm", "/", "weekly", "chore(deps)"], ["cargo", "/src-tauri", "weekly", "chore(deps)"], ["github-actions", "/", "weekly", "ci(deps)"]]; actual = config.fetch("updates").map { |u| [u["package-ecosystem"], u["directory"], u.dig("schedule", "interval"), u.dig("commit-message", "prefix")] }; abort("unexpected version: #{config["version"].inspect}") unless config["version"] == 2; abort("unexpected updates: #{actual.inspect}") unless actual == expected; puts "version=2; updates=#{actual.length}; entries=#{actual.inspect}"'` -- expected: version 2 and exactly the three intended update mappings in order.

**Manual checks (if no CLI):**
- Confirm the file has `version: 2` and exactly three update blocks for `npm`, `cargo`, and `github-actions` using `/`, `/src-tauri`, and `/` respectively.
- Confirm every block is weekly and uses the intended conventional commit prefix.

## Suggested Review Order

**Update coverage**

- Establishes the Dependabot format and keeps ecosystems independently diagnosable.
  [`dependabot.yml:1`](../../.github/dependabot.yml#L1)

- Updates frontend dependencies from the root with a release-safe prefix.
  [`dependabot.yml:3`](../../.github/dependabot.yml#L3)

- Updates Rust dependencies from the Tauri manifest directory.
  [`dependabot.yml:10`](../../.github/dependabot.yml#L10)

- Updates workflow actions while preserving the CI-only commit type.
  [`dependabot.yml:17`](../../.github/dependabot.yml#L17)

**Verification**

- Checks syntax, whitespace, and every release-sensitive mapping.
  [`spec-configure-dependabot.md:52`](spec-configure-dependabot.md#L52)
