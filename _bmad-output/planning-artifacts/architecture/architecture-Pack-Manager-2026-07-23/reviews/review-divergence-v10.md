# Reviewer Gate — adversarial divergence lens, v10

**Target:** `_bmad-output/planning-artifacts/architecture/architecture-Pack-Manager-2026-07-23/ARCHITECTURE-SPINE.md`, revision 9.
**Intent:** Validate. Report only. No file outside this review was modified.
**Level below:** stories, in `_bmad-output/planning-artifacts/epics.md`.
**Date:** 2026-07-25.

## Method and counts

Every count below comes from a command run this session, not an estimate.

| Fact | Command | Result |
| --- | --- | --- |
| Spine length | `wc -l ARCHITECTURE-SPINE.md` | `1052` |
| `epics.md` length | `wc -l epics.md` | `1319` |
| Live `AD` headings | `grep -c '^### AD-' ARCHITECTURE-SPINE.md` | `19` |
| `AD-27` mentions in `epics.md` | `grep -c 'AD-27' epics.md` | `32` |
| Stories in `epics.md` | `grep -c '^### Story ' epics.md` | `34` |
| `OpStatus` variants shipping | `sed -n '99,107p' src-tauri/src/ipc.rs \| grep -c '^    [A-Z]'` | `7` |
| `ErrorCode` variants shipping | `sed -n '68,81p' src-tauri/src/error.rs \| grep -c '^    [A-Z]'` | `12` |

Files read in full or in the cited ranges this session: `ARCHITECTURE-SPINE.md` (all
1053 lines), `epics.md` (all 1319), `src-tauri/src/process/runner.rs:288-332`. Files
quoted from line-numbered `grep`/`sed` output this session: `docs/SPEC.md`,
`_bmad-output/planning-artifacts/ux-designs/ux-Pack-Manager-2026-07-23/EXPERIENCE.md`,
`src-tauri/src/error.rs`, `src-tauri/src/ipc.rs`, `src-tauri/src/queue.rs`.
Prior lenses read for overlap: `reviews/review-divergence-v6.md`,
`review-divergence-v8.md`, `review-divergence-v9.md` (finding indexes plus the
specific findings my candidates touched: v6 H-1/H-3, v8 V-4/V-6/N-3, v9 C-6/H-2/H-3/H-5).

## Verdict

**The spine holds against every attack I could mount on its *stated* rules. It falls
open on the boundary between what a story writes and what another story reads.**

Eight of my candidate pairs died on contact with a source — `EXPERIENCE.md` carries a
Focus-transitions matrix (lines 290-302) and an explicit outcome taxonomy (226-227)
that the spine never imports but which do arbitrate. That is itself the pattern: the
19 live ADs legislate the *operation*-level enum, the draft's author, the journal's
home, and the focus *mechanism*, while the attempt-level enum, the record's field
list, the rejection payload, the verification deadline, and focus *destination* are
each governed only by a UX source with no stated precedence — and `epics.md:46` says
"Where this document and the spine disagree, **the spine is upstream and wins**",
which for a silent spine means the story text wins by default.

One CRITICAL: a confirmed attempt's admission record failing to write has two legal
outcomes, and one of them runs real `brew upgrade` commands with no durable record.

**11 findings: 1 CRITICAL, 5 HIGH, 3 MEDIUM, 2 LOW, plus 2 sharpenings of tracked
Open rows.** No finding re-reports a tracked row; REPEAT adjacency is flagged inline.

---

## CRITICAL

### C-1 — AD-18's "nonfatal append failure" and AD-16's "atomic all-or-none" give opposite answers to "does the attempt run when its record cannot be written?"

**Pair: UX-PB.2b ↔ UX-PB.2c.** Both obey every live AD. One runs the upgrade; the
other refuses it.

AD-18's first rule, `ARCHITECTURE-SPINE.md:626-629`:

> "Confirmed attempts persist to their own append-only NDJSON journal in the same
> Application Support directory as `operations.jsonl`, under the same discipline: **an
> append failure is nonfatal to package operations**, and compaction is temp file +
> fsync + rename, never truncate-in-place."

AD-16's admission rule, `ARCHITECTURE-SPINE.md:387-389`:

> "Admission is atomic and all-or-none. The complete derived operation set enters the
> scheduler together or nothing does, so a confirmed attempt's membership can never
> silently differ from the reviewed intent."

**UX-PB.2b's builder** reads `epics.md:673-674`:

> "**Then** `execute_plan` atomically returns one new durable `planAttemptId` plus the
> created Operation identities
> **And** the full plan is admitted as a unit with no partial silent admission."

Admission returns the ids; the scheduler has them; the processes spawn. Combined with
AD-18's "an append failure is nonfatal to package operations", a journal write that
fails at admission is a logged warning. The plan runs.

**UX-PB.2c's builder** reads `epics.md:696-698`:

> "**Given** a plan admitted under a new `planAttemptId`
> **When** persisting the reviewed intent or command snapshot fails
> **Then** the failure is surfaced, no partial attempt record is left behind, and **the
> prior consistent state is preserved rather than proceeding as if durably recorded**."

"Rather than proceeding" is the operative clause. Its builder aborts the admission and
restores the draft — AD-17 gives it the mechanism, `ARCHITECTURE-SPINE.md:563-566`: "a
failed or rejected admission restores it unchanged."

