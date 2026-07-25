---
title: Sprint Change Proposal — reconcile `epics.md` with ARCHITECTURE-SPINE revision 8
date: "2026-07-25"
workflow: bmad-correct-course
mode: batch
project: Pack-Manager
scope_classification: Minor (direct adjustment)
status: APPROVED AND APPLIED
supersedes: nothing
companion_to: sprint-change-proposal-2026-07-25.md
artifacts_modified:
  - _bmad-output/planning-artifacts/epics.md
authorities:
  - _bmad-output/planning-artifacts/architecture/architecture-Pack-Manager-2026-07-23/ARCHITECTURE-SPINE.md (revision 8)
  - _bmad-output/planning-artifacts/architecture/architecture-Pack-Manager-2026-07-23/DRIFT-NOTE.md
  - docs/SPEC.md
  - docs/DECISIONS.md
  - docs/RELEASE-CHECKLIST.md
---

# Sprint Change Proposal — `epics.md` ↔ Architecture Spine revision 8

**Not the same document as `sprint-change-proposal-2026-07-25.md`.** That one records the
revision-7 reconciliation, which retired the readiness-gate register inside `epics.md`, and
it is cited as a live input at `epics.md:14` and `epics.md:47`. It was deliberately not
overwritten. This is a separate, later pass against revision 8.

---

## 1. Issue Summary

`ARCHITECTURE-SPINE.md` reached **revision 8** on 2026-07-25. Across revisions 6, 7, and 8
it took explicit positions — six of them written as brand-new invariants, **AD-21** through
**AD-26** — that seven passages in `_bmad-output/planning-artifacts/epics.md` still
contradicted.

`epics.md` is the file `bmad-create-story` reads. Left alone, each contradiction would have
been copied verbatim into a UX-PB story context file and built.

### How the issue was discovered

Not by this run. The spine recorded its own divergences rather than editing `epics.md`,
by owner instruction, in a Decision Status row titled:

> `epics.md` divergence batch for `bmad-correct-course`

`ARCHITECTURE-SPINE.md:966` carries that row with items **(a)** through **(g)**, each
naming the exact story, the exact offending clause, and the AD that overrides it.
`DRIFT-NOTE.md:476-486` records the same decision from the architecture side:

> `epics.md` was not touched, by owner instruction — a single `bmad-correct-course`
> run will batch it after this revision lands.

### Evidence the divergences were real

Each was independently confirmed by the revision-8 reviewer gate before this run started.
`reviews/review-reconcile-v8.md:416-419`:

> The row is `ARCHITECTURE-SPINE.md:938`. **As finally reviewed, all six items are
> accurate and none understates its text.**

and `:430`:

> **No `epics.md` divergence was found outside the row.**

Three of the seven items exist *because* that gate corrected the row itself mid-review:
item (b) was widened from the conversion clause to the seed clause as well (H-5), item (d)
from one location to four (M-6), item (e) from "a missing case" to "states the *reverse* of
AD-22's ordering" (H-4). Item (f) was added outright (H-3), and item (g) was added by the
verification pass (`review-reconcile-v8.md:622`).

### Issue category

**Misunderstanding of original requirements — upstream, not downstream.** No implementation
work is affected because none has started: the Implementation-Entry Register carries
UX-PB.1–UX-PB.5 as `APPROVED TARGET — NOT IMPLEMENTED`.

---

## 2. Impact Analysis

### Epic impact — none structural

| Question | Answer |
| --- | --- |
| Can Epic UX-PB still complete as planned? | Yes. All 28 stories survive with their identity, order, dependencies, and blocking graph unchanged. |
| Epics added / removed / renumbered? | None. |
| Stories added / removed / resequenced? | None. |
| Epic priority or ordering changed? | No. Epic UX-PB remains the primary build queue and still runs first. |
| Future epics invalidated or newly required? | No. |

### Story impact — nine stories, criteria text only

| Story | What changed | Batch item |
| --- | --- | --- |
| UX-PB.1b | Crash/relaunch recovery criterion restated to the single fail-to-empty branch | (a) |
| UX-PB.1c | `Update Everything` seed + removal-conversion criterion restated; `AD-23` added to Dependencies | (b) |
| UX-PB.4b | Read-only replay criterion gains an explicit carve-out for the non-executing Retry affordance; `AD-24` added to Dependencies | (g) |
| UX-PB.4d | `Create new plan` restated as a derived `RetryIntent`; `AD-24` added to Dependencies | (c) |
| UX-PB.5b | Ordering inverted to admit-then-persist; rejected-admission criterion added; `AD-21` and `AD-22` added to Dependencies | (e) |
| Story 6.5 | Test level marked satisfiable, `AD-26` added to Governing invariants, new harness-constraint contract bullet | (d) |

