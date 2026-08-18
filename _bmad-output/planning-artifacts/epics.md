---
stepsCompleted:
  - step-01-validate-prerequisites
  - step-02-design-epics
  - step-03-create-stories
  - step-04-final-validation
# Live, authoritative inputs.
inputDocuments:
  # Requirements authority. Read the addendum with it.
  - _bmad-output/planning-artifacts/prds/prd-Pack-Manager-2026-07-25/prd.md
  - _bmad-output/planning-artifacts/prds/prd-Pack-Manager-2026-07-25/addendum.md
  # Architecture authority (revision 10).
  - _bmad-output/planning-artifacts/architecture/architecture-Pack-Manager-2026-07-23/ARCHITECTURE-SPINE.md
  - _bmad-output/planning-artifacts/ux-designs/ux-Pack-Manager-2026-07-23/DESIGN.md
  - _bmad-output/planning-artifacts/ux-designs/ux-Pack-Manager-2026-07-23/EXPERIENCE.md
  - _bmad-output/planning-artifacts/ux-designs/ux-Pack-Manager-2026-07-23/validation-report.md
  - _bmad-output/planning-artifacts/story-triage-2026-07-24.md
  - _bmad-output/planning-artifacts/sprint-change-proposal-2026-07-25.md
  - _bmad-output/planning-artifacts/sprint-change-proposal-2026-07-25-spine-rev10-residuals.md
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

Later the same day it was reconciled again, against **`ARCHITECTURE-SPINE.md`
revision 10**, the **finalized PRD**
(`prds/prd-Pack-Manager-2026-07-25/prd.md`, with its `addendum.md`), and
`docs/DECISIONS.md` **D36, D37, and D38**. That run applied the spine's
`epics.md` residuals row. Record:
`_bmad-output/planning-artifacts/sprint-change-proposal-2026-07-25-spine-rev10-residuals.md`.
Two authorities now sit above this document and they do not overlap: **the PRD
wins on requirements** and **the spine wins on architecture invariants**. This
document restates neither and cites both.

## Requirements Inventory

**Requirements authority is
`_bmad-output/planning-artifacts/prds/prd-Pack-Manager-2026-07-25/prd.md`**
(status `final`), read together with its `addendum.md`. The FR/NFR entries below
are a Phase 3 convenience index into that PRD, **not a second statement of the
requirements**: where this document and the PRD disagree, **the PRD wins**,
exactly as the spine wins on architecture invariants. Four consequences a reader
needs before reading a single entry:

- **FR-19 and NFR-6 below are restated per `docs/DECISIONS.md` D37.** Keyboard
  operability of primary actions, VoiceOver operability, live-region
  announcement of plan progress / verification / cancellation / failure /
  completion, and deterministic dialog and sidecar focus restoration are
  **removed as criteria** (`prd.md` FR-19 Notes). What stays, and stays for
  reasons that are not accessibility obligations: the focus indicator (D35,
  `ARCHITECTURE-SPINE.md` AD-27), `⌘X`/`⌘C`/`⌘V`/`⌘A` and RP-2's accelerator
  map, the 4.5:1 contrast floor (D36), and **pointer-facing** explanations of
  why a Package is ineligible.
- **Scope any D37 application by named section, never by a mention count.**
  Applying D37 means *recording* what was retired, and recording it uses the
  same words — the spine's own keyword count rose while it applied D37
  correctly. `docs/DECISIONS.md` D37 and `addendum.md` §3 carry the named
  sections; the counts D37 quotes were copied rather than measured and are
  wrong.
- **`Cancelling` is not a state, durable or otherwise.** `prd.md` FR-13 and
  `ARCHITECTURE-SPINE.md` AD-16 both refuse it by name: cancellation moves an
  Operation straight to its terminal state, and the 5-second SIGTERM grace
  window is never surfaced as its own status. No story below may add the
  variant.
- **FR-23 post-dates this document's inventory.** `prd.md` §4.3 carries it and
  this document did not, so it is listed below and in the FR Coverage Map, where
  it is owned by **Epic UX-PB** with no new story: its safety property ships and
  its unbuilt limb joins the 2026-07-24 amendment's supersession umbrella
  (owner decision, 2026-07-25).

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

FR-13: Expose queued, running, verifying, stalled, and terminal plan state with exact nested Operation commands/live output correlated by `planAttemptId` and `opId`; use the sidecar as live progress and Results; make Activity a first-class destination; bound live output; and preserve complete retained transcript output. **There is no distinct `cancelling` state** — cancellation moves an Operation to its terminal state and the 5-second SIGTERM grace window is not surfaced as its own status (`prd.md` FR-13, `ARCHITECTURE-SPINE.md` AD-16).

FR-14: Turn silence and excessive duration into honest actionable states using the 120-second default stall threshold, Keep waiting/Copy command/Cancel plan choices, trusted-only interaction classification, the 30-minute default hard cap, attempt-wide cancellation with process-group escalation, explicit terminal outcomes, and an explicit quit choice without promising rollback.

FR-15: Durably correlate each confirmed Plan Attempt's reviewed intent, command snapshot, Operations, verification, Results, and Retry lineage through `planAttemptId`; reconstruct unfinished work as Interrupted without signaling historical process identifiers; present one History row per attempt with Activity replay and nested transcript detail; preserve honest legacy Operations; apply only D26's closed literal repair; and retain/prune durable evidence as specified.

FR-16: Refresh affected state after successful work, retain prior useful Manager state on failure, provide actionable error feedback, and expose View log only when a corresponding log exists.

FR-17: Persist Settings before changing active values or the canonical revision; leave both unchanged on save failure; default upgrade confirmation on through `skipUpgradePlanConfirmation: false` (**Planned — D28**; the field does not ship today); retire `autoOpenDrawer` from the Settings view and the target field set along with the `ActivityDrawer` surface it controls (**Planned — D27–D30**, `prd.md` FR-17, `ARCHITECTURE-SPINE.md` AD-17) while an old persisted value stays tolerated on read and inert (AD-19); support editable thresholds/live log level; and provide Environment Report, Copy, Open Logs Folder, diagnostics export, and Re-detect.

FR-18: Export one timestamped diagnostics ZIP to the documented Desktop path containing `report.json`, the newest three application logs, newest 25 transcripts, and `operations.jsonl`; include app/OS/architecture, constructed ToolEnv and detection evidence, Settings, and log filter; exclude inherited environment values; and reject symlink substitution during selection and streaming.

FR-19: Preserve one coherent dark-only macOS interface across Dashboard, expandable Manager navigation, Manager workspaces, persistent Upgrade Plan, separate Confirmation Dialog, Activity, Results, one-plan-per-row History, Settings, status, and app menus; carry a text or icon equivalent for every color state; draw a visible focus indicator on every interactive element as a real `outline`, never a `ring-*` and never `outline-none` (D35, AD-27); preserve VersionDelta as display-only; honor reduced motion; meet at least 4.5:1 text contrast with the floor CI-asserted (D36); and remain usable at 900 × 600, 150–200% zoom, more than 100 Packages, and long output.

**Restated per `docs/DECISIONS.md` D37.** The prior wording required primary actions be "keyboard/VoiceOver operable with deterministic focus". Removed as criteria: keyboard operability of primary actions, VoiceOver operability, and live-region announcement of plan progress, verification, cancellation, failure, and completion. The focus **indicator** above is kept by D37 by name — it is a rendering mechanism governed by AD-27, not a keyboard-navigation obligation, and a pass scoping D37 by searching for the word *keyboard* will hit AD-27 and must not delete it.

FR-20: Check for application updates and automatically download a newer authorized release in the background while keeping install/restart under user control, Package work understandable, and checking/available/downloading/ready/failure states visible.

FR-21: Install a downloaded application update only after the user chooses Restart to update; never silently install or restart; refuse install/relaunch while a Package Operation is queued or running; relaunch as the intended version; produce manual-install-required for a non-writable install; and keep every update-stage failure actionable.

FR-22: Support the declared Apple-silicon and Intel promise through normal Finder/Dock launch and accept only updater payloads authorized for the installed application; report success only after relaunch as the intended version.

FR-23: Constrain which Manager-suggested fixes become runnable — only the exact recognized suggestion exposes a trusted fix command, the command exposed is the backend's canonical one and never a string scraped from Manager output, and altered, missing, or malformed suggestions stay visible as warning detail while being neither copyable nor runnable. **`prd.md` §4.3 places this between FR-12 and FR-13; it post-dates this document's inventory and is owned by Epic UX-PB with no new story** — see the FR Coverage Map for the split between the part that ships and the part the 2026-07-24 amendment supersedes. A Health issue's `Run fix` is the fourth immediate-execution call site, deliberately outside FR-6's scope and routed through the plan under this FR by D27–D30. The closed set of three immediate-execution *kinds* may not grow; a fourth kind is a new decision.

RP-1: Preserve launch, six-hour, and app-menu update checks; restore in-process update state after supported UI recreation; preserve saved trigger policy across normal relaunch; ensure failed/interrupted downloads never appear Ready; keep application-update state separate from Package Operation queue and History; and validate this mandatory prerequisite through `docs/RELEASE-CHECKLIST.md`.

RP-2: Preserve standard macOS Edit and Window menu actions, including cut/copy/paste/select-all in search and every copyable command surface, as a mandatory prerequisite validated through `docs/RELEASE-CHECKLIST.md`.

### NonFunctional Requirements

NFR-1: Fail closed so unreviewed, stale, altered, replayed, partially admissible, or privilege-seeking work never runs and all user exclusions and Manager protections remain authoritative.

NFR-2: Isolate and recover from detection, refresh, parse, network, update, crash, cancellation, timeout, and persistence failures without blanking another Manager or destroying a Last-good Snapshot.

NFR-3: Render progressive state without waiting for all Managers; remain interactive with more than 100 Package rows; prove reachability and correct actions at 101 rows; flush live output at 50 milliseconds, 64 lines, or 8 KiB; retain the newest 5,000 live lines at 5,001 while preserving the complete transcript; and keep navigation, plan, confirmation, Activity, Results, and recovery usable at 900 × 600 and 150–200% zoom.

NFR-4: Correlate status, output, transcript, structured log, History, and diagnostics through durable Plan Attempt identity and nested Operation identity; block spawn when transcript creation fails; and keep later noncritical logging failures from hanging Package work.

