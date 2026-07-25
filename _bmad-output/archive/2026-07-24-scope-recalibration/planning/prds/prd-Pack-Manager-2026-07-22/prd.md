---
title: Pack-Manager 100% P0 Product-and-Release Readiness Gate
status: final
created: 2026-07-22
updated: 2026-07-24
artifact_revision: 2
---

# PRD: Pack-Manager 100% P0 Product-and-Release Readiness Gate

> **Current planning-gate status: FAIL — 14 of 72 P0 criteria FULL (19.4%).**

## 0. Document purpose and authority

This brownfield PRD defines what Pack-Manager must mean by **100% P0
product-and-release ready**. It is for the product owner, engineering, QA, and
release owners who must decide whether one specific release candidate is safe
to ship. It defines required outcomes and evidence; it does not prescribe an
implementation or authorize product-code changes.

Readiness is evaluated through the three independent lanes defined in §3.1.
They may support each other, but they are not interchangeable.

### 0.1 Source authority

Authority is domain-specific:

- **Product behavior:** `docs/SPEC.md`, amended by later explicit entries in
  `docs/DECISIONS.md`.
- **Current mechanics:** production registration, configuration, manifests,
  lockfiles, and workflows.
- **Current readiness status:** the traceability matrix, limited by its recorded
  source and execution provenance.
- **Closure planning:** the test-design documents, which propose dependencies
  and evidence but are not themselves proof.
- **Readiness-gate policy:** this PRD only after explicit Product and QA
  approval.

While frontmatter status is `draft`, existing source authority remains
unchanged. After approval, this PRD governs readiness-gate policy only. It does
not alter product behavior or production mechanics unless the corresponding
SPEC, decision, or production source is updated.

D23a supersedes D23: `mas` is live-verified. D25/D25a supersede D20 and stale
ad-hoc-only delivery language: signed, notarized, stapled delivery and
user-controlled in-app updating are current product intent. D26 defines the
only permitted readability repair to otherwise faithful operation transcripts.
D27-D30 supersede immediate row execution, direct Manager-update execution,
Operation-row History, drawer-only Activity, and the obsolete
`autoOpenDrawer` setting. The finalized UX spines and approved
`sprint-change-proposal-2026-07-24.md` are authoritative for those revised
experience contracts.

Companion source extracts:

- `extract-product-intent.md`
- `extract-gap-evidence.md`
- `research-external-readiness.md`
- `readiness-coverage-map.md`
- `addendum.md`

## 1. Executive summary

Pack-Manager is a macOS desktop control plane for keeping software current
across Homebrew, mise, npm, uv, rustup, and `mas`. Its promise is confidence:
the user sees manager-reported truth, understands ownership and routing,
stages every update in one persistent Upgrade Plan, reviews the exact commands
before execution, controls every mutation, and can reconstruct what happened
afterward.

The status above comes from a traceability planning snapshot recorded at commit
`fe2881f3e48d26c0561857f72143c6570a5620fc` with a dirty working tree. The
snapshot reports **58 non-FULL** criteria. Because it is not reconstructible
from the commit alone, it is a planning baseline—not candidate-bound proof.

The later test-design plan provisionally assigns each of the 58 open rows to a
primary closure lane:

| Provisional primary lane           | Open P0 rows | Planning interpretation                                                                                                                                                                                        |
| ---------------------------------- | -----------: | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Product/source correction          |            1 | `D23a-AC4` is the one row classified as a confirmed source-truth defect. The repository contains at least two stale truth instances: `mas` described as unverified and notarization described as out of scope. |
| Test infrastructure/coverage       |           52 | 24 rows are provisionally test-only and 28 require native infrastructure. Behavior-present checks may move rows into product correction.                                                                       |
| Release evidence/operational proof |            5 | Only an immutable built candidate or live operational lane can provide proof.                                                                                                                                  |

The 1/52/5 split is not trace-matrix output, not a defect count, and not
permanently exclusive. A surface may have both a product-truth defect and a
release-evidence gap. The split exists to assign an initial closure owner; the
approved coverage map records secondary overlaps.

The latest recorded run is green—133 Vitest tests, 245 active Rust tests with
11 live/environment tests ignored, and 12 Playwright project executions
representing six logical cases across Chromium and WebKit. The Playwright lane
uses fake Tauri. These results do not establish the real JavaScript-to-Rust
boundary, packaged WKWebView behavior, target-Mac topology, updater
installation, or candidate authenticity and launchability. “More tests passed”
is therefore not the gate. The gate requires **proof of all required behavior
at the layer where it can fail, against the exact candidate proposed for
release**.

### 1.1 Gate thesis

Pack-Manager reaches 100% P0 product-and-release readiness only when one
immutable candidate passes the exit contract in §9.6. In summary, the approved
coverage map must report 72/72 P0 criteria FULL, the separate Release
Prerequisites must pass, and the exact candidate must satisfy the required
native and release evidence without a known P0 defect.

There is no weighted average across the three lanes and no “100%” result with a
waived P0 criterion. A gate-policy change made before validation changes the
rule for all candidates; a criterion waiver exempts a requirement; risk
acceptance acknowledges residual risk without changing the requirement. A
candidate-specific waiver or release-blocking risk acceptance can only produce
a conditional result, never 100% ready.

## 2. Product vision, user, and form factor

### 2.1 Vision

Pack-Manager gives a macOS power user one trustworthy place to understand and
update software managed by several command-line package managers. It removes
the need to remember each manager's syntax without removing the control and
transparency that make terminal workflows trustworthy.

The product is not merely a graphical command launcher. Its differentiator is
correctness across a mixed ownership topology: manager-reported outdatedness,
discovered self-update routing, conflict-aware scheduling, exact bulk plans,
isolated refresh failure, durable operation evidence, and a release path that
does not require privilege escalation.

This readiness effort is complete when Pack-Manager's existing P0 promise is
proven end to end. It is not an invitation to add adjacent P1 or P2 features.

### 2.2 Primary user and jobs to be done

The primary user is a macOS power user or developer who has several supported
package managers installed, including possible delegated relationships such as
npm or uv managed through mise.

The user needs to:

- see which supported Managers are installed, healthy, and current;
- understand which Manager owns a Package and how that Manager updates;
- update everything, a selected subset, one Package, or a Manager without
  accidentally changing excluded items;
- stage every update in one persistent plan and see its exact commands before
  authorizing it;
