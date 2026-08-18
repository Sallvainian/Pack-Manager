---
id: SPEC-upgrade-plan-redesign
companions:
  - story-map.md
  - ../../planning-artifacts/epics.md
  - ../../planning-artifacts/prds/prd-Pack-Manager-2026-07-25/prd.md
  - ../../planning-artifacts/prds/prd-Pack-Manager-2026-07-25/addendum.md
  - ../../planning-artifacts/architecture/architecture-Pack-Manager-2026-07-23/ARCHITECTURE-SPINE.md
  - ../../planning-artifacts/ux-designs/ux-Pack-Manager-2026-07-23/DESIGN.md
  - ../../planning-artifacts/ux-designs/ux-Pack-Manager-2026-07-23/EXPERIENCE.md
  - ../../../docs/SPEC.md
  - ../../../docs/DECISIONS.md
sources: []
---

> **Canonical contract.** This SPEC and the files in `companions:` are the complete, preservation-validated contract for what to build, test, and validate. Source documents listed in frontmatter are for traceability — consult them only if you need narrative rationale or prose color this contract intentionally omits.

> **Authority note.** This spec is a kernel over live authorities, and it supersedes none of them: the PRD wins on requirements, `ARCHITECTURE-SPINE.md` (revision 12) wins on architecture invariants and is the sole authority for `AD-` ids, `docs/DECISIONS.md` is the decision log, and `epics.md` carries the binding per-story acceptance criteria. Story tracking stays in `_bmad-output/implementation-artifacts/sprint-status.yaml` (never regenerate it — the generate step drops all Epic UX-PB keys).

# Upgrade Plan Redesign (D27–D30)

## Why

A mandate to meet: Decisions D27–D30 (2026-07-24) redesigned the entire upgrade experience — one persistent editable Upgrade Plan, a separate final confirmation gate, a durable plan attempt as the Activity/Results/History unit, and verification-gated success — and they are decided but unimplemented. `planAttemptId`, `Verifying`, `InteractionRequired`, `skipUpgradePlanConfirmation`, and `PlanIntent` occur zero times in `src/` and `src-tauri/src/`; the shipping build still executes a single-row upgrade immediately with no confirmation sheet. Epic UX-PB (28 stories, all backlog) is the primary build queue realizing them, and this spec is its kernel. The product promise at stake is SM-2: no Package or Manager update ever runs that the user did not see staged first — a single violation is a P0 defect.

## Capabilities

- **CAP-1 — Persistent draft plan membership**
  - **intent:** Every update entry point — Package row checkbox, count-labeled header checkbox, Manager-header `Update Manager`, Manager-wide action, `Update Everything` — edits one persistent draft Upgrade Plan of canonical identities, with Rust rebuilding the exact commands on every mutation; nothing executes from any entry point. (UX-PB.1a, 1c; D27; AD-16/17/23/24/28)
  - **success:** All four immediate-execution call sites (row action, both self-update paths, `Run fix`) are gone; toggling an eligible row round-trips through the Rust rebuild into draft membership; the e2e assertion expecting immediate row execution is rewritten to expect membership.
- **CAP-2 — Sidecar lifecycle and navigation persistence**
  - **intent:** The Upgrade Sidecar opens on the first staged item, persists unchanged across Dashboard/Manager navigation, closes on last removal, and is session-scoped — never written to disk. (UX-PB.1b)
  - **success:** Navigating away and back preserves exact membership; relaunch after quit, crash, or force-quit starts with an empty draft, hidden sidecar, and nothing executing.
- **CAP-3 — Ineligible-control inertness with a pointer-reachable reason**
  - **intent:** Pinned, current, excluded, and unavailable Package controls stay inert on every activation path and explain themselves on pointer interaction, using `aria-disabled="true"` — never native `disabled`, never gray styling alone. (UX-PB.1d; D38; AD-16)
  - **success:** Hovering or clicking each ineligible kind renders its exact plain-language reason (the four strings in `epics.md` UX-PB.1d); no activation path changes membership; the header checkbox denominator covers only eligible rows matching the active filter including off-screen ones.
- **CAP-4 — Standardized Manager workspace presentation**
  - **intent:** Every Manager Header and Card shows standardized description, path, installed version, status, ownership, counts in `34 managed packages · 8 package updates` order, and self-update delta; a staged self-update shows `IN PLAN` plus a named `Remove` action. (UX-PB.1e; AD-25)
  - **success:** All six Managers render the standardized identity area; `Update Manager` stages and never executes; a failed refresh retains the timestamped last-good snapshot with `Retry refresh` and no invented health value.
- **CAP-5 — Distinct identities and atomic single-attempt admission**
  - **intent:** The one-use preview `planId` and the durable `planAttemptId` are separate, non-interchangeable branded types across Rust wire, domain, persistence, and TypeScript; `execute_plan` atomically mints exactly one attempt, and a second confirmation fails closed while one is active. (UX-PB.2a, 2b; D29, D30)
  - **success:** Type-level tests reject cross-assignment at every boundary; admission returns one `planAttemptId` plus Operation ids all-or-none; a concurrent second confirmation mints nothing while cross-Manager concurrency continues inside the active attempt.