NFR-5: Send no telemetry, expose no generic shell surface, exclude inherited environment values from logs and diagnostics, and resist diagnostic symlink substitution.

NFR-6: Provide non-color status cues and **pointer-accessible** ineligibility reasons, meet at least 4.5:1 text contrast, keep a visible focus indicator on every interactive element (see FR-19 for the mechanism and why it is not an accessibility obligation), honor reduced motion, keep VersionDelta display-only, and stay usable at 900 × 600, at 150–200% zoom, and with more than 100 Packages.

**Restated per `docs/DECISIONS.md` D37.** Removed as criteria: keyboard operability of primary interactions, VoiceOver operability, deterministic dialog and sidecar focus restoration, and announcement of plan progress, verification, cancellation, failure, and completion. `prd.md` NFR-6 records the one unbuilt limb that survives — the explanatory-disabled treatment on ineligible rows, which is **pointer-facing** behavior (FR-5, AD-16, D38) and is owned by Story UX-PB.1d and Story 3.2 below.

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
  from a fixture or from the browser double. Proving delivery waits on the native
  Tauri harness, which `ARCHITECTURE-SPINE.md` records as **OPEN — owner Story
  6.5; shape named, not yet adopted**, not as a bare deferral. **AD-26** governs
  it, because the macOS route runs an embedded WebDriver server *inside* the
  application. A compliant shape exists — the automation surface excluded from
  release bits at compile time — so Story 6.5 is buildable; what remains open is
  the adoption itself, an AD-20 security-reviewed change.
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
- DR-2 is RESTATED by **D33** and then narrowed by **D36** and **D37**, and both halves of its former text are now closed rather than outstanding. Automated 4.5:1 contrast and reduced-motion checks belong in the existing Playwright/Vitest lane — and **both now exist and run in CI.** Reduced motion: `src/styles/theme.css` honors `@media (prefers-reduced-motion: reduce)` and `tests/e2e/browser-style-contract.spec.ts` emulates `{ reducedMotion: "reduce" }` and asserts transitions and animations resolve to `0s`. Contrast: **D36 landed the 4.5:1 guard in commit `a201fb0`** — that same spec reads the rendered primary button's real computed foreground and background, applies the WCAG 2.1 luminance formula, fails below 4.5:1, and carries a negative assertion against `text-white` returning. Both run on every push and pull request to `main` via `.github/workflows/test.yml`. **Neither is a gap to schedule; both are regression surfaces to preserve** — scheduling shipped work is the error `ARCHITECTURE-SPINE.md` AD-1 forbids. The guard is a *sample*, not a sweep: it measures the primary button only, so every other fill remains a by-eye check, and no story may read a green run as a whole-app contrast guarantee (AD-27). **The manual VoiceOver pass this restatement previously added to `docs/RELEASE-CHECKLIST.md` is deleted by D37** and must not be reinstated or reported as a gap; `docs/RELEASE-CHECKLIST.md` was rescoped in `5c8996f` and now states the removal affirmatively.
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
| Suites green while the real command/event boundary is broken | AD-3 (committed contract fixtures; delivery coverage explicitly unproven) and AD-26 (a native automation surface never reaches release bits — the harness that would prove delivery is OPEN with Story 6.5 as its owner, not deferred) |
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
| Product Behavior Prerequisite UX-PB.1..UX-PB.5 | `APPROVED TARGET — NOT IMPLEMENTED` | Product/UX/Architecture accept; Development implements | Epic UX-PB is the primary build queue and runs first, and nothing blocks starting it — the canonical design-token set that blocked UX-PB.1e and UX-PB.5d was decided and shipped (`docs/DECISIONS.md` D35), so both are startable. Any story or test text authored against immediate row execution, direct self-update execution, the Activity drawer, Operation-row History, or active `autoOpenDrawer` behavior is superseded by D27-D30. |
| Canonical design-token set | `CLOSED` — D35 | Resolved 2026-07-25 | Nothing blocked. `DESIGN.md`'s palette was adopted into `src/styles/theme.css`, focus gained a dedicated indicator, and the CI style contract moved with it in one change — see `ARCHITECTURE-SPINE.md` AD-27 and the *Canonical design-token set* row of its Decision Status table. Retained for the reasoning, since the conflict recurs whenever a story proposes its own values: previously `src/styles/theme.css` ships one palette and `tests/e2e/browser-style-contract.spec.ts` asserts it on every push and PR to `main`, while `DESIGN.md` and `EXPERIENCE.md` specify another plus a dedicated `focusRing` that `docs/SPEC.md`'s accent-coloured ring contradicts. Both stories are bound to build from the UX sources, so whichever lands first either rewrites the tokens and breaks the CI style contract on `main` — the same lane AD-11 relies on for reduced motion — or keeps the shipping values and ships focus rings `EXPERIENCE.md` forbids. The token set and the focus mechanism are decided together, then the CI assertion moves with them in one change. Not a story's call and not architecture's alone. |
| DR-1 — minimum supported macOS | `CLOSED` — D31 | Resolved 2026-07-24 | None. 15.0 declared and shipped in v1.0.0. The `notarytool` `minos 15.0` question is CLOSED by `docs/DECISIONS.md` D34: CI and release moved to `macos-15`, so the build SDK is no longer behind the declared floor and the mismatch the question was about no longer exists. A manual Release run verified signing and notarization on the new image. |
| DR-2 — packaged accessibility method | `CLOSED` — D33, then D36 and D37 | Existing Playwright/Vitest lane | **None, and nothing is outstanding.** Reduced motion *and* automated 4.5:1 contrast both run in CI (`tests/e2e/browser-style-contract.spec.ts` via `.github/workflows/test.yml`); the contrast guard landed with D36 in commit `a201fb0`. **No story owes a contrast check** — this row previously named one as the outstanding obligation and a builder consulting it would schedule shipped work, which AD-1 forbids. The guard is a named-sample assertion on the primary button, not a whole-app sweep. The manual VoiceOver pass is deleted by D37 and is not a gap. |
| DR-3 — physical Intel requirement | `NARROWED` — D32 | Resolved 2026-07-24 | None. Universal build retained; verification Apple silicon only. |
| DR-4 — P0 gate/retry policy | `DISSOLVED` — D33 | Retired with the gate | None. |
| Named assignees and calendar dates | `REMOVED` — D33 | n/a | None. The `Assignee` and `Calendar date` fields were removed from every surviving story on 2026-07-25. |
| Native Tauri E2E harness and runner | `OPEN` — owner Story 6.5; shape named, not yet adopted | Architecture accepts; Development implements | None. Story 6.5's "Real native Tauri E2E plus artifact inspection" test level needs no renegotiation: `ARCHITECTURE-SPINE.md` **AD-26** names a compliant shape — `tauri-driver` driven directly does not cover macOS, `@wdio/tauri-service` does by running an embedded WebDriver server inside the app, and that surface is excluded from release bits at **compile time** (`#[cfg(debug_assertions)]`), never by a runtime selector. Any choice must satisfy AD-2, AD-3, and AD-26; adopting the plugin is an AD-20 security-reviewed change, and the CrabNebula fork alternative carries a paid macOS API key. |
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
it names except 3.1, 3.2, 3.4, 3.5, and 6.5 was archived on 2026-07-25 — Stories 3.3,
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

FR-1: **Partly revived by a later decision.** The install-hint limb is **Epic 2 — Story 2.5**: `docs/DECISIONS.md` D40 (2026-08-18) postdates the D33 rescope and extends "install hint where one is known" to all six Managers, plus the all-absent first-run guidance. The `Triaged out (was Epic 4)` status stays for the detection-proving limbs — Prove Manager detection and refresh through the shared production-native boundary.

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

FR-14: **Partly revived by a later decision.** The quit-guard limb is **Epic 6 — Story 6.6**. The `Triaged out (was Epic 5)` status came from the D33 rescope of 2026-07-24; **D30 and AD-30 both postdate it** (2026-07-25) and require the guard, and `prd.md` FR-14 carries it as a requirement with the AD-30 architecture binding. The stall, cancellation, and timeout limbs stay as the triage left them, pending the broader FR reassignment the 2026-07-25 implementation-readiness report raised as its item 4.

FR-15: Epic 6 — Preserve reconstructible History, transcripts, journals, and crash evidence.

FR-16: Epic 2 — Preserve useful Manager state and actionable recovery after outcomes.

FR-17: Epic 3 — Expose and validate user-controlled Settings; Epic 6 supplies the cross-cutting persistence acceptance.

FR-18: Epic 6 — Export privacy-preserving diagnostics through native filesystem boundaries.

FR-19: Release checklist — Validate the coherent macOS interface in the installed packaged application. Its keyboard and VoiceOver limbs are removed by D37 and the checklist steps that carried them are gone; contrast and reduced motion are CI-asserted rather than checklist steps.

FR-20: Release checklist — Validate application-update discovery and background download.

FR-21: Release checklist — Validate explicit install/relaunch, active-operation refusal, and non-writable behavior.

FR-22: Release checklist — Attest normal packaged launch and authorized, coherent release/update artifacts.

FR-23: Epic UX-PB — Constrain which Manager-suggested fixes become runnable. **Owner decision, 2026-07-25; no new story is required, and the requirement splits by what ships.**

