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
  - "_bmad-output/test-artifacts/test-design/Pack-Manager-handoff.md"
  - "_bmad-output/planning-artifacts/ux-designs/ux-Pack-Manager-2026-07-23/EXPERIENCE.md"
  - "_bmad-output/planning-artifacts/sprint-change-proposal-2026-07-24.md"
---

# Test Design Workflow Progress

## Rerun Context

- On 2026-07-24, Correct Course approved a finalized UX contract that
  supersedes immediate row execution, direct Manager-update execution,
  drawer-only Activity, Operation-row History, and active `autoOpenDrawer`
  behavior.
- The Product Behavior Prerequisite UX-PB.1..UX-PB.5 and AD-16 now precede
  affected evidence work. This progress file records planning reconciliation
  only; implementation and behavior-present reclassification remain pending.
- `AUT-003` is retained as historical proof of superseded behavior and cannot
  support revised `F5-AC3`.

- The completed 2026-07-22 Test Design run predates the finalized PRD and
  formal architecture spine.
- Its four outputs are prior planning inputs only. This run regenerates those
  paths in place and does not carry forward a risk, classification, batch,
  dependency, estimate, evidence rule, or closure claim without current
  support.
- The readiness coverage map remains `final-pending-approval`.
- The traceability matrix supplies only the planning baseline: FAIL with
  14/72 P0 criteria FULL and 58 non-FULL. It is not newer authority and is not
  candidate evidence.

## Step 1 — Mode and Prerequisites

- **Mode:** System-Level Create.
- **Reason:** The request reconciles the product-wide PRD, architecture, all 72
  P0 criteria, five cross-cutting ASRs, eight closure batches, and release
  prerequisites. It is not scoped to an epic or story.
- **PRD prerequisites confirmed:**
  `_bmad-output/planning-artifacts/prds/prd-Pack-Manager-2026-07-22/prd.md`
  and `addendum.md`.
- **ADR/decision prerequisites confirmed:** `docs/DECISIONS.md`.
- **Architecture prerequisite confirmed:**
  `_bmad-output/planning-artifacts/architecture/architecture-Pack-Manager-2026-07-23/ARCHITECTURE-SPINE.md`
  with `status: final`.
- **Additional authorities confirmed:** the normative 72-row coverage map,
  `docs/SPEC.md`, Decisions D27-D30, the finalized UX spines, the approved
  Correct Course proposal, and the finalized project context.
- **Existing-plan inputs confirmed:** the prior Architecture, QA, progress, and
  handoff Test Design artifacts are present for reconciliation.
- **Scope boundary:** planning only. Do not implement or modify product
  behavior, tests, reusable infrastructure, CI, release workflows,
  configuration, or the traceability gate.
- **Readiness boundary:** do not promote a criterion, regenerate the gate, or
  claim release readiness. The 14 baseline-FULL rows require candidate-era
  revalidation under the finalized evidence-depth rules.

## Step 2 — Loaded Context

- **Authoritative requirements loaded:** finalized PRD and addendum, the
  `final-pending-approval` 72-row coverage map, `docs/SPEC.md`, and
  `docs/DECISIONS.md`.
- **Formal architecture loaded:** the final Architecture Spine, including
  AD-1 through AD-16, ASR-01 through ASR-05, the three non-substitutable
  evidence lanes, Candidate Identity Manifest, criterion Acceptance Profiles,
  append-only Evidence Index, RFC 8785 JCS plus SHA-256 identity rules,
  first-failure retention, append authority, and candidate invalidation.
- **Baseline evidence loaded:** the 2026-07-22 traceability matrix solely for
  the FAIL / 14-of-72-FULL planning snapshot. Its older oracle and policy
  conclusions do not override the finalized PRD or Architecture Spine.
- **Prior plan loaded:** the previous Architecture, QA, progress, and handoff
  outputs are reconciliation inputs only. No prior risk, classification,
  dependency, estimate, or evidence rule carries forward without current
  support.
- **Detected stack:** full-stack local macOS Tauri application: React,
  TypeScript, Vite, Vitest, Playwright, Tauri 2, and Rust/Cargo. The product
  boundary is local Tauri IPC, not an HTTP service API.
- **Current boundary baseline:** 20 registered production commands and six
  production events. These counts and current Operation-only models are a
  verified baseline, not permanent invariants; Plan Intent/Attempt,
  registration, wire-contract, persistence, and native-acceptance surfaces
  must change together.
- **Knowledge loaded:** system ADR-readiness, NFR, test-level, risk-governance,
  probability/impact, priority, and test-quality guidance; the full
  UI-plus-API Playwright Utils profile; and Playwright CLI guidance.
  Pact/Pact MCP were not loaded because they are disabled and no microservice
  contract exists.
- **Browser exploration:** intentionally skipped in System-Level mode. This
  run designs future evidence and does not collect or manufacture new
  evidence.
- **Known decision constraints:** DR-1 remains OPEN; DR-2 and DR-3 are
  APPROVED; DR-4 remains PROPOSED. The coverage map remains pending approval.
  No generic P1 policy substitutes for DR-4.
- **No missing planning authority:** all requested source artifacts are
  present. Remaining uncertainties are explicit implementation-handoff
  blockers, including DR-1, DR-4, map approval, named assignees and calendar
  dates, evidence retention/transport, provisioned Macs, release credentials,
  and immutable-candidate availability.

## Step 3 — Testability and Risk Assessment

### 🚨 Testability Concerns

All five formal ASRs are **ACTIONABLE**. The Architecture Spine specifies
their contracts, accountable roles, delivery boundaries, and acceptance
boundaries; this Test Design does not treat any enabler as implemented.

