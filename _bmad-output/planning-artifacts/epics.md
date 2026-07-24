---
stepsCompleted:
  - step-01-validate-prerequisites
  - step-02-design-epics
  - step-03-create-stories
  - step-04-final-validation
inputDocuments:
  - _bmad-output/planning-artifacts/prds/prd-Pack-Manager-2026-07-22/prd.md
  - _bmad-output/planning-artifacts/prds/prd-Pack-Manager-2026-07-22/addendum.md
  - _bmad-output/planning-artifacts/architecture/architecture-Pack-Manager-2026-07-23/ARCHITECTURE-SPINE.md
  - _bmad-output/test-artifacts/test-design-architecture.md
  - _bmad-output/test-artifacts/test-design-qa.md
  - _bmad-output/test-artifacts/test-design-progress.md
  - _bmad-output/test-artifacts/test-design/Pack-Manager-handoff.md
  - _bmad-output/planning-artifacts/prds/prd-Pack-Manager-2026-07-22/readiness-coverage-map.md
  - _bmad-output/planning-artifacts/ux-designs/ux-Pack-Manager-2026-07-23/DESIGN.md
  - _bmad-output/planning-artifacts/ux-designs/ux-Pack-Manager-2026-07-23/EXPERIENCE.md
  - _bmad-output/planning-artifacts/ux-designs/ux-Pack-Manager-2026-07-23/validation-report.md
  - _bmad-output/planning-artifacts/sprint-change-proposal-2026-07-24.md
  - docs/SPEC.md
  - docs/DECISIONS.md
---

# Pack-Manager - Epic Breakdown

## Overview

This document provides the complete epic and story breakdown for Pack-Manager, decomposing the finalized requirements, formal architecture, finalized UX contract, reconciled System-Level Test Design, normative readiness mapping, and product-behavior authorities into implementable stories. The 2026-07-24 Correct Course amendment below is binding and supersedes older immediate-row, direct Manager-update, Activity-drawer, Operation-History, and `autoOpenDrawer` wording in affected stories.

## Requirements Inventory

### Functional Requirements

FR-1: Detect Homebrew, mise, npm, uv, rustup, and `mas` at launch and on demand; report each Manager's path, available version, ownership, and evidence; treat absence as a normal Not installed state with a known install hint; support normal Finder/Dock launch; and replace detection state only with one coherent result.

FR-2: Treat each Manager's Outdated verdict as authoritative, preserve Manager-supplied version strings verbatim, keep version-delta styling display-only, retain unknown latest versions as unknown, and fail a Manager visibly rather than inventing state when parsing is incompatible.

FR-3: Refresh installed inventory and Outdated state independently per Manager, permit safe cross-Manager concurrency, show per-Manager phases and failures, coalesce duplicate refreshes, retain and label Last-good Snapshots, and refresh every affected subject and executor after successful updates.

FR-4: Derive Manager ownership and self-update Routes from current detection and refresh information, preserve inspectable evidence, reconsider Routes after fresh data, explain subject and executor, and disable unavailable routed work with a reason.

FR-5: Let the user browse, search, filter, and understand Manager-specific Package state; distinguish current, Outdated, pinned, greedy/self-updating, unknown-version, and error states; exclude pinned Packages; default-exclude greedy casks; apply only the specified rustup/mise plan deduplication; and retain useful Manager details.

FR-6: Support exact selection of eligible Outdated Packages through individual, range, toggle, filter-aware select-all, tri-state, and clear interactions; prevent current, pinned, or default-excluded Packages from entering selection; add exact identities to one persistent draft; keep the draft across Manager navigation; and never execute from selection or a row.

FR-7: Preview every Package and Manager update command exactly in one persistent Upgrade Plan; keep Manager updates independently removable; reveal native-produced commands on demand; explain exclusions/warnings/staleness; and use a separate final confirmation dialog whose safe default is reversible in Settings.

FR-8: Execute a bulk request only when it exactly matches the reviewed Upgrade Plan and a coherent rebuild from current state; replace stale plans and require reconfirmation; reject tampering, replay, eviction, missing authorization, active refresh, and conflicting mutation drift without enqueueing; keep Plan Capabilities bounded and one-use; and ignore dismissed or superseded late continuations.

FR-9: Admit a confirmed multi-group plan atomically; enqueue all groups or none; serialize conflicts; allow independent Managers within the global limit; explain queue relationships; and name external Homebrew contention without automatic retry.

FR-10: Provide a low-friction single-Package action that adds exactly that eligible Package to the persistent Upgrade Plan, follows the common confirmation path, retains all eligibility/Route/conflict/no-privilege protections, and never executes immediately or expands to unrelated Packages.

FR-11: Give every Manager a standardized title area with short description, path, installed/update version state, consistent Manager-status badge, ownership/Route explanation, npm-inside-mise consequences, and an action that adds independent removable Manager-update membership to the plan.

FR-12: Execute only product-defined structured Operations; expose no general shell, `sudo`, password, or administrator-prompt path; never treat display text as executable input; use null stdin; keep copy-to-terminal user-controlled; and convert an elevated app-update requirement into manual-install-required.

FR-13: Expose queued, running, verifying, stalled, cancelling, and terminal plan state with exact nested Operation commands/live output correlated by `planAttemptId` and `opId`; use the sidecar as live progress and Results; make Activity a first-class destination; bound live output; and preserve complete retained transcript output.

FR-14: Turn silence and excessive duration into honest actionable states using the 120-second default stall threshold, Keep waiting/Copy command/Cancel plan choices, trusted-only interaction classification, the 30-minute default hard cap, attempt-wide cancellation with process-group escalation, explicit terminal outcomes, and an explicit quit choice without promising rollback.

FR-15: Durably correlate each confirmed Plan Attempt's reviewed intent, command snapshot, Operations, verification, Results, and Retry lineage through `planAttemptId`; reconstruct unfinished work as Interrupted without signaling historical process identifiers; present one History row per attempt with Activity replay and nested transcript detail; preserve honest legacy Operations; apply only D26's closed literal repair; and retain/prune durable evidence as specified.

FR-16: Refresh affected state after successful work, retain prior useful Manager state on failure, provide actionable error feedback, and expose View log only when a corresponding log exists.

FR-17: Persist Settings before changing active values or the canonical revision; leave both unchanged on save failure; default upgrade confirmation on through `skipUpgradePlanConfirmation: false`; treat `autoOpenDrawer` as inactive legacy input; support editable thresholds/live log level; and provide Environment Report, Copy, Open Logs Folder, diagnostics export, and Re-detect.

FR-18: Export one timestamped diagnostics ZIP to the documented Desktop path containing `report.json`, the newest three application logs, newest 25 transcripts, and `operations.jsonl`; include app/OS/architecture, constructed ToolEnv and detection evidence, Settings, and log filter; exclude inherited environment values; and reject symlink substitution during selection and streaming.

FR-19: Preserve one coherent dark-only macOS interface across Dashboard, expandable Manager navigation, Manager workspaces, persistent Upgrade Plan, separate Confirmation Dialog, Activity, Results, one-plan-per-row History, Settings, status, and app menus; keep primary actions keyboard/VoiceOver operable with deterministic focus and non-color cues; preserve VersionDelta as display-only; honor reduced motion; meet contrast; and remain usable at 900 × 600, 150–200% zoom, more than 100 Packages, and long output.

FR-20: Check for application updates and automatically download a newer authorized release in the background while keeping install/restart under user control, Package work understandable, and checking/available/downloading/ready/failure states visible.

FR-21: Install a downloaded application update only after the user chooses Restart to update; never silently install or restart; refuse install/relaunch while a Package Operation is queued or running; relaunch as the intended version; produce manual-install-required for a non-writable install; and keep every update-stage failure actionable.

FR-22: Support the declared Apple-silicon and Intel promise through normal Finder/Dock launch and accept only updater payloads authorized for the installed application; report success only after relaunch as the intended version.

RP-1: Preserve launch, six-hour, and app-menu update checks; restore in-process update state after supported UI recreation; preserve saved trigger policy across normal relaunch; ensure failed/interrupted downloads never appear Ready; keep application-update state separate from Package Operation queue and History; and validate this mandatory prerequisite outside the 72-row P0 denominator.

RP-2: Preserve standard macOS Edit and Window menu actions, including cut/copy/paste/select-all in search and every copyable command surface, as a mandatory prerequisite outside the 72-row P0 denominator.

### NonFunctional Requirements

NFR-1: Fail closed so unreviewed, stale, altered, replayed, partially admissible, or privilege-seeking work never runs and all user exclusions and Manager protections remain authoritative.

NFR-2: Isolate and recover from detection, refresh, parse, network, update, crash, cancellation, timeout, and persistence failures without blanking another Manager or destroying a Last-good Snapshot.

NFR-3: Render progressive state without waiting for all Managers; remain interactive with more than 100 Package rows; prove reachability and correct actions at 101 rows; flush live output at 50 milliseconds, 64 lines, or 8 KiB; retain the newest 5,000 live lines at 5,001 while preserving the complete transcript; and keep navigation, plan, confirmation, Activity, Results, and recovery usable at 900 × 600 and 150–200% zoom.

NFR-4: Correlate status, output, transcript, structured log, History, and diagnostics through durable Plan Attempt identity and nested Operation identity; block spawn when transcript creation fails; and keep later noncritical logging failures from hanging Package work.

NFR-5: Send no telemetry, expose no generic shell surface, exclude inherited environment values from logs and diagnostics, and resist diagnostic symlink substitution.

NFR-6: Keep primary interactions keyboard/VoiceOver operable with visible focus and deterministic dialog/sidecar focus restoration, provide non-color status cues and accessible ineligibility reasons, meet at least 4.5:1 text contrast, honor reduced motion, and announce plan progress, verification, cancellation, failure, and completion without noisy output narration.

NFR-7: Support normal GUI launch and both promised architectures, fail visibly and locally on incompatible Manager output, and require Product and Release to declare the minimum supported macOS version before final candidate acceptance.

NFR-8: Keep direct-download and updater artifacts mutually consistent, cryptographically authorized, and attributable to one Release Candidate without weakening explicit install/restart control.

### Additional Requirements

#### Readiness and Scope Controls

- Preserve exactly 72 P0 criteria from `readiness-coverage-map.md`, whose status remains `final-pending-approval`.
- Preserve the historical planning baseline as FAIL with 14/72 FULL. Plan closure for all 58 non-FULL criteria and candidate-era revalidation of all 14 historical-FULL criteria at their mapped evidence depth; never carry a historical status forward automatically.
- Preserve the 58-row provisional concern split of 1 Product Behavior, 52 Reusable Test Infrastructure, and 5 Candidate-Specific Release Evidence, subject to TIR-1 behavior-present reclassification.
- Keep RP-1 and RP-2 mandatory but outside the denominator, baseline totals, concern totals, and batch counts.
- Do not promote criteria, approve or revise the coverage map, regenerate traceability, configure the gate, execute evidence, or claim product-and-release readiness in this planning artifact.
- Treat Product Behavior, Reusable Test Infrastructure, and Candidate-Specific Release Evidence as separate primary concerns. Every criterion-bearing story declares exactly one.
- For each of the 24 `BP` rows, check that required behavior is present before accepting regression work. Missing or incorrect behavior creates Product Behavior work and requires a reviewed map revision before regression evidence can receive credit.

#### Product Acceptance Journeys

- AJ-1: Prove normal Finder/Dock launch, progressive rendering, six-Manager detection, ownership, independent refresh, normal absence, Last-good Snapshot retention, and useful recovery.
- AJ-2: Prove exact Update Everything preview, explicit inclusions/exclusions/warnings, stale-plan replacement and reconfirmation, atomic admission, safe concurrency, and understandable queue reasons.
- AJ-3: Prove exact selection or row-level update, discovered Manager Route explanation, visible command, and affected-state refresh.
- AJ-4: Prove exact command/live output, stall choices, no automatic Homebrew-contention retry, explicit cancellation/timeout outcomes, and Interrupted reconstruction after crash/forced quit.
- AJ-5: Prove searchable History, command/outcome/transcript reconstruction, Finder reveal, and privacy-preserving diagnostics export.
- AJ-6: Prove trusted direct installation, normal launch, prior-public-version discovery/download, explicit Restart to update, intended-version relaunch, and non-writable manual-install behavior without privilege escalation.

#### Test Infrastructure Readiness

- TIR-1: Begin every provisionally test-only gap with behavior-present verification; reclassify missing/incorrect behavior into Product Behavior before regression credit; complete PC-1 before D23a recurrence credit; and use the formal FULL definition rather than test-file presence.
- TIR-2: Provide a deterministic forced-offline lane from a clean checkout with real outbound network denied, controlled process/state/time seams, no real Manager processes, no sleeps, no undeclared host state, separate live/release checks, full deterministic success/failure coverage, and exact D26 positive/negative boundaries.
- TIR-3: Provide a reusable real native lane crossing frontend invocation, Tauri serialization/registration, Rust handlers, and representative events using the shared production boundary; isolate state and executables; prove startup ordering, detection, Re-detect, and six-Manager refresh; and never relabel fake-browser evidence as native.
- TIR-4: Provide deterministic stdout/stderr/silence/exit/expected-nonzero/cancellation/escalation/timeout/external-lock conditions; prove null input, no-password behavior, process cleanup, and queue/lock timelines; and control opener, reveal, restart, discovery, writability, and time at acceptance boundaries.
- TIR-5: Provide disposable application-data lifecycle acceptance for crash, forced quit, relaunch, persistence, History, transcript, journal, interrupted recovery, Settings atomicity, diagnostics, retention, hostile filesystem cases, and historical-process non-signal.
- TIR-6: Provide a serialized provisioned-target-Mac lane with dated six-Manager topology including live `mas`, immutable real-capture provenance, explicit drift detection, and no credit for ignored live tests unless explicitly executed and admitted.
- TIR-7: Provide installed packaged-WKWebView acceptance for keyboard/focus, automated 4.5:1 text contrast, reduced motion, manual VoiceOver focus/completion announcements, updater check/metadata/download/signature/explicit install/relaunch/non-writable behavior, and before/after evidence from an actually installed prior public version; keep no-sign smoke separate; preserve one candidate identity.
- TIR-8: Record source-bound, environment-bound, and candidate-bound provenance at their exact depth; exclude collected-only and ignored checks; preserve first failures; disable automatic retries; and keep human- and machine-readable outputs consistent and available for later trace regeneration.

#### Release Evidence Requirements

- RE-1: Before candidate-bound validation, create an immutable identity-only Candidate Identity Manifest binding one clean source commit, tag, coherent versions, build run/attempt, toolchains, signing identities, final artifact names/checksums, and published metadata checksum; link results separately through an append-only Evidence Index; invalidate affected evidence on any candidate mutation.
- RE-2: Retain a complete first-attempt clean-checkout forced-offline result for required frontend/Rust formatting, static checks, production builds, contracts, and tests. A no-sign smoke cannot satisfy candidate trust requirements.
- RE-3: Attest that tag, bundle/package/Cargo/updater versions, asset names, and metadata agree and that the GitHub Release contains the DMG, direct ZIP, updater archive, detached signature, and `latest.json`.
- RE-4: Attest the exact candidate's required architectures, approved icon source/generated resources, packaged resources, and entitlements; include mandatory physical Intel acceptance in addition to universal-binary inspection.
- RE-5: Attest valid Developer ID identity, secure signatures, accepted notarization, required stapling, and Gatekeeper acceptance of the downloaded app and disk image without bypass.
- RE-6: Attest reachable complete HTTPS metadata for both Mac architecture identifiers, correct archive URLs, detached-signature validation against the embedded public key, and candidate-version consistency.
- RE-7: Produce exact-candidate DMG install and Finder/Dock launch evidence on Apple silicon and physical Intel, covering resources, entitlements, GUI environment discovery, and the packaged WKWebView.
- RE-8: Produce prior-public-version-to-candidate discovery, download, explicit Restart to update, install, and relaunch evidence on Apple silicon and physical Intel, including before/after versions, no administrator prompt, and refusal while a Package Operation is queued or running.
- RE-9: Prove that a non-writable install location produces manual-install-required and never invokes administrator authorization.
- RE-10: Regenerate traceability only in a later workflow against the complete exact-candidate Evidence Set and approved one-to-one coverage map; revalidate all 14 historical-FULL criteria; require QA/Development acceptance tied to the Candidate Manifest digest and Evidence Index; and never auto-promote a criterion.
- RE-11: Publish the final decision through the append-only Evidence Index with retained human/machine results; preserve every failed attempt and require later retries to explain the change without overwriting the first result.

