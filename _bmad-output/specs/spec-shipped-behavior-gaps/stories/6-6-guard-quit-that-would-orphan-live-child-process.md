---
title: 'Guard a Quit That Would Orphan a Live Child Process'
type: 'feature'
created: '2026-08-19'
status: 'done'
review_loop_iteration: 1
followup_review_recommended: false
context:
  - '_bmad-output/specs/spec-shipped-behavior-gaps/SPEC.md'
  - '_bmad-output/planning-artifacts/architecture/architecture-Pack-Manager-2026-07-23/ARCHITECTURE-SPINE.md'
warnings: ['oversized']
deferred:
  - 'Recover safely when one event-listener registration fails.'
---

<intent-contract>

## Intent

**Problem:** `QuitGuardDialog` is built and mounted, but nothing but the application-update path ever opens it — `src/` has no window-close listener and no `⌘Q` sink, and `src-tauri/src/lib.rs` handles only `RunEvent::Ready` and `RunEvent::Exit`, so a user-initiated quit with work in flight silently discards it. Separately, `Sched::try_start_all` keeps admitting pending ops after `cancel_all`, so a queued op can spawn a fresh child process *during* the shutdown wait and outlive the app.

**Approach:** Route both user-initiated quit paths — the red close button / `⌘W`, and a custom `⌘Q` menu item that replaces `PredefinedMenuItem::quit` — into a single Rust enforcement point that applies the *same* predicate function `refuse_app_update_while_busy` uses, emits one `quit:requested` event on refusal, and lets the existing `QuitGuardDialog` present the choice. Confirming calls a new `confirm_quit` command. An OS shutdown/logout is excluded structurally, not by a heuristic: it arrives as `terminate:` → `applicationWillTerminate:` → `RunEvent::Exit`, which never reaches the enforcement point and already runs the awaited kill hook.

## Boundaries & Constraints

**Always:**
- **One predicate.** Extract the `OpStatus::Queued | OpStatus::Running` filter out of `refuse_app_update_while_busy` into one shared function, and have both the app-update refusal and the quit guard call it. The two active sets must not be able to drift apart (AD-30, FR-21). A test must assert the two agree over the full seven-variant `OpStatus` matrix.
- **One dialog, one refusal.** Every quit trigger reaches the same enforcement function and the same `{ kind: "quitGuard", reason: "quit" }` dialog. No trigger decides for itself.
- **Queued counts as running.** Admission has already committed to the work.
- **Children never outlive the app.** The shutdown path must set a drain latch so no pending op starts after cancellation, finalize queued work, cancel every running op, and *await* the bounded idle wait — running cancellation only flips tokens; `runner.rs` `kill_group` does the SIGTERM → 5s grace → SIGKILL work inside each op's task and must be polled.
- **No rollback is promised.** Partially completed Manager work stays partially completed; the guard surfaces the choice and never offers to undo.
- An OS-initiated shutdown or logout gets **no dialog** and is best-effort.

**Never:**
- Never hand-edit `src-tauri/Cargo.toml`, `src-tauri/Cargo.lock`, `package.json`, `package-lock.json`, or `src-tauri/tauri.conf.json` — release-please owns them (`AGENTS.md`). This rules out adding a `tauri = { features = ["test"] }` dev-dependency, so `tauri::test::mock_builder` is not available.
- Never adopt a native E2E harness here — see Design Notes. Not in scope.
- Never distinguish user-initiated from OS-initiated quit by a heuristic, timer, or environment sniff. The distinction is which code path the OS uses.
- Never change the app-update refusal's observable behavior, its `ErrorCode::SelfUpdateUnavailable`, or its message.
- Never add a second frontend keyboard handler for `q` — the accelerator belongs to the native menu item.
- No new Tauri capability grant, no `plugin:app|exit` from the frontend, no `sudo`/shell path.

## I/O & Edge-Case Matrix

