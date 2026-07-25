---
stepsCompleted:
  - step-01-document-discovery
  - step-02-prd-analysis
  - step-03-epic-coverage-validation
  - step-04-ux-alignment
  - step-05-epic-quality-review
  - step-06-final-assessment
verdict: 'NOT READY — 3 blockers, 2 requiring an owner decision'
documentsAssessed:
  requirements: 'docs/SPEC.md'
  decisions: 'docs/DECISIONS.md'
  architecture: '_bmad-output/planning-artifacts/architecture/architecture-Pack-Manager-2026-07-23/ARCHITECTURE-SPINE.md'
  ux:
    - '_bmad-output/planning-artifacts/ux-designs/ux-Pack-Manager-2026-07-23/DESIGN.md'
    - '_bmad-output/planning-artifacts/ux-designs/ux-Pack-Manager-2026-07-23/EXPERIENCE.md'
  epics: '_bmad-output/planning-artifacts/epics.md'
  sprintStatus: '_bmad-output/implementation-artifacts/sprint-status.yaml'
  releaseVerification: 'docs/RELEASE-CHECKLIST.md'
prdStatus: 'deliberately absent (D33) — not a gap'
nonAuthoritative:
  - '_bmad-output/archive/2026-07-24-scope-recalibration/**'
  - '_bmad-output/project-context.md (stale, regenerating separately)'
---

# Implementation Readiness Assessment Report

**Date:** 2026-07-25
**Project:** Pack-Manager

## Step 1: Document Discovery

Discovery ran the skill's four glob patterns against `{planning_artifacts}` =
`_bmad-output/planning-artifacts`.

### PRD Documents

**Whole:** none — `_bmad-output/planning-artifacts/*prd*` returns "no matches found".
**Sharded:** none. `find _bmad-output/planning-artifacts -iname "*prd*"` returns zero paths.

This is the intended state, not a gap. `docs/DECISIONS.md` D33 retired the PRD. The
absence of any `*prd*` path under `planning-artifacts` is itself the positive finding:
the retired PRD, its addendum, and `readiness-coverage-map.md` live only under
`_bmad-output/archive/2026-07-24-scope-recalibration/`, so no BMAD glob can reload the
retired 72-criterion gate. No remediation required or recommended.

### Architecture Documents

**Whole:** none at the `planning-artifacts` root.
**Sharded:** `architecture/architecture-Pack-Manager-2026-07-23/`
- `ARCHITECTURE-SPINE.md` (1052 lines) — authoritative, revision 9
- `DRIFT-NOTE.md` (774 lines)
- `.memlog.md` (append-only run log)
- `reviews/` — 16 review files (`review-currency{,-v6,-v8,-v9}.md`,
  `review-divergence{,-v6,-v8,-v9}.md`, `review-reconcile-{decisions,epics,epics-v6,ux,v8,v9}.md`,
  `review-rubric{,-v6,-v8,-v9}.md`)

No index.md; the folder is a run directory, not a sharded document. No whole-vs-sharded
duplicate. Note the `*epic*` and `*ux*` globs also match files inside `reviews/`
(`review-reconcile-epics.md`, `review-reconcile-epics-v6.md`, `review-reconcile-ux.md`) —
these are architecture review records, not epic or UX sources, and are excluded as inputs.

### Epics & Stories Documents

**Whole:** `epics.md` (1319 lines) — authoritative.
**Sharded:** none.

### UX Design Documents

**Whole:** none at the `planning-artifacts` root.
**Sharded:** `ux-designs/ux-Pack-Manager-2026-07-23/`
- `DESIGN.md` (252 lines) and `EXPERIENCE.md` (460 lines) — both authoritative
- `mockups/` — 4 HTML mockups; `validation-report.{md,html}`;
  `review-accessibility.md`; `review-usability.md`; `.memlog.md`
- `.working/` — direction explorations, Playwright captures, PNG previews (scratch)

No whole-vs-sharded duplicate.

### Additional planning-artifacts files (in scope as corroborating records)

