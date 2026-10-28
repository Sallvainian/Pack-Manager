# Reconciliation — `ARCHITECTURE-SPINE.md` (rev 9) against the 2026-07-25 PRD

**Reviewer:** `ARCHITECTURE-SPINE.md`
**Date:** 2026-07-25
**Subjects:** `_bmad-output/planning-artifacts/prds/prd-Pack-Manager-2026-07-25/prd.md`,
`.../addendum.md`, and
`_bmad-output/planning-artifacts/architecture/architecture-Pack-Manager-2026-07-23/ARCHITECTURE-SPINE.md`
(revision 9, 19 live ADs).

## Method and its limits

The PRD is upstream of the spine, so the spine does not constrain it. Only two
classes of disagreement are reported:

1. a PRD requirement that is **impossible or incoherent** under a live AD, and
2. a **spine invariant that exists to serve a product requirement the PRD never
   states** — because that invariant has no authority once the PRD is the
   requirements authority, and a `bmad-architecture` Update reconciling to the
   PRD could legitimately delete it.

Everything else — mechanism, naming, wire shape, story bindings — is the spine's
to own and is not diffed.

**Deliberately not reported**, per `docs/DECISIONS.md` D33 and D37 and the
project's scale: the absence of a readiness gate, coverage percentages,
scenario contracts, an evidence manifest, a `contracts/` directory,
keyboard-operability criteria, VoiceOver criteria, live-region announcement
criteria, and deterministic focus restoration. Their absence from the PRD is
correct. `planAttemptId`, `plan_attempt_id`, `Verifying`, `InteractionRequired`
and `skipUpgradePlanConfirmation` were confirmed at zero occurrences across
`src/` and `src-tauri/src/` (`grep -rn … | wc -l` → `0` for each), exactly as
`prd.md:24` states; a Planned FR is not reported as a defect.

## Verdict

The PRD and the spine agree on the load-bearing product content: the no-entry-
point-executes rule, canonical intent over display strings, one-use bounded plan
capability, atomic all-or-none admission, per-Manager failure containment with
merge-not-replace recovery, the focus mechanism, both architecture keys, and the
two release-blocking checks. Fourteen findings survive verification. Six are
high: one is a factual mis-statement of shipping status, and five are places
where a product-level rule the spine spells out has no PRD home — which means a
builder working from the PRD alone would build a system that violates a promise
the PRD itself makes (SM-2 most often).

The PRD's claim that spine row `:1050` is answered by §9.1 is **partly true**.
The question the row asks at the product level is closed. The row's stated
disposition — *"Closing this means writing a new invariant"* — is not satisfied
by §9.1, and the instruction to **retire** rather than answer it would drop live
residue. Detail in §2 below.

---

## 1. AD-by-AD result for the checks requested

### AD-16 / AD-17 / AD-21 / AD-22 / AD-23 / AD-24 vs FR-6, FR-7, FR-8

