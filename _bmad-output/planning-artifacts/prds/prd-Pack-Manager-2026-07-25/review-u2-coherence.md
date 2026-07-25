# Internal-coherence review — `prd.md` after the revision-10 Update pass

**Reviewed:** `_bmad-output/planning-artifacts/prds/prd-Pack-Manager-2026-07-25/prd.md`
(768 lines, `wc -l`), read end to end 2026-07-25, against the working tree at
`chore/restore-phase-2-prd`.
**Lens:** internal coherence only — cross-references that resolve, sections that agree
with each other, one rule stated once.
**Method:** full read, then `git diff` against `HEAD` (`faa1a3e`) to isolate the 77
inserted / 18 deleted lines the Update pass wrote, then targeted verification of every
positional citation the pass touched against `ARCHITECTURE-SPINE.md` (revision 10,
`artifact_revision: 10` at `:11`) and `docs/DECISIONS.md`.

## Overall verdict

The Update pass is substantively sound. All fourteen new **Architecture binding** blocks
say what the AD they cite actually says — I opened AD-28 (`:1181`), AD-29 (`:1326`),
AD-30 (`:1450`) and AD-17 (`:774`-`780`) and each binding is faithful, including §9.2's
verbatim quotes of the AD-28 self-contradiction at `:1272`–`1278`. The revision-10 stamp
landed at both places. The five requirement fixes named in §9.2 are all present in the
FRs they claim to have changed.

The defects are in the **seams the pass moved past**: §9.1 still carries a pre-revision-10
instruction to `bmad-architecture` that revision 10 already discharged; the 5→4 renumber
left "Open Question 1" pointing at a question that no longer holds that ordinal; the
`Shipping`→`Partial` rule the pass wrote down for FR-18 was applied to FR-18 alone while
the pass simultaneously created a fresh instance of the same defect in RP-2; and three
summary/scope passages (§7.2, FR-17's field count, FR-19's decision id) disagree with the
FRs they summarize.

**11 findings: 0 critical, 2 high, 6 medium, 3 low.** Nothing here blocks the handoff;
all eleven are text edits inside this document.

---

## HIGH

### H-1 — §9.1 instructs `bmad-architecture` to retire a spine row that revision 10 already closed, at a line number that no longer holds it

**Location:** `prd.md:740` (§9.1), against `prd.md:28` and `prd.md:742`.

> `ARCHITECTURE-SPINE.md:1050` still records this as OPEN and routes it to the owner. It is now answered, and that row should be **retired** in the `bmad-architecture` Update that follows this PRD.

The same pass that left that sentence standing wrote, at `prd.md:28`:

> Architecture invariants and `AD-` ids: `ARCHITECTURE-SPINE.md` **revision 10** — the sole authority for `AD-` numbering. Revision 10 added AD-28, AD-29 and AD-30 and narrowed AD-12

and titled §9.2 (`prd.md:742`):

> ### 9.2 Closed by `ARCHITECTURE-SPINE.md` revision 10 (Update pass, 2026-07-25)

Revision 10 **is** the `bmad-architecture` Update §9.1 is waiting for. The row is no
longer OPEN — `ARCHITECTURE-SPINE.md:1595` now reads:

> | Transient selection has no owning invariant | **RESOLVED** | Closed by **AD-28**, written 2026-07-25 on the owner's decision: `EXPERIENCE.md`'s model wins, the checkbox *is* membership, and the transient selection plus `Add Selected` layer is eliminated.

and the cited line number is stale twice over: `ARCHITECTURE-SPINE.md:1049` is
`  ever substituted.` (the tail of AD-25), `:1050` is blank, and `:1051` is
`### AD-26 — A native automation surface never reaches release bits`.

**Note.** This is the highest-cost finding for the stated audience. §9.1 is the section a
`bmad-architecture` Update reads to learn what this PRD asks of it; it now asks for work
that is already done, cites a line that resolves to unrelated AD-25/AD-26 text, and calls
the closing revision "the Update that follows this PRD" when it precedes the document as
it now stands. A run that follows it literally either no-ops after a confusing search or,
worse, reopens a settled invariant. It is also the one contradiction the pass could not
have missed by inattention — it is the pass's own subject matter.

