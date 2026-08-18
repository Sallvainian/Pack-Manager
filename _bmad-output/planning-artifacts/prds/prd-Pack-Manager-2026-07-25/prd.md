---
title: Pack-Manager
status: final
created: 2026-07-25
updated: 2026-08-18
---

# PRD: Pack-Manager

## 0. Document Purpose

This is the Phase 2 requirements artifact for Pack-Manager. It exists because `docs/DECISIONS.md` D33 retired the previous PRD on 2026-07-24 and archived it, leaving requirements authority split between a hand-written file no workflow owns (`docs/SPEC.md`) and a Phase 3 artifact that had absorbed the FR/NFR block (`_bmad-output/planning-artifacts/epics.md`, lines 53–450). Solutioning had no separable requirements input. This document restores one.

**Audience:** the maintainer, and the BMAD workflows that consume Phase 2 output — `bmad-architecture`, `bmad-ux`, and `bmad-create-epics-and-stories`.

**Authority.** This PRD is the requirements authority. `ARCHITECTURE-SPINE.md` and `epics.md` are reconciled *against* it, not the reverse. Where this document and an older artifact disagree, this document wins, with one exception: `docs/DECISIONS.md` remains the decision record, and a decision later than 2026-07-25 supersedes anything here.

**Structure.** Vocabulary is Glossary-anchored (§3) and used verbatim throughout. Features are grouped (§4) with globally numbered FRs nested; cross-cutting NFRs live in §5. Requirement IDs are **preserved from the prior artifacts** — FR-1…FR-22, RP-1, RP-2, NFR-1…NFR-8 — because `epics.md` and `ARCHITECTURE-SPINE.md` already cite them, and renumbering would break every downstream reference.

**Status tags.** Pack-Manager ships at 1.0.1 today, but decisions D27–D30 redesigned the upgrade experience and are decided-but-unimplemented. Every FR therefore states the requirement normatively and carries one of:

- **Shipping** — implemented and verifiable in the current build.
- **Partial** — some limbs ship; the rest is named inline.
- **Planned — D27–D30** — decided, not implemented. `planAttemptId`, `plan_attempt_id`, `Verifying`, `InteractionRequired`, and `skipUpgradePlanConfirmation` return **zero** occurrences across `src/` and `src-tauri/src/`; do not go looking for them.

The tags are implementation status, not requirement strength. A Planned FR is as binding as a Shipping one.

**Upstream inputs already written; not duplicated here.** UX experience contract and interaction detail: `_bmad-output/planning-artifacts/ux-designs/ux-Pack-Manager-2026-07-23/EXPERIENCE.md` (journeys AJ-1…AJ-6, referenced by ID below) and `DESIGN.md` (the approved "Aurora Control Deck" palette, adopted by D35). Architecture invariants and `AD-` ids: `ARCHITECTURE-SPINE.md` **revision 10** — the sole authority for `AD-` numbering. Revision 10 added AD-28, AD-29 and AD-30 and narrowed AD-12; each is cited at the FRs it binds under an **Architecture binding** heading.

**Not carried forward.** D33 retired the 72-criterion P0 gate, the 55 versioned scenario contracts, `readiness-coverage-map.md`, the evidence-manifest and candidate-freeze process, all coverage percentages, and the multi-host environment requirements. None of it appears here. Release readiness is `docs/RELEASE-CHECKLIST.md` plus the two release-blocking checks in `release.yml`. No archived file was copied or moved into `planning-artifacts/`; BMAD skills glob `*prd*.md` from there and would silently reload the retired apparatus.

### 0.1 Relationship to `docs/SPEC.md`

`docs/SPEC.md` was created in the repo's second commit (`f395db3`, 2026-07-22) and has been the de-facto requirements contract since. It remains valuable and remains authoritative for UI specification, architecture detail, parser contracts, and the test plan. **It is no longer the requirements authority.**

A 2026-07-25 validation, re-verified against `HEAD` for this PRD, found it materially out of date on the following. These are recorded so no future reader reconciles back to a stale source:

| SPEC location | Says | Code says |
| --- | --- | --- |
| §5.9 "IPC contract (exact)" | 17 commands | `lib.rs` `generate_handler!` registers **20** — `check_for_app_update`, `get_app_update_state`, `install_app_update` are missing from SPEC |
| §5.9 events table | 5 events | `events.rs` defines **6** — `appUpdate:status` is missing from SPEC (D25a accepted it explicitly) |
| §F11 and §5.9 `Settings` | 7 fields each, and the two lists disagree with each other | `settings.rs` `Settings` has **8**; `autoCheckForUpdates` (default `true`) appears in neither SPEC section despite shipping |
| §F11 | `skipUpgradePlanConfirmation` is a current setting | Zero occurrences in `src-tauri/src/`; it is D28 target state |
| §1 P2 (line 128) | `notarized DMG` is out of scope | Line 108 of the same file says delivery is "signed, notarized, and stapled"; D25/D25a superseded D20 and the pipeline has notarized since |
| §0.1 supersession list | Retires "immediate-row, direct self-update, Operation-row History, Activity-drawer-only, global self-update toggle, and `autoOpenDrawer`" | **F5 was never added to that list**, so SPEC's transient-selection-plus-`Add N to Plan` model reads as current when D27 superseded it. This omission is the entire reason the FR-6 conflict stayed live — see FR-6 |
| §F1–F17 | — | 9 of 17 features carry no acceptance criterion, including four P0s |

Additionally, SPEC §0 invariant 7 (line 19) states the unimplemented durable `planAttemptId` in present tense under a "violations are bugs" heading, and sits **above** §0.1, which is itself undelivered target state. The Current-vs-Target split in this PRD's status tags supersedes that reading.

---

## 1. Vision

Pack-Manager is a macOS desktop app that gives one person one trustworthy control plane over a machine whose software arrives through six different package managers — Homebrew, mise, npm, uv, rustup, and the Mac App Store CLI. It detects what is installed, shows what each Manager says is outdated, explains which Manager actually owns and will update each tool, and updates everything, a chosen subset, a single Package, or a Manager itself.

The differentiator is not that it wraps update commands in a window. It is **confidence across a mixed Manager topology**. Nothing runs that was not staged and shown. No version judgment is invented — the Manager's verdict is the only verdict. No privilege prompt exists anywhere in the product. One Manager failing never blanks the others or destroys their last-good data. Routing is derived from the machine's real layout and explained in plain language, so "why did updating uv run mise?" has a visible answer. Every operation streams live, cancels immediately, and leaves durable evidence on disk.

That combination matters because the topology is genuinely confusing and the failure modes are quiet. `npm` installed through mise lives inside mise's Node tree, so upgrading Node resets it. `uv` resolves through a mise shim that is a symlink to the mise binary, so a naive path lookup misroutes its self-update to Homebrew. Real version strings include `2.0.14-1`, `1.6.2.dev0`, `stable`, and commit hashes, on which semver math produces confidently wrong answers. A tool that guesses at any of this is worse than the six terminals it replaces.

---

## 2. Target User

### 2.1 Jobs To Be Done

- **Functional:** "Keep my Mac's developer tools current without memorizing six syntaxes or accidentally updating the wrong thing."
- **Functional:** "Tell me what's actually outdated right now, across every Manager, in one place."
- **Emotional (the load-bearing one):** "Let me approve a bulk update without the low-grade fear that I'm about to break my toolchain." Every trust requirement in §4.2 exists to serve this.
- **Functional:** "When something fails at 11pm, let me find out what ran and what happened without reconstructing it from memory."
- **Contextual:** "Work when I launch it from the Dock" — Finder-launched apps inherit `PATH=/usr/bin:/bin:/usr/sbin:/sbin`, which finds none of these Managers. This is the product's single most-likely failure mode, and it is invisible to anyone who only ever runs the app from a terminal.
- **Builder's own:** "This is my tool for my machine." Per D33 the project is a personal, open-source utility — 1 star, 0 forks, 3 lifetime `.dmg` downloads — and rigor is calibrated to that, not to a commercial launch.

### 2.2 Non-Users (v1)

- **Teams and organizations.** No multi-user, no shared state, no fleet management, no policy enforcement.
- **Non-macOS users.** macOS 15.0+ only (D31), declared in `src-tauri/tauri.conf.json` `bundle.macOS.minimumSystemVersion`.
- **People who want to install or remove software.** Pack-Manager updates what is already there. See §6.
- **Users of package managers outside the supported six.** `cargo install`, pipx, gem, and everything else are out of scope.
- **Assistive-technology users.** Per D37 this is a single-user utility operated with a mouse, and keyboard navigation and screen-reader support are explicitly not release criteria. See the reconciliation note in §4.4.

### 2.3 Key User Journeys

The journeys already exist, in narrative form with named protagonist, entry state, path, climax, and failure path, in `EXPERIENCE.md` §"Key Flows". **This PRD mirrors those IDs rather than creating a parallel set** — there are already three ID namespaces in this project (`FR-`, `AD-`, `AJ-`) and a fourth would be a liability.

| ID | Journey | Realized by |
| --- | --- | --- |
| **AJ-1** | Launch, detect, and refresh — Sallvain opens the app from the Dock and learns current system state; the window renders immediately with progressive per-Manager states rather than a blank screen, and settles into `Ready` or `Warning` with exact failed-refresh context. | FR-1, FR-2, FR-3, FR-4, FR-5 |
| **AJ-2** | Review and authorize Update Everything — eligible work populates the Upgrade Plan with exclusions and reasons; exact commands are reviewable; final confirmation atomically admits the whole plan. | FR-6, FR-7, FR-8, FR-9, FR-11 |
| **AJ-3** | Update a selected Package or Manager — the same safe path at smaller scope; no row or header action executes immediately. | FR-5, FR-6, FR-10, FR-11 |
| **AJ-4** | Handle slow, blocked, failed, cancelled, or interrupted work — stalls, Homebrew contention, cancellation, hard cap, Results, Retry, and crash reconstruction. | FR-13, FR-14, FR-15, FR-16 |
| **AJ-5** | Diagnose and export support evidence — History replay and a privacy-preserving diagnostics export. | FR-15, FR-18 |
| **AJ-6** | Install and update Pack-Manager — background check and download, explicit Restart to update, refusal while Package work is active, manual-install fallback with no privilege escalation. | FR-20, FR-21, FR-22, RP-1 |

