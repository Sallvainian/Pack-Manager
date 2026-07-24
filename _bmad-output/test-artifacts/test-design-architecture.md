---
workflowStatus: "completed"
documentStatus: "planning-complete"
workflowType: "testarch-test-design"
mode: "system-level"
lastSaved: "2026-07-24"
inputDocuments:
  - "_bmad-output/planning-artifacts/prds/prd-Pack-Manager-2026-07-22/prd.md"
  - "_bmad-output/planning-artifacts/prds/prd-Pack-Manager-2026-07-22/addendum.md"
  - "_bmad-output/planning-artifacts/architecture/architecture-Pack-Manager-2026-07-23/ARCHITECTURE-SPINE.md"
  - "_bmad-output/planning-artifacts/prds/prd-Pack-Manager-2026-07-22/readiness-coverage-map.md"
  - "docs/SPEC.md"
  - "docs/DECISIONS.md"
  - "_bmad-output/test-artifacts/traceability-matrix.md"
  - "_bmad-output/test-artifacts/test-design-progress.md"
  - "_bmad-output/planning-artifacts/ux-designs/ux-Pack-Manager-2026-07-23/EXPERIENCE.md"
  - "_bmad-output/planning-artifacts/sprint-change-proposal-2026-07-24.md"
---

# Test Design for Architecture: Pack-Manager P0 Readiness Reconciliation

**Purpose:** Define the architecture and testability capabilities that
Architecture and Development must deliver so QA and Release can later produce
trustworthy evidence for the Pack-Manager readiness initiative. This document
states WHAT must be controllable or observable and WHY; it is not a detailed
test recipe.

**Date:** 2026-07-24  
**Author:** Sallvain  
**Status:** Planning Complete — Implementation Blockers Explicit  
**Project:** Pack-Manager  
**Requirements:** [PRD](../planning-artifacts/prds/prd-Pack-Manager-2026-07-22/prd.md)
and [addendum](../planning-artifacts/prds/prd-Pack-Manager-2026-07-22/addendum.md)  
**Formal architecture:** [Architecture Spine](../planning-artifacts/architecture/architecture-Pack-Manager-2026-07-23/ARCHITECTURE-SPINE.md)  
**Normative reconciliation map:** [readiness-coverage-map.md](../planning-artifacts/prds/prd-Pack-Manager-2026-07-22/readiness-coverage-map.md),
status `final-pending-approval`  
**Planning baseline:** [traceability-matrix.md](traceability-matrix.md), FAIL
with 14/72 P0 criteria FULL

---

## Executive Summary

This system-level design governs all eight closure batches and their later
epics and stories. It preserves a denominator of exactly 72 P0 criteria:
58 non-FULL criteria require planned closure, while all 14 baseline-FULL
criteria require candidate-era revalidation at their mapped evidence depth.
RP-1 and RP-2 remain mandatory Release Prerequisites outside that denominator.

The baseline remains **FAIL, 14/72 FULL**. This planning artifact executes no
evidence, promotes no criterion, approves no coverage-map row, and makes no
release-readiness claim.

### 2026-07-24 Correct Course overlay

The finalized UX and Decisions D27-D30 add a Product Behavior Prerequisite
before affected Batches 3–7. AD-16 is now binding:

- the persistent draft uses canonical `PlanIntent`;
- one-use preview `planId` and durable `planAttemptId` are distinct;
- one confirmed attempt is active at a time;
- Activity, Results, verification, cancellation, History, replay, and Retry
  are attempt-correlated;
- prompt classification is trusted and closed; unknown silence remains stalled;
- `skipUpgradePlanConfirmation` replaces active `autoOpenDrawer` behavior.

The current boundary baseline therefore must change atomically when this
prerequisite is implemented. No native vector or evidence case may encode
immediate row execution, direct Manager-update execution, Operation-row
History, or drawer-only Activity as the target. `AUT-003` is superseded
historical evidence and supplies no positive credit.

The formal architecture requires three concerns to remain distinct:

1. **Product Behavior** — the user-visible promise. Missing or incorrect
   behavior returns to product work before regression coverage can receive
   credit.
2. **Reusable Test Infrastructure** — deterministic, native, lifecycle, and
   environment capabilities. Infrastructure success is not candidate proof.
3. **Candidate-Specific Release Evidence** — immutable proof about the exact
   packaged candidate. It cannot repair product behavior or replace a missing
   infrastructure boundary.

