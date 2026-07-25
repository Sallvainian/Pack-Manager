---
title: Sprint Change Proposal - Finalized UX Contract Realignment
status: approved-applied-to-planning
created: 2026-07-24
updated: 2026-07-24
project: Pack-Manager
workflow: bmad-correct-course
mode: batch
change_classification: moderate
implementation_effort: moderate-to-high
recommended_path: direct-adjustment-with-product-behavior-prerequisite
authoritative_changes_applied: true
source_code_changes_applied: false
---

# Sprint Change Proposal: Finalized UX Contract Realignment

## 0. Proposal authority and safety boundary

This document began as a proposed course correction and became the durable
approval/application record when the user selected Continue in YOLO mode on
2026-07-24. The approved planning changes modify product and planning
authority, but they do not approve the readiness map, authorize an automatic
criterion-status change, modify application source, or claim
product-and-release readiness.

The finalized UX contract was completed after the current PRD, Architecture
Spine, and epic/story breakdown. The purpose of this proposal is to identify
the resulting conflicts, choose a safe path forward, and describe the exact
artifact changes that should be made only after explicit approval.

Current readiness remains:

- planning baseline: **FAIL**;
- historical coverage snapshot: **14/72 P0 criteria FULL**;
- normative coverage map: **final-pending-approval**; and
- no candidate-bound readiness claim.

## 1. Change trigger

### 1.1 What changed

The finalized UX contract makes the Upgrade Plan the central experience object:

> Every Package update, Manager self-update, Manager-wide action, and
> `Update Everything` action is staged in the same reviewable plan. Nothing
> executes from an individual row or Manager header.

It also establishes:

- a persistent, editable Upgrade Plan sidecar;
- a separate final Confirmation Dialog;
- one shared plan-level Activity and Results lifecycle;
- one immutable History entry per confirmed plan attempt;
- plan-linked Retry attempts;
- a Managers disclosure and dedicated Activity destination;
- immediate, atomic Settings persistence;
- removal of the Activity auto-open preference;
- specific high-zoom, keyboard, focus, and VoiceOver behavior; and
- standardized Manager, Package, status, application-update, and failure
  presentation.

### 1.2 Why Correct Course is required

The older planning and product-authority artifacts still require or describe:

- immediate single-Package execution without an Upgrade Plan;
- a modal Upgrade Plan Sheet rather than a persistent editable sidecar;
- a direct Manager self-update command surface;
- an Activity drawer that lists Operations;
- one History row per Operation;
- an `autoOpenDrawer` Setting;
- a flat sidebar Manager list without an Activity destination; and
- evidence that treats immediate row execution as the expected behavior.

These are behavioral and domain-model conflicts, not visual-polish
differences.

### 1.3 Triggering story

No active implementation story triggered this correction. The conflict was
found when the team deliberately returned to the UX workflow after the PRD,
Architecture Spine, epics/stories, and initial readiness assessment had already
been produced.

Checklist item 1.1 is therefore **not applicable**. The trigger is a
post-planning UX finalization.

## 2. Evidence of the conflict

### 2.1 Product authority

`docs/DECISIONS.md` D6 currently says:

> Row-level single-package upgrade executes immediately without the sheet ...
> sheet-for-every-single-row [was rejected].

`docs/SPEC.md` F5 and the current implementation follow that decision.

The finalized UX says:

> No row or header action executes immediately.

### 2.2 Current frontend mechanics

The current implementation:

- builds and immediately executes a one-Package plan from
  `ManagerPane.upgradeRow`;
- stores the Upgrade Plan in modal dialog state;
- has no persistent draft-plan, plan-activity, result-summary, or replay state
  in the UI store;
- supports Dashboard, Manager, History, and Settings views, but no Activity
  destination;
- always mounts an adjustable Activity drawer;
- presents one `SelfUpdateCard` per Manager; and
- renders History directly from Operation records.

### 2.3 Current native and persistence mechanics

The native model has a one-use preview `planId`, but `OperationRecord`,
operation events, transcript metadata, and journal records have no durable
plan-attempt identity.

The current plan request has:

- an optional Package selection;
- one global `includeSelfUpdates` boolean; and
- one greedy-cask boolean.

That shape cannot represent an editable plan in which one Manager self-update
or one Package can be removed independently after `Update Everything`.

The current `operations.jsonl` is simultaneously:

- the crash journal;
- the Operation history store; and
- the source of one History row per Operation.

It cannot reconstruct one immutable History entry per confirmed multi-Manager
plan without new correlation and attempt metadata.

### 2.4 Current Settings mechanics

The current persisted schema contains `autoOpenDrawer` and does not contain the
new upgrade-plan confirmation preference.

### 2.5 Current evidence conflict