| ASR                                               | Status     | Accountable role | Testability concern                                                                                                                                                                                                                                                | Delivery and acceptance boundary                                                                                                                                                                                                                                           |
| ------------------------------------------------- | ---------- | ---------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| ASR-01 — shared production command/event boundary | ACTIONABLE | Architecture     | Separate browser/fake-bridge and Rust tests cannot prove real frontend invocation, Tauri registration/serialization, handler execution, or event dispatch. The current 20-command/six-event set is only the verified baseline.                                     | Accepted by Batch 4 exit. The versioned catalog, production registration, wire contracts, wrappers/subscriptions, and native acceptance inventory must have set equality; every catalog command round-trips and every event dispatches through the isolated real boundary. |
| ASR-02 — deterministic process and OS controls    | ACTIONABLE | Development      | Safe native failure injection needs typed control of output, exits, cancellation/escalation, time, locks, null stdin, paths, permissions, open/reveal, restart, and updater effects without creating a production shell surface.                                   | Core controls accepted before Batch 5; filesystem/updater extensions before Batches 6–7. Production adapters remain fail-closed.                                                                                                                                           |
| ASR-03 — disposable lifecycle environment         | ACTIONABLE | QA               | Crash, forced quit, relaunch, durable state, hostile filesystem, retention, and historical-PGID non-signal cannot use operator data or processes.                                                                                                                  | Accepted before Batch 6 using disposable roots, controlled children, and retained lifecycle evidence; Development/Platform supports delivery.                                                                                                                              |
| ASR-04 — candidate identity and attestation       | ACTIONABLE | Release          | A plan or ad hoc artifact list cannot produce reproducible candidate identity or an auditable result history. The strict v1 schemas, RFC 8785 JCS bytes, SHA-256 boundaries, hash chain, single-head append, and invalidation rules need an operational Registrar. | Contract accepted before release preparation; manifest frozen before Batch 7; complete ledger accepted in Batch 8. Only the protected Release-owned Registrar may append.                                                                                                  |
| ASR-05 — three split evidence lanes               | ACTIONABLE | QA               | Forced-offline, provisioned target-Mac, and candidate-release evidence have different dependencies and proof depth. Sharing workspaces, credentials, or results allows invalid substitution.                                                                       | Lane isolation accepted before any Batch 1 evidence collection; candidate lane operational before Batch 7. CI is the execution mechanism, not a co-owner.                                                                                                                  |

Additional actionable blockers:

1. The normative coverage map is still `final-pending-approval`. Its 72 rows
   can be planned, but the Criterion Acceptance Profile cannot freeze until
   the Product/QA approval record exists.
2. DR-1 is OPEN. Product and Release must declare the minimum supported macOS
   version before TIR-7 or RE-4/RE-7/RE-8 environment implementation handoff;
   packaged compatibility slots and the Acceptance Profile remain blocked.
3. DR-4 is PROPOSED. Product/QA must approve the P0-specific gate and retry
   policy before profile freeze or candidate validation. No legacy or generic
   P1 threshold is applied here.
4. Evidence transport and retention duration remain a Release-owned handoff
   blocker. The choice must preserve the protected Registrar, lock/CAS,
   idempotency, write-once objects, a single index head, first failures, and
   full audit availability inside the existing GitHub framework.
5. Named individuals and calendar dates remain unassigned. Role ownership and
   batch-relative deadlines are binding, but each downstream work item is
   blocked from implementation entry until one person and a date are assigned.
6. Provisioned target-Mac access, Apple-silicon and physical Intel hosts,
   installed prior public version, signing/notarization credentials, and one
   immutable candidate are execution dependencies, not assumed evidence.

### ✅ Testability Assessment Summary

| Dimension                       | Architectural strength                                                                                                                                                                                                                                                     | Remaining proof boundary                                                                                                                                                                     |
| ------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Controllability                 | AD-2 through AD-5 define one production composition root, controlled non-distributable adapters, typed OS/process ports, disposable lifecycle roots, and fail-closed production defaults.                                                                                  | Implementation must demonstrate that test adapters cannot enter the distributable graph and that every material effect required by ASR-02/03 is controllable.                                |
| Observability                   | Product Operation IDs already correlate statuses, output, transcripts, journals, History, and diagnostics. AD-7 through AD-10 add exact candidate/profile/record digests, source/environment/candidate provenance, subject/result separation, and first-failure retention. | The Registrar, immutable objects, append chain, human/machine agreement, and complete result artifacts require executable contract validation; prose or workflow inspection is not evidence. |
| Reliability                     | AD-6 isolates three lanes; AD-8 serializes appends; AD-9 invalidates evidence on candidate mutation; AD-15 freezes each slot, retry rule, environment, subject, and evidence depth.                                                                                        | Parallel safety, stale/forked head rejection, retry-chain handling, lane substitution rejection, and candidate invalidation must be exercised before aggregation can be trusted.             |
| Existing lower-level foundation | Pure Rust logic, controlled runners/event sinks, fake IPC, temporary filesystems, real-capture fixtures, Vitest/component tests, and browser tests remain useful reusable foundations.                                                                                     | They retain their actual proof depth only. They do not cross the real Tauri boundary, prove a provisioned host, or attest a packaged candidate.                                              |

**FYI, not additional ASRs:** the production behavioral invariants in
`docs/SPEC.md`/`docs/DECISIONS.md`, and the existing lower-level seams above,
remain important inputs. They do not create ASR-06 or ASR-07; the formal
architecture has exactly ASR-01 through ASR-05.

### Reconciled Risk Register

The finalized PRD explicitly retains R-001 through R-008. Reassessment against
AD-1 through AD-15 supports all eight, but updates their mitigations and
ownership boundaries. Probability and impact use the BMAD 1–3 scale.

