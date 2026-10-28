# Completeness audit — revision 10 of `ARCHITECTURE-SPINE.md`

**Question asked:** not "is it good" — **what is missing?** What was skipped, left
unverified, claimed but not done, or produced but never reconciled.

**Targets (read-only; nothing was edited):**
`ARCHITECTURE-SPINE.md` (1541 lines), `.memlog.md` (231 lines, revision 10 opens at
`:164`), `DRIFT-NOTE.md` (929 lines, revision 10 is `## 2g` at `:795`), and the six
revision-10 review files plus `VALIDATION-REPORT-2026-07-25.md` in `reviews/`.

**Process contract:** `.claude/skills/bmad-architecture/SKILL.md` (85 lines) and
`.claude/skills/bmad-architecture/references/reviewer-gate.md` (13 lines).

**Repo state:** branch `chore/restore-phase-2-prd`, `HEAD` `1ac959e`. Every count below
states the command that produced it. Anything I could not verify is labelled
UNVERIFIED rather than asserted.

---

## 1. Every finding in all six review files — disposition

### Disposition rule used

- **APPLIED** — the spine's own text changed to enact the remedy (an `AD` rule,
  `Binds`, a convention row, the baseline, or a `RESOLVED` status row).
- **RECORDED** — the finding is named as a scheduled item inside an `OPEN` /
  record-only row (`epics.md` residuals, "Maintainer edits this spine cannot make",
  or an Open row of its own). The spine changed, but the *fix* has not happened —
  it is an `epics.md` or maintainer edit this run was forbidden to make.
- **NEITHER** — no trace in the spine at all.

Each row was verified by grepping the spine for the remedy the finding proposed, not
by reading the run's account of itself.

### Finding population — measured

```
$ grep -c … (per file, headings matching the finding-id pattern)
review-reconcile-prd-v10.md        10   (1 C / 4 H / 2 M / 3 L)
review-reconcile-decisions-v10.md  11   (0 C / 2 H / 6 M / 3 L)
review-currency-v10b.md            13   (3 C / 3 H / 4 M / 3 L)
review-rubric-rev10.md             12   (1 C / 4 H / 5 M / 2 L)
review-currency-rev10.md            8   (0 C / 0 H / 4 M / 4 L)
review-divergence-rev10.md         14   (2 C / 5 H / 3 M / 4 L)
                                   --
TOTAL                              68   (7 C / 18 H / 24 M / 19 L)
```

### `review-reconcile-prd-v10.md`

| Id | Sev | Disposition | Evidence in the spine |
| --- | --- | --- | --- |
| C-1 | CRITICAL | **APPLIED** | `:500` `"- **Rule:** **There is no \`Cancelling\` state, durable or otherwise.** Cancellation"` |
| H-1 | HIGH | **APPLIED** | `:434` `"**Three shipping call sites are retired by this rule, not one:** the Package row"` |
| H-2 | HIGH | **APPLIED** | `:521` `"- **Rule:** The opt-out **costs three compensations, and all three are required** —"` |
| H-3 | HIGH | **APPLIED** | `:624` `"- **Confirmation is unavailable while a rebuild is in flight, and after one fails.**"` |
| H-4 | HIGH | **APPLIED** | `:368-369` `"so rather than diverging quietly.** \`prd.md\` states in three places — §7.1, NFR-6,"` / `"and addendum §4 — that the contrast guard and its on-fill ink tokens are"` |
| M-1 | MEDIUM | **RECORDED** | `:1501` `"**The re-declaration obligation itself has no \`AD\`** — \`app.set_menu\` replaces Tauri's default menu wholesale"` |
| M-2 | MEDIUM | **RECORDED** | `:1542` `"| Quit-with-work-in-flight enforcement point | **OPEN — owner decision; surfaced by revision 10, outside its scope** |"` |
| L-1 | LOW | **APPLIED** | `:668` `"reason in the preview's exclusions. **The mise \`tool:rust\` entry is the excluded"` |
| L-2 | LOW | **APPLIED** | `:1441` `"The reduced-motion preference disables transitions and animations; D37 explicitly left that position untouched"` |
| L-3 | LOW | **APPLIED** | `:1199-1200` `"set. Row **ordering is presentation** and may change freely — including the"` / `"outdated-first ordering \`prd.md\` FR-5 leaves unbuilt"` |

**8 APPLIED / 2 RECORDED / 0 NEITHER.**

### `review-reconcile-decisions-v10.md`