Full narratives: `EXPERIENCE.md` lines 373–460. Where a journey beat and an FR here disagree, the FR wins and `EXPERIENCE.md` is reconciled through the queue in `addendum.md` §3 — never by hand.

**FR-12, FR-17, FR-19 and RP-2 appear in no row, deliberately.** They are cross-cutting: FR-12's no-privilege boundary and FR-19's interface constrain *every* journey rather than realizing any one, FR-17 owns Settings, and RP-2 owns menu and accelerator survival. A workflow source-extracting by journey will not reach them, so it must read §4 in full — this table is not a complete index of the FR set.

---

## 3. Glossary

Downstream workflows must use these terms exactly. No synonyms appear anywhere else in this document.

- **Manager** — one of Homebrew (`brew`), mise, npm, uv, rustup, or `mas`. Exactly six; the set is closed.
- **Package** — one Manager-owned update unit: a formula, cask, self-updating cask, tool, global package, toolchain, or App Store app.
- **Outdated** — a Manager's verdict that an update exists. Pack-Manager never infers it. See FR-2.
- **Snapshot** — the merged installed-plus-outdated view of one Manager at one moment.
- **Last-good Snapshot** — the most recent successful Snapshot, retained and labeled stale after a later refresh fails.
- **Operation** — one queued unit of work: Refresh, Upgrade, SelfUpdate, or HealthFix. Has an **executor** (whose binary runs), a **subject** (whose data it changes), and a lock set.
- **Executor** — the Manager whose binary actually runs an Operation.
- **Subject** — the Manager whose state an Operation changes. Often, but not always, the same as the executor.
- **Route** — the discovered way a Manager updates itself: in-band, routed through another Manager, via refresh, or unavailable.
- **Managed by** — the ownership relationship derived from the detected installation path, surfaced with human-readable evidence.
- **Upgrade Plan** — the reviewable set of Package and Manager updates, their exact commands, exclusions, notes, and warnings. Under D27 it is one persistent editable draft; see FR-7.
- **Plan Capability** — a bounded, one-use backend authorization bound to a reviewed Upgrade Plan and a coherent state revision. Invalid, evicted, or replayed capabilities enqueue nothing.
- **Plan Attempt** — one confirmed execution of an Upgrade Plan, durably identified by `planAttemptId`. The unit of Activity, Results, and History under D29.
- **VersionDelta** — the display treatment showing `installed → latest` with the changed segment highlighted. Display-only; never a source of truth.
- **Pinned formula** — a Homebrew Package the user pinned deliberately. Never updated in-app (D15).
- **Self-updating cask** — a Homebrew cask that manages its own updates ("greedy"). Excluded from bulk work unless explicitly opted in.
- **Health issue** — a Manager-reported warning about a broken Package or tool environment. Only a narrowly recognized fix may become runnable.
- **Stalled** — an Operation that has produced no output for the configured silence threshold but has not timed out.
- **Interrupted** — an Operation with a durable start record and no durable finish record, discovered on a later launch.
- **Application update** — an update to Pack-Manager itself. Deliberately outside the Operation queue, holds no Manager lock, and never appears in History.

---

## 4. Features

### 4.1 Manager discovery and state truth

**Description:** Pack-Manager's first job is to be right about the machine. It discovers which Managers exist, derives who owns each one from the real installation paths, keeps every Manager's data independent, and never substitutes its own judgment for a Manager's. Realizes AJ-1.

This group carries the product's two hardest-won correctness rules. Both look like implementation detail and are not. First, ownership is classified from the **raw** resolved path before canonicalization — mise shims are symlinks to the mise binary, so canonicalizing first misroutes uv and npm to Homebrew (D3). Second, the Manager's `outdated` verdict is the only verdict (D2).

**Functional Requirements:**

#### FR-1: Detect supported Managers

**Status:** Partial. Detection, ownership evidence, coherent replacement, and Finder/Dock launch all ship. The D40 install-hint extension — hints for all six Managers and the all-absent guidance panel — is Planned, owned by Story 2.5 (added 2026-08-18 by `sprint-change-proposal-2026-08-18.md`). Named inline below.

The user can see which of the six Managers are present on the machine, at launch and on demand, without configuring anything.