| ID    | Category   | Risk                                                                                                                             |   P |   I | Score | Required mitigation and planned evidence                                                                                                                                                    | Accountable role | Deadline                                                             |
| ----- | ---------- | -------------------------------------------------------------------------------------------------------------------------------- | --: | --: | ----: | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ---------------- | -------------------------------------------------------------------- |
| R-001 | OPS / DATA | Source, real-capture, and provisioned-host truth can drift, causing tests to prove an obsolete `mas` oracle.                     |   3 |   2 |     6 | Complete PC-1 before recurrence credit; preserve capture provenance; run serialized, dated topology evidence; reject unreported host drift.                                                 | Product          | Batch 1 exit                                                         |
| R-002 | TECH       | Frontend and Rust suites can pass while the shared production command/event boundary is broken or divergent.                     |   2 |   3 |     6 | Deliver ASR-01 and AD-3 set-equality checks; round-trip every catalog command and dispatch every catalog event through real Tauri with controlled state.                                    | Architecture     | Batch 4 exit                                                         |
| R-003 | BUS / TECH | Stale, failed, late, inaccessible, or misleading UI state can authorize or communicate the wrong action.                         |   2 |   3 |     6 | Begin BP rows with behavior-present checks; cover coherent state, error outcomes, keyboard/focus/status semantics, stale continuation rejection, and packaged accessibility where required. | Development      | Batches 2–3 for deterministic state; Batch 7 for packaged acceptance |
| R-004 | SEC / OPS  | Process output, locks, cancellation, timeout, shutdown, null stdin, or PID reuse can produce unsafe or dishonest terminal state. |   2 |   3 |     6 | Deliver ASR-02/03 controls; correlate native event, child, lock, journal, transcript, cleanup, and historical-PGID non-signal evidence.                                                     | Development      | Batch 5 core; Batch 6 lifecycle exit                                 |
| R-005 | DATA / SEC | Persistence, History, retention, or diagnostics can lose evidence or expose/follow hostile local data.                           |   2 |   3 |     6 | Use disposable app data; prove atomicity, corruption handling, retention, exact archive contents, no inherited values, and symlink/path rejection.                                          | QA               | Batch 6 exit                                                         |
| R-006 | SEC / OPS  | Updater metadata, signature, installed bytes, safety guards, or relaunched version can diverge while seam tests remain green.    |   2 |   3 |     6 | Bind installed-prior-version check/download/verify/refuse/install/relaunch and non-writable behavior to one Candidate Manifest and profile slot set.                                        | Release          | Batch 7 exit; ledger completion in Batch 8                           |
| R-007 | OPS / SEC  | A published set can be incomplete, inconsistent, unsigned, non-universal, unnotarized, unstapled, inaccessible, or unlaunchable. |   3 |   3 |     9 | Deliver ASR-04; inspect exact candidate artifacts; prove trust and launch on Apple silicon and physical Intel; replay the complete Evidence Index before decision.                          | Release          | Batch 8 exit                                                         |
| R-008 | TECH / OPS | Results can silently depend on network, mutable machine state, credentials, ignored tests, retries, or a different candidate.    |   2 |   3 |     6 | Deliver ASR-05; bind provenance depth; deny cross-lane substitution; exclude ignored/unexecuted checks; retain first failure; reject candidate/profile mismatch.                            | QA               | Lane contract before Batch 1; candidate lane before Batch 7          |

All eight scores remain high (six or nine) and require mitigation. R-007
remains the automatic critical blocker at score nine. No risk is marked
mitigated, waived, or accepted by this planning run.

### NFR Planning Assessment

| NFR area                             | Binding threshold or rule                                                                                                                                                                                                                                                               | Planned evidence source and level                                                                                                                              | Open question / risk linkage                                                                                        |
| ------------------------------------ | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------- |
| Security and privacy                 | No sudo/password or generic shell surface; null stdin; absolute executable and structured argv; constructed environment; no inherited environment disclosure; no diagnostic symlink substitution; signed updater; no OS authorization prompt.                                           | Forced-offline unit/component plus ASR-01/02 native process traces, hostile archive inspection, and candidate-bound updater/trust records.                     | Evidence producer and candidate credentials must remain separated; R-004 through R-008.                             |
| Performance and bounded presentation | Login-shell probe 5 s/64 KiB; version probe 10 s; output flush at 50 ms, 64 lines, or 8 KiB; 512 KiB/stream; newest 5,000 live lines; cancel grace 5 s; stall default 120 s; hard cap 30 min; 101-row reachability; 900×600 minimum layout.                                             | Paused-time deterministic checks, controlled native large-output/timing runs, and packaged WKWebView 101-row/5,001-line/minimum-window scenarios.              | No additional percentile, CPU, RSS, updater-duration, or soak target is authorized; do not invent one. R-003/R-004. |
| Reliability and recovery             | One Manager failure cannot blank peers/Last-good Snapshot; journal/transcript durability; Interrupted recovery; historical PGIDs never signaled; atomic settings; explicit crash/cancel/timeout/persistence outcomes.                                                                   | Failure injection, ASR-03 crash/forced-quit/relaunch runs, journal/transcript inspection, and updater relaunch evidence.                                       | Requires disposable lifecycle and prior-version environment. R-004/R-005/R-006.                                     |
| Capacity / scalability               | Local single-user desktop; horizontal scaling is not applicable. Binding caps include 2,048 selections, 512-byte IDs, 64 plans, four concurrent operations, 120 s aging guard, >100-row virtualization, 1,000 journal records, 14-day app logs, and 200-or-90-day transcript retention. | Boundary/property tests, native compaction/retention artifacts, packaged large-state runs.                                                                     | Evidence-ledger retention duration is still a separate Release blocker. R-005/R-008.                                |
| Maintainability and reproducibility  | Required frontend/Rust formatting, static checks, contracts, builds, and tests from a clean checkout with outbound network denied; ignored and unexecuted tests excluded; no automatic P0 retry; first failure retained.                                                                | Source-bound forced-offline lane, complete first-run reports, dependency/process/network isolation audit, and later candidate-bound trace regeneration.        | No generic coverage/duplication threshold is adopted; DR-4 governs the P0-specific gate. R-002/R-008.               |
| Accessibility                        | **DR-2 APPROVED:** packaged keyboard/focus, automated 4.5:1 text contrast, reduced motion, and manual VoiceOver focus-order/completion-announcement pass.                                                                                                                               | Candidate-release packaged WKWebView evidence with automated results plus manual VoiceOver record.                                                             | Approval fixes the method, not a passing result. R-003/R-007.                                                       |
| macOS compatibility                  | Normal GUI launch; both promised architectures; visible local parser incompatibility; **DR-3 APPROVED:** physical Intel fresh-install/Finder/Dock and prior-version update, in addition to Apple silicon.                                                                               | Provisioned environment profiles plus candidate-bound install/launch/update evidence on Apple silicon and physical Intel.                                      | **DR-1 OPEN:** minimum supported macOS blocks environment matrix/profile freeze. R-001/R-007/R-008.                 |
| Release/update integrity             | One clean immutable candidate; five fixed artifact subjects; coherent versions; Developer ID, notarization, stapling, Gatekeeper; reachable HTTPS metadata; valid updater signature; explicit restart; active-operation refusal; mutation invalidates downstream evidence.              | ASR-04 schema/JCS/vector/hash-chain validation, artifact inspection, candidate update/install runs, append-only ledger replay, and final trace/gate artifacts. | DR-4 and map approval block Acceptance Profile freeze. R-006/R-007/R-008.                                           |