**Agreement.** `ARCHITECTURE-SPINE.md:363` — *"No entry point executes. A Package
row action, a Manager-header action, a Manager-wide action, and `Update
Everything` all mutate the draft and return."* — is stated by `prd.md:230`:
*"Neither a checkbox nor a row action executes anything."* The spine's retirement
of the live call site (`:366` *"The shipping `ManagerPane.upgradeRow` →
`executePlan` call site is retired by this rule"*) matches `prd.md:230`
verbatim in substance. `AD-17:548`'s *"Rust owns the canonical `PlanIntent`"* is
carried by `prd.md:248`: *"The draft stores canonical intent, never executable
display strings. Commands are rebuilt by the backend whenever the draft changes
and again before execution."* FR-8's capability semantics match AD-16's
`planId`/`planAttemptId` separation with no conflict.

**Divergences** — findings 1, 2, 4, 5, 7, 10, 11, 12 below.

### AD-25 vs FR-3, NFR-2

**Agreement, and the strongest of the set.** `AD-25:842` — *"A failure never
replaces a good snapshot with an empty one, and never with a partial overlay:
recovered-parse output **merges** into the inventory already parsed from the
successful outputs"* — is stated almost word-for-word at `prd.md:519`:
*"Recovered partial data merges into the existing inventory rather than replacing
it — replacing a Snapshot with an outdated-only overlay would make every
up-to-date Package vanish."* Containment (`AD-25:837`) is `prd.md:519`'s first
sentence. AD-25's verification-failure containment (`:849`–`:853`) is covered by
NFR-2's *"refresh … failures are contained"* limb, since a verification refresh
is a refresh — no gap reported there.

**Divergences** — findings 3 and 14 below.

### AD-27 vs FR-19

**Agreement.** `AD-27:907` — *"Focus is drawn as a real 2px `outline` in
`--color-focus-ring` with `outline-offset`, on every interactive element.
**`ring-*` is forbidden, and `outline-none` is never added to a focusable
element**"* — is FR-19's `prd.md:425`: *"drawn as a real `outline` — never a
`ring-*` box-shadow, which WebKit does not paint on native-appearance form
controls, and never `outline-none`. Focus is a dedicated indicator, never the
accent."* Verified in the tree: `git grep -c 'focus-visible' HEAD -- src/`
totals **31** and `outline-offset` totals **31**, matching `prd.md:434`'s
"31 sites". No conflict on the mechanism.

**Divergence** — finding 13 below (sampling claim), and finding 6 (contrast,
which sits in the same FR-19 note).

### AD-11 vs NFR-8

**Agreement, exactly.** `AD-11:293` — *"Two checks in `release.yml` block
publication: the detached updater signature is base64-decoded and verified with
`minisign` against the public key the shipping app embeds, and the published
`latest.json` is asserted reachable and coherent after upload."* — is `prd.md:557`:
*"the updater's detached signature is verified against the public key the shipping
app embeds, and the published update metadata is asserted reachable and coherent
after upload."* AD-11's universal-build / both-architecture-keys rule (`:298`–`:302`)
is FR-22 (`prd.md:482`); its macOS 15.0 floor (`:303`) is NFR-7 (`prd.md:551`);
its *"Release readiness is `docs/RELEASE-CHECKLIST.md` — a manual checklist, not a
computed verdict"* (`:290`) is `prd.md:30`. Nothing in AD-11 requires a product
behavior the PRD omits, and nothing in NFR-8 is impossible under AD-11.

**Divergence** — finding 6, which is AD-11's *accessibility* rule (`:328`), not
its release rule.

---

## 2. Does §9.1 close the spine's OPEN row at `:1050`?

### What the row literally says

`ARCHITECTURE-SPINE.md:1050`, read literally:

> `| Transient selection has no owning invariant | **OPEN — new architecture, not
> this run's scope** | Surfaced by `reviews/review-divergence-v9.md` C-1 and judged
> real. No `AD` models the relationship between transient row selection and
> `PlanIntent` membership, and the two driving sources answer it oppositely:
> `docs/SPEC.md` F5 has Esc "clears the transient selection", while
> `EXPERIENCE.md` has selection "immediately adds/removes Upgrade Plan
> membership". `src/store/packages.ts` ships a live `selection` set that
> `PlanIntent` cannot represent. Under the `EXPERIENCE.md` reading, Esc would
> mass-write AD-23 tombstones; under the `SPEC.md` reading it writes nothing.
> Story 3.5 (keyboard selection) and UX-PB.1a (staging) can each obey every
> existing `AD` and still build opposite models. Closing this means writing a new
> invariant, which is new architecture rather than the reconciliation this run was
> authorized for — so it goes to the owner as a decision. |`

The row contains four separable things: (a) which of the two sources wins,
(b) what Esc does under the winning reading, (c) that `src/store/packages.ts`'s
live `selection` set is unrepresentable in `PlanIntent`, and (d) that closing it
requires **writing a new invariant**.

### What §9.1 closes

`prd.md:639` closes (a) unambiguously: *"Q1 — Does a Package checkbox mutate
Upgrade Plan membership directly, or is there a separate transient selection?
CLOSED 2026-07-25: directly."* It closes (c) by disposition — `prd.md:643`: *"The
live `selection` set in `src/store/packages.ts` is pre-D27 code, not a competing
decision."* Verified live: `src/store/packages.ts:17` declares
`selection: Partial<Record<ManagerId, Set<string>>>;`, so the row's premise was
accurate and the PRD's disposition of it is correct.

### The residue

**(b) is left open, and §9.1 picks the branch that makes it live.** The row states
that *"Under the `EXPERIENCE.md` reading, Esc would mass-write AD-23 tombstones"*
— and §9.1 adopts precisely the `EXPERIENCE.md` reading. The PRD mentions Esc once,
at `prd.md:236`, only to name the stale side: *"this resolves the conflict between
`docs/SPEC.md` F5 (Esc clears a transient selection) and `EXPERIENCE.md` (selection
immediately changes membership) in favor of `EXPERIENCE.md`."* It never says what a
bulk clear now means. `grep -ci 'tombstone' prd.md` returns **0**, so the concept
the row says would be mass-written does not exist in the requirements authority at
all.

**(d) is not satisfied, and the substitute invariant answers a different
question.** `addendum.md:53` asserts: *"Closing this writes the new invariant the
row was waiting for: membership mutation is batch-capable and Rust-authored."*
Batch-capability is a throughput requirement — `prd.md:231` grounds it in NFR-3
round-trips, not in the selection/membership relationship. The row asked for an
invariant modeling *the relationship between transient row selection and
`PlanIntent` membership*. Batch mutation is orthogonal to that: a per-row model and
a batch model can both be built with or without a distinct selection state.

**A concrete, still-live instance of the row's exact failure mode.** `prd.md:231`
introduces a first-class interaction the spine's domain minimum cannot express:

> *"A **range** or filter-wide interaction submits one membership operation
> covering every affected Package identity, not one per row."*

`ARCHITECTURE-SPINE.md:464` defines the only provenance vocabulary available:

> `origin: Explicit | Bulk { scope: Manager(ManagerId) | FilteredView | Everything }`

A shift-range across a subset of rows is none of the three scopes, and
`AD-23:766` defines the alternative narrowly — *"`Explicit` when the user staged
that exact item"*. The live store already ships the range op:
`src/store/packages.ts:5` — *"The store keeps *primitive* selection ops (toggle,
range, set, clear)."* So UX-PB.1a and Story 3.5 can still build opposite models
(range → `Explicit` members vs range → a fourth `Bulk` scope), which is verbatim
the condition the row exists to prevent. Retiring the row without recording this
loses it.

