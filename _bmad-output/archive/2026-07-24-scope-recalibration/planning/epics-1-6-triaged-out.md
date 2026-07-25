# Epics 1-6 — the 31 stories triaged out

Applied 2026-07-25. These story sections were removed verbatim from
`_bmad-output/planning-artifacts/epics.md` per the verdicts recorded in
`planning-artifacts/story-triage-2026-07-24.md`, which is the companion to
`docs/DECISIONS.md` **D33**.

**Nothing here is authoritative and nothing here is scheduled.** Like the rest of
this archive, it lives outside `_bmad-output/planning-artifacts/` on purpose: the
filename matches BMAD's `{planning_artifacts}/*epic*.md` glob, so leaving it there
would make `bmad-sprint-planning` silently re-queue all 31 stories.

Six stories survived the triage and remain live in `epics.md`:
**2.2, 3.1, 3.2, 3.4, 3.5, 6.5**. Epics 1, 4, and 5 lost every story and were
removed as whole sections; Epics 2, 3, and 6 remain with their survivors.

Full per-story reasoning — including the adversarial pass that overturned 14 of 20
initial KEEP verdicts — is in the triage document. The disposition line under each
heading below is that story's verdict only.

> Before rescheduling anything here, check whether the behavior already exists in
> the shipping code. That is why most of these were cut.

## Disposition index

| Story | Title | Verdict | Merged into |
|---|---|---|---|
| 1.1 | Restore Current `mas` and Release Truth | RETIRE | — |
| 1.2 | Qualify the Initial Split Evidence Lanes | RETIRE | — |
| 1.3 | Verify the Live Six-Manager Target-Mac Topology | MERGE | 2.1 |
| 1.4 | Capture the Real `mas` Correctness Oracle | MERGE | 1.5 |
| 1.5 | Enforce `mas` Provenance and Fixture Honesty | RETIRE | — |
| 2.1 | Preserve Honest Absence and Complete Environment Evidence | MERGE | see triage overturn table |
| 2.3 | Keep Offline Failures Isolated | RETIRE | — |
| 2.4 | Revalidate Stable Detection and Refresh State Truth | MERGE | 2.3 |
| 3.3 | Build Plans from Every User Entry Point | MERGE | UX-PB.1c |
| 3.6 | Revalidate Version Truth, mise Consequences, and Plan Defaults | MERGE | 3.1 |
| 4.1 | Establish the Versioned Production Boundary Contract | RETIRE | — |
| 4.2 | Deliver the Deterministic Process-Control Core | RETIRE | — |
| 4.3 | Cross Native Startup, Detection, and Re-detect | RETIRE | — |
| 4.4 | Cross All-Six Native Refresh Ordering | MERGE | 2.2 |
| 4.5 | Revalidate Native Refresh and Contract Equality | MERGE | 2.2 |
| 4.6 | Revalidate Reviewed Plans and Native Admission | MERGE | UX-PB.2b |
| 5.1 | Refresh Every Routed Subject and Executor | RETIRE | — |
| 5.2 | Prove Dynamic Self-Update Routes and Manager Header/Card Plan State | MERGE | see triage overturn table |
| 5.3 | Reject Unsafe Spawns and Hold Complete Locks | MERGE | see triage overturn table |
| 5.4 | Preserve Native Output and Activity Boundaries | MERGE | see triage overturn table |
| 5.5 | Cancel, Stall, Time Out, and Shut Down Honestly | MERGE | see triage overturn table |
| 5.6 | Repair Only the Allowlisted Unterminated `mas` Notice | MERGE | 5.4 |
| 5.7 | Prove the D26 Rule Cannot Become Heuristic | RETIRE | — |
| 5.8 | Revalidate Preview Bytes Against Spawned Bytes | MERGE | 5.3 |
| 5.9 | Revalidate Scheduler Locks, Parallelism, and Capacity | MERGE | 5.3 |
| 6.1 | Deliver ASR-02 Filesystem and Native-Utility Extensions | RETIRE | — |
| 6.2 | Deliver the Disposable Lifecycle Environment | RETIRE | — |
| 6.3 | Preserve Real Transcripts and Atomic Journals | MERGE | see triage overturn table |
| 6.4 | Reconstruct Interrupted Work Without Signaling History | MERGE | see triage overturn table |
| 6.6 | Reject Hostile or Private Diagnostic Inputs | RETIRE | — |
| 6.7 | Preserve Settings and Native Utility Actions Across Failure | MERGE | see triage overturn table |

**Totals:** 19 merged · 12 retired · 31 removed

---

## Epic 1: Restore Trustworthy `mas` and Target-Mac Truth — REMOVED

Every story in this epic was triaged out, so the whole section was removed
from `epics.md`. Its goal statement is preserved here verbatim.

Users and downstream acceptance work can rely on current, live-verified `mas` behavior and a dated six-Manager target-Mac oracle without stale unverified claims or synthetic-fixture substitution.

### Story 1.1: Restore Current `mas` and Release Truth

> **RETIRE** — evidence ceremony, no product content. See `story-triage-2026-07-24.md`.

As a Product Owner,
I want authoritative product and acceptance sources to reflect current `mas`, release-signing, and event behavior,
So that downstream evidence tests the product Pack-Manager actually intends to ship.

**Story Contract:**

- Criteria and historical baseline: `D23a-AC4` — `NONE`
- FR and requirement links: No direct FR primary mapping; PC-1 source-truth correction enables honest FR-1/FR-2 acceptance
- Primary readiness concern: Product Behavior
- Checkpoint: Batch 1
- Required test level: Product correction plus unit recurrence guard
- Execution lane / evidence depth: `forced-offline` / source-bound
- Dependencies: PC-1; map approval and DR-4 before evidence admission
- ASR and risk links: ASR-05 lane rules; ASR-04 admission contract; R-001, R-007, R-008
- Behavior-present handling: Not a `BP` row; the correction must precede recurrence-test credit
- Versioned scenario contract: `contracts/readiness/scenarios/v1/b1-pc1-source-truth.json`
- Scenario-contract digest: Unassigned; freeze the exact file as lowercase `sha256:<64-hex>` before implementation entry
- Expected evidence artifact: Immutable `b1-pc1-source-truth.json` result plus a human-readable source-truth report
- Accountable role: Product
- Assignee: Unassigned
- Calendar date: Unassigned
- Implementation entry: Blocked until assignee/date exist; admission additionally requires the approved map, frozen profile, and operational Registrar
- Candidate subjects and invalidation: Not applicable; this is source-bound work
- Attempt contract: Retain the first attempt; `runnerRetryCount = "0"`; any later authorized retry is a linked record and cannot replace the first result

**Acceptance Criteria:**

**Given** D23a, D25/D25a, D26, production registration, and the live `mas` captures define current behavior
**When** authoritative and user-visible product sources are reconciled
**Then** they no longer describe `mas` as unverified, synthetic-only, or categorically absent
**And** they no longer describe ad-hoc-only or non-notarized delivery as current
**And** they no longer enforce an obsolete five-event invariant that would place application-update state in Package Operation queue or History semantics.

**Given** the source correction is complete
**When** the versioned recurrence scenario runs from the forced-offline lane
**Then** it detects every prohibited stale-truth form and confirms the current six-event separation and superseding decisions
**And** the recurrence check cannot receive credit if the underlying correction is absent.

**Given** the first scenario attempt completes
**When** its immutable result bundle is produced
**Then** it records exact source provenance, the scenario contract and digest, the executed command, timestamps, and result artifacts
**And** ignored, skipped, collected-only, or automatically retried checks cannot produce PASS
**And** `D23a-AC4` becomes only **eligible for later FULL reassessment** through a separate candidate-bound Trace workflow.

### Story 1.2: Qualify the Initial Split Evidence Lanes

> **RETIRE** — evidence ceremony, no product content. See `story-triage-2026-07-24.md`.

As a QA Lead,
I want the forced-offline and provisioned-target-Mac lanes isolated and qualified before Batch 1 collection,
So that deterministic and live-host results cannot contaminate or substitute for one another.

**Story Contract:**

- Criteria and historical baseline: None; ASR-05 enabler work does not add denominator rows
- FR and requirement links: No direct FR implementation; ASR-05 and TIR-2/TIR-6/TIR-8 evidence-lane enabler
- Primary readiness concern: Reusable Test Infrastructure
- Checkpoint: Before any Batch 1 evidence collection
- Required test level: Contract, negative-isolation, and environment-qualification tests
- Execution lane / evidence depth: Enabler qualification across `forced-offline` and `provisioned-target-mac`; no criterion binding is assigned by this story
- Dependencies: Versioned provision profiles; isolated result namespaces; named assignee/date
- ASR and risk links: ASR-05 — QA accountable, CI execution mechanism; R-001, R-008
- Behavior-present handling: Not applicable
- Versioned scenario contract: `contracts/readiness/scenarios/v1/asr-05-initial-lane-isolation.json`
- Scenario-contract digest: Unassigned; freeze the exact file before implementation entry
- Expected evidence artifact: `asr-05-initial-lane-qualification.json` plus a human-readable isolation report
- Accountable role: QA
- Assignee: Unassigned
- Calendar date: Unassigned
- Implementation entry: Blocked until assignee/date and the two versioned provision profiles exist
- Candidate subjects and invalidation: No candidate subjects; the candidate-release lane is qualified separately before Epic 7
- Attempt contract: First qualification failure is retained; automatic runner/workflow retry count is zero

**Acceptance Criteria:**

**Given** prepared dependencies and a fresh clean checkout
**When** the forced-offline lane begins
**Then** outbound network is denied after preparation, real Manager mutation is unavailable, controlled state/process/time is used, undeclared host state is rejected, and results enter only the forced-offline namespace.

**Given** the designated target Mac and its versioned provision profile
**When** the provisioned-target-Mac lane begins
**Then** execution is serialized, host topology and drift are recorded, live dependencies are explicit, credentials/caches/results are isolated, and results enter only the target-Mac namespace.

**Given** deliberate cross-lane, shallow-depth, stale-profile, credential, cache, or result-substitution attempts
**When** the lane admission checks run
**Then** every substitution is rejected
**And** CI remains only the execution mechanism
**And** QA is the sole accountable role for ASR-05.

### Story 1.3: Verify the Live Six-Manager Target-Mac Topology

> **MERGE** into Story 2.1. See `story-triage-2026-07-24.md`.

As a macOS Pack-Manager user,
I want the designated target Mac to expose the required live Manager topology including `mas`,
So that compatibility work begins from a dated and drift-detected environment rather than an assumed fixture.

**Story Contract:**