Three further stories are *cited* by the new text but were not edited: UX-PB.1a (the
`PlanIntent` shape AD-23 fixes), UX-PB.2b (admission), UX-PB.5c (the confirmation-off path).

### Artifact conflicts

| Artifact | Conflict | Action |
| --- | --- | --- |
| **PRD** | Does not exist — retired in the D33 rescope; `project-context.md:86` excludes it from the authority list | None. `docs/SPEC.md` fills the slot and needs no change. |
| **`ARCHITECTURE-SPINE.md`** | Is the *source* of these edits | Out of scope by instruction. Not hand-edited. |
| **`docs/SPEC.md`** | None found | Unchanged. `DRIFT-NOTE.md:179` — "`docs/SPEC.md` is unchanged." |
| **`docs/DECISIONS.md`** | None. D33's own text at `:339-342` never claimed the checks were absent — that claim was `epics.md`'s own gloss. | Unchanged. |
| **`docs/RELEASE-CHECKLIST.md`** | Already corrected (commit `19e1b33`); `:91-93` now states reduced motion is automated | Unchanged. `epics.md` was the last document still wrong on it. |
| **`DESIGN.md` / `EXPERIENCE.md`** | None with the applied text | Unchanged. The design-token conflict is a separate OPEN row owned by `bmad-ux`. |
| **`sprint-status.yaml`** | No epic or story membership change | Untouched. |
| **Code, CI, infrastructure** | None — no implementation exists yet for the affected stories | None. |

### Technical impact

None to shipping code. Two of the restatements do, however, change what a builder will
*write* when these stories are implemented, and both were already verified as live defects
in the shipping tree by the spine's own gate:

- **(e)** — `src-tauri/src/commands.rs` `set_settings_core` bumps the canonical revision
  unconditionally for every key, and the execute path rejects on
  `issued.revision != coordinator.revision()`. Under the old criterion order the safety
  opt-out deterministically failed the run it rode on.
- **(d)** — both reveal paths are un-ported direct OS calls
  (`reveal_item_in_dir`, `open_path`), which Story 6.5 must port under AD-4.

---

## 3. Recommended Approach

**Option 1 — Direct Adjustment.** Effort: **Low.** Risk: **Low.** Timeline impact: **none.**

### Why the alternatives were not viable

| Option | Verdict | Reason |
| --- | --- | --- |
| **2 — Rollback** | Not viable | There is nothing to roll back. No UX-PB story has been implemented; the register carries them as `APPROVED TARGET — NOT IMPLEMENTED`. |
| **3 — MVP review** | Not viable | The MVP was already re-scoped by D33 and is unaffected. No requirement is added, removed, or reduced: the FR-1..FR-22 / NFR-1..NFR-8 inventory is byte-identical after this pass. |

### Rationale

Every one of the seven items had its answer already written by an authority upstream of
`epics.md`. The spine did not merely flag a problem — for (a), (b), (c), (e) and (g) it
stated what the restated criterion must say. This run is therefore transcription against a
decided contract, not a fresh decision, which is what makes it Low-risk. The one place
judgment was exercised is recorded in §4 under item (g).

**Constraint lifted for this run.** Editing acceptance criteria inside the 28 UX-PB stories
was the constraint that blocked the previous pass. It was lifted by the owner for exactly
items (a), (b), (c), (e), and (g). No story text the batch row does not name was touched.

---

## 4. Detailed Change Proposals

All fifteen edits applied to `_bmad-output/planning-artifacts/epics.md`.
`git diff --stat`: **1 file changed, 33 insertions(+), 20 deletions(-)**.

### (a) UX-PB.1b — crash/relaunch recovery criterion

**OLD**

> **Given** an in-progress draft when the app crashes or is force-quit
> **When** Pack-Manager relaunches
> **Then** the draft's canonical membership is reconstructed into the sidecar, or — if it cannot be recovered — the sidecar returns to empty with no fabricated membership and nothing executes; a draft is never surfaced as Activity or History.

**NEW**

> **Given** an in-progress draft when Pack-Manager is quit cleanly, crashes, or is force-quit
> **When** Pack-Manager relaunches
> **Then** it starts with an empty draft and a hidden sidecar — the draft is session-scoped and never written to disk, so membership is never reconstructed, never partially restored, and never fabricated, and nothing executes on relaunch
> **And** a draft is never surfaced as Activity or History.

**Rationale.** `ARCHITECTURE-SPINE.md:520-523` (AD-17):

> The draft is session-scoped and is never written to disk. Every relaunch — after a clean
> quit, a crash, or a force-quit — starts with an empty draft and a hidden sidecar. This
> takes the second branch of UX-PB.1b's recovery criterion unconditionally: membership is
> never reconstructed, never partially restored, and never fabricated, and nothing executes
> on relaunch.

The disjunction was the defect. `DRIFT-NOTE.md:82-85` records that UX-PB.1a could have built
a Zustand-only draft UX-PB.1b then could not reconstruct — two stories obeying every AD and
building incompatibly. Carried unresolved since revision 7.