**Why no AD arbitrates.** AD-18's "nonfatal" clause is inherited from the Operation
journal, where it is correct: a `brew upgrade` should not die because a log line
failed. Applied to the *admission* record it licenses the opposite of what the whole
attempt model exists for. Nothing in AD-16, AD-18, or AD-19 distinguishes "the journal
that records an operation" from "the journal that *is* the attempt's identity". And
`epics.md:109` (NFR-4) shows the project has already decided this exact question the
other way for the adjacent artifact:

> "block spawn when transcript creation fails; and keep later noncritical logging
> failures from hanging Package work"

That is a two-tier rule — creation blocks, later appends do not — and AD-18 has only
the second tier.

**The concrete incompatible artifacts.** Disk full, or Application Support read-only
after a permissions change. UX-PB.2b's build: six `brew upgrade` / `npm install -g`
processes execute, mutating the user's machine, with no admission record. UX-PB.4a
then has an attempt id it can never resolve — `epics.md:925`: "exactly one immutable
History row is created for that `planAttemptId`" — and `epics.md:936-939`'s new fold
rule cannot classify it, because that rule's `Given` is "an attempt whose admission
record is **present**". UX-PB.2c's build: nothing ran, the draft is intact, the user
sees an error. Same fault, opposite machine state.

**Hole to close.** AD-18 needs the two tiers NFR-4 already has: the admission record's
creation is a **precondition of admission** and its failure aborts the attempt
(all-or-none under AD-16 includes the record); every later append is nonfatal. State
which side of the line the terminal record is on.

**Adjacency, not a repeat:** `review-divergence-v9.md` C-6 quoted UX-PB.2c's "no
partial attempt record is left behind" as evidence for *three writers*, and the tracked
Open row "Plan-attempt journal: writer identity and record cardinality" inherits that
framing. Neither asks whether a failed write stops the run. Commit `5972109` resolved
the writer question inside `epics.md` and left this one untouched.

---

## HIGH

### H-1 — The attempt-level outcome enum has no owner: UX-PB.4a writes five values, UX-PB.3d reads six, and `timedOut` is unrepresentable in the record

**Pair: UX-PB.4a ↔ UX-PB.3d.** The spine enumerates the *operation*-level enum and
legislates how it lands atomically. It says nothing about the *attempt*-level one, and
the two stories on either side of that record enumerate different closed sets.

The spine's schema, `ARCHITECTURE-SPINE.md:477-483`:

```text
PlanAttempt
  planAttemptId: durable PlanAttemptId
  retryOfPlanAttemptId?: PlanAttemptId
  reviewedIntent + reviewedCommandSnapshot
  operationIds[]
  state: admitted | running | verifying | terminal
  verificationResults + resultSummary
```

`state` collapses every outcome into `terminal`; `resultSummary` has no shape. Contrast
what AD-16 does for the operation enum, `ARCHITECTURE-SPINE.md:422-424`:

> "`OpStatus` ships seven variants today, so every addition moves as one atomic AD-3
> change across the Rust enum, `src/lib/ipc/types.ts`, the guards, and
> `dev/fixtures/ipc/*.json`."

Verified: `grep -c` over `src-tauri/src/ipc.rs:99-107` returns `7`. The asymmetry is
the hole — the enum the spine enumerates is disciplined; the one it does not is free.

**UX-PB.3d, the reader**, `epics.md:840`:

> "**Then** the overall outcome is exactly one of **success, partial, failed,
> cancelled, timed out, or interrupted**, and each item is verified, failed, cancelled,
> or skipped"

**UX-PB.4a, the writer**, `epics.md:923`:

> "**Given** a confirmed plan attempt that reaches a terminal state — **succeeded,
> failed, cancelled, interrupted, or partially skipped**, and regardless of how many
> Managers, commands, Packages, failures, or skips it contained"

Three concrete divergences in one durable field:

1. **`timed out` exists in the reader's set and not the writer's.** This is not
   cosmetic. `docs/SPEC.md:112` ships `upgradeHardCapMins` (30);
   `EXPERIENCE.md:218` — "At the 30-minute hard cap, the Operation ends as `Timed
   out`"; and `src-tauri/src/ipc.rs:105` ships `    TimedOut,` today. An attempt that
   hits the hard cap has no value to write in UX-PB.4a's set, so its builder either
   invents an eighth or maps it onto `failed` — and UX-PB.3d's builder renders a
   `timed out` branch nothing can reach.
2. **`partial` vs `partially skipped`** are different concepts. `partial` covers "10 of
   12 verified · 2 failed" (UX-PB.3d's own example at `epics.md:836`);
   `partially skipped` covers cancellation leftovers. One writer value cannot serve
   both readers.
3. **`success` vs `succeeded`.** AD-3 makes this load-bearing, not stylistic —
   `ARCHITECTURE-SPINE.md:207-209`: "Every IPC enum declares its wire casing explicitly
   with `#[serde(rename_all = ...)]`, and each existing spelling is preserved."

**Who is right does not matter; that only a UX source says so does.**
`EXPERIENCE.md:226-227` resolves it:

> "- Overall states: success, partial, failed, cancelled, timed out, interrupted."
> "- Per-item states: verified, failed, cancelled, skipped."