- understand queued, running, stalled, cancelled, failed, and interrupted work;
- recover useful state when one Manager or the network fails;
- export enough local evidence to diagnose a problem without disclosing the
  inherited environment; and
- install and update Pack-Manager itself through a trusted macOS flow while
  retaining control of restart.

### 2.3 Form factor and operating context

- Native macOS desktop application distributed outside the Mac App Store.
- Universal release for Apple silicon and Intel architectures.
- Launched from Finder, the Dock, or the app menu—not only from a terminal.
- Local application with no account, database, cloud service, or telemetry.
- Operates alongside package-manager processes that the user may run in a
  terminal.

### 2.4 Explicit non-users

- Enterprise fleet administrators seeking centralized device management.
- Users seeking package installation, removal, or arbitrary shell execution.
- Users seeking unattended upgrades or password/administrator workflows.
- Non-macOS users.

## 3. Readiness model and vocabulary

### 3.1 Three independent lanes

**Lane A — Product Behavior**

Defines the observable user promise. A product requirement is either present
and correct, absent, or defective. Missing evidence alone must not be reported
as a product defect.

**Lane B — Test Infrastructure**

Defines reusable evidence-producing capabilities: deterministic offline
coverage, real Tauri boundary exercise, controllable process/lifecycle
conditions, a provisioned target-Mac lane, and packaged-app acceptance. Test
infrastructure is not a user-facing feature and does not enlarge product scope.

**Lane C — Release Evidence**

Defines candidate-bound proof: identity, artifacts, architecture, signing,
notarization, stapling, launch, updater integrity, installation, and final gate
attestation. Static workflow text or an artifact upload attempt is not release
evidence.

### 3.2 Definition of FULL

A P0 criterion is FULL only when:

1. the required behavior is present;
2. its acceptance consequences pass;
3. the evidence crosses every material boundary where the behavior can fail;
4. the evidence provenance identifies the tested source and, where applicable,
   the exact candidate;
5. ignored tests, source inspection, plans, and unexecuted collectors are not
   counted as passes; and
6. a retry does not erase or replace the first failure.

`readiness-coverage-map.md` is the normative companion for the legacy
denominator. Before implementation planning, every one of the 72 P0 criteria
must map one-to-one to its source priority, primary PRD requirement, readiness
lane, closure batch, baseline status, and any secondary defect/evidence
overlap. A missing, duplicated, or silently reprioritized row blocks use of the
72/72 claim.

Evidence depth is requirement-specific:

- Pure logic may close with deterministic offline evidence.
- Tauri registration, serialization, events, native actions, process signals,
  persistence, and relaunch require native evidence.
- Signing, notarization, packaged launch, and updater installation require
  candidate-bound release evidence.

### 3.3 Glossary

- **Manager** — One supported package manager: Homebrew, mise, npm, uv, rustup,
  or `mas`.
- **Package** — One Manager-owned update unit such as a formula, cask, tool,
  global package, toolchain, or App Store application.
- **Outdated** — A Manager's verdict that an update is available. Pack-Manager
  does not infer Outdated status from version comparison.
- **Snapshot** — The latest merged installed-and-outdated view for one Manager.
- **Last-good Snapshot** — The most recent successful Snapshot retained when a
  newer refresh fails.
- **Route** — The discovered way a Manager updates: in-band, through another
  Manager, through refresh, or unavailable.
- **Operation** — One queued unit of refresh, Package update, Manager update, or
  narrowly recognized health fix.
- **Upgrade Plan** — The persistent editable draft containing every selected
  Package and Manager update plus native-produced commands, exclusions,
  warnings, and notes.
- **Plan Capability** — One-use authorization bound to a reviewed Upgrade Plan
  and current coherent state; represented by the short-lived `planId`.
- **Plan Attempt** — One confirmed execution of an Upgrade Plan, durably
  identified by `planAttemptId` and correlated to its Operations, verification,
  Results, History entry, and optional Retry parent.
- **Release Candidate** — One immutable build proposed for release, identified
  by source commit, tag, product versions, and artifact checksums.
- **Evidence Set** — The retained results that prove the Release Candidate
  meets this PRD.
- **FULL** — The traceability status defined in §3.2; not a synonym for a test
  file existing or a test suite being green.

### 3.4 Scope and non-goals

#### In scope

- Existing P0 product capabilities F1–F12 from `docs/SPEC.md`.
- Current application self-update behavior from D25/D25a.
- PC-1 correction of the confirmed stale `mas` and notarization truth.
- Evidence infrastructure for the 52 rows provisionally assigned to the test
  lane, subject to behavior-present reclassification.
- Candidate-bound proof for the five rows provisionally assigned to the release
  lane and any overlapping product correction.
- Regenerated traceability and an unambiguous final P0 gate decision.

#### Out of scope

- Implementing this PRD within this documentation run.
- Adding P1 product behavior other than the explicitly retained RP-1 and RP-2:
  health fixes, snapshot cache, native notifications, detail popovers, or the
  Rust ownership note.
- Adding P2 behavior: light theme, menu-bar extra, scheduled Package refresh,
  broader cross-Manager deduplication, or `cargo install`.
- Package installation or removal.
- Unattended Package or Manager updates.
- Silent application installation/restart.
- `sudo`, password, or administrator-prompt workflows.
- Telemetry, accounts, cloud synchronization, or enterprise fleet management.
- A new distribution channel, release framework, or updater provider.
- General rollback of partially completed external Package-manager work.
- Treating a synthetic fixture as proof of a live Manager format.

## 4. Product acceptance journeys

This is a capability-first readiness PRD for a single-operator utility. It does
not invent personas. The following source-defined journeys identify the
end-to-end product experiences that must be proven.

### AJ-1 — Launch, detect, and refresh

The user launches Pack-Manager from Finder or the Dock. The app renders
progressive state, detects all supported Managers, resolves ownership, and
refreshes present Managers independently. An absent Manager is normal. A failed
Manager retains its Last-good Snapshot, is marked stale, and offers a useful
retry or log path while other Managers remain usable.

### AJ-2 — Review and authorize Update Everything

The user requests Update Everything. Every eligible Package and Manager update
enters the persistent Upgrade Plan as independent removable membership. The
sidecar shows Package version deltas, exclusions, warnings, and stale-data
conditions and reveals the exact Manager-grouped commands on request. Its
single blue action opens the separate final Confirmation Dialog by default. If
state changes, Rust replaces the stale preview and requires review again. A
valid confirmation creates one durable Plan Attempt and admits it all-or-none;
independent Managers may proceed concurrently inside that attempt.