| Id | Sev | Disposition | Evidence in the spine |
| --- | --- | --- | --- |
| H-1 | HIGH | **APPLIED** (+ also recorded) | `:649` `"so.** \`docs/DECISIONS.md\` **D15** specifies pinned formulae as a \"Disabled"`; the maintainer route is `:1543` |
| H-2 | HIGH | **APPLIED** | `:393` `"map** — \`⌘R\`, \`⌘⇧R\`, \`⌘⇧U\`, \`⌘L\`, \`⌘F\` and \`⌘1–9\` appear nowhere on it, while"` |
| M-1 | MEDIUM | **RECORDED** | `:1543` `"§4.11 still lists \`⌘U\` and an \`Esc\` clear-selection rung that AD-28 removes."` |
| M-2 | MEDIUM | **APPLIED** | `:1511` `"the count was 22 when this row was written and is 31 after the outline conversion, which is why it is stated without one"`; `grep -n "All 22" ARCHITECTURE-SPINE.md` → no match |
| M-3 | MEDIUM | **APPLIED** | `:434` (same as PRD H-1); the review itself marks it *"fixed in flight at `d34e9190`"* |
| M-4 | MEDIUM | **RECORDED** | `:1543` `"D37's \"Not yet applied\" counts were copied rather than measured and are wrong in all three columns"` |
| M-5 | MEDIUM | **APPLIED** | `:375` `"PRD's §7.1 caveat, NFR-6's \"does not hold at \`HEAD\`\" clause, and addendum §4's"` |
| M-6 | MEDIUM | **RECORDED** | `:1543` `"step 5 tells the verifier that row checkboxes and Manager headers \"stage into the draft plan\", which is D27 target state"` |
| L-1 | LOW | **APPLIED** | `:639` `"explanation wins (\`prd.md\` FR-5, and the \`EXPERIENCE.md\` Checkbox contract — its"` |
| L-2 | LOW | **RECORDED** | `:1543` `"D36's title claims \"D35's on-fill tokens get consumers\" plural, while \`--color-on-success\` still has none"` |
| L-3 | LOW | **APPLIED** | memlog `:212` records the rename; the spine's AD-27 sample now reads by accessible name (see gate-currency MEDIUM-3 below) |

**7 APPLIED / 4 RECORDED / 0 NEITHER.**

### `review-currency-v10b.md`

| Id | Sev | Disposition | Evidence in the spine |
| --- | --- | --- | --- |
| C1 | CRITICAL | **APPLIED** | `:1537` `"AD-27 was the one exception when revision 9 wrote this row … **that has since cleared too**: \`grep -c AD-27 epics.md\` returns 32"`; `grep -n "AD-27 is the exception" ARCHITECTURE-SPINE.md` → no match |
| C2 | CRITICAL | **APPLIED** | `:1539` `"the register's design-token row reads \`CLOSED\` — D35 with \"Nothing blocked\""` |
| C3 | CRITICAL | **APPLIED** | `:1539` `"the \`notarytool minos 15.0\` question reads CLOSED by D34"` |
| H1 | HIGH | **APPLIED** | `:1442` `"| Citations | Cite by **name**, never by position. No line numbers into a document under edit, no \`AD\` rule ordinals, no bare counts in a status row, and no pre-squash commit SHAs."` |
| H2 | HIGH | **APPLIED** | `:1539` `"UX-PB.3d now *states* AD-25's snapshot rule in criterion prose"` |
| H3 | HIGH | **APPLIED** | `:1539` `"AD-21's substance reaches UX-PB.5b's criteria"` |
| M1 | MEDIUM | **APPLIED** | `:1539` `"\`grep -c 'ARCHITECTURE-SPINE.md[:#]*[0-9]'\` returns **0**"` |
| M2 | MEDIUM | **APPLIED** | `:1539` `"(\`epics.md\` never names \`⌘U\` — \`grep -c\` returns 0 — so only \`docs/SPEC.md\` §4.11 carries that limb.)"` |
| M3 | MEDIUM | **APPLIED** | `:1539` `"**UX-PB.1d already carries AD-16's not-native-\`disabled\` rule, and that is exactly the trap**"` |
| M4 | MEDIUM | **APPLIED** | `:1511` (see decisions M-2) |
| L1 | LOW | **APPLIED** | `:1190` `"\"The interface stays interactive beyond 100 Packages, with correct actions"` |
| L2 | LOW | **APPLIED** | `:1039` — verified with `python3` `repr()`: `'service’s em'`, i.e. U+2019, matching the source |
| L3 | LOW | **APPLIED** | `:1539` `"Nor by *this* spine's keyword count, which **rose** in revision 10"` — the "eight" is gone |

**13 APPLIED / 0 RECORDED / 0 NEITHER.**

### `review-rubric-rev10.md`