### (b) UX-PB.1c — the `Update Everything` seed and the removal conversion

**OLD**

> **Given** a draft seeded by `Update Everything` as an `AllEligible` intent
> **When** I remove any item
> **Then** the draft converts to an `Explicit` intent of the surviving PackageRefs and Manager self-update identities and rebuilds the authenticated preview from the backend, never from edited display text.

**NEW**

> **Given** a draft seeded by `Update Everything`, whose expansion was frozen into concrete members at the moment I invoked it — each carrying `Bulk { scope: Everything }` provenance that is never re-evaluated
> **When** I remove any item
> **Then** that one member leaves the draft and a tombstone records the removal, so no later bulk expansion of any scope re-adds it; the surviving PackageRefs and Manager self-update identities keep their own per-member provenance, and Rust rebuilds the authenticated preview from those canonical identities, never from edited display text
> **And** no whole-intent `kind` is stored or converted — there is no `AllEligible` value to convert from and no `Explicit` value to convert to; a kind, where shown, is derived from member origins.

Dependencies also gained `AD-23 (per-member provenance and tombstones)`.

**Rationale — both halves, per the row.** The *seed*: `ARCHITECTURE-SPINE.md:342-344` (AD-16)

> A bulk mutation freezes its expansion into concrete members at the moment it is made —
> the scope predicate never runs a second time

The *conversion*: `ARCHITECTURE-SPINE.md:752-755` (AD-23)

> No durable intent-level `kind` field exists. Where a kind is displayed or explained it is
> derived … Nothing converts one kind into another, because there is no whole-intent value
> left to convert.

and `:742-745`, which is where the surviving observable outcome now lives:

> Removal writes a tombstone on the intent. A later bulk expansion of any scope does not
> re-add a tombstoned ref — a member list can record presence but not a deliberate absence

### (c) UX-PB.4d — "a new reviewable draft"

**OLD**

> … `Create new plan` **rebuilds current canonical intent into a new reviewable draft**, and confirming that draft creates a new attempt with a fresh `planAttemptId` linked by `retryOfPlanAttemptId` …

**NEW**

> … `Create new plan` **composes a derived `RetryIntent` in Rust — the source attempt's reviewed intent restricted to its failed members, canonically rebuilt against current eligibility and argv — and takes that separate reviewable object straight to preview and confirmation without ever writing to, merging with, or emptying the one persistent draft**, and confirming it creates a new attempt with a fresh `planAttemptId` linked by `retryOfPlanAttemptId` …

Dependencies gained `AD-24 (derived RetryIntent; the persistent draft has exactly one author)`.

**Rationale.** `ARCHITECTURE-SPINE.md:773-777` (AD-24):

> Retry does not write the draft. A retry scope is a **derived intent** — composed in Rust
> from the failed attempt's reviewed intent restricted to its failed members, canonically
> rebuilt against current eligibility and argv, and taken directly to preview and
> confirmation. It is a separate reviewable object and never merges with the persistent
> draft in either direction.

The old wording rested the entire distinction on an indefinite article. "no draft membership
can reach it" is what makes the `retryOfPlanAttemptId` lineage claim true by construction
rather than by each story remembering to filter (`:788-791`). The new text also states the
consequence AD-24 needed: a confirmed retry does **not** empty the draft (`:771-772`).

### (d) The native harness — four locations, now all citing AD-26

The spine no longer treats this as a deferral. `ARCHITECTURE-SPINE.md:953`:

> **OPEN — owner Story 6.5; shape named, not yet adopted** … No longer a premise-free
> deferral. … **AD-26** now governs.

**(d1) `epics.md:170-171`** — the narrative line that attributed the deferral to the spine:

> **OLD** … The native Tauri harness is Deferred **there**, with Story 6.5 as its only live consumer.
> **NEW** … Proving delivery waits on the native Tauri harness, which `ARCHITECTURE-SPINE.md` records as **OPEN — owner Story 6.5; shape named, not yet adopted**, not as a bare deferral. **AD-26** governs it, because the macOS route runs an embedded WebDriver server *inside* the application. A compliant shape exists — the automation surface excluded from release bits at compile time — so Story 6.5 is buildable; what remains open is the adoption itself, an AD-20 security-reviewed change.

**(d2) `epics.md:280`** — Governance and Risks row:

> **OLD** AD-3 (committed contract fixtures; delivery coverage explicitly unproven and **awaiting the deferred native harness**)
> **NEW** AD-3 (committed contract fixtures; delivery coverage explicitly unproven) and AD-26 (a native automation surface never reaches release bits — the harness that would prove delivery is OPEN with Story 6.5 as its owner, not deferred)

