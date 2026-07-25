# Drift note — ARCHITECTURE-SPINE.md revision 3 → 8

**Date:** 2026-07-25 · **Intent:** update · **Run folder:** this directory ·
**Memlog:** `.memlog.md` (136 entries by `grep -c '^- ('`; entry 86 opens the
revision 7 run and entry 103 opens revision 8, so 86–102 are revision 7 and
103–136 are revision 8) · **Reviews:** `reviews/` — six lenses against
revision 4, four `*-v6.md` lenses against revision 6, four `*-v8.md` lenses
against revision 8

Why this file exists: revision 3 removed most of the readiness-gate apparatus
`docs/DECISIONS.md` D33 retired, but left enough behind that a builder following
the spine would have been told to create a directory D33 explicitly names as
never having existed. This records what changed and what was deliberately left
alone, so the same residue is not re-litigated or re-introduced later.

---

## 1. What was wrong with revision 3

Each row was verified against the tree on 2026-07-25.

| # | Revision 3 said | Verified reality |
| --- | --- | --- |
| 1 | `ARCHITECTURE-SPINE.md:158` — "**Rule:** `contracts/tauri-boundary/v1.json` is the one versioned boundary catalog." Followed by a strict JCS object shape, per-schema `sha256` fields, `nativeVectors[]`, and `scenarioContractSha256`. | `ls -d contracts` → `No such file or directory`. `docs/DECISIONS.md:326` — the retired gate ran "against a `contracts/` directory that does not exist and that no story creates". `project-context.md:132` — "no scenario contract, no evidence manifest, and no `contracts/` directory — **do not build one**." |
| 2 | `:426` `TypeScript \| 5.8.3` | `package-lock.json` resolves `typescript@7.0.2` from `registry.npmjs.org`, declared `~7.0.2` |
| 3 | `:427` `Vite \| 7.3.6` | `package-lock.json` resolves `vite@8.1.5`, declared `^8.1.5` |
| 4 | `.memlog.md:20` — "application version 0.2.7" | Was `1.0.0` when this row was written; `1.0.1` since release `8a4cf6a` on 2026-07-25. Revision 7 stopped restating it — the value is release-please-owned. |
| 5 | `:451` `\| Epics 1–6 \| **HELD** \| Retained but unscheduled pending rescope into normal development stories.` | The rescope was applied on 2026-07-25. Six stories survive (2.2, 3.1, 3.2, 3.4, 3.5, 6.5); 31 were archived; Epics 1, 4, and 5 were removed outright. |
| 6 | `:455` `\| AD-1..AD-5 wording cleanup \| **Open** \| Rule prose still carries evidence-lane vocabulary from the retired gate.` | The spine's own logged debt. AD-1 at `:112` still bound "TIR-1, TIR-8, RE-1..RE-11" — identifiers belonging to the retired register. |
| 7 | The Stack table omitted Tailwind, Zustand, TanStack Virtual, Vitest, the app version, the macOS floor, and the CI runner image. | All are load-bearing and all are pinned in the lockfiles or config. |

## 2. What revision 4 changed

**AD ids are stable.** AD-6..AD-10 and AD-13..AD-15 stay retired and their ids
are never reused. Nothing was renumbered.

### Amended in place

- **AD-1** — dropped the third concern ("Candidate-Specific Release Evidence")
  and the TIR/RE bindings. What survives is the dependency direction (product
  code never depends on test infrastructure, CI, or release tooling) plus D33's
  one surviving habit at `docs/DECISIONS.md:344`: verify behavior-present in the
  shipping code before scheduling anything described as a test gap.
- **AD-2** — kept the composition-root and no-runtime-selector rules; dropped the
  "native acceptance composition" framing, which had no live consumer.
- **AD-3** — the substantive change. The versioned catalog, its JCS shape,
  per-schema digests, `nativeVectors`, and `scenarioContractSha256` are gone. The
  boundary invariant survives and is now anchored on the mechanism that already
  ships: `src-tauri/src/ipc.rs` byte-compares each serialized model against its
  committed fixture in `dev/fixtures/ipc/` (15 files), regenerated only with
  `PM_UPDATE_CONTRACT=1 cargo test ipc_contract`, with the TypeScript half
  asserting its fixture set exactly equals its guard map. 20 commands and six
  events remain a baseline, not a fixed count.
- **AD-4** — dropped the candidate-acceptance clause; added the coordinator-first
  lock order and the complete lock-set rule explicitly, because all 28 UX-PB
  stories read plan-relevant state.