**Fix:** replace the sentence with the closure. E.g. *"`ARCHITECTURE-SPINE.md` recorded
this as OPEN through revision 9; revision 10 closed it as **AD-28** and the register row
now reads RESOLVED (`ARCHITECTURE-SPINE.md:1595`). No further architecture action is
required for this question."* Cite the row by its title (`Transient selection has no
owning invariant`), not by line number — the spine's own register at `:1594` warns that
positional references in this run folder have failed three times.

---

### H-2 — FR-6 lists the pre-D27 behaviour as a testable consequence, contradicting the consequence two bullets above it

**Location:** `prd.md:236` (FR-6, inside **Consequences (testable):**).

> - The draft is transient dialog state discarded on close — pre-D27 behavior this FR removes.

Two bullets earlier, `prd.md:234`:

> - The draft persists while the user navigates between Managers and the Dashboard, and every staged item is individually removable from the Upgrade Plan.

and FR-7 at `prd.md:272`:

> - **Planned — D27:** the sidecar is hidden when empty, appears on first addition, persists across navigation, and offers Remove on every staged item.

**Note.** FR-6 already has the right home for this statement: an **Out of Scope:** block at
`prd.md:243`–`245`, which is where the other two removed behaviours live (`- A transient
selection distinct from draft membership, and any \`Add Selected\` submit step. Both are
eliminated.`). The block a `bmad-create-epics-and-stories` run extracts wholesale is
"Consequences (testable)", and this is the only bullet in the whole FR set that states a
requirement's negation inside that block. Truncated at the em-dash — which is exactly what
an acceptance-criterion extraction does — it becomes "the draft is transient dialog state
discarded on close", the inverse of the FR's headline and of the product's central promise
in §4.2 (`prd.md:217`: *"nothing runs that was not staged and shown"*). The trailing
qualifier is the only thing standing between this bullet and a wrong story.

**Fix:** move the bullet into FR-6's **Out of Scope:** block and reword to the negative
form the block uses — *"- Transient dialog-state drafts discarded on close. Eliminated; the
draft is persistent (see the consequence above)."*

---

## MEDIUM

### M-1 — after the 5→4 renumber, "Open Question 1" and "Q1" name three different questions

**Location:** `prd.md:744` (§9.2), `prd.md:720` (§9 item 1), `prd.md:731` (§9.1).

`prd.md:744` — added by this pass:

> **Q — What happens on quit with work *queued* but not running, and on an OS-initiated shutdown? CLOSED: both decided by AD-30.** This was Open Question 1 and is no longer open.

`prd.md:720` — the surviving open list's item 1:

> 1. **Does a downloaded application update survive a relaunch that was not the update restart?** RP-1 requires that failed or interrupted downloads never present as Ready

`prd.md:731` — §9.1's label:

> **Q1 — Does a Package checkbox mutate Upgrade Plan membership directly, or is there a separate transient selection? CLOSED 2026-07-25: directly.**

**Note.** The pass removed the ordinal it then referred to by ordinal. "Open Question 1"
at `:744` means the *former* item 1 (quit/shutdown); the document's current item 1 is a
different question entirely; and §9.1 independently owns "Q1" for a third. `prd.md:725`
(`All four are non-blocking`) confirms the open list is a live 1–4 namespace, so item 1 is
a citable id. This is the same class the prior gate raised on §9.1 alone; the pass added a
third referent rather than collapsing the two. Any downstream artifact citing "prd.md §9
Q1" now has a one-in-three chance.

**Fix:** at `:744`, drop the ordinal — *"This was the quit-and-shutdown open question,
removed from §9's list by this pass."* And relabel `:731` out of the numeric namespace,
e.g. **"Closed: Package-checkbox membership model."**

---

### M-2 — the `Shipping`-with-a-Planned-limb rule §9.2 states for FR-18 was applied to FR-18 only, and the same pass created a new instance in RP-2

**Location:** `prd.md:565` + `prd.md:571` (RP-2), against `prd.md:751` (§9.2).

§9.2's stated rule, `prd.md:751`:

> - **FR-18 was `Shipping` with a closed contents list**, so an implementer could ship an archive with no plan-attempt records and believe the requirement met. It now carries a Planned — D29 limb for that journal's raw lines.

RP-2's tag, `prd.md:565`:

> **Status:** Shipping.

and the paragraph this pass added six lines below it, `prd.md:571`:

> **⌘L is a focus move, not a toggle. Planned — D27–D30 for the behavior; the registration ships.**

Two pre-existing instances of the same shape, unmoved: FR-9 (`prd.md:293` `**Status:**
Shipping.` with `prd.md:300` `- **Planned — D30:** only one confirmed Plan Attempt may be
active at a time.`), and FR-19, whose tag at `prd.md:468` is not one of the three §0
enumerates at `prd.md:22`–`24`:

> **Status:** Shipping for the current navigation model. The D30 navigation changes — Activity as a first-class destination, the Results surface, and one-plan-per-row History — are Planned.

