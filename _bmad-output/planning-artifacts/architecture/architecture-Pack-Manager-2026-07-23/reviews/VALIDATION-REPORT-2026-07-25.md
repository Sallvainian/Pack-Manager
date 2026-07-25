# Spine Validation Report — revision 9

**Target:** `ARCHITECTURE-SPINE.md` (1053 lines, `artifact_revision: 9`, `status: final`, `updated: "2026-07-25"`)
**Intent:** Validate — report only. The spine was not modified.
**Date:** 2026-07-25
**Baseline commit:** spine written at `c8c1f9a` (2026-07-25T08:22:05-04:00); `main` is now at `5972109` (08:45:58).

Every finding below carries a `path:line "literal text"` citation from a file read
during this run. Counts come from the stated command, never from estimate.

---

## Gate verdict

**The spine needs another Update run.** Not for a defect in its invariants — the
19 live ADs hold, the deterministic lint is clean, and the paradigm is intact.
It needs one because **its own change-tracking rows have gone stale underneath
it**, and because the one genuinely open architectural gap it recorded has
gotten *worse* rather than staying still: the missing invariant is now being
substituted for by story prose in `epics.md`.

Deterministic pass, run this session:

```
uv run .claude/skills/bmad-architecture/scripts/lint_spine.py \
  --workspace _bmad-output/planning-artifacts/architecture/architecture-Pack-Manager-2026-07-23/
→ {"ok": true, "spine": "ARCHITECTURE-SPINE.md", "total_findings": 0, "by_severity": {}, "findings": []}
```

---

## F1 — CRITICAL — AD-18's missing invariant is now being written in story prose, and the two stories disagree

This is the sharpest finding in the run, and it is a *new* divergence pair — not
the one already tracked at `ARCHITECTURE-SPINE.md:1051`.

### The gap in AD-18 is confirmed

AD-18 spans lines 620–649. Its `Prevents` clause is the only place the word
"writer" appears anywhere in the AD:

- `ARCHITECTURE-SPINE.md:623` `"- **Prevents:** the writer, the History reader, and the diagnostics exporter each"`
- `ARCHITECTURE-SPINE.md:624` `"choosing a different home for attempt records — an extended \`operations.jsonl\`,"`

The clause is scoped to *choosing a home*, not to writer identity. Verified by
running `sed -n '620,650p' ARCHITECTURE-SPINE.md | grep -n "writer\|exactly one\|cardinality\|one record\|single\|per attempt\|fold\|supersede"` — the
only hit is line 623 above. AD-18 states **no writer and no record cardinality.**

### `epics.md` now carries both rules in its own text — and says so

`_bmad-output/planning-artifacts/epics.md:827` (UX-PB.3d Dependencies):

> `AD-18 (the plan-attempt journal's home, format and durability — note AD-18 does not itself name a writer or a record cardinality, so the terminal-write ownership below is stated here and belongs in AD-18 when it is next amended); UX-PB.4a owns the single durable terminal write and this story never writes one`

`epics.md:849` (UX-PB.3d acceptance criterion):

> `This story renders and announces Results; it never writes a durable record itself. An attempt accumulates several append-only records — UX-PB.2c writes the admission record at mint — but exactly one **terminal** record exists per \`planAttemptId\` and UX-PB.4a writes it, folding the attempt's records into the single immutable History row. A second terminal write here would append a duplicate with no rule for which record is authoritative.`

The spine's own memlog records the same read as a deliberate deferral:

- `.memlog.md` (third-from-last entry) `"RECORDED not applied, as new architecture outside this run's authorized scope: ... and AD-18 names the journal's home but neither its writer nor its per-attempt record cardinality."`

**The team lead's read is CONFIRMED, by three independent routes: AD-18's own
text, `epics.md`'s explicit note, and the run's memlog.**

### The new defect: the substitute rule is already being contradicted

`epics.md:849` assigns UX-PB.2c the admission record. UX-PB.2c's own story block
says its record carries the *outcome too*:

- `epics.md:693` `"**Then** the append-only record stores the reviewed Manager/Package scope, Manager self-update identities, exact command snapshot, version evidence, timestamps, and result/verification state as immutable plan-admission metadata"`