### AJ-3 — Update a selected Package or Manager

The user filters and selects eligible outdated Packages or invokes one
row-level update. Each action adds exact membership to the same persistent plan
without executing. A Manager title area explains the discovered Route, subject,
executor, unavailable state, and topology-specific consequences; its update
action also adds removable plan membership. The user follows the common
confirmation path, and successful work remains Verifying until affected
Manager state is refreshed.

### AJ-4 — Handle slow, blocked, failed, cancelled, or interrupted work

The active plan sidecar and Activity show human-readable per-item progress,
exact commands, and live output. Silence becomes an explicit stalled state with
Keep waiting, Copy command, and Cancel plan choices. Only trusted
Manager-specific classification may produce Interaction required. External
Homebrew contention is named and not retried automatically. Plan cancellation,
timeout, verification failure, and skipped work produce explicit Results.
After a crash or forced quit, the unfinished durable Plan Attempt appears as
Interrupted without signaling a historical process identifier.

### AJ-5 — Diagnose and export support evidence

The user searches one-row-per-attempt History and opens a prior plan as
read-only Activity replay with nested command/outcome/transcript evidence.
Retry rebuilds a new linked attempt without overwriting the original. The user
can export one diagnostics archive that contains correlated plan/Operation
state and evidence without inherited environment disclosure or symlink
substitution.

### AJ-6 — Install and update Pack-Manager

The user installs a trusted direct-download build and launches it through
normal macOS entry points without a security bypass. A prior public version
discovers and downloads the Release Candidate, then waits for the user's
explicit Restart to update action. The app relaunches as the intended version.
If the install location is not writable, the app explains manual installation
and never requests administrator privileges.

## 5. Product behavior requirements

This section contains only user-visible behavior and product invariants. Test
harnesses, CI topology, evidence files, signing commands, and release workflow
mechanics belong in §§6–7.

### 5.1 Manager discovery and state truth

#### FR-1: Detect supported Managers

Pack-Manager shall detect Homebrew, mise, npm, uv, rustup, and `mas` at launch
and on demand.

**Consequences:**

- Each present Manager shows detected path, version when available, ownership,
  and human-readable evidence.
- Each absent Manager shows a normal Not installed state rather than an error.
- When a known installation command exists, the absent state includes a
  copyable install hint.
- Detection works when the app is launched from Finder or the Dock.
- Re-detect replaces prior detection state only with a coherent new result.

#### FR-2: Preserve Manager-reported update truth

Pack-Manager shall mark a Package Outdated only when its Manager reports it as
Outdated.

**Consequences:**

- Manager-supplied version strings are displayed verbatim.
- Version-delta styling is explanatory only and never decides eligibility.
- An unknown latest version remains unknown and is never fabricated.
- Parser incompatibility fails visibly for that Manager rather than crashing
  the application or presenting an invented result.

#### FR-3: Refresh Managers independently

Pack-Manager shall refresh installed inventory and Outdated state per Manager
without allowing one failure to erase other useful state.

**Consequences:**

- Refresh All starts every present Manager while permitting independent work
  to proceed concurrently.
- Loading, phase, timeout, offline, and error states are shown per Manager.
- Duplicate refresh requests for the same Manager coalesce to the existing
  queued or running Operation rather than creating competing refreshes.
- A failed refresh retains and labels the Manager's Last-good Snapshot.
- Successful Package or Manager updates refresh all affected Manager state.

#### FR-4: Discover and explain ownership and update Routes

Pack-Manager shall derive Manager ownership and self-update Routes from current
detection and refresh information.

**Consequences:**

- Ownership evidence is inspectable by the user.
- Routes are reconsidered after fresh Manager data.
- A routed update explains both subject and executor in plain language.
- If the executor is unavailable, the action is disabled with the reason.

#### FR-5: Present Package state and eligibility

Pack-Manager shall let the user browse, search, filter, and understand Packages
without losing Manager-specific detail.

**Consequences:**

- Rows distinguish current, Outdated, pinned, self-updating/greedy, unknown
  version, and error states.
- Pinned Homebrew formulae are never eligible for in-app update.
- Self-updating/greedy casks are grouped separately, collapsed by default, and
  excluded unless the user explicitly opts in.
- When mise and rustup both report the Rust toolchain inside one Upgrade Plan,
  the plan updates it through rustup and visibly explains the mise exclusion;
  no broader cross-Manager deduplication is performed.
- Useful Manager details, such as Package kind or executable information,
  remain available.

### 5.2 Reviewable planning and intentional selection

#### FR-6: Select only eligible Packages

Pack-Manager shall support precise selection of eligible Outdated Packages.

**Consequences:**

- Individual, range, toggle, filter-aware select-all, tri-state, and clear
  interactions preserve the exact selected identities.
- Current, pinned, and default-excluded Packages cannot enter the selection.
- `Add to Plan` builds persistent draft membership from exactly the selected
  identities; it does not execute.
- A direct `Update Package` action adds that one eligible Package to the same
  draft and never bypasses the plan.
- The plan persists while the user changes Manager views.
- Selection clears only after the requested identities are admitted to the
  draft.

#### FR-7: Preview every update command exactly

Pack-Manager shall show the exact commands for every Package and Manager update
before the user authorizes execution.

**Consequences:**

- Row-level Package updates, Manager self-updates, Update Everything,
  per-Manager update-all, and update-selected use one persistent Upgrade Plan.
- Commands contain explicit Package names; a bare indiscriminate update is not
  accepted as a Package plan.
- Manager updates are independent removable plan items rather than one global
  include/exclude toggle.
- The preview identifies warnings and possibly stale inputs before confirmation.
- `Show Update Command` reveals native-produced commands inside the plan.
- By default `Confirm N Updates` opens a separate final dialog that repeats the
  commands and asks `Proceed with Upgrade Plan?`.
- The confirmation preference may skip only that dialog; it cannot skip plan
  staging, native rebuild, or explicit user action.

#### FR-8: Reject stale, altered, replayed, or invalid plans

Pack-Manager shall execute a plan request only when it exactly matches the
reviewed Upgrade Plan and a fresh coherent rebuild from current state.

**Consequences:**

- A changed state produces a replacement Upgrade Plan and requires another
  explicit confirmation.
- Tampering, replay, eviction, missing authorization, active refresh, or
  conflicting mutation drift enqueues nothing.
- A Plan Capability is bounded and one-use.
- Dismissing or superseding a plan prevents late results from changing state or
  starting work.

