---
title: 'Prove Refresh Phases and Per-Manager Timeouts'
type: 'bugfix'
created: '2026-08-19'
status: 'blocked'
baseline_revision: 'ea48b7d98ea87ce30b2ed1bcfe15ac4cf84e65df'
review_loop_iteration: 0
followup_review_recommended: false
context:
  - '_bmad-output/specs/spec-shipped-behavior-gaps/SPEC.md'
  - '_bmad-output/planning-artifacts/architecture/architecture-Pack-Manager-2026-07-23/ARCHITECTURE-SPINE.md'
  - 'docs/SPEC.md'
warnings: ['oversized']
deferred:
  - summary: >-
      Rehydrate retained refresh failures and stale markers from terminal refresh records when the frontend remounts against the same backend.
    evidence: |-
      App.bootstrap hydrates snapshots and operation records but never rebuilds the per-manager error or stale state from a failed terminal refresh record, so a frontend-only remount loses the Last-good failure presentation.
    location: >-
      src/App.tsx:25
    severity: medium
  - summary: >-
      Disable conflicting Dashboard and Manager-pane controls while a manager operation is queued or running.
    evidence: |-
      docs/SPEC.md requires busy cards to disable conflicting actions, but ManagerCard and ManagerPane still leave refresh, retry, upgrade, self-update, and re-detect affordances enabled. Duplicate refreshes are backend-coalesced, so this is an existing UX-contract gap rather than a duplicate-operation defect in this story.
    location: >-
      src/components/dashboard/ManagerCard.tsx:99; src/components/manager/ManagerPane.tsx:174
    severity: medium
  - summary: >-
      Dim a retained stale Manager snapshot and package table as specified.
    evidence: |-
      ManagerPane renders the stale timestamp but applies no dimming treatment to the retained snapshot/table, while docs/SPEC.md section 4.8 calls for the stale snapshot below ErrorState to be dimmed.
    location: >-
      src/components/manager/ManagerPane.tsx:224
    severity: low
  - summary: >-
      Make Manager-pane stderr detail collapsible.
    evidence: |-
      ErrorState permanently expands up to ten stderr lines, while docs/SPEC.md section 4.8 describes the Manager-pane error detail as collapsible.
    location: >-
      src/components/primitives/ErrorState.tsx:34
    severity: low
  - summary: >-
      Qualify the F2 overview so disabled Homebrew metadata refresh does not contradict the command-table setting.
    evidence: |-
      docs/SPEC.md F2 still says every Brew refresh starts with brew update, while the existing setting and command table correctly allow that metadata phase to be disabled.
    location: >-
      docs/SPEC.md:67
    severity: medium
  - summary: >-
      Prevent bootstrap hydration from replacing a newer live Manager snapshot with the older snapshot captured by get_state.
    evidence: |-
      App subscribes before get_state resolves, but bootstrap writes every hydrated snapshot without comparing it to a snapshot:updated event already received. A captured older snapshot can therefore replace newer live data and be presented as fresh after the terminal success event has cleared staleness.
    location: >-
      src/App.tsx:31
    severity: medium
  - summary: >-
      Refuse a successful Homebrew recovery when both outdated JSON sources are unusable and text recovery cannot establish formula outdated state.
    evidence: |-
      Brew recovery discards both JSON parse errors with .ok() and can merge an empty or cask-only text overlay into inventory, publishing every formula as current even though no usable source established formula outdatedness.
    location: >-
      src-tauri/src/managers/brew.rs:218
    severity: medium
  - summary: >-
      Show the Dashboard busy indicator during non-Homebrew retries that retain a prior Manager error.
    evidence: |-
      Error phase precedence hides the live refresh phase, and only Homebrew commands emit the phase labels used as the secondary busy signal. A running retry for mise, npm, uv, rustup, or mas can therefore render without the card's busy bar.
    location: >-
      src/components/dashboard/ManagerCard.tsx:45
    severity: low
  - summary: >-
      Keep the Outdated only default derived from the latest snapshot until the user explicitly chooses a filter value.
    evidence: |-
      The first snapshot materializes the derived default as persistent state. If that snapshot is clean and a later refresh introduces updates, the toggle stays off even though the product contract defaults it on whenever outdated packages exist.
    location: >-
      src/store/packages.ts:56
    severity: low