An append-only NDJSON record cannot hold "result/verification state" that is
"immutable" and also correct — the result does not exist at admission time. So
either UX-PB.2c writes a second record later (which UX-PB.3d:849 forbids: "exactly
one **terminal** record exists per `planAttemptId` and UX-PB.4a writes it"), or
UX-PB.2c's criterion is unbuildable as written.

And UX-PB.2c never reads the rule that binds it:

- `epics.md:684` `"**Dependencies:** UX-PB.2b; AD-16; D29; AD-27 (focus is a 2px \`outline\` in \`--color-focus-ring\`; never a \`ring-*\`/box-shadow, which WKWebView drops on native-appearance controls)"`

UX-PB.2c's Dependencies line **does not cite AD-18 at all**, even though the spine
binds it:

- `ARCHITECTURE-SPINE.md:622` `"- **Binds:** UX-PB.2c, UX-PB.2d, UX-PB.2f, UX-PB.3d, UX-PB.4a, UX-PB.4b, UX-PB.4e; Story 6.5"`

**Why this is architecture's problem, not a correct-course problem.** A rule that
governs three stories (UX-PB.2c writes admission, UX-PB.4a writes terminal,
UX-PB.3d writes neither) but lives in *one* story's acceptance criteria is
unenforceable by construction — the builder of UX-PB.2c has no path to it. This
is exactly the divergence class AD-18 exists to prevent, relocated one level down.

### Two independent lenses, both CONFIRMED

Run as separate subagents with no shared context — lens A on the story-contract
angle, lens B on the invariant-enforceability angle. They agree on the verdict and
each added material the other did not reach.

**The spine demonstrably knows the idiom it omitted here.** AD-24 is the control
case — the same spine, for the draft, writes exactly the rule AD-18 lacks:

- `ARCHITECTURE-SPINE.md:801` `"- **Rule:** The one persistent draft has exactly one author: a user staging or"`
- `ARCHITECTURE-SPINE.md:804` `"  rebuild may narrow it (AD-16); no other path adds, replaces, or clears"`
- `ARCHITECTURE-SPINE.md:805` `"  membership. **A confirmed retry does not empty the draft** — it admits a derived"`

So AD-18's silence is an omission, not a deferral of an unwritable rule.

**The rule was authored downstream and never travelled up.** `git blame`:
`epics.md:848-849` → `"Clear the epics.md residual pile (#36)"` (`5972109`);
AD-18's Rule 1 at `ARCHITECTURE-SPINE.md:626` → `"Reconcile the planning spine with reality, and fix what that surfaced (#35)"` (`c8c1f9a`).

**Lens B's concrete divergence, and why it is worse than a display bug.** Both
implementations write four records (AD-18 is append-only; AD-16's `state` has four
values at `ARCHITECTURE-SPINE.md:482` `"  state: admitted | running | verifying | terminal"`).
Impl A writes thin deltas; Impl B writes the full `PlanAttempt` each transition.
Both satisfy all six AD-18 Rules. The shipping fold that Rule 1's "the same
discipline" invites is **not idempotent** — I verified this in the real code:

- `src-tauri/src/journal.rs:131` `"            Ok(Line::Start(s)) => {"`
- `src-tauri/src/journal.rs:132` `"                index.insert(s.op_id.clone(), entries.len());"`
- `src-tauri/src/journal.rs:133` `"                entries.push((s, None));"`

Every start-shaped line pushes a **new** entry; only the finish half dedups, and
only against the latest index write. Unattached duplicates then read as
interrupted — `src-tauri/src/journal.rs:154` `"            None => (OpStatus::Interrupted, None, None),"`.
So Impl B replays as four History rows for one attempt, three of them
`Interrupted`. Worse, an Impl A crash-interrupted `{state:"running"}` record read
as current makes the active-attempt lookup resolve a dead attempt as live, and
AD-16's `ARCHITECTURE-SPINE.md:394` `"- **Rule:** Exactly one confirmed attempt may be active. A second confirmation"`
then fails **every** subsequent confirmation closed. That wedges admission
permanently, not just presentation.

**Single-writer is emergent placement, not a contract.** Verified:
`grep -rn "record_start\|record_finish" src-tauri/src/` has exactly two production
call sites — `src-tauri/src/queue.rs:1357` `"        self.deps.journal.record_start(&StartRecord {"` and
`src-tauri/src/queue.rs:1247` `"        self.deps.journal.record_finish(&FinishRecord {"` — every
other hit is a test in `journal.rs` or the unrelated `process/fake.rs` method.
`AppState` holds only the read snapshot (`src-tauri/src/state.rs:203`
`"    pub journal_records: Arc<RwLock<Vec<crate::ipc::OperationRecord>>>,"`), with the
writer handed into `QueueDeps` at `src-tauri/src/state.rs:247`
`"        let journal = Arc::new(Journal::new(journal_path));"`. But the *type* permits a
second writer — `src-tauri/src/journal.rs:58` `"pub struct Journal {"` with
`file: Mutex<Option<std::fs::File>>` and an `append(&self, …)` behind an `Arc`. So
"the same discipline" transfers a *file* discipline, not a *writer* discipline.

**Rule 6 is field-presence, not cardinality.** Its every subject is
Operation-side, and `grep -c "start/finish"` over the spine → `1`, this line,
naming the *other* journal:

- `ARCHITECTURE-SPINE.md:647` `"- **Rule:** Operation status, output and stall events, transcript metadata, and"`
- `ARCHITECTURE-SPINE.md:648` `"  journal start/finish records carry \`planAttemptId\` when one exists, so"`

`"when one exists"` is conditional presence; a cardinality invariant cannot be
conditional. The asymmetry is itself the finding — the one sentence in AD-18 that
names a per-record shape names it for `operations.jsonl` and stays silent for the
journal AD-18 creates.

**Lens B's escalation, which changes the fix.** The candidate terminal writers are
not two but **four**: `epics.md:849` names UX-PB.2c and UX-PB.4a, but UX-PB.2e
(cancellation drives an attempt terminal) and UX-PB.3g are equally exposed and are
named nowhere. And of the stories that actually write or correlate durable attempt
records, three cite AD-18 nowhere — `epics.md:684` (UX-PB.2c), `epics.md:711`
(UX-PB.2d), `epics.md:731` (UX-PB.2e) — two of which `:622` binds. **A bare AD-18
amendment would therefore be unenforceable**; the close needs a new AD whose
`Binds` names 2c/2d/2e/3d/3g/4a/4b/4e, with AD-18 carrying only a cross-reference.

One knock-on: `ARCHITECTURE-SPINE.md:1037`'s deferred "Plan-attempt file name and
serde shape" row is *downstream* of cardinality, not independent of it — the
shipping discriminator cannot separate Impl B's overlapping shapes at all
(`src-tauri/src/journal.rs:51` `"#[serde(untagged)]"`).

---

## F2 — HIGH — `ARCHITECTURE-SPINE.md:1051`'s diagnosis is now factually wrong in two clauses

The OPEN row reads:

> `ARCHITECTURE-SPINE.md:1051` `"UX-PB.3d and UX-PB.4a both own a terminal durable write, and append-only NDJSON guarantees at least two records for one attempt with no stated fold rule, so History could replay an attempt twice or replay a superseded record as current."`

Both clauses were true against the tree the spine was written from and are false
against committed `main`:

1. **"both own a terminal durable write"** — `epics.md:827` now says
   `"UX-PB.4a owns the single durable terminal write and this story never writes one"`,
   and `epics.md:849` says `"This story renders and announces Results; it never writes a durable record itself."`
2. **"no stated fold rule"** — `epics.md:936` now states one:
   `"**When** History folds that \`planAttemptId\`'s records into its row"`, resolved at
   `epics.md:937` `"**Then** the row is presented as \`Interrupted\` **only when the absence is genuine** — a terminal record that exists but failed to parse is reported as unreadable evidence rather than silently reclassifying a finished attempt as unfinished, and the fold states which it was."`

The row's *conclusion* (a new invariant is needed) survives. Its *evidence* does
not, and an Update must restate it against F1's actual shape or the next reader
will chase a divergence that no longer exists while missing the one that does.

---

## F3 — HIGH — `ARCHITECTURE-SPINE.md:1049` is stale by construction: all five residuals cleared

The row is titled `"| \`epics.md\` residuals for the next \`bmad-correct-course\` run | **OPEN — record only; do not edit \`epics.md\` here** |"` and lists five items.
`5972109` "Clear the epics.md residual pile (#36)" landed at 08:45:58, after the
spine's `c8c1f9a` at 08:22:05, and touched only `epics.md`:

```
git show --stat 5972109
→ _bmad-output/planning-artifacts/epics.md | 79 +++++++++++++++++---------------
  1 file changed, 43 insertions(+), 36 deletions(-)
```

Item by item, verified against the committed file:

| # | Residual as recorded at `:1049` | Status | Evidence |
| --- | --- | --- | --- |
| 1 | "**UX-PB.3d cites AD-25 but never states it**" | **CLEARED** | `epics.md:845` `"**And** the Manager's Last-good Snapshot is left in place with its timestamp (AD-25) — a verification refresh that errors or times out never replaces or clears the snapshot it failed to refresh, so the surface keeps showing the last state it actually knows rather than blanking."` — now criterion prose, not a Dependencies parenthetical. |
| 2 | "**AD-21's substance never reaches criterion text**, surviving only as a parenthetical on UX-PB.5b's Dependencies line" | **CLEARED** | `epics.md:1066` `"**And** \`skipUpgradePlanConfirmation\` is plan-inert (AD-21) — it is not a plan-determining input, so writing it never advances the canonical revision and never invalidates the preview it rides on."` — line 1066 sits inside UX-PB.5b (heading `:1048`, next story `:1084`). |
| 3 | register "still lists the canonical design-token set as `OPEN` and **\"Blocks UX-PB.1e and UX-PB.5d\"**" | **CLEARED** | `epics.md:308` now reads `"| Canonical design-token set | \`CLOSED\` — D35 | Resolved 2026-07-25 | Nothing blocked."` and `epics.md:307` `"nothing blocks starting it — the canonical design-token set that blocked UX-PB.1e and UX-PB.5d was decided and shipped (\`docs/DECISIONS.md\` D35), so both are startable."` |
| 4 | register "still records the `notarytool minos 15.0` question as OPEN" | **CLEARED** | `epics.md:309` `"The \`notarytool\` \`minos 15.0\` question is CLOSED by \`docs/DECISIONS.md\` D34: CI and release moved to \`macos-15\`, so the build SDK is no longer behind the declared floor and the mismatch the question was about no longer exists."` |
| 5 | "**AD-27 is cited nowhere**" | **CLEARED** | `grep -c "AD-27" epics.md` → `32`. Every story Dependencies line that renders a control now carries it, e.g. `epics.md:514`. |

The row's closing instruction also cleared: it warned `"Both cite this spine by **line number**, and those citations have already drifted"`.
`grep -n "ARCHITECTURE-SPINE.md[:#]*[0-9]" epics.md` now returns **nothing** — the
stale `ARCHITECTURE-SPINE.md:944` citation was removed with the rewritten row
(visible in the `5972109` diff as a deleted line).

**This row is an Update trigger on its own.** It is the spine's only OPEN row
whose entire content is now false.

---

## F4 — HIGH — `DRIFT-NOTE.md` is misordered and its Open-items list contradicts the spine's own RESOLVED row

The spine points every reader here as its change record — `ARCHITECTURE-SPINE.md:56`
`"Change record: \`DRIFT-NOTE.md\`."` (repeated at `:70`, `:80`, `:93`).

Section order, from `grep -n "^## " DRIFT-NOTE.md`:

```
19:## 1. What was wrong with revision 3
33:## 2. What revision 4 changed
114:## 2b. What the reviewer gate changed (revision 4 → 5)
162:## 3. Deliberately not changed
183:## 2c. Revision 6 — draft durability settled
207:## 4. Open items and the remaining tail
229:## 2d. Revision 7 — the epics.md register reconciled, and four false claims corrected
298:## 2e. Revision 8 — the three CRITICAL pairs closed, and one factual correction
595:## 2f. Revision 9 — five Open rows close, and one invariant comes out of a shipped defect
```

`## 4. Open items and the remaining tail` at `:207` is revision-4/5-era content
sitting physically *before* the revision 7, 8, and 9 sections. A reader working
top-down reaches the Open-items list before ever seeing revisions 7–9, and it
tells them:

- `DRIFT-NOTE.md:222` `"   view state, and five design-token names absent from \`src/styles/theme.css\`."`

That is false on committed `main`. All five are present —
`grep -c -- "--color-bg-shell\|--color-focus-ring\|--color-on-accent\|--color-on-success\|--color-violet" src/styles/theme.css` → `5`:

- `src/styles/theme.css:9` `"  --color-bg-shell:      #0F1420;   /* window chrome, stable navigation surfaces */"`
- `src/styles/theme.css:19` `"  --color-focus-ring:    #F4F7FB;"`
- `src/styles/theme.css:30` `"  --color-on-accent:     #07101D;   /* text/icons on bright blue fills */"`
- `src/styles/theme.css:32` `"  --color-on-success:    #07140D;   /* text/icons on bright green fills */"`
- `src/styles/theme.css:36` `"  --color-violet:        #B59CFF;   /* rare secondary accent; never a core status */"`

And it contradicts the spine's own RESOLVED row at `ARCHITECTURE-SPINE.md:1025`,
which states `"the five tokens \`DESIGN.md\` defined with no theme equivalent were added rather than dropped — including the \`--color-focus-ring: #F4F7FB\` whose absence was half this row's conflict."`

So the spine's designated change record actively contradicts the spine. Fixing
this is either a re-order (move `## 4` after `## 2f`) or a supersede note; either
way it is an Update, not a hand edit.

---

## F5 — MEDIUM — `ARCHITECTURE-SPINE.md:1026`'s currency caveat is stale for two of its three named files

The `macos-14 runner retirement` row ends:

> `ARCHITECTURE-SPINE.md:1026` `"**Caveat for a future currency check:** \`docs/SPEC.md\` §7.6 moved with the change, but \`docs/development-guide.md\`, \`docs/index.md\`, and \`_bmad-output/project-context.md\` still say \`macos-14\`."`

The `bmad-document-project` run that finished at 09:22:29 regenerated all nine
`docs/*.md`. Two of the three now say `macos-15`:

- `docs/development-guide.md:9` `"CI builds on \`macos-15\` (D34), which moved all three runner pins off the \`macos-14\` images GitHub began deprecating on 2026-07-06. The build SDK is no longer behind the declared floor."`
- `docs/index.md:85` `"macOS and Apple command-line build tools. The shipped bundle declares a \`15.0\` floor; CI builds on \`macos-15\` (D34)."`
- `docs/index.md:49` `"D34 moves CI and release builds to \`macos-15\`"`

The third named file is out of scope by instruction and not assessed.

The row's *substantive* claim is still fully correct — verified independently:
`grep -rn "runs-on: macos" .github/workflows/` returns exactly three lines, all
`macos-15` (`release.yml:63`, `ci.yml:28`, `ci.yml:70`), and the only remaining
`macos-14` strings in `.github/workflows/` are historical comments
(`ci.yml:10`, `release.yml:61`). Only the caveat needs retiring.

---

## F6 — MEDIUM — `ARCHITECTURE-SPINE.md:1050` splices two different sources into one quotation

The transient-selection OPEN row states:

> `ARCHITECTURE-SPINE.md:1050` `"\`docs/SPEC.md\` F5 has Esc \"clears the transient selection\", while \`EXPERIENCE.md\` has selection \"immediately adds/removes Upgrade Plan membership\""`

The `EXPERIENCE.md` half is exact —
`EXPERIENCE.md:143` `"On eligible Package rows, selection immediately adds/removes Upgrade Plan membership."`

The `SPEC.md` half is a conflation of two passages. F5 begins at `docs/SPEC.md:78`
(`"### F5 (P0) Multi-select upgrade"`), and the quoted phrase does appear inside
it — but attributed to a button, not to Esc:

- `docs/SPEC.md:80` `"\`Add N to Plan\` immediately adds the checked canonical identities to the persistent plan and clears the transient selection."`

Esc's clearing behavior lives in the keyboard map, outside F5, and is worded
differently:

- `docs/SPEC.md:288` `"Esc clear selection / close sheet / close drawer"`

**The divergence itself is real and the row should stay OPEN — and F7 escalates it
from a citation defect to an active build risk.** `SPEC.md:80`
describes a two-step model (check rows, then `Add N to Plan` commits and clears)
while `EXPERIENCE.md:143` describes a one-step model (the checkbox *is*
membership), and the live store still ships the selection set the row names:
`src/store/packages.ts:17` `"  selection: Partial<Record<ManagerId, Set<string>>>;"`.
Only the citation is wrong, and it is wrong in the direction that makes the
conflict look smaller than it is — the actual clash is button-versus-checkbox
commit semantics, not an Esc keybinding.

Worth flagging against the spine's own standing lesson, recorded in its memlog:
positional and verbatim references in this run folder have now rotted four times
(AD-16 rule ordinals, `epics.md` line numbers, spine line numbers, and a bare AD
count). This is a fifth instance, in the same class.

---

## F7 — CRITICAL — the transient-selection row gates nothing, and the first story in the queue has already picked a side

Raised by the rubric lens; I verified every citation. This escalates F6 from a
citation defect to an active build risk.

`ARCHITECTURE-SPINE.md:1050` states the pair is unresolvable at story level:

> `"Story 3.5 (keyboard selection) and UX-PB.1a (staging) can each obey every existing \`AD\` and still build opposite models."`

But nothing stops either from starting, and the register builders read says so:

- `epics.md:307` `"Epic UX-PB is the primary build queue and runs first, and nothing blocks starting it — the canonical design-token set that blocked UX-PB.1e and UX-PB.5d was decided and shipped (\`docs/DECISIONS.md\` D35), so both are startable."`

UX-PB.1a is the first story in that queue, and its criterion **already commits to
the `EXPERIENCE.md` one-step model**:

- `epics.md:522` `"**Then** the Package's canonical identity is added to the one persistent draft Upgrade Plan, nothing executes, and Rust rebuilds the exact command from canonical intent."`

Meanwhile the only checkbox that ships is labeled for the `SPEC.md` two-step
model that Story 3.5 will build from:

- `src/components/manager/PackageRow.tsx:96` ``"          aria-label={`Select ${pkg.name}`}"``
- `docs/SPEC.md:80` `"\`Add N to Plan\` immediately adds the checked canonical identities to the persistent plan and clears the transient selection."`

So the row is recorded as needing an owner decision, is marked "not this run's
scope", and carries no gate — while the story that would foreclose the decision is
scheduled first and unblocked. Whichever of UX-PB.1a and Story 3.5 lands first
silently settles an invariant the spine says it has not settled.

---

## F8 — HIGH — AD-27's `Binds` is narrower than AD-27's own Rule, and `epics.md` faithfully mirrors the wrong list

- `ARCHITECTURE-SPINE.md:901` `"- **Binds:** every story that renders an interactive control — all of Epic UX-PB,"`
- `ARCHITECTURE-SPINE.md:902` `"  Stories 3.1, 3.2, 3.5 — plus any change to \`src/styles/theme.css\` or the style"`

The Rule it introduces is unscoped — `ARCHITECTURE-SPINE.md:907`
`"- **Rule:** Focus is drawn as a real 2px \`outline\` in \`--color-focus-ring\` with"` /
`:908` `"\`outline-offset\`, on every interactive element."`

A prose preamble ("every story that renders an interactive control") followed by a
closed enumeration ("Stories 3.1, 3.2, 3.5") resolves as the enumeration.
`grep -n "Governing invariants" epics.md` returns six lines for the six surviving
stories, and they mirror it exactly — three carry AD-27, three do not:

| Story | `epics.md` line | Governing invariants | AD-27? |
| --- | --- | --- | --- |
| 2.2 | `:1166` | `"- Governing invariants: AD-4, AD-25"` | no |
| 3.1 | `:1200` | AD-16, AD-17, **AD-27** | yes |
| 3.2 | `:1224` | AD-16, AD-17, **AD-27** | yes |
| **3.4** | `:1247` | `"- Governing invariants: AD-4, AD-5, AD-19"` | **no** |
| 3.5 | `:1272` | AD-16, AD-17, **AD-27** | yes |
| **6.5** | `:1302` | `"- Governing invariants: AD-3, AD-4, AD-5, AD-16, AD-18, AD-26"` | **no** |

Story 3.4 is *Settings and Environment Report* — it renders settings controls, so
AD-27's Rule covers it and AD-27's `Binds` does not. The spine contradicts itself
on this within one table: `ARCHITECTURE-SPINE.md:1016` routes Story 3.4 to
`"AD-19, AD-21"` only, while `:1018` declares
`"| Keyboard focus indicator on any control | \`src/styles/theme.css\` + every component that renders a control | **AD-27** (mechanism, sampling limits, WebKit-vs-WKWebView), AD-11 (what a release may claim) |"`.

This is `epics.md` correctly implementing a defective list, so it is a spine fix,
not a correct-course fix.

---

## F9 — HIGH — AD-27's only shipping-engine enforcement point does not check focus

AD-27 concedes CI cannot prove the packaged app and names exactly one substitute:

- `ARCHITECTURE-SPINE.md:940` `"  story may describe it as such — the same substitution AD-3 and AD-26 refuse for"` … `:941` `"  the browser double and the fixtures. Verification in the real WKWebView remains"` / `:942` `"  the manual VoiceOver-and-keyboard pass on \`docs/RELEASE-CHECKLIST.md\` until a"`

That pass does not check focus visibility. Verified against both the working tree
and the committed file:

```
grep -n "focus\|indicator\|outline" docs/RELEASE-CHECKLIST.md          → exit 1, no output
git show HEAD:docs/RELEASE-CHECKLIST.md | grep -n "focus\|indicator\|outline" → exit 1, no output
```

The checklist's accessibility item asks only about reachability:

- `docs/RELEASE-CHECKLIST.md:91` `"9. **Keyboard and accessibility pass.** Tab and arrow navigation reach every control."`

Reachability is precisely what was *already true* of the nine controls whose focus
was invisible — the defect AD-27 exists because of. AD-27 delegates its only
real-engine verification to a checklist item that cannot catch its own founding
defect. (Sharpened repeat of a rubric-v9 finding that is itself tracked in no row.)

---

## F10 — HIGH — AD-12's "never hand-edited" is whole-file, and two other ADs require editing those files

- `ARCHITECTURE-SPINE.md:348` `"- **Rule:** Seven files are release-please-owned and never hand-edited:"`
- `:349–351` list `package.json`, `package-lock.json`, `src-tauri/tauri.conf.json`, `src-tauri/Cargo.toml`, `src-tauri/Cargo.lock`, `CHANGELOG.md`, `.release-please-manifest.json`.

release-please's actual ownership is field-scoped, not file-scoped —
`release-please-config.json` `extra-files`:

- `"path": "src-tauri/tauri.conf.json"`, `"jsonpath": "$.version"`
- `"path": "src-tauri/Cargo.toml"`, `"jsonpath": "$.package.version"`
- `"path": "src-tauri/Cargo.lock"`, `"jsonpath": "$.package[?(@.name.value==\"pack-manager\")].version"`

And two sibling ADs require hand edits inside those same files:

- `ARCHITECTURE-SPINE.md:303–304` `"- **Rule:** Minimum supported macOS is 15.0 at"` / `"  \`bundle.macOS.minimumSystemVersion\` (\`docs/DECISIONS.md\` D31)."` — that key lives at `src-tauri/tauri.conf.json:48` `"      \"minimumSystemVersion\": \"15.0\""`.
- `ARCHITECTURE-SPINE.md:883` `"  \`src-tauri/Cargo.toml\` declares no \`[profile.release]\`, so \`debug-assertions\`"` — AD-26's compile-time gate contemplates that section.

The updater public key is in the same file — `src-tauri/tauri.conf.json:30`
`"      \"pubkey\": \"dW50cnVzdGVkIGNvbW1lbnQ6…\""` — so under AD-12 read literally,
**key rotation has no legal mechanism.** A builder obeying AD-12 to the letter
cannot implement AD-11 or rotate the signing key. The rule needs narrowing to the
version fields it actually means.

---

## F11 — HIGH — the revision 8 and revision 9 reviewer-gate tails are tracked nowhere

The spine has exactly one tail row, and it is scoped to revision 6:

- `ARCHITECTURE-SPINE.md:1046` `"| Reviewer-gate tail (revision 6) | **Open** | The four \`*-v6\` lenses returned 44 findings: 5 CRITICAL, 14 HIGH, 18 MEDIUM, 7 LOW."`

`grep -n "review-.*-v8\|review-.*-v9"` over the spine returns four lines — `:1026`,
`:1034`, `:1050`, `:1051` — and each is a single promoted finding, not a tail
inventory. The run's own memlog records how large those gates were:

- `.memlog.md` final entry: `"Reviewer gate: four lenses, rubric verdict READY WITH FIXES, 9 CRITICAL / 20 HIGH / 28 MEDIUM / 15 LOW by each lens's own tally; currency checked 138 claims and verified 130 exactly."`

So revision 9's gate alone produced 72 findings, of which four received rows. The
revision-6 tail got an explicit accounting row with residual counts; revisions 8
and 9 got none. Anything not promoted is neither closed nor tracked, and the
`*-v8`/`*-v9` review files are the only record.

---

## F12 — MEDIUM — the post-publish operational envelope is silent

The rubric checklist treats a wholly silent dimension as a finding in itself. The
spine covers build, sign, notarize, and publish thoroughly (AD-11, AD-12), and
stops at publication. `grep -iE "rollback|retract|rotat|unpublish" ARCHITECTURE-SPINE.md`
returns nothing.

Unaddressed, and each is a call two builders could make incompatibly: what happens
to a published release found bad after `latest.json` is live; whether published
artifacts are immutable; how the updater key at `src-tauri/tauri.conf.json:30` is
rotated given every installed client embeds it — AD-11 names the failure mode at
`ARCHITECTURE-SPINE.md:288` `"- **Prevents:** a release shipping without the checks whose failure is silent and"` /
`:289` `"  simultaneous across every installed client"`, but names no recovery — and
where release artifacts are hosted as a durable dependency. This is
a *decide-or-defer* gap, not necessarily a missing invariant — a Deferred row with
a revisit condition would satisfy the checklist.

---

## What is genuinely clean

Confirmed, not assumed:

- **Deterministic lint:** zero findings (command and output above).
- **AD inventory:** 19 live ADs — AD-1..5, 11, 12, 16..27. No retired id (AD-6..10, AD-13..15) reused.
- **Design-token adoption (D35)** matches the spine's claims verbatim:
  `src/styles/theme.css:8` `"  --color-bg-base:       #090C13;   /* window background */"`,
  `:19` `"  --color-focus-ring:    #F4F7FB;"`,
  `:27` `"  --color-accent:        #65A7FF;   /* primary actions, active navigation, running */"`.
- **The sanctioned `ring-accent` survivor** is exactly where AD-27 and `:1052` say:
  `src/components/manager/PackageRow.tsx:85` `"        highlighted ? \"ring-2 ring-inset ring-accent\" : \"\","` — with no
  `focus-visible:` prefix, and pinned by a test asserting both directions
  (`src/components/manager/managerPane.test.tsx:114`, `:118`).
- **Runner pins (D34):** three `runs-on: macos-15`, zero `macos-14`, per the grep above.
- **AD-20's capability set** is current — it already names the third permission,
  `core:window:allow-start-dragging`, that landed with the window-drag fix.
- **The transient-selection OPEN row at `:1050` has not cleared** — the underlying
  `SPEC.md` / `EXPERIENCE.md` clash and the live `selection` set all still stand.
- **Resume point intact:** `.memlog.md` ends `"(event) spine finalized at revision 9"`.
  Its final entry records "epics.md untouched throughout, per owner instruction",
  which is consistent with `5972109` being a separate downstream run.
- **Every file path the spine cites resolves.** The two sprint-change proposals it
  names both exist (`sprint-change-proposal-2026-07-25.md`,
  `sprint-change-proposal-2026-07-25-spine-rev8.md`); the two `contracts/*.json`
  paths are absent by design (RETIRED rows at `:1030`, `:1029`).

Out of scope by instruction and not assessed: `_bmad-output/project-context.md`
(mid-update), everything under `_bmad-output/archive/` (non-authoritative; the
retired PRD is the intended state per D33).

---

## Recommendation

Run **Update**, resuming from `.memlog.md` at its revision-9 finalize event.
Scope it to the reconciliation, not to new architecture:

1. **Close `:1049`** — all five residuals cleared by `5972109`; cite the commit.
2. **Restate `:1051`** against F1's real shape — the pair is UX-PB.2c vs UX-PB.3d
   over admission-record contents, not UX-PB.3d vs UX-PB.4a over the terminal write.
3. **Decide F1.** This is the one item that is genuinely new architecture: AD-18
   needs a writer-and-cardinality rule (single writer per record class, exactly one
   terminal record per `planAttemptId`, and the fold rule for a missing or
   unreadable terminal record). `epics.md` has already written the rule for it,
   which is the argument for adopting rather than re-deriving — but adopting it
   means resolving the `epics.md:693` contradiction, and that is an owner decision.
4. **Fix `DRIFT-NOTE.md`'s ordering** and retire the `:222` token claim.
5. **Retire the `:1026` caveat** for the two regenerated docs.
6. **Repair the `:1050` citation** to `SPEC.md:80` / `SPEC.md:288` and restate the
   conflict as button-versus-checkbox commit semantics.

Items 1, 2, 4, 5, 6 are reconciliation and carry no new invariant. Item 3 needs
the owner.