**Answer:** §9.1 closes the row's product question. It does not close the row.
The correct disposition is **close-with-successor**, not retire: the successor
row (or the AD-23 amendment) must state whether a range interaction produces
`Explicit` members or a new `Bulk { scope }` value, and what a bulk clear does to
tombstones. See findings 2 and 10.

---

## 3. Findings

Severity ranks impact on the PRD's usefulness as the requirements authority.

### F1 — HIGH — Draft durability is decided in the spine and absent from the PRD, and the PRD's word for it points the other way

`ARCHITECTURE-SPINE.md:552`:

> *"The draft is session-scoped and is never written to disk. Every relaunch —
> after a clean quit, a crash, or a force-quit — starts with an empty draft and a
> hidden sidecar."*

`ARCHITECTURE-SPINE.md:1039` records it as a decision, not a derivation:
*"Draft durability | **RESOLVED** | Fail-to-empty."* `AD-17:561` is explicit that
it is a product-cost decision: *"the cost of that choice is a lost draft after a
crash, and it was accepted deliberately."*

The PRD never states it. Its only durability statement is scoped to navigation —
`prd.md:229`: *"The draft persists while the user navigates between Managers and
the Dashboard"* — and its scope summary calls the draft *"One **persistent**
editable Upgrade Plan draft, replacing transient dialog state"* (`prd.md:589`),
which a reader with no spine access would reasonably implement as disk-backed.
`prd.md:630` shows the PRD does track cross-relaunch persistence questions when it
sees them (open question 2, for a downloaded application update), so the omission
reads as an oversight rather than a deferral, and `prd.md:654` claims *"Where a
source was silent, §9 records the gap rather than filling it."*