The architecture exposes five actionable ASRs. The verified production surface
currently contains 20 commands and six events, but those numbers are a baseline,
not an invariant. The invariant is one shared production command/event boundary
whose catalog, registration, wire contracts, wrappers, subscriptions, and
native acceptance coverage change atomically.

**Risk summary:** exactly eight high-priority risks remain open: seven score 6
and R-007 scores 9. The reconciled QA-only effort is approximately 9–15
engineer-weeks after the applicable architecture dependencies are delivered;
Development, Platform, and Release implementation effort remains downstream
planning work.

## Quick Guide

### 🚨 Implementation-Handoff Blockers

1. **Coverage-map approval:** the map remains `final-pending-approval`.
   Planning may reference its 72 rows, but the Criterion Acceptance Profile
   cannot freeze without its Product/QA approval record.
2. **DR-1 — OPEN:** Product and Release must declare the minimum supported
   macOS version before TIR-7 or RE-4/RE-7/RE-8 environment implementation
   handoff. Packaged compatibility slots and profile freeze remain blocked.
3. **DR-4 — PROPOSED:** Product/QA must approve the P0-specific gate and retry
   policy before profile freeze, candidate validation, or gate configuration.
   Architecture does not approve it, and no generic or legacy P1 policy
   substitutes for it.
4. **Evidence transport and retention:** Release must choose a conforming
   existing-GitHub transport and retention duration before release preparation.
   It must preserve sole append authority, lock/CAS, idempotency, write-once
   objects, one head, first failures, and the complete audit set.
5. **People and dates:** accountable roles and batch-relative boundaries are
   fixed below, but named individuals and calendar dates remain unassigned.
   Each affected implementation work item stays blocked until both are assigned.
6. **Execution access:** the provisioned target Mac, Apple-silicon and physical
   Intel hosts, installed prior public version, signing/notarization credentials,
   and immutable candidate are dependencies, not assumed evidence.

### ⚠️ Binding Planning Constraints

- **DR-2 is APPROVED:** packaged keyboard/focus checks, automated 4.5:1 text
  contrast, reduced motion, and a manual VoiceOver focus-order and
  completion-announcement pass are required.
- **DR-3 is APPROVED:** physical Intel fresh-install, Finder/Dock launch, and
  prior-public-version update evidence is mandatory. Universal-binary
  inspection cannot substitute.
- Release preparation begins only after Batches 1–6 reach their required exits,
  then freezes one fully packaged, signed, notarized, stapled candidate and its
  metadata. It is a prerequisite, not a ninth batch.

### 📋 Established Direction

- The only evidence lanes are `forced-offline`, `provisioned-target-mac`, and
  `candidate-release`; they are isolated and non-substitutable.
- Source-bound, environment-bound, and candidate-bound provenance are distinct.
  A result cannot be promoted to a deeper binding level.
- Ignored, skipped, collected-only, cancelled, wrong-lane, wrong-source, or
  wrong-candidate results cannot satisfy an acceptance slot.
- Candidate/P0 collection has no automatic retry. The first failure remains
  indexed; any authorized retry is a new linked record under the future
  DR-4-approved policy.

## Risk Assessment

The finalized PRD retains R-001 through R-008. Reconciliation against AD-1
through AD-15 supports all eight and no additional risk ID. Probability and
impact use the BMAD 1–3 scale.

