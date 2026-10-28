---
type: epic
title: "Shipped-behavior gap closure (D33 survivors)"
parent: initiative-pack-manager
covers: [R1, R3, R5, R6, R7, R10, R14, R15, R16, R17, R18]
sources: [_bmad-output/initiative-pack-manager/archive-v6/spec-shipped-behavior-gaps/SPEC.md, _bmad-output/initiative-pack-manager/archive-v6/epics.md, _bmad-output/initiative-pack-manager/story-triage-2026-07-24/story-triage-2026-07-24.md]
companions: [_bmad-output/initiative-pack-manager/architecture-Pack-Manager-2026-07-23/architecture-Pack-Manager-2026-07-23.md, _bmad-output/initiative-pack-manager/ux-Pack-Manager-2026-07-23/DESIGN.md, _bmad-output/initiative-pack-manager/ux-Pack-Manager-2026-07-23/EXPERIENCE.md]
after: []
risk: high
---

# Shipped-behavior gap closure (D33 survivors)

## Description

The D33 rescope triaged 37 v6 Epic 1–6 stories against one test — *does this make the app better for someone using it?* — and found most behavior already shipped. Seven stories survive with a verified gap between the shipping 1.0.x build and the requirement: 2.2, 3.1, 3.2, 3.4, 3.5, 6.5, and 6.6 (the quit guard, revived by D30/AD-30 after the rescope). An eighth, Story 2.5, was added 2026-08-18 by `sprint-change-proposal-2026-08-18` implementing D40 — owner-added scope, not a resurrected triage story. Each is a defect the maintainer would feel: a hung refresh stalling everything, a pinned row that explains nothing on hover, Settings that claim `Saved` after a failed write, a diagnostics ZIP missing its evidence, a quit that orphans a live upgrade. The v6 grouping was Epic 2 (2.2, 2.5), Epic 3 (3.1, 3.2, 3.4, 3.5), and Epic 6 (6.5, 6.6); entry ids keep those numbers without the dash.

## Outcome

All eight entries reach done with their v6 criteria green: a deliberately hung Manager times out alone while five peers finish; a pinned row explains itself on hover; an absent Manager hands over its install command copy-only; a failed Settings save never reads `Saved`; a shift-range over 400 virtualized rows lands as one exact batch; the diagnostics ZIP opens with every documented entry; and quitting mid-upgrade asks first and leaves no orphaned child process.

## Requirements