- Criteria and historical baseline: `F1-AC7` — `NONE`; `D23a-AC1` — `PARTIAL`
- FR and requirement links: FR-1; TIR-6
- Primary readiness concern: Reusable Test Infrastructure
- Checkpoint: Batch 1
- Required test level: Live target-Mac acceptance
- Execution lane / evidence depth: `provisioned-target-mac` / environment-bound
- Dependencies: Stories 1.1 and 1.2; qualified target-Mac access/profile; live `mas`; approved map/profile and ASR-04 Registrar before evidence admission
- ASR and risk links: ASR-05, TIR-1, TIR-6, TIR-8; R-001, R-008
- Behavior-present handling: Both criteria are `BP`; absent or incorrect behavior creates Product Behavior work and a reviewed map revision before regression evidence can receive credit
- Versioned scenario contract: `contracts/readiness/scenarios/v1/b1-target-mac-topology.json`
- Scenario-contract digest: Unassigned; freeze the exact file before implementation entry
- Expected evidence artifact: `b1-target-mac-topology.json` with dated OS/build, architecture, provision-profile digest, ToolEnv, six Manager paths/versions/ownership/evidence, network mode, and drift result
- Accountable role: QA
- Assignee: Unassigned
- Calendar date: Unassigned
- Implementation entry: Blocked by target-Mac access/profile, assignee/date, map approval, frozen profile, and evidence admission capability
- Candidate subjects and invalidation: Not applicable; environment-bound evidence cannot be relabeled candidate-bound
- Attempt contract: First attempt retained; `runnerRetryCount = "0"`; any authorized retry is linked with an explanation

**Acceptance Criteria:**

**Given** the designated Mac matches its frozen provision profile
**When** the topology scenario executes serially
**Then** Homebrew, mise, npm, uv, rustup, and `mas` are detected with exact paths, versions when available, managed-by classification, and human-readable evidence
**And** the observed topology is compared with the profile and any drift fails visibly rather than changing the oracle silently.

**Given** either required behavior is absent or incorrect
**When** the behavior-present check evaluates the result
**Then** Product Behavior work is created before any regression claim
**And** neither criterion receives evidence credit from the environment result.

**Given** an executed first attempt with complete environment provenance
**When** the Registrar later admits the result under the frozen profile
**Then** ignored, skipped, collected-only, wrong-host, wrong-profile, or cross-lane results are rejected
**And** `F1-AC7` and `D23a-AC1` become only **eligible for later FULL reassessment**.

### Story 1.4: Capture the Real `mas` Correctness Oracle

> **MERGE** into Story 1.5. See `story-triage-2026-07-24.md`.

As a maintainer,
I want a provenance-bound live `mas` capture with a deterministic secondary replay,
So that parser correctness is grounded in observed bytes rather than a synthetic approximation.

**Story Contract:**

- Criteria and historical baseline: `D23a-AC2` — `PARTIAL`
- FR and requirement links: FR-2; TIR-2; TIR-6
- Primary readiness concern: Reusable Test Infrastructure
- Checkpoint: Batch 1
- Required test level: Live target-Mac capture plus secondary unit replay
- Execution lane / evidence depth: `provisioned-target-mac` / environment-bound
- Dependencies: Stories 1.1–1.3; qualified target Mac; immutable raw-capture destination; approved profile and Registrar before admission
- ASR and risk links: ASR-05, TIR-1, TIR-2, TIR-6, TIR-8; R-001, R-008
- Behavior-present handling: `BP`; missing or incorrect live behavior creates Product Behavior work before replay coverage can receive credit
- Versioned scenario contract: `contracts/readiness/scenarios/v1/b1-mas-live-capture.json`
- Scenario-contract digest: Unassigned; freeze the exact file before implementation entry
- Expected evidence artifact: Exact raw `mas list` and `mas outdated` bytes, `b1-mas-capture-provenance.json`, and the deterministic replay result
- Accountable role: QA
- Assignee: Unassigned
- Calendar date: Unassigned
- Implementation entry: Blocked by qualified target-Mac access, immutable storage, assignee/date, and evidence-admission prerequisites
- Candidate subjects and invalidation: Not applicable; replay remains source-bound support and does not upgrade the live result's environment binding
- Attempt contract: First live capture attempt and any failure remain immutable; `runnerRetryCount = "0"`

**Acceptance Criteria:**

**Given** live `mas` is installed on the qualified target Mac
**When** `mas list` and `mas outdated` are captured in one controlled refresh window
**Then** exact raw bytes, exit state, tool version, timestamp, OS/build, architecture, host/profile identity, and network/dependency mode are retained
**And** the capture is immutable and distinguishable from every synthetic fixture.

**Given** the captured bytes
**When** deterministic replay runs from the matching source commit
**Then** IDs, padded names, installed versions, latest versions, and whitespace boundaries are asserted without replacing or deepening the live evidence.

**Given** the behavior-present check fails or the live attempt is ignored, skipped, unexecuted, or automatically retried
**When** admission is evaluated
**Then** the attempt cannot PASS
**And** `D23a-AC2` becomes only **eligible for later FULL reassessment** after a valid admitted first-attempt chain exists.

### Story 1.5: Enforce `mas` Provenance and Fixture Honesty

> **RETIRE** — evidence ceremony, no product content. See `story-triage-2026-07-24.md`.

As a maintainer,
I want parser and fixture guards to distinguish real-format correctness from synthetic robustness,
So that future tests cannot silently reintroduce the obsolete `mas` oracle.

**Story Contract:**

- Criteria and historical baseline: `D23a-AC3` — `FULL`; `D23a-AC5` — `UNIT-ONLY`
- FR and requirement links: FR-2; TIR-2; TIR-6
- Primary readiness concern: Reusable Test Infrastructure
- Checkpoint: Batch 1 and `RV@B1`
- Required test level: Unit/contract
- Execution lane / evidence depth: `forced-offline` / source-bound
- Dependencies: Stories 1.1 and 1.4; exact candidate source association later; approved profile and Registrar before admission
- ASR and risk links: ASR-05, TIR-1, TIR-2, TIR-8, RE-10; R-001, R-008
- Behavior-present handling: `D23a-AC5` is `BP`; missing/incorrect policy behavior creates Product Behavior work before regression credit. `D23a-AC3` is historical FULL and receives no carry-forward
- Versioned scenario contract: `contracts/readiness/scenarios/v1/b1-mas-provenance-parser.json`
- Scenario-contract digest: Unassigned; freeze the exact file before implementation entry
- Expected evidence artifact: `b1-mas-provenance-parser.json` plus exact fixture inventory and raw-capture digest report
- Accountable role: QA
- Assignee: Unassigned
- Calendar date: Unassigned
- Implementation entry: Blocked by assignee/date and frozen scenario/profile inputs
- Candidate subjects and invalidation: No candidate artifact subjects; later association requires the same source commit and never upgrades source binding
- Attempt contract: First attempt retained; `runnerRetryCount = "0"`; retry branches or missing ordinal 1 fail closed

**Acceptance Criteria:**

**Given** the immutable real capture and any labeled synthetic robustness inputs
**When** the parser contract executes
**Then** the real capture proves ID, name, version, and padding behavior without stray whitespace
**And** synthetic inputs prove only crash resistance and expected shape
**And** no synthetic fixture can satisfy a real-format correctness assertion.

**Given** the historical `D23a-AC3` FULL status
**When** the revalidation scenario runs against the exact associated source commit
**Then** the prior status is treated only as planning history
**And** the result must independently meet the frozen profile slot.

**Given** a valid complete first-attempt result
**When** it is admitted at source depth
**Then** `D23a-AC3` and `D23a-AC5` become only **eligible for later FULL reassessment**
**And** no result is promoted, relabeled, or counted as readiness by this story.

---

## Epic 2: Make Detection and Refresh Fail Independently and Recover Usefully — PARTIAL

This epic is still live in `epics.md`, retaining Story 2.2. Only the stories below were removed.

### Story 2.1: Preserve Honest Absence and Complete Environment Evidence

> **MERGE** — owner recorded in the triage's overturn table.

As a Pack-Manager user,
I want absent Managers and Environment Report data to remain complete and actionable,
So that I can understand my Manager topology without false errors or missing evidence.

**Story Contract:**

- Criteria and historical baseline: `F1-AC6` — `PARTIAL`; `F1-AC8` — `PARTIAL`
- FR and requirement links: FR-1; FR-17; TIR-2
- Primary readiness concern: Reusable Test Infrastructure
- Checkpoint: Batch 2
- Required test level: Component
- Execution lane / evidence depth: `forced-offline` / source-bound
- Dependencies: Accepted Epic 1; qualified ASR-05 forced-offline lane; clipboard seam; approved profile and Registrar before admission
- ASR and risk links: ASR-05, TIR-1, TIR-2, TIR-8; R-001, R-003, R-008
- Behavior-present handling: Both criteria are `BP`; missing or incorrect behavior creates Product Behavior work and a reviewed map revision before regression evidence can receive credit
- Versioned scenario contract: `contracts/readiness/scenarios/v1/b2-absence-environment-report.json`
- Scenario-contract digest: Unassigned; freeze the exact file before implementation entry
- Expected evidence artifact: `b2-absence-environment-report.json` with component outcomes and clipboard call/result evidence
- Accountable role: Development
- Assignee: Unassigned
- Calendar date: Unassigned
- Implementation entry: Blocked by assignee/date and common profile/admission prerequisites
- Candidate subjects and invalidation: Not applicable; later candidate association requires an exact source match and never deepens source binding
- Attempt contract: Retain ordinal 1 and set `runnerRetryCount = "0"`; authorized retries remain linked and visible

**Acceptance Criteria:**

**Given** any supported Manager is absent
**When** detection state renders
**Then** the Manager is not invoked, displays muted Not installed treatment, shows its known install hint, and exposes copy success and failure without presenting absence as an error.

**Given** present and absent entries for all six Managers
**When** the user opens Environment Report
**Then** ToolEnv source/path and each Manager's path, version when available, managed-by state, evidence, and install hint are each rendered with their specified value
**And** Copy reports both success and actionable failure.

**Given** either behavior-present check fails
**When** the story result is classified
**Then** Product Behavior work is required before regression credit
**And** a valid admitted first attempt makes both criteria only **eligible for later FULL reassessment**.

### Story 2.3: Keep Offline Failures Isolated

> **RETIRE** — evidence ceremony, no product content. See `story-triage-2026-07-24.md`.

As a Pack-Manager user,
I want every network-dependent Manager to degrade independently when offline,
So that one network failure never blanks useful state across the application.

**Story Contract:**