- **CAP-6 — Durable attempt records and correlation**
  - **intent:** A confirmed attempt persists exactly two append-only records — admission (reviewed scope, exact command snapshot, timestamps) at mint, terminal (verification facts, results) on the terminal transition — and every Operation, event, transcript, and durable record carries the same `planAttemptId`. (UX-PB.2c, 2d; AD-18, AD-29)
  - **success:** The attempt fold reconstructs admission+terminal → outcome and admission-only → `Interrupted`; no persisted `PlanAttempt.state` field exists; a snapshot read back corrupted is refused as an execution source; append failure is surfaced, never fatal to admission.
- **CAP-7 — Plan-level cancellation with two labeled scopes**
  - **intent:** `Cancel plan` stops only that attempt: unstarted work becomes `Skipped`, running work escalates through existing process-group mechanics straight to terminal state, with no second confirmation and no rollback promise; `Cancel operation` is reserved for a deliberately Operation-scoped diagnostic. (UX-PB.2e, 3g; D30; AD-16)
  - **success:** No `Cancelling` state exists at any level and `OpStatus` gains no variant; partially failed escalation reports honestly rather than fabricating outcomes; the terminal record lands in the same critical section as the transition.
- **CAP-8 — Legacy Operation honesty**
  - **intent:** Records without a `planAttemptId` remain readable, individually labeled legacy Operations, never grouped or inferred into plan attempts. (UX-PB.2f, 4e; AD-18/29)
  - **success:** A mixed History renders legacy entries visibly distinct with Operation-level detail; no fabricated grouping appears for plan-like legacy records.
- **CAP-9 — Sidecar as live summary, full Activity as deep view**
  - **intent:** On admission the same sidecar transforms in place into the one live summary; full Activity is a detailed view of the same `planAttemptId`; each item shows honest queued/waiting/running/verifying/terminal state, with safety-critical state (stall handoff, `Interaction required`) reaching the user visibly, never via an announcement channel. (UX-PB.3a, 3b, 3c; AD-17)
  - **success:** No second surface opens on admission; sidecar and Activity render one shared state; stream disconnect keeps last honest state and surfaces the interruption; a status update never moves focus.
- **CAP-10 — Verification-gated Results with failure guidance**
  - **intent:** The attempt becomes Results only after affected-Manager refresh verification; outcomes use the closed taxonomy (attempt: success/partial/failed/cancelled/timed out/interrupted; item: verified/failed/cancelled/skipped); failed items present `What happened` and `What to do next` before a secondary Retry. (UX-PB.3d, 3e; AD-16/25/29)
  - **success:** A successful exit stays `Verifying` until verification resolves and is never colored successful on exit code alone; a failed verification refresh leaves the Last-good Snapshot in place; deterministic failures are not framed as retry-fixable.
- **CAP-11 — Trusted Interaction-required classification**
  - **intent:** `Interaction required` appears only when a closed Manager-specific classifier or explicit native signal recognizes a known prompt; all other null-stdin silence follows the ordinary 120-second stall path. (UX-PB.3f; D30)
  - **success:** Unmatched output at the stall threshold shows exactly `Keep waiting` / `Copy command` / `Cancel plan`; a recognized prompt is never left as a silent stall; no regex or heuristic guessing converts one into the other.
- **CAP-12 — One immutable History row per attempt, with replay**
  - **intent:** Each confirmed attempt folds to exactly one immutable History row with nested Operation evidence and verified-outcome wording; opening it routes Activity into read-only replay; a replay opened during live work stays clearly secondary to the live attempt. (UX-PB.4a, 4b, 4c; AD-29)
  - **success:** The fold is idempotent by `planAttemptId` — duplicate records never replay as extra rows; replay mutates nothing; `Interrupted` requires genuine record absence, with unreadable terminal records reported as unreadable evidence.
- **CAP-13 — Retry as a new linked attempt**
  - **intent:** Retry first reveals the failed-item scope, then `Create new plan` composes a derived `RetryIntent` in Rust — restricted to failed members, canonically rebuilt — that goes straight to preview and confirmation without touching the persistent draft; confirming mints a fresh `planAttemptId` linked by `retryOfPlanAttemptId`. (UX-PB.4d; AD-24)
  - **success:** The original failed result stays immutable and reachable via `View previous result`; a rebuild failure (item now pinned/current/removed) is explained and admits nothing; dangling lineage is surfaced, never repaired silently.
