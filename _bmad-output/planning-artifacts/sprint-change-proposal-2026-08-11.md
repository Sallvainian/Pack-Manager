---
title: Sprint Change Proposal — AD-30 ownership and UX-PB.3a's trigger
date: 2026-08-11
status: applied
applied: 2026-08-11
approved_by: maintainer (all five changes, as proposed)
trigger: sprint-planning gate 2026-08-11 (verdict CONCERNS)
scope_classification: Moderate
artifacts_changed:
  - _bmad-output/planning-artifacts/epics.md
  - _bmad-output/implementation-artifacts/sprint-status.yaml
artifacts_verified_unchanged:
  - _bmad-output/planning-artifacts/prds/prd-Pack-Manager-2026-07-25/prd.md
  - _bmad-output/planning-artifacts/architecture/architecture-Pack-Manager-2026-07-23/ARCHITECTURE-SPINE.md
  - _bmad-output/planning-artifacts/ux-designs/ux-Pack-Manager-2026-07-23/EXPERIENCE.md
---

# Sprint Change Proposal — 2026-08-11

## 1. Issue Summary

The 2026-08-11 sprint-planning gate returned **CONCERNS** on a planning stack that is
otherwise in sync: `sprint-status.yaml` projects `epics.md` exactly (42 keys — 4 epics,
34 stories, 4 retrospectives — derived independently and matched key for key). Two defects
sit in `epics.md` alone.

Both are **staleness**, not disagreement. The PRD and the architecture spine already state
the correct thing; `epics.md` did not catch up to them.

### Finding 1 — AD-30 is uncited, and the behavior it governs is unowned

`grep -c AD-30 epics.md` returns **0**, against AD-27 **39**, AD-28 **14**, AD-29 **17**.
The spine revision-10 residuals batch (commit `0960aab`) measured AD-18, AD-27, AD-28 and
AD-29 and **never listed AD-30**, so it fell out of that sweep silently.

The citation gap is the surface. Underneath it:

```
grep -iE 'quit guard|QuitGuard|window-close' epics.md   →  0 matches
```

**No story's acceptance criteria build the guard.** AD-30's `Prevents` clause describes
the shipping state exactly — "the dialog and its host are built and only the app-update
caller reaches them, so a restart is guarded and a quit is not" — and nothing in the plan
schedules the fix. `prd.md:397` says the same in requirements terms: "The dialog exists
and is rendered by the shared dialog host, but nothing listens for a quit — its only
caller is the application-update path, so the *restart* case is guarded (FR-21) and the
*quit* case is not."

`epics.md:489` still reads `FR-14: Triaged out (was Epic 5) — Handle stalls, cancellation,
timeout, and shutdown honestly.` That triage is from the **D33 rescope of 2026-07-24**.
**D30 and AD-30 both postdate it** (2026-07-25) and revive the quit-guard limb as required.
Reinstating it is therefore *reconciliation to a later decision*, not a reopening of D33 —
the PRD states the rule directly: "a decision later than 2026-07-25 supersedes anything
here."

### Finding 2 — UX-PB.3a keys its trigger off a surface that need not exist

`epics.md:851` reads `**When** final confirmation closes the Confirmation Dialog`. UX-PB.5a
builds that dialog and sits at `:1097`, two waves later. That ordering problem is real, but
it is the *symptom*.

The root cause is that the trigger names the wrong event. **UX-PB.5c ships a path where no
dialog ever appears** — `:1180`, with `skipUpgradePlanConfirmation` true: "the immediate
action is `Run N Updates`, and **no dialog opens**." A dialog-keyed trigger therefore leaves
UX-PB.3a **undefined on the D28 confirmation-off path**, regardless of what order the
stories run in.

UX-PB.3a's own `Given` already establishes the correct event — "a confirmed plan whose
atomic admission returned one durable `planAttemptId`" — and its `Dependencies` already
carry `UX-PB.2 complete (PB.2a-f)`, so UX-PB.2b's atomic admission is available at wave 3.
The `When` is both wrong and redundant with the `Given`.

## 2. Impact Analysis

### Epic impact