- Criteria and historical baseline: `F2-AC8` — `PARTIAL`
- FR and requirement links: FR-3; FR-16; TIR-2
- Primary readiness concern: Reusable Test Infrastructure
- Checkpoint: Batch 2
- Required test level: Unit
- Execution lane / evidence depth: `forced-offline` / source-bound
- Dependencies: Accepted Epic 1; host-wide outbound denial and controlled Manager adapters; qualified ASR-05 lane
- ASR and risk links: ASR-05, TIR-1, TIR-2, TIR-8; R-003, R-008
- Behavior-present handling: `BP`; missing or incorrect offline isolation creates Product Behavior work and map reclassification before regression credit
- Versioned scenario contract: `contracts/readiness/scenarios/v1/b2-offline-isolation.json`
- Scenario-contract digest: Unassigned; freeze the exact file before implementation entry
- Expected evidence artifact: `b2-offline-isolation.json` with per-adapter request denial, retained snapshot, peer state, and visible error outcomes
- Accountable role: Development
- Assignee: Unassigned
- Calendar date: Unassigned
- Implementation entry: Blocked by assignee/date, qualified host-wide denial, and common profile/admission prerequisites
- Candidate subjects and invalidation: Not applicable
- Attempt contract: First failure remains retained; `runnerRetryCount = "0"`; a retry cannot erase the original offline outcome

**Acceptance Criteria:**

**Given** outbound network is denied beyond ordinary browser `fetch`
**When** every network-dependent Manager refreshes through its controlled adapter
**Then** each failure is localized to that Manager with a timeout or actionable error
**And** its Last-good Snapshot remains visible and labeled stale
**And** unaffected Managers and the rest of the application remain usable.

**Given** an adapter attempts an undeclared network, process, DNS, service-worker, or host-state path
**When** isolation enforcement observes it
**Then** the attempt fails visibly and cannot be reported as deterministic offline evidence.

**Given** the complete first-attempt matrix passes
**When** the result is admitted at source depth
**Then** `F2-AC8` becomes only **eligible for later FULL reassessment**.

### Story 2.4: Revalidate Stable Detection and Refresh State Truth

> **MERGE** into Story 2.3. See `story-triage-2026-07-24.md`.

As a QA Lead,
I want historically FULL ownership, merge, loading, and stale-state behavior re-executed against current source,
So that earlier green evidence is not carried forward automatically.

**Story Contract:**

- Criteria and historical baseline: `F1-AC5` — `FULL`; `F2-AC2` — `FULL`; `F2-AC5` — `FULL`; `F2-AC7` — `FULL`
- FR and requirement links: FR-1; FR-2; FR-3; FR-4; FR-16
- Primary readiness concern: Reusable Test Infrastructure
- Checkpoint: `RV@B2`
- Required test level: Unit plus component
- Execution lane / evidence depth: `forced-offline` / source-bound
- Dependencies: Stories 2.1–2.3; exact later candidate source commit; frozen scenario/profile inputs
- ASR and risk links: ASR-05, TIR-2, TIR-8, RE-10; R-001, R-003, R-008
- Behavior-present handling: Not `BP`; all four historical FULL statuses are planning history and receive no carry-forward
- Versioned scenario contract: `contracts/readiness/scenarios/v1/rv-b2-detection-refresh-truth.json`
- Scenario-contract digest: Unassigned; freeze the exact file before implementation entry
- Expected evidence artifact: `rv-b2-detection-refresh-truth.json` covering raw-before-canonical ownership, merge/overlay corpus, loading coexistence, stale retention, Retry, and peer isolation
- Accountable role: QA
- Assignee: Unassigned
- Calendar date: Unassigned
- Implementation entry: Blocked by assignee/date, exact source association, and common admission prerequisites
- Candidate subjects and invalidation: No candidate artifacts; source-bound results may be associated only with a matching candidate commit and remain source-bound
- Attempt contract: Ordinal 1 is mandatory and immutable; `runnerRetryCount = "0"`; later retries form a visible gapless chain

**Acceptance Criteria:**

**Given** raw mise shim paths and their canonical targets
**When** ownership classification executes
**Then** classification uses the raw path before canonicalization and preserves human-readable evidence.

**Given** inventory, outdated overlays, populated/loading Managers, and a failing refresh
**When** the revalidation corpus executes
**Then** overlays patch/append without data loss, populated and loading states coexist, failure retains stale data, Retry is offered, and peers remain intact.

**Given** the four historical FULL rows
**When** the current source-bound first attempt is evaluated
**Then** no prior result is carried forward
**And** each criterion becomes only **eligible for later FULL reassessment** after its current frozen-profile slot is admitted.

---

## Epic 3: Keep Package Choice, Plans, and Settings Exact and Understandable — PARTIAL

This epic is still live in `epics.md`, retaining Story 3.1, 3.2, 3.4, 3.5. Only the stories below were removed.

### Story 3.3: Build Plans from Every User Entry Point

> **MERGE** into Story UX-PB.1c. See `story-triage-2026-07-24.md`.

As a Pack-Manager user,
I want every update entry point — one Package, one Manager self-update, a per-Manager-wide action, and Update Everything — to stage eligible work into the one persistent editable Upgrade Plan or explain why planning failed,
So that nothing executes from an entry point and no work is authorized without a reviewable, editable command set.

**Story Contract:**

- Criteria and historical baseline: `F4-AC1` — `PARTIAL`
- FR and requirement links: FR-6; FR-7; TIR-2
- Primary readiness concern: Reusable Test Infrastructure
- Checkpoint: Batch 3
- Required test level: Unit plus component
- Execution lane / evidence depth: `forced-offline` / source-bound
- Dependencies: Stories 3.1–3.2; deterministic plan-builder success/failure seams
- ASR and risk links: ASR-05, TIR-1, TIR-2, TIR-8; R-003, R-008
- Behavior-present handling: `BP`; missing or incorrect entry/failure behavior creates Product Behavior work before regression credit
- Versioned scenario contract: `contracts/readiness/scenarios/v1/b3-plan-entry-points.json`
- Scenario-contract digest: Unassigned; freeze the exact file before implementation entry
- Expected evidence artifact: `b3-plan-entry-points.json` with exact requests, rendered commands/exclusions/warnings, and visible failure results
- Accountable role: Development
- Assignee: Unassigned
- Calendar date: Unassigned
- Implementation entry: Blocked by assignee/date and common admission prerequisites
- Candidate subjects and invalidation: Not applicable
- Attempt contract: First attempt retained; `runnerRetryCount = "0"`

**Acceptance Criteria:**

**Given** eligible state from the Dashboard or a Manager pane
**When** a single Package, a single Manager self-update, a per-Manager-wide action, or Update Everything is invoked
**Then** each eligible identity is added to the one persistent editable Upgrade Plan whose sidecar opens on the first addition, nothing executes, and the exact commands, exclusions, notes, warnings, and every explicit per-Manager self-update — surfaced as an individually removable plan item — are visible before authorization.

**Given** plan construction fails or inputs are stale
**When** the user invokes any entry class
**Then** no plan preview (`planId`) or durable plan attempt (`planAttemptId`) is admitted, the failure is visible and actionable, and the interface cannot present a confirmable stale plan as current.

**Given** the scenario passes without ignored/unexecuted checks
**When** the result is admitted
**Then** `F4-AC1` becomes only **eligible for later FULL reassessment**.

### Story 3.6: Revalidate Version Truth, mise Consequences, and Plan Defaults

> **MERGE** into Story 3.1. See `story-triage-2026-07-24.md`.

As a QA Lead,
I want historically FULL display and plan-policy behavior re-executed against current source,
So that non-semver truth, mise consequences, and plan defaults are not inherited from older evidence.

**Story Contract:**

- Criteria and historical baseline: `F3-AC5` — `FULL`; `F3-AC7` — `FULL`; `F4-AC3` — `FULL`
- FR and requirement links: FR-2; FR-5; FR-7; FR-8
- Primary readiness concern: Reusable Test Infrastructure
- Checkpoint: `RV@B3`
- Required test level: Unit plus component
- Execution lane / evidence depth: `forced-offline` / source-bound
- Dependencies: Stories 3.1–3.5; exact later candidate source commit
- ASR and risk links: ASR-05, TIR-2, TIR-8, RE-10; R-003, R-008
- Behavior-present handling: Not `BP`; historical FULL is planning history only
- Versioned scenario contract: `contracts/readiness/scenarios/v1/rv-b3-version-plan-truth.json`
- Scenario-contract digest: Unassigned; freeze the exact file before implementation entry
- Expected evidence artifact: `rv-b3-version-plan-truth.json` with verbatim values, mise/rust outcomes, plan defaults, exclusions, rebuild, and confirm-current results
- Accountable role: QA
- Assignee: Unassigned
- Calendar date: Unassigned
- Implementation entry: Blocked by assignee/date, exact source association, and common admission prerequisites
- Candidate subjects and invalidation: No candidate artifacts; source association never upgrades binding depth
- Attempt contract: Ordinal 1 retained; `runnerRetryCount = "0"`; later retries linked

**Acceptance Criteria:**

**Given** semantic, non-semver, development, and hash-like version values
**When** Package state renders and eligibility is evaluated
**Then** installed/latest strings remain verbatim and the Manager's Outdated verdict—not local comparison—remains authoritative.

**Given** the mise fixture and the complete plan-policy matrix
**When** revalidation executes
**Then** six Outdated mise rows and clean stable Rust behavior remain correct
**And** each Manager self-update enters the persistent Upgrade Plan as an explicit, individually-removable per-Manager membership—seeded when `Update Everything` runs but never a single global default-on toggle—greedy casks default off, rust dedup/explanations hold, stale rebuild requires reconfirmation, and confirm-current behavior matches the source contract.

**Given** the historical FULL statuses
**When** the current first attempt passes and is admitted
**Then** none is carried forward automatically
**And** all three criteria become only **eligible for later FULL reassessment**.

---

## Epic 4: Prove the Real Desktop Command-and-Event Boundary — REMOVED

Every story in this epic was triaged out, so the whole section was removed
from `epics.md`. Its goal statement is preserved here verbatim.

Users gain confidence that the desktop application they operate crosses the same frontend invocation, Tauri registration/serialization, Rust handlers, and event channels that production ships, rather than relying on separately passing fake-browser and handler-only suites.

### Story 4.1: Establish the Versioned Production Boundary Contract

> **RETIRE** — evidence ceremony, no product content. See `story-triage-2026-07-24.md`.

As an Architecture owner,
I want one versioned catalog and registration source for the production command/event surface,
So that shipped and native-acceptance boundaries cannot drift independently.

**Story Contract:**

