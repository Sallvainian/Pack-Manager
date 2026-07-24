# Pack-Manager — IMPL_PLAN.md

## 0. Ground rules

- Authority: SPEC.md is the contract. Do not invent behavior; if the spec is silent, follow DECISIONS.md; if both are silent, choose the smallest behavior and note it.
- **Strict file ownership**: during any parallel wave, no two in-flight units write the same file. U1 runs alone and creates the full skeleton (compiling stubs); afterwards every file has exactly one owning unit per the tables below. A later unit may complete a stub created earlier (sequential handoff), never concurrently.
- Determinism bar per unit: the unit's own tests pass offline via `cargo test` / `npm test` on a clean checkout before the unit is done.
- Model marks: **fable** = tricky (concurrency, routing, contract, integration); **opus** = standard.
- Existing state (verified): repo at `/Users/sallvain/Projects/Pack-Manager`, commit d857cf5 scaffolded Tauri 2 + React 19 + Vite 7 + Tailwind v4 + Vitest 4; `src-tauri/Cargo.toml` has only tauri/serde/serde_json; `tauri.conf.json` exists (identifier `com.sallvain.pack-manager`, 1200×800/min 900×600, no overlay titlebar yet, default Tauri icons); real fixtures in `dev/fixtures/`. Units EXTEND this scaffold.

### 2026-07-24 superseding implementation prerequisite

This plan is historical for the current brownfield implementation. Before
affected U7/U8 behavior or readiness evidence is treated as target-complete,
implement the approved Product Behavior Prerequisite UX-PB.1..UX-PB.5:

1. persistent canonical Package/Manager Upgrade Plan membership;
2. distinct one-use `planId` and durable `planAttemptId`;
3. one active attempt with plan Activity, verification, Results, and
   attempt-wide cancellation;
4. one History row per attempt with replay and linked Retry; and
5. separate confirmation with `skipUpgradePlanConfirmation: false`, inactive
   legacy `autoOpenDrawer`, trusted prompt classification, and finalized
   accessibility/high-zoom behavior.

The original units below explain how the current system was built. Their
immediate row execution, direct self-update, modal plan sheet, Activity drawer,
and Operation-History wording is superseded by Decisions D27-D30 and AD-16.
No source change is claimed by this documentation amendment.

## 1. Dependency graph

```
Prerequisite: UX-PB.1 → UX-PB.2 → UX-PB.3/UX-PB.5 → UX-PB.4
              (before affected evidence or target-complete claims)
Wave 0:  U1 (alone)
Wave 1:  U2 ∥ U3 ∥ U6
Wave 2:  U4 ∥ U7          (U4 needs U2+U3; U7 needs U6)
Wave 3:  U5 ∥ U8          (U5 needs U2+U4; U8 needs U6)
Wave 4:  U9 (alone)       (needs everything)
```

---

## U1 — Contract, foundation, icon (fable, Wave 0, runs alone)

**Scope:** Everything shared. Add Rust deps (SPEC §5.1 list) and frontend deps (`zustand`, `@tanstack/react-virtual`). Update `tauri.conf.json`: `titleBarStyle: "Overlay"`, `hiddenTitle: true`, register `tauri-plugin-opener`. Write ALL IPC payload types in `src-tauri/src/ipc.rs` (serde camelCase) and the 1:1 mirror `src/lib/ipc/types.ts` (SPEC §5.9). Write `error.rs` (full taxonomy + IpcError serialization, SPEC §5.10). Write `events.rs` trait half: `EventSink` trait + event payload structs + `VecSink` (batching emitter left as stub for U2). Write `managers/mod.rs`: `ManagerId`, `ManagerAdapter` trait, `PlannedCommand`, `Timeout`, `ExitClass`, `SelfUpdateRoute` (no adapter impls). Write `process/mod.rs` + `process/runner.rs` trait half: `CommandRunner` trait, `CommandSpec`, `CommandOutput`, `LogLine` (no RealRunner). Write `settings.rs` complete (struct, defaults, load/save to Application Support, patch merge). Create compiling stubs: `state.rs`, `commands.rs` (all 17 commands registered, returning defaults/`Internal` placeholders), `lib.rs` builder wiring, empty module files for everything else in the SPEC layout (paths, detect, registry, queue, ops, journal, diagnostics, logging, process/fake, managers/_, managers/parse/_). Frontend: `src/styles/theme.css` with the full token block (SPEC §4.1); fold/remove `src/index.css`; keep App.tsx minimal. Contract fixtures: `dev/fixtures/ipc/*.json` + Rust byte-equality test + Vitest type-guard test (SPEC §7.4). Icon: `dev/icon/generate_icon.py` (Pillow, SPEC §4.12), run it, run `npx tauri icon dev/icon/icon-1024.png`, commit regenerated `src-tauri/icons/`. README skeleton (name, stack, dev commands, log locations).
**Owns:** `package.json`, `package-lock.json`, `src-tauri/Cargo.toml`, `src-tauri/Cargo.lock`, `src-tauri/tauri.conf.json`, `src-tauri/capabilities/default.json`, all `src-tauri/src/**` (as creator; later owners listed per unit), `src/lib/ipc/types.ts`, `src/styles/theme.css`, `src/index.css` (removal), `dev/icon/**`, `src-tauri/icons/**`, `dev/fixtures/ipc/**`, `src/lib/ipc/types.test.ts`, `README.md`, `vitest.config.ts` (if setup-file registration needed now).
**Acceptance:** `cargo check` + `cargo test` (contract test green) + `npm test` + `tsc --noEmit` green; `npm run tauri dev` opens a dark window with overlay traffic lights; `src-tauri/icons/icon.icns` regenerated from the script (not the Tauri default); both contract tests fail if either side of a type is edited alone (verified by a deliberate temporary mutation during development).
**Gate G0:** all of the above on a clean checkout.