---

<intent-contract>

## Intent

**Problem:** Refresh execution is mostly isolated already, but its trust contract is incomplete: Homebrew recovery can discard a successfully parsed pinned/outdated overlay, routed join updates can falsely clear a peer Manager's stale failure, and the Dashboard does not render Homebrew's emitted phase labels. Existing tests also do not prove every adapter's command timeouts, the setting-disabled phase sequence, or the exact Last-good Snapshot presentation.

**Approach:** Preserve every usable inventory and outdated source during recovery, make refresh completion—not an unqualified snapshot event—the authority that clears stale/error state, render the current Homebrew refresh phase on its card, and lock all six adapters' boundaries and failure-isolation behavior down with deterministic fake-time and component tests.

## Boundaries & Constraints

**Always:** Treat each Manager refresh as an independent queued Operation; preserve the exact ordered argv, structured process environment, null stdin, process-group handling, and adapter-owned absolute timeout. Homebrew with metadata enabled runs `update` before the two inventory and two outdated commands; disabling it removes only `update` and its phase. Recovery merges into the already-parsed inventory, preserves all current rows, OR-preserves `pinned`, and retains `pinnedVersion`. A failed Manager keeps its prior snapshot's real `refreshedAt`, real `health`, and exact `IpcError`; only a successful refresh terminal event may clear that Manager's stale/error state. Joined snapshots may update data but may not claim the joined subject refreshed successfully. At review finalization, if `_bmad-output/implementation-artifacts/deferred-work.md` is dirty, restore it to HEAD before the clean-tree check; this story's deferred items live only in this spec's `deferred:` frontmatter.

**Never:** Do not replace a snapshot with an outdated-only recovery overlay, infer outdatedness locally, unpin a row, fabricate health or staleness timestamps, turn a routed peer patch into refresh success, add wall-clock sleeps or real network access to tests, add a router or modal mount point, change the IPC contract for provenance when the existing terminal status event already identifies the refresh subject, or edit release-please-owned version/changelog files. Do not commit `_bmad-output/implementation-artifacts/deferred-work.md` and do not HALT finalization as `finalization left repository dirty` when that file is the remaining dirt — restore it instead. This story has no UX-PB dependency and does not implement persistent snapshots across process relaunch.

## I/O & Edge-Case Matrix

| Scenario | Input / State | Expected Output / Behavior | Error Handling |
|----------|--------------|---------------------------|----------------|
| Homebrew metadata enabled | `runBrewUpdateOnRefresh=true` | Card and events show metadata → inventory → outdated; argv/timeouts are update 600s, lists 60s each, outdated checks 120s each | A metadata non-zero exit remains best-effort as currently specified; a real timeout is terminal for Brew only |
| Homebrew metadata disabled | `runBrewUpdateOnRefresh=false` | Exact enabled-plan suffix remains: two list commands and two outdated checks with their labels/timeouts; only metadata command/phase is absent | No later phase is skipped, renamed, or shifted into global state |
| Partial Brew JSON recovery | Inventory valid; plain outdated JSON has a pinned formula; greedy JSON is malformed; greedy text recovery succeeds | Publish full inventory merged with every successfully parsed JSON overlay and recovered text; current rows survive and the formula remains outdated, pinned, and keeps `pinnedVersion` | If recovery parsing fails, keep the prior registry snapshot and finish only Brew as failed |
| Manager-specific timeout | Any of six adapters has a gated command at its documented absolute boundary | That Operation becomes `TimedOut` with its own subject, phase, seconds, and error while a disjoint peer can finish | No peer snapshot, error, or terminal state changes |
| Routed peer patch | A failed/stale uv or mise snapshot receives a `snapshot:updated` caused by its executor's cross-manager self-version join | Data fields may patch, but the subject's exact failure and stale marker remain | Do not infer success from an event with no refresh provenance |
| Successful retry | A terminal `Succeeded` refresh status arrives for one subject after its new snapshot | Clear stale/error for that subject only | Joined peers and unrelated Managers remain untouched |
| Last-good presentation | Fixed snapshot timestamp, empty health, up-to-date and pinned rows, then exact refresh error | Dashboard and Manager pane show the exact failure and timestamp beside `Retry refresh`; retained rows remain browsable; no Health issue is invented | Retry submits only that Manager's refresh |
| Review finalization with harvested ledger dirt | `_bmad-output/implementation-artifacts/deferred-work.md` is dirty from orchestrator harvest; other reviewed-diff files are committed | Restore that file to HEAD, then the working copy is clean; the story commit does not include the ledger | Do not HALT `finalization left repository dirty` for that file; restore it and continue |