**Fix:** add a consequence to FR-6 — the draft is session-scoped; every relaunch,
including after a crash or force-quit, starts empty; membership is never
reconstructed or partially restored — and change `prd.md:589`'s "persistent" to
"session-persistent" or equivalent.

### F2 — HIGH — §9.1 instructs retirement of spine `:1050` but leaves the range-provenance residue live

Grounded in full in §2 above. Key pairing: `prd.md:231` makes *"A **range** or
filter-wide interaction"* a required membership operation;
`ARCHITECTURE-SPINE.md:464` offers only
`Explicit | Bulk { scope: Manager(ManagerId) | FilteredView | Everything }`;
`prd.md:648` instructs *"that row should be **retired** in the
`bmad-architecture` Update that follows this PRD"*, and `addendum.md:53` hardens
it to *"must be **retired**, not re-answered."*

**Fix:** change the instruction from *retire* to *close with successor*, and state
in FR-6 which provenance a range interaction produces, plus what a bulk clear does
to previously staged members. This is one sentence in the PRD and it prevents the
two stories the row names from building opposite models.

### F3 — HIGH — FR-3's unqualified refresh-coalescing consequence defeats the D29/D30 verification it must support

`prd.md:172`, FR-3, tagged Shipping, with no exception:

> *"Duplicate refresh requests for the same Manager coalesce rather than queueing
> twice."*

`ARCHITECTURE-SPINE.md:406`:

> *"A verification refresh must be a fresh acquisition whose data collection begins
> strictly after the mutating process exited. Verification refreshes are exempt
> from AD-4's duplicate-refresh coalescing against any refresh already in flight at
> that instant; a coalesced refresh satisfies verification only if it started after
> it. A snapshot taken before the mutation can neither confirm nor refute it."*

FR-13's Planned limb introduces the dependent state — `prd.md:336`: *"a `Verifying`
state before success is declared"* — without carrying the exemption. Built to the
PRD literally, a `Verifying` attempt can be satisfied by a refresh that started
before the upgrade ran, and the product declares success from pre-mutation data.
That is a direct violation of the PRD's own FR-2 truth rule and of SM-4.

**Fix:** qualify FR-3's coalescing consequence — verification refreshes are exempt;
a coalesced refresh satisfies verification only if its data collection began after
the mutating process exited.

### F4 — HIGH — FR-8's rejection enumeration omits the single-active-attempt refusal

`prd.md:261`, FR-8, enumerating what rejects a submission:

> *"An in-progress state update, revision drift, an active refresh, or a lock-set
> overlap with any pending or running Upgrade, SelfUpdate, or HealthFix rejects the
> submission without enqueueing."*

`ARCHITECTURE-SPINE.md:394`:

> *"Exactly one confirmed attempt may be active. A second confirmation fails closed
> with a typed already-active result."*

Two confirmed plans over **disjoint** Managers have no lock-set overlap, so FR-8
as written admits the second one. The constraint appears in the PRD exactly once,
in a scope bullet — `prd.md:593`: *"One active attempt at a time"* — and never as a
requirement; `grep -n 'active attempt' prd.md` returns that line only. Scope
bullets are not what `bmad-create-epics-and-stories` turns into acceptance
criteria.

