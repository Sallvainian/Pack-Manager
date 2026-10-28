# Rubric Walk — ARCHITECTURE-SPINE.md revision 4

**Target:** `_bmad-output/planning-artifacts/architecture/architecture-Pack-Manager-2026-07-23/ARCHITECTURE-SPINE.md`
(30,442 bytes, 506 lines, `artifact_revision: 4`, `updated: "2026-07-25"`)
**Gate:** pre-handoff, initiative altitude
**Level below:** Epic UX-PB (28 stories) + Stories 2.2, 3.1, 3.2, 3.4, 3.5, 6.5
**Reviewed:** 2026-07-25

**Verdict: NOT READY.** One critical omission (the headline D27 invariant is
claimed in a `Prevents` but stated in no `Rule`), five high findings, six
medium, five low. The stack table, the Verified Brownfield Baseline, and both
mermaid diagrams pass.

Method note: every code, doc, and lockfile claim below was read this session and
is quoted verbatim as `path:line "text"`. Counts come from the commands shown.
No mermaid parser is installed in this repo (`ls node_modules | grep -i mermaid`
returns nothing), so diagram validity was checked by grammar inspection, not by
execution — recorded as such in §8.

---

## 1. CRITICAL — AD-16 `Prevents` "executing from a row or Manager header" is backed by no `Rule`

`ARCHITECTURE-SPINE.md:249-251`:

> - **Prevents:** executing from a row or Manager header, treating a short-lived
>   preview capability as History identity, fabricating plan groups from legacy
>   Operations, or reporting success before verification

AD-16 then states nine Rules (`:252-286`) plus four "Domain rules required by
the UX-PB acceptance criteria" (`:336-352`). Not one of them forbids an entry
point from executing. The closest is `:252-253`:

> - **Rule:** The draft holds canonical `PlanIntent`, never trusted executable
>   strings, with individually removable Package and Manager members plus explicit
>   option values.

That constrains what the draft *holds*. It says nothing about whether a row
action must route through the draft at all. Rule 7 (`:276-279`) is the only rule
mentioning bypass, and it is scoped to the confirmation opt-out:

> - **Rule:** Settings replace active `autoOpenDrawer` behavior with
>   `skipUpgradePlanConfirmation`, default `false`. A confirmation opt-out skips
>   only the final modal — never draft review, the Rust rebuild, stale detection,
>   or the explicit confirmation action.

A full-text grep of the spine for `execut|immediat|row|header` confirms the only
other occurrence is the Baseline's *description of current behavior*
(`:88-89` "a single-package row / action executes immediately"), which is a
starting condition, not a rule.

The source decision states the invariant explicitly. `docs/DECISIONS.md:166-168`:

> A
> Package row action, Manager-header action, Manager-wide action, and
> `Update Everything` all add eligible work to the same editable draft. No row
> or Manager-header update executes immediately.

**Why this is the worst finding here.** It is the exact two-units test. Five
independently-scheduled units each own one entry point and each could choose
incompatibly:

- `epics.md:396` (UX-PB.1a) — "nothing executes, and Rust rebuilds the exact command from canonical intent"
- `epics.md:453` (UX-PB.1c) — "each adds its eligible canonical identities to the same one persistent draft"
- `epics.md:507` (UX-PB.1e) — "the `Update Manager` action stages the self-update into the plan and never executes it"
- `epics.md:1059` (Story 3.1) — "the row plan action that adds or removes the Package's stable identity in the one persistent editable draft Upgrade Plan without executing"
- `epics.md:1133` (Story 3.5) — "nothing is built, submitted, enqueued, or executed"

And the shipping code today does the opposite, so the divergence is not
hypothetical — it is the default. `src/components/manager/ManagerPane.tsx:145-152`:

```
  async function upgradeRow(pkg: Package) {
    // Single-package plan executes immediately — no sheet (SPEC §F5).
    const plan = await buildUpgradePlan({
      selection: [{ managerId, packageId: pkg.id }],
      includeSelfUpdates: false,
      includeGreedyCasks: false,
    });
    await executePlan(plan);
  }
```

A builder who reads only the spine's Rules has no instruction to delete that
call site. A builder who reads the story ACs does. That is precisely the class of
inconsistency a spine exists to eliminate.

**Fix:** add an AD-16 rule of the form "No entry point executes. Every Package,
Manager-header, Manager-wide, and `Update Everything` action mutates the draft
and returns; `execute_plan` is reachable only from the confirmed-attempt path
(AD-16 rule 2). The `ManagerPane.upgradeRow` → `executePlan` path is retired."

---

## 2. HIGH — the operation status taxonomy (`Verifying`, `Skipped`) is left silent while three stories each require it

