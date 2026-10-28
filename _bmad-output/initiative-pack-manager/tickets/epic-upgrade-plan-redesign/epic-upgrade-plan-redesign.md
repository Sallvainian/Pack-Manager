---
type: epic
title: "The Upgrade Plan redesign (D27–D30)"
parent: initiative-pack-manager
covers: [R8, R23]
sources: [_bmad-output/initiative-pack-manager/archive-v6/spec-upgrade-plan-redesign/SPEC.md, _bmad-output/initiative-pack-manager/archive-v6/epics.md]
companions: [_bmad-output/initiative-pack-manager/tickets/epic-upgrade-plan-redesign/capability-story-map.md, _bmad-output/initiative-pack-manager/architecture-Pack-Manager-2026-07-23/architecture-Pack-Manager-2026-07-23.md, _bmad-output/initiative-pack-manager/ux-Pack-Manager-2026-07-23/DESIGN.md, _bmad-output/initiative-pack-manager/ux-Pack-Manager-2026-07-23/EXPERIENCE.md]
after: []
risk: high
---

# The Upgrade Plan redesign (D27–D30)

## Description

Decisions D27–D30 (2026-07-24) redesigned the entire upgrade experience — one persistent editable Upgrade Plan, a separate final confirmation gate, a durable plan attempt as the Activity/Results/History unit, and verification-gated success — and they are decided but unimplemented. `planAttemptId`, `Verifying`, `InteractionRequired`, `skipUpgradePlanConfirmation`, and `PlanIntent` occur zero times in `src/` and `src-tauri/src/`; the shipping build still executes a single-row upgrade immediately with no confirmation sheet. This epic's 28 stories (v6 Epic UX-PB, all backlog) realize them. The product promise at stake is SM-2: no Package or Manager update ever runs that the user did not see staged first — a single violation is a P0 defect.

## Outcome

Sallvain stages a mixed plan (Packages plus a Manager self-update) from several entry points, confirms once in `Proceed with Upgrade Plan?`, watches the same sidecar become live Activity and then verification-gated Results, and finds exactly one immutable History row that replays read-only — while `grep -r "planAttemptId" src/ src-tauri/src/` returns real implementations and no unstaged mutation path remains in the product.

## Requirements

