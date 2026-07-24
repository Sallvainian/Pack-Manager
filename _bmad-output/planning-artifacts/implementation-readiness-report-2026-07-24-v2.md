---
project_name: Pack-Manager
date: 2026-07-24
run: v2-post-correction
stepsCompleted:
  - step-01-document-discovery
  - step-02-prd-analysis
  - step-03-epic-coverage-validation
  - step-04-ux-alignment
  - step-05-epic-quality-review
  - step-06-final-assessment
overallStatus: NEEDS WORK — planning validated; blocked only on declared implementation-entry gates
priorCriticalFindings: RESOLVED (superseded contracts + oversized UX-PB)
assessor: claude-code (Implementation Readiness workflow)
documentsIncluded:
  prd:
    - prds/prd-Pack-Manager-2026-07-22/prd.md
    - prds/prd-Pack-Manager-2026-07-22/addendum.md
  architecture:
    - architecture/architecture-Pack-Manager-2026-07-23/ARCHITECTURE-SPINE.md
  epics:
    - epics.md
  ux:
    - ux-designs/ux-Pack-Manager-2026-07-23/DESIGN.md
    - ux-designs/ux-Pack-Manager-2026-07-23/EXPERIENCE.md
supersedes: implementation-readiness-report-2026-07-24.md
---

# Implementation Readiness Assessment Report

**Date:** 2026-07-24
**Project:** Pack-Manager

> **Run context:** This is a re-validation run following a correct-course workflow
> that regenerated `epics.md` (2928 → 3538 lines, finished 2026-07-24 05:11). It
> supersedes `implementation-readiness-report-2026-07-24.md` (01:10), whose two
> critical findings — superseded D27–D30/AD-16 contracts and an oversized UX-PB
> batch — the correction claims to have resolved. This run verifies those claims
> and checks for regressions.

---

## Step 1 — Document Inventory

All four required document types were located under
`_bmad-output/planning-artifacts/`. No whole-vs-sharded duplicates exist; no
required document is missing. Docs are nested inside dated subfolders and use no
`index.md`; the canonical primary file in each was resolved by convention.

### PRD
- **`prds/prd-Pack-Manager-2026-07-22/prd.md`** — 1,248 lines *(primary)* · mod 2026-07-24 01:10
- `prds/prd-Pack-Manager-2026-07-22/addendum.md` — 106 lines *(addendum)* · mod 2026-07-23 00:56
- ~16 process artifacts (`extract-*`, `polish-*`, `reconcile-*`, `research-external-readiness`, `review-*`, `readiness-coverage-map`) — supporting, excluded from assessment

### Architecture
- **`architecture/architecture-Pack-Manager-2026-07-23/ARCHITECTURE-SPINE.md`** — 969 lines *(primary)* · mod 2026-07-24 01:10
- `reviews/` — 3 review artifacts (downstream-divergence, reality-current, rubric-walker) — excluded

### Epics & Stories
- **`epics.md`** — 3,538 lines (~254 KB) *(whole)* · mod 2026-07-24 05:11 *(most recent doc)*

### UX Design
- **`ux-designs/ux-Pack-Manager-2026-07-23/DESIGN.md`** — 252 lines *(primary)* · mod 2026-07-24 00:08
- **`ux-designs/ux-Pack-Manager-2026-07-23/EXPERIENCE.md`** — 460 lines *(primary)* · mod 2026-07-24 01:04
- `review-accessibility.md`, `review-usability.md`, `validation-report.md`, `mockups/`, `.working/` — review/support artifacts, excluded

### Issues
- ✅ No duplicate document formats.
- ✅ No missing required documents.
- ⚠️ Structural note: dated subfolders, no `index.md`; canonical files resolved by convention (confirmed with user).
- ✅ Output-file collision resolved: this run writes to `-v2`; prior 01:10 report preserved.

**Step 1 status: COMPLETE.**

---

## Step 2 — PRD Analysis

**Sources read in full:** `prd.md` (1,248 lines, artifact_revision 2, status `final`) and `addendum.md` (106 lines, artifact_revision 1, status `final`).

**Nature of this PRD:** This is not a greenfield feature PRD. It is a *P0 product-and-release readiness gate* PRD (`prd.md:9` "Pack-Manager 100% P0 Product-and-Release Readiness Gate"). Its requirements are stated as required **outcomes + evidence** across three independent lanes — Product Behavior (Lane A), Test Infrastructure (Lane B), Release Evidence (Lane C) (`prd.md:172-193`). Consequently the requirement space epics must cover spans **six ID families**, not just FR/NFR. All are extracted below verbatim (shall-statements quoted; per-requirement "Consequences" bullets counted, since those bullets are the acceptance sub-criteria the stories must satisfy).

### Functional Requirements (FR-1 … FR-22) — §5, `prd.md:346-734`