</intent-contract>

## Code Map

- `src-tauri/src/managers/mod.rs:67-94` -- `ManagerAdapter` already passes all normal `refresh_outputs` into `parse_recovery`; preserve this seam and document its current signature.
- `src-tauri/src/managers/parse/mod.rs:46-95` -- shared deterministic inventory/overlay merge; it already ORs `pinned` and field-merges `pinned_version`, but lacks an explicit invariant regression.
- `src-tauri/src/managers/brew.rs:81-228` -- exact enabled/disabled phase plan and the defect: recovery rebuilds list inventory but ignores usable normal outdated JSON before merging text.
- `src-tauri/src/managers/{mise,npm}.rs` -- recovery already merges parsed inventory with text overlays; strengthen assertions for retained current-row fields rather than replacing these implementations.
- `src-tauri/src/managers/{uv,rustup,mas}.rs` -- no recovery path; their exact refresh argv and 60/120, 30/120, and 60/120 second boundaries complete the six-adapter contract.
- `src-tauri/src/queue.rs:150-179,1134-1181,1639-1806` -- preserves plan order, coalesces per subject, executes phase-labeled specs serially, enforces command timeouts through `FakeRunner`, passes recovery inputs, and publishes only after parse success. Its paused-time harness is the integration test seam.
- `src-tauri/src/process/fake.rs:269-280,404-449` -- gated buffered responses honor the real `CommandSpec.timeout`; use these gates, never injected timeout errors, to prove deadline behavior.
- `src-tauri/src/registry.rs:75-117` -- one upsert can emit changes for routed subjects as well as the refreshed executor; those secondary events are data patches, not freshness proof.
- `src/store/packages.ts:13-69` -- `setSnapshot` currently clears stale unconditionally; split data ingestion from explicit `clearStale`.
- `src/lib/ipc/events.ts:90-105` -- `onSnapshot` currently clears Manager error; move subject-only clearing to terminal successful refresh handling while retaining failure marking.
- `src/store/index.ts:20-51` and `src/store/operations.ts:68-108` -- Manager phase is derived and operation views already retain `phaseLabel`; add a narrow current-refresh-label selector without storing duplicate phase state.
- `src/components/dashboard/ManagerCard.tsx:34-221` -- render the live Homebrew phase, exact failure, real snapshot timestamp, and `Retry refresh` within the existing card.
- `src/components/manager/ManagerPane.tsx:208-238` and `src/components/primitives/ErrorState.tsx:5-53` -- pane already keeps health and stale snapshot separate; provide context-specific `Retry refresh` copy without making the primitive refresh-only.
- `src/__tests__/events.test.ts`, `src/__tests__/dashboard.test.tsx`, `src/components/manager/managerPane.test.tsx`, `src/__tests__/store.test.ts` -- current frontend coverage is loose or asserts the incorrect unconditional clear; replace it with exact subject, timestamp, failure, phase, row-retention, and no-invented-health checks.
- `docs/SPEC.md:382-423` -- read-only product authority during implementation except for correcting the stale `parse_recovery` signature and refresh-label wording; no IPC fixture regeneration is needed.