- Criteria and historical baseline: None; ASR-01 enabler work adds no denominator row
- FR and requirement links: No direct FR implementation; ASR-01/TIR-3 production-boundary enabler for FR-1, FR-3, FR-6–FR-9, and FR-12
- Primary readiness concern: Reusable Test Infrastructure
- Checkpoint: Batch 4 foundation
- Required test level: Contract/schema/set-equality
- Execution lane / evidence depth: `forced-offline` / source-bound qualification
- Dependencies: Accepted Epic 1; selected native harness/test runner; named assignee/date
- ASR and risk links: ASR-01 — Architecture accountable, Development/QA implement and use; ASR-05; R-002, R-008
- Behavior-present handling: Not applicable
- Versioned scenario contract: `contracts/readiness/scenarios/v1/asr-01-boundary-catalog.json`
- Scenario-contract digest: Unassigned; freeze the exact file before implementation entry
- Expected evidence artifact: `asr-01-boundary-catalog-qualification.json` plus catalog/registration/wrapper/subscription inventory
- Accountable role: Architecture
- Assignee: Unassigned
- Calendar date: Unassigned
- Implementation entry: Blocked by native harness decision, assignee/date, and frozen contract inputs
- Candidate subjects and invalidation: No candidate artifacts; production surface changes require one atomic catalog/registration/wire/acceptance update
- Plan-attempt boundary extension (AD-16): the same one atomic catalog/registration/wire/acceptance change also adds the revised `PlanIntent`, the one-use preview `planId`, the durable `planAttemptId`, the `execute_plan` admission return, the plan-attempt query/list/replay contract, `cancel_plan_attempt`, per-Operation `planAttemptId` event correlation, the trusted interaction-attention event or typed attention reason, and the revised Settings fields as one AD-3 contract change; command and event counts may change and stay a current baseline
- Attempt contract: First qualification attempt retained; automatic retry count zero

**Acceptance Criteria:**

**Given** the versioned `contracts/tauri-boundary/v1.json` schema
**When** the catalog is generated or validated
**Then** command and event entries use the architecture-defined closed fields, stable ordering, unique names/vectors, schema digests, and scenario-contract digests.

**Given** production Rust registration, Rust/TypeScript wire schemas, wrappers, subscriptions, fixtures, and native inventory
**When** set-equality validation runs
**Then** every set is exactly equal to the catalog
**And** the verified 20 commands and six events are reported as a current baseline rather than permanent counts.

**Given** a deliberate surface change
**When** any affected contract component is missing or divergent
**Then** qualification fails closed
**And** no duplicated test registry or test-only command/event can satisfy ASR-01.

**Given** the AD-16 plan-attempt boundary extension — the revised `PlanIntent`, the one-use preview `planId`, the durable `planAttemptId`, the `execute_plan` admission return, the plan-attempt query/list/replay contract, `cancel_plan_attempt`, per-Operation `planAttemptId` correlation across `op:status`, `op:output`, and attention events, the trusted interaction-attention event or typed attention reason, and the revised Settings fields
**When** their commands, types, events, and IPC fixtures enter the catalog
**Then** every revised command/type/event/fixture is admitted through the same single atomic catalog/registration/wire/acceptance change, with set-equality across production Rust registration, Rust/TypeScript wire schemas and guards, wrappers, subscriptions, shared fixtures, native acceptance vectors, and native inventory
**And** the changed command and event counts remain a current baseline rather than permanent counts, and no plan-attempt surface may be registered outside this one atomic contract change.

### Story 4.2: Deliver the Deterministic Process-Control Core

> **RETIRE** — evidence ceremony, no product content. See `story-triage-2026-07-24.md`.

As a Development owner,
I want typed control points for process, time, executable discovery, and ToolEnv effects,
So that the real boundary can exercise failure safely without weakening production behavior.

**Story Contract:**

- Criteria and historical baseline: None; ASR-02 enabler work adds no denominator row
- FR and requirement links: No direct FR implementation; ASR-02/TIR-4 process-control enabler for FR-9 and FR-11–FR-15
- Primary readiness concern: Reusable Test Infrastructure
- Checkpoint: Batch 4 foundation; ASR-02 process-control core accepted before Batch 5
- Required test level: Unit, contract, controlled native qualification
- Execution lane / evidence depth: `forced-offline` / controlled environment-bound
- Dependencies: Story 4.1; controlled-helper design; non-distributable harness composition
- ASR and risk links: ASR-02 — Development accountable, Platform capability area; ASR-01, ASR-05; R-002, R-004, R-008
- Behavior-present handling: Not applicable
- Versioned scenario contract: `contracts/readiness/scenarios/v1/asr-02-control-foundation.json`
- Scenario-contract digest: Unassigned; freeze the exact file before implementation entry
- Expected evidence artifact: `asr-02-control-foundation-qualification.json` with process/time/discovery port coverage, controlled-helper traces, and distributable-graph exclusion
- Accountable role: Development
- Assignee: Unassigned
- Calendar date: Unassigned
- Implementation entry: Blocked by controlled-helper decision and assignee/date
- Candidate subjects and invalidation: Not applicable; candidate acceptance later uses unchanged production adapters
- Attempt contract: Preserve first attempt; `runnerRetryCount = "0"`

**Acceptance Criteria:**

**Given** process spawn/output/exit/stdin/signals, monotonic/wall time, executable discovery, ToolEnv, and lock effects
**When** the application composition is inspected
**Then** material effects cross typed ports and direct covered OS calls are confined to production adapters.

**Given** the native acceptance composition
**When** controlled adapters and child helpers are constructed
**Then** they exist only as construction-time dependencies of a non-distributable target
**And** no release feature, environment variable, command, hidden selector, or alternate business path can activate them.

**Given** production adapters
**When** port extraction is completed
**Then** structured absolute argv, sanitized environment, null stdin, no shell/sudo/password path, and lock-set safety remain fail-closed
**And** this story does not claim acceptance of the separately timed ASR-02 filesystem/native-utility extension required before Batch 6 or updater extension required before Batch 7.

### Story 4.3: Cross Native Startup, Detection, and Re-detect

> **RETIRE** — evidence ceremony, no product content. See `story-triage-2026-07-24.md`.

As a Pack-Manager user,
I want launch and every Re-detect entry point to cross the real production boundary,
So that Finder/Dock startup, ToolEnv, and Manager-version behavior are not proven only below Tauri.

**Story Contract:**

- Criteria and historical baseline: `F1-AC1` — `PARTIAL`; `F1-AC2` — `PARTIAL`; `F1-AC3` — `UNIT-ONLY`; `F1-AC4` — `UNIT-ONLY`
- FR and requirement links: FR-1; TIR-3
- Primary readiness concern: Reusable Test Infrastructure
- Checkpoint: Batch 4
- Required test level: Real native Tauri E2E
- Execution lane / evidence depth: `forced-offline` / controlled environment-bound
- Dependencies: Stories 4.1–4.2; accepted Epic 1 truth; isolated application state and controlled executables
- ASR and risk links: ASR-01, ASR-02 foundations, ASR-05, TIR-3/TIR-4/TIR-8; R-002, R-008
- Behavior-present handling: Not `BP`; failures still require honest Product Behavior classification rather than infrastructure masking
- Versioned scenario contract: `contracts/readiness/scenarios/v1/b4-native-detection.json`
- Scenario-contract digest: Unassigned; freeze the exact file before implementation entry
- Expected evidence artifact: `b4-native-detection.json` with frontend invocation, wire bytes, handler trace, event trace, controlled process calls, and startup-order result
- Accountable role: Architecture
- Assignee: Unassigned
- Calendar date: Unassigned
- Implementation entry: Blocked by ASR-01/02 foundations, assignee/date, and common admission prerequisites
- Candidate subjects and invalidation: Not applicable; controlled native evidence is not packaged-candidate evidence
- Attempt contract: First attempt retained; `runnerRetryCount = "0"` for runner and workflow retries

**Acceptance Criteria:**

**Given** isolated startup with all six controlled Manager states
**When** the frontend subscribes, hydrates, and invokes detection
**Then** subscription precedes hydration, the real bridge/registration/serialization/handler/event path is crossed, and all six entries arrive coherently.

**Given** every required Re-detect UI entry
**When** the user invokes Re-detect
**Then** each entry crosses the same production path, rebuilds ToolEnv coherently, and cannot rely on fake-browser IPC or direct handler calls.

**Given** controlled PATH/login-shell success, noise, timeout, cleanup, present-version, missing-version, and probe-failure cases
**When** detection runs
**Then** the exact ToolEnv source/evidence and present-versus-absent consequences are correct
**And** all four criteria become only **eligible for later FULL reassessment** after valid admission.

### Story 4.4: Cross All-Six Native Refresh Ordering

> **MERGE** into Story 2.2. See `story-triage-2026-07-24.md`.

As a Pack-Manager user,
I want every present Manager to run inventory before Outdated checks through real Tauri,
So that refresh ordering is proven at the boundary where production can diverge.

**Story Contract:**

- Criteria and historical baseline: `F2-AC1` — `UNIT-ONLY`
- FR and requirement links: FR-3; TIR-3
- Primary readiness concern: Reusable Test Infrastructure
- Checkpoint: Batch 4
- Required test level: Real native Tauri E2E
- Execution lane / evidence depth: `forced-offline` / controlled environment-bound
- Dependencies: Stories 4.1–4.3; controlled all-six command plans and event capture
- ASR and risk links: ASR-01, ASR-02, ASR-05, TIR-3/TIR-4/TIR-8; R-002, R-008
- Behavior-present handling: Not `BP`
- Versioned scenario contract: `contracts/readiness/scenarios/v1/b4-native-refresh-order.json`
- Scenario-contract digest: Unassigned; freeze the exact file before implementation entry
- Expected evidence artifact: `b4-native-refresh-order.json` with exact per-Manager command order, wire requests/responses, phases, events, and peer concurrency
- Accountable role: Architecture
- Assignee: Unassigned
- Calendar date: Unassigned
- Implementation entry: Blocked by ASR foundations, assignee/date, and common admission prerequisites
- Candidate subjects and invalidation: Not applicable
- Attempt contract: Preserve complete first-run native trace; `runnerRetryCount = "0"`

**Acceptance Criteria:**

**Given** six present controlled Managers
**When** Refresh All crosses the production frontend-to-handler boundary
**Then** every Manager executes its documented inventory step before its Outdated step
**And** independent Managers may overlap while per-Manager order remains deterministic.

**Given** request, response, phase, snapshot, and Operation events
**When** the scenario reconciles human and machine traces
**Then** all wire bytes and event identities agree with the catalog and scenario contract.

**Given** a complete admitted first attempt
**When** aggregation later evaluates the slot
**Then** `F2-AC1` becomes only **eligible for later FULL reassessment**.

### Story 4.5: Revalidate Native Refresh and Contract Equality

> **MERGE** into Story 2.2. See `story-triage-2026-07-24.md`.

As a QA Lead,
I want historical refresh and IPC-contract evidence rerun across the current real boundary,
So that previous FULL status does not hide a registration, fixture, or runtime-guard divergence.

**Story Contract:**