- **AD-5** — reduced from a lifecycle-harness specification to the invariant that
  actually binds: injected disposable roots with no production-directory
  fallback, and historical PGIDs as data only.
- **AD-11** — reframed from enumerated release evidence to
  `docs/RELEASE-CHECKLIST.md` plus the two publication-blocking checks in
  `release.yml`, carrying D31 (macOS 15.0), D32 (universal build, both updater
  keys, Apple-silicon-only verification), and D33's restatement of the
  accessibility method.
- **AD-12** — added the seven release-please-owned files explicitly.
- **AD-16** — the normative domain block now carries the intent kind
  (`Explicit | AllEligible`) that story UX-PB.1c makes durable, and names the
  removal of the global `includeSelfUpdates` control. The prose rule for intent
  kind existed; the domain block did not carry it. The final rule's reference to
  the retired catalog was removed. Added a state diagram for the
  draft → preview → attempt → verifying → terminal lifecycle.

### Added

Three new invariants, each because two named stories one level down could obey
every existing AD and still build incompatibly.

- **AD-17 — Rust owns the canonical draft; the sidecar is a layout region.**
  `epics.md` UX-PB.1b's recovery criterion governs what happens to a draft across a crash or
  force-quit — "the draft's canonical membership is reconstructed into the
  sidecar, or … the sidecar returns to empty with no fabricated membership".
  Revision 4 read the first branch as primary; §2c records revision 6 taking the
  second. Either way an owner was needed. Revision 3's AD-16
  said only that "Frontend draft state stores canonical `PlanIntent`", and
  frontend-only state cannot satisfy that. UX-PB.1a could have built a
  Zustand-only draft that UX-PB.1b then could not reconstruct. Also fixes whether
  the sidecar is a dialog, a drawer, or a layout region — currently it is
  transient `ui.dialog` state discarded by `closeDialog`.
- **AD-18 — confirmed plan attempts have their own durable store.**
  `epics.md` Story 6.5 requires the diagnostics ZIP to contain
  "`operations.jsonl`, and the durable plan-attempt records" as distinct entries,
  while UX-PB.2c writes them and UX-PB.4a reads them for History. Three stories,
  three possible homes, no AD choosing one.
- **AD-19 — persisted schemas tolerate their own history.** `epics.md` Story 3.4 requires an old persisted `autoOpenDrawer` value to be "tolerated
  during migration without ever becoming active", and UX-PB.5b adds
  `skipUpgradePlanConfirmation`. Ratifies the shipping behavior at
  `src-tauri/src/lib.rs:181` — a corrupt `settings.json` degrades to defaults
  with a visible notice — and extends it to unknown and retired fields.

### Also

- Stack table rebuilt from `package-lock.json` and `src-tauri/Cargo.lock`, with
  the app version, macOS floor, and CI runner image added.
- Added a Structural Seed showing the Application Support layout, so AD-17's
  draft file and AD-18's attempt journal have one agreed home. (Revision 6
  removed the draft file; the Seed now records its absence instead. See §2c.)
- Added a dependency-direction diagram to the Design Paradigm, since the arrow
  direction is itself AD-1's rule.
- The Decision Status table now reflects the applied rescope and records the
  retired catalog and the retired ASR framing as retired rather than silently
  dropped.

## 2b. What the reviewer gate changed (revision 4 → 5)

Six independent lenses ran against revision 4. The rubric walker returned **NOT
READY**. Full reviews are in `reviews/`; this is what was applied.

**The worst finding was an omission, not an error.** AD-16's `Prevents` claimed
"executing from a row or Manager header", and not one of its nine rules forbade
an entry point from executing. D27's headline invariant was asserted in a
`Prevents` and written nowhere. Five stories own five entry points, and
`src/components/manager/ManagerPane.tsx` still calls `executePlan` directly from
a row — so a builder reading only the spine had no instruction to retire it. Now
AD-16's first rule.

Three more that would have shipped wrong behavior:

- **A rebuild could enlarge membership.** Nothing forbade it, `AllEligible` reads
  as a live predicate, and UX-PB.5c's confirmation-off path admits with no
  dialog. A background refresh between staging and click would have mutated
  packages the user never saw. AD-16 now forbids enlarging rebuilds and freezes
  `AllEligible` expansion at the mutation that created it.
