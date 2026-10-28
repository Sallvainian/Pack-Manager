---
type: initiative
title: "Pack-Manager: one trustworthy control plane over six macOS package managers"
sources:
  - _bmad-output/initiative-pack-manager/prd-Pack-Manager-2026-07-25/prd-Pack-Manager-2026-07-25.md
  - _bmad-output/initiative-pack-manager/prd-Pack-Manager-2026-07-25/addendum.md
companions:
  - _bmad-output/initiative-pack-manager/architecture-Pack-Manager-2026-07-23/architecture-Pack-Manager-2026-07-23.md
  - _bmad-output/initiative-pack-manager/ux-Pack-Manager-2026-07-23/DESIGN.md
  - _bmad-output/initiative-pack-manager/ux-Pack-Manager-2026-07-23/EXPERIENCE.md
after: []
risk: high
---

# Pack-Manager: one trustworthy control plane over six macOS package managers

## Description

Pack-Manager is a macOS desktop app that gives one person one trustworthy control plane over a machine whose software arrives through six package managers — Homebrew, mise, npm, uv, rustup, and the Mac App Store CLI. It detects what is installed, shows what each Manager says is outdated, explains which Manager owns and will update each tool, and updates everything, a chosen subset, a single Package, or a Manager itself. The differentiator is confidence across a mixed Manager topology: nothing runs that was not staged and shown, no version judgment is invented, no privilege prompt exists, one Manager failing never blanks the others, and every operation streams live, cancels immediately, and leaves durable evidence (PRD §1). Version 1.0.x ships; the remaining work is the D27–D30 Upgrade Plan redesign and the eight D33-survivor gap stories, the two epics below.

## Outcome

The maintainer reaches for Pack-Manager instead of six terminals and still does so a month later (PRD SM-1), with zero unreviewed mutations (SM-2) and zero privilege prompts (SM-3).

## Requirements

R1 (FR-1): The user can see which of the six Managers are present on the machine, at launch and on demand, without configuring anything.
R2 (FR-2): A Package is Outdated when and only when its Manager says so.
R3 (FR-3): Each Manager's inventory and Outdated state refresh on their own, and one Manager's failure never damages another's.
R4 (FR-4): The user can see which Manager owns each tool and how each Manager updates itself, with inspectable evidence.
R5 (FR-5): The user can browse, search, filter, and understand each Manager's Packages, and can tell at a glance what can and cannot be updated.
R6 (FR-6): A Package checkbox is the Upgrade Plan membership control. There is no separate selection to build and then submit.
R7 (FR-7): Before anything runs, the user can see precisely what will run.
R8 (FR-8): Execution proceeds only when the submitted plan matches both the reviewed plan and a fresh coherent rebuild from current state.
R9 (FR-9): A confirmed plan is admitted all-or-none. The user is never left with a partially submitted bulk update.
R10 (FR-10): The user has a low-friction path to update one Package, and it is the same safe path as everything else.
R11 (FR-11): Each Manager has a standardized identity area that explains what it is, where it is, and how it updates.
R12 (FR-12): There is no path through Pack-Manager to a shell, a password prompt, or elevated privilege.
R13 (FR-13): The user can watch work happen and understand where it is.
R14 (FR-14): Silence and excessive duration become honest, actionable states rather than an indefinite spinner.
R15 (FR-15): The user can answer "what ran, what happened, where is the evidence" after the fact.
R16 (FR-16): An outcome — good or bad — leaves the user with more information than they had, never less.
R17 (FR-17): A setting becomes active only after it is successfully saved.
R18 (FR-18): One action produces one support bundle that explains the machine without disclosing it.
R19 (FR-19): The app reads as one focused macOS control surface, not six command wrappers in a window.
R20 (FR-20): The user learns about a new version without asking, and the download does not interrupt their work.
R21 (FR-21): Nothing about the installed application changes without a deliberate user action.
R22 (FR-22): The universal release launches normally on both architectures, is signed, notarized and stapled, and accepts only updater payloads cryptographically authorized for the installed application.
R23 (FR-23): A Manager can tell the user its environment is broken and suggest a repair. Only a suggestion the product already recognizes, exactly, ever becomes something the user can run.
R24 (RP-1): Launch, six-hour, and app-menu update checks are preserved; in-process update state survives UI recreation; a failed download never presents as Ready; application-update state stays separate from the Operation queue and History.
R25 (RP-2): Standard Edit and Window menu actions — including cut, copy, paste, and select-all in the search field and in every copyable command surface — are preserved.

## Constraints