**Note.** RP-2 is the sharp one because the pass *created* it: before this pass RP-2's ⌘L
was described as a shipped toggle, so `Shipping` was true; the pass correctly re-pointed
the behaviour to Planned and left the tag alone. §7.1 (`prd.md:657`) is titled "Shipping
today (1.0.1)" and a reader mapping tags to that list now records a functional-menu RP as
fully shipped while its ⌘L limb is D27–D30 work. §9.2's own words — "an implementer could
… believe the requirement met" — are the argument.

**Fix:** RP-2 → `**Status:** Partial. The Edit/Window re-declaration and every accelerator
registration ship; ⌘L's target is Planned — D27–D30, and ⌘A's re-point is Planned — D27
(FR-6).` Either apply the same rule to FR-9 and FR-19 or add one line to §0's status-tag
paragraph saying a `Shipping` FR may carry an additive `Planned` bullet — the document
currently does both without saying which is the convention.

---

### M-3 — §7.2 enumerates the immediate-execution paths differently from FR-6, and omits the two FR-6 calls load-bearing

**Location:** `prd.md:679` (§7.2), against `prd.md:235` (FR-6).

§7.2:

> - One persistent editable Upgrade Plan draft, replacing transient dialog state, with every path converging on it and **no** immediate execution from a row, header, or selection (D27).

FR-6:

> - Neither a checkbox nor a row action executes anything. **Three immediate-execution call sites are in scope for removal**, not one: the Package row action, and *both* direct Manager self-update paths — the Dashboard Manager card and the Manager workspace self-update card each invoke the self-update command directly today, bypassing the plan entirely. Scoping the D27 work to the row action alone would leave two unstaged mutation paths alive and breach SM-2.

**Note.** §7.2 is titled "Decided, not yet implemented (D27–D30)" and `prd.md:685` calls it
"the live build queue — 28 stories under `epic-ux-pb`". It is therefore the scoping list an
epics run reads. Its three sources ("row, header, or selection") match neither FR-6's three
call sites nor FR-6's model: FR-6 says the header checkbox and the selection *never
executed anything* ("Neither a checkbox nor a row action executes anything"), while both
Manager self-update cards — the two FR-6 says must not be forgotten — appear nowhere in
§7.2's clause. This is precisely the mis-scoping FR-6's last sentence warns against.

**Fix:** restate `:679` from FR-6 — *"…and **no** immediate execution from any of the three
call sites named in FR-6: the Package row action and both direct Manager self-update paths
(D27)."*

---

### M-4 — FR-19 assigns one-plan-per-row History to D30; FR-15 and §7.2 assign it to D29

**Location:** `prd.md:468` (FR-19), against `prd.md:398` (FR-15) and `prd.md:682` (§7.2).

FR-19:

> **Status:** Shipping for the current navigation model. The D30 navigation changes — Activity as a first-class destination, the Results surface, and one-plan-per-row History — are Planned.

FR-15:

> - **Planned — D29:** one immutable History row per confirmed Plan Attempt, with Operation evidence nested inside it and Activity replay from the row.

§7.2:

> - A durable `planAttemptId` correlating reviewed intent, Operations, events, transcripts, journal records, verification, Results, and Retry lineage — with History carrying one immutable row per confirmed attempt (D29).

The decision record agrees with FR-15 and §7.2, not FR-19 — `docs/DECISIONS.md:198`:
`## D29. A confirmed plan attempt is the durable Activity, Results, and History unit`,
against `docs/DECISIONS.md:218`: `## D30. One active plan, explicit Activity and Results,
and trusted interaction classification`.

**Note.** Impact is on sequencing, not on the requirement: D29 and D30 are separate waves in
`sprint-status.yaml`, and FR-19's bundling puts a D29 obligation in a D30 bucket for anyone
scheduling from the FR's status line rather than from §7.2. FR-19 is also the only place in
the document that attributes it to D30.

**Fix:** `prd.md:468` → *"The Planned navigation changes — Activity as a first-class
destination and the Results surface (D30), and one-plan-per-row History (D29) — …"*

---

### M-5 — FR-17 states the target Settings field count as seven, contradicting the bullet directly above it and FR-7

**Location:** `prd.md:444` (FR-17), against `prd.md:443` and `prd.md:273`.

`prd.md:444`, last sentence:

> The other seven fields are the target set.

`prd.md:443`, the bullet immediately above:

> - **Planned — D28:** `skipUpgradePlanConfirmation` is added with a safe default of `false` and is reversible in Settings.

and FR-7 at `prd.md:273`:

> The opt-out checkbox appears *only* in that dialog and persists `skipUpgradePlanConfirmation`. The safe default is confirmation enabled (`false`), and Settings can restore it.

