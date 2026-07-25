# PRD Quality Review — Pack-Manager (2026-07-25)

Rubric: `.claude/skills/bmad-prd/assets/prd-validation-checklist.md`.
Calibration: hobby/solo stakes, but brownfield **and** chain-top. Dimensions 4 and 6 carry
full weight; commercial-launch formality does not. Scope exclusions honored: no finding
below asks for a readiness gate, coverage percentage, scenario contract, evidence manifest,
adoption metric, keyboard/screen-reader criterion, or excluded technical mechanism.

## Overall verdict

This is a genuinely good requirements document — the Vision is unswappable, the trade-offs
are stated as trade-offs, and the numeric consequences I spot-checked against `src-tauri/`
were correct without exception (64 capabilities, concurrency 4, 50 ms / 64 lines / 8 KiB,
5000 live lines, 14/90/200/1000 retention, 3 logs / 25 transcripts — all exact). What is at
risk is the handoff, not the thinking: the reconciliation queue in `addendum.md` §3 carries
mention counts that are wrong for all three artifacts and misses three whole `EXPERIENCE.md`
sections, so the three Update runs it authorizes would each stop short and leave live exactly
the obligations the PRD says it removed. Alongside that, RP-2 does not contain the keyboard
map FR-19 claims it contains, FR-6 deletes a selection model that four shipping accelerators
are wired to without saying so, and four FRs — including the no-privilege boundary the Vision
leads with — realize no journey.

---

## Decision-readiness — strong