| Id | Sev | Disposition | Evidence in the spine |
| --- | --- | --- | --- |
| C1 | CRITICAL | **APPLIED** | `:1539` `"**UX-PB.2e and UX-PB.3g still move running work to a \`Cancelling\` state**"` — corrected in both recording sites |
| H1 | HIGH | **RECORDED** | `:1539` `"**Stories 3.4 and 6.5 must gain AD-27**"` — the fix is an `epics.md` edit |
| H2 | HIGH | **RECORDED** | `:1539` `"**Two \`epics.md\` passages now assert the opposite of D36**"` |
| H3 | HIGH | **RECORDED** | `:1539` `"**Story UX-PB.5d is built almost entirely on D37-removed scope**"` |
| H4 | HIGH | **APPLIED** | `:1544` `"**This row gates, and that part is not deferred:** no story may retire the \`ActivityDrawer\` surface until both sinks are named."` |
| M1 | MEDIUM | **APPLIED** | `:1536` `"revision 9's returned **10 / 20 / 29 / 15** — 74 findings, summed from each lens's own tally line"` — I re-summed the four `*-v9` tally tables: divergence 6/8/9/4, rubric 2/5/8/5, reconcile 2/5/7/4, currency 0/2/5/2 → 10/20/29/15. Correct. |
| M2 | MEDIUM | **APPLIED** | `:1512` `"Get the current set from \`grep -rln macos-14 docs/ _bmad-output/*.md\`"` |
| M3 | MEDIUM | **APPLIED** | `:1160-1164` `"- **Binds:** Stories 3.1, 3.2, 3.5; UX-PB.1a–1e … UX-PB.1e and UX-PB.1b are in deliberately"` (the AD-29 half of M3 the reviewer itself judged defensible: *"I could not construct a violation for the AD-29 set"*) |
| M4 | MEDIUM | **RECORDED** | `:1539` `"**Story 3.2's pinned-row criterion still says \`disabled\`**"` |
| M5 | MEDIUM | **APPLIED** | `:1539` `"the stories AD-18 *binds* and that cite it nowhere are **UX-PB.2c, UX-PB.2d, UX-PB.2f and UX-PB.4b**"` — exactly M5's correction |
| L1 | LOW | **APPLIED** | memlog `:221` `"L1: AD-12's rule now points at the table row that calls it unimplementable"` |
| L2 | LOW | **RECORDED** | `:1539` `"**UX-PB.2c's persist-failure criterion may read against AD-29**"` |

**7 APPLIED / 5 RECORDED / 0 NEITHER.**

### `review-currency-rev10.md`

| Id | Sev | Disposition | Evidence in the spine |
| --- | --- | --- | --- |
| MEDIUM-1 | MEDIUM | **APPLIED** | `:1512` (see rubric M2) |
| MEDIUM-2 | MEDIUM | **APPLIED** | `:1521` `"the CrabNebula fork alternative carries a cost — \`llms-full.txt\` says it works on all platforms, \"a paid API key is required for macOS\""` |
| MEDIUM-3 | MEDIUM | **APPLIED** | memlog `:216` `"AD-27's named sample was 'a toolbar button' and the actual sample is the Refresh All button … renamed by accessible name"` |
| MEDIUM-4 | MEDIUM | **APPLIED** | `:368-375` (same supersession as PRD H-4) |
| LOW-1 | LOW | **APPLIED** | `:1039` verified U+2019 (see currency-v10b L2) |
| LOW-2 | LOW | **APPLIED** | `:1341` `"\`src-tauri/src/journal.rs\` is \"One line at op start, one at finish, flushed each"` — capital restored **at the cited location** (but see §4.4) |
| LOW-3 | LOW | **APPLIED** | `:1096` `"indicator. What v3 called \`outline-none\` was **not** a no-op — it set a transparent"` |
| LOW-4 | LOW | **APPLIED** | memlog `:215`; the baseline and AD-11 now split gate from detector |

**8 APPLIED / 0 RECORDED / 0 NEITHER.**

### `review-divergence-rev10.md`

| Id | Sev | Disposition | Evidence in the spine |
| --- | --- | --- | --- |
| C-1 | CRITICAL | **APPLIED** | AD-28's removal rule is now a closed three-way taxonomy (memlog `:224`) |
| C-2 | CRITICAL | **APPLIED** | memlog `:226`; `:1202-1203` `"filtered set the projection holds**, which includes off-screen virtualized rows"` |
| H-1 | HIGH | **APPLIED** | `:1314-1320` `"this rule ratifies the admission half and **overrides the terminal half** … UX-PB.4a is a **reader and folder only**"` |
| H-2 | HIGH | **APPLIED** | memlog `:228` — the archive carries raw journal lines per AD-18 |
| H-3 | HIGH | **APPLIED** | memlog `:228` — `PlanAttempt.state` is a derived read-model value, never persisted |
| H-4 | HIGH | **APPLIED** | `:734` `"- **Rule:** Below 720 usable CSS pixels the region stops being a fixed sidecar and"`; `:744-746` `"reachable **without navigation** … a **persistent, non-occludable indicator that routes to it**"` |
| H-5 | HIGH | **RECORDED** | `:1539` `"**This item is ordering-critical:** UX-PB.2e ships in wave 2 … it must be corrected *before* UX-PB.2e is built"` |
| M-1 | MEDIUM | **RECORDED** | `:1539` `"**Story 3.5 needs a criterion for the non-list views**"` |
| M-2 | MEDIUM | **APPLIED** | `:920` `"and the failure is surfaced inline **in the surface that owns the attempt** — by"` |
| M-3 | MEDIUM | **APPLIED** | `:1202-1203` and `:1289` `"matching the active filter**, including off-screen virtualized rows"` |
| L-1 | LOW | **APPLIED** | `:1162-1164` — UX-PB.1b added to AD-28's `Binds` with the reason inline |
| L-2 | LOW | **RECORDED** | `:1539` `"**Story 3.2's Dependencies should name UX-PB.1d**"` |
| L-3 | LOW | **APPLIED** | `:307-308` `"- **Binds:** persistence and lifecycle work; Stories 3.4, 6.5; UX-PB.1b, UX-PB.2c, UX-PB.4a"`; `:1523` adds UX-PB.4a to the crash/relaunch row |
| L-4 | LOW | **RECORDED** | `:1539` `"**UX-PB.2d's \"where applicable\"** is undefined for verification refreshes"` |

