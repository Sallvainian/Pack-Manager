---
title: "TEA Test Design → BMAD Handoff Document"
version: "2.1"
workflowType: "testarch-test-design-handoff"
workflowStatus: "complete"
runMode: "system-level"
runIntent: "authoritative-reconciliation"
inputDocuments:
  - "_bmad-output/planning-artifacts/prds/prd-Pack-Manager-2026-07-22/prd.md"
  - "_bmad-output/planning-artifacts/prds/prd-Pack-Manager-2026-07-22/addendum.md"
  - "_bmad-output/planning-artifacts/architecture/architecture-Pack-Manager-2026-07-23/ARCHITECTURE-SPINE.md"
  - "_bmad-output/planning-artifacts/prds/prd-Pack-Manager-2026-07-22/readiness-coverage-map.md"
  - "docs/SPEC.md"
  - "docs/DECISIONS.md"
  - "_bmad-output/test-artifacts/traceability-matrix.md"
  - "_bmad-output/test-artifacts/test-design-architecture.md"
  - "_bmad-output/test-artifacts/test-design-qa.md"
  - "_bmad-output/test-artifacts/test-design-progress.md"
  - "_bmad-output/planning-artifacts/ux-designs/ux-Pack-Manager-2026-07-23/EXPERIENCE.md"
  - "_bmad-output/planning-artifacts/sprint-change-proposal-2026-07-24.md"
sourceWorkflow: "testarch-test-design"
generatedBy: "TEA Master Test Architect"
generatedAt: "2026-07-24"
projectName: "Pack-Manager"
coverageMapStatus: "final-pending-approval"
planningGate: "FAIL — 14/72 FULL baseline"
readinessClaim: "none"
---

# TEA → BMAD Integration Handoff

## Purpose

This document is the implementation-planning bridge from the reconciled
System-Level Test Design to `create-epics-and-stories`. It governs the eight
closure batches and their later epics and stories without redesigning the
application or claiming that planned evidence exists.

The planning baseline remains **FAIL with 14/72 P0 criteria FULL**. The 58
non-FULL criteria require closure work, and all 14 historical baseline-FULL
criteria require candidate-era revalidation at their mapped evidence depth.
Nothing in this handoff promotes a criterion, regenerates the traceability
gate, approves the coverage map, or claims release readiness.

### 2026-07-24 Correct Course handoff amendment

The Product Behavior Prerequisite UX-PB.1..UX-PB.5 is now mandatory before
affected Epic 3–7 evidence stories. Downstream work must use D27-D30 and AD-16:
persistent plan membership, separate confirmation, durable
`planAttemptId`, one active attempt, Activity/Results/verification,
attempt-wide cancellation, trusted interaction classification, one History row
per attempt, replay/Retry lineage, and the revised safety setting.

The map's 14/72 and 1/52/5 figures remain historical revision-1 planning data.
They must be mechanically reclassified after implementation. `AUT-003` is
superseded and cannot support revised `F5-AC3`.

### Source Authority

| Order | Authority                              | Use in downstream planning                                                                                     |
| ----: | -------------------------------------- | -------------------------------------------------------------------------------------------------------------- |
|     1 | Finalized PRD and addendum             | Product/readiness requirements and closure-batch handoff                                                       |
|     2 | Final Architecture Spine               | Binding AD-1..AD-16 invariants, ASR contracts, product prerequisite, evidence model, and dependency boundaries |
|     3 | `readiness-coverage-map.md`            | Normative 72-row reconciliation map; preserve `final-pending-approval`                                         |
|     4 | `docs/SPEC.md` and `docs/DECISIONS.md` | Settled product behavior                                                                                       |
|     5 | `traceability-matrix.md`               | Historical FAIL / 14-of-72-FULL baseline only                                                                  |
|     6 | Reconciled Test Design outputs         | Risk, test-level, lane, depth, batch, estimate, and story-planning guidance                                    |

The exact 72-row primary planning matrix is in
`_bmad-output/test-artifacts/test-design-qa.md` and
`_bmad-output/test-artifacts/test-design-progress.md`. Epic/story generation
must consume it without changing the denominator or treating the 14 baseline
FULL rows as current-candidate proof.