The traceability matrix currently counts browser case `AUT-003` as evidence
that a row Upgrade executes immediately without a plan dialog. Under the
finalized UX, that test demonstrates superseded behavior and must not remain
positive evidence.

## 3. Impact analysis

### 3.1 Current sprint impact

No sprint-status or active-sprint artifact was found. This proposal therefore
does not move, cancel, or reschedule an in-progress sprint story.

The correction should be completed before the affected readiness stories enter
implementation. Existing unrelated source work and readiness infrastructure
remain untouched.

### 3.2 Epic impact

| Epic or prerequisite                           | Impact            | Required adjustment                                                                                                                       |
| ---------------------------------------------- | ----------------- | ----------------------------------------------------------------------------------------------------------------------------------------- |
| Epic 1 - `mas` and target-Mac truth            | None              | Preserve as written.                                                                                                                      |
| Epic 2 - detection and refresh                 | Minor             | Preserve behavior; add the finalized System-health and last-good presentation to downstream UI acceptance.                                |
| Epic 3 - Package choice, plans, Settings       | Major within epic | Replace immediate row execution and modal-plan assumptions; add persistent plan membership and confirmation preference.                   |
| Epic 4 - native boundary                       | Moderate          | Carry new plan-attempt, cancellation, Settings, and journal wire changes through the atomic boundary contract.                            |
| Epic 5 - Manager updates and process lifecycle | Moderate          | Stage Manager updates in the plan, add shared Activity/Results semantics, and define plan cancellation and trusted interaction detection. |
| Epic 6 - persistence, History, diagnostics     | Major within epic | Persist plan attempts, group Operations, support replay/Retry links, and preserve honest legacy records.                                  |
| Epic 7 - packaged accessibility and updater    | Moderate          | Validate the finalized navigation, sidecar/high-zoom behavior, confirmation, Manager workspace, Results, and update badge/card.           |
| Epic 8 - candidate evidence                    | Minor             | Consume the revised approved map/profile and updated scenario contracts; preserve all candidate/evidence invariants.                      |

No existing epic becomes unnecessary. The affected epics remain viable after a
new Product Behavior Prerequisite is completed.

### 3.3 Artifact conflicts

| Artifact                                                        | Conflict level              | Reason                                                                                                                             |
| --------------------------------------------------------------- | --------------------------- | ---------------------------------------------------------------------------------------------------------------------------------- |
| Final UX spines and mockups                                     | Clarification only          | They contain the intended direction but leave two runtime policies implicit.                                                       |
| `docs/DECISIONS.md`                                             | Direct conflict             | D6 explicitly rejects the newly selected all-updates-through-plan behavior; D12 and D18 describe old History/navigation semantics. |
| `docs/SPEC.md`                                                  | Direct conflict             | F4/F5/F7/F8/F11 and UI architecture describe the old modal, drawer, Operation-history, and Settings model.                         |
| PRD                                                             | Direct conflict             | FR-6/7/10/11/13/15/16/17/19 and AJ-2 through AJ-5 preserve older interaction semantics.                                            |
| Readiness coverage map                                          | Reconciliation required     | Several legacy criteria refer to immediate row execution, SelfUpdateCard, Operation History, and old Settings behavior.            |
| Architecture Spine                                              | Targeted amendment required | Existing invariants remain sound, but there is no durable plan-attempt identity or plan-level lifecycle boundary.                  |
| Epic/story breakdown                                            | Direct conflict             | It says no additional UX requirements exist and repeats the old behavior in multiple stories.                                      |
| Test design and traceability                                    | Direct conflict             | Scenario descriptions and `AUT-003` treat immediate row execution as correct.                                                      |
| Brownfield architecture/component inventory/implementation plan | Documentation drift         | They accurately describe the current implementation, not the newly approved target behavior.                                       |

## 4. Recommended path forward

### 4.1 Selected path: direct adjustment with a Product Behavior Prerequisite

Create a Product Behavior Prerequisite ahead of the affected readiness work,
then revise the source authorities and downstream planning artifacts in
dependency order.

This path is recommended because:

- the core package-manager adapters, plan builder, scheduler, structured
  command execution, process safety, refresh isolation, updater trust, and
  release-evidence architecture remain valid;
- the change is significant but bounded to experience orchestration,
  correlation, persistence shape, Settings, and acceptance;
- no implemented release candidate or active sprint must be rolled back; and
- trying to implement the old stories first would create evidence for behavior
  the product no longer wants.

### 4.2 Alternatives considered

#### Option A - Continue with the old stories and treat UX as polish

**Rejected.** This would preserve immediate execution and Operation-level
History, directly contradicting the finalized experience.

#### Option B - Roll back the finalized UX to match the current implementation

**Rejected.** The user deliberately selected the plan-centered experience
after detailed workflow review.

