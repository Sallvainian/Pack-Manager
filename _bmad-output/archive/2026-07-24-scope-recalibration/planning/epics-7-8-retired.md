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

**Given** the finalized packaged shell — a persistent editable Upgrade Plan sidecar, an eligible Package Grid of at least 101 rows, and Managers as a disclosure — at 100%, 150%, and 200% zoom in the 900×600 minimum window
**When** one roving row Tab stop, stable virtual Package identity, total/row-position metadata, exact filter-wide bulk scope, final-row reachability, and the high-zoom navigation collapse are exercised
**Then** the Package Grid preserves a single roving row focus with the final row and its actions reachable, and at 150–200% zoom navigation collapses so the Upgrade Plan, Activity, and Results present as full-workspace or stacked surfaces with no overlapping panes or two-dimensional scrolling for the primary task.

**Given** the separate Confirmation Dialog, the shared sidecar and full Activity surfaces, the persistent Results Summary, and the Settings `Pack-Manager Update Ready!` badge inside packaged WKWebView
**When** dialog focus trapping/restoration, the one atomic Activity/Results announcement channel, and the application-update badge presentation are evaluated with VoiceOver at 100%, 150%, and 200% zoom
**Then** the Confirmation Dialog traps focus on its heading and command summary and restores focus on dismissal, the shared Activity and Results surfaces announce plan start, each Manager's completion summary, and the final outcome without overlap, and the `Pack-Manager Update Ready!` badge stays reachable and separate from Package Upgrade Plans, Activity, Results, and History
**And** manual VoiceOver confirms dialog focus order and Activity/Results announcements without implying broader WCAG/legal compliance.

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

**Given** the installed prior version with an authorized update ready
**When** the finalized application-update presentation renders in the title/status area and in Settings → Pack-Manager updates
**Then** one restrained application-level badge labeled `Pack-Manager Update Ready!` announces availability and links to Settings → Pack-Manager updates, and the update card heads simply `Pack-Manager` and shows the installed-to-target version delta on one line with the installed version in warning yellow and the target version in success green
**And** the badge and card use the finalized presentation without entering Package Activity or History and stay separate from every Package Upgrade Plan and Results.

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
So that application updates remain understandable and separate from draft Upgrade Plans, live plan attempts, Results, and plan-attempt History.

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
- Expected evidence artifact: `rp-1-update-state-continuity.json` with trigger timing/menu invocations, UI recreation state, relaunch result, error/retry policy, and separation from draft Upgrade Plans, live plan attempts, Results, and plan-attempt History
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
**When** the draft Upgrade Plan(s), the live confirmed plan attempt (`planAttemptId`), Results, and plan-attempt History are inspected
**Then** application-update state remains separate from every one of those plan surfaces and no application update is admitted into a draft plan, a confirmed plan attempt, Results, or plan-attempt History
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
**Then** both hosts prove intended version, packaged resources/entitlements, GUI ToolEnv discovery, production WKWebView, and reach an interactive first paint.

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
- Approved map/profile revision scope: The story freezes and replays only the later approved revision-2 Candidate Manifest map and Acceptance Profile and their updated scenario-contract digests; the superseded revision-1 map/profile is not admitted, and the frozen map/profile referenced throughout is that revision-2 pair
- Superseded-evidence preservation: `AUT-003` is retained as historical evidence of superseded behavior, carried through the complete Evidence Index unchanged, and must not support revised `F5-AC3`; aggregation preserves it as superseded and never repairs, relabels, or promotes it
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

**Given** only the later approved revision-2 map/profile, its updated scenario-contract digests, and the retained superseded-evidence record `AUT-003`
**When** the ledger is frozen, replayed, and aggregated for handoff
**Then** replay and completeness consume only the revision-2 map/profile and its updated digests, and any earlier revision-1 map/profile is refused
**And** `AUT-003` is preserved as historical superseded-behavior evidence, never supports revised `F5-AC3`, and is neither repaired, relabeled, nor promoted.