## Tasks & Acceptance

**Execution:**

1. `src-tauri/src/managers/parse/mod.rs` -- add a focused merge regression proving an unpinned overlay cannot clear a pinned base or `pinned_version`, and an unmatched current row remains byte-for-byte meaningful (`latest == installed`, `outdated == false`).
2. `src-tauri/src/managers/brew.rs` -- make recovery merge list inventory, each independently parseable normal outdated JSON result, and recovered text in that order so partial JSON success is not discarded; retain the honest plain-cask fallback when greedy-only classification is unknowable. Strengthen enabled/disabled plan tests so the disabled plan equals the enabled suffix in argv, labels, environment, and timeouts.
3. `src-tauri/src/managers/mod.rs`, `src-tauri/src/managers/mise.rs`, and `src-tauri/src/managers/npm.rs` -- add the table-driven six-adapter argv/absolute-timeout contract and strengthen recovery regressions to compare retained current-row installed/latest/outdated/pinned fields, not mere presence.
4. `src-tauri/src/queue.rs` -- add paused-time, gated tests for all six Manager timeout identities and documented boundaries; prove a disjoint peer succeeds before a hung Manager times out, a fixed Last-good registry snapshot is unchanged after terminal failure, metadata-disabled Brew emits/runs exactly the remaining phases, and partial Brew recovery preserves both a pinned outdated formula and an unrelated current row through the production queue path.
5. `src/store/packages.ts` and `src/lib/ipc/events.ts` -- make snapshot ingestion data-only for freshness, add explicit stale clearing, and clear error/stale only from a successful refresh terminal event for its `subject`; preserve exact error/stale state across routed join-patch snapshot events.
6. `src/store/index.ts` and `src/components/dashboard/ManagerCard.tsx` -- derive the current running refresh label from operation records with a narrow selector and render it on the Homebrew card; change its recovery action to `Retry refresh`.
7. `src/components/primitives/ErrorState.tsx` and `src/components/manager/ManagerPane.tsx` -- allow caller-supplied retry copy and use `Retry refresh` for Manager refresh failures while keeping exact backend failure detail and retained timestamp adjacent.
8. `src/__tests__/events.test.ts`, `src/__tests__/dashboard.test.tsx`, `src/components/manager/managerPane.test.tsx`, and `src/__tests__/store.test.ts` -- cover joined-peer non-clearing, subject-only successful clearing, enabled/disabled phase rendering, fixed Last-good timestamp and exact error/detail, retained up-to-date `jq` and pinned `deno`, unaffected peer state, retry dispatch, and absence of a synthesized Health banner when `health` is empty.
9. `docs/SPEC.md` -- update the documented `parse_recovery` signature to include captured refresh outputs and align the Manager recovery action text with `Retry refresh`; do not change the command, event, or serialized model inventory.

**Acceptance Criteria:**

- Given Homebrew metadata refresh is enabled, when its real phase events reach the Dashboard, then the Homebrew card visibly progresses through `Updating Homebrew metadata…`, `Listing installed…`, and `Checking outdated…` in command order.
- Given Homebrew metadata refresh is disabled, when its refresh runs and renders, then the Homebrew card begins at `Listing installed…`, proceeds to `Checking outdated…`, never shows the metadata phase, and still finishes with the complete Package table.
- Given any of the six Managers reaches a documented command deadline under paused time, when its Dashboard card and Manager workspace receive the terminal state, then that Manager alone shows the correct timeout and actionable detail while a disjoint peer remains successfully populated without network access or sleeps.
- Given Brew plain JSON successfully marks a formula outdated and pinned but greedy JSON requires text recovery, when the production refresh path updates the Homebrew Package table, then the pinned formula stays pinned/outdated with its pin metadata and every current inventory row remains visible.
- Given a Manager has a fixed Last-good Snapshot and its later refresh fails or times out, when Dashboard and Manager workspace render, then they show that snapshot's exact timestamp and the exact failure beside `Retry refresh`, retain current and pinned rows, and render no invented Health issue.
- Given a routed executor refresh patches another Manager's snapshot, when the secondary snapshot event arrives, then the joined Manager's existing stale marker and exact error remain until that Manager's own successful refresh terminal event clears them.
- Given `_bmad-output/implementation-artifacts/deferred-work.md` is dirty solely from orchestrator harvest, when review finalizes, then that file is restored to HEAD, it is not in the story commit, and leftover dirt in that file is not a blocking condition.