- `story-triage-2026-07-24.md` (152 lines) — the Epic 1–6 triage
- `sprint-change-proposal-2026-07-25.md` (742 lines)
- `sprint-change-proposal-2026-07-25-spine-rev8.md` (654 lines)

### Issues Found

**Duplicates requiring resolution:** none. No document type exists in both whole and
sharded form.

**Missing documents:** PRD — deliberate per D33, see above. All other required document
types resolved to exactly one authoritative source.

**Flagged for verification in later steps:** two same-day sprint-change proposals
(`sprint-change-proposal-2026-07-25.md` and `-spine-rev8.md`) both carry the 2026-07-25
date stamp. Which one `epics.md` cites as authoritative is checked in Step 3, since the
`bmad-correct-course` date-stamped output path can collide within a single day.

### Documents confirmed for assessment

| Role | Path |
| --- | --- |
| Requirements | `docs/SPEC.md` (815 lines) |
| Decisions | `docs/DECISIONS.md` (486 lines, through D35) |
| Architecture | `.../architecture-Pack-Manager-2026-07-23/ARCHITECTURE-SPINE.md` (rev 9) |
| UX | `.../ux-Pack-Manager-2026-07-23/DESIGN.md` + `EXPERIENCE.md` |
| Epics & stories | `_bmad-output/planning-artifacts/epics.md` |
| Sprint status | `_bmad-output/implementation-artifacts/sprint-status.yaml` (118 lines) |
| Release verification | `docs/RELEASE-CHECKLIST.md` (113 lines) |

Excluded as non-authoritative: everything under
`_bmad-output/archive/2026-07-24-scope-recalibration/`, and `_bmad-output/project-context.md`
(known stale — asserts app version 1.0.0, CI on `macos-14`, and an unguarded
`install_app_update`; all three superseded. Being regenerated separately; not edited here).

## Step 2: Requirements Analysis

There is no PRD, by design (`docs/DECISIONS.md` D33). Requirements were extracted
from `docs/SPEC.md` (the requirements authority) and from the requirements
inventory `epics.md` carries at `### Requirements Inventory`, which is the live
restatement downstream stories cite.

### Functional Requirements — 22 FRs + 2 mandatory prerequisites

`epics.md:53-99` enumerates FR-1 … FR-22 plus RP-1 and RP-2. Counted:
`grep -c "^FR-[0-9]*: " epics.md` → 22 in the inventory; `grep -c "^RP-[0-9]: "` → 2.

Cross-referenced against `docs/SPEC.md` §1, which numbers its own features
F1 … F17 with priorities (P0 = MVP-required, P1 = ship-with polish, P2 = out of
scope). The two schemes are not 1:1 and neither document maps them to each other.
That is tolerable — `epics.md`'s FR ids are what stories cite — but it means
`docs/SPEC.md` F-numbers appear in no story and cannot be traced.

### Non-Functional Requirements — 8 NFRs

`epics.md:103-117` enumerates NFR-1 … NFR-8.

### PRD Completeness Assessment

The requirements set is complete, internally consistent in its own terms, and
unusually concrete (NFR-3 carries real numbers: 101 rows, 50 ms / 64 lines / 8 KiB,
5,000 retained lines, 900 × 600, 150–200 % zoom). The absence of a PRD is not a
gap and no recreation is recommended.

Two defects are in the requirements layer itself, not in the epics:

1. **`docs/SPEC.md` §5.9's IPC contract is behind `docs/SPEC.md` §1 and the spine.**
   `Settings` (`docs/SPEC.md:631-639`) declares `autoOpenDrawer: boolean` and no
   `skipUpgradePlanConfirmation`, while F11 (`docs/SPEC.md:112`) requires
   `skipUpgradePlanConfirmation` (default `false`) and demotes `autoOpenDrawer` to
   "inactive legacy input". `OpStatus` (`:501-508`) declares seven variants;
   AD-16 requires eleven. `OperationRecord` (`:600-614`) carries no
   `planAttemptId`; AD-18 requires it. `PlanRequest` (`:573-577`) still carries
   `includeSelfUpdates`, which F4 (`:71`) abolishes. The command list (`:468-485`)
   has 17 entries; production registers 20 (verified).
   §0.1 (`:43-46`) declares the *global self-update toggle* and `autoOpenDrawer`
   historical, so those two are covered by supersession. The **missing additions**
   are not covered by anything: a builder reading §5.9 as the contract targets a
   shape the architecture forbids.