This is an NFR validation plan only. It assigns no final NFR
PASS/CONCERNS/FAIL status and does not claim that any named evidence exists.

### Highest-Risk Mitigation Priority

1. Freeze approved planning inputs and lane contracts: map approval, DR-4,
   ASR-05, and downstream individual/date assignment.
2. Restore source/target truth in Batch 1 while the production/native
   boundary contract is delivered through Batch 4.
3. Use ASR-02 and ASR-03 to close process, lifecycle, persistence, and
   diagnostics uncertainty in Batches 5–6.
4. Resolve DR-1, provision both physical architectures, freeze the
   Candidate Manifest/Profile, and execute Batch 7 against the unchanged
   signed/notarized/stapled candidate.
5. Append and replay Batch 8 evidence through the Release-owned Registrar.
   Only a later Trace workflow may recompute readiness.

## Step 4 — Coverage Plan and Execution Strategy

### Reconciliation Controls

- The planning baseline remains **FAIL with 14/72 FULL**. This run neither
  executes evidence nor changes a status.
- The 58 non-FULL rows retain map revision 1's provisional primary split:
  1 Product Behavior, 52 Reusable Test Infrastructure, and 5
  Candidate-Specific Release Evidence.
- The 24 `BP` rows retain their provisional Test Infrastructure classification
  only until their behavior-present check. Missing or incorrect behavior moves
  the work to Product Behavior through a reviewed map revision; test work
  cannot conceal that correction.
- The 14 baseline-FULL rows remain outside the 58-row lane and batch totals.
  Each receives a revalidation checkpoint (`RV@B#`) without receiving a new
  normative closure batch.
- The later Criterion Acceptance Profile may require more than one evidence
  slot for a criterion. This table still maps the criterion exactly once to
  its primary planning concern, execution lane, and minimum binding depth.
  Secondary evidence cannot substitute for the primary slot.

### Common Admission Dependencies

These dependencies apply to every row below:

1. ASR-05 lane separation is accepted by QA before evidence collection.
2. The coverage map is approved, DR-1 is resolved, and DR-4 is approved before
   the Criterion Acceptance Profile freezes. Until then, the mappings below
   are authoritative planning recommendations, not admitted evidence slots.
3. ASR-04 supplies the locked contract, validated profile, sole-append
   Registrar, immutable attempt artifacts, exact provenance, first-failure
   retention, and valid hash-chain append.
4. Automatic retries are zero. Ignored, skipped, collected-only, filtered,
   wrong-lane, shallower-depth, wrong-source, or wrong-candidate results cannot
   satisfy a slot.
5. `C` means a non-FULL row becomes **eligible for later FULL reassessment**
   only after every frozen-profile slot passes, the Evidence Index validates,
   and a separate Trace workflow regenerates the candidate-bound decision.
   `RV` means historical baseline FULL requires the same revalidation and is
   not carried forward.

### 72-Criterion Primary Planning Matrix

Legend:

- Concern: `PB` Product Behavior; `TI` Reusable Test Infrastructure; `RE`
  Candidate-Specific Release Evidence; `TI-RV` historical-FULL revalidation
  outside the open-row split.
- Lane: `FO` forced-offline; `TM` provisioned-target-Mac; `CR`
  candidate-release.
- Depth: `S` source-bound; `E` environment-bound; `C` candidate-bound.
- Level: `U` unit/contract; `C` component; `B` browser E2E; `N` real native
  Tauri E2E; `T` live target-Mac; `P` installed packaged-app acceptance; `A`
  artifact/release attestation; `CI` clean-checkout quality run.