## Spec Change Log

- 2026-09-16 — Escalation resolution (`bmad-loop-resolve` 2-2): leftover uncommitted changes in orchestrator-owned `_bmad-output/implementation-artifacts/deferred-work.md` are restored to HEAD before the clean-tree check and are never committed. Deferred items stay in this spec's `deferred:` frontmatter. The previous review HALT (`finalization left repository dirty` on that file) is the known-bad state this removes.

## Review Triage Log

### 2026-08-19 — Review pass
- intent_gap: 0
- bad_spec: 0
- patch: 5: (high 0, medium 3, low 2)
- defer: 5: (high 0, medium 3, low 2)
- dismissed:
  - Isolating each real adapter CommandSpec in the deadline table fails to prove later-plan deadlines — dismissed because the same queue timeout seam wraps every retained real spec, while separate adapter and queue tests prove ordered execution and abort-on-failure.
  - The timeout test does not prove exact phase/deadline presentation in a component — dismissed because the Rust record asserts the exact manager, configured seconds, phase command, and error, while the frontend separately proves exact IpcError rendering without transforming it.
  - The all-adapter argv/timeout table omits environment, null-stdin, and process-group assertions — dismissed because those unchanged runner invariants are covered at their construction/runner seams and were not altered by this story's plan-contract test.
  - Passing the recovery command as the unit test's failed argument masks production behavior — dismissed because Brew deliberately ignores that parameter and consumes every independently usable captured output, so the named mismatch has no runtime consequence.
  - Retry refresh can submit duplicate operations while the old error remains visible — dismissed because the queue coalesces every queued/running refresh for the same subject to its existing operation id; the broader pre-existing disabled-control UX gap is deferred separately.
  - Disabled-setting phase rendering is not one native-boundary end-to-end test — dismissed because the queue test proves the setting-to-command/event sequence and the component test proves those exact event labels render; the typed IPC shape is unchanged.
  - Recovered pin/current rows are not rendered from the exact queue-produced object — dismissed because queue tests prove the published snapshot values and component tests now prove those retained values render as current and visibly pinned/non-selectable through the unchanged snapshot contract.
  - The actual backend failure is not carried through one native-boundary rendering test — dismissed because event tests preserve the exact IpcError object and component tests render its exact message/detail/timestamp; no serialization contract changed.
  - Last-good health preservation is only layered — dismissed because the registry test proves the real health vector is byte-preserved and ManagerPane renders only snapshot.health, with a component regression proving an empty vector creates no Health banner.
  - ManagerPane places the retained timestamp below ErrorState rather than in the retry button row — dismissed because the exact error and Retry refresh remain grouped in ErrorState and the immediately following stale-data label carries the exact retained timestamp in the same error presentation.
  - Story-wide deadline and freshness changes exceed a narrow recovery-only reading — dismissed because the invocation explicitly selected Story 2-2, whose refresh-phase, timeout, isolation, and Last-good surfaces authorize those changes.
