# Reviewer Gate — adversarial divergence lens, `ARCHITECTURE-SPINE.md` revision 10

**Target (read-only):**
`_bmad-output/planning-artifacts/architecture/architecture-Pack-Manager-2026-07-23/ARCHITECTURE-SPINE.md`,
`artifact_revision: 10`.
**Units one level down:** `_bmad-output/planning-artifacts/epics.md` — Epic UX-PB's 28
stories plus surviving Stories 2.2, 3.1, 3.2, 3.4, 3.5 and 6.5.

**Charter.** Construct two units one level down that each obey every `AD` to the
letter yet still build incompatibly — clashing shared-data shapes, two owners of one
entity, conflicting state-mutation paths. Every pair is a hole to close with a new or
tightened `AD`.

---

## Snapshot and verification record

**The spine was rewritten three times while this lens read it.** Every finding below
is verified against one pinned snapshot; two findings this lens had already written up
were closed by those concurrent edits before the pass finished, and are recorded as
closed rather than reported as open.

| Command | Result |
| --- | --- |
| `wc -l ARCHITECTURE-SPINE.md` (read 1) | `1338` |
| `wc -l ARCHITECTURE-SPINE.md` (read 2) | `1396` |
| `wc -l ARCHITECTURE-SPINE.md` (read 3) | `1436` |
| `wc -l ARCHITECTURE-SPINE.md` (**pinned snapshot**) | `1443` |
| `git hash-object` of the pinned snapshot | `b302ad41e972ec513046b9d998c33a9772970b82` |
| `git diff --stat -- ARCHITECTURE-SPINE.md` (at read 2) | `1 file changed, 420 insertions(+), 76 deletions(-)` |
| `wc -l _bmad-output/planning-artifacts/epics.md` | `1319` |
| `grep -c "Cancelling" _bmad-output/planning-artifacts/epics.md` | `2` |

**All `ARCHITECTURE-SPINE.md` line numbers below are against blob `b302ad4`
(1443 lines).** `epics.md` was not modified during this pass, so its line numbers are
current. The spine's own Citations convention (blob `b302ad4`:1340, "Cite by **name**,
never by position. No line numbers into a document under edit") and this lens's charter
(`path:line "verbatim"`) are both honoured: every citation carries the `AD` id or story
id **and** its subject, so it survives the next insertion; the line number is a locator
only. That convention just proved itself — the spine moved 105 lines under this review.

### Closed by concurrent edits mid-review (do not re-open)

Both were written up as findings and both were fixed before this file was finalized.
Recorded so the gate's arithmetic is honest and so a later pass does not re-raise them.

1. **AD-16's `Cancelling` rule misattributed its own evidence.** At read 2 the rule
   read "`epics.md` (UX-PB.2f and UX-PB.4c, "changes still-running Operations … to
   `Cancelling`")". `grep -c "Cancelling" epics.md` returns **2**, at lines 740
   (UX-PB.2e) and 903 (UX-PB.3g); UX-PB.2f and UX-PB.4c contain none, and the quoted
   fragment is UX-PB.3g's text attributed to UX-PB.4c. In the snapshot the rule reads
   "`epics.md` (**UX-PB.2e** … and **UX-PB.3g** …)" (`b302ad4`:499–505) and the
   residuals row now records the correction. **The ordering consequence it exposed is
   still open — see H-5.**
2. **Story 3.2's pinned-row criterion re-imports the native `disabled` attribute** that
   AD-16's corrected inertness rule forbids. Now recorded in the `epics.md` residuals
   row (`b302ad4`:1436, "Story 3.2's pinned-row criterion still says `disabled`").
   One residual is *not* recorded and is carried below as **L-2**.

---

## Verdict

**Two CRITICAL divergences sit in AD-28's new membership text — a removal taxonomy that
does not cover three of the four removal shapes the bound stories build, and a batch
whose resolution has two lawful and opposite outcomes because no rule says which side of
the IPC boundary owns the one eligibility predicate. Both wedge the primary membership
control, and both stories in each pair are fully AD-compliant.**

| Severity | Count |
| --- | --- |
| CRITICAL | 2 |
| HIGH | 5 |
| MEDIUM | 3 |
| LOW | 4 |

Ranked by blast radius: C-1 and C-2 wedge or silently invert the checkbox that is the
product's core interaction; H-1 loses every completed attempt's durable outcome across a
multi-story span; H-2 and H-3 make a fixture-backed record shape unbuildable and
unreadable; H-4 hides a safety-critical handoff at a supported window size; H-5 puts a
forbidden variant on the wire in an atomic AD-3 change that is expensive to withdraw.

---

# CRITICAL

## C-1 — AD-28's removal taxonomy is not total: three of the four removal shapes the bound stories build are neither of the two the tombstone rule names

**Story A: Story 3.5** — *Preserve Exact Keyboard Selection and Row Plan Actions*

> `epics.md`:1277 "**Given** eligible, current, pinned, greedy, filtered, and
> range-addressable rows"
> `epics.md`:1278 "**When** toggle, shift-range, tri-state, Cmd+A, Space, Cmd-click,
> Clear, and Esc interactions execute"
> `epics.md`:1279 "**Then** the exact selectable identities and visible filter semantics
> are preserved"

**Story B: UX-PB.1c** — *Remaining draft entry points as independent removable items*

> `epics.md`:587 "**Given** a draft seeded by `Update Everything`, whose expansion was
> frozen into concrete members at the moment I invoked it — each carrying
> `Bulk { scope: Everything }` provenance that is never re-evaluated"
> `epics.md`:588 "**When** I remove any item"
> `epics.md`:589 "**Then** that one member leaves the draft and a tombstone records the
> removal, so no later bulk expansion of any scope re-adds it"

**The `AD` text both obey:**

> `ARCHITECTURE-SPINE.md`:1187 "- **Rule:** Bulk removal is the **inverse** of bulk
> addition. A filter-wide or Manager-wide remove clears membership for the refs in its
> scope *and clears their tombstones*; only an individual single-ref removal writes one."