| Criterion  | Baseline         | Concern | Plan point | Required level       | Primary lane | Depth | Specific dependencies                                           | Result |
| ---------- | ---------------- | ------- | ---------- | -------------------- | ------------ | ----- | --------------------------------------------------------------- | ------ |
| `F1-AC1`   | PARTIAL          | TI      | B4         | N                    | FO           | E     | B1; ASR-01 catalog/composition; startup order                   | C      |
| `F1-AC2`   | PARTIAL          | TI      | B4         | N                    | FO           | E     | B1; ASR-01; every required UI entry                             | C      |
| `F1-AC3`   | UNIT-ONLY        | TI      | B4         | N                    | FO           | E     | B1; ASR-01/02; PATH, timeout, cleanup                           | C      |
| `F1-AC4`   | UNIT-ONLY        | TI      | B4         | N                    | FO           | E     | B1; ASR-01/02; success, timeout, missing version                | C      |
| `F1-AC5`   | FULL             | TI-RV   | RV@B2      | U+C                  | FO           | S     | Exact candidate source commit; raw-before-canonical evidence    | RV     |
| `F1-AC6`   | PARTIAL          | TI      | B2         | C                    | FO           | S     | B1; BP; install-hint copy success/failure                       | C      |
| `F1-AC7`   | NONE             | TI      | B1         | T                    | TM           | E     | BP; PC-1; dated six-Manager topology                            | C      |
| `F1-AC8`   | PARTIAL          | TI      | B2         | C                    | FO           | S     | B1; BP; all six Managers; clipboard outcomes                    | C      |
| `F2-AC1`   | UNIT-ONLY        | TI      | B4         | N                    | FO           | E     | B1; ASR-01/02; all-six command order                            | C      |
| `F2-AC2`   | FULL             | TI-RV   | RV@B2      | U                    | FO           | S     | Exact candidate source commit; merge/overlay corpus             | RV     |
| `F2-AC3`   | UNIT-ONLY        | TI      | B2         | U+C                  | FO           | S     | B1; BP; enabled/disabled phase order                            | C      |
| `F2-AC4`   | FULL             | TI-RV   | RV@B4      | U+C+N                | FO           | E     | B1; ASR-01 all-six crossing; fresh detection                    | RV     |
| `F2-AC5`   | FULL             | TI-RV   | RV@B2      | C                    | FO           | S     | B1; populated/loading coexistence                               | RV     |
| `F2-AC6`   | PARTIAL          | TI      | B2         | U                    | FO           | S     | B1; BP; every adapter; no live network                          | C      |
| `F2-AC7`   | FULL             | TI-RV   | RV@B2      | U+C                  | FO           | S     | B1; stale retention, Retry, peer isolation                      | RV     |
| `F2-AC8`   | PARTIAL          | TI      | B2         | U                    | FO           | S     | B1; BP; every network adapter; peer isolation                   | C      |
| `F2-AC9`   | PARTIAL          | TI      | B5         | U                    | FO           | S     | B4; BP; subject/executor dual refresh                           | C      |
| `F3-AC1`   | PARTIAL          | TI      | B3         | C                    | FO           | S     | B1; BP; complete representative table                           | C      |
| `F3-AC2`   | PARTIAL          | TI      | B3         | C                    | FO           | S     | B1; BP; expand/collapse/search details                          | C      |
| `F3-AC3`   | PARTIAL          | TI      | B3         | U+C                  | FO           | S     | B1; BP; every plan path excludes pinned                         | C      |
| `F3-AC4`   | PARTIAL          | TI      | B3         | U+C                  | FO           | S     | B1; BP; set difference/default/opt-in                           | C      |
| `F3-AC5`   | FULL             | TI-RV   | RV@B3      | U+C                  | FO           | S     | Exact candidate source; verbatim non-semver values              | RV     |
| `F3-AC6`   | PARTIAL          | TI      | B3         | C                    | FO           | S     | B1; BP; npm table and self card together                        | C      |
| `F3-AC7`   | FULL             | TI-RV   | RV@B3      | U+C                  | FO           | S     | Exact candidate source and mise fixture consequences            | RV     |
| `F3-AC8`   | PARTIAL          | TI      | B3         | C                    | FO           | S     | B1; BP; non-color status semantics                              | C      |
| `D23a-AC1` | PARTIAL          | TI      | B1         | T                    | TM           | E     | BP; PC-1; installed/live-verified `mas`                         | C      |
| `D23a-AC2` | PARTIAL          | TI      | B1         | T+U                  | TM           | E     | BP; immutable real capture; deterministic replay is secondary   | C      |
| `D23a-AC3` | FULL             | TI-RV   | RV@B1      | U                    | FO           | S     | B1 real-capture provenance; no synthetic substitution           | RV     |
| `D23a-AC4` | NONE             | PB      | B1         | Product correction+U | FO           | S     | PC-1 correction before recurrence-guard credit                  | C      |
| `D23a-AC5` | UNIT-ONLY        | TI      | B1         | U                    | FO           | S     | BP; provenance; synthetic means robustness only                 | C      |
| `F4-AC1`   | PARTIAL          | TI      | B3         | U+C                  | FO           | S     | B1; BP; both entry routes and visible failure                   | C      |
| `F4-AC2`   | FULL             | TI-RV   | RV@B4      | U+C+B+N              | FO           | E     | ASR-01; reviewed bytes and native serialization                 | RV     |
| `F4-AC3`   | FULL             | TI-RV   | RV@B3      | U+C                  | FO           | S     | Defaults, rebuild, exclusions, current state                    | RV     |
| `F4-AC4`   | FULL             | TI-RV   | RV@B5      | U+N                  | FO           | E     | B4; ASR-02; byte-identical preview to spawn                     | RV     |
| `F4-AC5`   | FULL             | TI-RV   | RV@B5      | U                    | FO           | S     | Complete locks, safe parallelism, cap four                      | RV     |
| `F5-AC1`   | PARTIAL          | TI      | B3         | C+B                  | FO           | S     | B1; BP; filter-aware macOS keyboard behavior                    | C      |
| `F5-AC2`   | FULL             | TI-RV   | RV@B4      | C+B+N                | FO           | E     | ASR-01; exact PackageRefs and admission semantics               | RV     |
| `F5-AC3`   | PARTIAL          | TI      | B3         | C+B                  | FO           | S     | B1; BP; one-package path and both rejections                    | C      |
| `F6-AC1`   | PARTIAL          | TI      | B5         | N                    | FO           | E     | B4; ASR-02 controlled executables                               | C      |
| `F6-AC2`   | UNIT-ONLY        | TI      | B5         | N                    | FO           | E     | B4; real IPC serialization                                      | C      |
| `F6-AC3`   | INTEGRATION-ONLY | TI      | B5         | N                    | FO           | E     | B4; production event dispatcher                                 | C      |
| `F6-AC4`   | UNIT-ONLY        | TI      | B5         | N                    | FO           | E     | B4; ASR-02 spawn/no-spawn proof                                 | C      |
| `F6-AC5`   | UNIT-ONLY        | TI      | B5         | N                    | FO           | E     | B4; ASR-02 complete lock timeline                               | C      |
| `F7-AC1`   | UNIT-ONLY        | TI      | B5         | N                    | FO           | E     | B4; ASR-01/02 event flush boundaries                            | C      |
| `F7-AC2`   | PARTIAL          | TI      | B5         | C+N                  | FO           | E     | B4; native events; component semantics secondary                | C      |
| `F7-AC3`   | PARTIAL          | TI      | B5         | C+N                  | FO           | E     | B4; ASR-02 signals, finalization, shutdown                      | C      |
| `F7-AC4`   | PARTIAL          | TI      | B5         | N                    | FO           | E     | B4; ASR-02 null-stdin silent helper; no prompt                  | C      |
| `F8-AC1`   | UNIT-ONLY        | TI      | B6         | N                    | FO           | E     | B4; ASR-03; retained real transcript                            | C      |
| `F8-AC2`   | UNIT-ONLY        | TI      | B6         | N                    | FO           | E     | B4; ASR-03 isolated journal lifecycle                           | C      |
| `F8-AC3`   | PARTIAL          | TI      | B6         | N crash/relaunch     | FO           | E     | B4; ASR-03 historical-PGID non-signal                           | C      |
| `F8-AC4`   | PARTIAL          | TI      | B6         | N                    | FO           | E     | B4; ASR-03 Reveal success/failure                               | C      |
| `D26-AC1`  | UNIT-ONLY        | TI      | B5         | N                    | FO           | E     | B4; real-format transcript through events                       | C      |
| `D26-AC2`  | UNIT-ONLY        | TI      | B5         | U                    | FO           | S     | B4; BP; closed property/table corpus                            | C      |
| `F9-AC1`   | PARTIAL          | TI      | B6         | N                    | FO           | E     | B4; ASR-03 destinations and permissions                         | C      |
| `F9-AC2`   | UNIT-ONLY        | TI      | B6         | N+A                  | FO           | E     | B4; ASR-03; inspect produced ZIP                                | C      |
| `F9-AC3`   | UNIT-ONLY        | TI      | B6         | U                    | FO           | S     | B4; BP; hostile bytes/symlinks/inherited values                 | C      |
| `F9-AC4`   | PARTIAL          | TI      | B6         | N                    | FO           | E     | B4; ASR-03 export/opener boundary                               | C      |
| `F10-AC1`  | PARTIAL          | TI      | B7         | P                    | CR           | C     | B1–6 exits; freeze; DR-1; approved DR-2                         | C      |
| `F10-AC2`  | NONE             | RE      | B8         | A                    | CR           | C     | Same B7 candidate; icon-source provenance is secondary          | C      |
| `F10-AC3`  | NONE             | RE      | B8         | P                    | CR           | C     | Same candidate; DR-1/DR-3; both physical architectures          | C      |
| `F10-AC4`  | NONE             | RE      | B8         | A                    | CR           | C     | Same candidate; complete signed/trusted artifact set            | C      |
| `F11-AC1`  | UNIT-ONLY        | TI      | B6         | N                    | FO           | E     | B4; ASR-03 actual isolated app-data root                        | C      |
| `F11-AC2`  | PARTIAL          | TI      | B3         | U+C                  | FO           | S     | B1; BP; every control, bound, failure, live level               | C      |
| `F11-AC3`  | PARTIAL          | TI      | B3         | C                    | FO           | S     | B1; BP; complete report and clipboard outcomes                  | C      |
| `F11-AC4`  | PARTIAL          | TI      | B6         | N                    | FO           | E     | B4; ASR-03 real commands and visible outcomes                   | C      |
| `F12-AC1`  | PARTIAL          | RE      | B8         | CI                   | FO           | S     | Exact clean candidate commit/lockfiles; source association only | C      |
| `F12-AC2`  | PARTIAL          | TI      | B8         | U+CI                 | FO           | S     | BP; network/process/DNS/service-worker/host isolation           | C      |
| `F12-AC3`  | FULL             | TI-RV   | RV@B4      | U+N                  | FO           | E     | ASR-01 strict catalog/schema equality plus real crossing        | RV     |
| `D25-AC2`  | PARTIAL          | TI      | B7         | P                    | CR           | C     | Freeze; real endpoint, metadata, archive, signature, events     | C      |
| `D25-AC3`  | PARTIAL          | TI      | B7         | P                    | CR           | C     | DR-1/DR-3; prior version; both architectures; refusal           | C      |
| `D25-AC4`  | PARTIAL          | TI      | B7         | P                    | CR           | C     | Profile OS matrix; controlled non-writable install; no auth     | C      |
| `D25A-AC2` | INTEGRATION-ONLY | RE      | B8         | A                    | CR           | C     | Same signed candidate; `--no-sign` smoke is inadmissible        | C      |