| Scenario | Input / State | Expected Output / Behavior | Error Handling |
|----------|--------------|---------------------------|----------------|
| Quit, idle | No record is `Queued` or `Running`; user hits `⌘Q` or the close button | No dialog; the app exits and the kill hook runs | No error expected |
| Quit, work in flight | ≥1 record `Queued` or `Running`; user hits `⌘Q` | `prevent`s the exit, emits `quit:requested` with those `opIds`; dialog lists them and offers `Keep running` / `Cancel operations and quit` | No error expected |
| Quit, queued only | 1 record `Queued`, none `Running` | Same as above — queued counts as running | No error expected |
| Window close, work in flight | ≥1 active record; red close button or `⌘W` | Identical enforcement point, identical dialog | No error expected |
| Keep running | Dialog open, user picks `Keep running` | Dialog closes; app stays open; no op is cancelled; `confirm_quit` is not invoked | No error expected |
| Quit anyway | Dialog open, user picks `Cancel operations and quit` | Each listed op is cancelled, then `confirm_quit` exits; the kill hook drains and awaits before the process ends | A failed `cancel_operation` is logged via `logFrontendEvent`; the exit still proceeds |
| Queued op mid-shutdown | 1 `Running` + 1 `Queued`; shutdown begins | The drain latch stops admission; the queued op never starts and spawns no child | No error expected |
| Grace elapses | A child ignores SIGTERM past the 7s bound | `wait_until_idle` returns `false`; `state.rs` logs the warning and the process still exits | Warn, do not hang |
| OS shutdown / logout | ≥1 active record; macOS sends `terminate:` | No dialog at all; `RunEvent::Exit` runs the awaited kill hook | No error expected |
| Programmatic exit | `app.exit(0)` from `confirm_quit`, or `app.restart()` from the update path | `ExitRequested` carries `code: Some(_)` and is never re-guarded — no dialog loop | No error expected |

</intent-contract>

## Code Map

Backend — `src-tauri/src/`:
- `commands.rs:764-795` — `refuse_app_update_while_busy`; its doc already says "The status set matches `activeOps` in `src/store/operations.ts` exactly." The `matches!(record.status, OpStatus::Queued | OpStatus::Running)` at `:776-779` is **the** predicate to extract. Sole caller `:810`. Existing matrix test `:857-901`.
- `commands.rs` — `install_app_update` atomically reserves idle admission before installation; `confirm_app_update` is the explicit cancel-and-drain sink used after the update guard.
- `ipc.rs:96-107` — `OpStatus`, seven variants, `#[serde(rename_all = "camelCase")]`.
- `ipc.rs:544-581` — contract-fixture `check()` helper; `:787` `ipc_contract_matches_committed_fixtures`. Fixtures live in `dev/fixtures/ipc/` (repo root), 15 files today.
- `events.rs:74-79` — the six `EVENT_*` name constants; `:82-90` `AppEvent`; `:92-113` `name()` + `payload_json()`; `:117-119` `EventSink`; `:159-173` `TauriSink` → `AppHandle::emit`.
- `queue.rs:713-717` — `Shared { records, buffers, tokens }`, `#[derive(Default)]` — where the drain latch goes.
- `queue.rs:875-883` — `cancel_all`; its doc already states the await requirement.
- `queue.rs:885-901` — `wait_until_idle(timeout) -> bool`; polls `running().is_empty()` every 25ms.
- `queue.rs:952-970` — the scheduler loop; **`:969` calls `sched.try_start_all_coordinated()` after every message including `Msg::Finished`** — this is why cancelling running ops can start queued ones mid-shutdown.
- `queue.rs:1290-1294` / `:1296+` — `try_start_all_coordinated` → `try_start_all`; the latch is honored here.
- `queue.rs:1330-1332` — tokens are registered only when an op *starts*, so `cancel_all` never touches queued ops.
- `queue.rs:839-846` `records()`, `:868-873` `running()`.
- `queue.rs:3007-3034` — `cancel_all_then_wait_until_idle_reaps_running_refresh`, the existing paused-time regression.
- `process/runner.rs:56-57` `TERM_GRACE = 5s`; `:259-275` `kill_group`; `:369` `_ = cancel.cancelled() => break LoopEnd::Cancelled`; `:410-413` dispatch. `:304` `.process_group(0)`.
- `state.rs:189-191` `SHUTDOWN_GRACE = 7s`; `:360-386` `shutdown()` — **already** cancels then `block_on(wait_until_idle(...))`. `:193-213` `AppState` fields (`pub queue`, `pub sink`). Its `mod tests` (`:389+`) has one unrelated test.
- `lib.rs:57-134` `build_menu`; **`:91` `&PredefinedMenuItem::quit(app, None)?`** — on macOS `muda` binds this to the `terminate:` selector (`muda/src/platform_impl/macos/mod.rs:994`), which bypasses the tao event loop entirely, so `⌘Q` can **never** be intercepted while this item stands. `:98` and `:128` are `close_window` (`⌘W`).
- `lib.rs:26` `MENU_CHECK_FOR_UPDATES`; `:67-73` the `MenuItem::with_id` precedent (signature takes `accelerator: Option<A>`); `:208-225` `on_menu_event`, which currently early-returns on any other id.
- `lib.rs:232-253` `generate_handler!`; `:256` `.run(|app_handle, event| match event {`; `:265` `Ready` arm; `:288-295` `Exit` arm → `state.shutdown()`; `:296` `_ => {}`.