#### Option C - Rescope the MVP by removing Activity, Results, History replay,

or confirmation

**Rejected.** These surfaces are part of the trust and recoverability model,
not optional decoration.

## 5. Detailed change proposals

### 5.1 Product decisions and canonical specification

#### Proposal DEC-1 - Supersede immediate row execution

**OLD**

> Row-level single-package upgrade executes immediately without the sheet.

**NEW**

> Every Package update and Manager self-update changes membership in one
> reviewable Upgrade Plan. A row or Manager header never directly executes an
> update. The first addition opens the persistent plan sidecar; removing the
> last item closes it.

**Artifacts**

- Supersede `docs/DECISIONS.md` D6 with a dated decision.
- Update `docs/SPEC.md` F4/F5 and all component/flow descriptions.
- Keep backend exact-plan reconstruction and one-use authorization.

#### Proposal DEC-2 - Make confirmation a separate, reversible safety step

**OLD**

> The Upgrade Plan Sheet itself previews and confirms execution.

**NEW**

> The editable sidecar shows human-readable Package/Manager changes and a
> secondary command disclosure. With confirmation enabled, `Confirm # updates`
> opens a modal that shows the exact commands and final scope. The optional
> skip-future preference appears only inside that dialog and is reversible in
> Settings. When confirmation is off, commands expand automatically and the
> immediate action is labeled `Run # updates`.

**Default**

`skipUpgradePlanConfirmation = false`.

#### Proposal DEC-3 - Make the confirmed plan attempt the durable History unit

**OLD**

> One `operations.jsonl` serves as both crash journal and History store; one
> History row represents one Operation.

**NEW**

> Operation records remain the crash-safe execution evidence, but every
> admitted Upgrade Plan receives a durable `planAttemptId`. One History row
> represents that confirmed attempt and links all of its Operations,
> verification refreshes, reviewed commands, results, and optional
> `retryOfPlanAttemptId`.

Legacy Operation records without a plan-attempt identity remain visible as
clearly labeled legacy Operation entries. Pack-Manager must not fabricate a
multi-Operation plan that was never recorded.

#### Proposal DEC-4 - Replace the old drawer/navigation model

**OLD**

> Flat sidebar Manager items plus History and Settings; a globally mounted,
> adjustable Activity drawer.

**NEW**

> Dashboard, Managers disclosure, Activity, History, and Settings are the
> primary destinations. The contextual right sidecar is absent when empty,
> persists as a draft across Dashboard/Manager navigation, and transforms into
> Activity and Results. Full Activity is a detailed view of the same state,
> not a second execution model.

### 5.2 PRD proposals

#### FR-6 - Selection becomes direct draft-plan membership

**OLD**

> Upgrade selected builds an Upgrade Plan from exactly the selected identities.

**NEW**

> Selecting an eligible Package immediately adds its stable identity to the
> current draft Upgrade Plan; clearing it removes that identity. Header/range
> actions apply to every eligible Package matching the active filter, including
> virtualized off-screen rows. Pinned, current, excluded, and unavailable
> Packages remain inert and explain why.

#### FR-7 - Every update intent uses the plan

**OLD**

> Preview every bulk command exactly.

**NEW**

> Stage every Package update, Manager self-update, Manager-wide update, and
> Update Everything action in the same exact Upgrade Plan. Every staged item is
> removable. Exact commands must be available before execution and must appear
> in the final confirmation step; when confirmation is disabled, they must be
> expanded before `Run # updates` becomes actionable.

#### FR-10 - Replace direct single-Package execution

**OLD**

> Provide a lower-friction row-level update for one eligible Package.

**NEW**

> Provide a one-action way to add or remove one eligible Package from the
> persistent Upgrade Plan. A one-Package plan uses the same review,
> confirmation, stale-plan, admission, execution, verification, Results, and
> History path as a multi-Package plan.

#### FR-11 - Stage Manager self-updates in the Manager header

**OLD**

> Give each Manager an update card that explains installed and latest state,
> Route, and action availability.

**NEW**

> Present Manager installed/latest state, Route explanation, action
> availability, and optional self-update delta in the Manager Card/Header.
> `Update Manager` adds that Manager self-update to the Upgrade Plan; `IN PLAN`
> and `Remove` reflect its draft membership. The action never executes directly.

#### FR-13 - Add plan-level Activity and Results

**OLD**

> Expose queued, running, stalled, cancelling, and terminal Operation state
> with exact command and live output.

**NEW**

> Expose one shared confirmed-plan Activity model in the contextual sidecar and
> full Activity destination. Present human-readable Package/Manager progress
> first and exact command/output as secondary evidence. Transform the sidecar
> into a persistent Results Summary after all plan Operations and required
> verification refreshes reach terminal state.

#### FR-14 - Define plan cancellation and trusted interaction detection