**(d3) `epics.md:308`** — Implementation-Entry Register row. Status `DEFERRED` →
`OPEN — owner Story 6.5; shape named, not yet adopted`; the effect column now names the
compliant shape, the compile-time gate, AD-2/AD-3/AD-26, the AD-20 review, and the paid-key
alternative.

**(d4) `epics.md:1287-1290`** — Story 6.5's own contract. Test level annotated *"Satisfiable
as written — AD-26 names a compliant shape and no renegotiation is needed"*; Governing
invariants gained `AD-26`; a new **Harness constraint (AD-26)** bullet carries the
compile-time exclusion and the production-composition requirement.

**Rationale, and the three external claims.** `ARCHITECTURE-SPINE.md:831-841` (AD-26) sources
the route question to `tauri.app/llms-full.txt` verified 2026-07-25 — `tauri-driver` driven
directly does not cover macOS, `@wdio/tauri-service` does via an embedded WebDriver server
inside the app. `:842-853` gives the compile-time gate (`#[cfg(debug_assertions)]`) and notes
`src-tauri/Cargo.toml` declares no `[profile.release]`, so the gate holds today. **The owner
independently confirmed all three external claims against Tauri's own documentation before
approving (d3)**, including the CrabNebula fork's paid macOS API key.

`DRIFT-NOTE.md:464-474` is worth reading alongside this item: AD-26's *first* draft quoted a
sentence that does not exist in the cited source, inverted the conclusion, and turned a
solvable adoption question into a fake blocker. The corrected version is what `epics.md` now
cites.

### (e) UX-PB.5b — the ordering was inverted, and the rejected case was missing

**OLD**

> **When** I choose the final `Confirm N Updates`
> **Then** `skipUpgradePlanConfirmation: true` is written atomically, **the new value takes effect only after persistence succeeds, and the plan is admitted.**

**NEW — criterion restated**

> **When** I choose the final `Confirm N Updates` **and admission succeeds**
> **Then** the ordering is validate, admit through the scheduler's revision-checked transaction, then persist the rider once admission has returned — `skipUpgradePlanConfirmation: true` is written atomically **only after** the plan is admitted, and it becomes active only after that write succeeds
> **And** the opt-out never precedes the admission it rides on; if that atomic save then fails, the admitted attempt stands, the prior `false` preference is retained as both active and persisted state, and the failure is surfaced inline.

**NEW — rejected-admission case added**

> **Given** the dialog with `Disable upgrade plan command execution confirmation` selected
> **When** I choose the final `Confirm N Updates` **and admission is rejected**
> **Then** nothing is persisted and nothing becomes active — the confirmation gate stays armed for a run I never got — and the dialog retains my selection so the choice is not silently lost.

Dependencies gained `AD-21 (skipUpgradePlanConfirmation is declared plan-inert)` and
`AD-22 (admit, then persist the rider)`.

**Rationale.** `ARCHITECTURE-SPINE.md:709-714` (AD-22) states the override in the spine's own
voice rather than leaving it implicit:

> Ordering is fixed — validate, admit through the scheduler's revision-checked transaction,
> then persist the rider once the admission has returned. A rider never precedes the
> admission it rides on. **This deliberately overrides UX-PB.5b's stated clause order**
> (persist, activate, then admit)

and `:715-722` gives the asymmetry the two new clauses encode:

> an unsaved opt-out costs one extra confirmation, while a saved opt-out on a refused run
> removes the gate from a run the user never got.

**AD-21 was added to Dependencies at the owner's explicit direction**, beyond what the batch
row named. Without it a story author reads AD-19's "a settings patch is persisted before it
becomes active … or advances the canonical revision" and rebuilds the original deadlock;
AD-21 is what declares `skipUpgradePlanConfirmation` plan-inert so the write cannot expire
the preview it rides on.

### (f) The accessibility claim, in two places

**(f1) `epics.md:264`**, the D33 restatement's final sentence:

> **OLD** Neither automated check exists yet, so this is an obligation on whichever story adds them, not a description of current coverage.
> **NEW** Reduced motion is already covered and runs in CI — `src/styles/theme.css` honors `@media (prefers-reduced-motion: reduce)` and `tests/e2e/browser-style-contract.spec.ts` emulates `{ reducedMotion: "reduce" }` and asserts transitions and animations resolve to `0s`, on every push and pull request to `main` via `.github/workflows/test.yml`. Automated 4.5:1 contrast does **not** exist; that same spec disclaims it. Contrast is therefore the outstanding obligation on whichever story adds it, while reduced motion is a regression surface to preserve, not a gap to schedule (`ARCHITECTURE-SPINE.md` AD-1, AD-11).

**(f2) `epics.md:308`**, the Implementation-Entry Register DR-2 row:

> **OLD** None. An obligation on whichever story adds the two automated checks, which do not exist yet.
> **NEW** None. Reduced motion is already automated and runs in CI (`tests/e2e/browser-style-contract.spec.ts` via `.github/workflows/test.yml`); automated 4.5:1 contrast does not exist and is the one outstanding obligation, on whichever story adds it.