Frontend — `src/`:
- `components/dialogs/QuitGuardDialog.tsx` — `:23-27` props (`opIds`, `reason?: "quit" | "update"`); `:39-48` `cancelAll()` — cancels each op, closes, and **only** the `updating` branch does anything after; `:72-77` the two buttons. `:10-12` says the quit trigger is "host wiring outside this unit".
- `components/dialogs/DialogHost.tsx:19-20` — `case "quitGuard":` render.
- `store/ui.ts:22-26` — the `quitGuard` union member; `:115-116` `openDialog`/`closeDialog`.
- `store/operations.ts:133-138` — `activeOps`, the frontend mirror of the predicate.
- `components/shell/UpdateStatusItem.tsx:31-43` — today's only dispatcher (`:36`).
- `lib/ipc/events.ts:143-167` — `subscribeEvents`, `Promise.allSettled` over six `listen()` calls.
- `lib/ipc/client.ts:68-70` / `:132-134` — binding convention (`invoke<T>("name", { args: {...} })`, args omitted when none).
- `lib/ipc/types.ts:319-324` — the six `EVENT_*` constants.
- `lib/ipc/types.test.ts:27-43,55-58` — `GUARDS` map; **`it("covers exactly the committed fixture set")` asserts the fixture directory and `GUARDS` match exactly**, so a new fixture without a new guard entry fails.
- `components/activity/dialogs.test.tsx:63-75`, `components/shell/appUpdate.test.tsx:107-131` — existing dialog tests to model new ones on.

Read-only evidence (verified in `~/.cargo/registry`, do not modify):
- `tauri-2.11.5/src/app.rs:225-232` — `ExitRequested { code, api }`; `code` is `None` "when the exit is requested by user interaction", `Some` "when requested programmatically via `AppHandle::exit`/`restart`".
- `tauri-2.11.5/src/app.rs:86-95` `ExitRequestApi::prevent_exit`; `:101-107` `CloseRequestApi::prevent_close`; `:116-121` `WindowEvent::CloseRequested { api }`.
- `tauri-runtime-wry-2.11.4/src/lib.rs:4438-4463` — `on_close_requested` reads the prevent signal with `rx.try_recv()` **immediately** after the callback, so `prevent_close()` must be called synchronously.
- `tauri-runtime-wry-2.11.4/src/lib.rs:4185-4187` — `Event::LoopDestroyed => callback(RunEvent::Exit)`; `tao-0.35.3/src/platform_impl/macos/app_delegate.rs:132-135` — `applicationWillTerminate:` → `AppState::exit()` → `LoopDestroyed`. This is the OS-shutdown path, and it already reaches the existing kill hook.

## Tasks & Acceptance

**Execution:**