- **Verification could be satisfied by a pre-mutation snapshot.** AD-16 required
  affected refreshes to *complete*; AD-4 mandates duplicate-refresh coalescing. A
  refresh already in flight when `brew upgrade` exits would attach, complete, and
  verify from data older than the mutation — or fail a successful update. AD-16
  now requires verification refreshes to begin strictly after process exit and
  exempts them from coalescing.
- **AD-17 contradicted itself.** "Visibility driven by draft non-emptiness"
  against "a confirmed attempt replaces the sidecar's content": if admission
  empties the draft the Results surface is hidden, and if it doesn't the draft has
  no home while an attempt runs. Found independently by three lenses. Admission
  now transfers custody atomically, and visibility is a three-way union of
  non-empty draft, non-terminal attempt, or undismissed Results.

Also applied: `Verifying`/`Skipped` fixed as durable wire states rather than
React-derived (they are journaled, exported, and replayed); AD-4's port list
rewritten after it was shown to assert five traits' worth of ports that do not
exist and to declare six shipping call sites in violation; AD-3's `Prevents`
narrowed because fixtures prove payload shape and never dispatch through Tauri;
AD-19 split so a journal can't be defaulted away like a config file; AD-11's
accessibility rule corrected from present tense to obligation, because neither
automated check exists; AD-17 given explicit launch ordering, one announcement
channel, and below-720px presentation; AD-18 given disclosure and retention
coupling; and **AD-20** added for the webview trust boundary, which was a wholly
silent dimension.

The `[ASSUMPTION]` tag inside AD-17 was moved out to an Open row. A tag reading
"this rule may be wrong" inside the rule cancels the rule.

## 3. Deliberately not changed

- **~~`epics.md` lines 100–207 still carry the retired register.~~ RESOLVED in
  revision 7 — see §2d.** As written for revisions 4–6 this read: TIR-1..TIR-8,
  RE-1..RE-11, the 72-criterion readiness controls, AD-6..AD-10 and AD-13..AD-15,
  the Candidate Identity Manifest, the Evidence Registrar,
  `contracts/readiness/v1/contract-lock.json`, and the batch waves; only the `DR-`
  rows and AD-15 were patched after D33, and `epics.md:150` — a pre-reconciliation
  line number, not resolvable against the current file — still required set
  equality against `contracts/tauri-boundary/v1.json`, the exact requirement
  revision 4 removed from the spine. That run was scoped to the spine plus this
  note, so the reconciliation was recorded rather than performed. **It was
  performed on 2026-07-25** under
  `_bmad-output/planning-artifacts/sprint-change-proposal-2026-07-25.md`. The row
  is now RESOLVED in the spine's Decision Status table.
- **`docs/architecture.md`** was not re-read this pass. It remains listed as a
  source because it fed revisions 1–3; whether it also carries retired-gate
  framing is unverified.
- **`docs/SPEC.md`** is unchanged. Its update-experience text is D27–D30 target
  state by design; the spine describes it as target, not as shipping behavior.

## 2c. Revision 6 — draft durability settled

Revision 5 assumed the draft was durably persisted. The owner chose the other
branch: **fail-to-empty**. The draft is session-scoped and never written to disk;
every relaunch — clean quit, crash, or force-quit — starts with an empty draft and
a hidden sidecar. `epics.md` UX-PB.1b's recovery criterion permits this branch explicitly, so the
unconditional reading is AC-compliant rather than a narrowing.

What this bought: no draft schema to version, no partial-write recovery path, no
stale membership surviving a crash into a package set that has since changed, and
nothing for AD-19 to migrate. What it costs: a crash mid-staging loses the staging
work. That cost was accepted deliberately, and AD-17 now says so, so a later story
cannot quietly add persistence back as an "improvement" — doing so is a new
decision, not an implementation detail.

Ripple sites updated with it: the Structural Seed no longer lists a draft file and
states its absence positively; AD-4's new-effects rule drops the draft's
filesystem need and keeps AD-18's journal; the Deferred row narrows to the
plan-attempt file alone; and the capability map now reads "persists across
navigation, not across relaunch", because "persistent draft" was one careless read
away from meaning disk.

AD ids unchanged. AD-17 was amended in place.

## 4. Open items and the remaining tail

**The tail that was not applied.** The six reviews carry roughly forty findings
between them. Every critical and every high was applied. What remains is medium
and low, and it clusters into three kinds:

1. *Constants and thresholds* — stall thresholds, flush intervals, per-Manager
   timeout boundaries, journal retention numbers. Genuinely absent from the
   spine, and arguably correct to leave there: they are code-owned seed, not
   invariants two builders could pick incompatibly without a test catching it.
