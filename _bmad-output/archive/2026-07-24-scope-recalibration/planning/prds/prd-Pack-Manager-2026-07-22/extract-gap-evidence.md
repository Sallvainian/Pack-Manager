---
title: Pack-Manager P0 Readiness Gap Evidence Extract
date: 2026-07-22
scope: P0 product-and-release readiness
gate_status: FAIL
source_commit: fe2881f3e48d26c0561857f72143c6570a5620fc
source_working_tree: dirty
---

# Pack-Manager P0 Readiness Gap Evidence Extract

## Verdict

The authoritative traceability gate is **FAIL**. Only **14 of 72 P0
criteria are FULL (19.4%)**; **58 P0 criteria remain non-FULL**: 35 PARTIAL,
16 UNIT-ONLY, 2 INTEGRATION-ONLY, and 5 NONE.

This is a readiness-evidence failure, not a report of red active test suites.
The latest execution record incorporated by the matrix is green:

- Vitest: 133/133 passed across 23 files.
- Rust: 245 active tests passed; 11 live/environment tests were ignored.
- Playwright: 12/12 project executions passed, representing six logical
  browser cases across Chromium and WebKit.
- Frontend build and the recorded static/CI checks passed.

Those green runs do not prove the real JavaScript-to-Rust Tauri boundary, a
packaged WKWebView application, target-Mac topology, updater installation, or
the authenticity and launchability of a release candidate. See
`_bmad-output/test-artifacts/traceability-matrix.md:931-977` and
`_bmad-output/test-artifacts/automation-summary.md:205-235`.

## Evidence Authority and Staleness Reconciliation

| Artifact | Provenance and execution status | Reconciled use |
| --- | --- | --- |
| `traceability-matrix.md` | Gate `FAIL`; source commit `fe2881f3e48d26c0561857f72143c6570a5620fc` plus a dirty working tree. It catalogs the newly added AUT browser tests and the current fast-terminal plan test. Discovery commands collected tests; the gate step used the latest recorded execution results. | **Authoritative for current criterion status, counts, blockers, and gate outcome.** |
| `automation-summary.md` | Records the 133 Vitest, 245 active Rust, and 12 Playwright passes and clearly limits Playwright to fake Tauri. Its warning that “the trace” was stale refers to an older `673dc717...` trace that predated this automation pass. | Current execution source, but its old-trace warning and “no test-design artifact” statement are chronologically superseded by the regenerated matrix and later design documents. |
| `test-design-progress.md` | Completed planning workflow generated after loading the current matrix. It classifies exactly 58 open P0 criteria into eight batches and explicitly states that it performed no product, test, CI, or release implementation. | Authoritative reconciliation of the 58-item closure plan; **not execution evidence**. |
| `test-design-architecture.md` | Status is “Architecture Review Pending.” It defines five architectural/testability blockers and eight high risks. | Dependency and ownership plan only. |
| `test-design-qa.md` | Status is “Draft.” Native harness, target Mac, signed updater candidate, signing verification, and forced-offline CI are all marked pending. | QA recipe and exit criteria only. |
| `test-design/pack-manager-handoff.md` | Summarizes the same matrix and design outputs for epic/story creation. | Convenience summary; not independent evidence. |

Two apparent contradictions are therefore only chronology:

1. The automation summary correctly rejected the older `673dc717...` trace.
   The current matrix was regenerated at `fe2881f...`, contains AUT-002 through
   AUT-004, and supersedes that warning.
2. The matrix says no test-design artifact was found at its collection step.
   The four test-design documents were generated later and take the matrix as
   input; they do not invalidate its baseline.

The matrix snapshot is still **not immutable release evidence** because its
own front matter records a dirty working tree. Its results are suitable for
planning and the current coverage decision, not for candidate attestation.

## Exact A/B/C Classification

The requested categories are applied once per non-FULL P0 criterion:

- **A — product behavior/correctness:** behavior or repository truth is known
  to violate the requirement.
- **B — test infrastructure/coverage:** behavior is not proven at the required
  level, or a reusable test capability is missing. This combines the design's
  24 `test-only` and 28 `native-test-infrastructure` rows.
- **C — release evidence/operational proof:** only an immutable built candidate,
  live operational lane, or release attestation can supply the missing proof.