1. `src-tauri/src/commands.rs` -- extract the active-set filter into one `pub(crate) fn active_op_ids(records: &[crate::ipc::OperationRecord]) -> Vec<String>` and rewrite `refuse_app_update_while_busy` to call it; add `pub(crate) fn quit_decision(records: &[..]) -> QuitDecision` (`Allow` | `Block(Vec<String>)`) built on the same function -- one predicate is what makes the two active sets structurally unable to drift (AD-30, FR-21).
2. `src-tauri/src/commands.rs` -- add `#[tauri::command] pub fn confirm_quit(app: tauri::AppHandle)` that calls `app.exit(0)` -- the confirmed-quit sink; exiting programmatically yields `code: Some(0)`, which the backstop deliberately ignores.
3. `src-tauri/src/events.rs` -- add `EVENT_QUIT_REQUESTED = "quit:requested"`, `QuitRequestedEvent { op_ids: Vec<String> }` with `#[serde(rename_all = "camelCase")]`, and the `AppEvent::QuitRequested` variant wired into `name()` and `payload_json()` -- the guard's one refusal signal, on the existing event seam.
4. `src-tauri/src/queue.rs` -- add a `closing: AtomicBool` to `Shared`; set it at the top of `cancel_all` (update its doc); make `try_start_all` return immediately when it is set -- without this, `:969`'s post-`Finished` pump starts queued ops mid-shutdown and their children outlive the app.
5. `src-tauri/src/lib.rs` -- replace `PredefinedMenuItem::quit` at `:91` with `MenuItem::with_id(app, MENU_QUIT, format!("Quit {}", pkg_info.name), true, Some("CmdOrCtrl+Q"))` -- `PredefinedMenuItem::quit` binds `terminate:`, which bypasses the event loop and makes `⌘Q` uninterceptable.
6. `src-tauri/src/lib.rs` -- add the enforcement point `fn on_quit_requested(app: &tauri::AppHandle) -> QuitDecision`: read `AppState`, call `quit_decision(&state.queue.records())`, and on `Block` emit `AppEvent::QuitRequested` through `state.sink` exactly once; return the decision -- one function, one dialog, one refusal.
7. `src-tauri/src/lib.rs` -- route the three triggers as thin adapters: extend `on_menu_event` so `MENU_QUIT` calls it and `app.exit(0)` only on `Allow`; add a `RunEvent::WindowEvent { event: WindowEvent::CloseRequested { api }, .. }` arm that calls `api.prevent_close()` only on `Block`; add a `RunEvent::ExitRequested { code: None, api }` arm that calls `api.prevent_exit()` only on `Block` -- the `code: None` guard is the AD-30 backstop for a future self-deciding path and cannot loop on our own `app.exit(0)`.
8. `src-tauri/src/lib.rs` -- register `commands::confirm_quit` in `generate_handler!` -- required for the frontend binding.
9. `src-tauri/src/ipc.rs` -- add `check("event_quit_requested.json", &QuitRequestedEvent { .. })` to `ipc_contract_matches_committed_fixtures`, then generate the fixture with `PM_UPDATE_CONTRACT=1 cargo test ipc_contract` from `src-tauri/` -- keeps the wire contract asserted on both sides.
10. `src/lib/ipc/types.ts` -- add `EVENT_QUIT_REQUESTED`, the `QuitRequestedEvent` type, and an `isQuitRequestedEvent` guard mirroring the generated fixture -- the TS half of the contract.
11. `src/lib/ipc/types.test.ts` -- add `"event_quit_requested.json": isQuitRequestedEvent` to `GUARDS` -- `covers exactly the committed fixture set` fails otherwise.
12. `src/lib/ipc/client.ts` -- add `export function confirmQuit(): Promise<void>` invoking `"confirm_quit"` with no args -- follows the `installAppUpdate` no-arg precedent.
13. `src/lib/ipc/events.ts` -- add `onQuitRequested(payload)` opening `{ kind: "quitGuard", opIds: payload.opIds, reason: "quit" }` and register it as a seventh `listen()` in `subscribeEvents` -- the only frontend path that opens the quit dialog.
14. `src/components/dialogs/QuitGuardDialog.tsx` -- in `cancelAll()`, give the non-`updating` branch its follow-up action: call `confirmQuit()`, routing failures through `describeError` + `logFrontendEvent` like the update branch; update the stale `:10-12` comment -- makes `Cancel operations and quit` actually quit.
15. `src-tauri/src/commands.rs` -- add a Rust test module covering the I/O matrix's predicate rows: `active_op_ids` over all seven `OpStatus` variants, `quit_decision` returning `Block` with ids in record order, and a **drift test** asserting `refuse_app_update_while_busy(...).is_err() == matches!(quit_decision(...), QuitDecision::Block(_))` for every variant and for mixed lists -- this is the executable form of "the two active sets stay identical".
16. `src-tauri/src/queue.rs` -- add paused-time tests: after `cancel_all`, a pending op never starts and `wait_until_idle` reaches idle; and `wait_until_idle` returns `false` when the bound elapses with an op still running (currently uncovered) -- proves the drain latch and the bounded await.
17. `src/lib/ipc/events.test.ts` or `src/__tests__/events.test.ts` -- assert a `quit:requested` payload opens the quit-guard dialog with `reason: "quit"` and the payload's `opIds` -- the event→dialog limb.
18. `src/components/activity/dialogs.test.tsx` -- add cases: `Cancel operations and quit` cancels each op and invokes `confirm_quit`; `Keep running` closes the dialog and invokes neither `cancel_operation` nor `confirm_quit` -- the choice limb, mirroring `appUpdate.test.tsx:107-131`.

**Acceptance Criteria:**

