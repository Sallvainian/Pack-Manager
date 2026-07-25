# Divergence review — ARCHITECTURE-SPINE.md revision 8

**Lens (verbatim from the reviewer config):** "Attack the spine as an adversary:
construct two units one level down that each obey every AD to the letter yet still
build incompatibly — clashing shared-data shapes, two owners of one entity,
conflicting state-mutation paths. Every pair you find is a hole to close with a new
or tightened AD."

**Target:** `_bmad-output/planning-artifacts/architecture/architecture-Pack-Manager-2026-07-23/ARCHITECTURE-SPINE.md`
(revision 8, 877 lines, `wc -l` on 2026-07-25).
**Level below:** `_bmad-output/planning-artifacts/epics.md` (1,293 lines) — Epic UX-PB
(UX-PB.1a–UX-PB.5e) plus Stories 2.2, 3.1, 3.2, 3.4, 3.5, 6.5.

**Scope respected:** AD-6..AD-10 and AD-13..AD-15 are retired ids and are not
discussed. `epics.md` is not edited this run. Divergences already itemised in the
Decision Status `epics.md divergence batch for bmad-correct-course` Open row
(`ARCHITECTURE-SPINE.md:876`, items (a)–(e)) are **not** re-reported; where a finding
below sits adjacent to one of those items, the section states explicitly what the row
does not cover.

**Counts:** 4 CRITICAL · 4 HIGH · 4 MEDIUM · 2 LOW (14 total).

**Headline verdicts on the three CRITICALs revision 8 claims to close:**

| Prior finding | Claimed closer | Verdict |
| --- | --- | --- |
| C-1 (settings write vs. revision drift) | AD-21 + AD-22 | **PARTIALLY CLOSED.** AD-21 genuinely closes the revision-drift mechanism. AD-22 does not close the sequencing half: it is **unbuildable** against the shipping admission path (V-2) and it **states the opposite ordering** from its own bound story (V-3). AD-21 also opens a new hole of its own (V-1). |
| C-2 (`PlanIntent.kind` scalar) | AD-23 | **CLOSED as to shape; NOT CLOSED as to consequence.** The scalar is genuinely gone and per-member `origin` is a coherent wire shape. But the tombstone set AD-23 introduces has no defined lifetime (V-4) and the `scope` enum does not cover a reachable staging path (V-5). |
| C-3 (Retry vs. the accumulating draft) | AD-24 | **CLOSED as to authorship; NOT CLOSED as to rendering.** "One author" is airtight. AD-17's amended four-way precedence does not cover every reachable combination, and two of the uncovered ones contradict bound story criteria verbatim (V-6). |

---

## CRITICAL

### V-1. AD-21's plan-determining test excludes the command timeout, so Story 3.4 can legally invalidate the exact snapshot UX-PB.2c persists

**Pair:** Story 3.4 ↔ UX-PB.2c (and UX-PB.5c).

AD-21 defines the closed set by a three-part test, `ARCHITECTURE-SPINE.md:622-627`:

> "The canonical revision AD-16 tests for drift advances only on a change to a
> **plan-determining input**: the closed set of state whose change can alter a
> preview's membership, its exclusions, or the argv Pack-Manager would construct."

and `ARCHITECTURE-SPINE.md:628-633`:

> "A persisted key is plan-determining unless it is explicitly marked plan-inert
> with a stated reason. … `skipUpgradePlanConfirmation` is plan-inert — it selects
> whether a modal renders and cannot reach membership, exclusions, or argv."

The stated reason template is therefore "cannot reach membership, exclusions, or
argv". Two settings keys satisfy that template exactly and are still load-bearing on
what runs. `src-tauri/src/queue.rs:98-104`:

```
            let timeout = match p.timeout {
                CmdTimeout::Absolute(d) => CmdTimeout::Absolute(d),
                CmdTimeout::Stall { .. } => CmdTimeout::Stall {
                    silence: Duration::from_secs(settings.stall_after_secs),
                    hard_cap: Duration::from_secs(settings.upgrade_hard_cap_mins * 60),
                },
            };
```

`stall_after_secs` and `upgrade_hard_cap_mins` (`src-tauri/src/settings.rs:31-32`)
reach `CommandSpec.timeout`, never `CommandSpec.args`. They alter no membership, no
exclusion, and no argv. Under AD-21's own literal test they are plan-inert, and
Story 3.4 — which owns them, `epics.md:1227` names "the retained editable stall
threshold, hard cap, and log level plus `skipUpgradePlanConfirmation` (default
`false`) as the configurable Settings" — will declare them so with the same
one-sentence reason AD-21 models.

Now the other side. UX-PB.2c, `epics.md:685`:

> "the append-only record stores the reviewed Manager/Package scope, Manager
> self-update identities, **exact command snapshot**, version evidence, timestamps,
> and result/verification state as immutable plan-admission metadata"

and UX-PB.5c, `epics.md:1080`:

> "Rust rebuilds the exact commands from canonical intent and runs the stale-plan
> check before the plan is atomically admitted"

AD-16 backs both, `ARCHITECTURE-SPINE.md:349-351`: "Execution must match the issued
preview and a fresh coherent rebuild, and is rejected on in-progress state change,
revision drift, an active refresh, or a lock-set overlap".

