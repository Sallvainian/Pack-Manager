# Adversarial review — `prd.md` after the revision-10 Update pass

**Reviewer:** hostile lens. **Date:** 2026-07-25. **Target:** `prd.md` (768 lines) as it
stands after the Update pass reconciling it against `ARCHITECTURE-SPINE.md` revision 10.
**Also read in full:** `addendum.md` (81 lines).

**Method note.** Every code and spine citation the pass added was opened and checked:
`src/hooks/useKeyboard.ts:34`, `:35`–`53`, `:70`–`78`, `:160`–`166`;
`src/components/manager/ManagerPane.tsx:92`–`107`, `:138`–`141`;
`docs/DECISIONS.md:193`; spine `:774`, `:1181`, `:1213`, `:1273`, `:1276`, `:1463`.
`grep -rnE "planAttemptId|plan_attempt_id|Verifying|InteractionRequired|skipUpgradePlanConfirmation" src/ src-tauri/src/ | wc -l` returns **0**, as §0 claims.
The AD-28, AD-29, AD-30, AD-24, AD-17 and AD-12 quotes are faithful. The evidentiary
floor is high; the findings below are not about fabrication.

---

## Overall verdict

The pass made the document **more correct and less usable as a requirements artifact.**
Its four named defect corrections are real and well-grounded. But it (a) made the PRD's
own authority claim at line 16 false in practice, (b) invented an undeclared block type
that now carries obligations the "Consequences (testable)" lists do not, (c) recorded
only four of the seven requirement changes it actually made, and (d) added roughly 25
lines that change no requirement by their own admission. The single sharpest problem is
not any one edit — it is that after this pass a downstream reader cannot tell which of
two channels inside one document states a requirement, and cannot tell from the document
which requirements moved.

0 critical, 5 high, 6 medium, 4 low.

---

## HIGH

### H1 — The PRD is now downstream of the spine in practice while claiming primacy

**Location:** prd.md:16, :530, :571, :744, :448–450, :753

**Quotes:**

- prd.md:16 — "**Authority.** This PRD is the requirements authority. `ARCHITECTURE-SPINE.md` and `epics.md` are reconciled *against* it, not the reverse. Where this document and an older artifact disagree, this document wins, with one exception: `docs/DECISIONS.md` remains the decision record".
- prd.md:530 — "A change to the refusal predicate above is now an AD-30 change, not a local one."
- prd.md:744 — "**Q — What happens on quit with work *queued* but not running, and on an OS-initiated shutdown? CLOSED: both decided by AD-30.**"
- prd.md:571 — "⌘L moves focus into the region, and when the region is hidden it is a **no-op** — it must not conjure the region into existence."
  Against `ARCHITECTURE-SPINE.md:770`–`772` — "When the region is hidden (all union members false) `⌘L` is a **no-op**, because there is nothing to focus and it must not conjure the region into existence."
- prd.md:239 — "**Esc is not handed the sidecar as a replacement sink.** … an Esc that dismissed the region would orphan a draft the user still holds — and `Done` already owns dismissing Results. A one-rung cascade is the intended end state, not an impoverished one".
  Against `ARCHITECTURE-SPINE.md:776`–`780` — "**`Esc` is not handed the sidecar as a replacement sink** — admission is what empties the draft (AD-24), so an `Esc` that dismissed the region would orphan a draft the user still holds, and `Done` already owns dismissing Results. A one-rung cascade is the intended end state, not an impoverished one."
- `ARCHITECTURE-SPINE.md:1598`, in the Open row titled *"Maintainer edits this spine cannot make"* — "**`prd.md` itself:** RP-2 describes `⌘L` as "(toggle the activity surface)", the retiring `ActivityDrawer` behaviour — AD-17 now makes `⌘L` a focus jump to the sidecar region, so the parenthetical is stale."
- prd.md:450 — "`[NOTE FOR PM]` — only a decision supersedes a decision, and D28's sentence still reads as demotion."