2. *Per-story contracts* — UX-PB.1e's Manager presentation contract, UX-PB.3e's
   failure guidance, verbatim ineligibility strings. These belong at story
   altitude; folding them in would make the spine a second copy of `epics.md`.
3. *Real but narrow gaps* — live-stream disconnect/reconnect resync, the
   `PlanAttempt` normative minimum's missing fields, session-scoped per-Manager
   view state, and five design-token names absent from `src/styles/theme.css`.
   Any of these is a reasonable follow-up; none blocks the first UX-PB story.

Reviewer files, if you want the detail: `reviews/review-rubric.md`,
`review-divergence.md`, `review-reconcile-epics.md`, `review-reconcile-ux.md`,
`review-reconcile-decisions.md`, `review-currency.md`.

## 2d. Revision 7 — the epics.md register reconciled, and four false claims corrected

`epics.md` was reconciled on 2026-07-25 under
`_bmad-output/planning-artifacts/sprint-change-proposal-2026-07-25.md`, closing the
last Open row revision 6 carried. Its six retired sections became retirement-record
stubs naming this spine as the sole AD authority; the AD-id collision that made the
divergence urgent — `epics.md` publishing its own AD-1..AD-15 under the same ids as
this spine but with different rules — is now an explicit collision table there.

A reviewer gate of four lenses ran against revision 6. What it changed:

- **The reconciliation did not pass on the first attempt.** The adversarial
  reconciliation lens returned **NO — the row cannot be closed as written**, because
  the rewrite left, and strengthened, an `R-001`..`R-008` risk register that
  re-imported the whole retired obligation set *by reference*. Those ids are defined
  only in archived gate artifacts, and the archived table's `Required mitigation`
  column IS the retired machinery: `R-002` reads "Deliver ASR-01 and AD-3
  set-equality checks", and `R-007` requires the physical-Intel acceptance **D32**
  dropped as undischargeable. The pre-rewrite text said "No mitigation is complete,
  waived, or accepted **through planning**"; the rewrite dropped the qualifier and
  added "**are NOT retired**", turning a scoped planning statement into a live claim
  that archived mitigations were still owed. The register is now retired, and the
  eight legitimate engineering concerns behind it are restated in `epics.md` as a
  table mapping each to the live AD that owns it.
- **The application version was wrong here, not in the tree.** This spine asserted
  1.0.0 at two places under a same-day "Verified against `package-lock.json`" banner
  while `package.json`, `src-tauri/tauri.conf.json`, `src-tauri/Cargo.toml`,
  `.release-please-manifest.json`, and the lockfile all read **1.0.1** — release
  `8a4cf6a` landed about a minute before revision 6 was written. The digit is inert,
  but AD-12 makes version drift a governed concern, so the spine now points at
  `.release-please-manifest.json` instead of restating a value it does not own.
- **Three Deferred rows rested on false premises about the bound stories.** The
  sharpest: "porting opener/reveal … No live story needs them controllable" against
  `epics.md` Story 6.5, which requires "native command/opener success and
  failure are controlled" — and both reveal paths are un-ported direct calls in
  `src-tauri/src/commands.rs`. A wrong `Prevents` is a weak rule; a wrong premise
  under a Deferred row silently removes the decision from everyone's queue. That row
  is now **OPEN, owner Story 6.5**. The crash/relaunch controller row likewise named
  no live consumer while four bound stories assert crash or relaunch behavior.
- **Citation rot, self-inflicted.** Reconciling `epics.md` shifted its line numbers,
  invalidating every `epics.md:N` citation in this folder. Repointed: the UX-PB.1b
  recovery criterion 437-439 → 521-523, Story 6.5's diagnostics criterion 1161 →
  1245, Story 3.4's migration criterion 1102 → 1189. The substance of each claim was
  unaffected; only the pointers rotted.

AD ids unchanged. No AD was renumbered or retired in this revision.

### What revision 7 did NOT resolve

The gate surfaced genuine architecture holes beyond this run's scope — closing them
means writing new invariants, not reconciling documents. They are recorded as Open
items in the Decision Status table and detailed in `reviews/review-divergence-v6.md`
and `reviews/review-rubric-v6.md`. The three sharpest, each a pair of stories that
obey every current AD and still build incompatibly:

1. **UX-PB.5b ↔ UX-PB.2b** — the confirmation opt-out writes a setting, AD-19 makes a
   settings write advance the canonical revision, and AD-16 rejects admission on
   revision drift. The safety opt-out deterministically fails its own run.