The spine fixes `Verifying` only at the *attempt* level. `ARCHITECTURE-SPINE.md:270-272`:

> - **Rule:** A mutating attempt is not successful until the required affected
>   Manager refreshes complete. The attempt explicitly enters `Verifying`, and
>   Results distinguish mutation failure from verification failure while preserving
>   the Last-good Snapshot rules.

and in the normative domain minimum, `:312`:

>   state: admitted | running | verifying | terminal

Both are attempt-scoped. But three stories require these states **per item**:

- `epics.md:685` (UX-PB.3c) — "it shows queued, waiting (with the lock or ownership reason), running (indeterminate unless the adapter provides a trustworthy total), verifying, or a terminal state"
- `epics.md:689` (UX-PB.3c) — "an unverified successful exit remains `Verifying`"
- `epics.md:611` (UX-PB.2e) — "unstarted attempt work is prevented from beginning and recorded as `Skipped`"
- `epics.md:711` (UX-PB.3d) — "each item is verified, failed, cancelled, or skipped ... `Skipped` marks only work that never started, and crash-reconstructed unfinished work reads as `Interrupted`"

The shipping wire enum has neither. `src-tauri/src/ipc.rs:99-107`:

```
pub enum OpStatus {
    Queued,
    Running,
    Succeeded,
    Failed,
    Cancelled,
    TimedOut,
    Interrupted,
}
```

`grep -rn "planAttemptId\|InteractionRequired\|skipUpgradePlanConfirmation" src/ src-tauri/src/ | wc -l` → `0`.

So each of UX-PB.2e / 3c / 3d must independently decide whether `Verifying` and
`Skipped` are **new `OpStatus` wire variants** (an IPC surface change that
AD-3's atomic-change rule governs, touching Rust models, TS guards, all five
`event_*`/`operation_*` fixtures, and subscriptions) or **derived presentation
states** computed in the React layer from `planAttemptId` correlation. Those two
choices are mutually incompatible, they are made by different stories in
different waves, and the spine says nothing.

This is a textbook divergence point at exactly this altitude, and AD-3 already
establishes the machinery it would slot into. It is missing.

---

## 3. HIGH — AD-17's sidecar visibility rule is falsified by UX-PB.3a/3d and by the UX spine

`ARCHITECTURE-SPINE.md:371-375`:

> - **Rule:** The sidecar is a single layout region whose visibility is driven by
>   draft non-emptiness — not a `ui.dialog` kind and not a `DialogHost` child.
>   Exactly one instance exists and it persists across `ActiveView` changes without
>   losing membership or scroll identity. When hidden, the workspace reclaims its
>   width with no reserved empty column. A confirmed attempt replaces the sidecar's
>   content with live attempt status rather than opening a second surface.

"driven by draft non-emptiness" is a single-driver rule, and it is wrong in two
independent ways.

**(a) It contradicts the attempt and Results lifecycle the same AD assumes.**
`epics.md:707` (UX-PB.3d):

> the sidecar transforms in place into a persistent Results Summary that remains until `Done`

and `epics.md:653` (UX-PB.3a):

> the new draft stays in the Upgrade Plan and cannot be confirmed until the active attempt is terminal

So a live attempt and its Results must keep the sidecar visible, while a *fresh*
draft can simultaneously be empty. A UX-PB.1b builder implementing the rule
literally writes `visible = draft.size > 0` and the Results Summary vanishes the
moment the draft empties. The real driver is a three-way union
(non-empty draft ∨ active attempt ∨ unacknowledged Results), which the spine
never states.

**(b) It contradicts the UX spine listed in this spine's own `sources` (`:22-23`).**
`.../ux-Pack-Manager-2026-07-23/DESIGN.md:173`:

> At 150–200% zoom, or whenever the usable CSS width drops below 720px, switch to a high-zoom layout: collapse the sidebar into an accessible navigation rail or temporary panel and present the Upgrade Plan, Activity, or Results as a full-workspace/stacked surface instead of retaining a fixed sidecar.

and `EXPERIENCE.md:366`:

> Below 720 usable CSS pixels—such as 150–200% zoom at the minimum window—enter high-zoom mode

Viewport width is a second visibility/placement driver. UX-PB.5d owns it
(`epics.md:980`). AD-17 gives UX-PB.1b no reason to expect it, and the
"Exactly one instance exists" clause is the only thing standing between the two
builders and two mount points.

---

## 4. HIGH — AD-4 Rule 1 asserts a port set that does not exist, and declares the shipping code in violation

`ARCHITECTURE-SPINE.md:163-172`:

> - **Rule:** The runtime-port set covers process spawn/output/exit/stdin/signals,
>   monotonic and wall time, executable discovery and `ToolEnv`, application and
>   log roots, filesystem and permissions, symlink metadata, opener and reveal,
>   current bundle and writability, focus and restart, and updater
>   check/download/install. `CommandRunner`, `EventSink`, `UpdateSource`, and
>   `PendingRelease` are existing ports and are extended rather than bypassed.
>   Direct calls for a covered effect live only in production adapters.

Present indicative. It is not true. `grep -rn "^pub trait \|^trait " src-tauri/src/`
returns exactly five traits:

```
src-tauri/src/events.rs:124:pub trait EventSink: Send + Sync {
src-tauri/src/managers/mod.rs:67:pub trait ManagerAdapter: Send + Sync {
src-tauri/src/app_update.rs:41:pub trait UpdateSource: Send + Sync {
src-tauri/src/app_update.rs:48:pub trait PendingRelease: Send + Sync {
src-tauri/src/process/runner.rs:26:pub trait CommandRunner: Send + Sync {
```

There is no clock port, no filesystem port, no opener port, no restart port, no
path/roots port. The effects the rule claims are "covered" are made as direct
calls from command handlers and orchestration, not from adapters:

- `src-tauri/src/commands.rs:672` — `tauri_plugin_opener::reveal_item_in_dir(PathBuf::from(&record.log_path)).map_err(|e| {`
- `src-tauri/src/commands.rs:681` — `tauri_plugin_opener::open_path(crate::logging::logs_dir(), None::<&str>).map_err(|e| {`
- `src-tauri/src/commands.rs:795` — `app.restart();`
- `src-tauri/src/app_update.rs:236` — `let exe = std::env::current_exe().map_err(|e| format!("cannot locate the running app: {e}"))?;`
- `src-tauri/src/queue.rs:1414` — `std::time::SystemTime::now()`
- `src-tauri/src/queue.rs:1139` — `enqueued_at: Instant::now(),`
- `src-tauri/src/paths.rs:506` — `let deadline = std::time::Instant::now() + Duration::from_secs(5);`

The spine's own Baseline says the same thing, three sections earlier
(`ARCHITECTURE-SPINE.md:80-82`):

> Opener, reveal, restart, current-executable,
>   bundle-parent writability, and some path/time behavior are still direct OS
>   calls.

So `:163` and `:80` contradict each other inside one document. Under the rubric's
"RATIFIES rather than contradicts the brownfield codebase", this fails: as
written, the rule makes `commands.rs:672`, `commands.rs:681`, `commands.rs:795`,
`app_update.rs:236`, `queue.rs:1414`, and `paths.rs:506` architecture violations
requiring remediation that no live story schedules.

Two secondary problems ride along:

- **Bloat.** "filesystem and permissions, symlink metadata, ... focus and restart"
  are ported for no live consumer. No surviving story requires a focus port or a
  symlink-metadata port. They fail the two-units test.
- **Unenforceable as a target.** If the intent is "new effects go through ports,"
  the rule should say so and name the covered set *for the live stories* — draft
  persistence (AD-17) and the plan-attempt journal (AD-18) both need filesystem
  access, and neither has a designated port.

**Fix:** rewrite Rule 1 in the imperative and scope it to effects the live queue
actually introduces, or move the aspirational port list to Deferred with a
named owner.

---

## 5. HIGH — AD-3's `Prevents` includes "event delivery ... broken", which no Rule prevents

`ARCHITECTURE-SPINE.md:135-137`:

> - **Prevents:** React and Rust suites agreeing internally while production
>   registration, invocation shape, serialization, event delivery, or startup
>   ordering is broken

AD-3's five Rules cover invocation shape (`:141-142`), serialization and
registration via byte-compared fixtures (`:144-150`), wire casing (`:151-153`),
the boundary-catalog prohibition (`:154-156`), and startup ordering (`:157-158`).
The enforcing mechanism named at `:146-148` is:

> The enforcing mechanism is the shipping contract test
>   — `src-tauri/src/ipc.rs` byte-compares each serialized model against its
>   committed fixture, and the TypeScript half asserts its fixture set exactly
>   equals its guard map.

Verified: `src-tauri/src/ipc.rs:788` `fn ipc_contract_matches_committed_fixtures() {`;
`src/lib/ipc/types.test.ts:56` `it("covers exactly the committed fixture set", () => {`;
`ls dev/fixtures/ipc/ | wc -l` → `15`.

That mechanism proves **payload shape on both sides**. It never dispatches an
event through Tauri. The spine says so itself at `:84-86`:

> - Rust tests construct handlers below Tauri; browser tests replace the Tauri
>   bridge with an in-browser double. Neither crosses the complete production
>   JavaScript-to-Tauri-to-Rust transport. No native harness exists.

And the only thing that would close it is Deferred (`:500`):