- addressed_findings:
  - `[medium]` `[patch]` Added reverse partial-JSON and ordinary-versus-greedy cask identity regressions so every independently parseable Brew source and both classification directions are protected.
  - `[medium]` `[patch]` Removed the paused-time watchdog race by waiting for the exact FakeRunner call before advancing and asserted the terminal timeout status/error identity.
  - `[medium]` `[patch]` Preserved live event-only phase and queue-position fields across same-operation hydration and proved terminal status clears the visible phase.
  - `[low]` `[patch]` Strengthened Last-good ManagerPane coverage to assert the retained pinned row visibly says Pinned and remains non-selectable.
  - `[low]` `[patch]` Corrected the successful-refresh isolation fixture so its embedded Manager identity matches the uv subject.

### 2026-08-19 — Review pass
- intent_gap: 0
- bad_spec: 0
- patch: 2: (high 0, medium 1, low 1)
- defer: 9: (high 0, medium 5, low 4)
- dismissed:
  - The metadata-disabled Dashboard test does not itself set the native setting — dismissed because the queue test proves setting-to-event causality and the component test observes the exact event labels at the requested Dashboard surface through the unchanged typed IPC contract.
  - The browser suite lacks a second metadata-disabled journey — dismissed because the deterministic queue and component layers already prove the disabled branch end to end across their respective native and visible seams, and duplicating the native setting in a browser-local IPC double would not add causality evidence.
  - The browser timeout fixture's message and detail are not the literal production Timeout payload — dismissed because Rust asserts the exact production timeout payload and frontend tests independently prove lossless IpcError rendering; the browser fixture exercises visible structured-error composition without claiming native serialization.
  - The routed-peer browser journey sends terminal success without first emitting another subject snapshot — dismissed because snapshot publication order is owned by the already-tested backend queue, while the frontend contract correctly treats subject terminal success as freshness authority and does not require snapshot provenance.
  - Deadline cases retain only the targeted real command instead of waiting through each full adapter plan — dismissed because adapter tests prove full plan order and timeout assignment, and every retained command traverses the same production queue deadline seam under paused time.
  - The browser routed-patch journey does not create the patch through a real registry join — dismissed because registry tests own join routing and the unchanged snapshot event contract carries no provenance; this story's frontend behavior begins at receipt of that typed event.
  - New adapter tests do not repeat base runner environment, null-stdin, and process-group assertions for all six managers — dismissed because those unchanged structured-runner invariants are already asserted at their shared construction and runner seams.
  - Last-good native retention and empty-health rendering use separate deterministic fixtures — dismissed because the unchanged snapshot contract connects byte-exact registry retention to component rendering, and each layer directly observes the consequence it owns.
  - The test-automation Definition of Done reports PASS despite unrelated product gaps — dismissed because it explicitly reports Story 2-2 automation coverage and its test-boundary residual risks; it does not claim that pre-existing deferred product work is resolved.
  - A connected adapter-to-browser process is the only defensible intent reading — dismissed because the intent explicitly calls for deterministic fake-time and component tests, making the implemented layered-seam reading authoritative without real package-manager or network execution.
  - The merged Playwright fixture migration is broader than a minimal production-fix reading — dismissed because it is test infrastructure used by the requested browser acceptance coverage and does not alter the product surface or contradict the intent.
- addressed_findings:
  - `[medium]` `[patch]` Reconciled subscribe-before-hydrate operation records by monotonic lifecycle progress, retained newer event-only operations, and added regressions for running, terminal, and absent-record races.
  - `[low]` `[patch]` Added a Dashboard assertion that a Homebrew retry keeps its busy indicator visible while the retained error still owns the Manager phase.

## Design Notes

The freshness authority is the terminal refresh event because it carries `kind`, `subject`, and final status. `snapshot:updated` deliberately has no provenance and can be emitted for cross-manager join patches, so treating it as proof of freshness is incorrect even when the data itself is valid.

Brew recovery uses nested merges rather than one flattened overlay: list inventory → every independently parseable JSON overlay → recovered text. Nested merging preserves a pin found in a successful JSON result even though the text fallback cannot report pins, while still allowing recovered latest/outdated values to win.