2. **UX-PB.1a ↔ UX-PB.1c** — `PlanIntent.kind` is one scalar, but AD-16 needs
   per-member provenance to keep a removed bulk item removed. The two stories cannot
   produce the same wire shape.
3. **UX-PB.4d ↔ UX-PB.1c** — Retry and the accumulating draft are two owners of one
   entity and write the draft's next state differently.

Also unresolved: per-Manager failure isolation and Last-good Snapshot retention is a
silent dimension that AD-16 references as though it were defined here (rubric H2),
and the app-update safety guard has no stated enforcement point while the shipping
guard is frontend-only (rubric H4).

## 2e. Revision 8 — the three CRITICAL pairs closed, and one factual correction

Revision 7 recorded these and declined to fix them, correctly: closing them means
writing new invariants, which was outside what that run was authorized for. This
run was authorized for exactly that. **Six new ADs; nothing renumbered; AD-6..AD-10
and AD-13..AD-15 stay retired.**

### The three pairs

Each was a pair of stories that obeyed every existing `AD` to the letter and would
still have built incompatibly — so each needed a new invariant, not a reworded one.

**1. UX-PB.5b ↔ UX-PB.2b — the safety opt-out deterministically failed its own run.**
`AD-19` advanced the canonical revision on a settings patch; `AD-16` rejected
admission on revision drift; UX-PB.5b writes `skipUpgradePlanConfirmation` inside
the very click that admits the plan. **This one was not theoretical — the mechanism
already ships.** `src-tauri/src/commands.rs` `set_settings_core` calls
`coordinator.bump_revision()` unconditionally for every key of every patch, and the
execute path rejects with `plan_stale("canonical state revision changed after
preview")` on `issued.revision != coordinator.revision()`. Any settings write of any
key already expires any live preview.

Closed by two ADs, because there were two questions and answering only one still
diverges:

- **AD-21** narrows what the revision *means*. Only a **plan-determining input** —
  state whose change can alter membership, exclusions, or constructed argv —
  advances the revision admission tests. Classification is **fail-closed and
  declared at the definition site**: a persisted key is plan-determining unless
  explicitly marked inert with a reason, because forgetting to classify should cost
  one unnecessary re-review, never an unreviewed execution.
  `skipUpgradePlanConfirmation` is declared inert — it selects whether a modal
  renders and cannot reach argv. Of the eight fields in `src-tauri/src/settings.rs`,
  only `include_greedy_by_default` can change plan membership.
- **AD-22** fixes the sequencing, and a **safety-reducing rider commits only if the
  action it rode on succeeded**: a rejected admission persists nothing and activates
  nothing, while a successful admission whose rider save fails leaves the attempt
  standing and the old preference active. The asymmetry is the whole point — an
  unsaved opt-out costs one extra confirmation; a saved opt-out on a refused run
  removes the gate from a run the user never got. This **deliberately overrides**
  UX-PB.5b's stated clause order (persist, activate, admit), and AD-22 says so
  rather than leaving the override implicit.

  *The first draft of this rule was unbuildable and the gate caught it.* It
  demanded one unbroken hold of `state.plan_coordinator` across validate → admit →
  persist. That cannot compile and would deadlock: the lock is a
  `std::sync::Mutex` (not reentrant, guard not `Send`), admission crosses an
  `await` into the scheduler task, and the scheduler takes the same lock —
  `execute_issued_plan` says so in its own doc comment, "No synchronous guard
  crosses an await." Worse, the rule's justification cited `set_settings_core`
  holding the lock across its `save_to`, which is true in isolation but inverts
  what the admission path actually does. The surviving invariant is atomicity
  against the **canonical revision**, not against one mutex hold — and the
  scheduler's `expected_revision` re-check already provides exactly that.

**2. UX-PB.1a ↔ UX-PB.1c — one scalar could not carry two facts.**
`PlanIntent.kind` was a single `Explicit | AllEligible` value, but AD-16's own
durability rule needs a bulk-added item the user removed to stay removed, and an
explicit item never to be absorbed into a later bulk action. Both are per-member
facts. Stage `brew:node` by row, then invoke `Update Everything`: if `kind` becomes
`AllEligible` the explicit item was absorbed; if it stays `Explicit` the bulk
machinery never applies to the action it was written for. Under AD-3 that is one
wire model with one committed fixture, so the two stories could not both land.