| Category | PARTIAL | UNIT-ONLY | INTEGRATION-ONLY | NONE | Total |
| --- | ---: | ---: | ---: | ---: | ---: |
| A — product behavior/correctness | 0 | 0 | 0 | 1 | **1** |
| B — test infrastructure/coverage | 34 | 16 | 1 | 1 | **52** |
| C — release evidence/operational proof | 1 | 0 | 1 | 3 | **5** |
| **Total open P0** | **35** | **16** | **2** | **5** | **58** |

Only one product defect is confirmed by the inspected evidence. The 24
`test-only` items remain Category B unless a required behavior-present check
finds the behavior absent; in that event, the affected item must be
reclassified to A before a regression test is authored. The current counts
must not pre-judge those checks.

## Five Critical P0 Blockers (Coverage NONE)

| Criterion | Category | Current evidence status | Required closure | Dependencies |
| --- | --- | --- | --- | --- |
| `D23a-AC4` | **A** | **NONE.** The SPEC text, Rust comments, fixture README, and ignored live smoke retain superseded absent/unverified/synthetic `mas` claims. | Correct every obsolete claim, then add a repository regression guard that prevents its return. | Product/Backend correction before QA acceptance. |
| `F1-AC7` | **B** | **NONE.** No active test proves current six-manager ownership; the ignored smoke expects `mas` absent. | Update the live expectation and retain a dated successful topology record with installed `mas` on the designated Mac. | Correct D23a truth; provisioned and serialized target-Mac lane. |
| `F10-AC2` | **C** | **NONE.** Icon scripts, assets, and configuration are static intent only. | Inspect the built candidate and attest the 1024px source, required generated icon set, and packaged resources. | Immutable candidate and candidate-bound artifact inspection. |
| `F10-AC3` | **C** | **NONE.** No collected packaged launch proves Finder/Dock startup, resources, entitlements, GUI PATH, or WKWebView. | Launch the same attested candidate through Finder and Dock and archive launch/WebView evidence. | Provisioned Mac and immutable candidate. |
| `F10-AC4` | **C** | **NONE.** No run, asset, signature, notarization ticket, staple, or updater archive was verified. | Attest universal architectures, Developer ID signature, app/DMG notarization and stapling, required assets, updater archive/signature, URLs, checksums, and version consistency. | Release credentials, signed immutable candidate, candidate identity, and machine-readable attestation. |

The three Category C NONE items roll up to `R-007`, the only score-9 risk.
`D23a-AC4` is the sole known Category A defect. `F1-AC7` is a coverage and
live-lane blocker rather than proof that current detection behavior is wrong.

## Full Material Gap Inventory

### A — Product Behavior/Correctness (1)

| Batch | Criteria and baseline | Evidence and closure |
| --- | --- | --- |
| 1 — Restore `mas` oracle | `D23a-AC4` — NONE | The repository currently contradicts the superseding D23a decision. Remove all stale claims before adding the guard; a test cannot close a requirement that the repository still violates. |

### B — Test Infrastructure/Coverage (52)

#### B1. Missing or incomplete deterministic/live coverage (24)

| Batch | Criteria and baseline | Decisive closure evidence |
| --- | --- | --- |
| 1 — `mas` oracle | `F1-AC7` — NONE; `D23a-AC1` — PARTIAL; `D23a-AC2` — PARTIAL; `D23a-AC5` — UNIT-ONLY | Dated serialized target-Mac topology smoke; real-capture provenance checks; fixture policy that prevents synthetic data from becoming a correctness oracle. |
| 2 — detection/refresh | `F1-AC6` — PARTIAL; `F1-AC8` — PARTIAL; `F2-AC3` — UNIT-ONLY; `F2-AC6` — PARTIAL; `F2-AC8` — PARTIAL | Component assertions for absent/report/phase-label behavior plus a deterministic all-adapter timeout/offline matrix with stale-state isolation. |
| 3 — package/plan/settings | `F3-AC1` — PARTIAL; `F3-AC2` — PARTIAL; `F3-AC3` — PARTIAL; `F3-AC4` — PARTIAL; `F3-AC6` — PARTIAL; `F3-AC8` — PARTIAL; `F4-AC1` — PARTIAL; `F5-AC1` — PARTIAL; `F5-AC3` — PARTIAL; `F11-AC2` — PARTIAL; `F11-AC3` — PARTIAL | Complete component/browser state, keyboard, plan-entry, error-feedback, settings-control, and clipboard contracts. Perform a behavior-present check before retaining “test-only.” |
| 5 — routed refresh/D26 | `F2-AC9` — PARTIAL; `D26-AC2` — UNIT-ONLY | Prove routed self-update refreshes both subject and executor; add terminated/repeated/near-match/unrelated byte cases that prohibit heuristic output rewriting. |
| 6 — diagnostics privacy | `F9-AC3` — UNIT-ONLY | Exhaustive hostile-environment, symlink, arbitrary-byte, and missing-source fixtures proving no inherited value or traversal enters the archive. |
| 8 — CI isolation | `F12-AC2` — PARTIAL | Expand the guard beyond `fetch` to WebSockets, service workers, child processes, DNS/network calls, and undeclared machine dependencies while keeping live tests separate. |