## TEA Artifacts Inventory

| Artifact                 | Path                                                                                                      | BMAD integration point                                                                            |
| ------------------------ | --------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------- |
| Architecture Test Design | `_bmad-output/test-artifacts/test-design-architecture.md`                                                 | ASR acceptance, NFR testability, risk mitigations, and architecture blockers                      |
| QA Test Design           | `_bmad-output/test-artifacts/test-design-qa.md`                                                           | Exact 72-row mapping, batch scenarios, evidence requirements, execution strategy, and QA estimate |
| Reconciliation record    | `_bmad-output/test-artifacts/test-design-progress.md`                                                     | Authority reconciliation, risk rationale, matrix validation, and workflow checklist evidence      |
| Formal architecture      | `_bmad-output/planning-artifacts/architecture/architecture-Pack-Manager-2026-07-23/ARCHITECTURE-SPINE.md` | Binding cross-cutting contracts that stories may not weaken                                       |
| Normative map            | `_bmad-output/planning-artifacts/prds/prd-Pack-Manager-2026-07-22/readiness-coverage-map.md`              | Exact 72 IDs and historical statuses; approval still pending                                      |
| Baseline snapshot        | `_bmad-output/test-artifacts/traceability-matrix.md`                                                      | Historical 14/72 FULL only; not candidate evidence                                                |

## Epic-Level Integration Guidance

### Non-Negotiable Planning Model

Every work item declares exactly one primary readiness concern:

1. **Product Behavior** — missing or incorrect behavior is corrected before
   regression coverage can receive credit.
2. **Reusable Test Infrastructure** — supplies repeatable capabilities and
   results but never a readiness verdict.
3. **Candidate-Specific Release Evidence** — consumes one immutable candidate
   and never changes product behavior or the infrastructure oracle.

Every execution also declares exactly one non-substitutable lane and one
minimum binding depth:

| Execution lane           | Permitted proof                                                                                                                             | Prohibited substitution                                                              |
| ------------------------ | ------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------ |
| `forced-offline`         | Source-bound or controlled environment-bound evidence from a clean checkout after pinned preparation, with outbound network denied          | No real Manager invocation, undeclared host state, live endpoint, or candidate claim |
| `provisioned-target-mac` | Serialized environment-bound evidence from the designated, drift-checked Mac and versioned provision profile                                | Cannot be relabeled candidate-bound or replaced by deterministic fixtures            |
| `candidate-release`      | Candidate-bound evidence using the exact signed candidate, approved endpoints/OS services, Apple silicon, and physical Intel where required | Credentialless or `--no-sign` output is inadmissible                                 |

The open-row concern split remains the map-revision-1 planning baseline:
**1 Product Behavior, 52 Reusable Test Infrastructure, and 5
Candidate-Specific Release Evidence**. The 24 behavior-present (`BP`) rows are
provisional: absent or incorrect behavior returns the row to Product Behavior
through a reviewed map revision. The 14 baseline-FULL rows are outside that
58-row split and receive revalidation checkpoints without changing their
historical status.

### Exact Eight-Batch Dependency Graph

```text
Batch 1
  ├─> Batch 2 ─┐
  ├─> Batch 3 ─┼─> Batches 1–6 exits accepted
  └─> Batch 4 ─┤
        ├─> Batch 5 ─┤
        └─> Batch 6 ─┘
                    └─> Release preparation and candidate freeze
                          └─> Batch 7
                                └─> Batch 8
                                      └─> later Trace workflow
```

Release preparation is an evidence prerequisite, **not a ninth closure
batch**. Batches 2, 3, and 4 may proceed in parallel after Batch 1. Batches 5
and 6 may proceed in parallel only after the accepted Batch 4 native
foundation.

### Batch and Revalidation Allocation

`C` below means “eligible for later FULL reassessment only after all frozen
profile slots pass, the Evidence Index validates, and a separate Trace
workflow regenerates the candidate-bound decision.” `RV` means historical
baseline FULL must be revalidated and is not carried forward.