**AD-23** moves provenance onto the member — `origin: Explicit | Bulk { scope }` —
with `Explicit` dominating, `scope` descriptive and never re-evaluated, and a
**tombstone set** on the intent so a deliberate removal has a home a member list
cannot provide. The intent-level `kind` field is **removed, not reworded**: with
per-member origin there is no whole-intent value left to convert, so the "a removal
converts `AllEligible` → `Explicit`" clause is deleted rather than rephrased.

**3. UX-PB.4d ↔ UX-PB.1c — two authors of the draft's next state.**
AD-17 lets membership staged during a running attempt accumulate in the canonical
draft; AD-16 sent Retry back through that same draft. Honouring "re-run only what
failed" discards the accumulated item; honouring "no item is doubled or lost"
merges it, which inflates the reviewed retry scope and makes the
`retryOfPlanAttemptId` lineage claim a package the original attempt never held.

**AD-24** gives the draft exactly one author — a user staging or removal action
resolved through the Rust rebuild — and makes Retry compose a **derived
`RetryIntent`** that goes straight to preview without ever touching the draft. The
lineage claim then holds by construction rather than by each story remembering to
filter. Rejected alternatives: Retry replaces the draft (needs a second holding
area — a new entity), and Retry refused while the draft is non-empty (punishes the
user for staging during a run).

Ripple, and the second thing the gate corrected: the first draft made the retry
review a **fourth member** of AD-17's visibility union, ranked above Results. Every
input says the opposite — `EXPERIENCE.md` puts the retry scope "**inside** Results"
with "Cancel closes the scope and returns focus to the Retry action", `DESIGN.md`
lists `retry scope` as a *state of the Results Summary component*, and UX-PB.4d says
"reveals the proposed failed-item scope **inline**". Ranking it above Results deletes
the Retry action that `Cancel` must return to, hides the failure detail the user is
deciding against, and buries a History-origin retry behind a live attempt. The union
was reverted to three-way with the retry scope as a content state inside whichever
surface Retry was invoked from — and then corrected again in the verification pass
below, because demoting the scope had also dropped the retry *preview*, which does
legitimately belong in the union.

**A third correction, and the sharpest:** AD-24 and AD-17 defeated each other on the
one path both were written for. AD-17 said "the draft is emptied atomically with the
mint of `planAttemptId`" — unqualified, in four places including the state diagram —
and a confirmed retry mints a `planAttemptId`. So confirming a retry destroyed
exactly the accumulated membership AD-24's `Prevents` clause names. Custody transfer
is now scoped to **what was admitted**, not to the mint: admitting the draft's own
preview empties the draft; admitting a derived intent leaves the draft and its
tombstones untouched. The lifecycle diagram gained its own
`RetryScope → RetryPreview → Admitted` path so it no longer inherits the
draft-emptying arrow.

### The factual correction

**AD-11's accessibility rule was wrong, in the opposite direction from last time.**
It read "Automated 4.5:1 text-contrast and reduced-motion checks belong in the
Playwright/Vitest lane — **neither exists yet**". Reduced motion does exist and runs
in CI: `src/styles/theme.css` carries
`@media (prefers-reduced-motion: reduce)`, `tests/e2e/browser-style-contract.spec.ts`
emulates `{ reducedMotion: "reduce" }` and asserts transitions and animations
resolve to `0s`, and `.github/workflows/test.yml` runs `npm run test:e2e` on every
push and pull request to `main`. Contrast genuinely does not exist — `grep -rni
contrast tests src` returns exactly one hit, that spec's own disclaimer: "It does
not claim measured contrast compliance or validate the native Tauri package."

Worth recording *why* this matters beyond the digit: §2b records revision 5
correcting this same rule from a present-tense claim that **both** checks existed.
Overcorrecting a false presence into a false absence is the same failure with the
sign flipped, and it is the more expensive direction — a rule that says coverage is
absent invites a story to build it again, and AD-1 exists precisely to stop work
being scheduled for a gap the shipping code already fills. The rule now carries its
evidence inline rather than an assertion either way.

### The two HIGHs revision 7 named rather than fixed

Both were taken, but only one was the shape the owner described.

