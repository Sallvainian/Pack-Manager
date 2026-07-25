---
stepsCompleted:
  - step-01-validate-prerequisites
  - step-02-design-epics
  - step-03-create-stories
  - step-04-final-validation
# Live, authoritative inputs.
inputDocuments:
  - _bmad-output/planning-artifacts/architecture/architecture-Pack-Manager-2026-07-23/ARCHITECTURE-SPINE.md
  - _bmad-output/planning-artifacts/ux-designs/ux-Pack-Manager-2026-07-23/DESIGN.md
  - _bmad-output/planning-artifacts/ux-designs/ux-Pack-Manager-2026-07-23/EXPERIENCE.md
  - _bmad-output/planning-artifacts/ux-designs/ux-Pack-Manager-2026-07-23/validation-report.md
  - _bmad-output/planning-artifacts/story-triage-2026-07-24.md
  - _bmad-output/planning-artifacts/sprint-change-proposal-2026-07-25.md
  - _bmad-output/project-context.md
  - docs/SPEC.md
  - docs/DECISIONS.md
  - docs/RELEASE-CHECKLIST.md
# Historical inputs that fed earlier revisions. Repointed to their real archive
# locations on 2026-07-25 — every path above them was stale. NOT authoritative:
# project-context.md states "Nothing under
# _bmad-output/archive/2026-07-24-scope-recalibration/ is authoritative", and
# archived files must never be moved back into planning-artifacts, because BMAD
# skills glob *prd*.md and *epic*/*.md from there and would silently reload the
# readiness gate that docs/DECISIONS.md D33 retired.
historicalInputDocuments:
  - _bmad-output/archive/2026-07-24-scope-recalibration/planning/prds/prd-Pack-Manager-2026-07-22/prd.md
  - _bmad-output/archive/2026-07-24-scope-recalibration/planning/prds/prd-Pack-Manager-2026-07-22/addendum.md
  - _bmad-output/archive/2026-07-24-scope-recalibration/planning/prds/prd-Pack-Manager-2026-07-22/readiness-coverage-map.md
  - _bmad-output/archive/2026-07-24-scope-recalibration/planning/sprint-change-proposal-2026-07-24.md
  - _bmad-output/archive/2026-07-24-scope-recalibration/test-artifacts-gate/test-design-architecture.md
  - _bmad-output/archive/2026-07-24-scope-recalibration/test-artifacts-gate/test-design-qa.md
  - _bmad-output/archive/2026-07-24-scope-recalibration/test-artifacts-gate/test-design-progress.md
  - _bmad-output/archive/2026-07-24-scope-recalibration/test-artifacts-gate/test-design/Pack-Manager-handoff.md
---

# Pack-Manager - Epic Breakdown

## Overview

This document provides the complete epic and story breakdown for Pack-Manager, decomposing the finalized requirements, formal architecture, finalized UX contract, and product-behavior authorities into implementable stories. The 2026-07-24 Correct Course amendment below is binding and supersedes older immediate-row, direct Manager-update, Activity-drawer, Operation-History, and `autoOpenDrawer` wording in affected stories.

On 2026-07-25 this document was reconciled with `ARCHITECTURE-SPINE.md` revision 6
and `docs/DECISIONS.md` D33, which retired the 72-criterion readiness gate and the
register this document previously carried at `#### Additional Requirements`. Where
this document and the spine disagree, **the spine is upstream and wins.** Record:
`_bmad-output/planning-artifacts/sprint-change-proposal-2026-07-25.md`.

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

RP-1: Preserve launch, six-hour, and app-menu update checks; restore in-process update state after supported UI recreation; preserve saved trigger policy across normal relaunch; ensure failed/interrupted downloads never appear Ready; keep application-update state separate from Package Operation queue and History; and validate this mandatory prerequisite through `docs/RELEASE-CHECKLIST.md`.

RP-2: Preserve standard macOS Edit and Window menu actions, including cut/copy/paste/select-all in search and every copyable command surface, as a mandatory prerequisite validated through `docs/RELEASE-CHECKLIST.md`.

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

#### Scope Controls

The 72-criterion P0 readiness gate this section carried is retired by
`docs/DECISIONS.md` **D33**. There is no P0 denominator, no coverage percentage,
no `readiness-coverage-map.md` oracle, no criterion promotion, and no three-way
primary-concern taxonomy. Release readiness is `docs/RELEASE-CHECKLIST.md` plus
the two publication-blocking checks in `release.yml`. The retired artifacts are
archived under `_bmad-output/archive/2026-07-24-scope-recalibration/` and are not
authoritative.

One habit survives the gate, and it binds every story below: **before scheduling
work described as a test gap, verify whether the behavior is already present in
the shipping code.** `docs/DECISIONS.md` D33 records why — an adversarial pass
over the Epics 1-6 triage overturned 14 of 20 initial keep verdicts for exactly
that reason. `ARCHITECTURE-SPINE.md` AD-1 carries it as a rule.

Every live story is product-behavior work. The `Primary concern` label retained
on the 28 Epic UX-PB stories records that; the retired Reusable Test
Infrastructure and Candidate-Specific Release Evidence alternatives no longer
exist.

RP-1 and RP-2 remain mandatory requirements, validated through the release
checklist per the FR Coverage Map below. They no longer sit inside or outside any
denominator.

#### Product Acceptance Journeys

- AJ-1: Prove normal Finder/Dock launch, progressive rendering, six-Manager detection, ownership, independent refresh, normal absence, Last-good Snapshot retention, and useful recovery.
- AJ-2: Prove exact Update Everything preview, explicit inclusions/exclusions/warnings, stale-plan replacement and reconfirmation, atomic admission, safe concurrency, and understandable queue reasons.
- AJ-3: Prove exact selection or row-level update, discovered Manager Route explanation, visible command, and affected-state refresh.
- AJ-4: Prove exact command/live output, stall choices, no automatic Homebrew-contention retry, explicit cancellation/timeout outcomes, and Interrupted reconstruction after crash/forced quit.
- AJ-5: Prove searchable History, command/outcome/transcript reconstruction, Finder reveal, and privacy-preserving diagnostics export.
- AJ-6: Prove trusted direct installation, normal launch, prior-public-version discovery/download, explicit Restart to update, intended-version relaunch, and non-writable manual-install behavior without privilege escalation.

#### Test Infrastructure

TIR-1 through TIR-8 are retired by `docs/DECISIONS.md` **D33**, together with the
evidence lanes they specified: the `forced-offline`, `provisioned-target-mac`, and
`candidate-release` lane separation, the source/environment/candidate provenance
depths, and the first-attempt/zero-automatic-retry admission terms.

Three obligations survive. Each is owned elsewhere and is deliberately not
restated here:

- **Determinism and offline defaults** — `ARCHITECTURE-SPINE.md`'s Determinism
  convention, and the testing rules in `_bmad-output/project-context.md`.
- **Real-versus-simulated honesty** — `ARCHITECTURE-SPINE.md` AD-3: the committed
  fixtures in `dev/fixtures/ipc/` prove payload shape on both sides and never
  dispatch anything through Tauri, so no story may claim event-delivery coverage
  from a fixture or from the browser double. The native Tauri harness is Deferred
  there, with Story 6.5 as its only live consumer.
- **Behavior-present verification** before scheduling a test gap — see Scope
  Controls above.

#### Release Acceptance

RE-1 through RE-11 are retired by `docs/DECISIONS.md` **D33**, together with the
Candidate Identity Manifest, the append-only Evidence Index, the Evidence
Registrar, `contracts/readiness/v1/contract-lock.json`, and the candidate-freeze
machinery. No `contracts/` directory exists and none is to be created.

Release acceptance is `docs/RELEASE-CHECKLIST.md` — a manual pass, not a computed
verdict or gate decision — plus two checks in `release.yml` that block
publication: the detached updater signature is base64-decoded and verified with
`minisign` against the public key the shipping app embeds, and the published
`latest.json` is asserted reachable and coherent after upload. Their failure modes
are silent and simultaneous across every installed client, which is why they are
automated rather than manual. `ARCHITECTURE-SPINE.md` AD-11 and AD-12 are the
architectural statement of this.

The physical-Intel obligation in the former RE-4, RE-7, and RE-8 is separately
dropped by **D32**: the build stays universal and `latest.json` keeps publishing
both `darwin-aarch64` and `darwin-x86_64`, but verification is Apple silicon only
and Intel is best-effort and unverified.

#### Architecture Invariants

`_bmad-output/planning-artifacts/architecture/architecture-Pack-Manager-2026-07-23/ARCHITECTURE-SPINE.md`
is the **single authority** for architecture invariants. Stories cite its AD ids
and no others. This section does not restate them, so that the two documents
cannot drift apart again.

The AD-1..AD-15 list this section previously carried was the **retired gate's own
numbering, and it is not the spine's.** It is removed rather than renumbered,
because the two schemes collide on the same ids with different meanings:

| Retired id here | Said | `ARCHITECTURE-SPINE.md` says under that id |
| --- | --- | --- |
| AD-1 | Each work item has one primary readiness concern | Dependencies point inward; test and release tooling are never product dependencies |
| AD-3 / ASR-01 | Exact set equality across the versioned `contracts/tauri-boundary/v1.json` catalog | The IPC surface changes atomically, proven by the committed fixtures in `dev/fixtures/ipc/`. "There is no separate versioned boundary-catalog file and none is to be created." |
| AD-11 | Packaged acceptance ends at the installed exact candidate | Release acceptance is the checklist plus two automated checks |
| AD-13 | Preserve the exact Batch 1-8 dependency waves | (no such rule; the batch ordering is void) |

`ARCHITECTURE-SPINE.md` records **AD-6..AD-10 and AD-13..AD-15 as retired ids that
are never reused.** The `ASR-01` / `ASR-02` / `ASR-03` enabler framing is retired
with the register that defined it; the spine states that the surviving obligations
are its **AD-2, AD-3, AD-4, and AD-5**.

The Candidate Identity Manifest v1 JCS shape, the
`contracts/readiness/v1/contract-lock.json` freeze, the Evidence Registrar append
rules, the attempt-ordinal terms, and the PASS-admission counters are retired by
`docs/DECISIONS.md` **D33**.

**Four invariants bind every story and are cited by none of them, because they
have no single owner.** A story that does not name them is still bound:

- **AD-1** — product code never imports, branches on, or requires test
  infrastructure, CI, or release tooling. Missing behavior is product work, not
  test work.
- **AD-2** — one composition root; no release build contains any runtime selector
  that could activate a controlled adapter.
- **AD-20** — the webview trust boundary widens only on purpose. `csp` is `null`
  today, tolerable only while nothing remote loads, so any story introducing a
  remote font, script, style, or navigation target sets a real CSP **in the same
  change**. Adding a permission, plugin, window, or capability is a
  security-sensitive change reviewed on its own terms and never folded into a
  feature story as a side effect.
- **The Determinism convention** — default suites stay offline and deterministic.

Cite AD ids by subject, never by rule ordinal. Rule ordinals within an AD are not
stable across spine revisions — revision 5 inserted AD-16's "no entry point
executes" as its first rule and shifted every later number.

#### Governance and Risks

GP-1, GP-2, PC-1, the coverage-map approval requirement, the evidence transport
and retention choice, the provisioned-target-Mac and multi-host environment
requirements, the per-story criterion-authoring rules, and the candidate-bound
story rules are retired by `docs/DECISIONS.md` **D33**.

PC-1's substance is closed independently of the gate: **D23a** withdrew the `mas`
UNVERIFIED label and recorded `mas` as verified live, and the current production
surface is 20 commands and six events, so the obsolete five-event invariant no
longer absorbs application-update state. One documented residual remains —
`src-tauri/tests/live_smoke.rs` still declares this machine as "mas absent".
`_bmad-output/project-context.md` records that precondition as stale rather than
as evidence of a code defect.

- DR-1 is CLOSED by `docs/DECISIONS.md` **D31**: the minimum supported macOS version is 15.0, declared as `bundle.macOS.minimumSystemVersion` in `src-tauri/tauri.conf.json` and shipped in v1.0.0. Nothing is blocked on it.
- DR-2 is RESTATED by **D33**: its substance survives without the gate framing. Automated 4.5:1 contrast and reduced-motion checks belong in the existing Playwright/Vitest lane; one manual VoiceOver pass joins `docs/RELEASE-CHECKLIST.md`. Accessibility here is product quality, not evidence ceremony. Neither automated check exists yet, so this is an obligation on whichever story adds them, not a description of current coverage.
- DR-3 is NARROWED by **D32**: the release still builds universal, but the obligation to verify on physical Intel hardware is dropped. Verification is Apple silicon only; Intel remains best-effort and unverified.
- DR-4 is DISSOLVED by **D33** along with the gate that defined it. There is no P0/P1 threshold, no Acceptance Profile, and no gate decision. Release readiness is `docs/RELEASE-CHECKLIST.md` plus the automated updater-signature and published-endpoint checks in `release.yml`.

**The `R-001`..`R-008` register is retired, and its ids must not be reimported.**
They were defined only in the gate's own test-design artifacts, now archived and
non-authoritative, and their `Required mitigation` column *was* the retired
machinery — `R-002`'s mitigation read "Deliver ASR-01 and AD-3 set-equality
checks", and `R-007`'s required physical-Intel acceptance that **D32** dropped as
undischargeable. Asserting those mitigations are still owed would resurrect by
reference exactly what D33 retired, so no live document carries the ids or their
scores.

The engineering concerns behind them are real and each has a live owner in
`ARCHITECTURE-SPINE.md`. That is where they are governed — not here, and not as a
parallel risk register:

| Concern | Governed by |
| --- | --- |
| Manager output or oracle drift; a parser proving an obsolete format | AD-4 (the manager's own `outdated` verdict is the sole authority; fixture provenance) |
| Suites green while the real command/event boundary is broken | AD-3 (committed contract fixtures; delivery coverage explicitly unproven and awaiting the deferred native harness) |
| Stale, failed, or misleading UI state authorizing the wrong action | AD-16, AD-17 (no entry point executes; no rebuild enlarges membership; verification gates success) |
| Process output, locks, cancellation, timeout, or PID reuse going dishonest | AD-4, AD-5 (complete lock-set rule; output fidelity floor; historical PGIDs never signalled) |
| Persistence, History, or diagnostics losing evidence or following hostile paths | AD-18, AD-19 (journal never defaulted away; symlinks rejected; disclosure not widened) |
| Updater metadata, signature, or relaunched version diverging silently | AD-11, AD-12 (the two publication-blocking checks in `release.yml`) |
| A published artifact set incomplete, unsigned, unnotarized, or unlaunchable | AD-11, AD-12 plus `docs/RELEASE-CHECKLIST.md` |
| Results depending on network, mutable host state, or ignored tests | The Determinism convention (offline, deterministic default suites) |

Secrets stay in fnox locally and GitHub Secrets in CI and never enter build
artifacts, manifests, or documentation. Apple Developer ID signing and
notarization are required for a published release, and updater signing is required
by the build; `ARCHITECTURE-SPINE.md` AD-12 owns this.

#### Implementation-Entry Register

The `Deadline boundary` column this table carried is removed: every value in it
was a Batch 1-8 boundary, and the evidence-batch ordering is void along with the
gate that defined it. The six surviving Epic 1-6 stories carry no inter-epic
dependencies.

| Decision or dependency | Current state | Accountable role | Effect on implementation entry |
| --- | --- | --- | --- |
| Product Behavior Prerequisite UX-PB.1..UX-PB.5 | `APPROVED TARGET — NOT IMPLEMENTED` | Product/UX/Architecture accept; Development implements | Nothing is blocked from starting — Epic UX-PB is the primary build queue and runs first. Any story or test text authored against immediate row execution, direct self-update execution, the Activity drawer, Operation-row History, or active `autoOpenDrawer` behavior is superseded by D27-D30. |
| DR-1 — minimum supported macOS | `CLOSED` — D31 | Resolved 2026-07-24 | None. 15.0 declared and shipped in v1.0.0. Whether `notarytool` accepts `minos 15.0` against the CI SDK is OPEN and is settled by a manual Release run, never by assertion. |
| DR-2 — packaged accessibility method | `RESTATED` — D33 | Existing Playwright/Vitest lane + release checklist | None. An obligation on whichever story adds the two automated checks, which do not exist yet. |
| DR-3 — physical Intel requirement | `NARROWED` — D32 | Resolved 2026-07-24 | None. Universal build retained; verification Apple silicon only. |
| DR-4 — P0 gate/retry policy | `DISSOLVED` — D33 | Retired with the gate | None. |
| Named assignees and calendar dates | `REMOVED` — D33 | n/a | None. The `Assignee` and `Calendar date` fields were removed from every surviving story on 2026-07-25. |
| Native Tauri E2E harness and runner | `DEFERRED` | Architecture accepts; Development implements | Story 6.5's "Real native Tauri E2E plus artifact inspection" test level is its only live consumer. Any choice must satisfy `ARCHITECTURE-SPINE.md` AD-2 and AD-3. |
| Controlled child-helper language | `DEFERRED` | Development | No live story requires one. Any choice must satisfy AD-4 and cannot add a production shell-command surface. |
| Plan-attempt journal filename and serde shape | `DEFERRED` | Development | Owned by Story UX-PB.2c. AD-18 fixes ownership, location, durability, and failure mode; the exact filename and field list are the story's. |

Every other row this register carried is retired by `docs/DECISIONS.md` **D33**:
normative coverage-map approval, evidence transport and retention duration, the
provisioned target Mac and versioned profile, Apple-silicon and physical Intel
hosts, the actually-installed prior public version as a gate dependency, candidate
credentials as a freeze precondition, evidence/profile approval records with
versioned scenario digests, and the one immutable candidate with all required
artifacts. The prior-version update check and the signing credentials survive as
release-checklist steps and AD-12 obligations respectively — not as entry
blockers.

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

**This table is a historical revision record, not a live instruction.** It
records the prior wording that D27-D30 superseded on 2026-07-24. Every story area
it names except 3.1, 3.4, 3.5, and 6.5 was archived on 2026-07-25 — Stories 3.3,
3.6, 4.1, 4.6, 5.2, 5.4, 5.5, 6.3, 6.4, and 6.7 along with every Epic 7 and Epic 8
story, when Epics 1, 4, 5, 7, and 8 were removed. The live contracts are the 28
Epic UX-PB stories below, whose local text was rewritten directly so the
superseded experience cannot be built from a single story. The table is retained
rather than rewritten because rewriting it would destroy what it records.

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

### 2026-07-24 Story repair and decomposition applied

The affected stories are no longer governed only by the global supersession
note above. On 2026-07-24 their local contracts were rewritten directly so an
implementer or test author reading a single story cannot build the superseded
experience:

- **UX-PB.1–UX-PB.5 decomposed** into 28 dependency-ordered vertical
  sub-stories (`UX-PB.1a`–`UX-PB.5e`), each delivering one shippable behavior
  across the TypeScript/Rust/persistence/acceptance layers it needs, with
  explicit Given/When/Then happy- and failure-path criteria. They remain inside
  this Product Behavior Prerequisite.
- **12 stories rewritten** to remove superseded wording and express Decisions
  D27–D30 and AD-16 directly: 3.2, 3.3, 3.5, 3.6, 4.6, 5.2, 5.4, 5.5, 6.3, 6.4,
  6.5, 7.10.
- **7 stories additively aligned** where the local text did not contradict the
  target but omitted required coverage: 3.1, 3.4, 4.1, 6.7, 7.6, 7.7, 8.7.

The amendment table above is retained as the revision record of the prior
wording. Named assignees/dates, the 55 versioned scenario-contract files and
their digests, and revision-2 coverage-map approval died with the gate retired
by **D33**. DR-1 through DR-4 are closed, restated, narrowed, or dissolved by
**D31**, **D32**, and **D33**; the `Assignee`/`Calendar date` fields were removed
from every surviving story on 2026-07-25.

### FR Coverage Map

This map assigns each FR exactly once to its primary epic for planning accountability. An epic may reference additional FRs as cross-cutting acceptance constraints without creating duplicate primary ownership.

Epics 1, 4, and 5 were removed on 2026-07-25 when the story triage recorded in
`docs/DECISIONS.md` D33 found every one of their stories already shipped or owned by
another story. FRs that named them are marked `Triaged out` and point to the archive.
The requirements themselves remain authoritative in `docs/SPEC.md`.

FR-1: Triaged out (was Epic 4) — Prove Manager detection and refresh through the shared production-native boundary.

FR-2: Triaged out (was Epic 1) — Restore trustworthy Manager-reported truth using the corrected live `mas` oracle.

FR-3: Epic 2 — Preserve independent refresh, failure isolation, coalescing, and affected-state recovery.

FR-4: Triaged out (was Epic 5) — Prove dynamic Manager ownership and update Route selection.

FR-5: Epic 3 — Present complete, Manager-specific Package state and eligibility.

FR-6: Epic 3 — Preserve exact eligible Package selection.

FR-7: Epic 3 — Preview every bulk command and exclusion before authorization.

FR-8: Epic 3 — Reject stale, altered, replayed, or otherwise invalid plans. The stories that previously carried this (3.3, 3.6, and 5.8) were archived on 2026-07-25, so Epic UX-PB now owns the whole requirement: UX-PB.2a's one-use `planId` capability expires on mutation, staleness, execution attempt, or eviction, and UX-PB.2b's atomic admission rejects on in-progress state change, revision drift, an active refresh, or a lock-set overlap. Epic 3 remains the nominal primary owner for map accounting; the realizing stories are cross-cutting per this map's convention.

FR-9: Triaged out (was Epic 5) — Admit multi-group work atomically and preserve scheduler protections.

FR-10: Epic 3 — Support intentional, bounded single-Package updates.

FR-11: Triaged out (was Epic 5) — Explain and execute Manager self-update Routes safely.

FR-12: Triaged out (was Epic 5) — Preserve structured execution, null stdin, and the no-shell/no-privilege boundary.

FR-13: Triaged out (was Epic 5) — Expose correlated live Operation state and output.

FR-14: Triaged out (was Epic 5) — Handle stalls, cancellation, timeout, and shutdown honestly.

FR-15: Epic 6 — Preserve reconstructible History, transcripts, journals, and crash evidence.

FR-16: Epic 2 — Preserve useful Manager state and actionable recovery after outcomes.

FR-17: Epic 3 — Expose and validate user-controlled Settings; Epic 6 supplies the cross-cutting persistence acceptance.

FR-18: Epic 6 — Export privacy-preserving diagnostics through native filesystem boundaries.

FR-19: Release checklist — Validate the coherent accessible interface in the installed packaged application.

FR-20: Release checklist — Validate application-update discovery and background download.

FR-21: Release checklist — Validate explicit install/relaunch, active-operation refusal, and non-writable behavior.

FR-22: Release checklist — Attest normal packaged launch and authorized, coherent release/update artifacts.

RP-1: Release checklist — Validate scheduled/menu update triggers and state continuity.

RP-2: Release checklist — Validate standard macOS Edit/Window menu behavior.

## Epic List

Each epic completes one coherent user-confidence outcome and produces accepted foundations for later epics without relying on future work to complete its own domain. Epic UX-PB is the primary build queue and runs first; the six surviving Epic 1-6 stories carry no inter-epic dependencies.

### Epic UX-PB: The Upgrade Plan redesign (Decisions D27–D30)

The primary build queue. Implements the persistent Upgrade Plan, the durable plan attempt, verification-gated Results and History, and the separate confirmation gate. Full stories appear in the Epic UX-PB body below.

### Epic 2: Make Detection and Refresh Fail Independently and Recover Usefully

Users can detect and refresh every supported Manager with clear phase, absence, timeout, offline, and failure behavior while peer Managers and Last-good Snapshots remain usable.

**Primary FR ownership:** FR-3, FR-16  
**Cross-cutting FRs:** FR-1, FR-2, FR-17  
**Retained stories:** 2.2. The other three were triaged out on 2026-07-25 (D33); see `_bmad-output/archive/2026-07-24-scope-recalibration/planning/epics-1-6-triaged-out.md`.

### Epic 3: Keep Package Choice, Plans, and Settings Exact and Understandable

Users can understand Package state, select only eligible work, review exact commands and exclusions, reject stale plans, perform bounded row-level updates, and control Settings without misleading or inaccessible UI state.

**Primary FR ownership:** FR-5, FR-6, FR-7, FR-8, FR-10, FR-17  
**Cross-cutting FRs:** FR-2, FR-11, FR-19  
**Retained stories:** 3.1, 3.2, 3.4, and 3.5. The other two were triaged out on 2026-07-25 (D33); see `_bmad-output/archive/2026-07-24-scope-recalibration/planning/epics-1-6-triaged-out.md`.

### Epic 6: Preserve State, Evidence, and Privacy Across Failure and Relaunch

Users can reconstruct Operations after crashes, trust Settings and durable stores across failure, reveal native evidence safely, and export exact diagnostics without inherited-environment disclosure or hostile-path traversal.

**Primary FR ownership:** FR-15, FR-18  
**Cross-cutting FRs:** FR-16, FR-17  
**Retained stories:** 6.5. The other six were triaged out on 2026-07-25 (D33); see `_bmad-output/archive/2026-07-24-scope-recalibration/planning/epics-1-6-triaged-out.md`.

### Release readiness

Release readiness is a short manual checklist plus automated checks in the release
pipeline, not a closure epic and not an evidence lane. See `docs/RELEASE-CHECKLIST.md`.

### Dependency Summary

1. Epic UX-PB is the primary build queue and runs first. It implements
   Decisions D27–D30.
2. The Epics 1–6 rescope that D33 called for was applied on 2026-07-25. Six
   stories survived the triage and stay live under Epics 2, 3, and 6; the
   other 31 moved to the archive. Epics 1, 4, and 5 lost every story and were
   removed. The original evidence-batch ordering is void along with the gate
   that defined it, so the survivors carry no inter-epic dependencies.
3. Release readiness is covered by `docs/RELEASE-CHECKLIST.md` rather than by a
   closure epic. There is no candidate freeze, evidence ledger, or gate decision.

## Epic UX-PB: The Upgrade Plan redesign (Decisions D27–D30)

This epic implements Decisions D27–D30: the persistent Upgrade Plan, the
durable plan attempt, verification-gated Results and History, and the separate
confirmation gate. It is the primary build queue.

### Story UX-PB.1a: Persistent draft domain with single-entry membership and Rust rebuild

**Primary concern:** Product Behavior  
**Dependencies:** D27-D30; AD-16; AD-17; finalized UX spines  
**Blocks:** UX-PB.1b, UX-PB.1c; Story 3.5  

As a Pack-Manager user, I want one eligible Package to become persistent draft-plan membership so that acting on a single row never executes and always has a reviewable home.

**Acceptance Criteria:**

**Given** an eligible Package row in a Manager workspace
**When** I toggle its plan Checkbox by pointer, Enter/Space, or the grid Space key
**Then** the Package's canonical identity is added to the one persistent draft Upgrade Plan, nothing executes, and Rust rebuilds the exact command from canonical intent.
**And** the frontend never authors or edits executable command text; executable display text is never trusted input.

**Given** a Package already staged in the draft
**When** I toggle its Checkbox off or activate its `Remove` control
**Then** its canonical identity leaves the draft, Rust rebuilds the remaining plan from canonical intent, and nothing executes.

**Given** a draft mutation (add or remove)
**When** the Rust canonical rebuild errors or rejects
**Then** the draft surfaces the specific error, the prior coherent draft and its last authenticated preview are preserved, no executable display text is trusted, and nothing is admitted for execution.

**Given** a Package that becomes pinned, already current, or removed between my add action and the Rust rebuild
**When** the rebuild resolves the draft from canonical identities
**Then** the now-ineligible item is dropped or flagged with what changed, the plan is rebuilt from current canonical truth rather than the stale display, and a fresh review is required before anything can run.

**Given** the committed end-to-end suite asserts that a single-row upgrade executes immediately without a plan dialog
**When** this story lands
**Then** that assertion is rewritten to expect draft membership with nothing executing, because Decision D27 supersedes the behavior it encodes.

### Story UX-PB.1b: Sidecar lifecycle and navigation-persistent visibility

**Primary concern:** Product Behavior  
**Dependencies:** UX-PB.1a; D27-D30; AD-16; AD-17  
**Blocks:** UX-PB.1d, UX-PB.1e  

As a Pack-Manager user, I want the Upgrade Sidecar to appear, persist, and close in step with the draft so that my proposed plan always has a stable reviewable home and no empty drawer clutters the workspace.

**Acceptance Criteria:**

**Given** an empty draft and no visible sidecar
**When** I add the first eligible item
**Then** the Upgrade Sidecar opens showing the draft grouped by Manager with `Updates`, `Managers`, and `Commands` counts, and focus stays on the source control that created it.

**Given** a non-empty draft with the sidecar open
**When** I switch between Dashboard and Manager workspaces
**Then** the sidecar and its exact membership persist unchanged across navigation, and when hidden the main workspace reclaims its width with no reserved empty column.

**Given** a draft with one remaining item
**When** I remove the last item
**Then** the sidecar closes, the draft returns to empty, and nothing lingers in Activity or History.

**Given** an in-progress draft when the app crashes or is force-quit
**When** Pack-Manager relaunches
**Then** the draft's canonical membership is reconstructed into the sidecar, or — if it cannot be recovered — the sidecar returns to empty with no fabricated membership and nothing executes; a draft is never surfaced as Activity or History.

### Story UX-PB.1c: Remaining draft entry points as independent removable items

**Primary concern:** Product Behavior  
**Dependencies:** UX-PB.1a; D27-D30; AD-16; AD-17  
**Blocks:** UX-PB.1d, UX-PB.1e  

As a Pack-Manager user, I want selected-Package, Manager-header, Manager-wide, and `Update Everything` actions to all feed the same draft as independent removable items so that every entry point stages into one plan and no global toggle bypasses it.

**Acceptance Criteria:**

**Given** eligible work reachable from the count-labeled header Checkbox, the Manager Header `Update Manager` action, a Manager-wide action, and `Update Everything`
**When** I invoke each entry point
**Then** each adds its eligible canonical identities to the same one persistent draft, `Update Everything` seeds all eligible work while remaining editable, every staged Package and every Manager self-update is an independent item with its own visible `Remove`, and no global `includeSelfUpdates` control exists.

**Given** a staged Manager self-update in the draft
**When** I remove it
**Then** only that Manager self-update leaves the plan, Package items in the same Manager group are unaffected, and Rust dedups and rebuilds the authenticated preview from the remaining canonical identities.

**Given** a draft seeded by `Update Everything` as an `AllEligible` intent
**When** I remove any item
**Then** the draft converts to an `Explicit` intent of the surviving PackageRefs and Manager self-update identities and rebuilds the authenticated preview from the backend, never from edited display text.

**Given** two entry classes mutating the same draft in close succession
**When** both mutations resolve
**Then** the draft converges to one coherent deduplicated membership set, no item is doubled or lost, and a single authenticated rebuild reflects the final canonical intent.

### Story UX-PB.1d: Ineligible-control inertness with keyboard, pointer, and VoiceOver explanation

**Primary concern:** Product Behavior  
**Dependencies:** UX-PB.1a, UX-PB.1c; D27-D30; AD-16; AD-17  
**Blocks:** Story 3.2  

As a Pack-Manager user, I want pinned, current, excluded, and unavailable Packages to stay inert and explain themselves through keyboard, pointer, and VoiceOver so that I understand why they cannot join the plan without guessing.

**Acceptance Criteria:**

**Given** pinned, current, excluded, and unavailable Package controls
**When** I activate any of them by pointer, Enter/Space, or the grid Space key
**Then** membership never changes and each exposes its plain-language reason — pinned `This Package is pinned and cannot be updated. Unpin it, then refresh Pack-Manager to make it selectable.`, excluded `This Package is excluded by your Settings. Change the setting, then refresh Pack-Manager.`, current `This Package is already current.`, and unavailable `An update target is not available. Refresh or view details.`
**And** the bulk header Checkbox scope covers only eligible Packages matching the active filter and adds no ineligible identity.

**Given** an explanatory-disabled Package control
**When** a keyboard or VoiceOver user reaches it
**Then** it uses `aria-disabled="true"` rather than native `disabled`, keeps focus, announces its persistent reason as an accessible description, stays inert on activation, and retains focus when Escape closes its supplemental Tooltip/Popover.

**Given** a Package whose update is delegated to another Manager
**When** its row renders
**Then** it reads `Managed through <Manager>` in plain language and explains the update is grouped and executed through that Manager rather than exposing internal route/owner jargon.

### Story UX-PB.1e: Standardized Manager workspace presentation

**Primary concern:** Product Behavior  
**Dependencies:** UX-PB.1c; D27-D30; AD-16; AD-17  
**Blocks:** Story 3.1  

As a Pack-Manager user, I want each Manager Header and Card to present standardized identity, version, status, ownership, counts, and deltas so that every Manager reads consistently and its self-update staging is obvious.

**Acceptance Criteria:**

**Given** a detected Manager
**When** its workspace Header and Dashboard Card render
**Then** they show a standardized short description (for example `macOS package manager` or `Runtime version manager`), executable path, installed version beside the name, Manager status, ownership, Package counts in `34 managed packages · 8 package updates` order, and the self-update delta beneath the Manager status.
**And** Manager self-state stays separate from managed-Package health, and update availability is never colored as a system-health problem.

**Given** a Manager whose self-update has been staged into the plan
**When** the Manager Header renders
**Then** it shows `IN PLAN` plus a separate visible `Remove` action named `Remove <Manager> update from Upgrade Plan`, keeps no separate self-update row, and the `Update Manager` action stages the self-update into the plan and never executes it.

**Given** a Manager whose refresh has failed
**When** its Header and Card render
**Then** they retain the last-good snapshot with its timestamp, state the exact failure summary with `Retry refresh`, and use text rather than an invented Health Meter value.

### Story UX-PB.2a: Distinct one-use preview planId and durable planAttemptId identity types

**Primary concern:** Product Behavior  
**Dependencies:** UX-PB.1 complete (PB.1a-e); AD-3; AD-16; D29  
**Blocks:** UX-PB.2b, UX-PB.2f  

As a Pack-Manager user, I want the one-use preview identity and the durable confirmed-attempt identity to be separate, non-interchangeable types so that a short-lived authorization can never masquerade as the permanent record of what I confirmed.

**Acceptance Criteria:**

**Given** the reviewed-preview authorization and the confirmed-attempt identity
**When** each is defined across the Rust wire model, the Rust/TypeScript domain, persistence, and the TypeScript surface
**Then** a one-use preview `planId` and a durable `planAttemptId` exist as distinct branded types that round-trip through every layer
**And** neither type is assignable to or substitutable for the other at any boundary.

**Given** a one-use preview `planId`
**When** any surface attempts to reuse it as a durable History or attempt identity
**Then** the type boundary and its guard reject the reuse, because `planId` is a bounded one-use authorization for exactly one reviewed preview and is never a durable identity.

### Story UX-PB.2b: Atomic admission mints one planAttemptId and fails a second attempt closed

**Primary concern:** Product Behavior  
**Dependencies:** UX-PB.2a; AD-3; AD-16; AD-18; D29-D30  
**Blocks:** UX-PB.2c, UX-PB.2d, UX-PB.2e  

As a Pack-Manager user, I want confirming a reviewed plan to atomically create exactly one durable attempt identity so that every Operation it launches shares one reconstructible identity and no two confirmed attempts can ever run at once.

**Acceptance Criteria:**

**Given** a reviewed plan authorized by a one-use preview `planId`
**When** I invoke the confirmed run action (`Confirm N Updates`, or the confirmation-off run action) and admission succeeds
**Then** `execute_plan` atomically returns one new durable `planAttemptId` plus the created Operation identities
**And** the full plan is admitted as a unit with no partial silent admission.

**Given** one confirmed Upgrade Plan attempt is already active
**When** a second confirmation is attempted
**Then** admission fails closed, no second `planAttemptId` is minted, and only that one confirmed attempt remains active
**And** the scheduler still permits safe cross-Manager concurrency inside the single active attempt.

### Story UX-PB.2c: Persist reviewed intent and the exact command snapshot durably

**Primary concern:** Product Behavior  
**Dependencies:** UX-PB.2b; AD-16; D29  
**Blocks:** UX-PB.3 (on UX-PB.2 completion)  

As a Pack-Manager user, I want the confirmed attempt to durably store exactly what I reviewed and the exact commands as a snapshot so that recovery and history are reconstructible and never rebuild executable input from display text.

**Acceptance Criteria:**

**Given** a plan admitted under a new `planAttemptId`
**When** the attempt is persisted
**Then** the append-only record stores the reviewed Manager/Package scope, Manager self-update identities, exact command snapshot, version evidence, timestamps, and result/verification state as immutable plan-admission metadata
**And** the stored command snapshot is read back only as evidence and is never converted back into executable input.

**Given** a plan admitted under a new `planAttemptId`
**When** persisting the reviewed intent or command snapshot fails
**Then** the failure is surfaced, no partial attempt record is left behind, and the prior consistent state is preserved rather than proceeding as if durably recorded.

**Given** a `planAttemptId` was minted but its durable record was lost to a crash or forced quit mid-admission
**When** Pack-Manager relaunches
**Then** it reconstructs the attempt only from durable plan-admission metadata that actually persisted, leaves no orphaned executable command text, and never resurrects an unpersisted attempt as a completed durable record.

**Given** a persisted attempt whose command snapshot is later read as corrupted or incomplete
**When** the record is loaded
**Then** the integrity failure is detected and the snapshot is refused as an execution source, blocking any display-to-executable round-trip so a damaged snapshot can never be silently re-run.

### Story UX-PB.2d: Correlate every Operation, event, and durable record by planAttemptId

**Primary concern:** Product Behavior  
**Dependencies:** UX-PB.2b; AD-16; D29  
**Blocks:** UX-PB.2e  

As a Pack-Manager user, I want every Operation, event, and durable record produced by a confirmed attempt to carry that attempt's identity so that its progress, output, and evidence reconstruct as one coherent whole.

**Acceptance Criteria:**

**Given** a plan admitted under one `planAttemptId`
**When** its Operations run and emit state
**Then** every produced Operation carries that same `planAttemptId` through the Rust and TypeScript wire models, the `op:status`/`op:output`/attention events, transcript metadata, and in-memory stores
**And** every live surface resolves each line back to the one admitting attempt.

**Given** the same admitted attempt
**When** its durable and diagnostic records are written
**Then** crash-journal start/finish records, diagnostics, and verification refreshes carry the same `planAttemptId` where applicable
**And** persisted evidence stays correlated to the attempt that produced it rather than standing as flat, uncorrelated Operation records.

### Story UX-PB.2e: Plan-level cancellation that skips unstarted work and escalates running process groups

**Primary concern:** Product Behavior  
**Dependencies:** UX-PB.2b, UX-PB.2d; AD-16; D30  
**Blocks:** UX-PB.3 (on UX-PB.2 completion)  

As a Pack-Manager user, I want cancelling the plan to stop only that attempt's work honestly so that unstarted items are marked Skipped, running work is escalated through existing mechanics, and every real outcome is preserved.

**Acceptance Criteria:**

**Given** a confirmed attempt with some Operations running and others not yet started
**When** I choose `Cancel plan`
**Then** cancellation operates only on the Operation IDs bound to that `planAttemptId`: running work moves to `Cancelling` and escalates through the existing process-group mechanics, unstarted attempt work is prevented from beginning and recorded as `Skipped`, no second confirmation is required, rollback is not promised
**And** every prior outcome is preserved.

**Given** a plan cancellation where process-group escalation cannot stop some running work
**When** the escalation partially fails
**Then** the work that could not be stopped is reported honestly and never falsely marked cancelled, the successfully cancelled and skipped outcomes remain preserved
**And** no terminal outcome is fabricated for work whose true state is unknown.

### Story UX-PB.2f: Keep legacy Operations honest without inferred plan grouping

**Primary concern:** Product Behavior  
**Dependencies:** UX-PB.2a; AD-16; D29  
**Blocks:** None  

As a Pack-Manager user, I want Operations that predate the attempt model to stay honestly labeled as legacy so that older records are never fabricated into plans that never existed.

**Acceptance Criteria:**

**Given** Operation records that have no `planAttemptId`
**When** they are read and displayed
**Then** they remain honest legacy Operation entries, stay readable, and are never silently grouped or inferred into a plan attempt.

**Given** a legacy Operation record that superficially resembles part of a plan
**When** it is loaded alongside genuine plan-attempt records
**Then** it is still presented as a standalone legacy Operation with no fabricated attempt grouping, preserving legacy readability without inventing plan structure.

### Story UX-PB.3a: Confirmed sidecar as the single active plan summary

**Primary concern:** Product Behavior  
**Dependencies:** UX-PB.2 complete (PB.2a-f); D27-D30; AD-16; AD-17; finalized UX spines  
**Blocks:** UX-PB.3b  

As a Pack-Manager user, I want the sidecar I confirmed to become the one live summary of the admitted attempt so that I follow a single plan from review into execution without a new surface appearing.

**Acceptance Criteria:**

**Given** a confirmed plan whose atomic admission returned one durable `planAttemptId`
**When** final confirmation closes the Confirmation Dialog
**Then** the same Upgrade Sidecar transforms in place into the one active plan summary for that `planAttemptId`, focus moves to its programmatically focusable Upgrade Activity summary heading, and the status channel announces plan start.

**Given** a confirmed attempt already summarized live in the sidecar
**When** the user keeps reviewing a draft or attempts a second confirmation
**Then** only one confirmed Upgrade Plan attempt is active — the new draft stays in the Upgrade Plan and cannot be confirmed until the active attempt is terminal, and no second live summary is created.

### Story UX-PB.3b: Full Activity as detailed view of the same attempt

**Primary concern:** Product Behavior  
**Dependencies:** UX-PB.3a; D29-D30; AD-16  
**Blocks:** UX-PB.3c  

As a Pack-Manager user, I want full Activity to be a deeper view of the very same attempt shown in the sidecar so that the compact summary and the detailed evidence are never two different executions.

**Acceptance Criteria:**

**Given** an active attempt rendered in the sidecar
**When** the Activity destination opens for the same `planAttemptId`
**Then** the sidecar and full Activity render one shared live state — the sidecar stays the compact live summary while full Activity shows detailed Operation evidence — and neither is a separate execution.

**Given** the compact sidecar while an Operation needs attention
**When** the condition is summarized there
**Then** the sidecar offers `View full Activity` and defers `Keep waiting`, `Copy command`, `Cancel plan`, and expanded command evidence to full Activity rather than crowding the summary.

### Story UX-PB.3c: Per-item live progress states

**Primary concern:** Product Behavior  
**Dependencies:** UX-PB.3b, UX-PB.2d; D29-D30; AD-16 (`Verifying`/`Skipped` as durable wire states)  
**Blocks:** UX-PB.3d, UX-PB.3f  

As a Pack-Manager user, I want each Package and Manager item to show its own honest live state so that I can see what is running, what is waiting, and what has verified without reading a terminal.

**Acceptance Criteria:**

**Given** an admitted attempt whose Operations carry the same `planAttemptId`
**When** each item advances
**Then** it shows queued, waiting (with the lock or ownership reason), running (indeterminate unless the adapter provides a trustworthy total), verifying, or a terminal state, and a row or status update never moves focus.

**Given** an item whose process has exited successfully
**When** its affected Manager state has refreshed and verified
**Then** only that verified row collapses its `old → new` delta to the single new current version, and an unverified successful exit remains `Verifying`.

**Given** an attempt in progress (live-state stream disconnect/reconnect)
**When** the per-item progress source drops mid-attempt and later reconnects
**Then** each item keeps its last known honest state and is never silently shown complete, the interruption to the live stream is surfaced rather than guessed, and reconnection resumes correlated `planAttemptId` state without fabricating progress.

### Story UX-PB.3d: Verification-gated Results with outcome taxonomy

**Primary concern:** Product Behavior  
**Dependencies:** UX-PB.3c; D29-D30; AD-16 (verification-gated success; post-exit fresh acquisition)  
**Blocks:** UX-PB.3e, UX-PB.3g; Story 6.5

As a Pack-Manager user, I want the plan to become Results only after affected state is verified so that success is earned, not assumed from a process exit.

**Acceptance Criteria:**

**Given** an active attempt whose mutations have all reached a process-terminal state
**When** the required refresh verification for the affected Managers completes
**Then** the attempt becomes terminal, the sidecar transforms in place into a persistent Results Summary that remains until `Done`, focus preserves the current viable node or moves to the Results heading, and one atomic outcome summary is announced (e.g. `12 of 12 updates verified` or `10 of 12 verified · 2 failed`).

**Given** a completed attempt
**When** Results renders
**Then** the overall outcome is exactly one of success, partial, failed, cancelled, timed out, or interrupted, and each item is verified, failed, cancelled, or skipped — mutation failure and verification failure are distinguished, `Skipped` marks only work that never started, and crash-reconstructed unfinished work reads as `Interrupted`.

**Given** an Operation whose process exited successfully (verification-refresh failure/timeout)
**When** the required refresh verification itself errors or times out, distinct from a mutation failure
**Then** the item does not declare success — it stays `Verifying` until it resolves, then reports verification failure with its evidence, and is never colored successful on the strength of the exit code alone.

**Given** an attempt reaching terminal state (Results persistence failure)
**When** the transformed persistent Results / terminal outcome cannot be written
**Then** the failure to persist is surfaced honestly, the visible Results are not presented as durably recorded, and no fabricated success is shown.

### Story UX-PB.3e: Failure guidance and safe next step before Retry

**Primary concern:** Product Behavior  
**Dependencies:** UX-PB.3d; D30; AD-16  
**Blocks:** UX-PB.4a-4e  

As a Pack-Manager user, I want a failed item to explain what happened and what to do next before I see Retry so that I fix the real cause instead of repeating a doomed attempt.

**Acceptance Criteria:**

**Given** a failed item with a known, curated cause
**When** I expand it in Results
**Then** it presents `What happened` and `What to do next` with evidence and safe contextual actions before a secondary Retry, and it names the object that failed (e.g. `rustup refresh failed`) rather than a generic message.

**Given** a failure whose cause is deterministic rather than transient
**When** guidance is shown
**Then** it is not framed as likely fixed by repeated retries; a repeated identical failure says it repeated and emphasizes resolving the known cause before Retry, and an unknown non-zero exit shows evidence without inventing advice.

### Story UX-PB.3f: Trusted Interaction-required classifier

**Primary concern:** Product Behavior  
**Dependencies:** UX-PB.3c; D30; AD-16 (interaction-required policy)  
**Blocks:** UX-PB.4a-4e  

As a Pack-Manager user, I want `Interaction required` to appear only when a trusted classifier recognizes a real prompt so that Pack-Manager never invents prompt meaning from arbitrary output.

**Acceptance Criteria:**

**Given** a running Operation with null input
**When** a closed Manager-specific classifier or explicit native signal recognizes a known prompt
**Then** the Operation shows `Interaction required` with a plain-language explanation plus `Copy command` and `Cancel plan`, and Pack-Manager never accepts the response inline or requests a password.

**Given** a running Operation that has gone silent
**When** no trusted classifier matches the output at the 120-second threshold
**Then** the Operation remains an ordinary stall presenting exactly `Keep waiting`, `Copy command`, and `Cancel plan`, never `Interaction required`.

**Given** output the classifier does not recognize, or a real recognized prompt (interaction-classifier false positive/negative)
**When** the state is derived
**Then** unmatched output is never guessed into `Interaction required` and a classifier-recognized prompt is never left as a silent stall — only trusted classification, never regex or heuristic guessing, converts a stall into interaction.

### Story UX-PB.3g: Two labeled cancellation scopes

**Primary concern:** Product Behavior  
**Dependencies:** UX-PB.3d, UX-PB.2e; D30; AD-16 (attempt-scoped cancellation; `Skipped` marks only never-started work)  
**Blocks:** UX-PB.4a-4e  

As a Pack-Manager user, I want the primary cancel action to clearly stop the whole plan, with an Operation-only cancel reserved for a deliberate diagnostic, so that I always know the scope of what I am stopping.

**Acceptance Criteria:**

**Given** an active confirmed attempt
**When** I choose the primary cancellation labeled `Cancel plan`
**Then** it requires no second confirmation, changes still-running Operations bound to that `planAttemptId` to `Cancelling`, prevents unstarted attempt work from beginning and marks it `Skipped`, promises no rollback, and never delays cancellation behind a dialog.

**Given** a deliberately Operation-scoped diagnostic action
**When** an Operation-level cancel is offered
**Then** it is the only place labeled `Cancel operation`, while generic `Cancel` is reserved for closing a dialog or retry-scope editor without mutating running work.

**Given** an attempt in the verifying window with processes exited and refresh verification pending (cancellation while verifying)
**When** `Cancel plan` is issued
**Then** cancellation is honored immediately for that `planAttemptId`, verifying items resolve to honest terminal outcomes (cancelled or skipped rather than falsely verified), and no item is reported successful because its exit preceded the cancel.

### Story UX-PB.4a: One immutable History row per confirmed attempt

**Primary concern:** Product Behavior  
**Dependencies:** D29; AD-16 (durable `planAttemptId` identity; atomic all-or-none admission); AD-18; UX-PB.3 complete (PB.3a-g)  
**Blocks:** UX-PB.4b, UX-PB.4e  

As a Pack-Manager user, I want each plan I confirm to become exactly one immutable History entry so that every attempt has one durable record instead of scattered per-command rows.

**Acceptance Criteria:**

**Given** a confirmed plan attempt that reaches a terminal state — succeeded, failed, cancelled, interrupted, or partially skipped, and regardless of how many Managers, commands, Packages, failures, or skips it contained
**When** it terminates
**Then** exactly one immutable History row is created for that `planAttemptId`, its Operation-level evidence is nested inside that row, and its summary uses verified-outcome wording such as `10 of 12 verified · 2 failed` rather than a generic completion ratio
**And** no attempt ever yields more than one row or a per-Package or per-command row.

**Given** a confirmed attempt has terminated
**When** its single immutable History row cannot be persisted
**Then** the write failure is surfaced honestly, no partial or fabricated row is presented as a complete History entry, and the durable Operation and crash-journal evidence for that `planAttemptId` remains recoverable rather than silently lost.

**Given** a confirmed attempt was admitted but the app crashed or relaunched before the attempt reached a terminal row
**When** History reconciles on the next launch
**Then** the in-flight attempt is reconciled from its durable `planAttemptId` records into one honest row, an attempt that never reached terminal is shown as interrupted, and no completed outcome is fabricated for work that did not finish.

### Story UX-PB.4b: Read-only Activity replay of a History row

**Primary concern:** Product Behavior  
**Dependencies:** D29-D30; AD-16; UX-PB.4a  
**Blocks:** UX-PB.4c, UX-PB.4d  

As a Pack-Manager user, I want opening a History row to route Activity into read-only replay so that I can inspect exactly what a prior attempt did instead of piecing together unrelated commands.

**Acceptance Criteria:**

**Given** a completed History row for a confirmed `planAttemptId`
**When** I open it
**Then** Activity enters a clearly labeled read-only replay that reconstructs the attempt's Manager groups, Package/version changes, Manager self-updates, exact commands, Operation outcomes, errors, timings, and retained output
**And** no control in the replay can mutate, re-run, or execute anything.

**Given** a History row whose persisted attempt is corrupted or missing
**When** I try to open its replay
**Then** the load failure states what could not be reconstructed, the History list stays intact and navigable, and no partial reconstruction is presented as a complete or trustworthy replay.

### Story UX-PB.4c: Live and replay coexistence with the live attempt primary

**Primary concern:** Product Behavior  
**Dependencies:** D30; UX-PB.4b  
**Blocks:** No dependent sub-story (leaf of the UX-PB.4 spine)  

As a Pack-Manager user, I want a replay I open during a live upgrade to stay clearly secondary so that the one running attempt never looks paused or lost while I inspect a past one.

**Acceptance Criteria:**

**Given** a confirmed plan attempt is running when I open a History replay
**When** the read-only replay opens
**Then** the live sidecar stays visibly live, full Activity is labeled `Viewing past activity`, `Back to live activity` is offered, and choosing it returns the main workspace to the one active attempt without disturbing its progress.

**Given** a replay is open alongside the live attempt
**When** the live attempt emits new status or reaches terminal Results
**Then** the live attempt remains the primary object with authoritative sidecar and Results, and the concurrent replay never suppresses, delays, or overwrites live status.

### Story UX-PB.4d: Retry scope preview and linked new attempt

**Primary concern:** Product Behavior  
**Dependencies:** D29; AD-16 (Retry mints a new linked `planAttemptId` and preserves the original failure); UX-PB.4b, UX-PB.2b  
**Blocks:** Story 6.5  

As a Pack-Manager user, I want Retry to first show the failed-item scope and then create a new linked attempt so that I can re-run only what failed while the original result stays untouched.

**Acceptance Criteria:**

**Given** a terminal Results or History entry with failed items and Retry available
**When** I invoke Retry
**Then** it first reveals the proposed failed-item scope inline with `Cancel` and `Create new plan`; `Create new plan` rebuilds current canonical intent into a new reviewable draft, and confirming that draft creates a new attempt with a fresh `planAttemptId` linked by `retryOfPlanAttemptId` and a `Retry of plan from <time>` History entry
**And** the original failed result stays immutable and reachable through `View previous result`.

**Given** Retry has exposed the failed-item scope
**When** current canonical intent cannot be rebuilt for that scope — for example an item is now pinned, current, removed, or unavailable
**Then** the rebuild failure is explained, no new attempt is admitted, and the original immutable failed result is left unchanged and still visible.

**Given** a Retry attempt links back to its source through `retryOfPlanAttemptId`
**When** the source is missing, the link is dangling or orphaned, or the original would be mutated by the Retry
**Then** the original attempt's History row and result remain immutable and are never overwritten, the lineage is surfaced honestly including when its source cannot be resolved, and no fabricated or repaired lineage is presented as valid.

### Story UX-PB.4e: Legacy Operation History honest labeling

**Primary concern:** Product Behavior  
**Dependencies:** D29; AD-16 (legacy honesty — no inferred plan grouping); AD-18; UX-PB.4a, UX-PB.2f  

As a Pack-Manager user, I want legacy Operation records that predate plan attempts to stay honestly labeled so that older history remains readable without being faked into plans it never had.

**Acceptance Criteria:**

**Given** legacy Operation History records that lack a `planAttemptId`
**When** History renders them
**Then** they remain accessible, are explicitly labeled as legacy Operation entries, are visibly distinct from plan-attempt History rows, and are never grouped or fabricated into a plan attempt they never belonged to.

**Given** a History list mixing legacy Operation entries and plan-attempt rows
**When** the user filters, searches, or opens detail
**Then** legacy entries open their own honest Operation-level detail rather than a fabricated plan replay, plan-attempt rows open read-only plan replay, and the two kinds never merge into a single invented grouping.

### Story UX-PB.5a: Separate final confirmation gate with the `Confirm N Updates` action and `Proceed with Upgrade Plan?` dialog

**Primary concern:** Product Behavior  
**Dependencies:** UX-PB.1 and UX-PB.2 complete; D27, D28; AD-16; finalized UX spines  
**Blocks:** UX-PB.5b, UX-PB.5d  

As a Pack-Manager user, I want the persistent Upgrade Plan to present one deliberate final confirmation before anything runs so that a review step always stands between staging and execution and nothing bypasses it silently.

**Acceptance Criteria:**

**Given** a non-empty Upgrade Plan with confirmation enabled (`skipUpgradePlanConfirmation` is `false`)
**When** the plan footer renders
**Then** it contains exactly one blue `Confirm N Updates` action where N is the count of staged updates, exact commands stay hidden behind `Show update command`, and no safety or skip checkbox appears on the base plan.

**Given** the enabled base plan
**When** I invoke `Confirm N Updates`
**Then** the `Proceed with Upgrade Plan?` Confirmation Dialog opens over a dimmed, focus-trapped background, shows the exact commands that will run, and offers `Change Plan` plus a final `Confirm N Updates`, and nothing executes until the final confirmation is chosen.

**Given** the open Confirmation Dialog
**When** focus lands and I use `Change Plan`, Escape, or the backdrop
**Then** focus moves to the dialog heading/command summary with `Change Plan` as the first actionable control so a final confirmation is never the accidental default for an unfocused Enter, `Change Plan` returns focus to the first staged Remove control or the plan heading, and Escape/backdrop dismiss only while no command has begun and restore focus to the originating `Confirm N Updates` action.

**Given** the open dialog
**When** I choose the final `Confirm N Updates`
**Then** the full plan is admitted atomically through the same review, execution, verification, Results, and History lifecycle as any plan, partial silent admission never occurs, and only one confirmed attempt becomes active.

**Given** a confirmed admission
**When** admission fails
**Then** nothing executes, the dialog explains why, and the plan remains editable for re-review.

### Story UX-PB.5b: Dialog-only disable control with atomic `skipUpgradePlanConfirmation` persistence and Settings migration

**Primary concern:** Product Behavior  
**Dependencies:** UX-PB.5a; D28; FR-17; AD-19; Settings migration  
**Blocks:** UX-PB.5c; Story 3.4  

As a Pack-Manager user, I want to deliberately disable the final confirmation from the dialog and restore it from Settings so that I can remove friction without ever losing a safe default.

**Acceptance Criteria:**

**Given** the `Proceed with Upgrade Plan?` dialog
**When** it renders
**Then** only this dialog contains the `Disable upgrade plan command execution confirmation` control, its safety explanation, and Settings-restoration guidance, and the base plan never surfaces that control.

**Given** the dialog with `Disable upgrade plan command execution confirmation` selected
**When** I choose the final `Confirm N Updates`
**Then** `skipUpgradePlanConfirmation: true` is written atomically, the new value takes effect only after persistence succeeds, and the plan is admitted.

**Given** Settings
**When** the confirmation preference renders
**Then** `skipUpgradePlanConfirmation` defaults to `false`, is reversible there, saves immediately and atomically with visible Saving/Saved/failure states, and any persisted `autoOpenDrawer` is tolerated as inactive legacy input without becoming active.

**Given** a change to `skipUpgradePlanConfirmation` from either the dialog or Settings
**When** the atomic save fails
**Then** the prior preference is retained as both active and persisted state, an inline error is shown, and no partial or legacy value becomes active.

**Given** an interrupted atomic write of the confirmation preference across a crash and relaunch
**When** the app relaunches
**Then** the setting reconstructs to exactly one coherent value, old or new and never partial, and migration re-applies the tolerate-`autoOpenDrawer`-as-inactive-legacy rule.

### Story UX-PB.5c: Confirmation-disabled bypass with expanded commands and native rebuild/stale-validation-gated run

**Primary concern:** Product Behavior  
**Dependencies:** UX-PB.5b; D27, D28; AD-16  
**Blocks:** None (leaf of the confirmation branch)  

As a user who has disabled the final confirmation, I want the run action to still rebuild the commands natively and stale-check the plan so that removing the dialog never removes the real safety.

**Acceptance Criteria:**

**Given** confirmation disabled (`skipUpgradePlanConfirmation` is `true`) and a non-empty plan
**When** the sidecar renders
**Then** exact commands automatically expand, a persistent `Confirmation is off. Changes will run immediately when you choose Run N Updates. Change in Settings.` warning links to Settings, the immediate action is `Run N Updates`, and no dialog opens.

**Given** the confirmation-disabled plan
**When** I choose `Run N Updates`
**Then** Rust rebuilds the exact commands from canonical intent and runs the stale-plan check before the plan is atomically admitted, so the bypass removes only the final dialog and never the persistent plan, native rebuild, stale check, or explicit user action.

**Given** the confirmation-disabled bypass path
**When** native rebuild or stale validation fails, for example a Package pinned, updated, or removed since staging
**Then** the run is blocked, the invalidated details are replaced and what changed is explained, and nothing executes until the plan is rebuilt and re-authorized.

### Story UX-PB.5d: Accessibility and responsiveness of the confirmation and safety surfaces

**Primary concern:** Product Behavior  
**Dependencies:** UX-PB.5a; finalized focus and high-zoom contracts; FR-19  

As a keyboard and VoiceOver user at high zoom, I want every safety action reachable and announced so that the confirmation gate protects everyone at the 900 x 600 minimum and at 150-200% zoom.

**Acceptance Criteria:**

**Given** the plan and the `Proceed with Upgrade Plan?` dialog
**When** a keyboard/VoiceOver user operates them with reduced motion active
**Then** the dialog traps focus, exposes meaningful names, roles, and states, honors reduced motion, and every safety action (`Confirm N Updates`, `Change Plan`, the disable checkbox, and `Run N Updates`) has an accessible name and a reachable focus order.

**Given** the 900 x 600 minimum window at 100%, 150%, and 200% zoom
**When** the Plan, Confirmation, Activity, and Results surfaces render
**Then** below 720 usable CSS pixels the layout enters high-zoom mode, navigation collapses to an accessible rail or temporary panel, and Plan/Confirmation/Activity/Results present as a full-workspace or stacked surface with a visible Back route, no overlapping panes, and no two-dimensional scrolling for the primary task, keeping every safety action reachable.

**Given** the open Confirmation Dialog
**When** it is dismissed via `Change Plan`, Escape, backdrop, or final confirm and the return target no longer survives
**Then** focus is restored to a defined fallback (the first staged Remove control or the plan heading) rather than lost to the document body, and focus is never stranded inside a closed dialog.

**Given** 150% zoom, 200% zoom, or the 900 x 600 minimum
**When** a safety action would otherwise clip or overflow
**Then** it remains fully visible and operable with its name, state, versions, primary action, error/recovery, focus order, and announcements preserved, and no safety action becomes unreachable behind an overlapping or two-dimensionally scrolling pane.

### Story UX-PB.5e: Application-update presentation kept separate from Package plans and History

**Primary concern:** Product Behavior  
**Dependencies:** UX-PB.4 complete (History must exist to assert separation); finalized application-update presentation  

As a Pack-Manager user, I want the application's own update to appear only as a restrained `Pack-Manager Update Ready!` badge that links into Settings so that it never mixes with Package Upgrade Plans, Activity, Results, or History.

**Acceptance Criteria:**

**Given** an available application update
**When** the shell and Settings render
**Then** one restrained application-level `Pack-Manager Update Ready!` badge links to Settings, Pack-Manager updates, where the update card heading is simply `Pack-Manager` and the installed-to-target version delta stays on one unbroken line with the installed version in warning yellow and the target version in success green.

**Given** active or historical Package work
**When** application-update state changes (checking, available, downloading, ready to restart, blocked by active work, or error)
**Then** it never appears in a Package Upgrade Plan, draft plan, live confirmed plan attempt, Results, or plan-attempt History, and Package Activity and History never absorb the application update.

**Given** a plan-attempt History row open in read-only Activity replay
**When** an application update becomes ready during the replay
**Then** readiness surfaces only via the separate `Pack-Manager Update Ready!` badge and the Settings card and never injects into the replayed attempt, its Operations, or the History list.

## Epic 2: Make Detection and Refresh Fail Independently and Recover Usefully

Users can detect and refresh every supported Manager with clear phase, absence, timeout, offline, and failure behavior while peer Managers and Last-good Snapshots remain usable.

### Story 2.2: Prove Refresh Phases and Per-Manager Timeouts

As a Pack-Manager user,
I want refresh settings, phases, and timeouts to behave consistently per Manager,
So that a slow or disabled step never creates misleading global state.

**Story Contract:**

- FR and requirement links: FR-3; FR-17
- Required test level: Unit plus component
- Governing invariants: AD-4
- Dependencies: deterministic adapters and fake time

**Acceptance Criteria:**

**Given** Homebrew metadata refresh is enabled or disabled
**When** a Brew refresh is planned and rendered
**Then** the enabled path shows the required update/inventory/outdated phase order
**And** the disabled path omits only the metadata-update phase without mislabeling later phases.

**Given** each of the six Manager adapters and its documented timeout boundary
**When** controlled time reaches success, timeout, or error outcomes
**Then** the correct Manager-specific terminal state and actionable detail appear
**And** peers continue independently without real network access or wall-clock sleeps.

## Epic 3: Keep Package Choice, Plans, and Settings Exact and Understandable

Users can understand Package state, select only eligible work, review exact commands and exclusions, reject stale plans, perform bounded row-level updates, and control Settings without misleading or inaccessible UI state.

### Story 3.1: Present Complete Package State and Manager Detail

As a Pack-Manager user,
I want complete Package rows, expandable details, self-update separation, and non-color status cues,
So that I can understand what each Manager reports without losing Manager-specific meaning.

**Story Contract:**

- FR and requirement links: FR-2; FR-5; FR-6; FR-10; FR-11; FR-19
- Required test level: Component
- Governing invariants: AD-16, AD-17
- Dependencies: representative all-state fixtures

**Acceptance Criteria:**

**Given** representative current, Outdated, pinned, self-updating/greedy, unknown-version, and error Packages
**When** Manager Package tables render
**Then** name, installed/latest values, status text, eligibility, selection, and the row plan action that adds or removes the Package's stable identity in the one persistent editable draft Upgrade Plan without executing are complete and understandable without relying on color.

**Given** uv executable details and npm's own outdated row
**When** the user expands/searches uv content and views npm
**Then** uv executables are reachable and searchable
**And** npm self state appears only in its Manager Card/Header — where `Update Manager` stages an independent, individually-removable self-update plan item surfaced as `IN PLAN` / `Remove` and never executes directly — while the four ordinary Package rows remain.

### Story 3.2: Enforce Pinned and Greedy Eligibility

As a Pack-Manager user,
I want pinned formulae and self-updating casks handled by their documented policies,
So that no plan silently overrides a pin or includes default-excluded work.

**Story Contract:**

- FR and requirement links: FR-5; FR-6; FR-7
- Required test level: Unit plus component
- Governing invariants: AD-16, AD-17
- Dependencies: Story 3.1; deterministic plan-builder and UI fixtures

**Acceptance Criteria:**

**Given** pinned Homebrew formulae
**When** selection, row plan-add, per-Manager update-all, update-selected, and Update Everything draft-entry paths are exercised across every active filter
**Then** pinned rows stay inert, add nothing to the draft Upgrade Plan, and are explained, disabled, and excluded from every plan with the correct reason.

**Given** ordinary and greedy-only casks
**When** the default and explicit opt-in flows execute
**Then** greedy-only casks are the documented set difference, remain separate/collapsed/default-excluded, and enter a plan only through explicit opt-in with visible disclosure.

### Story 3.4: Validate Every Settings Control and Environment Report

As a Pack-Manager user,
I want every Settings control and Environment Report action to validate, persist, and report failures clearly,
So that configuration changes and environment evidence remain trustworthy.

**Story Contract:**

- FR and requirement links: FR-17
- Required test level: Unit plus component
- Governing invariants: AD-4, AD-5, AD-19
- Dependencies: controlled persistence and clipboard seams

**Acceptance Criteria:**

**Given** the retained editable stall threshold, hard cap, and log level plus `skipUpgradePlanConfirmation` (default `false`) as the configurable Settings — each with its default, valid bounds, invalid input, and a persistence failure — and an old persisted `autoOpenDrawer` value carried over after the Activity auto-open preference was removed
**When** the user changes Settings
**Then** valid values persist before becoming active, invalid values are rejected, save failure changes neither active nor persisted state, and log-level changes apply live only after persistence
**And** `skipUpgradePlanConfirmation` is validated and persisted as a first-class control, the Activity auto-open preference is removed from active Settings while any old persisted `autoOpenDrawer` value is tolerated during migration without ever becoming active, the new value applies only after atomic persistence succeeds, and every control saves immediately and atomically with visible `Saving`/`Saved`/failure state.

**Given** the complete current detection and ToolEnv state
**When** Environment Report opens and Copy is used
**Then** every required field and evidence value is present
**And** copy success and failure are visible and actionable.

### Story 3.5: Preserve Exact Keyboard Selection and Row Plan Actions

As a Pack-Manager user,
I want keyboard selection and single-row plan actions to preserve exact Package identity,
So that I can act efficiently without adding excluded or unrelated Packages to the Upgrade Plan.

**Story Contract:**

- FR and requirement links: FR-6; FR-10; FR-13; FR-19
- Required test level: Component plus browser E2E
- Governing invariants: AD-16, AD-17
- Dependencies: Stories 3.1 and 3.2; semantic keyboard/focus locators; deterministic bridge

**Acceptance Criteria:**

**Given** eligible, current, pinned, greedy, filtered, and range-addressable rows
**When** toggle, shift-range, tri-state, Cmd+A, Space, Cmd-click, Clear, and Esc interactions execute
**Then** the exact selectable identities and visible filter semantics are preserved
**And** excluded rows never enter selection.

**Given** one eligible or ineligible Package row
**When** the single-row plan action is invoked
**Then** exactly one eligible Package's canonical identity is added to (or removed from) the persistent draft Upgrade Plan, nothing is built, submitted, enqueued, or executed, and the sidecar reflects the membership change
**And** ineligible, pinned, or current rows add nothing, stay inert with an explained reason, and never expand the selection
**And** the resulting one-Package plan enters the same review and separate-confirmation path as a multi-Package plan, with its execution, verification, Results, and History lifecycle proven by the later-wave stories that own those stages.

## Epic 6: Preserve State, Evidence, and Privacy Across Failure and Relaunch

Users can reconstruct Operations after crashes, trust Settings and durable stores across failure, reveal native evidence safely, and export exact diagnostics without inherited-environment disclosure or hostile-path traversal.

### Story 6.5: Export Exact Native Diagnostics and Visible Outcomes

As a Pack-Manager user,
I want diagnostics export to create the documented archive and report native outcomes,
So that support evidence is complete, inspectable, and actionable.

**Story Contract:**

- FR and requirement links: FR-18
- Required test level: Real native Tauri E2E plus artifact inspection
- Governing invariants: AD-3, AD-4, AD-5, AD-16, AD-18
- Dependencies: disposable logs/transcripts/journal

**Acceptance Criteria:**

**Given** documented default destination, alternate permission outcomes, and invocation from Settings and History
**When** diagnostics export runs through the production native command
**Then** the timestamped ZIP path and visible success/failure match the contract.

**Given** more than three app logs, 25 transcripts, 1,000 journal records, and durable plan-attempt records correlated by `planAttemptId`
**When** the produced ZIP is opened and inspected
**Then** it contains `report.json`, the newest three app logs, newest 25 transcripts, `operations.jsonl`, and the durable plan-attempt records that correlate the exported evidence — each carrying its `planAttemptId`, reviewed Manager/Package scope, exact commands, verification facts, results, and optional `retryOfPlanAttemptId` — with exact expected contents and no missing required entry, including those plan-attempt entries.

**Given** Export diagnostics and Open Logs actions
**When** native command/opener success and failure are controlled
**Then** the UI exposes actionable outcomes