- Criteria and historical baseline: `F2-AC4` — `FULL`; `F12-AC3` — `FULL`
- FR and requirement links: FR-3; FR-9; TIR-2; TIR-3
- Primary readiness concern: Reusable Test Infrastructure
- Checkpoint: `RV@B4`
- Required test level: Unit, component, and real native Tauri E2E
- Execution lane / evidence depth: `forced-offline` / controlled environment-bound
- Dependencies: Stories 4.1–4.4; exact source association
- ASR and risk links: ASR-01, ASR-05, TIR-3/TIR-8, RE-10; R-002, R-008
- Behavior-present handling: Not `BP`; historical FULL is not carried forward
- Versioned scenario contract: `contracts/readiness/scenarios/v1/rv-b4-refresh-contract.json`
- Scenario-contract digest: Unassigned; freeze the exact file before implementation entry
- Expected evidence artifact: `rv-b4-refresh-contract.json` with fresh-detection/parallel-refresh trace and Rust/TypeScript fixture/catalog equality report
- Accountable role: Architecture
- Assignee: Unassigned
- Calendar date: Unassigned
- Implementation entry: Blocked by accepted ASR-01, assignee/date, exact source association, and admission prerequisites
- Candidate subjects and invalidation: No candidate artifacts; association never deepens environment binding
- Attempt contract: Ordinal 1 retained; `runnerRetryCount = "0"`

**Acceptance Criteria:**

**Given** stale prior detection and six controlled Managers
**When** Refresh All is invoked through production Tauri
**Then** it obtains fresh detection, safely parallelizes independent Managers, and emits catalog-conforming responses/events.

**Given** every representative IPC fixture and the versioned catalog
**When** Rust byte serialization, TypeScript runtime guards, registration, wrappers, subscriptions, and native vectors are compared
**Then** exact set/byte equality holds and fake fixtures do not replace the real crossing.

**Given** both historical FULL rows
**When** the current attempt is admitted
**Then** neither prior status is carried forward
**And** both become only **eligible for later FULL reassessment**.

### Story 4.6: Revalidate Reviewed Plans and Native Admission

> **MERGE** into Story UX-PB.2b. See `story-triage-2026-07-24.md`.

As a QA Lead,
I want exact reviewed plans, PackageRefs, and Manager self-update identities revalidated through native serialization, the separate final confirmation, and durable plan-attempt admission,
So that an older FULL result cannot conceal a transport or capability mismatch.

**Story Contract:**

- Criteria and historical baseline: `F4-AC2` — `FULL`; `F5-AC2` — `FULL`
- FR and requirement links: FR-6; FR-7
- Primary readiness concern: Reusable Test Infrastructure
- Checkpoint: `RV@B4`
- Required test level: Unit, component, browser, and real native Tauri E2E
- Execution lane / evidence depth: `forced-offline` / controlled environment-bound
- Dependencies: Stories 4.1–4.4; Epic 3 plan/selection behavior; exact source association
- ASR and risk links: ASR-01, ASR-05, TIR-3/TIR-8, RE-10; R-002, R-003, R-008
- Behavior-present handling: Not `BP`; historical FULL is planning history only
- Versioned scenario contract: `contracts/readiness/scenarios/v1/rv-b4-plan-admission.json`
- Scenario-contract digest: Unassigned; freeze the exact file before implementation entry
- Expected evidence artifact: `rv-b4-plan-admission.json` with reviewed command bytes, exclusions/warnings/notes, exact PackageRefs and Manager self-update identities, one-use preview `planId` result, the durable `planAttemptId` and its propagation, the separate final confirmation, and draft/selection-clear-on-Activity timing
- Accountable role: Architecture
- Assignee: Unassigned
- Calendar date: Unassigned
- Implementation entry: Blocked by accepted ASR-01, assignee/date, exact source association, and admission prerequisites
- Candidate subjects and invalidation: Not applicable
- Attempt contract: Preserve first attempt and all failures; `runnerRetryCount = "0"`

**Acceptance Criteria:**

**Given** representative all-outdated and explicit-selection requests
**When** plans cross the production boundary
**Then** exact commands, exclusions, warnings, notes, request semantics, and serialized bytes match the reviewed plan.

**Given** update-selected staged into the draft plan and reviewed through component, browser, and native paths, then passed through the separate final `Proceed with Upgrade Plan?` confirmation
**When** the one-use preview `planId` authorizes the reviewed preview and atomic admission mints one durable `planAttemptId` with its created Operation IDs, or admission is rejected
**Then** the exact PackageRefs and Manager self-update identities reach final admission, no unintended Package is added, and draft/selection membership clears only when the confirmed plan becomes Activity — not at draft addition and not at the bounded one-use `planId` preview admission
**And** the minted `planAttemptId` propagates identically through Rust/TypeScript wire models, `op:status`/`op:output`/attention events, transcript metadata, crash-journal start/finish records, in-memory stores, and diagnostics, while a rejected admission mints no `planAttemptId`, admits no Package, and clears nothing.

**Given** both historical FULL rows
**When** current evidence is admitted
**Then** neither status is inherited
**And** both become only **eligible for later FULL reassessment**.

---

## Epic 5: Make Manager Updates and Process Lifecycles Safe and Honest — REMOVED

Every story in this epic was triaged out, so the whole section was removed
from `epics.md`. Its goal statement is preserved here verbatim.

Users can trust dynamic Manager update Routes, scheduler locks, exact live output, stall/timeout choices, cancellation, shutdown, null-input behavior, and the closed D26 transcript repair through controlled native process evidence.

### Story 5.1: Refresh Every Routed Subject and Executor

> **RETIRE** — evidence ceremony, no product content. See `story-triage-2026-07-24.md`.

As a Pack-Manager user,
I want successful routed updates to refresh every affected Manager,
So that subject and executor state cannot disagree after an update.

**Story Contract:**

- Criteria and historical baseline: `F2-AC9` — `PARTIAL`
- FR and requirement links: FR-3; FR-16; TIR-2
- Primary readiness concern: Reusable Test Infrastructure
- Checkpoint: Batch 5
- Required test level: Unit
- Execution lane / evidence depth: `forced-offline` / source-bound
- Dependencies: Accepted Epic 4; ASR-02 core; routed self-update fixtures
- ASR and risk links: ASR-02, ASR-05, TIR-1/TIR-2/TIR-8; R-003, R-004, R-008
- Behavior-present handling: `BP`; missing/incorrect dual refresh creates Product Behavior work before regression credit
- Versioned scenario contract: `contracts/readiness/scenarios/v1/b5-routed-dual-refresh.json`
- Scenario-contract digest: Unassigned; freeze the exact file before implementation entry
- Expected evidence artifact: `b5-routed-dual-refresh.json` with route, subject/executor refresh calls, ordering, coalescing, and final state
- Accountable role: Development
- Assignee: Unassigned
- Calendar date: Unassigned
- Implementation entry: Blocked by accepted Epic 4/ASR-02 core, assignee/date, and common admission prerequisites
- Candidate subjects and invalidation: Not applicable
- Attempt contract: First attempt retained; `runnerRetryCount = "0"`

**Acceptance Criteria:**

**Given** a successful in-band or same-Manager update
**When** terminal success is processed
**Then** the affected Manager refreshes exactly as specified without duplicate competing refreshes.

**Given** a successful routed self-update with different subject and executor
**When** terminal success is processed
**Then** both subject and executor refresh coherently and coalescing preserves one refresh per affected Manager.

**Given** the behavior-present check and all cases pass
**When** the source-bound result is admitted
**Then** `F2-AC9` becomes only **eligible for later FULL reassessment**.

### Story 5.2: Prove Dynamic Self-Update Routes and Manager Header/Card Plan State

> **MERGE** — owner recorded in the triage's overturn table.

As a Pack-Manager user,
I want Manager self-update Routes, versions, and queue consequences to reflect current topology,
So that I understand what will run, through which executor, and why.

**Story Contract:**

- Criteria and historical baseline: `F6-AC1` — `PARTIAL`; `F6-AC2` — `UNIT-ONLY`; `F6-AC3` — `INTEGRATION-ONLY`
- FR and requirement links: FR-2; FR-4; FR-11; TIR-3; TIR-4
- Primary readiness concern: Reusable Test Infrastructure
- Checkpoint: Batch 5
- Required test level: Component plus real native Tauri E2E
- Execution lane / evidence depth: `forced-offline` / controlled environment-bound
- Dependencies: Accepted Epic 4; ASR-02 controlled executables; current detection/snapshot fixtures
- ASR and risk links: ASR-01, ASR-02, ASR-05, TIR-3/TIR-4/TIR-8; R-002, R-003, R-004, R-008
- Behavior-present handling: Not `BP`; missing behavior is still classified honestly as Product Behavior rather than hidden by the harness
- Versioned scenario contract: `contracts/readiness/scenarios/v1/b5-self-update-routes.json`
- Scenario-contract digest: Unassigned; freeze the exact file before implementation entry
- Expected evidence artifact: `b5-self-update-routes.json` with route-precedence decisions, serialized self state, production events, Manager Header/Card text/actions with `IN PLAN`/`Remove` plan-membership states, and queue states
- Accountable role: Development
- Assignee: Unassigned
- Calendar date: Unassigned
- Implementation entry: Blocked by accepted ASR-01/02 core, assignee/date, and common admission prerequisites
- Candidate subjects and invalidation: Not applicable; controlled native evidence is not candidate-bound
- Attempt contract: Complete first native trace retained; `runnerRetryCount = "0"`

**Acceptance Criteria:**

**Given** in-band, delegated, refresh-based, standalone, and unavailable topologies
**When** route precedence executes using fresh detection and own-outdated rows
**Then** the exact dynamic Route and command are selected without hardcoding ownership
**And** npm's in-band override and mise/uv delegated consequences remain correct.

**Given** hoisted self rows and cross-Manager self-version joins
**When** snapshots cross real IPC
**Then** installed state is preserved, latest state is patched, the self row is not duplicated, and wire values match the catalog.

**Given** production status/snapshot events
**When** the Manager Header/Card renders
**Then** its Route, unavailable-executor, queued, npm-reset, and `IN PLAN`/`Remove` draft-membership states render their specified labels and actions, and its `Update Manager` action stages an independent, removable Manager self-update into the one persistent Upgrade Plan and never executes directly
**And** all three criteria become only **eligible for later FULL reassessment** after valid admission.

### Story 5.3: Reject Unsafe Spawns and Hold Complete Locks

> **MERGE** — owner recorded in the triage's overturn table.

As a Pack-Manager user,
I want self-updates to bind only approved commands and hold every required lock,
So that unavailable Routes and concurrent tree changes cannot start unsafe work.

**Story Contract:**

