---
id: SPEC-shipped-behavior-gaps
companions:
  - ../../planning-artifacts/epics.md
  - ../../planning-artifacts/prds/prd-Pack-Manager-2026-07-25/prd.md
  - ../../planning-artifacts/prds/prd-Pack-Manager-2026-07-25/addendum.md
  - ../../planning-artifacts/architecture/architecture-Pack-Manager-2026-07-23/ARCHITECTURE-SPINE.md
  - ../../planning-artifacts/ux-designs/ux-Pack-Manager-2026-07-23/DESIGN.md
  - ../../planning-artifacts/ux-designs/ux-Pack-Manager-2026-07-23/EXPERIENCE.md
  - ../../../docs/SPEC.md
  - ../../../docs/DECISIONS.md
sources:
  - ../../planning-artifacts/story-triage-2026-07-24.md
---

> **Canonical contract.** This SPEC and the files in `companions:` are the complete, preservation-validated contract for what to build, test, and validate. Source documents listed in frontmatter are for traceability — consult them only if you need narrative rationale or prose color this contract intentionally omits.

> **Authority note.** Same as its sibling `spec-upgrade-plan-redesign`: this spec is a kernel over live authorities and supersedes none — PRD on requirements, `ARCHITECTURE-SPINE.md` (revision 12) on architecture invariants, `docs/DECISIONS.md` as decision log, `epics.md` for binding per-story acceptance criteria, `sprint-status.yaml` for tracking (never regenerate it).

# Shipped-Behavior Gap Closure (D33 Survivors)

## Why

A pain to solve: the D33 rescope triaged 37 Epic 1–6 stories against one test — *does this make the app better for someone using it?* — and found most behavior already shipped. Seven stories survive with a verified gap between the shipping 1.0.x build and the requirement: 2.2, 3.1, 3.2, 3.4, 3.5, 6.5, and 6.6 (the quit guard, revived by D30/AD-30 after the rescope and hand-added to `sprint-status.yaml` on 2026-08-11). An eighth, Story 2.5, was added 2026-08-18 by `sprint-change-proposal-2026-08-18.md` implementing D40 — owner-added scope like 6.6, not a resurrected triage story. Each is a defect the maintainer would feel — a hung refresh stalling everything, a pinned row that explains nothing on hover, Settings that claim `Saved` after a failed write, a diagnostics ZIP missing its evidence, a quit that orphans a live upgrade. This spec is the kernel for that closure work.

## Capabilities

- **CAP-1 — Refresh phase and timeout honesty**
  - **intent:** Each Manager's refresh shows its correct phase order (the disabled Homebrew-metadata path omits only that phase), enforces its own timeout so a hung Manager gives up alone while peers finish, and merges recovered parse output into the already-parsed inventory. (Story 2.2; FR-3, FR-17; AD-4, AD-25)
  - **success:** Controlled-time tests show Manager-specific terminal states with peers unaffected; the merge never blanks up-to-date rows, never un-pins a row, and the retained Last-good Snapshot is labeled with its real timestamp and exact failure beside `Retry refresh`.
- **CAP-2 — Complete Package state and Manager detail**
  - **intent:** Every Package row is understandable without relying on color — name, installed/latest, status text, eligibility, and the plan action; uv executables are expandable and searchable; npm's self state lives only in its Manager Card/Header, where `Update Manager` stages and never executes. (Story 3.1; FR-2, FR-5, FR-6, FR-10, FR-11, FR-19)
  - **success:** All-state fixtures render complete rows with non-color cues; uv executable search works; npm's four ordinary Package rows remain while its self-update appears as `IN PLAN` / `Remove` in the header only.
- **CAP-3 — Pinned and greedy eligibility enforcement**
  - **intent:** Pinned rows stay inert with the correct explained reason across every entry path and filter, using `aria-disabled` — never native `disabled`; greedy-only casks are the two-call set difference, grouped separately, collapsed, default-excluded, and enter a plan only through explicit opt-in with visible disclosure. (Story 3.2; FR-5, FR-6, FR-7; D7, D38)
  - **success:** No membership path — row, header, Manager-wide, `Update Everything` — adds a pinned or default-excluded cask; hover on a pinned row renders its reason; opt-in shows the disclosure before a greedy cask joins.
- **CAP-4 — Settings and Environment Report trustworthiness**
  - **intent:** Every Settings control validates, persists atomically before activation, and shows `Saving`/`Saved`/failure honestly; `skipUpgradePlanConfirmation` is a first-class persisted control; a stale `autoOpenDrawer` value is tolerated as inactive legacy; the Environment Report is complete with visible Copy outcomes. (Story 3.4; FR-17; AD-19, AD-27)
  - **success:** Invalid input is rejected; a failed save changes neither active nor persisted state and never reads `Saved`; log-level changes apply live only after persistence; every required Environment Report field is present and Copy reports success or failure.