| Batch / recommended epic                              | Open-row count and IDs                                                                                                 | Historical-FULL revalidation checkpoint                                       | Entry and exit boundary                                                                                                                                                                                                                                        |
| ----------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **1 — Restore the `mas` oracle**                      | **5:** `F1-AC7`, `D23a-AC1`, `D23a-AC2`, `D23a-AC4`, `D23a-AC5`                                                        | `D23a-AC3` (`RV@B1`)                                                          | Runs first. PC-1/product correction precedes recurrence credit; real-capture provenance and dated target-Mac topology are retained.                                                                                                                            |
| **2 — Detection and refresh**                         | **5:** `F1-AC6`, `F1-AC8`, `F2-AC3`, `F2-AC6`, `F2-AC8`                                                                | `F1-AC5`, `F2-AC2`, `F2-AC5`, `F2-AC7` (`RV@B2`)                              | After B1; may run with B3/B4. Behavior-present checks precede test-only work; forced-offline deterministic outcomes cannot borrow target-Mac evidence.                                                                                                         |
| **3 — Package, plan, and Settings**                   | **11:** `F3-AC1`, `F3-AC2`, `F3-AC3`, `F3-AC4`, `F3-AC6`, `F3-AC8`, `F4-AC1`, `F5-AC1`, `F5-AC3`, `F11-AC2`, `F11-AC3` | `F3-AC5`, `F3-AC7`, `F4-AC3` (`RV@B3`)                                        | After B1; may run with B2/B4. Covers behavior-present state, keyboard, plan, error, clipboard, and Settings contracts.                                                                                                                                         |
| **4 — Shared native boundary foundation**             | **5:** `F1-AC1`, `F1-AC2`, `F1-AC3`, `F1-AC4`, `F2-AC1`                                                                | `F2-AC4`, `F4-AC2`, `F5-AC2`, `F12-AC3` (`RV@B4`)                             | ASR-01 accepted by B4 exit. Catalog, production registration, Rust/TypeScript wire contracts, wrappers/subscriptions, inventory, and real native coverage have set equality.                                                                                   |
| **5 — Manager update and process lifecycle**          | **12:** `F2-AC9`, `F6-AC1`–`F6-AC5`, `F7-AC1`–`F7-AC4`, `D26-AC1`, `D26-AC2`                                           | `F4-AC4`, `F4-AC5` (`RV@B5`)                                                  | Requires B4 and ASR-02 core. Controlled output, route, spawn/no-spawn, locks, events, null stdin, cancellation, escalation, stall, timeout, shutdown, and D26 boundaries are retained.                                                                         |
| **6 — Crash, persistence, diagnostics, and Settings** | **10:** `F8-AC1`–`F8-AC4`, `F9-AC1`–`F9-AC4`, `F11-AC1`, `F11-AC4`                                                     | None; final association still requires the frozen profile                     | Requires B4, ASR-02 filesystem controls, and ASR-03. Disposable roots prove crash/relaunch, Interrupted reconstruction, historical-PGID non-signal, persistence, retention, diagnostics privacy, and native actions.                                           |
| **Release preparation — prerequisite only**           | **0 denominator rows**                                                                                                 | No status movement                                                            | Begins after B1–B6 exits. Release accepts the locked v1 evidence contract and transport/retention; one clean GitHub Actions attempt freezes the fully packaged, signed, notarized, stapled candidate, final updater metadata, and Candidate Identity Manifest. |
| **7 — Packaged accessibility and updater**            | **4:** `F10-AC1`, `D25-AC2`, `D25-AC3`, `D25-AC4`                                                                      | None; all earlier RV results must remain eligible for the same profile/source | Uses only the frozen manifest. Applies approved DR-2; exercises installed-prior-version update, refusal during active operations, and non-writable/no-admin behavior. DR-3 requires Apple silicon and physical Intel execution.                                |
| **8 — Release and reproducibility evidence**          | **6:** `F10-AC2`, `F10-AC3`, `F10-AC4`, `F12-AC1`, `F12-AC2`, `D25A-AC2`                                               | All 14 RV checkpoints must be admitted at their mapped lane/depth             | Follows B1–B7 against the unchanged candidate. Completes artifact/trust/launch proof and ledger replay. `F12-AC1/2` remain forced-offline source evidence associated only when source/profile match. Final trace is planned but not run here.                  |