**Consequences (testable):**
- All six Managers are probed at launch and on an explicit Re-detect action.
- Each present Manager reports its resolved path, version where available, ownership classification, and a human-readable evidence string.
- Absence is a normal state — rendered as "Not installed" with a copyable install hint — never an error. **Planned — D40 (Story 2.5):** the hint is known for **all six** Managers, not only mas (today `detect.rs`'s `install_hint` returns one only for mas); hints are static, copy-only commands through the existing `CopyableCommand` treatment — never an executing Install button, which D40 rejects on the installer non-goal, SM-3's no-privilege promise, and the no-shell boundary — and a machine where all six Managers are absent gets a Dashboard guidance panel (no package managers found; install one yourself, Homebrew is the usual first; then `Refresh All`) that never reads as `Warning`, because absence is not failure — with `Update Everything` disabled with a stated reason while nothing is installed.
- Detection succeeds when the app is launched from Finder or the Dock, not only from a terminal.
- One coherent detection result replaces the previous one; a partial result never overwrites a complete one.
- Detection details appear in the Environment Report and in the diagnostics export.

#### FR-2: Preserve Manager-reported update truth

**Status:** Shipping.

A Package is Outdated when and only when its Manager says so.

**Consequences (testable):**
- Pack-Manager performs no version comparison to decide outdatedness.
- Manager-supplied version strings render verbatim, including non-semver forms.
- VersionDelta styling and severity chips are display affordances only.
- An unknown latest version stays unknown; the UI says "update available" rather than fabricating a version or a severity.
- Output a parser cannot handle fails that Manager visibly, with an excerpt, rather than presenting incomplete data as complete.

#### FR-3: Refresh Managers independently

**Status:** Shipping.

Each Manager's inventory and Outdated state refresh on their own, and one Manager's failure never damages another's.

**Consequences (testable):**
- Refresh All starts independent work per present Manager; unrelated Managers proceed concurrently.
- Each Manager shows its own loading, phase, timeout, and error state.
- A failed refresh retains the Last-good Snapshot, keeps it browsable, and labels it stale.
- Duplicate refresh requests for the same Manager coalesce rather than queueing twice.
- A successful Upgrade, SelfUpdate, or HealthFix refreshes every affected subject and executor.
- Homebrew's metadata refresh doubles as Homebrew's self-update Route.

#### FR-4: Discover and explain ownership and update Routes

**Status:** Partial. Detection, derivation, precedence, re-evaluation, and Route explanation all ship. The D21 npm consequence does not render on every route — named inline below.

The user can see which Manager owns each tool and how each Manager updates itself, with inspectable evidence.

**Consequences (testable):**
- Ownership and Route are derived from current detection and refresh data. No Route is hardcoded.
- Route precedence is fixed: in-band override → delegated-if-detected → native → unavailable.
- Routes are reconsidered after every fresh Snapshot, because a Manager's own Outdated row can change the correct Route.
- A routed action names both subject and executor in plain language.
- When the required executor is absent, the action is disabled with a stated reason.
- The npm-inside-mise consequence — upgrading the mise-managed Node runtime resets npm and its global packages — appears permanently at the point of action (D21). **Planned:** today it renders only when npm's Route is in-band. The note is computed for mise-owned npm but discarded on the routed branch, because the routed Route carries no channel for it — so mise-managed npm that is *not* itself outdated, the ordinary state on the target machine, shows ownership without the consequence. Making this permanent requires the routed Route to carry the note.

#### FR-5: Present Package state and eligibility

**Status:** Partial. Browsing, search, filtering, and every eligibility rule ship. Two limbs do not: outdated-first ordering, and the explanatory-disabled treatment on ineligible rows — both named inline below.

The user can browse, search, filter, and understand each Manager's Packages, and can tell at a glance what can and cannot be updated.

**Consequences (testable):**
- Current, Outdated, Pinned, self-updating, unknown-version, and error states are visually distinct and carry text or icon equivalents, not color alone.
- Pinned formulae cannot enter the Upgrade Plan. The row keeps a checkbox in place for table alignment, rendered in a *visually* disabled treatment that **must not be the native disabled state** — a natively disabled control cannot receive the pointer interaction this consequence requires. Activation stays inert and never changes membership, while the control remains an explanatory target that states how to unpin and refresh. **Planned:** the shipping row uses native `disabled` plus reduced opacity, which both blocks the explanation and leans on gray styling alone.
- Self-updating casks are excluded by default and grouped separately; including them requires explicit opt-in.
- Up-to-date and otherwise ineligible Packages cannot enter the Upgrade Plan and expose a plain-language reason on pointer interaction. Ineligibility never relies on gray styling alone.
- Manager-specific detail is preserved where useful — uv executables, mise source path, Package kind, Homebrew pinned version.
- A filter narrows the list to Packages with updates, defaulted on whenever anything is outdated, with a "show all" escape.
- **Planned:** within the list, Packages with updates sort first. No ordering is applied anywhere today — the default filter delivers most of the benefit, but the ordering requirement is real and unbuilt.
- The one permitted cross-Manager deduplication is the Rust rule (D10): a single Upgrade Plan never contains both mise's `tool:rust` and rustup toolchains; the mise entry is excluded with a visible reason. No broader cross-Manager deduplication is performed.

**Architecture binding:** the unbuilt outdated-first ordering is constrained by `ARCHITECTURE-SPINE.md` **AD-28**: row ordering is presentation and may change freely, but it may never change how a batch is derived — a range is an anchor and a target over the **ordered filtered set the projection holds**, including off-screen virtualized rows, and never the rendered DOM window. Implementing this ordering must not become a second way to compute membership.

**Feature-specific NFRs:**
- The Package list stays usable and responsive beyond 100 Packages.

---

### 4.2 Reviewable planning and intentional selection

**Description:** This is where the product's central promise lives: *nothing runs that was not staged and shown.* Every path to a mutation converges on one reviewable Upgrade Plan — a Package row, a Manager header, a Manager-wide action, `Update Everything`. Execution happens only against a plan the user confirmed and the backend can still prove coherent. Realizes AJ-2, AJ-3.

This group carries the largest concentration of D27–D30 target state. The trust machinery beneath it (Plan Capability, stale-plan rejection, atomic admission) already ships; what is Planned is the persistent draft, the separate confirmation gate, and the removal of the immediate-execution paths.

**Functional Requirements:**

#### FR-6: Control Upgrade Plan membership directly from eligible Packages

**Status:** Planned — D27. The current build ships a transient selection layer this FR replaces.

A Package checkbox **is** the Upgrade Plan membership control. There is no separate selection to build and then submit.

**Consequences (testable):**
- Checking an eligible Package immediately adds it to the Upgrade Plan draft; unchecking immediately removes it.
- The header checkbox adds or removes every eligible Package matching the active filter, including off-screen virtualized rows, and reports the exact count it will affect. **Its tri-state denominator is that same set** — the eligible Packages matching the active filter, including off-screen rows, and never only the rendered ones. It is **unchecked** when none of that set is staged, **mixed** when some, and **checked** when all. The state is derived from the membership projection and is never stored.
- Current, Pinned, and default-excluded Packages cannot enter the Upgrade Plan under any interaction, including the header checkbox.
- Membership is stored as exact canonical Package identities, never as display strings.
- The draft persists while the user navigates between Managers and the Dashboard, and every staged item is individually removable from the Upgrade Plan.
- Neither a checkbox nor a row action executes anything. **Three immediate-execution call sites are in scope for removal**, not one: the Package row action, and *both* direct Manager self-update paths — the Dashboard Manager card and the Manager workspace self-update card each invoke the self-update command directly today, bypassing the plan entirely. Scoping the D27 work to the row action alone would leave two unstaged mutation paths alive and breach SM-2. A **fourth** immediate-execution call site exists and is deliberately *not* in this FR's scope: a Health issue's `Run fix` (FR-23), which D27–D30 routes through the plan under that FR rather than this one.
- The draft is transient dialog state discarded on close — pre-D27 behavior this FR removes.
- **Membership mutation must accept a batch.** A range or filter-wide interaction submits one membership operation covering every affected Package identity, not one per row. This is a requirement, not an optimization: the canonical draft lives in the backend and every mutation round-trips before the projection updates, so a per-row mapping turns a shift-range across 100 rows into 100 round-trips and breaks NFR-3's "The interface stays interactive beyond 100 Packages, with correct actions reachable at 101 rows."
- **⌘A re-points to membership, and the predicate it reads is rebuilt.** There is **exactly one** eligibility-and-visibility predicate — the search term, the outdated-only filter and its derived default, the greedy-cask exclusion, and per-Package eligibility — and **it is the backend's**, projected to the frontend together with the snapshot it was computed against. The row, the header checkbox, ⌘A, the batch payload, and the tri-state denominator all read that same projected result, so a narrowed batch narrows the denominator with it. The frontend submits concrete identities **plus the snapshot token it read**, and the backend rejects a batch whose token is not its current snapshot. Without a single owner, membership is canonical in the backend while eligibility × filter is computed in the frontend, and the same click yields a different count on two compliant builds. ⌘A must also stop suppressing the native select-all on surfaces that have no Package list — see the Notes.
- **Esc collapses to close-dialog and nothing else.** The shipping cascade is close-dialog → clear-selection → close-drawer, all three rungs in `handleEscape` (`src/hooks/useKeyboard.ts`). This FR removes the middle rung, and the third rung loses its sink when the `ActivityDrawer` retires under `ARCHITECTURE-SPINE.md` AD-17 — so what remains is one rung. **Esc is not handed the sidecar as a replacement sink.** Admission of the draft's own preview is the only thing that empties it (AD-24: "Admission of the draft's own preview empties it as custody transfer … no other path adds, replaces, or clears membership"), so an Esc that dismissed the region would orphan a draft the user still holds — and `Done` already owns dismissing Results. A one-rung cascade is the intended end state, not an impoverished one: Esc is **narrowed, not deleted**, and the surviving close-dialog rung must not be removed alongside the two that are.

**Architecture binding:** `ARCHITECTURE-SPINE.md` **AD-28** — "A Package checkbox *is* membership, and a range is one batched operation" — is the invariant this FR generated, and it binds every consequence above: the direct-membership model, the batch requirement, concrete canonical identities in the batch payload rather than a predicate for the backend to re-expand, the single eligibility predicate behind ⌘A, the header checkbox's tri-state denominator, the closed removal taxonomy behind "individually removable", the Esc cascade, and ⌘U's removal as **dropped, not re-pointed**.

**Out of Scope:**
- A transient selection distinct from draft membership, and any `Add Selected` submit step. Both are eliminated.
- **⌘U (upgrade selected).** Once selection *is* membership, its scope becomes ambient and it collapses into ⌘⇧U (Update Everything) — two accelerators opening the same surface. The non-shift limb is dropped; ⌘⇧U is unaffected.

**Notes:** this resolves the conflict between `docs/SPEC.md` F5 (Esc clears a transient selection) and `EXPERIENCE.md` (selection immediately changes membership) in favor of `EXPERIENCE.md`. The decision was made during the UX run and is recorded verbatim at `ux-Pack-Manager-2026-07-23/.memlog.md:75`; the owner confirmed it on 2026-07-25. See §0.1 for why `docs/SPEC.md` still carries the stale side, and §9 for the reconciliation this triggers in `ARCHITECTURE-SPINE.md`.

**Notes — the ⌘A predicate, corrected.** An earlier draft of this FR stated that "the shipping select-all already computes eligibility through one predicate" and concluded that "no new predicate is needed." **Both claims are wrong**, and the consequence above is restated to match AD-28. The chain ships at least twice and the two copies do not agree:

| Site | What it computes |
| --- | --- |
| `src/hooks/useKeyboard.ts:35`–`53` | `visibleSelectableIds` — greedy-cask exclusion, search, `outdatedOnly ?? anyOutdated`, `isSelectable`. Its own comment at `:34` admits it "mirrors ManagerPane filters". |
| `src/components/manager/ManagerPane.tsx:92`–`107` | the same chain derived independently — `anyOutdated`, `outdatedOnly`, `matchesSearch`, `visibleMain`, then `orderedSelectable`. |

Two implementations of one rule is exactly the divergence AD-28 exists to close, and `useKeyboard.ts:34`'s own comment is the strongest evidence that the duplication is known rather than accidental. That is why the requirement is that the predicate be the backend's, projected with the snapshot it was computed against, rather than that an existing frontend copy be reused.

**Not evidence of divergence, and deliberately not listed above:** `upgradeAll()` at `ManagerPane.tsx:138`–`141` filters `mainPackages` rather than `visibleMain`, so its scope is Manager-wide rather than filter-scoped. That is by design and self-consistent — its label is fed `outdatedCount={outdatedTotal}` (`:234`), and `outdatedTotal` (`:110`) is computed over the same Manager-wide `mainPackages` set the action operates on, so the count and the action agree. The header checkbox is filter-scoped because it sits atop a filtered list. Two scopes by design is not two predicates in conflict. Under FR-6 this path becomes a staging path that must be re-expressed against the single predicate, but it is not the defect AD-28 names.

`[NOTE FOR PM]` — the re-pointed ⌘A must suppress the native select-all **only on surfaces where it actually stages something**. A shipping defect in the same handler currently violates this; it is diagnosed in `addendum.md` §4, and this FR must not inherit it.

#### FR-7: Preview every update command exactly, in one persistent Upgrade Plan

**Status:** Partial. Exact command preview, exclusions, and warnings ship. The persistent editable sidecar and the separate confirmation dialog are Planned — D27, D28.

Before anything runs, the user can see precisely what will run.

**Consequences (testable):**
- Every staged Package and Manager update appears in the plan before execution, grouped by Manager, showing `installed → latest`.
- The exact commands are revealable on demand and are byte-equal to the commands actually spawned.
- Exclusions, their reasons, warnings, and staleness notices are visible.
- The draft stores canonical intent, never executable display strings. Commands are rebuilt by the backend whenever the draft changes and again before execution.
- **Planned — D27:** the sidecar is hidden when empty, appears on first addition, persists across navigation, and offers Remove on every staged item.
- **Planned — D28:** `Confirm N Updates` opens a separate modal confirmation showing the exact commands, offering `Change Plan` and final confirmation. The opt-out checkbox appears *only* in that dialog and persists `skipUpgradePlanConfirmation`. The safe default is confirmation enabled (`false`), and Settings can restore it. The preference removes only the final dialog — never the draft review, the backend rebuild, or the stale-plan check.
- **Architecture binding — `ARCHITECTURE-SPINE.md` AD-28:** the per-item `Remove` this FR requires on the sidecar is a single-ref removal under AD-28's closed removal taxonomy, so each use writes a tombstone that no later bulk expansion re-adds (AD-23). A scope-wide removal — the header checkbox, `⌘A` on an all-staged view, a Manager-wide remove, `Clear`, or undoing an `Update Everything` seed — instead clears membership *and* the tombstones of the refs whose membership it actually cleared, never of refs that held none.
- **Planned — D28, and load-bearing:** when the opt-out is active, three compensations replace the dialog and all three are required. The plan **auto-expands the exact commands** before the action is enabled; a **persistent `Confirmation is off` notice** is shown and links to Settings; and the primary action **relabels from `Confirm` to `Run N updates`**. Removing the gate without them produces a button still reading `Confirm` that executes immediately with commands collapsed behind a reveal — the outcome `EXPERIENCE.md` names as an anti-pattern, and a direct breach of SM-2. The compensations are the price of the opt-out, not a nicety attached to it.

#### FR-8: Reject stale, altered, replayed, or invalid plans

**Status:** Shipping.

Execution proceeds only when the submitted plan matches both the reviewed plan and a fresh coherent rebuild from current state.

**Consequences (testable):**
- A Plan Capability is one-use and bounded — at most 64 unconsumed capabilities are retained per session, oldest evicted first.
- An unknown, evicted, already-replayed, altered, or state-mismatched plan enqueues **nothing** and returns the user to review with a newly issued plan.
- An in-progress state update, revision drift, an active refresh, or a lock-set overlap with any pending or running Upgrade, SelfUpdate, or HealthFix rejects the submission without enqueueing.
- A failed execution other than a stale-plan rejection is never retried speculatively, because the capability may already be consumed.
- Dismissed dialogs and superseded results cannot trigger later execution or state changes.
- Final confirmation is unavailable while a plan rebuild is in flight and after a rebuild failure. The user can never confirm a preview the backend has not just re-derived — during a pending rebuild the displayed commands still belong to the previous options, so a confirmation in that window would execute something other than what is on screen while passing every other check in this FR.

#### FR-9: Admit multi-group plans atomically

**Status:** Partial. Atomic all-or-none admission, conflict serialization, the concurrency cap, queue explanations, and the D22 no-retry contention rule all ship. The one-active-attempt rule is the Planned — D30 consequence below, owned by Story UX-PB.2b. Retagged from Shipping on 2026-08-18 under §9.2's rule — see §9.3.

A confirmed plan is admitted all-or-none. The user is never left with a partially submitted bulk update.

**Consequences (testable):**
- All derived groups enqueue together, or none do.
- Conflicting work serializes; independent Managers run concurrently within a global concurrency cap of 4.
- **Planned — D30:** only one confirmed Plan Attempt may be active at a time. A second confirmation fails closed while an attempt is unterminated, **independent of lock-set overlap** — two attempts touching disjoint Managers are still refused. Cross-Manager concurrency continues to occur *inside* the single active attempt. FR-8's lock-overlap rejection is a different test and does not imply this one.
- Queue relationships are explained to the user ("Queued behind Homebrew").
- External Homebrew lock contention is detected, named distinctly, and **never** retried automatically (D22).

**Architecture binding:** `ARCHITECTURE-SPINE.md` **AD-29** constrains the all-or-none admission above in two directions. Ordering is mint-and-admit **then** append, so the journal record never precedes the admission it records; and the append **gates nothing** — an attempt-journal write failure is surfaced, never fatal, so a full disk may not turn an all-or-none admission into "none". AD-29 also fixes what may count as *active* for the one-attempt rule: only an attempt the running process actually owns. A journal record read at launch is history, never liveness — the failure mode being prevented is a dead attempt resolving as live and refusing every subsequent confirmation, permanently.

#### FR-10: Support intentional single-Package updates

**Status:** Planned — D27.

The user has a low-friction path to update one Package, and it is the same safe path as everything else.

**Consequences (testable):**
- The action adds exactly that one eligible Package to the persistent Upgrade Plan.
- It follows the common confirmation path.
- It retains every eligibility, Route, conflict, and no-privilege protection.
- It never executes immediately and never expands to unrelated Packages.

**Notes:** This FR supersedes D6's immediate single-Package exception. D6's command-trust and stale-plan protections remain fully in force.

#### FR-11: Explain Manager self-update behavior

**Status:** Partial. The Route explanation and the version half of the identity area ship. **Two of the four elements enumerated below do not exist** — see the first consequence. Independent removable membership is Planned — D27.

Each Manager has a standardized identity area that explains what it is, where it is, and how it updates.

**Consequences (testable):**
- Short description, executable path, installed version, and a Manager-status badge reading `NO UPDATES` or `UPDATE AVAILABLE`. **Two of these are unbuilt:** the short description has no field to carry it — `ManagerInfo` (`src/lib/ipc/types.ts`) declares none — and the status badge does not exist; `grep -rni "no updates" src/` returns nothing. The installed version and the executable path do ship. Requirement unchanged; do not read the whole bullet as shipping.
- When an update exists, the installed and target versions appear with the established warning/success treatment.
- Ownership and Route are explained in plain language near the Manager identity, not as unexplained metadata.
- The npm-inside-mise consequence is stated here permanently, subject to the routed-Route limitation recorded under FR-4.
- **Planned — D27:** the Manager update action adds *independent, individually removable* plan membership. Today it is a single global all-or-nothing `include_self_updates` toggle — do not re-entrench a global toggle when implementing this.

---

### 4.3 Safe and reconstructible Operation lifecycle

**Description:** Once work is authorized, the product owes the user three things: it never acquires privilege, it never becomes an opaque spinner, and it leaves enough evidence to reconstruct what happened after a crash. Realizes AJ-4, AJ-5.

**Functional Requirements:**

#### FR-12: Exclude arbitrary shell and privilege paths

**Status:** Shipping.

There is no path through Pack-Manager to a shell, a password prompt, or elevated privilege.

**Consequences (testable):**
- Only product-defined structured Operations execute. Resolved absolute executables are spawned with structured argument vectors — never a shell command string.
- Child processes get a constructed environment, never the inherited one, and null stdin.
- Display text is never parsed back into executable arguments. A submitted preview that does not match the preview re-rendered from trusted arguments fails closed.
- Copy-to-terminal remains a user-controlled handoff, never an automated execution.
- An application update that would require elevation becomes a manual-install-required state rather than an administrator prompt.

#### FR-23: Constrain which Manager-suggested fixes become runnable

**Status:** Shipping.

A Manager can tell the user its environment is broken and suggest a repair. Only a suggestion the product already recognizes, exactly, ever becomes something the user can run.

**Consequences (testable):**
- A detected Health issue is surfaced with its severity, title, and the Manager's own detail text preserved verbatim, plus its suggested fix as copyable text.
- A fix becomes **runnable** only when the Manager-supplied suggestion is byte-equal to the command the product would itself construct for that issue, and the target name passes a safe-name pattern. Anything else stays copyable text with no run affordance. An altered suggestion is not merely rejected at execution — it never becomes an action.
- Running a fix is a **HealthFix Operation** and carries every FR-8, FR-9 and FR-12 protection without exception: a structured argument vector, a constructed environment, null stdin, no shell, and no privilege path.
- The command that runs is the product's canonical one, never the Manager's string. Equality is the *gate*; it is not the source of what executes.
- **Immediate execution here is deliberate, and this is a member of a closed set — not an exception to FR-7.** Three *kinds* of affordance run one known command against one named target without staging: a Package row's own update action, a Manager's self-update button, and a Health issue's `Run fix`. FR-6 counts the same thing by **call site** and arrives at four, because self-update has two entry points — the Dashboard Manager card and the Manager workspace card. Both counts are correct; they measure different things, and a story that removes "three" must confirm which. FR-7's "nothing runs that the user did not see staged" is not weakened by any of them, because each is a single named target the user pointed at directly. What the set may not do is **grow**: a fourth kind is a new decision, and SM-2 is the metric it would breach.
- **Planned — D27–D30:** this path routes through the persistent Upgrade Plan and the confirmation gate like every other mutation, at which point the set above collapses to zero. FR-10 already states that conversion for the row action; this FR states it for Health fixes.

**Notes:** the requirement is stated here rather than left to the Glossary because the Glossary is not where downstream workflows source requirements — `bmad-architecture` and `bmad-create-epics-and-stories` walk §4. Until this FR existed, a capability that executes Manager-suggested commands had no requirement anywhere in §4, and the relationship between `Run fix` and FR-7 was undecided rather than decided-and-narrow. The exactly-recognized-suggestion gate ships today in the uv parser, which computes its own canonical fix and compares the Manager's suggestion against it before marking the issue fixable; a regression test covers an attacker-supplied `--index-url` in the suggestion.

#### FR-13: Show live plan and Operation state

**Status:** Partial. Live streaming, exact command visibility, and `opId` correlation ship. Plan-level state, `planAttemptId` correlation, Activity as a first-class destination, and the Results summary are Planned — D29, D30.

The user can watch work happen and understand where it is.

**Consequences (testable):**
- Queued, running, stalled, and terminal states are exposed with the exact command and live output visible. There is no distinct `cancelling` state: cancellation moves an Operation to its terminal state, and the 5-second SIGTERM grace window is not surfaced as its own status.
- Live output is bounded in memory while the complete transcript is preserved on disk, and the UI says so when earlier output has moved to the log file.
- Refresh Operations never auto-open a live surface and never emit a success notification.
- Side effects fire only on a genuine status transition of an already-known Operation — rehydrated records and first sightings fire nothing, so relaunch is silent.
- **Planned — D29/D30:** plan-level progress correlated by `planAttemptId` with nested Operations by `opId`; a `Verifying` state before success is declared; Activity as a first-class navigation destination; a terminal Results summary with successes, failures, skipped work, verification outcomes, and Retry where appropriate.

**Architecture binding:** `ARCHITECTURE-SPINE.md` **AD-29** places the two halves of that Planned limb at different levels, and they are not the same journal. Per-Operation `Verifying` and `Skipped` are durable states in the **Operation** journal; verification outcomes and the Results summary ride the attempt's **terminal** record, so Results is served by one read. An attempt gets exactly two records — admission and terminal — never one per transition, because `operations.jsonl` already carries per-step detail under the same `planAttemptId`.

#### FR-14: Handle stalls, timeouts, and cancellation honestly

**Status:** Partial. The 120-second stall threshold, the 30-minute hard cap, immediate cancellation with process-group escalation, and explicit terminal states all ship. Attempt-wide `Cancel plan` and trusted interaction classification are Planned — D30. **The quit guard is unbuilt** — see the consequence below.

Silence and excessive duration become honest, actionable states rather than an indefinite spinner.

**Consequences (testable):**
- After the configured silence threshold (default 120 seconds) the Operation is visibly Stalled and offers Keep waiting, Copy command, and Cancel.
- The stall surface states that Pack-Manager never enters passwords.
- The configured hard cap (default 30 minutes) produces an explicit Timed out terminal state.
- Cancellation requires no confirmation dialog, escalates SIGTERM → 5s grace → SIGKILL over the process group, and promises no rollback of partially completed work.
- **Not yet built:** a **user-initiated** quit with work in flight — a window-close request or ⌘Q — presents an explicit choice and does not silently discard it. The dialog exists and is rendered by the shared dialog host, but nothing listens for a quit — its only caller is the application-update path, so the *restart* case is guarded (FR-21) and the *quit* case is not. Do not read this consequence as shipping.
- **An OS-initiated shutdown or logout is deliberately excluded from that promise.** It gets **no dialog**. The behavior is best-effort: run the existing kill hook — cancel every running Operation, then await the bounded idle wait, because cancellation only flips the tokens while the runner tasks perform the SIGTERM → grace → SIGKILL work, so a process that exits without awaiting may never poll them. The reason for the carve-out is stated rather than left implicit: blocking a logout to argue with the user is worse than losing the run. The invariant that holds on either path is that **children never outlive the app** — which is why this is a narrower promise, not a weaker guarantee.
- **Architecture binding — `ARCHITECTURE-SPINE.md` AD-30:** the quit guard has **one enforcement point** and every path reaches it — the OS window-close request and `⌘Q` resolve to it exactly as the application-update path already does. One predicate, one dialog, one refusal; a second path that decides for itself is the defect. AD-30 also fixes the guard's active set as `Queued` ∪ `Running` — **queued counts as running** — identical to FR-21's app-update guard, and the two may not drift apart. The no-rollback promise above extends to the quit path unchanged: the guard surfaces the choice, and partially completed Manager work stays partially completed.
- **Planned — D30:** `Interaction required` is shown **only** when a closed Manager-specific classifier or an explicit native signal recognizes a trusted prompt. All other null-stdin silence follows the ordinary stall path. The primary cancellation label becomes `Cancel plan` when the whole attempt is affected; `Cancel operation` is reserved for deliberately Operation-scoped diagnostics.

#### FR-15: Preserve History, transcripts, and crash evidence

**Status:** Partial. Per-Operation transcripts, the crash journal, `Interrupted` reconstruction, and Operation-level History ship. Plan Attempt as the History unit and Retry lineage are Planned — D29.

The user can answer "what ran, what happened, where is the evidence" after the fact.

**Consequences (testable):**
- Every Operation writes a durable transcript and crash-safe start and finish records.
- Work with a start record and no finish record surfaces as Interrupted on the next launch.
- Recorded process group identifiers are historical evidence only and are **never** signaled after relaunch — process IDs are reused.
- Transcript content is faithful to child output, with exactly one exception: D26's closed, literal list of unterminated `mas` notices, after which one readability newline may be inserted. No general heuristic rewriting.
- Retention is bounded and stated: application logs pruned beyond 14 days, transcripts kept to the newest 200 files or 90 days, History compacted to the newest 1,000 records.
- Durable evidence is also **reachable**, not merely retained. History is browsable and filterable by Manager, by status, and by free-text search; a record exposes its full command and transcript tail; and the on-disk transcript can be revealed in Finder. Retention without these affordances would satisfy every other consequence in this FR and still leave the evidence unusable.
- **Planned — D29:** one immutable History row per confirmed Plan Attempt, with Operation evidence nested inside it and Activity replay from the row. Retry creates a **new linked attempt** and never overwrites the first failure. Legacy Operation records without an attempt identity stay visibly legacy and are never fabricated into plans.

**Architecture binding:** `ARCHITECTURE-SPINE.md` **AD-29** makes "one immutable History row per attempt" true by construction rather than by a story remembering to deduplicate: an attempt is an **idempotent fold** over its records, keyed by `planAttemptId`, and a second terminal record is duplicate evidence, never a state change. AD-29 also narrows this FR's Interrupted test at attempt scope — `Interrupted` requires a **genuine** absence, and a terminal record that exists but is unreadable is reported as *unreadable evidence*, never silently reclassified as an unfinished attempt. Those are different facts and a reader must be able to tell them apart.

#### FR-16: Preserve useful state after Operation outcomes

**Status:** Shipping.

An outcome — good or bad — leaves the user with more information than they had, never less.

**Consequences (testable):**
- Successful work refreshes affected state automatically.
- Failed work retains the Manager's prior useful state.
- Errors state what happened and what to do next, in plain language, per error class.
- `View log` is exposed only when a corresponding log actually exists — the action never dangles.

---

### 4.4 Settings, diagnostics, and interface quality

**Description:** The preferences the user can tune, the evidence they can export, and the one coherent macOS surface all of it lives in. Realizes AJ-5.

**Functional Requirements:**

#### FR-17: Persist Settings atomically

**Status:** Partial. Atomic persistence and all eight shipping fields work. `skipUpgradePlanConfirmation` is Planned — D28. `autoOpenDrawer` is **retiring, not being demoted** — it goes with the surface it controls, and nothing in this FR gives it a future. See the Notes for the D28 wording reconciliation.

A setting becomes active only after it is successfully saved.

**Consequences (testable):**
- A patch is persisted **before** in-memory settings change and before the canonical state revision advances. A failed save changes neither.
- The settings and their defaults **as the current build ships them** are below. This table is the shipping inventory, not the target set — one row is retiring and is marked so:

  | Setting | Default |
  | --- | --- |
  | Run Homebrew metadata update during refresh | on |
  | Refresh automatically on launch | on |
  | Stall threshold | 120 seconds |
  | Upgrade hard cap | 30 minutes |
  | Application log level | debug, for the app's own code |
  | Auto-open the activity surface for mutations — **shipping only; retires with the drawer** (`autoOpenDrawer`) | on |
  | Include self-updating casks by default | off |
  | Check for application updates automatically | on |
- The Settings view also provides the read-only Environment Report, Copy, Open Logs Folder, diagnostics export, and Re-detect.
- **Planned — D28:** `skipUpgradePlanConfirmation` is added with a safe default of `false` and is reversible in Settings.
- **Planned — D27–D30:** `autoOpenDrawer` **retires.** It leaves the Settings view and the field set above along with the `ActivityDrawer` surface it controls; no story keeps it alive as an inert setting, and no story keeps the drawer alive as a second home for attempt status (`ARCHITECTURE-SPINE.md` AD-17). The other seven fields are the target set.

**Notes:** the eighth field — check for application updates automatically — ships today and is absent from both of `docs/SPEC.md`'s settings sections. It is recorded here because the byte-equality contract fixtures already carry it.

**Notes — `autoOpenDrawer` is reconciled, not restated.** Two sources describe its end state differently and the later one is stronger. `docs/DECISIONS.md:193` says the "obsolete `autoOpenDrawer` setting becomes inactive legacy input" — inert but still present. `ARCHITECTURE-SPINE.md` AD-17 (2026-07-25) says "The existing `ActivityDrawer` surface retires with the `autoOpenDrawer` setting." A setting whose only surface no longer exists has no inert state left to hold, so this FR records retirement rather than demotion; "inactive legacy input" describes a transitional state that only made sense while the drawer survived.

`[NOTE FOR PM]` — only a decision supersedes a decision, and D28's sentence still reads as demotion. If the decision record should say *retires*, that is a `docs/DECISIONS.md` edit for the owner; this PRD records the reconciliation rather than editing a file no workflow owns.

#### FR-18: Export privacy-preserving diagnostics

**Status:** Partial. The archive, its path, and every content enumerated below ship. The plan-attempt journal is Planned — D29, and its absence is not a defect in the shipping build: there are no attempt records to export yet.

One action produces one support bundle that explains the machine without disclosing it.

**Consequences (testable):**
- A timestamped archive is written to the documented Desktop path.
- It contains an environment and detection report, the newest three application logs, the newest 25 transcripts, and the History journal.
- It includes application, OS, and architecture information, the constructed search path and its source, detection evidence, settings, and the log filter.
- It contains **only** environment values Pack-Manager itself set. The inherited environment is never dumped.
- Symlink substitution is rejected both when selecting files and when streaming them.
- **Planned — D29:** the archive also carries the **plan-attempt journal**, as a second journal distinct from the Operation journal above and entered separately. It carries that journal's **raw lines** — never a synthesized record, and never a fold written back into the journal. An attempt's exported evidence is therefore the two-record *set* for its `planAttemptId`, admission plus terminal: its scope, exact commands, verification facts and results are carried *between* the two records, so no single record can satisfy the requirement. A folded attempt view may be added as an **additional** entry, marked as derived; it never replaces the raw lines.

#### FR-19: Provide one coherent macOS interface

**Status:** Partial. Shipping for the current navigation model; the D30 navigation changes — Activity as a first-class destination (Story UX-PB.3b), the Results surface (UX-PB.3d), and one-plan-per-row History (UX-PB.4a) — are Planned. Retagged from Shipping-prose on 2026-08-18 under §9.2's rule — see §9.3.

The app reads as one focused macOS control surface, not six command wrappers in a window.

**Consequences (testable):**
- One coherent dark-only interface spans the Dashboard, Manager navigation and workspaces, the Upgrade Plan, Activity, History, Settings, status surfaces, and application menus.
- The visual identity is the approved "Aurora Control Deck" palette in `DESIGN.md`, adopted by D35, with a recognizable package/update application icon.
- All color states carry text or icon equivalents; status chips do not wrap.
- Text contrast meets at least 4.5:1 on its surface, and the floor is CI-asserted (D36).
- The reduced-motion preference disables transitions.
- Every interactive element carries a visible focus indicator, drawn as a real `outline` — never a `ring-*` box-shadow, which WebKit does not paint on native-appearance form controls, and never `outline-none`. Focus is a dedicated indicator, never the accent, so focused and selected stay distinguishable.
- VersionDelta remains display-only and never decides whether an update exists.
- The interface remains usable at 900 × 600, at 150–200% zoom, with more than 100 Packages, and with long command output. Narrow widths scroll rather than letting essential content collide.
- The app launches correctly from Finder and the Dock.

**Notes — D37 reconciliation (the conflict this PRD resolves).** `epics.md` FR-19 (line 89) and NFR-6 (line 113) require primary actions be "keyboard/VoiceOver operable" with plan-progress announcements. D37 is the later decision and removes keyboard navigation and screen-reader support as criteria, on the same evidence D33 used to retire the enterprise apparatus. **This PRD restates FR-19 and NFR-6 without those obligations.** Specifically dropped: keyboard operability of primary actions, VoiceOver operability, live-region announcements of plan progress/verification/cancellation/failure/completion, and NFR-6's deterministic dialog/sidecar focus restoration.

Three things explicitly stay, and none of them is kept as an accessibility obligation:

1. **The focus indicator.** It ships across 31 sites, is asserted in CI by `tests/e2e/browser-style-contract.spec.ts`, and is governed by D35 and AD-27. Removing the rule would delete no work and would only un-guard working code against the next `ring-*` — the exact trap D35 documents.
2. **⌘X / ⌘C / ⌘V / ⌘A, and the application accelerator map** — both enumerated in RP-2. Copy, paste, and refresh are things a mouse user does constantly, and per D25a they break silently if the Edit and Window submenus are not re-declared. That is a functional copy/paste regression, not an accessibility check.
3. **Contrast** (D36's guard). It caught text unreadable to anyone looking at the button — product quality, not accommodation.

Pointer-facing explanations of *why* a Package is ineligible also stay; only their keyboard and screen-reader limbs are out of scope.

`epics.md` and `EXPERIENCE.md` still carry the removed obligations, and both are workflow-owned: they come out through `bmad-correct-course` and a `bmad-ux` Update respectively — never a hand edit. **Two artifacts previously listed here are done.** `ARCHITECTURE-SPINE.md` revision 10 **is** the `bmad-architecture` Update this was waiting on, and it applied D37 through AD-11, AD-16, AD-17 and AD-27. `docs/RELEASE-CHECKLIST.md` was rescoped in `5c8996f` and now states the removal affirmatively — "they are not oversights, and a future regeneration or review should not reinstate them." **Scope each run by section, not by a mention count** — `addendum.md` §3 carries the named headings, and the reason that matters is recorded there.

---

### 4.5 Pack-Manager application updates

**Description:** The app updates itself, and the boundary between updating the app and updating packages is deliberate and visible. Application updates sit outside the Operation queue entirely — they are not Operations, hold no Manager lock, and never appear in History. Realizes AJ-6.

**Functional Requirements:**

#### FR-20: Expose and automatically download available application updates

**Status:** Shipping.

The user learns about a new version without asking, and the download does not interrupt their work.

**Consequences (testable):**
- Checks run at launch, every six hours, and on demand from the application menu — the two automatic triggers hold while `Check for application updates automatically` (FR-17's eighth setting, default on) is enabled; the menu check is always available (D39 records the gate).
- A newer authorized release found by any check downloads automatically in the background. Automatic **download** is required behavior, not an optional outcome — installation is the machine mutation, and that stays user-gated.
- Checking, available, downloading, ready, and failure states are visible.
- Only a *manual* check may surface a notification. An automatic check that finds nothing, or that fails while offline, stays invisible, and a repeated payload for the same terminal state does not re-notify.
- Package work remains understandable and uninterrupted throughout.

#### FR-21: Require explicit installation and relaunch

**Status:** Shipping.

Nothing about the installed application changes without a deliberate user action.

**Consequences (testable):**
- A downloaded update installs only after the user chooses Restart to update. Never a silent install, never a silent restart.
- Installation and relaunch are **refused** while any Package Operation is queued or running. Queued counts as active — admission has already committed to the work, and a restart would drop it unstarted.
- This refusal is enforced independently in two layers, and both must stay: the frontend quit guard explains it to the user, and the backend refuses on its own because the install path terminates every child process before relaunching.
- A non-writable install location produces manual-install-required, never an administrator prompt.
- The app relaunches as the intended version, and success is reported only after that.
- Every update-stage failure is actionable.

**Architecture binding:** `ARCHITECTURE-SPINE.md` **AD-30** takes this FR's shipping `Queued` ∪ `Running` predicate as its source of truth for the Package-work quit guard (FR-14) and then binds this FR in return: **the two guards' active sets must stay identical and may not drift apart.** A change to the refusal predicate above is now an AD-30 change, not a local one. The two-layer enforcement this FR requires is likewise the shape AD-30 generalizes — its "children never outlive the app" is the invariant the backend layer here already states.

#### FR-22: Launch normally and accept only authorized updates

**Status:** Shipping.

**Consequences (testable):**
- The release is universal and supports both Apple silicon and Intel. Update metadata publishes **both** architecture keys, both pointing at the single universal archive; dropping the Intel key would strand every installed Intel user with no signal (D32).
- The application, disk image, and updater payload are signed and notarized, and the relevant bundles are stapled — so the normal update path does not require Gatekeeper to contact Apple on first launch. This supersedes D20 and `docs/SPEC.md`'s stale "notarized DMG is out of scope" line.
- Only updater payloads cryptographically authorized for the installed application are accepted.
- Verification is Apple-silicon only; Intel remains best-effort and unverified (D32).

**Architecture binding:** `ARCHITECTURE-SPINE.md` **AD-12** carries the signing and notarization obligation above as a release invariant and names the one path that exercises it without publishing: the manual workflow-dispatch run publishes nothing only when `attach_to_tag` is empty.

**Reconciled against AD-12's narrowing.** AD-12 was narrowed in spine revision 10 from a file-scoped reading to a **field-scoped** one: release-please owns the *version* in `package.json`, `package-lock.json`, `$.version` in `src-tauri/tauri.conf.json`, `$.package.version` in `src-tauri/Cargo.toml`, and the `pack-manager` entry in `src-tauri/Cargo.lock`, plus `CHANGELOG.md` and `.release-please-manifest.json` whole — and "**Everything else in those files stays maintainer-owned.**" This **enables** rather than contradicts anything this FR requires. The updater `pubkey` lives in `src-tauri/tauri.conf.json` but appears in no `extra-files` path, so release automation never reads or writes it; under the old file-scoped reading the product was, in AD-12's words, "unable to rotate its own signing key". Rotating it is now ordinary maintainer work. Nothing above changes; the constraint that blocked a rotation is gone.

---

### 4.6 Release prerequisites

These two are mandatory prerequisites rather than product features. They are validated through `docs/RELEASE-CHECKLIST.md`.

#### RP-1: Preserve application-update triggers and state continuity

**Status:** Shipping.

**Consequences (testable):**
- Launch, six-hour, and app-menu update checks are all preserved.
- In-process update state survives supported UI recreation.
- The saved trigger policy survives a normal relaunch.
- A failed or interrupted download never presents as Ready.
- Application-update state stays separate from the Operation queue and from History.

#### RP-2: Preserve standard macOS menu behavior

**Status:** Partial. The menu declarations and every accelerator registration ship and survive `app.set_menu`. ⌘L's sink is Planned — D27–D30, and ⌘A's re-point to membership is Planned — D27 (FR-6).

Standard Edit and Window menu actions — including cut, copy, paste, and select-all in the search field and in every copyable command surface — are preserved. This is a functional requirement, not an accessibility one: `app.set_menu` replaces Tauri's default menu wholesale, so these submenus must be re-declared or the shortcuts die silently (D25a).

The application accelerators outside the Edit and Window standards are equally in scope and must survive the same menu replacement: **⌘R** (refresh current, or all from the Dashboard), **⌘⇧R** (refresh all), **⌘⇧U** (Update Everything), **⌘L** (move focus into the Upgrade Plan sidecar region), **⌘F** (focus search), and **⌘1–9** (navigation jump). ⌘A is covered above as an Edit-menu action, and its re-pointing under D27 is specified in FR-6.

**⌘L is a focus move, not a toggle. Planned — D27–D30 for the behavior; the registration ships.** There is no activity surface to toggle: the sidecar is a layout region whose visibility is derived from its content (FR-7), so it holds no toggled state for an accelerator to own. ⌘L moves focus into the region, and when the region is hidden it is a **no-op** — it must not conjure the region into existence. The shipping handler instead toggles the `ActivityDrawer` (`src/hooks/useKeyboard.ts:164`–`166`, `toggleDrawer()`), and that sink retires with the surface: `ARCHITECTURE-SPINE.md` AD-17 — "The existing `ActivityDrawer` surface retires with the `autoOpenDrawer` setting; no story keeps it alive as a second home for attempt status." Re-pointing the accelerator is therefore part of the same D27–D30 work, not a separate concern. Like the rest of RP-2 this is a **functional** requirement on a shipped shortcut, not an accessibility one — D37 retired keyboard operability and deterministic focus *restoration* as criteria (FR-19 Notes, §10) and retired neither this accelerator nor the menu it lives in.

**Architecture binding:** `ARCHITECTURE-SPINE.md` **AD-28** names "the application accelerator map (`prd.md` RP-2)" in its own binding list and declares the enumeration above **complete** for the post-D27 product — which is what makes ⌘U's absence deliberate rather than an omission. AD-28 also generalizes the rule behind FR-6's `[NOTE FOR PM]`, and it belongs here too because this RP is where the Edit-menu guarantee lives: an accelerator that shadows a standard Edit-menu action suppresses the native default **only on surfaces where it performs its own action**. Re-declaring the Edit menu is therefore necessary but not sufficient — the shipping ⌘A handler calls `preventDefault()` before its helper early-returns on views with no Package list, so on the Dashboard, History and Settings it blocks native select-all and puts nothing in its place. D37 does not excuse it: ⌘A is an Edit-menu action D37 keeps by name.

---

## 5. Cross-Cutting NFRs

#### NFR-1: Fail closed

**Status:** Shipping.

Unreviewed, stale, altered, replayed, partially admissible, or privilege-seeking work never runs. User exclusions and Manager protections remain authoritative in every path.

#### NFR-2: Isolate and recover from failure

**Status:** Shipping.

Detection, refresh, parse, network, update, crash, cancellation, timeout, and persistence failures are contained. None of them blanks another Manager or destroys a Last-good Snapshot. Recovered partial data merges into the existing inventory rather than replacing it — replacing a Snapshot with an outdated-only overlay would make every up-to-date Package vanish.

#### NFR-3: Stay responsive

**Status:** Shipping.

State renders progressively without waiting for all Managers. The interface stays interactive beyond 100 Packages, with correct actions reachable at 101 rows. Live output flushes at 50 ms, 64 lines, or 8 KiB, whichever comes first. The newest 5,000 live lines are retained while the complete transcript is preserved on disk. Navigation, the plan, confirmation, Activity, Results, and recovery all remain usable at 900 × 600 and at 150–200% zoom.

**Architecture binding:** `ARCHITECTURE-SPINE.md` **AD-28** quotes the 101-rows sentence above by name as the requirement a per-row membership mapping breaks — the canonical draft lives in the backend and every mutation round-trips before the projection updates, so a shift-range across 100 rows would become 100 round trips. This is why FR-6's batch is a requirement rather than an optimization.

#### NFR-4: Correlate evidence

**Status:** Partial. `opId` correlation ships; `planAttemptId` is Planned — D29.

Status, output, transcript, structured log, History, and diagnostics correlate through durable identity. Failure to create an Operation transcript blocks the spawn — an unaudited command never starts. Later non-critical logging failures never hang Package work.

**Architecture binding:** `ARCHITECTURE-SPINE.md` **AD-29** reads the two halves of that sentence as a deliberate asymmetry and binds both: the transcript is a precondition for a spawn, while an attempt-journal append gates nothing and is surfaced rather than fatal. AD-29 also names this NFR as the requirement that forbids a persisted `PlanAttempt.state` field — a record stamped `admitted` that the fold reads as `Interrupted` would make a support bundle and the interface disagree about one attempt, which is precisely the correlation this NFR exists to guarantee.

#### NFR-5: Protect privacy and the trust boundary

**Status:** Shipping.

No telemetry. No generic shell surface. Inherited environment values are excluded from logs and diagnostics. Diagnostic export resists symlink substitution. Any external-content, capability, or permission change is security-sensitive.

#### NFR-6: Maintain interface quality

**Status:** Partial. Non-color cues, reduced motion, the focus indicator, the size and zoom floors, display-only VersionDelta, and the 4.5:1 contrast floor all ship — the last landed in `a201fb0` ("use the palette's dark ink on bright accent fills") with its automated guard, and D36 records it. The one unbuilt limb is the explanatory-disabled treatment on ineligible rows (FR-5).

Non-color status cues and pointer-accessible ineligibility reasons; at least 4.5:1 text contrast; reduced motion honored; a visible focus indicator on every interactive element (see FR-19 for the mechanism and why it is not an accessibility obligation); usability at 900 × 600 and 150–200% zoom, and with more than 100 Packages; VersionDelta display-only.

**Restated per D37** — the keyboard-operability, VoiceOver, announcement, and focus-restoration limbs of `epics.md` NFR-6 are removed. Full reconciliation in FR-19's notes.

#### NFR-7: Meet platform compatibility

**Status:** Shipping.

Normal GUI launch from Finder and the Dock. Both promised architectures supported. Incompatible Manager output fails visibly and locally rather than crashing the app. **Minimum supported macOS is 15.0**, declared in `src-tauri/tauri.conf.json` — D31 closed this; it is no longer an open prerequisite for candidate acceptance.

#### NFR-8: Keep delivery artifacts coherent

**Status:** Shipping.

Direct-download and updater artifacts stay mutually consistent, cryptographically authorized, and attributable to one release, without weakening explicit install/restart control. Two release-blocking checks enforce this: the updater's detached signature is verified against the public key the shipping app embeds, and the published update metadata is asserted reachable and coherent after upload. Without them, a drifted signing key produces a fully green release that silently breaks updates for every installed client.

**Architecture binding:** `ARCHITECTURE-SPINE.md` **AD-12** fixes where that embedded public key lives and who may change it. The `pubkey` sits in `src-tauri/tauri.conf.json` but in none of release-please's `extra-files` paths, so the release automation never reads or writes it — which narrows the drift this NFR guards against to exactly two causes: a maintainer edit to the key, or a signing secret that no longer matches it. Neither is something a release run can introduce on its own, and both are caught by the first of the two checks above.

---

## 6. Non-Goals (Explicit)

Pack-Manager is not, and will not become in v1:

- **A package installer or uninstaller.** It updates what is already installed. Nothing else.
- **A privilege-escalation path.** No `sudo`, no password entry, no administrator prompt, anywhere, for any reason. This is a hard product boundary, not a default.
- **An unattended updater.** No scheduled or automatic Package upgrades. Application-update *download* is automatic; application-update *installation* is not.
- **A terminal.** No general shell surface, no arbitrary command execution, no user-supplied argument vectors.
- **A version oracle.** It never replaces a Manager's outdated verdict with its own comparison.
- **A telemetry client.** Nothing is reported anywhere.
- **A pin-breaker.** Pinned Homebrew formulae are never force-updated in-app.
- **A retry loop.** External Homebrew lock contention is named and handed back to the user, never retried automatically.
- **A rollback engine.** Cancellation stops work; it does not undo a partially completed Manager command, and the product does not pretend otherwise.
- **A cross-Manager deduplicator.** The single Rust rule (D10) is the only exception, and it is scoped to one plan.
- **A team or fleet tool.** No multi-user, no shared state, no policy.

---

## 7. Scope

### 7.1 Shipping today (1.0.1)

- Manager detection, ownership derivation, and Route precedence.
- Independent per-Manager refresh with Last-good Snapshot retention.
- The Package table with full eligibility semantics.
- Exact-command plan preview with the complete fail-closed trust machinery, and atomic multi-group admission.
- The lock-set scheduler with cross-Manager concurrency.
- Live output streaming, stall detection, cancellation, and the hard cap.
- Operation transcripts, the crash journal, and `Interrupted` reconstruction.
- Operation-level History with search, filtering, and reveal-in-Finder.
- Diagnostics export.
- Settings with atomic persistence.
- The dark-only Aurora Control Deck interface with a CI-asserted focus mechanism.
- uv health fixes, with the exact-suggestion-only constraint on what becomes runnable.
- The full application self-update flow, including the two-layer refusal while Package work is in flight.
- The automated contrast guard — the 4.5:1 assertion and the on-fill ink tokens that make it pass — landed in `a201fb0` and recorded by D36. Contrast at release time is CI-asserted, not a by-eye check.
- Health-issue detection and the exactly-recognized-suggestion gate on what becomes runnable (FR-23).

### 7.2 Decided, not yet implemented (D27–D30)

The Upgrade Plan redesign, in one coherent block:

- One persistent editable Upgrade Plan draft, replacing transient dialog state, with every path converging on it and **no** immediate execution from a row, header, or selection (D27).
- Independent, individually removable Manager self-update membership, replacing the global all-or-nothing toggle (D27).
- A separate final confirmation dialog with a reversible skip preference (D28).
- A durable `planAttemptId` correlating reviewed intent, Operations, events, transcripts, journal records, verification, Results, and Retry lineage — with History carrying one immutable row per confirmed attempt (D29).
- One active attempt at a time, a `Verifying` state before success is declared, Activity as a first-class destination, a terminal Results summary, attempt-wide `Cancel plan`, and trusted-classifier-only `Interaction required` (D30).

This is the live build queue — 28 stories under `epic-ux-pb` in `_bmad-output/implementation-artifacts/sprint-status.yaml`, which D33 describes as "28 real product stories".

### 7.3 Deferred

- **Snapshot cache**, **native notifications when backgrounded**, **Package detail popover**, and the **"also managed by rustup"** note on mise's Rust row. `[NOTE FOR PM]` — these were P1 in `docs/SPEC.md`, but D33's surviving habit applies: verify whether each already ships before scheduling it as new work. An adversarial triage pass overturned 14 of 20 initial keep verdicts for exactly that reason.
  - **Health fixes were on this list and have been removed from it — they ship in full**, including the exact-recognized-suggestion constraint that gates which fixes become runnable. Deferring them was this document committing the very error the note above warns against, one sentence later. They are recorded in §7.1.
- **Light theme** — the tokens live in one file specifically so this is a value swap, but dark-only is the v1 commitment (D19).
- **Menu-bar extra**, **scheduled Package refresh** (distinct from the six-hour *application*-update check), **`cargo install` support**, and cross-Manager deduplication beyond the Rust rule.

---

## 8. Success Metrics

Calibrated to what this project is. D33 established the scale on observable evidence — 1 star, 0 forks, 3 lifetime `.dmg` downloads, with 27 of 30 recorded downloads being the app's own updater traffic. Adoption metrics would be theater.

**Primary**

- **SM-1: Sustained personal use.** The maintainer reaches for Pack-Manager instead of six terminals, and still does so a month from now. Validates the product thesis as a whole.
- **SM-2: Zero unreviewed mutations.** No Package or Manager update ever runs that the user did not see staged first. A single violation is a P0 defect, not a metric miss. Validates FR-7, FR-8, FR-10.
- **SM-3: Zero privilege prompts.** No release ever surfaces `sudo`, a password field, or an administrator dialog. Validates FR-12, FR-21.

**Secondary**

- **SM-4: Failure legibility.** When something fails, the transcript plus History answer "what ran and what happened" without reproducing the failure. Validates FR-15, FR-16, FR-18.
- **SM-5: Update path integrity.** Every release's updater signature verifies against the embedded public key and its metadata is reachable and coherent, checked automatically at release time. Validates FR-22, NFR-8.

**Counter-metrics (do not optimize)**

- **SM-C1: Time-to-update.** Making the path from intent to execution *faster* is not a goal and directly counterbalances SM-2. The review step, the confirmation gate, and the stale-plan rejection all exist to add deliberate friction. Anyone optimizing clicks-to-upgrade is optimizing away the product.
- **SM-C2: Feature count.** Counterbalances the temptation in §7.3. This is a six-Manager update tool for one machine. A larger surface is a larger thing to keep correct, and §6 exists to make the boundary expensive to cross by accident.

---

## 9. Open Questions

All four questions this section carried were closed on 2026-08-18 by `docs/DECISIONS.md` **D39–D42** — see §9.3 for each closure and where its consequences landed. This document carries **no phase-blockers** and, as of 2026-08-18, **no open questions**.

**Closed since the first draft:** what happens on quit with work *queued* but not running, and on an OS-initiated shutdown. Both are decided by `ARCHITECTURE-SPINE.md` AD-30 and are recorded in FR-14 — see §9.2.

### 9.1 Closed during this run

**Q1 — Does a Package checkbox mutate Upgrade Plan membership directly, or is there a separate transient selection? CLOSED 2026-07-25: directly.**

`EXPERIENCE.md` is correct and `docs/SPEC.md` F5 is the stale side. The decision was already made during the UX run and is recorded verbatim at `ux-Pack-Manager-2026-07-23/.memlog.md:75` — "Package checkboxes directly control Upgrade Plan membership: checking an eligible Package immediately adds it, unchecking immediately removes it, and the header checkbox adds or removes all eligible visible Packages. Eliminate the separate temporary selection and Add Selected layer." The owner confirmed it directly on 2026-07-25.

The conflict survived for a mechanical reason, not an unresolved one: `docs/SPEC.md` §0.1's supersession list retires six behaviors and **never added F5**. That omission is the whole defect, and it is now recorded in §0.1. The live `selection` set in `src/store/packages.ts` is pre-D27 code, not a competing decision.

Two consequences carried into this document:

- FR-6 is rewritten around direct membership control, with the transient selection and `Add Selected` layers explicitly out of scope, and it now states the **batch** requirement that comes with the model — one membership operation per range or filter-wide interaction, not one per row. The cost that makes this a requirement is recorded at `architecture-Pack-Manager-2026-07-23/reviews/review-reconcile-epics.md:166`: mapping selection onto draft membership per-row turns a shift-range across 100 rows into 100 Rust round-trips, against NFR-3.
- The spine's **"Transient selection has no owning invariant"** register row recorded this as OPEN and routed it to the owner. Revision 10 **closed it**: the row now reads RESOLVED, closed by **AD-28**, and the invariant it was waiting for exists. No architecture follow-up is owed.

### 9.2 Closed by `ARCHITECTURE-SPINE.md` revision 10 (Update pass, 2026-07-25)

**Q — What happens on quit with work *queued* but not running, and on an OS-initiated shutdown? CLOSED: both decided by AD-30.** This was Open Question 1 and is no longer open. AD-30 states "**Queued counts as running.** This is not an open question" and fixes the guard's active set as `Queued` ∪ `Running` — identical to FR-21's application-update guard, with which it may not drift apart. The second half is decided too: an OS-initiated shutdown or logout gets **no dialog** and is best-effort, on the stated ground that blocking a logout to argue with the user is worse than losing the run. Both are now consequences of FR-14, and FR-21 carries the binding that ties the two active sets together.

**Four requirement defects were corrected in the same pass**, each found by reconciling against a revision-10 AD rather than by review of the prose:

- **FR-6's ⌘A consequence** asserted a single shipping predicate and concluded none was needed. Two implementations exist and a third diverges; corrected against AD-28, with the evidence in FR-6's Notes.
- **FR-6's header-checkbox consequence** left the tri-state denominator unstated — AD-28 names that exact wording as the ambiguity "two stories would pick differently". The denominator is now explicit.
- **FR-14's quit promise** was unqualified over all quits, asserting a choice AD-30 deliberately withholds on OS-initiated shutdown. Now scoped, with the carve-out and its reason stated.
- **FR-18 was `Shipping` with a closed contents list**, so an implementer could ship an archive with no plan-attempt records and believe the requirement met. It now carries a Planned — D29 limb for that journal's raw lines.

**The rule behind that last one is general, and is stated here once rather than per-FR.** A requirement whose consequences include a **Planned** limb is **Partial**, never Shipping — §0's definition admits no third reading, and a Shipping tag over an unbuilt consequence is exactly the trap FR-18 set. Applying it cost RP-2 its Shipping tag in this pass (⌘L's sink and ⌘A's re-point are both Planned). **FR-9 and FR-19 had the same shape and were recorded here unreconciled** — FR-9 was tagged Shipping while carrying a `Planned — D30` consequence, and FR-19's status was prose that read Shipping over a limb the FR marked unmet. Both were recorded rather than silently retagged, because retagging is a requirements change to make deliberately. **Discharged 2026-08-18:** the owner directed the retag and both now read Partial. One deliberate exception remains: FR-23 keeps its Shipping tag over its `Planned — D27–D30` routing bullet, because the owner directed that tag explicitly when the FR was authored (its constraint on what becomes runnable ships in full; the Planned bullet describes Epic UX-PB's future re-routing of the call site, not an unmet limb of the constraint). Reconciling FR-23's tag with this rule is an owner call, recorded here rather than made silently.

**A Reviewer Gate then ran against this pass and found six more defects, all corrected here.** Four were stale-baseline damage predating the Update: the PRD asserted the 4.5:1 contrast fix and its CI guard were uncommitted and absent from `HEAD`, when `a201fb0` had landed both and three further commits followed — FR-19, NFR-6 and §7.1 all said so, and all three are corrected, with the code baseline re-stamped from `5972109` to `1ac959e`; the D37 reconciliation queue named four artifacts when two were already done; FR-11 certified two identity-area elements that exist nowhere in `src/`; and **health fixes shipped with no requirement anywhere in §4** — now FR-23. Two came from the Update itself: RP-2 gained normative prose while tagged Shipping, and FR-6's evidence table over-read `upgradeAll()` as a divergent predicate when its Manager-wide scope is deliberate and self-consistent with the count its own label reports. The gate also refuted six findings, including the sharpest available criticism — that this PRD had become downstream of the architecture while claiming primacy — on the ground that this very section rules *against* an AD and routes the fix out.

**One divergence is recorded, not fixed, because the stale side is the spine's.** `ARCHITECTURE-SPINE.md` **AD-28's `Esc` rule** contradicts itself inside a single bullet. Its opening clause says "the cascade drops from three rungs to two and keeps close-dialog", and its own correction four lines later says "There is no surviving second rung: `prd.md` FR-6 called it close-drawer and the drawer retires with AD-17, so the cascade is close-dialog alone." The opening clause quotes FR-6's **pre-AD-17** wording; the correction is current and agrees with **AD-17's `Esc` rule** and with FR-6 as it now stands. **FR-6 is correct and stays as written.** The fix belongs to `bmad-architecture`, against AD-28's `Esc` rule by name — cited by subject rather than by line, because this document's own line-number citations into the spine have already rotted once and the spine's Citations convention forbids them.

---

### 9.3 Closed by `docs/DECISIONS.md` D39–D42 (owner decisions, 2026-08-18)

All four remaining open questions were resolved by dated owner decisions; per §0, a decision later than 2026-07-25 supersedes anything here. Recorded so no future reader re-litigates them. The recommendations were adversarially verified against `HEAD` before the owner accepted them.

**Q1 — Does a downloaded application update survive a relaunch that was not the update restart? CLOSED by D39: no, and that is the decided behavior, not a gap.** The payload is process-scoped memory (`app_update.rs` holds it in `AppUpdater.downloaded`, never on disk; a fresh process constructs at `Idle` with no restore path), so `Ready` is always re-derived by the live process. Recovery is the launch check and six-hour heartbeat while `autoCheckForUpdates` (default on) holds; with it off, the manual menu check — consistent with opting out, not a defect. RP-1 is unchanged: the saved trigger policy still survives a normal relaunch; only in-flight download state is ephemeral.

**Q2 — What is the first-run experience for a machine with none of the six Managers installed? CLOSED by D40, owner-modified.** No onboarding flow. Six muted `Not installed` cards, each carrying a copyable install hint — mas's treatment extended to all six — plus a disabled `Update Everything` and a Dashboard guidance panel that never reads `Warning`, because absence is not failure. An executing Install button is rejected by D40 (installer non-goal §6, SM-3, no shell surface, FR-23's closed immediate-execution set). Realized by **Story 2.5**, added to Epic 2 by `sprint-change-proposal-2026-08-18.md`; FR-1 now carries the Planned limb and is retagged Partial under §9.2's rule.

**Q3 — Can the user clear or delete History entries? CLOSED by D41: no.** No per-row delete, no Clear History, no retention knob; automatic compaction to the newest 1,000 records stays the only pruning. History rows are immutable evidence (D29) and failure legibility is a success metric (SM-4). The one genuine deletion motive — keeping an entry out of a shared diagnostics bundle — is a sharing concern answered by Q4's closure, never by destroying local evidence. Out-of-band deletion of `operations.jsonl` remains possible and unsupported: History and Interrupted reconstruction promise nothing after it, while the application itself still starts and contains the loss (NFR-2, AD-19).

**Q4 — Does the diagnostics export get a preview or redaction step before the user shares it? CLOSED by D42: no.** The guarantee is construction-time — closed allowlist, inherited environment excluded, symlink substitution rejected (FR-18, NFR-5) — and the product has no transmit path (§6). The review affordance is the local ZIP plus the visible timestamped path and success/failure both invocation points must show, which is Story 6.5's acceptance criterion rather than new work. The honest residue — the home path and command output the archive legitimately carries — is named rather than denied: FR-18's closed allowlist is what bounds it today, and AD-18's per-field disclosure review keeps any future field from widening it.

**FR-9 and FR-19 retagged to Partial (owner-directed, 2026-08-18).** §9.2 recorded both as Shipping over Planned limbs and instructed that the retag be made deliberately rather than silently; the owner so directed in this pass. FR-9's shipping substance (atomic admission, serialization, the cap, queue reasons, D22) and FR-19's (the current navigation model, palette, contrast guard, focus mechanism, floors) are unchanged — only the tags now follow §9.2's Partial-never-Shipping rule, with the Planned limbs' owning stories named inline. FR-23's Shipping tag stands as §9.2's one recorded exception, per the owner's original direction.

## 10. Review Record and Judgment Calls

No `[ASSUMPTION]` tags were needed. Every requirement in this document traces to a named source — `docs/SPEC.md`, `docs/DECISIONS.md` D1–D42, `epics.md` FR/NFR lines 53–450, `ARCHITECTURE-SPINE.md` revision 10, `EXPERIENCE.md`, `DESIGN.md`, or verification against `src/` and `src-tauri/`. Code claims were first verified at `5972109` and re-verified at `1ac959e` during the revision-10 Update pass; where the two disagree, `1ac959e` governs and §9.2 records what moved. The 2026-08-18 Update pass (§9.3, FR-1, FR-20) verified its code claims against that day's `HEAD`. Where a source was wrong, §0.1 records the correction rather than assuming past it. Where a source was silent, §9 records the gap rather than filling it.

This document went through a reviewer gate on 2026-07-25: seven reconcilers, one per source input, and six review lenses including the quality-rubric walker, with every finding facing an adversarial verifier required to open the cited file and default to refuting. The gate raised 62 findings across the review lenses alone, of which 53 were refuted. Its verdict was *Good*, with **downstream usability** the one thin dimension — the handoff, not the thinking. Everything it confirmed has been applied, including two criticals: FR-7 had removed a safety gate without carrying the three compensations that were the price of removing it, and FR-14 described a quit guard that is not wired to anything. Reviewer output is preserved in this folder as `reconcile-*.md` and `review-*.md`.

Three judgment calls are recorded here because a reviewer might reasonably have made them differently:

- **The §2.3 journey table is not a complete index of the FR set.** FR-12, FR-17, FR-19 and RP-2 are cross-cutting and map to no journey. Declaring that in one line was chosen over stretching journeys to cover requirements they do not drive.

- **Requirement IDs were preserved rather than renumbered.** `epics.md` and `ARCHITECTURE-SPINE.md` already cite FR-1…FR-22 and NFR-1…NFR-8; renumbering would have broken every downstream reference for cosmetic gain.
- **NFR-6's "deterministic dialog/sidecar focus restoration" was dropped** along with the keyboard obligations, on the grounds that it is a keyboard-focus concern. D37 protects the focus *indicator* by name and nothing else; a mouse user clicks the next thing they want. Confirmed by the owner on 2026-07-25: it stays dropped, and the PRD stays consistent with D37 rather than carving an exception.