- Given ≥1 Operation is `Queued` or `Running`, when the user requests a quit by `⌘Q` **or** by window close, then both reach the same enforcement function, the exit is prevented, and exactly one `quit:requested` event carrying those `opIds` is emitted.
- Given the guard is presented, when the user chooses `Cancel operations and quit`, then every listed operation is cancelled and `confirm_quit` runs, and no rollback of partially completed Manager work is offered or performed.
- Given the guard is presented, when the user chooses `Keep running`, then the app stays open, no operation is cancelled, and `confirm_quit` is not invoked.
- Given no Operation is `Queued` or `Running`, when a quit is requested by either path, then no dialog appears and the app exits.
- Given any list of operation records, when both guards evaluate it, then `refuse_app_update_while_busy` refuses exactly when `quit_decision` blocks — asserted over all seven `OpStatus` variants.
- Given one op is `Running` and one is `Queued`, when shutdown begins, then the drain latch prevents the queued op from ever starting, and the shutdown path awaits the bounded idle wait before the process exits.
- Given a quit is OS-initiated, when the app receives it, then no dialog is presented and the existing awaited kill hook runs unchanged.

### Review Findings

- [x] [Review][Patch] Drain operations inside `confirm_quit` before requesting process exit, so Tauri's documented immediate-exit fallback cannot bypass the awaited shutdown hook. [src-tauri/src/commands.rs:821] — fixed; full Rust gate passed
- [x] [Review][Patch] Add a deterministic admission-gate interleaving test that pauses a scheduler pump after its first closing-latch read and proves the second check prevents a queued child from starting. [src-tauri/src/queue.rs:1312] — fixed; deterministic regression and full Rust gate passed
- [x] [Review][Defer] Close the pre-existing app-update check/install admission race so a package operation cannot enter after `refuse_app_update_while_busy` succeeds and before restart shutdown begins. [src-tauri/src/commands.rs:834] — deferred, pre-existing
- [x] [Review][Patch] Keep every refused operation visible even when its local status record is temporarily missing, instead of silently shrinking the list and count. [src/components/dialogs/QuitGuardDialog.tsx:39] — fixed; regression and frontend gates passed
- [x] [Review][Patch] Contain a rejection from the fallback logger after `confirm_quit` itself fails so the recovery path cannot create an unhandled promise rejection. [src/components/dialogs/QuitGuardDialog.tsx:68] — fixed; double-failure regression and frontend gates passed
- [x] [Review][Patch] Narrow the Vitest exclusion to the generated BMAD snippet directory instead of hiding every future test anywhere under `_bmad-output`. [vitest.config.ts:10] — fixed; 144-test suite passed with generated snippets excluded
- [x] [Review][Patch] Add the new `confirm_quit` command and `quit:requested` event to the exact IPC contract documentation. [docs/SPEC.md:481] — fixed
- [x] [Review][Patch] Sequence app-update cancellation and installation through an explicit backend drain before installing. [src/components/dialogs/QuitGuardDialog.tsx:57] — fixed after PR review
- [x] [Review][Patch] Reserve idle scheduler admission atomically so an operation cannot start between the app-update busy check and installation. [src-tauri/src/commands.rs] — fixed after PR review
- [x] [Review][Defer] Make the pre-existing all-or-nothing event subscription recover when one listener registration fails, so a transient failure cannot disable the quit-confirmation surface for the whole session. [src/lib/ipc/events.ts:152] — deferred, pre-existing
- [x] [Review][Patch] Normalize the new deferred-work entries to the ledger's `source_spec` / `summary` / `evidence` schema. [_bmad-output/implementation-artifacts/deferred-work.md:43] — fixed
- [x] [Review][Patch] Remove the superseded untracked TEA red-phase snippets, worker payloads, and result marker instead of committing stale instructions alongside their verified live replacements. [_bmad-output/test-artifacts:1] — fixed; live tests remain authoritative

#### Dismissed

