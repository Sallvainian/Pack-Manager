# Divergence Review — ARCHITECTURE-SPINE.md revision 4

**Lens:** Construct two units one level down that each obey every AD to the
letter yet still build incompatibly — clashing shared-data shapes, two owners of
one entity, conflicting state-mutation paths.

**Target:** `_bmad-output/planning-artifacts/architecture/architecture-Pack-Manager-2026-07-23/ARCHITECTURE-SPINE.md`
(revision 4, 505 lines)

**Units:** the 28 stories of Epic UX-PB and Stories 2.2, 3.1, 3.2, 3.4, 3.5, 6.5
in `_bmad-output/planning-artifacts/epics.md` (1,166 lines)

**Method:** every finding names story A, story B, the compliant-but-incompatible
choice each makes, and the AD text that fails to prevent it. Where an AD already
forbids one of the two units, the candidate is listed in
"Constructed and dropped" instead, with the AD that killed it.

---

## Verdict

The spine's *type* separation is airtight — `planId` vs `planAttemptId` (AD-16),
Rust-owns-intent (AD-17), own-journal (AD-18) are each unambiguous, and several
obvious divergences are correctly pre-empted. What is missing is *temporal and
custodial* law. The spine says who owns each entity's shape but almost never
says **when** an entity changes hands, **what happens to it at handoff**, or
**which of two concurrent writers wins**. Fourteen constructed pairs survive.
Three are critical: two of them let Pack-Manager run or report package mutations
the user never reviewed or never actually got.

---

## Critical

### D-1 — `AllEligible` re-expansion smuggles unreviewed members into a confirmation-off run

**Unit A — UX-PB.1c** builds `AllEligible` as a *live predicate*. This is the
only reading under which `AllEligible` is a distinct durable kind rather than a
cosmetic label on an `Explicit` list, and the spine's own wording assumes
re-evaluation:

- Spine 336–338: "`PlanIntent` distinguishes explicitly chosen membership from
  bulk `AllEligible` membership, durably: **a bulk-added item the user removes
  stays removed across a rebuild**". A removal can only "stay removed *across a
  rebuild*" if the rebuild otherwise re-derives the bulk set.