### Matrix Validation

| Check                                            |           Expected |                    Planned result |
| ------------------------------------------------ | -----------------: | --------------------------------: |
| Unique P0 criteria mapped                        |                 72 |                                72 |
| Historical baseline FULL                         |                 14 |               14, all marked `RV` |
| Non-FULL criteria                                |                 58 |                58, all marked `C` |
| Open Product Behavior concern                    |                  1 |                                 1 |
| Open Reusable Test Infrastructure concern        |                 52 |                                52 |
| Open Candidate-Specific Release Evidence concern |                  5 |                                 5 |
| Batch 1–8 open-row counts                        | 5/5/11/5/12/10/4/6 |                5/5/11/5/12/10/4/6 |
| Provisionally test-only BP rows                  |                 24 | 24, none reclassified by planning |

### Release Prerequisites Outside the Denominator

| Prerequisite | Retained consequences                                                                                               | Planning point                  | Primary lane/depth | Required validation                                                          |
| ------------ | ------------------------------------------------------------------------------------------------------------------- | ------------------------------- | ------------------ | ---------------------------------------------------------------------------- |
| RP-1         | `D25-AC1` scheduled/menu update checks and `D25-AC5` update-state rehydration without Package History contamination | B7 with final association in B8 | CR/C               | Installed prior-version packaged updater acceptance plus state/menu contract |
| RP-2         | `D25A-AC1` standard Edit and Window actions in the hand-built macOS menu                                            | B7 with final association in B8 | CR/C               | Installed packaged-app native-menu keyboard/interaction acceptance           |