> | Native Tauri E2E harness and runner | **Deferred** | Story 6.5 is the only live consumer ("Real native Tauri E2E plus artifact inspection"). Any choice must satisfy AD-2 and AD-3. |

So the `Prevents` overclaims. This matters concretely because AD-18 Rule 4
(`:396-398`) adds `planAttemptId` to four event/record shapes, and UX-PB.2d's AC
(`epics.md:591`) requires it to flow through "the `op:status`/`op:output`/attention
events" — i.e. exactly the delivery path nothing exercises.

**Fix:** either narrow the `Prevents` to what the fixtures actually prove, or
promote the delivery obligation into a Rule with a named owner (Story 6.5).

---

## 6. HIGH — the `epics.md` retired register is left `Open`, and its resolution pointer does not exist

`ARCHITECTURE-SPINE.md:505`:

> | `epics.md` retired register | **Open** | `_bmad-output/planning-artifacts/epics.md` still carries TIR-1..TIR-8, RE-1..RE-11, the 72-criterion controls, and a set-equality requirement against `contracts/tauri-boundary/v1.json`. It contradicts this spine and `docs/DECISIONS.md` D33; reconciling it was out of scope for this run. See `DRIFT-NOTE.md`. |

The claim is accurate. `epics.md:150`:

> - AD-3 / ASR-01: Architecture is solely accountable for the shared real native command/event boundary. Accept by Batch 4 exit through exact set equality across the versioned `contracts/tauri-boundary/v1.json` catalog, production registration, Rust/TypeScript wire contracts, wrappers/subscriptions, fixtures, inventory, and native vectors

`epics.md:104`:

> - Preserve exactly 72 P0 criteria from `readiness-coverage-map.md`, whose status remains `final-pending-approval`.

`grep -c "TIR-" _bmad-output/planning-artifacts/epics.md` → `9`.
`ls contracts` → `ls: cannot access 'contracts': No such file or directory`.

And the spine directly forbids the artifact epics.md mandates
(`ARCHITECTURE-SPINE.md:155-156`):

> There is no separate versioned boundary-catalog file and none is to be
>   created.

**Why this is high, not medium.** `epics.md` *is* the level-below input. A story
author working from `epics.md:150` builds a `contracts/tauri-boundary/v1.json`
set-equality gate; one working from `ARCHITECTURE-SPINE.md:155` refuses to. That
is a live, in-tree contradiction between the two documents that bind the same
28+6 stories, and marking it `Open` in the spine does not stop it — the
contradictory text is still the operative planning artifact.

Compounding it: the stated resolution pointer is absent.

```
$ ls -la _bmad-output/planning-artifacts/architecture/architecture-Pack-Manager-2026-07-23/
.memlog.md
ARCHITECTURE-SPINE.md
reviews
$ find _bmad-output -name "DRIFT-NOTE.md"      # (no output)
```

See also §7.

---

## 7. MEDIUM — `DRIFT-NOTE.md` is referenced twice and does not exist

`ARCHITECTURE-SPINE.md:38-39`:

> AD-6..AD-10 and AD-13..AD-15 remain retired and their ids are never
> reused. Change record: `DRIFT-NOTE.md` in this folder.

`ARCHITECTURE-SPINE.md:505` (end of the Open row):

> See `DRIFT-NOTE.md`.

Neither the run folder nor anywhere under `_bmad-output/` contains it (commands
in §6). The spine's entire change record for revision 4 — and the only stated
account of the retired-AD ids and the epics.md drift — is a dangling reference.

---

## 8. MEDIUM — the webview trust boundary (Tauri capabilities + CSP) is a wholly silent dimension

The rubric requires every dimension the initiative altitude owns to be decided,
deferred, or open. For a Tauri app the capability/permission surface is the
security envelope, and it is governed by nothing in this spine. Grepping the
spine for `capabilit`, `csp`, or `permission` finds only AD-4's process-safety
rule (`:172-176`, argv/`env_clear`/`sudo`) — which is about *child processes*,
not the webview.

Current state, `src-tauri/capabilities/default.json`:

```
  "permissions": [
    "core:default",
    "opener:default",
    "core:window:allow-start-dragging"
  ]
```

`src-tauri/tauri.conf.json:25`: `      "csp": null`