#### FR-9: Admit multi-group plans atomically

Pack-Manager shall admit a confirmed multi-group Upgrade Plan all-or-none.

**Consequences:**

- Admission failure leaves no subset queued.
- Conflicting Operations serialize before start.
- Independent Managers may proceed concurrently within the global resource
  limit.
- Only one confirmed plan attempt may be active at a time.
- Queue state explains blocking subject/executor relationships.
- External Homebrew contention is named and never retried automatically.

#### FR-10: Support intentional single-Package updates

Pack-Manager shall provide a lower-friction row-level update for one eligible
Package.

**Consequences:**

- The row action immediately adds exactly one Package to the persistent Upgrade
  Plan; it does not start an Operation.
- Starting the attempt requires the common plan confirmation action.
- The exact command is available in the plan, Activity, History, and transcript.
- The action uses the same eligibility, Route, conflict, and no-privilege
  protections as bulk work.
- A single-Package update does not silently expand to unrelated Packages.

#### FR-11: Explain Manager self-update behavior

Pack-Manager shall give each Manager a standardized title area that explains
its purpose, path, installed/latest state, Route, and action availability.

**Consequences:**

- In-band, delegated, refresh-based, and unavailable Routes are distinguishable.
- A Manager's own update row is not duplicated as an ordinary Package row.
- npm-inside-mise consequences remain visible at the point of action.
- Manager-status badges use consistent `NO UPDATES` or `UPDATE AVAILABLE`
  language and distinguish the Manager's update from Package-update counts.
- The Manager update action adds independent membership to the Upgrade Plan.
- Routed work identifies its subject/executor relationship without unexplained
  route metadata.

### 5.3 Safe and reconstructible Operation lifecycle

#### FR-12: Exclude arbitrary shell and privilege paths

Pack-Manager shall execute only product-defined Operations, without any general
shell command, `sudo`, password entry, or administrator prompt path.

**Consequences:**

- Display text is never treated as executable input.
- Operations cannot wait for stdin from the app.
- Copy-to-terminal is a user-controlled handoff, not hidden fallback execution.
- A required elevated application update becomes manual-install-required.

#### FR-13: Show live plan and Operation state

Pack-Manager shall expose queued, running, verifying, stalled, cancelling, and
terminal plan state, with nested Operation commands and live output.

**Consequences:**

- Subject, executor, queue reason, command, stdout/stderr, and status remain
  correlated by both Operation identity and durable Plan Attempt identity.
- The running Upgrade Plan sidecar becomes progress and then a Results summary.
- Activity is a first-class destination for the active attempt and for replay
  from History; a draft plan is not queued Activity.
- A verified Package completion collapses its displayed delta to the single new
  version.
- The live surface remains bounded while the durable transcript preserves the
  complete retained output.

#### FR-14: Handle stalls, timeouts, and cancellation honestly

Pack-Manager shall turn a silent or overlong Operation into an actionable state
and shall cancel the requested scope without a confirmation dialog.

**Consequences:**

- The default stall threshold is 120 seconds and offers Keep waiting, Copy
  command, and Cancel.
- `Interaction required` is emitted only by a closed Manager-specific classifier
  or explicit trusted native signal; unknown silence remains an ordinary stall.
- The default hard cap is 30 minutes and ends in an explicit timeout state.
- Primary `Cancel plan` stops queued work in the active attempt, escalates
  running process groups when needed, and preserves terminal cancelled, failed,
  and skipped outcomes.
- Quitting with running Operations lists them and requires the user to choose
  Cancel operations and quit or Keep running.
- The product does not promise rollback of a partially completed external
  package-manager command.

#### FR-15: Preserve History, transcripts, and crash evidence

Pack-Manager shall durably record enough plan-attempt and Operation evidence to
reconstruct what was reviewed, what ran, and what happened.

**Consequences:**

- Every confirmation creates a durable `planAttemptId` distinct from the
  one-use preview `planId`.
- Reviewed scope, command snapshot, start, incremental output, finish, subject,
  executor, verification, and Results remain correlated.
- Start-without-finish attempts appear as Interrupted after relaunch.
- Historical process identifiers are never signaled after relaunch.
- History contains one immutable entry per confirmed attempt. Opening an entry
  replays its detailed Activity and nested Operations.
- Retry creates a new attempt linked to the failed attempt and never overwrites
  the first result.
- Legacy Operation records without an attempt identity remain visibly legacy
  and are never fabricated into plan groups.
- Transcripts preserve child output except that a closed literal allowlist of
  known unterminated `mas` notices may receive one readability newline only
  when subsequent output is glued behind the notice.
- A normally terminated notice, a near match, repeated text, unrelated output,
  or a generic mid-line marker is not rewritten.
- Retention keeps the newest 1,000 History records and the newest 200
  transcripts or 90 days.
- Daily application logs older than 14 days are pruned.

#### FR-16: Preserve useful state after Operation outcomes

Pack-Manager shall make success and failure actionable without destroying
previously valid state.

**Consequences:**

- Successful updates refresh affected Managers.
- A plan remains Verifying until required post-mutation refreshes resolve.
- Terminal Results distinguish successful, failed, skipped, cancelled,
  interrupted, and verification-failed work.
- A failure retains prior useful Manager state and displays the new failure.
- Error feedback says what happened, identifies a likely next action when known,
  and retains Retry without implying that deterministic failures will fix
  themselves.
- A View log action appears only when a corresponding log exists.

### 5.4 Settings, diagnostics, and interface quality

#### FR-17: Persist Settings atomically

Pack-Manager shall expose product Settings and apply a change only after it is
successfully persisted.

**Consequences:**

- A failed save changes neither the active value nor the persisted value.
- Defaults remain: Homebrew metadata refresh on, launch refresh on, 120-second
  stall threshold, 30-minute hard cap, upgrade-plan confirmation on, and greedy
  casks off.
- `skipUpgradePlanConfirmation` defaults to false and is reversible from
  Settings. The obsolete `autoOpenDrawer` value is inactive legacy input.
- The application log level is configurable and defaults to debug for
  Pack-Manager's own code.
- Install threshold, operation hard cap, and application log level are editable.
- Settings includes a read-only Environment Report, Copy, Open Logs Folder,
  diagnostics export, and Re-detect.

#### FR-18: Export privacy-preserving diagnostics

Pack-Manager shall export one timestamped diagnostics archive containing the
information needed to investigate detection, routing, Settings, and recent
Operations.