**11 APPLIED / 3 RECORDED / 0 NEITHER.**

### Totals

| Disposition | Count |
| --- | --- |
| APPLIED | **54** |
| RECORDED | **14** |
| **NEITHER** | **0** |
| Total | 68 |

**No finding from the six revision-10 lenses is untracked.** That is the answer to the
question as posed, and it is a real result: I checked each remedy against the spine
rather than against the run's account, and every one lands somewhere.

**The gap is one level up** — see §2.1.

---

## 2. Finalize steps not performed

Walking `SKILL.md:66-77` literally.

### 2.1 Step 2 — "Reconcile inputs. A subagent per load-bearing input" — PARTIAL

`SKILL.md:71`: *"**Reconcile inputs.** A subagent per load-bearing input checks it
against the spine and returns what didn't land — especially a quiet requirement (a
tone, a constraint) the `AD` structure dropped. Before the gate."*

The spine's own frontmatter names **15** sources (`ARCHITECTURE-SPINE.md:15-33`).
Three reconcilers ran this revision: `prd.md`+`addendum.md`, `docs/DECISIONS.md`, and
the tree (currency). Sources with **no reconciler this revision**:

- `_bmad-output/planning-artifacts/ux-designs/ux-Pack-Manager-2026-07-23/EXPERIENCE.md`
- `_bmad-output/planning-artifacts/ux-designs/ux-Pack-Manager-2026-07-23/DESIGN.md`
- `docs/SPEC.md`
- `docs/RELEASE-CHECKLIST.md`

`ls -la reviews/` shows the only UX reconciler is `review-reconcile-ux.md`, mtime
`Jul 25 08:22` — a prior revision. No `*-v10` / `*-rev10` UX, SPEC, or checklist
reconciler exists.

This matters concretely rather than procedurally: **AD-28 is built on a single quoted
line of `EXPERIENCE.md`** — `ARCHITECTURE-SPINE.md:1194-1195` `"build and no \`Add
Selected\` submit step (\`EXPERIENCE.md\`: \"On eligible Package"` / `"rows, selection
immediately adds/removes Upgrade Plan membership\";"` — and the run separately concluded that `EXPERIENCE.md` still carries
the `Cancelling` state (`:1539` `"\`EXPERIENCE.md\` carries the state too, so it needs
the \`bmad-ux\` Update"`). Nothing checked what *else* in `EXPERIENCE.md` /
`DESIGN.md` the new AD-28 / AD-29 / D37 material dropped or contradicted. That is
exactly the "quiet requirement the `AD` structure dropped" the step exists to catch.

### 2.2 Step 4 — "Triage … the rest deferred with a revisit condition" — PARTIAL

`SKILL.md:73`: *"**Triage.** Open questions and `[ASSUMPTION]` tags: blockers (unsafe
for what's next) resolved one at a time; the rest deferred with a revisit condition in
the memlog."*

- `[ASSUMPTION]` tags: `grep -c "\[ASSUMPTION\]" ARCHITECTURE-SPINE.md` → **0**. Clean.
- Revisit conditions: revision 10 opened **four** record-only rows
  (`DRIFT-NOTE.md:901` `"Four rows, all outside the scope the owner set, all with the
  fix named:"`). Exactly **one** carries a revisit condition:
  `ARCHITECTURE-SPINE.md:1528` `"| Post-publish operational envelope | **Deferred —
  revisit on the first bad published release, or on any updater-key change** |"`.

  The other three carry an **owner** but no condition under which they come back:
  - `:1534` `"| AD-12's file-scoped \"never hand-edited\" | **OPEN — surfaced by
    \`reviews/VALIDATION-REPORT-2026-07-25.md\` F10; outside revision 10's scope** |"`
  - `:1542` `"| Quit-with-work-in-flight enforcement point | **OPEN — owner decision;
    surfaced by revision 10, outside its scope** |"`
  - `:1544` `"| Accelerator sinks that AD-17 moved: \`⌘L\` and \`Esc\`'s second rung |
    **OPEN — owner decision; surfaced by revision 10, outside its scope** |"`
    (this one carries a *gate*, which constrains stories — not a revisit trigger)

  Nor does the memlog carry conditions for them: memlog `:187` and `:198` record them
  as offered/recorded, with no trigger stated.

### 2.3 Step 5 — "Renderings & polish" — NO ARTIFACT, NO RECORD

`SKILL.md:74`: *"…produce any *additional* human-facing artifact the user needs…
The up-front question already flagged whether one's needed; if it wasn't, still offer
one here, seeding concrete options: an interactive HTML+SVG deck…, a fuller HTML/md
solution design, a C4 set, or a view of how the work splits across teams/epics."*

```
$ find …/architecture-Pack-Manager-2026-07-23/ -type f ! -name "*.md"
(no output)
```

The run folder contains only markdown. No deck, no HTML, no C4 set, no team/epic split
view exists. `grep -ni "rendering\|html\|deck\|C4\b" .memlog.md` returns nothing from
the revision-10 range — **no memlog entry records the offer being made or declined.**
Whether the offer was made conversationally is **UNVERIFIED**; what is verifiable is
that no artifact and no record of the step exists.