---

## U2 — Environment & process execution core (fable, Wave 1)

**Scope:** `paths.rs`: static list + sentinel `-lc` login-shell probe (5s timeout, 64KiB cap, last-resort StaticFallback), merge/dedupe, `ToolEnv`, child-env construction (SPEC §5.2), `which_in` resolution helpers. `process/runner.rs`: `RealRunner` (process_group(0), stdin null, line readers with `\r` split + ANSI strip, 512KiB caps, stall watchdog, absolute timeout, SIGTERM→5s→SIGKILL via nix killpg). `process/fake.rs`: `FakeRunner` builder (`.on(...).fixture(...)`, `.on_streaming(...).emits(...).gate(...)`, call recording, panic on unmatched). `logging.rs`: tracing init (JSON daily rolling + dev stderr layer), EnvFilter precedence, reload handle, startup prune (app logs >14d; transcripts >90d/200 files). `events.rs` batching emitter (50ms/64-line/8KiB flush) implemented against `EventSink`.
**Owns:** `src-tauri/src/paths.rs`, `src-tauri/src/process/runner.rs`, `src-tauri/src/process/fake.rs`, `src-tauri/src/logging.rs`, `src-tauri/src/events.rs`.
**Acceptance:** ToolEnv tests (sentinel noise, fallback ordering shims-first, merge dedupe) green; batching test green under paused time; FakeRunner self-tests green; `#[ignore]` real-process tests against `/bin/sh` scripts (spawn/kill/stall) pass when run explicitly on this machine; no `sleep` anywhere in default tests.

## U3 — Parsers & fixture governance (opus, Wave 1)

**Scope:** FIRST, capture missing offline-safe fixtures live on this machine via new `dev/capture-fixtures.sh` (date-stamped): `brew list --versions`, `brew list --cask --versions`, `mise ls --json` (confirm exact flags here — this capture decides the refresh_plan flag), `npm ls -g --depth=0 --json`, `rustup toolchain list`, re-capture `uv tool list`. These commands are local/offline. Commit captures. Write `dev/fixtures/README.md` documenting provenance of EVERY fixture (captured: date+machine; synthetic: `*_synthetic` suffix + source + retirement condition) and the capture backlog (populated mise/npm outdated JSON — machine had 7/5 outdated on 2026-07-21, so recapture opportunity recurs; populated uv `--outdated`; populated brew cask JSON; all mas). Create synthetic fixtures with values copied verbatim from the real text captures. THEN implement all pure parsers per SPEC §5.5 with the full §7.1 test list, including inventory parsers against the new real captures and adapter-level merge helpers (pure functions in `parse/mod.rs`).
**Owns:** `src-tauri/src/managers/parse/**`, `dev/fixtures/**` (except `dev/fixtures/ipc/` owned by U1), `dev/capture-fixtures.sh`.
**Acceptance:** every §7.1 test green offline; counts match the verified fixture facts (7 mise rows/6 outdated; 5 npm rows/4 after hoist; 12 uv tools; both rustup spacings; brew junk line skipped; greedy set-difference; 0-byte uv outdated = clean); no parser panics on any fixture fed to the wrong parser (ParseFailed instead).

## U6 — Frontend foundation & shell (opus, Wave 1)