2. **NFR-1 … NFR-8 are assigned to nothing.** See Step 3.

## Step 3: Epic Coverage Validation

### Structural parity — clean

- `epics.md` contains exactly 28 `### Story UX-PB.` headers.
- `sprint-status.yaml` contains exactly 28 `ux-pb-*` keys. Counts match.
- Six surviving Epic 1–6 stories (2.2, 3.1, 3.2, 3.4, 3.5, 6.5) appear in both.
- `epic-1`, `epic-4`, `epic-5` keys are absent from `sprint-status.yaml` — correct
  per D33, not corruption.
- 38 non-retrospective `backlog` entries = 4 epic keys + 28 + 6.
- No `*prd*` path, no `contracts/` directory, no scenario-contract or
  evidence-manifest file exists outside `_bmad-output/archive/` (verified). D33's
  retirement left nothing a BMAD glob could reload.

### FR Coverage — five FRs point at an archive while live stories build them

`epics.md:399-454` assigns each FR "exactly once to its primary epic". Eight FRs
are marked `Triaged out`. Five of those eight describe work that live Epic UX-PB
stories implement:

| FR | Marked | Actually built by |
| --- | --- | --- |
| FR-9 — "Admit a confirmed multi-group plan atomically" | `Triaged out (was Epic 5)` (`:424`) | UX-PB.2b |
| FR-11 — "…an action that adds independent removable Manager-update membership to the plan" | `Triaged out (was Epic 5)` (`:428`) | UX-PB.1e |
| FR-12 — "…no general shell, `sudo`, password…null stdin" | `Triaged out (was Epic 5)` (`:430`) | AD-4 floor; UX-PB-wide |
| FR-13 — "…queued, running, verifying, stalled, cancelling, and terminal plan state…correlated by `planAttemptId`…sidecar as live progress and Results; Activity a first-class destination" | `Triaged out (was Epic 5)` (`:432`) | UX-PB.3a, 3b, 3c |
| FR-14 — "…Cancel plan choices, trusted-only interaction classification…attempt-wide cancellation" | `Triaged out (was Epic 5)` (`:434`) | UX-PB.2e, 3f, 3g |

Two of these are additionally **self-contradicted inside the same document**:
FR-11 is `Triaged out` at `:428` yet listed as Epic 3 cross-cutting at `:477` and
claimed by Story 3.1's contract at `:1198`; FR-13 is `Triaged out` at `:432` yet
claimed by Story 3.5's contract at `:1270`.

FR-8 shows the correct handling and proves the omission was an oversight rather
than a policy: `:422` states "The stories that previously carried this (3.3, 3.6,
and 5.8) were archived on 2026-07-25, so Epic UX-PB now owns the whole
requirement… Epic 3 remains the nominal primary owner for map accounting; the
realizing stories are cross-cutting per this map's convention." That paragraph was
not written for the other five.

### FR traceability for the primary build queue is absent

26 of the 28 UX-PB stories cite no requirement at all. Only two do:
UX-PB.5b cites FR-17 (`:1051`) and UX-PB.5d cites FR-19 (`:1109`), both inside
their `Dependencies` line. The six Epic 1–6 survivors all carry a proper
`- FR and requirement links:` bullet (`:1164, :1198, :1222, :1245, :1270, :1300`).
So the 28 stories that constitute the whole live build have the weakest
requirements linkage in the document.

### NFR Coverage — no map exists

`epics.md` has a `### FR Coverage Map` and no NFR equivalent. None of NFR-1 … NFR-8
is assigned to an epic, a story, or the release checklist.