- **Last-good Snapshot / per-Manager failure isolation — same shape, taken.** AD-16
  said Results distinguish mutation from verification failure "while preserving the
  Last-good Snapshot rules", and the spine defined no such rules. A dangling
  reference in a `status: final` document is worse than silence because it reads as
  though the rule exists. It is genuinely a divergence pair: `project-context.md`
  names the exact trap — "`parse_recovery` must MERGE the recovered overlay into the
  inventory already parsed from the successful refresh outputs — replacing the
  snapshot with an outdated-only overlay makes every up-to-date package vanish" —
  and Story 2.2's refresh-failure path and UX-PB.3d's verification-refresh-failure
  path will each re-derive merge-versus-replace. **AD-25** states it, extends
  containment to a failed verification refresh, and is now the referent AD-16 cites.
  Given its own id rather than folded into AD-4 because AD-16 needs a stable
  citation target, and this run's own record already shows rule ordinals rot.
- **Story 6.5 — a different shape than described, and the opposite conclusion from
  the one revision 7 recorded.** The rubric framed it as an undecided, unowned
  choice: one story unbuildable, not two stories diverging. The real hole *is* a
  divergence pair — a builder satisfying Story 6.5 can put the automation surface
  in the shipping app or gate it out — because the macOS route works by running
  "an embedded WebDriver server **inside your app**". AD-2 forbids the first and
  AD-20 makes it a security-reviewed change, but neither mentions test automation,
  so neither reads as governing it. **AD-26** says it directly.

  But **Story 6.5 is buildable**, contrary to what revision 7 recorded and what the
  first draft of this revision repeated. `tauri-driver` driven *directly* excludes
  macOS; the recommended `@wdio/tauri-service` "works on **Windows, Linux, and
  macOS**", and its plugin is registered under `#[cfg(debug_assertions)]` — a
  compile-time exclusion from release bits, which is a *stronger* guarantee than
  the runtime selector AD-2 forbids. This repo declares no `[profile.release]`, so
  that gate holds today. The row is now **OPEN — owner Story 6.5; shape named, not
  yet adopted**: what actually remains is the adoption itself, an AD-20 review of a
  new plugin, with the CrabNebula fork alternative carrying a paid macOS API key.

  **How the first draft got this wrong is worth recording.** AD-26 originally
  quoted "Support is available on Windows and Linux, but not macOS due to the lack
  of a WKWebView driver tool" and stamped it "verified". That sentence does not
  exist in the cited source — `curl -sL https://tauri.app/llms-full.txt | grep -c`
  returns `0`. It was a documentation-retrieval tool's *summary*, presented under a
  source URL, which I treated as a quote. The real sentence is scoped to
  `tauri-driver` driven directly, and the paraphrase dropped that scope, which
  inverted the conclusion and turned a solvable adoption question into a fake
  blocker. A retrieval tool's paraphrase is not a citation, and "verified" must
  mean bytes were read. Caught by the currency lens and confirmed against raw
  bytes before rewriting.

### What revision 8 did NOT do

`epics.md` was not touched, by owner instruction — a single `bmad-correct-course`
run will batch it after this revision lands. Five divergences are recorded for it in
the spine's Decision Status table rather than fixed here: UX-PB.1b's crash/relaunch
criterion (carried from revision 7), UX-PB.1c's `AllEligible` → `Explicit`
conversion criterion (which AD-23 removes the mechanism for, though the observable
outcome survives), UX-PB.4d's "a new reviewable draft" (which must be read as AD-24's
derived intent, and currently rests the whole distinction on an indefinite article),
Story 6.5's test level and the register row that still cites only AD-2 and AD-3, and
UX-PB.5b's missing rejected-admission case.

Still open and untouched: the app-update safety guard has no stated enforcement point
while the shipping `install_app_update` has no Rust guard (rubric H4), and the
medium/low reviewer tail.

### The revision 8 reviewer gate

Four lenses ran against revision 8 — rubric walker, currency/reality, adversarial
divergence, input reconciliation. Rubric verdict: **READY WITH FIXES**. Taking each
lens's own tally: rubric 1/3/5/3, currency 1/3/2/2, divergence 4/4/4/2, reconcile
2/5/6/3 — **8 CRITICAL, 15 HIGH, 17 MEDIUM, 10 LOW, 50 total**. The lenses overlap
heavily at the top: three of the eight CRITICALs are the *same* AD-22 defect, found
independently by three lenses that each proposed the same remedy. Currency checked 82
claims and verified 80 exactly.

**Every CRITICAL was in the new material, not in the inputs, and all are fixed.**
The reconciliation lens re-read the file after the fixes landed and records "AD-21
and the rewritten AD-22 and AD-26 are clean", closing its own two CRITICALs. What the gate changed is recorded above:
AD-22 restated over the canonical revision, the custody-transfer scoping, the AD-17
precedence revert, AD-26's rewrite on verified source, AD-21's widened test, and six
`epics.md` items instead of five.