**Note.** The target set is eight: the eight shipping rows in the table at
`prd.md:432`–`441`, minus retiring `autoOpenDrawer`, plus `skipUpgradePlanConfirmation`.
"The other seven fields are the target set" is only true of the *survivors of the shipping
table*, and the sentence as written excludes the field the previous bullet adds and FR-7
requires Settings to expose. The number is doubly hazardous here because §0.1 already
carries a different "seven" about settings — `prd.md:42`: `| §F11 and §5.9 \`Settings\` | 7
fields each, and the two lists disagree with each other | \`settings.rs\` \`Settings\` has
**8**`. A Settings story counting from FR-17 can land on seven fields with no opt-out
toggle and believe it matched the PRD.

**Fix:** *"The other seven survive; with `skipUpgradePlanConfirmation` added by D28, the
target set is again eight."*

---

### M-6 — FR-6's "see the Notes" no longer resolves: the pass inserted a second Notes block that does not contain the referent

**Location:** `prd.md:238` (FR-6, end of the ⌘A consequence).

> ⌘A must also stop suppressing the native select-all on surfaces that have no Package list — see the Notes.

The referent is `prd.md:259`, which is not headed "Notes":

> `[NOTE FOR PM]` — the re-pointed ⌘A must suppress the native select-all **only on surfaces where it actually stages something**. A shipping defect in the same handler currently violates this; it is diagnosed in `addendum.md` §4, and this FR must not inherit it.

Between the pointer and its referent the pass inserted `prd.md:249`:

> **Notes — the ⌘A predicate, corrected.** An earlier draft of this FR stated that "the shipping select-all already computes eligibility through one predicate" …

**Note.** FR-6 now has three note-shaped blocks (`:247` `**Notes:**`, `:249` `**Notes — the
⌘A predicate, corrected.**`, `:259` `` `[NOTE FOR PM]` ``). A reader following "see the
Notes" from a ⌘A sentence lands on the block titled "the ⌘A predicate, corrected" — which
is about the eligibility predicate and says nothing about native select-all suppression.
The pointer was already loose before the pass; the pass made it resolve to the wrong block.
The content is now also duplicated a third time at `prd.md:573` (RP-2's Architecture
binding), unlinked from either.

**Fix:** make the pointer explicit — `— see the` `[NOTE FOR PM]` `below, and RP-2's
AD-28 binding.`

---

## LOW

### L-1 — FR-14 uses "OS" for both sides of the quit carve-out, and asserts "every path" over a set the previous bullet excludes

**Location:** `prd.md:381`–`382` (FR-14, adjacent bullets).

`prd.md:381`:

> - **An OS-initiated shutdown or logout is deliberately excluded from that promise.** It gets **no dialog**.

`prd.md:382`, the next bullet:

> - **Architecture binding — `ARCHITECTURE-SPINE.md` AD-30:** the quit guard has **one enforcement point** and every path reaches it — the OS window-close request and `⌘Q` resolve to it exactly as the application-update path already does. One predicate, one dialog, one refusal; a second path that decides for itself is the defect.

**Note.** The wording tracks AD-30 (`ARCHITECTURE-SPINE.md:1457`–`1470`) faithfully, so this
is a compression artifact rather than a divergence: the spine separates these into two
`**Rule:**` entries and states "every path" first, while the PRD collapses them into
consecutive bullets in the reverse order, so the absolute lands after its own exception. The
word "OS" then carries opposite outcomes in adjacent sentences — an *OS window-close request*
reaches the guard, an *OS-initiated shutdown* deliberately does not. Recoverable on a careful
read; a skim of the binding bullet alone reads as "every quit path shows the dialog", which
is the promise `:381` exists to withhold.

**Fix:** qualify the binding's scope — *"…every **user-initiated** quit path reaches it — the
window-close request and `⌘Q` … (the OS-shutdown carve-out above is the one deliberate
exception, and is not a second decision point: it runs the kill hook, not the guard)."*

---

### L-2 — §9.2's record of the pass omits two requirement changes the pass made

**Location:** `prd.md:746` (§9.2), against `prd.md:448` and `prd.md:571`.

`prd.md:746`:

> **Four requirement defects were corrected in the same pass**, each found by reconciling against a revision-10 AD rather than by review of the prose:

Two AD-17 reconciliations the pass wrote appear in no §9.2 bullet. `prd.md:448`:

> **Notes — `autoOpenDrawer` is reconciled, not restated.** Two sources describe its end state differently and the later one is stronger. `docs/DECISIONS.md:193` says the "obsolete `autoOpenDrawer` setting becomes inactive legacy input" — inert but still present.