Much NFR substance is genuinely carried by architecture invariants — NFR-2 by
AD-25, NFR-4 by AD-18, NFR-5 by AD-4 and AD-18, NFR-6 by AD-27 and AD-11 — and
those bind every story whether cited or not. The exposed remainder is NFR-3's
concrete thresholds: "prove reachability and correct actions at 101 rows", the
50 ms / 64-line / 8 KiB flush, and the 5,000-line retention at 5,001. No story
owns any of the three. All three already ship (`LOG_CAP` in
`src/store/operations.ts`; virtualization at 100 rows in `PackageTable.tsx` and
200 in `LiveLogView.tsx`), so under AD-1 they are regression surfaces to preserve
rather than gaps to schedule — but nothing states that, which is the exact
"verify before scheduling a test gap" trap D33 warns about.

### RP-1 is assigned to a checklist that does not validate it

`epics.md:452` — "RP-1: Release checklist — Validate scheduled/menu update
triggers and state continuity." RP-1 itself (`:97`) requires: launch, six-hour and
app-menu update checks; in-process update state restored after supported UI
recreation; saved trigger policy preserved across relaunch; failed or interrupted
downloads never appearing Ready; and application-update state kept separate from
the Package queue and History. RP-1 calls itself "this mandatory prerequisite
validated through `docs/RELEASE-CHECKLIST.md`".

`docs/RELEASE-CHECKLIST.md` items 1–9 contain no step for the six-hour timer, the
app-menu `Check for Updates…` trigger, trigger-policy persistence across relaunch,
or "failed downloads never appear Ready". Item 8 covers install-and-relaunch, 8a
the non-writable/no-admin-prompt branch, 8b refusal while busy, 9 the Edit/Window
menus. Only RP-1's final clause is covered, and by a story (UX-PB.5e), not by the
checklist. RP-2 by contrast is genuinely covered by item 9.

### FR-19 is a build requirement routed to a validation surface

FR-19 (`:89`) requires "one coherent dark-only macOS interface across Dashboard,
**expandable Manager navigation**, Manager workspaces, persistent Upgrade Plan,
separate Confirmation Dialog, Activity, Results, one-plan-per-row History,
Settings, status, and app menus". D30 explicitly supersedes "D18's flat Manager
navigation", and `EXPERIENCE.md:46` specifies the replacement — "Disclosure
control for detected Managers; Collapsed by default; expanding reveals Managers".

The map assigns FR-19 to "Release checklist" (`:444`). Several of its clauses are
built by UX-PB stories (high zoom by 5d, Results by 3d, one-row-per-attempt
History by 4a). **No story builds the expandable Manager navigation.** A
validation surface cannot originate behavior.

## Step 4: UX Alignment

`DESIGN.md` (`status: final`) and `EXPERIENCE.md` (normative) are internally
coherent and are correctly treated as upstream: D35 adopted `DESIGN.md`'s palette
into `src/styles/theme.css` and gave focus its own token, and AD-27 fixes the
mechanism. `epics.md:330-346` restates the UX contract as binding. That chain is
sound.

Four surfaces the normative UX sources define have **no owning story**:

| Surface | `EXPERIENCE.md` | `DESIGN.md` | `epics.md` |
| --- | --- | --- | --- |
| Summary Card | 3 mentions | 2 | **0** |
| Brief Notifications | 2 | 1 | **0** |
| F6 region cycling (`EXPERIENCE.md:277`) | 2 | 0 | **0** |
| Health Meter | 2 | 3 | 1 — negation only |