| ID    | Category   | Risk                                                                                                                             |   P |   I | Score | Required mitigation and evidence                                                                                                                                                                   | Accountable role | Deadline                                                             |
| ----- | ---------- | -------------------------------------------------------------------------------------------------------------------------------- | --: | --: | ----: | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ---------------- | -------------------------------------------------------------------- |
| R-001 | OPS / DATA | Source, real-capture, and provisioned-host truth can drift, causing tests to prove an obsolete `mas` oracle.                     |   3 |   2 | **6** | Complete PC-1 before recurrence credit; preserve capture provenance; collect serialized, dated topology evidence; reject unreported host drift.                                                    | Product          | Batch 1 exit                                                         |
| R-002 | TECH       | Frontend and Rust suites can pass while the shared production command/event boundary is broken or divergent.                     |   2 |   3 | **6** | Deliver ASR-01 and catalog set-equality checks; round-trip every catalog command and dispatch every catalog event through real Tauri with controlled state.                                        | Architecture     | Batch 4 exit                                                         |
| R-003 | BUS / TECH | Stale, failed, late, inaccessible, or misleading UI state can authorize or communicate the wrong action.                         |   2 |   3 | **6** | Start provisional test-only rows with behavior-present checks; validate coherent state, visible errors, keyboard/focus/status semantics, stale-continuation rejection, and packaged accessibility. | Development      | Batches 2–3 for deterministic state; Batch 7 for packaged acceptance |
| R-004 | SEC / OPS  | Process output, locks, cancellation, timeout, shutdown, null stdin, or PID reuse can produce unsafe or dishonest terminal state. |   2 |   3 | **6** | Deliver ASR-02/03 controls and correlate native events, children, locks, journals, transcripts, cleanup, and historical-PGID non-signal evidence.                                                  | Development      | Batch 5 core; Batch 6 lifecycle exit                                 |
| R-005 | DATA / SEC | Persistence, History, retention, or diagnostics can lose evidence or expose or follow hostile local data.                        |   2 |   3 | **6** | Use disposable app data; prove atomicity, corruption handling, retention, exact archive contents, no inherited values, and symlink/path rejection.                                                 | QA               | Batch 6 exit                                                         |
| R-006 | SEC / OPS  | Updater metadata, signature, installed bytes, safety guards, or relaunched version can diverge while seam tests remain green.    |   2 |   3 | **6** | Bind prior-version check/download/verify/refuse/install/relaunch and non-writable behavior to one Candidate Manifest and frozen profile.                                                           | Release          | Batch 7 exit; ledger completion in Batch 8                           |
| R-007 | OPS / SEC  | A published set can be incomplete, inconsistent, unsigned, non-universal, unnotarized, unstapled, inaccessible, or unlaunchable. |   3 |   3 | **9** | Deliver ASR-04; inspect exact candidate artifacts; prove trust and launch on Apple silicon and physical Intel; replay the complete Evidence Index before decision.                                 | Release          | Batch 8 exit                                                         |
| R-008 | TECH / OPS | Results can depend silently on network, mutable machine state, credentials, ignored tests, retries, or a different candidate.    |   2 |   3 | **6** | Deliver ASR-05; bind provenance depth; deny cross-lane substitution; exclude ignored/unexecuted checks; retain first failure; reject candidate/profile mismatch.                                   | QA               | Lane contract before Batch 1; candidate lane before Batch 7          |

All eight risks remain open and require mitigation. This planning run marks none
mitigated, waived, or accepted.

**Category legend:** `TECH` architecture/integration; `SEC` security/privacy;
`PERF` performance/capacity; `DATA` data integrity/durability; `BUS`
user/business impact; `OPS` operational/release integrity. No standalone
`PERF` risk is added because the binding performance thresholds are planned
under the existing technical and operational risks.

## NFR Testability Requirements

This section identifies architecture support and planned evidence. Final
PASS/CONCERNS/FAIL assessment belongs in `nfr-assess` after implementation
evidence exists.

