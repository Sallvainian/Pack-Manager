# Pack-Manager Source Tree Analysis

**Date:** 2026-07-22

## Overview

Pack-Manager is a single Tauri desktop application. The root npm project builds the React webview in `src/`; the embedded Rust crate in `src-tauri/` owns native execution, persistence, and packaging. They are versioned and released together and communicate only through typed local Tauri commands and events.

Generated dependencies and build output (`node_modules/`, `dist/`, `src-tauri/target/`, and `src-tauri/gen/`) are not source and were excluded from the deep scan.

## Annotated Directory Structure

```text
Pack-Manager/
├── src/                              # React/TypeScript webview interface
│   ├── main.tsx                      # Browser entry: mounts React StrictMode
│   ├── App.tsx                       # Event subscription, hydration, launch refresh
│   ├── components/
│   │   ├── shell/                    # Window layout, navigation, status, updates, toasts
│   │   ├── dashboard/                # Manager overview cards and ownership chips
│   │   ├── manager/                  # Manager pane, package table, health and upgrade UI
│   │   ├── activity/                 # Operations drawer, live logs, operation effects
│   │   ├── dialogs/                  # Plan, stall, and quit/update confirmation dialogs
│   │   ├── history/                  # Prior-operation browser
│   │   ├── settings/                 # Preferences, diagnostics, environment, updater
│   │   └── primitives/               # Reusable buttons, chips, states, tooltip, spinner
│   ├── store/                        # Five Zustand stores and derived selectors
│   ├── lib/
│   │   ├── ipc/
│   │   │   ├── bridge.ts             # Only frontend import of @tauri-apps/api
│   │   │   ├── client.ts             # Typed wrappers for 20 native commands
│   │   │   ├── events.ts             # Six native-event subscriptions and store updates
│   │   │   └── types.ts              # TypeScript wire types and runtime guards
│   │   ├── errors.ts                 # Stable user-facing error copy
│   │   └── versionDelta.ts           # Display-only version segment comparison
│   ├── hooks/useKeyboard.ts          # Global keyboard command map
│   ├── styles/theme.css              # Tailwind v4 import and dark design tokens
│   ├── test/                          # Fake IPC, fixtures, setup, browser polyfills
│   └── __tests__/                     # Cross-feature component/store integration tests
├── src-tauri/                         # Native Rust/Tauri crate and app bundle
│   ├── src/
│   │   ├── main.rs                    # Native binary entry: pack_manager_lib::run()
│   │   ├── lib.rs                     # Composition root, plugins, menu, startup, shutdown
│   │   ├── commands.rs                # Thin Tauri command handlers
│   │   ├── ipc.rs                     # Canonical Serde wire contract
│   │   ├── state.rs                   # Shared application state graph and startup
│   │   ├── settings.rs                # Defaults and atomic settings persistence
│   │   ├── detect.rs                  # Manager discovery, ownership, update routing
│   │   ├── managers/
│   │   │   ├── mod.rs                 # ManagerAdapter contract and shared plan types
│   │   │   ├── brew.rs                # Homebrew adapter
│   │   │   ├── mise.rs                # mise adapter
│   │   │   ├── npm.rs                 # global npm adapter
│   │   │   ├── uv.rs                  # uv tool adapter
│   │   │   ├── rustup.rs              # rustup adapter
│   │   │   ├── mas.rs                 # Mac App Store adapter
│   │   │   └── parse/                 # Pure fixture-grounded parsers for all managers
│   │   ├── queue.rs                   # Plan builder, lock scheduler, execution lifecycle
│   │   ├── ops.rs                     # Operation model and byte-faithful transcripts
│   │   ├── process/
│   │   │   ├── runner.rs              # Sole real child-process executor
│   │   │   └── fake.rs                # Scripted process test double
│   │   ├── registry.rs                # In-memory manager snapshots and joins
│   │   ├── events.rs                  # Typed event sinks and output batching
│   │   ├── journal.rs                 # Crash journal and prior-session history
│   │   ├── logging.rs                 # Structured logs, filter reload, retention
│   │   ├── diagnostics.rs             # Symlink-safe diagnostics ZIP generation
│   │   ├── app_update.rs              # Pack-Manager updater state machine
│   │   ├── paths.rs                   # Finder-safe PATH and clean child environment
│   │   └── error.rs                   # Internal and stable IPC error taxonomy
│   ├── tests/live_smoke.rs            # Ignored real-machine smoke tests
│   ├── capabilities/default.json      # Main-window Tauri permissions
│   ├── icons/                         # Generated platform packaging icons
│   ├── tauri.conf.json                # Window, build, bundle, updater configuration
│   ├── Cargo.toml / Cargo.lock         # Native dependencies and locked versions
│   └── build.rs                        # Tauri build integration
├── dev/
│   ├── fixtures/                      # Captured/synthetic parser and IPC evidence
│   │   ├── ipc/                       # Shared Rust/TypeScript wire-contract payloads
│   │   └── README.md                  # Fixture provenance and retirement rules
│   ├── capture-fixtures.sh            # Non-clobbering fixture recapture tool
│   └── icon/                          # Reproducible icon source and generator
├── docs/
│   ├── SPEC.md                        # Authoritative product/technical contract
│   ├── DECISIONS.md                   # Architecture decisions and rejected alternatives
│   ├── IMPL_PLAN.md                   # Historical dependency-ordered implementation plan
│   └── *.md                           # Generated brownfield documentation
├── .github/workflows/
│   ├── ci.yml                         # Rust/web checks and unsigned bundle smoke
│   ├── release-please.yml             # Version PR, tag, release orchestration
│   ├── release.yml                    # Universal signed/notarized macOS artifacts
│   ├── claude.yml                     # Mention-triggered GitHub agent workflow
│   └── claude-code-review.yml         # Human-authored PR review workflow
├── .agents/ / .claude/ / .codex/     # Local agent configuration and installed skills
├── _bmad/ / _bmad-output/             # BMad workflow installation and generated artifacts
├── index.html                         # Vite HTML entry
├── package.json / package-lock.json   # Frontend scripts and locked dependencies
├── vite.config.ts                     # Tauri-aware Vite server/build configuration
├── vitest.config.ts                   # jsdom frontend test configuration
├── tsconfig*.json                     # Strict TypeScript configuration
├── fnox.toml                          # Age-encrypted updater-signing secret references
├── release-please-config.json         # Automated release/version policy
├── README.md                          # Project overview and operator/developer commands
├── AGENTS.md / CLAUDE.md              # Workspace contribution and agent rules
├── node_modules/                      # Generated npm dependencies; excluded
├── dist/                              # Generated frontend build; excluded
└── src-tauri/target/                  # Generated Rust and bundle output; excluded
```