The open-row batch total remains
`5 + 5 + 11 + 5 + 12 + 10 + 4 + 6 = 58`. The 14 RV checkpoints do not
change those counts or the 72-row denominator.

### ASR Accountability and Delivery Boundaries

Exactly one role is accountable for each enabler. Supporting functions do not
share accountability.

| Enabler                                           | Accountable role | Capability/support                               | Binding delivery boundary                                                      | Story-level acceptance boundary                                                                                                                                                                                                                                                                         |
| ------------------------------------------------- | ---------------- | ------------------------------------------------ | ------------------------------------------------------------------------------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| ASR-01 — shared production command/event boundary | **Architecture** | Development and QA implement/use                 | Accepted by B4 exit; required before B5–B7                                     | Versioned catalog, production registration, wire schemas, wrappers/subscriptions, and inventory have set equality; every catalog command round-trips and every catalog event dispatches through the real isolated Tauri boundary. The verified 20 commands/six events are a baseline, not fixed counts. |
| ASR-02 — deterministic process and OS controls    | **Development**  | Platform capability area                         | Core before B5; filesystem/updater extensions before B6–B7                     | Typed ports and controlled helpers produce every required output, exit, signal, timeout, lock, stdin, path, permission, opener, restart, and updater condition while production adapters remain fail-closed.                                                                                            |
| ASR-03 — disposable lifecycle environment         | **QA**           | Development/Platform supports                    | Accepted before B6                                                             | Crash, forced quit, relaunch, persistence, retention, hostile filesystem, and historical-PGID non-signal run only from disposable roots with retained evidence.                                                                                                                                         |
| ASR-04 — candidate identity and attestation       | **Release**      | Evidence Registrar and existing GitHub transport | Contract before release preparation; manifest before B7; complete ledger in B8 | Manifest and ledger pass schema, JCS, digest, chain, artifact, provenance, retry, append-authority, and invalidation validation; accepted bytes are write-once.                                                                                                                                         |
| ASR-05 — split evidence lanes                     | **QA**           | CI is the execution mechanism                    | Separation before any B1 evidence; candidate lane before B7                    | Workspaces, credentials, caches, provenance, and outputs for all three lanes are isolated; the aggregator rejects cross-lane or shallower-depth substitution.                                                                                                                                           |

If an accountable role cannot accept its enabler by the stated boundary, the
affected batch and every downstream dependent batch remain blocked.

### Risk References

| Risk                                                         | Score | Epic-level mitigation requirement                                                                                                                     |
| ------------------------------------------------------------ | ----: | ----------------------------------------------------------------------------------------------------------------------------------------------------- |
| `R-001` — source/capture/target-Mac oracle drift             |     6 | Complete PC-1, preserve capture provenance, and reject unreported provisioned-host drift in B1.                                                       |
| `R-002` — divergent frontend/native contract                 |     6 | Deliver ASR-01 and atomic catalog/registration/wire/native-coverage changes by B4.                                                                    |
| `R-003` — misleading or inaccessible user-visible state      |     6 | Use behavior-present checks, coherent-state/error assertions, keyboard semantics, and approved packaged accessibility.                                |
| `R-004` — unsafe or dishonest process lifecycle              |     6 | Deliver ASR-02/03 and correlate child, signal, lock, event, journal, transcript, and cleanup results.                                                 |
| `R-005` — durability/privacy failure                         |     6 | Prove atomicity, corruption handling, retention, exact archive composition, inherited-value exclusion, and path/symlink rejection in disposable data. |
| `R-006` — updater integrity/install divergence               |     6 | Bind prior-version check/download/verify/refuse/install/relaunch and non-writable outcomes to one manifest/profile.                                   |
| `R-007` — invalid or unlaunchable release                    |     9 | Deliver ASR-04; inspect the exact candidate and prove trust/launch on Apple silicon and physical Intel before ledger replay.                          |
| `R-008` — lane, provenance, retry, or candidate substitution |     6 | Deliver ASR-05; deny cross-lane/depth substitution, ignored/unexecuted credit, retry laundering, and candidate/profile mismatch.                      |

All eight risks remain open planning risks. No mitigation is marked complete,
waived, or accepted by this handoff.