| NFR area                             | Binding threshold or rule                                                                                                                                                                                                                                                      | Architecture support and remaining gap                                                                                        | Planned evidence boundary                                                                                                                  |
| ------------------------------------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ | ----------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------ |
| Security and privacy                 | No sudo/password or generic shell surface; null stdin; allowlisted absolute executable and structured argv; constructed environment; no inherited-environment disclosure or diagnostic symlink substitution; signed updater; no authorization prompt.                          | Production safety defaults exist, but ASR-01/02, hostile filesystem control, and exact candidate trust must be executable.    | Forced-offline logic plus native process traces, inspected diagnostics/privacy output, and candidate-bound updater/trust records.          |
| Performance and bounded presentation | Login-shell probe 5 s/64 KiB; version probe 10 s; output flush at 50 ms, 64 lines, or 8 KiB; 512 KiB/stream; newest 5,000 live lines; cancel grace 5 s; stall default 120 s; hard cap 30 min; 101-row reachability; 900×600 minimum layout.                                    | Clock/output seams exist below the real boundary; packaged timing, large-state, and minimum-window behavior remain unproven.  | Paused-time checks, controlled native output/timing runs, and packaged WKWebView boundary scenarios.                                       |
| Reliability and recovery             | Manager failure isolation and Last-good Snapshot retention; durable journal/transcript; Interrupted recovery; historical PGIDs never signaled; atomic Settings; explicit crash/cancel/timeout/persistence outcomes.                                                            | Current journals and Settings provide a foundation; ASR-03 lifecycle and packaged quit/relaunch remain outstanding.           | Failure injection, forced quit/crash/relaunch, retained journal/transcript inspection, old-PGID sentinel, and updater relaunch records.    |
| Capacity / scalability               | Local single-user desktop; horizontal scaling is not applicable. Binding caps include 2,048 selections, 512-byte IDs, 64 plans, four concurrent operations, 120 s aging guard, more than 100 rows, 1,000 journal records, 14-day logs, and 200-or-90-day transcript retention. | Lower-level boundary tests exist; native compaction/retention and packaged large-state behavior still need decisive evidence. | Property/boundary tests, native retention artifacts, and packaged large-state interaction records.                                         |
| Maintainability and reproducibility  | Required frontend/Rust formatting, static checks, contracts, builds, and tests run from a clean checkout with outbound network denied; ignored/unexecuted tests excluded; automatic P0 retry disabled; first failure retained.                                                 | Existing suites are useful but current CI does not impose host-wide denial or the evidence contract.                          | Complete first-run forced-offline report, source/lockfile provenance, boundary-catalog validation, and schema/vector/ledger qualification. |
| Accessibility                        | **DR-2 APPROVED:** packaged keyboard/focus, automated 4.5:1 text contrast, reduced motion, and manual VoiceOver focus order and completion announcements.                                                                                                                      | The method is binding; its packaged-candidate capability and evidence are not yet delivered.                                  | Candidate-release packaged WKWebView automation plus manual VoiceOver record.                                                              |
| macOS compatibility                  | Normal GUI launch; both promised architectures; visible local parser incompatibility; **DR-3 APPROVED** physical Intel install/launch/update evidence.                                                                                                                         | Physical-host evidence is mandatory. **DR-1 OPEN** prevents the minimum-version environment matrix from being fixed.          | Candidate-bound install, Finder/Dock launch, and prior-version update on Apple silicon and physical Intel.                                 |
| Release and update integrity         | One immutable clean candidate; coherent artifacts and versions; Developer ID, notarization, stapling, Gatekeeper; reachable HTTPS metadata; valid updater signature; explicit restart; active-operation refusal; mutation invalidates downstream evidence.                     | The release workflow is a foundation, but no Manifest, frozen Profile, Registrar, or Evidence Index exists.                   | ASR-04 contract qualification, exact artifact inspection, candidate install/update runs, ledger replay, and later trace artifacts.         |

No generic code-coverage, duplication, CSP/vulnerability-count, CPU/RSS,
crash-free-rate, RTO/RPO, updater-duration, soak, or broader WCAG/legal target is
adopted for this initiative. The design must not invent one.

## Testability Concerns and ASR Acceptance Boundaries

All five formal ASRs are **ACTIONABLE**. The Architecture Spine resolves their
design contracts; this Test Design does not treat any enabler as implemented.
Exactly one role is accountable for each enabler.

| ASR                                            | Accountable role | Capability/support                                | Delivery boundary                                                                                                       | Acceptance boundary                                                                                                                                                                                                                                                                                                       |
| ---------------------------------------------- | ---------------- | ------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| ASR-01 — real native command/event boundary    | **Architecture** | Development and QA implement/use the boundary.    | Accepted by Batch 4 exit; required before Batches 5–7.                                                                  | The versioned catalog, production bridge/registration, wire schemas, wrappers/subscriptions, and inventory have set equality. Every catalog command round-trips and every catalog event dispatches through the isolated real production boundary, including startup order, detection, Re-detect, and six-Manager refresh. |
| ASR-02 — deterministic process and OS controls | **Development**  | Platform is the capability area.                  | Core process controls accepted before Batch 5; relevant filesystem/updater extensions before Batches 6–7.               | Deterministic helpers/adapters produce every required output, exit, signal, timeout, lock, stdin, path, permission, opener, restart, and updater condition while production adapters remain fail-closed.                                                                                                                  |
| ASR-03 — disposable lifecycle environment      | **QA**           | Development/Platform supports delivery.           | Accepted before Batch 6.                                                                                                | Crash, forced quit, relaunch, persistence, retention, hostile filesystem, and historical-PGID non-signal run from disposable roots with retained evidence and no contact with operator data or processes.                                                                                                                 |
| ASR-04 — candidate identity and attestation    | **Release**      | Evidence Registrar and existing GitHub transport. | Contract before release preparation; manifest frozen before Batch 7; complete ledger accepted in Batch 8.               | Manifest and ledger pass schema, canonicalization, digest, chain, artifact, provenance, retry, append-authority, and invalidation validation; exact bytes remain write-once.                                                                                                                                              |
| ASR-05 — split evidence lanes                  | **QA**           | CI is the execution mechanism.                    | Lane contract and isolation accepted before any Batch 1 evidence collection; candidate lane operational before Batch 7. | Forced-offline, target-Mac, and candidate-release workspaces, credentials, provenance, and outputs are isolated; aggregation rejects cross-lane substitution.                                                                                                                                                             |

