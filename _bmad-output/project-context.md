---
project_name: 'Pack-Manager'
user_name: 'Sallvain'
date: '2026-07-22'
sections_completed:
  - technology_stack
  - language_specific_rules
  - framework_specific_rules
  - testing_rules
  - code_quality_style_rules
  - development_workflow_rules
  - critical_dont_miss_rules
existing_patterns_found: 7
status: 'complete'
rule_count: 50
optimized_for_llm: true
---

# Project Context for AI Agents

_This file contains critical rules and patterns that AI agents must follow when implementing code in this project. Focus on unobvious details that agents might otherwise miss._

---

## Technology Stack & Versions

- One macOS Tauri application; the React interface and Rust core are internal layers of the same bundle. There is no HTTP API or database.
- Rust 2021, Tauri crate 2.11.5, Tokio 1.53.1, Tauri opener 2.5.4, and updater 2.10.1.
- Tauri JavaScript API 2.11.1 and CLI 2.11.4.
- React and React DOM 19.2.8, TypeScript 5.8.3, and Vite 7.3.6.
- Tailwind CSS 4.3.3, Zustand 5.0.14, and TanStack React Virtual 3.14.7.
- Vitest 4.1.10, React Testing Library 16.3.2, jsdom 29.1.1, and Cargo tests.
- CI uses Node 24 and stable Rust; local toolchain versions are not pinned.

## Critical Implementation Rules

### Language-Specific Rules

- Keep TypeScript strict and no-emit. Unused locals/parameters and switch fallthrough are errors. Use relative imports and `import type` for type-only imports; no path aliases are configured.
- Rust IPC struct fields and ordinary multiword enum variants use lowerCamelCase; single-word enum values naturally serialize lowercase, while stable `ErrorCode` values use snake_case. Preserve each existing wire spelling. `Option<T>` with `skip_serializing_if` maps to an optional TypeScript field, a plain serialized option maps to `T | null`, and `#[serde(skip)]` data is backend-only with no TypeScript mirror.
- Define TypeScript wire enums as `as const` arrays with derived unions and runtime guards. Preserve stable `IpcError` codes and context fields.
- Route caught `unknown` frontend errors through `describeError`; do not render raw Tauri rejection objects.
- Package IDs are `${kind}:${name}` and must split on the first colon only. Preserve manager-supplied version strings verbatim and represent unknown versions as `null`.
- Do not hold `std::sync` guards across `.await`, and do not use `unwrap()` on fallible user-reachable paths. Prefer a revision/recheck handoff across async boundaries; hold an async guard across `.await` only when a documented invariant truly requires it.

### Framework-Specific Rules

- `src/lib/ipc/bridge.ts` is the only frontend module allowed to import Tauri APIs. Components use typed wrappers from `client.ts`; `events.ts` owns native event subscriptions.
- Keep payloads for argument-taking commands wrapped as `{ args: ... }`; no-argument commands omit the payload. Any IPC change must update Rust models, TypeScript mirrors/guards, and `dev/fixtures/ipc/*.json` together.
- Subscribe to native events before `get_state` hydration so startup detection cannot be missed. If a real detection event arrives during hydration, it takes priority over the placeholder report returned by `get_state`. Clean up every listener; preserve the `Promise.allSettled` rollback behavior for partial subscription failure.
- Use narrow Zustand selectors in components and the specific hook's static accessor (for example, `useUiStore.getState()`) outside React. Replace objects/Sets immutably; put cross-store derived state in `src/store/index.ts` instead of duplicating it.
- Keep feature components under `src/components/<feature>/`, shared controls under `components/primitives/`, and navigation in the existing discriminated `ActiveView` state rather than adding a router.
- Hooks must precede early returns. Effects must clean up subscriptions/timers and declare correct dependencies. Guard async dialog continuations with mounted/latest-request refs so dismissal cannot trigger stale rebuilds or state writes.
- Design tokens live in `src/styles/theme.css`; the product is dark-only. Do not add hardcoded hex colors elsewhere. Color states need text or icon equivalents, status chips must not wrap, and reduced-motion behavior must remain intact.
- Preserve the package-table virtualization threshold and the 5,000-line per-operation live-log cap unless an explicit design decision changes them.