## Critical Directories

### `src/`

The complete webview interface and its session state. Feature folders own user-facing behavior; native calls are deliberately centralized in `src/lib/ipc/`.

**Entry points:** `src/main.tsx`, `src/App.tsx`, `src/components/shell/AppLayout.tsx`

**Integration:** Typed commands flow through `client.ts`; native events flow through `events.ts` into Zustand stores.

### `src-tauri/src/`

The native application core. It detects package managers, builds trusted upgrade plans, schedules lock-safe operations, executes absolute binaries, streams output, persists settings/history, creates diagnostics, and updates the app.

**Entry points:** `src-tauri/src/main.rs`, `src-tauri/src/lib.rs`

**Integration:** `commands.rs` exposes the local command boundary; `events.rs` publishes state/output changes.

### `src-tauri/src/managers/`

Six adapters implement manager-specific inventory, outdated, update, self-update, parsing, error recovery, and exit classification behavior behind `ManagerAdapter`. The `parse/` subtree remains pure and fixture-grounded.

### `src-tauri/src/process/` and `queue.rs`

The execution trust boundary. `queue.rs` owns plan validation, lock-set scheduling, cancellation, and post-success refresh. `runner.rs` is the only real process launcher and enforces absolute executables, cleared environments, null stdin, timeouts, process groups, and termination escalation.

### `dev/fixtures/`

Committed evidence for parsers and IPC contracts. Fixture provenance distinguishes captured from synthetic data and documents retirement conditions.