So UX-PB.3d is faithful and UX-PB.4a is not. But UX-PB.4a is the story that *writes the
record* (`epics.md:849`: "exactly one **terminal** record exists per `planAttemptId` and
UX-PB.4a writes it"), a builder reading UX-PB.4a alone never sees `EXPERIENCE.md:226`,
and `epics.md:46` tells them the spine wins over `epics.md` — which is silent.

**Hole to close.** AD-16 (or the Normative domain minimum) must enumerate the
attempt-level outcome set with the same words `EXPERIENCE.md:226` uses, declare its
wire casing, and give it AD-23's tie-break sentence: "whichever runs first lands the
complete shape and the other builds against it" (`ARCHITECTURE-SPINE.md:792-793`).

**Adjacency:** `review-divergence-v9.md` H-5 and `review-divergence-v6.md` H-1 both
attack the *operation*-level enum (`waiting`, `Verifying`, `verified`). Neither touches
the attempt-level `resultSummary`. Distinct.

---

### H-2 — The spine delegates the attempt record's field list to UX-PB.2c, whose criterion omits the one field Story 6.5 asserts and `docs/SPEC.md` requires

**Pair: UX-PB.2c ↔ Story 6.5** (UX-PB.4d is the third party).

The spine hands the field list to one story by name, `ARCHITECTURE-SPINE.md:1037`:

> "| Plan-attempt file name and serde shape | **Deferred** | AD-18 fixes ownership,
> location, durability, and failure mode; **the exact filename and field list belong to
> UX-PB.2c**. |"

**UX-PB.2c's field list**, `epics.md:693`:

> "**Then** the append-only record stores the reviewed Manager/Package scope, Manager
> self-update identities, exact command snapshot, version evidence, timestamps, and
> result/verification state as immutable plan-admission metadata"

No lineage field. **Story 6.5 asserts the export contains one**, `epics.md:1314`:

> "**Then** it contains `report.json`, the newest three app logs, newest 25 transcripts,
> `operations.jsonl`, and the durable plan-attempt records that correlate the exported
> evidence — each carrying its `planAttemptId`, reviewed Manager/Package scope, exact
> commands, verification facts, results, and **optional `retryOfPlanAttemptId`** — with
> exact expected contents and **no missing required entry**"

**UX-PB.4d mints it**, `epics.md:990`: "creates a new attempt with a fresh
`planAttemptId` linked by `retryOfPlanAttemptId`". And `docs/SPEC.md:99` requires it:

> "Every confirmed plan also persists one immutable attempt record containing reviewed
> scope, command snapshot, `planAttemptId`, Operation identities, verification results,
> terminal Results, and **optional retry lineage**."

The spine's own schema has it too (`ARCHITECTURE-SPINE.md:479`:
`retryOfPlanAttemptId?: PlanAttemptId`) — but the Normative domain minimum opens with
`ARCHITECTURE-SPINE.md:450-451`: "Names may be refined during story implementation; the
semantic separation is fixed", which a builder can read as licence to drop an optional
field, and the Decision Status row above says the field list is UX-PB.2c's call.

**Why this is a build divergence, not a typo.** `retryOfPlanAttemptId` is known **at
mint** — UX-PB.4d confirms a derived intent and the link exists before the first
process spawns. So it belongs in the *admission* record UX-PB.2c writes, not in a later
append. UX-PB.2c's builder, working from its enumerated list, ships a schema with no
slot for it. UX-PB.4d's builder then has a link with nowhere to put it and adds a field
to a persisted schema someone else owns — which AD-18 charges to it,
`ARCHITECTURE-SPINE.md:640-641`: "A story that adds a field to the attempt record owns
its disclosure review." Story 6.5's assertion "no missing required entry" fails against
the first build and passes against the second.

**Hole to close.** Either AD-18 states the attempt record's required field set
(planAttemptId, reviewedIntent, commandSnapshot, operationIds, outcome, verification,
optional retry lineage) as the floor UX-PB.2c may extend but not reduce, or the
Decision Status row stops delegating the field list to a story whose criterion is
narrower than `docs/SPEC.md:99`.

---

### H-3 — The admission-rejection payload has three incompatible consumers, one 12-variant `ErrorCode`, and a spine precedent that tells stories not to add one

**Pair: UX-PB.5c ↔ UX-PB.2b** (UX-PB.5a is the third consumer).

AD-16 enumerates four rejection causes plus one typed result but never a payload shape.
`ARCHITECTURE-SPINE.md:389-392`:

> "Execution must match the issued preview and a fresh coherent rebuild, and is rejected
> on in-progress state change, revision drift, an active refresh, or a lock-set overlap
> with any pending or running mutating operation."

`ARCHITECTURE-SPINE.md:394-395`:

> "Exactly one confirmed attempt may be active. A second confirmation fails closed with
> **a typed already-active result**."

What exists today: `src-tauri/src/error.rs:87-98` ships
`IpcError { code, message, detail?, manager_id?, op_id?, log_path? }` — a flat shape
with no member list — and `src-tauri/src/error.rs:68-81` ships exactly 12 `ErrorCode`
variants (`grep -c` → `12`), none of them an already-active code. `docs/SPEC.md:672`
collapses all four AD-16 causes into one:

> "`plan_stale` covers unknown, evicted, replayed, altered, or current-state-mismatched
> bulk plans and requires review of a newly issued plan."

Three stories each need something different out of that one shape:

- **UX-PB.2b**, `epics.md:678`: "**Then** admission fails closed, no second
  `planAttemptId` is minted, and only that one confirmed attempt remains active" — needs
  AD-16's *typed* already-active discriminant, which is a 13th `ErrorCode`.
