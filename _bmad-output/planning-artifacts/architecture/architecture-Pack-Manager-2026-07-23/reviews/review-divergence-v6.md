# Divergence review — ARCHITECTURE-SPINE.md revision 6

**Lens:** Attack the spine as an adversary. Construct two units one level down that
each obey every AD to the letter yet still build incompatibly — clashing shared-data
shapes, two owners of one entity, conflicting state-mutation paths. Every pair is a
hole to close with a new or tightened AD.

**Target:** `_bmad-output/planning-artifacts/architecture/architecture-Pack-Manager-2026-07-23/ARCHITECTURE-SPINE.md`
(revision 6, status `final`, 648 lines).

**Units:** the 28 Epic UX-PB stories (`epics.md:468`–`epics.md:1093`) and surviving
Stories 2.2, 3.1, 3.2, 3.4, 3.5, 6.5 (`epics.md:1099`–`epics.md:1250`).

**Out of scope (closed by prior rounds, not re-reported):** no entry point executes;
canonical rebuild may not enlarge membership / `AllEligible` expansion frozen;
verification refreshes begin strictly after process exit and are coalescing-exempt;
admission transfers custody atomically / three-way sidecar union; `Verifying` and
`Skipped` are durable wire states; the draft is session-scoped and fail-to-empty.

**Verification note.** Code-grounded claims below were checked against the tree on
2026-07-25: `src-tauri/src/capabilities/default.json` grants exactly the three
permissions AD-20 names; `src-tauri/src/ipc.rs:99-107` declares
`OpStatus { Queued, Running, Succeeded, Failed, Cancelled, TimedOut, Interrupted }`;
`grep -rin "cancelling\|verifying\|skipped" src/ src-tauri/src/` returns only
unrelated prose and comments — no such symbol exists in product code.

**Counts:** 3 CRITICAL, 5 HIGH, 4 MEDIUM, 2 LOW. 14 total.

---

## CRITICAL

### C-1. UX-PB.5b's in-admission settings write trips UX-PB.2b's revision-drift rejection — the safety opt-out deterministically fails its own run

**Stories:** UX-PB.5b (`epics.md:997`) ↔ UX-PB.2b (`epics.md:616`), via UX-PB.5a (`epics.md:967`).

**ADs each obeys.** AD-19 says a settings write advances the canonical revision:

> `ARCHITECTURE-SPINE.md:537-539` — "**Rule:** A settings patch is persisted before it
> becomes active in memory or advances the canonical revision; a failed save changes
> neither. Every control saves immediately and atomically with visible `Saving` /
> `Saved` / failure state."

AD-16 rejects admission on revision drift:

> `ARCHITECTURE-SPINE.md:311-314` — "membership can never silently differ from the
> reviewed intent. Execution must match the issued preview and a fresh coherent
> rebuild, and is rejected on in-progress state change, revision drift, an active
> refresh, or a lock-set overlap with any pending or running mutating operation."

AD-4 makes `settings` plan-relevant state read under the coordinator lock:

> `ARCHITECTURE-SPINE.md:197-198` — "Any code reading plan-relevant state —
> `detection`, `registry`, `queue.records()`, `settings`, `tool_env` — must already
> hold `state.plan_coordinator`."

**The clash.** UX-PB.5b binds a settings write to the *same click* that admits the plan:

> `epics.md:1011-1013` — "**Given** the dialog with `Disable upgrade plan command
> execution confirmation` selected **When** I choose the final `Confirm N Updates`
> **Then** `skipUpgradePlanConfirmation: true` is written atomically, the new value
> takes effect only after persistence succeeds, and the plan is admitted."

UX-PB.2b builds the other half:

> `epics.md:626-629` — "**Given** a reviewed plan authorized by a one-use preview
> `planId` **When** I invoke the confirmed run action (`Confirm N Updates`, or the
> confirmation-off run action) and admission succeeds **Then** `execute_plan`
> atomically returns one new durable `planAttemptId`".

**Concrete incompatible artifacts.** The preview `planId` was minted at canonical
revision *R*. UX-PB.5b's handler persists `skipUpgradePlanConfirmation: true` →
revision advances to *R+1* (AD-19) → UX-PB.2b's `execute_plan` validates the preview
against the coordinator epoch → **revision drift → admission rejected** (AD-16). The
user's very first use of the checkbox produces:

- `epics.md:993-995` — "**When** admission fails **Then** nothing executes, the dialog
  explains why, and the plan remains editable for re-review." — a rejection with no
  explicable cause; and
- `skipUpgradePlanConfirmation` is now `true`, so the *next* attempt opens no dialog at
  all and lands on UX-PB.5c's `Run N Updates` path (`epics.md:1039`).

The one click both disarmed the safety gate and refused to run. The reverse ordering
(admit, then persist) is equally spine-legal and produces the opposite artifact: the
attempt runs but the setting is not yet active, and a save failure at
`epics.md:1019-1021` ("the prior preference is retained as both active and persisted
state") now has to unwind nothing while an attempt is already live.

The lock order compounds it: the settings write must take `state.plan_coordinator`
(AD-4:197), which `execute_plan` already holds for the atomic admission — the two
stories will pick different lock lifetimes (nested acquire vs. release-and-reacquire),
and release-and-reacquire is exactly what re-opens the drift window.

**AD to add/tighten.** AD-19 must carve out an ordering rule for a settings patch issued
inside a confirmation action, or AD-16 must state that a settings-only revision advance
originating from the confirming action is not drift. Naming which of the two is the
tie-break is the whole fix; leaving it to the stories guarantees divergence.

---

### C-2. `PlanIntent.kind` is one scalar, but AD-16 requires per-member provenance — UX-PB.1a and UX-PB.1c cannot produce the same wire shape

**Stories:** UX-PB.1a (`epics.md:468`) ↔ UX-PB.1c (`epics.md:525`); Story 3.2
(`epics.md:1150`) is the third writer.

**ADs each obeys.** The normative domain minimum gives the intent exactly one kind:

> `ARCHITECTURE-SPINE.md:363-366` —
> ```
> PlanIntent
>   kind: Explicit | AllEligible      # durable; a removal converts AllEligible -> Explicit
>   packageUpdates: ordered unique PackageRef[]
>   managerUpdates: ordered unique ManagerId[]   # independent removable members
> ```

But the domain rule under AD-16 demands provenance *per member*:

> `ARCHITECTURE-SPINE.md:406-412` — "**Intent kind.** `PlanIntent` distinguishes
> explicitly chosen membership from bulk `AllEligible` membership, durably: a
> bulk-added item the user removes stays removed across a rebuild, and **an explicit
> item is never silently absorbed into a later bulk action.** `AllEligible` carries
> the scope of the action that created it — one Manager, the current filtered view, or
> everything ... It never silently widens to a larger scope."

**The clash.** UX-PB.1a stages one row at a time, producing explicit membership:

> `epics.md:478-480` — "**When** I toggle its plan Checkbox by pointer, Enter/Space, or
> the grid Space key **Then** the Package's canonical identity is added to the one
> persistent draft Upgrade Plan".

UX-PB.1c stages four bulk entry points into that same draft:

> `epics.md:535-537` — "**Given** eligible work reachable from the count-labeled header
> Checkbox, the Manager Header `Update Manager` action, a Manager-wide action, and
> `Update Everything` **When** I invoke each entry point **Then** each adds its eligible
> canonical identities to the same one persistent draft, `Update Everything` seeds all
> eligible work while remaining editable".

**Concrete incompatible artifacts.** Two sequences, both spine-legal:

1. Stage `brew:node` by row (UX-PB.1a) → `{kind: "explicit", packageUpdates: ["brew:node"]}`.
   Then invoke `Update Everything` (UX-PB.1c). The single `kind` field must take one
   value. If it becomes `allEligible`, `brew:node` "was silently absorbed into a later
   bulk action" — forbidden at `:408`. If it stays `explicit`, `Update Everything` never
   created an `AllEligible` intent, so the frozen-expansion and re-seed-offer machinery
   at `:299-302` never applies to the very action it was written for.

2. Invoke Manager-wide update-all on brew, then `Update Everything`. Both are
   `AllEligible`, but `:409` says the kind "carries the scope of the action that created
   it" and "never silently widens to a larger scope". The struct has one `kind` and no
   scope field at all — there is nowhere to record `Manager(brew)` and nowhere to record
   that a second, wider bulk action arrived. Story 3.2 requires this exact combination
   across filters: `epics.md:1164-1166` — "**When** selection, row plan-add, per-Manager
   update-all, update-selected, and Update Everything draft-entry paths are exercised
   across every active filter".

UX-PB.1a's builder emits `PlanIntent { kind: Explicit }`; UX-PB.1c's emits
`PlanIntent { kind: AllEligible, scope: … }` with a field the domain minimum does not
have. Under AD-3 these are one wire model with one committed fixture
(`ARCHITECTURE-SPINE.md:152-154` — "byte-compares each serialized model against its
committed fixture, and the TypeScript half asserts its fixture set exactly equals its
guard map"), so the two stories cannot both land.

**AD to add/tighten.** The domain minimum must either (a) move provenance onto the member
(`packageUpdates: {ref, origin: Explicit | Bulk{scope}}[]`), or (b) state that a bulk
mutation over an already-`Explicit` intent is rejected/downgraded, and say which. AD-16's
`:408` prohibition is unimplementable against the struct at `:363-366` as written.

---

### C-3. Retry and the accumulating draft are two owners of one entity — UX-PB.4d and UX-PB.1c write the draft's next state differently

**Stories:** UX-PB.4d (`epics.md:927`) ↔ UX-PB.1c (`epics.md:525`), with UX-PB.1a.

**ADs each obeys.** AD-16 routes Retry back through the draft:

> `ARCHITECTURE-SPINE.md:339-341` — "**Rule:** Retry always creates a new
> `planAttemptId`, links to the preceding failed attempt, and preserves the original
> failure. The backend rebuilds current intent rather than replaying historical
> executable text."
> `ARCHITECTURE-SPINE.md:401` (state diagram) — "Terminal --> Draft: Retry rebuilds
> current intent as a new linked attempt".

AD-17 says the draft is not empty when Retry arrives:

> `ARCHITECTURE-SPINE.md:460-463` — "While an attempt is non-terminal the region is
> owned by attempt status, and new membership staged during that attempt accumulates in
> the canonical draft without displacing it — surfacing in the region only once the
> attempt's Results are dismissed."

**The clash.** UX-PB.4d builds Retry as a draft author:

> `epics.md:937-940` — "**When** I invoke Retry **Then** it first reveals the proposed
> failed-item scope inline with `Cancel` and `Create new plan`; `Create new plan`
> rebuilds current canonical intent into a new reviewable draft, and confirming that
> draft creates a new attempt with a fresh `planAttemptId` linked by
> `retryOfPlanAttemptId`".

UX-PB.1c builds the convergence rule for that same draft:

> `epics.md:547-549` — "**Given** two entry classes mutating the same draft in close
> succession **When** both mutations resolve **Then** the draft converges to one
> coherent deduplicated membership set, no item is doubled or lost, and a single
> authenticated rebuild reflects the final canonical intent."

**Concrete incompatible artifacts.** Attempt A runs 12 items; while it runs the user
stages `mise:python` (AD-17:461 says it accumulates in the canonical draft). A ends with
2 failures. The user invokes Retry *from Results*, i.e. before dismissal.

- UX-PB.4d, honouring "re-run only what failed" (`epics.md:933`), **replaces** the draft:
  `PlanIntent{packageUpdates: [failed1, failed2]}`. `mise:python` is silently discarded —
  which breaks AD-17's promise at `:461` that accumulated membership surfaces "once the
  attempt's Results are dismissed."
- UX-PB.1c, honouring "no item is doubled or **lost**", **merges**:
  `PlanIntent{packageUpdates: [failed1, failed2, mise:python]}`. Now the user reviewed a
  Retry scope of 2 (`epics.md:942` — "**Given** Retry has exposed the failed-item scope")
  and gets a draft of 3, and the resulting attempt's `retryOfPlanAttemptId` lineage
  claims a package the original attempt never contained — contradicting `epics.md:946-948`
  ("no fabricated or repaired lineage is presented as valid").

The merge branch additionally trips AD-16's own no-enlargement rule
(`ARCHITECTURE-SPINE.md:299-300` — "it may never add a member the user has not seen"),
since the reviewed retry scope showed two items and the draft holds three.

**AD to add/tighten.** AD-16's Retry rule and AD-17's accumulation rule both define the
draft's post-Results state and disagree. One of them must state the tie-break: does Retry
replace, merge, or fail-closed against a non-empty accumulated draft? This seam is new in
revision 6 — before the draft was assumed durable, "accumulates during the attempt" and
"Retry rebuilds into a draft" had no reason to collide in one session.

---

## HIGH

### H-1. "Verified" means refresh-completion to UX-PB.3d and version-evidence to UX-PB.3c — same Operation, two durable states

**Stories:** UX-PB.3d (`epics.md:779`) ↔ UX-PB.3c (`epics.md:757`).

**AD each obeys.** AD-16 defines verification as *refresh completion*, attempt-wide:

> `ARCHITECTURE-SPINE.md:322-326` — "**Rule:** A mutating attempt is not successful until
> the required affected Manager refreshes complete. The attempt explicitly enters
> `Verifying`, and Results distinguish mutation failure from verification failure while
> preserving the Last-good Snapshot rules. \"Affected\" is the executor and the subject
> of each mutating operation in the attempt — the same set the scheduler locked."

**The clash.** UX-PB.3d terminalises on refresh completion:

> `epics.md:789-791` — "**Given** an active attempt whose mutations have all reached a
> process-terminal state **When** the required refresh verification for the affected
> Managers completes **Then** the attempt becomes terminal, the sidecar transforms in
> place into a persistent Results Summary".

UX-PB.3c requires per-item evidence:

> `epics.md:771-773` — "**Given** an item whose process has exited successfully **When**
> its affected Manager state has refreshed **and verified** **Then** only that verified
> row collapses its `old → new` delta to the single new current version, and an
> unverified successful exit remains `Verifying`."

**Concrete incompatible artifacts.** `brew upgrade node` exits 0 but the upgrade is a
no-op (a pin re-applied out of band, or brew reporting the same version). The affected
Manager refresh completes cleanly.

- UX-PB.3d: refresh completed → attempt terminal → journal record `state: "succeeded"`,
  Results announce `12 of 12 updates verified` (`epics.md:791`).
- UX-PB.3c: the row's `old → new` delta never collapses because the new current version
  equals the old → the item is stuck at `state: "verifying"` forever, with no rule to
  exit it. UX-PB.3d's own escape hatch at `epics.md:797-799` covers only the case where
  "the required refresh verification itself errors or times out" — here it did neither.

These are two different values in AD-18's durable journal for one `opId`
(`ARCHITECTURE-SPINE.md:516-518`) and two different History summaries under UX-PB.4a
(`epics.md:879` — "its summary uses verified-outcome wording such as `10 of 12 verified ·
2 failed`"). AD-4 pushes toward UX-PB.3c's reading — `ARCHITECTURE-SPINE.md:200-201`,
"The manager's own `outdated` verdict is the only authority on whether a package is
outdated" — but AD-16 never says verification *consults* that verdict; it says only that
the refresh must *complete*.

This is distinct from the closed finding about refresh *timing*: that one fixed when the
refresh may start. This one is about what the refresh has to *show* to count.

**AD to add/tighten.** AD-16:322 must state whether an item's verification is satisfied by
its Manager's refresh completing, or by that refresh's post-state contradicting the
pre-mutation `outdated` verdict for that subject — and what an exited-0-but-unchanged item
resolves to.

---

### H-2. Cancel arriving after process exit but before verification has no AD rule — UX-PB.3g and UX-PB.3d write opposite terminal states

**Stories:** UX-PB.3g (`epics.md:845`) ↔ UX-PB.3d (`epics.md:779`).

**AD each obeys.** AD-16's cancellation rule enumerates exactly two populations:

> `ARCHITECTURE-SPINE.md:318-321` — "**Rule:** Primary cancellation targets
> `planAttemptId`: unstarted work becomes `Skipped`, running process groups use the
> existing escalation, and every terminal state stays durable."

and closes `Skipped` off:

> `ARCHITECTURE-SPINE.md:337-338` — "`Skipped` marks only work that never started;
> crash-reconstructed unfinished work stays `Interrupted`."

Work that *exited successfully and is awaiting verification* is neither unstarted nor
running. AD-16 has no rule for it.

**The clash.** UX-PB.3g resolves it to cancellation:

> `epics.md:863-865` — "**Given** an attempt in the verifying window with processes exited
> and refresh verification pending (cancellation while verifying) **When** `Cancel plan`
> is issued **Then** cancellation is honored immediately for that `planAttemptId`,
> verifying items resolve to honest terminal outcomes (cancelled or skipped rather than
> falsely verified), and no item is reported successful because its exit preceded the
> cancel."

UX-PB.3d resolves the identical state to verification failure:

> `epics.md:797-799` — "**Then** the item does not declare success — it stays `Verifying`
> until it resolves, then reports verification failure with its evidence, and is never
> colored successful on the strength of the exit code alone."

**Concrete incompatible artifacts.** `npm install -g typescript@latest` exited 0; the
package *is* upgraded on disk; the user hits `Cancel plan` during the verify window.

- UX-PB.3g journals `{opId, planAttemptId, state: "cancelled"}` — durably asserting the
  mutation did not happen, when it did. `Skipped` is explicitly unavailable per `:337`,
  yet `epics.md:865` offers it as an option, so UX-PB.3g's own AC contradicts AD-16.
- UX-PB.3d journals `{state: "failed", failureKind: "verification"}` — durably asserting
  a verification failure that never ran.

Neither is honest, and they disagree. Worse, the two stories will also disagree on
whether the in-flight verification refresh is *aborted* by "cancellation is honored
immediately": abort it and the app's displayed inventory is knowingly stale after a real
mutation, with AD-16:322 ("not successful until the required affected Manager refreshes
complete") never satisfied and no story owning the recovery.

**AD to add/tighten.** AD-16's cancellation rule needs a third population — *mutation
complete, verification pending* — with a named durable state and a statement on whether
cancel aborts or lets the verification refresh drain.

---

### H-3. AD-18 never says whether one attempt is one journal append or a sequence — UX-PB.2c and UX-PB.4a write different files

**Stories:** UX-PB.2c (`epics.md:636`) ↔ UX-PB.4a (`epics.md:867`).

**AD each obeys.** AD-18 fixes home, discipline, and failure mode but not cardinality:

> `ARCHITECTURE-SPINE.md:495-498` — "**Rule:** Confirmed attempts persist to their own
> append-only NDJSON journal in the same Application Support directory as
> `operations.jsonl`, under the same discipline: an append failure is nonfatal to package
> operations, and compaction is temp file + fsync + rename, never truncate-in-place."

AD-19 forbids the obvious reconciliation shortcut:

> `ARCHITECTURE-SPINE.md:529-532` — "an unparseable line is skipped and counted, the
> surrounding records stay readable, and a plan-attempt or Operation record is never
> silently replaced by a synthesized one."

**The clash.** UX-PB.2c writes at admission:

> `epics.md:646-649` — "**Given** a plan admitted under a new `planAttemptId` **When** the
> attempt is persisted **Then** the append-only record stores the reviewed Manager/Package
> scope, Manager self-update identities, exact command snapshot, version evidence,
> timestamps, and result/verification state as immutable plan-admission metadata".

(Note the internal strain: at admission there is no "result/verification state" to store.)

UX-PB.4a writes at terminal:

> `epics.md:877-880` — "**When** it terminates **Then** exactly one immutable History row
> is created for that `planAttemptId` ... **And** no attempt ever yields more than one row
> or a per-Package or per-command row."

**Concrete incompatible artifacts.** Both stories are literal about append-only:

- Two NDJSON lines for one `planAttemptId` — `{planAttemptId, reviewedIntent, resultSummary: null}`
  at admission and `{planAttemptId, resultSummary: {...}}` at terminal. Nothing in AD-18
  or AD-19 defines last-writer-wins over the attempt journal, and AD-19:531 arguably
  forbids the reader collapsing them into a synthesized record. UX-PB.4a's "exactly one
  immutable History row" reader now has two candidate rows and no precedence rule.
- Or UX-PB.2c defers the whole write to terminal (a defensible reading of "no partial
  attempt record is left behind", `epics.md:653`) — and then UX-PB.4a's crash path has
  nothing to read: `epics.md:886-888` — "**Given** a confirmed attempt was admitted but
  the app crashed or relaunched before the attempt reached a terminal row **When** History
  reconciles on the next launch **Then** the in-flight attempt is reconciled from its
  durable `planAttemptId` records into one honest row". Every crashed attempt becomes
  invisible, which is the exact failure AD-19:532-533 calls out ("Losing history quietly
  is worse than surfacing that some of it is unreadable").

AD-18's compaction rule makes it worse: `ARCHITECTURE-SPINE.md:511-515` — "Compacting the
Operation journal may not orphan an attempt whose Operations it drops, and compacting the
attempt journal may not leave Operations pointing at an attempt that no longer resolves."
If an attempt is two lines, compaction can drop the admission line and keep the terminal
one — the reviewed intent and command snapshot vanish while the History row survives, and
UX-PB.4b's replay (`epics.md:900-903`, "reconstructs the attempt's Manager groups,
Package/version changes, Manager self-updates, exact commands") has nothing to replay.

**AD to add/tighten.** AD-18 must state the attempt record's cardinality (one terminal
append vs. start+finish appends), the precedence rule when more than one line carries a
`planAttemptId`, and that compaction operates on the *attempt*, never on individual lines
of one attempt.

---

### H-4. `Cancelling` and `waiting` are wire states no AD blesses, and AD-16's "one atomic contract change" is split across two stories in different waves

**Stories:** UX-PB.2e (`epics.md:683`) ↔ UX-PB.3c (`epics.md:757`).

**AD each obeys.** AD-16 blesses exactly two new operation states and requires them to
land together:

> `ARCHITECTURE-SPINE.md:333-338` — "**Rule:** `Verifying` and `Skipped` are durable
> wire-level operation states, not presentation states derived in React. They are
> journaled, exported in diagnostics, and replayed from History ... **Adding them is one
> atomic contract change under AD-3.**"

AD-3 makes the fixture set and the guard map exactly equal:

> `ARCHITECTURE-SPINE.md:152-154` — "`src-tauri/src/ipc.rs` byte-compares each serialized
> model against its committed fixture, and the TypeScript half asserts its fixture set
> exactly equals its guard map."

**The clash.** Shipping today (`src-tauri/src/ipc.rs:99-107`) is
`OpStatus { Queued, Running, Succeeded, Failed, Cancelled, TimedOut, Interrupted }`.
The stories demand four additions, not two:

- UX-PB.2e: `epics.md:695` — "running work moves to `Cancelling` and escalates through
  the existing process-group mechanics, unstarted attempt work is prevented from beginning
  and recorded as `Skipped`". So UX-PB.2e needs `Skipped` **and** `Cancelling`.
- UX-PB.3c: `epics.md:769` — "**Then** it shows queued, **waiting** (with the lock or
  ownership reason), running (indeterminate unless the adapter provides a trustworthy
  total), **verifying**, or a terminal state". So UX-PB.3c needs `Verifying` **and**
  `waiting` — and its enumeration omits `Cancelling` entirely.
- UX-PB.3g repeats `Cancelling`: `epics.md:857` — "changes still-running Operations bound
  to that `planAttemptId` to `Cancelling`".

**Concrete incompatible artifacts.** UX-PB.2e depends on UX-PB.2b (`epics.md:686`);
UX-PB.3c depends on UX-PB.3b/UX-PB.2d (`epics.md:760`) — different waves, so they ship at
different times.

- UX-PB.2e regenerates `dev/fixtures/ipc/*.json` with `OpStatus` gaining `skipped` and
  `cancelling`. UX-PB.3c later regenerates it again adding `verifying`. That is **two**
  atomic contract changes, contradicting AD-16:336.
- Or whichever ships first adds all four so the change is atomic — and then ships a wire
  state it does not implement, whose TypeScript guard has no producer, which the other
  half of AD-3's assertion (`fixture set exactly equals its guard map`) will still pass
  but which UX-PB.3c's renderer at `epics.md:769` has no case for, because `Cancelling`
  is not in its enumeration.
- `waiting` is a fifth question: AD-16 blesses only `Verifying`/`Skipped` as durable, so
  UX-PB.3c may derive `waiting` in React — but its content is "the lock or ownership
  reason", which lives behind AD-4's coordinator lock (`ARCHITECTURE-SPINE.md:197-198`)
  and cannot be derived frontend-side.

**AD to add/tighten.** AD-16:333 must name the complete set of new operation states
(`Verifying`, `Skipped`, `Cancelling`, and whether `waiting` is durable or a
`Queued`+reason projection) and assign the single atomic AD-3 change to exactly one story.

---

### H-5. "Legacy" is defined by field-absence in UX-PB.2f and by unresolvable-id in AD-18 — the orphan case fabricates a History row

**Stories:** UX-PB.2f (`epics.md:703`) ↔ UX-PB.4e (`epics.md:950`) / Story 6.5 (`epics.md:1225`).

**AD each obeys.** AD-18's retention rule creates orphans and names their reading:

> `ARCHITECTURE-SPINE.md:511-515` — "**Rule:** The two journals share a retention policy.
> Compacting the Operation journal may not orphan an attempt whose Operations it drops,
> and compacting the attempt journal may not leave Operations pointing at an attempt that
> no longer resolves. A record that loses its counterpart reads as legacy, never as
> corrupt."

AD-18 also anchors legacy to field-absence one rule earlier:

> `ARCHITECTURE-SPINE.md:499-501` — "`operations.jsonl` keeps its record shape and carries
> `planAttemptId` only where one exists. A record without one stays an individually
> labeled legacy Operation."

**The clash.** UX-PB.2f implements the field-absence predicate:

> `epics.md:713-715` — "**Given** Operation records that have **no** `planAttemptId`
> **When** they are read and displayed **Then** they remain honest legacy Operation
> entries, stay readable, and are never silently grouped or inferred into a plan attempt."

UX-PB.4e implements the same predicate in History:

> `epics.md:959-961` — "**Given** legacy Operation History records that **lack** a
> `planAttemptId` **When** History renders them **Then** they remain accessible, are
> explicitly labeled as legacy Operation entries".

**Concrete incompatible artifacts.** After compaction, an Operation record carries
`planAttemptId: "pa_7f3"` whose attempt record no longer resolves. AD-18:514 requires it
to "read as legacy". Both stories' predicate is `record.planAttemptId == null`, which is
false here, so:

- UX-PB.2f's reader classifies it as plan-attempt work and hands it to the plan grouping.
- UX-PB.4a must then produce "exactly one immutable History row ... for that
  `planAttemptId`" (`epics.md:879`) from an attempt record that does not exist — a
  fabricated row, which is precisely what `epics.md:961` forbids ("never grouped or
  fabricated into a plan attempt they never belonged to").
- UX-PB.4b's replay opens it and hits `epics.md:905-907` ("**Given** a History row whose
  persisted attempt is corrupted or missing"), presenting a retention artifact as
  corruption — which AD-18:514 explicitly forbids ("never as corrupt").
- Story 6.5 exports both journals and asserts exact contents at the 1,000-record boundary
  — `epics.md:1243-1245`, "**Given** more than three app logs, 25 transcripts, 1,000
  journal records, and durable plan-attempt records correlated by `planAttemptId` ...
  with exact expected contents and no missing required entry" — so the boundary where
  orphans are manufactured is the one Story 6.5 pins.

**AD to add/tighten.** AD-18 must state the legacy predicate once, as
`planAttemptId is absent OR does not resolve`, and say which side of the retention pair
gives way when the bound would orphan (raise the Operation-journal bound, or drop the
Operations with the attempt).

---

## MEDIUM

### M-1. The `IN PLAN` badge lives outside the region AD-17 gates — UX-PB.1e advertises a draft the user cannot reach

**Stories:** UX-PB.1e (`epics.md:574`) ↔ UX-PB.3a (`epics.md:721`); Story 3.1 (`epics.md:1127`) is the third writer.

AD-17 hides accumulated draft membership during an attempt:

> `ARCHITECTURE-SPINE.md:460-463` — "While an attempt is non-terminal the region is owned
> by attempt status, and new membership staged during that attempt accumulates in the
> canonical draft without displacing it — surfacing in the region only once the attempt's
> Results are dismissed."

UX-PB.1e puts a draft-membership indicator on the Manager Header, outside that region:

> `epics.md:589-591` — "**Given** a Manager whose self-update has been staged into the
> plan **When** the Manager Header renders **Then** it shows `IN PLAN` plus a separate
> visible `Remove` action named `Remove <Manager> update from Upgrade Plan`".

Story 3.1 duplicates it: `epics.md:1148` — "`Update Manager` stages an independent,
individually-removable self-update plan item surfaced as `IN PLAN` / `Remove`".

**Concrete incompatible artifacts.** During attempt A, the user invokes `Update Manager`
on brew. AD-16 rule 1 requires it to mutate the draft and return; AD-17:461 requires that
membership to stay hidden. The header nonetheless renders `IN PLAN` + `Remove`, pointing
at a plan with no visible surface — and if brew's self-update is *also* running inside
attempt A, `IN PLAN` is now ambiguous between "queued in the invisible draft" and
"executing in the visible attempt", with `Remove` targeting the former while the user
reads it as the latter. UX-PB.3c owns per-item live state inside the region
(`epics.md:767-769`); UX-PB.1e owns the header badge; neither AD says which source
`IN PLAN` derives from.

**AD to add/tighten.** AD-17 must state that out-of-region draft affordances follow the
same visibility gate as the region — or that the header badge derives from the union of
draft membership and active-attempt membership, with distinct labels.

---

### M-2. The single announcement channel has no live/replay scoping — UX-PB.4b narrates a past attempt over a running one

**Stories:** UX-PB.4b (`epics.md:890`) ↔ UX-PB.4c (`epics.md:909`).

> `ARCHITECTURE-SPINE.md:484-487` — "**Rule:** There is exactly one polite
> status-announcement channel for plan and attempt progress, owned alongside the sidecar
> region. Stories announce through it; none adds a second live region for the same
> information. Two live regions narrating one attempt is a defect, not additive coverage."

UX-PB.4b reconstructs full attempt evidence in replay (`epics.md:900-903`), and UX-PB.4c
requires the live attempt to stay authoritative:

> `epics.md:923-925` — "**Given** a replay is open alongside the live attempt **When** the
> live attempt emits new status or reaches terminal Results **Then** the live attempt
> remains the primary object with authoritative sidecar and Results, and the concurrent
> replay never suppresses, delays, or overwrites live status."

**Concrete incompatible artifacts.** A single `aria-live="polite"` node is a
last-write-wins queue. If UX-PB.4b announces replay state through it, opening a History
row mid-upgrade replaces the pending live announcement — VoiceOver reads the *past*
attempt's `10 of 12 verified` while the live attempt is still running, violating
`epics.md:925`. If UX-PB.4c gives replay its own region to protect live status, it adds
the second live region AD-17:486 forbids. AD-17 never scopes the channel to the live
attempt, and never says replay must be silent.

**AD to add/tighten.** AD-17:484 should state that the single channel is reserved for the
live attempt and the draft, and that read-only replay announces nothing through it.

---

### M-3. The 720px boundary is a remount hazard the "single instance" rule does not cover

**Stories:** UX-PB.5d (`epics.md:1049`) ↔ UX-PB.1b (`epics.md:499`) / UX-PB.3c (`epics.md:757`).

> `ARCHITECTURE-SPINE.md:472-474` — "**Rule:** Below 720 usable CSS pixels the region stops
> being a fixed sidecar and the same single instance is presented as a full-workspace or
> stacked surface. Viewport is a placement driver, never a second mount point."

The persistence guarantee is scoped to view changes only:

> `ARCHITECTURE-SPINE.md:465-466` — "Exactly one instance exists and it persists across
> `ActiveView` changes without losing membership or scroll identity."

UX-PB.5d builds the crossing:

> `epics.md:1062-1064` — "**Then** below 720 usable CSS pixels the layout enters high-zoom
> mode, navigation collapses to an accessible rail or temporary panel, and
> Plan/Confirmation/Activity/Results present as a full-workspace or stacked surface".

**Concrete incompatible artifacts.** "Never a second mount point" forbids two
simultaneous instances; it does not forbid unmounting one and mounting the other on
resize. UX-PB.5d implemented as a conditional render (`isNarrow ? <StackedRegion/> :
<SidebarRegion/>`) satisfies AD-17 literally and destroys scroll identity, focus, and the
`aria-live` node on every zoom change — breaking UX-PB.3c's "a row or status update never
moves focus" (`epics.md:769`) and dropping in-flight announcements (M-2's channel is
"owned alongside the sidecar region", AD-17:485). UX-PB.1b implemented as CSS-only
reflow keeps them. Both are spine-legal.

**AD to add/tighten.** Extend AD-17:465 to "persists across `ActiveView` **and viewport**
changes without losing membership, scroll identity, focus, or the announcement channel."

---

### M-4. AD-4's coordinator-first rule fixes lock order but not lock lifetime — Story 2.2's refresh can freeze every UX-PB draft mutation

**Stories:** Story 2.2 (`epics.md:1099`) ↔ UX-PB.1a (`epics.md:468`).

> `ARCHITECTURE-SPINE.md:197-199` — "Any code reading plan-relevant state — `detection`,
> `registry`, `queue.records()`, `settings`, `tool_env` — must already hold
> `state.plan_coordinator`. Taking those locks without it yields a mixed-time collection
> instead of one canonical epoch."

Story 2.2 exercises long-running refreshes under controlled time:

> `epics.md:1118-1121` — "**Given** each of the six Manager adapters and its documented
> timeout boundary **When** controlled time reaches success, timeout, or error outcomes
> **Then** the correct Manager-specific terminal state and actionable detail appear
> **And** peers continue independently".

UX-PB.1a requires a Rust rebuild on *every* draft mutation:

> `epics.md:478-480` — "**Then** the Package's canonical identity is added to the one
> persistent draft Upgrade Plan, nothing executes, and Rust rebuilds the exact command
> from canonical intent."

**Concrete incompatible artifacts.** Story 2.2's refresh reads `detection` and `registry`
and must hold `plan_coordinator` to do so (AD-4:198). If it holds it across the adapter
I/O — the natural reading of "one canonical epoch" — then every checkbox toggle blocks for
up to the documented per-Manager timeout, because each rebuild also needs the coordinator.
If it snapshots and releases, the rebuild it feeds is a mixed-time collection, which
AD-4:199 names as the failure mode. The two stories will pick different lifetimes and
their integration deadlocks or stutters. AD-16's admission gate also rejects "on ... an
active refresh" (`ARCHITECTURE-SPINE.md:313`), so a long refresh additionally blocks
UX-PB.5c's `Run N Updates` with no story owning the user-facing wait.

**AD to add/tighten.** AD-4 should state the coordinator's lock lifetime: acquired to take
a snapshot and released before adapter I/O, with the epoch carried as a value.

---

## LOW

### L-1. AD-16's re-seed offer has no story owner, so the enlargement case falls between UX-PB.1c and UX-PB.5c

**Stories:** UX-PB.1c (`epics.md:525`) ↔ UX-PB.5c (`epics.md:1027`).

> `ARCHITECTURE-SPINE.md:299-303` — "newly eligible work discovered later surfaces as an
> explicit offer to re-seed, never as silent membership. If a rebuild would enlarge
> membership, the preview `planId` expires and re-review is required. This holds
> identically on the confirmation-off path, which otherwise has no moment at which the
> user could see the addition."

`grep -n "re-seed\|reseed" epics.md` returns **no matches** — the offer exists only in the
spine. UX-PB.5c's rebuild-failure AC enumerates only shrink cases:

> `epics.md:1045-1047` — "**When** native rebuild or stale validation fails, for example a
> Package pinned, updated, or removed since staging **Then** the run is blocked, the
> invalidated details are replaced and what changed is explained".

**Concrete incompatible artifacts.** With confirmation off and an `AllEligible` draft, a
rebuild that finds newly eligible work must "expire the preview `planId` and require
re-review" — but UX-PB.5c builds a path where "no dialog opens" (`epics.md:1039`) and its
only blocking branch is for invalidation. UX-PB.1c owns the seeding but builds no
re-seed affordance. The enlargement case lands in neither story, and each will improvise
differently (silent re-seed, which AD-16:300 forbids; or a hard block with no offer).

**AD to add/tighten.** Name the story that owns the re-seed offer surface, or drop the
offer from AD-16 and require a hard block.

---

### L-2. An integrity-failed-but-parseable record is a third AD-19 category

**Stories:** UX-PB.2c (`epics.md:636`) ↔ UX-PB.4a (`epics.md:867`) / UX-PB.4b (`epics.md:890`).

> `ARCHITECTURE-SPINE.md:527-533` — "A journal must never be defaulted away: an
> unparseable line is skipped and counted, the surrounding records stay readable, and a
> plan-attempt or Operation record is never silently replaced by a synthesized one.
> Losing history quietly is worse than surfacing that some of it is unreadable."

UX-PB.2c requires detection of a *parseable* record that fails integrity:

> `epics.md:659-661` — "**Given** a persisted attempt whose command snapshot is later read
> as corrupted or incomplete **When** the record is loaded **Then** the integrity failure
> is detected and the snapshot is refused as an execution source".

**Concrete incompatible artifacts.** AD-19 covers only *unparseable* → skip and count. A
record that deserialises cleanly but fails an integrity check is a third category. UX-PB.2c
refuses it; UX-PB.4a must still yield "exactly one immutable History row" per attempt
(`epics.md:879`). Skipping it (AD-19's stated handling) yields **zero** rows and loses
history quietly, which AD-19:532 calls the worse outcome; keeping it yields a row whose
replay UX-PB.4b must refuse (`epics.md:905-907`). AD-19 gives contradictory pull and the
two stories can land on opposite sides. AD-19 also never says where the skip *count*
surfaces, though UX-PB.4a, UX-PB.4e, and Story 6.5 each need it.

**AD to add/tighten.** Extend AD-19:529 to three categories — unparseable, parseable but
integrity-failed, and valid — and state that an integrity-failed attempt still yields one
History row marked unreadable, plus where the skip count is surfaced.

---

## Summary table

| # | Sev | Stories | Spine seam | One-line |
| --- | --- | --- | --- | --- |
| C-1 | CRITICAL | UX-PB.5b ↔ UX-PB.2b | AD-19:537 × AD-16:313 | Settings write inside the confirm action advances the revision the admission rejects on |
| C-2 | CRITICAL | UX-PB.1a ↔ UX-PB.1c | AD-16:363-366 × AD-16:408 | One `kind` scalar cannot carry the per-member provenance AD-16 mandates |
| C-3 | CRITICAL | UX-PB.4d ↔ UX-PB.1c | AD-16:339 × AD-17:461 | Retry and accumulated-draft rules both define the draft's next state |
| H-1 | HIGH | UX-PB.3d ↔ UX-PB.3c | AD-16:322 | "Verified" = refresh completed vs. version evidence changed |
| H-2 | HIGH | UX-PB.3g ↔ UX-PB.3d | AD-16:318 | Cancel after exit, before verification, has no AD population |
| H-3 | HIGH | UX-PB.2c ↔ UX-PB.4a | AD-18:495 | One attempt = one append or a sequence is undefined |
| H-4 | HIGH | UX-PB.2e ↔ UX-PB.3c | AD-16:333 × AD-3:152 | `Cancelling`/`waiting` unblessed; the "one atomic change" spans two waves |
| H-5 | HIGH | UX-PB.2f ↔ Story 6.5 / UX-PB.4e | AD-18:511 | Legacy predicate is field-absence, not unresolvable-id; orphans fabricate rows |
| M-1 | MEDIUM | UX-PB.1e ↔ UX-PB.3a | AD-17:460 | `IN PLAN` header badge escapes the region's visibility gate |
| M-2 | MEDIUM | UX-PB.4b ↔ UX-PB.4c | AD-17:484 | The one announcement channel has no live/replay scoping |
| M-3 | MEDIUM | UX-PB.5d ↔ UX-PB.1b | AD-17:472 | "Never a second mount point" does not forbid a remount at 720px |
| M-4 | MEDIUM | Story 2.2 ↔ UX-PB.1a | AD-4:197 | Coordinator lock order is fixed; lock lifetime is not |
| L-1 | LOW | UX-PB.1c ↔ UX-PB.5c | AD-16:299 | The re-seed offer exists only in the spine; no story owns it |
| L-2 | LOW | UX-PB.2c ↔ UX-PB.4a | AD-19:529 | Parseable-but-integrity-failed is a third category AD-19 omits |