#### Architecture Invariants and ASR Enablers

- AD-1: Each work item and result has exactly one primary readiness concern; missing behavior returns to Product Behavior; infrastructure never issues a readiness status; candidate evidence cannot change product behavior or its oracle.
- AD-2: Use one production composition root and two construction-time adapter sets. Production and native acceptance use the same application core, commands, events, handlers, and wire types; controlled adapters exist only in a non-distributable harness and cannot be activated from release bits.
- AD-3 / ASR-01: Architecture is solely accountable for the shared real native command/event boundary. Accept by Batch 4 exit through exact set equality across the versioned `contracts/tauri-boundary/v1.json` catalog, production registration, Rust/TypeScript wire contracts, wrappers/subscriptions, fixtures, inventory, and native vectors; round-trip every catalog command and dispatch every catalog event through real isolated Tauri. The current 20 commands/six events are a baseline, not fixed counts.
- AD-4 / ASR-02: Development is solely accountable; Platform is the capability area. Accept core deterministic process controls before Batch 5 and relevant filesystem/updater extensions before Batches 6–7. Typed ports/helpers must produce all required output, exit, signal, timeout, lock, stdin, path, permission, opener, restart, and updater conditions while production adapters retain fail-closed safety.
- AD-5 / ASR-03: QA is solely accountable; Development/Platform supports. Accept before Batch 6 using disposable roots and a lifecycle controller that proves crash, forced quit, relaunch, persistence, retention, hostile filesystem behavior, packaged quit wiring, and historical-PGID non-signal without touching operator data or processes.
- AD-6 / ASR-05: QA is solely accountable; CI is the execution mechanism. Accept lane separation before any Batch 1 evidence and make the candidate lane operational before Batch 7. Isolate `forced-offline`, `provisioned-target-mac`, and `candidate-release` workspaces, credentials, caches after lane entry, outputs, and provenance; reject cross-lane substitution.
- AD-7 / ASR-04: Release is solely accountable for candidate identity and attestation. Accept the contract before release preparation, freeze the manifest before Batch 7, and accept the complete ledger in Batch 8.
- Candidate Identity Manifest v1 must use strict closed JSON Schema Draft 2020-12/I-JSON inputs; reject duplicate keys, invalid Unicode, non-NFC strings, JSON numbers, unknown fields, and invalid ordering; serialize validated values as exact RFC 8785 JCS UTF-8 bytes with no BOM, insignificant whitespace, or trailing newline; compute lowercase `sha256:<64-hex>` over exact bytes; and contain identity rather than results.
- Freeze the `/v1` evidence contract through `contracts/readiness/v1/contract-lock.json`, the three strict schemas, and canonicalization vectors. Any locked-byte change requires `/v2`.
- Candidate Manifest artifacts are exactly `direct-app-zip`, `dmg`, `updater-archive`, `updater-metadata`, and `updater-signature`, calculated after signing, notarization, stapling, packaging, signature, and metadata generation.
- AD-8: Store Evidence Index records as strict LF-terminated JCS envelopes in a hash-chained, single-head, append-only ledger. A protected Release-owned Evidence Registrar using one allowlisted workflow identity is the sole append authority; producers submit immutable attempt bundles but cannot edit the index.
- The Registrar must enforce schema and canonical form, candidate/profile binding, exact source/environment/candidate provenance, subject/result byte hashes, sequence/predecessor, idempotency, lock/CAS, stale/fork rejection, write-once/no-clobber storage, human/machine agreement, retry linkage, and full retention through the audit period.
- AD-9: A source commit, tag, version, signing identity, artifact byte or name, metadata byte, rebuild, resign, retag, repackage, replacement, new release-build workflow run, or new release-build run attempt creates a new Candidate Manifest/evidence root and requires affected Batch 7/8 scenarios to rerun; evidence-collection retry against an unchanged candidate creates only a new linked record.
- AD-10: Preserve source-, environment-, and candidate-bound evidence depths. Association with a matching candidate never upgrades `bindingLevel`; provisioned-target-Mac evidence cannot be relabeled candidate-bound.
- AD-11: Packaged acceptance ends at the installed exact candidate. Browser, source, universal-header, no-sign, or workflow evidence may support diagnosis but cannot substitute for installed candidate acceptance.
- AD-12: Keep release-please and GitHub Actions as the release framework and transport, add a write-once staging/freeze hold point, fail candidate preparation closed when required credentials/artifacts are missing, and prevent promotion from replacing manifest-bound assets.
- AD-13: Preserve the exact dependency waves: Batch 1 first; Batches 2/3/4 may follow in parallel; Batches 5/6 require accepted Batch 4 and may run in parallel; release preparation follows accepted Batches 1–6 and freezes one fully signed/notarized/stapled candidate plus metadata; Batch 7 uses it; Batch 8 follows Batch 7 against it unchanged. Release preparation is not a ninth batch.
- AD-14: Preserve the map's `final-pending-approval` state, exactly 72 rows, historical FAIL/14-of-72 baseline, 14 mapped revalidations, RP separation, and fail-closed aggregation; architecture/planning does not approve the map or infer FULL/readiness.
- AD-15: Freeze one `pack-manager.criterion-acceptance-profile/v1` only after map approval, DR-1 resolution, and DR-4 approval. It covers all 72 criteria plus RP-1/RP-2 through required slots that fix concern, lane, binding level, versioned scenario contract, candidate subjects, environment matrix, and future approved retry policy.
- Exactly one first attempt exists per profile slot with `attempt.ordinal = "1"` and `attempt.runnerRetryCount = "0"`. A manually authorized retry is a new gapless linked record; branches, missing attempts, duplicate ordinals, and automatic retries fail closed. The first failure remains visible.
- PASS admission requires every scenario-required check to be collected, executed, and passed with failed, errored, skipped, ignored, cancelled, filtered, and unreported counts all zero. Wrong-lane, wrong-depth, wrong-source, wrong-candidate, incomplete-subject, or conflicting results fail closed.

#### Governance, Risks, and Entry Blockers

- PC-1: Correct stale source truth so `mas` is represented as supported/live-verified, synthetic fixtures are not correctness proof, obsolete notarization/ad-hoc-only claims are removed, and the obsolete five-event invariant does not absorb application-update state. Product correction precedes recurrence-test credit.
- GP-1: Freeze the denominator at 72 P0 rows and keep RP-1/RP-2 as mandatory external prerequisites; do not import unrelated P1 scope or the conflicting legacy strict-FULL policy.
- GP-2: Distinguish prospective policy change, incompatible criterion waiver, and risk acceptance. Any candidate-specific conditional release requires Product Owner, QA Lead, and Release Owner approval and must be labeled `CONDITIONAL — NOT 100% PRODUCT-AND-RELEASE READY`.
- DR-1 remains OPEN. Product and Release own the decision and must declare the minimum supported macOS version before TIR-7 or RE-4/RE-7/RE-8 environment implementation handoff. Until then, packaged compatibility, fresh-install and prior-version environments, final support copy, Acceptance Profile freeze, and affected story implementation entry remain blocked.
- DR-2 is APPROVED and binding: packaged keyboard/focus, automated 4.5:1 contrast, reduced motion, and manual VoiceOver focus-order/completion-announcement acceptance are required; this approval is not evidence.
- DR-3 is APPROVED and binding: physical Intel fresh-install, Finder/Dock launch, and prior-public-version update evidence is mandatory; universal-binary inspection cannot substitute; this approval is not evidence.
- DR-4 remains PROPOSED under Product/QA governance. Do not use the legacy 80% P1 threshold or generic 95%/80% defaults. Acceptance Profile retry values, candidate validation, gate configuration, and any readiness decision remain blocked.
- R-001 through R-008 remain open high risks: source/oracle drift (6), fake/native boundary gap (6), misleading UI state (6), process lifecycle uncertainty (6), persistence/diagnostics failure (6), updater integrity failure (6), invalid shipped artifact (9), and environmental dependency/contamination (6). No mitigation is complete, waived, or accepted through planning.
- Product and QA must approve and mechanically verify the 72-row map before it becomes a frozen oracle.
- Every affected implementation work item requires exactly one accountable role, one named assignee, and one calendar date. Where no assignment exists, record `Assignee: Unassigned` and `Calendar date: Unassigned`; implementation entry remains blocked.
- Release must choose a conforming evidence transport primitive and retention duration before release preparation while preserving protected Registrar identity, candidate/profile lock/CAS, idempotency, write-once/no-clobber objects, one head, complete-set retention, and audit availability.
- QA must secure and serialize a qualified provisioned target Mac, Apple-silicon and physical Intel hosts, disposable roots/helpers, and an actually installed prior public version by their batch boundaries.
- Release must provide current signing/notarization/updater credentials and one immutable candidate; missing credentials or required artifacts fail candidate preparation closed. Secrets remain in fnox/GitHub Secrets and never enter manifests or evidence.
- Every criterion-bearing story must state exact criterion IDs and historical status, one primary concern, batch or revalidation point, required test level, lane, minimum depth, dependencies, ASR/risk links, behavior-present handling where applicable, expected immutable evidence artifact, versioned scenario-contract path/digest, later-FULL-reassessment wording, one accountable role, explicit assignee/date fields, and first-attempt/zero-automatic-retry terms.
- Every candidate-bound story must name exact Candidate Manifest subject roles and specify that candidate mutation invalidates affected results and restarts the required Batch 7/8 slots.
- A plan, source inspection, collector, ignored test, green suite, workflow upload attempt, architecture approval, or this planning document is not evidence of criterion closure or readiness.

#### Implementation-Entry Blocker Register

| Decision or dependency                                           | Current state                       | Accountable/decision role                              | Deadline boundary                                                                                          | Blocked implementation entry                                                                                                                                                   |
| ---------------------------------------------------------------- | ----------------------------------- | ------------------------------------------------------ | ---------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| Product Behavior Prerequisite UX-PB.1..UX-PB.5                   | `APPROVED TARGET — NOT IMPLEMENTED` | Product/UX/Architecture accept; Development implements | Before affected Epic 3–7 evidence stories                                                                  | Any evidence or acceptance authored against immediate row execution, direct self-update execution, Activity drawer, Operation-row History, or active `autoOpenDrawer` behavior |
| Normative coverage-map approval                                  | `final-pending-approval`            | Product and QA                                         | Before the map is frozen as the implementation oracle or any Acceptance Profile slot is admitted           | Profile freeze and all criterion-evidence admission                                                                                                                            |
| DR-1 — minimum supported macOS                                   | `OPEN`                              | Product and Release                                    | Before TIR-7 or RE-4/RE-7/RE-8 environment implementation handoff                                          | Packaged OS matrix, compatibility slots, final support copy, profile freeze, fresh-install work, and prior-version work                                                        |
| DR-2 — packaged accessibility method                             | `APPROVED` and binding              | QA executes                                            | Candidate lane before Batch 7; complete evidence required in Batch 8                                       | No policy decision remains, but keyboard/focus, automated 4.5:1 contrast, reduced-motion, and manual VoiceOver execution are still unperformed                                 |
| DR-3 — physical Intel requirement                                | `APPROVED` and binding              | QA executes                                            | Physical Intel host before Batch 7; complete install/launch/update evidence required in Batch 8            | Candidate-lane qualification and physical Intel install, Finder/Dock launch, and prior-version update                                                                          |
| DR-4 — P0 gate/retry policy                                      | `PROPOSED`                          | Product and QA governance                              | Before retry values, candidate validation, gate configuration, profile freeze, or later readiness decision | Acceptance Profile policy fields and candidate evidence admission; no legacy P1 or generic policy may substitute                                                               |
| Named assignees and calendar dates                               | `UNASSIGNED — BLOCKER`              | Downstream planning                                    | Before each story enters implementation                                                                    | Every story whose `Assignee` and `Calendar date` remain `Unassigned`                                                                                                           |
| Native harness/test runner                                       | `DEFERRED`                          | Architecture accepts; Development implements           | Accepted by Batch 4 exit                                                                                   | ASR-01 native crossing and every dependent Batch 4–7 story                                                                                                                     |
| Controlled-helper implementation language                        | `DEFERRED`                          | Development                                            | Before Batch 5                                                                                             | ASR-02 process controls and dependent Batch 5–7 work                                                                                                                           |
| Evidence transport and retention duration                        | `DEFERRED — BLOCKER`                | Release                                                | Before release preparation                                                                                 | Protected sole-append Registrar, lock/CAS, one head, write-once objects, complete-set retention, and audit availability                                                        |
| Provisioned target Mac and versioned profile                     | `EXECUTION DEPENDENCY`              | QA                                                     | Before Batch 1 target-Mac collection                                                                       | Environment-bound topology and live-capture stories; no other lane may substitute                                                                                              |
| Apple-silicon and physical Intel hosts                           | `EXECUTION DEPENDENCY`              | QA                                                     | Before the candidate-release lane is operational                                                           | Packaged accessibility, fresh install, Finder/Dock launch, and updater execution                                                                                               |
| Actually installed prior public version                          | `EXECUTION DEPENDENCY`              | QA with Release support                                | Before Batch 7 updater execution                                                                           | Prior-version discovery, download, explicit install/relaunch, and refusal scenarios on both architectures                                                                      |
| Signing, notarization, and updater credentials                   | `EXECUTION DEPENDENCY`              | Release                                                | Before candidate freeze                                                                                    | Candidate preparation; secrets remain in fnox/GitHub Secrets and outside manifests/evidence                                                                                    |
| Evidence/profile approval records and versioned scenario digests | `EXECUTION DEPENDENCY`              | Product/QA for policy; QA for profile                  | Before Acceptance Profile freeze                                                                           | Every profile slot and all evidence admission                                                                                                                                  |
| One immutable candidate and all required artifacts               | `NOT YET AVAILABLE`                 | Release                                                | After accepted Epics 1–6 and before Batch 7                                                                | Candidate-release qualification, Batches 7–8, and later Trace eligibility                                                                                                      |

### UX Design Requirements

The finalized UX spines are authoritative for the update experience. They add
the following binding requirements:

- every Package and Manager update enters one persistent editable Upgrade Plan;
- the plan appears only when non-empty and persists across Manager navigation;
- final confirmation is a separate dialog whose opt-out exists only inside that
  dialog and is reversible in Settings;
- exactly one confirmed attempt may be active, with concurrency inside it;
- the sidecar transforms into Activity and then Results, while full Activity
  provides detailed evidence;
- History contains one immutable row per confirmed attempt and supports replay
  and linked Retry;
- success follows verification, not process exit alone;
- `Interaction required` needs a trusted closed Manager-specific classifier;
- navigation, high zoom, keyboard, focus, VoiceOver, Manager cards, Summary
  Cards, Package health, update-ready presentation, and error explanations
  follow `DESIGN.md`, `EXPERIENCE.md`, and `validation-report.md`.

