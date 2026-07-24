---
workflowStatus: "completed"
totalSteps: 5
stepsCompleted:
  - "step-01-detect-mode"
  - "step-02-load-context"
  - "step-03-risk-and-testability"
  - "step-04-coverage-plan"
  - "step-05-generate-output"
lastStep: "step-05-generate-output"
nextStep: ""
lastSaved: "2026-07-24"
workflowType: "testarch-test-design"
mode: "system-level"
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

# Test Design for QA: Pack-Manager P0 Readiness Reconciliation

**Purpose:** Define the system-level QA execution recipe for all 72 P0
criteria, the eight closure batches, the 14 historical-FULL revalidations, and
the separate RP-1/RP-2 Release Prerequisites. It plans evidence; it does not
implement tests or claim that evidence exists.

**Date:** 2026-07-24  
**Author:** Sallvain  
**Status:** Planning Complete — Execution Blocked as Recorded  
**Project:** Pack-Manager

**Related:** The companion
[`test-design-architecture.md`](test-design-architecture.md) defines the
architecture/testability contract. The formal Architecture Spine remains
binding. The coverage map remains `final-pending-approval`.

## Executive Summary

The current planning gate remains **FAIL with 14/72 P0 criteria FULL**. This
plan closes all 58 non-FULL rows and revalidates all 14 historical FULL rows at
the finalized evidence depth; no row carries forward automatically.

The 58 open rows retain map revision 1's provisional primary split:

- 1 Product Behavior;
- 52 Reusable Test Infrastructure; and
- 5 Candidate-Specific Release Evidence.

All 24 `BP` rows begin with a behavior-present check. Missing or incorrect
behavior becomes Product Behavior work through a reviewed map revision before
regression coverage can receive credit.

### 2026-07-24 Correct Course overlay

The 1/52/5 split above is preserved only as revision-1 planning history.
Revision 2 changes the required consequences for affected F3–F8, F10, F11, and
D25 rows. Before QA authors or admits their evidence:

1. UX-PB.1 through UX-PB.5 implement the persistent plan, durable attempt,
   Activity/Results, plan History/Retry, and confirmation/settings contracts;
2. TIR-1 rechecks behavior presence against Decisions D27-D30 and AD-16;
3. the map's lanes, batches, scenario contracts, and totals are mechanically
   reconciled and approved; and
4. existing evidence for superseded behavior receives no positive credit.

Affected coverage must include browser/component, real native Tauri,
lifecycle, and packaged-WKWebView cases as appropriate for: row and Manager
add-to-plan behavior, plan persistence, separate confirmation, one active
attempt, plan cancellation, verifying-before-success, trusted prompt
classification, one-plan-per-History-row, replay/Retry lineage, high zoom,
focus/VoiceOver, and `Pack-Manager Update Ready!`.

Exactly eight risks remain open: seven score 6 and R-007 scores 9. The
reconciled QA-only estimate is approximately **9–15 engineer-weeks after all
applicable ASR capabilities, environments, profile inputs, and candidate are
available**. It excludes Development/Platform/Release implementation, waiting
time, and candidate-invalidating reruns.

## Not in Scope

| Item                                                                          | Reason                                                        | Planning treatment                                                                        |
| ----------------------------------------------------------------------------- | ------------------------------------------------------------- | ----------------------------------------------------------------------------------------- |
| Product, test, harness, CI, release-workflow, or configuration implementation | This is a design workflow only.                               | Create separately authorized epics/stories after blockers are assigned.                   |
| Traceability/gate regeneration                                                | The user explicitly excluded it.                              | Run a later candidate-bound Trace workflow only after the complete Evidence Index exists. |
| Unrelated P1/P2 features                                                      | They cannot compensate for an open P0 row.                    | Keep excluded; retain only RP-1/RP-2 outside the denominator.                             |
| New updater provider, channel, or release framework                           | The spine binds the existing GitHub/release-please framework. | A separate product/architecture change would be required.                                 |
| Final NFR rating                                                              | Planned evidence does not yet exist.                          | Run `nfr-assess` after qualified evidence collection.                                     |
| HTTP/API service tests                                                        | Pack-Manager is a local Tauri app with no product HTTP API.   | Use unit, component, browser, native Tauri, packaged-app, and artifact evidence.          |

## Dependencies & Test Blockers

### Architecture and Development Dependencies