This PRD does the thing most PRDs dodge. §0.1 opens by attacking its own upstream source with
a seven-row defect table, each row citing a specific `docs/SPEC.md` section against specific
code, and every row I checked was right (`lib.rs:232-253` registers 20 commands, `events.rs:77-82`
defines 6 events, `settings.rs:28-39` has 8 fields). §9.1 closes a question *and* names the
mechanical reason it stayed open ("`docs/SPEC.md` §0.1's supersession list retires six
behaviors and **never added F5**. That omission is the whole defect"), which is the opposite
of smoothing. §10 volunteers two judgment calls "because a reviewer might reasonably have made
them differently" — including the one most likely to be second-guessed, dropping NFR-6's focus
restoration. `SM-C1` is a real counter-metric with teeth: "Anyone optimizing clicks-to-upgrade
is optimizing away the product."

The one crack is in §9 itself. Q1 (`prd.md:629`) lists "application-update installation during
Package activity" as undefined, but FR-21 (`prd.md:471`) defines it flatly, and defines the
queued case Q1 also claims is open. An Open Question that the document answers 158 lines
earlier is the rubric's stated red flag inverted — not a rhetorical question, but a live one
that has already been closed and not retracted.

### Findings

- **medium** Open Question 1 contradicts FR-21 on two of its three limbs (§9 Q1, §4.5 FR-21) —
  Q1 reads "Queued-only work, application-update installation during Package activity, and
  OS-initiated shutdown are not [defined]", while FR-21's consequence reads "Installation and
  relaunch are **refused** while any Package Operation is queued or running. Queued counts as
  active — admission has already committed to the work". FR-21 decides both the queued case
  and the app-update-during-activity case. A story author scoping the quit path will read Q1
  as license to re-decide something FR-21 already binds. *Fix:* narrow Q1 to the two genuinely
  open cases — quit (not app-update restart) with work queued-only, and OS-initiated shutdown —
  and cross-reference FR-21 for the rest.

---

## Substance over theater — strong

No personas at all, which is correct: §2.1 is six Jobs To Be Done, one of them explicitly
labeled "the load-bearing one" with a pointer to the FR group that serves it, and one labeled
"Builder's own". That is a target-user section that earns its place at a fraction of the usual
length.

The Vision passes the swap test decisively. `prd.md:58` names the actual mechanics of the
actual failure modes — "`uv` resolves through a mise shim that is a symlink to the mise binary,
so a naive path lookup misroutes its self-update to Homebrew" and "Real version strings include
`2.0.14-1`, `1.6.2.dev0`, `stable`, and commit hashes". Neither sentence fits any other PRD.

The NFRs are not boilerplate. NFR-3 gives four hard numbers; NFR-8 states the specific failure
it prevents ("a drifted signing key produces a fully green release that silently breaks updates
for every installed client") rather than asserting reliability. §8's opening line — "Adoption
metrics would be theater" — is self-aware in a way that is rare and, here, accurate.

No findings.

---

## Strategic coherence — strong

The thesis is stated once and load-bearing: "The differentiator is not that it wraps update
commands in a window. It is **confidence across a mixed Manager topology**" (`prd.md:56`). Each
feature group then states which limb of that thesis it carries — §4.1 is "be right about the
machine", §4.2 is "*nothing runs that was not staged and shown*", §4.3 is the three things owed
after authorization. §6's eleven non-goals are each a refusal that follows from the thesis
rather than a scope list ("A version oracle", "A rollback engine", "A retry loop").

The metrics validate the thesis rather than measuring activity: SM-2 ("Zero unreviewed
mutations. … A single violation is a P0 defect, not a metric miss") and SM-3 are boolean
invariants, which is the right instrument for a trust product with one user. SM-C2 explicitly
counterbalances the deferred list in §7.3 — a counter-metric aimed at a specific section of the
same document is unusually disciplined.

No findings.

---

## Done-ness clarity — adequate

Held to the "be unforgiving" standard, this lands short of strong, though the body of work is
solid. Where the PRD commits to a number it is right: `prd.md:259` "at most 64 unconsumed
capabilities" matches `src-tauri/src/state.rs:25` `pub const ISSUED_PLAN_LIMIT: usize = 64;`;
`prd.md:273` "global concurrency cap of 4" matches `src-tauri/src/queue.rs:48`
`pub const MAX_CONCURRENCY: usize = 4;`; NFR-3's "50 ms, 64 lines, or 8 KiB" matches
`events.rs:183-187`; FR-15's retention matches `logging.rs:26-28` and `journal.rs:19`
`pub const COMPACT_KEEP: usize = 1000;`; FR-18's counts match the `diagnostics.rs:197,206`
test comments. Consequences like FR-19's "status chips do not wrap" and FR-12's "A submitted
preview that does not match the preview re-rendered from trusted arguments fails closed" are
directly testable.

The problem is a residue of adjective-only consequences, and it clusters in the FRs that have
the *least* backup elsewhere. FR-20's "Package work remains understandable and uninterrupted
throughout" and FR-21's "Every update-stage failure is actionable" are the terminal consequences
of two Shipping FRs, and neither can be turned into an assertion. FR-16's "Errors state what
happened and what to do next, in plain language, per error class" gestures at an enumeration —
"per error class" — that the PRD never provides, so the criterion has no denominator. FR-5's
feature NFR, "The Package list stays usable and responsive beyond 100 Packages", is strictly
weaker than NFR-3's own "correct actions reachable at 101 rows" for the same property, so the
bounded version and the adjective version coexist. FR-19's "The interface remains usable at
900 × 600 … Narrow widths scroll rather than letting essential content collide" leaves both
"usable" and "essential content" undefined.

The mistagged FR-14 quit guard (already known and queued) compounds this: a Shipping tag is
itself an acceptance claim in a brownfield PRD, and a wrong one costs a story author a
verification pass before they discover there is nothing to verify.

### Findings

- **medium** Six consequences are adjectives where the rest of the document gives bounds
  (§4.1 FR-1, §4.1 FR-5, §4.3 FR-16, §4.4 FR-19, §4.5 FR-20, §4.5 FR-21) — the offenders, in
  order: `prd.md:144` "a copyable install hint where one is known" (which are known?);
  `prd.md:207` "The Package list stays usable and responsive beyond 100 Packages";
  `prd.md:375` "Errors state what happened and what to do next, in plain language, per error
  class" (classes never enumerated); `prd.md:427` "remains usable … rather than letting
  essential content collide"; `prd.md:461` "Package work remains understandable and
  uninterrupted throughout"; `prd.md:475` "Every update-stage failure is actionable". Each is
  the *only* consequence covering its property. *Fix:* for FR-16 and FR-21, enumerate the
  classes/stages (the error taxonomy and the check → download → install → relaunch stages
  already exist implicitly in FR-20/FR-21); for FR-1, name the Managers with known install
  hints; for FR-5, delete the feature NFR and cite NFR-3, which already states the bound;
  for FR-19/FR-20, replace "usable"/"understandable" with the observable — no clipped or
  overlapping primary controls, and Package Operation state remains visible and its Operations
  keep running.

---

## Scope honesty — strong

Omissions are stated, not inferred. §6 is eleven explicit non-goals with reasons attached
("This is a hard product boundary, not a default"). §2.2 names five non-user classes including
the one that will be questioned. §7 splits Shipping / Decided-not-implemented / Deferred, and
the Status tag on every FR carries that split down to requirement level — the right instrument
for brownfield, and used consistently. §7.3 carries a real `[NOTE FOR PM]` at a real tension:
"verify whether each already ships before scheduling it as new work. An adversarial triage pass
overturned 14 of 20 initial keep verdicts for exactly that reason" — which is, ironically, the
exact trap §7.3 then fell into with Health fixes (known item b).

Open-items density is right for the stakes: five Open Questions, all declared non-blocking with
a stated resolution route ("each can be resolved during the epic that touches it"), one
`[NOTE FOR PM]`, zero `[ASSUMPTION]` tags. The zero is defensible rather than evasive, because
§10 explains the mechanism that replaced it — §0.1 records source corrections, §9 records source
silences — and the §0.1 corrections check out. The claim I most expected to be soft, `prd.md:46`
"9 of 17 features carry no acceptance criterion, including four P0s", is exactly right:
`docs/SPEC.md` has 12 P0 features (`### F1`–`### F12`) and 5 P1 features (F13–F17 under
`### P1 (after all P0 green)`, line 118), and `grep -c "Acceptance"` over lines 54–126 returns
8 — F9, F10, F11, F12 and all five P1s carry none.

No findings.

---

## Downstream usability — thin

This is where the PRD's quality does not survive the handoff, and it is the dimension that
matters most for a chain-top document.

The primitives are clean. FR-1…FR-22 are contiguous and unique (`grep -c "^#### FR-"` = 22),
NFR-1…NFR-8 and RP-1/RP-2 complete, SM-1…SM-5 plus SM-C1/SM-C2, AJ-1…AJ-6 mirrored rather than
duplicated — mirroring the UX IDs instead of minting a fourth namespace is the right call and
`prd.md:83` says why. The Glossary is disciplined and its terms are used verbatim.

But the reconciliation queue that the three downstream workflows will actually execute is
inaccurate in both directions. `prd.md:440` and `addendum.md:52-54` state "10, 3, and 4 mentions
respectively" for `epics.md`, `ARCHITECTURE-SPINE.md`, and `EXPERIENCE.md`. Measured:
`grep -ciE "voiceover|keyboard"` returns **13**, **8**, and **11** lines respectively (as raw
occurrences, 21 / 9 / 13). The numbers were inherited verbatim from `docs/DECISIONS.md:559-561`
and never re-measured. Worse than the arithmetic, the queue's *scope* for `EXPERIENCE.md` names
only "the `Accessibility Floor` section (lines 313–332)", while that file also carries
`## Keyboard` (line 272), `## Package Grid keyboard model` (line 282), and a `## Focus
transitions` matrix (lines 291–303) — and the matrix is, line by line, the "deterministic
dialog/sidecar focus restoration" that `prd.md:430` and `prd.md:659` record as dropped. A
`bmad-ux` Update run driven by this queue removes the Accessibility Floor and leaves three
sections mandating what the PRD deleted. That is the reinstate-by-omission failure D37 warns
about, arriving through the PRD's own instructions.

Separately, `prd.md:435` asserts that "**⌘X / ⌘C / ⌘V / ⌘A and the ⌘R / ⌘A / ⌘L map** (RP-2)"
survives. RP-2 (`prd.md:503`) covers only "Standard Edit and Window menu actions". ⌘R and ⌘L are
neither, so the cross-reference dangles and the PRD states no requirement for a surface it names
as protected. (`reconcile-spec.md` §3.4 reached this during the run; it did not reach the PRD.)
The unreported half is larger: FR-6 abolishes the transient selection layer, and four shipping
accelerators are wired directly to it — `src/hooks/useKeyboard.ts:90` `setSelection(...)` for
⌘A, `:97` reading `selection` into `buildUpgradePlan` for ⌘U, `:74` `clearSelection(...)` for
Esc, plus ⌘⇧U. The PRD neither requires them, re-specifies them against draft membership, nor
lists them in FR-6's "Out of Scope".

Finally, four requirements realize no journey. §2.3's table maps AJ-1…AJ-6 across FR-1…FR-11,
FR-13…FR-16, FR-18, FR-20…FR-22 and RP-1. FR-12, FR-17, FR-19 and RP-2 appear in no row —
including FR-12, the no-privilege boundary the Vision leads with (`prd.md:56` "No privilege
prompt exists anywhere in the product") and that SM-3 validates. A UX or epics run source-
extracting by journey will not reach them.

### Findings

- **high** `addendum.md` §3's reconciliation queue is wrong on counts and short on scope, so
  all three Update runs would stop early (addendum §3; prd §4.4 FR-19 notes) — the stated
  "10, 3, and 4 mentions" measure as 13, 8 and 11 lines
  (`grep -ciE "voiceover|keyboard"` over `epics.md`, `ARCHITECTURE-SPINE.md`, `EXPERIENCE.md`).
  For `EXPERIENCE.md` the queue names only `Accessibility Floor` (313–332), omitting
  `## Keyboard` (272), `## Package Grid keyboard model` (282) and `## Focus transitions`
  (291–303) — the last of which is exactly the focus-restoration obligation `prd.md:659`
  records as dropped, e.g. `EXPERIENCE.md:296` "Escape or backdrop dismissal | Restore focus to
  the originating `Confirm # updates` action." *Fix:* replace the mention counts with
  section-level scope per artifact (named headings and line ranges, re-measured at the commit
  the PRD cites), and state that counts are indicative only — the Update runs must sweep the
  file, not hit a number. `reconcile-spine.md` F7 records the equivalent under-scoping for
  `ARCHITECTURE-SPINE.md`.

- **high** RP-2 does not contain the map FR-19 says it contains, and FR-6 orphans four shipping
  accelerators (§4.4 FR-19 note 2; §4.6 RP-2; §4.2 FR-6) — `prd.md:435` protects
  "the ⌘R / ⌘A / ⌘L map (RP-2)"; `prd.md:503` in full reads "Standard Edit and Window menu
  actions — including cut, copy, paste, and select-all in the search field and in every copyable
  command surface — are preserved." ⌘R (refresh) and ⌘L (toggle activity) are neither. Meanwhile
  `src/hooks/useKeyboard.ts:7-10` documents a shipping map of ⌘R / ⌘⇧R / ⌘U / ⌘⇧U / ⌘A / Esc /
  ⌘L / ⌘F / ⌘1-9, of which ⌘A, ⌘U and Esc read and write the `selection` store FR-6 eliminates
  (`useKeyboard.ts:90`, `:97`, `:74`). Nothing in the PRD says whether those four accelerators
  are re-pointed at draft membership, dropped, or left as-is. This is not a request to reinstate
  anything D37 removed — D37 keeps this map by name and classifies it as functional. *Fix:*
  extend RP-2 to name the application accelerators that must survive `app.set_menu` (at minimum
  ⌘R, ⌘L, ⌘F alongside the Edit/Window standards), and add one FR-6 consequence stating that
  ⌘A now stages every eligible Package matching the active filter through the same batch
  membership operation, with ⌘U / Esc either re-specified or listed under FR-6's Out of Scope.

- **medium** FR-12, FR-17, FR-19 and RP-2 realize no journey (§2.3 mapping table) — the AJ table
  covers 18 of 22 FRs plus RP-1. FR-12 is the trust boundary the Vision opens on and SM-3
  validates, yet no journey reaches it; FR-19 owns the entire interface; FR-17 owns Settings.
  Additionally §4.4's header says "Realizes AJ-5" while §2.3 maps AJ-5 to FR-15 and FR-18 only,
  so the group→journey and journey→FR mappings disagree (also recorded as `reconcile-ux.md` F13).
  *Fix:* either extend AJ-3/AJ-5/AJ-6 rows to cite FR-12, FR-17, FR-19 and RP-2, or add one line
  under §2.3 stating that these four are cross-cutting and deliberately journey-independent —
  the latter is honest and cheaper, but it has to be said, not left as a gap in a table.

---

## Shape fit — strong

The shape matches the product almost everywhere. Personas are absent and JTBD carries their
weight; UJs are delegated to `EXPERIENCE.md` by ID rather than restated, which is the correct
brownfield chain-top move and avoids a fourth ID namespace; success metrics are operational
invariants, not funnel measures; there is no rollout plan, sign-off block, or risk register.
The Status tag mechanism is the right brownfield instrument and `prd.md:26` guards it against
the obvious misreading — "The tags are implementation status, not requirement strength."

The `addendum.md` split is the strongest structural decision in the package: mechanism is
excluded from the PRD *and* routed to a named owner, with §1's table stating for each item why
it is not a requirement, so nothing is lost and nobody re-derives it into the PRD later. §2's
rejected-alternatives list is scoped exactly to things "proposed again by someone who does not
know why it was refused."

The one shape cost is the one already reported under dimension 6: delegating journeys means the
FR→journey mapping table is the *only* traceability surface, so its four gaps are load-bearing
in a way they would not be if the narratives lived here.

No further findings.

---

## Mechanical notes

- **Dangling cross-reference in the Glossary.** `prd.md:104` — "**Outdated** — a Manager's
  verdict that an update exists. Pack-Manager never infers it. See PI-1 in FR-2." `PI-1` occurs
  exactly once in the PRD (`grep -rn "PI-1"` → `prd.md:104` only) and is defined in no current
  planning artifact; it is inherited from the retired PRD's Product Invariants. This sits in the
  Glossary, the section downstream workflows source-extract from hardest. Drop the pointer or
  restore the invariant. (Also `reconcile-archived-prd.md` Finding 7.)
- **`docs/SPEC.md` F5 is mis-cited for the Esc behavior.** `prd.md:236` describes the conflict as
  "`docs/SPEC.md` F5 (Esc clears a transient selection)". F5 (`docs/SPEC.md:78-80`) says
  "`Add N to Plan` immediately adds the checked canonical identities to the persistent plan and
  clears the transient selection" — no Esc. Esc-clears-selection lives at `docs/SPEC.md:288`
  under `### 4.11 Keyboard map`. The framing was inherited from `ARCHITECTURE-SPINE.md:1050`.
  Consequence: a maintainer acting on `addendum.md:56` ("**adding F5 to the §0.1 supersession
  list**") fixes F5 and leaves §4.11 stating an Esc behavior FR-6 abolishes. Widen that
  instruction to F5 **and** §4.11.
- **ID continuity is clean.** FR-1…FR-22 contiguous and unique (`grep -c "^#### FR-"` = 22),
  NFR-1…NFR-8, RP-1/RP-2, SM-1…SM-5 + SM-C1/SM-C2, AJ-1…AJ-6, no duplicates, no gaps.
- **Assumptions Index roundtrips trivially** — zero inline `[ASSUMPTION]` tags, zero index
  entries, and §10 explains the mechanism that replaced them rather than leaving the zero bare.
- **Glossary discipline holds inside the PRD.** Manager / Package / Operation / Executor /
  Subject / Route / Snapshot / Upgrade Plan / Plan Capability / Plan Attempt are used verbatim
  throughout. One term crosses a boundary uncleanly: Glossary "**Managed by**" vs the
  user-facing string required by UX (recorded as `reconcile-ux.md` F10) — carried here only as
  a note because it is already in the queue.
- **Spot-checked source claims that hold exactly:** 20 IPC commands (`lib.rs:232-253`), 6 events
  (`events.rs:77-82`), 8 `Settings` fields (`settings.rs:28-39`), `ISSUED_PLAN_LIMIT = 64`
  (`state.rs:25`), `MAX_CONCURRENCY = 4` (`queue.rs:48`), batch flush constants
  (`events.rs:183-187`), retention constants (`logging.rs:26-28`, `journal.rs:19`),
  `ARCHITECTURE-SPINE.md:1050` is indeed the transient-selection OPEN row, `.memlog.md:75`
  matches the §9.1 quotation verbatim, and the "9 of 17 / four P0s" count is exact.