**Rationale, verified against the tree this run rather than taken on authority:**

| Claim | Verified at |
| --- | --- |
| The product honors reduced motion | `src/styles/theme.css:51-55` — `@media (prefers-reduced-motion: reduce) { *, *::before, *::after { transition: none !important; animation: none !important; } }` |
| A spec emulates it and asserts `0s` | `tests/e2e/browser-style-contract.spec.ts:8` (`[P0] AUT-004 … reduced-motion suppression`), `:46` (`await page.emulateMedia({ reducedMotion: "reduce" })`) |
| It runs in CI on every push and PR to `main` | `.github/workflows/test.yml:9-12` — `push: branches: [main]`, `pull_request: branches: [main]` |
| Contrast is genuinely absent | `docs/RELEASE-CHECKLIST.md:95-97` — "Contrast (4.5:1) is **not** automated — check it by eye here." |

**Why this one matters beyond the digit.** `DRIFT-NOTE.md:420-426` records that revision 5
corrected this same rule from a false claim that *both* checks existed, and revision 8 had to
correct the overcorrection. A rule that says coverage is absent invites a story to build it
again — the exact trap AD-1 exists to prevent. `epics.md` was the last live document still
carrying the false absence; `docs/RELEASE-CHECKLIST.md` had already been fixed in commit
`19e1b33`.

### (g) UX-PB.4b ↔ UX-PB.4d — Retry from a History entry

`epics.md` contradicted itself. UX-PB.4d offers Retry from "a terminal Results **or History
entry**"; UX-PB.4b's replay criterion forbade any control that can "mutate, re-run, or
execute anything".

**OLD**

> **And** no control in the replay can mutate, re-run, or execute anything.

**NEW**

> **And** no control in the replay can mutate, re-run, or execute anything, with exactly one carve-out: the non-executing `Retry` affordance UX-PB.4d offers from a History entry. Invoking it reveals the failed-item scope inline inside the replay and executes nothing; the replayed attempt and its records stay immutable, and any execution still goes only through `Create new plan`, the derived `RetryIntent`, and the ordinary preview and confirmation path.

Dependencies gained `AD-24 (Retry derives its own intent; revealing the scope executes nothing)`.

**Rationale, and the one judgment call in this run.** The batch row offers two remedies —
carve-out in 4b, *or* delete 4d's History origin — and the spine had already chosen which
side it was on:

> This spine takes UX-PB.4d's side — revealing a retry scope executes nothing, and any
> execution still goes through the ordinary preview and confirmation path
> (`ARCHITECTURE-SPINE.md:966`)

AD-24 `:778-780` places the scope "in the surface Retry was invoked from — Results, or a
**read-only History replay**". Since the spine is upstream and wins, deleting 4d's History
origin would have reversed a decision this run has no standing to reverse. The carve-out was
therefore the only remedy consistent with the authority order. It is written narrowly — one
named affordance, executing nothing, with execution still routed through
`Create new plan` — so it cannot be read as a general licence to add actions to a replay.

*Recorded for the next reviewer:* `review-reconcile-v8.md:602-615` argues the opposite side —
`DESIGN.md:217` gives the History Plan Row only an "optional **source-retry link**" and the
state `retry-linked`, "a lineage pointer, not an action". That objection is against the
*spine*, not against `epics.md`, and closing it would be a `bmad-architecture` Update run.
`epics.md` now matches the spine either way.

---

## 5. Implementation Handoff

**Scope classification: Minor.** No backlog reorganization, no replan, no new decision.