**ADD**

- One active confirmed Upgrade Plan attempt is allowed at a time.
- `Cancel plan` cancels every still-running Operation belonging to that
  attempt, prevents queued attempt Operations from starting, and records those
  not started as `Skipped`.
- Cancellation remains immediate and has no second confirmation.
- No rollback is promised.
- `Interaction required` is presented only when a closed, adapter-specific
  classifier recognizes a supported prompt signature.
- Unrecognized silence remains `Stalled`; arbitrary output is never guessed to
  be a prompt.

#### FR-15 - Persist one History entry per confirmed attempt

**OLD**

> Preserve Operation History, transcripts, and crash evidence.

**NEW**

> Preserve Operation-level transcripts and crash evidence while grouping every
> admitted Upgrade Plan into one immutable History attempt. Replay reconstructs
> Manager groups, Package/version changes, commands, outcomes, timing, errors,
> and retained output. A confirmed Retry creates a new linked attempt and never
> rewrites the original.

#### FR-16 - Require verification before success

**ADD**

- A zero exit becomes `Verifying`, not success.
- Success is declared only after affected Manager state refreshes and verifies
  the intended result.
- Results provide `What happened`, `What to do next`, evidence, contextual
  actions, and a secondary user-controlled Retry.
- Retry scope is reviewable before it becomes a new draft.

#### FR-17 - Update Settings

**OLD**

- mutation Activity auto-open is configurable;
- `autoOpenDrawer` defaults to true.

**NEW**

- remove the Activity auto-open preference;
- add `skipUpgradePlanConfirmation`, default false;
- preserve editable stall threshold, hard cap, and log level;
- save every control immediately and atomically with visible
  `Saving`/`Saved`/failure state.

#### FR-19 / NFR-3 / NFR-6 - Bind the finalized shell and accessibility model

**ADD OR STRENGTHEN**

- Managers is a disclosure, not a redundant permanent Dashboard list.
- Activity is a primary destination.
- Status, versions, Package health, and Manager self-state remain distinct.
- Package Grid uses one roving row focus, stable virtual identity, total/row
  metadata, exact filter-wide bulk scope, and final-row reachability.
- At 150-200% zoom in the 900 x 600 minimum, navigation collapses and
  Plan/Activity/Results become full-workspace or stacked surfaces without
  overlap or two-dimensional scrolling for the primary task.
- Apply the finalized focus-transition and announcement contracts.

#### Acceptance journeys

Revise AJ-2 through AJ-5 to match the finalized UX flows verbatim:

- AJ-2: editable plan -> Confirmation Dialog -> Activity -> Results -> one
  History attempt;
- AJ-3: row/header actions add to the plan and never execute immediately;
- AJ-4: shared Activity, trusted interaction detection, plan cancellation,
  Results diagnosis, and linked Retry;
- AJ-5: one History Plan Row and read-only Activity replay.

AJ-1 and AJ-6 retain their product purpose and receive the finalized visual and
navigation wording.

### 5.3 Readiness coverage-map proposal

#### Rules

1. Preserve exactly 72 legacy P0 criterion IDs and the historical baseline
   statuses as planning history.
2. Increment `map_revision`.
3. Update consequence wording only where the approved product behavior changed.
4. Run TIR-1 behavior-present reconciliation after the source authorities are
   updated.
5. Recompute the provisional Product Behavior / Test Infrastructure / Release
   Evidence split. Do not carry `1/52/5` forward automatically.
6. Require a new mechanical validation summary plus Product and QA approval.
7. Preserve the prior map revision in source control.

#### Rows requiring review

| Criterion | Proposed consequence adjustment                                                                                                          |
| --------- | ---------------------------------------------------------------------------------------------------------------------------------------- |
| `F3-AC1`  | Package rows expose direct Upgrade Plan membership rather than immediate row execution.                                                  |
| `F3-AC3`  | Pinned controls remain focusable/inert with an accessible explanation and never enter a draft.                                           |
| `F3-AC6`  | Manager self-state is hoisted into Manager Card/Header plan semantics rather than an independently executing card.                       |
| `F4-AC1`  | Every entry point, including one Package and one Manager self-update, updates the persistent plan or reports failure.                    |
| `F4-AC2`  | The plan and final authorization expose exact commands, exclusions, warnings, notes, and editable scope.                                 |
| `F4-AC3`  | Replace the global all-self-updates toggle expectation with explicit per-Manager self-update membership.                                 |
| `F5-AC1`  | Keyboard/range/header interactions directly control plan membership across the full active filter.                                       |
| `F5-AC2`  | Exact PackageRefs and Manager self-update identities reach final admission; draft membership clears only when the plan becomes Activity. |
| `F5-AC3`  | One eligible Package is staged, reviewed, confirmed, executed, verified, and recorded through the common plan lifecycle.                 |
| `F6-AC3`  | Manager header/card exposes Route, unavailable, queued, and `IN PLAN` states.                                                            |
| `F7-AC2`  | Sidecar and full Activity share one plan model and transform into Results without losing Operation evidence.                             |
| `F7-AC3`  | Plan cancellation cancels running attempt work and skips queued attempt work while preserving evidence.                                  |
| `F7-AC4`  | Null-input stall behavior plus trusted prompt classification and noninteractive handoff are proven.                                      |
| `F8-AC2`  | Durable records preserve both Operation crash truth and plan-attempt correlation.                                                        |
| `F8-AC4`  | History represents confirmed plan attempts and opens read-only Activity replay with native evidence actions.                             |
| `F10-AC1` | Packaged UI validates the finalized shell, sidecar lifecycle, focus matrix, grid semantics, and 100/150/200% zoom behavior.              |
| `F11-AC2` | Every current Setting, including confirmation preference and removal/migration of `autoOpenDrawer`, validates and persists.              |
| `D25-AC2` | Pack-Manager update readiness uses the finalized badge/card presentation without entering Package Activity/History.                      |