- Edge Case Hunter: frontend quit-event listener missing — dismissed because `onQuitRequested` and the seventh subscription exist in `src/lib/ipc/events.ts`.
- Edge Case Hunter: `confirm_quit` lacks a pending-dialog authorization token — dismissed because the trusted bundled frontend is the only caller and the story explicitly defines this command as the confirmed sink.
- Edge Case Hunter: an operation admitted after an idle quit snapshot is cancelled without appearing in the dialog — dismissed because shutdown closes admission, cancels and drains it; the request-time active set was correctly empty and no child is orphaned.
- Edge Case Hunter: abort exit when the shutdown grace expires — dismissed because the story explicitly requires warning and exit rather than hanging indefinitely.
- Edge Case Hunter: admission-lock acquisition can hang shutdown — dismissed because the guarded scheduler section contains no await or unbounded child work; it only records and spawns already-admitted operations.
- Blind Hunter: TypeScript fixture guard missing — dismissed because `isQuitRequestedEvent` and the exact fixture-map entry exist.
- Blind Hunter: frontend `quit:requested` surface missing — dismissed because the event constant, type, handler, subscription and tests exist in the complete worktree.
- Blind Hunter: frontend `confirm_quit` wrapper/call missing — dismissed because `confirmQuit()` and the dialog follow-up call exist with unit and browser-double coverage.
- Blind Hunter: app update and quit do not share one predicate — dismissed because both call `active_op_ids`; the story requires a structurally shared predicate, not the quit event function on the update path.
- Blind Hunter: idle-quit check/admission race can orphan a child — dismissed because the subsequent shutdown closes admission, captures registered tokens and awaits the drain.
- Blind Hunter: shutdown must abort after the grace bound — dismissed because the specified behavior is to log the timeout and exit without hanging.
- Blind Hunter: detection probes are outside queue cancellation — dismissed from this review because the governing contract deliberately defines the guard over queued/running Operations; expanding that set requires a separate specification decision.
- Blind Hunter: failed event emission leaves no recovery path — dismissed because `TauriSink` records the failure and blocking the quit remains the fail-safe outcome.
- Blind Hunter: submissions remain accepted after the closing latch — dismissed because the process is already irreversibly exiting and the claimed orphan cannot occur; scheduler admission and child starts are closed.
- Blind Hunter: native menu/window wiring has no automated test — dismissed because the story explicitly records this as native manual evidence pending an AD-26-compliant harness.
- Blind Hunter: authoritative IPC documentation/frontend surface missing — dismissed because the complete worktree moves Rust, TypeScript, fixture, wrapper and subscription together.
- Verification Gap Reviewer: quit event and confirmation are not adopted by frontend — dismissed because the reviewer saw only the backend chunk; the complete worktree contains both limbs and their tests.
- Verification Gap Reviewer: native quit-path registration is not automated — dismissed because this is the story's explicitly documented manual macOS evidence gap, not missing ordinary test coverage.
- Verification Gap Reviewer: TypeScript guard-map entry missing — dismissed because it exists in the complete worktree.
- Acceptance Auditor: `cancel_all` sets the latch after taking the gate — dismissed as a product defect because the gate is the linearization point: every earlier start registers its token before cancellation, and every later pump sees the latch; its missing regression proof is retained as the second patch finding.
- Blind Hunter: confirmed-quit failure closes the dialog without visible recovery — dismissed because structured logging is the project's established failure surface here and the story does not specify a retry modal.
- Blind Hunter: a cancellation IPC that never settles can strand the confirmation — dismissed because `cancel_operation` only sends the scheduler message and returns; a permanently hung Tauri transport is outside this command's normal contract.
- Blind Hunter: closing the dialog briefly permits a new operation before confirmed quit — dismissed because the user has already explicitly chosen to quit and the backend shutdown atomically closes admission, cancels, and drains all work without orphaning a child.
- Blind Hunter: malformed native payload can crash dialog rendering — dismissed because the sole producer is the typed Rust event, with its camelCase fixture and TypeScript guard contract verified together; event handlers do not independently validate trusted native payloads.
- Blind Hunter: an empty refusal opens a zero-operation dialog — dismissed because `QuitDecision::Block` is only constructed from a non-empty active-id set, so the native producer cannot emit that state.
- Blind Hunter: a quit request replaces another open modal — dismissed because `DialogHost` is intentionally a single-modal surface and the safety-critical quit decision must become the active modal.
- Blind Hunter: browser coverage does not exercise native menu/window wiring — dismissed because the story explicitly records that as pending native manual evidence under AD-26 rather than claiming browser delivery coverage for it.
- Blind Hunter: OS logout is not covered by browser tests — dismissed because the story explicitly separates this as native/manual evidence and browser tests do not claim it.
- Blind Hunter: cancellation-call order assertion over-specifies concurrency — dismissed because the test asserts synchronous invocation order from the authoritative `opIds` list, not asynchronous completion order.
- Edge Case Hunter: a cancellation or its failure logger can remain pending indefinitely — dismissed because cancellation normally returns immediately after scheduler-message admission and a permanently hung IPC transport is outside the command contract.
- Edge Case Hunter: malformed quit payload reaches the dialog — dismissed because Rust is the only typed producer and the cross-language fixture contract verifies its shape.
- Edge Case Hunter: an empty quit payload reaches the dialog — dismissed because the backend's `Block` constructor requires a non-empty active set.
- Blind Hunter: TEA coverage arithmetic is inconsistent — disposed by removing the superseded pre-implementation bundle rather than publishing stale metadata.
- Blind Hunter: story filename and sprint key use different slugs — dismissed because the manifest maps Story `6-6` explicitly and the loop already resolved and executed this exact artifact; the suggested fix also edits the spec under review.
- Blind Hunter: implementation completion is provisional until the final native build — dismissed as a spec edit; the Auto Run Result already states that the post-review native build remains to be rerun.
- Blind Hunter: native acceptance evidence is pending — dismissed because the story explicitly records that limitation and does not claim the browser double as native evidence.
- Blind Hunter: AC7 lacks a logout procedure — dismissed because adding one edits the spec under review and performing a real logout is outside the safe verification authority for this run.
- Blind Hunter: generated red-phase bundle and landing instructions are stale — disposed by removing the superseded untracked bundle and retaining the integrated live tests.
- Blind Hunter: generated cases are called executable before integration — disposed with the superseded generated bundle.
- Blind Hunter: generated test extensions forced a runner exclusion — disposed by removing those concrete stale files; the narrowed exclusion remains a workflow-safety guard for future temporary generated snippets.
- Blind Hunter: oversized warning lacks a split rationale — dismissed because restructuring the story under review is outside this code-review patch pass.
- Blind Hunter: Review Triage Log is empty — dismissed because populating the spec under review is not an implementation patch and the persisted Review Findings section already records each verdict.
- Blind Hunter: grace-timeout wording conflates SIGTERM resistance with failed finalization — dismissed because correcting it edits the spec under review; runtime behavior is covered by the bounded-wait regression.
- Blind Hunter: bounded exit and absolute child-lifetime language conflict — dismissed because resolving that wording changes the approved intent contract; the implemented behavior follows the explicit warn-and-exit matrix row.
- Blind Hunter: detection probes fall outside the broad child-process wording — dismissed because changing that boundary edits the approved spec; this story's binding active set is queued/running Operations.
- Blind Hunter: event-emission failure has no interactive recovery — dismissed because the approved story specifies fail-safe blocking plus logging and no secondary recovery UX.
- Blind Hunter: manual `ps` guidance can match unrelated processes — dismissed because rewriting the spec's manual procedure is outside this implementation review.
- Blind Hunter: historical TEA result scopes its modifications ambiguously — disposed by removing the superseded result marker.
- Blind Hunter: checked deferred bullets can resemble completed fixes — dismissed because the mandated review format uses checked `Defer` entries to mean triaged into the open ledger, not fixed.
- Acceptance Auditor: generated cancellation-order test can pass too early — disposed with the superseded generated test; the live unit regression explicitly holds both cancellation promises before allowing `confirm_quit`.
- Acceptance Auditor: generated rollback assertion can run after the dialog disappears — disposed with the superseded generated test; the live browser assertion runs while the guard is visible.
- Acceptance Auditor: generated coverage does not exercise `AppState::shutdown` awaiting — disposed with the superseded generated report; the live implementation is verified by queue drain tests and the awaited command path.
- Acceptance Auditor: generated drift-matrix metadata undercounts one list — disposed with the superseded worker payload.