Aggravating detail for two of them. `epics.md:344-345` names "Summary Cards"
itself as binding UX ("…Manager cards, Summary Cards, Package health,
update-ready presentation…follow `DESIGN.md`, `EXPERIENCE.md`, and
`validation-report.md`") while no story builds them. And AD-17
(`ARCHITECTURE-SPINE.md:617`) leans on Brief Notifications to keep the
announcement channel from double-narrating — "Brief Notifications suppress speech
the channel already emitted" — so an invariant depends on a surface no story
delivers.

The Health Meter appears in `epics.md` exactly once, at `:640`, and only as a
prohibition: UX-PB.1e requires a failed refresh to "use text rather than an
invented Health Meter value". The component `DESIGN.md` defines three times is
never built, only constrained.

### The selection model — the one true contradiction

See Step 5, Blocker 1. It is a UX-versus-SPEC conflict, and it is the finding that
should stop the build.

## Step 5: Epic Quality Review

### Blocker 1 — Transient selection and plan membership are two models; the first story in the queue picks one and Story 3.5 picks the other

Verbatim, from four sources:

- `EXPERIENCE.md:143` — "On eligible Package rows, **selection immediately
  adds/removes Upgrade Plan membership.**"
- `docs/SPEC.md:80` (F5) — "`Add N to Plan` immediately adds the checked canonical
  identities to the persistent plan **and clears the transient selection.**"
- `docs/SPEC.md:288` — "Esc **clear selection** / close sheet / close drawer".
- `src/store/packages.ts:17` — `selection: Partial<Record<ManagerId, Set<string>>>;`
  — a live transient selection set ships today.

Under `EXPERIENCE.md` the checkbox *is* plan membership and no transient selection
exists. Under `docs/SPEC.md` F5 the checkbox is a transient selection and
`Add N to Plan` is a separate transfer step. These are different data models, not
different wordings.

Both readings are now written into live stories:

- **UX-PB.1a** (`epics.md:522-523`) — "**When** I toggle its plan Checkbox by
  pointer, Enter/Space, or the grid Space key **Then** the Package's canonical
  identity is added to the one persistent draft Upgrade Plan" → the
  `EXPERIENCE.md` model.
- **Story 3.5** (`epics.md:1277-1280`) — "**When** toggle, shift-range, tri-state,
  Cmd+A, Space, Cmd-click, Clear, and Esc interactions execute **Then** the exact
  selectable identities and visible filter semantics are preserved **And** excluded
  rows never enter selection." → the F5 model, with `selection` intact.

And they are coupled: UX-PB.1a declares "**Blocks:** UX-PB.1b, UX-PB.1c; **Story
3.5**" (`epics.md:515`).

The spine has already found this and declined to fix it. `ARCHITECTURE-SPINE.md:1050`,
row *Transient selection has no owning invariant*, status **"OPEN — new
architecture, not this run's scope"**: "No `AD` models the relationship between
transient row selection and `PlanIntent` membership, and the two driving sources
answer it oppositely… Under the `EXPERIENCE.md` reading, Esc would mass-write
AD-23 tombstones; under the `SPEC.md` reading it writes nothing. Story 3.5
(keyboard selection) and UX-PB.1a (staging) can each obey every existing `AD` and
still build opposite models… so it goes to the owner as a decision."

**Why this blocks:** UX-PB.1a is the first story in the primary build queue, and
it is where the choice gets frozen into the Rust `PlanIntent` and the Zustand
projection. Deciding after 1a lands means reworking the domain model plus every
story built on it. This needs an owner decision and a new invariant, not a
document edit.

### Blocker 2 — `OpStatus` grows from 7 to 11 durable wire variants across four stories in three waves, with no single owner

AD-16 makes all four new states durable wire-level states, each an atomic
contract change:

- `ARCHITECTURE-SPINE.md:412-417` — "`Verifying` and `Skipped` are durable
  wire-level operation states, not presentation states derived in React… Adding
  them is one atomic contract change under AD-3."
- `ARCHITECTURE-SPINE.md:418-425` — "The same answer governs every new operation
  state the UX-PB stories introduce, not only those two. `Cancelling` and
  `Interaction required` are durable wire-level states on `OpStatus` as well…
  **`OpStatus` ships seven variants today**, so every addition moves as one atomic
  AD-3 change across the Rust enum, `src/lib/ipc/types.ts`, the guards, and
  `dev/fixtures/ipc/*.json`."

In `epics.md`, only one story acknowledges this, and only for two of the four
states: UX-PB.3c's `Dependencies` reads "AD-16 (`Verifying`/`Skipped` as durable
wire states)" (`:805`). The other two states are set by stories that never call
them wire states:

- `Cancelling` — UX-PB.**2e** (`:740`) and UX-PB.**3g** (`:903`), waves 2 and 3.
- `Interaction required` — UX-PB.**3f**, never described as a wire state anywhere
  in `epics.md`.

So four stories across three waves each independently mutate the same Rust enum,
the same `types.ts` union, the same guard map, and the same 15 committed contract
fixtures. Whichever lands first breaks the other three's fixtures.