`AUT-003` must be retained as historical evidence of superseded behavior or
replaced, but it must not support the revised `F5-AC3`.

### 5.4 Architecture proposal

#### Preserve AD-1 through AD-15

The existing separation of Product Behavior, reusable infrastructure, and
candidate evidence remains valid. The Tauri monolith, typed ports, atomic
boundary catalog, evidence lanes, Candidate Manifest, and Evidence Index do not
need replacement.

#### Add AD-16 - A confirmed Plan Attempt is the durable coordination boundary

**Status:** proposed

**Binds:** FR-6 through FR-16, F4/F5/F7/F8, ASR-01, ASR-03, TIR-3, TIR-5

**Rules**

1. `planId` remains a bounded one-use authorization for one reviewed preview.
   It is not a durable History identity.
2. Successful atomic admission creates one new `planAttemptId`.
3. `execute_plan` returns `planAttemptId` plus the created Operation IDs.
4. Every Operation produced by that admission carries the same
   `planAttemptId` through:
   - Rust and TypeScript wire models;
   - `op:status`, `op:output`, and attention events;
   - transcript metadata;
   - crash-journal start/finish records;
   - in-memory stores; and
   - diagnostics.
5. The admitted attempt stores the reviewed Manager/Package scope, Manager
   self-updates, exact commands, version evidence, timestamps, result state,
   verification state, and optional `retryOfPlanAttemptId`.
6. A plan attempt becomes terminal only after its mutations and required
   verification refreshes reach terminal state.
7. A successful process exit is not equivalent to a verified item.
8. Plan cancellation operates only on Operation IDs bound to that attempt.
9. Legacy Operation records without `planAttemptId` remain honest legacy
   entries and are never silently grouped.
10. Only one confirmed Upgrade Plan attempt may be active at a time. The
    scheduler still permits safe cross-Manager concurrency inside that attempt.

#### Plan-intent contract

Replace the insufficient global self-update toggle with an explicit canonical
intent capable of representing:

- `AllEligible` before editing; or
- `Explicit` PackageRefs plus explicit Manager self-update identities after
  editing;
- greedy/self-updating Package policy; and
- user-visible exclusions.

After `Update Everything`, removing any item converts the draft into an
explicit intent and rebuilds the authenticated preview from the backend. The
frontend never edits executable command text.

#### Native boundary changes

Coordinate these as one AD-3 contract change:

- revised plan request/response;
- plan-attempt response and records;
- Operation `planAttemptId`;
- plan-attempt query/list/replay contract;
- `cancel_plan_attempt`;
- trusted interaction-attention event or typed attention reason;
- revised Settings fields;
- Rust/TypeScript types and guards;
- shared IPC fixtures;
- production registration/wrappers/subscriptions; and
- native acceptance vectors.

The exact command count and event count are allowed to change; the boundary
catalog already treats 20 commands and six events as a baseline rather than a
permanent invariant.

#### Persistence and migration

Prefer one append-only persistence family that keeps:

- immutable plan-admission metadata;
- Operation start/finish facts;
- verification facts; and
- Retry links.

The detailed schema is an architecture follow-up, but it must preserve:

- crash reconstruction;
- first-failure evidence;
- atomic compaction;
- legacy readability;
- newest-1,000 plan-attempt retention semantics without prematurely deleting
  referenced Operation evidence; and
- diagnostics privacy and symlink protections.

#### Settings migration

- Add `skipUpgradePlanConfirmation`, default false.
- Remove `autoOpenDrawer` from active Settings.
- Tolerate an old persisted `autoOpenDrawer` value during migration without
  making it active.