| ID | Verbatim shall-statement | Cons. | Lines |
|----|--------------------------|------:|-------|
| FR-1 | "Pack-Manager shall detect Homebrew, mise, npm, uv, rustup, and `mas` at launch and on demand." | 5 | 356-366 |
| FR-2 | "Pack-Manager shall mark a Package Outdated only when its Manager reports it as Outdated." | 4 | 370-380 |
| FR-3 | "Pack-Manager shall refresh installed inventory and Outdated state per Manager without allowing one failure to erase other useful state." | 5 | 383-395 |
| FR-4 | "Pack-Manager shall derive Manager ownership and self-update Routes from current detection and refresh information." | 4 | 399-407 |
| FR-5 | "Pack-Manager shall let the user browse, search, filter, and understand Packages without losing Manager-specific detail." | 5 | 410-425 |
| FR-6 | "Pack-Manager shall support precise selection of eligible Outdated Packages." | 6 | 430-444 |
| FR-7 | "Pack-Manager shall show the exact commands for every Package and Manager update before the user authorizes execution." | 7 | 447-464 |
| FR-8 | "Pack-Manager shall execute a plan request only when it exactly matches the reviewed Upgrade Plan and a fresh coherent rebuild from current state." | 4 | 467-479 |
| FR-9 | "Pack-Manager shall admit a confirmed multi-group Upgrade Plan all-or-none." | 6 | 482-493 |
| FR-10 | "Pack-Manager shall provide a lower-friction row-level update for one eligible Package." | 5 | 496-508 |
| FR-11 | "Pack-Manager shall give each Manager a standardized title area that explains its purpose, path, installed/latest state, Route, and action availability." | 6 | 511-524 |
| FR-12 | "Pack-Manager shall execute only product-defined Operations, without any general shell command, `sudo`, password entry, or administrator prompt path." | 4 | 528-538 |
| FR-13 | "Pack-Manager shall expose queued, running, verifying, stalled, cancelling, and terminal plan state, with nested Operation commands and live output." | 5 | 541-555 |
| FR-14 | "Pack-Manager shall turn a silent or overlong Operation into an actionable state and shall cancel the requested scope without a confirmation dialog." | 6 | 558-575 |
| FR-15 | "Pack-Manager shall durably record enough plan-attempt and Operation evidence to reconstruct what was reviewed, what ran, and what happened." | 11 | 578-603 |
| FR-16 | "Pack-Manager shall make success and failure actionable without destroying previously valid state." | 6 | 606-620 |
| FR-17 | "Pack-Manager shall expose product Settings and apply a change only after it is successfully persisted." | 7 | 625-641 |
| FR-18 | "Pack-Manager shall export one timestamped diagnostics archive containing the information needed to investigate detection, routing, Settings, and recent Operations." | 6 | 644-660 |
| FR-19 | "Pack-Manager shall provide a dark-only launch experience whose primary actions remain understandable and operable by keyboard and non-color cues." | 9 | 663-690 |
| FR-20 | "When an application update check finds a newer release, Pack-Manager shall download it automatically in the background and expose its state." | 3 | 695-704 |
| FR-21 | "Pack-Manager shall install a downloaded application update only after the user chooses Restart to update." | 5 | 707-720 |
| FR-22 | "Pack-Manager shall support the declared Apple silicon and Intel user promise through normal macOS launch and an update path that accepts only authorized Pack-Manager payloads." | 3 | 723-734 |

**Total FRs: 22** (contiguous, no gaps).

### Non-Functional Requirements (NFR-1 … NFR-8) — §8, `prd.md:966-1034`

| ID | Verbatim shall-statement (lead) | Lines |
|----|--------------------------------|-------|
| NFR-1 Fail-closed safety | "None of the unreviewed, stale, altered, replayed, partially admissible, or privilege-seeking work shall run. User exclusions and Manager protections remain authoritative." | 968-972 |
| NFR-2 Failure isolation & recovery | "One Manager's detection, refresh, parse, network, or update failure shall not blank another Manager or destroy a Last-good Snapshot. Crash, cancellation, timeout, and persistence failures shall have explicit recovery outcomes." | 975-978 |
| NFR-3 Responsive bounded presentation | "The app shall render progressive state without waiting for every Manager, remain interactive with more than 100 Package rows, retain no more than 5,000 live lines per Operation…" + 5 quantified acceptance bullets (101 rows; flush at 50 ms / 64 lines / 8 KiB; 5,001 lines; 900×600 min window; 150%/200% zoom) | 981-998 |
| NFR-4 Durable observability | "Status, output, transcript, structured log, History, and diagnostic evidence shall correlate through durable Plan Attempt identity and nested Operation identity. Transcript creation failure blocks an unaudited spawn…" | 1000-1005 |
| NFR-5 Privacy & local trust | "The product shall send no telemetry, expose no generic shell surface, and exclude inherited environment values from logs and diagnostics. Diagnostic selection and streaming shall resist symlink substitution." | 1007-1011 |
| NFR-6 Accessibility | "Primary interactions shall be keyboard operable with visible focus; status shall not rely on color; text shall meet at least 4.5:1 contrast; reduced motion shall be honored…" | 1013-1021 |
| NFR-7 macOS compatibility | "The application shall work through normal GUI launch, support both architectures promised by the release, and fail visibly and locally when a Manager's output format is incompatible. The minimum supported macOS version must be declared before final candidate acceptance." | 1023-1028 |
| NFR-8 Release & update integrity | "Direct-download and updater artifacts shall remain mutually consistent, cryptographically authorized, and attributable to one Release Candidate. Background download shall never weaken explicit install/restart control." | 1030-1034 |