### Testing Rules

- Default suites must be deterministic and offline: processes use `CommandRunner`/`FakeRunner`, events use `EventSink`/`VecSink`, frontend IPC uses `bridge.ts`/`fakeIpc`, and time uses paused Tokio or fake timers. Do not add real processes, network calls, sleeps, or machine state to default tests.
- Keep parser tests grounded in committed captures under `dev/fixtures/`. Never invent an unlabeled fixture; synthetic inputs require `_synthetic` plus their source and retirement condition. Synthetic fixtures prove crash resistance and expected shape only; they cannot establish real-format correctness and must be replaced by live captures when available.
- For intentional IPC changes, regenerate fixtures with `PM_UPDATE_CONTRACT=1 cargo test ipc_contract`, then run both language suites.
- Focused tests may be colocated; cross-feature frontend tests belong in `src/__tests__/`. DOM tests import `src/test/setup.ts` explicitly.
- Cover relevant success, failure, cancellation/concurrency, stale-state, and contract edge cases. A manager refresh failure must retain its prior snapshot.
- Frontend gates: `npm test`, `npx tsc --noEmit`, and `npm run build`.
- Rust gates from `src-tauri/`: `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, and `cargo test --locked`.
- Run `cargo test -- --ignored` only when live detection, command compatibility, routing, or packaging behavior needs real-machine verification.

### Code Quality & Style Rules

- Resolve guidance by role: `AGENTS.md` governs workflow; `docs/SPEC.md` governs product behavior; a later explicit `docs/DECISIONS.md` entry overrides an earlier decision; manifests, lockfiles, workflows, and production registration define current mechanics; fixtures/tests support claims only within their documented provenance. Treat `docs/IMPL_PLAN.md` as history and surface any conflict these rules do not resolve.
- Keep Tauri command handlers thin. Put orchestration in focused Rust modules, manager behavior in adapters, and pure, non-logging parsers under `src-tauri/src/managers/parse/`.
- Components and component files use PascalCase; hooks use `useX`; Zustand stores use `useXStore`; helpers use lower camelCase; constants use uppercase names. Follow surrounding Rust naming and module conventions.
- There is no configured Prettier or ESLint script. Match surrounding TypeScript/Markdown formatting; run `cargo fmt` on modified Rust before committing.
- Reuse existing primitives, stores, adapters, error mappings, and nondeterminism seams before introducing a new abstraction.
- Treat `queue.rs`, `process/runner.rs`, `ops.rs`, IPC types, persistence, diagnostics, and updater installation as high-risk trust boundaries requiring focused tests and review.

### Development Workflow Rules

- Preserve unrelated working-tree changes and read configuration files before editing them.
- Use conventional commits: `fix:` triggers a patch, `feat:` a minor, and `feat!:` or `BREAKING CHANGE:` a major release. `chore:`, `docs:`, `ci:`, `refactor:`, and `test:` do not trigger a release alone.
- Never manually edit versions in `package.json`, `package-lock.json`, `src-tauri/tauri.conf.json`, `src-tauri/Cargo.toml`, or `src-tauri/Cargo.lock`. Do not edit `CHANGELOG.md` or `.release-please-manifest.json`; release-please owns all seven files.
- A releasable commit reaching `main` enters release automation with no later human gate: release-please handles the release PR and the signed/notarized artifacts are published. Keep work off `main` until it is ready to ship.
- The manual Release workflow only uploads run artifacts; it does not modify a GitHub Release.
- Keep secrets in fnox locally and GitHub Secrets in CI. Never place decrypted keys in `.env`, source, documentation, shell history, or workflow text.

### Critical Don't-Miss Rules

- The package manager's `outdated` verdict is authoritative. Version-delta logic is display-only and must never decide whether an update exists.
- Derive manager ownership and self-update routing from detected paths. Classify the raw path before canonicalizing it; mise shims are symlinks and canonicalizing first misroutes npm/uv.
- Nothing bulk-executes before the user sees the exact commands. Canonicalize explicit plan selections before issuance: preserve `null`, enforce the 2,048-entry/512-byte-ID bounds, and remove exact manager/package duplicates first-seen-order. A bulk `planId` is a bounded, one-use backend capability bound to one monotonic canonical-state revision: execution must exactly match the issued plan and a fresh coherent rebuild, reject active refreshes/revision drift or locks held by an earlier queued/running mutation, and submit only freshly re-derived groups through the scheduler's atomic all-or-none batch admission. Tampering, replay, eviction, or drift returns `plan_stale`; the UI must display a new plan and require another confirmation.
- The single scheduler atomically checks and acquires each operation's complete lock set before start. All Homebrew work takes the Brew lock; routed operations lock executor and subject; mise-managed npm/uv work also protects Mise. Preserve global concurrency, fairness, and refresh coalescing.
- Never run shell command strings. Spawn resolved absolute executables with structured argv, `env_clear`, an explicit environment, null stdin, and a new process group. Self-update and health-fix argv stays backend-only; derive previews from argv and never split display text back into arguments. No sudo or password prompt path is allowed.
- Add `HOMEBREW_NO_AUTO_UPDATE=1` to every Brew command except the explicit `brew update` operation. Do not automatically retry external Homebrew lock contention.
- One manager failure must not blank other managers or overwrite its previous successful snapshot. Parser recovery must merge a complete inventory, not replace it with an outdated-only overlay.
- A manager-declared expected nonzero exit is not an operation failure; notably, usable npm outdated JSON may exit 1 and must reach the parser before error classification.
- Keep operations reconstructible: status/output events, transcript, structured log, and crash journal share the operation ID. Preserve the process reader's literal unterminated-notice handling and bounded post-exit EOF grace. On shutdown, cancel and reap process groups; never signal journaled PIDs after restart because PID reuse is unsafe.
- Settings and journal rewrites remain atomic. Persist a settings patch before publishing it in memory or advancing the canonical revision; a failed save changes neither. Journal-append and structured app-log write failures are nonfatal to package operations; transcript creation failure blocks spawn, while later transcript-write failures are best-effort. Diagnostics must reject symlinks both when selecting and when streaming files.
- Application-update checks/downloads may run in the background, but installation requires an explicit user Restart action. If the bundle parent is not writable, require manual installation rather than triggering an administrator prompt.
- Selection must always exclude pinned packages and exclude greedy casks unless the user explicitly opts in. Unknown latest versions remain `null`; never fabricate a version delta.
- Re-check self-update routes after every freshly parsed refresh snapshot; a manager's own outdated row can override delegated routing, especially npm inside mise.
- Only the exact recognized uv reinstall suggestion may expose `fixCommand` and trusted fix argv. Altered, missing, or malformed suggestions remain visible in the warning detail but are neither copyable nor runnable.
- Derive current command/event counts from production registration rather than treating old prose counts as invariants; the present surface is 20 commands and six events. `DECISIONS.md` D23a supersedes D23 for live-verified mas behavior, and D25 supersedes D20 for signed/notarized in-app updates.
- Tauri CSP is currently `null` and main-window capabilities are deliberately narrow. Treat any external-content, capability, or permission change as security-sensitive.

---

## Usage Guidelines

**For AI agents:**

- Read this file before implementing code and follow every applicable rule.
- When guidance conflicts or is unclear, consult `docs/SPEC.md` and `docs/DECISIONS.md` and prefer the safer, more restrictive interpretation.
- Update this file when a technology version, architectural invariant, or established implementation pattern changes.

**For humans:**

- Keep this file focused on project-specific rules that agents could otherwise miss.
- Review it after major architecture, tooling, testing, security, or release changes and remove obsolete guidance.

Last Updated: 2026-07-22