(`{workflow.doc_standards}` is `["skill:bmad-review lenses=structure,prose"]` and
applies *only* to such a prose artifact, so its non-execution is a consequence of this,
not a separate miss.)

### 2.4 Step 6 — "External handoffs" — VACUOUS + ONE UNRECORDED OFFER

`customize.toml` sets `external_handoffs = []`, so the first half is a no-op. The
second half — `SKILL.md:75` *"Offer to invoke the `bmad-spec` skill to adopt the spine
as a companion, keeping `AD` IDs stable so downstream can cite them"* — has **no
record**. `grep -n "bmad-spec" .memlog.md` → no match anywhere in the file; likewise
in `ARCHITECTURE-SPINE.md` and `DRIFT-NOTE.md`.

### 2.5 Step 7 — "Close" — DONE except the next-skill routing

Verified done:
- `ARCHITECTURE-SPINE.md:8` `"status: final"`, `:9` `"updated: \"2026-07-25\""`,
  `:11` `"artifact_revision: 10"`.
- memlog `:231` `"- (event) spine finalized at revision 10."`

**Not done / not recorded:** `SKILL.md:76` *"Next, **lead with `bmad-spec`** — recommend
adopting/refreshing the spine as a spec companion (always the top recommendation when a
spec was an input, and a useful next step even when it wasn't), then
`bmad-create-epics-and-stories` or — epic altitude — `bmad-create-story`; or invoke
`bmad-help` to route."*

`docs/SPEC.md` **is** a declared input (`ARCHITECTURE-SPINE.md:18`). No artifact
records any next-skill recommendation of `bmad-spec`, `bmad-create-epics-and-stories`,
`bmad-create-story`, or `bmad-help`. The only forward routing anywhere is to
`bmad-correct-course` and `bmad-ux`, which are consequences of the residuals row rather
than the close step's recommendation. Whether the routing was spoken is **UNVERIFIED**;
no deliverable carries it.

### 2.6 Step 8 — `{workflow.on_complete}` — vacuous

`customize.toml`: `on_complete = ""`. Nothing to run.

### 2.7 Reviewer Gate mechanics — fully satisfied

For completeness, the gate itself checks out. `reviewer-gate.md:5` requires the
deterministic pass; I re-ran it this session:

```
$ uv run .claude/skills/bmad-architecture/scripts/lint_spine.py --workspace …
{"ok": true, "spine": "ARCHITECTURE-SPINE.md", "total_findings": 0, "by_severity": {}, "findings": []}
```

`reviewer-gate.md:7` requires the rubric walker **plus every**
`{workflow.finalize_reviewers}` entry. `customize.toml` lists two (currency lens,
adversarial-divergence lens); both ran (`review-currency-rev10.md`,
`review-divergence-rev10.md`), plus the rubric walker. `reviewer-gate.md:9` requires
each to write to `{doc_workspace}/reviews/review-{slug}.md`; all six files exist there.

---

## 3. Claims that are not true

### 3.1 The spine cites a file that does not exist — `ARCHITECTURE-SPINE.md:1535`

> `"the per-finding disposition is audited in \`reviews/review-completeness-rev10.md\`
> rather than tallied here, because a count in a status row is what this folder keeps
> getting wrong."`

```
$ ls -la …/reviews/review-completeness-rev10.md
ls: cannot access '…': No such file or directory
```

At the moment revision 10 was closed, the spine's **only** stated record of the
per-finding gate disposition pointed at a file that did not exist. This audit is that
file; before it, the citation resolved to nothing. This is the sixth instance of the
reference-rot class the same revision wrote a convention against
(`:1442` `"| Citations | Cite by **name**, never by position."`).

### 3.2 `gate-currency 0/0/4/3` is wrong — it is 0/0/4/**4** — stated in all three deliverables

`reviews/review-currency-rev10.md:41-42`, the lens's own tally table:

```
| MEDIUM | 4 |
| LOW | 4 |
```

Confirmed by heading count — the file carries `## LOW-1`, `## LOW-2`, `## LOW-3`,
`## LOW-4` (`grep -n "^## LOW" review-currency-rev10.md` → 4 lines: 246, 284, 319, 367).

The wrong figure appears in every deliverable:
- `ARCHITECTURE-SPINE.md:1535` `"Counts are each lens's own tally: 1/4/2/3, 0/2/6/3,
  3/3/4/3, 1/4/5/2, **0/0/4/3**, 2/5/3/4"`
- `DRIFT-NOTE.md:860` `"gate-rubric 1/4/5/2, gate-currency **0/0/4/3** (94 claims"`
- `.memlog.md:231` `"gate-currency **0/0/4/3**, divergence 2/5/3/4."`

All three sentences claim the numbers are "each lens's own tally". One is not. The
total is 68 findings, not 67.

This is the exact defect `review-rubric-rev10.md` M1 raised against the revision-9 row
(*"The revision-9 gate counts in the tail row are wrong; they were copied, not
measured"*) — the run fixed the revision-9 counts and then reproduced the same class of
error one row down, in its own.

### 3.3 "Five independent lenses" — there were six

- `DRIFT-NOTE.md:853-854`: `"Five independent lenses: three reconcilers (PRD,
  \`docs/DECISIONS.md\`, currency against the tree) and three gate lenses (rubric
  walker, currency/reality, adversarial divergence)"` — the enumeration is 3 + 3 = **6**
  under a headline of five.
- `.memlog.md:231`: `"Reviewer gate: five independent lenses plus the revision-9
  VALIDATION-REPORT found in-folder mid-run."` — then lists **six** tallies.

The spine gets it right: `:1535` `"Six lenses ran: three reconcilers … and three gate
lenses …"`. So the false count is in the memlog and the change record, and it
**contradicts the spine** (see §4.1). `ls -la` shows the spine's mtime is `13:27` and
both `.memlog.md` and `DRIFT-NOTE.md` are `13:29` — the two files written *last* carry
the stale number.

### 3.4 "Positional and numeric references have rotted **five** times" — the list has six

`DRIFT-NOTE.md:922-925`:

> `"Positional and numeric references have rotted **five** times in this folder: AD-16's
> rule ordinals, \`epics.md\` line numbers (twice), this spine's own line numbers, a count
> that went stale inside the revision that wrote it, and now commit hashes — six of the
> seven the spine cites resolve locally but sit on no branch after a squash merge."`

Enumerated: (1) AD-16 rule ordinals, (2)+(3) `epics.md` line numbers *twice*, (4) the
spine's own line numbers, (5) a stale count, (6) commit hashes = **six** items after
"five". A bare count in prose, wrong, inside the paragraph announcing the convention
against bare counts.

### 3.5 "Every CRITICAL and HIGH was applied" is false as stated — `ARCHITECTURE-SPINE.md:1535`

> `"Every CRITICAL and HIGH was applied; the MEDIUM and LOW tail was applied or recorded
> in a row"`

The sentence reserves "recorded in a row" for the MEDIUM/LOW tail. Four HIGHs were
**recorded, not applied** — their remedy is an `epics.md` edit the run was forbidden to
make, so they sit in the residuals row exactly like the MEDIUM/LOW tail:

- `review-rubric-rev10.md` **H1** → `:1539` `"**Stories 3.4 and 6.5 must gain AD-27**"`
- `review-rubric-rev10.md` **H2** → `:1539` `"**Two \`epics.md\` passages now assert the opposite of D36**"`
- `review-rubric-rev10.md` **H3** → `:1539` `"**Story UX-PB.5d is built almost entirely on D37-removed scope**"`
- `review-divergence-rev10.md` **H-5** → `:1539` `"**This item is ordering-critical:** UX-PB.2e ships in wave 2"`

The work is tracked — that part is fine — but a reader taking the sentence at face
value would believe no HIGH remains outstanding, and four do.

### 3.6 The `VALIDATION-REPORT` disposition accounts for 8 of 12 findings

`grep -n "^## F" VALIDATION-REPORT-2026-07-25.md` → **12** findings, F1–F12.

All three deliverables give the same disposition and it names only eight:
- `.memlog.md:191` `"F1, F2, F3, F7 and F8 resolved this revision; F10, F11 and F12
  recorded as rows rather than fixed"`
- `DRIFT-NOTE.md:856-857` `"whose F1, F2, F3, F7 and F8 this revision resolved and whose
  F10, F11 and F12 it recorded."`
- `ARCHITECTURE-SPINE.md:1536` `"whose F1, F2, F3, F7 and F8 revision 10 resolved, and
  whose F10, F11 and F12 it recorded rather than fixed."`

**F4, F5, F6 and F9 are named in no disposition list.** Checked individually against the
tree — all four were in fact discharged, three of them as side effects, and none was
credited:

| Id | Sev | Actual state | Evidence |
| --- | --- | --- | --- |
| **F4** | HIGH | Remedy taken *this revision* — F4 offered "a re-order … or a supersede note"; the note was added. The misordering itself remains (`## 4` at `:214` still sits above `## 2d`/`2e`/`2f`/`2g` at `:248`/`:317`/`:614`/`:795`). | `DRIFT-NOTE.md:226` `"(\`reviews/VALIDATION-REPORT-2026-07-25.md\` F4)."` — `git diff` confirms the whole block is a revision-10 addition (`@@ -207 +214,13 @@`) |
| **F5** | MEDIUM | Resolved by the *gate-currency MEDIUM-1* route instead | `ARCHITECTURE-SPINE.md:1512` `"The set is not enumerated here because it rots"` |
| **F6** | MEDIUM | Moot — the row carrying the spliced quotation was rewritten to `RESOLVED` | `grep -n "clears the transient selection" ARCHITECTURE-SPINE.md` → no match |
| **F9** | HIGH | Resolved as a consequence of the D37 AD-27 rewrite | `:1143-1146` `"D37 removed the manual keyboard-and-VoiceOver pass this rule previously named as the fallback … What replaces it is the rule above: a story adding an interactive control verifies that control at runtime in a macOS Tauri build"` |

So the *work* is not missing; the **accounting** is. And that matters here more than
usual, because the tail row's own closing rule is
`:1536` `"Anything not named in a row is neither closed nor scheduled; the review files
are the inventory."` Under that rule, F4, F5, F6 and F9 read as neither closed nor
scheduled, when in fact all four are closed. The row understates the run's own work and
leaves four findings in an undefined state.

### 3.7 Claims verified TRUE (checked, not assumed)

Listed so the negative findings above are not mistaken for a general verdict.

- `.memlog.md:231` `"AD inventory 1-5, 11, 12, 16-29 - 21 live ids, zero duplicates"` —
  `grep -n "^### AD-"` returns exactly AD-1..5, 11, 12, 16..29 = **21** headings, no
  duplicates, no headings for the retired 6–10 / 13–15.
- `.memlog.md:231` `"Deterministic lint zero findings"` — re-run this session, `0`.
- `.memlog.md:231` `"both mermaid diagrams re-rendered valid SVG"` —
  `grep -c '\`\`\`mermaid'` → **2**. (SVG validity itself: **UNVERIFIED**, I did not
  re-render.)
- `.memlog.md:231` / `DRIFT-NOTE.md:804-805` `"epics.md untouched throughout … nothing
  committed or staged"` — `git status --porcelain` shows `epics.md` absent from the
  change set, and all six review files untracked / three deliverables unstaged.
- `ARCHITECTURE-SPINE.md:1536` `"revision 8's gate returned 8 CRITICAL / 15 HIGH / 17
  MEDIUM / 10 LOW"` — re-summed from each `*-v8` file's own counts line: divergence
  4/4/4/2 (`:21`), rubric 1/3/5/3 (`:697`), reconcile 2/5/6/3 (`:57-60`), currency
  1/3/2/2 (`:54`) → **8/15/17/10**. Exact.
- `ARCHITECTURE-SPINE.md:1536` `"revision 9's returned **10 / 20 / 29 / 15** — 74
  findings"` — re-summed from the four `*-v9` tally tables → exact.
- All six revision-10 review filenames listed at `:1535` resolve on disk.

---

## 4. Internal inconsistencies between the three deliverables

### 4.1 Lens count: spine says six, memlog and DRIFT-NOTE say five

`ARCHITECTURE-SPINE.md:1535` `"Six lenses ran"` vs `DRIFT-NOTE.md:853` `"Five
independent lenses"` vs `.memlog.md:231` `"five independent lenses"`. Same run, same
six files enumerated, two different totals. Detail in §3.3.

### 4.2 The `Cancelling` story attribution — fixed in the spine, still wrong in the DRIFT-NOTE's own narration? No — but the DRIFT-NOTE omits the AD-29 half

`DRIFT-NOTE.md:870-871` correctly reports the correction: `"filed against UX-PB.2f and
UX-PB.4c, which contain the word nowhere. It is UX-PB.2e and UX-PB.3g."` Consistent
with `ARCHITECTURE-SPINE.md:1539`. **No inconsistency here** — recorded because it is
the first place one would look.

### 4.3 The plan-attempt status row does not carry the divergence H-1 reversal

`ARCHITECTURE-SPINE.md:1541` (the `RESOLVED` row for the closed Open item):

> `"AD-29 also names the single writer (the Rust plan-attempt store, with History,
> replay and diagnostics as readers), which resolves the UX-PB.3d / UX-PB.4a
> double-write"`

`AD-29` itself was reversed mid-gate and now says something materially different about
who owns the terminal append — `:1314-1320` `"this rule ratifies the admission half and
**overrides the terminal half**. UX-PB.2c appends at admission. The **terminal append
fires on the attempt's terminal transition inside the store** … owned by whichever story
first makes an attempt reachable terminal — **UX-PB.2e** … UX-PB.4a is a **reader and
folder only**"`. The status row's summary is not false, but it stops at the pre-reversal
account, so the row and the `AD` it summarises tell a reader two different stories about
which story owns the write. The same applies to `DRIFT-NOTE.md:812`, whose table cell
for this row makes no mention of the reversal (it appears only later, at `:888-891`).

### 4.4 The same `journal.rs` quotation is verbatim in the `AD` and non-verbatim in the status row

`review-currency-rev10.md` LOW-2 caught the lowercased first word and it was fixed at
the cited location — `:1341` `"\`src-tauri/src/journal.rs\` is \"One line at op start,
one at finish, flushed each"`. The **second** occurrence was not:

`:1541` `"The shape ratifies the shipping Operation journal rather than inventing one
(\`src-tauri/src/journal.rs\`: \"one line at op start, one at finish\")."`

Source is `src-tauri/src/journal.rs:4` `"//! One line at op start, one at finish, flushed
each write."`. One
document now carries the same source sentence two ways, one of them inside quotation
marks and not verbatim — the failure class this run folder names as the one it breaks
most often (`.memlog.md:207` `"the verbatim-quotation rule is the one this run folder
has broken most often"`).

### 4.5 The DRIFT-NOTE is still physically misordered

`grep -n "^## " DRIFT-NOTE.md`:

```
26:## 1.   40:## 2.   121:## 2b.  169:## 3.   190:## 2c.
214:## 4.  248:## 2d.  317:## 2e.  614:## 2f.  795:## 2g.
```

`## 3` (`:169`) and `## 4` (`:214`) are revision-4/5-era content sitting above the
revision 7–10 sections. Revision 10 added a supersede banner at `:215-226`, which
satisfies the letter of `VALIDATION-REPORT` F4's remedy ("either a re-order … or a
supersede note"), so this is recorded as an observation rather than a violation — but
the spine points every reader here as its change record
(`ARCHITECTURE-SPINE.md:100` `"> Open items below. Change record: \`DRIFT-NOTE.md\`."`,
repeated at `:51`, `:75`, `:89`, `:112`) and a top-down reader still reaches revision-5
open items before revision 10.

---

## 5. Is revision 10's own reviewer-gate tail tracked?

**Yes — the F11 gap was not recreated, and the row says so explicitly.**

`ARCHITECTURE-SPINE.md:1535`:

> `"| Reviewer-gate tail (revision 10) | **Open — inventory only** | Recorded because the
> row below exists precisely for the failure of *not* recording this, and revision 10
> would otherwise have recreated it for its own gate. Six lenses ran: three reconcilers
> (\`review-reconcile-prd-v10.md\`, \`review-reconcile-decisions-v10.md\`,
> \`review-currency-v10b.md\`) and three gate lenses (\`review-rubric-rev10.md\`,
> \`review-currency-rev10.md\`, \`review-divergence-rev10.md\`), plus
> \`VALIDATION-REPORT-2026-07-25.md\` from a prior Validate pass against revision 9."`

All six named files exist. The prior gap it answers is
`reviews/VALIDATION-REPORT-2026-07-25.md:448` `"## F11 — HIGH — the revision 8 and
revision 9 reviewer-gate tails are tracked nowhere"`, and the pre-existing row at
`:1536` was widened to cover revisions 8, 9 and the validation pass.

**Three defects inside the row**, all covered above and all in the same class the row
was created to prevent:

1. Its per-lens tally reproduces the copied-count error: `0/0/4/3` for a lens whose own
   table says `4` LOW (§3.2).
2. Its "Every CRITICAL and HIGH was applied" is false for four HIGHs (§3.5).
3. Its escape from tallying — deferring the per-finding disposition to
   `reviews/review-completeness-rev10.md` — pointed at a file that did not exist (§3.1).

The row's design is right; its contents were not verified against the files it names.

---

## Summary of what is missing

| # | What | Where |
| --- | --- | --- |
| 1 | Spine cites `reviews/review-completeness-rev10.md`, which did not exist | `ARCHITECTURE-SPINE.md:1535` |
| 2 | `gate-currency 0/0/4/3` — the lens's own tally is `0/0/4/4`; total is 68, not 67 | spine `:1535`, DRIFT-NOTE `:860`, memlog `:231` |
| 3 | "Five independent lenses" against a six-item enumeration; contradicts the spine | DRIFT-NOTE `:853`, memlog `:231` |
| 4 | "rotted **five** times" against a six-item enumeration | DRIFT-NOTE `:922` |
| 5 | "Every CRITICAL and HIGH was applied" — 4 HIGHs were recorded, not applied | spine `:1535` |
| 6 | `VALIDATION-REPORT` disposition names 8 of 12 findings; F4, F5, F6, F9 uncredited | spine `:1536`, DRIFT-NOTE `:857`, memlog `:191` |
| 7 | No reconciler ran against `EXPERIENCE.md`, `DESIGN.md`, `docs/SPEC.md`, `docs/RELEASE-CHECKLIST.md` — and AD-28 rests on one `EXPERIENCE.md` line | Finalize step 2 |
| 8 | 3 of the 4 rows revision 10 opened carry no revisit condition | Finalize step 4; spine `:1534`, `:1542`, `:1544` |
| 9 | No rendering artifact and no record of the offer | Finalize step 5 |
| 10 | No record of the `bmad-spec` handoff offer or of any next-skill routing | Finalize steps 6–7 |
| 11 | Status row `:1541` summarises AD-29 pre-reversal, contradicting AD-29 at `:1314` | spine, DRIFT-NOTE `:812` |
| 12 | `journal.rs` quotation verbatim at `:1341`, non-verbatim at `:1541` | spine |

**Per-finding buckets across the six revision-10 lenses: 54 APPLIED / 14 RECORDED /
0 NEITHER (68 total).**

---

## Method

Read this session, in full or in the cited ranges: `SKILL.md`, `reviewer-gate.md`,
`customize.toml`, `ARCHITECTURE-SPINE.md` (frontmatter, `AD` headers, `:306-312`,
`:368-380`, `:390-400`, `:425-680`, `:730-780`, `:900-930`, `:1030-1210`, `:1280-1345`,
`:1427-1546`), `.memlog.md:164-231`, `DRIFT-NOTE.md:214-232` and `:795-929`, all six
revision-10 review files (headings, verdicts, tallies, and the finding bodies cited),
and `VALIDATION-REPORT-2026-07-25.md` finding headings plus F4, F6, F9 bodies.

Commands whose output is quoted above: `ls -la`, `wc -l`, `find … ! -name "*.md"`,
`git status --porcelain`, `git diff --unified=0`, `grep -n`, `grep -c`,
`python3 -c "…repr(…)"` for the U+2019 check, and
`uv run .claude/skills/bmad-architecture/scripts/lint_spine.py --workspace …`.

No file in the run folder was modified by this audit other than the creation of this
file.