AD-23 solved this exact hazard for `PlanIntent` with an explicit tie-break
(`ARCHITECTURE-SPINE.md:790-793`): "This shape is one atomic surface change under
AD-3… UX-PB.1a and UX-PB.1c may not land it independently; **whichever runs first
lands the complete shape and the other builds against it.**" AD-16 has no
equivalent sentence for `OpStatus`. It should, and the same wording works.

Compounding: `docs/SPEC.md:501-508` still declares the seven-variant enum, so the
requirements document and the architecture disagree on the target shape (Step 2,
defect 1).

### Blocker 3 — Waves 2 and 3 have acceptance criteria that require the wave-5 confirmation gate

- **UX-PB.2b** (`epics.md:672`) — "**When** I invoke the confirmed run action
  (`Confirm N Updates`, or the confirmation-off run action) and admission
  succeeds…". `Confirm N Updates` and its dialog are delivered by **UX-PB.5a**;
  the confirmation-off `Run N Updates` action by **UX-PB.5c**.
- **UX-PB.3a** (`epics.md:777`) — "**When** final confirmation closes the
  Confirmation Dialog **Then** the same Upgrade Sidecar transforms in place…".
  The Confirmation Dialog is **UX-PB.5a**.
- **UX-PB.5a** (`epics.md:1021`) — "**Dependencies:** UX-PB.1 and UX-PB.2
  complete".

So 5a waits on wave 2, while 2b's and 3a's acceptance criteria depend on 5a's
artifacts. Neither 2b nor 3a lists 5a as a dependency, and nothing sequences 5a
ahead of 3a — both merely require "UX-PB.2 complete", so a scheduler reading the
declared dependencies may run 3a first.

UX-PB.2b is survivable in practice: it is backend admission work and can be driven
below the UI. UX-PB.3a is not — it is a UI story whose trigger does not exist yet.
Fix by resequencing 5a ahead of 3a, or by rewording the two criteria to name a
provisional confirm entry point.

### Medium — one dependency edge is asserted from one side only

- UX-PB.**1b** (`epics.md:546`) — "**Blocks:** UX-PB.1d, UX-PB.1e".
- UX-PB.**1d** (`:599`) — "**Dependencies:** UX-PB.1a, UX-PB.1c" — no 1b.
- UX-PB.**1e** (`:622`) — "**Dependencies:** UX-PB.1c" — no 1b.

UX-PB.1c makes the identical `Blocks` claim and *is* reciprocated by both. So the
defect is specific to 1b. Practical impact is limited because UX-PB.2a requires
"UX-PB.1 complete (PB.1a-e)", so all of wave 1 precedes wave 2 regardless — but
intra-wave ordering is unenforced.

Also cosmetic: 25 `**Blocks:**` lines for 28 stories. UX-PB.4e, 5d and 5e have
none.

### Medium — story-level fix for an architecture-level gap, self-declared

UX-PB.3d's `Dependencies` (`epics.md:827`) says: "AD-18 (the plan-attempt
journal's home, format and durability — **note AD-18 does not itself name a writer
or a record cardinality, so the terminal-write ownership below is stated here and
belongs in AD-18 when it is next amended**)".

`epics.md` then resolves it correctly in story prose (`:849`): UX-PB.4a owns the
single terminal write, UX-PB.2c writes the admission record, "exactly one
**terminal** record exists per `planAttemptId`". The spine confirms the underlying
gap is still open — `ARCHITECTURE-SPINE.md:1051`, *Plan-attempt journal: writer
identity and record cardinality*, **"OPEN — new architecture, not this run's
scope"**.

So the contradiction between 3d and 4a is resolved for a builder reading either
story, but the invariant lives in one story's dependency note instead of in AD-18.
Lower severity than the spine's OPEN status implies, because the two stories now
agree; it should be folded into AD-18 rather than left as prose.

### Medium — deferred infrastructure with four live consumers