- Apply the new value only after atomic persistence succeeds.

#### Interaction-required policy

Use a closed adapter-specific prompt classifier or another explicit native
signal. Arbitrary regex/heuristic prompt guessing is prohibited. When no trusted
classification exists, the existing null-input stall path remains authoritative.

### 5.5 Epic and story proposal

#### Add: Product Behavior Prerequisite - Implement the Finalized UX Contract

This prerequisite belongs before the affected readiness stories. Its stories
have `Product Behavior` as their primary concern so evidence stories do not
silently absorb implementation work.

##### Story UX-PB.1 - Build the persistent editable Upgrade Plan

**Acceptance**

- Every eligible Package and Manager self-update can be added/removed without
  execution.
- Update Everything seeds all eligible work and remains editable.
- The sidecar opens on first addition, closes when empty, and persists across
  Dashboard/Manager navigation.
- Every draft mutation rebuilds an authenticated preview from canonical
  identities.
- Pinned, current, excluded, and unavailable controls remain inert and explain
  why.

##### Story UX-PB.2 - Add final confirmation and its atomic preference

**Acceptance**

- With confirmation enabled, the plan has one `Confirm # updates` action that
  opens the final modal.
- The modal shows exact commands, `Change Plan`, final confirmation, and the
  skip-future preference.
- The skip preference does not appear on the base plan.
- With confirmation disabled, commands expand automatically and the action is
  `Run # updates`.
- Persistence failure retains the prior preference and shows an inline error.

##### Story UX-PB.3 - Correlate and present one live plan attempt

**Acceptance**

- Admission returns one durable `planAttemptId`.
- Sidecar and full Activity render the same attempt.
- Operation state, locks, output, attention, and cancellation remain correlated.
- Only trusted prompt classification creates `Interaction required`.
- Plan cancellation cancels running work, skips queued work, and promises no
  rollback.
- Successful exits remain `Verifying` until affected state confirms the result.
- The sidecar transforms into persistent Results.

##### Story UX-PB.4 - Persist plan History, replay, and linked Retry

**Acceptance**

- One confirmed attempt creates one History row.
- Replay reconstructs Manager groups, Packages, commands, timing, outcomes,
  errors, and retained output.
- Results remains until `Done`.
- Retry first shows scope, then creates a new draft and new linked attempt if
  confirmed.
- Legacy Operations remain clearly labeled and readable without fabricated
  grouping.

##### Story UX-PB.5 - Align the accessible shell and Manager workspaces

**Acceptance**

- Sidebar contains Dashboard, Managers disclosure, Activity, History, and
  Settings with coherent outlined icons.
- Manager Header/Card semantics match the finalized descriptions, paths,
  version/status, Package counts, health meter, and self-update placement.
- Package Grid, focus transitions, announcements, explanatory unavailable
  controls, and high-zoom behavior match the UX spines.
- Settings removes Activity auto-open and keeps editable advanced values.
- Application update detail lives in Settings with the
  `Pack-Manager Update Ready!` badge and one-line yellow-to-green version delta.

#### Modify existing stories

| Story | Proposed change                                                                                                                                  |
| ----- | ------------------------------------------------------------------------------------------------------------------------------------------------ |
| 3.1   | Replace row Upgrade/SelfUpdateCard assertions with Package plan-membership and Manager Header/Card semantics.                                    |
| 3.2   | Assert explanatory unavailable controls across every draft-entry path and filter.                                                                |
| 3.3   | Cover all four entry classes and persistent editable plan behavior.                                                                              |
| 3.4   | Add confirmation preference; remove `autoOpenDrawer`; verify immediate atomic save states.                                                       |
| 3.5   | Replace immediate execution with keyboard-accessible direct plan membership and common plan lifecycle.                                           |
| 3.6   | Replace global Manager-self-update toggle assumptions with explicit Manager membership while preserving rust dedup and stale reconfirmation.     |
| 4.1   | Include every revised command/type/event/fixture in the boundary catalog change.                                                                 |
| 4.6   | Revalidate exact admission using `planAttemptId` and the separate confirmation step.                                                             |
| 5.2   | Validate Manager Header/Card `IN PLAN`, Remove, Route, unavailable, and queue states.                                                            |
| 5.4   | Validate shared sidecar/full Activity, bounded evidence, verification, and Results transformation.                                               |
| 5.5   | Add plan cancellation, queued-item Skip, and trusted interaction-required behavior.                                                              |
| 6.3   | Preserve plan-admission, Operation, verification, and Retry correlation through persistence.                                                     |
| 6.4   | Replace Operation-table History acceptance with plan-attempt History and Activity replay; retain legacy honesty and PGID non-signal.             |
| 6.5   | Ensure diagnostics exports the revised attempt/Operation evidence without losing required data.                                                  |
| 6.7   | Validate Settings migration, confirmation preference, and removal of Activity auto-open.                                                         |
| 7.6   | Validate the complete finalized shell at 100/150/200% zoom, Package Grid semantics, dialog focus, Activity/Results, and VoiceOver announcements. |
| 7.7   | Validate the finalized application-update badge/card presentation in addition to updater state truth.                                            |
| 7.10  | Preserve application-update separation from draft plans, live attempts, Results, and plan History.                                               |
| 8.7   | Freeze and replay only the revised approved map/profile and updated scenario-contract digests.                                                   |