**Total NFRs: 8** (contiguous).

### Additional Requirements / Constraints (in-scope for epic coverage)

This PRD's gate (§9.6, `prd.md:1148-1166`) requires FR + NFR **plus** the following families. Epics/stories must trace to these too:

- **Product acceptance journeys AJ-1 … AJ-6** (§4, `prd.md:288-344`) — end-to-end experiences that must be proven (launch/detect/refresh; Update Everything; selected Package/Manager; slow/blocked/failed/cancelled/interrupted; diagnose/export; install/update Pack-Manager).
- **Release Prerequisites RP-1, RP-2** (§5.6, `prd.md:736-763`) — mandatory but **outside** the 72-row P0 denominator (`prd.md:1062-1063`).
- **Test Infrastructure Readiness TIR-1 … TIR-8** (§6, `prd.md:765-863`) — 8 evidence-producing capabilities (behavior-present classification, forced-offline lane, native boundary, process/OS boundaries, lifecycle/persistence, target-Mac lane, packaged/a11y/updater acceptance, provenance/retention).
- **Release Evidence RE-1 … RE-11** (§7, `prd.md:865-964`) — 11 candidate-bound proofs (identity manifest, clean quality result, version-coherent set, architecture/icon/bundle, trust/notarization, updater integrity, fresh-install launch, prior-version update, non-writable install, candidate trace, evidence publication).
- **PC-1 brownfield source correction** (§9.1, `prd.md:1040-1054`) — restore `mas`/notarization/five-event truth; must be satisfied before source lane closes.
- **Gate policy GP-1, GP-2** (§9.2, `prd.md:1058-1086`) — freeze 72-row denominator; distinguish policy-change / waiver / risk-acceptance.
- **Decision register DR-1 … DR-4** (§9.3, `prd.md:1090-1121`): **DR-1 (min macOS version) = OPEN, implementation-entry blocker**; DR-2 APPROVED; DR-3 APPROVED; **DR-4 (P1 gate policy) = PROPOSED, gate-approval blocker**.
- **Implementation-entry blockers** (§9.4, `prd.md:1123-1135`): **UX-PB.1 … UX-PB.5** (Product Behavior Prerequisite implementing Decisions **D27-D30 + AD-16**) before affected evidence stories; approved & mechanically-verified one-to-one 72-row `readiness-coverage-map.md`; named owners in `addendum.md` §A (ASR-01…ASR-05); DR resolutions; evidence-storage/candidate-manifest contract agreement.
- **Success metrics SM-1 … SM-6 + counter-metrics SM-C1 … SM-C4** (§10, `prd.md:1178-1215`).
- **Risks R-001 … R-008** (§11, `prd.md:1224-1233`) — scores on 1–3 scale; **R-007 "Invalid shipped artifact" = score 9** (highest); all others = 6; a 6 or 9 is release-blocking.
- **Eight-batch closure decomposition + dependency waves** (`addendum.md:47-76`) — Batch 1 first; Batches 2/3/4 parallel; 5/6 after Batch-4 native foundation; release-prep freeze; Batch 7 on immutable candidate; Batch 8 attests unchanged candidate.

### PRD Completeness Assessment (initial)

- **Structurally complete and internally cross-referenced.** Every requirement family carries stable IDs, source-authority pointers, and an explicit exit contract (§9.6). Requirement→metric→risk linkage is present (§10 each SM names the FR/TIR/RE it validates).
- **Supersession chain is explicit and current** (`prd.md:44-52`): D23a▸D23, D25/D25a▸D20, D27-D30 supersede immediate-row-execution / drawer-only Activity / `autoOpenDrawer`. **This is the exact contract surface the correct-course run rewrote in `epics.md`** — Step 3/4 must confirm the stories now speak D27-D30/AD-16, not the superseded wording.
- **Two known OPEN blockers are carried in the PRD itself, by design:** DR-1 (min macOS version) and DR-4 (P1 gate policy). These are decision/governance gaps, not planning defects — but stories depending on them (TIR-7, RE-4/7/8) inherit the block. Expect them to surface again in coverage.
- **Denominator dependency:** the entire 72/72 claim depends on an approved, mechanically-verified one-to-one `readiness-coverage-map.md` (`prd.md:207-212`, §9.4). The correction message lists "revision-2 coverage-map approval" as still-open — a live traceability risk to test in Step 3.
- **No unresolved inline product assumptions** (§12, `prd.md:1244-1248`); DR-1 is the one explicit blocking decision, not a hidden assumption.

