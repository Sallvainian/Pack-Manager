---
title: Pack-Manager P0 Readiness PRD Addendum
status: final
created: 2026-07-22
updated: 2026-07-22
artifact_revision: 1
---

# Addendum: Readiness closure handoff

## Purpose

This addendum preserves downstream planning detail that does not belong in the
product narrative. It does not define product behavior, prove readiness, or
authorize implementation. The PRD defines required outcomes; the test-design
artifacts remain the detailed source for scenarios and estimates.

## A. Blocking evidence enablers

Before implementation planning begins, Architecture must record accountable
owners, delivery timing, and acceptance boundaries for:

| Enabler | Accountable role | Required capability |
| --- | --- | --- |
| ASR-01 — Real native command-and-event boundary | Platform/Architecture | Exercise actual invocation, serialization, handler, and representative event paths while keeping application state isolated. |
| ASR-02 — Controllable process and OS boundaries | Backend/Platform | Produce deterministic output, exit, cancellation, escalation, timeout, lock, input, writability, opener, restart, and permission conditions. |
| ASR-03 — Lifecycle environment | Platform/QA | Use disposable application data for safe crash, forced quit, relaunch, persistence, and historical-process safety. |
| ASR-04 — Candidate attestation | Release | Freeze identity in one Candidate Identity Manifest, then link launch, updater, and other results through an append-only Evidence Index. |
| ASR-05 — Split evidence lanes | QA/CI | Keep forced-offline deterministic evidence separate from the provisioned target-Mac and candidate-bound release lanes. |

Named individuals and schedules belong in architecture and implementation
planning. An unassigned enabler blocks that handoff.

## B. Evidence classification guardrails

- The traceability matrix supplies the 14/72 FULL baseline and status classes.
- The 1/52/5 primary-lane split comes from later test-design planning and is
  provisional.
- For the 24 provisionally test-only rows, begin with behavior-present checks.
- If required behavior is missing, move the row to product/source correction
  before accepting test work.
- A product defect and an evidence gap may overlap on one surface.
- A plan, static source check, collector result, ignored test, or green build
  does not close native or candidate-bound acceptance.

## C. Eight-batch closure decomposition

The eight batches in
`_bmad-output/test-artifacts/test-design-progress.md` are the current planning
decomposition of the 58 non-FULL P0 rows. Their classifications are provisional
until behavior-present checks and the approved coverage map confirm them.

| Batch | Planning outcome | Dependency and exit boundary |
| --- | --- | --- |
| 1 — Restore the `mas` oracle | PC-1 current truth, real-capture provenance, recurrence protection, and dated target-Mac evidence. | Runs first. Product/source correction precedes acceptance of its regression guard. |
| 2 — Detection and refresh | Deterministic absent, phase, failure, timeout, offline, coalescing, and Last-good Snapshot coverage. | May run with Batches 3 and 4 after behavior-present checks. |
| 3 — Package, plan, and Settings | Package states, keyboard selection, plan entry, error feedback, clipboard, and Settings controls. | May run with Batches 2 and 4 after behavior-present checks. |
| 4 — Native boundary foundation | Reusable real command/event, startup, detection, Re-detect, and refresh crossings with isolated state. | May run with Batches 2 and 3; its exit criteria must be met before Batches 5–7 begin. |
| 5 — Manager update and process lifecycle | Routing, locking, live output, D26 boundaries, cancellation, shutdown, stall, timeout, and no-input behavior. | Requires Batch 4 and controllable process/OS boundaries. |
| 6 — Crash, persistence, diagnostics, and Settings | Relaunch, Interrupted state, historical PID safety, durable evidence, diagnostics privacy, native file actions, and Settings persistence. | Requires Batch 4 and the lifecycle environment; may run with Batch 5. |
| 7 — Packaged accessibility and updater | Packaged keyboard/focus/contrast/reduced-motion/VoiceOver evidence and real update behavior from an installed prior public version, including refusal during an active Package Operation. | Requires Batch 4, DR-1, the prior version, current credentials, and one immutable, fully packaged, signed, notarized, and stapled candidate with frozen updater metadata and an RE-1 Candidate Identity Manifest. |
| 8 — Release and reproducibility evidence | Candidate-identity and artifact attestation, architecture/resources, signing/notarization/stapling verification, launch, cross-asset consistency, updater reachability, forced-offline clean evidence, GP-1 policy alignment, and final trace regeneration. | Runs after Batches 1–7 against the unchanged Batch 7 candidate and issues the candidate-bound decision. |

### Dependency waves

1. Batch 1 first.
2. Batches 2, 3, and 4 may proceed in parallel.
3. Batches 5 and 6 follow the Batch 4 native foundation.
4. Release preparation freezes the complete signed, notarized, and stapled
   candidate, updater metadata, and RE-1 Candidate Identity Manifest.
5. Batch 7 uses that immutable candidate.
6. Batch 8 follows Batches 1–7 and attests the same unchanged candidate.

Release preparation is an evidence prerequisite, not a ninth closure batch.
The wave order reflects evidence dependencies. It does not prescribe libraries,
repository structure, or implementation mechanisms.

## D. Release-evidence handoff

The Release Owner must produce the RE-1 Candidate Identity Manifest before any
candidate-bound result is accepted. Results are appended separately to the
Evidence Index. Rebuilding, re-signing, retagging, repackaging, replacing an
artifact, or changing metadata creates a different candidate and invalidates
dependent results.

The release handoff must include:

- fresh-checkout forced-offline quality output;
- direct-download and updater artifact inventory plus checksums;
- architecture, icon, bundle-resource, and entitlement attestation;
- Developer ID, notarization, stapling, and Gatekeeper evidence;
- updater metadata, URL, archive, and detached-signature verification;
- Apple silicon and physical Intel fresh-install launch evidence;
- Apple silicon and physical Intel prior-version update evidence;
- active Package Operation install-refusal evidence and non-writable-install
  evidence;
- retained first failures and explained retries; and
- regenerated candidate-bound traceability using
  `readiness-coverage-map.md`.

## E. Deferred technical decisions

This addendum intentionally does not choose a harness framework, test runner,
process-helper design, CI provider, updater provider, signing storage design,
or implementation branch plan. Those decisions belong to architecture and
implementation planning after the PRD gate is approved.