- Criteria and historical baseline: `F6-AC4` — `UNIT-ONLY`; `F6-AC5` — `UNIT-ONLY`
- FR and requirement links: FR-4; FR-9; FR-11; FR-12; TIR-3; TIR-4
- Primary readiness concern: Reusable Test Infrastructure
- Checkpoint: Batch 5
- Required test level: Real native Tauri E2E
- Execution lane / evidence depth: `forced-offline` / controlled environment-bound
- Dependencies: Story 5.2; ASR-02 controlled spawn/lock helpers; accepted scheduler foundation
- ASR and risk links: ASR-02, ASR-05, TIR-4/TIR-8; R-004, R-008
- Behavior-present handling: Not `BP`
- Versioned scenario contract: `contracts/readiness/scenarios/v1/b5-route-spawn-locks.json`
- Scenario-contract digest: Unassigned; freeze the exact file before implementation entry
- Expected evidence artifact: `b5-route-spawn-locks.json` with exact spawn requests/no-spawn rejections and timestamped executor/subject/shared-tree lock timelines
- Accountable role: Development
- Assignee: Unassigned
- Calendar date: Unassigned
- Implementation entry: Blocked by ASR-02 core, assignee/date, and common admission prerequisites
- Candidate subjects and invalidation: Not applicable
- Attempt contract: First attempt and failures retained; `runnerRetryCount = "0"`

**Acceptance Criteria:**

**Given** an available or unavailable self-update Route
**When** the native command handler runs
**Then** the exact resolved structured argv is bound and spawned only for the available Route
**And** unavailable, stale, altered, or privilege-seeking requests reject before spawn.

**Given** routed and mise-managed operations
**When** scheduler timelines execute
**Then** executor, subject, and applicable shared-tree locks are acquired atomically, held through terminal state, and released once
**And** unrelated work may proceed safely.

**Given** complete admitted evidence
**When** the two slots are evaluated
**Then** both criteria become only **eligible for later FULL reassessment**.

### Story 5.4: Preserve Native Output and Activity Boundaries

> **MERGE** — owner recorded in the triage's overturn table.

As a Pack-Manager user,
I want live output and Activity state to remain complete, bounded, and correlated,
So that I can follow work without losing durable context.

**Story Contract:**

- Criteria and historical baseline: `F7-AC1` — `UNIT-ONLY`; `F7-AC2` — `PARTIAL`
- FR and requirement links: FR-13; NFR-3; TIR-3; TIR-4
- Primary readiness concern: Reusable Test Infrastructure
- Checkpoint: Batch 5
- Required test level: Component plus real native Tauri E2E
- Execution lane / evidence depth: `forced-offline` / controlled environment-bound
- Dependencies: Accepted Epic 4; ASR-02 controlled output/time; transcript sink
- ASR and risk links: ASR-01, ASR-02, ASR-05, TIR-3/TIR-4/TIR-8; R-003, R-004, R-008
- Behavior-present handling: Not `BP`
- Versioned scenario contract: `contracts/readiness/scenarios/v1/b5-output-activity.json`
- Scenario-contract digest: Unassigned; freeze the exact file before implementation entry
- Expected evidence artifact: `b5-output-activity.json` with stream bytes, flush-trigger timestamps/counts, event identity, Activity state, memory cap, and transcript comparison
- Accountable role: Development
- Assignee: Unassigned
- Calendar date: Unassigned
- Implementation entry: Blocked by ASR-02 controlled output/time, assignee/date, and common admission prerequisites
- Candidate subjects and invalidation: Not applicable
- Attempt contract: Retain complete first-attempt event/transcript output; `runnerRetryCount = "0"`

**Acceptance Criteria:**

**Given** controlled stdout/stderr, carriage returns, 50-millisecond time, 64-line count, 8-KiB size, and drop/cap boundaries
**When** native output batching runs
**Then** stream identity and byte order are preserved and each documented boundary flushes exactly as required.

**Given** a confirmed plan attempt with more than 5,000 live lines and production Operation events correlated by its `planAttemptId`
**When** the one shared confirmed-plan model renders in the contextual right sidecar and the full Activity destination
**Then** both surfaces render that single plan model identically, presenting human-readable Package/Manager progress first and exact command/output as secondary evidence, and they append, repaint, pin/unpin, bound memory over the 5,000+ lines, and preserve Operation evidence and the complete durable transcript.

**Given** the same shared confirmed-plan model across Manager navigation and at terminal state
**When** the draft sidecar lifecycle is exercised
**Then** the sidecar is absent when empty and otherwise persists as a draft across Manager navigation, transforming into a persistent Results Summary only once all plan Operations and required verification refreshes reach terminal state.

**Given** human and machine outputs agree on the first attempt
**When** admission evaluates them
**Then** both criteria become only **eligible for later FULL reassessment**.

### Story 5.5: Cancel, Stall, Time Out, and Shut Down Honestly

> **MERGE** — owner recorded in the triage's overturn table.

As a Pack-Manager user,
I want plan cancellation, stalls, timeouts, trusted interaction detection, and shutdown to reach explicit safe outcomes,
So that silent or stuck work never waits for hidden input or leaves dishonest state.

**Story Contract:**

- Criteria and historical baseline: `F7-AC3` — `PARTIAL`; `F7-AC4` — `PARTIAL`
- FR and requirement links: FR-12; FR-14; FR-15; TIR-4
- Primary readiness concern: Reusable Test Infrastructure
- Checkpoint: Batch 5
- Required test level: Component plus real native Tauri E2E
- Execution lane / evidence depth: `forced-offline` / controlled environment-bound
- Dependencies: ASR-02 signal/time/stdin/descendant helpers; accepted Epic 4 close/run wiring foundation
- ASR and risk links: ASR-01, ASR-02, ASR-05, TIR-4/TIR-8; R-003, R-004, R-008
- Behavior-present handling: Not `BP`
- Versioned scenario contract: `contracts/readiness/scenarios/v1/b5-cancel-stall-timeout.json`
- Scenario-contract digest: Unassigned; freeze the exact file before implementation entry
- Expected evidence artifact: `b5-cancel-stall-timeout.json` with child/descendant signal traces, null-stdin observation, state/events, journal/transcript footers, and cleanup proof
- Accountable role: Development
- Assignee: Unassigned
- Calendar date: Unassigned
- Implementation entry: Blocked by ASR-02 core, assignee/date, and common admission prerequisites
- Candidate subjects and invalidation: Not applicable
- Attempt contract: Preserve the first terminal-path result and failure; `runnerRetryCount = "0"`

**Acceptance Criteria:**

**Given** the one active confirmed Upgrade Plan attempt with controlled Operations that exit on SIGTERM or require SIGKILL
**When** `Cancel plan` or application shutdown occurs
**Then** every still-running Operation bound to that `planAttemptId` is cancelled, its complete process group is signaled with grace/escalation recorded and children reaped, queued attempt Operations never start and are recorded as `Skipped`, terminal state/events/journal/transcript agree, no rollback is promised, and cancellation stays immediate with no second confirmation dialog.

**Given** a null-stdin silent process under the active plan attempt and controlled time
**When** stall and hard-cap thresholds are reached
**Then** the stalled Operation presents exactly `Keep waiting`, `Copy command`, and `Cancel plan`, the hard cap ends it as `Timed out`, and every consequence matches the contract
**And** no password/admin input is possible.

**Given** native output from an Operation in the active plan attempt
**When** a closed, adapter-specific classifier recognizes a supported prompt signature
**Then** `Interaction required` is presented with `Copy command` and `Cancel plan` and no response is ever accepted
**And** unrecognized silence remains `Stalled` and arbitrary output is never guessed to be a prompt.

**Given** complete first-attempt cleanup evidence
**When** admission occurs
**Then** both criteria become only **eligible for later FULL reassessment**.

### Story 5.6: Repair Only the Allowlisted Unterminated `mas` Notice

> **MERGE** into Story 5.4. See `story-triage-2026-07-24.md`.

As a Pack-Manager user,
I want the known unterminated `mas` notice displayed readably through native output,
So that the transcript remains understandable without general heuristic rewriting.

**Story Contract:**

- Criteria and historical baseline: `D26-AC1` — `UNIT-ONLY`
- FR and requirement links: FR-15; TIR-2; TIR-3
- Primary readiness concern: Reusable Test Infrastructure
- Checkpoint: Batch 5
- Required test level: Real native Tauri E2E
- Execution lane / evidence depth: `forced-offline` / controlled environment-bound
- Dependencies: Story 5.4; real-format `mas` bytes from Epic 1; ASR-02 controlled output
- ASR and risk links: ASR-01, ASR-02, ASR-05, TIR-2/TIR-3/TIR-4/TIR-8; R-001, R-004, R-008
- Behavior-present handling: Not `BP`
- Versioned scenario contract: `contracts/readiness/scenarios/v1/b5-d26-native-notice.json`
- Scenario-contract digest: Unassigned; freeze the exact file before implementation entry
- Expected evidence artifact: `b5-d26-native-notice.json` with raw bytes, native event batches, rendered lines, and transcript bytes
- Accountable role: Development
- Assignee: Unassigned
- Calendar date: Unassigned
- Implementation entry: Blocked by Epic 1 capture, ASR-02 output control, assignee/date, and admission prerequisites
- Candidate subjects and invalidation: Not applicable
- Attempt contract: Preserve first raw/native attempt; `runnerRetryCount = "0"`

**Acceptance Criteria:**

**Given** the exact allowlisted unterminated notice followed by glued output
**When** bytes pass through the production reader, event dispatcher, UI, and transcript
**Then** exactly one readability break is inserted after the literal notice and subsequent bytes remain unchanged.

**Given** a notice already at the end of a terminated buffer
**When** it is processed
**Then** no extra break is inserted.

**Given** an admitted first attempt with raw/native comparison
**When** the slot is evaluated
**Then** `D26-AC1` becomes only **eligible for later FULL reassessment**.

### Story 5.7: Prove the D26 Rule Cannot Become Heuristic

> **RETIRE** — evidence ceremony, no product content. See `story-triage-2026-07-24.md`.

As a maintainer,
I want a closed negative corpus around the D26 exception,
So that unrelated or nearly matching output is never rewritten.

**Story Contract:**

- Criteria and historical baseline: `D26-AC2` — `UNIT-ONLY`
- FR and requirement links: FR-15; TIR-2
- Primary readiness concern: Reusable Test Infrastructure
- Checkpoint: Batch 5
- Required test level: Unit/property table
- Execution lane / evidence depth: `forced-offline` / source-bound
- Dependencies: Story 5.6; closed literal allowlist and versioned corpus
- ASR and risk links: ASR-02, ASR-05, TIR-1/TIR-2/TIR-8; R-004, R-008
- Behavior-present handling: `BP`; missing/incorrect closed-corpus behavior creates Product Behavior work before regression credit
- Versioned scenario contract: `contracts/readiness/scenarios/v1/b5-d26-negative-corpus.json`
- Scenario-contract digest: Unassigned; freeze the exact file before implementation entry
- Expected evidence artifact: `b5-d26-negative-corpus.json` with input/output byte pairs for terminated, repeated, near-match, unrelated, and generic mid-line cases
- Accountable role: Development
- Assignee: Unassigned
- Calendar date: Unassigned
- Implementation entry: Blocked by assignee/date and common admission prerequisites
- Candidate subjects and invalidation: Not applicable
- Attempt contract: First corpus run retained; `runnerRetryCount = "0"`