> `ARCHITECTURE-SPINE.md`:924 "- **Rule:** Removal writes a tombstone on the intent. A
> later bulk expansion of any scope does not re-add a tombstoned ref — a member list can
> record presence but not a deliberate absence, so the "stays removed" guarantee needs
> this home."

> `ARCHITECTURE-SPINE.md`:1158 "- **Rule:** **A range or filter-wide interaction is one
> membership operation.** It submits every affected identity in a single batch — one
> round trip, one canonical rebuild — never one operation per row."

> `ARCHITECTURE-SPINE.md`:1181 "- **Rule:** Provenance follows the shape of the act,
> under AD-23. A **range is `Explicit`** for every member: it names concrete visible rows
> rather than a predicate, AD-23's `scope` enum has no token for a contiguous span"

The taxonomy has exactly two members — "an individual single-ref removal" and "a
filter-wide or Manager-wide remove". Four removal shapes the bound stories build fall
outside both:

**(a) A shift-range uncheck.** AD-28:1158 makes it **one** batched operation, so it is
not "an individual single-ref removal". AD-28:1181 files a range as `Explicit` with **no
scope token**, so "the refs in its scope" is undefined for it, and a range is neither
filter-wide nor Manager-wide. Story 3.5 owns it (`epics.md`:1278, "shift-range").

**(b) `Clear`.** `epics.md`:1278 lists it. A draft-wide clear is not filter-wide and not
Manager-wide.

**(c) An `Everything`-scope removal.** AD-23's scope enum has three tokens —

> `ARCHITECTURE-SPINE.md`:562 "  origin: Explicit | Bulk { scope: Manager(ManagerId) | FilteredView | Everything }"

— and AD-28's bulk-removal rule enumerates **two of the three**. Undoing an
`Update Everything` seed, which is exactly UX-PB.1c's Given at `epics.md`:587, is the
missing token.

