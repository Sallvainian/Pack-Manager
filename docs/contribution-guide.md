# Pack-Manager Contribution Guide

**Date:** 2026-07-22

Pack-Manager does not currently have a standalone `CONTRIBUTING.md`. This guide consolidates the active repository rules, architectural constraints, verification gates, and release conventions.

## Source of Truth

For product behavior:

1. `docs/SPEC.md` is the authoritative contract.
2. `docs/DECISIONS.md` explains the accepted design and rejected alternatives.
3. If both are silent, choose the smallest behavior that fits existing patterns and record the decision when it is consequential.

Use `README.md` for current developer/operator commands and this generated documentation for codebase navigation. Treat `docs/IMPL_PLAN.md` as implementation history; some of its pre-implementation facts can be stale.

## Before Editing

- Read the relevant SPEC feature/invariant and matching decision entries.
- Locate existing implementations and tests before introducing a new pattern.
- Preserve unrelated working-tree changes; do not assume untracked or modified files are disposable.
- Read configuration files before changing them. Do not guess paths or contents.
- Keep secrets in fnox or GitHub Secrets; never hardcode them in source or `.env` files.

## Load-Bearing Invariants

Changes must preserve these system rules:

- The package manager's own outdated verdict is authoritative; local version comparison is visual only.
- Manager ownership and self-update routing are derived from detected paths, with raw mise-shim classification before canonicalization.
- Bulk upgrades preview exact commands before anything runs.
- Homebrew work is serialized through lock sets; routed and shared-tree operations acquire all required manager locks atomically.
- The app never requests sudo or a password. Child stdin is null and stalled work offers a terminal handoff.
- One manager failure does not erase other managers or the previous snapshot.
- Operations remain reconstructible through events, transcripts, structured logs, and the crash journal.
- Rust–TypeScript IPC payloads stay byte-contract compatible.

See `docs/architecture.md` for where each invariant is implemented.

## Code Organization

- Put feature-specific React code in the matching `src/components/<feature>/` folder.
- Put reusable visual controls in `src/components/primitives/`.
- Keep frontend native calls in `src/lib/ipc/client.ts`; only `src/lib/ipc/bridge.ts` imports Tauri APIs.
- Keep native command handlers thin and put logic in the dedicated Rust module.
- Keep manager output parsers pure and fixture-grounded under `src-tauri/src/managers/parse/`.
- Treat `queue.rs`, `process/runner.rs`, `ops.rs`, IPC types, and updater installation as high-risk trust boundaries.

## Tests With Every Change

Choose the smallest relevant tests while developing, then run the full affected gates before handing off.

Frontend:

```sh
npm test
npx tsc --noEmit
npm run build
```

Native core:

```sh
cd src-tauri
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test --locked
```

Run machine-dependent smoke tests only when the change affects live detection, command compatibility, routing, or packaging:

```sh
cd src-tauri
cargo test -- --ignored
```

Tests should remain deterministic and offline by default. Use `FakeRunner`, fake IPC, paused/fake time, and committed fixtures instead of live network/process behavior.

## Formatting

- Run `cargo fmt` on modified Rust files before committing, then verify with `cargo fmt --check`.
- The project currently has no configured Prettier/ESLint script for TypeScript or Markdown. Match the established formatting in surrounding files and rely on the typecheck, test, and build gates.
- Keep Markdown tables, headings, command blocks, and internal links renderable and current.

## Fixtures and Contracts

- Never create an unlabeled guessed parser fixture.
- Name synthetic captures with `_synthetic` and document their source and retirement condition.
- Use `dev/capture-fixtures.sh` for offline-safe recapture; opt into network probes explicitly with `PM_CAPTURE_ONLINE=1`.
- When IPC types change, update Rust models, TypeScript mirrors/guards, and `dev/fixtures/ipc/` together.
- Regenerate intentional IPC fixture changes with `PM_UPDATE_CONTRACT=1 cargo test ipc_contract`, then run both language suites.

## Commit Messages and Releases

Use conventional commits because release-please derives versions from them:

- `fix: ...` for a patch release.
- `feat: ...` for a minor release.
- `feat!: ...` or a `BREAKING CHANGE:` footer for a major release.
- `chore:`, `docs:`, `ci:`, `refactor:`, and `test:` when no release is warranted by that commit alone.

Use a separate `Release-As: X.Y.Z` footer only when intentionally forcing a specific release version.

Do not manually edit:

- Versions in `package.json`, `package-lock.json`, `src-tauri/tauri.conf.json`, `src-tauri/Cargo.toml`, or `src-tauri/Cargo.lock`.
- `CHANGELOG.md`.
- `.release-please-manifest.json`.

Release-please updates those files in its release PR.

## Pull Request Expectations

A change is ready for review when:

- Its behavior matches the SPEC/decisions or documents an intentional course correction.
- Tests cover success, failure, and concurrency/contract edge cases relevant to the change.
- Required format, lint, type, test, and build gates pass.
- User-facing or developer-facing documentation is updated when commands, architecture, or limitations change.
- No secret values, personal paths beyond intentional project examples, generated dependencies, or local build artifacts are included.
- The PR description explains the behavior change and concrete verification performed.

GitHub CI runs independent Rust, frontend, and Playwright browser gates. Human-authored pull requests may also receive automated review against Pack-Manager's invariants.

## Sensitive and Generated Files

Do not commit credential values or local runtime data. In particular, protect:

- Decrypted updater/Apple signing credentials.
- `~/Library/Application Support/Pack-Manager/` settings and journal data.
- `~/Library/Logs/Pack-Manager/` logs and operation transcripts.
- Diagnostics ZIP files, which can contain recent operational context.

Do not treat generated/vendor folders as source changes: `node_modules/`, `dist/`, `src-tauri/target/`, and `src-tauri/gen/`.

## Review Hotspots

Give extra scrutiny to changes that affect:

- Previewed command equivalence with executed commands.
- Manager lock sets, fairness, cancellation, or post-success refresh.
- Child environment clearing, stdin, process groups, timeout escalation, or transcript fidelity.
- Detection ownership and self-update routing.
- IPC serialization and runtime guards.
- Atomic settings/journal writes and diagnostics symlink filtering.
- Updater signature checks, writable-bundle handling, restart guards, signing, or notarization.