- **The safety property ships today and needs no story.** The load-bearing rule is that a Manager's *scraped* suggestion never becomes runnable text: `src-tauri/src/managers/parse/uv.rs:82`-`:83` computes `fixable` as `SAFE_TOOL_NAME_RE.is_match(&name) && suggested_fix.as_deref() == Some(canonical_fix.as_str())` — a byte-equality test between the scraped suggestion and the argv Pack-Manager constructed itself — and `:93`-`:94` expose `fix_command` / `fix_args` only when that holds, leaving an altered or malformed suggestion visible in `detail` but neither copyable nor runnable. `src-tauri/src/queue.rs:334` re-checks `!issue.fixable` at submission and refuses, and the command reaches the frontend as a distinct `HealthFix` kind (`src-tauri/src/ipc.rs:93`) registered at `src-tauri/src/lib.rs:240` (`commands::run_health_fix`) and invoked from `src/components/manager/HealthBanner.tsx:43`. **Recorded as satisfied. Do not schedule it** — `ARCHITECTURE-SPINE.md` AD-1's second rule forbids scheduling shipped behavior.
- **The unbuilt limb is routing `Run fix` through the draft plan under D27–D30**, and it is the identical work as the other immediate-execution bypasses — a Package row's own update action and a Manager's self-update. Those are handled by the **2026-07-24 Correct Course amendment's supersession umbrella** (see the Overview above and `#### 2026-07-24 Correct Course story amendment`) rather than by a dedicated story, and FR-23 joins that umbrella under Epic UX-PB on the same terms. `prd.md` FR-23 counts three immediate-execution *kinds* while FR-6 counts four *call sites*; both are correct and measure different things, and a story that removes "three" must confirm which. **The set may not grow: a fourth kind is a new decision, and SM-2 is the metric it would breach.**

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
**Retained stories:** 2.2, plus 2.5 — added 2026-08-18 by `sprint-change-proposal-2026-08-18.md`, implementing D40. The other three were triaged out on 2026-07-25 (D33); see `_bmad-output/archive/2026-07-24-scope-recalibration/planning/epics-1-6-triaged-out.md`.

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
**Dependencies:** D27-D30; AD-16; AD-17; AD-28 (a Package checkbox **is** membership — there is no transient selection set, and the live `selection` set in `src/store/packages.ts` retires with this story); finalized UX spines; AD-27 (focus is a 2px `outline` in `--color-focus-ring`; never a `ring-*`/box-shadow, which WKWebView drops on native-appearance controls)  
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
**Dependencies:** UX-PB.1a; D27-D30; AD-16; AD-17; AD-28 (this story renders the `Updates` / `Managers` / `Commands` counts, and the count a batch reports is defined there — it is the size of the concrete identity set the batch carried, computed from the snapshot the user was looking at); **AD-30** (a quit that would orphan a live child process is guarded at **one** enforcement point, and its active set is `Queued` ∪ `Running` — the relaunch behavior below describes state after a *guarded* quit, and Story 6.6 builds that guard; this story does not build it and must not add a second one); AD-27 (focus is a 2px `outline` in `--color-focus-ring`; never a `ring-*`/box-shadow, which WKWebView drops on native-appearance controls)  
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

**Given** an in-progress draft when Pack-Manager is quit cleanly, crashes, or is force-quit
**When** Pack-Manager relaunches
**Then** it starts with an empty draft and a hidden sidecar — the draft is session-scoped and never written to disk, so membership is never reconstructed, never partially restored, and never fabricated, and nothing executes on relaunch
**And** a draft is never surfaced as Activity or History.

### Story UX-PB.1c: Remaining draft entry points as independent removable items

**Primary concern:** Product Behavior  
**Dependencies:** UX-PB.1a; D27-D30; AD-16; AD-17; AD-23 (per-member provenance and tombstones); AD-28 (provenance follows the shape of the act — the header checkbox and `⌘A` are `Bulk { scope: FilteredView }`, a range is `Explicit` for every member; and removal is a closed three-way taxonomy in which a scope-wide removal clears the tombstones **only** of the refs whose membership it actually cleared); AD-27 (focus is a 2px `outline` in `--color-focus-ring`; never a `ring-*`/box-shadow, which WKWebView drops on native-appearance controls)  
**Blocks:** UX-PB.1d, UX-PB.1e  

As a Pack-Manager user, I want selected-Package, Manager-header, Manager-wide, and `Update Everything` actions to all feed the same draft as independent removable items so that every entry point stages into one plan and no global toggle bypasses it.

**Acceptance Criteria:**

**Given** eligible work reachable from the count-labeled header Checkbox, the Manager Header `Update Manager` action, a Manager-wide action, and `Update Everything`
**When** I invoke each entry point
**Then** each adds its eligible canonical identities to the same one persistent draft, `Update Everything` seeds all eligible work while remaining editable, every staged Package and every Manager self-update is an independent item with its own visible `Remove`, and no global `includeSelfUpdates` control exists.

**Given** a staged Manager self-update in the draft
**When** I remove it
**Then** only that Manager self-update leaves the plan, Package items in the same Manager group are unaffected, and Rust dedups and rebuilds the authenticated preview from the remaining canonical identities.

**Given** a draft seeded by `Update Everything`, whose expansion was frozen into concrete members at the moment I invoked it — each carrying `Bulk { scope: Everything }` provenance that is never re-evaluated
**When** I remove any item
**Then** that one member leaves the draft and a tombstone records the removal, so no later bulk expansion of any scope re-adds it; the surviving PackageRefs and Manager self-update identities keep their own per-member provenance, and Rust rebuilds the authenticated preview from those canonical identities, never from edited display text
**And** no whole-intent `kind` is stored or converted — there is no `AllEligible` value to convert from and no `Explicit` value to convert to; a kind, where shown, is derived from member origins.

**Given** two entry classes mutating the same draft in close succession
**When** both mutations resolve
**Then** the draft converges to one coherent deduplicated membership set, no item is doubled or lost, and a single authenticated rebuild reflects the final canonical intent.

### Story UX-PB.1d: Ineligible-control inertness with a pointer-reachable explanation

**Primary concern:** Product Behavior  
**Dependencies:** UX-PB.1a, UX-PB.1c; D27-D30; AD-16 (ineligible-item inertness: the control is **inert, not inactive**, and may not use the native `disabled` state); AD-17; AD-28 (the header checkbox's tri-state denominator is the eligible set matching the active filter, including off-screen virtualized rows); D38 (D15's disabled-checkbox *mechanism* is superseded; its substance — a pinned formula is never upgradable in-app — is unchanged); AD-27 (focus is a 2px `outline` in `--color-focus-ring`; never a `ring-*`/box-shadow, which WKWebView drops on native-appearance controls)  
**Blocks:** Story 3.2  

**Restated per `docs/DECISIONS.md` D37, and deliberately not deleted.** D37 names this story and says explicitly that it **is not to be deleted**: its pointer-hover explanation of why a Package is ineligible is mouse-facing behavior D37 protects by name, and only its keyboard and VoiceOver limbs were in scope. The trap this restatement avoids is recorded in `ARCHITECTURE-SPINE.md`: the not-native-`disabled` rule AD-16 now **requires** was previously stated only inside a criterion gated on a keyboard-or-VoiceOver `Given`/`When`, so stripping those limbs would have deleted the only stated trigger for the one rule that must survive. It is re-gated on pointer interaction below. D37 equally forbids stripping ARIA that already ships in `src/` to satisfy a scope decision about plans.

As a Pack-Manager user, I want pinned, current, excluded, and unavailable Packages to stay inert and explain themselves when I point at them so that I understand why they cannot join the plan without guessing.

**Acceptance Criteria:**

**Given** pinned, current, excluded, and unavailable Package controls
**When** any activation path reaches one of them — the criterion is pointer activation, and the inertness is a fail-closed property of the control that no other activation path may defeat either
**Then** membership never changes and each exposes its plain-language reason — pinned `This Package is pinned and cannot be updated. Unpin it, then refresh Pack-Manager to make it selectable.`, excluded `This Package is excluded by your Settings. Change the setting, then refresh Pack-Manager.`, current `This Package is already current.`, and unavailable `An update target is not available. Refresh or view details.`
**And** the bulk header Checkbox scope covers only eligible Packages matching the active filter, including off-screen virtualized rows, and adds no ineligible identity.

**Given** an explanatory-disabled Package control
**When** I hover or click it with the pointer
**Then** it uses `aria-disabled="true"` rather than native `disabled` and stays inert on activation — **a natively disabled form control dispatches no mouse events, so the native state and the pointer-reachable explanation are mutually exclusive and the explanation wins** (AD-16, `prd.md` FR-5)
**And** the reason renders on that pointer interaction rather than living only in an attribute, because the shipping defect D38 diagnoses is exactly a `title` attached to a natively `disabled` input at `src/components/manager/PackageRow.tsx:95`/`:92`, which therefore never renders on hover
**And** reduced opacity alone is not the treatment: ineligibility never rests on gray styling without a text or icon equivalent, and `disabled:opacity-40` (`:100`) is the shipping defect this names, not the reference.

**Given** a Package whose update is delegated to another Manager
**When** its row renders
**Then** it reads `Managed through <Manager>` in plain language and explains the update is grouped and executed through that Manager rather than exposing internal route/owner jargon.

### Story UX-PB.1e: Standardized Manager workspace presentation

