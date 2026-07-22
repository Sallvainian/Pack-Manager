# Pack-Manager Documentation Index

- **Type:** Single-part macOS desktop application
- **Primary languages:** Rust and TypeScript
- **Architecture:** React component/store interface over a command- and event-driven Tauri core
- **Last updated:** 2026-07-22

## Project Overview

Pack-Manager detects local macOS package managers, shows installed and outdated packages, previews exact upgrade commands, and executes safe, observable, cancellable operations. The React webview and Rust native core are internal layers of one versioned and distributed Tauri application.

Start with [project-overview.md](./project-overview.md) for the concise system summary or [architecture.md](./architecture.md) for implementation constraints and data flow.

## Quick Reference

- **Tech stack:** Tauri 2.11.5, Rust 2021, React 19.2.8, TypeScript 5.8.3, Vite 7.3.6, Tailwind CSS 4.3.3, Zustand 5.0.14.
- **Frontend entry:** `src/main.tsx` → `src/App.tsx` → `src/components/shell/AppLayout.tsx`.
- **Native entry:** `src-tauri/src/main.rs` → `pack_manager_lib::run()` in `src-tauri/src/lib.rs`.
- **Integration:** 20 typed Tauri commands and six typed events; no HTTP API.
- **Persistence:** JSON settings, JSONL operation journal, structured logs, and line-flushed transcripts; no database.
- **Tests:** Cargo tests plus Vitest/Testing Library; machine-dependent Rust tests are ignored by default.
- **Deployment:** Universal macOS app with release-please, GitHub Actions, signing/notarization, DMG/ZIP, and updater assets.

## Generated Documentation

### Core Architecture

- [Project Overview](./project-overview.md) — purpose, classification, stack, features, and concise handoff.
- [Architecture](./architecture.md) — invariants, layers, IPC, scheduler, execution, persistence, security, tests, and deployment.
- [Source Tree Analysis](./source-tree-analysis.md) — annotated repository tree, entry points, critical folders, assets, and configuration.
- [Component Inventory](./component-inventory.md) — UI catalog, primitives, Zustand stores, IPC seam, styling, and extension guidance.

### Development and Operations

- [Development Guide](./development-guide.md) — setup, commands, tests, IPC fixtures, package-manager fixtures, common change paths, and local data.
- [Deployment Guide](./deployment-guide.md) — release-please flow, version ownership, signing/notarization, artifacts, and pipeline testing.
- [Contribution Guide](./contribution-guide.md) — sources of truth, invariants, verification, formatting, conventional commits, and review hotspots.

### Workflow State

- [Project Scan Report](./project-scan-report.json) — resumable BMad scan status, classification, batch summaries, outputs, and validation state.

## Existing Project Documentation

### Product and Design

- [README](../README.md) — current features, stack, development commands, releases, logs, diagnostics, and limitations.
- [Authoritative Specification](./SPEC.md) — product behavior, invariants, UX, architecture, IPC, tests, and packaging contract.
- [Architecture Decisions](./DECISIONS.md) — accepted decisions, rejected alternatives, and rationale.
- [Implementation Plan](./IMPL_PLAN.md) — dependency-ordered implementation history and verification gates.
- [Fixture Provenance](../dev/fixtures/README.md) — capture provenance, parser facts, synthetic-fixture policy, and backlog.

### Automation Definitions

- [Continuous Integration](../.github/workflows/ci.yml) — Rust checks, web checks, and main-branch bundle smoke.
- [Claude Issue/PR Agent](../.github/workflows/claude.yml) — mention-triggered repository assistance.
- [Automated PR Review](../.github/workflows/claude-code-review.yml) — human-authored pull-request review workflow.
- [Release Please](../.github/workflows/release-please.yml) — release PR, tag, GitHub Release, and build orchestration.
- [Release Build](../.github/workflows/release.yml) — universal build, signing, notarization, verification, and asset publication.

## Getting Started

### Prerequisites

- macOS and Apple command-line build tools.
- Node.js/npm (CI uses Node 24).
- Stable Rust/Cargo.
- fnox through mise for a signed updater build.

### Install and Run

```sh
npm install
npm run tauri dev
```

### Verify

```sh
npm test
npx tsc --noEmit
npm run build

cd src-tauri
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test --locked
```

### Build the App

```sh
fnox exec -- npm run tauri build
```

Use `npm run tauri build -- --no-sign` only for an intentional unsigned smoke build.

## For AI-Assisted Development

Use this index as the primary retrieval source. Choose the smallest relevant documentation set:

### UI-only changes

Read:

1. [Component Inventory](./component-inventory.md)
2. [Architecture — Frontend Architecture](./architecture.md#frontend-architecture)
3. Relevant sections of [SPEC.md](./SPEC.md)

Preserve the central Tauri bridge, Zustand domain boundaries, shared primitives, and the rule that version comparison is display-only.

### Native/package-manager changes

Read:

1. [Architecture](./architecture.md)
2. [Development Guide](./development-guide.md)
3. [Fixture Provenance](../dev/fixtures/README.md)
4. Relevant [Architecture Decisions](./DECISIONS.md)

Preserve adapter purity, fixture grounding, detected ownership/routing, exact-command preview, lock sets, and process safety.

### IPC or cross-layer changes

Read:

1. [Architecture — IPC Design](./architecture.md#ipc-design)
2. [Development Guide — IPC Contract Changes](./development-guide.md#ipc-contract-changes)
3. `src-tauri/src/ipc.rs`, `src/lib/ipc/types.ts`, and `dev/fixtures/ipc/`

Update Rust models, TypeScript mirrors/guards, command/event wiring, and shared fixtures together.

### Release or packaging changes

Read:

1. [Deployment Guide](./deployment-guide.md)
2. [Contribution Guide — Commit Messages and Releases](./contribution-guide.md#commit-messages-and-releases)
3. `.github/workflows/release-please.yml` and `.github/workflows/release.yml`

Never hand-edit the synchronized version files or release-please-owned changelog/manifest.

### Planning a brownfield feature

Use:

1. This index.
2. [Project Overview](./project-overview.md).
3. [Architecture](./architecture.md).
4. [Component Inventory](./component-inventory.md) for UI work.
5. [Source Tree Analysis](./source-tree-analysis.md) to locate implementation points.
6. `SPEC.md` and `DECISIONS.md` for behavioral authority.

## Known Documentation Caveats

- Production currently registers 20 commands and six events. Some older code comments/tests and design prose still use the pre-updater counts of 17 commands or five events.
- Fixture/spec history contains conflicting statements about machine-specific mas availability/verification; current code and captured fixtures are the implementation evidence until authoritative prose is reconciled.
- `docs/IMPL_PLAN.md` records the original implementation plan and can describe older scaffold state.

---

_Documentation generated by the BMad Method `document-project` workflow._