**Acceptance Criteria:**

**Given** normally terminated, repeated, near-match, unrelated, and generic mid-line-marker inputs
**When** the closed corpus executes
**Then** every output byte remains unchanged except the one exact glued allowlisted case covered by Story 5.6.

**Given** a new notice candidate
**When** it is not a verbatim entry in the closed list
**Then** it receives no rewrite and cannot be matched by regex or heuristic.

**Given** the behavior-present check and corpus pass
**When** source-bound evidence is admitted
**Then** `D26-AC2` becomes only **eligible for later FULL reassessment**.

### Story 5.8: Revalidate Preview Bytes Against Spawned Bytes

> **MERGE** into Story 5.3. See `story-triage-2026-07-24.md`.

As a QA Lead,
I want historically FULL preview-to-spawn equality rerun through controlled native execution,
So that no command can diverge after the user reviews it.

**Story Contract:**

- Criteria and historical baseline: `F4-AC4` — `FULL`
- FR and requirement links: FR-8; NFR-1
- Primary readiness concern: Reusable Test Infrastructure
- Checkpoint: `RV@B5`
- Required test level: Unit plus real native Tauri E2E
- Execution lane / evidence depth: `forced-offline` / controlled environment-bound
- Dependencies: Accepted Epic 4; Stories 5.2–5.3; exact source association
- ASR and risk links: ASR-01, ASR-02, ASR-05, TIR-4/TIR-8, RE-10; R-002, R-003, R-004, R-008
- Behavior-present handling: Not `BP`; historical FULL is not carried forward
- Versioned scenario contract: `contracts/readiness/scenarios/v1/rv-b5-preview-spawn.json`
- Scenario-contract digest: Unassigned; freeze the exact file before implementation entry
- Expected evidence artifact: `rv-b5-preview-spawn.json` with reviewed argv bytes, fresh rebuild, capability validation, native spawn trace, and rejection cases
- Accountable role: QA
- Assignee: Unassigned
- Calendar date: Unassigned
- Implementation entry: Blocked by assignee/date, exact source association, and admission prerequisites
- Candidate subjects and invalidation: Not applicable
- Attempt contract: Retain ordinal 1 and all failure bytes; `runnerRetryCount = "0"`

**Acceptance Criteria:**

**Given** reviewed plans and changed/unchanged current state
**When** execution revalidates and reaches controlled native spawn
**Then** only a fresh exact plan spawns commands byte-identical to review
**And** stale, altered, replayed, evicted, or conflicting plans enqueue nothing.

**Given** the historical FULL status
**When** the current first attempt is admitted
**Then** no prior result is carried forward
**And** `F4-AC4` becomes only **eligible for later FULL reassessment**.

### Story 5.9: Revalidate Scheduler Locks, Parallelism, and Capacity

> **MERGE** into Story 5.3. See `story-triage-2026-07-24.md`.

As a QA Lead,
I want historical scheduler guarantees rerun against current source,
So that safe parallelism and the four-operation cap are not assumed from old tests.

**Story Contract:**

- Criteria and historical baseline: `F4-AC5` — `FULL`
- FR and requirement links: FR-9
- Primary readiness concern: Reusable Test Infrastructure
- Checkpoint: `RV@B5`
- Required test level: Unit with controlled scheduling
- Execution lane / evidence depth: `forced-offline` / source-bound
- Dependencies: ASR-02 controlled gates/time; Story 5.3; exact source association
- ASR and risk links: ASR-02, ASR-05, TIR-4/TIR-8, RE-10; R-004, R-008
- Behavior-present handling: Not `BP`; historical FULL receives no carry-forward
- Versioned scenario contract: `contracts/readiness/scenarios/v1/rv-b5-scheduler.json`
- Scenario-contract digest: Unassigned; freeze the exact file before implementation entry
- Expected evidence artifact: `rv-b5-scheduler.json` with atomic lock acquisition/release, conflict serialization, safe overlap, FIFO/aging, and concurrency-cap timelines
- Accountable role: QA
- Assignee: Unassigned
- Calendar date: Unassigned
- Implementation entry: Blocked by assignee/date, exact source association, and admission prerequisites
- Candidate subjects and invalidation: Not applicable
- Attempt contract: Complete first-attempt timeline retained; `runnerRetryCount = "0"`

**Acceptance Criteria:**

**Given** conflicting, independent, routed, shared-tree, skipped-ahead, and aged Operations
**When** the scheduler executes under controlled gates/time
**Then** complete lock sets acquire atomically, conflicts serialize, safe independent work overlaps, the aging guard prevents starvation, and global concurrency never exceeds four.

**Given** the historical FULL status
**When** the current source-bound attempt is admitted
**Then** the previous result is not carried forward
**And** `F4-AC5` becomes only **eligible for later FULL reassessment**.

---

## Epic 6: Preserve State, Evidence, and Privacy Across Failure and Relaunch — PARTIAL

This epic is still live in `epics.md`, retaining Story 6.5. Only the stories below were removed.

### Story 6.1: Deliver ASR-02 Filesystem and Native-Utility Extensions

> **RETIRE** — evidence ceremony, no product content. See `story-triage-2026-07-24.md`.

As a Development owner,
I want controlled filesystem, permission, path, Settings, diagnostics, and opener effects behind the accepted typed ports,
So that Batch 6 can exercise destructive and hostile lifecycle outcomes without touching operator data.

**Story Contract:**

- Criteria and historical baseline: None; the ASR-02 Batch 6 extension adds no denominator row
- FR and requirement links: No direct FR implementation; ASR-02/TIR-4 extension enabler for FR-15, FR-17, and FR-18
- Primary readiness concern: Reusable Test Infrastructure
- Checkpoint: ASR-02 filesystem/native-utility extension accepted before Batch 6
- Required test level: Unit, contract, negative-isolation, and controlled native qualification
- Execution lane / evidence depth: `forced-offline` / controlled environment-bound
- Dependencies: Accepted Epic 4 and Story 4.2 core; controlled-helper language decision; disposable-root design
- ASR and risk links: ASR-02 — Development accountable, Platform capability area; ASR-03 support boundary; ASR-05; R-004, R-005, R-008
- Behavior-present handling: Not applicable
- Versioned scenario contract: `contracts/readiness/scenarios/v1/asr-02-filesystem-native-utility-extension.json`
- Scenario-contract digest: Unassigned; freeze the exact file before implementation entry
- Expected evidence artifact: `asr-02-filesystem-native-utility-extension-qualification.json` with path/permission/symlink/atomic-write/opener/reveal/diagnostics control coverage and production-adapter exclusion
- Accountable role: Development
- Assignee: Unassigned
- Calendar date: Unassigned
- Implementation entry: Blocked by accepted ASR-02 core, controlled-helper decision, disposable-root design, and assignee/date
- Candidate subjects and invalidation: No candidate subjects; this forced-offline extension cannot substitute for later installed-candidate evidence
- Attempt contract: First extension qualification attempt retained; `runnerRetryCount = "0"`

**Acceptance Criteria:**

**Given** the accepted ASR-02 process-control core and the production filesystem/native-utility adapters
**When** the Batch 6 extension is composed
**Then** roots, atomic Settings and journal writes, path and permission outcomes, symlink metadata, diagnostics selection/streaming, opener/reveal, and related failure conditions cross typed ports
**And** production adapters remain fail-closed.

**Given** the non-distributable controlled composition
**When** success, missing, corrupt, partial-write, permission-denied, hostile-path, symlink-replacement, and opener outcomes are requested
**Then** each required result is deterministic, isolated under disposable roots, and observable without contacting operator files or processes
**And** no release bit, environment variable, hidden selector, or alternate business path can activate a controlled adapter.

**Given** the extension qualification result
**When** Batch 6 entry is evaluated
**Then** Development is the sole accountable ASR-02 role for the extension
**And** Batch 6 criterion work remains blocked until this extension and ASR-03 are separately accepted.

### Story 6.2: Deliver the Disposable Lifecycle Environment

> **RETIRE** — evidence ceremony, no product content. See `story-triage-2026-07-24.md`.

As a QA Lead,
I want native crash, relaunch, persistence, and hostile-filesystem scenarios isolated from operator data and processes,
So that lifecycle acceptance can execute safely and repeatably.

**Story Contract:**

- Criteria and historical baseline: None; ASR-03 enabler work adds no denominator row
- FR and requirement links: No direct FR implementation; ASR-03/TIR-5 lifecycle enabler for FR-15, FR-17, and FR-18
- Primary readiness concern: Reusable Test Infrastructure
- Checkpoint: Accepted before Batch 6 criterion work
- Required test level: Native lifecycle and negative-isolation qualification
- Execution lane / evidence depth: `forced-offline` / controlled environment-bound
- Dependencies: Story 6.1; accepted Epic 4; disposable root design; named assignee/date
- ASR and risk links: ASR-03 — QA accountable, Development/Platform support; ASR-02, ASR-05; R-004, R-005, R-008
- Behavior-present handling: Not applicable
- Versioned scenario contract: `contracts/readiness/scenarios/v1/asr-03-disposable-lifecycle.json`
- Scenario-contract digest: Unassigned; freeze the exact file before implementation entry
- Expected evidence artifact: `asr-03-disposable-lifecycle-qualification.json` with root inventory, launch/kill/relaunch traces, sentinel-process proof, and operator-isolation report
- Accountable role: QA
- Assignee: Unassigned
- Calendar date: Unassigned
- Implementation entry: Blocked by ASR-02 extensions, disposable-root design, and assignee/date
- Candidate subjects and invalidation: No candidate subjects; packaged lifecycle later uses external OS isolation without hidden release bits
- Attempt contract: First qualification attempt retained; automatic retries zero

**Acceptance Criteria:**

**Given** one native lifecycle scenario
**When** the controller constructs its environment
**Then** Application Support, Settings, journal, logs, transcripts, diagnostics destination, temporary files, and controlled executables resolve only under its disposable roots with no production-user fallback.

**Given** controlled running and historical process groups
**When** the controller launches, force-terminates, and relaunches the same native composition
**Then** it cleans up only processes it created, preserves disposable data between launches, and proves a live historical-PGID sentinel is not signaled.

**Given** deliberate operator-path or process contact
**When** isolation checks run
**Then** the scenario fails closed
**And** QA remains the sole accountable role for ASR-03.

### Story 6.3: Preserve Real Transcripts and Atomic Journals

> **MERGE** — owner recorded in the triage's overturn table.