### Epic/Story Entry Blockers

| Decision or dependency                                   | Status                     | Decision/accountable role                    | Required by                                                              | Handoff consequence                                                                                                                                                              |
| -------------------------------------------------------- | -------------------------- | -------------------------------------------- | ------------------------------------------------------------------------ | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Normative map approval                                   | **final-pending-approval** | Product and QA                               | Before implementation treats it as the frozen oracle                     | Profile freeze and criterion-evidence admission are blocked; do not approve or alter the map here.                                                                               |
| DR-1 — minimum supported macOS                           | **OPEN**                   | Product and Release                          | Before TIR-7 or RE-4/RE-7/RE-8 environment implementation handoff        | Packaged OS matrix, compatibility slots, and profile freeze remain blocked.                                                                                                      |
| DR-2 — packaged accessibility method                     | **APPROVED**               | QA executes                                  | Candidate lane before B7; evidence complete in B8                        | Hybrid keyboard/focus, automated 4.5:1 contrast, reduced motion, and manual VoiceOver are binding methods, not existing evidence.                                                |
| DR-3 — physical Intel requirement                        | **APPROVED**               | QA executes                                  | Physical host before B7; evidence complete in B8                         | Intel fresh-install, Finder/Dock, and prior-version update are mandatory; universal-header inspection cannot substitute.                                                         |
| DR-4 — P0-specific gate/retry policy                     | **PROPOSED**               | Product and QA governance                    | Before profile retry values, candidate validation, or gate configuration | Do not apply the legacy 80% P1 rule or generic 95%/80% defaults.                                                                                                                 |
| Named individuals and calendar dates                     | **UNASSIGNED — BLOCKER**   | Downstream planning must assign              | Before each affected work item enters implementation                     | Role accountability and batch-relative deadlines are binding, but no person/date is invented here.                                                                               |
| Native harness/test runner                               | **Deferred**               | Architecture accepts; Development implements | B4 exit                                                                  | Must satisfy AD-2/AD-3; fake IPC, duplicated registration, or a test-only business path is non-conforming.                                                                       |
| Controlled-helper implementation language                | **Deferred**               | Development                                  | Before B5                                                                | Must satisfy AD-4 and add no production shell-command surface.                                                                                                                   |
| Evidence transport and retention duration                | **Deferred — BLOCKER**     | Release                                      | Before release preparation                                               | Must provide protected Registrar identity, lock/CAS, one head, idempotency, write-once objects, complete-set retention, and audit availability in the existing GitHub framework. |
| Provisioned target-Mac access/profile                    | **Execution dependency**   | QA                                           | Before qualified target-Mac collection                                   | Environment-bound evidence cannot move to another lane.                                                                                                                          |
| Apple-silicon and physical Intel hosts                   | **Execution dependency**   | QA                                           | Before candidate lane is operational                                     | Missing physical Intel evidence blocks eligibility for a 100% decision.                                                                                                          |
| Installed prior public version                           | **Execution dependency**   | QA with Release support                      | Before B7 updater execution                                              | Update acceptance must start from an actually installed prior public version on both architectures.                                                                              |
| Signing/notarization/updater credentials                 | **Execution dependency**   | Release                                      | Before candidate freeze                                                  | Missing credentials fail candidate preparation closed; secrets never enter evidence.                                                                                             |
| Evidence/profile approval records and scenario contracts | **Execution dependency**   | Product/QA for policy; QA for profile        | Before profile freeze                                                    | Every slot must name immutable map, policy, approval, and scenario bytes by digest.                                                                                              |

Epic/story creation may encode these as prerequisite work and explicit
blockers. It may not silently resolve them or let blocked criterion work enter
implementation.

## Story-Level Integration Guidance

### Required Story Contract

Every criterion-bearing story must carry:

1. Exact criterion ID(s), historical baseline status, and one primary
   readiness concern from the reconciled 72-row matrix.
2. One batch/revalidation point, required test level, execution lane, minimum
   binding depth, risk links, ASR links, and predecessor dependencies.
3. A behavior-present check for every `BP` row. Missing or incorrect behavior
   creates Product Behavior work before regression credit.