**Note.** Make the criticism properly rather than dismiss it. Three things are true at
once. First, the spine filed defects *against the PRD* (`:1598`, listing `prd.md` beside
`docs/SPEC.md` and `docs/DECISIONS.md` as if it were workflow-unowned), and this pass
executed them. Second, two of the PRD's requirement passages — RP-2's ⌘L behavior and
FR-6's Esc consequence — are sentence-order transcriptions of AD-17's rule text; the
requirements authority is quoting the artifact it is supposed to govern, as its own
requirement. Third, prd.md:530 explicitly transfers change control over a **Shipping**
FR-21 predicate to AD-30.

The one place the PRD asserts primacy is §9.2:753 — "**FR-6 is correct and stays as
written.** The fix belongs to `bmad-architecture`" — and that is where the spine
contradicts *itself* (`:1273` vs `:1276`), not where it contradicts the PRD. Operational
summary: **the PRD wins ties and loses conflicts.** That is what downstream means.

The substance is probably right — spine `:773` records "Owner decision, 2026-07-25" — so
this is not a claim that the requirements are wrong. It is that owner decisions are
reaching the requirements authority *through the architecture document*, which line 16
names as the thing reconciled against it, and which line 16 does not list among the things
that can supersede it. The author half-knows: prd.md:450 states the rule ("only a decision
supersedes a decision") in the one case where it was noticed, and then makes the change
anyway.

**Fix.** Either (a) amend line 16 to state the real rule — "an ADOPTED spine invariant
recording an owner decision supersedes this document until the decision is back-filled to
`docs/DECISIONS.md`" — or (b) back-fill D38/D39 for the ⌘L, Esc and `autoOpenDrawer`
decisions and cite *those* at FR-6, FR-17 and RP-2 instead of AD-17. Do not leave line 16
as written: three downstream workflows read it to decide who wins a conflict.

---

### H2 — §9.2 records four of the seven requirement changes this pass made, and presents the list as complete

**Location:** prd.md:746

**Quote:** "**Four requirement defects were corrected in the same pass**, each found by
reconciling against a revision-10 AD rather than by review of the prose:"

**Note.** The pass also changed three requirements that §9.2 does not list:

1. **RP-2's ⌘L** — from "**⌘L** (toggle the activity surface)" to prd.md:569 "**⌘L** (move focus into the Upgrade Plan sidecar region)", plus the whole new normative paragraph at :571.
2. **FR-6's Esc consequence** — from "Esc collapses from three rungs to two … Esc keeps close-dialog and close-drawer" to prd.md:239 "**Esc collapses to close-dialog and nothing else.**"
3. **FR-17's `autoOpenDrawer`** — from "the demotion of `autoOpenDrawer` … Planned — D28" to prd.md:424 "`autoOpenDrawer` is **retiring, not being demoted**", with a new normative bullet at :444 ("no story keeps it alive as an inert setting").

All three trace to AD-17, which is as much a revision-10-era rule as AD-28/29/30, so the
exclusion is not principled. These are precisely the three a `bmad-ux` run needs: prd.md
addendum:56 routes `EXPERIENCE.md`'s "`## Focus transitions` (290–303)" matrix to that run,
and an Esc-cascade change is exactly what that matrix encodes. A run that trusts §9.2's
"four" will reconcile the wrong set.

An incomplete changelog is worse than no changelog, because readers stop diffing once one
exists.

**Fix.** List all seven, or retitle to "Four defects found by AD reconciliation; see also
the AD-17 corrections at FR-6, FR-17 and RP-2" and enumerate them.

---

### H3 — "Architecture binding" is an undeclared second requirements channel, and the document cannot say whether it is normative

**Location:** prd.md:28, :208, :241, :274, :304, :367, :382, :400, :530, :542, :544, :573, :597, :605, :633

**Quotes:**

- prd.md:28 — "each is cited at the FRs it binds under an **Architecture binding** heading." That is the only definition the document gives. It never says whether a binding is a requirement, who owns it, or what happens when a binding and a Consequence disagree.
- Twelve bindings are standalone paragraphs *after* the "Consequences (testable):" list. Two are bullets *inside* it: prd.md:274 "- **Architecture binding — `ARCHITECTURE-SPINE.md` AD-28:** the per-item `Remove` this FR requires…" and prd.md:382 "- **Architecture binding — `ARCHITECTURE-SPINE.md` AD-30:** the quit guard has **one enforcement point**…".
- Obligations that exist *only* in a binding, in requirement voice: prd.md:274 "each use writes a tombstone that no later bulk expansion re-adds (AD-23)"; prd.md:367 "An attempt gets exactly two records — admission and terminal — never one per transition"; prd.md:400 "a terminal record that exists but is unreadable is reported as *unreadable evidence*, never silently reclassified as an unfinished attempt"; prd.md:530 "**the two guards' active sets must stay identical and may not drift apart.**"; prd.md:605 "AD-29 also names this NFR as the requirement that forbids a persisted `PlanAttempt.state` field".

**Note.** The strongest case against the pattern, stated plainly:

1. **It is inconsistently placed, so mechanical extraction breaks.** `bmad-create-epics-and-stories` harvests the "Consequences (testable)" lists. It will pick up FR-7's and FR-14's bindings as consequences and miss the other twelve — including AD-29's two-record rule and AD-30's don't-drift rule, both of which are testable and neither of which appears in any consequence list.
2. **It creates obligations with no home in the FR's own contract.** FR-7's tombstone semantics are a complete, testable requirement stated nowhere else. A reader asked "is this a requirement?" has only the heading to go on, and the heading is undefined.
3. **It contradicts the section it lives under.** prd.md:28's own heading is "**Upstream inputs already written; not duplicated here.**" — and that sentence is what introduces fourteen paraphrases of upstream AD content. The spine revises fast (revisions 6, 8, 9, 10 are all cited in this tree). Fourteen restatements are fourteen things to re-verify at revision 11, in a document whose §0 says the spine is "the sole authority for `AD-` numbering".
4. **It is the wrong direction of citation.** The spine already cites the PRD by FR id — `ARCHITECTURE-SPINE.md:1183` "**Binds:** Stories 3.1, 3.2, 3.5; UX-PB.1a–1e; the application accelerator map", `:1213` "(`prd.md` FR-6, NFR-3)". The binding relation was already recorded, once, in the artifact that owns AD numbering. The PRD now records it a second time, in prose, in the artifact that does not.

The honest alternative is one line per FR — "Bound by AD-28" — and nothing else, with the
substance staying in the spine. Where a binding contains something the PRD genuinely
needs (the AD-30 don't-drift rule, the tri-state denominator), promote it into
"Consequences (testable)" where it can be tested, and delete the block.

**Fix.** Pick one: promote the normative content into Consequences and reduce bindings to
bare `AD-` references, or add an explicit rule to §0 stating that Architecture-binding
blocks are normative and rank below Consequences on conflict. The current silence is the
defect.

---

### H4 — FR-6's new evidence table over-reads `ManagerPane.tsx:138`–`141`, and its sharpest claim is the wrong one

**Location:** prd.md:255, :257

**Quotes:**

- prd.md:255 — "| `src/components/manager/ManagerPane.tsx:138`–`141` | `isSelectable` a third time — but over `mainPackages`, **not** `visibleMain`, so it ignores the search term and the outdated-only filter entirely. |"
- prd.md:257 — "The third is the sharpest: it is not a duplicate of the predicate but a divergent one, and it is reachable today."

**Note.** `ManagerPane.tsx:138` is `function upgradeAll()`, and its only consumer is
`PackageToolbar.tsx:87`–`88`, a button reading `Upgrade all ({outdatedCount})`. It is not
a third copy of the eligibility-and-visibility predicate; it is a *different feature*
whose scope is deliberately Manager-wide, and its count (`outdatedCount`) is computed
unfiltered to match. "Ignores the search term" is the intended semantics of a button that
says "Upgrade all", not a divergence.

Worse, the second half of the claim is vacuous: `src/store/packages.ts:125`–`126` is
`return pkg.outdated && !pkg.pinned && pkg.kind !== "caskGreedy";`. Because `isSelectable`
already requires `outdated`, layering the outdated-only filter on top of it changes
nothing. "Ignores … the outdated-only filter entirely" describes a difference with no
observable effect.

The first two table rows are correct and sufficient — `useKeyboard.ts:34`'s own comment
does read "mirrors ManagerPane filters", and `ManagerPane.tsx:107`'s `orderedSelectable`
is an independent derivation of the same chain. Two copies of one rule already justify
the requirement. The third row weakens the argument and, presented as "reachable today",
invites a story writer to "fix" a shipped button by scoping it to the search term —
against its own label.

**Fix.** Delete the third row, or relabel it: "`upgradeAll()` is Manager-wide by design;
under FR-6 it becomes a staging path and must be re-expressed against the single
predicate — its present unfiltered scope is not evidence of divergence."

---

### H5 — Line-number citations into fast-revising files, one already stale after the revision this pass was reconciling against

**Location:** prd.md:740, :753, :448

**Quotes:**

- prd.md:740 — "`ARCHITECTURE-SPINE.md:1050` still records this as OPEN and routes it to the owner. It is now answered, and that row should be **retired** in the `bmad-architecture` Update that follows this PRD."
- prd.md:753 — "`:1272`–`1273` says "the cascade drops from three rungs to two and keeps close-dialog", while `:1276`–`1278` corrects it four lines later".

**Note.** At revision 10 — the revision this pass stamped at prd.md:28 and prd.md:759 —
spine line 1050 is inside AD-25/AD-26 ("**Rule:** Health and staleness presentation derive
from the snapshot's real timestamp"). The row the PRD is pointing at is now at `:1595` and
reads "| Transient selection has no owning invariant | **RESOLVED** | Closed by **AD-28**,
written 2026-07-25 on the owner's decision". The "Update that follows this PRD" already
happened; the invariant `addendum.md:55` says must be written first ("Write the invariant,
then retire the row") **is** AD-28. §9.1 now instructs a run to do work that is done.

This is the pass's own failure mode: it stamped revision 10 in two places and cited four
rev-10 ADs at fourteen FRs, without re-checking the one line-numbered spine citation the
document already carried.

The pass then *added* two more line-numbered spine citations at :753 (and they are already
imprecise — the quoted text at ":1272–1273" actually spans `:1273`–`:1274`). The spine's
own instruction, `ARCHITECTURE-SPINE.md:1594`, is "Cite by `AD` id and subject, never by
rule ordinal and never by line number", and the same row records that `epics.md` was
cleaned of exactly this ("`grep -c 'ARCHITECTURE-SPINE.md[:#]*[0-9]'` returns **0**").
The PRD is re-introducing the pattern the tree just finished removing.

**Fix.** Repoint :740 to "the `Transient selection has no owning invariant` row, now
RESOLVED by AD-28 at revision 10 — nothing remains for the next architecture run", and
restate :753's divergence as "AD-28's `Esc` rule contradicts itself within one bullet"
with no line numbers. Correct `addendum.md:55` in the same edit; it still says "(rev 9)".

---

## MEDIUM

### M1 — FR-22's AD-12 paragraph changes no requirement, says so, and duplicates NFR-8's binding

**Location:** prd.md:544, :633

**Quotes:**

- prd.md:544 — "This **enables** rather than contradicts anything this FR requires. … Rotating it is now ordinary maintainer work. **Nothing above changes**; the constraint that blocked a rotation is gone." (emphasis added)
- prd.md:544 — "release-please owns the *version* in `package.json`, `package-lock.json`, `$.version` in `src-tauri/tauri.conf.json`, `$.package.version` in `src-tauri/Cargo.toml`, and the `pack-manager` entry in `src-tauri/Cargo.lock`"
- prd.md:633 — "The `pubkey` sits in `src-tauri/tauri.conf.json` but in none of release-please's `extra-files` paths, so the release automation never reads or writes it"

**Note.** This is the clearest scope creep in the pass. A ten-line paragraph in a
requirements document that states it changes no requirement is, by construction, not a
requirement. Its content — which fields of `Cargo.lock` release automation owns — is
release tooling; `addendum.md:20` quarantines exactly this class ("Updater API choice,
Rust update-state module, release metadata transport, **signing environment** … the
transport is architecture"), and `CLAUDE.md` already documents the five-file lockstep.
The `pubkey`-not-in-`extra-files` fact is then stated twice, at :544 and :633.

**Fix.** Delete prd.md:544. Keep the one sentence that carries a consequence for NFR-8's
guard, which :633 already states.

### M2 — The bindings import mechanism the addendum exists to keep out, and add Glossary-unanchored vocabulary

**Location:** prd.md:238, :274, :367, :400, :464, :605 against prd.md:102 and addendum.md:3, :17, :18, :22

**Quotes:**

- prd.md:102 — "Downstream workflows must use these terms exactly. No synonyms appear anywhere else in this document."
- addendum.md:3 — "it is placed here so the PRD stays a statement of *what*"
- addendum.md:17 — "| IPC command signatures, wire casing, payload structs, event subscription architecture, frontend store shape | … | The contract is architecture's to own. |"
- addendum.md:18 — "| Transcript header syntax, log library configuration, span names | … | Format is implementation. |"
- prd.md:238 — "The frontend submits concrete identities **plus the snapshot token it read**, and the backend rejects a batch whose token is not its current snapshot."
- prd.md:605 — "AD-29 also names this NFR as the requirement that forbids a persisted `PlanAttempt.state` field"
- prd.md:464 — "It carries that journal's **raw lines** — never a synthesized record, and never a fold written back into the journal."

**Note.** prd.md:238 specifies a membership IPC's payload contents; addendum.md:17 assigns
payload structs to architecture. prd.md:605 forbids a struct field. prd.md:367/:464
specify journal record counts and file contents; addendum.md:18 assigns format to
implementation. addendum.md:22's list of "**Technical detail that *did* earn a place in
the PRD**" was not amended for any of it, so the addendum now under-describes its own
companion.

Separately, the pass introduced four terms that §3 does not define and that collide with
terms it does: **snapshot token** (:238) against Glossary "**Snapshot** — the merged
installed-plus-outdated view of one Manager at one moment" (:107) and against FR-8's
"canonical state revision" / "revision drift" (:286); **tombstone** (:274); **idempotent
fold** (:400); **membership projection** (:231). A story writer implementing FR-6's batch
rejection cannot tell from this document whether the token is per-Manager (a Snapshot),
global (the state revision), or a third thing.

**Fix.** Either add the four terms to §3 with their scope stated, or reduce the bindings
to `AD-` references per H3 and let the spine own the vocabulary. Amend addendum.md:22
either way.

### M3 — §10's review record now over-claims coverage, and nothing in the frontmatter distinguishes the gated PRD from this one

**Location:** prd.md:1–6, :761

**Quotes:**

- prd.md:3–5 — "status: final / created: 2026-07-25 / updated: 2026-07-25"
- prd.md:761 — "This document went through a reviewer gate on 2026-07-25: seven reconcilers, one per source input, and six review lenses including the quality-rubric walker … The gate raised 62 findings across the review lenses alone, of which 53 were refuted. Its verdict was *Good*"

**Note.** `git diff HEAD` on this file is 90 insertions, 19 deletions. None of it was
before that gate. §10 reads as though it describes the document a reader is holding; it
describes an earlier one. The frontmatter offers no way to tell them apart — same status,
same date, no revision field — while the artifact this PRD reconciles against solved the
same problem explicitly (`ARCHITECTURE-SPINE.md:36` "> **Revision 10, 2026-07-25.**").
`epics.md` and the spine cite this file as "`prd.md` §X"; there are now two documents
answering to that citation.

**Fix.** Add `revision: 2` (or `u2`) to the frontmatter, and one sentence to §10: "The
gate described here covered the document as of the initial run; the 2026-07-25 Update pass
recorded in §9.2 is ungated."

### M4 — Range selection is constrained in three places and required in none

**Location:** prd.md:208, :237, :597

**Quotes:**

- prd.md:208 — "a range is an anchor and a target over the **ordered filtered set the projection holds**, including off-screen virtualized rows, and never the rendered DOM window."
- prd.md:237 — "A range or filter-wide interaction submits one membership operation covering every affected Package identity, not one per row."
- prd.md:597 — "a shift-range across 100 rows would become 100 round trips."

**Note.** No consequence anywhere in FR-5 or FR-6 states that the user can select a
contiguous range. The capability is referenced only as the thing three constraints
constrain — and the FR-5 binding, which is new in this pass, is the one that adds the most
detail about it. A story writer working FR-5/FR-6 gets ordering rules, batch rules and
anchor semantics for an interaction the requirements never require. (The spine does state
it: `ARCHITECTURE-SPINE.md:1201` "The **anchor survives; the selection set does not**.")

**Fix.** Add one consequence under FR-6: "A shift-range interaction over the ordered
filtered set adds or removes every eligible Package between anchor and target, in one
batch." The three constraints then have a subject.

### M5 — RP-2 gained a new normative ⌘L behavior while §4.6 routes its validation to a file containing zero accelerators

**Location:** prd.md:550, :569, :571; addendum.md:57

**Quotes:**

- prd.md:550 — "These two are mandatory prerequisites rather than product features. They are validated through `docs/RELEASE-CHECKLIST.md`."
- prd.md:571 — "**⌘L is a focus move, not a toggle. Planned — D27–D30 for the behavior; the registration ships.**"

**Note.** `grep -ncE "⌘R|⌘⇧R|⌘⇧U|⌘L|⌘F|⌘1" docs/RELEASE-CHECKLIST.md` returns **0**.
RP-2's accelerator half has no validation step at all, and this pass added a *new*
requirement to that half. `addendum.md:57`, the PRD's own reconciliation queue for this
file, lists only the two D37 lines to remove — nothing to add. The gap was caught, and
drafted, by the architecture run instead
(`architecture-…/MAINTAINER-EDITS-2026-07-25.md` Edit 4), which is the H1 pattern again
at a smaller scale.

**Fix.** Add a row or clause to `addendum.md` §3's `docs/RELEASE-CHECKLIST.md` entry:
"add an accelerator regression step covering RP-2's enumerated map, framed as functional,
not accessibility." Note there that MAINTAINER-EDITS Edit 4's draft still says "⌘L (toggle
the activity surface)" and must be updated to the focus-move wording before it is applied.

### M6 — A "Consequences (testable)" bullet states pre-D27 behavior as if it were a consequence

**Location:** prd.md:236 (against :234)

**Quotes:**

- prd.md:234 — "- The draft persists while the user navigates between Managers and the Dashboard, and every staged item is individually removable from the Upgrade Plan."
- prd.md:236 — "- The draft is transient dialog state discarded on close — pre-D27 behavior this FR removes."

**Note.** Two bullets apart, inside one testable-consequences list, the document asserts a
behavior and its negation. The trailing clause disambiguates for a careful human and not
at all for an extraction pass — and the entire FR-6 consequence list is what
`bmad-create-epics-and-stories` reads. Every other retirement in this document is marked
by a *leading* qualifier ("**Planned — D28:**", "**Not yet built:**", "**Out of Scope:**").
This one is not. The pass rewrote the two bullets immediately below it (:237, :238) and
left it.

**Fix.** Move it under "**Out of Scope:**" at :243, where the transient selection layer is
already retired, or prefix it: "**Removed by this FR:** the draft is transient dialog
state discarded on close."

---

## LOW

### L1 — "Open Question 1" now refers to two different questions twenty lines apart

**Location:** prd.md:720, :744

**Quotes:** prd.md:720 — "1. **Does a downloaded application update survive a relaunch that was not the update restart?**"; prd.md:744 — "This was Open Question 1 and is no longer open."

**Note.** The renumbering that closing the old Q1 required makes §9.2's back-reference
point at a live, different question. **Fix:** "This was the first Open Question in the
initial draft".

### L2 — The AD-24 quote's ellipsis removes the carve-out that makes the sentence non-absolute

**Location:** prd.md:239

**Quote:** prd.md:239 — "Admission of the draft's own preview empties it as custody transfer … no other path adds, replaces, or clears membership"

**Note.** `ARCHITECTURE-SPINE.md:995`–`998` reads "Admission of the draft's own preview
empties it as custody transfer (AD-17) **and a canonical rebuild may narrow it (AD-16)**;
no other path adds, replaces, or clears membership." The elided span is the exception, and
the quote is being used to support "Admission … is the only thing that empties it". The
claim survives (narrowing is not emptying), but the elision makes a hedged source sentence
read as absolute — and the narrowing behavior it hides is stated nowhere in the PRD, while
prd.md:234 tells the reader every staged item is removable by them.

**Fix.** Quote the clause in full, or elide with the carve-out preserved.

### L3 — §7 was not updated for FR-18's new Planned limb

**Location:** prd.md:454, :667, :682

**Quotes:** prd.md:454 — "**Status:** Partial. … The plan-attempt journal is Planned — D29"; prd.md:667 — "- Diagnostics export."; prd.md:682 — "A durable `planAttemptId` correlating reviewed intent, Operations, events, transcripts, journal records, verification, Results, and Retry lineage — with History carrying one immutable row per confirmed attempt (D29)."

**Note.** The pass created a new Planned requirement and did not enter it in the section
prd.md:685 calls "the live build queue". §7.2's D29 bullet names eight correlated things;
the diagnostics archive is not among them. (`epics.md:367` does cover it downstream, so
this is a PRD-internal inconsistency, not a lost requirement.)

**Fix.** Append ", and the diagnostics archive carrying that journal's raw lines (FR-18)"
to prd.md:682.

### L4 — The same rationale is now stated three times

**Location:** prd.md:237, :597, :739

**Quote (representative, prd.md:597):** "the canonical draft lives in the backend and
every mutation round-trips before the projection updates, so a shift-range across 100 rows
would become 100 round trips."

**Note.** The identical reasoning appears at :237 (FR-6 consequence), :597 (NFR-3 binding,
new this pass) and :739 (§9.1). In a document whose §0 heading is "not duplicated here",
a correction pass adding a third copy is padding. **Fix.** Reduce prd.md:597 to "AD-28
names NFR-3's 101-rows sentence as the requirement a per-row mapping breaks; see FR-6."

---

## What is not a finding

Checked and clean, recorded so the next reviewer does not re-open them:

- Every AD quote in the new bindings is faithful to `ARCHITECTURE-SPINE.md` revision 10, including the AD-28 self-contradiction at `:1273` vs `:1276`, which is real and correctly attributed to the spine.
- Every code citation added by the pass resolves to the right lines: `useKeyboard.ts:34` does read "mirrors ManagerPane filters"; `:70`–`78` is the three-rung Esc cascade; `:164`–`166` is `toggleDrawer()`; `ManagerPane.tsx:107` is `orderedSelectable`.
- The zero-occurrence claim at prd.md:24 verifies exactly.
- FR-14's "run the existing kill hook" is real: `src-tauri/src/state.rs:371` `self.queue.cancel_all();` with the token-flip caveat documented at `:363`.
- Preserved ID gaps, status tags, the retired D33/D37 apparatus and the absent adoption metrics are deliberate conventions and were not reviewed as defects.