As a Pack-Manager user,
I want every Operation transcript and journal transition to survive failure coherently and stay correlated to the plan attempt that admitted it,
So that I can reconstruct both Operation crash truth and its plan-attempt context without partial or corrupt durable state.

**Story Contract:**

- Criteria and historical baseline: `F8-AC1` — `UNIT-ONLY`; `F8-AC2` — `UNIT-ONLY`
- FR and requirement links: FR-15; TIR-5
- Primary readiness concern: Reusable Test Infrastructure
- Checkpoint: Batch 6
- Required test level: Real native Tauri E2E
- Execution lane / evidence depth: `forced-offline` / controlled environment-bound
- Dependencies: Stories 6.1–6.2; controlled Operation output
- ASR and risk links: ASR-02, ASR-03, ASR-05, TIR-4/TIR-5/TIR-8; R-004, R-005, R-008
- Behavior-present handling: Not `BP`
- Versioned scenario contract: `contracts/readiness/scenarios/v1/b6-transcript-journal.json`
- Scenario-contract digest: Unassigned; freeze the exact file before implementation entry
- Expected evidence artifact: `b6-transcript-journal.json` plus retained transcript/journal files, plan-attempt-correlated durable records (`planAttemptId`, immutable plan-admission metadata, persisted verification facts, and `retryOfPlanAttemptId` links), and byte/digest inventory
- Accountable role: QA
- Assignee: Unassigned
- Calendar date: Unassigned
- Implementation entry: Blocked by accepted ASR-03, ASR-02 filesystem controls, assignee/date, and admission prerequisites
- Candidate subjects and invalidation: Not applicable
- Attempt contract: Retain the first native lifecycle attempt and all partial/failure artifacts; `runnerRetryCount = "0"`

**Acceptance Criteria:**

**Given** controlled successful, failed, cancelled, timed-out, and abruptly terminated Operations
**When** real transcript creation, incremental line flush, and terminal footer behavior execute
**Then** canonical metadata — including the admitting plan attempt's `planAttemptId` — output, and available terminal state remain reconstructible
**And** transcript creation failure prevents spawn while later write failure is recorded without hanging work.

**Given** journal start/finish, cancel-before-start, corruption, rewrite failure, and compaction boundaries
**When** the native journal lifecycle executes
**Then** append/read/compaction remain atomic, corruption is handled explicitly, and every crash-journal start/finish record and compaction carries the admitting `planAttemptId` with immutable plan-admission metadata, persisted verification facts, and `retryOfPlanAttemptId` links, so durable records preserve both Operation crash truth and plan-attempt correlation
**And** newest-1,000 plan-attempt retention semantics are enforced without partial state and without prematurely deleting referenced Operation evidence.

**Given** the complete first attempt is admitted
**When** the two slots are evaluated
**Then** both criteria become only **eligible for later FULL reassessment**.

### Story 6.4: Reconstruct Interrupted Work Without Signaling History

> **MERGE** — owner recorded in the triage's overturn table.

As a Pack-Manager user,
I want crashes and relaunches to show Interrupted work and preserve searchable History safely,
So that I can diagnose failures without endangering a reused process identifier.

**Story Contract:**

- Criteria and historical baseline: `F8-AC3` — `PARTIAL`; `F8-AC4` — `PARTIAL`
- FR and requirement links: FR-15; TIR-5
- Primary readiness concern: Reusable Test Infrastructure
- Checkpoint: Batch 6
- Required test level: Native crash/relaunch E2E
- Execution lane / evidence depth: `forced-offline` / controlled environment-bound
- Dependencies: Stories 6.1–6.3; historical-PGID sentinel; controlled Finder opener
- ASR and risk links: ASR-02, ASR-03, ASR-05, TIR-5/TIR-8; R-004, R-005, R-008
- Behavior-present handling: Not `BP`
- Versioned scenario contract: `contracts/readiness/scenarios/v1/b6-interrupted-history.json`
- Scenario-contract digest: Unassigned; freeze the exact file before implementation entry
- Expected evidence artifact: `b6-interrupted-history.json` with crash/relaunch traces, retained data, sentinel observations, plan-attempt History rows with nested Operation evidence, read-only Activity replay reconstruction, and native Reveal success/failure
- Accountable role: QA
- Assignee: Unassigned
- Calendar date: Unassigned
- Implementation entry: Blocked by accepted ASR-03, assignee/date, and admission prerequisites
- Candidate subjects and invalidation: Not applicable
- Attempt contract: First crash/relaunch attempt retained; `runnerRetryCount = "0"`

**Acceptance Criteria:**

**Given** an Operation with a durable start and no finish
**When** the native app is forcibly terminated and relaunched against the same disposable roots
**Then** the Operation appears as Interrupted with retained transcript/history context
**And** a live sentinel at the recorded historical process identifier is never signaled.

**Given** durable and current-session confirmed plan attempts, each carrying a `planAttemptId` and nested Operation, verification-refresh, reviewed-command, and result evidence
**When** the user filters and searches History, opens a plan attempt, and enters its read-only Activity replay
**Then** History shows exactly one immutable row per confirmed `planAttemptId` — linking its Operations, verification refreshes, reviewed commands, results, and optional `retryOfPlanAttemptId` — and the replay reconstructs Manager groups, Package/version changes, commands, outcomes, timing, errors, and retained output
**And** legacy Operation records without a plan-attempt identity stay clearly labeled legacy entries and are never fabricated into a grouping, while native Reveal success/failure remains visible without dangling actions.

**Given** the first lifecycle attempt passes
**When** admission evaluates it
**Then** both criteria become only **eligible for later FULL reassessment**.

### Story 6.6: Reject Hostile or Private Diagnostic Inputs

> **RETIRE** — evidence ceremony, no product content. See `story-triage-2026-07-24.md`.

As a Pack-Manager user,
I want diagnostics to exclude inherited values and reject hostile paths,
So that exporting support evidence cannot disclose or follow unintended local data.

**Story Contract:**

- Criteria and historical baseline: `F9-AC3` — `UNIT-ONLY`
- FR and requirement links: FR-18; NFR-5; TIR-2; TIR-5
- Primary readiness concern: Reusable Test Infrastructure
- Checkpoint: Batch 6
- Required test level: Unit/contract
- Execution lane / evidence depth: `forced-offline` / source-bound
- Dependencies: Story 6.5; exhaustive hostile-environment and filesystem fixture set
- ASR and risk links: ASR-02, ASR-03, ASR-05, TIR-1/TIR-2/TIR-5/TIR-8; R-005, R-008
- Behavior-present handling: `BP`; missing/incorrect privacy behavior creates Product Behavior work before regression credit
- Versioned scenario contract: `contracts/readiness/scenarios/v1/b6-diagnostics-hostile-inputs.json`
- Scenario-contract digest: Unassigned; freeze the exact file before implementation entry
- Expected evidence artifact: `b6-diagnostics-hostile-inputs.json` with inherited/constructed environment comparison and hostile byte/path/symlink results
- Accountable role: QA
- Assignee: Unassigned
- Calendar date: Unassigned
- Implementation entry: Blocked by complete fixture corpus, assignee/date, and admission prerequisites
- Candidate subjects and invalidation: Not applicable
- Attempt contract: First hostile-input matrix retained; `runnerRetryCount = "0"`

**Acceptance Criteria:**

**Given** secret-like inherited values and Pack-Manager's explicit constructed environment
**When** report/log/diagnostic content is built
**Then** only explicitly constructed values are eligible and inherited values never appear.

**Given** hostile bytes, absolute/traversal/backslash paths, symlinks introduced before selection or before streaming, and replaced sources
**When** diagnostics selection and streaming execute
**Then** every unsafe input is rejected or safely omitted with a visible result and no external content is followed.

**Given** the behavior-present matrix passes completely
**When** the source-bound result is admitted
**Then** `F9-AC3` becomes only **eligible for later FULL reassessment**.

### Story 6.7: Preserve Settings and Native Utility Actions Across Failure

> **MERGE** — owner recorded in the triage's overturn table.

As a Pack-Manager user,
I want Settings persistence and native utility actions to survive missing, corrupt, partial, and failed state,
So that configuration and recovery controls remain trustworthy after relaunch.

**Story Contract:**

- Criteria and historical baseline: `F11-AC1` — `UNIT-ONLY`; `F11-AC4` — `PARTIAL`
- FR and requirement links: FR-17; FR-18; TIR-3; TIR-5
- Primary readiness concern: Reusable Test Infrastructure
- Checkpoint: Batch 6
- Required test level: Real native Tauri E2E
- Execution lane / evidence depth: `forced-offline` / controlled environment-bound
- Dependencies: Stories 6.1–6.2; Epic 3 control behavior
- ASR and risk links: ASR-02, ASR-03, ASR-05, TIR-3/TIR-5/TIR-8; R-003, R-005, R-008
- Behavior-present handling: Not `BP`
- Versioned scenario contract: `contracts/readiness/scenarios/v1/b6-settings-native-actions.json`
- Scenario-contract digest: Unassigned; freeze the exact file before implementation entry
- Expected evidence artifact: `b6-settings-native-actions.json` with persisted bytes, before/after active values, relaunch outcomes, and Re-detect/export/Open Logs command results
- Accountable role: QA
- Assignee: Unassigned
- Calendar date: Unassigned
- Implementation entry: Blocked by ASR-02/03 controls, assignee/date, and admission prerequisites
- Candidate subjects and invalidation: Not applicable
- Attempt contract: Preserve first persistence/action attempt and all failure artifacts; `runnerRetryCount = "0"`

**Acceptance Criteria:**

**Given** missing, corrupt, partial, valid, and write-failing Settings files
**When** the native app loads, changes, persists, and relaunches
**Then** defaults and valid values behave as specified, writes are atomic, and a failed save changes neither active nor persisted state.

**Given** a persisted legacy `autoOpenDrawer` value, an absent `skipUpgradePlanConfirmation` key, and the retained editable stall threshold, hard cap, and log level
**When** the native app migrates Settings, sets and reverses the confirmation preference, and edits each retained control
**Then** the Activity auto-open preference is absent from active Settings while any old persisted `autoOpenDrawer` value is tolerated without becoming active, `skipUpgradePlanConfirmation` defaults to `false`, persists atomically, and is reversible, the stall threshold, hard cap, and log level stay editable, and every control shows `Saving`, activates the new value only after atomic persistence succeeds, then shows `Saved`, with a failed write retaining or restoring the prior value behind a visible failure state.

**Given** Re-detect, diagnostics export, and Open Logs actions
**When** their native success and failure outcomes execute
**Then** the real commands cross production Tauri and visible UI state matches the native result.

**Given** the complete first attempt is admitted
**When** both slots are evaluated
**Then** `F11-AC1` and `F11-AC4` become only **eligible for later FULL reassessment**.

---