This has *just moved* — `core:window:allow-start-dragging` is new as of commit
`8158bc1 fix: restore window dragging from the title bar`, and
`_bmad-output/project-context.md:136` is already stale on it ("grants only
`core:default` and `opener:default`").

Live stories that could each widen it independently: AD-17's durable draft file
and AD-18's plan-attempt journal (filesystem reach), Story 6.5's diagnostics
destination and `Open Logs` (opener/dialog reach), UX-PB.5d's focus and
high-zoom work (window reach). AD-2 governs *composition*; AD-3 governs the
*command surface shape*; nothing governs "may a story add a plugin permission or
relax CSP, and who decides."

**Fix:** one rule under AD-2 or AD-4 — capability and CSP changes are a single
reviewed decision, `csp: null` and the three-permission grant are the current
floor, and no story widens them unilaterally.

---

## 9. MEDIUM — launch-time reconstruction is unowned, and the Deferred row asserting otherwise is false

`ARCHITECTURE-SPINE.md:502`:

> | Crash/relaunch lifecycle controller | **Deferred** | No live story requires one. AD-5 binds whoever builds it. |

Four live stories require crash/relaunch reconstruction behavior:

- `epics.md:439` (UX-PB.1b) — "the draft's canonical membership is reconstructed into the sidecar, or — if it cannot be recovered — the sidecar returns to empty with no fabricated membership"
- `epics.md:573` (UX-PB.2c) — "it reconstructs the attempt only from durable plan-admission metadata that actually persisted"
- `epics.md:804` (UX-PB.4a) — "the in-flight attempt is reconciled from its durable `planAttemptId` records into one honest row"
- `epics.md:941` (UX-PB.5b) — "the setting reconstructs to exactly one coherent value, old or new and never partial"

Even reading "controller" narrowly as test scaffolding, the *product* obligation
is unowned. At launch the app must reconstruct, in some order: settings
(AD-19), the draft (AD-17 `:365-369` "reconstructed at launch"), the
plan-attempt journal (AD-18), and the existing subscribe-before-`get_state`
sequence AD-3 already fixes (`:157-158`). Whether an in-flight attempt is
reconciled *before* the draft is rehydrated determines whether UX-PB.1b's
sidecar shows a draft or UX-PB.4a's interrupted attempt. AD-3 already
demonstrates the spine knows startup ordering is an invariant — it fixes exactly
one edge of it and leaves the other three to four different stories.

---

## 10. MEDIUM — "affected Manager" is undefined in the verification rule, though the code defines it

`ARCHITECTURE-SPINE.md:270-271`:

> - **Rule:** A mutating attempt is not successful until the required affected
>   Manager refreshes complete.

"required affected Manager" is the load-bearing term and is never defined. The
shipping code has a precise definition — `src-tauri/src/queue.rs:1268`,
`:1276-1278`:

```
        // Successful upgrades auto-enqueue a refresh of the affected managers.
...
                let mut targets = vec![info.subject];
                if info.executor != info.subject {
                    targets.push(info.executor);
                }
```

i.e. `{subject} ∪ {executor}`. AD-4's lock rule at `:182-184` already uses that
same pairing ("The set is the executor lock union the subject lock"), so the
spine is one clause away from ratifying it and does not.

Divergence: Story 2.2 owns refresh phases and per-manager timeouts
(`epics.md:1034` "each of the six Manager adapters and its documented timeout
boundary"); UX-PB.3d owns verification gating (`epics.md:713-715`, "the required
refresh verification itself errors or times out"). If 3d's builder counts only
the subject and 2.2's builder scopes timeouts to subject+executor, a
mise-managed npm upgrade verifies against the wrong Manager and the attempt
reports success it did not earn. That is the failure AD-16 rule 5 exists to stop.

---

## 11. MEDIUM — AD-17's `[ASSUMPTION]` cancels the rule it is attached to, and is not surfaced as an open question

`ARCHITECTURE-SPINE.md:365-369`:

> - **Rule:** The draft is durable. It is written to Application Support under the
>   same atomic-replace discipline settings use, and reconstructed at launch. A
>   missing, unreadable, or incoherent draft file yields an empty draft — never a
>   partial or inferred membership. A draft is never surfaced as Activity or
>   History. `[ASSUMPTION]` Durable persistence is the reading taken from
>   UX-PB.1b, which also permits an always-empty-on-relaunch fallback.

AD-17's own `Prevents` (`:357-359`) is:

> - **Prevents:** UX-PB.1a building a frontend-only draft that UX-PB.1b cannot
>   reconstruct after a crash

The rule fixes durability; the trailing sentence hands back permission to build
the non-durable variant. A UX-PB.1a builder reading `:368-369` may reasonably
ship an in-memory draft and cite the spine. The `Prevents` is then not prevented.

Two independent problems:

1. The escape clause belongs in the Decision Status table as an open question,
   not inside an `[ADOPTED]` rule. The table (`:491-505`) does not list it.
2. The story text it cites is genuinely permissive — `epics.md:439` "or — if it
   cannot be recovered — the sidecar returns to empty" — but that reads as a
   *recovery-failure* branch, not as a licence to never persist. The spine's job
   is to close that reading, and it re-opens it.

---

## 12. MEDIUM — Activity as a first-class navigation destination is silent

`docs/DECISIONS.md:236-238` (D30):

> Activity is a first-class navigation destination for the active attempt and
> for replaying a completed History entry. A queued draft remains in the Upgrade
> Plan, not Activity.

and D30's supersession line, `docs/DECISIONS.md:243`:

> **Supersedes:** D18's flat Manager navigation and drawer-only Activity model.

The spine mentions `ActiveView` exactly once, in passing (`:373` "it persists
across `ActiveView` changes"), and never fixes what Activity *is*. The project's
own constraint is `_bmad-output/project-context.md:62`: "navigation in the
existing discriminated `ActiveView` state rather than adding a router."

Three units need to agree: UX-PB.3b ("the Activity destination opens for the
same `planAttemptId`", `epics.md:666`), UX-PB.4b ("Activity enters a clearly
labeled read-only replay", `epics.md:818`), UX-PB.4c ("full Activity is labeled
`Viewing past activity`, `Back to live activity` is offered", `epics.md:837`).
Whether Activity is a new `ActiveView` variant carrying a
`planAttemptId | { replay: planAttemptId }` discriminant, or a modal, or a
drawer, is a shape all three must share and none owns. AD-17 fixed the *sidecar*
region and stopped.

---

## 13. LOW — bloat: AD-1 Rule 2 is a planning-process rule, not an invariant

`ARCHITECTURE-SPINE.md:115-117`:

> - **Rule:** Missing or incorrect behavior is product work, not test work. Before
>   scheduling anything described as a test gap, verify whether the behavior is
>   already present in the shipping code (`docs/DECISIONS.md` D33).

Two units building one level down cannot choose incompatibly about *how work is
triaged before it is scheduled*. This governs the sprint-planning step, not the
build. It is correct guidance and it is already recorded at
`_bmad-output/project-context.md:135` ("Before scheduling anything described as
a test gap, verify whether the behavior already ships"). Cut or move to a
planning note.

---

## 14. LOW — bloat: AD-3 Rule 4, first sentence, is non-normative narration

`ARCHITECTURE-SPINE.md:154-156`:

> - **Rule:** The verified 20 commands and six events are a baseline, not a fixed
>   count. There is no separate versioned boundary-catalog file and none is to be
>   created.

Sentence 2 is a real prohibition and should stay. Sentence 1 constrains nothing —
it restates the Baseline (`:72-74`) and duplicates the Consistency Conventions
row (`:420`, "20 commands / six events is the current baseline, not an
invariant"). Third statement of the same non-rule.

(The counts themselves verify: `src-tauri/src/lib.rs:233-252` registers 20
handlers; `src-tauri/src/events.rs:77-82` declares six event constants.)

---

## 15. LOW — bloat: two Consistency Conventions rows are code-owned seed

`ARCHITECTURE-SPINE.md:426`:

> | Frontend state | Narrow Zustand selectors in components; the store's static accessor outside React. Objects and Sets are replaced immutably; cross-store derived state lives in `src/store/index.ts`. Per-manager phase is derived, never stored. |

`ARCHITECTURE-SPINE.md:427`:

> | Styling | Design tokens live in `src/styles/theme.css`; the product is dark-only and adds no hardcoded hex elsewhere. Color states always carry a text or icon equivalent. |

Both are verbatim-equivalent to `_bmad-output/project-context.md:60` and `:64`,
which is the file agents are instructed to read before implementing. These are
coding conventions the existing code already owns — the rubric's "structural
(stack, tree, full data shape) is seed, owned by the code once it exists". Two
units cannot diverge incompatibly on selector granularity; they can only be
individually better or worse.

---

## 16. LOW — AD-3 Rule 1 has no enforcement, and the AD does not say so

`ARCHITECTURE-SPINE.md:138-140`:

> - **Rule:** `src/lib/ipc/bridge.ts` is the only frontend module that imports
>   Tauri APIs, re-exporting exactly `invoke`, `listen`, and `UnlistenFn`.

Currently true — `grep -rn "@tauri-apps" src/ | grep -v "bridge.ts"` returns
nothing, and `src/lib/ipc/bridge.ts` exports exactly those three. But
`_bmad-output/project-context.md:56` records the gap: "The rule is convention-only
— no lint enforces it."

AD-3 Rule 2 explicitly names its enforcing mechanism ("The enforcing mechanism is
the shipping contract test", `:146`). Rule 1 names none, and the formatting makes
the two look equally binding. With 28 new stories about to touch the frontend
IPC layer, one stray `import { invoke } from "@tauri-apps/api/core"` silently
breaks the mock seam every Vitest suite depends on and nothing catches it.
Either state that Rule 1 is review-enforced, or make it a lint rule.

---

## 17. LOW — live-stream disconnect/reconnect resync is unowned

`epics.md:691-693` (UX-PB.3c):

> **Given** an attempt in progress (live-state stream disconnect/reconnect)
> **When** the per-item progress source drops mid-attempt and later reconnects
> **Then** each item keeps its last known honest state and is never silently shown complete, the interruption to the live stream is surfaced rather than guessed, and reconnection resumes correlated `planAttemptId` state without fabricating progress.

AD-3 fixes only cold-start ordering (`:157-158`, "Startup subscribes to native
events before `get_state` hydration"). Mid-session listener loss and resync is a
different problem with the same shape, and the sidecar (UX-PB.3a), full Activity
(UX-PB.3b), and per-item progress (UX-PB.3c) would each need the same answer.
Low rather than medium only because UX-PB.3c's AC is prescriptive enough to
serve as the de facto contract, and 3a/3b precede it in the dependency chain.

---

## Checks that PASS

**Stack is verified-current.** Every row in `ARCHITECTURE-SPINE.md:435-456` was
checked against the lockfiles it claims (`:432`, "Verified against
`package-lock.json` and `src-tauri/Cargo.lock` on 2026-07-25"). All 13
version-bearing rows match exactly:

| Spine | Lockfile | Match |
| --- | --- | --- |
| Application 1.0.0 | `package.json:4` `"version": "1.0.0",` | ✓ |
| Rust edition 2021 | `src-tauri/Cargo.toml:6` `edition = "2021"` | ✓ |
| Tauri Rust crate 2.11.5 | `Cargo.lock:3832-3833` | ✓ |
| Tauri JS API 2.11.1 | `package-lock.json` `@tauri-apps/api` | ✓ |
| Tauri CLI 2.11.4 | `package-lock.json` `@tauri-apps/cli` | ✓ |
| Tauri updater plugin 2.10.1 | `Cargo.lock:3983-3984` | ✓ |
| Tauri opener plugin 2.5.4 | `Cargo.lock:3961-3962` | ✓ |
| Tokio 1.53.1 | `Cargo.lock:4243-4244` | ✓ |
| React / React DOM 19.2.8 | `package-lock.json` | ✓ |
| TypeScript 7.0.2 | `package-lock.json` | ✓ |
| Vite 8.1.5 | `package-lock.json` | ✓ |
| Tailwind CSS 4.3.3 | `package-lock.json` | ✓ |
| Zustand 5.0.14 | `package-lock.json` | ✓ |
| TanStack React Virtual 3.14.8 | `package-lock.json` | ✓ |
| Vitest 4.1.10 | `package-lock.json` | ✓ |
| Playwright 1.61.1 | `package-lock.json` `@playwright/test` | ✓ |
| Minimum supported macOS 15.0 | `src-tauri/tauri.conf.json:48` `"minimumSystemVersion": "15.0"` | ✓ |

**Verified Brownfield Baseline (`:67-102`) holds at every checkable claim.**

- 20 commands: `src-tauri/src/lib.rs:233-252`, one handler per line. Six events:
  `src-tauri/src/events.rs:77-82`.
- `ls dev/fixtures/ipc/ | wc -l` → `15`. Byte-compare:
  `src-tauri/src/ipc.rs:545` "Contract test (SPEC §7.4) — byte-equality against
  dev/fixtures/ipc/*.json." and `:578` "The committed bytes must also round-trip
  through Deserialize."
- Journal: `src-tauri/src/journal.rs:19` `pub const COMPACT_KEEP: usize = 1000;`
  and `:179-180` "The rewrite is ATOMIC: content goes to a sibling temp file
  (fsynced), then `rename` replaces `operations.jsonl` in one step."
- Diagnostics: `src-tauri/src/diagnostics.rs:23`
  `pub const TRANSCRIPTS_INCLUDED: usize = 25;`, `:129`
  `zip.start_file("report.json", SimpleFileOptions::default())`, symlink refusal
  at `:75` `std::fs::symlink_metadata(path)`.
- Release checks: `.github/workflows/release.yml:318`
  `minisign -V -p "$RUNNER_TEMP/updater.pub" -x "$RUNNER_TEMP/updater.minisig" -m "$UPDATER_TGZ" \`
  and `:390` `echo "::error::latest.json reports $PUBLISHED, expected $VERSION"; exit 1`.
- D27–D30 symbols absent: `grep -rn "planAttemptId\|InteractionRequired\|skipUpgradePlanConfirmation" src/ src-tauri/src/ | wc -l` → `0`;
  `grep -rn "autoOpenDrawer" src/ src-tauri/src/ | wc -l` → `13`.
- Transient dialog state: `src/store/ui.ts:20` `| { kind: "upgradePlan"; plan: UpgradePlan }`.
- Plan capability bound: `src-tauri/src/state.rs:25` `pub const ISSUED_PLAN_LIMIT: usize = 64;`,
  ratified by AD-16 rule 1 (`:255-257`).
- Concurrency: `src-tauri/src/queue.rs:2398` `async fn semaphore_caps_concurrency_at_4() {`
  and `:50` `pub const AGING_GUARD: Duration = Duration::from_secs(120);`, ratified
  by AD-4 (`:184-185`).

**Both mermaid diagrams are valid.** Checked by grammar inspection (no mermaid
parser in `node_modules`).

- `:51-62`, `flowchart LR`: eight nodes, all ids `[A-Z]+` alphanumeric; every
  label is a double-quoted string, so the commas in `PORTS[...]` and `OS[...]`
  and the `<br/>` breaks are inside quotes and safe; all nine edges are `-->`
  between declared ids. Valid.
- `:320-332`, `stateDiagram-v2`: `[*] --> Draft` start, ten transitions, all
  labels after `:` are free text terminated by newline — the commas in
  "mutation, staleness, or eviction expires planId" and the parentheses in
  "(one-use planId)" are legal in transition descriptions. Self-transition
  `Draft --> Draft` is legal. No unclosed composite states. Valid.

**Retired-id hygiene is correct.** `:38-39` "AD-6..AD-10 and AD-13..AD-15 remain
retired and their ids are never reused" — confirmed: the document's AD sequence
is 1, 2, 3, 4, 5, 11, 12, 16, 17, 18, 19, with no reuse.

**Two Deferred rows are clean.** "Controlled child-helper language" (`:501`) has
no live consumer and a binding constraint. "Draft and plan-attempt file names and
serde shapes" (`:503`) names its owners — "the exact filenames and field lists
belong to UX-PB.1a and UX-PB.2c" — so the writer/reader/exporter triangle AD-18
worries about resolves to a single decider rather than three.

---

## Summary table

| # | Sev | Finding | Anchor |
| --- | --- | --- | --- |
| 1 | **critical** | AD-16 `Prevents` "executing from a row or Manager header" has no backing Rule; five entry-point units can diverge and the shipping code already executes immediately | `:249`, `ManagerPane.tsx:152` |
| 2 | high | `Verifying`/`Skipped` item-level status taxonomy silent; `OpStatus` has 7 variants; wire-change-vs-derived is unowned across UX-PB.2e/3c/3d | `:270`, `ipc.rs:99-107` |
| 3 | high | AD-17 sidecar visibility "driven by draft non-emptiness" falsified by UX-PB.3a/3d Results persistence and by DESIGN.md high-zoom mode | `:371`, `epics.md:707`, `DESIGN.md:173` |
| 4 | high | AD-4 Rule 1 asserts a port set that does not exist and contradicts the spine's own Baseline; declares six shipping call sites in violation | `:163` vs `:80`, `commands.rs:672/681/795` |
| 5 | high | AD-3 `Prevents` "event delivery ... broken" is not prevented by any Rule; the only mechanism is Deferred | `:135`, `:84-86`, `:500` |
| 6 | high | `epics.md` retired register left `Open` while still mandating a `contracts/tauri-boundary/v1.json` gate AD-3 forbids | `:505`, `epics.md:150`, `:155` |
| 7 | medium | `DRIFT-NOTE.md` referenced twice, does not exist | `:39`, `:505` |
| 8 | medium | Tauri capability/CSP trust boundary is a wholly silent dimension | `capabilities/default.json`, `tauri.conf.json:25` |
| 9 | medium | Launch-time reconstruction unowned; "No live story requires one" is false | `:502`, `epics.md:439/573/804/941` |
| 10 | medium | "required affected Manager refreshes" undefined though code defines subject ∪ executor | `:270`, `queue.rs:1276-1278` |
| 11 | medium | AD-17 `[ASSUMPTION]` re-permits the divergence AD-17 prevents; not surfaced in the status table | `:368-369`, `:357` |
| 12 | medium | Activity as a first-class navigation destination (D30) silent; 3 units need one `ActiveView` shape | `DECISIONS.md:236`, `epics.md:666/818/837` |
| 13 | low | Bloat: AD-1 Rule 2 is a planning-triage rule, not an invariant | `:115` |
| 14 | low | Bloat: AD-3 Rule 4 sentence 1 is non-normative, stated three times | `:154`, `:72`, `:420` |
| 15 | low | Bloat: Consistency Conventions "Frontend state" / "Styling" duplicate code-owned conventions | `:426-427` |
| 16 | low | AD-3 Rule 1 (sole Tauri importer) has no enforcement and does not say so | `:138` |
| 17 | low | Live-stream disconnect/reconnect resync unowned | `epics.md:691-693` |