#### B2. Missing real Tauri/macOS/process test infrastructure (28)

| Batch | Criteria and baseline | Decisive closure evidence and dependency |
| --- | --- | --- |
| 4 — native harness foundation | `F1-AC1` — PARTIAL; `F1-AC2` — PARTIAL; `F1-AC3` — UNIT-ONLY; `F1-AC4` — UNIT-ONLY; `F2-AC1` — UNIT-ONLY | A reusable real invoke/event harness with isolated app state and executable fixtures. It must cross actual serialization for startup, re-detection, ToolEnv/version probes, and all-six-manager refresh order. This is foundational for Batches 5–7. |
| 5 — self-update/process lifecycle | `F6-AC1` — PARTIAL; `F6-AC2` — UNIT-ONLY; `F6-AC3` — INTEGRATION-ONLY; `F6-AC4` — UNIT-ONLY; `F6-AC5` — UNIT-ONLY; `F7-AC1` — UNIT-ONLY; `F7-AC2` — PARTIAL; `F7-AC3` — PARTIAL; `F7-AC4` — PARTIAL; `D26-AC1` — UNIT-ONLY | Native command/event traces, controlled process helpers, lock timelines, stdout/stderr batching, null stdin, ordinary cancellation and escalation, shutdown reaping, stall/hard-cap/no-password evidence, and real-format `mas` transcript handling. Requires Batch 4 and injectable OS/process boundaries. |
| 6 — lifecycle/filesystem/native actions | `F8-AC1` — UNIT-ONLY; `F8-AC2` — UNIT-ONLY; `F8-AC3` — PARTIAL; `F8-AC4` — PARTIAL; `F9-AC1` — PARTIAL; `F9-AC2` — UNIT-ONLY; `F9-AC4` — PARTIAL; `F11-AC1` — UNIT-ONLY; `F11-AC4` — PARTIAL | Disposable app data, forced crash/relaunch, explicit old-PGID non-signal proof, real transcript/journal/history evidence, inspected diagnostics ZIP, Application Support persistence, and native opener/export outcomes. Requires Batch 4 plus lifecycle isolation. |
| 7 — packaged accessibility/updater | `F10-AC1` — PARTIAL; `D25-AC2` — PARTIAL; `D25-AC3` — PARTIAL; `D25-AC4` — PARTIAL | Packaged WKWebView evidence plus real endpoint/plugin/event/download/install/relaunch and controlled non-writable-bundle proof. Requires Batch 4 and an immutable signed candidate. Contrast cannot be marked FULL until an approved oracle exists. |

### C — Release Evidence/Operational Proof (5)

| Batch | Criteria and baseline | Decisive closure evidence |
| --- | --- | --- |
| 8 — candidate assets | `F10-AC2` — NONE | Candidate-bound icon and bundle-content attestation. |
| 8 — packaged launch | `F10-AC3` — NONE | Finder/Dock launch and WKWebView evidence from the attested candidate. |
| 8 — release authenticity | `F10-AC4` — NONE | Universal architecture, signing, notarization, stapling, Gatekeeper, updater, checksums, and required-assets attestation. |
| 8 — cross-asset consistency | `D25A-AC2` — INTEGRATION-ONLY | Compare tag/version, DMG, ZIP, updater archive/signature, and `latest.json`; prove required no-sign build-smoke behavior. Static workflow inspection is insufficient. |
| 8 — reproducible quality run | `F12-AC1` — PARTIAL | Fresh-checkout Rust/frontend quality suites with outbound network denied and complete first-run output retained. Current local/CI greens are not this proof. |