**Concrete incompatible artifacts.** The user reviews a plan whose bound commands
carry `timeout: Stall { silence: 120s, hard_cap: 1800s }`. Before confirming, the
user opens Settings and moves the hard cap from 30 to 5 minutes. Story 3.4's
classification table says plan-inert → no revision bump → AD-16's drift test passes →
admission succeeds. UX-PB.2c writes the durable record with the reviewed snapshot; the
attempt executes with a 1,800s cap in the record and a 300s cap in the process. When
UX-PB.3d reports the outcome as `timed out` (`epics.md:832` — "the overall outcome is
exactly one of success, partial, failed, cancelled, timed out, or interrupted"), the
immutable History replay UX-PB.4b reconstructs (`epics.md:939`, "exact commands,
Operation outcomes, errors, timings") shows a 30-minute cap that never applied. AD-16's
"Execution must match the issued preview" is false, by construction, with every AD
obeyed.

**This is a regression revision 7 did not have.** The shipping call site bumps for
every key — `src-tauri/src/commands.rs:649`, `coordinator.bump_revision();`,
unconditional — which closes this window today. AD-21 explicitly directs it open:
`ARCHITECTURE-SPINE.md:638-641`, "The shipping call site bumps unconditionally for
every key … **Narrowing it is product work** owned by whichever of UX-PB.5b or
Story 3.4 lands first".

A second key demonstrates the test is not even single-valued.
`run_brew_update_on_refresh` (`src-tauri/src/settings.rs:29`) changes refresh argv but
not upgrade-plan argv. AD-21 says "the argv Pack-Manager would construct" without
qualifying *which* argv. Story 2.2, which owns refresh phases (`epics.md:1151-1154`,
"the enabled path shows the required update/inventory/outdated phase order **And** the
disabled path omits only the metadata-update phase"), reads that as argv and classifies
plan-determining. Story 3.4 reads it as preview argv and classifies plan-inert. One
key, two definition-site declarations, two revision behaviours.

**Hole to close.** AD-21's set must be defined over the *reviewed artifact* — every
field of the `CommandSpec` and `UpgradePlan` the user was shown, timeout included —
not over the three-part membership/exclusions/argv test. And "argv" needs a scope
qualifier.

---

### V-2. AD-22's unbroken critical section is unbuildable: admission happens in the scheduler task, behind an `await`, under the same non-reentrant mutex

**Pair:** UX-PB.5b ↔ UX-PB.2b.

AD-22 rule 1, `ARCHITECTURE-SPINE.md:649-652`:

> "A confirming action holds `state.plan_coordinator` once, unbroken, across
> validation, admission, and any side effect it carries. **No release-and-reacquire:**
> that window is precisely what admits an interleaved writer and re-opens the drift
> AD-21 just closed."

Three facts from the tree make that unsatisfiable.

**(a) The lock is a `std::sync::Mutex`.** `src-tauri/src/state.rs:8`,
`use std::sync::{Arc, Mutex, RwLock};`; `src-tauri/src/state.rs:212`,
`pub plan_coordinator: Arc<Mutex<PlanCoordinator>>,`. It is not reentrant and its
guard is not `Send`.

**(b) Admission is an `await` across a channel to another task.**
`src-tauri/src/commands.rs:462-465`:

```
    let op_ids = state
        .queue
        .submit_plan_batch(subs, expected_revision)
        .await
```

`submit_plan_batch` (`src-tauri/src/queue.rs:814-828`) sends `Msg::SubmitPlanBatch`
and blocks on `rrx.await` (`src-tauri/src/queue.rs:827`). The shipping code documents why the guard is dropped first
— `src-tauri/src/commands.rs:351-353`:

> "Consumes and re-validates a backend-issued plan against one canonical revision,
> then asks the scheduler to atomically re-check that revision and enqueue the
> complete batch. **No synchronous guard crosses an await.**"

The guard is scoped to the block that ends at `src-tauri/src/commands.rs:460`,
`(subs, coordinator.revision())`. Holding it across line 465 does not compile.

**(c) The scheduler takes the same lock.** `src-tauri/src/queue.rs:1005-1011`:

```
    fn handle_plan_batch(
        &mut self,
        subs: Vec<OpSubmission>,
        expected_revision: u64,
        reply: oneshot::Sender<Result<Vec<String>, PlanBatchError>>,
    ) {
        let coordinator = self.deps.plan_coordinator.clone();
        let mut coordinator = coordinator.lock().expect("plan coordinator poisoned");
```

Even if (a) and (b) were waived, a confirming action still holding the guard would
deadlock against line 1011.

**Concrete incompatible artifacts.** UX-PB.2b's builder ("Atomic admission mints one
`planAttemptId`", `epics.md:665`, "`execute_plan` atomically returns one new durable
`planAttemptId` plus the created Operation identities") extends the shipping shape:
validate under the guard, drop it, `await` the scheduler, which re-checks the revision
and enqueues all-or-none under its own acquisition. That obeys AD-4's scheduler rule
verbatim, `ARCHITECTURE-SPINE.md:242-245`: "the single scheduler atomically checks and
acquires each operation's full lock set before start."

UX-PB.5b's builder obeys AD-22 and puts validation, admission, mint, and the rider
persist inside one guard. To do that it must move admission out of the scheduler task
and into the command handler — which deletes the single-scheduler property AD-4
requires and the all-or-none atomicity UX-PB.2b promises, or it must not compile.

There is no composition satisfying both. AD-22 as written also contradicts AD-4's own
architecture: the coordinator-first rule (`ARCHITECTURE-SPINE.md:223-226`) fixes lock
*order* for readers; AD-4's scheduler rule makes the scheduler the *only* admitter.
AD-22 asks the confirming action to be both reader and admitter under one hold.

**Secondary hazard inside the same rule.** AD-22 rule 2,
`ARCHITECTURE-SPINE.md:653-656`, "validate, admit and mint `planAttemptId`, then
persist the rider", puts a settings write inside the guard. The shipping persist path
takes the same lock (`src-tauri/src/commands.rs:636-640`) — a self-deadlock for any
builder that reuses it. A builder that inlines a lock-free variant instead performs
`create_dir_all`, `File::create`, `write_all`, **`sync_all()`**, and `rename`
(`src-tauri/src/settings.rs:128-146`) — an fsync — under the global mutex that AD-4
requires every reader of `detection`, `registry`, `queue.records()`, `settings`, and
`tool_env` to hold first. (The general lock-lifetime concern is `review-divergence-v6.md`
M-4; the deadlock and the architecture contradiction above are not.)

**Hole to close.** AD-22 must be restated over the *admission transaction* the
scheduler already provides — one revision-checked message — rather than over one
mutex hold. The invariant that survives is "no writer interleaves between validation
and admission", and the scheduler's `expected_revision` re-check
(`src-tauri/src/queue.rs:1011`) already enforces it.

---

### V-3. AD-22 orders the rider after admission; UX-PB.5b's shipped criterion orders it before — the Open row records this as a missing case, not a contradiction

**Pair:** UX-PB.5b ↔ UX-PB.2b.

AD-22 rule 2, `ARCHITECTURE-SPINE.md:653-656`:

> "Ordering inside that section is fixed — validate, admit and mint `planAttemptId`,
> **then persist the rider.** A rider never precedes the admission it rides on."

AD-22 rule 3, `ARCHITECTURE-SPINE.md:657-663`:

> "A rider that **reduces** a safety default commits only if the action it rode on
> succeeded. On rejected admission nothing is persisted and nothing becomes active …
> a saved opt-out on a refused run removes the gate from a run the user never got."

UX-PB.5b's live criterion, `epics.md:1048-1050`:

> "**Given** the dialog with `Disable upgrade plan command execution confirmation`
> selected
> **When** I choose the final `Confirm N Updates`
> **Then** `skipUpgradePlanConfirmation: true` is written atomically, **the new value
> takes effect only after persistence succeeds, and the plan is admitted.**"

The criterion's clause order is persist → activate → admit. AD-22's is admit → mint →
persist. These are not two readings of one sentence; they are opposite orderings of
the same two operations, and the observable difference is exactly the hazard AD-22
rule 3 names. A builder implementing `epics.md:1050` literally, on a rejected
admission, has already written `skipUpgradePlanConfirmation: true` to disk and made it
active — the safety gate is disarmed for a run that was refused. That is the precise
outcome AD-22 exists to forbid.

**What the Open row does not cover.** `ARCHITECTURE-SPINE.md:876` item (e) reads:

> "**UX-PB.5b** — its criteria cover the successful admission and the failed save but
> not a *rejected* admission carrying the opt-out rider; AD-22 decides it (persist
> nothing, keep the user's selection) and the criterion should say so."

That characterises the defect as a **gap** ("not a … rejected admission"). It is not a
gap. The criterion at `epics.md:1050` affirmatively specifies the wrong order for the
*successful* path too, and a builder following it produces the forbidden behaviour on
the rejected path as a direct consequence. The correct-course instruction "the
criterion should say so" would add a case to a criterion whose existing clause order
must first be reversed. A correct-course pass executed against item (e) as written
will leave the contradiction in place.

**Concrete incompatible artifacts.** UX-PB.2b's admission returns
`Result<OpIds, IpcError>` and is the transaction boundary; the rider is not one of its
inputs. UX-PB.5b's builder, reading `epics.md:1050`, calls `set_settings` first and
`execute_plan` second — two IPC round trips in that order, with the settings write
committed and `state.settings` published (`src-tauri/src/commands.rs:648`) before
`execute_plan` is even invoked. A builder reading AD-22 emits one call whose handler
persists last. Two command sequences, two failure matrices, one user gesture.

---

### V-4. AD-23's tombstone set has no lifetime, no bound, and no admission behaviour — UX-PB.1c and UX-PB.2b build opposite answers to "does `Update Everything` bring it back?"

**Pair:** UX-PB.1c ↔ UX-PB.2b.

AD-23 rule 4, `ARCHITECTURE-SPINE.md:683-687`:

> "Removal writes a tombstone on the intent. A later bulk expansion of any scope does
> not re-add a tombstoned ref — a member list can record presence but not a deliberate
> absence, so the 'stays removed' guarantee needs this home. Explicitly re-staging a
> tombstoned ref clears its tombstone: a user reversing themselves deliberately is not
> a silent re-add."

The domain minimum, `ARCHITECTURE-SPINE.md:409`:

> "  removed: unique Ref[]             # tombstones; no bulk expansion re-adds one"

That is the complete specification. It gives exactly one clearing event (explicit
re-staging). It says nothing about admission, nothing about a bound, and nothing about
inheritance into `RetryIntent`.

AD-17's custody rule, `ARCHITECTURE-SPINE.md:512-513`:

> "Admission transfers custody. The draft is emptied atomically with the mint of
> `planAttemptId`; a failed or rejected admission restores it unchanged."

AD-24 restates it, `ARCHITECTURE-SPINE.md:704-706`: "Admission empties it as custody
transfer (AD-17)".

**"Emptied" is undefined over `removed`.** `PlanIntent` has four fields
(`ARCHITECTURE-SPINE.md:406-411`); `removed` is one of them.

**Concrete incompatible artifacts.** Sequence: user invokes `Update Everything`
(`epics.md:574`, "`Update Everything` seeds all eligible work while remaining
editable"), removes `brew:wget`, confirms. The attempt runs and terminates. The user
later invokes `Update Everything` again.

- UX-PB.2b's builder reads "emptied" as `PlanIntent::default()` — a fresh intent with
  `removed: []`. `brew:wget` re-enters the new bulk expansion. Obeys AD-17:512 and
  AD-16's "may never add a member the user has not seen"
  (`ARCHITECTURE-SPINE.md:336-337`), because the new preview shows it.
- UX-PB.1c's builder reads AD-23:683 as unconditional — the tombstone is durable state
  with exactly one clearing event, and admission is not that event. `brew:wget` is
  silently suppressed from every subsequent bulk expansion for the rest of the
  session, with no surface anywhere telling the user why the count is one short.

Same gesture sequence, two package sets in the resulting `PlanIntent`, two wire
payloads, two fixtures under AD-3.

**Unbounded within a session.** AD-17:503-505 scopes the draft to a session ("never
written to disk … Every relaunch … starts with an empty draft"), which bounds
`removed` to one session. Within that session there is no bound at all: on the second
builder's reading, `removed` accumulates every ref the user ever removed and is
consulted on every bulk expansion. A user working through a 200-package inventory
across a morning accumulates a silent 200-entry suppression filter.

**Inheritance into `RetryIntent` is undefined.** `ARCHITECTURE-SPINE.md:417-419`:

```
RetryIntent                         # derived; never the persistent draft (AD-24)
  sourcePlanAttemptId: PlanAttemptId
  intent: PlanIntent                # the source's reviewed intent restricted to its failed members
```

`RetryIntent.intent` is typed `PlanIntent`, so it structurally carries `removed`.
AD-24 (`ARCHITECTURE-SPINE.md:707-711`) never says whether the source's tombstones come
along. UX-PB.4d's builder either inherits them (and the retry's canonical rebuild
silently drops a failed member that had been tombstoned pre-confirmation) or drops them
(and the retry re-expands work the user deliberately removed) — and AD-24's own promise,
`ARCHITECTURE-SPINE.md:715-718` ("its reviewed membership is exactly what the retry
review showed … the lineage claim stays true by construction"), holds either way.

**Hole to close.** AD-23 needs a fifth rule: the tombstone set's lifetime (cleared on
admission, or explicitly not), its bound, and whether `RetryIntent` inherits it.

---

## HIGH

### V-5. AD-23's `scope` enum does not cover selection-staged membership — Story 3.5 and Story 3.2 write different `origin` values for one gesture

**Pair:** Story 3.5 ↔ Story 3.2.

AD-23 rule 3, `ARCHITECTURE-SPINE.md:679-682`:

> "`scope` is descriptive. It records which action created the member — one Manager,
> the current filtered view, or everything — and is never re-evaluated"

The domain minimum fixes the closed set, `ARCHITECTURE-SPINE.md:414`:

```
  origin: Explicit | Bulk { scope: Manager(ManagerId) | FilteredView | Everything }
```

Three scopes. Now the staging paths the stories actually enumerate.

Story 3.5, `epics.md:1252-1253`:

> "**Given** eligible, current, pinned, greedy, filtered, and range-addressable rows
> **When** toggle, shift-range, tri-state, Cmd+A, Space, Cmd-click, Clear, and Esc
> interactions execute"

Story 3.2, `epics.md:1204-1206`:

> "**When** selection, row plan-add, per-Manager update-all, **update-selected**, and
> Update Everything draft-entry paths are exercised across every active filter"

FR-6, `epics.md:63`, makes the staging obligation explicit: "Support exact selection …
through individual, range, toggle, filter-aware select-all, tri-state, and clear
interactions … **add exact identities to one persistent draft**".

A shift-range selection of rows 4–11 followed by `update-selected` is neither one
Manager, nor the current filtered view, nor everything. It is an arbitrary subset.

**Concrete incompatible artifacts.**

- Story 3.5's builder — owning the per-row identity contract, `epics.md:1259`
  ("exactly one eligible Package's canonical identity is added to … the persistent
  draft Upgrade Plan") — records each range-selected member as
  `origin: "explicit"`, because the user addressed each row.
- Story 3.2's builder — owning `update-selected` as one of five "draft-entry paths"
  alongside `Update Everything` — records them as
  `origin: { "bulk": { "scope": "filteredView" } }`, because it is one action staging
  many members and `FilteredView` is the only enum value that is not plainly wrong.

Same gesture, two `PlanMember.origin` values, two `dev/fixtures/ipc/*.json` payloads.
AD-3's contract test byte-compares (`ARCHITECTURE-SPINE.md:178-180`), so the second
story to land breaks the first's fixture.

The behavioural divergence is not cosmetic. Under AD-23 rule 2
(`ARCHITECTURE-SPINE.md:675-678`, "`Explicit` dominates") plus rule 4's tombstone
clearing, the two artifacts differ on re-staging: an `Explicit` member that is removed
and re-staged clears its tombstone; a `Bulk` member that is removed and then covered by
a later select-all does not return. The user's subsequent select-all either restores the
row or does not, depending on which story landed the origin.

**Hole to close.** AD-23 must either add a `Selection` scope variant, or state that
selection-staged membership is `Explicit` per member regardless of how many rows one
gesture staged.

---

### V-6. AD-17's four-way precedence has two reachable combinations that contradict bound criteria verbatim

**Pairs:** UX-PB.4d ↔ UX-PB.1b, and UX-PB.4d ↔ UX-PB.4c.

AD-17's amended rule, `ARCHITECTURE-SPINE.md:522-528`:

> "Its visibility is a four-way union: a non-terminal attempt, an open retry review
> (AD-24), undismissed Results, or a non-empty draft — and that is also their content
> precedence, highest first. … Higher precedence hides lower content, never destroys
> it. When all four are false the region is hidden and the workspace reclaims its
> width with no reserved empty column."

Precedence, highest first: (1) non-terminal attempt, (2) retry review, (3) undismissed
Results, (4) non-empty draft.

**Combination A — retry review open, user stages a new item.** AD-24 guarantees the
staging succeeds, `ARCHITECTURE-SPINE.md:704-706`: "The one persistent draft has
exactly one author: a user staging or removal action resolved through the Rust
canonical rebuild." The draft becomes non-empty at precedence 4; the retry review at
precedence 2 hides it. But UX-PB.1b, `epics.md:546-548`:

> "**Given** an empty draft and no visible sidecar
> **When** I add the first eligible item
> **Then** the Upgrade Sidecar opens showing the draft grouped by Manager with
> `Updates`, `Managers`, and `Commands` counts, and focus stays on the source control
> that created it."

UX-PB.1b's criterion carries no precedence qualifier. Its builder makes the region
render the draft on first stage; AD-17's builder renders the retry review. Same
trigger, two region contents, two focus outcomes, two announcement payloads on the
single channel AD-17:541-544 permits.

The same criterion's inverse also breaks — `epics.md:554-556`:

> "**Given** a draft with one remaining item
> **When** I remove the last item
> **Then** **the sidecar closes**, the draft returns to empty, and nothing lingers in
> Activity or History."

With a retry review open, the region must *not* close. UX-PB.1b's builder closes it
(destroying the retry review, which AD-17:526 forbids: "never destroys it"); AD-17's
builder leaves it open. Note this is the *fourth*-precedence interaction that revision
7's three-way rule did not have — the retry review is new in revision 8, so this is a
pair the amendment introduced, not one it inherited.

**Combination B — retry review opened from History while an attempt is live.**
AD-17:525 explicitly contemplates the History origin: "a retry review opened from a
History entry makes the region visible without touching the draft underneath it." But
precedence 1 is a non-terminal attempt, so during a live attempt the retry review is
hidden. UX-PB.4d, `epics.md:974-977`:

> "**Given** a terminal Results or History entry with failed items and Retry available
> **When** I invoke Retry
> **Then** it first reveals the proposed failed-item scope **inline** with `Cancel` and
> `Create new plan`"

and UX-PB.4c, `epics.md:956-958`, which exists *specifically* to define the live-plus-replay
case:

> "**Given** a confirmed plan attempt is running when I open a History replay
> **When** the read-only replay opens
> **Then** the live sidecar stays visibly live, full Activity is labeled `Viewing past
> activity`, `Back to live activity` is offered"

UX-PB.4c's builder supports History inspection during a live attempt and offers Retry
from the replayed row; UX-PB.4d's builder must then "reveal the proposed failed-item
scope inline" — and AD-17 says the region shows the live attempt instead. The user
clicks Retry and nothing appears.

**Combination C — where does the retry review render at all?** UX-PB.4d says
"inline" (`epics.md:976`) — inline in Results. UX-PB.3d says Results is the region's
content, `epics.md:828`: "the sidecar transforms in place into a persistent Results
Summary that remains until `Done`". AD-17 makes the retry review a *sibling* content
state at higher precedence that hides Results. So UX-PB.4d's builder nests the retry
scope inside a visible Results surface; AD-17's builder replaces Results with the retry
review, reachable again only through UX-PB.4d's `View previous result`
(`epics.md:977`). Two DOM trees, two focus contracts, and two answers to what `Done`
dismisses.

**Hole to close.** AD-17 needs the precedence rule stated as a *lattice with stated
transitions*, not a linear order: which combinations are reachable, what the region
shows for each, and what a lower-precedence state's own trigger criteria mean when a
higher one is active. Combination B in particular needs a decision on whether Retry is
offered at all during a live attempt.

---

### N-1. The greedy cask has three homes — item ineligibility, plan-composition exclusion, and an intent-level option — and AD-16 forbids collapsing exactly the two it collapses

**Pair:** UX-PB.1d ↔ Story 3.2.

AD-16's ineligible-item rule, `ARCHITECTURE-SPINE.md:472-475`:

> "An item that is pinned, already current, **a non-opted-in greedy cask**, or removed
> between staging and rebuild is inert: its control is non-interactive to pointer and
> keyboard, it carries a stated reason for assistive technology, and **it can never
> enter a `PlanIntent`.**"

The very next rule, `ARCHITECTURE-SPINE.md:476-484`:

> "**Item ineligibility is not plan-composition exclusion.** The two resolve at
> different times and must not be collapsed. Item-level ineligibility is a property of
> the item alone and bars it from `PlanIntent` entirely. A plan-composition exclusion
> depends on what else is in the plan — `rustDedup` … and `alreadyRunning` — so the
> item stays in `PlanIntent` and is surfaced with its reason in the preview's
> exclusions."

And the domain minimum keeps the opt-in *on the intent*,
`ARCHITECTURE-SPINE.md:410`:

```
  includeGreedyCasks: boolean
```

These three statements cannot all hold. If a non-opted-in greedy cask "can never enter
a `PlanIntent`", then `includeGreedyCasks` — a field *of* the `PlanIntent` — can never
be the thing that opts it in; the item is barred before the field is reachable. AD-16
puts the greedy cask on the ineligibility side and its control on the exclusion side,
in consecutive rules that forbid collapsing the two.

**The tree says exclusion.** `src-tauri/src/queue.rs:527-531`:

```
        if p.kind == ipc::PackageKind::CaskGreedy && !req.include_greedy_casks {
            ...
                reason: ExcludeReason::GreedyCask,
```

The greedy cask is a plan-composition exclusion today, carried in the preview's
`exclusions`, gated by `PlanRequest.include_greedy_casks`
(`src-tauri/src/ipc.rs:364`), and the committed fixture `upgrade_plan.json` carries
`include_greedy_casks: false` (`src-tauri/src/ipc.rs:826`).

**Concrete incompatible artifacts.**

- UX-PB.1d builds it as item ineligibility. `epics.md:598-600` puts the greedy cask
  under the "excluded" reason string: "**Given** pinned, current, excluded, and
  unavailable Package controls … excluded `This Package is excluded by your Settings.
  Change the setting, then refresh Pack-Manager.`" and `epics.md:605` fixes the DOM:
  "it uses `aria-disabled="true"` rather than native `disabled`, keeps focus, announces
  its persistent reason as an accessible description, **stays inert on activation**".
  The reason text points the user at **Settings** — `include_greedy_by_default`,
  `src-tauri/src/settings.rs:35` — and at a **refresh**. No `PlanMember` is ever created
  and no exclusion entry appears.
- Story 3.2 builds it as a plan-level opt-in. `epics.md:1208-1210`: "**Given** ordinary
  and greedy-only casks **When** the default and explicit opt-in flows execute **Then**
  greedy-only casks are the documented set difference, remain
  separate/collapsed/default-excluded, and **enter a plan only through explicit opt-in
  with visible disclosure.**" An opt-in flow requires an interactive control and
  produces `ExcludeReason::GreedyCask` entries in the preview while off — matching the
  tree.

One row, two DOM contracts (`aria-disabled` inert vs. interactive opt-in), two
`UpgradePlan` payloads (no exclusion entry vs. a `GreedyCask` exclusion entry), two
fixtures under AD-3.

**Third owner.** `include_greedy_by_default` (settings) and `includeGreedyCasks`
(intent) are different fields with the same meaning. UX-PB.1d's reason text wires
eligibility to the settings key; AD-16's domain minimum wires it to the intent field;
the tree wires it to `PlanRequest`. `include_greedy_by_default` has **no consumer in
the plan path at all** — `grep -rn "include_greedy_by_default" src-tauri/src src`
returns only `settings.rs`, `SettingsView.tsx:140-141`, and fixtures. Whichever story
lands first decides which of three fields is authoritative, and AD-21 must then classify
a key whose plan-determining status depends on that undecided wiring.

**Hole to close.** AD-16 must place the greedy cask on exactly one side of its own
ineligibility/exclusion boundary, and name which of the two `includeGreedy*` fields is
the opt-in.

---

### N-3. AD-25's merge rule produces a snapshot with two timestamps; AD-25's presentation rule requires one

**Pair:** Story 2.2 ↔ UX-PB.1e.

AD-25 rule 2, `ARCHITECTURE-SPINE.md:733-739`:

> "A Manager that has ever produced a successful snapshot retains it on failure,
> labeled with **its own timestamp** and the exact failure, with a Retry affordance. A
> failure never replaces a good snapshot with an empty one, and never with a partial
> overlay: **recovered-parse output merges into the inventory already parsed from the
> successful outputs.**"

AD-25 rule 4, `ARCHITECTURE-SPINE.md:745-748`:

> "Health and staleness presentation derive from **the snapshot's real timestamp**. A
> Manager relying on a Last-good Snapshot reads as degraded with that timestamp and the
> specific failure; **no invented or interpolated value is ever substituted.**"

A merge of recovered-parse output into a Last-good Snapshot yields one inventory whose
rows carry two acquisition times: the rows recovered now, and the rows retained from
the earlier success. Rule 4 demands "the snapshot's real timestamp", singular, and
forbids interpolation — so there is no legal way to compute one value for a two-time
artifact.

**Concrete incompatible artifacts.**

- Story 2.2 owns the failure path per Manager (`epics.md:1156-1159`: "**Given** each of
  the six Manager adapters and its documented timeout boundary **When** controlled time
  reaches success, timeout, or error outcomes **Then** the correct Manager-specific
  terminal state and actionable detail appear"). Its builder stamps the merged snapshot
  with the recovery time — that is the time at which the current inventory was
  assembled, and rule 4 forbids inventing anything else.
- UX-PB.1e owns the rendering (`epics.md:630-632`): "**Given** a Manager whose refresh
  has failed **When** its Header and Card render **Then** they retain the last-good
  snapshot with **its timestamp**, state the exact failure summary with `Retry
  refresh`, and use text rather than an invented Health Meter value." Its builder
  reads AD-25 rule 2's "labeled with its own timestamp" — the Last-good Snapshot's
  timestamp — and renders the *older* value.

One merged `DetectionOutcome`, two `snapshotAt` values, two staleness verdicts, and
two different answers to "how old is this data" on the Dashboard Card versus the
Manager Header. Both obey AD-25.

The consequence reaches the plan path. AD-4:223-226 makes `detection` plan-relevant
state, and the plan builder resolves membership against it. A merged snapshot has no
per-row freshness, so a `PlanMember` can be built for a package that exists only in the
retained half — and AD-16:335-337 ("A canonical rebuild may remove or invalidate
membership") gives the rebuild no signal with which to invalidate it.

**Hole to close.** AD-25 needs per-row (or per-phase) acquisition provenance on a
merged snapshot, and rule 4 must say which timestamp governs presentation when the two
differ.

---

## MEDIUM

### N-2. AD-21 assigns classification ownership by landing order, which is a schedule, not a decision — the second story is free to reclassify

**Pair:** UX-PB.5b ↔ Story 3.4.

AD-21, `ARCHITECTURE-SPINE.md:638-641`:

> "The shipping call site bumps unconditionally for every key
> (`src-tauri/src/commands.rs` `set_settings_core`). Narrowing it is product work
> **owned by whichever of UX-PB.5b or Story 3.4 lands first**, not a test concern
> (AD-1)."

Ownership-by-landing-order settles who *writes* the table. It does not settle its
*contents*, and it does not forbid the second story from amending it. There are eight
persisted keys (`src-tauri/src/settings.rs:29-38`): `run_brew_update_on_refresh`,
`auto_refresh_on_launch`, `stall_after_secs`, `upgrade_hard_cap_mins`, `log_level`,
`auto_open_drawer`, `include_greedy_by_default`, `auto_check_for_updates`. UX-PB.5b
owns exactly one of them (`skipUpgradePlanConfirmation`, which does not exist yet);
Story 3.4 owns four (`epics.md:1227`).

If UX-PB.5b lands first it declares its own key inert and leaves the other seven
plan-determining under the fail-closed default (`ARCHITECTURE-SPINE.md:628-631`) —
which is safe but means changing `log_level` mid-review expires the preview. Story 3.4
then arrives with `epics.md:1229` ("log-level changes apply live only after
persistence") and a user-visible complaint, and reclassifies. Nothing in AD-21 stops it.

The observable divergence is a false explanation: under UX-PB.5b's table, a log-level
change makes UX-PB.5c's stale check fire, and UX-PB.5c's criterion
(`epics.md:1082-1084`) renders the reason as "for example a Package pinned, updated, or
removed since staging" — none of which happened.

**Hole to close.** AD-21 should carry the classification for the eight existing keys
inline, so neither story authors it.

### N-4. AD-17's "every relaunch starts with a hidden sidecar" is stated unconditionally but the retry review is reconstructible from durable state

**Pair:** UX-PB.1b ↔ UX-PB.4d.

AD-17, `ARCHITECTURE-SPINE.md:503-507`:

> "The draft is session-scoped and is never written to disk. **Every relaunch — after
> a clean quit, a crash, or a force-quit — starts with an empty draft and a hidden
> sidecar.**"

That rule clears the draft (precedence 4). It says nothing about precedence 2. The
retry review is derived from a durable `PlanAttempt` record (AD-24:707-711, "composed
in Rust from the failed attempt's reviewed intent restricted to its failed members"),
so unlike the draft it is fully reconstructible after a relaunch. UX-PB.4d's builder may
legitimately restore an open retry review from `retryOfPlanAttemptId` plus the source
record — producing a *visible* sidecar on relaunch, contradicting AD-17's unconditional
"hidden sidecar" and UX-PB.1b's relaunch criterion (`epics.md:558-560`).

**Hole to close.** AD-17 should state the relaunch state of all four content states,
not only the draft.

### N-5. AD-22's "the dialog retains the user's selection" and UX-PB.5a's "the plan remains editable for re-review" are two different post-rejection surfaces

**Pair:** UX-PB.5b ↔ UX-PB.5a.

AD-22 rule 3, `ARCHITECTURE-SPINE.md:659-660`:

> "On rejected admission nothing is persisted and nothing becomes active, and **the
> dialog retains the user's selection** so the choice is not silently lost."

"Retains the user's selection" requires the dialog to still exist. UX-PB.5a,
`epics.md:1030-1032`:

> "**Given** a confirmed admission
> **When** admission fails
> **Then** nothing executes, the dialog explains why, and **the plan remains editable
> for re-review.**"

"The plan remains editable" is the sidecar state UX-PB.5a's `Change Plan` route reaches
(`epics.md:1024`: "`Change Plan` returns focus to the first staged Remove control or the
plan heading"). One builder keeps the modal open with the checkbox ticked; the other
closes it back to an editable plan. UX-PB.5d's focus-restoration criterion
(`epics.md:1103-1105`) then has two different return targets to honour. `DialogHost`
shows one dialog at a time (`ARCHITECTURE-SPINE.md:538-540`), so this is a single
mutually exclusive state, not a layering question.

### N-6. Story 6.5's controlled-opener criterion and AD-26's production-composition rule cancel each other

**Pair:** Story 6.5 ↔ UX-PB.2d (and the AD-4 Deferred row).

AD-26 rule 3, `ARCHITECTURE-SPINE.md:768-772`:

> "Until such a composition exists and is verified, no story may claim
> native-transport coverage. AD-3 closes the fixture and browser-double routes; this
> closes the third — **it may not be claimed from an automation build that would never
> ship, because the coverage is only meaningful against the production composition.**"

Story 6.5's criterion, `epics.md:1290-1292`:

> "**Given** Export diagnostics and Open Logs actions
> **When** native command/opener success and failure are **controlled**
> **Then** the UI exposes actionable outcomes"

Controlling opener *failure* requires a controlled adapter, which AD-2
(`ARCHITECTURE-SPINE.md:157-159`) confines to "a construction-time dependency of a
non-distributable target". AD-26 rule 3 then denies that composition the right to
carry the coverage, because it is not the production composition. The production
composition uses the real macOS opener, whose failure cannot be induced.

This is adjacent to the known Open row at `ARCHITECTURE-SPINE.md:863`, but that row
scopes the problem to the **harness** ("whether a compliant non-distributable
composition exists at all"). The finding here is narrower and different: even *with* a
compliant harness, Story 6.5's controlled-failure criterion and AD-26's
production-composition requirement are mutually exclusive by construction, so the
harness decision does not unblock the criterion.

Pair-wise: UX-PB.2d builds correlation coverage from fixtures, which AD-3 permits
(`ARCHITECTURE-SPINE.md:194-198`, "Any story adding a field to an event payload …
owns fixture coverage of the shape"); Story 6.5 needs the same events proven through
real delivery. The two stories therefore produce different evidence for the same
`op:status` / `op:output` payload, and only one of them is allowed to call it coverage.

---

## LOW

### N-7. AD-11 says automated contrast is "an obligation on whichever story adds it"; no bound story adds it

**Pair:** UX-PB.5d ↔ the release-checklist owner of FR-19.

AD-11 as corrected in revision 8, `ARCHITECTURE-SPINE.md:292-296`:

> "**Automated 4.5:1 text contrast does not exist**; that same spec disclaims it …
> Contrast is therefore an obligation on whichever story adds it; reduced motion is a
> regression surface to preserve, not a gap to schedule (AD-1)."

NFR-6 requires it (`epics.md:113`, "meet at least 4.5:1 text contrast"). FR-19 is
assigned away from the stories entirely — `epics.md:438`: "FR-19: Release checklist —
Validate the coherent accessible interface in the installed packaged application", and
AD-11:297 makes that a "by-eye contrast check". UX-PB.5d is the only accessibility
story in the queue and its criteria (`epics.md:1096-1097`) name focus trapping, names,
roles, states, and reduced motion — never contrast.

So the obligation AD-11 creates lands on no one: UX-PB.5d's builder does not add it
(not in its criteria), and the release-checklist owner performs a manual check that
AD-11 itself distinguishes from the automated one. This is a gap rather than a clash,
hence LOW, but it is a gap revision 8's AD-11 rewrite newly created by naming an owner
that does not exist.

### N-8. AD-4 enumerates five ports as a closed list while two Open rows each direct a story to add a sixth

**Pair:** Story 6.5 ↔ UX-PB.2c.

AD-4 rule 1, `ARCHITECTURE-SPINE.md:205-208`:

> "**Five ports exist today and are extended rather than bypassed:** `CommandRunner`,
> `EventSink`, `UpdateSource`, `PendingRelease`, and `ManagerAdapter`."

AD-4 rule 2 (`ARCHITECTURE-SPINE.md:209-214`) names two new port obligations — "the
filesystem access AD-18's attempt journal requires, and the clock any verification or
staleness deadline reads" — owned by UX-PB.2c and the verification path. The Deferred
row at `ARCHITECTURE-SPINE.md:867` names a third: "Story 6.5 must introduce an
opener/reveal seam as **a sixth port** under AD-4".

Three new seams, one of them explicitly numbered "sixth", from two stories that will
land independently. Whichever lands second finds AD-4's enumeration already stale and
its own seam unnumbered. Low impact — the seams do not collide semantically — but the
rule's closed-list phrasing will be false the moment either story lands, and a builder
reading "five ports exist today … Effects already behind one of them stay behind it"
has no textual authority for a sixth beyond a Deferred-items row.

---

## Summary table

| # | Sev | Story pair | Clashing AD text | One-line |
| --- | --- | --- | --- | --- |
| V-1 | CRITICAL | Story 3.4 ↔ UX-PB.2c | AD-21:622-627 "membership, its exclusions, or the argv" × AD-16:349 "Execution must match the issued preview" | The bound command timeout is none of AD-21's three things, so the hard cap can legally change between review and execution |
| V-2 | CRITICAL | UX-PB.5b ↔ UX-PB.2b | AD-22:649-652 "holds `state.plan_coordinator` once, unbroken … No release-and-reacquire" × AD-4:242-245 "the single scheduler atomically checks and acquires" | Admission is an `await` to the scheduler task, which takes the same non-reentrant mutex — the unbroken hold does not compile and would deadlock |
| V-3 | CRITICAL | UX-PB.5b ↔ UX-PB.2b | AD-22:653-656 "admit and mint `planAttemptId`, **then** persist the rider" × `epics.md:1050` "takes effect only after persistence succeeds, and the plan is admitted" | The AD and its bound story specify opposite orderings; Open row (e) records this as a missing case, not a contradiction |
| N-1 | CRITICAL | UX-PB.1d ↔ Story 3.2 | AD-16:472-475 "a non-opted-in greedy cask … can never enter a `PlanIntent`" × AD-16:410 `includeGreedyCasks: boolean` × AD-16:476 "must not be collapsed" | The greedy cask's opt-in lives on the intent the item is barred from entering; three fields claim the same meaning |
| V-4 | HIGH | UX-PB.1c ↔ UX-PB.2b | AD-23:683-687 "Removal writes a tombstone on the intent" × AD-17:512 "The draft is emptied atomically with the mint of `planAttemptId`" | "Emptied" is undefined over `removed`; one builder re-adds the package on the next `Update Everything`, the other suppresses it for the session |
| V-5 | HIGH | Story 3.5 ↔ Story 3.2 | AD-23:679-682 "one Manager, the current filtered view, or everything" × `epics.md:1205` "update-selected … draft-entry paths" | A shift-range selection fits no `scope` variant; one story writes `Explicit`, the other `Bulk{FilteredView}` |
| V-6 | HIGH | UX-PB.4d ↔ UX-PB.1b, UX-PB.4c | AD-17:522-526 "four-way union … Higher precedence hides lower content" × `epics.md:548` "the Upgrade Sidecar opens showing the draft" / `epics.md:976` "reveals the proposed failed-item scope inline" | Staging during a retry review, and Retry during a live attempt, both render something other than what the bound criterion states |
| N-3 | HIGH | Story 2.2 ↔ UX-PB.1e | AD-25:737-739 "recovered-parse output **merges**" × AD-25:745-748 "derive from the snapshot's real timestamp … no invented or interpolated value" | A merged snapshot has two acquisition times; the presentation rule demands one and forbids computing it |
| N-2 | MEDIUM | UX-PB.5b ↔ Story 3.4 | AD-21:638-641 "owned by whichever of UX-PB.5b or Story 3.4 lands first" | Landing order names an author, not a classification; the second story may reclassify all eight keys |
| N-4 | MEDIUM | UX-PB.1b ↔ UX-PB.4d | AD-17:503-505 "Every relaunch … starts with an empty draft and a hidden sidecar" × AD-24:707-711 retry review derived from durable records | Only the draft is cleared on relaunch; the reconstructible retry review may reopen the region |
| N-5 | MEDIUM | UX-PB.5b ↔ UX-PB.5a | AD-22:659-660 "the dialog retains the user's selection" × `epics.md:1032` "the plan remains editable for re-review" | Post-rejection is either an open modal or a closed one; `DialogHost` allows only one state |
| N-6 | MEDIUM | Story 6.5 ↔ UX-PB.2d | AD-26:768-772 "coverage is only meaningful against the production composition" × `epics.md:1291` "native command/opener success and failure are controlled" | Controlling failure needs a non-distributable composition AD-26 denies coverage to — distinct from the harness question in Open row 863 |
| N-7 | LOW | UX-PB.5d ↔ FR-19 release checklist | AD-11:294-296 "an obligation on whichever story adds it" × `epics.md:438` FR-19 assigned to the release checklist | The automated 4.5:1 check is owed by a story that does not exist |
| N-8 | LOW | Story 6.5 ↔ UX-PB.2c | AD-4:205-208 "Five ports exist today" × spine:867 "a sixth port under AD-4" | Two stories each add a seam to a rule phrased as a closed list |

---

## Method note

Every AD quotation is from `ARCHITECTURE-SPINE.md` as read this session (877 lines);
every story quotation is from `epics.md` as read this session (1,293 lines). Line counts
are `wc -l`. Code claims are quoted from the tree: `src-tauri/src/settings.rs`,
`src-tauri/src/commands.rs`, `src-tauri/src/queue.rs`, `src-tauri/src/state.rs`,
`src-tauri/src/ipc.rs`, `src-tauri/src/managers/mod.rs`. The
`include_greedy_by_default` no-consumer claim in N-1 is from
`grep -rn "include_greedy_by_default\|caskGreedy\|includeGreedy" src-tauri/src src`,
whose only hits outside `settings.rs`, `SettingsView.tsx`, and test fixtures are the
`caskGreedy:` id-prefix sites.

Findings deliberately **not** re-reported: `review-divergence-v6.md` H-1 through H-5,
M-1 through M-4, L-1, L-2 (the reviewer-gate tail at `ARCHITECTURE-SPINE.md:875`), and
items (a)–(e) of the correct-course batch at `ARCHITECTURE-SPINE.md:876`. V-3 and N-6
each state explicitly what the corresponding known row does not cover.

---

# Verification pass — fixes applied

Re-read against `ARCHITECTURE-SPINE.md` at 948 lines (`wc -l`, 2026-07-25). All line
citations in this section are to the **948-line** revision; the sections above cite the
877-line snapshot and are left unchanged. Scope: only the six changes named in the
verification request. Same standard — a finding is real only if it names two stories,
quotes the AD text each obeys, and describes concrete incompatible artifacts. Items
that are audit questions rather than pair questions are labelled as such.

| # | Change | Verdict |
| --- | --- | --- |
| 1 | AD-22 rules 1–3 rewritten (answers V-2) | **HOLDS** |
| 2 | AD-22 rule 3 explicit override of UX-PB.5b (answers V-3) | **HOLDS** |
| 3 | Custody transfer scoped to what was admitted, four sites | **HOLDS** |
| 4 | AD-17 reverted four-way → three-way; retry scope relocated (answers V-6) | **WEAKENED** |
| 5 | AD-21 plan-determining test widened (answers V-1) | **WEAKENED** |
| 6 | AD-16 gains `Cancelling` / `Interaction required` as durable states | **HOLDS** |

New findings: **1 HIGH, 3 MEDIUM, 1 LOW** (W-1 … W-5).

---

## 1. AD-22 rules 1–3 — **HOLDS**

The rewrite is a correct instruction. `ARCHITECTURE-SPINE.md:676-683`:

> "The confirming action is atomic against the **canonical revision**, not against one
> mutex hold. Validation reads under `state.plan_coordinator`; the guard is released
> before admission; the scheduler re-checks the same `expected_revision` under its own
> acquisition and enqueues all-or-none or nothing."

That is the shipping shape verbatim: guard scoped to the block ending at
`src-tauri/src/commands.rs:460`, released at 461, `.await` at 465, re-check at
`src-tauri/src/queue.rs:1011`. Rule 2 (`ARCHITECTURE-SPINE.md:684-689`) states the
`std::sync::Mutex` / `!Send` / scheduler-reacquisition reasons explicitly and forbids
persisting under a held guard. V-2 is closed on all three counts.

### The guard-release → re-check window: attacked, and it is closed

This was the stated main worry. It survives the attack, for a reason the rewrite does
not spell out but which the tree makes decisive.

A writer landing in the window falls into exactly two cases.

- **Plan-determining writer.** It bumps the revision, `handle_plan_batch`'s
  `coordinator.revision() != expected_revision` (`src-tauri/src/queue.rs:1011`) fails,
  and the admission is rejected as `PlanBatchError::RevisionChanged` → `plan_stale`
  (`src-tauri/src/commands.rs:468-470`). Fail-closed, and AD-16:352-357 designates
  rejection-on-drift as the intended outcome.
- **Plan-inert writer.** It does not bump, so the re-check passes — but it cannot alter
  what runs, because `subs` was already fully bound **under the guard**.
  `execute_issued_plan` snapshots settings at `src-tauri/src/commands.rs:376`
  (`current_plan_state(state)`) and builds every `OpSubmission` from that snapshot at
  lines 395-459; `bind_commands` resolves `CommandSpec.timeout` from those values at
  `src-tauri/src/queue.rs:98-104`. The scheduler enqueues the `subs` it was handed. A
  settings write after line 460 cannot reach them.

So the window admits no divergence in either branch. The residual gap I looked for —
`handle_plan_batch` re-checking only four predicates (revision,
`state_update_in_progress`, `active_refresh`, lock conflict; `src-tauri/src/queue.rs:1011-1030`)
rather than re-deriving the plan — is not a gap, because the derivation it would repeat
is frozen in `subs`.

**One asymmetry worth recording, not a finding.** AD-22 rule 3 persists the rider
"once the admission has returned" — i.e. outside the section rule 1 defines. A crash
between admission and rider persist therefore leaves the attempt admitted and the
opt-out unsaved. That lands on the safe side, and AD-22 rule 4
(`ARCHITECTURE-SPINE.md:701-703`) accepts it by name: "an unsaved opt-out costs one
extra confirmation, while a saved opt-out on a refused run removes the gate from a run
the user never got." Deliberate, and correct.

## 2. AD-22 rule 3's override of UX-PB.5b — **HOLDS**

`ARCHITECTURE-SPINE.md:690-695`:

> "Ordering is fixed — validate, admit through the scheduler's revision-checked
> transaction, then persist the rider once the admission has returned. A rider never
> precedes the admission it rides on. **This deliberately overrides UX-PB.5b's stated
> clause order** (persist, activate, then admit); see the rider rule below for why, and
> the `epics.md` batch row for the criterion that must be restated."

A builder cannot miss it: it is bolded, names the story, restates the wrong order in
parentheses so no lookup is needed, gives a forward pointer for the rationale, and
UX-PB.5b is first in AD-22's `Binds` line (`ARCHITECTURE-SPINE.md:672`). The
correct-course row was re-scoped to match — `ARCHITECTURE-SPINE.md:947` item (e) now
reads "not merely a missing case: its criterion states the *reverse* of AD-22's fixed
ordering … The criterion must be restated, and the rejected-admission case added." V-3's
complaint (that the row characterised a contradiction as a gap) is closed.

## 3. Custody transfer scoped to what was admitted — **HOLDS**

All four sites agree, and they agree on the *trigger*, which was the substance:

- `ARCHITECTURE-SPINE.md:527-533` — "Admission transfers custody **of what it
  admitted**. … Admitting a derived intent — a retry scope (AD-24) — consumes that
  intent and leaves the draft and its tombstones untouched … **Minting a
  `planAttemptId` is not by itself the trigger; being the admitted intent's source is.**"
- `ARCHITECTURE-SPINE.md:748-753` — "Admission of the draft's own preview empties it as
  custody transfer (AD-17) … **A confirmed retry does not empty the draft**".
- `ARCHITECTURE-SPINE.md:728-732` — "an admission that empties the draft carries them
  off with it, while a retry admission leaves both untouched (AD-24). A new draft starts
  with none."
- `ARCHITECTURE-SPINE.md:460` / `:469` — `Preview --> Admitted: … draft emptied
  atomically` versus `RetryPreview --> Admitted: … draft NOT emptied`.

**No fifth site claims the mint empties the draft.** `grep -n "empt"` over the file
returns no other custody assertion; the Consistency table (`:854`), the Structural Seed
(`:901`), and the Capability map (`:910`) are all silent on the trigger.

**V-4 is closed as a by-product.** AD-23's new lifetime rule (`:728-732`) answers all
three questions V-4 raised: cleared on draft admission, session-scoped and never
persisted, "A new draft starts with none" bounds growth. The third sub-point —
whether `RetryIntent.intent.removed` inherits the source attempt's tombstones — is
closed *in effect* rather than in text: AD-24:754-758 composes the retry intent
"restricted to its failed members", and a restriction performs no bulk expansion, so a
tombstone is inert inside a `RetryIntent` whichever way a builder populates it. No
observable divergence remains.

## 4. AD-17 three-way union and the relocated retry scope — **WEAKENED**

**Combinations B and C are genuinely gone**, not relocated.

- **C** (where the retry scope renders): `ARCHITECTURE-SPINE.md:546-549` now fixes it —
  "It is a content state *inside* the surface the user invoked Retry from — Results, or
  a read-only History replay". AD-24:759-762 restates it. That matches UX-PB.4d's
  "inline" (`epics.md:976`) exactly. Closed.
- **B** (Retry during a live attempt): the retry scope no longer competes for the
  sidecar, and a History replay lives in `ActiveView`/Activity, not the region
  (AD-17:557-562). UX-PB.4c's live-plus-replay contract (`epics.md:956-958`) now
  composes cleanly. Closed.

**Combination A is relocated, and the rule that used to adjudicate it was deleted in
the same edit.** That is W-1.

### W-1 (HIGH) — the precedence rule was deleted with the fourth member, and AD-24 still cites it

**Pair:** UX-PB.1b ↔ UX-PB.3d.

The four-way rule carried two sentences that the three-way rule does not:

> "…or a non-empty draft — **and that is also their content precedence, highest first.**"
> "**Higher precedence hides lower content, never destroys it.**"

The replacement, `ARCHITECTURE-SPINE.md:538-545`, reads in full:

> "Its visibility is a three-way union: a non-empty draft, a non-terminal attempt, or
> undismissed Results. A confirmed attempt replaces its content in place rather than
> opening a second surface, and Results remain until dismissed even though the draft
> behind them is empty. When all three are false the region is hidden and the workspace
> reclaims its width with no reserved empty column."

Neither the precedence declaration nor the non-destruction guarantee survives.
`grep -n "precedence"` over the 948-line file returns only AD-4's unrelated route
precedence (`:234`), AD-24's cross-reference (`:764`), and a stale Decision Status row
(`:943`). **The word does not appear in AD-17 at all.**

Two consequences.

**(a) A dangling cross-reference.** AD-24 rule 4, `ARCHITECTURE-SPINE.md:763-765`:

> "It stays in the persistent draft and surfaces once **higher-precedence content** is
> dismissed, **exactly as AD-17 promises.**"

AD-17 no longer promises it. A builder following the citation finds no precedence rule.

**(b) The list order inverted.** The four-way list ran attempt → retry → Results →
draft and said "highest first". The three-way list runs **draft → attempt → Results**.
A builder who reads list order as significant — which the previous revision trained them
to do — now reads the draft as *highest*, the exact inversion of what AD-17:534-537
intends ("new membership staged during that attempt accumulates in the canonical draft
without displacing it — surfacing in the region only once the attempt's Results are
dismissed").

**Concrete incompatible artifacts.** Results are undismissed and the user stages an
item.

- UX-PB.1b's builder, `epics.md:546-548`: "**Given** an empty draft and no visible
  sidecar **When** I add the first eligible item **Then** the Upgrade Sidecar opens
  showing the draft grouped by Manager with `Updates`, `Managers`, and `Commands`
  counts, and focus stays on the source control that created it." Unqualified, and the
  sentence that used to override it is gone. The region shows the draft; focus and the
  announcement follow the draft.
- UX-PB.3d's builder, `epics.md:828`: "the sidecar transforms in place into a persistent
  Results Summary that **remains until `Done`**". The region shows Results.

One region, two contents, two focus targets, and two payloads on the single
announcement channel AD-17:566-569 permits. The old rule resolved this in one clause;
the new rule resolves it only by inference from the parenthetical "the draft surfacing
behind Results" at `ARCHITECTURE-SPINE.md:552` — which is inside the *retry-scope* rule,
where a builder implementing UX-PB.1b has no reason to look.

**Hole to close.** Restore an explicit precedence declaration and the non-destruction
guarantee to the three-way rule, and repoint AD-24:764.

### W-2 (MEDIUM) — `Done` destroys a retry scope that fix 4 moved inside Results

**Pair:** UX-PB.3d ↔ UX-PB.4d.

Under the four-way rule the retry review outranked Results, and "Higher precedence
hides lower content, never destroys it" protected it. Fix 4 moved it *inside* Results
(`ARCHITECTURE-SPINE.md:546-549`) and deleted that guarantee in the same edit, so
Results dismissal now reaches it.

- UX-PB.3d, `epics.md:828`: Results "remains until `Done`" — its builder wires `Done`
  to dismiss Results unconditionally, and AD-17:543 backs it ("Results remain until
  dismissed").
- UX-PB.4d, `epics.md:974-977`: the retry scope's only stated exits are `Cancel` and
  `Create new plan`. Its builder has no criterion for "the surface I am nested in was
  dismissed underneath me", and AD-24:773-776 covers only the rebuild-failure exit.

Concrete: with a reviewed retry scope open, does `Done` become disabled, prompt, or
silently discard the scope? Two builders, three defensible answers, and the discard
branch loses a reviewed object with no announcement — which AD-17:566-569's single
channel has no message for.

## 5. AD-21's widened plan-determining test — **WEAKENED**

**The timeout hole is genuinely closed.** `ARCHITECTURE-SPINE.md:647-654`:

> "…membership, its exclusions, the argv Pack-Manager would construct, **or any
> execution parameter the reviewed snapshot records — timeouts and stall thresholds
> included**, because the user reviewed a plan that runs under them."

Naming timeouts and stall thresholds explicitly removes the classification discretion
V-1 exploited: Story 3.4 can no longer declare `stall_after_secs` or
`upgrade_hard_cap_mins` inert. I re-verified the settings→execution surface is exactly
those two: `make_upgrade_submission` and `make_self_update_submission` both funnel
`&Settings` into `bind_commands`, which touches only `CommandSpec.timeout`
(`src-tauri/src/queue.rs:98-104`); the child environment is constructed from `ToolEnv`
and constants, carrying no settings-derived value
(`src-tauri/src/paths.rs:273-298` — `PATH`, `HOME`, `USER`/`LOGNAME`/`TMPDIR`, `LANG`,
`NO_COLOR`, `TERM`, `GIT_TERMINAL_PROMPT`, four `HOMEBREW_*`). So `log_level` is
cleanly inert, and no other key reaches a reviewed execution parameter. That half of
V-1 holds.

### W-4 (MEDIUM) — clause 3 still has no scope qualifier, and the fail-closed worked example still recites the old three-part test

**Pair:** Story 2.2 ↔ Story 3.4.

The fourth clause is scoped ("any execution parameter **the reviewed snapshot**
records"). The third is not — "the argv Pack-Manager would construct", unqualified.
And rule 2's worked example, `ARCHITECTURE-SPINE.md:655-660`, still models the reason a
builder is meant to copy in **three** parts:

> "`skipUpgradePlanConfirmation` is plan-inert — it selects whether a modal renders and
> cannot reach membership, exclusions, **or argv**."

The classification of that key is correct either way. The template is what leaks.

`run_brew_update_on_refresh` sits exactly on the seam. `src-tauri/src/managers/brew.rs:86-95`:

```
        if settings.run_brew_update_on_refresh {
            plan.push(PlannedCommand {
                label: "brew update",
                argv: argv(&["update"]),
```

It constructs argv — for a *refresh*, never for an upgrade preview.

- **Story 2.2** owns it. Its criterion turns on this key alone, `epics.md:1151-1154`:
  "**Given** Homebrew metadata refresh is enabled or disabled **When** a Brew refresh is
  planned and rendered **Then** the enabled path shows the required
  update/inventory/outdated phase order **And** the disabled path omits only the
  metadata-update phase without mislabeling later phases." Reading clause 3 literally —
  the key changes argv — its builder declares it plan-determining.
- **Story 3.4** owns the Settings surface (`epics.md:1227-1230`). Reading clause 4's
  scope qualifier back onto clause 3 — an upgrade plan's reviewed snapshot never
  contains `brew update` — its builder declares it plan-inert, with rule 2's three-part
  reason.

**Concrete incompatible artifacts.** One key, two definition-site declarations, two
revision behaviours. Under Story 2.2's reading, toggling Homebrew metadata refresh
while an Upgrade Plan preview is open expires it, and UX-PB.5c renders the reason from
its own criterion, `epics.md:1082-1084`: "for example a Package pinned, updated, or
removed since staging" — none of which happened. Under Story 3.4's reading the preview
survives. Same gesture, two outcomes, and the fail-closed default does not adjudicate
because both stories *do* classify.

Severity is MEDIUM rather than CRITICAL because both branches are safe — the
disagreement costs a false-reason re-review, not an unreviewed execution parameter.

**Hole to close.** Give clause 3 the same scope qualifier clause 4 has ("the argv **of
the reviewed plan**"), and restate rule 2's worked example in four parts so the copied
template is complete.

## 6. AD-16's `Cancelling` / `Interaction required` durable-state rule — **HOLDS**

No contradiction with the existing emission rule. The two are orthogonal: the emission
rule (`ARCHITECTURE-SPINE.md:401-403`) constrains **when** the state may be entered
("only from a closed Manager-specific classifier or an explicit typed native signal");
the new rule (`:384-391`) constrains **what kind of thing it is** (a durable `OpStatus`
variant, not a transient flag). The new rule names `Interaction required` explicitly at
`:385`, so a builder reading both cannot conclude they are the same axis.

The `OpStatus` count is accurate — `src-tauri/src/ipc.rs:99-107` declares exactly seven
variants (`Queued`, `Running`, `Succeeded`, `Failed`, `Cancelled`, `TimedOut`,
`Interrupted`).

### W-5 (LOW) — "emitted" now means the forbidden thing

The new rule closes with `ARCHITECTURE-SPINE.md:391`: "Emitting an event alongside a
state is fine; **emitting one *instead of* a state is the defect.**" It thereby fixes
"emit" to mean *event*. The older rule at `:401` then reads: "`Interaction required` is
**emitted** only from a closed Manager-specific classifier". Under the new vocabulary
that sentence describes exactly the defect. A builder of UX-PB.3f reading only `:401` —
and UX-PB.3f's own criterion is presentation-neutral, `epics.md:872`, "the Operation
**shows** `Interaction required`" — has textual cover for an event-only implementation.
Adjacent rules, not contradictory ones; one word ("emitted" → "entered") closes it.

### Not claimed as new: the AD-3 atomic-change span widened

Fix 6 requires `OpStatus` to gain four variants — `Verifying` and `Skipped` (`:378-383`)
plus `Cancelling` and `Interaction required` (`:384-391`) — while both rules demand one
atomic AD-3 change. Their story owners span waves: UX-PB.3c (`epics.md:806`,
`verifying`), UX-PB.2e (`epics.md:732`, `Cancelling` and `Skipped`), UX-PB.3f
(`epics.md:872`), UX-PB.3g (`epics.md:894`). That is `review-divergence-v6.md` H-4
("the 'one atomic change' spans two waves") **widened from two stories to four**, not a
new finding, so it is recorded here rather than tiered.

---

## Audit defects (do not meet the two-story pair bar)

### W-3 (MEDIUM) — two Decision Status rows still describe the pre-fix rules

Both are stale in the direction that matters: they describe the formulation the fixes
repudiated, so a builder who reads the status table for orientation is pointed at the
wrong rule.

`ARCHITECTURE-SPINE.md:941`:

> "Closed by **AD-21** … and **AD-22** (**one unbroken critical section**; the rider
> persists only after a successful admission)."

AD-22 rule 2 (`:684-689`) now says a rule demanding one unbroken hold "would not
compile, and would deadlock if it did."

`ARCHITECTURE-SPINE.md:943`:

> "Closed by **AD-24** … **AD-17's visibility rule gained a fourth content state with
> explicit precedence.**"

Both halves are now false: AD-17:546 says "The retry scope is **not** a fourth member of
that union", and the explicit precedence was deleted (W-1).

These are documentation defects rather than divergence pairs — no two stories build
incompatibly from them — but they are the residue of fixes 1/2 and 4 respectively and
should move with them.

---

## Verification summary table

| # | Sev | Story pair | Clashing text | One-line |
| --- | --- | --- | --- | --- |
| W-1 | HIGH | UX-PB.1b ↔ UX-PB.3d | AD-17:538-545 three-way union (no precedence clause) × `epics.md:548` "the Upgrade Sidecar opens showing the draft" / `epics.md:828` "remains until `Done`" | The precedence declaration and the "never destroys it" guarantee were deleted with the fourth member; AD-24:764 still cites them and the list order inverted |
| W-2 | MEDIUM | UX-PB.3d ↔ UX-PB.4d | AD-17:546-549 retry scope "inside … Results" × `epics.md:828` `Done` dismisses Results | Relocating the retry scope inside Results, without the deleted non-destruction rule, lets `Done` discard a reviewed retry scope |
| W-3 | MEDIUM | *(audit — no pair)* | AD-22:684-689 × spine:941; AD-17:546 × spine:943 | Two Decision Status rows still describe the repudiated "unbroken critical section" and the removed "fourth content state with explicit precedence" |
| W-4 | MEDIUM | Story 2.2 ↔ Story 3.4 | AD-21:649 "the argv Pack-Manager would construct" (unscoped) × AD-21:659-660 three-part worked reason | `run_brew_update_on_refresh` builds refresh argv (`brew.rs:86-95`); clause 3 lacks clause 4's scope qualifier so both stories classify it, oppositely |
| W-5 | LOW | UX-PB.3f ↔ *(AD-16 internal)* | AD-16:391 "emitting one *instead of* a state is the defect" × AD-16:401 "`Interaction required` is **emitted** only from…" | The new rule fixes "emit" to mean event; the older rule then literally describes the defect |