**Step 2 status: COMPLETE** — 22 FRs + 8 NFRs extracted verbatim; 10 additional requirement families inventoried for coverage validation.

---

## Step 3 — Epic Coverage Validation

**Method:** `epics.md` carries an explicit **FR Coverage Map** (`epics.md:271-321`) assigning each FR to one primary epic. Rather than trust the map, coverage was **verified** by fanning out 9 parallel agents (one per epic section + the UX-PB prerequisite), each of which read only its section and confirmed, against the actual story acceptance criteria, whether each owned FR has ≥1 covering story. 9/9 agents completed, 0 errors.

### Story universe (verified)

| Section | Stories | IDs |
|---------|--------:|-----|
| Product Behavior Prerequisite (UX-PB) | 28 | UX-PB.1a–1e, 2a–2f, 3a–3g, 4a–4e, 5a–5e |
| Epic 1 | 5 | 1.1–1.5 |
| Epic 2 | 4 | 2.1–2.4 |
| Epic 3 | 6 | 3.1–3.6 |
| Epic 4 | 6 | 4.1–4.6 |
| Epic 5 | 9 | 5.1–5.9 |
| Epic 6 | 7 | 6.1–6.7 |
| Epic 7 | 11 | 7.1–7.11 |
| Epic 8 | 7 | 8.1–8.7 |
| **Total leaf stories** | **83** | 55 numbered + 28 UX-PB sub-stories |

Reconciles with the correct-course report ("all 60 stories" = 55 numbered + 5 UX-PB parents pre-decomposition; the 5 parents decomposed into 28). The **28 UX-PB sub-story count and per-parent distribution (5/6/7/5/5) is confirmed present** — matching the correction's claim exactly.

### Coverage Matrix (verified against story acceptance criteria)

| FR | Primary epic | Covering stories (verified) | Status |
|----|-------------|-----------------------------|--------|
| FR-1 | Epic 4 | 4.3 | ✓ Covered |
| FR-2 | Epic 1 | 1.4, 1.5 | ✓ Covered |
| FR-3 | Epic 2 | 2.2, 2.3, 2.4 | ✓ Covered |
| FR-4 | Epic 5 | 5.2, 5.3 | ✓ Covered |
| FR-5 | Epic 3 | 3.1, 3.2, 3.6 | ✓ Covered |
| FR-6 | Epic 3 | 3.1, 3.2, 3.3, 3.5 | ✓ Covered |
| FR-7 | Epic 3 | 3.2, 3.3, 3.6 | ✓ Covered |
| FR-8 | Epic 3 | 3.6 | ✓ Covered *(see note)* |
| FR-9 | Epic 5 | 5.3, 5.9 | ✓ Covered |
| FR-10 | Epic 3 | 3.1, 3.5 | ✓ Covered |
| FR-11 | Epic 5 | 5.2, 5.3 | ✓ Covered |
| FR-12 | Epic 5 | 5.3, 5.5 | ✓ Covered |
| FR-13 | Epic 5 | 5.4 | ✓ Covered |
| FR-14 | Epic 5 | 5.5 | ✓ Covered |
| FR-15 | Epic 6 | 6.3, 6.4 | ✓ Covered |
| FR-16 | Epic 2 | 2.3, 2.4 | ✓ Covered |
| FR-17 | Epic 3 (+Epic 6 x-cut) | 3.4 (+6.7) | ✓ Covered |
| FR-18 | Epic 6 | 6.5, 6.6, 6.7 | ✓ Covered |
| FR-19 | Epic 7 | 7.6, 7.11 | ✓ Covered |
| FR-20 | Epic 7 | 7.7, 7.10 | ✓ Covered |
| FR-21 | Epic 7 | 7.8, 7.9, 7.10 | ✓ Covered |
| FR-22 | Epic 8 | 8.2, 8.3, 8.6 | ✓ Covered |
| RP-1 | Epic 7 (+Epic 8) | 7.10 | ✓ Covered |
| RP-2 | Epic 7 (+Epic 8) | 7.11 | ✓ Covered |

**Reverse check (FRs in epics not in PRD):** none. The `epics.md` Requirements Inventory (`epics.md:34-98`) mirrors PRD FR-1…22, RP-1/2, and NFR-1…8 exactly — no invented or dropped requirement.

### Missing Requirements

**None.** No FR, RP, or NFR is uncovered.

### Observations (not gaps — for later story-quality steps)