and `prd.md:571`:

> **⌘L is a focus move, not a toggle. Planned — D27–D30 for the behavior; the registration ships.**

**Note.** Both are substantive: FR-17's status line moved a setting from *demoted* to
*retired*, and RP-2's ⌘L moved from a toggle to a focus jump. §9.2 is the only place the
document records what the pass changed, and it is what a reader diffing against the
revision-9-era PRD in `HEAD` will consult. FR-17's `[NOTE FOR PM]` at `prd.md:450` also
leaves a second reconciliation deliberately unfixed (`D28's sentence still reads as
demotion`), which sits oddly beside §9.2's "**One divergence is recorded, not fixed**" at
`prd.md:753` — that claim is spine-scoped and so survives, but only just.

**Fix:** add a third §9.2 paragraph — *"Three passages overtaken by AD-17 were corrected
without being defects: RP-2's ⌘L (toggle → focus move), FR-6's Esc cascade (two rungs →
one), and FR-17's `autoOpenDrawer` (demotion → retirement). The first and third leave
`docs/DECISIONS.md` wording behind them; see FR-17's `[NOTE FOR PM]`."*

---

### L-3 — the eliminated submit control is named two ways

**Location:** `prd.md:45` (§0.1 table) against `prd.md:244` (FR-6) and `prd.md:739` (§9.1).

`prd.md:45`:

> **F5 was never added to that list**, so SPEC's transient-selection-plus-`Add N to Plan` model reads as current when D27 superseded it.

`prd.md:244`:

> - A transient selection distinct from draft membership, and any `Add Selected` submit step. Both are eliminated.

`prd.md:739`:

> - FR-6 is rewritten around direct membership control, with the transient selection and `Add Selected` layers explicitly out of scope

**Note.** Both names are faithful to their sources — `docs/SPEC.md:80` really says
`` `Add N to Plan` immediately adds the checked canonical identities to the persistent plan
and clears the transient selection ``, and the UX memlog really says "Add Selected". But
`Add N to Plan` is the actual control label and `Add Selected` is the decision's shorthand,
and the document never says they are the same thing. §3 opens with `prd.md:102` "Downstream
workflows must use these terms exactly. No synonyms appear anywhere else in this document."
A run searching the codebase or `epics.md` for the control to delete will find one name and
not the other.

**Fix:** at `prd.md:244`, name both once — ``any `Add Selected` submit step (`docs/SPEC.md`
F5 calls the control `Add N to Plan`; they are the same control)``.

---

## Checked and clean

Recorded so the next reviewer does not re-derive them:

- **Every `AD-` citation added by the pass resolves and is quoted faithfully.** AD-28
  (`ARCHITECTURE-SPINE.md:1181`), AD-29 (`:1326`), AD-30 (`:1450`), AD-17 (`:774`–`780`),
  AD-12. §9.2's two verbatim quotes of AD-28's self-contradiction reproduce `:1272`–`1273`
  and `:1276`–`1278` exactly, and its claim that AD-17 `:774` agrees with the second is
  correct — `:774` reads "**Rule:** `Esc` collapses to **close-dialog and nothing else.**"
- **FR-6's Esc consequence and RP-2's ⌘L paragraph do not collide.** FR-6 `:239` forbids
  handing Esc the sidecar as a sink; RP-2 `:571` makes ⌘L a focus move that is a no-op on a
  hidden region. Different keys, different verbs, no overlap.
- **FR-14 and FR-21 state one active set, not two.** `prd.md:382` "`Queued` ∪ `Running` —
  **queued counts as running**", `prd.md:524` "queued or running. Queued counts as active",
  `prd.md:530` "the two guards' active sets must stay identical". Consistent in all three.
- **The 5→4 renumber is otherwise complete.** `prd.md:725` reads "All four are
  non-blocking" and §9 lists exactly four items; no surviving "all five", and no other
  section cites a §9 item by ordinal. M-1 is the sole residue.
- **§2.3's journey table and its exclusion note agree.** Working the table at
  `prd.md:87`–`92`, the FRs that appear in no row are exactly FR-12, FR-17, FR-19 and RP-2
  — matching `prd.md:96` and `prd.md:765`.
- **§10's three-judgment-call promise matches three bullets** (`prd.md:765`, `:767`,
  `:768`), and §9.2's "Four requirement defects" matches four bullets
  (`prd.md:748`–`751`).
- **The revision stamp landed at both places named in the brief** — `prd.md:28` and
  `prd.md:759`; `grep -n "revision 9"` returns nothing.