**(d) `⌘A` on an all-staged view.** AD-28:1181 files `⌘A` as
`Bulk { scope: FilteredView }` and AD-28:1222 makes the header `checked` when all of the
set is staged, so the natural next press is a removal — but AD-28 describes `⌘A` only as
an addition, and `prd.md` FR-6 (`prd.md`:236, "Only the sink changes: the same identity
set goes to the batch membership operation instead of a selection set") does not say
whether it toggles.

**What each builds.** Story 3.5's builder reads "only an individual single-ref removal
writes one", observes that a range is one batch of N refs rather than an individual
removal, and writes **no** tombstones for a 50-row shift-range uncheck — consistent with
AD-28:1187's inverse framing. UX-PB.1c's builder reads AD-23:924 "Removal writes a
tombstone on the intent" as the default for every shape the narrower rule does not carve
out, notes that a range is neither of AD-28's two named bulk scopes, and writes **50**
tombstones.

**What breaks at runtime.** The same user action, opposite outcomes, both lawful:

- *Story 3.5's build* — the user shift-unchecks 50 rows, then clicks `Update Everything`.
  All 50 re-enter the draft. The deliberate removal is silently reversed, defeating the
  guarantee AD-23:924 says the tombstone exists to provide: "a member list can record
  presence but not a deliberate absence".
- *UX-PB.1c's build* — the same 50 refs become session-permanent vetoes and the next
  `Update Everything` stages nothing for them. That is verbatim the failure AD-28 says
  its own rule exists to prevent: `ARCHITECTURE-SPINE.md`:1187 (continuing) "Without this
  rule one header-uncheck writes N session-permanent vetoes and the next `Update
  Everything` stages nothing, which a user would rightly call broken."

Both stories are bound by AD-28 (`ARCHITECTURE-SPINE.md`:1133 ff., "**Binds:** Stories
3.1, 3.2, 3.5; UX-PB.1a, UX-PB.1c, UX-PB.1d") and both are AD-compliant, so this is a
**spine hole, not a story defect**. The residuals row's instruction to restate Story 3.5
in membership terms (`ARCHITECTURE-SPINE.md`:1436) does not cure it — the restatement
still has to pick a tombstone rule, and the `AD` has no answer to give it.

**Sub-case, same root — "their tombstones" is ambiguous.** AD-28:1187's "clears
membership for the refs in its scope *and clears their tombstones*" reads either as *the
refs whose membership this act cleared* or as *every ref in the scope*. UX-PB.1c reads it
narrowly and preserves a tombstone for a ref that held no membership; Story 3.5, owning
the header act, reads it widely and clears it. Under the wide reading: the user
explicitly removes `foo` (tombstone written), later unchecks the header — an act covering
`foo`'s ref — `foo`'s tombstone is cleared, and `Update Everything` re-stages `foo`,
which the user never re-staged. AD-23:924 makes re-staging the **only** thing that clears
a tombstone ("Explicitly re-staging a tombstoned ref clears its tombstone: a user
reversing themselves deliberately is not a silent re-add"), and AD-28's justification —
"A tombstone exists so a **narrower** deliberate act survives a **wider** automatic one …
the wider act is the newer intent" — points the wrong way here, because both acts were
removals and the newer intent was to have *less*.

**Proposed new `AD` text — AD-28, replacing the bulk-removal rule:**

> - **Rule:** Removal is a **closed three-way taxonomy** and every membership removal is
>   exactly one of them. **(1) A single-ref removal** — one checkbox, one `Remove`
>   control — writes a tombstone (AD-23). **(2) A scope-wide removal** — the header
>   checkbox, `⌘A` on an all-staged view, a Manager-wide remove, `Clear`, or undoing an
>   `Update Everything` seed — carries one of AD-23's three scope tokens (`Manager`,
>   `FilteredView`, `Everything`), clears membership for the refs in that scope, and
>   clears the tombstones **of the refs whose membership it actually cleared**, never of
>   refs that held none, so a wider removal cannot erase a narrower veto the user has not
>   reversed. **(3) A range removal** is a batch of single-ref removals for tombstone
>   purposes and writes one tombstone per member, on the same ground AD-28 already files
>   a range as `Explicit`: it names concrete rows rather than a predicate. **Batching is a
>   transport requirement and never changes a removal's taxonomy class.** A shape not on
>   this list is a new decision, not an implementation choice.

---

## C-2 — One batch, two lawful outcomes: AD-28's all-or-none against AD-16's narrowing rebuild, because no rule says which side of the boundary owns the one predicate

**Story A: Story 3.5** — owns the range, `⌘A`, and the tri-state

> `epics.md`:1278 "**When** toggle, shift-range, tri-state, Cmd+A, Space, Cmd-click,
> Clear, and Esc interactions execute"
> `epics.md`:1280 "**And** excluded rows never enter selection."

**Story B: UX-PB.1a** — owns the staleness resolution

> `epics.md`:534 "**Given** a Package that becomes pinned, already current, or removed
> between my add action and the Rust rebuild"
> `epics.md`:535 "**When** the rebuild resolves the draft from canonical identities"
> `epics.md`:536 "**Then** the now-ineligible item is dropped or flagged with what
> changed, the plan is rebuilt from current canonical truth rather than the stale display,
> and a fresh review is required before anything can run."

**The `AD` text both obey:**

> `ARCHITECTURE-SPINE.md`:1176 "- **Rule:** A batch is **all-or-none**. One batch
> resolves through one canonical rebuild; if the rebuild errors or rejects, the prior
> coherent draft and its last authenticated preview are preserved unchanged (AD-16). A
> 100-row range never half-applies. Ineligible rows inside the range are simply not in the
> batch — item-level ineligibility bars them from `PlanIntent` entirely (AD-16)."

> `ARCHITECTURE-SPINE.md`:447 "- **Rule:** A canonical rebuild may remove or invalidate
> membership; it may never add a member the user has not seen."

> `ARCHITECTURE-SPINE.md`:1166 "- **Rule:** A batch carries **concrete canonical
> identities computed from the snapshot the user is looking at**, never a predicate for
> Rust to re-expand. … The count reported to the user is the size of that same set."

> `ARCHITECTURE-SPINE.md`:1222 "- **Rule:** The header checkbox's tri-state denominator
> is **the eligible set matching the active filter**, including off-screen virtualized
> rows — unchecked when none of that set is staged, `mixed` when some, checked when all. …
> The state is derived from the membership projection, never stored."

**The unowned question.** AD-28:1166 makes the batch a **frontend** computation — "the
snapshot the user is looking at", never a predicate Rust re-expands. AD-17 makes the
frontend an authority-free projection:

> `ARCHITECTURE-SPINE.md`:682 "- **Rule:** Rust owns the canonical `PlanIntent`. The
> Zustand draft store is a projection of the last authenticated rebuild — never the
> authority, never the author of executable text. Every mutation round-trips through Rust
> before the projection updates."

AD-28:1214 requires "**One predicate.** Exactly one eligibility-and-visibility predicate
serves the row, the header checkbox, `⌘A`, and the batch payload" — and **never says
which side of the IPC boundary that predicate lives on.** Both copies it names as the
shipping duplication are TypeScript (`ManagerPane` inline and `visibleSelectableIds`),
while AD-16's inertness and ineligibility rules (`ARCHITECTURE-SPINE.md`:625 ff.) make
item eligibility a canonical Rust property. So the numerator (membership) has one
authority and the denominator (eligibility × filter) may have another.

**What each builds.** A 500-row filtered Manager view; a refresh lands between render and
click; the batch of 500 reaches Rust, which now finds 3 pinned.

- *Story 3.5's builder* applies AD-28:1176 literally: the batch is all-or-none, the
  rebuild rejects 3 members, so the batch is rejected, "the prior coherent draft and its
  last authenticated preview are preserved unchanged", and the header stays `unchecked`.
  Nothing is staged. Clicking again reproduces it exactly.
- *UX-PB.1a's builder* applies AD-16:447 and its own criterion: the rebuild narrows, 497
  are staged, 3 are "dropped or flagged with what changed".

**What breaks at runtime.**

1. **The tri-state control wedges.** Under UX-PB.1a's build the denominator is 500
   (AD-28:1222 — the eligible set matching the active filter) and the projection holds
   497, so the header is **permanently `mixed`**. AD-28:1222 forbids storing the state
   ("derived from the membership projection, never stored"), so there is nowhere to record
   "497 is all that can be staged": the control can never reach `checked` and every
   further click re-derives the same result. The primary bulk-membership control for the
   product's core flow is stuck with no user-reachable exit.
2. **The count the user is told diverges for the identical act.** AD-28:1166 "The count
   reported to the user is the size of that same set" yields 500 on one build and 497 on
   the other — and `epics.md`:554 makes **UX-PB.1b** render those counts ("the draft
   grouped by Manager with `Updates`, `Managers`, and `Commands` counts") while AD-28's
   Binds omits UX-PB.1b entirely (see L-1).
3. **The batch verb's wire contract differs.** AD-28:1158 makes it "one atomic surface
   change under AD-3", so one build ships a verb whose success type is total-reject and
   the other ships one whose success type is applied-plus-dropped — two shapes for one
   fixture-backed model, which is exactly what AD-3 exists to prevent.

Both sides are AD-compliant: AD-28:1176 and AD-16:447 are each satisfiable and give
opposite answers for the same input, because AD-28's "Ineligible rows inside the range are
simply not in the batch" silently assumes the frontend already knew they were ineligible —
which is the case AD-28:1166 itself says cannot be assumed ("a refresh landing between
render and click").

**Proposed new `AD` text — AD-28, replacing the all-or-none rule and extending the
one-predicate rule:**

> - **Rule:** A batch is **all-or-none in application and narrowing in resolution**, and
>   the two are not in tension. One batch resolves through one canonical rebuild; the
>   rebuild applies every member it still finds eligible and drops the rest **as one
>   transaction**, reporting the dropped refs and their reasons. A batch never
>   half-applies *silently* and never applies *partially without saying so*; it is
>   rejected outright only when the rebuild errors, in which case the prior coherent draft
>   and its last authenticated preview are preserved unchanged (AD-16). This is the batch
>   form of AD-16's "a canonical rebuild may remove or invalidate membership; it may never
>   add a member the user has not seen" — the narrowing direction is already lawful and
>   batching does not make it unlawful.
> - **Rule:** The **one predicate is Rust's, projected to the frontend together with the
>   snapshot it was computed against.** The frontend derives a batch from that projected
>   result and submits concrete identities plus the snapshot token it read; Rust rejects a
>   batch whose token is not its current snapshot. That is what makes AD-28's "never a
>   predicate for Rust to re-expand" and AD-16's canonical-eligibility rule one rule
>   instead of two, and it is what AD-17's projection rule already implies for every other
>   plan value. The tri-state denominator, the row, `⌘A` and the batch payload all read
>   that one projected result, so a narrowed batch narrows the denominator with it and the
>   header can reach `checked`.

---

# HIGH

## H-1 — AD-29's sole terminal appender is ordered strictly after every story that drives an attempt terminal, so shippable builds persist admission-only attempts the fold must read as `Interrupted`

**Story A: UX-PB.2e** — *Plan-level cancellation …*

> `epics.md`:741 "**And** every prior outcome is preserved."
> `epics.md`:745 "**Then** the work that could not be stopped is reported honestly and
> never falsely marked cancelled, the successfully cancelled and skipped outcomes remain
> preserved"
> `epics.md`:732 "**Blocks:** UX-PB.3 (on UX-PB.2 completion)"

**Story B: UX-PB.4a** — *One immutable History row per confirmed attempt*

> `epics.md`:916 "**Dependencies:** D29; AD-16 (durable `planAttemptId` identity; atomic
> all-or-none admission); AD-18; UX-PB.3 complete (PB.3a-g); AD-27 …"
> `epics.md`:923 "**Given** a confirmed plan attempt that reaches a terminal state —
> succeeded, failed, cancelled, interrupted, or partially skipped …"
> `epics.md`:924 "**When** it terminates"
> `epics.md`:925 "**Then** exactly one immutable History row is created for that
> `planAttemptId` …"

**The `AD` text both obey:**

> `ARCHITECTURE-SPINE.md`:1246 "- **Rule:** **One append authority:** the Rust
> plan-attempt store. … `epics.md` already assigns the two call sites and this rule
> ratifies rather than overrides them: UX-PB.2c appends at admission, **UX-PB.4a appends
> at terminal**, and UX-PB.3d "renders and announces Results; it never writes a durable
> record itself"."

> `ARCHITECTURE-SPINE.md`:1293 "- **Rule:** An attempt is a **fold** over its records,
> resolved in one direction only. Admission plus terminal yields the terminal record's
> outcome. **Admission with no terminal record yields `Interrupted` once no live attempt
> owns it** … No completed outcome is ever synthesized (AD-19)."

**Why an intermediate build is real.** `ARCHITECTURE-SPINE.md`:408 "A conventional commit
reaching `main` enters release automation with no later human gate, so work stays off
`main` until it is ready to ship" — and each UX-PB story is a shippable vertical slice
(`epics.md`:381–385, "28 dependency-ordered vertical sub-stories … each delivering one
shippable behavior"). The span between two stories is therefore a shipping configuration,
not scaffolding.

**What each builds.** UX-PB.2e lands in wave 2 and makes an attempt reachable terminal by
cancellation. AD-29 forbids it the terminal append, so it persists nothing at the attempt
level and satisfies "every prior outcome is preserved" at the Operation level only, which
AD-29 sanctions ("Per-Operation `Verifying` and `Skipped` remain durable states in the
**Operation** journal"). UX-PB.4a — ordered after **all** of UX-PB.3a–3g — is the only
story permitted to write the attempt's terminal record.

**What breaks at runtime.** From UX-PB.2e landing until UX-PB.4a lands, every attempt that
terminates — cancelled by UX-PB.2e, completed by UX-PB.3d, cancelled mid-verify by
UX-PB.3g — persists as **admission-only**. On the next launch AD-29's fold is *required*
to read each as `Interrupted`. A user who watched `12 of 12 updates verified`
(`epics.md`:836) and dismissed Results sees `Interrupted` in History. AD-29:1293 forbids
the obvious repair ("No completed outcome is ever synthesized"), and AD-19 forbids the
other one — a journal record "is never silently replaced by a synthesized one".

**The steady-state half is also unbounded.** AD-29 fixes ordering for the admission append
only (`ARCHITECTURE-SPINE.md`:1281 "Ordering is mint-and-admit, **then** append") and
states `ARCHITECTURE-SPINE.md`:1286 "The append **gates nothing**". No rule bounds the
window between an attempt's terminal transition and its append, so a force-quit inside
that window loses the outcome permanently in the finished product too.

**Proposed tightened `AD` text — AD-29, replacing the append-authority rule's call-site
sentence:**

> The terminal append fires **on the attempt's terminal transition inside the Rust
> plan-attempt store**, in the same critical section that makes the transition, and it is
> owned by **whichever story first makes an attempt reachable terminal — UX-PB.2e** — not
> by the History story that reads it. UX-PB.4a is a **reader and folder only** and appends
> nothing; its "one immutable History row" is the fold's output, not a write. This
> corrects an ordering the story list cannot satisfy: UX-PB.4a depends on all of
> UX-PB.3a–3g while UX-PB.2e ships in wave 2, so assigning the append to UX-PB.4a leaves
> every attempt terminated in between persisted as admission-only — which this AD's own
> fold rule must then read as `Interrupted`. No window may exist between a terminal
> transition and its append in which a force-quit loses the outcome.

## H-2 — Story 6.5 asserts an exported record shape AD-29 makes impossible, and no rule says whether diagnostics exports raw journal lines or a fold

**Story A: Story 6.5** — *Export Exact Native Diagnostics and Visible Outcomes*

> `epics.md`:1314 "**Then** it contains `report.json`, the newest three app logs, newest
> 25 transcripts, `operations.jsonl`, and the durable plan-attempt records that correlate
> the exported evidence — **each carrying its `planAttemptId`, reviewed Manager/Package
> scope, exact commands, verification facts, results**, and optional `retryOfPlanAttemptId`
> — with exact expected contents and no missing required entry, including those
> plan-attempt entries."
> `epics.md`:1301 "- Required test level: Real native Tauri E2E plus artifact inspection."

**Story B: UX-PB.2c** — owner of the record's field list

> `epics.md`:693 "**Then** the append-only record stores the reviewed Manager/Package
> scope, Manager self-update identities, exact command snapshot, version evidence,
> timestamps, and result/verification state as immutable plan-admission metadata"
> `epics.md`:316 "| Plan-attempt journal filename and serde shape | `DEFERRED` |
> Development | Owned by Story UX-PB.2c. …"

**The `AD` text:**

> `ARCHITECTURE-SPINE.md`:1265 "- **Rule:** Verification and result state ride the
> **terminal** record, and this **deliberately overrides UX-PB.2c's stated record
> contents** … The admission record carries reviewed intent, the exact command snapshot,
> identities, and timestamps"

> `ARCHITECTURE-SPINE.md`:773 (AD-18) "- **Rule:** Diagnostics export carries both
> journals as distinct entries alongside `report.json`, the newest three app logs, and the
> newest 25 transcripts. Existing retention bounds are unchanged."

**What each builds.** UX-PB.2c writes AD-29's split: the admission record carries intent +
command snapshot + identities + timestamps and **no** result; the terminal record carries
verification + result and **no** command snapshot. Story 6.5's artifact-inspection
assertion demands **each** exported plan-attempt record carry both halves. Under AD-29 no
such record can exist.

**What breaks at runtime.** Story 6.5's suite — the most expensive level in the plan and
the one AD-26 exists to make admissible at all — fails against a fully compliant writer.
Its only compliant alternative is to export a **folded** attempt object carrying both
halves, and nothing authorises that: AD-18:773 authorises "both journals as distinct
entries" (raw NDJSON), AD-29:1293 makes the fold a read-model, and AD-29 plus AD-19 forbid
synthesised records. The Deferred row hands the field list to UX-PB.2c
(`ARCHITECTURE-SPINE.md`:1422) and says nothing about the exported shape, so the reader
and the writer are each free to assume the other's form.

**Which side is the defect.** Story 6.5's criterion contradicts AD-29:1265 and is a
**story defect** — and it is not in the residuals row. The **spine hole is separate and
survives fixing it**: AD-18 and AD-29 together never state whether the diagnostics archive
contains journal lines or folded attempts, and AD-18's "A story that adds a field to the
attempt record owns its disclosure review" assumes records, not folds.

**Proposed new `AD` text — AD-29, new rule:**

> - **Rule:** The **fold is a read-model and is never serialized as evidence.**
>   Diagnostics export ships the plan-attempt journal's raw lines as one entry, exactly as
>   AD-18 ships `operations.jsonl` — two lines per attempt, admission and terminal — never
>   a folded or summarized attempt object, because a fold is derived and AD-19 forbids a
>   synthesized record standing in for a real one. A reader that wants the folded view
>   runs the fold. A story asserting the archive's contents asserts **records**, and no
>   single record carries both the command snapshot and the result: Story 6.5's "each
>   carrying … exact commands, verification facts, results" describes a shape this AD
>   forbids and must be restated per record kind.

## H-3 — `PlanAttempt.state`'s four values have no durable home, and the attempt's state has two owners: a field on a record and the fold

**Story A: UX-PB.2c** — writer, and owner of the field list (`epics.md`:316;
`ARCHITECTURE-SPINE.md`:1422)

**Story B: UX-PB.4a** — the fold

> `epics.md`:937 "**When** History folds that `planAttemptId`'s records into its row"
> `epics.md`:938 "**Then** the row is presented as `Interrupted` **only when the absence
> is genuine** — a terminal record that exists but failed to parse is reported as
> unreadable evidence rather than silently reclassifying a finished attempt as unfinished,
> and the fold states which it was."

**The `AD` text:**

> `ARCHITECTURE-SPINE.md`:546 "Names may be refined during story implementation; the
> semantic separation is fixed."
> `ARCHITECTURE-SPINE.md`:580 "  state: admitted | running | verifying | terminal"

> `ARCHITECTURE-SPINE.md`:1255 "- **Rule:** **Exactly two records per attempt: admission
> and terminal. Not per transition, and not "several".**"

> `ARCHITECTURE-SPINE.md`:1318 "- **Rule:** A record is never mistaken for liveness. Only
> an attempt the running process actually owns is active; a journal record read at launch,
> **whatever state it names**, is history."

AD-16 declares `state` a field of the durable `PlanAttempt` under a heading that fixes the
semantics. AD-29 provides no record in which `running` or `verifying` can ever appear, and
AD-29:1318 presumes a record that *does* name a state — so the field is simultaneously
normative, half-unrepresentable, and unarbitrated against the fold.

**What each builds.** UX-PB.2c stamps `state: "admitted"` into the admission record,
because AD-16 makes it a field of the entity that record represents and nothing forbids
it. UX-PB.4a derives state from the fold, because AD-29:1293 says an attempt *is* a fold.
Equally lawfully, UX-PB.2c omits `state` entirely — and then UX-PB.4b's replay
(`epics.md`:953, which reconstructs "Operation outcomes, errors, timings, and retained
output") and Story 6.5's export read a field that is not there.

**What breaks at runtime.** A crashed attempt's record says `admitted` while History says
`Interrupted`, and Story 6.5's archive ships the record (AD-18:773) — so a support bundle
and the UI disagree about the same attempt, which is precisely the correlation NFR-4
(`epics.md`:109) exists to guarantee. Separately, `running` and `verifying` are declared
variants of a fixture-backed model whose TypeScript half "asserts its fixture set exactly
equals its guard map" (`ARCHITECTURE-SPINE.md`:223 ff., AD-3) with no producer — dead
variants in one build, omitted in the other.

**Proposed tightened `AD` text — AD-16's normative domain minimum, plus an AD-29 rule:**

> `PlanAttempt.state` is a **derived read-model value, not a persisted field.** No journal
> record carries it. `admitted` and `terminal` are implied by which records exist;
> `running` and `verifying` exist **only in the live process's memory** and are
> unrepresentable durably by design — which is why a relaunch reads an unfinished attempt
> as `Interrupted` rather than as `running`. AD-29's fold is the single authority for an
> attempt's state, and AD-29's liveness rule's "whatever state it names" refers to the
> Operation journal's per-step states, never to an attempt-level `state` field, because no
> such field is written.

## H-4 — Below 720 CSS pixels, AD-17's single region and the `ActiveView` replay both claim the whole workspace: UX-PB.4c requires both visible, UX-PB.5d forbids overlap — and a safety-critical state goes off-screen

**Story A: UX-PB.4c** — *Live and replay coexistence with the live attempt primary*

> `epics.md`:972 "**Then** the live sidecar stays visibly live, full Activity is labeled
> `Viewing past activity`, `Back to live activity` is offered, and choosing it returns the
> main workspace to the one active attempt without disturbing its progress."
> `epics.md`:976 "**Then** the live attempt remains the primary object with authoritative
> sidecar and Results, and the concurrent replay never suppresses, delays, or overwrites
> live status."

**Story B: UX-PB.5d** — *Accessibility and responsiveness of the confirmation and safety
surfaces*

> `epics.md`:1121 "**Then** below 720 usable CSS pixels the layout enters high-zoom mode,
> navigation collapses to an accessible rail or temporary panel, and
> Plan/Confirmation/Activity/Results present as a full-workspace or stacked surface with a
> visible Back route, **no overlapping panes**, and no two-dimensional scrolling for the
> primary task, keeping every safety action reachable."

**The `AD` text both obey:**

> `ARCHITECTURE-SPINE.md`:709 "- **Rule:** The sidecar is a single layout region … **Exactly
> one instance exists** and it persists across `ActiveView` changes without losing
> membership or scroll identity."

> `ARCHITECTURE-SPINE.md`:732 "- **Rule:** Below 720 usable CSS pixels the region stops
> being a fixed sidecar and the same single instance is presented as a full-workspace or
> stacked surface. Viewport is a placement driver, never a second mount point."

> `ARCHITECTURE-SPINE.md`:735 "- **Rule:** Activity is a first-class destination in the
> existing discriminated `ActiveView` state — for the active attempt and **for replaying a
> completed History entry** — not a drawer and not a sidecar mode."

> `ARCHITECTURE-SPINE.md`:744 "- **Rule:** A safety-critical attempt state reaches the
> user through a **visible** surface and never depends on an announcement channel. The
> stall handoff and `Interaction required` are the two that qualify, and both are visible
> in the region that owns the attempt."

**What each builds.** At 900 × 600 or 200 % zoom, with a live attempt running and a
History replay open, the region is a full-workspace surface (AD-17:732) and the replay is
a full-workspace `ActiveView` destination (AD-17:735). UX-PB.4c's builder keeps the live
surface persistently on screen — two panes, split or overlaid. UX-PB.5d's builder stacks
them behind a Back route with "no overlapping panes" — one visible at a time.

**What breaks at runtime.** 4c's build violates 5d's "no overlapping panes" and "no
two-dimensional scrolling for the primary task" at a width `epics.md`:107 (NFR-3) declares
supported. 5d's build puts the **live** attempt behind the replay, breaking 4c's "the live
sidecar stays visibly live" — and takes AD-17:744's safety-critical states with it:
`Interaction required` and the stall handoff render into a surface that is not on screen,
while `ARCHITECTURE-SPINE.md`:754 makes the announcement channel that used to reach a user
looking elsewhere **optional** ("No story is obliged to build a status-announcement
channel"). The attempt then sits to the 30-minute hard cap (`epics.md`:79, FR-14) with the
handoff invisible and nothing narrating it. This is the exact mechanism revision 10 says
it was closing — AD-17:744 "This has to be said now because it used to be carried
implicitly: the rule this replaces routed them through an assertive live region, so speech
was the mechanism that reached a user looking elsewhere" — and the new rule does not
survive AD-17's own responsive rule twelve lines above it.

**Proposed tightened `AD` text — AD-17, extending the visible-surface rule and the
sub-720 rule:**

> - **Rule:** "Visible" means **on screen, not merely mounted.** When a safety-critical
>   attempt state is entered — the stall handoff or `Interaction required` — the region
>   that owns the attempt is brought to the top of whatever presentation the viewport
>   dictates, and any stacked or replay surface yields to it with its return route
>   preserved. A state rendered into a surface the user cannot see does not satisfy this
>   rule, and below 720 usable CSS pixels that is the default outcome unless this clause
>   forces it.
> - **Rule:** Below 720 usable CSS pixels the region and an `ActiveView` replay are **one
>   stack, not two panes**, with the live attempt **permanently reachable in one action**
>   and its liveness indicated on the replay surface itself. UX-PB.4c's "the live sidecar
>   stays visibly live" is satisfied at that width by a persistent live indicator plus
>   `Back to live activity`, never by a second simultaneous pane — which UX-PB.5d's "no
>   overlapping panes" forbids. Above 720 pixels both are visible as sidecar plus
>   workspace, unchanged.

## H-5 — UX-PB.2e ships the forbidden `Cancelling` variant before the correct-course run that would remove it, and the addition is an atomic AD-3 change to a durable replayed state

The misattribution that made this invisible was fixed mid-review (see *Closed by
concurrent edits*). **The consequence is not recorded anywhere and is still open.**

**Story A: UX-PB.2e**

> `epics.md`:740 "**Then** cancellation operates only on the Operation IDs bound to that
> `planAttemptId`: running work moves to `Cancelling` and escalates through the existing
> process-group mechanics …"
> `epics.md`:732 "**Blocks:** UX-PB.3 (on UX-PB.2 completion)"

**Story B: UX-PB.3c** — *Per-item live progress states*, the compliant reference nobody names

> `epics.md`:814 "**Then** it shows queued, waiting (with the lock or ownership reason),
> running (indeterminate unless the adapter provides a trustworthy total), verifying, or a
> terminal state, and a row or status update never moves focus."

A five-state closed list with **no `Cancelling`**, in the story that renders live per-item
state.

**The `AD` text:**

> `ARCHITECTURE-SPINE.md`:499 "- **Rule:** **There is no `Cancelling` state, durable or
> otherwise.** … A builder must not add the variant."

> `ARCHITECTURE-SPINE.md`:424 ff. (AD-16) "`OpStatus` ships seven variants today, so every
> addition moves as one atomic AD-3 change across the Rust enum, `src/lib/ipc/types.ts`,
> the guards, and `dev/fixtures/ipc/*.json`."

> `ARCHITECTURE-SPINE.md`:424 ff. (AD-16) "`Verifying` and `Skipped` are durable
> wire-level operation states … They are journaled, exported in diagnostics, and replayed
> from History, so a derived state could not survive a crash or a replay."

**What breaks.** UX-PB.2e is a wave-2 story that **blocks all of UX-PB.3**, so it starts
before UX-PB.3c exists and before the residuals row's `bmad-correct-course` run is
scheduled to land. Its own criterion requires the variant, so its builder adds an eighth
`OpStatus` variant with a committed fixture and a guard entry. UX-PB.3c then ships the
five-state renderer AD-16 sanctions and never emits or handles it. Withdrawing the variant
afterwards is a **second** atomic AD-3 change plus a fixture regeneration against a state
AD-16 has already made durable and replayable — any `Cancelling` already journaled becomes
unreadable, which AD-19 forbids resolving by synthesis. The residuals row records *what*
to restate; nothing records that the restatement must precede UX-PB.2e starting, or that
this one is not a wording fix.

**Proposed tightened `AD` text — AD-16, appended to the "no `Cancelling` state" rule:**

> **UX-PB.2e lands first and blocks all of UX-PB.3, so it is the story that would add the
> variant — its criterion must be corrected before it starts, not after.** The addition is
> not a label: it is one atomic AD-3 change across the Rust enum, `src/lib/ipc/types.ts`,
> the guards and `dev/fixtures/ipc/*.json`, on a state this AD makes durable, journaled and
> replayed, so withdrawing it later leaves history that cannot be read and that AD-19
> forbids synthesizing away. **UX-PB.3c's five-state enumeration — queued, waiting,
> running, verifying, terminal — is the compliant reference.**

---

# MEDIUM

## M-1 — AD-28's `⌘A` native-default rule has no story owner: RP-2 is assigned to the release checklist and Story 3.5's criteria never leave the row list

> `ARCHITECTURE-SPINE.md`:1228 "- **Rule:** An accelerator that shadows a standard
> Edit-menu action suppresses the native default **only on surfaces where it performs its
> own action**. … bound because the shipping handler violates it today: `⌘A` calls
> `preventDefault()` before its helper early-returns on views with no Package list, so on
> the Dashboard, History and Settings it blocks native select-all and puts nothing in its
> place."

**Story A: Story 3.5** — the only story that names `Cmd+A`, with a Given that is a row
list (`epics.md`:1277–1278). The Dashboard, History and Settings appear in no criterion of
it.

**Story B: none — the requirement is assigned to a document**

> `epics.md`:454 "RP-2: Release checklist — Validate standard macOS Edit/Window menu
> behavior."

**What breaks.** Story 3.5's builder fixes `⌘A`'s sink for the Manager workspace, which is
all its criteria describe; the global handler keeps suppressing native select-all on three
other views. RP-2's checklist step then fails at release with no story to route the fix
to — and AD-1 forbids absorbing it there: `ARCHITECTURE-SPINE.md`:206 "- **Rule:** Missing
or incorrect behavior is product work, not test work." AD-11 agrees the checklist is not
where behaviour is built. AD-28's Binds names "the application accelerator map (`prd.md`
RP-2)" — a requirement, not a unit that can build.

**Proposed tightened `AD` text — AD-28, appended to the accelerator rule:**

> The **global key handler is product code with a story owner, and the owner is Story
> 3.5**, whose scope widens from its row list to the handler itself: the `⌘A` sink, the
> surfaces on which it acts, and the surfaces on which it must not call
> `preventDefault()`. RP-2's release-checklist entry validates that behaviour; it never
> owns it (AD-1, AD-11). An accelerator defect on a view no story's Given mentions is
> still that story's work.

## M-2 — AD-22's rider-failure notice must be "surfaced inline" in a surface AD-22's own ordering guarantees is already gone

**Story A: UX-PB.5b**

> `epics.md`:1065 "**And** the opt-out never precedes the admission it rides on; if that
> atomic save then fails, the admitted attempt stands, the prior `false` preference is
> retained as both active and persisted state, and the failure is surfaced inline."

**Story B: UX-PB.3a**

> `epics.md`:777 "**When** final confirmation closes the Confirmation Dialog"
> `epics.md`:778 "**Then** the same Upgrade Sidecar transforms in place into the one
> active plan summary for that `planAttemptId` …"

> `ARCHITECTURE-SPINE.md`:891 "- **Rule:** Ordering is fixed — validate, admit through the
> scheduler's revision-checked transaction, then persist the rider once the admission has
> returned."
> `ARCHITECTURE-SPINE.md`:897 "- **Rule:** A rider that **reduces** a safety default
> commits only if the action it rode on succeeded. … On successful admission with a failed
> rider save the attempt stands, the prior preference is retained as both active and
> persisted state, and the failure is surfaced inline."

AD-22's mandated ordering puts the rider's save strictly **after** admission returns, and
UX-PB.3a's criterion closes the dialog **at** that moment and repurposes the region.
"Inline" therefore names a surface that no longer exists. AD-17's visible-surface rule
cannot absorb it: `ARCHITECTURE-SPINE.md`:744 names a **closed set of two** — "The stall
handoff and `Interaction required` are the two that qualify" — and a silently failed
safety-default write is not one. Harm direction is safe (the gate stays armed), which is
why this is MEDIUM: the user believes the opt-out took and is told nothing.

**Proposed tightened `AD` text — AD-22, appended to the rider rule:**

> "Surfaced inline" means **in the region that owns the attempt the rider rode on**, not
> in the confirmation dialog — this AD's own ordering guarantees the dialog has closed and
> UX-PB.3a's criterion has already transformed the region by the time the rider resolves.
> A failed safety-default write is a third state AD-17's visible-surface rule covers,
> alongside the stall handoff and `Interaction required`.

## M-3 — "The snapshot the user is looking at" against virtualization: rendered window or ordered filtered set

> `ARCHITECTURE-SPINE.md`:1166 "… a range is an anchor and a target over the order the
> user is looking at, so the derivation reads **that same rendered order and nothing
> else**."
> `ARCHITECTURE-SPINE.md`:1222 "… **the eligible set matching the active filter**,
> including **off-screen virtualized rows** …"

The product ships TanStack React Virtual (`ARCHITECTURE-SPINE.md` Stack table), so at 500
rows roughly twenty are rendered. A shift-range from an anchor to a target 400 rows away
spans rows never rendered. **Story 3.5** (owns the range, `epics.md`:1278) can read
"rendered order" as the DOM window; **Story 3.1** (owns the tables — `epics.md`:1207,
"name, installed/latest values, status text, eligibility, selection, and the row plan
action …") supplies the ordered filtered list the header's denominator must use. Two
lawful readings of one phrase, in the same rule set that fixes the denominator explicitly
to include off-screen rows.

**Proposed tightened `AD` text — AD-28, in the batch-derivation rule:** replace "that same
rendered order and nothing else" with "**the ordered filtered set the rendered window is a
view of — the same set the tri-state denominator uses, off-screen rows included.
'Rendered' names the ordering, never the subset.**"

---

# LOW

## L-1 — UX-PB.1b renders the counts AD-28 defines and is not bound by AD-28

`epics.md`:554 "**Then** the Upgrade Sidecar opens showing the draft grouped by Manager
with `Updates`, `Managers`, and `Commands` counts …" consumes AD-28's "The count reported
to the user is the size of that same set", while AD-28's Binds
(`ARCHITECTURE-SPINE.md`:1133 ff.) omits UX-PB.1b. Add it.

## L-2 — UX-PB.1d claims to block Story 3.2; Story 3.2's Dependencies do not agree

`epics.md`:600 "**Blocks:** Story 3.2" against `epics.md`:1225 "- Dependencies: Story 3.1;
deterministic plan-builder and UI fixtures", with `epics.md`:501 recording that the
surviving Epic 1–6 stories "carry no inter-epic dependencies". The edge exists in one
direction only, so nothing sequences the inertness contract even after the residuals row's
wording fix lands. Add UX-PB.1d to Story 3.2's Dependencies.

## L-3 — The Crash/relaunch lifecycle-controller row's live-consumer list omits UX-PB.4a

`ARCHITECTURE-SPINE.md`:1421 lists "UX-PB.1b, UX-PB.2f, UX-PB.4e, and Story 6.5".
UX-PB.4a asserts relaunch reconciliation (`epics.md`:932–934) and AD-29 makes it the fold's
owner; AD-5's Binds (`ARCHITECTURE-SPINE.md`:306 ff.) covers UX-PB.2c but not UX-PB.4a.
Add UX-PB.4a to both.

## L-4 — "Where applicable" is undefined for verification refreshes

`epics.md`:725 "**Then** crash-journal start/finish records, diagnostics, and verification
refreshes carry the same `planAttemptId` where applicable" against AD-18's field-presence
rule, which is scoped to Operation-journal records and says of itself "this is *field
presence*, and conditional — it is not a cardinality rule". A verification refresh has no
`opId` until it becomes an Operation, so UX-PB.2d and UX-PB.3d can read "applicable"
oppositely. Correlation degrades; nothing wedges.

---

## Attacks run and what they returned

| Charter attack | Result |
| --- | --- |
| 1. AD-28 tombstone symmetry vs AD-23; what is a *range* removal | **C-1** — a range is neither shape the rule names, and `Clear`, an `Everything`-scope removal and `⌘A`-as-remove are equally unmapped; "their tombstones" is ambiguous |
| 2. AD-29's two records vs AD-16's four-valued `PlanAttempt.state`; where `running` becomes durable | **H-3** — nowhere, and the field has two owners; and **H-1**, the terminal record has no writer for the span that matters |
| 3. AD-28's snapshot identities vs AD-16's rebuild and AD-17's projection; who computes the set | **C-2** — the one predicate's side of the boundary is unassigned, so one batch has two lawful outcomes and the tri-state wedges; **M-3** on the virtualized reading |
| 4. AD-17's visible-surface rule vs the stall and `Interaction required` paths | **H-4** — below 720 px the region is a stacked surface that can be off-screen while the announcement channel is now optional; **M-2** — the rider failure has no surface at all |
| 5. Open/Deferred rows letting two stories diverge silently | **H-2** (serde-shape row is silent on the exported shape), **L-3** (incomplete live-consumer list), **L-2** (asymmetric story edge). The `⌘L` / `Esc` second-rung row is properly disclosed and is **not** re-raised |
| Found in revision 10's new text, outside the charter | **H-5** — the `Cancelling` variant reaches the wire in wave 2, before the run that removes it; **M-1** — AD-28's `⌘A` rule binds a requirement, not a builder |

## What this lens did not find

- **No divergence on AD-29's fold semantics** — direction, the `Interrupted`-versus-
  unreadable distinction, idempotence, or the liveness rule. AD-29 closes each of the ways
  two stories could have read them, including the wedge it names itself ("a dead attempt
  resolving as live refuses **every** subsequent confirmation, permanently").
- **No divergence on AD-24's Retry/draft separation, or on AD-23's `Explicit` dominance on
  the addition side.** Both are total over the acts the bound stories perform. The removal
  side is C-1.
- **No divergence on AD-27's focus mechanism.** Its Binds now enumerates 3.1, 3.2, 3.4,
  3.5 and 6.5 explicitly and every UX-PB story cites it inline.
- **AD-16's three new rules — the three opt-out compensations, `tool:rust` as the excluded
  side, and confirmation-unavailable-during-rebuild — are each stated tightly enough that
  a compliant builder cannot diverge.** One reconcile note, not a divergence: **no
  `epics.md` criterion carries the confirmation-unavailable rule on the confirmation-*on*
  path.** UX-PB.5a enables the action on non-emptiness alone (`epics.md`:1030, "**Then** it
  contains exactly one blue `Confirm N Updates` action where N is the count of staged
  updates") while UX-PB.5c *does* carry the guard for the bypass path (`epics.md`:1104,
  "**Then** the run is blocked, the invalidated details are replaced and what changed is
  explained"). AD-16 binds all 28 UX-PB stories and gives one answer, so this belongs in
  the `epics.md` residuals row, not in an `AD`.