- **UX-PB.5a**, `epics.md:1046`: "**Then** nothing executes, **the dialog explains
  why**, and the plan remains editable for re-review" — satisfiable from `message`.
- **UX-PB.5c**, `epics.md:1104`: "**Then** the run is blocked, **the invalidated details
  are replaced and what changed is explained**, and nothing executes until the plan is
  rebuilt and re-authorized" — needs *per-member* structured data (which packages became
  pinned, current, or removed). `IpcError` cannot carry a member list, and AD-3 freezes
  it: `ARCHITECTURE-SPINE.md:208-209`, "Stable `ErrorCode` values and `IpcError` context
  fields do not change meaning."

**And the spine's own precedent pushes them apart.** `ARCHITECTURE-SPINE.md:1045`:

> "It reuses `ErrorCode::SelfUpdateUnavailable` deliberately: a new `ErrorCode` variant
> is an AD-3 atomic boundary change across Rust, TypeScript, the guard map, and the
> committed fixtures, **which is not worth spending on a refusal message**."

A builder reading that reuses `PlanStale` for everything — which makes UX-PB.2b's
"typed already-active" untestable and UX-PB.5c's "what changed is explained"
unbuildable. A builder reading AD-16 literally adds a variant *and* a structured
rejection payload, and lands it as one AD-3 change that the other two stories then must
build against without any AD saying who goes first.