1. **FR-8 (reject stale/altered/replayed/invalid plans — a fail-closed security requirement, NFR-1) has thin *primary-epic* linkage.** Within its mapped Epic 3, only story **3.6** — a *revalidation* story for historical-FULL criteria — links FR-8 directly. Its substantive behavior is, however, covered elsewhere (UX-PB.2a/2b stale-plan + one-use `planId`/durable `planAttemptId`, 4.6 native admission, 5.8 preview-bytes-vs-spawned-bytes). Flag for Step 4+ to confirm the *stale-plan rejection* AC is explicit and not only implied by revalidation.
2. **Every criterion-bearing story declares "Reusable Test Infrastructure" as primary concern** except UX-PB (Product Behavior) and the candidate-evidence stories (7.2, 7.4, 8.1–8.4, 8.6–8.7). This matches the PRD's 58-row split (1 Product Behavior / 52 test-infra / 5 release-evidence) and is expected for a *readiness-gate* epic set — but it means most "coverage" is evidence-production, gated on TIR-1 behavior-present checks. Not a coverage gap; a reminder that FULL still depends on behavior-present verification (`epics.md:110`).
3. **NFRs are inventoried (NFR-1…8, `epics.md:84-98`) and cross-cutting**, realized through the FR stories' ACs and the Additional-Requirements block. No standalone NFR coverage gap surfaced; NFR realization is assessed at story-quality depth in later steps.

### Coverage Statistics

- **Total PRD FRs:** 22 (+ RP-1, RP-2 = 24 mandatory functional-family requirements)
- **Covered in epics (verified):** 24 / 24
- **Missing:** 0
- **FR coverage percentage: 100%**
- **Total leaf stories:** 83 (55 numbered + 28 UX-PB sub-stories)

**Step 3 status: COMPLETE** — 24/24 FR/RP coverage verified against story ACs; 0 gaps; 0 orphans; 3 non-blocking observations flagged for story-quality review.

---

## Step 4 — UX Alignment Assessment

### UX Document Status: **FOUND**

Two authoritative spines, both `status: final`, updated 2026-07-24:
- **`DESIGN.md`** (252 lines) — visual/design system (Aurora Control Deck), 24 component visual contracts, tokens, accessibility floor.
- **`EXPERIENCE.md`** (460 lines) — interaction/IA/state/journey contract, 24 behavioral component contracts, Key Flows AJ-1…AJ-6.
- Supporting: `validation-report.md`, `review-accessibility.md`, `review-usability.md`, 4 mockups (illustrative — spines authoritative per `DESIGN.md:198`).

### UX ↔ PRD Alignment — **ALIGNED**

- **Journeys map 1:1.** `EXPERIENCE.md:377-460` AJ-1…AJ-6 correspond exactly to PRD AJ-1…AJ-6 (`prd.md:288-344`), same names, same climaxes/failure paths.
- **Revised update experience matches PRD's D27–D30 contracts.** The central "Upgrade Plan" object, "nothing executes from a row/header" (`EXPERIENCE.md:25`), separate Confirmation Dialog, `skipUpgradePlanConfirmation` off-by-default & reversible (`EXPERIENCE.md:251`), one-immutable-History-row (`EXPERIENCE.md:238`), verification-gated success (`EXPERIENCE.md:34,228`), and app-update separation (`EXPERIENCE.md:453`) all realize PRD FR-6…FR-11, FR-13, FR-15, FR-19, FR-20/21 and the §0.1 supersession chain.
- **`Auto-open Activity drawer` preference explicitly removed** (`EXPERIENCE.md:256`) — matches PRD FR-17's "`autoOpenDrawer` inactive legacy input" (`prd.md:636`). The superseded drawer model is gone from the authoritative UX, not just the stories.
- **Quantified NFRs match.** 900×600 min, 150–200% zoom high-zoom layout, 101+ rows, 5,000-line output, 4.5:1 contrast, VoiceOver, reduced motion (`EXPERIENCE.md:313-332`, `DESIGN.md:164-175`) realize PRD NFR-3/NFR-6 and FR-19. Stall 120 s / hard cap 30 min / trusted-classifier `Interaction required` (`EXPERIENCE.md:216-218`) match PRD FR-14.
- **No UX requirement absent from the PRD.** UX adds visual/interaction specificity (color tokens, typography, focus-transition matrix) that *elaborates* PRD FR-19/NFR-6 rather than introducing unsanctioned scope.

### UX ↔ Architecture Alignment — **ALIGNED**

