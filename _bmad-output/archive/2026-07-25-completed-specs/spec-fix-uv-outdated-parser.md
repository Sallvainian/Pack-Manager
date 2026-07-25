---
title: 'Fix populated UV outdated parsing'
type: 'bugfix'
created: '2026-07-23'
status: 'done'
baseline_commit: 'fe2881f3e48d26c0561857f72143c6570a5620fc'
review_loop_iteration: 0
context:
  - '{project-root}/_bmad-output/project-context.md'
---

<frozen-after-approval reason="human-owned intent — do not modify unless human renegotiates">

## Intent

**Problem:** Pack-Manager parses executable child lines from populated `uv tool list --outdated` output as packages. With the current live output, duplicate `tool:-` overlay rows collapse to a bogus final row displayed as name `-` and installed version `voice-type`; the real package also loses its reported latest version.

**Approach:** Ground the parser in the captured UV 0.11.30 output, recognize its `name vINSTALLED [latest: LATEST]` parent rows, ignore executable child rows in the outdated overlay, and retain the existing safe behavior for unknown parent suffixes.

## Boundaries & Constraints

**Always:** Treat UV's outdated verdict as authoritative; preserve manager-supplied version text except for UV's structural `v` marker; merge the outdated overlay onto the complete inventory so executable metadata remains attached; use a committed live capture for correctness claims; retain lenient handling of unknown parent suffixes without fabricating a latest version.

**Ask First:** Any change to UV refresh commands, upgrade routing, package IDs, generic inventory-overlay semantics, or user-facing table behavior beyond correcting this malformed snapshot.

**Never:** Treat executable names as independently upgradable packages; infer outdatedness by comparing versions; add network or real-machine behavior to the default test suite; alter unrelated working-tree changes.

## I/O & Edge-Case Matrix

| Scenario | Input / State | Expected Output / Behavior | Error Handling |
|----------|--------------|---------------------------|----------------|
| Populated current output | `claude-code-tools v1.19.0 [latest: 1.19.2]` followed by `- executable` lines | One outdated `claude-code-tools` overlay row with installed `1.19.0`, latest `1.19.2`; no `tool:-` row | Child lines are ignored by the outdated overlay |
| Inventory merge | Normal tool inventory plus populated outdated capture | Existing package is marked outdated while its executable metadata is preserved | No overlay-only executable package is appended |
| Unknown parent suffix | Valid package parent with an unrecognized note | Package remains outdated with `latest: null` | UI may show generic “update available”; no version is fabricated |
| Empty output | Zero-byte outdated output | Empty overlay and no outdated packages | Treated as a clean result |

</frozen-after-approval>

## Code Map

- `src-tauri/src/managers/parse/uv.rs` -- Pure normal-list and outdated-list parsing.
- `src-tauri/src/managers/uv.rs` -- UV refresh plan, snapshot assembly, and adapter-level tests.
- `src-tauri/src/managers/parse/mod.rs` -- Existing inventory/overlay merge whose last-write collapse made the final `tool:-` child visible.
- `dev/fixtures/uv_tool_list_outdated_2026-07-23.txt` -- Exact populated output captured from mise-managed UV 0.11.30.
- `dev/fixtures/README.md` -- Fixture provenance and verification status.
- `docs/SPEC.md` -- Normative UV parsing contract.
- `docs/DECISIONS.md` -- Record that populated UV output is now verified.

## Tasks & Acceptance

**Execution:**
- [x] `dev/fixtures/uv_tool_list_outdated_2026-07-23.txt` and `dev/fixtures/README.md` -- commit the exact live populated output with provenance.
- [x] `src-tauri/src/managers/parse/uv.rs` -- skip executable child rows, parse current bracketed latest versions, normalize structural `v` markers, and preserve unknown-suffix fallback behavior.
- [x] `src-tauri/src/managers/uv.rs` -- remove the obsolete under-verification warning and add a merged-snapshot regression test.
- [x] `docs/SPEC.md` and `docs/DECISIONS.md` -- replace the zero-byte-only/unknown-format claim with the verified populated contract.

**Acceptance Criteria:**
- Given the captured populated UV output, when it is parsed and merged with `uv tool list`, then the snapshot contains exactly one outdated package named `claude-code-tools` with installed `1.19.0` and latest `1.19.2`.
- Given the same snapshot, when package IDs and metadata are inspected, then no `tool:-` package exists and all 17 `claude-code-tools` executables remain attached.
- Given empty or unknown-suffix parent output, when parsed, then existing clean-result and non-fabrication behavior remains intact.

## Spec Change Log

## Verification

**Commands:**
- `cargo fmt --check` from `src-tauri/` -- expected: formatting is clean.
- `cargo test managers::parse::uv --locked` from `src-tauri/` -- expected: UV parser tests pass.
- `cargo test managers::uv --locked` from `src-tauri/` -- expected: UV adapter tests pass.
- `cargo clippy --all-targets -- -D warnings` from `src-tauri/` -- expected: no warnings.
- `cargo test --locked` from `src-tauri/` -- expected: complete Rust suite passes.

**Results (2026-07-23):**
- `cargo fmt --check` passed.
- UV parser tests passed: 9 passed, 0 failed.
- UV adapter tests passed: 6 passed, 0 failed.
- Clippy passed with warnings denied.
- Full Rust suite passed: 248 passed, 0 failed, 11 intentionally ignored across unit and live-smoke targets.

## Suggested Review Order

**Parsing boundary**

- Start with the parent-row grammar and child-row rejection that remove the bogus package.
  [`uv.rs:155`](../../src-tauri/src/managers/parse/uv.rs#L155)

- Confirm the merged snapshot preserves all executables and produces only the real outdated tool.
  [`uv.rs:212`](../../src-tauri/src/managers/uv.rs#L212)

**Product contract**

- Review the normative UV format, fallback behavior, and non-fabrication guarantee.
  [`SPEC.md:323`](../../docs/SPEC.md#L323)

- See why executable children are excluded and the observed format is intentionally scoped.
  [`DECISIONS.md:27`](../../docs/DECISIONS.md#L27)

**Evidence and tests**

- Check the live-capture provenance and exact verification scope.
  [`README.md:56`](../../dev/fixtures/README.md#L56)

- Inspect the captured UV output that originally reproduced the malformed row.
  [`uv_tool_list_outdated_2026-07-23.txt:1`](../../dev/fixtures/uv_tool_list_outdated_2026-07-23.txt#L1)

- Review parser regression coverage for current, legacy, malformed, and whitespace variants.
  [`uv.rs:358`](../../src-tauri/src/managers/parse/uv.rs#L358)
