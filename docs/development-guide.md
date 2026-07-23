# Pack-Manager Development Guide

**Date:** 2026-07-22

## Prerequisites

Pack-Manager is developed as one Tauri application with two toolchains:

- macOS for the supported desktop target and real package-manager smoke tests.
- Node.js and npm. `.nvmrc` pins Node 24 for local development and CI.
- Stable Rust with Cargo. The crate uses Rust edition 2021; the repository does not include `rust-toolchain.toml`.
- Tauri's macOS build prerequisites, including the Apple command-line build tools.
- `fnox` through `mise` when producing updater-signed local builds.

No database, local service, container, or `.env` file is required.

## Install

From the repository root:

```sh
nvm install
nvm use
npm ci
```

The Tauri CLI is a project dev dependency, so a separate global install is unnecessary. Cargo resolves the native dependencies from `src-tauri/Cargo.lock` when a native command is first run.

## Run Locally

Run the complete desktop application:

```sh
npm run tauri dev
```

Tauri starts the Vite frontend on port 1420 and opens the native window. When `TAURI_DEV_HOST` is set, hot-module reload uses port 1421.

For frontend-only work:

```sh
npm run dev
```

The browser-rendered frontend cannot perform real native operations without Tauri, but the command/event seam can be mocked through `src/test/fakeIpc.ts` in tests.

## Configuration and Secrets

Runtime user preferences are stored by the application at:

```text
~/Library/Application Support/Pack-Manager/settings.json
```

The file is optional. Missing or partial files use defaults; writes use a temporary file and atomic replacement.

Updater-signing credentials are managed by `fnox.toml` with age-encrypted values. Never copy those values into `.env`, source code, documentation, shell history, or CI workflow text.

Run a signed updater build through fnox:

```sh
fnox exec -- npm run tauri build
```

The relevant variables are injected only for the child build process. GitHub Actions receives corresponding values from GitHub Secrets.

## Build

Build and type-check the frontend only:

```sh
npm run build
```

Build the complete app with updater signing:

```sh
fnox exec -- npm run tauri build
```

Build without updater signing for a smoke check:

```sh
npm run tauri build -- --no-sign
```

Bundle output is written beneath:

```text
src-tauri/target/release/bundle/
```

The macOS application/updater archive appears under `macos/`, and the installer appears under `dmg/`.

Because `bundle.createUpdaterArtifacts` and an updater public key are configured, a normal signed build needs the updater private key. Use `--no-sign` only when intentionally testing an unsigned build.

## Test and Verification Commands

### Frontend

```sh
npm test
npx tsc --noEmit
npm run build
npm run test:e2e:typecheck
npm run test:e2e:install
npm run test:e2e
```

The Vitest suite uses jsdom and React Testing Library. Playwright runs browser-visible journeys in Chromium and WebKit against a deterministic in-browser Tauri transport. Use `npm run test:e2e:install:ci` only on Linux CI runners; local machines use `npm run test:e2e:install`. See [`tests/README.md`](../tests/README.md) for its boundaries and failure-evidence guide.

### Native Core

```sh
cd src-tauri
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test --locked
```

The Rust tests cover manager parsers/adapters, detection and routing, scheduling, process lifecycle, cancellation, journaling, transcripts, settings, diagnostics, IPC serialization, and the updater state machine.

### Real-Machine Smoke Tests

```sh
cd src-tauri
cargo test -- --ignored
```

Ignored tests can run installed package-manager commands on the current Mac. They are developer-run only and are not part of CI.

## Test Architecture

The default suites are intended to be deterministic and offline:

- Native process behavior is injected through `CommandRunner`; `FakeRunner` scripts buffered and streaming results.
- Time-sensitive Rust tests use paused Tokio time where possible.
- Frontend Tauri calls are isolated behind `src/lib/ipc/bridge.ts` and replaced by `src/test/fakeIpc.ts`.
- Parser tests use committed captures from `dev/fixtures/`.
- Rust and TypeScript validate the same IPC JSON payloads from `dev/fixtures/ipc/`.

The Playwright suite exercises the real React interface while replacing native Tauri commands and events with a deterministic browser fixture. It does not replace the ignored real-machine Rust smoke tests, which remain responsible for installed package-manager behavior.

## IPC Contract Changes

The Rust contract in `src-tauri/src/ipc.rs` and TypeScript mirror in `src/lib/ipc/types.ts` must change together.

When a representative payload intentionally changes, regenerate the contract fixture from the Rust side:

```sh
cd src-tauri
PM_UPDATE_CONTRACT=1 cargo test ipc_contract
```

Then run both Rust and frontend tests. The TypeScript runtime-guard test reads the same JSON files and should fail if either side drifts.

New native commands should be registered in `src-tauri/src/lib.rs`, wrapped in `src/lib/ipc/client.ts`, and exercised through the frontend fake IPC seam. Keep UI modules away from raw `invoke` and `listen` imports.

## Fixture Workflows

Re-capture offline-safe package-manager inventory fixtures:

```sh
dev/capture-fixtures.sh
```

Include network-dependent outdated probes only when intended:

```sh
PM_CAPTURE_ONLINE=1 dev/capture-fixtures.sh
```

The script date-stamps captures and does not overwrite an existing same-day file. Follow `dev/fixtures/README.md`:

- Real captures record command, date, machine, and parser-relevant facts.
- Synthetic files use the `_synthetic` suffix.
- Every synthetic fixture states its real-data source and retirement condition.

## Common Change Paths

### Add or change a manager adapter

1. Update the adapter in `src-tauri/src/managers/`.
2. Keep output parsing pure in `src-tauri/src/managers/parse/`.
3. Ground parser behavior in a committed fixture.
4. Preserve path-derived ownership and self-update routing.
5. Add plan, parse, recovery, and route tests.

### Add frontend behavior

1. Place screen-specific components in the matching `src/components/<feature>/` folder.
2. Reuse `src/components/primitives/` for shared controls.
3. Put domain state in the closest existing Zustand store; derive cross-store state in `src/store/index.ts`.
4. Use typed IPC wrappers and events for native integration.
5. Add Testing Library coverage and fake IPC assertions.

### Change operation execution

Treat `queue.rs`, `process/runner.rs`, and `ops.rs` as a safety boundary. Preserve:

- Exact-command preview before bulk execution.
- Atomic lock-set acquisition and global concurrency limits.
- Absolute executable paths and a cleared child environment.
- Null stdin and no password/sudo prompts.
- Process-group cancellation and timeout escalation.
- Incrementally flushed, byte-faithful operation transcripts.

## Logs and Local Data

```text
~/Library/Logs/Pack-Manager/pack-manager.log.YYYY-MM-DD
~/Library/Logs/Pack-Manager/operations/
~/Library/Application Support/Pack-Manager/operations.jsonl
~/Library/Application Support/Pack-Manager/settings.json
```

Diagnostics export creates a timestamped ZIP on the Desktop containing a report, recent logs/transcripts, and the operation journal. The exporter rejects symlinked inputs.

## Known Development Caveats

- The project has no configured frontend lint or formatting script. Match the existing TypeScript/TSX style and rely on typecheck, tests, and production build gates.
- Node and Rust versions are standardized in CI but not pinned for local development.
- `tauri.conf.json` currently sets the content security policy to `null`; changes affecting loaded content should account for that existing security posture.
- The source currently registers 20 native commands and six events. A few older comments/tests still refer to 17 commands or five events; treat the registered production surface as authoritative.