Planning baseline revision: `ea48b7d98ea87ce30b2ed1bcfe15ac4cf84e65df`.

## Verification

**Commands:**

- `cd src-tauri && cargo fmt --check` -- expected: formatting is clean.
- `cd src-tauri && cargo clippy --all-targets -- -D warnings` -- expected: no warnings.
- `cd src-tauri && cargo test --locked` -- expected: all adapter, queue, recovery, and fake-time tests pass offline.
- `npx tsc --noEmit` -- expected: frontend types are clean.
- `npx vitest run` -- expected: all event/store/component regressions pass.
- `npm run build` -- expected: production frontend build succeeds.
- `npm run test:e2e:typecheck` -- expected: browser test sources type-check.
- `npm run test:e2e` -- expected: required browser lane remains green.

**Manual checks (if no CLI):**

- Start a Homebrew refresh with the metadata setting on and off; confirm the card cycles the exact visible phase sequence and the off path begins at `Listing installed…`.
- With a retained Manager snapshot, induce a refresh failure; confirm its old current and pinned rows remain available, its actual timestamp and failure appear beside `Retry refresh`, and other Manager cards remain unchanged.

## Auto Run Result

Status: blocked
Blocking condition: finalization left repository dirty because the orchestrator-owned `_bmad-output/implementation-artifacts/deferred-work.md` remains modified and was explicitly excluded from review and commit.

### Summary

Implemented and reviewed the Story 2-2 refresh trust contract: Homebrew recovery preserves every usable inventory/outdated source and pin metadata; all six adapters' argv and deadline boundaries are covered; terminal refresh status is the sole frontend freshness authority; Homebrew phases and exact Last-good failures render on the requested surfaces; and deterministic browser journeys now cover the composed refresh UI. This review additionally hardened subscribe-before-hydrate operation reconciliation so stale `get_state` records cannot erase a newer running phase, resurrect a terminal operation, or drop an event-only operation.

### Files changed

- `src-tauri/src/managers/brew.rs` — layered usable JSON and text recovery over retained inventory and added recovery/phase regressions.
- `src-tauri/src/managers/mise.rs`, `src-tauri/src/managers/npm.rs`, `src-tauri/src/managers/mod.rs`, `src-tauri/src/managers/parse/mod.rs` — strengthened six-adapter command contracts and merge invariants.
- `src-tauri/src/queue.rs` — added deterministic per-command deadline, peer-isolation, Last-good, disabled-phase, and production recovery coverage.
- `src/lib/ipc/events.ts`, `src/store/packages.ts` — separated snapshot data ingestion from terminal-success freshness clearing.
- `src/store/index.ts`, `src/store/operations.ts` — derived visible refresh labels and reconciled event/hydration lifecycle races monotonically.
- `src/components/dashboard/ManagerCard.tsx`, `src/components/manager/ManagerPane.tsx`, `src/components/primitives/ErrorState.tsx` — rendered Homebrew phases, exact retained failures/timestamps, and `Retry refresh` copy.
- `src/__tests__/dashboard.test.tsx`, `src/__tests__/events.test.ts`, `src/__tests__/store.test.ts`, `src/components/manager/managerPane.test.tsx` — covered phase, freshness, hydration, retry-busy, isolation, and Last-good presentation behavior.
- `docs/SPEC.md` — aligned recovery signature and Manager retry wording with the implemented contract.
- `tests/e2e/refresh-trust-journeys.spec.ts` — added three deterministic browser journeys for Last-good timeout, routed-peer freshness, and Homebrew phases.
- `tests/support/merged-fixtures.ts`, `tests/support/fixtures/pack-manager-fixture.ts`, `tests/support/fixtures/index.ts`, `tests/support/fixtures/factories/pack-manager.ts` — composed Playwright utilities with the typed fake-Tauri fixture and deterministic factories.
- `tests/e2e/browser-style-contract.spec.ts`, `tests/e2e/framework-contract.spec.ts`, `tests/e2e/package-search.spec.ts`, `tests/e2e/quit-guard.spec.ts`, `tests/e2e/upgrade-journeys.spec.ts`, `tests/README.md` — migrated and documented the shared browser fixture entrypoint.
- `_bmad-output/test-artifacts/**`, `_bmad-output/implementation-artifacts/bmad-build-auto-result-2-2-tea.automate-1.md` — recorded test-automation generation and verification evidence.
- `_bmad-output/specs/spec-shipped-behavior-gaps/stories/2-2-prove-refresh-phases-and-per-manager-timeouts.md` — recorded review triage, deferred findings, verification, and final status.