### 2026-07-24 Correct Course story amendment

The Product Behavior Prerequisite below precedes affected evidence work.
Where older story text conflicts, these replacements are binding:

| Existing story area         | Superseding requirement                                                                                                                                                           |
| --------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Stories 3.1–3.3 and 3.5–3.6 | Package/Manager actions edit persistent plan membership; row actions never execute; Manager self-updates are individually removable; final confirmation is separate.              |
| Stories 3.4 and 6.7         | Replace active `autoOpenDrawer` behavior with `skipUpgradePlanConfirmation: false` by default; thresholds and log level remain editable and atomic.                               |
| Stories 4.1 and 4.6         | Add atomic boundary contracts for `PlanIntent`, one-use `planId`, durable `planAttemptId`, attempt queries/cancel/replay, event correlation, and plan-level native admission.     |
| Stories 5.2, 5.4, and 5.5   | Stage Manager updates, present shared plan Activity/Results, enforce one active attempt, use attempt-wide cancellation, and restrict Interaction required to trusted classifiers. |
| Stories 6.3–6.5 and 6.7     | Persist plan attempts, verification, Results, Retry lineage, and honest legacy Operations; diagnostics include the new correlation.                                               |
| Stories 7.6, 7.7, and 7.10  | Exercise finalized packaged navigation, plan/confirmation/Activity/Results, 150–200% zoom, VoiceOver/focus, and `Pack-Manager Update Ready!` presentation.                        |
| Story 8.7                   | Consume only the later approved revision-2 map/profile and preserve the superseded-evidence record.                                                                               |

`AUT-003` is retained as historical evidence of superseded behavior and must
not support revised `F5-AC3`.

### FR Coverage Map

This map assigns each FR exactly once to its primary epic for planning accountability. An epic may reference additional FRs as cross-cutting acceptance constraints without creating duplicate primary ownership. Exact P0 criterion ownership is governed separately by the 72-row story allocation.

FR-1: Epic 4 — Prove Manager detection and refresh through the shared production-native boundary.

FR-2: Epic 1 — Restore trustworthy Manager-reported truth using the corrected live `mas` oracle.

FR-3: Epic 2 — Preserve independent refresh, failure isolation, coalescing, and affected-state recovery.

FR-4: Epic 5 — Prove dynamic Manager ownership and update Route selection.

FR-5: Epic 3 — Present complete, Manager-specific Package state and eligibility.

FR-6: Epic 3 — Preserve exact eligible Package selection.

FR-7: Epic 3 — Preview every bulk command and exclusion before authorization.

FR-8: Epic 3 — Reject stale, altered, replayed, or otherwise invalid plans.

FR-9: Epic 5 — Admit multi-group work atomically and preserve scheduler protections.

FR-10: Epic 3 — Support intentional, bounded single-Package updates.

FR-11: Epic 5 — Explain and execute Manager self-update Routes safely.

FR-12: Epic 5 — Preserve structured execution, null stdin, and the no-shell/no-privilege boundary.

FR-13: Epic 5 — Expose correlated live Operation state and output.

FR-14: Epic 5 — Handle stalls, cancellation, timeout, and shutdown honestly.

FR-15: Epic 6 — Preserve reconstructible History, transcripts, journals, and crash evidence.

FR-16: Epic 2 — Preserve useful Manager state and actionable recovery after outcomes.

FR-17: Epic 3 — Expose and validate user-controlled Settings; Epic 6 supplies the cross-cutting persistence acceptance.

FR-18: Epic 6 — Export privacy-preserving diagnostics through native filesystem boundaries.

FR-19: Epic 7 — Validate the coherent accessible interface in the installed packaged application.

FR-20: Epic 7 — Validate application-update discovery and background download.

FR-21: Epic 7 — Validate explicit install/relaunch, active-operation refusal, and non-writable behavior.

FR-22: Epic 8 — Attest normal packaged launch and authorized, coherent release/update artifacts.

RP-1: Epic 7 with final Epic 8 association — Validate scheduled/menu update triggers and state continuity outside the denominator.

RP-2: Epic 7 with final Epic 8 association — Validate standard macOS Edit/Window menu behavior outside the denominator.

## Epic List

The eight epics are dependency-ordered closure outcomes required by the finalized planning authorities. Each epic completes one coherent user-confidence outcome and produces accepted foundations for later epics without relying on future work to complete its own domain. No epic, infrastructure result, or evidence plan changes a criterion status.

### Product Behavior Prerequisite: Finalize the Upgrade Plan lifecycle

This prerequisite is approved product work, not evidence work and not a ninth
closure batch. It must complete before affected Stories in Epics 3–7 can enter
evidence implementation. Its completion does not change any readiness status;
TIR-1 behavior-present reconciliation must follow.

#### Story UX-PB.1: Make every update persistent Upgrade Plan membership

**Primary concern:** Product Behavior  
**Dependencies:** Decisions D27-D30; finalized UX spines; AD-16  
**Blocks:** Stories 3.1–3.3, 3.5–3.6, 5.2, and their affected evidence  
**Assignee:** Unassigned  
**Calendar date:** Unassigned

As a Pack-Manager user, I want every Package and Manager update action to edit
one persistent plan so that nothing bypasses the visual organization layer.

**Acceptance Criteria:**

- [ ] Package-row, selected-Package, Manager-header, Manager-wide, and
      `Update Everything` actions add eligible canonical identities to one
      draft and never execute.
- [ ] The sidecar is hidden when empty, appears after the first addition,
      persists across Manager navigation, and closes after the last removal.
- [ ] Manager updates are independent removable items; no global
      `includeSelfUpdates` control remains.
- [ ] Pinned, current, excluded, and unavailable Packages remain inert while
      keyboard, pointer, and VoiceOver interaction explains why.
- [ ] The Manager workspace uses standardized description, path, version,
      Manager-status, ownership, Package counts, and update delta presentation.
- [ ] Rust rebuilds the plan from canonical intent after each draft change;
      executable display text is never trusted input.

#### Story UX-PB.2: Establish durable confirmed Plan Attempts

**Primary concern:** Product Behavior  
**Dependencies:** Story UX-PB.1; AD-3; AD-16  
**Blocks:** Stories 4.1, 4.6, 6.3, 6.4, and their affected evidence  
**Assignee:** Unassigned  
**Calendar date:** Unassigned

As a user, I want each confirmed plan to have one durable identity so that its
Operations, verification, Results, and recovery remain reconstructible.

**Acceptance Criteria:**

- [ ] One-use preview `planId` and durable `planAttemptId` are distinct wire,
      domain, persistence, and TypeScript types.
- [ ] Confirmation atomically returns a new `planAttemptId` and the admitted
      Operation identities; a second active confirmation fails closed.
- [ ] Operations, events, transcripts, journal records, diagnostics, and
      verification refreshes carry `planAttemptId` where applicable.
- [ ] Attempt persistence stores reviewed intent and exact command snapshot
      without converting historical display strings back into executable input.
- [ ] Plan-level cancellation marks unstarted work `Skipped`, escalates running
      process groups through existing mechanics, and preserves every outcome.
- [ ] Legacy Operations without attempt identity remain honest legacy records
      and are never inferred into plan groups.

#### Story UX-PB.3: Transform the sidecar into trusted Activity and Results

**Primary concern:** Product Behavior  
**Dependencies:** Stories UX-PB.1 and UX-PB.2  
**Blocks:** Stories 5.4, 5.5, 6.5, 7.6, and their affected evidence  
**Assignee:** Unassigned  
**Calendar date:** Unassigned

As a user, I want the same plan to show live progress and verified Results so
that I understand what is happening without reading a terminal.

**Acceptance Criteria:**

- [ ] The confirmed sidecar becomes the one active plan summary; full Activity
      is a first-class detailed view of the same state.
- [ ] Per-item state shows queued/waiting/running/verifying/terminal outcomes;
      verified completion replaces the old-to-new delta with the single new
      version.
- [ ] Terminal Results wait for required refresh verification and distinguish
      mutation failure, verification failure, cancelled, skipped, timed out,
      and interrupted work.
- [ ] Known failures explain what happened and a safe next step before
      presenting Retry; deterministic causes are not framed as likely fixed by
      repeated retries.
- [ ] `Interaction required` comes only from a closed Manager-specific
      classifier or explicit native signal; unmatched silence remains a stall.
- [ ] Primary cancellation is labeled `Cancel plan`; `Cancel operation` is used
      only for an explicitly Operation-scoped diagnostic action.

#### Story UX-PB.4: Make History and Retry plan-attempt based

**Primary concern:** Product Behavior  
**Dependencies:** Stories UX-PB.2 and UX-PB.3  
**Blocks:** Stories 6.3–6.5, 8.7, and their affected evidence  
**Assignee:** Unassigned  
**Calendar date:** Unassigned

As a user, I want one History entry per plan I confirmed so that I can replay
the complete outcome instead of piecing together unrelated commands.

**Acceptance Criteria:**

- [ ] Every confirmed attempt creates exactly one immutable History row,
      including failed, cancelled, interrupted, or partially skipped attempts.
- [ ] Opening a row routes Activity into read-only replay with Manager groups,
      Package/version changes, commands, Operation outcomes, errors, timings,
      and retained output.
- [ ] A concurrent live attempt remains clearly primary and offers
      `Back to live activity`.
- [ ] Retry first exposes the proposed failed-item scope, rebuilds current
      intent, and creates a new attempt linked by `retryOfPlanAttemptId`.
- [ ] The original failed result remains immutable and visible after Retry.
- [ ] Legacy Operation History remains accessible and explicitly labeled
      without fabricated plan grouping.

#### Story UX-PB.5: Deliver the separate confirmation and finalized safety settings

**Primary concern:** Product Behavior  
**Dependencies:** Stories UX-PB.1 and UX-PB.2; finalized focus and high-zoom contracts  
**Blocks:** Stories 3.4, 6.7, 7.6, 7.7, 7.10, and their affected evidence  
**Assignee:** Unassigned  
**Calendar date:** Unassigned

As a user, I want a clear final confirmation that I can deliberately disable
and restore so that the default stays safe without forcing permanent friction.

**Acceptance Criteria:**

- [ ] With confirmation enabled, the plan contains one blue
      `Confirm N Updates` action and no safety checkbox.
- [ ] That action opens `Proceed with Upgrade Plan?`, dims the background,
      shows exact commands, and provides `Change Plan` and
      `Confirm N Updates`.
- [ ] Only the dialog contains
      `Disable upgrade plan command execution confirmation`, its safety
      explanation, and Settings restoration guidance.
- [ ] `skipUpgradePlanConfirmation` defaults to `false`, persists atomically,
      and is reversible in Settings; `autoOpenDrawer` is inactive legacy input.
- [ ] With confirmation disabled, command display expands and the explicit run
      action still performs native rebuild/stale validation.
- [ ] Focus trapping/restoration, VoiceOver names/announcements, reduced
      motion, 900 × 600, 150%, and 200% zoom keep every safety action reachable.
- [ ] The application updater uses `Pack-Manager Update Ready!` and remains
      separate from Package Upgrade Plans and History.

### Epic 1: Restore Trustworthy `mas` and Target-Mac Truth

Users and downstream acceptance work can rely on current, live-verified `mas` behavior and a dated six-Manager target-Mac oracle without stale unverified claims or synthetic-fixture substitution.

**Primary FR ownership:** FR-2  
**Cross-cutting FRs:** FR-1  
**Primary readiness concerns:** Product Behavior for `D23a-AC4`; Reusable Test Infrastructure for the remaining rows  
**Open P0 allocation (5):** `F1-AC7`, `D23a-AC1`, `D23a-AC2`, `D23a-AC4`, `D23a-AC5`  
**Historical-FULL revalidation (1):** `D23a-AC3` at `RV@B1`  
**Required enablers and risks:** ASR-05 (QA accountable; CI execution mechanism), PC-1, TIR-1/TIR-2/TIR-6/TIR-8, R-001, R-008  
**Dependency and exit boundary:** Runs first. ASR-05 lane separation is accepted before any evidence collection. Product/source correction precedes recurrence-test credit. Exit retains real-capture provenance and serialized, dated provisioned-target-Mac topology.

### Epic 2: Make Detection and Refresh Fail Independently and Recover Usefully

Users can detect and refresh every supported Manager with clear phase, absence, timeout, offline, and failure behavior while peer Managers and Last-good Snapshots remain usable.

**Primary FR ownership:** FR-3, FR-16  
**Cross-cutting FRs:** FR-1, FR-2, FR-17  
**Primary readiness concern:** Reusable Test Infrastructure  
**Open P0 allocation (5):** `F1-AC6`, `F1-AC8`, `F2-AC3`, `F2-AC6`, `F2-AC8`  
**Historical-FULL revalidation (4):** `F1-AC5`, `F2-AC2`, `F2-AC5`, `F2-AC7` at `RV@B2`  
**Required enablers and risks:** Accepted Epic 1 truth, ASR-05, TIR-1/TIR-2/TIR-8, R-001, R-003, R-008  
**Dependency and exit boundary:** Follows Epic 1 and may proceed in parallel with Epics 3 and 4. Every `BP` row receives behavior-present handling before regression work. Forced-offline results remain at their mapped source depth and cannot borrow target-Mac evidence.

### Epic 3: Keep Package Choice, Plans, and Settings Exact and Understandable

Users can understand Package state, select only eligible work, review exact commands and exclusions, reject stale plans, perform bounded row-level updates, and control Settings without misleading or inaccessible UI state.

**Primary FR ownership:** FR-5, FR-6, FR-7, FR-8, FR-10, FR-17  
**Cross-cutting FRs:** FR-2, FR-11, FR-19  
**Primary readiness concern:** Reusable Test Infrastructure  
**Open P0 allocation (11):** `F3-AC1`, `F3-AC2`, `F3-AC3`, `F3-AC4`, `F3-AC6`, `F3-AC8`, `F4-AC1`, `F5-AC1`, `F5-AC3`, `F11-AC2`, `F11-AC3`  
**Historical-FULL revalidation (3):** `F3-AC5`, `F3-AC7`, `F4-AC3` at `RV@B3`  
**Required enablers and risks:** Accepted Epic 1 truth, ASR-05, TIR-1/TIR-2/TIR-8, R-003, R-008  
**Dependency and exit boundary:** Follows Epic 1 and may proceed in parallel with Epics 2 and 4. Behavior-present checks precede test-only work. Exit covers complete Package state, keyboard selection, plan entry/failure, clipboard, Settings controls, and historical source-level revalidation without claiming packaged accessibility.

### Epic 4: Prove the Real Desktop Command-and-Event Boundary

Users gain confidence that the desktop application they operate crosses the same frontend invocation, Tauri registration/serialization, Rust handlers, and event channels that production ships, rather than relying on separately passing fake-browser and handler-only suites.

**Primary FR ownership:** FR-1  
**Cross-cutting FRs:** FR-3, FR-6, FR-7, FR-8, FR-9, FR-12  
**Primary readiness concern:** Reusable Test Infrastructure  
**Open P0 allocation (5):** `F1-AC1`, `F1-AC2`, `F1-AC3`, `F1-AC4`, `F2-AC1`  
**Historical-FULL revalidation (4):** `F2-AC4`, `F4-AC2`, `F5-AC2`, `F12-AC3` at `RV@B4`  
**Required enablers and risks:** ASR-01 (Architecture accountable; Development/QA implement and use), ASR-02 foundations (Development accountable; Platform capability area), ASR-05, TIR-3/TIR-4/TIR-8, R-002, R-008  
**Dependency and exit boundary:** Follows Epic 1 and may proceed in parallel with Epics 2 and 3. By exit, the versioned boundary catalog, production registration, Rust/TypeScript contracts, wrappers/subscriptions, fixtures, inventory, and native vectors have exact set equality; every catalog command round-trips and every event dispatches through one isolated real production boundary. The current 20 commands and six events remain a baseline, not fixed counts. Accepted Epic 4 is mandatory before Epics 5 and 6 and before candidate-era Epic 7.