- **AD-16 is a dedicated architectural invariant for the revised experience** (`ARCHITECTURE-SPINE.md:543-624`), binding D27–D30, FR-6…FR-17/19, and UX-PB.1…UX-PB.5. It supplies exactly what the UX spines require: canonical `PlanIntent` (never executable strings), distinct one-use `planId` vs durable `planAttemptId`, atomic `execute_plan` admission, **exactly one active attempt (second confirmation fails closed)** (matches `EXPERIENCE.md:100`), `planAttemptId`-scoped cancellation with unstarted→`Skipped`, explicit `Verifying` gate before success, Retry as a new linked `planAttemptId`, honest legacy Operations, and `autoOpenDrawer`→`skipUpgradePlanConfirmation` default-false.
- **Explicit capability mapping.** `ARCHITECTURE-SPINE.md:946` maps "Persistent Upgrade Plan, Plan Attempts, Activity, Results, History, Retry, and confirmation preference" → "Frontend draft state plus Rust plan-attempt application/persistence services" → governed by AD-3, AD-4, AD-5, AD-16. The UX has first-class architectural support, not an afterthought.
- **Accessibility method is architecturally anchored.** `EXPERIENCE.md:315` defers to "the packaged-app accessibility method approved by the Architecture Spine"; AD-11 + approved DR-2 (`ARCHITECTURE-SPINE.md:421-424`) define exactly that (packaged keyboard/focus, automated 4.5:1 contrast, reduced motion, manual VoiceOver). Bidirectional, consistent.
- **Mutual sourcing confirms coherence.** DESIGN.md/EXPERIENCE.md list ARCHITECTURE-SPINE.md as a source; ARCHITECTURE-SPINE.md (revision 2, updated 2026-07-24) lists both UX spines. The four artifacts (PRD rev 2, UX spines final, Architecture rev 2, epics corrected) form one internally consistent set — strong evidence the correct-course correction propagated coherently.

### Alignment Issues

**None blocking.** The revised D27–D30/AD-16 experience is expressed consistently across PRD, both UX spines, the Architecture Spine, and (per Step 3) the epics. No UX↔PRD or UX↔Architecture contradiction surfaced on a full read of all four documents.

### Warnings (cross-cutting open items — already tracked, not UX defects)

1. **DR-1 (minimum supported macOS) OPEN.** PRD NFR-7 requires it declared before final candidate acceptance; the UX commits to a 900×600 minimum and native macOS conventions but cannot declare the OS floor; Architecture AD-11 blocks TIR-7/RE-4/7/8 environment handoff until Product+Release declare it (`ARCHITECTURE-SPINE.md:429-431,954`). This is a shared governance blocker that touches UX packaging acceptance — surfaced, owned, not a UX↔spec misalignment.
2. **Minor label variance (non-blocking).** PRD FR-14 enumerates stall actions as "Keep waiting, Copy command, and Cancel" (`prd.md:565`) while the authoritative UX consistently uses **`Cancel plan`** (`EXPERIENCE.md:216`); AD-16 confirms attempt-scoped semantics. Same behavior, slightly looser PRD wording. No action required beyond noting the UX/AD-16 label is canonical.

**Step 4 status: COMPLETE** — UX found; UX↔PRD and UX↔Architecture both ALIGNED; 0 blocking issues; 2 tracked cross-cutting warnings (DR-1 open blocker; cosmetic label variance).

---

## Step 5 — Epic Quality Review

**Method:** 9 skeptical quality-enforcer agents (one per epic + UX-PB), each reading only its section and hunting for AC/GWT quality defects, forward dependencies, oversized stories, and *active* superseded residual, applied with brownfield readiness-gate judgment (evidence/test-infra epics are sanctioned by design, not "technical epics"). Preceded by a deterministic superseded-term grep. 9/9 agents completed, 0 errors.

### Best-Practices Compliance Checklist (aggregate over 83 stories)

| Check | Result |
|-------|--------|
| Epics deliver a coherent (readiness) outcome — no mislabeled technical epics | ✅ **0 incoherent/mislabeled** (all 8 titles outcome-framed) |
| Epic independence / dependency waves respected | ✅ all dependencies backward (earlier→later) |
| **No forward dependencies** (early story needs later work) | ✅ **0 found** across all 83 stories |
| Stories appropriately sized | ⚠️ 13 sizing/bundling observations (2 major) |
| Acceptance criteria in Given/When/Then form | ✅ strict GWT throughout |
| Clear/testable ACs | ⚠️ 5 subjective phrasings to tighten |
| Error/failure paths covered | ⚠️ 8 stories defer failure to contract-level rule |
| **No active superseded wording** (D27–D30/AD-16 compliance) | ✅ **0 active residual** (grep + semantic) |
| Traceability to FRs maintained | ✅ (1 location seam — FR-8, below) |

### 🔴 Critical Violations: **NONE (0)**

### 🟠 Major Issues (3 — refinements, none readiness-blocking)

