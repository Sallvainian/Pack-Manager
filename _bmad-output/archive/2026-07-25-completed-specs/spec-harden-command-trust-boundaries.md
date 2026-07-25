---
title: 'Harden command trust boundaries'
type: 'bugfix'
created: '2026-07-22'
status: 'done'
review_loop_iteration: 1
baseline_commit: '54243bd99df56fdf8bf5df3661a1720f58202b6a'
context:
  - '{project-root}/_bmad-output/project-context.md'
---

<frozen-after-approval reason="human-owned intent — do not modify unless human renegotiates">

## Intent

**Problem:** `execute_plan` accepts a frontend-round-tripped plan without proving that the backend issued it or that its packages, exclusions, routes, locks, and previews are still current. Self-update and health-fix paths also split display text back into executable arguments, weakening the structured-command boundary.

**Approach:** Make `planId` a bounded, one-use backend capability; require exact equality with the issued plan and a freshly rebuilt canonical plan before enqueueing. Echo the originating `PlanRequest` in the plan so the UI can preserve user intent when a stale plan must be rebuilt. Keep self-update and health-fix argv backend-only, derive display previews from those arguments, and fail closed on missing or inconsistent structured data.

## Boundaries & Constraints

**Always:** Preserve “nothing runs that was not shown,” current native-state validation, atomic lock policy, absolute executable resolution, explicit environment, null stdin, and no-shell execution. Consume plans once; reject tampering, replay, eviction, or state drift before any submission. Keep existing command-preview bytes and normal IPC casing: object fields and ordinary enum values use lowerCamelCase, while stable error codes use snake_case.

**Ask First:** Any change to visible command text, package-manager semantics, selection/exclusion policy, or release behavior; any solution that needs persistent storage or a new external dependency.

**Never:** Trust executable groups, locks, routes, package IDs, or argv merely because they came from the frontend. Never execute a freshly rebuilt but previously unseen plan automatically, parse display strings with whitespace splitting, normalize every wire token to one casing style, or hand-edit release-owned version files.

## I/O & Edge-Case Matrix

| Scenario | Input / State | Expected Output / Behavior | Error Handling |
|----------|---------------|----------------------------|----------------|
| Valid execution | Unchanged backend-issued plan; native state unchanged | Consume capability, rebuild identically, enqueue exact displayed commands | None |
| Tamper or replay | Altered/unknown/evicted/already-used `planId` or plan fields | Enqueue nothing | Return actionable `plan_stale` |
| State drift | Package, exclusion, busy state, route, lock, warning, or preview changed | Enqueue nothing; UI rebuilds and displays the current plan for another confirmation | Keep sheet open and explain that review is required |
| Concurrent overlap | Two previously issued plans target the same package | Validation-through-enqueue is serialized; at most one plan succeeds | Other plan becomes `plan_stale` |
| Structured self-update | Current native route contains trusted argv | Execute argv elements unchanged; displayed preview remains identical | Missing/mismatched internal argv fails closed |
| Structured health fix | Exact recognized uv reinstall suggestion | Execute canonical `uv tool install <name> --reinstall` argv | Manipulated suggestion remains visible as a warning but is not runnable |

</frozen-after-approval>

## Code Map

- `src-tauri/src/state.rs` -- bounded plan store, monotonic canonical-state revision, and cancellation-safe redetection lease.
- `src-tauri/src/commands.rs` -- issue, consume, compare, rebuild, and enqueue plans; health/self-update entrypoints.
- `src-tauri/src/queue.rs` -- canonical plan builder, structured submission builders, revision-aware state publication, and atomic batch admission.
- `src-tauri/src/ipc.rs`, `src-tauri/src/error.rs` -- plan request echo, backend-only argv fields, and `plan_stale` error contract.
- `src-tauri/src/managers/*.rs`, `src-tauri/src/managers/parse/uv.rs` -- construct trusted self-update/health-fix argv and derived previews.
- `src/lib/ipc/types.ts`, `src/lib/errors.ts`, `src/components/dialogs/UpgradePlanSheet.tsx` -- error mirror/copy and stale-plan review flow.
- `_bmad-output/project-context.md` -- approved elicitation corrections and final enforced invariants.

## Tasks & Acceptance