4. A versioned scenario-contract path/digest and the exact evidence artifact
   expected from execution. A plan, collector, ignored test, or green build is
   not the artifact.
5. The accountable role, supporting capability, and explicit implementation
   assignee/date fields. Unassigned person/date fields block implementation
   entry.
6. Acceptance wording that says “eligible for later FULL reassessment,” never
   “moves to FULL” or “proves readiness.”
7. For candidate-bound work, the exact Candidate Manifest subjects and
   invalidation consequence. Rebuild, re-sign, re-tag, repackage, replacement,
   metadata change, new release-build workflow run, or new release-build run
   attempt starts a new evidence root and reruns affected B7/B8 slots.
8. First-attempt retention and `runnerRetryCount = "0"`. An
   evidence-collection retry against the unchanged candidate is a new linked
   record under the still-PROPOSED DR-4 policy; it cannot hide the first
   failure and does not create a new Candidate Manifest.

The QA Test Design is the source for the exact 72 criterion mappings. Do not
duplicate a criterion into multiple primary stories. Secondary slots may be
referenced from the same primary story or a linked enabler/evidence story.

### Evidence Contract Acceptance Criteria

These are ASR qualification requirements and do not add denominator rows:

#### Candidate Identity Manifest

- The `/v1` contract is the exact byte set of the three strict JSON Schema
  Draft 2020-12 schemas, canonicalization vectors, and `contract-lock.json`.
  Changing a locked byte requires `/v2`.
- Inputs satisfy I-JSON; unknown/duplicate keys, JSON numbers, invalid Unicode,
  non-NFC strings, invalid ordering, BOMs, and non-canonical bytes fail closed.
- Identity/profile values use RFC 8785 JCS UTF-8 bytes with no trailing
  newline. All digests are lowercase `sha256:<64-hex>` over the specified
  exact bytes, and independent implementations pass the shared vectors.
- The manifest contains identity only: clean source/commit/tag/lockfiles,
  immutable GitHub Actions run/attempt and toolchains, coherent versions,
  universal target, signing/updater-key identity, and exactly five final
  artifacts: direct app ZIP, DMG, updater archive, updater metadata, and
  updater signature.

#### Criterion Acceptance Profile

- The profile preserves all 72 criterion IDs and adds RP-1/RP-2 slots outside
  the denominator. It binds approved map/policy records, DR-1 minimum macOS,
  scenario contracts, concern, lane, depth, candidate subjects, OS/architecture
  matrix, physical/packaged requirements, and retry disposition.
- It cannot freeze while the map is pending approval, DR-1 is OPEN, or DR-4 is
  PROPOSED. A profile change creates a new Evidence Set namespace and requires
  revalidation without renaming unchanged candidate artifacts.

#### Evidence Index and Registrar

- Each evidence record is strict canonical
  `JCS({"payload": P, "recordDigest": D}) + LF`; the payload digest, sequence,
  predecessor, index digest, source/environment/candidate provenance,
  subject/result hashes, attempt data, and human-readable results reconcile
  exactly.
- The Release-controlled Registrar is the sole append authority through one
  allowlisted workflow identity and protected GitHub environment. Producers
  submit immutable attempt bundles but cannot edit the index.
- Candidate/profile locking or CAS, a single head, idempotency, stale/fork
  rejection, monotonically named immutable snapshots, write-once objects, and
  full retention prevent clobbering and retry laundering.
- PASS requires collected = executed = passed > 0 with failed, errored,
  skipped, ignored, cancelled, filtered, and unreported counts all zero.
  Automatic retries, missing first attempts, branches, wrong lane/depth/source,
  candidate mismatch, incomplete subjects, or conflicting human reports fail
  closed.

### Packaged-App Acceptance Boundary

- Use the exact installed candidate inside its packaged WKWebView; dev-server
  browser or DOM-only checks support diagnosis but cannot satisfy this
  boundary.
- Inspect the exact app, DMG, ZIP, updater archive/signature, resources,
  entitlements, architectures, Developer ID signatures, notarization,
  stapling, Gatekeeper result, metadata, URLs, embedded key, and versions.
- Fresh-install and Finder/Dock launch run on Apple silicon and physical
  Intel. Prior-version check, verified download, explicit install, and relaunch
  reach the same unchanged candidate on both architectures.