If an accountable role cannot accept its ASR by the stated boundary, the
affected batch and every downstream dependent batch remain blocked.

## Cross-Cutting Evidence Invariants

### Shared production boundary

`contracts/tauri-boundary/v1.json` is the versioned catalog. Production
registration, Rust models, TypeScript wrappers/types/guards, subscriptions,
shared fixtures, inventory, and native vectors must have exact set equality.
The current 20 commands and six events are not permanent magic numbers. A
deliberate surface change updates all affected production and acceptance
contracts together.

### Three non-substitutable lanes

| Lane                     | Purpose and binding limit                                                                                                                                                                                                                                |
| ------------------------ | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `forced-offline`         | Begins from a fresh clean checkout after pinned dependencies/toolchains are prepared, denies outbound network, uses controlled state/processes, and invokes no real Manager mutation. It produces source- or controlled-environment-bound evidence only. |
| `provisioned-target-mac` | Runs serially on a designated Mac, records dated Manager/tool topology including installed `mas`, detects drift, and produces environment-bound evidence only.                                                                                           |
| `candidate-release`      | Begins after freeze, uses the exact signed candidate and approved OS/endpoints, and produces candidate-bound evidence on Apple silicon and physical Intel where required. Credentialless or `--no-sign` builds are inadmissible.                         |

One lane cannot compensate for another.

### Candidate Identity Manifest and Acceptance Profile

- The Candidate Identity Manifest uses the strict
  `pack-manager.candidate-identity/v1` Draft 2020-12 schema with closed objects.
  It contains identity, not results.
- Validated I-JSON input must already be NFC and use schema-defined sorted,
  unique arrays. Unknown fields, duplicate keys, JSON numbers, invalid Unicode,
  and non-NFC values fail closed.
- RFC 8785 JCS serializes the validated value to UTF-8 with no BOM, insignificant
  whitespace, or trailing newline. The candidate digest is lowercase
  `sha256:<64-hex>` over exactly those canonical bytes.
- Raw candidate artifact byte lengths and SHA-256 values are computed after all
  signing, notarization, stapling, packaging, signature, and metadata work.
- The Criterion Acceptance Profile is canonicalized and hashed by the same
  rules. It covers all 72 criteria plus RP-1 and RP-2, fixing each evidence
  slot's concern, lane, minimum binding depth, subjects, environment, scenario,
  and future DR-4-approved retry rule.
- The profile cannot freeze while the coverage map is pending approval, DR-1 is
  OPEN, or DR-4 is PROPOSED.

### Append-only Evidence Index

- `evidence-index.ndjson` is a strict, LF-terminated, hash-chained ledger.
  Record payloads are JCS-canonicalized and hashed; the index digest covers
  every stored file byte including line feeds.
- A protected, Release-owned Evidence Registrar is the sole append authority.
  Producers submit immutable attempt bundles and cannot edit the index.
- Provider-verifiable identity, candidate/profile-scoped lock or CAS,
  idempotency, stale/fork rejection, one head, and write-once/no-clobber storage
  are acceptance requirements.
- Source-, environment-, and candidate-bound records must identify their exact
  provenance. Candidate subjects and result artifacts are distinct and cannot
  substitute for each other.
- A PASS requires every profile-required check to be executed and passed.
  Ignored, skipped, filtered, unexecuted, failed, errored, cancelled, missing,
  wrong-lane, wrong-depth, or conflicting results fail closed.
- Automatic retries are disabled. A failed first attempt remains immutable.
  Any later authorized attempt links to its predecessor and explains the
  change; the future retry disposition remains blocked on DR-4.

### Candidate invalidation

Any source commit, tag, version, signing identity, artifact byte or name,
package, signature, updater metadata byte, rebuild, resign, retag, repack,
replacement, new release-build workflow run, or new release-build run attempt
creates a new Candidate Manifest and evidence root. An evidence-collection
attempt against an unchanged candidate does not change candidate identity: it
remains a new linked record under the future DR-4-approved retry policy, with
the first failure retained. Previous evidence remains immutable history but is
ineligible for a new candidate. Affected Batch 7 and Batch 8 candidate
scenarios must rerun after candidate invalidation.

## Risk Mitigation Verification