## Systemic Blockers and Dependencies

| Blocker | Category | Impact | Closure dependency/status |
| --- | --- | --- | --- |
| Correct D23a repository truth | **A** | `D23a-AC4` cannot close, and target-Mac acceptance would use a contradictory oracle. | Product/Backend correction is pending and must precede Batch 1 exit. |
| ASR-01 real Tauri invoke/event harness | **B** | Frontend and Rust suites can both pass while registration, payloads, serialization, ordering, or native side effects are broken. It is the foundation for 28 native-infrastructure criteria. | Platform-owned harness with isolated state; architecture approval and implementation pending. |
| ASR-02 controllable OS/process boundaries | **B** | Failure, signal, stdin, timing, writability, opener, restart, and permission paths cannot be exercised safely or deterministically. | Injectable paths/clocks/executable discovery/signals/stdin/open/reveal/restart/writability; pending. |
| ASR-03 lifecycle harness | **B** | Crash recovery, durable persistence, stale-PGID safety, and relaunch remain assumptions. | Disposable persistent app-data root and safe forced termination/relaunch; pending. |
| ASR-04 candidate-bound release attestation | **C** | A green workflow cannot prove a candidate is authentic, consistent, launchable, or updater-compatible. | Immutable candidate identity spanning commit, tag, versions, checksums, assets, signing/notarization, launch, and updater; pending. |
| ASR-05 isolated evidence lanes | **B** | Default/clean evidence can silently depend on network or host state, and live topology can drift. | Forced-offline clean CI plus a separate serialized, provisioned target-Mac lane; pending. Candidate-specific outputs from that lane remain Category C evidence. |

Additional dependencies recorded by the design:

- A designated target Mac must be available for Batches 1 and 8.
- Disposable app-data roots and controlled executable/process fixtures are
  required before Batches 4–6.
- An immutable signed candidate and required release credentials are required
  before Batches 7–8; secrets remain in fnox/GitHub Secrets.
- Architecture must approve owners and timelines for ASR-01 through ASR-05 and
  risks R-001 through R-008.
- Evidence storage must retain the first failure; retries cannot erase it.
- Every Category B “test-only” scenario starts with a behavior-present check.

See `test-design-architecture.md:47-62,105-119,171-190`,
`test-design-qa.md:66-109,171-198,419-437`, and
`test-design-progress.md:296-327,358-362`.

## Evidence Status and Heuristic Limits

### What is currently proven

- The matrix has a high-confidence formal oracle from `docs/SPEC.md` amended
  by `docs/DECISIONS.md`.
- The current inventory contains 395 cases: 384 active and 11 ignored.
- Lower-level deterministic behavior is strongest for plan authentication,
  fast-terminal/coherent-state revalidation, tamper/replay rejection,
  structured argv, atomic all-or-none admission, locks/concurrency, parser
  behavior, IPC fixtures, and many component states.
- Browser evidence covers search, selected-plan review/confirm, immediate
  row-level execution, dark tokens, focus visibility, and reduced motion
  through fake Tauri.

### What the evidence cannot establish

1. **Collection is not execution.** Vitest/Cargo/Playwright collectors built the
   catalog without executing bodies. Gate execution status comes from the
   separately recorded automation run.
2. **Fake Tauri is not native Tauri.** All 20 registered commands lack a real
   frontend-to-handler crossing, and no representative real event round-trip
   exists. Browser WebKit is not the packaged Tauri WKWebView.
3. **Ignored tests are not acceptance evidence.** Eleven real-process,
   real-shell, or live-manager tests were not executed; the current-machine
   `mas` expectation among them is obsolete.
4. **Static source/workflow intent cannot close native or release criteria.**
   Configuration is useful support but does not prove a produced artifact.
5. **The network guard is narrow.** It exercises ordinary `fetch`; WebSockets,
   service workers, DNS/child-process calls, same-origin traffic, and the
   `ALLOW_REMOTE_E2E=1` bypass are outside its proof.
6. **Heuristic counts are directional, not exhaustive proof.** The matrix
   reports 20 untested native commands, 9 happy-path-only criteria, 6
   E2E-missing UI journey families, and 3 missing UI states. These identify
   review targets; they do not replace criterion-specific execution.
