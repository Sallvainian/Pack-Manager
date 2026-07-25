# Pack-Manager Documentation Index

- **Type:** Single-part macOS desktop application
- **Primary languages:** Rust and TypeScript
- **Architecture:** React component/store interface over a command- and event-driven Tauri core
- **Last updated:** 2026-07-24

## Project Overview

Pack-Manager detects local macOS package managers, shows installed and outdated packages, previews exact upgrade commands, and executes safe, observable, cancellable operations. The React webview and Rust native core are internal layers of one versioned and distributed Tauri application.

Start with [project-overview.md](./project-overview.md) for the concise system summary or [architecture.md](./architecture.md) for implementation constraints and data flow.

## Quick Reference

- **Tech stack:** Tauri 2.11.5, Rust 2021, React 19.2.8, TypeScript 7.0.2, Vite 8.1.5, Tailwind CSS 4.3.3, Zustand 5.0.14.
- **Frontend entry:** `src/main.tsx` → `src/App.tsx` → `src/components/shell/AppLayout.tsx`.
- **Native entry:** `src-tauri/src/main.rs` → `pack_manager_lib::run()` in `src-tauri/src/lib.rs`.
- **Integration:** 20 typed Tauri commands and six typed events; no HTTP API.
- **Persistence:** JSON settings, JSONL operation journal, structured logs, and line-flushed transcripts; no database.
- **Tests:** Cargo tests (248 passing, 11 ignored) plus Vitest/Testing Library (23 files, 133 tests) and Playwright browser journeys; machine-dependent Rust tests are ignored by default.
- **Deployment:** Universal macOS app, minimum macOS `15.0`, with release-please, GitHub Actions, signing/notarization, DMG/ZIP, and updater assets.

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
- [Architecture Decisions](./DECISIONS.md) — accepted decisions, rejected alternatives, and rationale. D23a supersedes D23, D25 supersedes D20, D27–D30 define the target update experience, and D31–D33 recalibrate scope.
- [Release Checklist](./RELEASE-CHECKLIST.md) — the ~15-minute manual pass and the two automated release checks that replaced the retired readiness gate.
- [Implementation Plan](./IMPL_PLAN.md) — dependency-ordered implementation history and verification gates.
- [Fixture Provenance](../dev/fixtures/README.md) — capture provenance, parser facts, synthetic-fixture policy, and backlog.
- [Browser Test Guide](../tests/README.md) — Playwright suite boundaries, the in-browser Tauri double, and failure-evidence handling.

### Workflow and Agent Rules

- [AGENTS.md](../AGENTS.md) — workspace contribution and agent workflow rules.
- [CLAUDE.md](../CLAUDE.md) — release/versioning rules for agents working in this repository.
- [Project Context](../_bmad-output/project-context.md) — condensed implementation rules for AI agents, with D27–D30 split into Current vs Target.

### Planning Artifacts

- [Epics](../_bmad-output/planning-artifacts/epics.md) — epic and story definitions.
- [Architecture Spine](../_bmad-output/planning-artifacts/architecture/architecture-Pack-Manager-2026-07-23/ARCHITECTURE-SPINE.md) — architecture decisions driving the UX-PB queue.
- [Story Triage 2026-07-24](../_bmad-output/planning-artifacts/story-triage-2026-07-24.md) — per-story keep/merge/retire verdicts for the 37 Epic 1–6 entries.
- [Sprint Status](../_bmad-output/implementation-artifacts/sprint-status.yaml) — the 28 UX-PB stories that form the primary build queue.
- [UX Design](../_bmad-output/planning-artifacts/ux-designs/ux-Pack-Manager-2026-07-23/DESIGN.md) and [Experience](../_bmad-output/planning-artifacts/ux-designs/ux-Pack-Manager-2026-07-23/EXPERIENCE.md) — target upgrade experience.
- [Retired Artifacts](../_bmad-output/archive/2026-07-24-scope-recalibration/README.md) — what the scope recalibration archived and why.

### Automation Definitions

- [Continuous Integration](../.github/workflows/ci.yml) — Rust checks, web checks, and main-branch bundle smoke.
- [Browser Test CI](../.github/workflows/test.yml) — Playwright validation, two shards covering Chromium and WebKit, burn-in, and merged reports.
- [CI Guide](./ci.md) — triggers, stages, artifacts, local parity, and troubleshooting.
- [CI Secrets Checklist](./ci-secrets-checklist.md) — credential requirements and safe configuration boundaries.
- [Claude Issue/PR Agent](../.github/workflows/claude.yml) — mention-triggered repository assistance.
- [Automated PR Review](../.github/workflows/claude-code-review.yml) — human-authored pull-request review workflow.
- [Release Please](../.github/workflows/release-please.yml) — release PR, tag, GitHub Release, and build orchestration.
- [Release Build](../.github/workflows/release.yml) — universal build, signing, notarization, verification, and asset publication.

## Getting Started

### Prerequisites

- macOS and Apple command-line build tools. The shipped bundle declares a `15.0` floor; CI builds on `macos-14`.
- Node.js/npm (Node 24 from `.nvmrc`).
- Stable Rust/Cargo.
- fnox through mise for a signed updater build.

### Install and Run

```sh
nvm install && nvm use
npm ci
npm run tauri dev
```

### Verify

```sh
npm test
npx tsc --noEmit
npm run build
npm run test:e2e:typecheck
npm run test:e2e

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
- The D27–D30 upgrade experience is **decided but not implemented**. `planAttemptId`, a `Verifying` status, `InteractionRequired`, the separate confirmation dialog, and per-manager removable self-updates do not exist in the current source. Documents that describe them mark those sections as target state; do not search the code for those symbols.
- `sprint-status.yaml` marks all 37 Epic 1–6 stories `backlog`, but D33 declares them UNSCHEDULED, not queued. Check `story-triage-2026-07-24.md` before scheduling one.

---

_Documentation generated by the BMad Method `document-project` workflow._