- **CAP-14 — Separate confirmation gate with a reversible skip**
  - **intent:** `Confirm N Updates` opens the `Proceed with Upgrade Plan?` dialog showing exact commands with `Change Plan` and final confirmation; the disable control exists only inside that dialog, persists `skipUpgradePlanConfirmation` atomically only after admission succeeds, and Settings can reverse it; the disabled path auto-expands commands, shows a persistent `Confirmation is off` warning, relabels to `Run N Updates`, and keeps the native rebuild and stale check. (UX-PB.5a, 5b, 5c; D28; AD-21/22)
  - **success:** Nothing executes before final confirmation (or the explicit `Run N Updates`); the rider never precedes its admission and a failed save leaves `false` active and persisted; the setting is plan-inert so persisting it never expires the plan it rode on; stale validation blocks the bypass run until re-review.
- **CAP-15 — Safety surfaces at the size and zoom floors**
  - **intent:** Plan, Confirmation, Activity, and Results stay fully visible and operable at 900 × 600 and 150–200% zoom, entering the high-zoom stacked layout below 720 usable CSS pixels with a visible Back route and a persistent non-occludable indicator for safety-critical attempt state. (UX-PB.5d; NFR-3; AD-17)
  - **success:** Every safety action keeps a visible AD-27 outline focus state verified at runtime; no safety action clips, overlaps, or requires two-dimensional scrolling at the floors; reduced motion suppresses transitions.
- **CAP-16 — Application-update separation**
  - **intent:** The app's own update surfaces only as the restrained `Pack-Manager Update Ready!` badge linking to the Settings card, and never enters Package plans, Activity, Results, or History — including during replay. (UX-PB.5e)
  - **success:** Update-state transitions during live or replayed Package work never inject into those surfaces; the Settings card shows the one-line warning-yellow → success-green version delta.

## Constraints

- `epics.md` acceptance criteria are binding per story; this kernel never overrides them, and where a capability and a story criterion differ in detail, the story text wins.
- No `Cancelling` state at any level — not wire, not derived presentation, not an event; any `OpStatus` change is one atomic AD-3 change across the Rust enum, `src/lib/ipc/types.ts`, guards, and `dev/fixtures/ipc/` (prd FR-13, AD-16, D17).
- Exactly two attempt-journal records with one append authority: UX-PB.2c writes admission, UX-PB.2e owns the terminal write inside the transition's critical section, UX-PB.4a reads and folds only; appends gate nothing (AD-29, AD-18).
- The draft has exactly one author, is session-scoped, and is never persisted; the frontend never authors or edits executable command text — Rust rebuilds from canonical intent (AD-16, AD-24).
- One confirmed attempt active at a time; concurrency inside it goes through the existing lock-set scheduler (D30, D4).
- Membership mutations are batched: one operation per range or filter-wide interaction, carrying concrete canonical identities plus the snapshot token — never a predicate for Rust to re-expand (AD-28, NFR-3).
- Focus is a real 2px `outline` in `--color-focus-ring`, never `ring-*` or `outline-none` (AD-27, D35).
- D37: keyboard operability, VoiceOver, and live-region announcements are removed criteria — build none, report none as gaps, and strip no shipped ARIA or focus code.
- Sequencing follows `epics.md` dependencies; UX-PB.3a triggers on atomic admission (UX-PB.2b), never on the UX-PB.5a dialog.
- `EXPERIENCE.md` still carries `Cancelling` rows and D37-removed keyboard/announcement sections — known-stale divergence, not authority; fixes route through a `bmad-ux` Update, never a hand edit.
- No sudo, shell, or password path anywhere; structured argv, constructed environment, null stdin (FR-12, NFR-1).

## Non-goals

- Keyboard operability, VoiceOver support, or announcement channels as criteria (D37).
- Rollback of partially completed Manager work, in cancellation or anywhere else.
- Persisting the draft plan across relaunch, or reconstructing it.
- A second live-summary surface, a second status channel, or a router.
- Growing the immediate-execution set — the target is zero kinds; a fourth kind is a new decision breaching SM-2.
- The seven Epic 2/3/6 survivor stories — owned by `spec-shipped-behavior-gaps` (Story 6.6's quit guard included; UX-PB stories presume it, never build it).
- Reinstating anything D33 retired: readiness gates, scenario contracts, coverage maps, evidence lanes.
- User-initiated History deletion — no per-row delete, no Clear History, no retention knob; automatic retention stays the only pruning (D41).

## Success signal

Sallvain stages a mixed plan (Packages plus a Manager self-update) from several entry points, confirms once in `Proceed with Upgrade Plan?`, watches the same sidecar become live Activity and then verification-gated Results, and finds exactly one immutable History row that replays read-only — while `grep -r "planAttemptId" src/ src-tauri/src/` returns real implementations and no unstaged mutation path remains in the product.

## Assumptions

- `ARCHITECTURE-SPINE.md` `artifact_revision: 12` (read 2026-08-18) is current; a later revision requires re-deriving this spec.
- `sprint-status.yaml` remains the tracking authority and `epics.md` the acceptance-criteria authority; this spec adds no story keys and restates no ACs.