**Execution:**
- [x] `src-tauri/src/state.rs`, `src-tauri/src/commands.rs`, `src-tauri/src/queue.rs` -- add a bounded one-use plan store, bind plans to one coherent monotonic revision, and admit complete plan batches atomically.
- [x] `src-tauri/src/commands.rs`, `src-tauri/src/queue.rs` -- compare submitted, issued, and freshly rebuilt canonical plans; derive submissions only from the fresh plan.
- [x] `src-tauri/src/ipc.rs`, `src-tauri/src/error.rs`, `src/lib/ipc/types.ts`, `src/lib/errors.ts`, `dev/fixtures/ipc/` -- echo the original request in each plan and add `plan_stale` with coordinated Rust/TypeScript handling.
- [x] `src-tauri/src/ipc.rs`, `src-tauri/src/managers/`, `src-tauri/src/queue.rs` -- replace preview parsing with backend-only structured argv and fail-closed consistency checks.
- [x] `src-tauri/src/managers/parse/uv.rs` -- construct only the exact allowlisted reinstall argv; make altered suggestions non-fixable.
- [x] `src/components/dialogs/UpgradePlanSheet.tsx` -- refresh a stale plan without auto-executing and require renewed confirmation.
- [x] Rust and frontend test files -- cover tampering, replay, drift, concurrency, argument boundaries, malformed fixes, error copy, and UI refresh behavior.
- [x] `_bmad-output/project-context.md` -- apply approved factual refinements and document the safeguards as enforced only after verification.

**Acceptance Criteria:**
- Given a backend-issued unchanged plan, when the user confirms it, then the backend consumes it once and enqueues only a freshly rebuilt byte-equivalent plan.
- Given any altered, replayed, missing, evicted, or stale plan, when execution is requested, then zero operations are submitted and the UI displays a refreshed plan requiring another click.
- Given overlapping plans, when execution races, then validation and enqueue are atomic enough that active-operation exclusions make all later conflicting plans stale.
- Given self-update or health-fix execution, when a command is built, then argv never comes from frontend or display-text parsing and its preview remains byte-equivalent.
- Given a uv warning with an altered reinstall suggestion, when parsed, then the warning remains visible but offers no runnable fix.
- Given the completed change, when contract and project gates run, then Rust/TypeScript error codes agree and all existing behavior remains green.

## Spec Change Log

- 2026-07-22: Implementation completed. Added one-use plan capabilities, exact current-state revalidation, structured backend-only self-update/health argv, stale-plan review UI, coordinated IPC changes, regression coverage, and current project-context/SPEC documentation.
- 2026-07-22: Review hardening completed. Replaced per-operation acknowledgement serialization with a coherent canonical-state revision and scheduler-level all-or-none batch admission; added active-refresh, fast-terminal, redetection-drift, cancellation, and frontend rebuild-race coverage.
- 2026-07-22: Review iteration 1 hardening completed. Canonicalized and bounded plan requests, rejected batches that would queue behind an earlier conflicting mutation, preserved malformed uv warnings as non-runnable issues, guarded every late sheet continuation after dismissal, and expanded end-to-end/regression coverage.

## Design Notes

- Cache at most 64 issued plans and evict the oldest. Store the original `PlanRequest` beside the canonical `UpgradePlan`; also echo that request in the wire plan solely so stale-plan UI refresh preserves whether the intent was “all outdated” or an explicit selection. Never trust the echoed copy during execution.
- Canonicalize untrusted requests before issue/cache: preserve `selection: null`, reject more than 2,048 selections or package IDs over 512 bytes, and remove identical `(managerId, packageId)` pairs first-seen-order. At both limits, raw selected package-ID bytes are bounded to 1 MiB per request before the plan-store's 64-entry cap.
- One shared synchronous coordinator owns the issued-plan cache, a monotonic revision, and the state-update barrier. Detection/routes, registry refresh publication, queue busy/stale transitions, settings, and ToolEnv follow coordinator-first locking. Execution removes the entry before comparison, validates one coherent snapshot, releases all guards, then sends the complete derived group set to the scheduler with the expected revision. The scheduler rechecks revision plus active-refresh/update barriers, enqueues all groups or none, and bumps once on success so even an immediately completed first plan invalidates every other prebuilt plan.
- Batch admission also rejects any incoming lock that intersects an already queued/running Upgrade, SelfUpdate, or HealthFix. Overlaps among groups inside that same incoming reviewed batch remain valid; the scheduler serializes them after atomic admission.
- Redetection uses an RAII state-update lease: no mutex is held over `detect_all().await`; successful publication completes under the coordinator, while cancellation/drop clears the barrier and advances the revision so execution cannot become permanently wedged.
- Keep `commandPreview`/`fixCommand` on the wire for unchanged UI copy. Add skipped backend-only argv fields, construct preview and argv together, and reject skipped/empty/mismatched argv if a deserialized wire model ever reaches execution.
- `plan_stale` is an intentional snake_case error code, not a change to ordinary enum casing.