**Scope:** `lib/ipc/bridge.ts` (sole importer of `@tauri-apps/api`), `client.ts` (typed wrappers for all 17 commands), `events.ts` (single subscription → store dispatch), `lib/errors.ts` (per-code user copy incl. brew_lock_busy and offline/timeout strings), stores (`managers`, `packages`, `operations` with 5000-line ring buffer, `ui`), test harness (`test/setup.ts`, `test/fakeIpc.ts`, `test/fixtures.ts` hand-derived from real fixture values), shell components (`AppLayout`, `Sidebar`, `SidebarManagerItem`, `StatusBar`, `ToastHost`), dashboard (`DashboardView`, `ManagerCard`, `ManagedByChip` with evidence tooltip + phase labels), `SettingsView` (settings form + Environment Report + Open Logs/Export/Re-detect), primitives (`Button`, `Checkbox`, `Chip`, `Tooltip`, `Spinner`, `EmptyState`, `ErrorState`, `CopyableCommand`, `SkeletonRows`), `App.tsx`/`main.tsx` wiring, and compiling placeholder stubs for `ManagerPane`, `ActivityDrawer`, `HistoryView`, `DialogHost` (replaced by U7/U8 in later waves — sequential handoff, no concurrency).
**Owns:** `src/lib/ipc/{bridge,client,events}.ts`, `src/lib/errors.ts`, `src/store/**`, `src/test/**`, `src/components/{shell,dashboard,settings,primitives}/**`, `src/App.tsx`, `src/main.tsx`, stub files for `src/components/{manager/ManagerPane.tsx, activity/ActivityDrawer.tsx, history/HistoryView.tsx, dialogs/DialogHost.tsx}`, `src/__tests__/smoke.test.tsx` (update/remove).
**Acceptance:** §7.5 tests for dashboard/sidebar/settings/error-isolation/absent-mas green with fakeIpc; store unit tests green; `tsc` green; app renders full shell against fake data in dev.

---

## U4 — Adapters, detection, routing, registry (fable, Wave 2; after U2+U3)

**Scope:** `detect.rs`: pure `classify_managed_by` (raw-path mise rule BEFORE canonicalize, evidence strings), detection orchestration (which_in + fixed candidates + `--version` probe 10s via CommandRunner), route precedence (in-band override → delegated-if-detected → native → unavailable). Six adapter impls (`brew.rs` … `mas.rs`): refresh/recovery/upgrade plans, classify_exit (npm exit-1 rule, brew lock stderr → BrewLockBusy), snapshot assembly using U3 parsers, phase labels. `registry.rs`: snapshot store + cross-manager self-version join + selfPackage extraction.
**Owns:** `src-tauri/src/detect.rs`, `src-tauri/src/registry.rs`, `src-tauri/src/managers/{brew,mise,npm,uv,rustup,mas}.rs`.
**Acceptance:** §7.2 tests green including `classify_mise_shim_path_is_mise_managed_without_canonicalizing` and the counterfactual standalone-uv case; adapter tests with FakeRunner (npm exit-1 both ways, brew two-phase sequence + phase labels via VecSink, uv warning → HealthIssue with report Ok, timeout → error snapshot, mas Absent never calls runner, recovery fallback on bad mise JSON); cross-join tests green.

## U7 — Frontend package UX (opus, Wave 2; after U6)

**Scope:** `ManagerPane` (replaces stub), `SelfUpdateCard` (routes, queued-behind-executor, npm mise-reset note), `HealthBanner` (+ Run fix wiring), `PackageToolbar`, `PackageTable` (virtualized) + `PackageRow`, `VersionDelta` + pure `lib/versionDelta.ts`, `StatusBadge`, `SelectionToolbar` (selection semantics: tri-state, shift-range, filter-aware select-all, pinned/greedy exclusion), `dialogs/UpgradePlanSheet.tsx` (command previews, toggles, excluded reasons, warnings) and plan-flow wiring (build → sheet → execute; row button = immediate single-package plan).
**Owns:** `src/components/manager/**`, `src/components/dialogs/UpgradePlanSheet.tsx`, `src/lib/versionDelta.ts`.
**Acceptance:** §7.5 selection/plan-sheet/delta/self-update/health tests green; `version_delta` unit table green (patch/major/plain/unknown cases); select-all provably never includes pinned or greedy rows.

---

## U5 — Scheduler, operations, command handlers (fable, Wave 3; after U2+U4)

