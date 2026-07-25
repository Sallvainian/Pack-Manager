# Drift note — ARCHITECTURE-SPINE.md revision 3 → 7

**Date:** 2026-07-25 · **Intent:** update · **Run folder:** this directory ·
**Memlog:** `.memlog.md` (95 entries; 47–87 are the revision 4–6 run, 88–95 are
revision 7) · **Reviews:** `reviews/` — six lenses against revision 4, four
`*-v6.md` lenses against revision 6

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