**Hole to close.** Decide the rejection surface at this altitude: is a refused
admission an `IpcError` (then AD-16's "typed already-active result" must name the code,
and UX-PB.5c's criterion must be restated as message-only), or a typed
`AdmissionRejection { reason, invalidatedMembers[] }` returned in the `Ok` arm (then
say so, and give it AD-23's whichever-lands-first sentence)? Also state whether the
`ARCHITECTURE-SPINE.md:1045` "not worth a new variant" reasoning is a general rule or a
one-off.

---

### H-4 — The `Verifying` window has no deadline and no owner: verification refreshes contend for `Semaphore(4)` on a spawn-relative clock

**Pair: UX-PB.3d ↔ Story 2.2.**

AD-16 makes verification mandatory and exempts it from coalescing but never bounds it.
`ARCHITECTURE-SPINE.md:401-411`:

> "A mutating attempt is not successful until the required affected Manager refreshes
> complete. The attempt explicitly enters `Verifying` … A verification refresh must be a
> fresh acquisition whose data collection begins strictly after the mutating process
> exited. **Verification refreshes are exempt from AD-4's duplicate-refresh coalescing**
> against any refresh already in flight at that instant"

The exemption is named and scoped to coalescing only. Nothing exempts them from the
cap, and AD-4 preserves it — `ARCHITECTURE-SPINE.md:267-269`:

> "The global concurrency cap of 4, the 120s aging guard, and duplicate-refresh
> coalescing are preserved."

Verified in the shipping tree: `src-tauri/src/queue.rs:47-50` —

```
/// Global concurrency cap (16GB machine — SPEC §5.7).
pub const MAX_CONCURRENCY: usize = 4;
/// Skip-ahead is disabled past any op that has waited this long.
pub const AGING_GUARD: Duration = Duration::from_secs(120);
```

— and `docs/SPEC.md:453` confirms verification refreshes are ordinary capped ops:
"bounded by a global `Semaphore(4)` (16GB headroom) … **Starvation guard**: skip-ahead is
disabled past any op that has waited >120s … successful Upgrade/SelfUpdate/HealthFix
auto-enqueues Refresh for `subject` (and `executor` if different)."

**The clock is spawn-relative, not enqueue-relative.** `src-tauri/src/process/runner.rs`
:296 `let started = Instant::now();`, :305 `let mut child = cmd.spawn()`, :324
`let overall_deadline = started + overall;`. Queue wait is not counted by any timeout.
The aging guard at `queue.rs:1312` measures `enqueued_at.elapsed()` but only to disable
skip-ahead — it never terminates anything.

**The two builders.** **Story 2.2** owns the per-Manager timeout, `epics.md:1176-1179`:

> "**Given** each of the six Manager adapters and **its documented timeout boundary**
> **When** controlled time reaches success, timeout, or error outcomes
> **Then** the correct Manager-specific terminal state and actionable detail appear
> **And** peers continue independently without real network access or wall-clock sleeps."

Its builder implements what ships: a spawn-relative `Timeout::Absolute`, so a refresh
waiting for a semaphore permit is not timing out. **UX-PB.3d** depends on that timeout
firing, `epics.md:842-844`:

> "**When** the required refresh verification itself **errors or times out**, distinct
> from a mutation failure
> **Then** the item does not declare success — **it stays `Verifying` until it
> resolves**, then reports verification failure with its evidence"

"Stays `Verifying` until it resolves" with a spawn-relative clock and a cap of 4 is
unbounded. A six-Manager `Update Everything` whose upgrades are still draining the four
permits leaves two verification refreshes queued, never spawned, never timing out, and
the attempt never leaves `Verifying`. UX-PB.3d's builder therefore invents a *second*,
attempt-level verification deadline; Story 2.2's builder has only the per-Manager one;
and `verificationResults` (`ARCHITECTURE-SPINE.md:483`) records the outcome of whichever
clock happened to fire.

**The spine already knows this clock exists and never assigns it.** AD-4:233-237: "the
clock any verification or staleness deadline reads" — the only mention of a verification
deadline anywhere in 19 ADs, and it is a sentence about *ports*, not about who sets the
value or what it bounds. `EXPERIENCE.md:218`'s 30-minute cap is explicitly per-Operation
("the **Operation** ends as `Timed out`"), not per-attempt.

**Hole to close.** AD-16 needs one sentence: whether verification refreshes consume
permits from AD-4's cap of 4 or run outside it, and what bounds the `Verifying` window —
naming the deadline's owner. If it is bounded by the per-Manager timeout, say the clock
starts at *enqueue* for verification refreshes, because the shipping clock does not.

---

### H-5 — UX-PB.5d's dismissal fallback covers `final confirm`, where both of its named targets are destroyed by AD-17's custody transfer

**Pair: UX-PB.5d ↔ UX-PB.3a.**

**UX-PB.5d**, `epics.md:1123-1125`:

> "**Given** the open Confirmation Dialog
> **When** it is dismissed via `Change Plan`, Escape, backdrop, **or final confirm** and
> the return target no longer survives
> **Then** focus is restored to a defined fallback (**the first staged Remove control or
> the plan heading**) rather than lost to the document body, and focus is never stranded
> inside a closed dialog."

**UX-PB.3a**, `epics.md:776-778`:

> "**Given** a confirmed plan whose atomic admission returned one durable
> `planAttemptId`
> **When** final confirmation closes the Confirmation Dialog
> **Then** the same Upgrade Sidecar transforms in place into the one active plan summary
> for that `planAttemptId`, **focus moves to its programmatically focusable Upgrade
> Activity summary heading**, and the status channel announces plan start."

On final confirm, UX-PB.5d's fallback is not merely different — it is **unreachable**,
by two ADs. AD-17:563-565: "Admitting the draft's own preview **empties the draft
atomically with the mint of `planAttemptId`**" — so no staged Remove control exists.
AD-17:583-584: "A confirmed attempt **replaces its content in place** rather than opening
a second surface" — so the plan heading is gone too. UX-PB.5d's `When` clause ("the
return target no longer survives") is *always* true for final confirm, which makes the
unreachable fallback the only branch that ever runs.

A UX source arbitrates and sides with UX-PB.3a — `EXPERIENCE.md:297`:

> "| Final confirmation | Close the dialog and move focus to the programmatically
> focusable Upgrade Activity summary heading in the transformed sidecar. |"

**No AD does.** AD-27 governs how focus is *painted* and says so —
`ARCHITECTURE-SPINE.md:907-912`: "Focus is drawn as a real 2px `outline` in
`--color-focus-ring` with `outline-offset`, on every interactive element." AD-17 owns the
region's visibility and the announcement channel but not focus. Nothing in 19 ADs owns
focus *destination*, while `epics.md:113` (NFR-6) makes it a requirement: "Keep primary
interactions keyboard/VoiceOver operable with visible focus and **deterministic
dialog/sidecar focus restoration**".

The same silence produces a second, softer pair a builder can read either way:
UX-PB.3c, `epics.md:814` — "and **a row or status update never moves focus**" — against
UX-PB.3d, `epics.md:836` — "**focus preserves the current viable node or moves to the
Results heading**". Reaching terminal is a status update by UX-PB.3c's own enumeration.
`EXPERIENCE.md:298-299` separates them ("Activity row/status update | Never move focus"
vs "Activity transforms to Results | Preserve the current viable focus. If that node is
removed, move focus to the Results heading"), so this one is resolvable from a source —
but only from that source.

**Hole to close.** Import `EXPERIENCE.md`'s Focus-transitions matrix (lines 290-302) into
an AD, or add a rule to AD-17 making that matrix normative and naming it as the single
authority for focus destination — the way AD-27 is the single authority for focus paint.
Then UX-PB.5d's `final confirm` clause has to be struck.

**Adjacency:** `review-divergence-v9.md` H-3 established that no precedence exists among
the spine, `docs/SPEC.md`, `DESIGN.md`, and `EXPERIENCE.md`; this is a new, concrete
instance whose victim criterion is unbuildable rather than merely ambiguous. v9 M-3
concerns whether a programmatically focused heading matches `:focus-visible` — paint,
not destination.

---

## MEDIUM

### M-1 — AD-23's `scope` cannot distinguish the header Checkbox inside a filtered Manager workspace, and the value it records is durable and never re-evaluated

**Pair: UX-PB.1c ↔ UX-PB.1d.**

AD-23 fixes a three-value enum, `ARCHITECTURE-SPINE.md:464`:

```text
  origin: Explicit | Bulk { scope: Manager(ManagerId) | FilteredView | Everything }
```

and `ARCHITECTURE-SPINE.md:772-775`: "`scope` is descriptive. It records which action
created the member — one Manager, the current filtered view, or everything — and is never
re-evaluated".

**UX-PB.1c** names four bulk entry points, `epics.md:579-581`:

> "**Given** eligible work reachable from **the count-labeled header Checkbox**, the
> Manager Header `Update Manager` action, **a Manager-wide action**, and `Update
> Everything`
> **When** I invoke each entry point
> **Then** each adds its eligible canonical identities to the same one persistent draft"

**UX-PB.1d** fixes the header Checkbox's domain as filter-scoped, `epics.md:609`:

> "**And** the bulk header Checkbox scope covers only eligible Packages **matching the
> active filter** and adds no ineligible identity."

The header Checkbox lives inside one Manager's workspace with one filter active. Its
scope is simultaneously `Manager(brew)` and `FilteredView`, and AD-23 offers no
composite. UX-PB.1c's builder, reading "a Manager-wide action" as the Manager case,
records `Manager(brew)`; UX-PB.1d's builder, reading its own criterion, records
`FilteredView`. Both durable, both never re-evaluated, and AD-23's stated justification
fails on either — `ARCHITECTURE-SPINE.md:514-516`: "a provenance without a scope cannot
be re-derived or **explained**." A member stamped `Manager(brew)` cannot explain that
only the 12 filtered rows were staged, and one stamped `FilteredView` cannot explain
which Manager.

`Update Everything` under an active filter has the same defect from the other end:
UX-PB.1c hardcodes the value, `epics.md:587` — "each carrying `Bulk { scope: Everything }`
provenance that is never re-evaluated" — while UX-PB.1d's rule makes a bulk expansion
filter-scoped, so the recorded `Everything` describes a set that was never expanded.

**Hole to close.** Make `scope` a record, not an enum: `{ manager?: ManagerId, filter?:
FilterId, breadth: Manager | FilteredView | Everything }`, or state that a filtered bulk
action records `FilteredView` and carries the Manager separately.

**Adjacency:** `review-divergence-v9.md` H-2 attacks the bulk expansion's *domain*
(rendered viewport vs every matching identity); `review-divergence-v8.md` V-5 attacks
the absence of a selection-staged value. This attacks the recorded *value* when two
existing values are both true. Distinct axis.

### M-2 — Between `terminal` and `Done` the draft is confirmable and invisible

**Pair: UX-PB.3a ↔ UX-PB.3d.**

**UX-PB.3a** gates confirmability on terminality, `epics.md:781-782`:

> "**When** the user keeps reviewing a draft or attempts a second confirmation
> **Then** only one confirmed Upgrade Plan attempt is active — the new draft stays in the
> Upgrade Plan and **cannot be confirmed until the active attempt is terminal**"

**AD-17** gates the draft's *visibility* on dismissal, not terminality —
`ARCHITECTURE-SPINE.md:570-573`:

> "While an attempt is non-terminal the region is owned by attempt status, and new
> membership staged during that attempt accumulates in the canonical draft without
> displacing it — **surfacing in the region only once the attempt's Results are
> dismissed**."

**UX-PB.3d** makes those two moments different, `epics.md:836`:

> "**Then** the attempt becomes terminal, the sidecar transforms in place into **a
> persistent Results Summary that remains until `Done`**"

So the window between "terminal" and "`Done`" is a state where UX-PB.3a's precondition is
satisfied and AD-17 keeps the draft hidden. The action the user would need is in the
hidden surface — UX-PB.5a, `epics.md:1030`: "**Then** it contains exactly one blue
`Confirm N Updates` action where N is the count of staged updates". UX-PB.3a's builder
enables confirmation at terminal (and must surface a control from somewhere); UX-PB.3d's
builder keeps Results on screen until `Done` and shows no plan footer. AD-17's own
"Higher precedence hides lower content, never destroys it" (`:584-585`) permits both — it
says nothing about whether a hidden lower-precedence surface's *actions* are live.

**Hole to close.** AD-17 should say whether a hidden lower-precedence content state's
actions are reachable, and AD-16/UX-PB.3a should gate re-confirmation on **Results
dismissed**, not on terminal — or state the escape (an inline `Review plan` in Results).

**Adjacency:** `review-divergence-v8.md` V-6 attacks the precedence *order* and which
combinations are reachable. This attacks a single boundary inside one ordering — terminal
vs dismissed — that V-6 does not name.

### M-3 — AD-23 makes every removal write a tombstone; UX-PB.1a's removal criterion writes none and UX-PB.1c's writes one

**Pair: UX-PB.1a ↔ UX-PB.1c.**

AD-23 is unconditional, `ARCHITECTURE-SPINE.md:776-780`:

> "Removal writes a tombstone on the intent. A later bulk expansion of any scope does not
> re-add a tombstoned ref … Explicitly re-staging a tombstoned ref clears its tombstone".

**UX-PB.1a**, which owns the single-row remove control, `epics.md:526-528`:

> "**Given** a Package already staged in the draft
> **When** I toggle its Checkbox off or activate its `Remove` control
> **Then** its canonical identity leaves the draft, Rust rebuilds the remaining plan from
> canonical intent, and nothing executes."

No tombstone. **UX-PB.1c** writes one, but only in the bulk-seeded `Given`,
`epics.md:587-589`:

> "**Given** a draft **seeded by `Update Everything`** … **When** I remove any item
> **Then** that one member leaves the draft and **a tombstone records the removal**, so
> no later bulk expansion of any scope re-adds it"

UX-PB.1a lands first (it Blocks UX-PB.1c, `epics.md:515`). Its builder implements removal
as set-subtraction. UX-PB.1c's builder then has to retrofit tombstones onto the removal
path it inherited, and the observable divergence is the exact guarantee AD-23 exists for:
after removing an `Explicit` member, does `Update Everything` bring it back? AD-23 says
no; UX-PB.1a's build says yes.

This one is a **residual-class** finding — the AD *does* arbitrate, so it is closable by
restating AD-23 in UX-PB.1a's criterion rather than by new architecture. It belongs on the
`epics.md` residual row for the next `bmad-correct-course` run alongside the two items
already listed there.

**Adjacency:** `review-divergence-v8.md` V-4 attacked the tombstone set's *lifetime and
bound* (fixed by AD-23's `:781-785` rule). This attacks *which removals create one*.

---

## LOW

### L-1 — User-facing copy has no owning surface, and `src/lib/errors.ts` is a shared single file three stories must edit

`docs/SPEC.md:672` names one home: "User-facing copy per code lives in
`src/lib/errors.ts` (state what happened + next action …)". No AD mentions it. Three
stories each mandate exact user-visible text: UX-PB.1d four literal reason strings
(`epics.md:608`, e.g. "pinned `This Package is pinned and cannot be updated. Unpin it,
then refresh Pack-Manager to make it selectable.`"), UX-PB.5c one literal warning
(`epics.md:1096`: "`Confirmation is off. Changes will run immediately when you choose Run
N Updates. Change in Settings.`"), UX-PB.3e a structure rather than a string
(`epics.md:863`: "it presents `What happened` and `What to do next` with evidence and safe
contextual actions before a secondary Retry"). This produces a merge surface rather than
two incompatible builds, so it does not meet the pair bar — recorded as an observation.

### L-2 — NFR-5's no-telemetry guarantee and NFR-3's output-flush thresholds have no AD home

`epics.md:111` (NFR-5): "Send no telemetry, expose no generic shell surface, exclude
inherited environment values from logs and diagnostics, and resist diagnostic symlink
substitution." Three of those four clauses have AD homes — no-shell in AD-4
(`ARCHITECTURE-SPINE.md:244-246`), environment exclusion in AD-18 (`:637-641`), symlinks in
AD-5 (`:283-284`). **No telemetry has none**, and AD-20 covers the webview's inbound trust
boundary, not outbound egress. Likewise `epics.md:107` (NFR-3) fixes "flush live output at
50 milliseconds, 64 lines, or 8 KiB" and a 5,000-line retention that no AD carries, while
the Consistency Conventions table (`:959`) covers determinism instead. I could not
construct a two-story pair for either, so both are observations: a dimension no AD owns is
a dimension a story is free to redefine, but no live story does.

---

## Sharpenings of the two tracked Open rows

Per charter these are not re-reports. Each adds evidence the tracked row does not carry.

### S-1 — on "Plan-attempt journal: writer identity and record cardinality"

The row (`ARCHITECTURE-SPINE.md:1051`) says AD-18 "names no single writer and no record
cardinality per attempt", and that "append-only NDJSON guarantees at least two records for
one attempt with no stated fold rule".

**New evidence, and it points the other way.** `docs/SPEC.md:99` already fixes the
cardinality at **one**:

> "Every confirmed plan also persists **one immutable attempt record** containing reviewed
> scope, command snapshot, `planAttemptId`, Operation identities, verification results,
> **terminal Results**, and optional retry lineage."

One record that contains terminal Results can only be written *at* terminal. That
contradicts what commit `5972109` just wrote into `epics.md:849`:

> "An attempt accumulates several append-only records — UX-PB.2c writes the admission
> record at mint — but exactly one **terminal** record exists per `planAttemptId` and
> UX-PB.4a writes it"

So the `epics.md` fix for v9 C-6 moved the story layer *away* from `docs/SPEC.md`, and it
did so in a document whose own header says the spine wins where they disagree
(`epics.md:46`) — while the spine records the question as open. Under `docs/SPEC.md:99`,
UX-PB.2c's entire admission-time record has no home and its crash criterion
(`epics.md:700-702`, "a `planAttemptId` was minted but its durable record was lost to a
crash or forced quit mid-admission") is unreachable, because nothing was written at mint.

**Sharpened disposition:** the row should not be closed by inventing a fold rule. It should
first decide between `docs/SPEC.md:99`'s one-record model and `epics.md:849`'s two-record
model, because they imply different stories. C-1 above is the same seam from the failure
side.

### S-2 — on "Transient selection has no owning invariant"

The row (`ARCHITECTURE-SPINE.md:1050`) frames the question as "the relationship between
transient row selection and `PlanIntent` membership", citing `docs/SPEC.md` F5's Esc
against `EXPERIENCE.md`'s selection-adds-membership.

**New evidence: the framing understates it.** `EXPERIENCE.md`'s grid contract is written
with no selection concept at all — line 149:

> "Space toggles the active eligible Package; **Shift+Up/Down extends a contiguous
> membership range from the anchor**; the header Checkbox adds/removes all eligible
> identities matching the active filter."

Range extension — the most selection-shaped gesture in the app — is specified as a
*membership* operation. Under this reading Story 3.5's "shift-range" (`epics.md:1278`:
"**When** toggle, shift-range, tri-state, Cmd+A, Space, Cmd-click, Clear, and Esc
interactions execute") writes `PlanIntent` members directly and the shipping `selection`
set must be **deleted**, not reconciled. `docs/SPEC.md:288` keeps the opposite: "Cmd+A
select all visible selectable rows · Space toggle focused row · **Esc clear selection** /
close sheet / close drawer".

**Sharpened disposition:** the decision the owner is being asked for is not "how do
selection and membership relate" but "does a selection model exist at all". If it does not,
Story 3.5's criterion needs rewriting and `src/store/packages.ts`'s `selection` set is
retired work, not brownfield state to preserve — and AD-23 then needs an `origin` value for
range-staged membership (which is `review-divergence-v8.md` V-5, still open).

---

## Attacks that failed — the spine held

Recorded so a later revision does not spend effort re-deciding these.

| Attack | Why it failed |
| --- | --- |
| UX-PB.1a's checkbox-off vs UX-PB.1d's inert control both writing membership | AD-16's "Ineligible-item inertness" (`:525-527`) and UX-PB.1d's `aria-disabled` rule (`epics.md:611-613`) agree; UX-PB.1d Blocks Story 3.2, which defers to it. |
| Two owners of the delegated-package refresh set | AD-16:404-405 defines "affected" exactly: "the executor and the subject of each mutating operation in the attempt — the same set the scheduler locked". |
| AD-22's rider ordering vs AD-19's persist-before-active vs Story 3.4's settings path | AD-22:734-742 explicitly scopes its ban to the confirming action and preserves `set_settings_core`'s own guard-held save. It holds. |
| AD-25's merge vs AD-16's post-exit verification refresh | Already `review-divergence-v8.md` N-3 (two timestamps on a merged snapshot); no new axis found. |
| AD-27 vs the sanctioned `ring-accent` survivor | Already `review-divergence-v9.md` M-1, and `docs/SPEC.md:208` now carries the mechanism verbatim. |
| `waiting` as an unclassified `OpStatus` variant | Already `review-divergence-v9.md` H-5. |
| Retry from a replay during a live attempt | Already `review-divergence-v9.md` C-5 and `review-divergence-v8.md` V-6. |
| UX-PB.3d ↔ UX-PB.3c on "verified" vs `Succeeded` | Already `review-divergence-v6.md` H-1. |

---

## Summary table

| ID | Sev | Pair | Hole |
| --- | --- | --- | --- |
| C-1 | CRITICAL | UX-PB.2b ↔ UX-PB.2c | AD-18's nonfatal append vs AD-16's all-or-none: does a confirmed attempt run with no durable record? |
| H-1 | HIGH | UX-PB.4a ↔ UX-PB.3d | Attempt-level outcome enum unenumerated; writer has 5 values, reader 6, `timedOut` missing |
| H-2 | HIGH | UX-PB.2c ↔ Story 6.5 | Spine delegates the attempt record's field list to a story whose list omits `retryOfPlanAttemptId` |
| H-3 | HIGH | UX-PB.5c ↔ UX-PB.2b | Admission-rejection payload unshaped against a 12-variant `ErrorCode` and a flat `IpcError` |
| H-4 | HIGH | UX-PB.3d ↔ Story 2.2 | `Verifying` has no deadline; verification refreshes contend for `Semaphore(4)` on a spawn-relative clock |
| H-5 | HIGH | UX-PB.5d ↔ UX-PB.3a | Focus destination has no AD; UX-PB.5d's final-confirm fallback targets controls AD-17 destroys |
| M-1 | MEDIUM | UX-PB.1c ↔ UX-PB.1d | AD-23's `scope` enum has no value for a filtered Manager-workspace bulk action |
| M-2 | MEDIUM | UX-PB.3a ↔ UX-PB.3d | Draft is confirmable at `terminal` but hidden until `Done` |
| M-3 | MEDIUM | UX-PB.1a ↔ UX-PB.1c | AD-23 tombstones every removal; UX-PB.1a's criterion writes none (residual class) |
| L-1 | LOW | — | User-facing copy has no owning surface; `src/lib/errors.ts` shared by three stories |
| L-2 | LOW | — | No-telemetry (NFR-5) and output-flush thresholds (NFR-3) have no AD home |
| S-1 | sharpening | tracked row | `docs/SPEC.md:99` fixes cardinality at one record; `epics.md:849` now says two |
| S-2 | sharpening | tracked row | `EXPERIENCE.md:149` has no selection concept; the question is whether one exists |

## Proposed changes, in priority order

1. **AD-18** — two tiers, matching NFR-4's transcript precedent: the admission record's
   creation is a precondition of admission and its failure aborts under AD-16's
   all-or-none; later appends stay nonfatal. Say which tier the terminal record is in. (C-1)
2. **Decide the record model before the fold rule** — `docs/SPEC.md:99`'s one immutable
   record against `epics.md:849`'s admission-plus-terminal pair. Then name the single
   writer and the fold semantics the tracked Open row asks for. (S-1, C-1)
3. **AD-16 / Normative domain minimum** — enumerate the attempt-level outcome set in
   `EXPERIENCE.md:226`'s words, declare its wire casing, and give it AD-23's
   whichever-lands-first sentence. (H-1)
4. **AD-18** — state the attempt record's required field floor including optional retry
   lineage, and stop delegating the field list to UX-PB.2c alone. (H-2)
5. **AD-16 or AD-3** — decide the rejection surface (`IpcError` code vs typed
   `AdmissionRejection` with invalidated members) and whether `:1045`'s "not worth a new
   variant" is general. (H-3)
6. **AD-16** — bound the `Verifying` window, name the deadline's owner, and say whether
   verification refreshes consume AD-4's four permits. (H-4)
7. **AD-17 (or a new AD)** — make `EXPERIENCE.md`'s Focus-transitions matrix normative and
   the single authority for focus destination, as AD-27 is for focus paint. Then strike
   UX-PB.5d's `final confirm` clause. (H-5)
8. **AD-23** — make `scope` carry Manager and filter together. (M-1)
9. **AD-17** — say whether a hidden lower-precedence content state's actions are live, and
   gate re-confirmation on Results dismissed rather than terminal. (M-2)
10. **`epics.md` residual row** — add UX-PB.1a's missing tombstone restatement. (M-3)