7. **Fixture evidence has provenance limits.** Positive IPC fixture guards do
   not broadly prove malformed-payload rejection. Synthetic manager fixtures
   prove robustness/shape only, not real-format correctness.
8. **No generic source-code coverage threshold exists.** The 19.4% figure is
   strict acceptance-criterion coverage, not line or branch coverage.
9. **Unknown NFR oracles must not be invented.** Vulnerability/CSP policy,
   general performance/SLO targets, code-coverage/lint/freshness gates,
   WCAG/contrast, supported macOS versions, and required Intel-hardware
   coverage remain stakeholder decisions. They are not extra P0 criteria, but
   an affected criterion cannot be promoted to FULL on an undefined oracle.
10. **Plans and estimates are not proof.** The architecture, QA, progress, and
    handoff documents describe future closure work; none executes a native or
    release scenario.

## Closure Sequence and Exit Criteria

| Wave | Work | Dependency | Required result |
| --- | --- | --- | --- |
| 0 | Batch 1 — D23a/`mas` oracle | None; complete first | Corrected repository truth, provenance guard, and dated target-Mac topology evidence. |
| 1, parallel | Batches 2, 3, and 4 | Behavior-present checks; native harness ownership | Deterministic UI/adapter coverage and a reusable native startup/refresh foundation. |
| 2, parallel | Batches 5 and 6 | Batch 4 plus process/lifecycle fixtures | Native process, operation, crash, persistence, diagnostics, settings, and filesystem evidence. |
| 3 | Batch 7 | Batch 4 and immutable signed candidate | Packaged accessibility and real updater check/download/install/relaunch/no-admin evidence. |
| 4 | Batch 8 and trace regeneration | Completed candidate and evidence from Batches 1–7 | Candidate attestation, packaged launch, cross-asset consistency, forced-offline clean run, and regenerated matrix. |

P0 readiness closes only when:

1. All 58 open P0 criteria move to FULL and the regenerated matrix reports
   **72/72 P0 FULL**.
2. Every planned P0 validation passes with no retry hiding the first failure.
3. The D23a product/repository defect is corrected before its regression test
   is accepted.
4. Native and release criteria are backed by real native/candidate evidence,
   never source inspection alone.
5. All score-6 and score-9 risk mitigations are complete, or an explicit
   time-bounded waiver exists.
6. Every in-scope NFR category has its named evidence artifact.
7. QA Lead and Dev Lead accept the evidence and no open P0/P1-severity defect
   affects the candidate.
8. `bmad-testarch-trace` is rerun against the exact evidence-bearing candidate.

### Release-gate policy caveat

The later test-design documents intentionally scope work to P0 and sometimes
describe 72/72 P0 FULL as the “final release” gate. The current authoritative
matrix also applies an independent **P1 strict-full threshold of 80%**, while
P1 is currently 0/8 FULL. Therefore:

- 72/72 P0 FULL is the correct exit for this P0 readiness extract.
- It does **not by itself guarantee that the unchanged deterministic release
  gate will become PASS**.
- Release-gate PASS additionally requires either enough P1 closure to meet the
  configured threshold or an explicit, documented policy change/waiver.

This is a **Category C operational gate-policy dependency**, not one of the 58
non-FULL P0 criterion rows and therefore is not included in the A/B/C counts.
See `traceability-matrix.md:943-951` versus
`test-design-qa.md:56-64,186-198,338-340`.

## Source Pointers

- Gate, coverage, blockers, heuristics, and execution:
  `_bmad-output/test-artifacts/traceability-matrix.md:754-767,875-996`
- Current automation execution and limitations:
  `_bmad-output/test-artifacts/automation-summary.md:205-251`
- Exact 58-item classification and batch plan:
  `_bmad-output/test-artifacts/test-design-progress.md:130-283`
- Architectural blockers, risks, and dependencies:
  `_bmad-output/test-artifacts/test-design-architecture.md:22-62,70-190`
- QA dependencies, entry/exit criteria, and tooling state:
  `_bmad-output/test-artifacts/test-design-qa.md:37-198,419-456`
- BMAD handoff and phase gates:
  `_bmad-output/test-artifacts/test-design/pack-manager-handoff.md:17-108`