RP-1 and RP-2 receive frozen Acceptance Profile slots but never enter the
72-row denominator, baseline totals, lane totals, or closure-batch counts.

### ASR Contract-Qualification Scenarios

These are enabler acceptance, not extra denominator rows:

1. Validate the locked Draft 2020-12 schemas, exact `/v1` contract bytes,
   canonicalization vectors, I-JSON/NFC/order rules, RFC 8785 JCS output,
   UTF-8/no-BOM/no-trailing-newline identity/profile bytes, and lowercase
   `sha256:<64-hex>` digests across machines and runs.
2. Prove the Candidate Identity Manifest contains identity only, exactly the
   five final artifact subjects, raw-byte sizes/digests after all candidate
   mutation, and one clean GitHub Actions producer attempt.
3. Prove the frozen Acceptance Profile covers all 72 criteria plus RP-1/RP-2,
   binds the approved map/policy/minimum macOS, fixes slot lane/depth/subjects/
   environment/retry semantics, and changes Evidence Set identity on any
   profile change.
4. Validate evidence-record payload/envelope digests, LF-terminated canonical
   NDJSON, sequence/predecessor chain, source/environment/candidate binding,
   subject/result separation, attempt counts, and human/machine agreement.
5. Prove the Release-owned Registrar is the sole append authority through one
   protected GitHub environment and enforces idempotency, lock/CAS, single
   head, no clobber, stale/fork rejection, and immutable snapshots.
6. Prove automatic retries, missing first attempts, branches, ignored/skipped/
   unexecuted results, wrong lanes, shallow provenance, candidate mismatch,
   and incomplete PASS counts fail closed. Manual retry disposition remains
   blocked until DR-4 approval.
7. Prove any rebuild, resign, retag, repackage, artifact replacement, metadata
   change, new release-build workflow run, or new release-build run attempt
   produces a new Candidate Manifest and invalidates affected candidate-bound
   evidence. An evidence-collection retry against the unchanged candidate
   remains a new linked record under DR-4 and retains the first failure; it does
   not create a new Candidate Manifest. Prior records remain immutable history;
   Batch 7/8 candidate slots rerun after candidate invalidation.

### NFR Coverage and Evidence Plan

The concise NFR plan is recorded in Step 3. Each category maps to the matrix
and evidence artifacts as follows:

| Category                        | Primary scenarios                                                                                        | Later `nfr-assess` input                                                                              |
| ------------------------------- | -------------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------- |
| Security/privacy                | B5 process/no-input/no-password; B6 hostile diagnostics; B7 updater/no-admin; B8 trust                   | Native process traces, inspected ZIP/privacy report, updater record, signature/Gatekeeper attestation |
| Performance/capacity            | B2/B3 deterministic state; B5 output limits; B6 retention; B7 packaged 101-row/5,001-line/900×600 checks | Timing/event-count report, retention artifacts, packaged resource/interaction record                  |
| Reliability/recovery            | B2 isolation; B5 cancellation/timeout/shutdown; B6 crash/relaunch/persistence; B7 update/relaunch        | Offline matrix, journals/transcripts, old-PGID non-signal record, relaunch results                    |
| Maintainability/reproducibility | B4 catalog equality; B8 clean forced-offline source run and evidence-contract validation                 | Complete first-run CI report, source/lockfile provenance, schema/vector/ledger validation             |
| Accessibility                   | B7 approved DR-2 packaged method                                                                         | Automated keyboard/focus/4.5:1/reduced-motion results plus manual VoiceOver record                    |
| macOS/release integrity         | B7 updater/install; B8 both-architecture launch/trust/artifact checks                                    | Candidate Manifest/Profile/Index plus Apple-silicon and physical-Intel evidence                       |

Final NFR PASS/CONCERNS/FAIL remains deferred to `nfr-assess`.

### Execution Strategy

Cadence does not change architectural lane or evidence depth:

| Cadence                        | Eligible work                                                                                                                                      | Boundary                                                                                            |
| ------------------------------ | -------------------------------------------------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------- |
| PR                             | Forced-offline source-bound unit, component, browser, contract, format, static, and build checks when the lane remains under about 15 minutes.     | No network/live Manager/candidate claim; failure artifacts retained.                                |
| Nightly                        | Longer forced-offline controlled-environment native/process/lifecycle, large-state, and isolation suites after ASR-01 through ASR-03 are accepted. | Still forced-offline; cannot substitute for target-Mac or candidate evidence.                       |
| Weekly / qualified host window | Serialized provisioned-target-Mac topology and real-capture checks after environment profile qualification.                                        | Environment-bound only; ignored tests count only when explicitly executed and admitted.             |
| Per immutable candidate        | Release preparation after B1–6, then B7 and B8 against the same manifest.                                                                          | Candidate-release is not an ordinary nightly/weekly run. Any mutation invalidates affected results. |

### Resource Estimate

The prior 6–10 QA-engineer-week estimate is not retained because it omitted
the 14 revalidations, RP slots, evidence-contract qualification, three-lane
admission, packaged VoiceOver work, and mandatory physical Intel execution.