`ARCHITECTURE-SPINE.md:1036`, *Crash/relaunch lifecycle controller*, status
**"Deferred (live consumers)"**: "UX-PB.1b, UX-PB.2f, UX-PB.4e, and Story 6.5 each
assert crash, force-quit, or relaunch behavior, so the earlier 'no live story
requires one' premise was false. AD-5 binds whoever builds it; until it exists
those stories own their own disposable-root setup and may not resolve a production
directory by fallback."

Honestly recorded and given an interim rule, so it is not a blocker — but three
UX-PB stories will each build their own crash-relaunch scaffolding, and nothing
nominates one of them to generalize it.

### Low — the spine's own residual row is stale; all five items are already fixed

`ARCHITECTURE-SPINE.md:1049`, *`epics.md` residuals for the next
`bmad-correct-course` run*, status **"OPEN — record only; do not edit `epics.md`
here"**, lists five items. Every one is now satisfied:

| Spine residual | State in `epics.md` |
| --- | --- |
| (1) "UX-PB.3d cites AD-25 but never states it" | **Stated** at `:845` — "the Manager's Last-good Snapshot is left in place with its timestamp (AD-25) — a verification refresh that errors or times out never replaces or clears the snapshot it failed to refresh" |
| (2) "AD-21's substance never reaches criterion text" | **In criterion text** at `:1066` — "`skipUpgradePlanConfirmation` is plan-inert (AD-21) — it is not a plan-determining input, so writing it never advances the canonical revision…" |
| (3) register "still lists the canonical design-token set as `OPEN`… Blocks UX-PB.1e and UX-PB.5d" | **`CLOSED` — D35**, "Nothing blocked" at `:308` |
| (4) register "still records the `notarytool minos 15.0` question as OPEN" | **`CLOSED` by D34** at `:309` |
| (5) "**AD-27 is cited nowhere**" | **32 occurrences** in `epics.md` |

Harmless to the build, but a reader following the spine would dispatch a
correct-course run to re-fix five fixed things.

### Low — `epics.md` provenance understates and misattributes its own currency

- `epics.md:43` — "this document was reconciled with `ARCHITECTURE-SPINE.md`
  **revision 6**". The spine is `artifact_revision: 9`. `epics.md` has plainly
  absorbed revision 8 (it cites AD-21 through AD-26) and revision 9 (32 AD-27
  citations), so the header understates the document.
- `epics.md:14` lists `sprint-change-proposal-2026-07-25.md` as an input. The
  spine (`:1047`) records the batch that produced `epics.md`'s current story text
  as landing "under `sprint-change-proposal-2026-07-25-spine-rev8.md`" (commit
  `8d36cdf`). Both proposals carry the same `2026-07-25` date stamp, and
  `epics.md`'s provenance omits the one that shaped it.
- `epics.md:264` — "shipped in v1.0.0". Current version is 1.0.1 (verified in
  lockstep across all five release-please files). True of D31's original ship, so
  cosmetic.

### Low — 26 unresolved reviewer findings, 6 of them HIGH

`ARCHITECTURE-SPINE.md:1046`, *Reviewer-gate tail (revision 6)*, status **"Open"**:
44 findings originally (5 CRITICAL, 14 HIGH, 18 MEDIUM, 7 LOW); after revisions 7
and 8 the remaining tail is "**6 HIGH, 15 MEDIUM, 5 LOW** across
`reviews/review-divergence-v6.md`, `review-rubric-v6.md`,
`review-reconcile-epics-v6.md`, and `review-currency-v6.md`. Each finding names its
own affected stories." None is triaged into the build queue.

### What is genuinely sound

Worth stating plainly, because most of the stack is in good shape:

- **The deliberate absences are clean.** No `*prd*` path exists anywhere under
  `planning-artifacts` (the glob returns "no matches found"); no `contracts/`
  directory, scenario-contract, or evidence-manifest file exists outside
  `_bmad-output/archive/`. D33's retirement left nothing a BMAD glob can reload.