### Epic 5: Make Manager Updates and Process Lifecycles Safe and Honest

Users can trust dynamic Manager update Routes, scheduler locks, exact live output, stall/timeout choices, cancellation, shutdown, null-input behavior, and the closed D26 transcript repair through controlled native process evidence.

**Primary FR ownership:** FR-4, FR-9, FR-11, FR-12, FR-13, FR-14  
**Cross-cutting FRs:** FR-3, FR-15, FR-16  
**Primary readiness concern:** Reusable Test Infrastructure  
**Open P0 allocation (12):** `F2-AC9`, `F6-AC1`, `F6-AC2`, `F6-AC3`, `F6-AC4`, `F6-AC5`, `F7-AC1`, `F7-AC2`, `F7-AC3`, `F7-AC4`, `D26-AC1`, `D26-AC2`  
**Historical-FULL revalidation (2):** `F4-AC4`, `F4-AC5` at `RV@B5`  
**Required enablers and risks:** Accepted Epic 4, ASR-02 core accepted before entry, ASR-05, TIR-1/TIR-3/TIR-4/TIR-8, R-003, R-004, R-008  
**Dependency and exit boundary:** Requires accepted Epic 4 and may proceed in parallel with Epic 6. Exit proves Route selection, spawn/no-spawn behavior, complete lock timelines, native events/output boundaries, null stdin, no password/admin path, cancellation/escalation, stall, timeout, shutdown, affected-state refresh, and exact D26 positive/negative boundaries.

### Epic 6: Preserve State, Evidence, and Privacy Across Failure and Relaunch

Users can reconstruct Operations after crashes, trust Settings and durable stores across failure, reveal native evidence safely, and export exact diagnostics without inherited-environment disclosure or hostile-path traversal.

**Primary FR ownership:** FR-15, FR-18  
**Cross-cutting FRs:** FR-16, FR-17  
**Primary readiness concern:** Reusable Test Infrastructure  
**Open P0 allocation (10):** `F8-AC1`, `F8-AC2`, `F8-AC3`, `F8-AC4`, `F9-AC1`, `F9-AC2`, `F9-AC3`, `F9-AC4`, `F11-AC1`, `F11-AC4`  
**Historical-FULL revalidation:** None assigned to this checkpoint; final association still requires the frozen Acceptance Profile  
**Required enablers and risks:** Accepted Epic 4, ASR-02 filesystem extensions (Development accountable), ASR-03 (QA accountable; Development/Platform support), ASR-05, TIR-1/TIR-4/TIR-5/TIR-8, R-004, R-005, R-008  
**Dependency and exit boundary:** Requires accepted Epic 4 and may proceed in parallel with Epic 5. ASR-03 is accepted before criterion work enters this epic. Exit uses disposable roots to prove transcript/journal/history lifecycle, Interrupted reconstruction, historical-PGID non-signal, Settings atomicity, retention, diagnostics contents/privacy, hostile filesystem cases, and native opener/export outcomes without contacting operator data or processes.

### Release Preparation Prerequisite — Freeze One Immutable Candidate

This prerequisite begins only after the required exits of Epics 1–6 are accepted. It is not an epic, not Batch 9, and owns zero denominator rows.

Release, accountable through ASR-04, accepts the locked `/v1` evidence contract and conforming protected Registrar/transport/retention design; QA freezes the eligible Criterion Acceptance Profile only after map approval, DR-1 resolution, and DR-4 approval; and one clean GitHub Actions build attempt freezes the fully packaged, signed, notarized, and stapled app, DMG, ZIP, updater archive/signature, updater metadata, and identity-only Candidate Manifest. Missing credentials, required hosts, prior version, policy/profile inputs, or required artifacts fail preparation closed.

Any change to source, tag, version, signing identity, artifact bytes/names, metadata bytes, build workflow run, or build run attempt creates a new Candidate Manifest and evidence root. Release preparation must be repeated before candidate-bound work continues.

### Epic 7: Validate the Installed Accessible App and Explicit Updater Journey

Users can operate the exact installed packaged application accessibly and can update from an actually installed prior public version to the frozen candidate through an authorized background download and explicit Restart to update, with active-operation refusal and no privilege escalation.

**Primary FR ownership:** FR-19, FR-20, FR-21  
**Cross-cutting FRs:** FR-12, FR-22, RP-1, RP-2  
**Primary readiness concern:** Reusable Test Infrastructure producing candidate-bound packaged acceptance  
**Open P0 allocation (4):** `F10-AC1`, `D25-AC2`, `D25-AC3`, `D25-AC4`  
**Historical-FULL revalidation:** None newly assigned; all earlier revalidation results must remain eligible for the same frozen profile/source  
**Release Prerequisites:** RP-1 and RP-2 receive separate candidate-bound profile slots and do not enter the denominator  
**Required enablers and risks:** Accepted Epics 1–6, completed release preparation, ASR-01, ASR-02 updater controls, ASR-04 frozen manifest, ASR-05 candidate lane, TIR-7/TIR-8, RE-1/RE-6/RE-8/RE-9, approved DR-2, approved DR-3, resolved DR-1, R-003, R-006, R-007, R-008  
**Dependency and exit boundary:** Uses only the frozen Candidate Manifest. Runs packaged-WKWebView keyboard/focus/4.5:1 contrast/reduced-motion automation and manual VoiceOver acceptance; validates application-update state, prior-version download/signature/explicit install/relaunch, active Package Operation refusal, non-writable manual-install-required, Apple-silicon and physical-Intel execution, and RP menu/state continuity. Candidate mutation invalidates affected results and returns work to release preparation.

### Epic 8: Attest the Unchanged Release and Produce a Reproducible Evidence Handoff

Users and release decision-makers can identify, install, launch, and audit one unchanged, complete, trusted Pack-Manager candidate across both promised architectures, with exact source-quality, artifact, provenance, and append-only evidence suitable for a later independent Trace decision.

**Primary FR ownership:** FR-22  
**Cross-cutting FRs:** FR-19, FR-20, FR-21, RP-1, RP-2  
**Primary readiness concern:** Candidate-Specific Release Evidence, except `F12-AC1`/`F12-AC2`, which retain forced-offline source depth and are only associated when source/profile match  
**Open P0 allocation (6):** `F10-AC2`, `F10-AC3`, `F10-AC4`, `F12-AC1`, `F12-AC2`, `D25A-AC2`  
**Historical-FULL revalidation boundary:** All 14 earlier checkpoint results must be admitted at their mapped lane/depth; none is relabeled or carried forward  
**Required enablers and risks:** Accepted Epic 7 against the same manifest, ASR-04 complete ledger, ASR-05 lane enforcement, TIR-2/TIR-8, RE-1 through RE-11 as applicable, GP-1/GP-2, resolved DR-1, approved DR-4 policy/profile, R-006, R-007, R-008  
**Dependency and exit boundary:** Follows Epic 7 against the unchanged candidate. Attests icon/resources/entitlements/architectures, Developer ID signatures, notarization, stapling, Gatekeeper, complete coherent assets, metadata/URLs/signature/key/version agreement, Apple-silicon and physical-Intel fresh install and Finder/Dock launch, retained first-run forced-offline quality output, and valid single-head Evidence Index replay. Exit means only “eligible to invoke the later candidate-bound Trace workflow”; it does not regenerate traceability, move a criterion to FULL, or claim readiness.

### Dependency Summary

1. Product Behavior Prerequisite UX-PB.1–UX-PB.5 implements the approved
   update-experience target before affected evidence stories in Epics 3–7.
2. TIR-1 behavior-present reconciliation follows that implementation; it does
   not automatically change readiness status.
3. Epic 1 runs first for the original closure dependency chain.
4. Epics 2, 3, and 4 may follow Epic 1 in parallel only where their affected
   stories also satisfy the Product Behavior Prerequisite.
5. Epics 5 and 6 require accepted Epic 4 and the applicable UX-PB stories and
   may proceed in parallel.
6. Release preparation follows accepted required exits from Epics 1–6 and is
   not a ninth epic.
7. Epic 7 uses the one frozen immutable candidate after the revised UX
   acceptance contracts are admitted.
8. Epic 8 follows Epic 7 against that unchanged candidate.
9. A later, separately invoked Trace workflow may assess the complete Evidence
   Set; no epic performs that decision.

## Epic 1: Restore Trustworthy `mas` and Target-Mac Truth

Users and downstream acceptance work can rely on current, live-verified `mas` behavior and a dated six-Manager target-Mac oracle without stale unverified claims or synthetic-fixture substitution.

### Story 1.1: Restore Current `mas` and Release Truth

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

## Epic 2: Make Detection and Refresh Fail Independently and Recover Usefully

Users can detect and refresh every supported Manager with clear phase, absence, timeout, offline, and failure behavior while peer Managers and Last-good Snapshots remain usable.

### Story 2.1: Preserve Honest Absence and Complete Environment Evidence

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
**Then** ToolEnv source/path and each Manager's path, version when available, managed-by state, evidence, and install hint are represented coherently
**And** Copy reports both success and actionable failure.

**Given** either behavior-present check fails
**When** the story result is classified
**Then** Product Behavior work is required before regression credit
**And** a valid admitted first attempt makes both criteria only **eligible for later FULL reassessment**.

### Story 2.2: Prove Refresh Phases and Per-Manager Timeouts

As a Pack-Manager user,
I want refresh settings, phases, and timeouts to behave consistently per Manager,
So that a slow or disabled step never creates misleading global state.

**Story Contract:**

- Criteria and historical baseline: `F2-AC3` — `UNIT-ONLY`; `F2-AC6` — `PARTIAL`
- FR and requirement links: FR-3; FR-17; TIR-2
- Primary readiness concern: Reusable Test Infrastructure
- Checkpoint: Batch 2
- Required test level: Unit plus component
- Execution lane / evidence depth: `forced-offline` / source-bound
- Dependencies: Accepted Epic 1; deterministic adapters and fake time; qualified ASR-05 lane
- ASR and risk links: ASR-05, TIR-1, TIR-2, TIR-8; R-003, R-008
- Behavior-present handling: Both criteria are `BP`; missing or incorrect phase/timeout behavior creates Product Behavior work before test credit
- Versioned scenario contract: `contracts/readiness/scenarios/v1/b2-refresh-phases-timeouts.json`
- Scenario-contract digest: Unassigned; freeze the exact file before implementation entry
- Expected evidence artifact: `b2-refresh-phases-timeouts.json` containing every adapter case, phase sequence, timeout boundary, and visible result
- Accountable role: Development
- Assignee: Unassigned
- Calendar date: Unassigned
- Implementation entry: Blocked by assignee/date and common profile/admission prerequisites
- Candidate subjects and invalidation: Not applicable
- Attempt contract: Complete first-attempt output is retained; `runnerRetryCount = "0"`

**Acceptance Criteria:**

**Given** Homebrew metadata refresh is enabled or disabled
**When** a Brew refresh is planned and rendered
**Then** the enabled path shows the required update/inventory/outdated phase order
**And** the disabled path omits only the metadata-update phase without mislabeling later phases.

**Given** each of the six Manager adapters and its documented timeout boundary
**When** controlled time reaches success, timeout, or error outcomes
**Then** the correct Manager-specific terminal state and actionable detail appear
**And** peers continue independently without real network access or wall-clock sleeps.

**Given** all scenario-required cases execute on the first attempt
**When** source-bound results are admitted
**Then** skipped, ignored, unexecuted, or automatically retried cases fail closed
**And** both criteria become only **eligible for later FULL reassessment**.

### Story 2.3: Keep Offline Failures Isolated

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

## Epic 3: Keep Package Choice, Plans, and Settings Exact and Understandable

Users can understand Package state, select only eligible work, review exact commands and exclusions, reject stale plans, perform bounded row-level updates, and control Settings without misleading or inaccessible UI state.

### Story 3.1: Present Complete Package State and Manager Detail

As a Pack-Manager user,
I want complete Package rows, expandable details, self-update separation, and non-color status cues,
So that I can understand what each Manager reports without losing Manager-specific meaning.

**Story Contract:**

- Criteria and historical baseline: `F3-AC1` — `PARTIAL`; `F3-AC2` — `PARTIAL`; `F3-AC6` — `PARTIAL`; `F3-AC8` — `PARTIAL`
- FR and requirement links: FR-2; FR-5; FR-6; FR-10; FR-11; FR-19; TIR-2
- Primary readiness concern: Reusable Test Infrastructure
- Checkpoint: Batch 3
- Required test level: Component
- Execution lane / evidence depth: `forced-offline` / source-bound
- Dependencies: Accepted Epic 1; representative all-state fixtures; qualified ASR-05 lane
- ASR and risk links: ASR-05, TIR-1, TIR-2, TIR-8; R-003, R-008
- Behavior-present handling: All four rows are `BP`; any absent/incorrect state creates Product Behavior work and a reviewed map revision before regression credit
- Versioned scenario contract: `contracts/readiness/scenarios/v1/b3-package-state-detail.json`
- Scenario-contract digest: Unassigned; freeze the exact file before implementation entry
- Expected evidence artifact: `b3-package-state-detail.json` with rendered roles/text, row-state matrix, expansion/search outcomes, and self-card/table assertions
- Accountable role: Development
- Assignee: Unassigned
- Calendar date: Unassigned
- Implementation entry: Blocked by assignee/date and common profile/admission prerequisites
- Candidate subjects and invalidation: Not applicable; source-bound UI evidence cannot satisfy packaged acceptance
- Attempt contract: Preserve ordinal 1; `runnerRetryCount = "0"`; later attempts stay linked

**Acceptance Criteria:**

**Given** representative current, Outdated, pinned, self-updating/greedy, unknown-version, and error Packages
**When** Manager Package tables render
**Then** name, installed/latest values, status text, eligibility, selection, and row action are complete and understandable without relying on color.

**Given** uv executable details and npm's own outdated row
**When** the user expands/searches uv content and views npm
**Then** uv executables are reachable and searchable
**And** npm self state appears only on its Manager card while the four ordinary Package rows remain.

**Given** any behavior-present assertion fails
**When** classification occurs
**Then** Product Behavior work precedes test credit
**And** a complete admitted first attempt makes all four criteria only **eligible for later FULL reassessment**.

### Story 3.2: Enforce Pinned and Greedy Eligibility

As a Pack-Manager user,
I want pinned formulae and self-updating casks handled by their documented policies,
So that no plan silently overrides a pin or includes default-excluded work.

**Story Contract:**

- Criteria and historical baseline: `F3-AC3` — `PARTIAL`; `F3-AC4` — `PARTIAL`
- FR and requirement links: FR-5; FR-6; FR-7; TIR-2
- Primary readiness concern: Reusable Test Infrastructure
- Checkpoint: Batch 3
- Required test level: Unit plus component
- Execution lane / evidence depth: `forced-offline` / source-bound
- Dependencies: Story 3.1; deterministic plan-builder and UI fixtures
- ASR and risk links: ASR-05, TIR-1, TIR-2, TIR-8; R-003, R-008
- Behavior-present handling: Both criteria are `BP`; missing/incorrect policy creates Product Behavior work before regression credit
- Versioned scenario contract: `contracts/readiness/scenarios/v1/b3-pinned-greedy-eligibility.json`
- Scenario-contract digest: Unassigned; freeze the exact file before implementation entry
- Expected evidence artifact: `b3-pinned-greedy-eligibility.json` with eligibility sets, every plan-entry path, disclosure text, default/opt-in outcomes, and exclusions
- Accountable role: Development
- Assignee: Unassigned
- Calendar date: Unassigned
- Implementation entry: Blocked by assignee/date and common admission prerequisites
- Candidate subjects and invalidation: Not applicable
- Attempt contract: Retain complete first-attempt results; `runnerRetryCount = "0"`