**Consequences:**

- The archive is written to
  `~/Desktop/Pack-Manager-diagnostics-<YYYYMMDD-HHmmss>.zip`.
- It contains `report.json`, the last three application-log files, the last 25
  transcripts, and `operations.jsonl`.
- The `report.json` file contains app, OS, and architecture information; the
  resolved search path and source; the full detection report and evidence;
  Settings; and the log filter.
- It includes only environment values Pack-Manager explicitly constructed.
- It never dumps the inherited environment.
- File selection and streaming reject symlink substitution.

#### FR-19: Provide one coherent and accessible macOS interface

Pack-Manager shall provide a dark-only launch experience whose primary actions
remain understandable and operable by keyboard and non-color cues.

**Consequences:**

- Dashboard, Sidebar, Manager panes, Upgrade Plan, Activity, History, Settings,
  status, dialogs, and app-menu actions use consistent state language.
- Managers expands in the Sidebar to reveal individual Manager destinations;
  the active Upgrade Plan persists across those destinations.
- Primary refresh, selection, update, search, navigation, and Activity paths
  are keyboard operable with visible focus.
- At high zoom and the minimum window, primary navigation, plan membership,
  confirmation, cancellation, Results, and recovery actions remain reachable
  without clipped or overlapping controls.
- Dialog opening, closing, validation, and completion use deterministic focus
  restoration and VoiceOver announcements.
- `VersionDelta` remains the product's visual signature: it shows
  `installed → latest` in monospace, highlights only comparable changed
  segments, keeps a text-labeled severity chip, and never changes the Manager's
  Outdated verdict.
- The Sidebar, Dashboard, and Manager panes preserve one-glance ownership,
  count, phase, error, queue, and Route information instead of flattening the
  product into six generic command wrappers.
- Status never relies on color alone, reduced motion is honored, operation
  completion is announced, and text contrast reaches at least 4.5:1.
- The app remains usable at its minimum window size, with more than 100 Package
  rows, and with long Operation output.

### 5.5 Pack-Manager application updates

#### FR-20: Expose and automatically download available application updates

When an application update check finds a newer release, Pack-Manager shall
download it automatically in the background and expose its state.

**Consequences:**

- Checking and downloading do not install or restart the application.
- Package-management work remains understandable while a download occurs.
- The app communicates the checking, available, downloading, ready, and
  failure states.

#### FR-21: Require explicit installation and relaunch

Pack-Manager shall install a downloaded application update only after the user
chooses Restart to update.

**Consequences:**

- The application never silently installs or restarts.
- The app refuses installation and relaunch while any Package Operation is
  queued or running; the user is told to let it finish or cancel it first.
- The relaunched app reports the intended new version.
- A non-writable installation location produces manual-install-required without
  an administrator prompt.
- Check, download, signature, install, and relaunch failures remain actionable
  and do not masquerade as success.

#### FR-22: Launch normally and reject unauthorized updates

Pack-Manager shall support the declared Apple silicon and Intel user promise
through normal macOS launch and an update path that accepts only authorized
Pack-Manager payloads.

**Consequences:**

- A normal downloaded install launches through Finder and the Dock without a
  security bypass.
- The updater accepts only a payload authorized for the installed application.
- The installed application communicates a successful update only after it
  relaunches as the intended version.

### 5.6 Release Prerequisites outside the 72-row P0 denominator

These existing application-update behaviors are P1 in the legacy oracle but
are mandatory prerequisites for releasing the current product. They are
validated separately and do not change the 72-row P0 denominator.

#### RP-1: Preserve application-update triggers and state continuity

- Update checks run at launch, every six hours, and on demand from the
  Pack-Manager app menu.
- During one app process, initial UI mount and supported window/UI recreation
  restore the current check, error, progress, downloaded, or ready state.
- A normal app relaunch starts from saved trigger policy and performs the
  launch check; the product does not claim that an in-progress download
  survives termination.
- An updater-driven restart relaunches into Current state for the installed
  version. A failed or interrupted download enters Error and may retry only
  through the next manual or policy-triggered check; it never appears Ready
  without a complete authorized download.
- Application-update state remains separate from Package Operation queue and
  History state.
- A non-Operation application update never appears as a Package History record.

#### RP-2: Preserve standard macOS menu behavior

- The custom app menu retains the standard Edit and Window actions.
- Cut, copy, paste, and select-all continue to work in Package search and every
  copyable command surface.

## 6. Test infrastructure readiness requirements

This section defines evidence-producing capability, not product behavior. A
TIR can be complete while a product defect still exists; conversely, correct
behavior cannot be called ready until the appropriate TIR can prove it.

### TIR-1: Behavior-present checks and honest classification

- Every gap currently labeled test-only begins with a check that the required
  behavior exists.
- Missing or incorrect behavior is reclassified into the product lane before a
  regression test is accepted.
- PC-1 in §9 must be satisfied before a D23a recurrence guard can count; the
  correction itself is product/source-correctness work, not test
  infrastructure.
- Coverage status uses the §3.2 FULL definition rather than test-file presence.

### TIR-2: Deterministic forced-offline lane

- Default frontend and Rust validation runs from a clean checkout without real
  network access, real Manager processes, sleeps, or undeclared machine state.
- Isolation covers relevant network and process paths, not only ordinary
  browser `fetch`.
- Live and release checks remain separate so they do not contaminate the
  default lane.
- Parsing, routing, scheduling, stale-plan, settings, diagnostics, keyboard,
  and contract behavior has deterministic success and failure coverage.
- D26 coverage proves the exact glued-notice case and the normally terminated,
  repeated, near-match, and unrelated-output boundaries so the exception cannot
  become heuristic transcript rewriting.

### TIR-3: Real native command-and-event boundary

- A reusable native lane crosses actual frontend invocation, Tauri
  serialization/registration, Rust handlers, and representative events.
- The lane isolates application state and controlled executables.
- It proves startup subscription/order, detection, Re-detect, and six-Manager
  refresh behavior across the real boundary.
- Browser evidence using a fake bridge is retained for UI value but is not
  mislabeled native evidence.

### TIR-4: Controllable process and operating-system boundaries

- Evidence can deterministically produce stdout, stderr, silence, exit,
  expected nonzero exit, cancellation, escalation, timeout, and external-lock
  conditions.
- It can prove null input, no-password behavior, complete process cleanup, and
  queue/lock timelines without invoking unsafe real mutations.
- Opener, reveal, restart, executable discovery, writability, and time can be
  controlled at their acceptance boundaries.