## Implementation Notes

- The issued-plan store retains at most 64 session-local entries, evicts oldest-first, and consumes an entry before any comparison so failed validation cannot be replayed.
- Each issued capability records the coordinator revision. Execution authenticates the complete submitted wire plan, rejects revision drift and active refreshes, rebuilds from one coherent detection/routes/snapshots/queue/settings/ToolEnv epoch using the cached request, normalizes only the new UUID, and verifies re-derived submission executors, subjects, lock sets, package IDs, and command previews before enqueue.
- Plan groups cross the async boundary in one scheduler message. Admission rechecks the expected revision and state barriers before any record is created; the infallible scheduler-owned insertion loop exposes all groups or none and advances the revision exactly once on success. Ordinary queue submissions, terminal busy/stale transitions, refresh route/snapshot publication, settings changes, and redetection also advance the same revision.
- Settings persistence completes before the in-memory value and canonical revision are published; a failed save changes neither. The plan sheet guards both execute and rebuild continuations with mounted/latest-request refs, so dismissing it cannot trigger a stale rebuild or state update.
- `SelfUpdateRoute.command_args` and `HealthIssue.fix_args` are skipped by Serde. Queue builders require nonempty trusted argv and exact preview equality; a wire-deserialized route therefore fails closed.
- The uv parser recognizes the base tool warning independently from its optional suffix and exposes fix data only when a safe tool name produces the exact canonical reinstall suggestion. Altered, missing, or malformed suggestions remain in warning `detail` with no copy/run affordance.
- Matrix coverage that ran and passed:
  - Valid execution: `commands::tests::issued_plan_executes_once_and_replay_submits_nothing` and `commands::tests::round_tripped_multi_group_plan_executes_routed_self_update_structurally` cover round-trip serialization, routed self-update plus ordinary upgrade, op IDs/records/routes/locks, and actual argv.
  - Tamper/replay/eviction: `commands::tests::every_round_tripped_plan_section_is_authenticated`, `commands::tests::issued_plan_executes_once_and_replay_submits_nothing`, `commands::tests::oldest_plan_is_stale_after_bounded_cache_eviction`, and `state::tests::issued_plan_store_is_bounded_oldest_first_and_one_use`.
  - State drift and renewed UI review: `commands::tests::snapshot_drift_consumes_plan_and_submits_nothing`, `commands::tests::redetection_revision_drift_consumes_plan_and_submits_nothing`, `commands::tests::settings_change_after_issue_invalidates_plan_without_submission`, `commands::tests::failed_settings_persistence_changes_neither_memory_nor_revision`, `commands::tests::cancelled_pending_redetection_releases_revision_barrier`, and `stale_plan_refresh_requires_renewed_confirmation`.
  - Concurrent overlap and refresh barriers: `commands::tests::overlapping_prebuilt_plans_serialize_validation_through_enqueue`, `commands::tests::fast_terminal_first_plan_still_invalidates_second_prebuilt_plan`, `commands::tests::plan_issued_during_active_refresh_is_rejected_without_upgrade`, and `commands::tests::plan_cannot_queue_behind_earlier_direct_mutation_on_same_lock`.
  - Atomic group admission and preserved refresh coalescing: `queue::tests::plan_batch_revision_mismatch_enqueues_all_or_none`, `queue::tests::plan_batch_rejects_existing_mutation_with_intersecting_locks`, `queue::tests::plan_batch_rejects_running_self_update_with_intersecting_locks`, `queue::tests::plan_batch_rejects_queued_health_fix_with_intersecting_locks`, and `queue::tests::duplicate_refresh_coalesces_to_same_opid` cover every mutating kind plus both pending/running storage paths.
  - Request canonicalization: `queue::tests::canonical_request_deduplicates_before_rust_dedup_and_argv_planning`, `queue::tests::canonical_request_rejects_selection_and_package_id_limits`, and `commands::tests::issued_plan_echoes_deduplicated_request_and_rejects_oversized_selection` cover limits, echoed intent, duplicate Rust selections, dedup policy, and argv uniqueness.
  - Frontend rebuild readiness: `plan_rebuild_readiness_gate` covers pending, failure/retry, out-of-order responses, and non-stale execute failure; the stale-plan cases assert exact refreshed-plan submission, preservation of non-default and `selection: null` intent, and no late rebuild after Cancel/unmount.
  - Atomic derivation failure: `commands::tests::later_group_missing_backend_argv_submits_no_partial_batch` proves a later routed group cannot leave records, events, or runner calls behind.
  - Structured self-update: `queue::tests::self_update_submission_preserves_structured_argument_boundaries` and `queue::tests::self_update_submission_fails_closed_without_matching_backend_argv`.
  - Structured health fix: `queue::tests::health_fix_submission_binds_the_fix_command`, `queue::tests::health_fix_submission_fails_closed_for_untrusted_or_mismatched_argv`, `managers::parse::uv::tests::altered_uv_reinstall_suggestion_remains_visible_but_is_not_runnable`, `managers::parse::uv::tests::uv_warning_without_reinstall_suggestion_remains_visible`, `managers::parse::uv::tests::uv_warning_with_malformed_reinstall_suggestion_remains_visible`, and the HealthBanner altered-suggestion no-Copy/no-Run test.