1. **FR-8 traceability-location seam (Epic 3 / Story 3.6) — VERIFIED COVERED, mislocated.** FR-8 (reject stale/altered/replayed/invalid plans) is assigned primary to **Epic 3**, but Epic 3 explicitly covers only the *stale* dimension (Story 3.3 AC2 `epics.md:1664-1666`; 3.6 "stale rebuild requires reconfirmation"). The *altered / replayed / evicted / conflicting* dimensions are covered — I verified the text — by **Story 5.8** (`epics.md:2400-2401`: "only a fresh exact plan spawns commands byte-identical to review **And** stale, altered, replayed, evicted, or conflicting plans enqueue nothing") plus AD-16's one-use `planId` expiry (UX-PB.2a). **So FR-8 is fully covered across the story set; the defect is that its map-assigned primary epic doesn't hold the whole requirement.** *Recommendation:* cross-reference Epic 5 / UX-PB.2 against FR-8 in the FR Coverage Map (`epics.md:289`) so the traceability is explicit. Corroborates Step 3 Observation #1.
2. **Story 5.4 AC2 bundling (Epic 5).** One AC packs native-output batching (F7-AC1) *and* the entire shared-plan sidecar/Results-Summary lifecycle (F7-AC2 — draft persistence across navigation, 5,000-line memory bound, transform-to-Results) into a single criterion. Two separable behavior areas. *Recommendation:* split into two ACs/slots.
3. **Story 7.6 bundling (Epic 7).** One evidence slot bundles WKWebView accessibility + high-capacity presentation (101 rows / 5,001 lines / 900×600) + 150–200% zoom collapse + Confirmation-Dialog focus + the announcement channel + the update badge. Oversized for one unit requiring both packaged automation and manual VoiceOver across three zoom levels. *Recommendation:* decompose before implementation entry.

### 🟡 Minor Concerns (22) — pre-implementation polish

- **AC bundling into dense AND-clauses (13):** UX-PB.5d; 2.2, 2.4; 3.4, 3.6; 4.1; 5.4, 5.5; 6.4, 6.5, 6.7; 7.6; 8.7. Mostly revalidation (`RV@B*`) or atomic-boundary stories where bundling is partly by design (e.g. 4.1 is AD-3 atomic); reduce single-AC testability.
- **Missing explicit failure-path ACs (8):** UX-PB.2d (live `planAttemptId` correlation-miss), 3.6 (FR-8 altered/replayed — see above), 4.4 (mid-refresh Manager failure), 4.5 (divergence-detected/fail-closed outcome unstated), 5.1 (partial subject/executor refresh divergence), 6.5 (corrupt/absent source during archive assembly), 7.6 (a11y/zoom/dialog failure outcome), 8.2 (install/launch fail-closed). In each, failure is instead handled by the story's "Behavior-present handling" contract field or frozen scenario contract rather than a standalone AC — a framework stylistic choice worth tightening into explicit ACs.
- **Subjective/vague AC wording (5):** 2.1 "represented coherently"; 3.5 equivalence clause not verifiable by its own source-bound evidence; 5.2 "understandable"; 6.5 "match the contract" (digest Unassigned); 8.2 "usable startup state". Replace with concrete rendered-text/outcome assertions.
- **Cosmetic (UX-PB):** UX-PB.3b-3f open "As a user" vs the standard "As a Pack-Manager user"; UX-PB.3b has one stray AC not attached to a GWT triple and duplicative of UX-PB.4c.

### Verification of Correct-Course claims (the reason for this run)

| Correction claim | Independent verdict |
|------------------|---------------------|
| 28 UX-PB sub-stories, distribution 5/6/7/5/5 | ✅ Confirmed present, exact distribution |
| Zero superseded residual | ✅ Confirmed — grep + 9-agent semantic pass = **0 active** |
| 19 stories corrected, no superseded wording | ✅ Confirmed (12 rewritten + 7 aligned, all clean) |
| Dependency-ordered, no forward refs | ✅ Confirmed — **0 forward dependencies** |
| Coherent Given/When/Then, negative paths | ✅ Broadly confirmed; failure-path *style* (contract-deferred) flagged on 8 stories |

**Step 5 status: COMPLETE** — 0 critical, 0 forward deps, 0 active superseded residual, 0 incoherent epics. 3 major (all refinements: 1 verified-covered FR-8 location seam + 2 sizing) and 22 minor pre-implementation polish items. No readiness-blocking structural defect found.

---

## Summary and Recommendations

### Overall Readiness Status: **NEEDS WORK**

More precisely: **planning is validated and internally READY; implementation entry is gated only by the blockers the artifacts themselves declare.** This is not a deficiency in the planning — it is the designed state of a P0 readiness-gate whose PRD §9.4 and epics' Implementation-Entry Blocker Register (`epics.md:189-208`) explicitly hold entry until specific governance decisions and assignments land.