### TIR-5: Lifecycle, persistence, and filesystem acceptance

- A disposable application-data environment supports safe crash, forced quit,
  relaunch, and persistence scenarios.
- Evidence covers History, transcript, journal, interrupted-state recovery,
  settings atomicity, diagnostics contents, retention, and stale historical
  process identifiers.
- Filesystem failure and hostile-path cases prove the product's stated
  durability and privacy behavior.

### TIR-6: Provisioned target-Mac compatibility lane

- A designated Mac provides serialized, dated evidence for the supported
  Manager topology, including installed `mas`.
- Live captures record provenance and cannot be replaced by synthetic fixtures
  as correctness oracles.
- Target-machine drift is detected and reported rather than silently changing
  expected behavior.
- Ignored live tests do not count unless they were executed successfully for
  the Evidence Set.

### TIR-7: Packaged application, accessibility, and updater acceptance

- Acceptance can exercise the packaged WKWebView application rather than a
  browser approximation.
- It can verify keyboard and focus paths, the fixed 4.5:1 text-contrast
  threshold, and reduced motion through packaged-app checks, plus a manual
  VoiceOver check of focus order and completion announcements.
- It can exercise check, metadata retrieval, download, signature validation,
  explicit install/relaunch, and non-writable-install behavior.
- It can begin from an actually installed prior public version and retain
  before/after version and interaction evidence.
- A credentialless or `--no-sign` build smoke remains available for ordinary
  build validation but is explicitly separated from signed candidate evidence.
- Candidate identity remains stable across the entire acceptance run.

### TIR-8: Evidence provenance and retention

- **Source-bound** results name source commit, clean/dirty state, and the exact
  executed command or scenario.
- **Environment-bound** results additionally name macOS version, architecture,
  relevant Manager/tool versions, and controlled versus live dependencies.
- **Candidate-bound** results additionally name the candidate identity-manifest
  digest and artifact checksum under test.
- Collected-but-unexecuted and ignored tests are visibly excluded.
- The first failure is retained even when a retry later succeeds.
- Human-readable and machine-readable outputs agree and remain available for
  the final trace regeneration.

## 7. Release evidence requirements

This section defines what must be produced for one Release Candidate. It does
not prescribe release workflow steps. TIRs define reusable capability; REs are
the candidate-specific outputs from using that capability.

### RE-1: Immutable candidate identity

Before candidate-bound validation begins, the Release Owner shall create an
immutable **Candidate Identity Manifest** binding one clean source commit,
release tag, application versions, artifact names, artifact checksums, signing
identities, and published metadata checksum. The manifest contains identity,
not results.

As validation runs, an append-only **Evidence Index** shall link every result
to the Candidate Identity Manifest digest and tested artifact checksum. Neither
a dirty working tree nor a rebuilt, untracked candidate can serve as final
attestation.
Any rebuild, resign, retag, repack, artifact replacement, or metadata change
creates a new identity manifest and invalidates downstream candidate-bound
evidence until it is rerun.

### RE-2: Clean reproducible quality result

The Release Candidate shall have a retained fresh-checkout result for required
frontend and Rust formatting, static checks, production builds, contracts, and
tests with outbound network denied. The result shall include complete first-run
output and may not substitute a retry for the original failure. An unsigned
`--no-sign` build smoke may prove that bundling still works without release
credentials, but it cannot satisfy RE-3 through RE-9.

### RE-3: Complete version-coherent release set

The candidate's tag, bundle version, package/Cargo versions, asset names,
updater version, and metadata shall agree. The GitHub Release shall contain
every direct-download and updater artifact required by the approved channel,
including the DMG, ZIP, updater archive, detached signature, and `latest.json`.

### RE-4: Architecture, icon, and bundle-content attestation

The exact candidate shall have retained evidence of both supported
architectures, the approved application icon source and generated resources,
and the expected packaged resources and entitlements. Physical Intel hardware
shall be included in architecture acceptance rather than relying on
universal-binary inspection alone.

### RE-5: macOS trust attestation

The exact application and disk image shall have valid Developer ID identity,
secure signatures, accepted notarization, stapled tickets where required, and
Gatekeeper acceptance of the downloaded deliverable without a bypass.

### RE-6: Updater integrity and reachability attestation

The published HTTPS metadata shall be reachable and complete for both supported
Mac architecture identifiers. Its URLs shall return the intended updater
archive, the detached signature shall validate against the public key embedded
in the candidate, and all entries shall describe the candidate version.

### RE-7: Fresh-install launch evidence

The exact downloaded candidate shall install through the approved disk-image
journey and launch from Finder and the Dock. On Apple silicon and physical
Intel hardware, the evidence shall cover packaged resources, entitlements, GUI
environment discovery, and the packaged WKWebView.

### RE-8: Previous-release-to-candidate update evidence

An actually installed prior public version shall discover, download, and
install the Release Candidate through Pack-Manager's explicit Restart to update
interaction, then relaunch as the intended version without an administrator
prompt. The Evidence Set shall identify the prior version and retain before and
after evidence. It shall also prove that install/relaunch is refused while a
Package Operation is queued or running. The prior-version update journey shall
pass on Apple silicon and physical Intel hardware.

### RE-9: Non-writable-install evidence

The candidate shall demonstrate that an installation location requiring
elevation produces the manual-install-required outcome and does not invoke an
administrator authorization path.

### RE-10: Candidate-bound trace and risk decision

The traceability matrix shall be regenerated against the complete Evidence Set
for the exact Release Candidate. It shall report 72 of 72 P0 criteria FULL, no
unresolved P0 defect, and completed mitigation for every release-blocking risk.
The approved `readiness-coverage-map.md` shall be the one-to-one source
reconciliation; it may not drop, duplicate, or auto-promote a criterion. The 14
previously FULL criteria shall be revalidated under this PRD's evidence-depth
rules rather than carried forward automatically. QA Lead and Dev Lead
acceptance shall identify the Candidate Identity Manifest digest and Evidence
Index.

### RE-11: Evidence publication and first-failure preservation

The final decision shall link the append-only Evidence Index and its retained
human-readable and machine-readable results. A failed attempt remains part of
the record; later retries explain the change and do not overwrite the first
result.

## 8. Cross-cutting non-functional requirements

### NFR-1: Fail-closed safety

None of the unreviewed, stale, altered, replayed, partially admissible, or
privilege-seeking work shall run. User exclusions and Manager protections
remain authoritative.