- Spine 295: "`kind: Explicit | AllEligible      # durable; a removal converts
  AllEligible -> Explicit`" — no scope qualifier, no freeze semantics.
- epics.md 453: "`Update Everything` seeds all eligible work while remaining
  editable".

**Unit B — UX-PB.5c** builds the confirmation-off run exactly as specified:

- epics.md 959: "Rust rebuilds the exact commands from canonical intent and runs
  the stale-plan check before the plan is atomically admitted".
- epics.md 955: "the immediate action is `Run N Updates`, and no dialog opens".

**The incompatibility.** A background refresh (FR-3; Story 2.2) discovers three
newly-Outdated packages between staging and the click. Unit A's rebuild
re-expands the `AllEligible` set to include them. Unit B admits and runs the
rebuilt plan with no dialog. Three packages the user never saw are mutated.

Both units obey every AD. AD-16's staleness machinery only runs in the
*subtractive* direction — spine 347–348: "pinned, already current, a
non-opted-in greedy cask, **or removed** between staging and rebuild is inert" —
and UX-PB.5c's own stale check enumerates only subtractions (epics.md 962: "a
Package pinned, updated, or removed since staging"). Nothing anywhere forbids a
rebuild from **adding** membership.

**Missing AD.** A rebuild may never enlarge reviewed membership. Proposed rule
under AD-16: *"A canonical rebuild may remove or invalidate membership; it may
never add a member the user has not seen. An `AllEligible` intent freezes its
expansion at the mutation that created it; newly eligible items surface as an
explicit offer to re-seed, never as silent membership. If a rebuild would enlarge
membership, the preview `planId` expires and re-review is required."*

---

### D-2 — Verification can be satisfied by a coalesced *pre-mutation* refresh

**Unit A — Story 2.2** owns refresh behavior and preserves the coalescer,
because AD-4 orders it to. Spine 184–185: "The global concurrency cap of 4, the
120s aging guard, and **duplicate-refresh coalescing** are preserved."

**Unit B — UX-PB.3d** owns verification and implements AD-16 rule 5 literally.
Spine 269–270: "A mutating attempt is not successful until the required affected
Manager refreshes complete." epics.md 707: "**When** the required refresh
verification for the affected Managers completes **Then** the attempt becomes
terminal".

**The incompatibility.** `brew upgrade` finishes at T+0. A user-initiated
Homebrew refresh started at T−5s is still in flight. Unit B requests the
verification refresh; Unit A's coalescer — correctly, per AD-4 — attaches it to
the in-flight refresh instead of starting a new one. That refresh's `brew
outdated` snapshot was taken *before* the mutation. It completes at T+3s. Unit B
sees "the required affected Manager refresh completed" and marks the attempt
verified — from data that predates the mutation it claims to verify.

The symmetric failure is equally reachable: the pre-mutation snapshot still lists
the package as Outdated, so a successful update reports **verification failure**.

Both units obey every AD. AD-16 rule 5 says the refresh must *complete*; it never
says it must have *started after* the mutating process exited. AD-4 mandates the
coalescing that makes the violation invisible. This defeats the exact property
AD-16 rule 5 exists to create — spine 246 lists "reporting success before
verification" among the things AD-16 prevents.

**Missing AD.** Proposed rule under AD-16 (or a cross-reference in AD-4): *"A
verification refresh must be a fresh acquisition whose data collection begins
strictly after the mutating process exited. Verification refreshes are exempt
from duplicate-refresh coalescing against refreshes already in flight at
mutation-exit time; a coalesced refresh may satisfy verification only if it was
started after that instant."*

---

### D-3 — The draft's fate at admission is unowned, and AD-17 contradicts itself either way

**Unit A — UX-PB.1b** owns sidecar lifecycle and implements AD-17's visibility
rule literally. Spine 370–371: "The sidecar is a single layout region **whose
visibility is driven by draft non-emptiness**". epics.md 427: "**When** I add the
first eligible item **Then** the Upgrade Sidecar opens showing the draft grouped
by Manager".

**Unit B — UX-PB.3a / UX-PB.3d** own the confirmed sidecar and implement AD-17's
content rule literally. Spine 374–375: "A confirmed attempt replaces the
sidecar's content with live attempt status rather than opening a second surface."
epics.md 649: "the same Upgrade Sidecar transforms in place into the one active
plan summary". epics.md 707: "the sidecar transforms in place into a persistent
Results Summary **that remains until `Done`**".

**The incompatibility, branch 1 — admission consumes the draft.** Unit B's
sidecar now shows live attempt status while the draft is empty. AD-17's
visibility rule (Unit A) says the region is hidden. Unit A hides the only surface
Unit B has. Worse at terminal: Results must "remain until `Done`" over an empty
draft, which the visibility rule forbids for as long as it takes the user to
click.

**The incompatibility, branch 2 — admission leaves the draft intact.** Then
UX-PB.3a's own requirement is unsatisfiable: epics.md 653 says "**the new draft
stays in the Upgrade Plan** and cannot be confirmed until the active attempt is
terminal" — but the Upgrade Plan region is displaying attempt status per spine
374–375, and spine 372 forbids a second instance ("**Exactly one instance
exists**"). The draft has no home while an attempt runs, yet UX-PB.1a/1c/3.5
require every entry point to keep staging into it (epics.md 1133: "the sidecar
reflects the membership change"). When the attempt terminates, Results and a
stale draft both claim the region.

Both units obey every AD. No AD says whether admission consumes, clears,
snapshots, or preserves the draft; AD-16 defines the *types* on both sides of
admission (spine 258–260) but not the custody transfer.

**Missing AD.** Proposed addition to AD-17: *"Admission moves reviewed membership
out of the draft: the draft is emptied atomically with the mint of
`planAttemptId`, and a failed admission restores it unchanged. While an attempt
is non-terminal the sidecar region is owned by attempt status, and its visibility
is driven by `attempt is non-terminal OR results not dismissed OR draft
non-empty`. New draft membership staged during a live attempt accumulates in the
canonical draft and surfaces in the region only once the attempt's Results are
dismissed."*

---

## High

### D-4 — Two stories can assign different terminal states to the same item

**Unit A — UX-PB.3g** implements cancel-while-verifying. epics.md 781: "verifying
items resolve to honest terminal outcomes (**cancelled or skipped** rather than
falsely verified)".

**Unit B — UX-PB.3d** owns the closed item taxonomy. epics.md 711: "each item is
verified, failed, cancelled, or skipped — mutation failure and verification
failure are distinguished, **`Skipped` marks only work that never started**".

**The incompatibility.** `brew upgrade jq` exits 0; the package *is* upgraded on
disk; verification is pending; the user hits `Cancel plan`. Unit A must pick
`cancelled` or `skipped`. `skipped` is forbidden by Unit B (the work started).
`cancelled` tells the user the update did not happen — it did. Unit B has no
fourth option: `verified` is forbidden by AD-16 rule 5 and by 3g's own "no item
is reported successful because its exit preceded the cancel", and `failed` is a
lie in the other direction.

Both units obey every AD. AD-16 supplies four item outcomes and a `verifying`
attempt state (spine 311: "state: admitted | running | verifying | terminal") but
no item outcome for *mutation applied, verification never ran*. The state machine
at spine 330 covers only "Running --> Terminal: cancelled, unstarted work
Skipped" — it has no transition out of `Verifying` other than to Terminal via
Results.

The same gap fires on crash: UX-PB.4a reconciles a non-terminal attempt as
Interrupted (epics.md 804), so a fully-applied-but-unverified upgrade is reported
Interrupted, and the user re-runs work that already succeeded.

**Missing AD.** Proposed rule under AD-16: *"The item outcome set is closed and
distinguishes mutation from verification: `verified`, `applied-unverified`,
`mutation-failed`, `verification-failed`, `cancelled` (mutation did not complete),
`skipped` (never started). Cancellation or interruption during `Verifying` yields
`applied-unverified`; no path may report an applied mutation as cancelled,
skipped, or interrupted."*

### D-5 — Persist-before-admit vs. "an append failure is nonfatal": a running attempt with no durable record

**Unit A — UX-PB.2c** builds persist-then-admit. epics.md 569: "the failure is
surfaced, **no partial attempt record is left behind**, and the prior consistent
state is preserved rather than proceeding as if durably recorded."

**Unit B — UX-PB.2b** builds admit-then-persist, on the strength of AD-18. Spine
387–388: "under the same discipline: **an append failure is nonfatal to package
operations**". Unit B mints `planAttemptId`, admits atomically (epics.md 544:
"`execute_plan` atomically returns one new durable `planAttemptId`"), starts the
operations, and treats the journal append as best-effort exactly as AD-18 says.

**The incompatibility.** Under Unit B a full multi-Manager upgrade runs with no
durable record: UX-PB.4a's reconciliation finds nothing, UX-PB.4b's replay has
nothing to reconstruct, and Story 6.5's export omits an attempt that really ran.
Under Unit A the same append failure blocks all upgrades — which is defensible,
but is the opposite policy, and violates AD-18's plain text if operations were
already admissible.

Both units obey every AD. AD-19's persist-before-active rule is written for
settings only — spine 411: "**A settings patch** is persisted before it becomes
active in memory or advances the canonical revision" — and AD-18 affirmatively
licenses Unit B.

**Missing AD.** Proposed rule under AD-18: *"An attempt's admission record is
persisted before any of its Operations spawn; a failed admission append fails the
admission closed and mints nothing. AD-18's nonfatal-append rule applies only to
progress and finish records of an already-persisted attempt, never to its
admission record."*

### D-6 — UX-PB.5b's own acceptance criterion is unreachable: the settings write expires the capability it is admitting with

**Unit A — UX-PB.5b** implements its criterion in the order the criterion states.
epics.md 929: "**Then** `skipUpgradePlanConfirmation: true` is written
atomically, **the new value takes effect only after persistence succeeds**, and
the plan is admitted." AD-19 defines what "takes effect" costs — spine 411–412:
"is persisted before it becomes active in memory or **advances the canonical
revision**".

**Unit B — UX-PB.2b / UX-PB.5a** implements admission as AD-16 specifies:
admission consumes a one-use preview capability that is invalidated by staleness.
Spine 255–256: "expires on mutation, **staleness**, execution attempt, or
eviction". The verified baseline names the staleness token — spine 88–89: "the
durable token is a monotonic `revision` in `PlanCoordinator`".

**The incompatibility.** One click, two effects, and the spine fixes their order
in opposite directions. Unit A's ordering (persist → activate → advance revision
→ admit) hands Unit B a `planId` minted against revision *N* while the
coordinator now reads *N+1*: admission fails closed, and the user who ticked the
checkbox gets a rejection instead of the run epics.md 929 promises. Unit B's
ordering (admit → persist) satisfies both stories but violates AD-19's
persist-before-active rule for that same click if the setting is treated as
active during admission.

Both units obey every AD. The spine never says whether a settings-revision
advance is a plan-staleness event, and never orders a combined
preference-write-plus-admission.

**Missing AD.** Proposed rule under AD-19 or AD-16: *"Not every canonical-revision
advance is a plan-staleness event. `planId` staleness is evaluated only against
the plan-relevant epoch — detection, registry, queue records, and the eligibility
inputs — never against a settings patch that touches no plan input. A combined
preference-write-and-admission is one ordered unit: persist, then admit against
the pre-existing capability."*

### D-7 — Retry writes into the one persistent draft with no rule for the draft already there

**Unit A — UX-PB.1c** owns multi-entry-point staging and its convergence
contract. epics.md 465: "the draft **converges to one coherent deduplicated
membership set**, no item is doubled or lost". Every entry point feeds "the same
one persistent draft" (epics.md 453).

**Unit B — UX-PB.4d** owns Retry. epics.md 855: "`Create new plan` **rebuilds
current canonical intent into a new reviewable draft**, and confirming that draft
creates a new attempt with a fresh `planAttemptId` linked by
`retryOfPlanAttemptId`". Spine 273–275 backs it: "Retry always creates a new
`planAttemptId`, links to the preceding failed attempt... The backend rebuilds
current intent".

**The incompatibility.** The user has eleven packages staged, then opens History
and retries a failed attempt of two. Unit A's law says the draft merges and
deduplicates — so the retry scope silently absorbs nine unrelated packages, and
the resulting attempt carries `retryOfPlanAttemptId` while containing work the
original never had, corrupting the lineage UX-PB.4d exists to preserve. Unit B's
law ("a new reviewable draft") says the draft is replaced — silently destroying
eleven staged items with no confirmation and no undo, which AD-17's durable-draft
rule then faithfully persists.

Both units obey every AD. AD-17 establishes exactly one canonical draft (spine
361: "Rust owns the canonical `PlanIntent`", singular) and AD-16 requires the
retry rebuild, but nothing arbitrates the collision.

**Missing AD.** Proposed rule under AD-16: *"Retry's rebuilt scope never merges
into existing draft membership. If the draft is non-empty when Retry is invoked,
the user is told and chooses replace-or-cancel; a replaced draft is recoverable
until the retry attempt is confirmed. An attempt carrying `retryOfPlanAttemptId`
contains only membership derived from its source attempt."*

### D-8 — Launch ordering is unspecified, so a compliant rebuild empties the draft on every relaunch

**Unit A — UX-PB.1b** reconstructs the draft at launch, as AD-17 requires. Spine
364–365: "The draft is durable... and **reconstructed at launch**." epics.md 439:
"the draft's canonical membership is reconstructed into the sidecar".

**Unit B — UX-PB.1a** owns the rebuild and implements AD-17's round-trip rule and
AD-16's ineligibility drop. Spine 363–364: "Every mutation round-trips through
Rust before the projection updates." epics.md 409: "the now-ineligible item is
dropped or flagged with what changed, the plan is rebuilt from current canonical
truth".

**The incompatibility.** Unit A reconstructs the draft at launch and asks for a
rebuild. Detection has not completed. Under AD-4's coordinator-first rule the
rebuild reads `detection` (spine 178: "Any code reading plan-relevant state —
`detection`, `registry`...") and gets the pre-detection placeholder the baseline
documents (spine 158: "a real detection report is never clobbered by the
pre-detection placeholder"). Against a placeholder, every staged package is
"removed between staging and rebuild" (spine 347), so Unit B drops the entire
membership — correctly, by its own rules — and AD-17 then persists the emptied
draft. The user's plan silently vanishes at every launch.

Both units obey every AD. AD-3's rule protects the *detection report* from the
placeholder; it says nothing about other consumers reading the placeholder. No AD
orders draft reconstruction, attempt reconciliation, first detection, and
`get_state` hydration relative to one another.

**Missing AD.** Proposed rule under AD-3 or AD-17: *"Launch order is fixed:
subscribe, hydrate, reconstruct durable draft and attempt records as inert data,
run first detection, then rebuild. No canonical rebuild, eligibility decision, or
membership drop may execute against the pre-detection placeholder; a rebuild
requested before first detection completes is deferred, not resolved against
absent state."*

### D-9 — The two journals have uncoupled retention, producing records that are neither attempt nor legacy

**Unit A — UX-PB.2c / UX-PB.2d** writes attempt records to the new journal and
`planAttemptId`-bearing records to `operations.jsonl`, whose compaction the
baseline fixes at "the newest 1,000 records" (spine 94–95).

**Unit B — UX-PB.4a / UX-PB.4b / Story 6.5** reads them back. epics.md 795:
"exactly one immutable History row is created for that `planAttemptId`, **its
Operation-level evidence is nested inside that row**". epics.md 818: replay
"reconstructs the attempt's Manager groups, Package/version changes... exact
commands, Operation outcomes, errors, timings, and retained output".

**The incompatibility.** The two files compact independently, so either half can
outlive the other:

- `operations.jsonl` rolls past 1,000 while the attempt journal keeps its rows →
  UX-PB.4a's History row has empty nested evidence and UX-PB.4b's replay cannot
  reconstruct, though 4b's failure branch is written for corruption, not for
  lawful retention (epics.md 821–823).
- The attempt journal compacts first → `operations.jsonl` retains records bearing
  a `planAttemptId` with no attempt. These are a third class the spine has no
  name for: UX-PB.2f and UX-PB.4e define "legacy" strictly as records *lacking*
  the field (epics.md 629, 875), and AD-18 agrees — spine 391–392: "A record
  without one stays an individually labeled legacy Operation." An orphan with the
  field is undefined, and any reader that groups by `planAttemptId` fabricates
  precisely the phantom grouping AD-16 forbids (spine 247).

Both units obey every AD. AD-18 fixes the compaction *mechanism* — spine 388:
"compaction is temp file + fsync + rename, never truncate-in-place" — but never
couples the two files' retention or requires an attempt to be compacted as a
whole.

**Missing AD.** Proposed rule under AD-18: *"The two journals compact as one
retention domain. Compaction never splits an attempt: an attempt's admission,
progress, and finish records and its `operations.jsonl` records are retained or
dropped together, and no record bearing a `planAttemptId` may outlive its attempt
record. A `planAttemptId` that resolves to no attempt is a load-integrity error,
never a grouping key."*

---

## Medium

### D-10 — The attempt record's field list is deferred to one story while two others impose fields on it

**Unit A — UX-PB.2c** is the designated schema owner. Spine 503: "Draft and
plan-attempt file names and serde shapes | **Deferred** | ... the exact filenames
and field lists belong to UX-PB.1a and UX-PB.2c." Its own field list is closed
and does *not* include retry lineage — epics.md 564: "the reviewed
Manager/Package scope, Manager self-update identities, exact command snapshot,
version evidence, timestamps, and result/verification state".

**Unit B — Story 6.5** asserts a different field list on the same records.
epics.md 1161: "the durable plan-attempt records... **each carrying** its
`planAttemptId`, reviewed Manager/Package scope, exact commands, verification
facts, results, and **optional `retryOfPlanAttemptId`**". UX-PB.4d
(epics.md 855) owns the lineage link and would, absent a slot in 2c's schema,
reasonably keep it in its own structure.

**The incompatibility.** Unit A ships a schema without `retryOfPlanAttemptId`;
Unit B's export assertion fails on a record set that is otherwise correct. AD-16's
normative minimum does list `retryOfPlanAttemptId?` (spine 307), but the spine
also says of that block "Names may be refined during story implementation" (spine
289) and hands the field list to 2c at line 503 — two authorities, one schema.

**Missing AD.** Make AD-16's normative minimum binding as a *floor* on the
persisted shape and say so at the deferral row: *"UX-PB.2c owns filenames and
serialization; it may extend but never omit the fields of AD-16's normative
domain minimum. Any story requiring a field on the attempt record adds it through
2c's schema, not a parallel store."*

### D-11 — `planId` staleness is undefined, so the confirmation-off path silently gets a weaker guarantee

**Unit A — UX-PB.5a** mints the capability when the dialog opens; the
review-to-confirm window is seconds (epics.md 899: the dialog "shows the exact
commands that will run" then the user confirms).

**Unit B — UX-PB.5c** has no dialog. Its "reviewed preview" is the auto-expanded
sidecar (epics.md 955: "exact commands automatically expand"), so the capability
it admits with is whatever the last draft mutation produced — potentially hours
old.

**The incompatibility.** Both satisfy AD-16 — spine 252–253: "a bounded one-use
capability for **one reviewed preview**" that "expires on mutation, staleness,
execution attempt, or eviction" — because the spine never defines *staleness*.
Time-based, revision-based, and refresh-based readings are all compliant and give
windows that differ by orders of magnitude. UX-PB.5c's own promise (epics.md 959:
"the bypass removes only the final dialog and never the persistent plan, native
rebuild, stale check, or explicit user action") is then false in the dimension it
claims to protect.

**Missing AD.** Define it once under AD-16: *"`planId` staleness is a predicate on
the plan-relevant epoch: a capability is stale when the coordinator's
plan-relevant revision has advanced since it was minted, or when its bounded
lifetime has elapsed. The same predicate and the same bound govern the
confirmation and confirmation-off paths."*

### D-12 — Greedy-cask opt-in has two plausible owners, and the two disagree per-draft vs. globally

**Unit A — UX-PB.1c / UX-PB.1a** treat it as a draft option, because AD-16's
normative minimum puts it there. Spine 298: "`includeGreedyCasks: boolean`"
inside `PlanIntent`; spine 250–251: the draft holds intent "plus **explicit
option values**".

**Unit B — Story 3.2 / UX-PB.1d** treat it as a Setting, because the normative
user-facing copy says so. epics.md 479 fixes the exact string: "excluded `This
Package is excluded by **your Settings**. Change the **setting**, then refresh
Pack-Manager.`" epics.md 1086: greedy casks "enter a plan **only through explicit
opt-in with visible disclosure**".

**The incompatibility.** Two authorities over one eligibility predicate. A cask
with `includeGreedyCasks: true` in the draft but excluded by the Setting is both
a legal member of `PlanIntent` and an item that "can never enter a `PlanIntent`"
(spine 348–349). Whichever wins, the other story's acceptance criterion fails —
and Story 3.4, the settings-validation story, enumerates the configurable set
without it (epics.md 1102: "the retained editable stall threshold, hard cap, and
log level plus `skipUpgradePlanConfirmation`"), so on the Settings reading the
control has no validation owner at all.

**Missing AD.** Name the owner under AD-16: *"Greedy-cask opt-in is a per-intent
option on `PlanIntent` and is not a persisted Setting. Ineligibility copy for
greedy casks names the plan option, not Settings."* (Or the converse — but one of
the two, and the reason string at epics.md 479 must be corrected to match.)

### D-13 — AD-17's `[ASSUMPTION]` marker voids AD-17's own durability rule

**Unit A — UX-PB.1a** implements the rule: a durable draft file under
atomic-replace, which the Structural Seed reserves a slot for (spine 468:
"`<draft>                # canonical PlanIntent; atomic replace, fail-to-empty
(AD-17)`").

**Unit B — UX-PB.1b** takes the escape the same rule offers. Spine 368–369:
"`[ASSUMPTION]` Durable persistence is the reading taken from UX-PB.1b, **which
also permits an always-empty-on-relaunch fallback**." Its own criterion supplies
an unconditional out — epics.md 439: "**or — if it cannot be recovered** — the
sidecar returns to empty with no fabricated membership".

**The incompatibility.** Unit A writes a file Unit B never reads. The user-facing
promise ("your plan survives a crash") is delivered by neither story and owned by
neither. A rule that names its own negation as permitted is not an invariant, and
this is the one AD in the document that does so.

**Missing AD.** Resolve the assumption before the gate: either strike the fallback
clause and keep durability binding, or strike the durability rule and delete the
`<draft>` row from the Structural Seed. Leaving both is the only outcome that
guarantees divergence.

---

## Low

### D-14 — The one-active-attempt gate has two owners: a disabled affordance vs. a typed backend refusal

**Unit A — UX-PB.3a** gates in the frontend. epics.md 653: "the new draft stays in
the Upgrade Plan and **cannot be confirmed** until the active attempt is
terminal".

**Unit B — UX-PB.5a** lets the action fire and reports the typed failure, which
is what AD-16 describes. Spine 262–263: "A second confirmation **fails closed with
a typed already-active result**." epics.md 911: "**When** admission fails
**Then** nothing executes, **the dialog explains why**, and the plan remains
editable for re-review."

**The incompatibility.** Unit A's `Confirm N Updates` is inert; Unit B's opens a
dialog and explains a refusal. UX-PB.5d requires every safety action to keep "an
accessible name and a reachable focus order" (epics.md 976) — an inert control
must then follow UX-PB.1d's `aria-disabled` discipline (epics.md 484), which no
one has assigned to the confirm action. UX-PB.5c's `Run N Updates` is addressed by
neither. Severity is low because both converge on one attempt; the divergence is
user-visible behavior and accessibility ownership, not safety.

**Missing AD.** One line under AD-16: *"The backend typed already-active refusal is
the gate. Frontend surfaces reflect it — the confirm and run actions remain
focusable and named, announce the active-attempt reason, and are never natively
disabled."*

---

## Constructed and dropped

Pairs that looked incompatible but where an AD already forbids one unit. Listed
so the gate can see they were tested.

- **`planAttemptId` required vs. optional on the Operation wire model.** Dropped:
  AD-18 settles it. Spine 396–397: "Operation status, output and stall events,
  transcript metadata, and journal start/finish records carry `planAttemptId`
  **when one exists**" — the field is optional, so a required-field unit is
  non-compliant.
- **Ineligible item "dropped or flagged" (epics.md 409) as two divergent
  readings.** Dropped: AD-16 forbids the retained-in-intent reading. Spine
  348–349: an ineligible item "**can never enter a `PlanIntent`**", so "flagged"
  can only mean flagged in the projection while absent from intent.
- **A second live surface for the confirmed attempt.** Dropped: AD-17 forbids it
  twice — spine 372 "Exactly one instance exists" and spine 374–375 "rather than
  opening a second surface".
- **The application update leaking into plan History.** Dropped: AD-16's last rule
  is exhaustive. Spine 283–285: "It never enters a `PlanIntent`, draft, confirmed
  attempt, Results, or plan-attempt History".
- **A draft rebuild racing a verification refresh on a mixed-time snapshot.**
  Dropped: AD-4's coordinator-first rule serializes them. Spine 177–180: readers
  of plan-relevant state "must already hold `state.plan_coordinator`", which
  "yields... one canonical epoch". (The *freshness* gap that survives is D-2, a
  different defect.)
- **A test-only path or fixture becoming load-bearing for attempt evidence.**
  Dropped: AD-1 and AD-2 forbid it explicitly (spine 111–114, 127–130).

---

## Cross-cutting observation

Nine of the fourteen findings are the same shape: **the spine fixes an entity's
owner and shape but not its custody transitions.** Draft → attempt (D-3), attempt
→ record (D-5), record → History row (D-9), item → terminal state (D-4), intent →
rebuilt intent (D-1, D-7), capability → admission (D-6, D-11), disk → memory at
launch (D-8). AD-16 through AD-19 are strong nouns and weak verbs.

If only one thing is added before the gate, add a lifecycle AD — one that fixes,
for each durable entity, the ordered points at which it is created, handed off,
consumed, and destroyed, and which single unit performs each transition. Six of
the nine collapse under it.