R1 (CAP-1; parent R23): Persistent draft plan membership — every update entry point (Package row checkbox, count-labeled header checkbox, Manager-header `Update Manager`, Manager-wide action, `Update Everything`) edits one persistent draft Upgrade Plan of canonical identities, with Rust rebuilding the exact commands on every mutation; nothing executes from any entry point (D27; AD-16/17/23/24/28). Success: all four immediate-execution call sites (row action, both self-update paths, `Run fix`) are gone; toggling an eligible row round-trips through the Rust rebuild into draft membership; the e2e assertion expecting immediate row execution is rewritten to expect membership.
R2 (CAP-2): Sidecar lifecycle and navigation persistence — the Upgrade Sidecar opens on the first staged item, persists unchanged across Dashboard/Manager navigation, closes on last removal, and is session-scoped, never written to disk. Success: navigating away and back preserves exact membership; relaunch after quit, crash, or force-quit starts with an empty draft, hidden sidecar, and nothing executing.
R3 (CAP-3): Ineligible-control inertness with a pointer-reachable reason — pinned, current, excluded, and unavailable Package controls stay inert on every activation path and explain themselves on pointer interaction, using `aria-disabled="true"`, never native `disabled`, never gray styling alone (D38; AD-16). Success: hovering or clicking each ineligible kind renders its exact plain-language reason; no activation path changes membership; the header checkbox denominator covers only eligible rows matching the active filter including off-screen ones.
R4 (CAP-4): Standardized Manager workspace presentation — every Manager Header and Card shows standardized description, path, installed version, status, ownership, counts in `34 managed packages · 8 package updates` order, and self-update delta; a staged self-update shows `IN PLAN` plus a named `Remove` action (AD-25). Success: all six Managers render the standardized identity area; `Update Manager` stages and never executes; a failed refresh retains the timestamped last-good snapshot with `Retry refresh` and no invented health value.
R5 (CAP-5; parent R8): Distinct identities and atomic single-attempt admission — the one-use preview `planId` and the durable `planAttemptId` are separate, non-interchangeable branded types across Rust wire, domain, persistence, and TypeScript; `execute_plan` atomically mints exactly one attempt, and a second confirmation fails closed while one is active (D29, D30). Success: type-level tests reject cross-assignment at every boundary; admission returns one `planAttemptId` plus Operation ids all-or-none; a concurrent second confirmation mints nothing while cross-Manager concurrency continues inside the active attempt.
R6 (CAP-6): Durable attempt records and correlation — a confirmed attempt persists exactly two append-only records, admission (reviewed scope, exact command snapshot, timestamps) at mint and terminal (verification facts, results) on the terminal transition, and every Operation, event, transcript, and durable record carries the same `planAttemptId` (AD-18, AD-29). Success: the attempt fold reconstructs admission+terminal → outcome and admission-only → `Interrupted`; no persisted `PlanAttempt.state` field exists; a snapshot read back corrupted is refused as an execution source; append failure is surfaced, never fatal to admission.
R7 (CAP-7): Plan-level cancellation with two labeled scopes — `Cancel plan` stops only that attempt: unstarted work becomes `Skipped`, running work escalates through existing process-group mechanics straight to terminal state, with no second confirmation and no rollback promise; `Cancel operation` is reserved for a deliberately Operation-scoped diagnostic (D30; AD-16). Success: no `Cancelling` state exists at any level and `OpStatus` gains no variant; partially failed escalation reports honestly rather than fabricating outcomes; the terminal record lands in the same critical section as the transition.
R8 (CAP-8): Legacy Operation honesty — records without a `planAttemptId` remain readable, individually labeled legacy Operations, never grouped or inferred into plan attempts (AD-18/29). Success: a mixed History renders legacy entries visibly distinct with Operation-level detail; no fabricated grouping appears for plan-like legacy records.
R9 (CAP-9): Sidecar as live summary, full Activity as deep view — on admission the same sidecar transforms in place into the one live summary; full Activity is a detailed view of the same `planAttemptId`; each item shows honest queued/waiting/running/verifying/terminal state, with safety-critical state (stall handoff, `Interaction required`) reaching the user visibly, never via an announcement channel (AD-17). Success: no second surface opens on admission; sidecar and Activity render one shared state; stream disconnect keeps last honest state and surfaces the interruption; a status update never moves focus.
R10 (CAP-10): Verification-gated Results with failure guidance — the attempt becomes Results only after affected-Manager refresh verification; outcomes use the closed taxonomy (attempt: success/partial/failed/cancelled/timed out/interrupted; item: verified/failed/cancelled/skipped); failed items present `What happened` and `What to do next` before a secondary Retry (AD-16/25/29). Success: a successful exit stays `Verifying` until verification resolves and is never colored successful on exit code alone; a failed verification refresh leaves the Last-good Snapshot in place; deterministic failures are not framed as retry-fixable.
R11 (CAP-11): Trusted Interaction-required classification — `Interaction required` appears only when a closed Manager-specific classifier or explicit native signal recognizes a known prompt; all other null-stdin silence follows the ordinary 120-second stall path (D30). Success: unmatched output at the stall threshold shows exactly `Keep waiting` / `Copy command` / `Cancel plan`; a recognized prompt is never left as a silent stall; no regex or heuristic guessing converts one into the other.
R12 (CAP-12): One immutable History row per attempt, with replay — each confirmed attempt folds to exactly one immutable History row with nested Operation evidence and verified-outcome wording; opening it routes Activity into read-only replay; a replay opened during live work stays clearly secondary to the live attempt (AD-29). Success: the fold is idempotent by `planAttemptId` — duplicate records never replay as extra rows; replay mutates nothing; `Interrupted` requires genuine record absence, with unreadable terminal records reported as unreadable evidence.
R13 (CAP-13): Retry as a new linked attempt — Retry first reveals the failed-item scope, then `Create new plan` composes a derived `RetryIntent` in Rust, restricted to failed members and canonically rebuilt, that goes straight to preview and confirmation without touching the persistent draft; confirming mints a fresh `planAttemptId` linked by `retryOfPlanAttemptId` (AD-24). Success: the original failed result stays immutable and reachable via `View previous result`; a rebuild failure (item now pinned/current/removed) is explained and admits nothing; dangling lineage is surfaced, never repaired silently.
R14 (CAP-14; parent R8): Separate confirmation gate with a reversible skip — `Confirm N Updates` opens the `Proceed with Upgrade Plan?` dialog showing exact commands with `Change Plan` and final confirmation; the disable control exists only inside that dialog, persists `skipUpgradePlanConfirmation` atomically only after admission succeeds, and Settings can reverse it; the disabled path auto-expands commands, shows a persistent `Confirmation is off` warning, relabels to `Run N Updates`, and keeps the native rebuild and stale check (D28; AD-21/22). Success: nothing executes before final confirmation (or the explicit `Run N Updates`); the rider never precedes its admission and a failed save leaves `false` active and persisted; the setting is plan-inert so persisting it never expires the plan it rode on; stale validation blocks the bypass run until re-review.
R15 (CAP-15): Safety surfaces at the size and zoom floors — Plan, Confirmation, Activity, and Results stay fully visible and operable at 900 × 600 and 150–200% zoom, entering the high-zoom stacked layout below 720 usable CSS pixels with a visible Back route and a persistent non-occludable indicator for safety-critical attempt state (NFR-3; AD-17). Success: every safety action keeps a visible AD-27 outline focus state verified at runtime; no safety action clips, overlaps, or requires two-dimensional scrolling at the floors; reduced motion suppresses transitions.
R16 (CAP-16): Application-update separation — the app's own update surfaces only as the restrained `Pack-Manager Update Ready!` badge linking to the Settings card, and never enters Package plans, Activity, Results, or History, including during replay. Success: update-state transitions during live or replayed Package work never inject into those surfaces; the Settings card shows the one-line warning-yellow → success-green version delta.

