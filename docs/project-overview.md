# Pack-Manager Project Overview

- **Date:** 2026-07-22
- **Type:** macOS desktop application
- **Architecture:** Single Tauri bundle with a React interface and Rust native core

## Executive Summary

Pack-Manager is a macOS application that detects installed package managers (`brew`, `mise`, `npm`, `uv`, `rustup`, and `mas`), displays their packages and update state, and runs selected upgrades with a preview of the exact commands.

The product emphasizes safe local automation: manager ownership and self-update routing are detected from executable paths, conflicting operations are coordinated through lock sets, output streams live, cancellation terminates process groups, and every operation leaves correlated logs/transcripts/history.

## Project Classification

- **Repository type:** Monolith / single cohesive application.
- **Project type:** Desktop (`desktop` in the BMad documentation taxonomy).
- **Primary languages:** Rust and TypeScript/TSX.
- **Architecture pattern:** Component/store frontend over a command- and event-driven native core.
- **Distribution:** Universal macOS app, DMG, ZIP, and signed Tauri updater artifacts.
- **External services:** None required at runtime beyond network access used by package managers and GitHub update checks.
- **Database:** None; settings, operation history, logs, and transcripts are file-backed.

The `src/` and `src-tauri/` folders are internal layers, not separately deployed projects. They build, version, test, and release as one application.

## Technology Stack

| Category | Technology |
| --- | --- |
| Desktop runtime | Tauri 2.11.5 (Rust crate), Tauri JS API 2.11.1 |
| Native core | Rust 2021, Tokio 1.53.1, Serde, tracing |
| Interface | React 19.2.8, TypeScript 5.8.3 |
| Build/styling | Vite 7.3.6, Tailwind CSS 4.3.3 |
| State/rendering | Zustand 5.0.14, TanStack React Virtual 3.14.7 |
| Tests | Cargo test, Vitest 4.1.10, React Testing Library, jsdom |
| Secrets | fnox with age-encrypted local values; GitHub Secrets in CI |
| Delivery | release-please and GitHub Actions |

## Key Features

- Finder-safe package-manager detection and version probing.
- Dynamic ownership and self-update routing derived from executable paths.
- Isolated inventory/outdated refresh for six package managers.
- Virtualized package tables with search, filtering, selection, and pinned/greedy safeguards.
- Exact-command upgrade plan preview with exclusions, locks, warnings, and notes.
- Lock-set scheduler with cross-manager parallelism, a global cap, fairness guard, and refresh coalescing.
- Live stdout/stderr, cancellation, stall detection, timeout escalation, and terminal handoff.
- Persistent settings, crash journal/history, structured logs, and full operation transcripts.
- Diagnostics ZIP export with regular-file/symlink protection.
- In-app application update discovery/download with user-triggered install/restart.
- Automated universal, signed, notarized release artifacts.

## Architecture Highlights

### Typed local boundary

Twenty native commands cover detection, refresh/upgrade planning, operations, settings/diagnostics, and app updates. Six events deliver detection, snapshots, operation state/output/stalls, and application-update state.

`src/lib/ipc/bridge.ts` is the only frontend Tauri API importer. Rust and TypeScript contract fixtures detect wire drift across both languages.

### Safe execution core

The native plan builder derives exact manager commands. The scheduler atomically acquires all manager locks, and the process runner uses absolute executables, a cleared explicit environment, null stdin, process groups, and termination escalation.

### Failure isolation and observability

Manager refreshes are independent and retain prior successful snapshots on failure. Operation IDs correlate frontend events, journal entries, structured logs, and line-flushed transcripts.

### File-backed durable state

Settings and operation history use atomic/crash-aware local files. Logs and transcripts have explicit retention policies. Runtime snapshots and view state remain in memory.

## Development Overview

### Prerequisites

- macOS and Apple command-line build tools.
- Node.js/npm (CI uses Node 24; no local version pin exists).
- Stable Rust/Cargo.
- fnox via mise for an updater-signed local build.

### Getting Started

```sh
npm install
npm run tauri dev
```

### Key Commands

- **Frontend tests:** `npm test`
- **Typecheck:** `npx tsc --noEmit`
- **Frontend production build:** `npm run build`
- **Native checks:** `cd src-tauri && cargo fmt --check && cargo clippy --all-targets -- -D warnings && cargo test --locked`
- **Ignored live smoke:** `cd src-tauri && cargo test -- --ignored`
- **Signed app build:** `fnox exec -- npm run tauri build`
- **Unsigned smoke build:** `npm run tauri build -- --no-sign`

At scan time, all 120 frontend tests passed across 22 files. Native verification should still be run as part of final documentation validation because this workflow performed a source deep scan rather than changing product code.

## Repository Structure

```text
src/                       React components, stores, IPC mirror, styles, tests
src-tauri/src/             Tauri composition, adapters, scheduler, runner, persistence
src-tauri/tests/           Ignored real-machine smoke tests
dev/fixtures/              Parser captures and shared IPC contract payloads
dev/icon/                  Reproducible application icon source/generator
.github/workflows/         CI, review automation, release and distribution
docs/                      Authoritative design and generated brownfield docs
```

Generated dependencies/output (`node_modules/`, `dist/`, `src-tauri/target/`, `src-tauri/gen/`) are not product source.

## Documentation Map

- [index.md](./index.md) — master navigation and AI handoff.
- [architecture.md](./architecture.md) — detailed system architecture and trust boundaries.
- [component-inventory.md](./component-inventory.md) — React components, Zustand state, IPC seam, assets.
- [source-tree-analysis.md](./source-tree-analysis.md) — annotated folder/file organization.
- [development-guide.md](./development-guide.md) — setup, tests, fixtures, common change paths.
- [deployment-guide.md](./deployment-guide.md) — release-please, signing, notarization, and artifacts.
- [contribution-guide.md](./contribution-guide.md) — invariants, verification, formatting, commits, and review hotspots.

Existing authoritative/historical sources:

- [SPEC.md](./SPEC.md) — product and technical contract.
- [DECISIONS.md](./DECISIONS.md) — decisions and rejected alternatives.
- [IMPL_PLAN.md](./IMPL_PLAN.md) — original dependency-ordered implementation plan.
- [README.md](../README.md) — current project overview and commands.

## Current Constraints

- Local Node/Rust toolchains are not repository-pinned.
- No frontend lint/format command or browser end-to-end test suite is configured.
- The interface is English-only.
- Tauri CSP is currently `null`.
- Some older comments/spec text still reflect the pre-updater count of 17 commands/five events and conflicting historical mas status; use production registration and current fixtures as current implementation evidence.

---

_Generated using the BMad Method `document-project` workflow._