- **The spine's Verified Brownfield Baseline is accurate on every claim checked** —
  20 registered commands and 6 events, `macos-15` on all three pins, version 1.0.1
  in lockstep across five files, zero occurrences of `planAttemptId`,
  `Verifying`, `InteractionRequired` or `skipUpgradePlanConfirmation` (including
  snake_case variants) in `src/` and `src-tauri/src/`, `autoOpenDrawer` still
  active with 13 references, and the `install_app_update` guard live at
  `src-tauri/src/commands.rs:810`. An architecture document whose factual claims
  survive independent verification is the strongest signal here.
- **Story quality is high.** All 28 UX-PB stories carry explicit Given/When/Then
  happy- **and** failure-path criteria, including crash, partial-failure,
  persistence-failure, and reconnect paths. AD-21 through AD-27 were each written
  to close a specific two-story divergence, and they do.
- **`sprint-status.yaml` is a faithful projection of `epics.md`** with no key drift.
- **`project-context.md` is stale exactly as briefed** (version 1.0.0, `macos-14`,
  unguarded `install_app_update` — all three superseded) and was correctly
  excluded, not edited.

## Step 6: Final Assessment

**Verdict: NOT READY to start Epic UX-PB as written. Three items must close first;
two of them require an owner decision, not a document edit.**

The planning stack is far better than typical — the architecture spine is
verified against the tree, the stories are unusually rigorous, and the D33 rescope
was executed cleanly with no archive leakage. The gap is narrow and concentrated,
not systemic.

**Must close before UX-PB.1a starts:**

1. **Decide the selection model** (Blocker 1). `EXPERIENCE.md:143` and
   `docs/SPEC.md:80`/`:288` specify incompatible models; UX-PB.1a and Story 3.5
   each implement one; `src/store/packages.ts:17` ships the SPEC one today. The
   spine routes this to the owner as new architecture. UX-PB.1a freezes the
   `PlanIntent` domain model, so deciding later means reworking it and everything
   above it.
2. **Assign the `OpStatus` expansion to one story** (Blocker 2). Add AD-23's
   tie-break sentence to AD-16: whichever of UX-PB.2e / 3c / 3f / 3g runs first
   lands all four variants (`Verifying`, `Skipped`, `Cancelling`,
   `Interaction required`) as one atomic AD-3 change, and the others build against
   it. Update `docs/SPEC.md:501-508` in the same change.
3. **Sequence UX-PB.5a ahead of UX-PB.3a**, or reword the criteria at
   `epics.md:672` and `:777` to name a provisional confirm entry point
   (Blocker 3).

**Should close in parallel — traceability, not blocking:**

4. Reassign FR-9, FR-11, FR-12, FR-13, FR-14 from `Triaged out` to their realizing
   UX-PB stories, using FR-8's existing paragraph (`epics.md:422`) as the template;
   resolve the FR-11 and FR-13 self-contradictions at `:428`/`:477`/`:1198` and
   `:432`/`:1270`.
5. Add FR links to the 26 UX-PB stories that have none.
6. Add an NFR coverage map; state explicitly that NFR-3's 101-row, flush-bound and
   5,000-line thresholds are shipping regression surfaces under AD-1, not gaps.
7. Either add RP-1 steps to `docs/RELEASE-CHECKLIST.md` or reassign RP-1 to a
   story. As written it validates itself through a checklist that has no step for it.
8. Give "expandable Manager navigation" (FR-19, `EXPERIENCE.md:46`, required by
   D30's supersession of D18) an owning story.
9. Decide whether Summary Cards, Brief Notifications, F6 region cycling and the
   Health Meter are in scope. AD-17 depends on Brief Notifications; `epics.md:344`
   declares Summary Cards binding; neither is built.
10. Fold UX-PB.3d's terminal-write ownership note into AD-18.
11. Refresh `ARCHITECTURE-SPINE.md:1049` — all five residuals are fixed — and
    correct `epics.md:43` to revision 9 plus its missing `-spine-rev8` provenance.
12. Triage the 6 HIGH reviewer-gate findings still open at
    `ARCHITECTURE-SPINE.md:1046`.

**Not gaps, and not to be "fixed":** the missing PRD (D33), the missing
`contracts/` directory (D33), the missing `epic-1`/`epic-4`/`epic-5` keys (D33),
and `_bmad-output/project-context.md`'s stale claims (regenerating separately).
