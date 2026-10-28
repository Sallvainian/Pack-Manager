# PRD Quality Review — Pack-Manager (post-Update-2, spine revision 10)

Reviewed: `prd.md` (768 lines) and `addendum.md` (81 lines), 2026-07-25, against
`.claude/skills/bmad-prd/assets/prd-validation-checklist.md`. Every quote below was read this
session; every line number was resolved with `Read`/`awk` against the file at the time of review.

## Overall verdict

The thinking holds up: the Vision, the Glossary, the trust arc through §4.2, and the numeric
bounds in §5 are earned rather than furnished, and the fourteen new **Architecture binding**
blocks spot-check clean against the spine (AD-17 at `:763`, AD-24 at `:995`, AD-28 at `:1181`,
AD-30's "**Queued counts as running.**" at `:1463`). What is at risk is the handoff, again: the
reconciliation queue in `addendum.md` §3 and the closing bullet of `prd.md` §9.1 were not carried
forward with the rest of the Update pass and still address the spine as revision 9, instructing a
`bmad-architecture` run to write an invariant that revision 10 already wrote and to retire a row
that already reads `RESOLVED`. One shipping capability — health fixes — has a Glossary entry, an
Operation kind, a registered IPC command and a §7.1 scope line, and no FR anywhere.

## Decision-readiness — strong

Decisions read as decisions and the losing side is named. FR-14's carve-out is the clearest
example: `:381` states the exclusion, the mechanism *and* the reason in one bullet — "**An
OS-initiated shutdown or logout is deliberately excluded from that promise.** It gets **no
dialog**" … "blocking a logout to argue with the user is worse than losing the run" — and then
refuses to let the reader read it as a weakening: "which is why this is a narrower promise, not a
weaker guarantee." FR-7's `:275` does the same in the opposite direction, refusing to let the D28
opt-out arrive without its price: "The compensations are the price of the opt-out, not a nicety
attached to it."

The four Open Questions at `:720`–`:723` are genuinely open — each names the thing that is
*unstated* rather than posing a question the next sentence answers (`:722` "Automatic retention is
defined (newest 1,000). User-initiated deletion is not."). §9.2 `:753` is the strongest
decision-readiness signal in the document: it finds AD-28 self-contradicting on the Esc cascade,
quotes both halves, rules for the PRD, and then declines to fix the spine — "**One divergence is
recorded, not fixed, because the stale side is the spine's.**" I verified both halves at
`ARCHITECTURE-SPINE.md:1272`–`1278`; the reading is correct.

The four `[NOTE FOR PM]` callouts sit at real tensions (`:259` a defect the new FR could inherit,
`:450` a decision-record edit only the owner may make, `:689` a scheduling trap), not at safe
checkpoints. §10's three judgment calls each name the alternative a reviewer might have chosen.
No findings.

## Substance over theater — strong

There are no personas — §2 is a JTBD list, and it marks its own load-bearing entry (`:68`
"**Emotional (the load-bearing one):**"). The Vision is unswappable: `:58` grounds it in
`2.0.14-1`, `1.6.2.dev0`, `stable`, commit hashes, and the mise-shim misrouting, and closes with a
claim that could fail — "A tool that guesses at any of this is worse than the six terminals it
replaces."

The NFRs carry thresholds, not adjectives: NFR-3 `:595` ("Live output flushes at 50 ms, 64 lines,
or 8 KiB, whichever comes first. The newest 5,000 live lines are retained"), FR-8 `:284` ("at most
64 unconsumed capabilities are retained per session, oldest evicted first"), FR-15 `:396`
("application logs pruned beyond 14 days, transcripts kept to the newest 200 files or 90 days,
History compacted to the newest 1,000 records"). §8 refuses adoption metrics on stated evidence
rather than omitting them silently. The one soft patch is FR-5's feature NFR at `:211` — see the
done-ness findings.

## Strategic coherence — strong

The thesis is stated once and load-bears: `:56` "The differentiator is not that it wraps update
commands in a window. It is **confidence across a mixed Manager topology**." §4.2's description
`:217` restates it as the product's central promise — "*nothing runs that was not staged and
shown*" — and SM-2 makes it falsifiable: `:703` "A single violation is a P0 defect, not a metric
miss." The counter-metrics are real counters, not decoration: SM-C1 `:713` "Anyone optimizing
clicks-to-upgrade is optimizing away the product," which directly contradicts the usual instinct
about FR-7's confirmation gate.

Prioritization follows the thesis rather than ease: §7.2 groups the entire D27–D30 block as "one
coherent block" `:677` rather than cherry-picking the cheap limbs, and §7.3 removes health fixes
from Deferred with the reason stated against the document's own error (`:690` "Deferring them was
this document committing the very error the note above warns against, one sentence later").

### Findings

- **medium** SM-2's traceability omits the FR that most directly serves it (§8, `prd.md:703`) —
  SM-2 reads "Validates FR-7, FR-8, FR-10", but FR-6 is the requirement whose failure breaches it:
  `:235` "Scoping the D27 work to the row action alone would leave two unstaged mutation paths
  alive and breach SM-2." FR-11's `:331` global self-update toggle and FR-9's atomic admission are
  in the same class. A reader tracing SM-2 → FRs to decide what must be tested before release
  reaches neither the three call-site removals nor the self-update membership change.
  *Fix:* extend SM-2's list to "FR-6, FR-7, FR-8, FR-9, FR-10, FR-11".

## Done-ness clarity — adequate

Most FRs carry testable consequences at the right grain, and several state the *reason* a weaker
test would pass while the requirement failed — FR-8 `:289` on the rebuild window ("a confirmation
in that window would execute something other than what is on screen while passing every other
check in this FR"), FR-15 `:397` on reachability ("Retention without these affordances would
satisfy every other consequence in this FR and still leave the evidence unusable"). FR-5 `:200`
and FR-19 `:478` both specify the mechanism *and* the forbidden mechanism, which is what makes them
buildable without a second conversation.

Two things keep this from *strong*. One shipping capability has no requirement at all, and FR-6 —
the FR with the largest story load — carries a bullet inside its testable list that asserts the
opposite of the bullet three lines above it.

### Findings

- **high** Health fixes ship but no FR states the requirement (§3 `:120`, §4 whole, §7.1 `:670`) —
  `HealthFix` is one of the four Operation kinds (`:109`), it appears in FR-3's refresh rule
  (`:175` "A successful Upgrade, SelfUpdate, or HealthFix refreshes every affected subject and
  executor") and in FR-8's lock rule (`:286`), §7.1 lists it as shipping with a safety constraint —
  "uv health fixes, with the exact-suggestion-only constraint on what becomes runnable" — and
  `src-tauri/src/lib.rs` registers `commands::run_health_fix` among the 20 handlers. Yet
  `grep -n "^#### \(FR\|RP\|NFR\)" prd.md` returns no requirement covering it. The only normative
  statement of the constraint is a Glossary clause (`:120` "Only a narrowly recognized fix may
  become runnable"), which is exactly where downstream workflows do *not* source-extract
  requirements. `bmad-create-epics-and-stories` and `bmad-architecture` walk §4; they will not see
  a capability that runs Manager-suggested commands.
  *Fix:* add an FR in §4.3 (a free ID, or an explicit limb on FR-16) stating: a Manager-reported
  Health issue is surfaced with its evidence; only a fix matching an exactly recognized suggestion
  becomes runnable; a runnable fix is a HealthFix Operation subject to every FR-8/FR-9/FR-12
  protection; nothing else in the Manager's output becomes an executable affordance.

- **high** FR-6 states removed behavior as a testable consequence, contradicting the bullet three
  lines above (§4.2 FR-6, `prd.md:236`) — inside "**Consequences (testable):**" the list carries
  "The draft is transient dialog state discarded on close — pre-D27 behavior this FR removes."
  while `:234` in the same list carries "The draft persists while the user navigates between
  Managers and the Dashboard, and every staged item is individually removable from the Upgrade
  Plan." A story author converting FR-6's consequences into acceptance criteria gets two ACs that
  cannot both hold, on the FR that anchors the largest D27 story cluster. FR-6 already has an
  **Out of Scope** block at `:243` whose existing entries are phrased exactly this way, and whose
  current content ("A transient selection distinct from draft membership, and any `Add Selected`
  submit step") does *not* already cover draft transience.
  *Fix:* move `:236` into **Out of Scope** as "The draft as transient dialog state discarded on
  close (pre-D27)."

- **medium** "Retry where appropriate" leaves the Retry predicate undefined (§4.3 FR-13, `:365`) —
  the Planned D29/D30 consequence ends "a terminal Results summary with successes, failures,
  skipped work, verification outcomes, and Retry where appropriate." Nothing in the document says
  *when* Retry is offered. FR-15 `:398` defines only what it does once invoked ("Retry creates a
  **new linked attempt** and never overwrites the first failure"), and FR-8 `:287` states the
  adjacent prohibition ("A failed execution other than a stale-plan rejection is never retried
  speculatively"). Two stories will read "appropriate" differently — one offering Retry on any
  non-success, the other on failures only — and both will pass review.
  *Fix:* replace "where appropriate" with the condition, e.g. "Retry is offered for failed and
  skipped work within a terminated attempt, and is never offered while the attempt is unterminated
  or for work whose capability was consumed by a completed Operation."

- **low** Three consequences are adjectives without bounds (FR-20 `:514`, FR-21 `:528`, FR-5
  `:211`) — "Package work remains understandable and uninterrupted throughout"; "Every
  update-stage failure is actionable"; "The Package list stays usable and responsive beyond 100
  Packages." Each has a bounded sibling elsewhere that it could simply invoke: FR-16 `:411`
  ("Errors state what happened and what to do next, in plain language, per error class") and
  NFR-3 `:595` ("with correct actions reachable at 101 rows").
  *Fix:* point each at its bounded sibling — FR-21's at FR-16's per-error-class rule, FR-5's at
  NFR-3's 101-row test — and give FR-20's "uninterrupted" its real content (no Operation is
  queued, cancelled, or delayed by an application-update check or download).

## Scope honesty — strong

Omissions are explicit and, unusually, dated to evidence. §7.1 `:673` refuses to claim a guard it
does not have — "**Not in this list, deliberately:** the automated contrast guard. The 4.5:1
assertion and the on-fill ink tokens that make it pass are **uncommitted working-tree changes**,
absent from `HEAD` `5972109`" — and FR-19 `:476` and NFR-6 `:615` both carry the same admission
rather than letting §7 hold it alone. FR-14's `:371` "**The quit guard is unbuilt**" and `:380`
"Do not read this consequence as shipping" are the correct treatment of a dialog that exists but is
unwired. §6 Non-Goals is doing real work — eleven entries, each stating the boundary in the terms
the product would be pushed across it ("A rollback engine", "A retry loop", "A version oracle").

Open-items density is right for the stakes: four Open Questions, all declared non-blocking at
`:725` ("This document carries **no phase-blockers**"), four `[NOTE FOR PM]`, and zero
`[ASSUMPTION]` tags with the reason stated at `:759` ("Every requirement in this document traces to
a named source"). For a brownfield PRD written against a readable codebase, zero assumptions is
credible rather than suspicious. The one scope statement without a requirement behind it — health
fixes — is reported under Done-ness.

## Downstream usability — adequate

The document itself extracts cleanly. IDs are contiguous and unique (FR-1…FR-22, RP-1, RP-2,
NFR-1…NFR-8, verified by heading grep — no gaps, no duplicates), the §2.3 journey table declares
its own incompleteness rather than hiding it (`:96` "this table is not a complete index of the FR
set"), and every **Architecture binding** block I checked resolves to real spine text. Cross-refs
inside the PRD resolve.

What does not hold up is the operational output — `addendum.md` §3, the queue that tells the three
consuming workflows what to fix. It was written against spine revision 9 and the Update pass did
not carry it forward, so its spine row now instructs work that revision 10 already did, at line
numbers that now point at unrelated text; and its epics row omits the two supersessions the
PRD asserts most sharply. This is the same dimension the prior gate called thin (`:761`
"**downstream usability** the one thin dimension — the handoff, not the thinking"), and it is
still the thin one.

### Findings

- **high** The spine reconciliation instruction is stale against revision 10 (`prd.md:740`;
  `addendum.md:55`) — `prd.md:740` reads "`ARCHITECTURE-SPINE.md:1050` still records this as OPEN
  and routes it to the owner. It is now answered, and that row should be **retired** in the
  `bmad-architecture` Update that follows this PRD." `addendum.md:55` labels the artifact
  "`ARCHITECTURE-SPINE.md` (rev 9)" and elaborates: "the OPEN row at 1050 (transient selection vs
  plan membership) is **now closed** … Retiring it is not enough on its own: the row exists because
  no invariant models the relationship, and FR-6's batch membership operation still needs one.
  Write the invariant, then retire the row." All three claims are overtaken. At revision 10 the row
  reads `ARCHITECTURE-SPINE.md:1595` "| Transient selection has no owning invariant | **RESOLVED**
  | Closed by **AD-28**, written 2026-07-25 on the owner's decision"; the invariant exists at
  `:1181` "### AD-28 — A Package checkbox *is* membership, and a range is one batched operation";
  and line 1050 is now blank (AD-26's heading is at `:1051`). The row's other citations have moved
  too — `addendum.md:55` cites "the manual pass at 332 and 941", where the spine now reads `:332`
  "coherent *after* upload, which cannot gate" and `:941` "stands, the prior preference is
  retained as both active and persisted state". The PRD stamps revision 10 as the authority at
  `:28`, so the two documents disagree about which revision they are reconciled to.
  *Fix:* rewrite `prd.md:740` to record the row as already closed by AD-28 (no further
  architecture action), and rewrite the `addendum.md` §3 spine row against revision 10 — drop
  "(rev 9)", drop the "write the invariant" instruction, and either re-anchor the remaining
  keyboard/VoiceOver work by AD id and section name or delete the line numbers entirely, per the
  row's own "never by a mention count" discipline.

- **high** The epics reconciliation row omits two supersessions this PRD asserts by name
  (`addendum.md:54`, with `addendum.md:50` "**Scope every run by the named sections below, never by
  a mention count.**") — the row names only the FR/NFR block, "FR-19 (line 89) and NFR-6 (line
  113)", Story UX-PB.1d, and FR-17's unqualified settings. It does not name the two places
  `epics.md` contradicts this PRD's *requirements* rather than its accessibility scope. (1)
  `epics.md:1262` "### Story 3.5: Preserve Exact Keyboard Selection and Row Plan Actions", whose
  criterion at `:1278`–`:1280` runs "**When** toggle, shift-range, tri-state, Cmd+A, Space,
  Cmd-click, Clear, and Esc interactions execute … **And** excluded rows never enter selection" —
  the transient-selection model FR-6 `:244` eliminates, plus the `Clear` and `Esc` rungs FR-6
  `:239` removes. (2) `epics.md:740` "running work moves to `Cancelling` and escalates through the
  existing process-group mechanics" and `epics.md:903` "changes still-running Operations bound to
  that `planAttemptId` to `Cancelling`" — a state FR-13 `:361` forbids by name: "There is no
  distinct `cancelling` state". A `bmad-correct-course` run scoped to the named sections, as
  instructed, converts the accessibility limbs and leaves both. Partially mitigated:
  `ARCHITECTURE-SPINE.md:1594` carries both and calls the `Cancelling` item ordering-critical
  ("it must be corrected *before* UX-PB.2e is built, not after") — but the PRD's queue presents
  itself as the scope, and a run driven from it alone is under-scoped.
  *Fix:* add both rows to the `addendum.md` §3 epics entry, naming Story 3.5, UX-PB.2e and
  UX-PB.3g and the PRD line each contradicts, and note the ordering constraint on UX-PB.2e.

- **medium** Three normative surface names are absent from the Glossary (§3, `:102`) — §3 opens
  "Downstream workflows must use these terms exactly. No synonyms appear anywhere else in this
  document," yet **sidecar**, **Activity** and **Results** all carry normative weight and none is
  defined. `sidecar` appears in a Planned consequence (FR-7 `:272` "the sidecar is hidden when
  empty, appears on first addition, persists across navigation"), in an accelerator definition
  (RP-2 `:569` "**⌘L** (move focus into the Upgrade Plan sidecar region)"), and in FR-6's Esc
  reasoning (`:239`). `Activity` and `Results` are named as destinations in FR-13 `:365`, FR-19
  `:468` and NFR-3 `:595`, and appear *inside* the Glossary's own Plan Attempt entry (`:116` "The
  unit of Activity, Results, and History under D29") without being entries themselves. A `bmad-ux`
  or `bmad-architecture` run cannot tell from §3 whether the sidecar is the Upgrade Plan or the
  surface that renders it.
  *Fix:* add three entries — "Upgrade Plan sidecar" (the persistent layout region rendering the
  draft, visibility derived from content), "Activity" (the first-class destination showing the
  active Plan Attempt), "Results" (the terminal summary surface for a completed attempt).

- **low** The addendum's re-measured keyword counts no longer hold for the spine
  (`addendum.md:50`) — the paragraph exists to replace copied counts with measured ones: "Re-measured
  with `grep -ciE \"voiceover|keyboard\"`, the real figures are **13, 8, and 11** lines (21, 9, and
  13 raw occurrences)." Re-run this session with the same command, `epics.md` returns 13 lines / 21
  raw and `EXPERIENCE.md` returns 11 / 13 — both still exact — but `ARCHITECTURE-SPINE.md` returns
  **15 lines / 33 raw**, against the stated 8 / 9. The cause is benign and documented at
  `ARCHITECTURE-SPINE.md:1594` ("Nor by *this* spine's keyword count, which **rose** in revision
  10 — applying D37 means recording what was retired, and recording it uses the words"), and the
  paragraph hedges ("Counts below are indicative only and will drift again"), but a paragraph whose
  entire argument is "measured, not copied" now carries a stale figure of its own.
  *Fix:* drop the spine figure, or restate it as measured against revision 10 with the reason the
  count rose.

## Shape fit — strong

The shape matches the product. This is a single-operator brownfield tool, and the PRD takes the
capability-spec shape rather than pretending at a consumer product: no personas, JTBD instead, and
success metrics that are operational — SM-1 `:702` "The maintainer reaches for Pack-Manager instead
of six terminals, and still does so a month from now." §2.3 declines to mint a fourth ID namespace
and mirrors `EXPERIENCE.md`'s AJ ids instead (`:83` "there are already three ID namespaces in this
project (`FR-`, `AD-`, `AJ-`) and a fourth would be a liability") — the right call for a chain-top
PRD whose UX artifact already exists.

Brownfield handling is the strongest shape signal: §0.1's table names each stale `docs/SPEC.md`
claim against what the code says, and the Status tags keep Current and Target separable per FR
rather than per document. Rigor is calibrated to the stated scale rather than to a template
(`addendum.md:72`–`:79`), and §5's numbers survive that calibration — light rigor, not light
substance. No findings.

## Mechanical notes

- **ID continuity.** `grep -n "^#### \(FR\|RP\|NFR\)" prd.md` returns FR-1…FR-22 in order with no
  gaps or duplicates, RP-1 and RP-2, NFR-1…NFR-8. Every FR referenced from §2.3, §8 and
  `addendum.md` resolves.
- **Assumptions Index roundtrip.** No inline `[ASSUMPTION]` tags and no index; §10 `:759` states
  the reason. Roundtrip is vacuously clean.
- **UJ protagonists.** AJ-1 names Sallvain inline (`:87`); AJ-2…AJ-6 do not, but the narratives are
  explicitly held in `EXPERIENCE.md` lines 373–460 (`:94`) rather than restated, so the table is a
  pointer, not a UJ set. Consistent with the §2.3 decision; not a defect.
- **Settings arithmetic, FR-17.** `:444` closes "The other seven fields are the target set", but
  `:443` immediately above adds one — "**Planned — D28:** `skipUpgradePlanConfirmation` is added
  with a safe default of `false`". The target set is the seven survivors *plus*
  `skipUpgradePlanConfirmation`, i.e. eight. The shipping table at `:434`–`:441` does list exactly
  eight rows as the status line claims. *(low)*
- **Citation precision, §9.2.** `:753` cites "`:1272`–`1273`" for the quote "the cascade drops from
  three rungs to two and keeps close-dialog"; in the current spine that sentence spans `:1272`–
  `:1274` (the words "close-dialog" fall on `:1274`). The companion citation "`:1276`–`1278`" is
  exact, and the substance of the finding is correct.
- **Spine quotes spot-checked and clean.** AD-17 at `ARCHITECTURE-SPINE.md:763`, AD-24's "custody
  transfer" at `:995`, AD-30's "**Queued counts as running.**" at `:1463`, AD-28's heading at
  `:1181`. Fourteen **Architecture binding** blocks are present (`grep -c`), matching the Update
  pass's stated scope.
- **§0.1's code claims re-verified.** `src-tauri/src/lib.rs` `generate_handler!` registers 20
  commands (`commands::detect_managers` … `commands::install_app_update`), including
  `commands::run_health_fix`; the §0.1 row claiming SPEC's "17 commands" against a real 20 holds.