### 5.6 Test-design and traceability proposal

#### Test-design changes

- Update Batch 3 scenarios for persistent plan membership, confirmation, and
  Settings migration.
- Update Batch 4 boundary vectors for the new request, attempt, cancellation,
  replay, event, and Settings types.
- Update Batch 5 scenarios for shared Activity/Results, verification, trusted
  prompt classification, and plan cancellation.
- Update Batch 6 scenarios for plan-attempt persistence, legacy migration,
  History replay, Retry linkage, diagnostics, and crash reconstruction.
- Expand Batch 7 packaged acceptance to all four authoritative UX mockup
  surfaces and the complete focus/zoom contracts.
- Preserve all lane/provenance/candidate rules.

#### Traceability changes

- Mark the existing immediate-row `AUT-003` as superseded behavior.
- Do not delete its historical record when regenerating traceability; explain
  why it no longer supports the revised criterion.
- Add browser and native cases for the one-Package common plan path.
- Re-run behavior-present classification before assigning regression credit.
- Regenerate the matrix only through the appropriate later workflow; this
  proposal does not change any status.

### 5.7 UX-spine clarification proposal

The finalized UX remains the selected design. Add two narrow clarifications:

1. **One active confirmed attempt:** A draft may not be confirmed while another
   Upgrade Plan attempt is active. Cross-Manager concurrency occurs inside the
   active attempt.
2. **Trusted interaction classification:** `Interaction required` appears only
   from a closed Manager-specific classifier or explicit native signal.
   Otherwise a silent null-input command follows the ordinary stall path.

Also make the cancellation label consistently `Cancel plan` when the
consequence affects the entire attempt, while retaining `Cancel operation` only
for a deliberately Operation-scoped diagnostic action.

### 5.8 Brownfield documentation proposal

After product authority and architecture are approved, update:

- `docs/architecture.md`;
- `docs/component-inventory.md`;
- `docs/IMPL_PLAN.md`; and
- any generated project-context summary.

These documents should describe the implemented target only when the
corresponding product work exists. Until then, clearly distinguish current
mechanics from approved target behavior.

## 6. Sequencing and dependencies

```text
Approve this course correction
  -> supersede product decisions and update SPEC
  -> revise PRD and coverage-map wording
  -> approve AD-16 and the native/persistence contracts
  -> apply the two UX clarifications
  -> add the Product Behavior Prerequisite and revise affected stories
  -> reconcile test design and scenario contracts
  -> rerun implementation-readiness assessment
  -> implement Product Behavior stories
  -> execute affected evidence stories
  -> later regenerate traceability against actual evidence
```

Do not implement downstream evidence stories against the old immediate-row,
Operation-history, or drawer assumptions.

## 7. Scope, effort, and risk

### 7.1 Classification

- **Scope classification:** Moderate course correction.
- **Implementation effort:** Moderate-to-high.
- **Architecture involvement:** Required.
- **MVP impact:** No fundamental product rescope.
- **Release-evidence architecture:** Preserved.

### 7.2 Relative effort

| Area                           | Relative effort  | Why                                                                                                                |
| ------------------------------ | ---------------- | ------------------------------------------------------------------------------------------------------------------ |
| Planning/source reconciliation | Moderate         | Multiple authorities and mapped acceptance rows must remain coherent.                                              |
| Frontend experience/state      | High             | Persistent draft, sidecar lifecycle, new navigation, confirmation, Activity/Results, replay, focus, and high zoom. |
| Native wire/domain model       | Moderate         | Plan-attempt identity, explicit Manager update membership, cancellation, replay, and Settings changes.             |
| Persistence/migration          | Moderate-to-high | Plan grouping, verification, Retry links, crash truth, compaction, and legacy honesty.                             |
| Test/evidence updates          | High             | Existing positive evidence is superseded and several layers require new scenarios.                                 |
| Release pipeline               | Low              | Candidate/evidence rules stay intact; only frozen inputs and scenarios change.                                     |

No calendar estimate is assigned because the existing planning artifacts have
no named assignees or dates.

### 7.3 Primary risks and mitigations