**Fix:** add "or an already-active confirmed Plan Attempt" to FR-8's rejection
list, with the typed already-active result as the outcome.

### F5 — HIGH — The rule that makes the confirmation-off path safe exists only in the spine

`ARCHITECTURE-SPINE.md:374`:

> *"A canonical rebuild may remove or invalidate membership; it may never add a
> member the user has not seen. A bulk mutation freezes its expansion into concrete
> members at the moment it is made — the scope predicate never runs a second time,
> and newly eligible work discovered later surfaces as an explicit offer to
> re-seed, never as silent membership. If a rebuild would enlarge membership, the
> preview `planId` expires and re-review is required. **This holds identically on
> the confirmation-off path, which otherwise has no moment at which the user could
> see the addition.**"*

The PRD requires the rebuild (`prd.md:248`) and preserves it under the opt-out
(`prd.md:250`: *"The preference removes only the final dialog — never the draft
review, the backend rebuild, or the stale-plan check"*) but never says the rebuild
may not grow membership. Its headline metric depends on that rule —
`prd.md:612`: *"**SM-2: Zero unreviewed mutations.** No Package or Manager update
ever runs that the user did not see staged first. A single violation is a P0
defect, not a metric miss."* With `skipUpgradePlanConfirmation` on and a
filter-wide staging action, a rebuild that re-evaluates the scope predicate runs
work the user never saw, and every PRD sentence is still satisfied.

**Fix:** add to FR-7 — a canonical rebuild may narrow or invalidate membership and
may never add a member the user has not seen; bulk expansion freezes at the moment
it is made; an enlarging rebuild forces re-review even when the confirmation
dialog is off.

### F6 — HIGH — The PRD claims a shipping contrast guard that does not exist at the HEAD it says it verified against

`prd.md:583`, §7.1 "Shipping today (1.0.1)":

> *"the dark-only Aurora Control Deck interface with a CI-asserted focus mechanism
> and contrast guard"*

`prd.md:423`, FR-19 (Status: Shipping): *"Text contrast meets at least 4.5:1 on its
surface."* NFR-6 (`prd.md:541`, Status: Shipping) restates it. `prd.md:654` grounds
the document: *"verification against `src/` and `src-tauri/` at `HEAD` `5972109`."*

`ARCHITECTURE-SPINE.md:328` says the opposite:

> *"**Automated 4.5:1 text contrast does not exist**; that same spec disclaims it —
> "It does not claim measured contrast compliance or validate the native Tauri
> package." Contrast is therefore an obligation on whichever story adds it"*

The spine is right at HEAD. `git rev-parse --short HEAD` → `5972109`.
`git grep -n 'text-white' HEAD -- src/` returns three sites:
`src/components/primitives/Button.tsx:7`, `:11`, and
`src/components/shell/UpdateStatusItem.tsx:63`. `docs/DECISIONS.md:497` measures
them: *"white on `--color-accent #65A7FF` is **2.46:1**, on `--color-accent-hover
#7DB3FF` **2.15:1**, on `--color-danger #FF8793` **2.30:1**. All three fail the
4.5:1 floor and also fail 3:1."* `git show HEAD:tests/e2e/browser-style-contract.spec.ts
| grep -nE 'contrast|4\.5'` returns only the disclaimer at line 129 — no assertion.
`git show HEAD:docs/DECISIONS.md | grep -nE '^## D3[5-7]'` returns D35 only: **D36
and D37 are not committed either.**

The fix exists in the working tree (`git diff --stat` shows +98 lines in the style
contract and the three `text-white` sites converted), so this is a status
statement running ahead of the tree, not a wrong requirement. It matters because
§7.1 is what a reader consults to decide what needs building, and because AD-1
forbids scheduling work the shipping code already covers — a false "already
shipping" is the same error in the other direction.

**Fix:** either land D36's change and its decision record before dating the PRD, or
qualify `prd.md:583` and tag the 4.5:1 limb of FR-19/NFR-6 as Partial with the
guard named as uncommitted. Note also that D37's third retained item
(`prd.md:436`, *"**Contrast** (D36's guard)"*) presently cites an uncommitted
guard.

### F7 — HIGH — `addendum.md` §3 under-scopes the spine reconciliation, leaving a live AD that mandates what the PRD deleted

`addendum.md:53` scopes the `bmad-architecture` Update as:

> *"3 keyboard/VoiceOver mentions, including the manual pass referenced at lines
> 332 and 941."*

`grep -ci 'voiceover' ARCHITECTURE-SPINE.md` → **3**, so the count is right, but
the two lines named (`:332`, `:941`) are both release-checklist mentions. The third
is not a mention — it is an entire Rule under AD-17, `ARCHITECTURE-SPINE.md:610`:

> *"There is exactly one status-announcement channel for plan and attempt progress,
> owned alongside the sidecar region. It announces at **polite** priority by default
> and **assertive** only for an immediate safety action … Stories announce through
> it; none adds a second live region for the same information … Two live regions
> narrating one attempt is a defect, not additive coverage."*

`prd.md:430` deletes exactly that obligation: *"Specifically dropped: … live-region
announcements of plan progress/verification/cancellation/failure/completion."*
Also unlisted: `ARCHITECTURE-SPINE.md:526`'s *"its control is non-interactive to
pointer and keyboard, it carries a stated reason for assistive technology"* under
AD-16's domain rules, against `prd.md:438`'s *"Pointer-facing explanations … also
stay; only their keyboard and screen-reader limbs are out of scope."*

An Update run scoped by "3 mentions at lines 332 and 941" leaves AD-17 requiring a
live-region channel the PRD removed, binding UX-PB.1a–1e and Stories 3.1/3.2/3.5.

**Fix:** rewrite the addendum's spine row to name the artifacts by `AD` id and rule
subject rather than by mention count and line number — AD-17's announcement-channel
Rule, AD-16's assistive-technology clause in the ineligible-item rule, AD-11's
checklist sentence, AD-27's closing sentence — and add F6's AD-11 contrast rule to
the same queue. `ARCHITECTURE-SPINE.md:1049` already records that line-number
citations into this spine *"have already drifted — the same positional-reference
failure this run folder has now hit three times"*.

### F8 — MEDIUM — FR-7 says nothing about when the safety-reducing opt-out commits

`prd.md:250` states only that the checkbox *"appears **only** in that dialog and
persists `skipUpgradePlanConfirmation`."* `ARCHITECTURE-SPINE.md:743` fixes the
ordering and the failure semantics as a product rule:

> *"Ordering is fixed — validate, admit through the scheduler's revision-checked
> transaction, then persist the rider once the admission has returned. A rider never
> precedes the admission it rides on."*

and `:749`:

> *"A rider that **reduces** a safety default commits only if the action it rode on
> succeeded. On rejected admission nothing is persisted and nothing becomes active,
> and the dialog retains the user's selection … The asymmetry is the point: an
> unsaved opt-out costs one extra confirmation, while a saved opt-out on a refused
> run removes the gate from a run the user never got."*

That last sentence is a requirement, not a mechanism: it describes when the
product's confirmation gate may be disarmed. NFR-1 (`prd.md:513`) covers work
never running, not preference state. As the PRD stands, an implementation that
persists the opt-out on a rejected admission satisfies every sentence in it.

**Fix:** one consequence under FR-7 — the opt-out is persisted only after the
admission it accompanied succeeded; a rejected admission persists nothing, changes
nothing active, and retains the user's selection in the dialog.

### F9 — MEDIUM — FR-18's export enumeration is closed and excludes the plan-attempt journal AD-18 requires

`prd.md:408`: *"It contains an environment and detection report, the newest three
application logs, the newest 25 transcripts, and the History journal."* Singular,
and the FR is tagged Shipping with no Planned limb.

`ARCHITECTURE-SPINE.md:633`: *"Diagnostics export carries **both journals** as
distinct entries alongside `report.json`, the newest three app logs, and the newest
25 transcripts."*

Under D29 the plan-attempt journal is a second file (`ARCHITECTURE-SPINE.md:998`
shows `<plan-attempts>.jsonl` in the structural seed). A builder honoring FR-18's
enumeration literally would omit it, and the export would then fail SM-4's
*"transcript plus History answer 'what ran and what happened'"* (`prd.md:617`) for
exactly the attempts D29 introduces.

**Fix:** add a `Planned — D29` limb to FR-18: the export carries the plan-attempt
journal as a distinct entry, under the same allowlist and retention bounds.

### F10 — MEDIUM — FR-6's "individually removable" does not carry the removal-durability guarantee AD-23 exists to provide

`prd.md:229`: *"every staged item is individually removable from the Upgrade
Plan."* `ARCHITECTURE-SPINE.md:776` supplies what that must mean once a header
checkbox exists:

> *"Removal writes a tombstone on the intent. A later bulk expansion of any scope
> does not re-add a tombstoned ref — a member list can record presence but not a
> deliberate absence, so the "stays removed" guarantee needs this home."*

`grep -ci 'tombstone' prd.md` → **0**; `grep -ci 'provenance' prd.md` → **0**. The
user-visible behavior — remove one Package from a 40-item plan, then re-hit the
header checkbox, and the removed item does or does not come back — is a product
decision with no PRD statement, while FR-6 (`prd.md:226`) explicitly requires the
header checkbox to *"add or remove every eligible Package matching the active
filter."*

**Fix:** state the outcome in FR-6 (not the mechanism): an item the user removed
stays removed against any later bulk action in the same draft, until the user
stages it again explicitly.

### F11 — MEDIUM — What Retry actually retries is unstated

`prd.md:364`, FR-15 Planned: *"Retry creates a **new linked attempt** and never
overwrites the first failure."* It does not say what the new attempt contains.
`ARCHITECTURE-SPINE.md:807` does:

> *"A retry scope is a **derived intent** — composed in Rust from the failed
> attempt's reviewed intent **restricted to its failed members**, canonically
> rebuilt against current eligibility and argv."*

and `:801`: *"**A confirmed retry does not empty the draft**."* Built from the PRD
alone, Retry re-running the whole original plan — including its succeeded members —
is fully compliant, and would re-run successful upgrades. `AD-24:826` also defines
the all-ineligible case, which the PRD does not.

**Fix:** state in FR-15 that Retry's scope is the failed members of the source
attempt, re-derived against current eligibility, and that it leaves any separately
staged draft untouched.

### F12 — MEDIUM — FR-17's revision sentence presupposes what AD-21 exists to deny

`prd.md:393`: *"A patch is persisted **before** in-memory settings change and before
the canonical state revision advances. A failed save changes neither."*

`ARCHITECTURE-SPINE.md:705`:

> *"A persisted key is plan-determining unless it is explicitly marked plan-inert
> with a stated reason … `skipUpgradePlanConfirmation` is **plan-inert** — it selects
> whether a modal renders and cannot reach membership, exclusions, or argv."*

`ARCHITECTURE-SPINE.md:1041` records why this is not academic: *"Verified as a
shipping defect, not just a paper one: `set_settings_core` bumps for every key and
the execute path rejects on `issued.revision != coordinator.revision()`."* Combined
with FR-8's *"revision drift … rejects the submission"* (`prd.md:261`), the PRD as
written makes the D28 opt-out — written in the same click as the confirmation —
deterministically invalidate the plan it rides on. FR-17 is tagged Shipping and
records the defect as if it were correct behavior.

**Fix:** qualify FR-17 — only a setting that can change plan membership,
exclusions, or the argv of the operations a plan would run invalidates a reviewed
plan; the confirmation opt-out does not.

### F13 — LOW — FR-19's note reads the style contract as population coverage, which AD-27 refuses by name

`prd.md:434`: *"It ships across 31 sites, is asserted in CI by
`tests/e2e/browser-style-contract.spec.ts`, and is governed by D35 and AD-27."*

`ARCHITECTURE-SPINE.md:929`:

> *"The style contract proves the mechanism on **named samples, not a sweep**: today
> a toolbar `<button>` and the package-row plan-membership checkbox … It does not
> enumerate every interactive element and it does not measure contrast. **No story
> may read a green run as proof that the element *it* added has a visible focus
> state**."*

Confirmed against the committed spec: it focuses `refreshAll` (line 79) and one
package-row checkbox (line 204) — two samples, not 31. `AD-27:926` records the cost
of the other reading: *"the first count was wrong by a factor of three — three
reported, **nine** actual."* The PRD sentence is literally true and invites the
false inference.

**Fix:** trim to "ships across 31 sites, with the mechanism pinned in CI on two
named samples."

### F14 — LOW — FR-3's Last-good Snapshot consequence is weaker than the presentation AD-25 requires

`prd.md:171`: *"A failed refresh retains the Last-good Snapshot, keeps it browsable,
and labels it stale."* `ARCHITECTURE-SPINE.md:840`: *"A Manager that has ever
produced a successful snapshot retains it on failure, **labeled with its own
timestamp and the exact failure, with a Retry affordance**"*, and `:854`: *"Health
and staleness presentation derive from the snapshot's real timestamp … no invented
or interpolated value is ever substituted."*

The timestamp, the exact failure text, and the Retry affordance are user-facing
outcomes that FR-3 compresses into "labels it stale". FR-16 (`prd.md:375`) supplies
the error-legibility half generically but not the Retry affordance or the
real-timestamp rule.

**Fix:** extend FR-3's consequence to name the snapshot's own timestamp, the exact
failure, and a Retry affordance, and to forbid an interpolated staleness value.

---

## 4. Checked and found clean

Recorded so a later pass does not re-derive them:

- **AD-16 plan/attempt identity separation** (`:382`, `:952`) against FR-8's
  capability rules (`prd.md:259`–`:263`) — no conflict; the PRD's *"at most 64
  unconsumed capabilities are retained per session, oldest evicted first"* is
  compatible with AD-16's eviction-fails-closed rule.
- **AD-16 application-update exclusion** (`:440`) against PRD §4.5 (`prd.md:446`):
  *"Application updates sit outside the Operation queue entirely — they are not
  Operations, hold no Manager lock, and never appear in History."* Verbatim
  agreement.
- **AD-4's raw-path-before-canonicalization rule** (`:255`) against FR-4
  (`prd.md:131`) — agreement, and the PRD correctly keeps the algorithm in the
  addendum rather than the FR.
- **AD-4's fidelity floor** (`:260`) against FR-15 (`prd.md:362`) — both state the
  closed unterminated-notice list with no general rewriting.
- **AD-20's webview trust boundary** against NFR-5 (`prd.md:537`) — the PRD's *"Any
  external-content, capability, or permission change is security-sensitive"* is the
  requirement AD-20 implements.
- **AD-12, AD-26, AD-2, AD-5, AD-19** — no PRD requirement is impossible under any
  of them, and none carries an unstated product requirement material at this
  altitude. (AD-19's journal-corruption stance at `:659` is the closest call; NFR-2's
  *"persistence failures are contained"* covers it thinly but does cover it.)
- **`prd.md:24`'s zero-occurrence claim** — verified: `planAttemptId`,
  `plan_attempt_id`, `Verifying`, `InteractionRequired`, `skipUpgradePlanConfirmation`
  each return 0 across `src/` and `src-tauri/src/`.
- **`prd.md:434`'s "31 sites"** — verified: 31 `focus-visible` and 31
  `outline-offset` occurrences under `src/` at HEAD.