**Primary concern:** Product Behavior  
**Dependencies:** UX-PB.1c; D27-D30; AD-16; AD-17; AD-25 (Last-good Snapshot retention on refresh failure); AD-28 (this story renders the Manager `Remove` affordance, so it reads AD-23's provenance and AD-28's closed removal taxonomy — a Manager self-update `Remove` is a single-ref removal and writes a tombstone); AD-27 (focus is a 2px `outline` in `--color-focus-ring`; never a `ring-*`/box-shadow, which WKWebView drops on native-appearance controls)  
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
**Dependencies:** UX-PB.1 complete (PB.1a-e); AD-3; AD-16; D29; AD-27 (focus is a 2px `outline` in `--color-focus-ring`; never a `ring-*`/box-shadow, which WKWebView drops on native-appearance controls)  
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
**Dependencies:** UX-PB.2a; AD-3; AD-16; AD-18; AD-25 (a Manager failure is contained and never destroys a Last-good Snapshot); D29-D30; AD-27 (focus is a 2px `outline` in `--color-focus-ring`; never a `ring-*`/box-shadow, which WKWebView drops on native-appearance controls)  
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
**Dependencies:** UX-PB.2b; AD-16; AD-18 (the plan-attempt journal's home, format, and durability discipline — an append failure is nonfatal, compaction is temp file + fsync + rename); AD-29 (one append authority, exactly two records per attempt, and this story appends the **admission** record); D29; AD-27 (focus is a 2px `outline` in `--color-focus-ring`; never a `ring-*`/box-shadow, which WKWebView drops on native-appearance controls)  
**Blocks:** UX-PB.3 (on UX-PB.2 completion)  

As a Pack-Manager user, I want the confirmed attempt to durably store exactly what I reviewed and the exact commands as a snapshot so that recovery and history are reconstructible and never rebuild executable input from display text.

**Acceptance Criteria:**

**Given** a plan admitted under a new `planAttemptId`
**When** the attempt is persisted
**Then** the append-only **admission** record stores the reviewed Manager/Package scope, Manager self-update identities, exact command snapshot, version evidence, and timestamps as immutable plan-admission metadata
**And** the stored command snapshot is read back only as evidence and is never converted back into executable input
**And** **AD-29 deliberately overrides this story's former record contents:** result and verification state do **not** ride the admission record — they ride the **terminal** record, because an admission record cannot hold a result that does not exist yet and writing the field at admission would either persist a placeholder History must then distinguish from a real outcome, or require the in-place update an append-only journal forbids. Exactly two records exist per attempt, admission and terminal, and this story writes the first
**And** `PlanAttempt.state` is a **derived read-model value, never a persisted field** — no record written here carries it (AD-29). `admitted` and `terminal` are implied by which records exist; `running` and `verifying` live only in the live process's memory.

**Given** a plan admitted under a new `planAttemptId`
**When** persisting the reviewed intent or command snapshot fails
**Then** the failure is surfaced and the prior consistent state is preserved rather than proceeding as if durably recorded — meaning **the visible and durable claims stay honest**, not that admission is blocked and not that a line is deleted from an append-only file. **The append gates nothing** (AD-29, AD-18): an attempt-journal append failure is nonfatal and is surfaced, never fatal to admission, because a full disk must not stop all Package work. "No partial attempt record is left behind" is satisfied by an append that either lands whole or does not land — never by removing a written line
**And** the ordering is mint-and-admit, **then** append; nothing precedes the admission it records. A crash in that window leaves Operations pointing at an attempt that does not resolve, which AD-18 dispositions as legacy, never as corrupt.

**Given** a `planAttemptId` was minted but its durable record was lost to a crash or forced quit mid-admission
**When** Pack-Manager relaunches
**Then** it reconstructs the attempt only from durable plan-admission metadata that actually persisted, leaves no orphaned executable command text, and never resurrects an unpersisted attempt as a completed durable record.

**Given** a persisted attempt whose command snapshot is later read as corrupted or incomplete
**When** the record is loaded
**Then** the integrity failure is detected and the snapshot is refused as an execution source, blocking any display-to-executable round-trip so a damaged snapshot can never be silently re-run.

### Story UX-PB.2d: Correlate every Operation, event, and durable record by planAttemptId

**Primary concern:** Product Behavior  
**Dependencies:** UX-PB.2b; AD-16; AD-18 (`operations.jsonl` keeps its record shape and carries `planAttemptId` only where one exists — *field presence*, conditional, and not a cardinality rule); AD-29 (an attempt is a fold over exactly two records; correlation never depends on a persisted attempt `state` field); D29; AD-27 (focus is a 2px `outline` in `--color-focus-ring`; never a `ring-*`/box-shadow, which WKWebView drops on native-appearance controls)  
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
**And** **"where applicable" is defined here rather than left to two stories to read oppositely:** a record carries `planAttemptId` when the attempt is known at the moment the record is written, and carries `opId` only once the work has become an Operation. A verification refresh has no `opId` until it becomes an Operation, so before that point it correlates by `planAttemptId` alone and its `opId` is absent rather than null-filled or invented; once it is an Operation it carries both. Absence is never a correlation failure, and nothing wedges on it — but a fabricated identifier would break NFR-4
**And** persisted evidence stays correlated to the attempt that produced it rather than standing as flat, uncorrelated Operation records.

### Story UX-PB.2e: Plan-level cancellation that skips unstarted work and escalates running process groups

**Primary concern:** Product Behavior  
**Dependencies:** UX-PB.2b, UX-PB.2d; AD-16 (attempt-scoped cancellation; **there is no `Cancelling` state, durable or otherwise**); AD-29 (this story is the terminal-record writer — see below); D30; AD-27 (focus is a 2px `outline` in `--color-focus-ring`; never a `ring-*`/box-shadow, which WKWebView drops on native-appearance controls)  
**Blocks:** UX-PB.3 (on UX-PB.2 completion)  

**Ordering-critical, and the reason is wire surface rather than story text.** An earlier version of this story moved running work to a `Cancelling` state. `prd.md` FR-13 forbids that state by name and `ARCHITECTURE-SPINE.md` AD-16 refuses it. **This story ships in wave 2**, and adding an `OpStatus` variant is one atomic AD-3 change across the Rust enum, `src/lib/ipc/types.ts`, the guards, and the committed fixtures in `dev/fixtures/ipc/` under D17's byte-equality drift guard — so the correction had to land **before** this story is built, not after, or the fix would be undoing shipped wire surface instead of editing a criterion. It is applied below. **A builder must not add the variant.** `EXPERIENCE.md` still carries the state at its Activity Operation Row and 120-second stall rows; that is a `bmad-ux` Update, and it is a known divergence rather than authority.

**This story owns the single durable terminal write (AD-29).** The terminal append fires on the attempt's terminal transition inside the Rust plan-attempt store, in the same critical section that makes the transition, so it is owned by whichever story first makes an attempt reachable terminal — which is this one, via cancellation, not the History story that reads it. UX-PB.4a is a reader and folder only.

As a Pack-Manager user, I want cancelling the plan to stop only that attempt's work honestly so that unstarted items are marked Skipped, running work is escalated through existing mechanics, and every real outcome is preserved.

**Acceptance Criteria:**

**Given** a confirmed attempt with some Operations running and others not yet started
**When** I choose `Cancel plan`
**Then** cancellation operates only on the Operation IDs bound to that `planAttemptId`: running work escalates through the existing process-group mechanics and moves **straight to its terminal state**, unstarted attempt work is prevented from beginning and recorded as `Skipped`, no second confirmation is required, rollback is not promised
**And** **no `Cancelling` state is introduced at any level** — not a durable wire state, not a presentation state derived in React, and not an event standing in for one. The 5-second SIGTERM grace window is never surfaced as its own status (`prd.md` FR-13, AD-16). `OpStatus` gains no variant from this story
**And** the attempt's terminal transition appends its **terminal** record inside the same critical section that makes the transition, so **no window exists in which a force-quit loses the outcome** (AD-29); verification and result state ride that record, and the append gates nothing
**And** every prior outcome is preserved.

**Given** a plan cancellation where process-group escalation cannot stop some running work
**When** the escalation partially fails
**Then** the work that could not be stopped is reported honestly and never falsely marked cancelled, the successfully cancelled and skipped outcomes remain preserved
**And** no terminal outcome is fabricated for work whose true state is unknown.

### Story UX-PB.2f: Keep legacy Operations honest without inferred plan grouping

**Primary concern:** Product Behavior  
**Dependencies:** UX-PB.2a; AD-16; AD-18 (a record without a `planAttemptId` stays an individually labeled legacy Operation, and a record that loses its counterpart under shared retention reads as legacy rather than as corrupt); D29; **AD-30** (a quit that would orphan a live child process is guarded at **one** enforcement point, and its active set is `Queued` ∪ `Running` — an Operation abandoned by an *unguarded* quit is not the legacy shape this story labels, so the honest-labeling rules here presume Story 6.6's guard rather than substituting for it); AD-27 (focus is a 2px `outline` in `--color-focus-ring`; never a `ring-*`/box-shadow, which WKWebView drops on native-appearance controls)  
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
**Dependencies:** UX-PB.2 complete (PB.2a-f); D27-D30; AD-16; AD-17; finalized UX spines; AD-27 (focus is a 2px `outline` in `--color-focus-ring`; never a `ring-*`/box-shadow, which WKWebView drops on native-appearance controls). **UX-PB.5a is deliberately *not* a dependency:** the trigger above is atomic admission, which UX-PB.2b provides, so this story does not wait on the Confirmation Dialog and must not be resequenced behind it. Keying the trigger to the dialog instead leaves this story undefined on the D28 confirmation-off path, where UX-PB.5c guarantees no dialog opens — do not restore that wording  
**Blocks:** UX-PB.3b  

As a Pack-Manager user, I want the sidecar I confirmed to become the one live summary of the admitted attempt so that I follow a single plan from review into execution without a new surface appearing.

**Acceptance Criteria:**

**Given** a confirmed plan whose atomic admission returned one durable `planAttemptId`
**When** atomic admission completes and returns that `planAttemptId` — reached either through the Confirmation Dialog or, when `skipUpgradePlanConfirmation` is `true`, through the `Run N Updates` bypass that opens no dialog (UX-PB.5c)
**Then** the same Upgrade Sidecar transforms in place into the one active plan summary for that `planAttemptId`, and no second surface opens
**And** **no story is obliged to build a status-announcement channel** (AD-17, D37): "the status channel announces plan start" was a required obligation in the prior wording and is now **optional**. If a channel exists there is exactly **one**, owned alongside the sidecar region — two live regions narrating one attempt is a defect, not additive coverage — and that convergence rule survives D37 because it is about not building two of something
**And** a safety-critical attempt state — the stall handoff and `Interaction required` — reaches the user through a **visible** surface and never depends on an announcement channel (AD-17). Below 720 usable CSS pixels, where the owning surface may be stacked behind another, that state surfaces in a persistent, non-occludable indicator that routes to it.

**Given** a confirmed attempt already summarized live in the sidecar
**When** the user keeps reviewing a draft or attempts a second confirmation
**Then** only one confirmed Upgrade Plan attempt is active — the new draft stays in the Upgrade Plan and cannot be confirmed until the active attempt is terminal, and no second live summary is created.

### Story UX-PB.3b: Full Activity as detailed view of the same attempt

**Primary concern:** Product Behavior  
**Dependencies:** UX-PB.3a; D29-D30; AD-16; AD-27 (focus is a 2px `outline` in `--color-focus-ring`; never a `ring-*`/box-shadow, which WKWebView drops on native-appearance controls)  
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
**Dependencies:** UX-PB.3b, UX-PB.2d; D29-D30; AD-16 (`Verifying`/`Skipped` as durable wire states); AD-27 (focus is a 2px `outline` in `--color-focus-ring`; never a `ring-*`/box-shadow, which WKWebView drops on native-appearance controls)  
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
**Dependencies:** UX-PB.3c; D29-D30; AD-16 (verification-gated success; post-exit fresh acquisition); AD-18 (the plan-attempt journal's home, format and durability — and **only** those: AD-18 deliberately fixes nothing about who appends or how many records an attempt produces); **AD-29** (which does: one append authority, exactly two records, and the fold — it **supersedes this story's former terminal-write assignment**, moving ownership from UX-PB.4a to UX-PB.2e); AD-25 (a failed verification refresh leaves the Last-good Snapshot in place); AD-27 (focus is a 2px `outline` in `--color-focus-ring`; never a `ring-*`/box-shadow, which WKWebView drops on native-appearance controls)  
**Blocks:** UX-PB.3e, UX-PB.3g; Story 6.5

As a Pack-Manager user, I want the plan to become Results only after affected state is verified so that success is earned, not assumed from a process exit.

**Acceptance Criteria:**

**Given** an active attempt whose mutations have all reached a process-terminal state
**When** the required refresh verification for the affected Managers completes
**Then** the attempt becomes terminal, the sidecar transforms in place into a persistent Results Summary that remains until `Done`, and it **displays** one atomic outcome summary (e.g. `12 of 12 updates verified` or `10 of 12 verified · 2 failed`). Announcing that summary is optional rather than required (AD-17, D37); the summary must be **visible**.

**Given** a completed attempt
**When** Results renders
**Then** the overall outcome is exactly one of success, partial, failed, cancelled, timed out, or interrupted, and each item is verified, failed, cancelled, or skipped — mutation failure and verification failure are distinguished, `Skipped` marks only work that never started, and crash-reconstructed unfinished work reads as `Interrupted`.

**Given** an Operation whose process exited successfully (verification-refresh failure/timeout)
**When** the required refresh verification itself errors or times out, distinct from a mutation failure
**Then** the item does not declare success — it stays `Verifying` until it resolves, then reports verification failure with its evidence, and is never colored successful on the strength of the exit code alone.
**And** the Manager's Last-good Snapshot is left in place with its timestamp (AD-25) — a verification refresh that errors or times out never replaces or clears the snapshot it failed to refresh, so the surface keeps showing the last state it actually knows rather than blanking.

**Given** an attempt reaching terminal state (Results persistence failure)
**When** the single durable terminal write owned by **UX-PB.2e** fails
**Then** the failure to persist is surfaced honestly, the visible Results are not presented as durably recorded, and no fabricated success is shown. This story **renders** Results; it never writes a durable record itself, and it is not obliged to announce them (AD-17, D37). An attempt accumulates **exactly two** append-only records — UX-PB.2c writes the admission record at mint, and the terminal record fires on the terminal transition inside the plan-attempt store, owned by UX-PB.2e — never one per transition, because `operations.jsonl` already carries per-step detail under the same `planAttemptId` and a per-transition attempt record is the duplication AD-18 exists to prevent
**And** **the "no rule for which record is authoritative" gap this story previously recorded is now answered by AD-29's fold rule** rather than left open: an attempt is a fold over its records resolved in one direction only — admission plus terminal yields the terminal record's outcome, admission with no terminal record yields `Interrupted` once no live attempt owns it, and a second terminal record for one attempt is duplicate evidence and **never** a state change, because a terminal state is durable and an attempt cannot leave one. The fold is idempotent and keyed by `planAttemptId`
**And** per-Operation `Verifying` and `Skipped` remain durable states in the **Operation** journal (AD-16); "durably journaled" at the Operation level and "not per transition" at the attempt level are two different journals and are consistent, not contradictory.

### Story UX-PB.3e: Failure guidance and safe next step before Retry

**Primary concern:** Product Behavior  
**Dependencies:** UX-PB.3d; D30; AD-16; AD-27 (focus is a 2px `outline` in `--color-focus-ring`; never a `ring-*`/box-shadow, which WKWebView drops on native-appearance controls)  
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
**Dependencies:** UX-PB.3c; D30; AD-16 (interaction-required policy); AD-27 (focus is a 2px `outline` in `--color-focus-ring`; never a `ring-*`/box-shadow, which WKWebView drops on native-appearance controls)  
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
**Dependencies:** UX-PB.3d, UX-PB.2e; D30; AD-16 (attempt-scoped cancellation; `Skipped` marks only never-started work; **there is no `Cancelling` state**); AD-29 (this story can also drive an attempt terminal, so the terminal record and the fold bind it — the append still belongs to the store, in the critical section that makes the transition, and this story never writes a second one); AD-27 (focus is a 2px `outline` in `--color-focus-ring`; never a `ring-*`/box-shadow, which WKWebView drops on native-appearance controls)  
**Blocks:** UX-PB.4a-4e  

As a Pack-Manager user, I want the primary cancel action to clearly stop the whole plan, with an Operation-only cancel reserved for a deliberate diagnostic, so that I always know the scope of what I am stopping.

**Acceptance Criteria:**

**Given** an active confirmed attempt
**When** I choose the primary cancellation labeled `Cancel plan`
**Then** it requires no second confirmation, moves still-running Operations bound to that `planAttemptId` **straight to their terminal state** through the existing process-group escalation, prevents unstarted attempt work from beginning and marks it `Skipped`, promises no rollback, and never delays cancellation behind a dialog
**And** **no `Cancelling` state is introduced** — the 5-second SIGTERM grace window is never surfaced as its own status, at any level (`prd.md` FR-13, AD-16). This is the same atomic AD-3 wire concern UX-PB.2e carries, and neither story adds the variant.

**Given** a deliberately Operation-scoped diagnostic action
**When** an Operation-level cancel is offered
**Then** it is the only place labeled `Cancel operation`, while generic `Cancel` is reserved for closing a dialog or retry-scope editor without mutating running work.

**Given** an attempt in the verifying window with processes exited and refresh verification pending (cancellation while verifying)
**When** `Cancel plan` is issued
**Then** cancellation is honored immediately for that `planAttemptId`, verifying items resolve to honest terminal outcomes (cancelled or skipped rather than falsely verified), and no item is reported successful because its exit preceded the cancel.

### Story UX-PB.4a: One immutable History row per confirmed attempt

**Primary concern:** Product Behavior  
**Dependencies:** D29; AD-16 (durable `planAttemptId` identity; atomic all-or-none admission); AD-18; **AD-29** (this story is a **reader and folder only** and appends nothing — its "one immutable History row" is the fold's output, not a write; the terminal record is UX-PB.2e's, because this story depends on all of UX-PB.3a–3g while UX-PB.2e ships in wave 2, and giving the terminal append to this story would leave every attempt terminating in between persisted admission-only and therefore read as `Interrupted`); UX-PB.3 complete (PB.3a-g); AD-27 (focus is a 2px `outline` in `--color-focus-ring`; never a `ring-*`/box-shadow, which WKWebView drops on native-appearance controls)  
**Blocks:** UX-PB.4b, UX-PB.4e  

As a Pack-Manager user, I want each plan I confirm to become exactly one immutable History entry so that every attempt has one durable record instead of scattered per-command rows.

**Acceptance Criteria:**

**Given** a confirmed plan attempt that reaches a terminal state — succeeded, failed, cancelled, interrupted, or partially skipped, and regardless of how many Managers, commands, Packages, failures, or skips it contained
**When** it terminates
**Then** exactly one immutable History row is created for that `planAttemptId`, its Operation-level evidence is nested inside that row, and its summary uses verified-outcome wording such as `10 of 12 verified · 2 failed` rather than a generic completion ratio
**And** no attempt ever yields more than one row or a per-Package or per-command row — **true by construction under AD-29 rather than by this story remembering to deduplicate**: the fold is idempotent and keyed by `planAttemptId`, and two records of the same kind for one attempt fold to one attempt. This story must **not** inherit the shipping Operation loader's behavior, which is not idempotent — every start-shaped line pushes a new entry and only the finish half dedups, so a duplicate admission line would replay as an extra row.

**Given** a confirmed attempt has terminated
**When** its single immutable History row cannot be persisted
**Then** the write failure is surfaced honestly, no partial or fabricated row is presented as a complete History entry, and the durable Operation and crash-journal evidence for that `planAttemptId` remains recoverable rather than silently lost.

**Given** a confirmed attempt was admitted but the app crashed or relaunched before the attempt reached a terminal row
**When** History reconciles on the next launch
**Then** the in-flight attempt is reconciled from its durable `planAttemptId` records into one honest row, an attempt that never reached terminal is shown as interrupted, and no completed outcome is fabricated for work that did not finish.

**Given** an attempt whose admission record is present but whose terminal record is absent, unparseable, or skipped by the read (AD-19)
**When** History folds that `planAttemptId`'s records into its row
**Then** the row is presented as `Interrupted` **only when the absence is genuine** — a terminal record that exists but failed to parse is reported as unreadable evidence rather than silently reclassifying a finished attempt as unfinished, and the fold states which it was.
**And** the direction holds both ways: a missing terminal record never fabricates a completed outcome, and an unreadable one never erases a completed attempt.

### Story UX-PB.4b: Read-only Activity replay of a History row

**Primary concern:** Product Behavior  
**Dependencies:** D29-D30; AD-16; AD-18 (the attempt journal is where a replay's evidence is read from, under the same retention policy as `operations.jsonl` — a record that loses its counterpart reads as legacy, never as corrupt); AD-29 (the replay is a **reader**; it appends nothing, and it reconstructs the attempt by folding its two records rather than by trusting any single one); AD-24 (Retry derives its own intent; revealing the scope executes nothing); UX-PB.4a; AD-27 (focus is a 2px `outline` in `--color-focus-ring`; never a `ring-*`/box-shadow, which WKWebView drops on native-appearance controls)  
**Blocks:** UX-PB.4c, UX-PB.4d  

As a Pack-Manager user, I want opening a History row to route Activity into read-only replay so that I can inspect exactly what a prior attempt did instead of piecing together unrelated commands.

**Acceptance Criteria:**

**Given** a completed History row for a confirmed `planAttemptId`
**When** I open it
**Then** Activity enters a clearly labeled read-only replay that reconstructs the attempt's Manager groups, Package/version changes, Manager self-updates, exact commands, Operation outcomes, errors, timings, and retained output
**And** no control in the replay can mutate, re-run, or execute anything, with exactly one carve-out: the non-executing `Retry` affordance UX-PB.4d offers from a History entry. Invoking it reveals the failed-item scope inline inside the replay and executes nothing; the replayed attempt and its records stay immutable, and any execution still goes only through `Create new plan`, the derived `RetryIntent`, and the ordinary preview and confirmation path.

**Given** a History row whose persisted attempt is corrupted or missing
**When** I try to open its replay
**Then** the load failure states what could not be reconstructed, the History list stays intact and navigable, and no partial reconstruction is presented as a complete or trustworthy replay.

### Story UX-PB.4c: Live and replay coexistence with the live attempt primary

**Primary concern:** Product Behavior  
**Dependencies:** D30; UX-PB.4b; AD-27 (focus is a 2px `outline` in `--color-focus-ring`; never a `ring-*`/box-shadow, which WKWebView drops on native-appearance controls)  
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
**Dependencies:** D29; AD-16 (Retry mints a new linked `planAttemptId` and preserves the original failure); AD-24 (derived `RetryIntent`; the persistent draft has exactly one author); UX-PB.4b, UX-PB.2b; AD-27 (focus is a 2px `outline` in `--color-focus-ring`; never a `ring-*`/box-shadow, which WKWebView drops on native-appearance controls)  
**Blocks:** Story 6.5  

As a Pack-Manager user, I want Retry to first show the failed-item scope and then create a new linked attempt so that I can re-run only what failed while the original result stays untouched.

**Acceptance Criteria:**

**Given** a terminal Results or History entry with failed items and Retry available
**When** I invoke Retry
**Then** it first reveals the proposed failed-item scope inline with `Cancel` and `Create new plan`; `Create new plan` composes a derived `RetryIntent` in Rust — the source attempt's reviewed intent restricted to its failed members, canonically rebuilt against current eligibility and argv — and takes that separate reviewable object straight to preview and confirmation without ever writing to, merging with, or emptying the one persistent draft, and confirming it creates a new attempt with a fresh `planAttemptId` linked by `retryOfPlanAttemptId` and a `Retry of plan from <time>` History entry
**And** the original failed result stays immutable and reachable through `View previous result`.

**Given** Retry has exposed the failed-item scope
**When** current canonical intent cannot be rebuilt for that scope — for example an item is now pinned, current, removed, or unavailable
**Then** the rebuild failure is explained, no new attempt is admitted, and the original immutable failed result is left unchanged and still visible.

**Given** a Retry attempt links back to its source through `retryOfPlanAttemptId`
**When** the source is missing, the link is dangling or orphaned, or the original would be mutated by the Retry
**Then** the original attempt's History row and result remain immutable and are never overwritten, the lineage is surfaced honestly including when its source cannot be resolved, and no fabricated or repaired lineage is presented as valid.

### Story UX-PB.4e: Legacy Operation History honest labeling

**Primary concern:** Product Behavior  
**Dependencies:** D29; AD-16 (legacy honesty — no inferred plan grouping); AD-18; AD-29 (an attempt record that loses its counterpart is legacy, and `Interrupted` requires a **genuine** absence — a terminal record present but unreadable is reported as unreadable evidence, never silently reclassified); **AD-30** (a quit that would orphan a live child process is guarded at **one** enforcement point, and its active set is `Queued` ∪ `Running` — an attempt cut short by an *unguarded* quit is not the genuine absence AD-29 requires, so `Interrupted` labeling here presumes Story 6.6's guard); UX-PB.4a, UX-PB.2f; AD-27 (focus is a 2px `outline` in `--color-focus-ring`; never a `ring-*`/box-shadow, which WKWebView drops on native-appearance controls)  

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
**Dependencies:** UX-PB.1 and UX-PB.2 complete; D27, D28; AD-16; finalized UX spines; AD-27 (focus is a 2px `outline` in `--color-focus-ring`; never a `ring-*`/box-shadow, which WKWebView drops on native-appearance controls)  
**Blocks:** UX-PB.5b, UX-PB.5d  

**Restated per `docs/DECISIONS.md` D37 — owner decision, 2026-07-25.** This story was **not** in D37's named list of sections, and D37's rule is "scope by named section, never by a mention count". The owner extended the list to include this story rather than making an exception to the rule, on the ground that its dismissal criterion carried focus-restoration language of exactly the class `prd.md` NFR-6 drops. Removed as criteria: "focus moves to the dialog heading/command summary with `Change Plan` as the first actionable control so a final confirmation is never the accidental default for an unfocused Enter" (the failure it guards requires a keyboard), the *focus* half of `Change Plan`'s return target, and "restore focus to the originating `Confirm N Updates` action".

**The criterion was cut surgically rather than stripped wholesale, and the reason is the same trap Story UX-PB.1d carried:** one sentence bundled three separate things, and only two of them were keyboard. `Change Plan`'s **return target survives as navigation** — where the dialog closes back to, which a mouse user experiences as scroll position. **`Escape`/backdrop "dismiss only while no command has begun" survives untouched** — it is a dismissal-safety rule about not tearing the dialog down mid-execution, not focus restoration. Pattern-stripping the sentence would have deleted a safety property along with the keyboard limbs. The dimmed, focus-trapped background in the criterion above is modal behavior and is likewise unaffected.

As a Pack-Manager user, I want the persistent Upgrade Plan to present one deliberate final confirmation before anything runs so that a review step always stands between staging and execution and nothing bypasses it silently.

**Acceptance Criteria:**

**Given** a non-empty Upgrade Plan with confirmation enabled (`skipUpgradePlanConfirmation` is `false`)
**When** the plan footer renders
**Then** it contains exactly one blue `Confirm N Updates` action where N is the count of staged updates, exact commands stay hidden behind `Show update command`, and no safety or skip checkbox appears on the base plan.

**Given** the enabled base plan
**When** I invoke `Confirm N Updates`
**Then** the `Proceed with Upgrade Plan?` Confirmation Dialog opens over a dimmed, focus-trapped background, shows the exact commands that will run, and offers `Change Plan` plus a final `Confirm N Updates`, and nothing executes until the final confirmation is chosen.

**Given** the open Confirmation Dialog
**When** I use `Change Plan`, Escape, or the backdrop
**Then** `Change Plan` returns to the first editable plan item or the plan heading — a **navigation** target rather than a focus target, which a mouse user experiences as scroll position and as what the dialog closes back to (`EXPERIENCE.md`'s Confirmation Dialog contract)
**And** Escape or the backdrop dismiss **only while no command has begun** — a dismissal-safety rule about not tearing down the dialog mid-execution, which is **not** focus restoration and is **not** in D37's scope.

**Given** the open dialog
**When** I choose the final `Confirm N Updates`
**Then** the full plan is admitted atomically through the same review, execution, verification, Results, and History lifecycle as any plan, partial silent admission never occurs, and only one confirmed attempt becomes active.

**Given** a confirmed admission
**When** admission fails
**Then** nothing executes, the dialog explains why, and the plan remains editable for re-review.

### Story UX-PB.5b: Dialog-only disable control with atomic `skipUpgradePlanConfirmation` persistence and Settings migration

**Primary concern:** Product Behavior  
**Dependencies:** UX-PB.5a; D28; FR-17; AD-19; AD-21 (`skipUpgradePlanConfirmation` is declared plan-inert); AD-22 (admit, then persist the rider); Settings migration; AD-27 (focus is a 2px `outline` in `--color-focus-ring`; never a `ring-*`/box-shadow, which WKWebView drops on native-appearance controls)  
**Blocks:** UX-PB.5c; Story 3.4  

As a Pack-Manager user, I want to deliberately disable the final confirmation from the dialog and restore it from Settings so that I can remove friction without ever losing a safe default.

**Acceptance Criteria:**

**Given** the `Proceed with Upgrade Plan?` dialog
**When** it renders
**Then** only this dialog contains the `Disable upgrade plan command execution confirmation` control, its safety explanation, and Settings-restoration guidance, and the base plan never surfaces that control.

**Given** the dialog with `Disable upgrade plan command execution confirmation` selected
**When** I choose the final `Confirm N Updates` and admission succeeds
**Then** the ordering is validate, admit through the scheduler's revision-checked transaction, then persist the rider once admission has returned — `skipUpgradePlanConfirmation: true` is written atomically only after the plan is admitted, and it becomes active only after that write succeeds
**And** the opt-out never precedes the admission it rides on; if that atomic save then fails, the admitted attempt stands, the prior `false` preference is retained as both active and persisted state, and the failure is surfaced inline.
**And** `skipUpgradePlanConfirmation` is plan-inert (AD-21) — it is not a plan-determining input, so writing it never advances the canonical revision and never invalidates the preview it rides on. Without that, this story's own save would expire the plan it just admitted and the safety opt-out would deterministically fail its own run.

**Given** the dialog with `Disable upgrade plan command execution confirmation` selected
**When** I choose the final `Confirm N Updates` and admission is rejected
**Then** nothing is persisted and nothing becomes active — the confirmation gate stays armed for a run I never got — and the dialog retains my selection so the choice is not silently lost.

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
**Dependencies:** UX-PB.5b; D27, D28; AD-16; AD-27 (focus is a 2px `outline` in `--color-focus-ring`; never a `ring-*`/box-shadow, which WKWebView drops on native-appearance controls)  
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

### Story UX-PB.5d: Responsiveness of the confirmation and safety surfaces at the size and zoom floors

**Primary concern:** Product Behavior  
**Dependencies:** UX-PB.5a; finalized high-zoom contracts; **NFR-3** (the size and zoom floors this story now rests on); AD-17 (below 720 usable CSS pixels the sidecar region becomes a full-workspace or stacked surface, one surface visible at a time, with a persistent non-occludable indicator for safety-critical attempt state); AD-27 (focus is a 2px `outline` in `--color-focus-ring`; never a `ring-*`/box-shadow, which WKWebView drops on native-appearance controls)  

**Restated per `docs/DECISIONS.md` D37, and deliberately not deleted.** This story was built almost entirely on D37-removed scope — a "keyboard and VoiceOver user at high zoom" persona, a keyboard/VoiceOver `When`, a focus-order-and-announcements `Then`, and a deterministic focus-restoration criterion that `prd.md` NFR-6 names among the dropped obligations. **Its zoom and minimum-window half survives under NFR-3** and is what remains below; the accessibility half is removed as a criterion, not as shipped code. `prd.md` §10 records the owner's 2026-07-25 confirmation that NFR-6's "deterministic dialog/sidecar focus restoration" stays dropped rather than being carved out as an exception. The story's former dependency on FR-19 is repointed to NFR-3, because the limb it actually needs is the responsiveness floor rather than the interface FR.

As a Pack-Manager user at the smallest supported window and at high zoom, I want every safety action to stay fully visible and operable so that the confirmation gate protects me at the 900 x 600 minimum and at 150-200% zoom.

**Acceptance Criteria:**

**Given** the plan and the `Proceed with Upgrade Plan?` dialog with reduced motion active
**When** they render
**Then** transitions and animations are suppressed, and every safety action (`Confirm N Updates`, `Change Plan`, the disable checkbox, and `Run N Updates`) carries a visible focus indicator drawn as a real `outline` per AD-27 — a rendering mechanism D37 keeps by name, verified for each control at runtime in a macOS Tauri build rather than inferred from a green CI run.

**Given** the 900 x 600 minimum window at 100%, 150%, and 200% zoom
**When** the Plan, Confirmation, Activity, and Results surfaces render
**Then** below 720 usable CSS pixels the layout enters high-zoom mode, navigation collapses to a rail or temporary panel, and Plan/Confirmation/Activity/Results present as a full-workspace or stacked surface with a visible Back route, no overlapping panes, and no two-dimensional scrolling for the primary task, keeping every safety action reachable.

**Given** 150% zoom, 200% zoom, or the 900 x 600 minimum
**When** a safety action would otherwise clip or overflow
**Then** it remains fully visible and operable with its name, state, versions, primary action, and error/recovery preserved, and no safety action becomes unreachable behind an overlapping or two-dimensionally scrolling pane.

**Given** a live attempt whose owning surface is stacked behind another at that width
**When** a safety-critical attempt state occurs — the stall handoff or `Interaction required`
**Then** it reaches the user through a persistent, non-occludable indicator that routes to the surface owning it (AD-17), and never depends on an announcement channel, because the announcement channel that used to carry it is now optional.

### Story UX-PB.5e: Application-update presentation kept separate from Package plans and History

**Primary concern:** Product Behavior  
**Dependencies:** UX-PB.4 complete (History must exist to assert separation); finalized application-update presentation; AD-27 (focus is a 2px `outline` in `--color-focus-ring`; never a `ring-*`/box-shadow, which WKWebView drops on native-appearance controls)  

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
- Governing invariants: AD-4, AD-25
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

**Given** a Manager that has already produced a successful snapshot, and a later refresh whose detection, parse, network, timeout, or persistence path fails
**When** the failure resolves and recovered-parse output is available
**Then** the failure stays contained to that Manager, its Last-good Snapshot is retained and labeled with its own timestamp and the exact failure alongside a `Retry refresh` affordance, and the recovered output **merges** into the inventory already parsed from the successful refresh outputs
**And** the snapshot is never replaced by an empty one and never by an outdated-only overlay — which would make every up-to-date Package vanish — the merge never un-pins a row, and health and staleness presentation read from the snapshot's real timestamp with no invented or interpolated value substituted.

### Story 2.5: Offer Copyable Install Guidance for Absent Managers

Added 2026-08-18 by `sprint-change-proposal-2026-08-18.md`, implementing `docs/DECISIONS.md` **D40**. Like Story 6.6, this postdates the D33 rescope: it is new scope decided by the owner, not a resurrected triage story.

As a Pack-Manager user,
I want every absent Manager to show a copyable install command, and an all-absent machine to tell me where to start,
So that a Manager I lack — or a machine with none at all — hands me the terminal command instead of a dead end.

**Story Contract:**

- FR and requirement links: FR-1 (the install-hint limb, extended by D40 from "where one is known" to all six)
- Required test level: Unit plus component
- Governing invariants: AD-4 (hints are static data through existing typed surfaces; no new process effect), AD-27 (focus is a 2px `outline` in `--color-focus-ring` on any added control; never a `ring-*`/box-shadow, which WKWebView drops on native-appearance controls)
- Governing decisions: **D40** (copyable hints, never an executing Install button — installer non-goal, SM-3 no-privilege, no shell surface, and FR-23's closed immediate-execution set may not grow); D14 (copy-to-terminal is the product's handoff ethos)
- Dependencies: none — buildable now; no Epic UX-PB surface is involved. The existing `installHint` render paths (`Sidebar.tsx`, `ManagerCard.tsx`, `ManagerPane.tsx`) are reused, not rebuilt.

**Acceptance Criteria:**

**Given** any of the six Managers is detected absent
**When** its Dashboard card, sidebar entry, and Manager workspace absent state render
**Then** each shows that Manager's copyable install command through the existing `CopyableCommand` treatment (extending the mas behavior to all six), the command is copy-only, nothing in the app can execute it, and no `Install` button or other execution affordance exists (D40)
**And** the absent presentation still explains that Refresh All / Re-detect picks the Manager up after installation.

**Given** the indicative commands recorded in D40 (brew → the official Homebrew installer one-liner; mise → `brew install mise`; npm → `mise use -g node@lts`; uv → `mise use -g uv`; rustup → the official rustup installer one-liner; mas → `brew install mas`)
**When** the hints are implemented
**Then** each shipped hint is verified against that Manager's current official installation documentation, and hints are static per-Manager strings — no context-aware suppression or rewriting based on which other Managers are present, per D40's rejected-alternatives record.

**Given** a machine where all six Managers are absent
**When** detection completes
**Then** the Dashboard presents a state panel carrying D40's guidance — no package managers were found, the user installs one themselves (Homebrew is the usual first), and `Refresh All` re-detects afterward — the system never reads as an error or `Warning` for absence alone, and `Update Everything` stays disabled with a reason.


## Epic 3: Keep Package Choice, Plans, and Settings Exact and Understandable

Users can understand Package state, select only eligible work, review exact commands and exclusions, reject stale plans, perform bounded row-level updates, and control Settings without misleading or inaccessible UI state.

### Story 3.1: Present Complete Package State and Manager Detail

As a Pack-Manager user,
I want complete Package rows, expandable details, self-update separation, and non-color status cues,
So that I can understand what each Manager reports without losing Manager-specific meaning.

**Story Contract:**

- FR and requirement links: FR-2; FR-5; FR-6; FR-10; FR-11; FR-19
- Required test level: Component
- Governing invariants: AD-16, AD-17, AD-28 (the row Checkbox **is** membership — no transient selection, and the row reads the same single eligibility-and-visibility predicate as the header Checkbox, `⌘A`, the batch payload, and the tri-state denominator, projected from Rust with the snapshot it was computed against), AD-27 (focus is a 2px `outline` in `--color-focus-ring`; never a `ring-*`/box-shadow, which WKWebView drops on native-appearance controls)
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
- Governing invariants: AD-16 (ineligible-item inertness: **inert, not inactive** — the control may not use the native `disabled` state), AD-17, AD-28 (membership, the single predicate, and the closed removal taxonomy), AD-27 (focus is a 2px `outline` in `--color-focus-ring`; never a `ring-*`/box-shadow, which WKWebView drops on native-appearance controls)
- Governing decisions: **D38** — D15's disabled-checkbox *mechanism* is superseded; D15's substance (a pinned formula is never upgradable in-app and is excluded from every plan) is unchanged and not reopened
- Dependencies: Story 3.1; **Story UX-PB.1d** (which declares `Blocks: Story 3.2` and owns the inertness contract this story renders — the edge previously existed in one direction only, so nothing sequenced the contract); deterministic plan-builder and UI fixtures

**Acceptance Criteria:**

**Given** pinned Homebrew formulae
**When** membership mutation, row plan-add, per-Manager update-all, and Update Everything draft-entry paths are exercised across every active filter
**Then** pinned rows stay inert, add nothing to the draft Upgrade Plan, and are explained and excluded from every plan with the correct reason
**And** the control uses `aria-disabled="true"` and **not** the native `disabled` state — this story renders the same control as UX-PB.1d and takes the same correction. A natively disabled form control dispatches no mouse events, so `disabled` and the pointer-reachable reason are mutually exclusive and the reason wins (AD-16, D38). Reduced opacity alone is not the treatment either.

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
- Governing invariants: AD-4, AD-5, AD-19, **AD-27** (this story renders Settings controls, and AD-27 revision 10 widened its `Binds` to name Story 3.4 explicitly — an earlier enumeration omitted it and this document faithfully mirrored the omission. Focus is a 2px `outline` in `--color-focus-ring`; never a `ring-*`/box-shadow, which WKWebView drops on native-appearance controls, and never `outline-none`. Verify each added control's focus state at runtime — no grep and no green suite substitutes for that)
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

### Story 3.5: Preserve Exact Batched Plan Membership and Row Plan Actions

As a Pack-Manager user,
I want membership interactions and single-row plan actions to preserve exact Package identity,
So that I can act efficiently without adding excluded or unrelated Packages to the Upgrade Plan.

**Restated per `ARCHITECTURE-SPINE.md` AD-28.** This story's criteria were written on the transient-selection model AD-28 abolishes — "the exact selectable identities", a `Clear` that cleared a selection set, and an `Esc` rung. AD-28 exists precisely to prevent this story building a transient selection set while UX-PB.1a builds direct membership: two stories that obey every other invariant and ship opposite models of the same checkbox. The criteria below are restated in membership terms and batched. `epics.md` never named `⌘U`, so no `⌘U` limb is removed here — only `docs/SPEC.md` §4.11 carries that, and it is hand-written and workflow-unowned.

**Story Contract:**

- FR and requirement links: FR-6; FR-10; FR-13; FR-19; RP-2 (`⌘A` as an Edit-menu action, and the surviving accelerator map); NFR-3 (the 101-rows requirement that makes batching a requirement rather than an optimization)
- Required test level: Component plus browser E2E
- Governing invariants: AD-16, AD-17, **AD-28** (a Package checkbox **is** membership; the anchor survives and the selection set does not; a range or filter-wide interaction is **one** membership operation; a batch carries concrete canonical identities computed from the snapshot the user is looking at, never a predicate for Rust to re-expand; one predicate, and it is Rust's), AD-27 (focus is a 2px `outline` in `--color-focus-ring`; never a `ring-*`/box-shadow, which WKWebView drops on native-appearance controls)
- Dependencies: Stories 3.1 and 3.2; **UX-PB.1a** (which retires the live `selection` set in `src/store/packages.ts`); deterministic bridge

**Acceptance Criteria:**

**Given** eligible, current, pinned, greedy, filtered, and range-addressable rows
**When** a toggle, a shift-range, the tri-state header Checkbox, `⌘A`, Space, or `⌘`-click mutates membership
**Then** each acts **directly on Upgrade Plan draft membership** — checking an eligible Package adds it and unchecking removes it, with no separate selection set to build and no `Add Selected` submit step — the exact canonical identities and visible filter semantics are preserved, and ineligible rows never enter membership under any of them, including the header Checkbox
**And** a range or filter-wide interaction submits **one** membership operation covering every affected identity — one round trip, one canonical rebuild — never one operation per row, because the canonical draft lives in Rust and every mutation round-trips before the projection updates (NFR-3, AD-28)
**And** the batch carries **concrete canonical identities computed from the snapshot the user is looking at**, plus the snapshot token it read, and Rust rejects a batch whose token is not its current snapshot — never a predicate for Rust to re-expand, which would resolve against whatever snapshot Rust holds at that instant and stage a Package the user never saw
**And** a range is an anchor and a target over the **ordered filtered set the projection holds**, including off-screen virtualized rows, and **not** the rendered DOM window — the product virtualizes, so at 500 rows roughly twenty are rendered and a 400-row range would otherwise mean two different things to this story and the story that owns the table
**And** the frontend may hold the shift-range anchor, the search term, and the filter — *where the user is* — and never *what is staged*: the **anchor survives, the selection set does not**, and losing the anchor degrades a range into a single toggle, which is already the shipping fallback
**And** the header Checkbox's tri-state denominator is the eligible set matching the active filter including off-screen rows — unchecked when none of that set is staged, `mixed` when some, checked when all — derived from the membership projection and never stored
**And** the batch is **all-or-none in application and narrowing in resolution**: one rebuild applies every member it still finds eligible and drops the rest as one transaction, reporting the dropped refs and their reasons; it never half-applies silently and is rejected outright only when the rebuild **errors**, in which case the prior coherent draft and its last authenticated preview are preserved unchanged (AD-16)
**And** provenance follows the shape of the act (AD-23, AD-28): a **range is `Explicit`** for every member because it names concrete visible rows, while the header Checkbox and `⌘A` are `Bulk { scope: FilteredView }`
**And** **`Clear` survives with a changed meaning and `Esc` does not.** `Clear` is no longer a selection clear — it is a **scope-wide membership removal** under AD-28's closed three-way taxonomy, clearing membership for the refs in its scope and clearing the tombstones **only** of the refs whose membership it actually cleared, never of refs that held none. **`Esc` never touches membership**: its clear-selection rung is deleted rather than re-pointed, and `Esc` is not handed the sidecar as a replacement sink, so the cascade is **close-dialog alone** (AD-17, `prd.md` FR-6). One `Esc` mass-writing tombstones and poisoning a draft against `Update Everything` is the failure this closes.

**Given** the Dashboard, History, and Settings — the views with no Package list
**When** `⌘A` is pressed
**Then** the native select-all is **not** suppressed and behaves normally, because an accelerator that shadows a standard Edit-menu action suppresses the native default **only on surfaces where it performs its own action** (AD-28)
**And** this is a named shipping defect with an owner rather than a hypothetical: the handler calls `preventDefault()` before the select-all helper early-returns on views with no Package list, so today `⌘A` blocks native select-all on those three views and puts nothing in its place. The search field escapes it only because the handler bails on editable targets first
**And** `docs/DECISIONS.md` D37 does not excuse it — `⌘A` is an Edit-menu action D37 keeps by name, and RP-2 makes it a release prerequisite. It is a functional regression in select-all, not an accessibility item
**And** re-pointing `⌘A` to membership must not inherit the defect (`prd.md` FR-6).

**Given** one eligible or ineligible Package row
**When** the single-row plan action is invoked
**Then** exactly one eligible Package's canonical identity is added to (or removed from) the persistent draft Upgrade Plan, nothing is built, submitted, enqueued, or executed, and the sidecar reflects the membership change
**And** ineligible, pinned, or current rows add nothing, stay inert with an explained reason, and never expand membership
**And** a single-row removal is a **single-ref removal** under AD-28's taxonomy and writes a tombstone (AD-23), so no later bulk expansion of any scope re-adds it
**And** the resulting one-Package plan enters the same review and separate-confirmation path as a multi-Package plan, with its execution, verification, Results, and History lifecycle proven by the later-wave stories that own those stages.

## Epic 6: Preserve State, Evidence, and Privacy Across Failure and Relaunch

Users can reconstruct Operations after crashes, trust Settings and durable stores across failure, reveal native evidence safely, and export exact diagnostics without inherited-environment disclosure or hostile-path traversal.

### Story 6.5: Export Exact Native Diagnostics and Visible Outcomes

As a Pack-Manager user,
I want diagnostics export to create the documented archive and report native outcomes,
So that support evidence is complete, inspectable, and actionable.

**Story Contract:**

- FR and requirement links: FR-18
- Required test level: Real native Tauri E2E plus artifact inspection. Satisfiable as written — **AD-26** names a compliant shape and no renegotiation is needed.
- Governing invariants: AD-3, AD-4, AD-5, AD-16, AD-18, AD-26, **AD-29** (the archive carries the **raw journal lines**, never a synthesized record — this story's assertion that the exported plan-attempt records carry scope, exact commands, verification facts and results is satisfied by the record **set** for a `planAttemptId` and cannot be satisfied by any single record, because AD-29's admission/terminal split makes such a record impossible. A folded attempt view may be added as an **additional** entry marked as derived; it never replaces the raw lines and is never written back to the journal), **AD-27** (this story renders the diagnostics action, and AD-27 revision 10 widened its `Binds` to name Story 6.5 explicitly — an earlier enumeration omitted it and this document faithfully mirrored the omission. Focus is a 2px `outline` in `--color-focus-ring`; never a `ring-*`/box-shadow, and never `outline-none`; verify the added control at runtime), **AD-30** (a quit that would orphan a live child process is guarded at **one** enforcement point, and its active set is `Queued` ∪ `Running` — the interrupted-run evidence this export carries is evidence of a run whose quit reached that guard; Story 6.6 builds it, and this story neither builds nor duplicates it)
- Harness constraint (AD-26): the native automation surface is excluded from release bits at **compile time**, never by a runtime selector, and the harness must drive the **production composition** — the same registered commands and events, the same handlers, the same serialization. No delivery coverage may be claimed from a fixture, from the browser double, or from a harness that introduces a test-only command, a second composition root, or a different registration set.
- Dependencies: disposable logs/transcripts/journal

**Acceptance Criteria:**

**Given** documented default destination, alternate permission outcomes, and invocation from Settings and History
**When** diagnostics export runs through the production native command
**Then** the timestamped ZIP path and visible success/failure match the contract.

**Given** more than three app logs, 25 transcripts, 1,000 journal records, and durable plan-attempt records correlated by `planAttemptId`
**When** the produced ZIP is opened and inspected
**Then** it contains `report.json`, the newest three app logs, newest 25 transcripts, `operations.jsonl`, and the durable plan-attempt records that correlate the exported evidence — with exact expected contents and no missing required entry, including those plan-attempt entries
**And** the required fields are carried **between** an attempt's two records rather than duplicated into each (AD-29): the **admission** record carries `planAttemptId`, reviewed Manager/Package scope, exact commands, identities and timestamps, and the **terminal** record carries verification facts and results; `retryOfPlanAttemptId` accompanies the attempt where one exists. The assertion is satisfied by the record **set** for a `planAttemptId` and may not be written as a single-record assertion
**And** the archive carries the raw journal lines, so widening the export does not widen disclosure — plan-attempt records enter under the same allowlist the export already applies, inherited environment values stay excluded, and a record carries the reviewed intent and the exact argv Pack-Manager constructed and never ambient environment or user paths (AD-18).

**Given** Export diagnostics and Open Logs actions
**When** native command/opener success and failure are controlled
**Then** the UI exposes actionable outcomes

### Story 6.6: Guard a Quit That Would Orphan a Live Child Process

As a Pack-Manager user,
I want a quit that would abandon running or queued work to ask me first,
So that work is never silently discarded and the app never leaves child processes behind.

**Story Contract:**

- FR and requirement links: FR-14 (the quit-guard limb, revived by D30 and AD-30 after the D33 rescope triaged it out); FR-21 (its shipping `Queued` ∪ `Running` refusal predicate is the source of truth AD-30 takes, and AD-30 binds FR-21 in return — the two active sets may not drift apart)
- Required test level: Real native Tauri E2E for the window-close and `⌘Q` paths, since the enforcement point is a native window/menu event and no browser double exercises it (**AD-26** governs the harness). The OS-shutdown limb is verified at the unit level against the kill hook.
- Governing invariants: **AD-30** (the whole story), AD-3, AD-4, AD-5
- Dependencies: none — this story guards the **shipping** operation queue and needs nothing from Epic UX-PB. It does not use `planAttemptId`, the persistent draft, or any D27–D30 surface, and it may be built at any point.

**Acceptance Criteria:**

**Given** at least one Operation is `Queued` or `Running`
**When** a **user-initiated** quit is requested — an OS window-close request or `⌘Q`
**Then** both paths resolve to the **same** enforcement point the application-update path already uses — one predicate, one dialog, one refusal — and the user is presented an explicit choice rather than having the work silently discarded
**And** the guard's active set is `Queued` ∪ `Running`: **queued counts as running**, because admission has already committed to the work and a quit would drop it unstarted
**And** that active set stays **identical** to FR-21's application-update refusal — a change to either predicate is an AD-30 change, not a local one
**And** no rollback is promised: partially completed Manager work stays partially completed, and the guard surfaces the choice rather than offering to undo.

**Given** the quit guard is presented and the user chooses to quit anyway
**When** the app exits
**Then** every child process is terminated before exit — children never outlive the app.

**Given** at least one Operation is `Queued` or `Running`
**When** the quit is **OS-initiated** — a system shutdown or logout
**Then** **no dialog is presented**; the guard is best-effort only and runs the existing kill hook — cancel every running Operation, then **await the bounded idle wait**, because `cancel_all` only flips the cancellation tokens and the runner tasks perform the `SIGTERM` → grace → `SIGKILL` work, so a shutdown path that does not await it exits before the children die
**And** the logout is not blocked to argue with the user: losing the run is the accepted cost, and this asymmetry with the user-initiated case is deliberate rather than an oversight.

**Given** a second code path that could decide on its own whether to quit
**When** it is introduced
**Then** it routes to the same enforcement point rather than deciding for itself — a second deciding path is the defect **AD-30** names, and it is how the current build ended up with a `QuitGuardDialog` its host renders and nothing but the application-update path calls.