**Acceptance Criteria:**

**Given** pinned Homebrew formulae
**When** selection, row update, per-Manager update-all, update-selected, and Update Everything paths are exercised
**Then** pinned rows are explained, disabled, and excluded from every plan with the correct reason.

**Given** ordinary and greedy-only casks
**When** the default and explicit opt-in flows execute
**Then** greedy-only casks are the documented set difference, remain separate/collapsed/default-excluded, and enter a plan only through explicit opt-in with visible disclosure.

**Given** all required paths pass on the first attempt
**When** admission evaluates the result
**Then** both criteria become only **eligible for later FULL reassessment**.

### Story 3.3: Build Plans from Every User Entry Point

As a Pack-Manager user,
I want global and per-Manager update actions to open an exact plan or explain why planning failed,
So that no bulk work starts without a reviewable command set.

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
**When** Update Everything or per-Manager update-all is selected
**Then** the exact eligible request is built, the Upgrade Plan opens, and commands, exclusions, notes, warnings, and Manager self-update defaults are visible before authorization.

**Given** plan construction fails or inputs are stale
**When** the user initiates either entry path
**Then** no Operation is admitted, the failure is visible and actionable, and the interface cannot present a confirmable stale plan as current.

**Given** the scenario passes without ignored/unexecuted checks
**When** the result is admitted
**Then** `F4-AC1` becomes only **eligible for later FULL reassessment**.

### Story 3.4: Validate Every Settings Control and Environment Report

As a Pack-Manager user,
I want every Settings control and Environment Report action to validate, persist, and report failures clearly,
So that configuration changes and environment evidence remain trustworthy.

**Story Contract:**

- Criteria and historical baseline: `F11-AC2` — `PARTIAL`; `F11-AC3` — `PARTIAL`
- FR and requirement links: FR-17; TIR-2
- Primary readiness concern: Reusable Test Infrastructure
- Checkpoint: Batch 3
- Required test level: Unit plus component
- Execution lane / evidence depth: `forced-offline` / source-bound
- Dependencies: Accepted Epic 1 truth; controlled persistence and clipboard seams
- ASR and risk links: ASR-05, TIR-1, TIR-2, TIR-8; R-003, R-005, R-008
- Behavior-present handling: Both criteria are `BP`; missing/incorrect control or report behavior creates Product Behavior work before regression credit
- Versioned scenario contract: `contracts/readiness/scenarios/v1/b3-settings-environment-report.json`
- Scenario-contract digest: Unassigned; freeze the exact file before implementation entry
- Expected evidence artifact: `b3-settings-environment-report.json` with all controls/defaults/bounds, persistence outcomes, live log-level result, complete report, and clipboard outcomes
- Accountable role: Development
- Assignee: Unassigned
- Calendar date: Unassigned
- Implementation entry: Blocked by assignee/date and common admission prerequisites
- Candidate subjects and invalidation: Not applicable
- Attempt contract: Retain first-attempt human/machine results; `runnerRetryCount = "0"`

**Acceptance Criteria:**

**Given** every configurable Setting, its default, valid bounds, invalid input, and a persistence failure
**When** the user changes Settings
**Then** valid values persist before becoming active, invalid values are rejected, save failure changes neither active nor persisted state, and log-level changes apply live only after persistence.

**Given** the complete current detection and ToolEnv state
**When** Environment Report opens and Copy is used
**Then** every required field and evidence value is present
**And** copy success and failure are visible and actionable.

**Given** all behavior-present checks pass
**When** the source-bound attempt is admitted
**Then** both criteria become only **eligible for later FULL reassessment**.

### Story 3.5: Preserve Exact Keyboard Selection and Row Updates

As a Pack-Manager user,
I want keyboard selection and direct row updates to preserve exact Package identity,
So that I can act efficiently without updating excluded or unrelated Packages.

**Story Contract:**

- Criteria and historical baseline: `F5-AC1` — `PARTIAL`; `F5-AC3` — `PARTIAL`
- FR and requirement links: FR-6; FR-10; FR-13; FR-19; TIR-2
- Primary readiness concern: Reusable Test Infrastructure
- Checkpoint: Batch 3
- Required test level: Component plus browser E2E
- Execution lane / evidence depth: `forced-offline` / source-bound
- Dependencies: Stories 3.1–3.3; semantic keyboard/focus locators; deterministic bridge
- ASR and risk links: ASR-05, TIR-1, TIR-2, TIR-8; R-003, R-008
- Behavior-present handling: Both criteria are `BP`; missing/incorrect selection or row-update behavior creates Product Behavior work before regression credit
- Versioned scenario contract: `contracts/readiness/scenarios/v1/b3-selection-row-update.json`
- Scenario-contract digest: Unassigned; freeze the exact file before implementation entry
- Expected evidence artifact: `b3-selection-row-update.json` with selected PackageRefs, keyboard/focus state, plan/execute calls, command visibility, and both rejection paths
- Accountable role: Development
- Assignee: Unassigned
- Calendar date: Unassigned
- Implementation entry: Blocked by assignee/date and common admission prerequisites
- Candidate subjects and invalidation: Not applicable; browser evidence is not packaged evidence
- Attempt contract: Preserve first attempt; `runnerRetryCount = "0"` disables test-runner and workflow retries

**Acceptance Criteria:**

**Given** eligible, current, pinned, greedy, filtered, and range-addressable rows
**When** toggle, shift-range, tri-state, Cmd+A, Space, Cmd-click, Clear, and Esc interactions execute
**Then** the exact selectable identities and visible filter semantics are preserved
**And** excluded rows never enter selection.

**Given** one eligible or ineligible Package row
**When** direct Upgrade is invoked
**Then** exactly one eligible Package is submitted and its command becomes visible
**And** build and execute rejection are visible, enqueue nothing, and do not expand the selection.

**Given** all required interactions execute and pass
**When** the result is admitted
**Then** both criteria become only **eligible for later FULL reassessment**.

### Story 3.6: Revalidate Version Truth, mise Consequences, and Plan Defaults

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
**And** Manager self-updates default on, greedy casks default off, rust dedup/explanations hold, stale rebuild requires reconfirmation, and confirm-current behavior matches the source contract.

**Given** the historical FULL statuses
**When** the current first attempt passes and is admitted
**Then** none is carried forward automatically
**And** all three criteria become only **eligible for later FULL reassessment**.

## Epic 4: Prove the Real Desktop Command-and-Event Boundary

Users gain confidence that the desktop application they operate crosses the same frontend invocation, Tauri registration/serialization, Rust handlers, and event channels that production ships, rather than relying on separately passing fake-browser and handler-only suites.

### Story 4.1: Establish the Versioned Production Boundary Contract

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

### Story 4.2: Deliver the Deterministic Process-Control Core

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

As a QA Lead,
I want exact reviewed plans and PackageRefs revalidated through native serialization and admission,
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
- Expected evidence artifact: `rv-b4-plan-admission.json` with reviewed command bytes, exclusions/warnings/notes, exact PackageRefs, one-use capability result, and selection-clear timing
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

**Given** update-selected through component, browser, and native paths
**When** the one-use capability is admitted or rejected
**Then** the exact PackageRefs reach the backend, no unintended Package is added, and selection clears only after successful admission.

**Given** both historical FULL rows
**When** current evidence is admitted
**Then** neither status is inherited
**And** both become only **eligible for later FULL reassessment**.

## Epic 5: Make Manager Updates and Process Lifecycles Safe and Honest

Users can trust dynamic Manager update Routes, scheduler locks, exact live output, stall/timeout choices, cancellation, shutdown, null-input behavior, and the closed D26 transcript repair through controlled native process evidence.

### Story 5.1: Refresh Every Routed Subject and Executor

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

### Story 5.2: Prove Dynamic Self-Update Routes and Native Card State

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
- Expected evidence artifact: `b5-self-update-routes.json` with route-precedence decisions, serialized self state, production events, card text/actions, and queue states
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
**When** the SelfUpdateCard renders
**Then** route, queue, npm-reset, and unavailable-executor states are understandable
**And** all three criteria become only **eligible for later FULL reassessment** after valid admission.

### Story 5.3: Reject Unsafe Spawns and Hold Complete Locks

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

**Given** more than 5,000 live lines and production Operation events
**When** Activity renders
**Then** it appends, repaints, pins/unpins, bounds memory, preserves Operation context, and points to the complete durable transcript.

**Given** human and machine outputs agree on the first attempt
**When** admission evaluates them
**Then** both criteria become only **eligible for later FULL reassessment**.

### Story 5.5: Cancel, Stall, Time Out, and Shut Down Honestly

As a Pack-Manager user,
I want cancellation, stalls, timeouts, and shutdown to reach explicit safe outcomes,
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

**Given** controlled processes that exit on SIGTERM or require SIGKILL
**When** cancellation or application shutdown occurs
**Then** the complete process group is signaled, grace/escalation is recorded, children are reaped, terminal state/events/journal/transcript agree, and no confirmation dialog delays cancellation.

**Given** a null-stdin silent process and controlled time
**When** stall and hard-cap thresholds are reached
**Then** Keep waiting, Copy command, Cancel, and Timed out consequences match the contract
**And** no password/admin input is possible.

**Given** complete first-attempt cleanup evidence
**When** admission occurs
**Then** both criteria become only **eligible for later FULL reassessment**.

### Story 5.6: Repair Only the Allowlisted Unterminated `mas` Notice

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

## Epic 6: Preserve State, Evidence, and Privacy Across Failure and Relaunch

Users can reconstruct Operations after crashes, trust Settings and durable stores across failure, reveal native evidence safely, and export exact diagnostics without inherited-environment disclosure or hostile-path traversal.

### Story 6.1: Deliver ASR-02 Filesystem and Native-Utility Extensions

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

As a Pack-Manager user,
I want every Operation transcript and journal transition to survive failure coherently,
So that I can reconstruct what ran without partial or corrupt durable state.

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
- Expected evidence artifact: `b6-transcript-journal.json` plus retained transcript/journal files and byte/digest inventory
- Accountable role: QA
- Assignee: Unassigned
- Calendar date: Unassigned
- Implementation entry: Blocked by accepted ASR-03, ASR-02 filesystem controls, assignee/date, and admission prerequisites
- Candidate subjects and invalidation: Not applicable
- Attempt contract: Retain the first native lifecycle attempt and all partial/failure artifacts; `runnerRetryCount = "0"`

**Acceptance Criteria:**

**Given** controlled successful, failed, cancelled, timed-out, and abruptly terminated Operations
**When** real transcript creation, incremental line flush, and terminal footer behavior execute
**Then** canonical metadata, output, and available terminal state remain reconstructible
**And** transcript creation failure prevents spawn while later write failure is recorded without hanging work.

**Given** journal start/finish, cancel-before-start, corruption, rewrite failure, and compaction boundaries
**When** the native journal lifecycle executes
**Then** append/read/compaction remain atomic, corruption is handled explicitly, and newest-1,000 retention is enforced without partial state.

**Given** the complete first attempt is admitted
**When** the two slots are evaluated
**Then** both criteria become only **eligible for later FULL reassessment**.

### Story 6.4: Reconstruct Interrupted Work Without Signaling History

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
- Expected evidence artifact: `b6-interrupted-history.json` with crash/relaunch traces, retained data, sentinel observations, History results, transcript detail, and Reveal success/failure
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

**Given** durable and current-session History records
**When** the user filters, searches, opens detail, inspects transcript content, and selects Reveal
**Then** records combine coherently and native Reveal success/failure is visible without dangling actions.

**Given** the first lifecycle attempt passes
**When** admission evaluates it
**Then** both criteria become only **eligible for later FULL reassessment**.

### Story 6.5: Export Exact Native Diagnostics and Visible Outcomes

As a Pack-Manager user,
I want diagnostics export to create the documented archive and report native outcomes,
So that support evidence is complete, inspectable, and actionable.

**Story Contract:**

- Criteria and historical baseline: `F9-AC1` — `PARTIAL`; `F9-AC2` — `UNIT-ONLY`; `F9-AC4` — `PARTIAL`
- FR and requirement links: FR-18; TIR-3; TIR-5
- Primary readiness concern: Reusable Test Infrastructure
- Checkpoint: Batch 6
- Required test level: Real native Tauri E2E plus artifact inspection
- Execution lane / evidence depth: `forced-offline` / controlled environment-bound
- Dependencies: Stories 6.1–6.2; disposable logs/transcripts/journal
- ASR and risk links: ASR-02, ASR-03, ASR-05, TIR-3/TIR-5/TIR-8; R-005, R-008
- Behavior-present handling: Not `BP`
- Versioned scenario contract: `contracts/readiness/scenarios/v1/b6-diagnostics-export.json`
- Scenario-contract digest: Unassigned; freeze the exact file before implementation entry
- Expected evidence artifact: `b6-diagnostics-export.json`, the produced ZIP, archive inventory/digests, and native command/opener outcomes
- Accountable role: QA
- Assignee: Unassigned
- Calendar date: Unassigned
- Implementation entry: Blocked by ASR-02/03 controls, assignee/date, and admission prerequisites
- Candidate subjects and invalidation: Not applicable; produced ZIP is a result artifact, not a candidate subject
- Attempt contract: Preserve first ZIP and failure artifacts; `runnerRetryCount = "0"`

**Acceptance Criteria:**

**Given** documented default destination, alternate permission outcomes, and invocation from Settings and History
**When** diagnostics export runs through the production native command
**Then** the timestamped ZIP path and visible success/failure match the contract.

**Given** more than three app logs, 25 transcripts, and 1,000 journal records
**When** the produced ZIP is opened and inspected
**Then** it contains `report.json`, the newest three app logs, newest 25 transcripts, and `operations.jsonl` with exact expected contents and no missing required entry.

**Given** Export diagnostics and Open Logs actions
**When** native command/opener success and failure are controlled
**Then** the UI exposes actionable outcomes
**And** all three criteria become only **eligible for later FULL reassessment** after valid admission.

### Story 6.6: Reject Hostile or Private Diagnostic Inputs

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

**Given** Re-detect, diagnostics export, and Open Logs actions
**When** their native success and failure outcomes execute
**Then** the real commands cross production Tauri and visible UI state matches the native result.

**Given** the complete first attempt is admitted
**When** both slots are evaluated
**Then** `F11-AC1` and `F11-AC4` become only **eligible for later FULL reassessment**.

## Epic 7: Validate the Installed Accessible App and Explicit Updater Journey

Users can operate the exact installed packaged application accessibly and can update from an actually installed prior public version to the frozen candidate through an authorized background download and explicit Restart to update, with active-operation refusal and no privilege escalation.

Story 7.1 is the final post-Batch-6 ASR-02 technical-enablement prerequisite. Story 7.2 is the ASR-04 contract gate that must be accepted before release preparation begins. Stories 7.3–7.5 are release-preparation prerequisites with zero denominator rows. All five follow accepted Epics 1–6 and precede Batch 7 collection; none creates Batch 9.

### Story 7.1: Deliver ASR-02 Updater-Control Extensions

As a Development owner,
I want update discovery, download, signature, writability, refusal, installation, restart, and relaunch effects behind the accepted typed ports,
So that candidate-era updater journeys can be driven deterministically without weakening production authorization.

**Story Contract:**