### NFR-2: Failure isolation and recovery

One Manager's detection, refresh, parse, network, or update failure shall not
blank another Manager or destroy a Last-good Snapshot. Crash, cancellation,
timeout, and persistence failures shall have explicit recovery outcomes.

### NFR-3: Responsive bounded presentation

The app shall render progressive state without waiting for every Manager,
remain interactive with more than 100 Package rows, retain no more than 5,000
live lines per Operation, and preserve older retained output in the transcript.
Acceptance shall prove that:

- at 101 Package rows, the final row remains reachable and filter, selection,
  status, and row actions remain correct;
- output is delivered to the live surface when any flush boundary is reached:
  50 milliseconds, 64 lines, or 8 KiB;
- at 5,001 live lines, the surface retains the newest 5,000, shows that earlier
  output remains in the transcript, and does not lose the complete transcript;
  and
- at the 900 × 600 minimum window size, essential columns remain reachable
  without overlap by using the specified scrolling behavior; and
- at 150% and 200% zoom, Sidebar navigation, Package actions, the persistent
  Upgrade Plan, confirmation, Activity, Results, and recovery controls remain
  reachable without clipped primary actions.

### NFR-4: Durable observability

Status, output, transcript, structured log, History, and diagnostic evidence
shall correlate through durable Plan Attempt identity and nested Operation
identity. Transcript creation failure blocks an unaudited spawn; later
noncritical logging failures shall not hang Package work.

### NFR-5: Privacy and local trust

The product shall send no telemetry, expose no generic shell surface, and
exclude inherited environment values from logs and diagnostics. Diagnostic
selection and streaming shall resist symlink substitution.

### NFR-6: Accessibility

Primary interactions shall be keyboard operable with visible focus; status
shall not rely on color; text shall meet at least 4.5:1 contrast; reduced motion
shall be honored; and plan admission, progress, verification, completion,
failure, and cancellation shall be announced. Modal confirmation traps focus,
labels its command content, and restores focus to the invoking plan action when
closed. Non-selectable Package reasons are available to keyboard and VoiceOver
users, not only pointer hover.

### NFR-7: macOS compatibility

The application shall work through normal GUI launch, support both architectures
promised by the release, and fail visibly and locally when a Manager's output
format is incompatible. The minimum supported macOS version must be declared
before final candidate acceptance.

### NFR-8: Release and update integrity

Direct-download and updater artifacts shall remain mutually consistent,
cryptographically authorized, and attributable to one Release Candidate.
Background download shall never weaken explicit install/restart control.

## 9. Gate governance

### 9.1 Brownfield source-correction prerequisite

#### PC-1: Restore current product truth

Before the product/source-correctness lane can close:

- `mas` must be represented as supported and live-verified rather than absent,
  synthetic-only, or unverified;
- synthetic `mas` fixtures may prove robustness or shape but not real-format
  correctness;
- user-visible and authoritative acceptance text must no longer say that
  notarization is out of scope or that ad-hoc-only delivery is current; and
- the obsolete five-event invariant must not force application-update state
  into Package Operation queue or History semantics.

PC-1 is product/source correction. Its recurrence guard belongs to TIR-1, and
its release proof remains in the applicable REs.

### 9.2 P0 and P1 policy

#### GP-1: Freeze the P0 denominator and release prerequisites

- The P0 Product Gate denominator is the 72 P0 criteria in the approved
  coverage map.
- RP-1 and RP-2 are mandatory Release Prerequisites because they preserve
  already-adopted update behavior, but they do not enter the 72-row denominator.
- Other legacy P1 product features remain out of scope.
- The existing 80% strict-FULL P1 threshold conflicts with this P0-specific
  gate. Before validation begins, Product and QA shall approve and configure
  the P0-specific policy represented here. That prospective policy change is
  not a candidate waiver.

#### GP-2: Distinguish policy change, waiver, and risk acceptance

- A policy change applies prospectively to all candidates and must be frozen
  before candidate validation starts.
- A criterion waiver exempts a required outcome and is incompatible with a
  100% P0 PASS.
- Risk acceptance leaves the requirement intact and acknowledges residual
  uncertainty.
- Only the Product Owner, QA Lead, and Release Owner acting together may issue
  a candidate-specific conditional release. Such a decision is never labeled
  100% ready.
- The conditional decision must be a durable, candidate-bound record naming the
  exact criterion/risk IDs, approvers, rationale, compensating controls,
  expiration/revisit date, Candidate Identity Manifest digest, and links to
  supporting Evidence Index entries.
- Its visible decision label must be
  `CONDITIONAL — NOT 100% PRODUCT-AND-RELEASE READY`.

### 9.3 Decision register

#### DR-1 — Minimum supported macOS version — OPEN, implementation-entry blocker

- **Owner:** Product Owner and Release Owner.
- **Decision deadline:** Before implementation planning for TIR-7, RE-4, RE-7,
  or RE-8 begins.
- **Blocked dependencies:** Packaged compatibility matrix, fresh-install
  environments, prior-version update environments, and final support copy.
- **Reason open:** The authoritative product sources and current configuration
  do not declare a minimum version; this PRD does not invent one.

#### DR-2 — Packaged accessibility evidence — APPROVED

Use packaged-app keyboard/focus checks, automated 4.5:1 text-contrast checks,
reduced-motion checks, and a manual VoiceOver pass for focus order and
completion announcements. A broader WCAG or legal-compliance target remains
outside this gate unless separately adopted. Approval establishes the method;
it is not evidence that the method has passed.

#### DR-3 — Intel execution evidence — APPROVED

The Apple silicon and Intel support promise requires a physical Intel
fresh-install, Finder/Dock launch, and previous-public-version update run.
Universal-binary inspection alone is insufficient for 100% readiness. Product,
QA, and Release approval establishes this requirement; the required physical
execution remains unperformed.

#### DR-4 — P1 gate policy — PROPOSED, gate-approval blocker

GP-1 defines the 72-row P0 denominator and the separate RP-1/RP-2 Release
Prerequisites. Product and QA must approve this P0-specific policy and replace
the conflicting legacy 80% strict-FULL P1 threshold before validation begins;
unrelated P1 features remain out of scope.

### 9.4 Implementation-entry blockers

Before architecture or story implementation begins:

- Product Behavior Prerequisite UX-PB.1..UX-PB.5 implements Decisions D27-D30
  and AD-16 before affected evidence stories.
- `readiness-coverage-map.md` is approved and mechanically verified as a
  one-to-one 72-row map.