R1 (CAP-1; parent R3, R16, R17): Refresh phase and timeout honesty — each Manager's refresh shows its correct phase order (the disabled Homebrew-metadata path omits only that phase), enforces its own timeout so a hung Manager gives up alone while peers finish, and merges recovered parse output into the already-parsed inventory (Story 2.2; FR-3, FR-17; AD-4, AD-25). Success: controlled-time tests show Manager-specific terminal states with peers unaffected; the merge never blanks up-to-date rows, never un-pins a row, and the retained Last-good Snapshot is labeled with its real timestamp and exact failure beside `Retry refresh`.
R2 (CAP-2; parent R5, R6, R10): Complete Package state and Manager detail — every Package row is understandable without relying on color (name, installed/latest, status text, eligibility, and the plan action); uv executables are expandable and searchable; npm's self state lives only in its Manager Card/Header, where `Update Manager` stages and never executes (Story 3.1; FR-2, FR-5, FR-6, FR-10, FR-11, FR-19). Success: all-state fixtures render complete rows with non-color cues; uv executable search works; npm's four ordinary Package rows remain while its self-update appears as `IN PLAN` / `Remove` in the header only.
R3 (CAP-3; parent R5, R6, R7): Pinned and greedy eligibility enforcement — pinned rows stay inert with the correct explained reason across every entry path and filter, using `aria-disabled`, never native `disabled`; greedy-only casks are the two-call set difference, grouped separately, collapsed, default-excluded, and enter a plan only through explicit opt-in with visible disclosure (Story 3.2; FR-5, FR-6, FR-7; D7, D38). Success: no membership path — row, header, Manager-wide, `Update Everything` — adds a pinned or default-excluded cask; hover on a pinned row renders its reason; opt-in shows the disclosure before a greedy cask joins.
R4 (CAP-4; parent R17): Settings and Environment Report trustworthiness — every Settings control validates, persists atomically before activation, and shows `Saving`/`Saved`/failure honestly; `skipUpgradePlanConfirmation` is a first-class persisted control; a stale `autoOpenDrawer` value is tolerated as inactive legacy; the Environment Report is complete with visible Copy outcomes (Story 3.4; FR-17; AD-19, AD-27). Success: invalid input is rejected; a failed save changes neither active nor persisted state and never reads `Saved`; log-level changes apply live only after persistence; every required Environment Report field is present and Copy reports success or failure.
R5 (CAP-5; parent R6, R10): Exact batched plan membership and row plan actions — toggle, shift-range, tri-state header checkbox, `⌘A`, Space, and `⌘`-click act directly on draft membership as one batched operation carrying concrete canonical identities plus the snapshot token; ineligible rows never enter membership; `⌘A` stays native on surfaces with no Package list; `Esc` is close-dialog only (Story 3.5; FR-6, FR-10, RP-2, NFR-3; AD-16, AD-23, AD-28). Success: a 400-row range over virtualized rows is one round trip resolved against the projection's ordered filtered set, all-or-none in application and narrowing in resolution with dropped refs reported; `⌘A` on Dashboard/History/Settings performs native select-all; a single-row removal writes a tombstone no later bulk expansion re-adds.
R6 (CAP-6; parent R15, R18): Exact native diagnostics with visible outcomes — `Export diagnostics` produces the documented timestamped ZIP (`report.json`, newest three app logs, newest 25 transcripts, `operations.jsonl`, and the raw plan-attempt record sets correlated by `planAttemptId`), proven through the production native command, with actionable success and failure for both Export and Open Logs (Story 6.5; FR-18; AD-3, AD-18, AD-26, AD-29). Success: the opened archive contains exactly the documented entries with the admission/terminal record set per attempt, never a synthesized single record; inherited environment values stay excluded; permission failures surface real errors instead of silence; the harness satisfies AD-26 (compile-time-excluded automation surface driving the production composition).
R7 (CAP-7; parent R14): Quit guard for live child processes — a user-initiated quit (window close or `⌘Q`) with any Operation `Queued` or `Running` routes to the single enforcement point the application-update path already uses and presents an explicit choice; an OS-initiated shutdown gets no dialog and runs the awaited kill hook; children never outlive the app (Story 6.6; FR-14, FR-21; AD-30). Success: both quit paths hit one predicate, one dialog, one refusal; choosing to quit anyway terminates every child before exit; the shutdown path awaits the bounded idle wait so SIGTERM→grace→SIGKILL actually completes; no rollback is promised.
R8 (CAP-8; parent R1): Copyable install guidance for absent Managers — every absent Manager's Dashboard card, sidebar entry, and workspace state shows that Manager's copyable install command through the existing `CopyableCommand` paths (extending mas's treatment to all six), and an all-absent machine gets a Dashboard guidance panel — no package managers found, install one yourself (Homebrew is the usual first), then `Refresh All` (Story 2.5; FR-1's install-hint limb; D40, D14). Success: all six absent presentations carry verified copy-only commands with no `Install` button or execution affordance anywhere; the all-absent Dashboard shows the guidance panel, never reads `Warning` for absence alone, and `Update Everything` stays disabled with a reason.

## Constraints