## Verification

**Commands:**
- `cd src-tauri && cargo fmt --check` -- passed; no formatting differences.
- `cd src-tauri && cargo clippy --all-targets -- -D warnings` -- passed; no warnings.
- `cd src-tauri && cargo test --locked` -- passed: 245 default native tests, 9 ignored unit tests, and 2 ignored live tests; zero failures.
- `npm test` -- passed: 22 files, 130 tests; zero failures.
- `npx tsc --noEmit` -- passed with strict no-emit typechecking.
- `npm run build` -- passed; TypeScript and Vite production build completed.
- `cd src-tauri && PM_UPDATE_CONTRACT=1 cargo test ipc_contract` -- passed and regenerated the intentional `UpgradePlan.request` fixture change; `cargo test --locked ipc::tests::ipc_contract_matches_committed_fixtures -- --exact` and the subsequent full Rust/frontend suites passed.

## Suggested Review Order

**Capability and canonical-state boundary**

- Start here: authenticate, rebuild, derive, and hand one complete plan batch to the scheduler.
  [`commands.rs:354`](../../src-tauri/src/commands.rs#L354)

- Coordinate capabilities and canonical-state mutations under one monotonic epoch.
  [`state.rs:51`](../../src-tauri/src/state.rs#L51)

- Recheck epoch, refresh state, and mutation conflicts before admitting any batch group.
  [`queue.rs:1003`](../../src-tauri/src/queue.rs#L1003)

- Reject pre-existing mutators by their complete scheduler lock sets.
  [`queue.rs:1045`](../../src-tauri/src/queue.rs#L1045)

- Bound and deduplicate explicit selections before storing capabilities.
  [`queue.rs:406`](../../src-tauri/src/queue.rs#L406)

**Trusted structured commands**

- Keep executable self-update and health-fix argv backend-only across IPC.
  [`ipc.rs:200`](../../src-tauri/src/ipc.rs#L200)

- Re-derive self-update submissions from current trusted routes and argv.
  [`queue.rs:239`](../../src-tauri/src/queue.rs#L239)

- Require exact safe UV suggestions while preserving malformed warnings visibly.
  [`uv.rs:56`](../../src-tauri/src/managers/parse/uv.rs#L56)

**Renewed frontend confirmation**

- Sequence rebuilds and ignore every continuation after sheet dismissal.
  [`UpgradePlanSheet.tsx:46`](../../src/components/dialogs/UpgradePlanSheet.tsx#L46)

- Require a new review after stale or consumed-capability failures.
  [`UpgradePlanSheet.tsx:148`](../../src/components/dialogs/UpgradePlanSheet.tsx#L148)

**Evidence and contract trail**

- Exercise serialized multi-group execution with a routed self-update.
  [`commands.rs:1313`](../../src-tauri/src/commands.rs#L1313)

- Prove later derivation failure cannot partially submit earlier groups.
  [`commands.rs:1376`](../../src-tauri/src/commands.rs#L1376)

- Cover Upgrade, SelfUpdate, and HealthFix conflict admission paths.
  [`queue.rs:2528`](../../src-tauri/src/queue.rs#L2528)

- Verify stale, failed, out-of-order, unmounted, and all-packages UI rebuilds.
  [`planSheet.test.tsx:140`](../../src/components/manager/planSheet.test.tsx#L140)

- Record the enforced invariants and category-based IPC casing for future agents.
  [`project-context.md:90`](../project-context.md#L90)