| Recipient | Responsibility |
| --- | --- |
| **Developer agent** (`bmad-create-story`, then `bmad-dev-story`) | Consume the corrected `epics.md`. The nine touched stories are now safe to turn into story context files. UX-PB.1a and UX-PB.1c must land the complete AD-23 wire shape together — `ARCHITECTURE-SPINE.md:756-759`: "UX-PB.1a and UX-PB.1c may not land it independently; whichever runs first lands the complete shape and the other builds against it." |
| **`bmad-architecture` (Update run)** | Mark the batch row RESOLVED. This run cannot: `ARCHITECTURE-SPINE.md` is out of scope by instruction, and the spine is never hand-edited (Validate → Update, resuming from the run folder's append-only `.memlog.md`). |
| **`bmad-ux`** | The design-token conflict blocking UX-PB.1e and UX-PB.5d. Untouched here and still OPEN. |
| **No one, yet** | The `macos-14` runner retirement — OPEN, dated, release-blocking after 2026-11-02. Workflow work, not planning work. Recorded so it does not fall off. |

### Success criteria

1. No `epics.md` clause contradicts a live AD (AD-1..AD-5, AD-11, AD-12, AD-16..AD-26).
2. All seven batch items closed, so a later architecture run can mark the row RESOLVED.
3. No story text the batch row does not name was modified.
4. `ARCHITECTURE-SPINE.md`, `project-context.md`, `docs/*.md`, and `sprint-status.yaml`
   unchanged.

---

## 6. Verification

### 6a. Mechanical — every offending string is gone

`grep -c` over `_bmad-output/planning-artifacts/epics.md` after the edits:

| Pattern from the batch row | Hits |
| --- | --- |
| `AllEligible' intent` | 0 |
| ``converts to an `Explicit` intent`` | 0 |
| `reconstructed into the sidecar` | 0 |
| `new reviewable draft` | 0 |
| `Neither automated check exists yet` | 0 |
| `which do not exist yet` | 0 |
| `awaiting the deferred native harness` | 0 |
| `harness is Deferred` | 0 |

New citations present: `AD-26` on 6 lines, `AD-21` on 1, `AD-22` on 1, `AD-23` on 1,
`AD-24` on 2. File length 1,306 lines (`wc -l`), from 1,293.

### 6b. Scope — what was deliberately not touched

- `ARCHITECTURE-SPINE.md` — not hand-edited. Closing the Open row is a `bmad-architecture`
  Update run.
- `_bmad-output/project-context.md` and `docs/*.md` — workflow output, regenerated by
  `bmad-generate-project-context` and `bmad-document-project`.
- The design-token conflict — a UX decision, `bmad-ux`'s.
- `sprint-status.yaml` — no epic or story membership change, so no edit was forced.
- `sprint-change-proposal-2026-07-25.md` — **not overwritten.** The workflow's
  `default_output_file` resolves to that path, which already held the revision-7
  reconciliation record and is cited as a live input at `epics.md:14` and `:47`. This
  document was written to a distinct filename at the owner's direction.
- `epics.md:574` — "`Update Everything` seeds all eligible work while remaining editable"
  was left alone. It carries no `AllEligible` framing and does not contradict AD-16's frozen
  expansion; the batch row names one criterion in UX-PB.1c, and only that one was edited.

### 6c. Adversarial — six-lens sweep with three-way refutation

A multi-agent verification sweep ran against the edited file: six independent audit lenses
(plan domain AD-16/AD-17; the six revision-8 invariants AD-21..AD-26; foundations and release
AD-1..AD-5, AD-11, AD-12, AD-18..AD-20; a self-audit of the applied diff; cross-document
reconciliation against SPEC / DECISIONS / RELEASE-CHECKLIST / DESIGN / EXPERIENCE; and
`epics.md` internal self-consistency), each finding then attacked by three distinct skeptics
— one verifying the quoted bytes exist, one verifying the cited AD is live and actually
forbids the clause, one arguing the finding is a silence rather than a contradiction — with
findings killed on a majority refutation. A batch-row closure agent and a completeness critic
ran alongside.

Results are recorded in §6d.

### 6d. Sweep results

**116 subagents, 0 errors, 36 raw findings.** Each finding was attacked by three skeptics
and killed on a majority refutation: **34 killed, 2 survived.** Both survivors were then
re-verified by hand against the file before being recorded here.

#### Batch-row closure: **RESOLVABLE**

The closure agent read the current file rather than any change summary and returned
`overallResolvable: true`, with no partial closure and no residue inside `epics.md`.

| Item | Verdict | Confirming evidence |
| --- | --- | --- |
| (a) UX-PB.1b recovery | **CLOSED** | `:563-566` states AD-17's second branch unconditionally |
| (b) UX-PB.1c seed + conversion | **CLOSED — both halves** | `grep -n AllEligible` returns exactly one hit, `:589`, and it is a negation |
| (c) UX-PB.4d derived intent | **CLOSED** | `grep -n "new reviewable draft"` returns no hits; `RetryIntent` named explicitly with all three non-interactions (write, merge, empty) |
| (d) Native harness ×4 | **CLOSED — all four** | `grep -c harness` = 4 (`:171`, `:285`, `:313`, `:1290`); `grep -c AD-26` = 6; the quoted narrative phrase is absent |
| (e) UX-PB.5b ordering | **CLOSED — both parts** | Fixed sequence reproduced at `:1056-1058`; rejected-admission case its own criterion at `:1060-1062` |
| (f) Accessibility ×2 | **CLOSED** | Both erroneous phrasings absent; claims re-verified against the tree, not only against AD-11 |
| (g) UX-PB.4b ↔ 4d | **CLOSED** | `:947` permits exactly one affordance and pins execution to `Create new plan` + `RetryIntent` + the ordinary path |

**One dependent cleanup belongs to the architecture Update run, not here.**
`ARCHITECTURE-SPINE.md:967` — the already-RESOLVED `epics.md` retired-register row — still
ends:

> Residual: UX-PB.1b `epics.md` UX-PB.1b's recovery criterion still offers the
> draft-reconstruction branch AD-17 forbids.

Item (a) has made that sentence false. It is spine text and was not touched.

#### The two surviving findings — both OUTSIDE the batch row

Neither is in the seven items, so neither was touched during the batch pass. Both were put
to the owner as a separate decision and both were **approved and fixed** — see §7.

**S-1 (HIGH) — AD-25 is cited nowhere in `epics.md`, and Story 2.2 asserts a closed set that excludes it.**

`grep -c "AD-25" _bmad-output/planning-artifacts/epics.md` returns **0**. Every other live
AD id is cited (AD-16 × 31 down to AD-21/22/23 × 1). AD-25 names its holders explicitly —
`ARCHITECTURE-SPINE.md:799`:

> **Binds:** Story 2.2; UX-PB.1e, UX-PB.2b, UX-PB.3d; the verification path

and all four cite something else instead. Story 2.2's contract at `epics.md:1158` reads
`- Governing invariants: AD-4`. `epics.md` closes the escape hatch itself at `:230`:

> **Four invariants bind every story and are cited by none of them, because they have no
> single owner.** A story that does not name them is still bound

— and that list is AD-1, AD-2, AD-20, and the Determinism convention. AD-25 is not in it.
The concrete loss is AD-25's merge-not-replace rule, which the spine says names a live
defect (`:812-814`, `ManagerAdapter::parse_recovery` discarding `refresh_outputs`), and its
failed-verification-refresh containment rule (`:815-819`), which UX-PB.3d's own
verification-failure criterion never states.

**S-2 (HIGH) — the amendment preamble reads as though Story 3.2 were archived.**

`epics.md:350-353` says "Every story area it names except 3.1, 3.4, 3.5, and 6.5 was
archived on 2026-07-25 — Stories 3.3, 3.6, 4.1, 4.6, 5.2, 5.4, 5.5, 6.3, 6.4, and 6.7 …".
The table it describes names "Stories 3.1–3.3 and 3.5–3.6" at `:362`, which includes 3.2.
So 3.2 is a named story area, is not in the exception list, and is not in the archived
enumeration — the sentence's two halves disagree. Story 3.2 is live: retained at `:477`,
full contract at `:1201`. A story author reading `:351` literally skips it.

#### Categories the seven batch items structurally could not catch

Surfaced by the completeness critic. All are outside this run's original scope. Category 1
was subsequently approved and fixed (§7); categories 2–5 remain open.

1. **Spine Decision Status rows that carry no AD id have no route into `epics.md`.** One is
   a direct contradiction, not a silence: `epics.md:307` asserts *"Nothing is blocked from
   starting"* while `ARCHITECTURE-SPINE.md:944` records the canonical design-token set as
   **"OPEN — needs an owner decision, blocks UX-PB.1e and UX-PB.5d"**. `grep -c "macos-14"`
   and `grep -c "focusRing\|design-token"` both return 0 in `epics.md`. *Deciding* the token
   set is excluded from this run by instruction and belongs to `bmad-ux`; recording that the
   blocker exists would not decide it.
2. **The `macos-14` runner retirement is absent from `epics.md`'s Release Acceptance
   section**, which asserts release acceptance at `:187-191` with no open-blocker qualifier.
   Release-blocking after 2026-11-02.
3. **Item (d) fixed one corrected-premise Deferred row and nothing generalized it.** The
   crash/relaunch lifecycle controller (`ARCHITECTURE-SPINE.md:955`) and the opener/reveal
   seam Story 6.5 must add as a sixth port (`:957`) have no counterpart in `epics.md`'s
   register, though Story 6.5's own criterion at `:1304` requires *"native command/opener
   success and failure are controlled"*.
4. **`Blocks:` edges are one-directional.** Six UX-PB stories declare `Blocks: Story 3.x` or
   `Story 6.5`; none of the five blocked survivors names the blocker in its own
   `Dependencies:` line.
5. **Requirement-id provenance.** FR-1..FR-22 / NFR-1..NFR-8 / AJ-1..AJ-6 are inherited from
   the retired PRD; `grep -c 'FR-' docs/SPEC.md` = 0 (SPEC is organized F1..F17). The owner's
   standing instruction for this run treats `epics.md`'s own inventory as the requirements
   record, so this is informational.

#### Tree claims the critic verified independently

Checked against the source tree rather than against any document: the 20 commands / six
events count (`src-tauri/src/lib.rs:232`, `src-tauri/src/events.rs:77-82`); the reduced-motion
chain in all five parts; `AUT-003` at `tests/e2e/upgrade-journeys.spec.ts:169`, which is the
suite UX-PB.1a's final criterion says must be rewritten; `mas absent` at
`src-tauri/tests/live_smoke.rs:4`; the absence of a `contracts/` directory;
`minimumSystemVersion: "15.0"` at `src-tauri/tauri.conf.json:48`; 28 UX-PB stories and 28
`Primary concern` labels; 6 live survivor stories against 31 archived. **All verified true.**

---

## 7. Follow-up applied after approval

The sweep's three surviving defects were put to the owner as an explicit separate decision,
outside the batch row. **All three were approved and fixed in the same pass.** Seven further
edits; running total for `epics.md` is **45 insertions, 26 deletions**.

### 7a. S-1 — AD-25 now reaches the four stories it binds

`ARCHITECTURE-SPINE.md:799` binds AD-25 to Story 2.2, UX-PB.1e, UX-PB.2b, and UX-PB.3d.
`grep -c "AD-25"` in `epics.md` went **0 → 4**:

| Story | Line | Citation added |
| --- | --- | --- |
| UX-PB.1e | `:622` | `AD-25 (Last-good Snapshot retention on refresh failure)` |
| UX-PB.2b | `:664` | `AD-25 (a Manager failure is contained and never destroys a Last-good Snapshot)` |
| UX-PB.3d | `:827` | `AD-25 (a failed verification refresh leaves the Last-good Snapshot in place)` |
| Story 2.2 | `:1159` | `Governing invariants: AD-4` → `AD-4, AD-25` |

A citation alone would have left AD-25 a pointer with no local contract — the failure mode
the 2026-07-24 story-repair pass exists to prevent ("an implementer or test author reading a
single story cannot build the superseded experience"). **Story 2.2 therefore gained one
acceptance criterion** carrying AD-25's substantive rules in its own domain: containment,
Last-good Snapshot retention with its own timestamp and the exact failure plus
`Retry refresh`, recovered-parse output **merging** into the inventory already parsed from
the successful outputs, never an empty replacement and never an outdated-only overlay,
merging never un-pinning a row, and health/staleness reading the snapshot's real timestamp
with no interpolated value substituted (`ARCHITECTURE-SPINE.md:806-823`).

The merge rule was the concrete loss. `ARCHITECTURE-SPINE.md:812-814` states it names a live
shipping defect:

> The seam already exists — `ManagerAdapter::parse_recovery` takes `refresh_outputs`
> alongside the failed command's output precisely so the merge is possible; discarding that
> argument is the defect this rule names.

### 7b. S-2 — the Story 3.2 omission

`epics.md:352`, one word:

> **OLD** … except 3.1, 3.4, 3.5, and 6.5 was archived on 2026-07-25 …
> **NEW** … except 3.1, **3.2,** 3.4, 3.5, and 6.5 was archived on 2026-07-25 …

The archived enumeration in the same sentence was already correct and was not changed.

### 7c. The design-token blocker is now recorded, not decided

`epics.md:307` asserted **"Nothing is blocked from starting"** while
`ARCHITECTURE-SPINE.md:944` records the canonical design-token set as
**"OPEN — needs an owner decision, blocks UX-PB.1e and UX-PB.5d"**. That claim is corrected,
and the Implementation-Entry Register gained a dedicated row at `:308` naming the conflict —
`theme.css` plus the CI style contract against `DESIGN.md`/`EXPERIENCE.md`'s palette and
dedicated `focusRing` — with accountability recorded as *"UX decides; Development
implements"* and the closing line *"Not a story's call and not architecture's alone."*

`grep -c "Nothing is blocked from starting"` now returns **0**. Nothing here decides the
palette; the decision remains `bmad-ux`'s, exactly as scoped.

### 7d. One residual deliberately left, awaiting its own approval

UX-PB.3d now **cites** AD-25 but its verification-failure criterion still does not **state**
the snapshot-preservation half. `ARCHITECTURE-SPINE.md:815-819`:

> A verification refresh that fails or times out marks the attempt's verification failed and
> leaves the Manager's Last-good Snapshot in place — a failed verification must not destroy
> the inventory that would show what actually happened.

The approved scope named a Story 2.2 criterion, not a UX-PB.3d one, so adding an **And**
clause to UX-PB.3d was not done. It is a silence rather than a contradiction, and it is a
one-clause fix whenever the owner wants it.

### 7e. Categories still open after this pass

Unchanged from §6d, and none of them is inside any approved scope:

- The `macos-14` runner retirement is absent from `epics.md`'s Release Acceptance section.
  OPEN, dated, release-blocking after **2026-11-02**.
- The crash/relaunch lifecycle controller and the opener/reveal sixth port have no
  counterpart in the register, though Story 6.5's own criterion requires the seam.
- `Blocks:` edges are one-directional — no blocked survivor names its blocker.
- FR-/NFR-/AJ- ids have no live upstream carrier by id; the owner's standing instruction
  treats `epics.md`'s own inventory as the requirements record.
- `ARCHITECTURE-SPINE.md:967`'s "Residual: UX-PB.1b …" sentence is now false. Spine-side;
  belongs to the `bmad-architecture` Update run that marks the batch row RESOLVED.