Two HIGHs were applied rather than deferred:

- **`Cancelling` and `Interaction required` were left undecided** while
  `Verifying`/`Skipped` were settled — the spine answered the durable-versus-derived
  question for two of the four states its own bound stories need, and the asymmetry
  made the silence read as deliberate. `Interaction required` was the exposed one:
  the spine said it is *emitted*, which points at an event, and the shipping
  precedent is an event with no status variant. UX-PB.3f would reasonably have built
  a transient flag that UX-PB.4b's replay then cannot reconstruct. Added to AD-16 as
  a rule rather than a new `AD`, because it completes a decision already half-made.
- **The `macos-14` clock**, below.

One HIGH was **recorded rather than decided**, because it is a product call:
`DESIGN.md` and `EXPERIENCE.md` specify a token set and a dedicated `focusRing` that
`src/styles/theme.css`, `docs/SPEC.md`, and a CI-enforced style assertion all
contradict. UX-PB.1e and UX-PB.5d are both bound to build from the UX sources, so
whichever lands first either breaks the CI contract on `main` or ships focus rings
`EXPERIENCE.md` forbids. It has its own Open row; choosing the palette is not
architecture's call to make alone.

### A verification pass on the gate's own fixes

The fixes above were themselves new material, and this revision's record is that new
material is where the defects live — so the divergence and reconciliation lenses were
sent back to attack the fixes rather than the original text. That found **one HIGH I
had introduced while fixing something else**, which is the most useful thing the whole
gate produced.

Reverting AD-17's union to three-way correctly demoted the retry **scope**, but
silently dropped the retry **preview** with it: the old four-way member "an open retry
review" had been covering both by accident. A derived intent under review is not the
draft (AD-24 forbids merging), not yet an attempt, and `EXPERIENCE.md` says in three
separate places that `Create new plan` *replaces* Results — so all three members went
false and the plan the user is being asked to confirm rendered nowhere. The union is
four-way again but with the **right** member, "a derived intent under review", ranked
above Results because replacing Results is what `Create new plan` is *for*. The
original objection does not apply: it was to the *scope* outranking Results, and the
scope stays inside Results. The two are now named as different things.

Two more, both the same shape — a fix that overshot:

- AD-22's persist ban read "**Nothing** may be persisted under a held coordinator
  guard", which condemns the exact shipping sequence AD-19 holds up as its reference.
  That is the mirror of the original C-1 error: the first version cited
  `set_settings_core` to license too much, the fix banned its pattern too broadly.
  Now scoped to "no *confirming action*".
- AD-17 fixed the announcement channel as **polite**, dropping `EXPERIENCE.md`'s
  "assertive priority only for an immediate safety action". The stall handoff and
  `Interaction required` are exactly that, and a polite region is announced only when
  the user is otherwise idle — so a VoiceOver user working elsewhere misses a safety
  prompt. Taken rather than deferred because the failure mode is *silent*: the
  neighbouring sentence ("Two live regions narrating one attempt is a defect") pushes
  an implementer away from the correct escalation, so the wrong build looks compliant
  and surfaces only in a manual VoiceOver pass, if at all.

The batch row also gained item **(g)**: UX-PB.4b and UX-PB.4d contradict each other on
Retry from History — 4d offers it from "a terminal Results **or History entry**", 4b
says "no control in the replay can mutate, re-run, or execute anything". The spine
takes 4d's side, on the ground that revealing a scope executes nothing. Taking a side
is the spine's job; recording it is too.

**One finding this run did not go looking for, and it has a deadline.** The currency
lens surfaced that GitHub deprecated the `macos-14` runner family on **2026-07-06** —
nineteen days before this revision — and the images are "fully unsupported by
November 2nd, 2026", after which workflows using the label "will be terminated with
an error" (`actions/runner-images` #13518). `ci.yml` pins it twice and `release.yml`
pins it for the signed, notarized build, so after that date Pack-Manager cannot cut a
release at all. AD-11 previously asserted "CI stays on `macos-14`" flatly; it now
states the clock, and the move has its own dated Open row. It is not fixed here —
this run was scoped to invariants and the runner move is workflow work — and it
perturbs the still-open `notarytool minos 15.0` residual, since a newer image ships a
newer SDK.

AD ids unchanged. Nothing was renumbered or retired in this revision.