| Risk                                                              | Mitigation                                                                                                    |
| ----------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------- |
| Preview `planId` is mistakenly reused as durable History identity | Keep `planId` and `planAttemptId` explicitly separate in names, types, schemas, and tests.                    |
| Editable Update Everything silently changes executable text       | Store canonical identities only; rebuild every preview in Rust.                                               |
| Plan-level cancellation leaves queued work runnable               | Backend cancellation targets the full attempt identity and terminally marks unstarted work `Skipped`.         |
| Success appears before refresh verification                       | Model `Verifying` explicitly and finish Results only after required refresh outcomes.                         |
| Legacy journal records are falsely grouped                        | Display them as legacy Operations unless a recorded attempt identity exists.                                  |
| Prompt detection fabricates meaning                               | Use closed adapter-specific classifiers; default to ordinary stall.                                           |
| Settings migration changes safety preference unexpectedly         | Default confirmation to enabled and ignore obsolete `autoOpenDrawer` as inactive legacy input.                |
| Historical FULL evidence is carried into changed behavior         | Preserve statuses as baseline history only; require new map/profile/scenario admission.                       |
| UX work delays readiness closure                                  | Complete the Product Behavior Prerequisite before affected evidence stories so testing is not authored twice. |

## 8. Handoff

If approved:

- **Product Owner:** approve the behavior change and superseding decisions.
- **UX:** approve the two runtime clarifications.
- **Architecture:** define AD-16, plan-intent, plan-attempt, persistence, and
  boundary contracts.
- **Development:** implement the Product Behavior Prerequisite.
- **QA/Test Architecture:** revise the coverage map, behavior-present
  classification, scenario contracts, and downstream evidence plans.
- **Release:** retain existing candidate identity, updater, and evidence-ledger
  requirements; consume only the later approved map/profile.

The implementation-readiness assessment must be rerun after the authoritative
artifacts and epic/story breakdown are reconciled. Approval of this proposal is
not an implementation-readiness PASS.

## 9. Approval record

- **Decision:** Continue `[c]`.
- **Execution mode:** YOLO; complete the required workflow stages using best
  judgment without pausing at each menu.
- **Approved on:** 2026-07-24.
- **Approved path:** Direct adjustment with a Product Behavior Prerequisite.
- **Application boundary:** Planning, UX, architecture, test-planning, project
  context, and brownfield documentation only.
- **Application source code:** Unchanged.
- **Criterion/readiness status:** Unchanged by approval; a separate readiness
  rerun is required.

## Appendix A - Correct Course checklist result

| Checklist area             | Result                                                                                                   |
| -------------------------- | -------------------------------------------------------------------------------------------------------- |
| Trigger and context        | Complete; post-planning UX finalization confirmed.                                                       |
| Current story assessment   | Not applicable; no triggering implementation story.                                                      |
| Epic impact                | Complete; Epics 3-7 materially affected, Epics 1/2/8 preserved with bounded adjustments.                 |
| Artifact conflict analysis | Complete; product authority, PRD, map, architecture, epics, test design, and brownfield docs identified. |
| Path evaluation            | Direct adjustment selected; rollback and MVP rescope rejected.                                           |
| Proposal components        | Complete; issue, impact, old/new changes, path, effort, risk, and handoff included.                      |
| Approval                   | Complete; user selected Continue in YOLO mode on 2026-07-24.                                             |

## 10. Application record

### 10.1 Applied authoritative changes

- Added Decisions D27-D30 and the explicit supersession boundary in
  `docs/DECISIONS.md`.
- Revised the canonical update-experience requirements in `docs/SPEC.md`.
- Revised the PRD and added revision-2 coverage-map interpretation without
  carrying historical evidence into the changed behavior.
- Adopted AD-16 in the Architecture Spine, separating editable `PlanIntent`,
  one-use preview `planId`, and durable `planAttemptId`.
- Finalized runtime UX clarification for one active attempt, trusted
  interaction classification, and plan-versus-operation cancellation.
- Added the approved Product Behavior Prerequisite and Correct Course story
  amendment to `epics.md`.
- Reconciled system/QA/progress test-design artifacts, the test handoff, and
  traceability wording while retaining the historical FAIL gate.
- Updated project context and brownfield architecture/component/implementation
  documents to distinguish the current implementation from the approved
  target.

### 10.2 Readiness rerun

The formal rerun is recorded in
`implementation-readiness-report-2026-07-24.md`.

Result: **NOT READY**.

The PRD, UX, and architecture are aligned and FR traceability is 22/22. Direct
implementation handoff remains blocked because affected local story contracts
still contain superseded wording, UX-PB.1..UX-PB.5 require decomposition, all
stories remain unassigned/undated, scenario contracts are absent, and declared
governance/execution dependencies remain open.

This outcome completes the approved Correct Course workflow honestly. It does
not invalidate the UX direction; it identifies story repair and decomposition
as the next planning action.