- Each entry's v6 acceptance criteria (carried in its `description`) are binding; where a Requirements line and a story criterion differ in detail, the story text wins.
- Cross-epic sequencing, as entry `after`: `epic-upgrade-plan-redesign` 1a blocks 35, 1d blocks 32, 1e blocks 31, 5b blocks 34, 3d and 4d block 65. Only 66, 22, and 25 are buildable with no upgrade-plan dependency.
- Before treating any limb as new work, verify it against shipping code — the adversarial triage overturned 14 of 20 initial keeps for exactly that reason (AD-1, D33).
- Entry 65's harness is an AD-26-compliant native surface: excluded from release bits at compile time, never by runtime selector, driving the production composition; adopting it is an AD-20 security-reviewed change. No delivery coverage may be claimed from fixtures or the browser double.
- The quit guard's active set is `Queued` ∪ `Running` and must stay identical to FR-21's application-update refusal — changing either predicate is an AD-30 change, not a local one.
- Ineligible controls use `aria-disabled`, never native `disabled` — a natively disabled control dispatches no mouse events, killing the pointer-reachable reason (D38, AD-16).
- Greedy-only casks are identified by the two-call set difference, never an in-JSON heuristic (D7).
- No sudo, shell, or password path; structured argv, constructed environment, null stdin (FR-12, NFR-1).

## Done when

- A deliberately hung Manager times out alone while its peers finish, and recovered output never blanks the inventory.
- Every absent Manager hands over a copy-only install command, and an all-absent machine shows the guidance panel.
- Package rows, pinned and greedy eligibility, and batched membership behave exactly as their criteria state.
- A failed Settings save never reads `Saved`, and the Environment Report is complete.
- The diagnostics ZIP holds every documented entry, and a quit mid-upgrade asks first and orphans no child process, shipped in a release.

## Boundaries

- The Upgrade Plan redesign itself belongs to `epic-upgrade-plan-redesign`.
- Not re-proving shipped behavior: the 31 archived, merged, or retired stories stay retired; scheduling shipped work violates AD-1.
- Not keyboard operability or VoiceOver criteria (D37).
- Not reinstating anything D33 retired: readiness gates, scenario contracts, evidence lanes, coverage percentages.
- Not moving anything out of `_bmad-output/archive/2026-07-24-scope-recalibration/`.
- Not a diagnostics-export preview or redaction step — the construction-time allowlist plus the visible ZIP path is the review affordance (D42).

## References

- parent — `_bmad-output/initiative-pack-manager/initiative-pack-manager.md`, section Requirements (R1, R3, R5–R7, R10, R14–R18)
- architecture — `_bmad-output/initiative-pack-manager/architecture-Pack-Manager-2026-07-23/architecture-Pack-Manager-2026-07-23.md`, AD-1, AD-4, AD-16, AD-18, AD-19, AD-23, AD-25–AD-30
- design — `_bmad-output/initiative-pack-manager/ux-Pack-Manager-2026-07-23/DESIGN.md`
- experience — `_bmad-output/initiative-pack-manager/ux-Pack-Manager-2026-07-23/EXPERIENCE.md`
- prd — `_bmad-output/initiative-pack-manager/prd-Pack-Manager-2026-07-25/prd-Pack-Manager-2026-07-25.md` with `addendum.md`
- triage — `_bmad-output/initiative-pack-manager/story-triage-2026-07-24/story-triage-2026-07-24.md`
- decisions — `docs/SPEC.md`, `docs/DECISIONS.md` (D7, D14, D30, D33, D37, D38, D40, D42)
- history — `_bmad-output/initiative-pack-manager/archive-v6/spec-shipped-behavior-gaps/` (v6 SPEC.md and stories.yaml), `archive-v6/epics.md` (Epics 2, 3, 6)

## Notes

- Assumption: the 19 triage merge residuals were folded into the surviving story text when the rescope was applied to v6 `epics.md` on 2026-07-25, so each entry's carried text is complete.
- Assumption: `ARCHITECTURE-SPINE` `artifact_revision: 12` (read 2026-08-18) is current.
- Source conflict: v6 `stories.yaml` orders 35 before 31 and 32, but v6 `epics.md` declares Story 3.5 depends on Stories 3.1 and 3.2, and Story 3.2 on Story 3.1. The entries keep the `stories.yaml` order and take `after` from the declared dependencies (35 after 31, 32, 1.1a; 32 after 31, 1.1d), so `tickets.py next` will run 31 and 32 first.
- Decision: 2026-10-28 — migrated from the v6 spec kernel `spec-shipped-behavior-gaps`; its `.memlog.md` moved here unchanged.