- Apply approved DR-2 accessibility and approved DR-3 physical-Intel
  requirements. DR-1 still blocks the minimum-macOS matrix.
- `--no-sign` and credentialless builds are non-candidate smoke only and never
  satisfy a candidate-release slot.

### Release Prerequisites Outside the Denominator

| Prerequisite | Retained consequences                                                                                               | Planning point              | Required validation                                                          |
| ------------ | ------------------------------------------------------------------------------------------------------------------- | --------------------------- | ---------------------------------------------------------------------------- |
| RP-1         | `D25-AC1` scheduled/menu update checks and `D25-AC5` update-state rehydration without Package History contamination | B7, final association in B8 | Installed-prior-version packaged updater acceptance plus state/menu contract |
| RP-2         | `D25A-AC1` standard Edit and Window actions in the custom macOS menu                                                | B7, final association in B8 | Installed packaged-app native-menu keyboard/interaction acceptance           |

RP-1 and RP-2 receive frozen profile slots but never enter the 72-row
denominator, status totals, primary-concern totals, or batch counts.

### Data-TestId Requirements

- Prefer accessible roles, labels, focus state, and visible status text.
- Add a stable test ID only when native event/domain state has no reliable
  semantic locator: operation identity, self-update route, Activity stream,
  History record, updater state, or packaged launch-ready marker.
- Test IDs identify domain state, not CSS or layout, and remain stable across
  visual changes.

## Risk-to-Story Mapping

| Risk ID | Category   | P×I | Recommended epic/story point          | Required validation level                                      |
| ------- | ---------- | --: | ------------------------------------- | -------------------------------------------------------------- |
| `R-001` | OPS / DATA | 3×2 | B1                                    | Unit/contract plus provisioned target-Mac                      |
| `R-002` | TECH       | 2×3 | ASR-01 and B4; consumed by B5–B7      | Real native Tauri boundary                                     |
| `R-003` | BUS / TECH | 2×3 | B2/B3 and packaged B7                 | Component/browser plus packaged-app acceptance                 |
| `R-004` | SEC / OPS  | 2×3 | ASR-02/03, B5/B6                      | Controlled native process and lifecycle                        |
| `R-005` | DATA / SEC | 2×3 | B6                                    | Unit/contract plus disposable native filesystem/lifecycle      |
| `R-006` | SEC / OPS  | 2×3 | Release preparation, B7, ledger in B8 | Candidate-bound installed updater                              |
| `R-007` | OPS / SEC  | 3×3 | ASR-04, release preparation, B8       | Candidate artifact/trust/launch attestation plus ledger replay |
| `R-008` | TECH / OPS | 2×3 | ASR-05 before B1; all batches         | Lane/provenance/profile/attempt contract validation            |

## Estimate and Scheduling Boundary

The reconciled QA-only planning estimate is **approximately 9–15 QA
engineer-weeks after all required dependencies are ready**. It includes the 58
closure rows, 14 revalidations, RP slots, evidence-contract qualification,
three-lane admission, packaged VoiceOver work, mandatory physical Intel
execution, evidence review, and expected debugging.

It excludes Development/Platform/Release implementation, hardware or
credential waiting time, evidence-transport implementation, named-person
scheduling, and candidate-invalidating reruns. Non-QA implementation effort
remains TBD for epic/story estimation. Calendar dates remain unassigned; each
downstream work item must receive one named person and one date before
implementation entry.

## Recommended BMAD → TEA Workflow Sequence

1. **TEA System-Level Test Design** — produces this reconciled planning
   handoff; no evidence execution or status change.
2. **BMAD Create Epics & Stories** — creates the eight dependency-aware
   batches, ASR enablers, release-preparation prerequisite, decision blockers,
   and RP work without adding a ninth batch.
3. **Governance and assignment** — Product/QA approve the map and DR-4,
   Product/Release resolve DR-1, Release resolves transport/retention, and
   downstream planning assigns people/dates at the required boundaries.
4. **TEA ATDD and BMAD implementation** — separate workflows, run one approved
   story at a time; behavior correction precedes regression scaffolds.