### Review findings

- Patches applied: 2 entries — high 0, medium 1, low 1.
  - Reconciled newer live, terminal, and event-only operation records against stale bootstrap hydration.
  - Added direct coverage for the busy indicator while a Homebrew retry retains its prior error.
- Items deferred: 9 entries — high 0, medium 5, low 4. Five were already recorded and left unchanged; four new pre-existing gaps were appended to this story's `deferred` list. The orchestrator-owned deferred-work ledger was not opened or modified.
- Dismissed findings:
  - Metadata-disabled component coverage does not set the native setting — dismissed because deterministic queue coverage owns setting-to-event causality and the component observes the requested Dashboard surface.
  - No metadata-disabled browser journey — dismissed because a browser-local IPC double cannot add native-setting causality beyond the existing queue and component layers.
  - Browser timeout text differs from the literal production Timeout payload — dismissed because Rust asserts that exact payload and frontend tests prove lossless structured-error rendering.
  - Routed-peer browser success has no immediately preceding subject snapshot — dismissed because backend queue ordering is separately tested and terminal subject success is intentionally the frontend freshness authority.
  - Deadline cases isolate their target command — dismissed because full adapter order/assignment and the shared production queue deadline seam are independently proven.
  - Routed-patch browser coverage does not create a real registry join — dismissed because registry routing is separately tested and the unchanged snapshot event intentionally carries no provenance.
  - Adapter tests do not repeat runner environment/null-stdin/process-group checks — dismissed because those unchanged invariants are owned by shared runner tests.
  - Native Last-good retention and empty-health rendering use separate fixtures — dismissed because the unchanged snapshot contract connects the directly observed native and visible consequences.
  - Test-automation PASS omits unrelated deferred product gaps — dismissed because that artifact is explicitly scoped to Story 2-2 automation and states its actual boundary risks.
  - Only a connected native-to-browser intent reading is valid — dismissed because the intent explicitly requests deterministic fake-time and component testing at layered seams.
  - The merged Playwright fixture migration exceeds a minimal production fix — dismissed because it directly supports the requested browser acceptance coverage and changes no product behavior.

### Follow-up review recommendation

`false` — patched findings: high 0, medium 1, low 1; score = `3 × 1 + 1 × 1 = 4`, below the threshold of 5.

### Verification performed

- `cd src-tauri && cargo fmt --check` — passed.
- `cd src-tauri && cargo clippy --all-targets -- -D warnings` — passed with no warnings.
- `cd src-tauri && cargo test --locked` — passed: 269 tests, 11 ignored, 0 failed.
- `npx tsc --noEmit` — passed.
- `npx vitest run` — passed: 153 tests in 23 files.
- `npm run build` — passed with Vite production output generated successfully.
- `npm run test:e2e:typecheck` — passed.
- `npm run test:e2e` — passed: 28 tests across Chromium and WebKit.
- Focused hydration/Dashboard regression run — passed: 24 tests in 2 files.
- Complete frontmatter YAML parse — passed; `deferred` is one list with nine well-formed entries.

### Residual risks

- The nine deferred entries remain intentionally outside this story's patch set and are owned by the orchestrator for later resolution.
- Browser tests use the deterministic fake-Tauri boundary; native process timing and package-manager output remain proven by Rust fake-time and adapter tests rather than live network execution.