| Scope                                                                                         | QA-only effort range after dependencies are ready |
| --------------------------------------------------------------------------------------------- | ------------------------------------------------- |
| Batches 1–3 and associated revalidation                                                       | ~2–3 engineer-weeks                               |
| Batches 4–6 and associated revalidation                                                       | ~3.5–6 engineer-weeks                             |
| Release preparation support, Batches 7–8, RP slots, ledger review, and remaining revalidation | ~3.5–6 engineer-weeks                             |
| **Total**                                                                                     | **~9–15 QA engineer-weeks**                       |

This excludes Development/Platform/Release implementation, hardware or
credential waiting time, named-person scheduling, and candidate-invalidating
reruns. Non-QA implementation effort remains TBD for epics/stories because the
native runner, helper language, evidence transport, and retention mechanics
remain deferred.

### Quality Gates

| Gate                              | Planned threshold/status                                                                                                                                                                                                                     |
| --------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Current planning gate             | **FAIL, 14/72 FULL.** No change in this workflow.                                                                                                                                                                                            |
| Profile admission                 | Blocked while the map is pending approval, DR-1 is OPEN, or DR-4 is PROPOSED.                                                                                                                                                                |
| P0 candidate eligibility          | Future target is 100% executed/admitted profile slots for all 72 criteria; no waiver is compatible with a 100% label.                                                                                                                        |
| Release prerequisites             | RP-1 and RP-2 must pass separately; neither changes the denominator.                                                                                                                                                                         |
| Risk                              | All score-6 and score-9 mitigations have admitted evidence at each frozen profile slot's mapped lane and depth, associated with the exact candidate Evidence Set where permitted without relabeling provenance; no current mitigation claim. |
| Evidence                          | Valid Candidate Manifest, frozen Profile, complete single-head Evidence Index, exact lane/depth/source/environment/subject binding, first failures retained, no ignored/unexecuted substitution.                                             |
| Candidate stability               | Any identity-changing mutation invalidates affected B7/B8 evidence and starts a new evidence root.                                                                                                                                           |
| P1 rate / generic coverage target | **Not applied.** DR-4 remains PROPOSED; neither the legacy 80% P1 rule nor the workflow template's generic 95%/80% defaults may substitute.                                                                                                  |
| NFR decision                      | Evidence sources identified here; final rating deferred to `nfr-assess`.                                                                                                                                                                     |
| Final readiness decision          | Only a later candidate-bound Trace workflow after all prerequisites. This workflow does not regenerate the gate or claim readiness.                                                                                                          |

## Step 5 — Outputs and Validation

The System-Level Create rerun regenerated the existing artifact set in place:

- `_bmad-output/test-artifacts/test-design-architecture.md`
- `_bmad-output/test-artifacts/test-design-qa.md`
- `_bmad-output/test-artifacts/test-design-progress.md`
- `_bmad-output/test-artifacts/test-design/Pack-Manager-handoff.md`

### Reconciliation Checks

- The QA plan and this progress ledger each map exactly 72 unique P0
  criteria from the normative coverage map: no missing, extra, or duplicate
  criterion.
- Baseline classifications remain 14 FULL, 35 PARTIAL, 16 UNIT-ONLY, 2
  INTEGRATION-ONLY, and 5 NONE. The planning gate remains FAIL.
- All 58 non-FULL criteria have one conditional closure row. All 14
  historical-FULL criteria have one candidate-era revalidation row and no
  automatic carry-forward.
- The primary concern split remains 1 Product Behavior, 52 Reusable Test
  Infrastructure, 5 Candidate-Specific Release Evidence, plus 14
  infrastructure-led revalidations.
- Open-row batch counts remain `5/5/11/5/12/10/4/6` for Batches 1–8. Release
  preparation remains an intervening prerequisite after Batches 1–6, not a
  ninth batch.
- RP-1 and RP-2 remain mandatory Release Prerequisites outside the 72-row
  denominator.
- ASR-01 through ASR-05, all three non-substitutable lanes, the versioned
  Manifest/Profile/Index contracts, RFC 8785 JCS and SHA-256 identity rules,
  Release-only append authority, first-failure retention, and candidate
  invalidation are carried into the execution and handoff contracts.
- DR-1 remains OPEN; DR-2 and DR-3 remain APPROVED planning constraints; DR-4
  remains PROPOSED and blocked on Product/QA governance. No substitute P1
  policy is applied.
- Product behavior, reusable infrastructure, and candidate-specific evidence
  remain separate. A missing behavior must be reclassified through a reviewed
  map revision before regression evidence can receive credit.
- The traceability matrix, product behavior, tests, reusable infrastructure,
  CI, release workflows, and configuration were not modified. No criterion
  was promoted and no release-readiness claim was made.

### Independent Reviewer Gate

The first read-only review returned **FAIL pending corrections**. It confirmed
the exact 72-row reconciliation, 24 behavior-present flags, baseline/concern/
batch counts, RP separation, ASR accountability, decision statuses, lane/depth
model, formatting, links, and unchanged authority hashes. It found:

1. three gate statements that incorrectly described all high-risk evidence as
   candidate-specific instead of preserving mapped provenance depth;
2. three invalidation statements that did not distinguish release-build
   run/attempt identity from evidence-collection retry records;
3. one whitespace-sensitive example parser;
4. three zero-padded AD references, one inconsistent R-003 deadline, and a
   missing risk-category legend.

All findings were corrected in the four regenerated artifacts. The same
independent reviewer then completed a read-only correction pass and returned
**PASS**. That pass independently reconfirmed:

- 72/72 unique normative IDs with no missing, extra, or duplicate row;
- the exact baseline, concern, disposition, eight-batch, and 24-BP counts;
- mapped provenance depth and release-build/evidence-attempt separation;
- DR statuses, ASR accountability, RP separation, and the no-readiness boundary;
- working 72-row example parsing, formatting, and local links; and
- unchanged hashes for the trace snapshot, SPEC, DECISIONS, coverage map, and
  Architecture Spine.

The independent reviewer gate is complete. The handoff is ready for
`create-epics-and-stories` as a planning input; every explicit governance,
environment, assignment, and release-preparation blocker remains binding.