### `.github/workflows/`

Verification and delivery automation. CI tests Rust and web layers separately; the release workflow combines them into one universal macOS application and distribution artifacts.

## Entry and Startup Flow

```text
index.html
  → src/main.tsx
    → src/App.tsx
      → subscribeEvents()
      → getState() / getAppUpdateState()
      → Zustand stores
      → AppLayout

src-tauri/src/main.rs
  → pack_manager_lib::run()
    → load settings
    → initialize logging and retention
    → construct AppState and Tauri plugins/menu
    → expose 20 commands
    → asynchronously probe PATH and detect managers
    → emit detection/snapshot/operation/update events
```

Subscribing before frontend hydration prevents the UI from missing the asynchronous native detection result.

## File Organization Patterns

- React features are grouped by screen or concern; reusable controls live in `components/primitives/`.
- Zustand stores are split by domain, while `store/index.ts` owns cross-store derived selectors.
- The frontend's native boundary is centralized and typed; components do not import Tauri APIs directly.
- Rust Tauri commands remain thin and delegate to focused modules.
- Package-manager behavior is adapter-based, and parsers are separated from command planning/execution.
- Tests are colocated for focused modules and collected in `src/__tests__/` for cross-feature behavior.
- Cross-language payload fixtures live outside either implementation so both sides validate the same bytes.

## Key File Types

| Pattern | Purpose | Examples |
| --- | --- | --- |
| `src/**/*.tsx` | React views and components | `AppLayout.tsx`, `ManagerPane.tsx` |
| `src/store/*.ts` | Zustand state domains and selectors | `operations.ts`, `packages.ts` |
| `src/lib/ipc/*.ts` | Tauri command/event contract | `client.ts`, `types.ts` |
| `src/**/*.test.ts(x)` | Vitest/Testing Library coverage | `bootstrap.test.tsx`, `planSheet.test.tsx` |
| `src-tauri/src/*.rs` | Native orchestration and services | `queue.rs`, `settings.rs` |
| `src-tauri/src/managers/*.rs` | Manager adapters | `brew.rs`, `uv.rs` |
| `src-tauri/src/managers/parse/*.rs` | Pure output parsers | `mise.rs`, `npm.rs` |
| `dev/fixtures/**/*` | Captured and contract test inputs | `ipc/app-state.json` |
| `.github/workflows/*.yml` | CI and release automation | `ci.yml`, `release.yml` |

## Asset Locations

| Asset type | Location | Inventory |
| --- | --- | --- |
| Platform app icons | `src-tauri/icons/` | 17 files: 15 PNG, one ICNS, one ICO; about 304 KiB. |
| Icon source and generator | `dev/icon/` | One PNG source and one Python generator; about 54 KiB. |

The React interface has no `public/` directory, web-font assets, or imported images.

## Configuration Files

| Path | Responsibility |
| --- | --- |
| `package.json` | npm scripts and frontend dependencies. |
| `src-tauri/Cargo.toml` | Native crate, features, and Rust dependencies. |
| `src-tauri/tauri.conf.json` | App identity, window, frontend build, bundle, updater, and icon settings. |
| `src-tauri/capabilities/default.json` | Main-window Tauri permission set. |
| `vite.config.ts` | React/Tailwind plugins and fixed Tauri dev/HMR ports. |
| `vitest.config.ts` | React-enabled jsdom test environment. |
| `tsconfig.json` | Strict ES2020 no-emit TypeScript checks. |
| `fnox.toml` | Encrypted local updater-signing secret references. |
| `release-please-config.json` | Conventional-commit release automation. |

## Development Notes

- Do not treat `src/` and `src-tauri/` as separately deployable projects; every release bundles both.
- Keep Tauri API access behind `src/lib/ipc/bridge.ts` so the frontend remains testable.
- Keep arbitrary shell strings out of execution paths; the native layer launches derived absolute commands.
- Update Rust types, TypeScript mirrors/guards, and IPC fixtures together.
- Do not hand-edit versions or release-owned changelog/manifest files.
- Generated dependency, build, skill-cache, and workflow-output folders should not be analyzed as product source.