## Spec Change Log

- 2026-08-19: Implemented the shared `Queued | Running` guard, native quit routing, one quit-dialog event path, cancellation-before-confirm behavior, and an awaited bounded shutdown drain. Added Rust, frontend, IPC-contract, and browser-double regression coverage. No rollback behavior was added.
- 2026-08-19: Applied PR #48 follow-up review: added an explicit confirmed-update drain, atomically reserved update admission, and corrected active-operation copy.

## Review Triage Log

- PR #48 Claude Code Review: applied the app-update sequencing suggestion and queued/running copy correction; the final verdict reported no merge blocker.

## Design Notes

**Why `PredefinedMenuItem::quit` must go.** On macOS `muda` binds that item to the `terminate:` selector (`muda/src/platform_impl/macos/mod.rs:994`). `terminate:` goes straight to `applicationWillTerminate:`, which is already past the point of no return — tao has no `applicationShouldTerminate:` hook (verified: no such selector in `tao-0.35.3/src/platform_impl/macos/`). So while that item stands, `⌘Q` cannot be guarded by any amount of Rust or frontend code. A custom `MenuItem` with the `CmdOrCtrl+Q` accelerator routes through `on_menu_event` instead.

**Why the user/OS asymmetry needs no heuristic.** A logout or system shutdown still sends `terminate:` directly to `NSApp`, bypassing the menu item entirely. It therefore lands on `applicationWillTerminate:` → `LoopDestroyed` → `RunEvent::Exit` → `state.shutdown()` — no dialog, awaited kill hook, exactly what AD-30 asks for. The two cases are separated by which OS mechanism fires, not by a guess.