- Criteria and historical baseline: None; the ASR-02 Batch 7 extension adds no denominator row
- FR and requirement links: No direct FR implementation; ASR-02/TIR-4 updater-control enabler for FR-20 and FR-21
- Primary readiness concern: Reusable Test Infrastructure
- Checkpoint: ASR-02 updater extension accepted before Batch 7 and before candidate-lane qualification
- Required test level: Unit, contract, controlled native, and negative-admission qualification
- Execution lane / evidence depth: `forced-offline` / controlled environment-bound; no candidate evidence is produced
- Dependencies: Accepted Epics 1–6 and Story 4.2 core; controlled-helper language decision; production updater adapter inventory
- ASR and risk links: ASR-02 — Development accountable, Platform capability area; ASR-04, ASR-05; R-004, R-006, R-008
- Behavior-present handling: Not applicable
- Versioned scenario contract: `contracts/readiness/scenarios/v1/asr-02-updater-control-extension.json`
- Scenario-contract digest: Unassigned; freeze the exact file before implementation entry
- Expected evidence artifact: `asr-02-updater-control-extension-qualification.json` with check/download/signature/state/install/refusal/writability/restart/relaunch control coverage and production-adapter exclusion
- Accountable role: Development
- Assignee: Unassigned
- Calendar date: Unassigned
- Implementation entry: Blocked by accepted Epics 1–6, accepted ASR-02 core, controlled-helper decision, production updater inventory, and assignee/date
- Candidate subjects and invalidation: No candidate subjects; the extension drives controlled conditions only and cannot substitute for unchanged-candidate Batch 7 evidence
- Attempt contract: First extension qualification attempt retained; `runnerRetryCount = "0"`

**Acceptance Criteria:**

**Given** the accepted ASR-02 core and production updater composition
**When** the updater extension is composed
**Then** check state, metadata response, archive download, detached-signature verification, active-operation refusal, bundle writability, explicit install, restart, and intended-version relaunch cross typed ports
**And** production adapters retain authorized-key, explicit-user-action, no-shell, no-sudo, no-password, and fail-closed behavior.

**Given** the non-distributable controlled composition
**When** success, stale/malformed/incomplete metadata, hash/signature mismatch, download failure, queued/running Package Operation, non-writable bundle, install failure, restart failure, and wrong-version relaunch are requested
**Then** each outcome is deterministic and observable
**And** no release feature, environment variable, hidden selector, or alternate business path can activate a controlled adapter.

**Given** the extension qualification result
**When** candidate-lane or Batch 7 entry is evaluated
**Then** Development is the sole accountable ASR-02 role for the updater extension
**And** candidate-bound execution remains separately blocked on release preparation, DR-1, hosts, prior version, credentials, and the frozen candidate.

### Story 7.2: Accept the Locked Evidence Contract and Append Transport

As a Release Owner,
I want the strict v1 schemas, canonicalization vectors, protected Registrar, and retention transport accepted before candidate freeze,
So that candidate identity and evidence cannot be clobbered, forked, or rewritten.

**Story Contract:**

- Criteria and historical baseline: None; ASR-04 prerequisite work adds no denominator row
- FR and requirement links: No direct FR implementation; ASR-04 and RE-1/RE-11 evidence-contract/Registrar enabler
- Primary readiness concern: Candidate-Specific Release Evidence
- Checkpoint: Pre-release-preparation ASR-04 contract gate; accepted before Story 7.3 and any release preparation
- Required test level: Schema/vector/ledger contract and protected-transport qualification
- Execution lane / evidence depth: `forced-offline` contract qualification plus provider-verifiable environment qualification; no criterion binding is assigned
- Dependencies: Story 7.1; accepted Epics 1–6; Release decision for evidence transport and retention duration; protected GitHub Environment/workflow identity; named assignee/date
- ASR and risk links: ASR-04 — Release accountable; ASR-05; R-006, R-007, R-008
- Behavior-present handling: Not applicable
- Versioned scenario contract: `contracts/readiness/scenarios/v1/asr-04-contract-registrar.json`
- Scenario-contract digest: Unassigned; freeze the exact file before implementation entry
- Expected evidence artifact: `asr-04-contract-registrar-qualification.json` with schema/vector results, provider identity, lock/CAS/idempotency cases, one-head/no-clobber proof, and retention declaration
- Accountable role: Release
- Assignee: Unassigned
- Calendar date: Unassigned
- Implementation entry: Blocked by evidence transport/retention decision and assignee/date
- Candidate subjects and invalidation: No frozen candidate yet; changing any locked `/v1` byte requires `/v2`
- Attempt contract: Qualification ordinal 1 retained; automatic runner/workflow retry count zero

**Acceptance Criteria:**

**Given** the three strict Draft 2020-12 schemas, canonicalization vectors, and `contract-lock.json`
**When** contract qualification runs across independent implementations
**Then** I-JSON/NFC/order rules, duplicate/unknown key rejection, RFC 8785 JCS bytes, UTF-8/BOM/newline boundaries, raw-file hashes, and lowercase SHA-256 digests agree exactly
**And** any locked-byte change requires `/v2`.

**Given** immutable producer attempt bundles and the protected Release-owned Registrar
**When** append qualification exercises success, repeated idempotency key, stale head, fork, second head, clobber, missing object, and retention cases
**Then** only the allowlisted workflow identity may append under candidate/profile lock or CAS
**And** one monotonic head and write-once objects/snapshots are preserved.

**Given** a qualification failure or rerun
**When** a later attempt occurs
**Then** the first failure remains immutable and visible
**And** automatic retry cannot launder the result.

### Story 7.3: Freeze the Criterion Acceptance Profile

As a QA Lead,
I want one canonical profile that fixes every P0 and Release Prerequisite evidence slot,
So that epics cannot choose conflicting lanes, depths, subjects, environments, or retry rules.

**Story Contract:**

- Criteria and historical baseline: None; profile governance adds no denominator row
- FR and requirement links: No direct FR implementation; GP-1, AD-15, and RE-10 acceptance-profile governance
- Primary readiness concern: Reusable Test Infrastructure
- Checkpoint: Release preparation prerequisite before candidate validation
- Required test level: Schema, completeness, negative-admission, and canonicalization contract
- Execution lane / evidence depth: `forced-offline` / source-bound profile qualification
- Dependencies: Story 7.2; Product/QA approval of the coverage map and DR-4; Product/Release resolution of DR-1; immutable scenario contracts; approval-record digests
- ASR and risk links: ASR-05 — QA accountable, CI execution mechanism; ASR-04; R-001, R-006, R-007, R-008
- Behavior-present handling: Profile must preserve all reviewed `BP` dispositions and any approved map revision; it cannot silently reclassify behavior
- Versioned scenario contract: `contracts/readiness/scenarios/v1/acceptance-profile-freeze.json`
- Scenario-contract digest: Unassigned; freeze the exact file before implementation entry
- Expected evidence artifact: Canonical `criterion-acceptance-profile.json`, its digest, completeness report, and retained map/policy/approval/scenario inputs by digest
- Accountable role: QA
- Assignee: Unassigned
- Calendar date: Unassigned
- Implementation entry: Blocked while the map is `final-pending-approval`, DR-1 is OPEN, DR-4 is PROPOSED, any scenario digest is unassigned, or assignee/date are missing
- Candidate subjects and invalidation: The profile contains candidate subject requirements but no candidate digest; a profile change creates a new Evidence Set namespace without renaming unchanged candidate artifacts
- Attempt contract: Profile qualification first attempt retained; automatic retries zero; retry disposition values come only from approved DR-4

**Acceptance Criteria:**

**Given** the map remains pending, DR-1 remains OPEN, DR-4 remains PROPOSED, or an approval/scenario input is missing
**When** profile freeze is attempted
**Then** it fails closed and produces no acceptance-profile digest.

**Given** approved immutable inputs
**When** `pack-manager.criterion-acceptance-profile/v1` is validated and canonicalized
**Then** slots collectively cover exactly all 72 unique P0 IDs plus RP-1 and RP-2 outside the denominator
**And** every slot fixes exactly one concern, lane, minimum binding level, scenario path/digest, subject set, OS/architecture/physical/packaged matrix, and approved retry disposition.

**Given** wrong-lane, shallow-depth, missing-first-attempt, automatic-retry, branching-retry, ignored/unexecuted, or incomplete PASS test cases
**When** profile/aggregator qualification runs
**Then** every case fails closed
**And** the canonical profile digest is reproducible across machines.

### Story 7.4: Freeze One Immutable Signed Candidate

As a Release Owner,
I want one clean, fully packaged, signed, notarized, stapled candidate frozen into a canonical identity manifest,
So that all candidate-bound scenarios test the exact same release bytes.

**Story Contract:**

- Criteria and historical baseline: None; Candidate Manifest preparation adds no denominator row
- FR and requirement links: No direct FR implementation; RE-1/RE-3 candidate-freeze prerequisite supporting FR-19–FR-22
- Primary readiness concern: Candidate-Specific Release Evidence
- Checkpoint: Release preparation prerequisite after accepted Epics 1–6 and before Batch 7
- Required test level: Artifact/release attestation and manifest-contract validation
- Execution lane / evidence depth: Candidate preparation preceding `candidate-release`; candidate-bound identity
- Dependencies: Stories 7.1–7.3; accepted Epics 1–6; one clean GitHub Actions run/attempt; current Apple/updater credentials; all required final artifacts and metadata
- ASR and risk links: ASR-04 — Release accountable; ASR-05; RE-1; R-006, R-007, R-008
- Behavior-present handling: Not applicable
- Versioned scenario contract: `contracts/readiness/scenarios/v1/candidate-freeze.json`
- Scenario-contract digest: Unassigned; freeze the exact file before implementation entry
- Expected evidence artifact: Exact canonical `candidate-identity.json`, `candidate-identity.sha256`, raw artifact inventory, and freeze attestation
- Accountable role: Release
- Assignee: Unassigned
- Calendar date: Unassigned
- Implementation entry: Blocked by accepted Epics 1–6, Stories 7.1–7.3, credentials, required artifacts, assignee/date, and one eligible clean build attempt
- Candidate subjects and invalidation: Exactly `direct-app-zip`, `dmg`, `updater-archive`, `updater-metadata`, and `updater-signature`; any source/tag/version/signing/artifact/name/metadata/build-run/build-attempt mutation creates a new manifest/evidence root
- Attempt contract: Candidate-build workflow automatic retries are disabled; a new workflow run or run attempt is a different candidate, not a retry of the same identity

**Acceptance Criteria:**

**Given** one clean GitHub Actions release build attempt
**When** packaging, signing, notarization, stapling, updater signing, and metadata generation finish
**Then** the five final artifact subjects exist with exact logical IDs, names, media types, HTTPS URLs, decimal byte lengths, and raw SHA-256 values
**And** all versions, universal target, source/tag/lockfiles, workflow identity, toolchains, certificate fingerprint, Team ID, and embedded updater-key digest are coherent.

**Given** the identity-only manifest value
**When** strict schema/I-JSON/NFC/order validation and RFC 8785 JCS canonicalization run
**Then** `candidate-identity.json` equals the canonical bytes exactly and the separately stored digest is reproducible lowercase `sha256:<64-hex>`
**And** no result, mutable status, timestamp, or machine-local path appears in the manifest.

**Given** any identity-changing mutation or another release-build run/attempt
**When** freeze validation reruns
**Then** a new Candidate Manifest and evidence root are mandatory
**And** prior candidate results remain immutable history but are ineligible for the new candidate.

### Story 7.5: Qualify the Candidate-Release Lane

As a QA Lead,
I want the candidate-release lane isolated and operational on the required physical environments,
So that no no-sign build, mutable host, or other lane can substitute for installed-candidate evidence.

**Story Contract:**

- Criteria and historical baseline: None; ASR-05 candidate-lane qualification adds no denominator row
- FR and requirement links: No direct FR implementation; ASR-05 and TIR-7/TIR-8 candidate-lane enabler
- Primary readiness concern: Reusable Test Infrastructure
- Checkpoint: Candidate lane operational before Batch 7
- Required test level: Lane admission, environment qualification, and negative-substitution tests
- Execution lane / evidence depth: `candidate-release` / candidate-bound qualification
- Dependencies: Stories 7.1–7.4; resolved DR-1; approved DR-2/DR-3; Apple-silicon and physical-Intel hosts; installed prior public version; current credentials/endpoints
- ASR and risk links: ASR-05 — QA accountable, CI execution mechanism; ASR-04; R-006, R-007, R-008
- Behavior-present handling: Not applicable
- Versioned scenario contract: `contracts/readiness/scenarios/v1/asr-05-candidate-lane.json`
- Scenario-contract digest: Unassigned; freeze the exact file before implementation entry
- Expected evidence artifact: `asr-05-candidate-lane-qualification.json` with host/provision profiles, exact manifest/artifact checks, credential isolation, endpoint mode, and negative substitution results
- Accountable role: QA
- Assignee: Unassigned
- Calendar date: Unassigned
- Implementation entry: Blocked by DR-1, hosts, prior version, credentials/endpoints, Story 7.4, and assignee/date
- Candidate subjects and invalidation: All five manifest subjects; any candidate mutation invalidates qualification for the affected new root and requires rerun
- Attempt contract: First qualification attempt retained; `runnerRetryCount = "0"`; evidence-collection retry on unchanged bytes is linked and does not create a new manifest

**Acceptance Criteria:**

**Given** the frozen manifest, eligible profile, required hosts, and approved endpoints/OS services
**When** candidate-lane admission runs
**Then** manifest/artifact checksums, environment profiles, architecture, physical-host requirement, credentials, caches, workspaces, and result namespaces match exactly.

**Given** a no-sign/credentialless build, wrong artifact, different manifest, target-Mac result, forced-offline result, mutable metadata, or missing physical Intel host
**When** admission is attempted
**Then** the candidate lane rejects it without relabeling or substitution.

**Given** a valid qualification attempt
**When** its result is appended
**Then** the protected Registrar binds it to the exact manifest/profile digests and candidate subjects
**And** any candidate mutation requires a new root and rerun.

### Story 7.6: Validate Packaged Accessibility and Bounded Presentation

As a Pack-Manager user,
I want the installed candidate to remain accessible and usable under real packaged constraints,
So that browser-only checks cannot conceal a WKWebView, focus, contrast, motion, or capacity defect.

**Story Contract:**

- Criteria and historical baseline: `F10-AC1` — `PARTIAL`
- FR and requirement links: FR-19; NFR-6; TIR-7
- Primary readiness concern: Reusable Test Infrastructure
- Checkpoint: Batch 7
- Required test level: Installed packaged-app automation plus manual VoiceOver
- Execution lane / evidence depth: `candidate-release` / candidate-bound
- Dependencies: Stories 7.1–7.5; approved DR-2; resolved DR-1 environment matrix; exact installed candidate
- ASR and risk links: ASR-01, ASR-02 updater/packaged controls, ASR-04, ASR-05, TIR-7/TIR-8; R-003, R-007, R-008
- Behavior-present handling: Not a map `BP` row; any discovered missing/incorrect behavior creates Product Behavior work and invalidates the affected candidate slot
- Versioned scenario contract: `contracts/readiness/scenarios/v1/b7-packaged-accessibility.json`
- Scenario-contract digest: Unassigned; freeze the exact file before implementation entry
- Expected evidence artifact: Automated packaged interaction/contrast/motion/capacity results, screenshots where permitted, resource/timing report, and signed manual VoiceOver record
- Accountable role: QA
- Assignee: Unassigned
- Calendar date: Unassigned
- Implementation entry: Blocked by DR-1, Stories 7.1–7.5, exact candidate, hosts, assignee/date, and approved profile slot
- Candidate subjects and invalidation: `dmg` with role `installed-from`; `direct-app-zip` with role `executed`; any candidate mutation creates a new root and reruns this Batch 7 slot
- Attempt contract: Retain first automated and manual attempt; `runnerRetryCount = "0"`; an unchanged-candidate retry is a new linked record with retained failure