## Constraints

- Each entry's v6 acceptance criteria (carried in its `description`) are binding; where a Requirements line and a story criterion differ in detail, the story text wins.
- No `Cancelling` state at any level — not wire, not derived presentation, not an event; any `OpStatus` change is one atomic AD-3 change across the Rust enum, `src/lib/ipc/types.ts`, guards, and `dev/fixtures/ipc/` (prd FR-13, AD-16, D17).
- Exactly two attempt-journal records with one append authority: entry 2c writes admission, 2e owns the terminal write inside the transition's critical section, 4a reads and folds only; appends gate nothing (AD-29, AD-18).
- The draft has exactly one author, is session-scoped, and is never persisted; the frontend never authors or edits executable command text — Rust rebuilds from canonical intent (AD-16, AD-24).
- One confirmed attempt active at a time; concurrency inside it goes through the existing lock-set scheduler (D30, D4).
- Membership mutations are batched: one operation per range or filter-wide interaction, carrying concrete canonical identities plus the snapshot token — never a predicate for Rust to re-expand (AD-28, NFR-3).
- Focus is a real 2px `outline` in `--color-focus-ring`, never `ring-*` or `outline-none` (AD-27, D35).
- D37: keyboard operability, VoiceOver, and live-region announcements are removed criteria — build none, report none as gaps, and strip no shipped ARIA or focus code.
- Sequencing follows the entries' `after`; entry 3a triggers on atomic admission (2b), never on the 5a dialog.
- `EXPERIENCE.md` still carries `Cancelling` rows and D37-removed keyboard/announcement sections — known-stale divergence, not authority; fixes route through a `bmad-ux` Update, never a hand edit.
- No sudo, shell, or password path anywhere; structured argv, constructed environment, null stdin (FR-12, NFR-1).

## Done when

- All 28 entries are done with their v6 acceptance criteria green.
- No immediate-execution call site remains: every Package, Manager, and `Run fix` update is staged in the draft first (SM-2).
- One confirmed plan produces exactly one `planAttemptId`, two journal records, verification-gated Results, and one immutable History row that replays read-only.
- The confirmation gate, its dialog-only opt-out, and the confirmation-off bypass all keep the native rebuild and stale check.
- Every safety surface is usable at 900 × 600 and 150–200% zoom, shipped in a release.

## Boundaries

- Not keyboard operability, VoiceOver support, or announcement channels as criteria (D37).
- Not rollback of partially completed Manager work, in cancellation or anywhere else.
- Not persisting the draft plan across relaunch, or reconstructing it.
- Not a second live-summary surface, a second status channel, or a router.
- Not growing the immediate-execution set — the target is zero kinds; a fourth kind is a new decision breaching SM-2.
- The eight survivor stories belong to `epic-shipped-behavior-gaps` (its entry 66, the quit guard, included; entries 1b, 2f, and 4e presume that guard and never build it).
- Not reinstating anything D33 retired: readiness gates, scenario contracts, coverage maps, evidence lanes.
- Not user-initiated History deletion — no per-row delete, no Clear History, no retention knob; automatic retention stays the only pruning (D41).

## References

- parent — `_bmad-output/initiative-pack-manager/initiative-pack-manager.md`, section Requirements (R8, R23)
- companion — `capability-story-map.md` beside this file (the v6 `story-map.md`, renamed so `tickets.py` does not read it as a `story-*` leaf): CAP → story → wave map and the load-bearing ordering
- architecture — `_bmad-output/initiative-pack-manager/architecture-Pack-Manager-2026-07-23/architecture-Pack-Manager-2026-07-23.md`, AD-16, AD-17, AD-18, AD-21–AD-24, AD-27–AD-30
- design — `_bmad-output/initiative-pack-manager/ux-Pack-Manager-2026-07-23/DESIGN.md`
- experience — `_bmad-output/initiative-pack-manager/ux-Pack-Manager-2026-07-23/EXPERIENCE.md`
- prd — `_bmad-output/initiative-pack-manager/prd-Pack-Manager-2026-07-25/prd-Pack-Manager-2026-07-25.md` with `addendum.md`
- decisions — `docs/SPEC.md`, `docs/DECISIONS.md` (D27–D30, D35, D37, D38, D41)
- history — `_bmad-output/initiative-pack-manager/archive-v6/spec-upgrade-plan-redesign/` (v6 SPEC.md and stories.yaml), `archive-v6/epics.md` (Epic UX-PB)

## Notes

- Assumption: `ARCHITECTURE-SPINE` `artifact_revision: 12` (read 2026-08-18) is current; a later revision requires re-deriving this epic's Requirements.
- Decision: 2026-10-28 — migrated from the v6 spec kernel `spec-upgrade-plan-redesign`; its `.memlog.md` moved here unchanged and its v6 story ids (`1a`…`5e`) are kept as entry ids.