| Dependency                                      | Accountable role                      | Required by                                                           | QA acceptance need                                                                                                                                                                                                                         |
| ----------------------------------------------- | ------------------------------------- | --------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| ASR-01 shared production command/event boundary | Architecture                          | Batch 4 exit                                                          | Catalog, registration, wire contracts, wrappers/subscriptions, and inventory have set equality; every catalog command round-trips and every catalog event dispatches through real isolated Tauri. Current 20/six is a changeable baseline. |
| ASR-02 deterministic process/OS controls        | Development; Platform capability area | Core before B5; extensions before B6–B7                               | Controlled output, exits, signals, timeout, locks, null stdin, paths, permissions, opener, restart, and updater effects with production fail-closed defaults.                                                                              |
| ASR-03 disposable lifecycle environment         | QA; Development/Platform support      | Before B6                                                             | Safe crash, forced quit, relaunch, persistence, retention, hostile filesystem, and historical-PGID non-signal without operator data/processes.                                                                                             |
| ASR-04 candidate identity and attestation       | Release                               | Contract before release preparation; manifest before B7; ledger in B8 | Locked schemas/vectors, reproducible JCS/SHA-256 identity, sole-append Registrar, hash chain, provenance/retry/invalidation validation, and write-once bytes.                                                                              |
| ASR-05 three split evidence lanes               | QA; CI execution mechanism            | Before any B1 evidence; candidate lane before B7                      | Isolated workspaces, credentials, caches, provenance, and outputs for forced-offline, provisioned-target-Mac, and candidate-release; cross-lane substitution rejected.                                                                     |

### Governance and Execution Blockers

- The coverage map remains `final-pending-approval`; Product/QA approval is
  required before it becomes a frozen profile input.
- DR-1 remains **OPEN**. Product and Release must declare minimum supported
  macOS before TIR-7 or RE-4/RE-7/RE-8 environment handoff and profile freeze.
- DR-2 and DR-3 are **APPROVED** planning constraints. Packaged 4.5:1
  contrast/keyboard/focus/reduced-motion/manual-VoiceOver evidence and physical
  Intel install/launch/update evidence are mandatory methods, not existing
  results.
- DR-4 remains **PROPOSED** on Product/QA governance. No legacy 80% P1 rule or
  generic 95%/80% default is applied.
- Release must select a conforming evidence transport and retention duration
  before release preparation.
- Named people and calendar dates are unassigned. Each downstream work item is
  blocked from implementation entry until both are assigned.
- QA needs a qualified target Mac, disposable roots/helpers, Apple-silicon and
  physical Intel hosts, an actually installed prior public version, current
  credentials, and one unchanged signed/notarized/stapled candidate.

### QA Infrastructure Setup

1. Versioned deterministic fixtures for all six Managers, UI states, settings,
   paths, hostile archive inputs, process outcomes, and evidence-contract
   negative cases.
2. A non-distributable real-Tauri composition using the production
   registration and wire surface with controlled adapters.
3. Disposable Application Support, logs, journal, transcript, export, bundle,
   opener, process, and updater roots.
4. Three separately qualified execution workspaces with versioned provision
   profiles and immutable result destinations.
5. Release-owned Manifest/Profile/Index tooling that preserves exact bytes,
   first attempts, failures, and human/machine result agreement.

The configured Playwright Utils package is a harness option, not an installed
fact. Because no HTTP product API exists, this example validates the local
planning contract:

```typescript
import { readFile } from "node:fs/promises";
import { test } from "@seontechnologies/playwright-utils/api-request/fixtures";
import { expect } from "@playwright/test";

test("@P0 @Contract maps every P0 criterion once", async () => {
  const markdown = await readFile(
    "_bmad-output/test-artifacts/test-design-progress.md",
    "utf8",
  );
  const section = markdown.match(
    /### 72-Criterion Primary Planning Matrix([\s\S]*?)### Matrix Validation/,
  )?.[1];
  expect(section).toBeTruthy();

  const ids = [...section!.matchAll(/^\|\s*`((?:F|D)[^`]+-AC\d+)`\s*\|/gm)].map(
    (match) => match[1],
  );
  expect(ids).toHaveLength(72);
  expect(new Set(ids).size).toBe(72);
});
```

## Risk Assessment

Full rationale and architecture mitigations are in the companion Architecture
document. No risk is marked mitigated, waived, or accepted here.

| Risk                                        | Category   | Score | QA validation                                                                                                     |
| ------------------------------------------- | ---------- | ----: | ----------------------------------------------------------------------------------------------------------------- |
| R-001 — source/capture/target-Mac drift     | OPS / DATA |     6 | PC-1 guard, real-capture provenance, dated serialized target-Mac topology, explicit drift failure.                |
| R-002 — divergent fake/native boundary      | TECH       |     6 | Catalog set equality plus every-command/every-event real production-boundary acceptance.                          |
| R-003 — misleading/inaccessible state       | BUS / TECH |     6 | Behavior-present checks; deterministic error/state/keyboard semantics; approved packaged accessibility.           |
| R-004 — unsafe process lifecycle            | SEC / OPS  |     6 | Controlled child, output, lock, null-input, signal, timeout, cleanup, journal, and transcript evidence.           |
| R-005 — persistence/diagnostics failure     | DATA / SEC |     6 | Disposable crash/relaunch, atomicity/corruption, retention, archive composition, privacy/path rejection.          |
| R-006 — updater/install divergence          | SEC / OPS  |     6 | Exact-candidate metadata/signature/refusal/install/relaunch/non-writable journeys from a prior version.           |
| R-007 — invalid candidate/artifact/ledger   | OPS / SEC  |     9 | Manifest/Profile/Index validation, exact artifact trust, both-architecture install/launch/update, ledger replay.  |
| R-008 — lane/provenance/retry contamination | TECH / OPS |     6 | Lane isolation, denial controls, source/environment/candidate binding, first failure, invalid-substitution cases. |

There are no medium- or low-priority risks in this P0-only initiative.

## NFR Test Coverage Plan

This is a plan for later `nfr-assess`, not a current rating.

| Category                        | Binding requirement                                                                                                                                           | Planned validation                                                                                  | Level/tool | Expected evidence                                                                                       |
| ------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------- | ---------- | ------------------------------------------------------------------------------------------------------- |
| Security/privacy                | No shell/sudo/password/admin path; null stdin; absolute structured argv; constructed environment; diagnostics privacy; signed updater/trust.                  | B5 controlled process, B6 hostile diagnostics, B7 non-writable updater, B8 trust.                   | U/N/P/A    | Process trace, inspected ZIP/privacy record, updater/no-admin record, signature/Gatekeeper attestation. |
| Performance/capacity            | 5 s/64 KiB shell probe; 10 s version; 50 ms/64 lines/8 KiB flush; 512 KiB/stream; 5,000 live lines; 5 s cancel; 120 s stall; 30 min cap; 101 rows; 900×600.   | Paused-time boundaries, controlled native large output, packaged large-state/minimum-window checks. | U/C/N/P    | Timing/event counts, retention/transcript record, packaged interaction/resource record.                 |
| Reliability/recovery            | Manager isolation/Last-good Snapshot; durable journal/transcript; Interrupted recovery; old PGIDs never signaled; atomic Settings; explicit failure outcomes. | Offline matrix, cancellation/timeout/shutdown, crash/relaunch, persistence, updater relaunch.       | U/C/N/P    | Offline report, journals/transcripts, non-signal proof, before/after relaunch evidence.                 |
| Maintainability/reproducibility | Clean checkout; required format/static/contract/build/tests; outbound denied; no ignored/unexecuted credit; no automatic retry; first failure retained.       | B4 catalog contract, B8 clean source run, schema/vector/ledger qualification.                       | U/CI/A     | Complete first-run output, commit/lockfile provenance, contract and ledger reports.                     |
| Accessibility                   | DR-2 approved packaged method and 4.5:1 text contrast.                                                                                                        | Keyboard/focus/contrast/reduced-motion automation plus manual VoiceOver.                            | P + manual | Candidate-bound automated output and VoiceOver record.                                                  |
| macOS/release integrity         | Normal GUI launch, both promised architectures, DR-3 physical Intel, coherent trusted artifacts and explicit updater control.                                 | B7 updater/install and B8 artifact/trust/launch on Apple silicon and physical Intel.                | P/A        | Candidate Manifest/Profile/Index and physical-host evidence.                                            |

DR-1's minimum macOS remains the binding unknown. Generic coverage,
duplication, CSP/vulnerability-count, CPU/RSS, crash-free-rate, RTO/RPO,
updater-duration, soak, or broader WCAG/legal thresholds are not adopted and
are not invented here.

## Entry Criteria

- [ ] Map approval, DR-1, and DR-4 are resolved before profile freeze.
- [ ] Each affected story has one named assignee and calendar date.
- [ ] The required ASR is accepted at its batch-relative boundary.
- [ ] BP rows have a documented behavior-present result before test authoring.
- [ ] Lane/profile/scenario contracts and immutable result storage are ready.
- [ ] Required hosts, prior version, credentials, and candidate are available.
- [ ] Candidate work identifies one Manifest digest; any mutation restarts the
      affected B7/B8 slots.

## Exit Criteria

- [ ] Every planned P0 slot has an executed, admitted PASS at its frozen lane,
      minimum depth, environment, and candidate subject.
- [ ] All 14 historical-FULL rows have candidate-era revalidation records; no
      status was carried forward.
- [ ] RP-1 and RP-2 pass separately outside the denominator.
- [ ] PC-1 and every behavior-present reclassification are resolved honestly.
- [ ] All score-6/9 mitigations have admitted evidence at each frozen profile
      slot's mapped lane and depth, associated with the exact candidate
      Evidence Set where permitted without relabeling provenance.
- [ ] First failures remain visible; no ignored/unexecuted/automatic-retry
      substitution exists.
- [ ] QA and Development can hand the complete index to a separate Trace
      workflow. This is eligibility to regenerate the gate, not readiness.

The future P0 candidate threshold is 100% of required frozen-profile slots.
DR-4 still blocks gate configuration and any P1 threshold. Final NFR status is
deferred to `nfr-assess`.

## Test Coverage Plan

P0/P1/P2/P3 describe priority and risk, **not execution timing**. All 72 rows
below are P0 planning scenarios. The exact requirement text remains in the
pending normative map; the table fixes the primary concern, planning point,
level, execution lane, minimum depth, dependency, and conditional disposition.
A later profile may add secondary slots without duplicating a criterion's
primary planning row.

Legend: `PB` Product Behavior; `TI` Reusable Test Infrastructure; `RE`
Candidate-Specific Release Evidence; `TI-RV` historical-FULL revalidation.
`FO` forced-offline; `TM` provisioned-target-Mac; `CR` candidate-release.
`S/E/C` source/environment/candidate depth. `U/C/B/N/T/P/A/CI` means
unit/component/browser/native/target-Mac/packaged/artifact/clean-CI.
`C` means conditionally eligible for later FULL reassessment; `RV` means
historical FULL must be revalidated and is not carried forward.

### P0 (Critical)

**Criteria:** exact 72-row denominator; all block the 100% P0 initiative.

| Criterion  | Baseline         | Concern | Point | Level     | Lane | Depth | Specific dependency                         | Disposition |
| ---------- | ---------------- | ------- | ----- | --------- | ---- | ----- | ------------------------------------------- | ----------- |
| `F1-AC1`   | PARTIAL          | TI      | B4    | N         | FO   | E     | B1; ASR-01; startup order                   | C           |
| `F1-AC2`   | PARTIAL          | TI      | B4    | N         | FO   | E     | B1; ASR-01; every UI entry                  | C           |
| `F1-AC3`   | UNIT-ONLY        | TI      | B4    | N         | FO   | E     | B1; ASR-01/02; PATH/timeout/cleanup         | C           |
| `F1-AC4`   | UNIT-ONLY        | TI      | B4    | N         | FO   | E     | B1; ASR-01/02; probe outcomes               | C           |
| `F1-AC5`   | FULL             | TI-RV   | RV@B2 | U+C       | FO   | S     | Exact source; raw-before-canonical          | RV          |
| `F1-AC6`   | PARTIAL          | TI      | B2    | C         | FO   | S     | B1; BP; copy outcomes                       | C           |
| `F1-AC7`   | NONE             | TI      | B1    | T         | TM   | E     | BP; PC-1; dated topology                    | C           |
| `F1-AC8`   | PARTIAL          | TI      | B2    | C         | FO   | S     | B1; BP; all six; clipboard                  | C           |
| `F2-AC1`   | UNIT-ONLY        | TI      | B4    | N         | FO   | E     | B1; ASR-01/02; all-six order                | C           |
| `F2-AC2`   | FULL             | TI-RV   | RV@B2 | U         | FO   | S     | Exact source; merge corpus                  | RV          |
| `F2-AC3`   | UNIT-ONLY        | TI      | B2    | U+C       | FO   | S     | B1; BP; phase order                         | C           |
| `F2-AC4`   | FULL             | TI-RV   | RV@B4 | U+C+N     | FO   | E     | B1; ASR-01; fresh detection                 | RV          |
| `F2-AC5`   | FULL             | TI-RV   | RV@B2 | C         | FO   | S     | B1; coexistence states                      | RV          |
| `F2-AC6`   | PARTIAL          | TI      | B2    | U         | FO   | S     | B1; BP; every adapter                       | C           |
| `F2-AC7`   | FULL             | TI-RV   | RV@B2 | U+C       | FO   | S     | B1; stale/Retry/isolation                   | RV          |
| `F2-AC8`   | PARTIAL          | TI      | B2    | U         | FO   | S     | B1; BP; network adapters                    | C           |
| `F2-AC9`   | PARTIAL          | TI      | B5    | U         | FO   | S     | B4; BP; dual refresh                        | C           |
| `F3-AC1`   | PARTIAL          | TI      | B3    | C         | FO   | S     | B1; BP; complete table                      | C           |
| `F3-AC2`   | PARTIAL          | TI      | B3    | C         | FO   | S     | B1; BP; expand/search                       | C           |
| `F3-AC3`   | PARTIAL          | TI      | B3    | U+C       | FO   | S     | B1; BP; pinned exclusions                   | C           |
| `F3-AC4`   | PARTIAL          | TI      | B3    | U+C       | FO   | S     | B1; BP; greedy policy                       | C           |
| `F3-AC5`   | FULL             | TI-RV   | RV@B3 | U+C       | FO   | S     | Exact source; verbatim versions             | RV          |
| `F3-AC6`   | PARTIAL          | TI      | B3    | C         | FO   | S     | B1; BP; npm table/card                      | C           |
| `F3-AC7`   | FULL             | TI-RV   | RV@B3 | U+C       | FO   | S     | Exact source; mise consequences             | RV          |
| `F3-AC8`   | PARTIAL          | TI      | B3    | C         | FO   | S     | B1; BP; non-color status                    | C           |
| `D23a-AC1` | PARTIAL          | TI      | B1    | T         | TM   | E     | BP; PC-1; live `mas`                        | C           |
| `D23a-AC2` | PARTIAL          | TI      | B1    | T+U       | TM   | E     | BP; real capture; replay secondary          | C           |
| `D23a-AC3` | FULL             | TI-RV   | RV@B1 | U         | FO   | S     | B1 provenance; no synthetic substitution    | RV          |
| `D23a-AC4` | NONE             | PB      | B1    | Product+U | FO   | S     | PC-1 before guard credit                    | C           |
| `D23a-AC5` | UNIT-ONLY        | TI      | B1    | U         | FO   | S     | BP; fixture policy                          | C           |
| `F4-AC1`   | PARTIAL          | TI      | B3    | U+C       | FO   | S     | B1; BP; entries/failure                     | C           |
| `F4-AC2`   | FULL             | TI-RV   | RV@B4 | U+C+B+N   | FO   | E     | ASR-01; reviewed/native bytes               | RV          |
| `F4-AC3`   | FULL             | TI-RV   | RV@B3 | U+C       | FO   | S     | Defaults/rebuild/exclusions                 | RV          |
| `F4-AC4`   | FULL             | TI-RV   | RV@B5 | U+N       | FO   | E     | B4; ASR-02; preview=spawn                   | RV          |
| `F4-AC5`   | FULL             | TI-RV   | RV@B5 | U         | FO   | S     | Locks/parallelism/cap four                  | RV          |
| `F5-AC1`   | PARTIAL          | TI      | B3    | C+B       | FO   | S     | B1; BP; keyboard/filter                     | C           |
| `F5-AC2`   | FULL             | TI-RV   | RV@B4 | C+B+N     | FO   | E     | ASR-01; PackageRefs/admission               | RV          |
| `F5-AC3`   | PARTIAL          | TI      | B3    | C+B       | FO   | S     | B1; BP; one package/rejections              | C           |
| `F6-AC1`   | PARTIAL          | TI      | B5    | N         | FO   | E     | B4; ASR-02 executables                      | C           |
| `F6-AC2`   | UNIT-ONLY        | TI      | B5    | N         | FO   | E     | B4; real serialization                      | C           |
| `F6-AC3`   | INTEGRATION-ONLY | TI      | B5    | N         | FO   | E     | B4; production events                       | C           |
| `F6-AC4`   | UNIT-ONLY        | TI      | B5    | N         | FO   | E     | B4; spawn/no-spawn                          | C           |
| `F6-AC5`   | UNIT-ONLY        | TI      | B5    | N         | FO   | E     | B4; lock timeline                           | C           |
| `F7-AC1`   | UNIT-ONLY        | TI      | B5    | N         | FO   | E     | B4; event flush limits                      | C           |
| `F7-AC2`   | PARTIAL          | TI      | B5    | C+N       | FO   | E     | B4; native events                           | C           |
| `F7-AC3`   | PARTIAL          | TI      | B5    | C+N       | FO   | E     | B4; signals/finalization                    | C           |
| `F7-AC4`   | PARTIAL          | TI      | B5    | N         | FO   | E     | B4; null input/no prompt                    | C           |
| `F8-AC1`   | UNIT-ONLY        | TI      | B6    | N         | FO   | E     | B4; ASR-03; transcript                      | C           |
| `F8-AC2`   | UNIT-ONLY        | TI      | B6    | N         | FO   | E     | B4; ASR-03; journal                         | C           |
| `F8-AC3`   | PARTIAL          | TI      | B6    | N         | FO   | E     | B4; ASR-03; non-signal                      | C           |
| `F8-AC4`   | PARTIAL          | TI      | B6    | N         | FO   | E     | B4; ASR-03; Reveal                          | C           |
| `D26-AC1`  | UNIT-ONLY        | TI      | B5    | N         | FO   | E     | B4; real-format events                      | C           |
| `D26-AC2`  | UNIT-ONLY        | TI      | B5    | U         | FO   | S     | B4; BP; closed corpus                       | C           |
| `F9-AC1`   | PARTIAL          | TI      | B6    | N         | FO   | E     | B4; ASR-03; permissions                     | C           |
| `F9-AC2`   | UNIT-ONLY        | TI      | B6    | N+A       | FO   | E     | B4; ASR-03; inspect ZIP                     | C           |
| `F9-AC3`   | UNIT-ONLY        | TI      | B6    | U         | FO   | S     | B4; BP; hostile inputs                      | C           |
| `F9-AC4`   | PARTIAL          | TI      | B6    | N         | FO   | E     | B4; ASR-03; opener                          | C           |
| `F10-AC1`  | PARTIAL          | TI      | B7    | P         | CR   | C     | B1–6; freeze; DR-1/DR-2                     | C           |
| `F10-AC2`  | NONE             | RE      | B8    | A         | CR   | C     | Same candidate; icon provenance             | C           |
| `F10-AC3`  | NONE             | RE      | B8    | P         | CR   | C     | DR-1/DR-3; both hosts                       | C           |
| `F10-AC4`  | NONE             | RE      | B8    | A         | CR   | C     | Complete trusted set                        | C           |
| `F11-AC1`  | UNIT-ONLY        | TI      | B6    | N         | FO   | E     | B4; ASR-03; app data                        | C           |
| `F11-AC2`  | PARTIAL          | TI      | B3    | U+C       | FO   | S     | B1; BP; controls/failures                   | C           |
| `F11-AC3`  | PARTIAL          | TI      | B3    | C         | FO   | S     | B1; BP; report/clipboard                    | C           |
| `F11-AC4`  | PARTIAL          | TI      | B6    | N         | FO   | E     | B4; ASR-03; native outcomes                 | C           |
| `F12-AC1`  | PARTIAL          | RE      | B8    | CI        | FO   | S     | Exact clean commit/lockfiles                | C           |
| `F12-AC2`  | PARTIAL          | TI      | B8    | U+CI      | FO   | S     | BP; host/network/process isolation          | C           |
| `F12-AC3`  | FULL             | TI-RV   | RV@B4 | U+N       | FO   | E     | ASR-01 catalog/schema/crossing              | RV          |
| `D25-AC2`  | PARTIAL          | TI      | B7    | P         | CR   | C     | Frozen endpoint/artifacts/events            | C           |
| `D25-AC3`  | PARTIAL          | TI      | B7    | P         | CR   | C     | DR-1/DR-3; prior version/refusal            | C           |
| `D25-AC4`  | PARTIAL          | TI      | B7    | P         | CR   | C     | Non-writable; no authorization              | C           |
| `D25A-AC2` | INTEGRATION-ONLY | RE      | B8    | A         | CR   | C     | Same signed candidate; no-sign inadmissible | C           |

**P0 planning total:** 72 unique criteria: 58 conditional closures plus 14
revalidations. Open batch counts remain `5/5/11/5/12/10/4/6`.

### P1 (High)

**Criteria:** No new P1 closure scenarios. RP-1/RP-2 are mandatory Release
Prerequisites, not P1 denominator rows for this initiative.

### P2 (Medium)

**Criteria:** None; excluded from this initiative.

### P3 (Low)

**Criteria:** None; excluded from this initiative.

### Release Prerequisites Outside the Denominator

| Prerequisite | Retained consequences                                                                                        | Point              | Lane/depth | Validation                                               |
| ------------ | ------------------------------------------------------------------------------------------------------------ | ------------------ | ---------- | -------------------------------------------------------- |
| RP-1         | `D25-AC1` scheduled/menu checks and `D25-AC5` update-state rehydration without Package History contamination | B7; association B8 | CR/C       | Installed-prior-version updater plus state/menu contract |
| RP-2         | `D25A-AC1` standard Edit/Window actions in the custom macOS menu                                             | B7; association B8 | CR/C       | Installed packaged-app native-menu keyboard/interaction  |

### ASR Evidence-Contract Qualification

These enabler scenarios do not add denominator rows:

1. Validate exact locked `/v1` schemas/vectors, I-JSON/NFC/order constraints,
   RFC 8785 JCS bytes, UTF-8/BOM/newline rules, and lowercase SHA-256 digests
   reproducibly across machines/runs.
2. Validate identity-only Candidate Manifest content and exact five final
   candidate artifacts after signing/notarization/stapling/packaging.
3. Validate that the frozen Profile covers all 72 criteria plus RP-1/RP-2 and
   binds approved map/policy, DR-1, scenario, concern, lane, depth, subjects,
   OS/architecture/physical-host matrix, and retry disposition.
4. Validate canonical LF-terminated record/envelope bytes, sequence/
   predecessor chain, subject/result separation, provenance, exact attempt
   counts, and human/machine agreement.
5. Prove Release is sole append authority through the protected Registrar with
   idempotency, lock/CAS, one head, stale/fork rejection, and no-clobber
   immutable objects/snapshots.
6. Prove automatic/branching/missing-first retries, ignored/skipped/
   unexecuted results, wrong lane/depth/source/candidate, and incomplete PASS
   counts fail closed. Manual retry remains blocked on DR-4.
7. Prove every rebuild, resign, retag, repackage, replacement, metadata change,
   new release-build workflow run, or new release-build run attempt creates a
   new Manifest/evidence root and reruns affected B7/B8 slots while preserving
   prior history. An evidence-collection retry against the unchanged candidate
   remains a new linked record under DR-4 and retains the first failure; it does
   not create a new Candidate Manifest.

## Execution Strategy

**Philosophy:** Run all eligible functional work in PRs when it stays under
about 15 minutes; defer only infrastructure-heavy, long, live-host, manual, or
candidate-bound execution. Cadence never changes a result's lane or depth.

| Cadence                        | Work                                                                                                | Boundary                                                                            |
| ------------------------------ | --------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------- |
| Every PR                       | Forced-offline source unit/component/browser/contract/static/build checks, parallelized where safe. | No live Manager, host, or candidate claim.                                          |
| Nightly                        | Longer forced-offline controlled-native/process/lifecycle and large-state suites after ASR-01..03.  | Still forced-offline; cannot substitute for TM or CR.                               |
| Weekly / qualified host window | Serialized provisioned-target-Mac topology and capture checks.                                      | Environment-bound only; ignored tests count only when explicitly executed/admitted. |
| Per immutable candidate        | After B1–6, release preparation, B7, then B8 on the same Manifest.                                  | Not a nightly/weekly lane; any mutation invalidates affected evidence.              |

Playwright functional work should use safe parallelization to keep ordinary
browser feedback in the roughly 10–15 minute PR target.

## QA Effort Estimate

| Priority |                                  Planning count | QA-only range            |
| -------- | ----------------------------------------------: | ------------------------ |
| P0       | 72 criterion rows plus RP/enabler qualification | **~9–15 engineer-weeks** |
| P1/P2/P3 |                         0 new closure scenarios | N/A                      |

The range starts only after applicable dependencies are ready and includes
design refinement, automation, debugging, lane integration, packaged/manual
execution, evidence review, and first-attempt triage. It excludes non-QA
implementation, hardware/credential waits, scheduling, and invalidation
reruns. No calendar duration can be assigned until people and dates exist.

## Implementation Planning Handoff

| Work                               | Primary role                                      | Relative boundary  | Dependency                                        |
| ---------------------------------- | ------------------------------------------------- | ------------------ | ------------------------------------------------- |
| B1 source/oracle restoration       | Product + QA execution                            | Runs first         | ASR-05; PC-1; target profile                      |
| B2/B3 deterministic behavior       | Development + QA                                  | After B1, parallel | BP checks and forced-offline lane                 |
| B4 native foundation               | Architecture accountable; Development/QA delivery | B4 exit            | ASR-01/02                                         |
| B5 process behavior                | Development accountable; QA validates             | After B4           | ASR-02 core                                       |
| B6 lifecycle/filesystem            | QA accountable; Development/Platform support      | After B4           | ASR-03 and ASR-02 extensions                      |
| Release preparation                | Release accountable                               | After B1–6         | Map/DR-1/DR-4/profile/transport/hosts/credentials |
| B7 packaged/updater                | QA execution with Release support                 | After freeze       | Approved DR-2/DR-3 and unchanged candidate        |
| B8 artifact/ledger/reproducibility | Release + QA                                      | After B7           | ASR-04; same candidate                            |

Each implementation item still requires one named person and one calendar
date. This table does not authorize work.

## Tooling & Access

| Tool/access                                               | Purpose                            | Status                                             |
| --------------------------------------------------------- | ---------------------------------- | -------------------------------------------------- |
| Existing Vitest/RTL, Playwright, Cargo/Rust test stack    | Deterministic source/UI foundation | Ready as lower-depth foundation only               |
| Playwright Utils                                          | Optional configured fixture layer  | Pending dependency decision; not assumed installed |
| Production-surface native Tauri harness                   | ASR-01 native crossing             | Pending implementation                             |
| Controlled process/lifecycle helpers and disposable roots | ASR-02/03                          | Pending implementation                             |
| Forced-offline enforcement                                | ASR-05 source/environment lane     | Pending qualification                              |
| Provisioned target Mac and profile                        | Live topology/capture              | Pending access/qualification                       |
| Apple-silicon and physical Intel hosts                    | DR-3 candidate acceptance          | Pending access                                     |
| Installed prior public version                            | Updater journey                    | Pending                                            |
| Release credentials and exact candidate                   | Candidate freeze                   | Pending; secrets remain in fnox/GitHub Secrets     |
| Protected Registrar/write-once evidence transport         | ASR-04 append/index                | Pending Release decision                           |

## Interworking & Regression

| Surface                  | Regression scope                                              | Decisive boundary                                                            |
| ------------------------ | ------------------------------------------------------------- | ---------------------------------------------------------------------------- |
| React UI/store           | Existing unit/component/browser behavior plus B2/B3 states    | Browser evidence remains UI-only unless packaged/native slot says otherwise. |
| Tauri IPC                | Catalog, registration, Rust/TS contracts, every command/event | B4 real production-boundary acceptance.                                      |
| Manager adapters/routing | Six-manager parsing, order, timeout, offline, provenance      | Forced-offline plus qualified target Mac where mapped.                       |
| Scheduler/process        | Plans, locks, output, cancel/stall/timeout/shutdown           | Controlled native B5; lower-level logic avoids duplicate assertions.         |
| Persistence/diagnostics  | Settings, journal, transcripts, History, ZIP/privacy          | Disposable native B6.                                                        |
| Updater/release          | Real endpoint/signature/install/relaunch and artifact trust   | Exact candidate B7/B8 on both physical architectures.                        |
| Evidence control plane   | Manifest/Profile/Index/Registrar/invalidation                 | Contract tests plus protected append/replay qualification.                   |

Existing suites must remain green at their honest proof depth. New decisive
boundary tests supplement rather than duplicate lower-level algorithm checks.

## Appendix A: Tags

Recommended tags: `@P0`, `@Contract`, `@Component`, `@Native`, `@LiveMac`,
`@Packaged`, `@Release`, and `@Offline`. Tags select execution; the frozen
Profile—not a tag—defines admissible lane, depth, subjects, and environment.

## Appendix B: Knowledge Base References

- `risk-governance.md`
- `probability-impact.md`
- `test-levels-framework.md`
- `test-priorities-matrix.md`
- `nfr-criteria.md`
- `test-quality.md`
- `overview.md` and the loaded Playwright Utils profile
- `playwright-cli.md`

---

**Generated by:** BMad TEA Agent  
**Workflow:** `bmad-testarch-test-design`  
**Version:** 4.0 (BMad v6)