- NFR-1 Fail closed: unreviewed, stale, altered, replayed, partially admissible, or privilege-seeking work never runs; user exclusions and Manager protections remain authoritative in every path.
- NFR-2 Isolate and recover: no failure blanks another Manager or destroys a Last-good Snapshot; recovered partial data merges into the existing inventory rather than replacing it.
- NFR-3 Stay responsive: progressive rendering; interactive beyond 100 Packages with correct actions at 101 rows; live output flushes at 50 ms / 64 lines / 8 KiB; newest 5,000 live lines retained; usable at 900 × 600 and 150–200% zoom.
- NFR-4 Correlate evidence: status, output, transcript, log, History, and diagnostics correlate through durable identity; transcript-creation failure blocks the spawn.
- NFR-5 Privacy and trust boundary: no telemetry, no generic shell surface, inherited environment excluded from logs and diagnostics, symlink-resistant export; any capability or permission change is security-sensitive.
- NFR-6 Interface quality: non-color cues, pointer-accessible ineligibility reasons, ≥ 4.5:1 text contrast, reduced motion, a visible focus indicator (a real `outline`, AD-27), VersionDelta display-only.
- NFR-7 Platform: normal Finder/Dock launch, both architectures, incompatible Manager output fails visibly and locally; minimum macOS 15.0 (D31).
- NFR-8 Delivery: direct-download and updater artifacts stay consistent and authorized; the updater signature and published metadata are checked at release time.
- Structured argv only, constructed environment, null stdin; no `sudo` or password path anywhere (PRD §6, FR-12).
- `ARCHITECTURE-SPINE` is the sole authority for `AD-` ids; `docs/DECISIONS.md` is the decision log, and a decision dated after 2026-07-25 supersedes the PRD.
- Keyboard operability, VoiceOver, and live-region announcements are not criteria (D37).

## Done when

- The maintainer uses Pack-Manager instead of six terminals and still does a month later (SM-1).
- No Package or Manager update ever runs that the user did not see staged first (SM-2); D27–D30 ship and `planAttemptId` exists in `src/` and `src-tauri/src/`.
- No release surfaces `sudo`, a password field, or an administrator dialog (SM-3).
- A failure is answerable from transcript plus History without reproducing it (SM-4).
- Every release's updater signature verifies and its metadata is reachable and coherent, checked automatically (SM-5).

## Boundaries

- Not a package installer or uninstaller, a privilege-escalation path, an unattended updater, a terminal, a version oracle, a telemetry client, a pin-breaker, a retry loop, a rollback engine, a cross-Manager deduplicator beyond D10, or a team or fleet tool (PRD §6).
- Not optimizing time-to-update or feature count (SM-C1, SM-C2).
- Release readiness is `docs/RELEASE-CHECKLIST.md` plus the two release-blocking checks in `release.yml`, not an epic (D33).
- Touch point: shipped requirements R2, R4, R9, R11, R12, R13 — no epic; their remaining limbs are realized inside `epic-upgrade-plan-redesign`.
- Touch point: R19–R22, R24, R25 — validated through `docs/RELEASE-CHECKLIST.md`; no epic.
- Tracer path: a Package staged in the persistent draft (`epic-upgrade-plan-redesign` wave 1) is admitted under one `planAttemptId`, runs, verifies, lands as one History row, and is exported in diagnostics (`epic-shipped-behavior-gaps` story 65).

## References

- prd — `_bmad-output/initiative-pack-manager/prd-Pack-Manager-2026-07-25/prd-Pack-Manager-2026-07-25.md`, §4 Features (FRs), §5 NFRs, §6 Non-Goals, §8 Success Metrics; read with `addendum.md`
- architecture — `_bmad-output/initiative-pack-manager/architecture-Pack-Manager-2026-07-23/architecture-Pack-Manager-2026-07-23.md` (`artifact_revision: 12`)
- design — `_bmad-output/initiative-pack-manager/ux-Pack-Manager-2026-07-23/DESIGN.md`
- experience — `_bmad-output/initiative-pack-manager/ux-Pack-Manager-2026-07-23/EXPERIENCE.md`
- decisions — `docs/DECISIONS.md`
- product spec — `docs/SPEC.md`
- release — `docs/RELEASE-CHECKLIST.md`
- history — `_bmad-output/initiative-pack-manager/archive-v6/epics.md` (v6 FR Coverage Map and story text), the five `sprint-change-proposal-*/` folders, `story-triage-2026-07-24/`, `implementation-readiness-report-2026-07-25/`

## Notes

- Parked: R2, R4, R9, R11, R12, R13 — triaged out by D33 as shipped (`archive-v6/epics.md` FR Coverage Map); no epic.
- Parked: R19, R20, R21, R22, R24, R25 — validated through `docs/RELEASE-CHECKLIST.md`; no epic.
- Decision: 2026-10-28 — the v7 epics are the two v6 spec kernels (`spec-upgrade-plan-redesign`, `spec-shipped-behavior-gaps`) rather than `epics.md`'s four epics, so the 36 stories exist once; recorded in `_bmad-output/migration-v6-v7/migration-v6-v7.md`.
- Assumption: `ARCHITECTURE-SPINE` `artifact_revision: 12` is current; both epics were derived against it.