**Acceptance Criteria:**

**Given** the exact installed candidate inside packaged WKWebView
**When** keyboard navigation, focus visibility/order, non-color status, automated 4.5:1 contrast, reduced motion, and completion announcements are evaluated
**Then** the approved DR-2 method passes with human/machine agreement
**And** manual VoiceOver verifies focus order and completion announcements without implying broader WCAG/legal compliance.

**Given** 101 Package rows, 5,001 live lines, and the 900×600 minimum window
**When** packaged capacity scenarios execute
**Then** the final row/actions remain reachable, the newest 5,000 live lines remain usable with full transcript disclosure, and essential columns remain reachable without overlap.

**Given** a valid first attempt bound to both candidate subjects
**When** the Registrar admits it
**Then** `F10-AC1` becomes only **eligible for later FULL reassessment**
**And** browser/dev-server evidence cannot substitute.

### Story 7.7: Validate Real Updater State and Authorized Download

As a Pack-Manager user,
I want the installed prior version to expose accurate check, availability, download, progress, ready, and error states,
So that I can understand a real authorized update without installing it silently.

**Story Contract:**

- Criteria and historical baseline: `D25-AC2` — `PARTIAL`
- FR and requirement links: FR-20; TIR-7
- Primary readiness concern: Reusable Test Infrastructure
- Checkpoint: Batch 7
- Required test level: Installed packaged-app updater acceptance
- Execution lane / evidence depth: `candidate-release` / candidate-bound
- Dependencies: Stories 7.1–7.5; installed prior public version; frozen real endpoint/metadata/archive/signature/events
- ASR and risk links: ASR-01, ASR-02 updater controls, ASR-04, ASR-05, TIR-7/TIR-8, RE-6; R-003, R-006, R-007, R-008
- Behavior-present handling: Not `BP`; missing/incorrect updater state creates Product Behavior work and invalidates the affected slot
- Versioned scenario contract: `contracts/readiness/scenarios/v1/b7-updater-state-download.json`
- Scenario-contract digest: Unassigned; freeze the exact file before implementation entry
- Expected evidence artifact: `b7-updater-state-download.json` with endpoint responses, metadata/archive/signature hashes, production event sequence, UI states, and failure outcomes
- Accountable role: QA
- Assignee: Unassigned
- Calendar date: Unassigned
- Implementation entry: Blocked by prior version, exact endpoint/subjects, Stories 7.1–7.5, assignee/date, and profile slot
- Candidate subjects and invalidation: `updater-metadata` as `served-metadata`, `updater-archive` as `inspected`, and `updater-signature` as `verified-signature`; candidate mutation requires a new root and rerun
- Attempt contract: First download/state attempt retained; `runnerRetryCount = "0"`; unchanged-candidate retry is linked

**Acceptance Criteria:**

**Given** an actually installed prior public version and the frozen HTTPS endpoint
**When** check and automatic background download execute
**Then** current, available, downloading, progress, ready, and failure states cross the production updater/event boundary accurately
**And** no install or restart occurs without explicit user action.

**Given** wrong/missing metadata, URL failure, archive mismatch, invalid signature, interrupted download, or event failure
**When** each controlled outcome occurs
**Then** the state is actionable and never appears Ready without a complete authorized download.

**Given** the first candidate-bound attempt passes
**When** the Registrar validates exact subjects and provenance
**Then** `D25-AC2` becomes only **eligible for later FULL reassessment**.

### Story 7.8: Validate Explicit Update, Active-Operation Refusal, and Relaunch

As a Pack-Manager user,
I want Restart to update to reach the exact candidate only when Package work is inactive,
So that application updating remains explicit and cannot interrupt package-management Operations.

**Story Contract:**

- Criteria and historical baseline: `D25-AC3` — `PARTIAL`
- FR and requirement links: FR-21; TIR-7
- Primary readiness concern: Reusable Test Infrastructure
- Checkpoint: Batch 7
- Required test level: Installed packaged-app acceptance on Apple silicon and physical Intel
- Execution lane / evidence depth: `candidate-release` / candidate-bound
- Dependencies: Stories 7.1–7.7; DR-1; approved DR-3; prior public version installed on both architectures; exact candidate
- ASR and risk links: ASR-02 updater/restart controls, ASR-04, ASR-05, TIR-7/TIR-8, RE-8; R-003, R-006, R-007, R-008
- Behavior-present handling: Not `BP`; missing/incorrect refusal/install/relaunch behavior creates Product Behavior work and invalidates the slot
- Versioned scenario contract: `contracts/readiness/scenarios/v1/b7-explicit-update-relaunch.json`
- Scenario-contract digest: Unassigned; freeze the exact file before implementation entry
- Expected evidence artifact: Before/after version and interaction records for both architectures, active-operation refusal traces, installed-byte checks, and relaunch results
- Accountable role: QA
- Assignee: Unassigned
- Calendar date: Unassigned
- Implementation entry: Blocked by DR-1, physical hosts, installed prior version, exact candidate, assignee/date, and profile slot
- Candidate subjects and invalidation: `updater-metadata` as `served-metadata`, `updater-archive` as `installed-from`, and `updater-signature` as `verified-signature`; mutation creates a new root and reruns this slot
- Attempt contract: First attempt per required host/slot retained; `runnerRetryCount = "0"`; unchanged-candidate retries remain linked

**Acceptance Criteria:**

**Given** a downloaded authorized update and a queued or running Package Operation
**When** the user chooses Restart to update
**Then** install/relaunch is refused, the user is told to finish or cancel Package work, and no updater installation begins.

**Given** no active Package Operation on Apple silicon and physical Intel
**When** the user explicitly chooses Restart to update from the installed prior version
**Then** the authorized archive installs without an administrator prompt and relaunches as the exact intended candidate version with retained before/after evidence.

**Given** both physical-host first attempts pass and match the manifest
**When** records are admitted
**Then** `D25-AC3` becomes only **eligible for later FULL reassessment**.

### Story 7.9: Refuse Privileged Installation on a Non-Writable Bundle

As a Pack-Manager user,
I want a non-writable installation to require manual installation without an authorization prompt,
So that Pack-Manager never weakens its no-administrator boundary.

**Story Contract:**

- Criteria and historical baseline: `D25-AC4` — `PARTIAL`
- FR and requirement links: FR-12; FR-21; TIR-7
- Primary readiness concern: Reusable Test Infrastructure
- Checkpoint: Batch 7
- Required test level: Installed packaged-app acceptance
- Execution lane / evidence depth: `candidate-release` / candidate-bound
- Dependencies: Stories 7.1–7.7; resolved DR-1 matrix; controlled non-writable candidate installation
- ASR and risk links: ASR-02 permission/updater controls, ASR-04, ASR-05, TIR-7/TIR-8, RE-9; R-004, R-006, R-007, R-008
- Behavior-present handling: Not `BP`; missing/incorrect non-writable behavior creates Product Behavior work and invalidates the slot
- Versioned scenario contract: `contracts/readiness/scenarios/v1/b7-nonwritable-install.json`
- Scenario-contract digest: Unassigned; freeze the exact file before implementation entry
- Expected evidence artifact: `b7-nonwritable-install.json` with writability preflight, updater call trace, authorization-prompt observation, UI state, and candidate binding
- Accountable role: QA
- Assignee: Unassigned
- Calendar date: Unassigned
- Implementation entry: Blocked by DR-1, controlled environment, exact candidate, assignee/date, and profile slot
- Candidate subjects and invalidation: `updater-archive` as `installed-from`; candidate mutation requires a new root and rerun
- Attempt contract: First non-writable attempt retained; `runnerRetryCount = "0"`

**Acceptance Criteria:**

**Given** the exact candidate update and an installation parent that is not writable
**When** explicit install is requested
**Then** preflight prevents the plugin's administrator fallback, no authorization prompt appears, no install/restart begins, and the UI enters actionable manual-install-required state.

**Given** the first candidate-bound attempt and exact updater-archive subject
**When** the Registrar validates the result
**Then** any missing prompt observation, wrong artifact, wrong environment, or automatic retry fails closed
**And** `D25-AC4` becomes only **eligible for later FULL reassessment**.

### Story 7.10: Validate Application-Update Triggers and State Continuity

As a Pack-Manager user,
I want update checks and in-process state continuity to follow the adopted trigger policy,
So that application updates remain understandable and separate from Package History.

**Story Contract:**

- Criteria and historical baseline: RP-1 retains legacy `D25-AC1` and `D25-AC5`; both remain outside the 72-row denominator
- FR and requirement links: FR-20; FR-21; RP-1
- Primary readiness concern: Reusable Test Infrastructure
- Checkpoint: Batch 7 with final association in Batch 8
- Required test level: Installed packaged-app updater and state/menu contract
- Execution lane / evidence depth: `candidate-release` / candidate-bound
- Dependencies: Stories 7.1–7.8; installed prior version; frozen profile RP-1 slot
- ASR and risk links: ASR-01, ASR-02, ASR-04, ASR-05, TIR-7/TIR-8; R-003, R-006, R-008
- Behavior-present handling: RP-1 is a mandatory prerequisite; missing behavior creates Product Behavior work and cannot be waived into the denominator
- Versioned scenario contract: `contracts/readiness/scenarios/v1/rp-1-update-state-continuity.json`
- Scenario-contract digest: Unassigned; freeze the exact file before implementation entry
- Expected evidence artifact: `rp-1-update-state-continuity.json` with trigger timing/menu invocations, UI recreation state, relaunch result, error/retry policy, and Package History separation
- Accountable role: QA
- Assignee: Unassigned
- Calendar date: Unassigned
- Implementation entry: Blocked by Stories 7.1–7.8, prior version, assignee/date, and frozen RP-1 slot
- Candidate subjects and invalidation: `updater-metadata` as `served-metadata`, `updater-archive` as `installed-from`, and `updater-signature` as `verified-signature`; mutation reruns the RP-1 slot
- Attempt contract: First RP-1 attempt retained; `runnerRetryCount = "0"`

**Acceptance Criteria:**

**Given** launch, six-hour policy time, and app-menu demand
**When** update checks are observed in the installed application
**Then** each trigger uses the same backend path and saved policy without duplicate or hidden install behavior.

**Given** check/error/progress/downloaded/ready state and supported window/UI recreation
**When** the UI rehydrates in the same process or the app normally relaunches
**Then** in-process state is restored, normal relaunch starts from saved policy, failed/interrupted download becomes Error rather than Ready, and updater restart returns Current for the installed version.

**Given** application-update state
**When** Package Operation queue and History are inspected
**Then** updater state remains separate and no non-Operation update appears as Package History
**And** RP-1 is only **eligible for later FULL reassessment** as an external prerequisite
**And** that wording neither creates a denominator row nor changes any criterion status.

### Story 7.11: Validate Standard macOS Menu Behavior

As a Pack-Manager user,
I want standard Edit and Window actions to remain available in the custom app menu,
So that search and every copyable command surface retain normal macOS keyboard behavior.

**Story Contract:**

- Criteria and historical baseline: RP-2 retains legacy `D25A-AC1`; it remains outside the 72-row denominator
- FR and requirement links: FR-19; RP-2
- Primary readiness concern: Reusable Test Infrastructure
- Checkpoint: Batch 7 with final association in Batch 8
- Required test level: Installed packaged-app native-menu keyboard/interaction
- Execution lane / evidence depth: `candidate-release` / candidate-bound
- Dependencies: Stories 7.1–7.6; frozen profile RP-2 slot
- ASR and risk links: ASR-01, ASR-04, ASR-05, TIR-7/TIR-8; R-003, R-007, R-008
- Behavior-present handling: RP-2 is a mandatory prerequisite; missing behavior creates Product Behavior work
- Versioned scenario contract: `contracts/readiness/scenarios/v1/rp-2-macos-menu.json`
- Scenario-contract digest: Unassigned; freeze the exact file before implementation entry
- Expected evidence artifact: `rp-2-macos-menu.json` with native menu inventory, focus/selection state, keyboard events, and clipboard results
- Accountable role: QA
- Assignee: Unassigned
- Calendar date: Unassigned
- Implementation entry: Blocked by exact candidate, assignee/date, and frozen RP-2 slot
- Candidate subjects and invalidation: `direct-app-zip` with role `executed`; candidate mutation reruns the RP-2 slot
- Attempt contract: First RP-2 attempt retained; `runnerRetryCount = "0"`

**Acceptance Criteria:**

**Given** the installed candidate's custom app menu
**When** menu inventory is inspected
**Then** standard Edit and Window actions are present with correct native behavior.

**Given** Package search and every copyable command surface
**When** Cut, Copy, Paste, and Select All are used through menus and standard shortcuts
**Then** focused content changes exactly as expected without intercepting unrelated Package selection behavior.

**Given** the first candidate-bound menu attempt passes
**When** the RP-2 record is admitted
**Then** RP-2 is only **eligible for later FULL reassessment** as an external prerequisite
**And** that wording neither creates a denominator row nor changes any criterion status.

## Epic 8: Attest the Unchanged Release and Produce a Reproducible Evidence Handoff

Users and release decision-makers can identify, install, launch, and audit one unchanged, complete, trusted Pack-Manager candidate across both promised architectures, with exact source-quality, artifact, provenance, and append-only evidence suitable for a later independent Trace decision.

### Story 8.1: Attest Icon Source and Packaged Resources

As a Release Owner,
I want the approved icon source and required generated resources attested in the exact candidate,
So that source intent and shipped bundle contents cannot diverge silently.

**Story Contract:**

- Criteria and historical baseline: `F10-AC2` — `NONE`
- FR and requirement links: No direct FR primary mapping; RE-4 packaged-resource attestation supports FR-19/FR-22 without changing the normative primary mapping
- Primary readiness concern: Candidate-Specific Release Evidence
- Checkpoint: Batch 8
- Required test level: Artifact/release attestation
- Execution lane / evidence depth: `candidate-release` / candidate-bound
- Dependencies: Accepted Epic 7; unchanged manifest; approved icon source; exact app/DMG/ZIP contents
- ASR and risk links: ASR-04, ASR-05, TIR-8, RE-4; R-007, R-008
- Behavior-present handling: Not `BP`; missing/incorrect source or packaged resources creates Product Behavior or release-preparation correction as appropriate and invalidates the slot
- Versioned scenario contract: `contracts/readiness/scenarios/v1/b8-icon-resources.json`
- Scenario-contract digest: Unassigned; freeze the exact file before implementation entry
- Expected evidence artifact: `b8-icon-resources.json` with source icon digest/provenance, generated icon inventory, bundle-resource paths/digests, and candidate subject checks
- Accountable role: Release
- Assignee: Unassigned
- Calendar date: Unassigned
- Implementation entry: Blocked by accepted Epic 7, unchanged candidate, assignee/date, and frozen profile slot
- Candidate subjects and invalidation: `direct-app-zip` and `dmg`, both with role `inspected`; any candidate mutation creates a new root and reruns this Batch 8 slot
- Attempt contract: First artifact attempt retained; `runnerRetryCount = "0"`; unchanged-candidate retry remains linked

**Acceptance Criteria:**

**Given** the approved 1024px icon source and candidate manifest
**When** source and packaged resource attestation runs
**Then** source provenance/digest and the required generated icon/resource set are complete, correctly named, and present in the exact inspected candidate subjects.

**Given** a missing, stale, substituted, differently generated, or wrong-candidate resource
**When** attestation evaluates it
**Then** the attempt fails closed and cannot be replaced by source inspection alone.

