---
name: Sprint Change Proposal — apply the ARCHITECTURE-SPINE revision 10 epics.md residuals batch
date: "2026-07-25"
project: Pack-Manager
workflow: bmad-correct-course
mode: Batch
status: applied
trigger_type: upstream-authority-revision
scope_classification: Moderate
requirements_authority_used: _bmad-output/planning-artifacts/prds/prd-Pack-Manager-2026-07-25/prd.md (status final) + addendum.md
architecture_authority_used: ARCHITECTURE-SPINE.md revision 10
code_baseline: 5c28dcb (branch chore/restore-phase-2-prd), clean tree at start
closes: "ARCHITECTURE-SPINE.md Decision Status row `epics.md` residuals for the next `bmad-correct-course` run — **OPEN**"
# This filename is deliberately distinct. Two same-day proposals already exist and
# epics.md cites the first as authoritative; neither was overwritten:
#   sprint-change-proposal-2026-07-25.md            (spine revision 6 / D33 reconciliation)
#   sprint-change-proposal-2026-07-25-spine-rev8.md
sources_read_this_session:
  - _bmad-output/planning-artifacts/epics.md
  - _bmad-output/planning-artifacts/architecture/architecture-Pack-Manager-2026-07-23/ARCHITECTURE-SPINE.md
  - _bmad-output/planning-artifacts/prds/prd-Pack-Manager-2026-07-25/prd.md
  - _bmad-output/planning-artifacts/prds/prd-Pack-Manager-2026-07-25/addendum.md
  - _bmad-output/planning-artifacts/ux-designs/ux-Pack-Manager-2026-07-23/EXPERIENCE.md
  - docs/DECISIONS.md
  - _bmad-output/project-context.md
  - _bmad-output/implementation-artifacts/sprint-status.yaml
  - .claude/skills/bmad-correct-course/checklist.md
---

# Sprint Change Proposal — 2026-07-25 (spine revision 10 residuals)

Apply the `epics.md` residuals batch that `ARCHITECTURE-SPINE.md` revision 10
recorded rather than applied, and reconcile `epics.md` with the finalized PRD and
with `docs/DECISIONS.md` **D36**, **D37**, and **D38**.

---

## 1. Issue Summary

### Problem statement

`epics.md` is the file `bmad-create-story` reads. Three upstream authorities moved
underneath it on 2026-07-25 and it did not move with them:

1. **`ARCHITECTURE-SPINE.md` revision 10** created `AD-28` (membership, batching,
   the removal taxonomy, the single predicate) and `AD-29` (the plan-attempt
   journal's writer, record cardinality, and fold), widened `AD-27`'s `Binds` to
   Stories 3.4 and 6.5, and reaffirmed `AD-16`'s refusal of a `Cancelling` state.
   It deliberately did **not** edit `epics.md` — its Decision Status row for that
   work reads `OPEN — record only; do not edit epics.md here`. This run is the
   one that edits it.
2. **The PRD became final** and is now the requirements authority. `epics.md`
   carried its own FR/NFR inventory, which is Phase 2 content embedded in a
   Phase 3 artifact, and two of its entries (FR-19, NFR-6) stated obligations the
   PRD explicitly removed.
3. **D36, D37, and D38** landed after the previous correct-course run.

The failure mode is concrete rather than theoretical. A builder opening the file
before this run would have read, as live contract: a `Cancelling` operation state
that the PRD forbids by name; a contrast guard described as not existing when it
shipped in `a201fb0`; a selection model `AD-28` abolishes; and a set of
keyboard/VoiceOver criteria `D37` retired.

### Evidence

Measured in the file as it stood at `5c28dcb`, not estimated:

| Claim | Command | Result before |
| --- | --- | --- |
| `AD-28` cited nowhere | `grep -c "AD-28" epics.md` | `0` |
| `AD-29` cited nowhere | `grep -c "AD-29" epics.md` | `0` |
| `Cancelling` still required | `grep -in "cancelling" epics.md` | 4 lines — FR-13's inventory entry, UX-PB.2e's story sentence, and the two story criteria |
| Spine line-number citations | `grep -c 'ARCHITECTURE-SPINE.md[:#]*[0-9]' epics.md` | `0` — already clean, and kept clean |

The three most dangerous were the ones that **contradicted a rule above them**:
`Cancelling`, the D36 contrast claim, and Story 3.5's selection model.

### Why the ordering mattered

`UX-PB.2e` ships in wave 2. Adding a `Cancelling` variant is one atomic `AD-3`
change across the Rust `OpStatus` enum, `src/lib/ipc/types.ts`, the guards, and
the committed fixtures in `dev/fixtures/ipc/` under D17's byte-equality drift
guard. Correcting the story after it was built would have meant **undoing shipped
wire surface**, not editing story text. That is why this run existed now rather
than at the next convenient point.

---

## 2. Impact Analysis

### Epic impact

**None.** No epic is added, removed, resequenced, or repriced. Epic UX-PB remains
the primary build queue and runs first; the six surviving Epic 1–6 stories carry
no inter-epic dependencies. Every change is inside an existing story's contract or
inside the shared preamble every story reads.

### Story impact

| Story | Change class |
| --- | --- |
| UX-PB.1a, 1b, 1c, 1e | `AD-28` citation added with the subject each story actually binds |
| **UX-PB.1d** | **Retitled and restated** — keyboard/VoiceOver limbs removed, `AD-16`'s not-native-`disabled` rule **re-gated on pointer interaction**, `D38` added |
| UX-PB.2c | `AD-18` + `AD-29` added; admission-record contents corrected; persist-failure wording disambiguated |
| UX-PB.2d | `AD-18` + `AD-29` added; `"where applicable"` defined |
| **UX-PB.2e** | `AD-29` added; **`Cancelling` removed**; declared the terminal-record writer; ordering note added |
| UX-PB.2f | `AD-18` added |
| UX-PB.3a | Plan-start announcement demoted from required to optional |
| UX-PB.3d | `AD-29` added; "several records" → **exactly two**; the authoritative-record gap closed; terminal-write owner moved from UX-PB.4a to UX-PB.2e; "announces" → "renders" |
| **UX-PB.3g** | `AD-29` added; **`Cancelling` removed** |
| UX-PB.4a | `AD-29` added; declared reader-and-folder only; idempotence stated |
| UX-PB.4b | `AD-18` + `AD-29` added |
| UX-PB.4e | `AD-29` added |
| UX-PB.5a | **Restated (owner decision — D37's named list extended)** — keyboard focus-order and focus-restoration limbs removed; `Change Plan`'s return target kept as **navigation**; `Escape`/backdrop "only while no command has begun" kept **unchanged** as a dismissal-safety rule |
| **UX-PB.5d** | **Retitled and restated** — accessibility half removed, zoom/size half repointed to NFR-3 and AD-17 |
| Story 3.1 | `AD-28` added |
| Story 3.2 | `AD-28`, `D38` added; `disabled` → `aria-disabled`; **Dependencies now name UX-PB.1d** |
| Story 3.4 | **`AD-27` added** (rev 10 widened its `Binds`) |
| **Story 3.5** | **Retitled and substantially restated** in membership terms, batched; new non-list-views `⌘A` criterion |
| Story 6.5 | **`AD-27` and `AD-29` added**; export assertion restated as a record-*set* assertion |

### Artifact conflicts

| Artifact | State after this run |
| --- | --- |
| `epics.md` | **Updated by this run.** |
| `ARCHITECTURE-SPINE.md` | Its residuals row is now dischargeable. **One known self-contradiction is deliberately not fixed** — see §5. |
| `prd.md` / `addendum.md` | Unchanged. `epics.md` now cites them as requirements authority rather than restating them. |
| `EXPERIENCE.md` | **Still carries `Cancelling`** at its Activity Operation Row and its 120-second stall row, and still carries the `Keyboard`, `Package Grid keyboard model`, `Focus transitions`, and `Accessibility Floor` sections. Routed to `bmad-ux` (Update intent). Recorded in `epics.md` as a known divergence, not authority. |
| `docs/SPEC.md` | Hand-written, workflow-unowned. §4.11 still lists `⌘U` and `Esc clear selection`; F5 still carries the transient-selection model. Maintainer edit. |
| `docs/RELEASE-CHECKLIST.md` | Already rescoped in `5c8996f`; no action. |
| `sprint-status.yaml` | **Updated** — three renamed story keys, all still `backlog`. |

### Technical impact

The one item with a code consequence is the `Cancelling` removal, and its
consequence is **negative work**: `OpStatus` ships seven variants and none of the
corrected stories now asks for an eighth, so the atomic `AD-3` change across the
Rust enum, `src/lib/ipc/types.ts`, the guards, and `dev/fixtures/ipc/*.json` is no
longer scheduled at all.

The `AD-28` restatement of Story 3.5 changes what that story builds — direct
membership with a batch verb, rather than a transient selection set — but it
changes it to what `UX-PB.1a` was already going to build, which is the divergence
`AD-28` exists to close.

---

## 3. Recommended Approach

**Direct Adjustment.** Rollback is not viable (nothing is built — every affected
story is `backlog`), and an MVP review is not warranted (no requirement changed;
this run reconciles a downstream artifact with authorities that already moved).

- **Effort:** Low. Documentation-only, and it *removes* scheduled build work.
- **Risk:** Low, with one asymmetry worth stating: the risk of **not** doing it
  before `UX-PB.2e` is built is high, because the `Cancelling` correction would
  then be a wire-surface revert.
- **Timeline:** No change to the queue.

---

## 4. Detailed Change Proposals

### 4.1 Shared preamble — `## Requirements Inventory`

**NEW (added):** an authority header naming the PRD as requirements authority,
declaring the FR/NFR entries a convenience index rather than a second statement,
and stating four things a reader needs before reading any entry: the D37
restatement of FR-19/NFR-6, the *scope-by-named-section-never-by-count* rule, the
non-existence of `Cancelling`, and FR-23's unowned status.

**Rationale:** `addendum.md` §3 routes exactly this to `bmad-correct-course`. The
inventory is **not deleted**, because the FR Coverage Map and every story's
`FR and requirement links` line resolve against it; it is demoted to an index.

### 4.2 FR-13

**OLD:** `Expose queued, running, verifying, stalled, cancelling, and terminal plan state…`

**NEW:** `Expose queued, running, verifying, stalled, and terminal plan state…` plus
`**There is no distinct \`cancelling\` state** — cancellation moves an Operation to
its terminal state and the 5-second SIGTERM grace window is not surfaced as its own
status (\`prd.md\` FR-13, \`ARCHITECTURE-SPINE.md\` AD-16).`

**Rationale:** this is a **third** `Cancelling` site the spine's residuals row did
not name. That row measured capital-`C` `` `Cancelling` `` and correctly found two
story-block occurrences; FR-13's inventory entry carries the lowercase word and was
outside that measurement. Recorded here because it is the exact failure mode both
the spine row and `addendum.md` §3 warn about — a count is not the scope.

### 4.3 FR-19 and NFR-6

Both restated from `prd.md` §4.4 FR-19 and §5 NFR-6, each followed by an explicit
**Restated per D37** paragraph naming what was removed. Removed as criteria:
keyboard operability of primary actions, VoiceOver operability, live-region
announcement of plan progress/verification/cancellation/failure/completion, and
deterministic dialog and sidecar focus restoration.

The focus **indicator** is kept and labelled as a rendering mechanism governed by
`AD-27`, with the trap stated inline: a pass scoping D37 by searching for the word
*keyboard* will hit `AD-27`, and deleting it is the error.

### 4.4 FR-17

Gained status qualification: `skipUpgradePlanConfirmation` is **Planned — D28**
and does not ship today; `autoOpenDrawer` **retires** from the Settings view and
the target field set along with the `ActivityDrawer` (`prd.md` FR-17, `AD-17`),
while an old persisted value stays tolerated on read and inert (`AD-19`).

### 4.5 FR-23 — new inventory entry, deliberately unowned

`prd.md` §4.3 carries FR-23 ("Constrain which Manager-suggested fixes become
runnable"), which post-dates this document's inventory. It is added to the
inventory so a builder learns it exists, and its FR Coverage Map row reads
**`Unassigned — owner decision`** rather than being assigned by inference. See §5.

### 4.6 DR-2 restatement and the Implementation-Entry register row

Both said automated 4.5:1 contrast "does not exist" and named it as the
outstanding obligation on whichever story adds it. **Commit `a201fb0` landed it**,
with the guard in `tests/e2e/browser-style-contract.spec.ts` reading the rendered
primary button's real computed foreground and background, applying the WCAG 2.1
luminance formula, failing below 4.5:1, and carrying a negative assertion against
`text-white` returning.

Both are corrected, and the register row is retagged `RESTATED` → **`CLOSED`**.
The register is the more dangerous of the two because it is what a builder
consults to decide what is startable, and scheduling shipped work is the error
`AD-1`'s second rule forbids. Both now also state that the guard is a **named
sample, not a sweep**, and the DR-2 bullet records that D37 deleted the manual
VoiceOver pass this restatement previously added to the release checklist.

### 4.7 Story UX-PB.1d — restated, **not deleted**

**Title OLD:** `Ineligible-control inertness with keyboard, pointer, and VoiceOver explanation`
**Title NEW:** `Ineligible-control inertness with a pointer-reachable explanation`

**The criterion that mattered.** The old text read:

> **Given** an explanatory-disabled Package control
> **When** a keyboard or VoiceOver user reaches it
> **Then** it uses `aria-disabled="true"` rather than native `disabled`, keeps
> focus, announces its persistent reason as an accessible description, stays
> inert on activation, and retains focus when Escape closes its supplemental
> Tooltip/Popover.

The not-native-`disabled` rule is the one `AD-16` now **requires**, and the whole
criterion was gated on a keyboard-or-VoiceOver `Given`/`When`. A run that stripped
those limbs by pattern would have deleted the only stated trigger for the rule that
must survive. It is **re-gated on pointer interaction** instead, with the mechanism
stated: a natively disabled form control dispatches no mouse events, so the native
state and the pointer-reachable explanation are mutually exclusive and the
explanation wins.

`D38` is added as a governing decision, and the shipping defect it diagnoses is
named in the criterion — a `title` attached at `PackageRow.tsx:95` to an input
marked natively `disabled` at `:92`, which therefore never renders on hover,
leaving `disabled:opacity-40` at `:100` as gray styling with the explanation
unreachable. Removing native `disabled` **restores** what D15 asked for rather
than regressing from it.

The first criterion's activation trigger is restated as a fail-closed property
("no other activation path may defeat it either") with pointer activation named as
the criterion, so the safety semantics do not depend on a keyboard obligation.

### 4.8 Stories UX-PB.2e and UX-PB.3g — `Cancelling` removed

**UX-PB.2e OLD:** `running work moves to \`Cancelling\` and escalates through the existing process-group mechanics`
**UX-PB.2e NEW:** `running work escalates through the existing process-group mechanics and moves **straight to its terminal state**` + an explicit `**And** no \`Cancelling\` state is introduced at any level` clause and an **Ordering-critical** preamble stating the AD-3 wire-surface reason.

**UX-PB.3g OLD:** `changes still-running Operations bound to that \`planAttemptId\` to \`Cancelling\``
**UX-PB.3g NEW:** `moves still-running Operations bound to that \`planAttemptId\` **straight to their terminal state** through the existing process-group escalation` + the same explicit prohibition.

These were the only two story-block occurrences. An earlier draft of the spine's
row named UX-PB.2f and UX-PB.4c, which contain none; a run following that literally
would have opened two clean stories and left both defective ones. Verified by
`grep -in "cancelling"` against the committed file before editing.

### 4.9 AD-29 — record cardinality, the writer, and the fold

Three coupled corrections, all from `AD-29`:

- **UX-PB.2c's admission record no longer lists "result/verification state".** An
  admission record cannot hold a result that does not exist yet. That state moves
  to the terminal record. The story now carries the override explicitly, the way
  UX-PB.5b already carries AD-22's.
- **UX-PB.3d's "an attempt accumulates several append-only records"** → **exactly
  two**, admission and terminal. Its self-declared gap — "no rule for which record
  is authoritative" — is answered by AD-29's fold rule rather than left open.
- **Terminal-write ownership moves from UX-PB.4a to UX-PB.2e.** UX-PB.4a depends
  on all of UX-PB.3a–3g while UX-PB.2e ships in wave 2, so the old assignment left
  every attempt terminating in between persisted admission-only, which the fold
  must then read as `Interrupted`. UX-PB.4a is now declared a **reader and folder
  only**. UX-PB.3d's "this story never writes one" is preserved and repointed.

`PlanAttempt.state` is additionally declared a derived read-model value that no
record carries, in UX-PB.2c.

### 4.10 Citation coverage

`AD-18` added to UX-PB.2c, UX-PB.2d, UX-PB.2f, UX-PB.4b — the four stories it
binds that cited it nowhere. `AD-29` added to all nine stories it binds. `AD-28`
added to Stories 3.1, 3.2, 3.5 and UX-PB.1a–1e. `AD-27` added to Stories 3.4 and
6.5, whose omission `AD-27` revision 10 diagnoses as this document faithfully
mirroring a closed list that had dropped them.

Each citation carries the **subject** the story actually binds, never a rule
ordinal and never a line number. `grep -c 'ARCHITECTURE-SPINE.md[:#]*[0-9]'`
remains `0`.

### 4.11 Story 3.5 — restated in membership terms

**Title OLD:** `Preserve Exact Keyboard Selection and Row Plan Actions`
**Title NEW:** `Preserve Exact Batched Plan Membership and Row Plan Actions`

The criteria were written on the transient-selection model `AD-28` abolishes. They
now state: direct membership with no separate set and no `Add Selected` step; one
batched membership operation per range or filter-wide interaction; concrete
canonical identities plus a snapshot token rather than a predicate for Rust to
re-expand; a range resolved over the ordered filtered set the projection holds
rather than the rendered DOM window; the anchor surviving while the selection set
does not; the tri-state denominator; the all-or-none-in-application /
narrowing-in-resolution rule; and provenance following the shape of the act.

**A new second criterion** covers the Dashboard, History, and Settings — the views
with no Package list — where the shipping `⌘A` handler calls `preventDefault()`
before its helper early-returns, blocking native select-all and putting nothing in
its place. Story 3.5 was the only story naming `⌘A` and its `Given` was a row list,
so that defect had no owner. It now does. Stated as a functional select-all
regression, not an accessibility item — D37 keeps `⌘A` by name and RP-2 makes it a
release prerequisite.

`epics.md` never named `⌘U` (`grep -c` → `0` before this run), so no `⌘U` limb was
removed here; only `docs/SPEC.md` §4.11 carries that.

### 4.12 Story UX-PB.5d — restated, **not deleted**

**Title OLD:** `Accessibility and responsiveness of the confirmation and safety surfaces`
**Title NEW:** `Responsiveness of the confirmation and safety surfaces at the size and zoom floors`

The persona, the keyboard/VoiceOver `When`, the focus-order-and-announcements
`Then`, and the deterministic focus-restoration criterion are removed. The zoom and
minimum-window half survives and is repointed from `FR-19` to **`NFR-3`** — the
limb the story actually needs is the responsiveness floor — plus `AD-17` for the
below-720px layout rule. A new criterion carries `AD-17`'s persistent,
non-occludable indicator for safety-critical attempt state at that width, which is
the mechanism that replaces the announcement channel D37 made optional.

`prd.md` §10 records the owner's 2026-07-25 confirmation that NFR-6's
"deterministic dialog/sidecar focus restoration" stays dropped rather than being
carved out as an exception; that is cited in the story so the next reader does not
reopen it.

### 4.13 Story 3.2 and Story 6.5

Story 3.2's pinned-row criterion said `pinned rows stay inert … and are explained,
**disabled**, and excluded from every plan`. `AD-16` binds Story 3.2 and forbids the
native state; it renders the same control as UX-PB.1d and takes the same
correction, with `D38` cited. Its Dependencies now name **UX-PB.1d**, closing an
edge that existed in one direction only (UX-PB.1d declared `Blocks: Story 3.2`
while Story 3.2 listed only Story 3.1).

Story 6.5's export assertion is restated as a **record-set** assertion: the fields
are carried *between* an attempt's two records rather than duplicated into each, so
the assertion cannot be written against any single record — `AD-29`'s split makes
such a record impossible.

### 4.14 `sprint-status.yaml`

Three keys renamed to track the retitled stories, all still `backlog`, no status
lost:

- `ux-pb-1d-ineligible-control-inertness-with-keyboard-pointer-and-voiceover-explanation` → `ux-pb-1d-ineligible-control-inertness-with-a-pointer-reachable-explanation`
- `ux-pb-5d-accessibility-and-responsiveness-of-the-confirmation-and-safety-surfaces` → `ux-pb-5d-responsiveness-of-the-confirmation-and-safety-surfaces-at-the-size-and-zoom-floors`
- `3-5-preserve-exact-keyboard-selection-and-row-plan-actions` → `3-5-preserve-exact-batched-plan-membership-and-row-plan-actions`

---

## 5. Deliberately Not Done, and Why

Each of these was reachable in this run and was left alone on purpose.

1. **`AD-28`'s self-contradicting `Esc` bullet is not edited.** Its opening clause
   says "the cascade drops from three rungs to two and keeps close-dialog" — stale,
   quoting `prd.md` FR-6's pre-`AD-17` wording — while its own correction four
   lines later says "There is no surviving second rung … so the cascade is
   close-dialog alone." **This run followed the second clause everywhere**: Story
   3.5 states `Esc` never touches membership and the cascade is close-dialog alone.
   The spine fix belongs to `bmad-architecture` (Update intent), against `AD-28`'s
   `Esc` rule by name. `prd.md` §10 already records this as a divergence and rules
   that FR-6 is correct and stays as written.

2. **`Clear` is retained with a changed meaning rather than deleted. CONFIRMED by
   the owner, 2026-07-25.** The spine's residuals row and `AD-28`'s rule text
   disagree, and **the rule text governs** — recorded here so a later reader does
   not re-file this as an unapplied residual.

   - The **residuals row** compresses it to "a `Clear` action and an `Esc` rung
     AD-28 deletes".
   - **`AD-28`'s rule text**, at the closed removal taxonomy, names it: "**(2) A
     scope-wide removal** — the header checkbox, `⌘A` on an all-staged view, a
     Manager-wide remove, **`Clear`**, or undoing an `Update Everything` seed".

   The row is compressing "AD-28 deletes `Clear`-as-selection-clear". `Clear` as a
   *membership* removal is lawful and named, and deleting it outright would leave
   a named shape in a closed taxonomy with no story owning it. Story 3.5 states
   both halves explicitly. `Esc` is genuinely deleted and carries no replacement
   sink — the two are not symmetrical and the row's phrasing joins them.

3. **`EXPERIENCE.md` is not touched.** It still carries `Cancelling` at its
   Activity Operation Row and its 120-second stall row, and still carries the four
   D37-affected sections. Routed to `bmad-ux` (Update intent) — never a hand edit.
   `epics.md` now records it as a known divergence so a builder does not read it as
   authority.

4. **`docs/SPEC.md` is not touched.** Hand-written and workflow-unowned. §4.11
   still lists `⌘U` and `Esc clear selection`, and F5 still carries the transient
   selection model. Maintainer decision.

5. **Story UX-PB.5a — RESOLVED by the owner, 2026-07-25: D37 extended to it,
   surgically.** This is now applied, and it is recorded here rather than under
   §4 because the *reason* is the part that must survive.

   **UX-PB.5a was not in D37's named list.** The owner **extended the list**
   rather than making an exception to the rule, so "scope by named section, never
   by a mention count" still stands unqualified. The extension is recorded in the
   story itself so a reader does not mistake it for a run that scoped by class.

   The dismissal criterion bundled three separate things in one sentence and only
   two were keyboard — the same trap Story UX-PB.1d carried. Cut:

   | Clause | Disposition |
   | --- | --- |
   | "focus moves to the dialog heading/command summary with `Change Plan` as the first actionable control so a final confirmation is never the accidental default for an unfocused Enter" | **Removed** — the failure it guards against requires a keyboard |
   | "`Change Plan` returns **focus** to the first staged Remove control or the plan heading" | **Focus half removed; navigation kept** — `Change Plan` returns *to* the first editable plan item or the plan heading. `EXPERIENCE.md`'s Confirmation Dialog contract, which a mouse user experiences as scroll position and as what the dialog closes back to |
   | "Escape/backdrop dismiss **only while no command has begun**" | **Kept unchanged** — a dismissal-safety rule about not tearing the dialog down mid-execution. Not focus restoration, not in D37's scope |
   | "restore focus to the originating `Confirm N Updates` action" | **Removed** |

   Pattern-stripping the sentence would have deleted a safety property along with
   the keyboard limbs. The dimmed, focus-trapped background in the criterion above
   it is modal behavior and is unaffected.

6. **The `### UX Design Requirements` bullet naming "keyboard, focus, VoiceOver"
   is left as written.** It is a *pointer* to `DESIGN.md`, `EXPERIENCE.md`, and
   `validation-report.md`, all of which still carry those sections, so it is
   accurate as a pointer. Editing it here would front-run the `bmad-ux` Update.

7. **The `2026-07-24 Correct Course story amendment` table is left as written.**
   It declares itself "a historical revision record, not a live instruction", and
   every story area it names except 3.1, 3.2, 3.4, 3.5 and 6.5 is archived.
   Rewriting it would destroy what it records.

8. **The FR/NFR inventory is demoted, not deleted.** `addendum.md` §3 observes it
   is Phase 2 content in a Phase 3 artifact. Deleting it would break the FR Coverage
   Map and every story's `FR and requirement links` line in the same edit. It now
   carries an authority header declaring the PRD upstream. **A full excision is a
   separate, larger change and is an owner decision.**

---

## 6. Owner Decisions — All Three Resolved 2026-07-25

The run surfaced three decisions rather than making them. All three were decided
by the owner on 2026-07-25 and all three are **applied**.

### 6.1 FR-23 — owner is **Epic UX-PB**, and no new story is required

The requirement splits by what ships, and each half is dispositioned:

**The safety property ships today. Recorded as satisfied; do not schedule it.**
The load-bearing rule is that a Manager's *scraped* suggestion never becomes
runnable text. Verified in the working tree this session:

| Site | Literal |
| --- | --- |
| `src-tauri/src/managers/parse/uv.rs:82`-`:83` | `let fixable = SAFE_TOOL_NAME_RE.is_match(&name)` / `&& suggested_fix.as_deref() == Some(canonical_fix.as_str());` |
| `src-tauri/src/managers/parse/uv.rs:93`-`:94` | `fix_command: fixable.then_some(canonical_fix),` / `fix_args: fixable.then_some(canonical_args),` |
| `src-tauri/src/queue.rs:334` | `if issue.manager_id != manager \|\| !issue.fixable {` — refuses at submission |
| `src-tauri/src/ipc.rs:93` | `HealthFix,` — the distinct `OpKind` variant |
| `src-tauri/src/lib.rs:240` | `commands::run_health_fix,` — production registration |
| `src/components/manager/HealthBanner.tsx:43` | `onClick={() => void runHealthFix(managerId, issue.id)}` |

`parse/uv.rs:82`-`:83` is a byte-equality test between the scraped suggestion and
the argv Pack-Manager constructed itself; `:93`-`:94` gate exposure on it, leaving
an altered or malformed suggestion visible in `detail` but neither copyable nor
runnable. Scheduling this would be the error `AD-1`'s second rule forbids.

**The unbuilt limb joins the 2026-07-24 amendment's supersession umbrella.**
Routing `Run fix` through the draft plan under D27–D30 is the identical work as
the other immediate-execution bypasses — a Package row's own update action and a
Manager's self-update — which `epics.md:46` already handles by supersession rather
than by a dedicated story. FR-23 joins on the same terms under Epic UX-PB.

`prd.md` FR-23 counts three immediate-execution *kinds* while FR-6 counts four
*call sites*. Both are correct and measure different things; a story that removes
"three" must confirm which. The set may not grow.

The FR Coverage Map row is changed from `Unassigned — owner decision` to Epic
UX-PB with that split stated. The preamble flag and the inventory entry are kept —
a builder reading only `epics.md` still needs to learn the requirement exists —
with their "has no epic owner here" wording corrected.

### 6.2 UX-PB.5a — D37 extended, surgically

See §5 item 5 for the clause-by-clause disposition and the reason the criterion was
cut rather than stripped. The list was **extended**; the scope-by-named-section
rule is unqualified and still stands.

### 6.3 `Clear` — kept as a scope-wide membership removal

See §5 item 2. The residuals row's summary and `AD-28`'s rule text disagree and
the rule text governs.

---

## 7. Implementation Handoff

**Scope classification: Moderate.** No code changes; backlog contracts and one
generated tracking file changed, and the retitles touch `bmad-create-story` lookup
keys.

| Recipient | Deliverable |
| --- | --- |
| **`bmad-architecture` (Update intent)** | Fix `AD-28`'s `Esc` bullet — its stale opening clause contradicts its own correction four lines later. Resume from the run folder's `.memlog.md`. Then discharge the `epics.md` residuals row, which this run has cleared. |
| **`bmad-ux` (Update intent)** | `EXPERIENCE.md`: remove `Cancelling` from the Activity Operation Row and the 120-second stall row; apply D37 to `## Keyboard`, `## Package Grid keyboard model`, `## Focus transitions`, and `Accessibility Floor`. Its membership model and its explanatory-disabled rule are **correct and survive**. |
| **Maintainer** | `docs/SPEC.md` §4.11 (`⌘U`, `Esc clear selection`, roving tabindex, completion-announcing live region) and F5's transient-selection model; and adding F5 to §0.1's supersession list. |
| **Owner** | The three decisions in §6. |
| **Development** | Build from `epics.md` as it now stands. `UX-PB.2e` no longer asks for an `OpStatus` variant; do not add one. |

### Success criteria

- No story requires a `Cancelling` state at any level — verified: the eight
  surviving occurrences are prohibitions or the ordinary English verb.
- `grep -c "AD-28"` and `grep -c "AD-29"` are non-zero and every story each AD
  binds cites it by subject.
- No story schedules an automated 4.5:1 contrast check.
- Stories UX-PB.1d and UX-PB.5d exist, are retitled, and retain their surviving
  behavior — neither was deleted.
- `grep -c 'ARCHITECTURE-SPINE.md[:#]*[0-9]'` stays `0`.