**Scope:** `queue.rs`: lock-set scheduler (single task, atomic lock acquisition, FIFO + skip-ahead + 120s aging guard, Semaphore(4), coalescing, lock rules incl. routed dual-lock and npm/uv→Mise guards), spec execution with classify_exit/recovery, auto re-refresh, cancellation plumbing. `ops.rs`: Operation model + transcript writer (header/stream/footer, line-flushed). `journal.rs`: operations.jsonl append/flush/compact/interrupted-scan (never signal recorded pgids). `diagnostics.rs`: zip export per SPEC F9. Plan builder (`build_upgrade_plan` pure fn incl. rust-dedup + exclusions + warnings) — lives in `queue.rs` or `ops.rs`, owner's choice, exported via commands. Complete `commands.rs` (all 17 handlers), `state.rs` (AppState wiring), `lib.rs` (builder, managed state, startup sequence §5.12, quit-guard kill hook).
**Owns:** `src-tauri/src/{queue.rs, ops.rs, journal.rs, diagnostics.rs, commands.rs, state.rs, lib.rs, main.rs}`.
**Acceptance:** full §7.3 scheduler suite green under paused time; plan-builder tests green (exact argv, rust-dedup, exclusions); journal round-trip + interrupted tests green; transcript golden-format test green; `cargo test` for the whole crate green.

## U8 — Frontend operations UX & system views (opus, Wave 3; after U6)

**Scope:** `ActivityDrawer` (replaces stub) + `OperationList`/`OperationRow` + `LiveLogView` (virtualized, batch append, pin/unpin + jump chip, `\r` collapse, 5000-line cap banner, command header + Reveal), `HistoryView` (replaces stub; filters, detail, Interrupted rendering, Export diagnostics), `dialogs/DialogHost.tsx` (replaces stub) + `StalledOperationDialog` + `QuitGuardDialog`, `hooks/useKeyboard.ts` (full map §4.11 dispatching store actions), toast completion flows.
**Owns:** `src/components/activity/**`, `src/components/history/**`, `src/components/dialogs/{DialogHost.tsx, StalledOperationDialog.tsx, QuitGuardDialog.tsx}`, `src/hooks/useKeyboard.ts`.
**Acceptance:** §7.5 streaming/stall/cancel/history/keyboard tests green with fake timers; log view single-render-per-batch verified; drawer auto-open honors setting.

---

## U9 — Integration, live smoke, packaging gate, README (fable, Wave 4, runs alone)

**Scope:** End-to-end wiring fixes across the whole tree (sole writer at this point). `src-tauri/tests/live_smoke.rs` (`#[ignore]`: real detection classifications, real brew JSON round-trip). `.github/workflows/ci.yml` per SPEC §7.6. Complete README (features, screenshots optional, dev/test/build commands, log locations, diagnostics, fixture-capture how-to, known limitations: mas unverified, notarization out of scope). Run the manual smoke checklist and fix what it surfaces.
**Owns:** everything (exclusive final pass); new files `src-tauri/tests/live_smoke.rs`, `.github/workflows/ci.yml`; `README.md` completion.
**Final gate (all must pass):**

1. Clean checkout → `cargo fmt --check` && `cargo clippy --all-targets -- -D warnings` && `cargo test --locked` — green, offline.
2. `npm ci && tsc --noEmit && npm test && npm run build` — green, offline.
3. `cargo test -- --ignored` live smoke on this machine — green.
4. `npm run tauri build` → `src-tauri/target/release/bundle/macos/Pack-Manager.app` exists with the generated icon.
5. Launch the built .app via `open` (Finder-equivalent, NOT `tauri dev`): detection finds brew/mise/npm/uv/rustup (mas Not-installed), routing chips show via-brew/via-mise with evidence tooltips, Refresh All populates all managers with per-manager isolation, upgrade one real package end-to-end with live streaming + transcript on disk, cancel a running op, plan sheet previews match spawned commands (verify via transcript header), Export diagnostics produces a zip, logs exist at the SPEC §6.1 paths.
6. Kill the app mid-refresh; relaunch; the op shows Interrupted; no stray signals sent.

## 2. Verification gates summary

| Gate | After  | Bar                                                                                                                     |
| ---- | ------ | ----------------------------------------------------------------------------------------------------------------------- |
| G0   | U1     | cargo check/test + npm test + tsc + tauri dev boots + contract tests + icon regenerated                                 |
| G1   | Wave 1 | U2/U3 cargo tests + U6 vitest green; combined `cargo test` green                                                        |
| G2   | Wave 2 | U4 cargo tests + U7 vitest green; whole-repo `cargo test` + `npm test` green                                            |
| G3   | Wave 3 | Full `cargo test` + `npm test` + `tsc` green; `tauri dev` manual sanity (refresh, select, upgrade against real machine) |
| G4   | U9     | Final gate 1–6 above                                                                                                    |