5. **TEA Automate / qualified execution** — produces reusable forced-offline
   and provisioned-host results at their declared depth; it does not infer
   readiness.
6. **Release preparation** — after B1–B6 exits, freeze one exact candidate,
   manifest, profile, and candidate-release lane. This is a prerequisite, not
   Batch 9.
7. **Batches 7 and 8** — execute and append evidence for the unchanged
   candidate, including RP slots and all 14 revalidations at mapped depth.
8. **TEA Trace** — a later, explicitly invoked workflow replays the evidence,
   regenerates traceability, and makes the candidate-bound gate decision.

## Phase Transition Quality Gates

| From phase          | To phase                   | Gate criteria                                                                                                                                                                                                                                                                                                                                                        |
| ------------------- | -------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Test Design         | Epic/Story Creation        | Exactly 72 P0 criteria mapped once; 58 closure rows and 14 RV checkpoints accounted for; eight batch counts/DAG preserved; all eight risks and five ASRs linked; RP-1/RP-2 separate; decision/execution blockers explicit.                                                                                                                                           |
| Epic/Story Creation | Story implementation entry | Story preserves criterion/concern/lane/depth/batch/ASR/risk/evidence contract; behavior-present rule applied; accountable role plus named assignee/date present; no blocked decision is silently resolved.                                                                                                                                                           |
| Any execution       | Evidence admission         | ASR-05 accepted; approved map, resolved DR-1, approved DR-4, frozen profile, valid scenario contract, and operational ASR-04 Registrar exist for the applicable slot.                                                                                                                                                                                                |
| Batch 1             | Batches 2–4                | PC-1 correction precedes recurrence credit; real-capture provenance and dated target-Mac evidence retained.                                                                                                                                                                                                                                                          |
| Batch 4             | Batches 5–6                | ASR-01 accepted; ASR-02 core ready; B5/B6-specific ASR-02/03 dependencies assigned.                                                                                                                                                                                                                                                                                  |
| Batches 1–6         | Release preparation        | Required exits accepted; v1 contract/transport/retention, profile inputs, hosts, prior version, and credentials ready.                                                                                                                                                                                                                                               |
| Release preparation | Batch 7                    | One clean, fully packaged/signed/notarized/stapled candidate and canonical manifest frozen; candidate lane operational; no-sign artifacts excluded.                                                                                                                                                                                                                  |
| Batch 7             | Batch 8                    | Same manifest remains valid; required Apple-silicon/physical-Intel packaged and updater records appended without identity mutation.                                                                                                                                                                                                                                  |
| Batch 8             | Later Trace                | Complete single-head index replays; all required 72/RP slots and first attempts are present at exact lane/depth; all high-risk mitigations have admitted evidence at their mapped provenance depth and are associated with the exact candidate Evidence Set where permitted, without relabeling provenance. This is eligibility to run Trace, not a readiness claim. |

## Handoff Validation

- [x] Authority order reflects the finalized PRD and formal Architecture Spine.
- [x] Coverage-map status remains `final-pending-approval`.
- [x] Planning baseline remains FAIL with 14/72 FULL; no criterion was promoted.
- [x] All 58 non-FULL rows and all 14 revalidation checkpoints are represented.
- [x] Open-row counts remain `5/5/11/5/12/10/4/6`.
- [x] RP-1/RP-2 remain mandatory and outside the denominator.
- [x] ASR-01..ASR-05 have exactly one accountable role and binding deadlines.
- [x] Three readiness concerns, three execution lanes, and three evidence depths remain distinct.
- [x] Manifest/Profile/Index/Registrar/JCS/SHA-256, first-failure, append-authority, and invalidation rules are carried forward.
- [x] Release preparation is a prerequisite and not a ninth batch.
- [x] DR-1 remains OPEN, DR-2/DR-3 APPROVED, and DR-4 PROPOSED.
- [x] Estimates use ranges and do not invent names or calendar dates.
- [x] No product, test, infrastructure, CI, release-workflow, configuration, or traceability change is authorized by this document.

This handoff is ready for `create-epics-and-stories` as a planning input. Its
explicit blockers remain binding implementation-entry conditions, and its
completion is not evidence of product-and-release readiness.