- **CAP-5 — Exact batched plan membership and row plan actions**
  - **intent:** Toggle, shift-range, tri-state header checkbox, `⌘A`, Space, and `⌘`-click act directly on draft membership as one batched operation carrying concrete canonical identities plus the snapshot token; ineligible rows never enter membership; `⌘A` stays native on surfaces with no Package list; `Esc` is close-dialog only. (Story 3.5; FR-6, FR-10, RP-2, NFR-3; AD-16, AD-23, AD-28)
  - **success:** A 400-row range over virtualized rows is one round trip resolved against the projection's ordered filtered set, all-or-none in application and narrowing in resolution with dropped refs reported; `⌘A` on Dashboard/History/Settings performs native select-all; a single-row removal writes a tombstone no later bulk expansion re-adds.
- **CAP-6 — Exact native diagnostics with visible outcomes**
  - **intent:** `Export diagnostics` produces the documented timestamped ZIP — `report.json`, newest three app logs, newest 25 transcripts, `operations.jsonl`, and the raw plan-attempt record sets correlated by `planAttemptId` — proven through the production native command, with actionable success and failure for both Export and Open Logs. (Story 6.5; FR-18; AD-3, AD-18, AD-26, AD-29)
  - **success:** The opened archive contains exactly the documented entries with the admission/terminal record *set* per attempt (never a synthesized single record); inherited environment values stay excluded; permission failures surface real errors instead of silence; the harness satisfies AD-26 (compile-time-excluded automation surface driving the production composition).
- **CAP-7 — Quit guard for live child processes**
  - **intent:** A user-initiated quit (window close or `⌘Q`) with any Operation `Queued` or `Running` routes to the single enforcement point the application-update path already uses and presents an explicit choice; an OS-initiated shutdown gets no dialog and runs the awaited kill hook; children never outlive the app. (Story 6.6; FR-14, FR-21; AD-30)
  - **success:** Both quit paths hit one predicate, one dialog, one refusal; choosing to quit anyway terminates every child before exit; the shutdown path awaits the bounded idle wait so SIGTERM→grace→SIGKILL actually completes; no rollback is promised.

- **CAP-8 — Copyable install guidance for absent Managers**
  - **intent:** Every absent Manager's Dashboard card, sidebar entry, and workspace state shows that Manager's copyable install command through the existing `CopyableCommand` paths (extending mas's treatment to all six), and an all-absent machine gets a Dashboard guidance panel — no package managers found, install one yourself (Homebrew is the usual first), then `Refresh All`. (Story 2.5; FR-1's install-hint limb; D40, D14)
  - **success:** All six absent presentations carry verified copy-only commands with no `Install` button or execution affordance anywhere; the all-absent Dashboard shows the guidance panel, never reads `Warning` for absence alone, and `Update Everything` stays disabled with a reason.

## Constraints

- `epics.md` acceptance criteria are binding per story; where this kernel and a story criterion differ in detail, the story text wins.
- Cross-spec sequencing: UX-PB.1d blocks 3.2, UX-PB.1a blocks 3.5, UX-PB.1e blocks 3.1, UX-PB.5b blocks 3.4, UX-PB.3d and 4d block 6.5. Only 2.2, 2.5, and 6.6 are buildable now with no UX-PB dependency.
- Before treating any limb as new work, verify it against shipping code — the adversarial triage overturned 14 of 20 initial keeps for exactly that reason (AD-1, D33).
- Story 6.5's harness is an AD-26-compliant native surface: excluded from release bits at compile time, never by runtime selector, driving the production composition; adopting it is an AD-20 security-reviewed change. No delivery coverage may be claimed from fixtures or the browser double.
- The quit guard's active set is `Queued` ∪ `Running` and must stay identical to FR-21's application-update refusal — changing either predicate is an AD-30 change, not a local one.
- Ineligible controls use `aria-disabled`, never native `disabled` — a natively disabled control dispatches no mouse events, killing the pointer-reachable reason (D38, AD-16).
- Greedy-only casks are identified by the two-call set difference, never an in-JSON heuristic (D7).
- No sudo, shell, or password path; structured argv, constructed environment, null stdin (FR-12, NFR-1).

## Non-goals

- The Upgrade Plan redesign itself — owned by `spec-upgrade-plan-redesign`.
- Re-proving shipped behavior: the 31 archived/merged/retired stories stay retired; scheduling shipped work violates AD-1.
- Keyboard operability or VoiceOver criteria (D37).
- Reinstating anything D33 retired: readiness gates, scenario contracts, evidence lanes, coverage percentages.
- Moving anything out of `_bmad-output/archive/2026-07-24-scope-recalibration/`.
- A diagnostics-export preview or redaction step — the construction-time allowlist plus the visible ZIP path (Story 6.5's own criterion) is the review affordance (D42).

## Success signal

All eight `sprint-status.yaml` keys reach done with their `epics.md` criteria green: a deliberately hung Manager times out alone while five peers finish; a pinned row explains itself on hover; an absent Manager hands over its install command copy-only; a failed Settings save never reads `Saved`; a shift-range over 400 virtualized rows lands as one exact batch; the diagnostics ZIP opens with every documented entry; and quitting mid-upgrade asks first and leaves no orphaned child process.

## Assumptions

- The 19 triage merge residuals were folded into the surviving story text when the rescope was applied to `epics.md` on 2026-07-25, so `epics.md` is complete as the acceptance-criteria authority.
- `ARCHITECTURE-SPINE.md` `artifact_revision: 12` (read 2026-08-18) is current.