| Risk  | Mitigation exit                                                                                                                     | Verification boundary                                                                                                          |
| ----- | ----------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------ |
| R-001 | PC-1 is complete and source, capture, and target topology agree without hiding drift.                                               | Source guard and provenance plus dated provisioned-target-Mac record.                                                          |
| R-002 | ASR-01 is accepted with exact catalog equality and complete command/event crossing.                                                 | Source contract validation plus real native Tauri evidence.                                                                    |
| R-003 | Behavior-present checks and coherent deterministic/package UI consequences are accepted.                                            | Component/browser evidence for deterministic behavior; packaged evidence only where the criterion requires it.                 |
| R-004 | Controlled process and lifecycle terminal states correlate across all durable artifacts.                                            | Native child, signal, lock, event, journal, transcript, shutdown, and non-signal records.                                      |
| R-005 | Disposable persistence and diagnostics survive failures without disclosure or traversal.                                            | Relaunch archive, retention/atomicity records, and inspected diagnostics privacy output.                                       |
| R-006 | The installed prior version safely reaches or refuses the exact candidate as specified.                                             | Candidate-bound endpoint, signature, active-operation refusal, install, relaunch, and non-writable records.                    |
| R-007 | The exact candidate is coherent, trusted, installed, and launchable on both required architectures, and its ledger replays cleanly. | Candidate Manifest, artifact/trust attestation, Apple-silicon and physical-Intel launch/update records, complete index replay. |
| R-008 | Every result is admitted only at its declared lane and provenance depth with first failure preserved.                               | Lane-isolation qualification and negative Registrar/aggregator contract cases.                                                 |

## Assumptions, Dependencies, and Delivery Sequence

### Assumptions

1. `docs/SPEC.md` and `docs/DECISIONS.md` remain authoritative for product
   behavior; this design does not reopen settled behavior.
2. The formal Architecture Spine remains binding for cross-batch invariants.
   Its approval is architecture authority, not implementation or readiness
   evidence.
3. Controlled adapters remain construction-time dependencies of a
   non-distributable native harness composition. No release bit contains a
   hidden selector that can enable them.
4. The coverage map's current lane and batch assignments remain planning inputs
   until approval and any TIR-1 behavior-present reclassification.

### Dependencies and sequence

1. ASR-05 lane separation is accepted before any evidence collection.
2. Batch 1 runs first.
3. Batches 2, 3, and 4 may then proceed in parallel; ASR-01 is accepted at
   Batch 4 exit.
4. Batches 5 and 6 require the accepted Batch 4 foundation and may proceed in
   parallel; ASR-02/03 must meet their stated boundaries.
5. After all Batch 1–6 exits are accepted, release preparation freezes the
   complete signed, notarized, stapled candidate, updater metadata, Candidate
   Manifest, and eligible Acceptance Profile. Release preparation is not a
   ninth batch.
6. Batch 7 uses that immutable candidate.
7. Batch 8 follows Batch 7, attests the same unchanged candidate, and completes
   the Evidence Index for later trace regeneration.

Required execution dependencies are a qualified target Mac; disposable app
state and controlled helpers; an installed prior public version; Apple-silicon
and physical Intel hosts; current Apple/updater credentials; and conforming
write-once evidence storage. Absence blocks the affected evidence and cannot be
waived by another lane.

## Planning Handoff Status

This architecture-facing Test Design is **planning complete** and ready to
inform `create-epics-and-stories`. It does not authorize implementation and does
not assert that any ASR, mitigation, test, lane, candidate, evidence record, or
criterion is complete.

Downstream decomposition must preserve:

- the eight-batch dependency sequence;
- exactly one accountable role and the stated acceptance boundary for each ASR;
- Product Behavior, Reusable Test Infrastructure, and Candidate-Specific
  Release Evidence as separate work;
- the three execution lanes and provenance depths;
- Candidate Manifest/Profile/Index contracts and invalidation rules;
- DR-1 OPEN, DR-2 APPROVED, DR-3 APPROVED, DR-4 PROPOSED;
- the coverage map's `final-pending-approval` status;
- 72 P0 rows plus RP-1/RP-2 outside the denominator; and
- no readiness status change until a later candidate-bound Trace workflow.

Named individuals, calendar dates, map approval, DR-1, DR-4, and
Release-owned evidence transport/retention must be assigned or resolved at
their stated boundaries before affected implementation work enters execution.

---

**Generated by:** BMad TEA Agent  
**Workflow:** `bmad-testarch-test-design`  
**Artifact role:** Architecture/Development WHAT-and-WHY handoff