**Why `ExitRequested` is a backstop and not the main hook.** `tauri-runtime-wry` only emits `ExitRequested { code: None }` once the *last window is destroyed* (`lib.rs:4310-4325`) — after a close has already been accepted. `WindowEvent::CloseRequested` is the earlier, preventable hook, so that is the real close adapter. Keeping the `code: None` arm still satisfies AD-30's fourth criterion — a future path that destroys the window on its own is routed rather than deciding for itself — while `code: Some(_)` (our `confirm_quit`, and `app.restart()`) passes through untouched, so no dialog loop is possible.

**Test level — native E2E is deliberately not built here.** `epics.md` names "Real native Tauri E2E for the window-close and `⌘Q` paths" and says AD-26 governs the harness. No such harness exists: `playwright.config.ts:79-97` drives Chromium/WebKit against a plain Vite dev server, `src-tauri/tests/` holds only `live_smoke.rs`, and `tauri::test`/`mock_builder` appear nowhere. The spine records the harness as `OPEN — owner Story 6.5; shape named, not yet adopted`, calls adoption "an AD-20 security-reviewed change", and says the paid CrabNebula alternative "is a procurement decision this spine does not make" (`ARCHITECTURE-SPINE.md:1607`). The in-process alternative is foreclosed too: `tauri = { features = ["test"] }` would mean editing `src-tauri/Cargo.toml`, which `AGENTS.md` forbids. So this story delivers the behavior and covers everything coverable — the predicate, the drift equality, the decision, the drain latch, the bounded await, and the whole frontend limb — and the native window/menu *wiring* stays uncovered until Story 6.5 adopts the harness. That gap is recorded here rather than papered over, and per AD-26 no delivery coverage is claimed from the browser double.

## Verification

**Commands:**
- `cd src-tauri && cargo fmt` -- run before committing; `cargo fmt --check` is CI-enforced
- `cd src-tauri && cargo clippy --all-targets -- -D warnings` -- expected: clean
- `cd src-tauri && cargo test --locked` -- expected: all pass, including the new predicate-drift, drain-latch, and idle-wait-timeout tests
- `cd src-tauri && PM_UPDATE_CONTRACT=1 cargo test ipc_contract` -- run **once**, only after task 9 adds the `check(...)` call, to generate `dev/fixtures/ipc/event_quit_requested.json`; never during a verification run
- `npx tsc --noEmit` -- expected: clean
- `npx vitest run` -- expected: all pass, including `ipc_types_accept_contract_fixtures` with the new fixture
- `npm run build` -- expected: succeeds
- `npm run tauri build -- --debug --no-sign` -- expected: succeeds; the menu change must still compile and bundle

**Manual checks (if no CLI):**
- With an upgrade running, press `⌘Q`: the dialog appears and the app stays open. `Keep running` leaves the operation running; `Cancel operations and quit` ends it and exits. Repeat with the red close button and `⌘W` — identical dialog.
- With nothing running, `⌘Q` quits immediately with no dialog, and the app menu still reads `Quit Pack-Manager` with `⌘Q` shown beside it.
- With an upgrade running, confirm the quit and then check `ps` for surviving `brew`/`npm` children — there must be none.

## Auto Run Result

Status: done
Implementation: complete; no rollback behavior introduced.
Automated verification: `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, and `cargo test --locked` passed after PR follow-up fixes (257 passed, 9 ignored); `npx tsc --noEmit`, `npx vitest run` (144 passed), `npm run build`, `npm run test:e2e:typecheck`, the full two-engine Playwright suite (22 passed), and the focused quit-guard Playwright suite (6 passed) passed; `npm run tauri build -- --debug --no-sign` passed after all review fixes.
Native verification: the real debug app exposed the `Quit Pack-Manager` menu item; idle `⌘Q` and the red close button exited without a guard; `⌘Q` during an isolated read-only Refresh All opened the guard; `Keep running` dismissed it and kept the app open; `Cancel operations and quit` finalized the operations and exited cleanly; a post-exit process check found no surviving Pack-Manager or isolated-operation children. The red close path and `⌘W` share the same source-reviewed adapter. A real macOS logout/terminate was deliberately not invoked because it would disrupt the user session; that path was verified structurally through the shared backend quit predicate and shutdown drain.
Planning baseline revision: `408d1bfdb329357ae11b17cf068ef958fa7f9b6d`