**Given** the valid first candidate-bound attempt
**When** the Registrar admits it
**Then** `F10-AC2` becomes only **eligible for later FULL reassessment**.

### Story 8.2: Prove Fresh Install and Finder/Dock Launch on Both Architectures

As a macOS Pack-Manager user,
I want the downloaded candidate to install and launch normally on Apple silicon and physical Intel,
So that a universal header cannot substitute for the experience users actually run.

**Story Contract:**

- Criteria and historical baseline: `F10-AC3` — `NONE`
- FR and requirement links: FR-22; RE-7
- Primary readiness concern: Candidate-Specific Release Evidence
- Checkpoint: Batch 8
- Required test level: Installed packaged-app acceptance
- Execution lane / evidence depth: `candidate-release` / candidate-bound
- Dependencies: Accepted Epic 7; resolved DR-1; approved DR-3; Apple-silicon and physical-Intel hosts; exact downloaded DMG
- ASR and risk links: ASR-04, ASR-05, TIR-7/TIR-8, RE-7; R-007, R-008
- Behavior-present handling: Not `BP`; launch failure creates Product Behavior or candidate correction and invalidates the slot
- Versioned scenario contract: `contracts/readiness/scenarios/v1/b8-fresh-install-launch.json`
- Scenario-contract digest: Unassigned; freeze the exact file before implementation entry
- Expected evidence artifact: Per-host install, Gatekeeper, Finder launch, Dock launch, GUI environment, resource/entitlement, WKWebView, process, and version records
- Accountable role: QA
- Assignee: Unassigned
- Calendar date: Unassigned
- Implementation entry: Blocked by DR-1, both physical hosts, unchanged candidate, assignee/date, and profile slot
- Candidate subjects and invalidation: `dmg` as `installed-from`; `direct-app-zip` as `executed`; candidate mutation creates a new root and reruns both host attempts
- Attempt contract: First attempt per profile-fixed host/slot retained; `runnerRetryCount = "0"`

**Acceptance Criteria:**

**Given** the exact downloaded DMG on an approved Apple-silicon host and physical Intel host
**When** the approved install journey executes
**Then** the same manifest-bound candidate is installed without security bypass or administrator prompt.

**Given** the installed candidate
**When** it launches through Finder and then the Dock
**Then** both hosts prove intended version, packaged resources/entitlements, GUI ToolEnv discovery, production WKWebView, and usable startup state.

**Given** both first-attempt host records match the profile and manifest
**When** they are admitted
**Then** `F10-AC3` becomes only **eligible for later FULL reassessment**
**And** universal-binary inspection alone cannot satisfy the slot.

### Story 8.3: Attest Universal, Signed, Notarized, Stapled, Updater-Complete Trust

As a Release Owner,
I want the entire candidate trust chain and artifact set attested exactly,
So that a published but incomplete or unauthorized release cannot be mistaken for a valid candidate.

**Story Contract:**

- Criteria and historical baseline: `F10-AC4` — `NONE`
- FR and requirement links: FR-22; RE-3; RE-4; RE-5; RE-6
- Primary readiness concern: Candidate-Specific Release Evidence
- Checkpoint: Batch 8
- Required test level: Artifact/release attestation
- Execution lane / evidence depth: `candidate-release` / candidate-bound
- Dependencies: Accepted Epic 7; unchanged candidate; current Apple/updater trust services and credentials
- ASR and risk links: ASR-04, ASR-05, TIR-8, RE-3/RE-4/RE-5/RE-6; R-006, R-007, R-008
- Behavior-present handling: Not `BP`; stale source truth is already corrected by PC-1 and cannot substitute for candidate proof
- Versioned scenario contract: `contracts/readiness/scenarios/v1/b8-candidate-trust.json`
- Scenario-contract digest: Unassigned; freeze the exact file before implementation entry
- Expected evidence artifact: `b8-candidate-trust.json` with architecture, resources, entitlements, signatures, certificate identity, notarization, stapling, Gatekeeper, metadata/URL/signature/key, and artifact-inventory results
- Accountable role: Release
- Assignee: Unassigned
- Calendar date: Unassigned
- Implementation entry: Blocked by unchanged candidate, current trust endpoints, assignee/date, and profile slot
- Candidate subjects and invalidation: All five manifest subjects with profile-fixed roles: direct app ZIP/DMG/updater archive inspected, updater metadata served, updater signature verified; mutation creates a new root and reruns this slot
- Attempt contract: Preserve the first trust attempt and raw tool outputs; `runnerRetryCount = "0"`

**Acceptance Criteria:**

**Given** the exact five manifest subjects
**When** architecture, bundle-resource, entitlement, version, and completeness checks run
**Then** the required universal architecture and complete coherent direct-download/updater set agree with the manifest.

**Given** the exact app/DMG/archive/metadata/signature
**When** Developer ID, certificate, secure-signature, notarization, stapling, Gatekeeper, HTTPS reachability, URL, archive hash, detached signature, embedded key, and version checks run
**Then** every trust boundary passes against the exact candidate without bypass.

**Given** any incomplete, no-sign, mismatched, inaccessible, unnotarized, unstapled, or wrong-candidate subject
**When** admission evaluates it
**Then** the attempt fails closed
**And** a valid first attempt makes `F10-AC4` only **eligible for later FULL reassessment**.

### Story 8.4: Retain the First Clean Forced-Offline Quality Run

As a Release Owner,
I want complete first-run quality output from the candidate's exact clean source with outbound network denied,
So that reproducibility evidence cannot be replaced by a green retry or a different checkout.

**Story Contract:**

- Criteria and historical baseline: `F12-AC1` — `PARTIAL`
- FR and requirement links: No direct FR primary mapping; TIR-2 and RE-2 clean-source release-quality evidence
- Primary readiness concern: Candidate-Specific Release Evidence
- Checkpoint: Batch 8
- Required test level: Clean-checkout CI quality run
- Execution lane / evidence depth: `forced-offline` / source-bound
- Dependencies: Accepted Epic 7; unchanged Candidate Manifest; exact candidate commit/lockfiles; pinned dependencies/toolchains prepared before lane entry; qualified host-wide network denial; profile association rule
- ASR and risk links: ASR-04, ASR-05, TIR-2/TIR-8, RE-2; R-007, R-008
- Behavior-present handling: Not `BP`
- Versioned scenario contract: `contracts/readiness/scenarios/v1/b8-clean-forced-offline-quality.json`
- Scenario-contract digest: Unassigned; freeze the exact file before implementation entry
- Expected evidence artifact: Complete first-run frontend/Rust format, static, contract, production build, and test outputs with commit/lockfile/toolchain/command/network provenance
- Accountable role: Release
- Assignee: Unassigned
- Calendar date: Unassigned
- Implementation entry: Blocked by exact clean source, qualified denial, assignee/date, frozen profile, and Registrar
- Candidate subjects and invalidation: No candidate artifact subjects; association is permitted only when source commit/lockfiles match the candidate and never relabels source evidence candidate-bound
- Attempt contract: First run retained in full; `runnerRetryCount = "0"`; later authorized retry cannot replace the failure

**Acceptance Criteria:**

**Given** prepared pinned dependencies/toolchains and a fresh clean checkout matching the candidate commit/lockfiles
**When** the forced-offline lane begins
**Then** outbound network is denied and the exact required frontend/Rust formatting, static checks, contracts, production builds, and tests execute with complete first-run output.

**Given** a first-run failure
**When** a later authorized retry is requested
**Then** the failure remains indexed, the retry is a new linked attempt with explanation, and the original output is never replaced.

**Given** a passing source-bound attempt
**When** candidate Evidence Set association is evaluated
**Then** commit/lockfiles/profile must match exactly, binding remains source-level, and `F12-AC1` becomes only **eligible for later FULL reassessment**.

### Story 8.5: Prove Default-Test Isolation Beyond Browser Fetch

As a maintainer,
I want default tests to reject real network, Manager process, sleep, and machine-state dependencies,
So that forced-offline reproducibility is an enforced behavior rather than a convention.

**Story Contract:**

- Criteria and historical baseline: `F12-AC2` — `PARTIAL`
- FR and requirement links: No direct FR primary mapping; TIR-2 default-test isolation
- Primary readiness concern: Reusable Test Infrastructure
- Checkpoint: Batch 8
- Required test level: Unit plus clean-checkout CI
- Execution lane / evidence depth: `forced-offline` / source-bound
- Dependencies: Accepted Epic 7; unchanged Candidate Manifest source association; qualified ASR-05 denial; controlled process/network/DNS/service-worker/host seams
- ASR and risk links: ASR-02, ASR-05, TIR-1/TIR-2/TIR-8; R-002, R-004, R-008
- Behavior-present handling: `BP`; any missing or incorrect product behavior creates Product Behavior work before regression credit, and any deterministic-seam defect creates separate infrastructure work
- Versioned scenario contract: `contracts/readiness/scenarios/v1/b8-default-test-isolation.json`
- Scenario-contract digest: Unassigned; freeze the exact file before implementation entry
- Expected evidence artifact: `b8-default-test-isolation.json` with dependency-attempt inventory and rejection results across process, network, DNS, service-worker, time, filesystem, and host-state paths
- Accountable role: QA
- Assignee: Unassigned
- Calendar date: Unassigned
- Implementation entry: Blocked by qualified denial, assignee/date, and profile/admission prerequisites
- Candidate subjects and invalidation: No candidate artifacts; source association requires exact candidate source and never deepens binding
- Attempt contract: First isolation run retained; `runnerRetryCount = "0"`

**Acceptance Criteria:**

**Given** the default frontend and Rust suites
**When** deliberate real network, real Manager process, wall-clock sleep, DNS, service-worker, undeclared filesystem, and mutable-host dependencies are introduced or attempted
**Then** isolation detects/rejects them and directs the suite through the documented deterministic seam.

**Given** ignored or live-only checks
**When** the default suite runs
**Then** they are visibly excluded from PASS counts and cannot be reported as executed evidence.

**Given** the behavior-present and clean-checkout isolation matrix passes
**When** source-bound admission occurs
**Then** `F12-AC2` becomes only **eligible for later FULL reassessment**.

### Story 8.6: Attest Cross-Asset Authenticity and Keep No-Sign Smoke Separate

As a Release Owner,
I want every release asset and updater reference mutually authenticated and version-consistent,
So that a no-sign smoke or static workflow cannot substitute for the signed candidate.

**Story Contract:**

- Criteria and historical baseline: `D25A-AC2` — `INTEGRATION-ONLY`
- FR and requirement links: FR-22; RE-2; RE-3; RE-6
- Primary readiness concern: Candidate-Specific Release Evidence
- Checkpoint: Batch 8
- Required test level: Artifact/release attestation
- Execution lane / evidence depth: `candidate-release` / candidate-bound
- Dependencies: Stories 8.1–8.5; unchanged signed candidate; final published metadata/assets
- ASR and risk links: ASR-04, ASR-05, TIR-8, RE-2/RE-3/RE-6; R-006, R-007, R-008
- Behavior-present handling: Not `BP`
- Versioned scenario contract: `contracts/readiness/scenarios/v1/b8-cross-asset-authenticity.json`
- Scenario-contract digest: Unassigned; freeze the exact file before implementation entry
- Expected evidence artifact: `b8-cross-asset-authenticity.json` with cross-asset version/name/hash/URL/signature/key relations and no-sign-smoke exclusion
- Accountable role: Release
- Assignee: Unassigned
- Calendar date: Unassigned
- Implementation entry: Blocked by unchanged published candidate set, assignee/date, and frozen profile slot
- Candidate subjects and invalidation: All five manifest subjects with profile-fixed inspect/serve/verify roles; any replacement or metadata change creates a new root and reruns this slot
- Attempt contract: First attestation retained; `runnerRetryCount = "0"`

**Acceptance Criteria:**

**Given** the candidate tag, versions, DMG, ZIP, updater archive, metadata, and detached signature
**When** cross-asset attestation runs
**Then** names, versions, raw hashes, URLs, embedded updater key, and signature relationships agree exactly with the manifest.

**Given** a credentialless or `--no-sign` build smoke
**When** candidate admission is attempted
**Then** it remains explicitly non-candidate build evidence and cannot satisfy any signed asset subject.

**Given** a valid first candidate-bound attestation
**When** the Registrar admits it
**Then** `D25A-AC2` becomes only **eligible for later FULL reassessment**.

### Story 8.7: Complete and Replay the Evidence Ledger for Trace Handoff

As a Release Owner,
I want one complete, single-head Evidence Index replayed against the unchanged manifest and profile,
So that QA and Development can hand a coherent Evidence Set to a later Trace workflow without claiming readiness.

**Story Contract:**

- Criteria and historical baseline: None; ledger completion and Trace handoff add no denominator row and duplicate no primary allocation
- FR and requirement links: No direct FR implementation; RE-10/RE-11 and GP-1/GP-2 evidence-handoff governance
- Primary readiness concern: Candidate-Specific Release Evidence
- Checkpoint: Batch 8 exit
- Required test level: Evidence-contract replay, completeness aggregation, and negative governance validation
- Execution lane / evidence depth: Evidence aggregation across mapped lanes/depths without relabeling; final index is manifest/profile-bound
- Dependencies: All preceding Epic 1–8 criterion/RP/enabler stories; unchanged Candidate Manifest; frozen Acceptance Profile; protected Registrar; complete immutable objects and records
- ASR and risk links: ASR-04 — Release accountable; ASR-05; TIR-8, RE-10/RE-11, GP-1/GP-2; R-001 through R-008
- Behavior-present handling: Every approved behavior-present reclassification must already be represented in the frozen map/profile; aggregation cannot repair or hide missing behavior
- Versioned scenario contract: `contracts/readiness/scenarios/v1/b8-ledger-trace-handoff.json`
- Scenario-contract digest: Unassigned; freeze the exact file before implementation entry
- Expected evidence artifact: Complete `evidence-index.ndjson`, immutable index snapshots/records/objects/Registrar attestations, replay report, slot-completeness report, and Trace handoff manifest; no regenerated trace/gate decision
- Accountable role: Release
- Assignee: Unassigned
- Calendar date: Unassigned
- Implementation entry: Blocked until all required records, objects, assignee/date, approved policy/profile, and unchanged candidate exist
- Candidate subjects and invalidation: All profile-required candidate subjects across the five manifest logical IDs; candidate mutation creates a new root and makes prior candidate-bound records ineligible
- Attempt contract: Every slot has exactly one retained ordinal 1 with `runnerRetryCount = "0"`; authorized retries are gapless linked chains under the approved DR-4 disposition

**Acceptance Criteria:**

**Given** the complete stored index from sequence `00000001`
**When** replay recomputes canonical payload/record/index digests, sequence/predecessor, idempotency, manifest/profile binding, raw object hashes, producer/Registrar identity, subjects, attempts, and human/machine agreement
**Then** one valid head reproduces the complete `evidence-index.ndjson` byte-for-byte
**And** missing objects, stale/forked heads, clobber, branches, automatic retries, wrong lane/depth/source/candidate, or incomplete PASS counts fail closed.

**Given** all 72 P0 criteria, all 14 historical-FULL revalidation checkpoints, RP-1, RP-2, and all score-6/9 mitigation slots
**When** completeness aggregation runs
**Then** every required slot is present at its exact profile lane, minimum depth, environment, subject set, and retry disposition
**And** source-/environment-bound evidence is associated only where permitted without relabeling.

**Given** a valid complete ledger and unchanged candidate
**When** the handoff package is produced
**Then** QA and Development receive the exact Manifest/Profile/Index digests and immutable result links
**And** the package states only “eligible to invoke the later candidate-bound Trace workflow”
**And** this story does not regenerate traceability, move any criterion to FULL, or claim readiness.