| Epic | Impact |
| --- | --- |
| **Epic UX-PB** | UX-PB.3a's trigger rewritten. UX-PB.1b, UX-PB.2f, UX-PB.4e gain an AD-30 citation. **No resequencing** — the wave order is unchanged and UX-PB.5a stays at wave 5. |
| **Epic 6** | Gains one story (the quit guard). Story 6.5 gains an AD-30 citation. |
| **Epic 2, Epic 3** | None. |

### Artifact conflicts — none outside `epics.md`

Checked, and each is already correct:

- **PRD — no change needed, and it is ahead of `epics.md`.** `prd.md:397` marks the quit
  guard "Not yet built" with the diagnosis; `:399` carries the AD-30 architecture binding
  including the `Queued` ∪ `Running` active set; `:547` has FR-21 binding back — "the two
  guards' active sets must stay identical and may not drift apart"; `:761` records the
  OS-shutdown decision as closed.
- **Architecture — no change needed.** AD-30 is well-formed, with `Binds`, `Prevents`, and
  three rules. The spine is at revision 12 and its register carries no stale `OPEN` row.
- **UX — no change needed.** `EXPERIENCE.md` specifies the *post*-quit reconstruction
  (`:221`, `:432` — "After a crash or forced quit, reconstruct confirmed unfinished work as
  `Interrupted`") and native quit conventions (`:369`), but never the guard itself. That is
  consistent rather than defective: the guard was Epic 5 scope, which D33 removed, and its
  dialog is already built and already has settled UX through the app-update path.
- **Secondary artifacts** — none. No CI, deployment, or test-strategy change is implied;
  the new story carries its own acceptance criteria.

### Technical impact

The quit guard is **independent of Epic UX-PB**. It guards any live child process, which
ships today; it needs no `planAttemptId`, no draft domain, and nothing from D27–D30. It is
buildable now, against the existing operation queue and the existing `QuitGuardDialog`.

## 3. Recommended Approach — Option 1, Direct Adjustment

**Effort: Low. Risk: Low. Timeline impact: none to the UX-PB queue.**

Option 2 (Rollback) is **not viable and not needed** — no story has been implemented; all
38 keys are `backlog`. Option 3 (MVP review) is **not viable and not warranted** — MVP scope
is untouched, because neither change adds a requirement. Finding 1 schedules a requirement
the PRD already carries and a decision already made; finding 2 corrects a criterion's
wording.

Two judgment calls inside Option 1, both taken deliberately:

**(a) The quit guard goes to Epic 6, not Epic UX-PB.** Epic 6 is "Preserve State, Evidence,
and Privacy Across Failure and Relaunch" — a quit that orphans a child process is exactly
that. AD-30 already binds Story 6.5, so the epic is in its scope. Placing it under UX-PB
would couple an independent safety fix to the 28-story plan redesign and bury it behind the
whole queue, and would break the property `epics.md` states of the rescope survivors — "the
survivors carry no inter-epic dependencies."

**(b) Finding 2 is fixed by rewording, not resequencing.** Moving UX-PB.5a ahead of
UX-PB.3a would fix the ordering and leave the confirmation-off hole intact, because on that
path no dialog exists to be reordered. Rekeying the trigger to admission fixes both, changes
no story's position, and removes UX-PB.3a's dependency on UX-PB.5a entirely.

## 4. Detailed Change Proposals — all in `epics.md`

### Change 1 — UX-PB.3a's trigger (`:851`)

**OLD**
```
**When** final confirmation closes the Confirmation Dialog
```

**NEW**
```
**When** atomic admission completes and returns that `planAttemptId` — reached either
through the Confirmation Dialog or, when `skipUpgradePlanConfirmation` is `true`, through
the `Run N Updates` bypass that opens no dialog (UX-PB.5c)
```

**Rationale:** the trigger is admission, not the surface that requested it. Keyed to the
dialog, this criterion is undefined on the D28 confirmation-off path, where UX-PB.5c
guarantees "no dialog opens". Admission is minted by UX-PB.2b, already a dependency.

### Change 2 — UX-PB.3a's Dependencies (`:843`), append

**NEW (appended)**
```
; AD-30 is not implicated here — this story observes admission, not quit. **UX-PB.5a is
deliberately not a dependency:** the trigger above is admission, which UX-PB.2b provides,
so this story does not wait on the Confirmation Dialog and must not be resequenced behind
it.
```

**Rationale:** records why the ordering concern is closed, so a later gate does not
re-raise it and a later editor does not "fix" the trigger back to the dialog.

### Change 3 — AD-30 citations at the four stories it binds

AD-30 declares `Binds: UX-PB.2f, UX-PB.1b, UX-PB.4e; Story 6.5; any window-close, ⌘Q, or
app-relaunch path`. Append to each story's `Dependencies` line, matching the citation shape
the residuals batch used for AD-28 and AD-29:

```
AD-30 (a quit that would orphan a live child process is guarded at one enforcement point;
the guard's active set is `Queued` ∪ `Running` and may not drift from the app-update
guard's — work this story treats as interrupted is work that reached that guard, never work
silently discarded by an unguarded quit)
```

- **UX-PB.1b** (`:605`) — its relaunch criterion at `:624` describes state after a quit.
- **UX-PB.2f** (`:825`) — legacy operations without a `planAttemptId`.
- **UX-PB.4e** (`:1083`) — legacy operation history labeling.
- **Story 6.5** (`:1397`) — diagnostics evidence of interrupted runs.

### Change 4 — new Story 6.6 under Epic 6

```
### Story 6.6: Guard a quit that would orphan a live child process

**Primary concern:** Product Behavior
**Dependencies:** D30; AD-30; FR-21's shipping `Queued` ∪ `Running` refusal predicate
(the source of truth AD-30 takes and binds in return); AD-27 (focus is a 2px `outline` in
`--color-focus-ring`; never a `ring-*`/box-shadow, which WKWebView drops on
native-appearance controls)
**Blocks:** None

As a Pack-Manager user, I want a quit that would abandon running or queued work to ask me
first, so that work is never silently discarded and the app never leaves child processes
behind.

**Acceptance Criteria:**

**Given** at least one operation is `Queued` or `Running`
**When** I request a user-initiated quit — an OS window-close request or `⌘Q`
**Then** both paths resolve to the **same** enforcement point the application-update path
already uses, one predicate and one dialog decide, and I am presented an explicit choice
rather than having the work silently discarded
**And** the guard's active set is `Queued` ∪ `Running`, identical to FR-21's app-update
refusal — **queued counts as running**, because admission has already committed to the work
**And** no rollback is promised: partially completed Manager work stays partially completed.

**Given** the quit guard is showing
**When** I choose to quit anyway
**Then** every child process is terminated before the app exits — children never outlive
the app.

**Given** at least one operation is `Queued` or `Running`
**When** the quit is **OS-initiated** — a system shutdown or logout
**Then** **no dialog is presented** and the existing kill hook runs best-effort: cancel
every running operation, then **await the bounded idle wait**, because `cancel_all` only
flips the cancellation tokens and the runner tasks perform the SIGTERM → grace → SIGKILL
work — a shutdown path that does not await it exits before the children die
**And** blocking a logout to argue with the user is not done; losing the run is the accepted
cost.

**Given** a second code path that could decide to quit on its own
**When** it is added
**Then** it routes to the same enforcement point — a path that decides for itself is the
defect AD-30 names, and it is how the current build ended up with a `QuitGuardDialog` its
host renders and nothing but the update path calls.
```

### Change 5 — the FR-14 line (`:489`)

**OLD**
```
FR-14: Triaged out (was Epic 5) — Handle stalls, cancellation, timeout, and shutdown honestly.
```

**NEW**
```
FR-14: **Partly revived by a later decision.** The quit-guard limb is Epic 6 — Story 6.6.
The `Triaged out (was Epic 5)` status came from the D33 rescope of 2026-07-24; **D30 and
AD-30 both postdate it** (2026-07-25) and require the guard, and `prd.md` FR-14 carries it
as a requirement with the AD-30 binding. The stall, cancellation and timeout limbs stay as
the triage left them pending the broader FR reassignment the 2026-07-25 readiness report
raised as item 4.
```

**Rationale:** a `Triaged out` line beside a scheduled story is the same
contradiction-between-a-status-and-the-file class this session has been closing. Scoped to
the limb AD-30 revives; the other limbs are named as still-open rather than silently
reassigned.

## 5. Implementation Handoff

**Scope classification: Moderate** — backlog reorganization (one story added, one criterion
rewritten, five citations added), no fundamental replan.

| Recipient | Responsibility |
| --- | --- |
| This workflow | Apply changes 1–5 to `epics.md`. |
| This workflow | Add the `6-6-…` key to `sprint-status.yaml` under `epic-6`, status `backlog`. |
| `bmad-build` | Story 6.6 is buildable immediately — it depends on nothing in Epic UX-PB. |

> **`sprint-status.yaml` must be hand-edited for this, not regenerated.** The
> `sprint_plan.py` generate step drops all 30 Epic UX-PB keys; the 2026-08-11 gate hit this
> and derived the keys by hand instead. Add the single new key in place.

### Success criteria — all met, verified after applying

1. **AD-30 is cited exactly once at each of the four stories it binds.** `grep -c AD-30
   epics.md` returns **9**, not the 5 this proposal first predicted — that prediction was
   wrong and is corrected here rather than left to read as a failure. The distribution is
   what matters and it is right: `:605` UX-PB.1b, `:825` UX-PB.2f, `:1083` UX-PB.4e,
   `:1407` Story 6.5 — one each; `:489` the FR-14 line; and `:1435`, `:1437`, `:1446`,
   `:1460` inside Story 6.6, which builds the guard and cites the AD in its contract, its
   governing invariants, and two criteria.
2. `grep -ciE 'quit guard|window-close' epics.md` returns **3**, was **0** — the guard has
   an owner.
3. `grep -c 'closes the Confirmation Dialog' epics.md` returns **0**.
4. `sprint-status.yaml` carries **43 keys** (39 backlog, 4 optional) and projects
   `epics.md` exactly: 35 story headings, 35 story keys, **zero** drift in either
   direction.
5. No change to `prd.md`, `ARCHITECTURE-SPINE.md`, or `EXPERIENCE.md` — `git status`
   returns zero lines for all three paths.

### Applied

Story 6.6 was written in **Epic 6's story template** (`As a… / **Story Contract:** / FR and
requirement links / Required test level / Governing invariants / Dependencies`), not the
UX-PB template this proposal's §4 drafted it in. Epic 6's existing Story 6.5 uses the
former; matching the epic it joins was the right call and the substance is unchanged.

## 6. Checklist Record

| § | Item | Status |
| --- | --- | --- |
| 1.1–1.3 | Trigger, problem statement, evidence | Done — gate output plus `grep` counts against the committed file |
| 2.1–2.5 | Epic impact; resequencing considered | Done — resequencing evaluated and **rejected** with reason (§3b) |
| 3.1 | PRD conflicts | N/A — PRD verified correct and ahead of `epics.md` |
| 3.2 | Architecture conflicts | N/A — AD-30 well-formed; spine revision 12 clean |
| 3.3 | UI/UX conflicts | N/A — `EXPERIENCE.md` consistent; guard was removed Epic 5 scope |
| 3.4 | Secondary artifacts | N/A — none implicated |
| 4.1 | Option 1 Direct Adjustment | **Viable — selected.** Effort Low, Risk Low |
| 4.2 | Option 2 Rollback | Not viable — nothing implemented; all 38 keys `backlog` |
| 4.3 | Option 3 MVP review | Not viable — no requirement added; both changes reconcile to existing decisions |
| 5.1–5.5 | Proposal components | Done |
| 6.4 | `sprint-status.yaml` update | Action-needed — one key, hand-edited |

### Observations, not changes

Surfaced by this analysis and deliberately **left alone** as out of scope:

- **FR-12 and FR-13** (`:485`, `:487`) also read `Triaged out (was Epic 5)`. The
  2026-07-25 readiness report's item 4 asked for FR-9, FR-11, FR-12, FR-13 and FR-14 to be
  reassigned; only FR-14 is in this proposal's scope, because only FR-14 has a later
  decision reviving it.
- **`epics.md` cites "revision 10"** at `:12` and `:55`; the spine is at revision 12.
  Revisions 11 and 12 changed no `AD` rule, so this is a stamp rather than drift.