- The closure-plan companion `addendum.md` has named accountable owners and
  delivery timing for its five blocking evidence enablers.
- The decisions in §9.3 are resolved by their deadlines.
- Product, QA, Architecture, and Release agree on the evidence storage and
  candidate-manifest contract.

### 9.5 Final candidate-validation entry criteria

- Product intent, the coverage map, and gate policy are frozen.
- PC-1 is satisfied.
- TIR-1 through TIR-8 are operational.
- A designated target Mac, disposable application state, actually installed
  prior public version, current release credentials, and immutable Candidate
  Identity Manifest are available.
- Every candidate-bound scenario starts from the same manifest; any change
  invokes RE-1 invalidation.

### 9.6 Exit criteria

The overall P0 product-and-release gate is PASS only when:

1. The regenerated decision reports 72/72 P0 FULL using the approved coverage
   map.
2. Every P0 consequence in FR-1 through FR-22 passes at its mapped evidence
   depth.
3. RP-1 and RP-2 pass as separate Release Prerequisites.
4. TIR-1 through TIR-8 have produced trustworthy, repeatable evidence.
5. RE-1 through RE-11 are satisfied for one Candidate Identity Manifest.
6. PC-1 and GP-1 are satisfied.
7. No known P0 defect, unmitigated release risk with a score of 6 or 9, ignored
   required check, or candidate-identity break remains.
8. QA Lead and Dev Lead accept the candidate-bound Evidence Set, and the
   Product Owner and Release Owner accept the final release decision.

Any unmet condition produces FAIL. A candidate-specific exception may only
produce the GP-2 conditional result.

### 9.7 Evidence dependency order

Product/source truth and the target-Mac oracle close first. Deterministic
product-state work and the native boundary can then proceed together. Native
process/lifecycle evidence precedes packaged accessibility and updater
acceptance. Candidate attestation and trace regeneration come last.

The eight-batch execution handoff and evidence-enabler ownership live in
`addendum.md`; they are downstream planning detail, not product requirements.

## 10. Success metrics

### Primary

- **SM-1 — P0 acceptance completeness:** 72/72 P0 criteria are FULL in the
  candidate-bound regenerated matrix through the approved one-to-one coverage
  map. This validates the legacy P0 denominator; it does not by itself validate
  Release Prerequisites or every RE.
- **SM-2 — Product correctness:** zero known P0 product defects and every
  product acceptance journey passes at its required evidence layer. Validates
  FR-1–FR-22, PC-1, and AJ-1–AJ-6.
- **SM-3 — Candidate trust:** one immutable candidate has a complete coherent
  artifact set and passes trust, fresh-install, launch, and previous-version
  update acceptance on both promised architectures. Validates RP-1–RP-2 and
  RE-1 and RE-3–RE-9.
- **SM-4 — Evidence repeatability:** required deterministic gates pass from a
  clean checkout with outbound network denied, and native/release results name
  their controlled environments. Validates TIR-2–TIR-8 and RE-2.

### Secondary

- **SM-5 — Gate clarity:** the human-readable decision, machine-readable
  outputs, and traceability counts agree and identify the same candidate.
  Validates RE-10–RE-11 and GP-1–GP-2.
- **SM-6 — Failure provenance:** 100% of failed gate attempts remain retained
  and linked when a retry occurs. Validates TIR-8 and RE-11.

### Counter-metrics

- **SM-C1 — Raw test count:** increasing test count is not success unless it
  promotes a requirement with appropriate executed evidence.
- **SM-C2 — Green workflow:** a green workflow is not success when published
  artifacts, updater metadata, or installed journeys are absent.
- **SM-C3 — Coverage percentage by dilution:** P0 readiness shall not improve by
  reclassifying requirements, weakening oracles, skipping boundaries, or adding
  easy tests.
- **SM-C4 — Scope expansion:** shipping P1/P2 features does not compensate for
  an open P0 criterion.

## 11. Risks and dependencies

This table carries forward the eight high-priority risks from
`_bmad-output/test-artifacts/test-design-progress.md`. Probability and impact
use the 1–3 BMAD scale; a score of 6 or 9 is high and release-blocking for this
P0 closure scope.

| ID    | Risk                               | Score | Readiness impact                                                                                         | Required response                                                                |
| ----- | ---------------------------------- | ----: | -------------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------- |
| R-001 | Source/oracle drift                |     6 | Tests can faithfully prove obsolete `mas` or target-Mac assumptions.                                     | Correct repository truth, retain provenance, and detect topology drift.          |
| R-002 | Fake/native boundary gap           |     6 | Frontend and Rust suites can both pass while registration, payload, event, or native behavior is broken. | Use the real native boundary for material Tauri behavior.                        |
| R-003 | Misleading UI state                |     6 | Stale plans, failed refreshes, or late dialog results can present or execute the wrong state.            | Prove coherent state, reconfirmation, and stale-continuation guards.             |
| R-004 | Process lifecycle uncertainty      |     6 | Cancellation, shutdown, stdin, timeout, or PID reuse can leave unsafe or dishonest outcomes.             | Use controlled process/lifecycle evidence and explicit terminal states.          |
| R-005 | Persistence or diagnostics failure |     6 | History can be incomplete or diagnostics can disclose or follow hostile paths.                           | Prove atomic/durable behavior and privacy boundaries.                            |
| R-006 | Updater integrity failure          |     6 | Metadata, signature, or installed version can diverge while CI remains green.                            | Bind updater acceptance to the immutable candidate and embedded trust key.       |
| R-007 | Invalid shipped artifact           |     9 | A release can exist but be unsigned, incomplete, non-universal, unlaunchable, or mismatched.             | Require candidate-bound attestation and real install/launch evidence.            |
| R-008 | Environmental dependency           |     6 | Tests can silently depend on network, host topology, or mutable credentials.                             | Separate forced-offline, target-Mac, and release lanes with explicit provenance. |

Key dependencies:

- Designated and serialized target Mac with the approved Manager topology.
- Disposable application-data and controlled process environments.
- Approved native, accessibility, and release acceptance oracles.
- Immutable signed candidate and current Apple/updater credentials.
- Evidence storage that preserves complete first-run output and failures.
- Product, QA, engineering, and release ownership for the final decision.

## 12. Assumptions index

There are no unresolved inline product assumptions in this draft. DR-1 is an
explicit blocking decision with an owner, deadline, and affected dependencies;
it is not a hidden assumption.