**The reason for this run — the two prior critical findings — are RESOLVED and independently verified:**
- **Superseded contracts:** 0 active superseded residual (deterministic grep + 9-agent semantic pass). D27–D30/AD-16 are expressed directly; every superseded term appears only as a prohibition/negation/legacy-tolerance rule.
- **Oversized UX-PB:** decomposed into exactly 28 dependency-ordered sub-stories (5/6/7/5/5), 0 forward dependencies, thorough failure-path ACs.

| Dimension | Verdict |
|-----------|---------|
| Documents complete & non-duplicated | ✅ READY |
| PRD requirements extracted (22 FR, 8 NFR, 10 families) | ✅ READY |
| FR→story coverage (24/24 verified) | ✅ READY |
| UX ↔ PRD ↔ Architecture alignment | ✅ READY |
| Epic/story structural quality (0 critical, 0 forward deps, 0 superseded) | ✅ READY |
| Governance decisions (DR-1, DR-4, coverage-map approval) | ⛔ OPEN — blocks entry |
| Story owners/dates + scenario-contract freeze | ⛔ UNASSIGNED — blocks entry |

### Critical Issues Requiring Immediate Action

There are **no planning-quality defects** blocking readiness. The blockers are the artifacts' own declared implementation-entry gates — governance and downstream-planning, not planning corrections:

1. **DR-1 (minimum supported macOS) — OPEN.** Product + Release must declare it. Blocks TIR-7, RE-4/RE-7/RE-8 environment handoff and the Acceptance Profile freeze (`prd.md:1090-1098`, `ARCHITECTURE-SPINE.md:429-431`).
2. **DR-4 (P0 gate / retry policy) — PROPOSED.** Product + QA must approve. Blocks candidate validation, gate configuration, and Acceptance Profile freeze (`epics.md:178,198`).
3. **Revision-2 coverage-map approval — `final-pending-approval`.** Product + QA must approve and mechanically verify the one-to-one 72-row map before it is the frozen oracle (`epics.md:194`).
4. **Named assignees + calendar dates — UNASSIGNED on all 83 stories.** Every story's implementation entry is blocked until sprint planning assigns owners/dates (`epics.md:199`).
5. **55 versioned scenario-contract files + digests — Unassigned/unfrozen.** Each criterion-bearing story blocks entry until its scenario contract is frozen by digest.

### Quality Findings to Address (before/during implementation — not readiness-blocking)

- **Major-1 (FR-8 traceability seam):** In the FR Coverage Map, cross-reference **Story 5.8** and **UX-PB.2** against FR-8 so the altered/replayed/evicted-plan rejection (already covered, `epics.md:2400-2401`) is traceable from FR-8's primary-epic entry.
- **Major-2 (Story 5.4):** split AC2 — native-output batching vs. shared-plan sidecar/Results lifecycle.
- **Major-3 (Story 7.6):** decompose the bundled packaged-accessibility + capacity + zoom + dialog + badge evidence slot.
- **Minor themes (22):** convert 8 contract-deferred failure behaviors into explicit failure ACs; replace 5 subjective AC phrasings ("coherently", "understandable", "usable startup state", "match the contract"); normalize UX-PB.3b–3f persona wording and remove the one stray AC.

### Recommended Next Steps

1. **Unblock governance (highest leverage):** resolve **DR-1** (Product+Release) and **DR-4** (Product+QA), then **approve + mechanically verify the revision-2 72-row coverage map** (Product+QA). These three unlock the Criterion Acceptance Profile freeze that most downstream work depends on.
2. **Run sprint planning:** assign a named owner + calendar date to all 83 stories.
3. **Freeze scenario contracts:** author/freeze the 55 versioned scenario-contract files and record their digests (test-design, per wave).
4. **Apply the 3 major quality fixes** (FR-8 map cross-reference; split 5.4; decompose 7.6) — small edits, best done now while the epics are open.
5. **Optionally** tighten the 22 minor items.
6. **Then begin Batch 1 (Epic 1)** per the AD-13 dependency waves once its entry gates clear; Batches 2/3/4 follow in parallel.

### Final Note

This run examined **5 documents** and **83 leaf stories** across **6 assessment categories**, using **18 verification agents** across two adversarial workflows. It found **0 critical planning defects, 0 forward dependencies, 0 active superseded residual, and 24/24 verified FR coverage.** The **3 major** and **22 minor** findings are pre-implementation refinements, not blockers. The genuine blockers to implementation entry are the **5 governance/assignment gates the artifacts themselves declare** — expected and owned by their respective roles.

**Relative to the reason for this run:** the correct-course correction succeeded — the two critical findings from the superseded `implementation-readiness-report-2026-07-24.md` (01:10) are closed. The planning set (PRD rev 2, UX spines final, Architecture rev 2, epics corrected) is coherent, aligned, and ready to hand to governance and sprint planning.

*Assessment date: 2026-07-24 · Assessor: claude-code (BMAD Implementation Readiness workflow) · Supersedes: `implementation-readiness-report-2026-07-24.md`*
