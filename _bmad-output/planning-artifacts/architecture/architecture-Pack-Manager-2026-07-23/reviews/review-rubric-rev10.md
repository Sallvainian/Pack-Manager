# Rubric Walk — `ARCHITECTURE-SPINE.md` revision 10

**Target:** `_bmad-output/planning-artifacts/architecture/architecture-Pack-Manager-2026-07-23/ARCHITECTURE-SPINE.md`
**Snapshot judged:** 1395 lines, `sha256 a9fc1d9433cdaba8ec910340ff66ee2821c65346a46dea73f8f84c9548607f6c`, taken 2026-07-25 13:06:48 to `/tmp/spine-rev10-snapshot.md`.
**Note on the moving target:** the file was 1338 lines when this run opened it and 1395 lines eight minutes later — AD-11 gained the PRD-supersession rule, AD-16 gained the three-call-sites, no-`Cancelling`, rebuild-in-flight, `rustDedup`-direction and three-compensations rules, AD-28 gained the ordering and SPEC §4.11 rules, and two Open rows were added. **Every line number below is the snapshot's.**
**Repo state:** working tree; `HEAD` `1ac959e` (`git log -1 --format="%h %s"`).
**Mode:** read-only. The spine was not modified.

Every finding carries a `path:line "literal text"` citation from a file opened during this run. Every count states the command that produced it.

---

## Verdict

**READY WITH FIXES.**

The invariants themselves are sound, and that is the part that matters. **AD-28** and **AD-29** are correctly shaped: each names a pair of units that obey every prior `AD` and still build incompatibly, states rules that reach that divergence, and ratifies a shipping referent rather than inventing one. I checked every `Prevents` clause in the new and changed material against its own `Rule` list and found **no** instance of the revision-5 defect — nothing is prevented that no rule forbids. The deterministic lint is clean. Every technical claim I sampled against the tree verified exactly (commits, versions, counts, call sites, CI pins, the D36 guard, the `⌘A` `preventDefault` defect, the three direct-execution call sites).

What is defective is the **hand-off record**, not the substrate. `epics.md` is deliberately unedited this revision, so the residuals row *is* the entire mechanism by which revision 10's decisions reach the stories they govern — and it instructs the next run to scope "by named heading, not by mention count". Two of its named headings are wrong and four divergences revision 10 itself created are not named at all. A correct-course run executed literally against it would edit two stories that carry no defect, leave the two that do, and leave four more untouched.

That is an editing pass over one table plus one rule, not a redesign. Hence READY WITH FIXES rather than NOT READY.

### Tally (my own)

| Tier | Count |
| --- | --- |
| CRITICAL | 1 |
| HIGH | 4 |
| MEDIUM | 5 |
| LOW | 2 |
| **Total** | **12** |

### Deterministic pass, run this session

```
uv run .claude/skills/bmad-architecture/scripts/lint_spine.py \
  --workspace _bmad-output/planning-artifacts/architecture/architecture-Pack-Manager-2026-07-23/
→ "total_findings": 0, "by_severity": {}, "findings": []
```

---

## CRITICAL

### C1 — The `Cancelling` retirement is filed against the wrong two stories, twice, in the only two places that record it

Revision 10 removed `Cancelling` as a state. The rule:

- `ARCHITECTURE-SPINE.md:480` `"- **Rule:** **There is no \`Cancelling\` state, durable or otherwise.** Cancellation"`
- `:486–488` `"\`epics.md\` (UX-PB.2f and UX-PB.4c, \"changes"` / `"still-running Operations … to \`Cancelling\`\") and \`EXPERIENCE.md\` still carry the"` / `"state; both are superseded and are recorded as divergences"`

And the residuals row repeats the same attribution:

- `:1389` `"**UX-PB.2f and UX-PB.4c still move running work to a \`Cancelling\` state** that \`prd.md\` FR-13 forbids by name and AD-16 now refuses — \"changes still-running Operations bound to that \`planAttemptId\` to \`Cancelling\`\""`

**Both story ids are wrong.** `grep -n "Cancelling" _bmad-output/planning-artifacts/epics.md` returns exactly two lines:

- `epics.md:740` (inside **UX-PB.2e**, heading at `:728` `"### Story UX-PB.2e: Plan-level cancellation that skips unstarted work and escalates running process groups"`) — `"running work moves to \`Cancelling\` and escalates through the existing process-group mechanics"`
- `epics.md:903` (inside **UX-PB.3g**, heading at `:891` `"### Story UX-PB.3g: Two labeled cancellation scopes"`) — `"changes still-running Operations bound to that \`planAttemptId\` to \`Cancelling\`"`

UX-PB.2f (`:748`–`:765`) and UX-PB.4c (`:960`–`:977`) contain no occurrence of the word. The quotation in the spine is *verbatim from UX-PB.3g:903* — so the spine quoted the right text and attributed it to the wrong story.

The spine contradicts itself on this inside one document. AD-29's `Prevents` gets the same two stories right:

- `:1196–1197` `"the candidates are **four**, not the two \`epics.md\` names: UX-PB.2e drives an"` / `"attempt terminal through cancellation and UX-PB.3g is equally exposed"`

**Why CRITICAL rather than a typo.** The residuals row is `"**OPEN — record only; do not edit \`epics.md\` here**"` (`:1389`) — it is the sole channel from this revision to the stories, and it closes by instructing `"Scope that run **by named heading, not by mention count**"` (`:1389`). Followed literally, the next `bmad-correct-course` run opens UX-PB.2f and UX-PB.4c, finds nothing to remove, and closes. UX-PB.2e and UX-PB.3g keep a criterion that requires an `OpStatus` variant the requirements authority forbids by name — `prd.md:344` `"There is no distinct \`cancelling\` state: cancellation moves an Operation to its terminal state, and the 5-second SIGTERM grace window is not surfaced as its own status."` — and adding it is an atomic AD-3 boundary change across the Rust enum, `src/lib/ipc/types.ts`, the guards and `dev/fixtures/ipc/*.json` (`:476–478`). A story that builds it and a story that does not produce incompatible fixtures.

This is also the fifth instance of the reference-rot class the run folder already tracks; the row's own closing sentence warns about it — `:1389` `"Cite by \`AD\` id and subject, never by rule ordinal and never by line number."` — and then misses on story id.

**Fix:** replace `UX-PB.2f and UX-PB.4c` with `UX-PB.2e and UX-PB.3g` at `:486` and `:1389`. One-word-each correction; no invariant changes.

---

## HIGH

### H1 — AD-27's `Binds` was widened to Stories 3.4 and 6.5 this revision, and the residuals row does not record that `epics.md` still omits it

AD-27 now binds two stories it did not bind in revision 9, and says so:

- `:1013–1018` `"- **Binds:** every story that renders an interactive control — all of Epic UX-PB"` / `"and, of the surviving Epic 1–6 stories, **3.1, 3.2, 3.4, 3.5 and 6.5** — plus any"` / `"change to \`src/styles/theme.css\` or the style contract. Story 3.4 renders Settings"` / `"controls and Story 6.5 renders the diagnostics action, so the Rule below covers"` / `"both; an earlier enumeration omitted them, and because a prose preamble followed"` / `"by a closed list resolves as the list, \`epics.md\` faithfully mirrored the omission."`

The Capability Map moved with it — `:1353` `"| Settings and Environment Report (Story 3.4) | Settings persistence + detection state | AD-19, AD-21, **AD-27** |"` and `:1354` routes Story 6.5 to `**AD-27**`.

`epics.md` did not, and nothing schedules it:

- `epics.md:1247` `"- Governing invariants: AD-4, AD-5, AD-19"` (Story 3.4)
- `epics.md:1302` `"- Governing invariants: AD-3, AD-4, AD-5, AD-16, AD-18, AD-26"` (Story 6.5)

Verified those blocks carry no AD-27 anywhere: `sed -n '1237,1262p' epics.md | grep -n "AD-"` → one line, `:1247` above; `sed -n '1292,1319p' epics.md | grep -n "AD-"` → `:1301` and `:1302`, both AD-26/AD-18, neither AD-27.

The residuals row's only statement about AD-27 is *clearance evidence*: `:1389` `"\`grep -c AD-27 epics.md\` returns 32"`. That count is correct (`grep -c "AD-27" epics.md` → `32`) and it is exactly the wrong measure — it was true before this revision widened the list, so it reads as "done" while the two newly bound stories are the two that still do not read the rule. This is `VALIDATION-REPORT-2026-07-25.md` F8 reproduced one level down: F8's whole point was that a closed enumeration binds nobody outside it, and the fix has landed in the spine and not in the record that carries it downstream.

### H2 — D36's guard landing makes two `epics.md` passages false, and the residuals row records neither

Revision 10 folded D36 in and asserts the guard ships:

- `:1060–1062` `"(\`docs/DECISIONS.md\` D36, commit \`a201fb0\`). The guard in the same style-contract"` / `"lane measures the **rendered** foreground and background of a named sample and"` / `"fails below 4.5:1"`

Verified in the tree: `git log -1 --format="%h %ad %s" --date=iso a201fb0` → `a201fb0 2026-07-25 12:31:30 -0400 fix(ui): use the palette's dark ink on bright accent fills`; `tests/e2e/browser-style-contract.spec.ts:226` `"test(\"[P0] paints bright accent fills with ink that clears the 4.5:1 contrast floor\", async ({"` and `:320` `"expect(measured.ratio).toBeGreaterThanOrEqual(4.5);"`; `grep -rn "text-white" src/ | wc -l` → `0`.

Two `epics.md` passages now state the opposite, and both direct a story to build it:

- `epics.md:265` `"Automated 4.5:1 contrast does **not** exist; that same spec disclaims it. Contrast is therefore the outstanding obligation on whichever story adds it"`
- `epics.md:310` (the DR-2 Implementation-Entry Register row) `"automated 4.5:1 contrast does not exist and is the one outstanding obligation, on whichever story adds it."`

Neither appears in the residuals row (`:1389`), which lists Story 3.5, FR-19, NFR-6, UX-PB.1d, UX-PB.3d, UX-PB.2c, the AD-18/AD-29 citations, AD-28, and the `Cancelling` item — and nothing about contrast.

A builder reading either line schedules work that already ships, which is the failure AD-1 names by name: `:202–204` `"- **Rule:** Missing or incorrect behavior is product work, not test work. Before"` / `"scheduling anything described as a test gap, verify whether the behavior is"` / `"already present in the shipping code"`. The register row is the more dangerous of the two, because the register is what a builder consults to decide what is startable.

### H3 — Story UX-PB.5d is built entirely on D37-removed scope and is not named in the residuals row

`epics.md:1106` `"### Story UX-PB.5d: Accessibility and responsiveness of the confirmation and safety surfaces"`, whose narrative and every criterion but the zoom one sit in the scope D37 removed:

- `epics.md:1111` `"As a keyboard and VoiceOver user at high zoom, I want every safety action reachable and announced so that the confirmation gate protects everyone at the 900 x 600 minimum and at 150-200% zoom."`
- `epics.md:1116` `"**When** a keyboard/VoiceOver user operates them with reduced motion active"`
- `epics.md:1129` `"**Then** it remains fully visible and operable with its name, state, versions, primary action, error/recovery, focus order, and announcements preserved"`

Against `prd.md:454` `"**This PRD restates FR-19 and NFR-6 without those obligations.** Specifically dropped: keyboard operability of primary actions, VoiceOver operability, live-region announcements of plan progress/verification/cancellation/failure/completion, and NFR-6's deterministic dialog/sidecar focus restoration."` — which reaches UX-PB.5d's focus-restoration criterion (`epics.md:1125–1127`) directly.

Its zoom half survives (`prd.md:556` NFR-3 `"remain usable at 900 × 600 and at 150–200% zoom"`), so this is a restatement, not a deletion — the same treatment the row gives UX-PB.1d.

The residuals row names FR-19, NFR-6, UX-PB.1d and UX-PB.3d for this class (`:1389`) and then says `"Scope that run **by named heading, not by mention count**"`. UX-PB.5d is not a named heading, so it survives the run intact. Two further unnamed instances of the same class:

- `epics.md:778` `"focus moves to its programmatically focusable Upgrade Activity summary heading, and the status channel announces plan start"` (UX-PB.3a) — an obligation AD-17 now makes optional (`:718` `"- **Rule:** No story is obliged to build a status-announcement channel."`).
- `epics.md:265` `"one manual VoiceOver pass joins \`docs/RELEASE-CHECKLIST.md\`"` — deleted by D37 and verified gone: `docs/RELEASE-CHECKLIST.md:102` `"**Keyboard navigation and screen-reader support are explicitly not release criteria.**"`

### H4 — The `⌘L` / `Esc`-second-rung Open row carries no gate, and the stories that would foreclose it are unblocked

The row is honest about the divergence:

- `:1393` `"| Accelerator sinks that AD-17 moved: \`⌘L\` and \`Esc\`'s second rung | **OPEN — owner decision; surfaced by revision 10, outside its scope** |"` … `"The divergence is real in both cases — whichever UX-PB.3x story retires the drawer and whoever owns the accelerator map can each leave a dead toggle or delete the key, and RP-2 forbids both."`

And AD-28 leaves the referent open inside a Rule: `:1154–1157` `"What the surviving *second* rung closes is not settled here: \`prd.md\` FR-6 calls it close-drawer, while AD-17 retires the \`ActivityDrawer\` in favour of Activity as an \`ActiveView\` destination, so the referent moves"`.

Both referents are real in the shipping handler, verified:

- `src/hooks/useKeyboard.ts:78` `"  if (ui.drawerOpen) ui.setDrawerOpen(false);"` (Escape's third rung)
- `src/hooks/useKeyboard.ts:166` `"          useUiStore.getState().toggleDrawer();"` (`⌘L`)

against `:701–702` `"The existing"` / `"\`ActivityDrawer\` surface retires with the \`autoOpenDrawer\` setting; no story"` / `"keeps it alive as a second home for attempt status."`

Nothing gates it. `epics.md:307` `"Epic UX-PB is the primary build queue and runs first, and nothing blocks starting it"`. Nothing on the release checklist catches a dead `⌘L` either — `docs/RELEASE-CHECKLIST.md:97` covers only `"⌘X / ⌘C / ⌘V / ⌘A work in the package search field"`.

This is the shape `VALIDATION-REPORT-2026-07-25.md` F7 ruled CRITICAL one revision ago — recorded, marked out of scope, no gate, unblocked story. I rank it HIGH rather than CRITICAL on one distinction: F7's row said the *model* was unsettled and two stories would build opposite domains; here AD-17 and AD-28 already settle the model, and what is open is where two keys point. The failure is a functional regression on one accelerator against `prd.md:534` RP-2, not a durable-state or wire-shape divergence. The row also names the exact fix — `"a clause on AD-17's Activity rule naming both sinks"` — which is one clause, not new architecture.

---

## MEDIUM

### M1 — The revision-9 gate counts in the tail row are wrong; they were copied, not measured

- `:1386` `"revision 8's gate returned 8 CRITICAL / 15 HIGH / 17 MEDIUM / 10 LOW and revision 9's returned 9 / 20 / 28 / 15"`

Measured from each lens's own tally line, in the four `*-v9` files:

| File | Line | Literal | C | H | M | L |
| --- | --- | --- | --- | --- | --- | --- |
| `review-divergence-v9.md` | `:47` | `"**Tally: 27 findings — 6 CRITICAL, 8 HIGH, 9 MEDIUM, 4 LOW.**"` | 6 | 8 | 9 | 4 |
| `review-rubric-v9.md` | `:53–59` | tally table `"CRITICAL | 2"`, `"HIGH | 5"`, `"MEDIUM | 8"`, `"LOW | 5"` | 2 | 5 | 8 | 5 |
| `review-reconcile-v9.md` | `:33` | `"Tally: **2 CRITICAL, 5 HIGH, 7 MEDIUM, 4 LOW — 18 findings.**"` | 2 | 5 | 7 | 4 |
| `review-currency-v9.md` | `:31` | `"**Findings by severity: CRITICAL 0 · HIGH 2 · MEDIUM 5 · LOW 2 (9 total).**"` | 0 | 2 | 5 | 2 |
| **Sum** | | | **10** | **20** | **29** | **15** |

Actual **10 / 20 / 29 / 15** (74 findings), not 9 / 20 / 28 / 15 (72). The revision-8 half of the same sentence is **exact** — verified `review-divergence-v8.md:21` `"**Counts:** 4 CRITICAL · 4 HIGH · 4 MEDIUM · 2 LOW (14 total)."`, `review-rubric-v9.md:61` `"revision 8's rubric lens returned 1 CRITICAL / 3 HIGH / 5 MEDIUM /"` + `:62` `"3 LOW"`, `review-reconcile-v8.md:57–60` (`"| CRITICAL | 2 | 2 | **0** |"` and the three rows below it: `2/5/6/3` raised), `review-currency-v8.md:54` `"**Findings by severity:** CRITICAL **1** · HIGH **3** · MEDIUM **2** · LOW **2**"` → 8/15/17/10. The v6 figure is exact too (`review-currency-v9.md:390` `"| v6 lenses returned 44 findings: 5 CRITICAL, 14 HIGH, 18 MEDIUM, 7 LOW | **VERIFIED EXACTLY**"`).

So exactly the one set of numbers the row took from the memlog rather than from the files is the one that is wrong. The row exists *because* of `VALIDATION-REPORT-2026-07-25.md` F11, and it is the inventory of what is neither closed nor scheduled — an undercount by one CRITICAL is an undercount of unscheduled work. The same row warns the next reader about precisely this: `:1389` `"the counts in \`docs/DECISIONS.md\` D37 were copied rather than measured and are wrong"`.

### M2 — The `macos-14` currency caveat is false for all three files it names

- `:1363` `"**Caveat for a future currency check:** \`docs/SPEC.md\` §7.6 moved with the change, but \`docs/development-guide.md\`, \`docs/index.md\`, and \`_bmad-output/project-context.md\` still say \`macos-14\`."`

All three now say `macos-15`:

- `docs/development-guide.md:9` `"CI builds on \`macos-15\` (D34), which moved all three runner pins off the \`macos-14\` images GitHub began deprecating on 2026-07-06."`
- `docs/index.md:85` `"The shipped bundle declares a \`15.0\` floor; CI builds on \`macos-15\` (D34)."` — and `grep -rn "macos-14" docs/*.md` does not return `docs/index.md` at all.
- `_bmad-output/project-context.md:40` `"all \`macos-15\` since D34 retired the \`macos-14\` pin ahead of GitHub's 2026-11-02 unsupported date"`

`VALIDATION-REPORT-2026-07-25.md` F5 raised this against revision 9 and it is carried forward unchanged. The row's substantive claim is still correct — `grep -rn "runs-on:" .github/workflows/*.yml` returns exactly three macOS lines, all `macos-15` (`ci.yml:28`, `ci.yml:70`, `release.yml:63`). Only the caveat needs retiring. Low blast radius, but it is a checkable factual claim in a document whose baseline section says `"Verified against the tree on 2026-07-25."` (`:143`).

### M3 — `AD-28` and `AD-29` `Binds` are narrower than the Capability → Architecture Map rows that name them

The rubric treats a `Binds` list disagreeing with its own map row as the revision-9 defect. Three rows disagree:

- `:1345` `"| Draft Upgrade Plan and sidecar … (UX-PB.1a–1e) | … | AD-16, AD-17, AD-23, AD-24, AD-27, **AD-28** |"` vs `:1091` `"- **Binds:** Stories 3.1, 3.2, 3.5; UX-PB.1a, UX-PB.1c, UX-PB.1d; the application"` — 1b and 1e are in the row and not in `Binds`.
- `:1346` `"| Plan attempts, admission, cancellation (UX-PB.2a–2f) | … **AD-29** |"`, `:1347` `"(UX-PB.3a–3g) | … **AD-29** |"`, `:1348` `"(UX-PB.4a–4e) | … **AD-29** |"` vs `:1193` `"- **Binds:** UX-PB.2c, UX-PB.2d, UX-PB.2e, UX-PB.3d, UX-PB.3g, UX-PB.4a,"` / `:1194` `"UX-PB.4b, UX-PB.4e; Story 6.5"` — 2a, 2b, 2f, 3a–3c, 3e, 3f, 4c and 4d are in the rows and not in `Binds`.

Most of that gap is defensible (a reader story cannot violate a writer rule), and I could not construct a violation for the AD-29 set. **One AD-28 case is concrete.** UX-PB.1e renders the Manager removal affordance — `epics.md:636` `"**Then** it shows \`IN PLAN\` plus a separate visible \`Remove\` action named \`Remove <Manager> update from Upgrade Plan\`"` — and whether that removal writes a tombstone is decided by AD-23's rule at `:888` `"- **Rule:** Removal writes a tombstone on the intent. A later bulk expansion of"` and by AD-28's inverse rule at `:1141–1143` `"- **Rule:** Bulk removal is the **inverse** of bulk addition. A filter-wide or Manager-wide remove clears membership for the refs in its scope *and clears their tombstones*; only an individual single-ref removal writes one."` UX-PB.1e is bound by **neither** AD (AD-23 `Binds` at `:872` is `"UX-PB.1a, UX-PB.1c, UX-PB.1d, UX-PB.2a; Story 3.2"`). The mutation semantics do live in UX-PB.1c, which is bound — so this is imprecision rather than an unowned rule, hence MEDIUM.

### M4 — Story 3.2's pinned-row criterion still says `disabled`, and the inertness correction is recorded only against UX-PB.1d

AD-16's corrected rule:

- `:614–617` `"It therefore **may not use the native \`disabled\` state** — a"` / `"natively disabled control cannot receive the pointer interaction the reason"` / `"requires, so \`disabled\` and the explanation are mutually exclusive and the"` / `"explanation wins (\`prd.md\` FR-5, \`EXPERIENCE.md\` Checkbox contract)."`

AD-16 binds Story 3.2 — `:407` `"- **Binds:** D27–D30; Epic UX-PB (all 28 stories); Stories 3.1, 3.2, 3.5, 6.5"` — and Story 3.2's criterion says the opposite:

- `epics.md:1231` `"**Then** pinned rows stay inert, add nothing to the draft Upgrade Plan, and are explained, disabled, and excluded from every plan with the correct reason."`

The residuals row records this correction against UX-PB.1d only: `:1389` `"**UX-PB.1d's ineligibility criterion needs AD-16's corrected inertness rule** — not native \`disabled\`, pointer-interactive, inert."` Story 3.2 is a named surviving story that renders the same control and is not named.

### M5 — The residuals row's AD-18 under-citation list names a story AD-18 does not bind and omits two it does

- `:1389` `"**AD-18 and AD-29 are cited by too few of the stories they bind** — UX-PB.2c, UX-PB.2d and UX-PB.2e each write or correlate durable attempt records and cite AD-18 nowhere"`

AD-18's `Binds` is `:726` `"- **Binds:** UX-PB.2c, UX-PB.2d, UX-PB.2f, UX-PB.3d, UX-PB.4a, UX-PB.4b, UX-PB.4e; Story 6.5"` — **UX-PB.2e is not in it**, and **UX-PB.2f and UX-PB.4b are**.

`grep -n "AD-18" _bmad-output/planning-artifacts/epics.md` returns `288, 316, 664, 827, 916, 1004, 1302` — story-block hits at UX-PB.2b (`:664`), UX-PB.3d (`:827`), UX-PB.4a (`:916`), UX-PB.4e (`:1004`), Story 6.5 (`:1302`). So the stories AD-18 binds that cite it nowhere are **UX-PB.2c, UX-PB.2d, UX-PB.2f and UX-PB.4b** — the row gets two of four right, adds one AD-18 does not bind, and drops two it does. (The AD-29 half of the same sentence is correct: `grep -c "AD-29" epics.md` → `0`, and `grep -c "AD-28" epics.md` → `0`.)

---

## LOW

### L1 — AD-12's rule is left in force after the spine's own table declares it unimplementable

- `:396` `"- **Rule:** Seven files are release-please-owned and never hand-edited:"` — unchanged from revision 9.
- `:1385` `"| AD-12's file-scoped \"never hand-edited\" | **OPEN — …** | … so under AD-12 read literally, **key rotation has no legal mechanism and AD-11 cannot be implemented**. The fix is to narrow the rule to the version fields it actually means, which is a one-clause correction rather than new architecture, but it is release-side and outside the scope this revision was given."`

Recording it satisfies the instruction I was given, and the diagnosis is correct — I re-verified `release-please-config.json`'s `extra-files` is field-scoped and that `src-tauri/Cargo.toml` declares no `[profile.release]` (`grep -n "profile.release" src-tauri/Cargo.toml` → no match), which is what AD-26's gate depends on at `:995` `"  \`src-tauri/Cargo.toml\` declares no \`[profile.release]\`, so \`debug-assertions\`"`. The residual risk is that a build substrate carries a rule its own decision table calls unimplementable, and `Binds: release, all commits` (`:390`) means every commit author reads the rule and not the table. LOW because no live story is blocked by it today.

### L2 — UX-PB.2c's persist-failure criterion is arguably at odds with AD-29's "the append gates nothing", and only the record-contents override is recorded

- `:1240–1246` `"- **Rule:** The append **gates nothing** … An attempt-journal append failure is nonfatal (AD-18) and is surfaced, never fatal to admission. No story may make admission depend on a journal write"`
- `epics.md:698` `"**Then** the failure is surfaced, no partial attempt record is left behind, and the prior consistent state is preserved rather than proceeding as if durably recorded."`

"the prior consistent state is preserved rather than proceeding" is readable as *do not proceed with the admission*, which the rule forbids; and "no partial attempt record is left behind" is readable as removing a line from an append-only file, where AD-19's disposition is `:772–773` `"an unparseable line is skipped and counted, the surrounding records stay readable"`. Both readings are recoverable in the story author's favour, which is why this is LOW rather than a divergence — but the residuals row records only the other 2c override (`:1389` `"**UX-PB.2c's admission record still lists \"result/verification state\"**"`), so if the reading is the adverse one nothing catches it.

---

## What is genuinely clean

Confirmed by opening the file, not assumed.

**The two new invariants are correctly shaped.** AD-28 names a pair that obeys every prior `AD` and still diverges (`:1093–1096`), and its rules reach every limb of that divergence — no transient selection (`:1097`), batch-not-per-row (`:1112`), concrete identities not a predicate (`:1120`), all-or-none (`:1130`), range-is-`Explicit` (`:1135`), bulk removal clears tombstones (`:1141`), tri-state denominator (`:1176`). AD-29 does the same for the journal: one append authority (`:1200`), exactly two records (`:1209`), result rides terminal (`:1219`), mint-then-append (`:1235`), append gates nothing (`:1240`), fold direction (`:1247`), genuine-vs-unreadable `Interrupted` (`:1256`), idempotent fold (`:1266`), and the liveness rule (`:1272`) — which is the one that keeps a dead attempt from wedging admission against `:448` `"- **Rule:** Exactly one confirmed attempt may be active."`

**No `Prevents`-without-`Rule` anywhere in the new or changed material.** I walked AD-11, AD-16, AD-17, AD-18, AD-27, AD-28 and AD-29 clause by clause. The revision-5 CRITICAL does not recur.

**AD-29's `Binds` is exactly the set the validation report's lens B specified.** `VALIDATION-REPORT-2026-07-25.md:166` `"the close needs a new AD whose \`Binds\` names 2c/2d/2e/3d/3g/4a/4b/4e, with AD-18 carrying only a cross-reference"` — matched verbatim at `:1193–1194`, and AD-18 carries exactly that cross-reference at `:756–761`.

**Every technical claim I sampled verified against the tree.**

- Commits: `a201fb0`, `22ed41e`, `5972109`, `8d36cdf`, `7cc7b5f`, `419dc32`, `be1f0e6`, `5c8996f` all resolve with the messages the spine implies (`git log -1 --format="%h %ad %s" --date=iso <sha>`).
- Stack table (`:1301–1322`) against the lockfiles: react/react-dom `19.2.8`, `@tauri-apps/api` `2.11.1`, `@tauri-apps/cli` `2.11.4`, typescript `7.0.2`, vite `8.1.5`, tailwindcss `4.3.3` from `package-lock.json`; `tauri 2.11.5`, `tokio 1.53.1`, `tauri-plugin-updater 2.10.1`, `tauri-plugin-opener 2.5.4` from `src-tauri/Cargo.lock`; `edition = "2021"` at `src-tauri/Cargo.toml:6`; `node-version: 24` at `ci.yml:57`, `:76`, `release.yml:80`. **All exact.**
- Brownfield baseline: `ls dev/fixtures/ipc/*.json | wc -l` → `15`, matching `:150` `"\`dev/fixtures/ipc/\` holds 15 committed contract fixtures."`
- Focus remediation: `grep -rn "focus-visible:" src/ | wc -l` → `31`; `grep -rn "outline-none" src/ | wc -l` → `0`; the one sanctioned survivor is where the spine says — `src/components/manager/PackageRow.tsx:85` `"        highlighted ? \"ring-2 ring-inset ring-accent\" : \"\","`.
- AD-28's shipping-defect claims: `src/store/packages.ts:17` `"  selection: Partial<Record<ManagerId, Set<string>>>;"` and `:19` `"  anchor: Partial<Record<ManagerId, string | null>>;"`; the duplicated predicate at `src/hooks/useKeyboard.ts:35` `"function visibleSelectableIds(managerId: ManagerId): string[] {"`; and the `⌘A` defect exactly as described — `src/hooks/useKeyboard.ts:160–162` `"        case \"a\":"` / `"          e.preventDefault();"` / `"          selectAllVisible();"` against `:89` `"  if (ui.view.kind !== \"manager\") return;"`.
- AD-16's new three-call-sites rule (`:414–421`): `src/components/manager/ManagerPane.tsx:145` `"  async function upgradeRow(pkg: Package) {"` → `:152` `"    await executePlan(plan);"`; `src/components/dashboard/ManagerCard.tsx:128` `"                    void selfUpdateManager(info.id);"`; `src/components/manager/SelfUpdateCard.tsx:116` `"              onClick={() => void selfUpdateManager(managerId)}"`. **Three, as claimed.**
- AD-27's named samples (`:1050–1052`) exist and sit on opposite sides of the `appearance` discriminator: `tests/e2e/browser-style-contract.spec.ts:74` `"      const refreshAll = page.getByRole(\"button\", {"` and `:188` `"      .getByRole(\"checkbox\")"`.
- AD-11's CI claim (`:351–352`): `.github/workflows/test.yml:9–12` `"on:"` / `"  push:"` / `"    branches: [main]"` / `"  pull_request:"`.
- D37 applied to the checklist: `docs/RELEASE-CHECKLIST.md:102` `"**Keyboard navigation and screen-reader support are explicitly not release criteria.**"`, with `:97` `"9. **Clipboard and menus still work.** ⌘X / ⌘C / ⌘V / ⌘A work in the package search field"` retained — exactly what `:374–382` claims.

**AD-11's new supersession rule is the right way to disagree with the requirements authority.** `:357–359` `"- **Rule:** On one point this AD **supersedes the requirements authority, and says so rather than diverging quietly.** \`prd.md\` states in three places — §7.1, NFR-6, and addendum §4 — that the contrast guard and its on-fill ink tokens are uncommitted working-tree changes absent from \`HEAD\`"`. Verified both sides: `prd.md:628` `"**Not in this list, deliberately:** the automated contrast guard."` and `prd.md:572` `"**The 4.5:1 contrast floor does not hold at \`HEAD\`**"` are indeed the stale side, since `a201fb0` (12:31:30) postdates the PRD's stated `HEAD` `5972109` (08:45:58). The spine names the disagreement, cites the later decision, and applies the PRD's own precedence rule — this is the failure mode H2 shows is *not* being applied to `epics.md`.

**F1, F2, F3, F7, F8 are resolved as claimed; F10, F11, F12 are recorded as claimed.** F9 is dissolved rather than deferred — D37 removed the checklist item F9 was about, and AD-27 `:1072–1081` replaces it with a per-story runtime check and says why the surviving mouse-driven steps are not a substitute. F4 (`DRIFT-NOTE.md` ordering) is unaddressed: `grep -n "^## " DRIFT-NOTE.md` still places `":213 ## 4. Open items and the remaining tail — *as of revision 5; superseded*"` before the revision 7/8/9 sections — though the heading now carries the `"superseded"` qualifier F4 asked for as one of its two acceptable fixes, so I do not raise it.

**The operational envelope is no longer silent.** `:1381` `"| Post-publish operational envelope | **Deferred — revisit on the first bad published release, or on any updater-key change** |"`, naming four undecided questions and a revisit condition. That satisfies the checklist's decide-or-defer bar, and `VALIDATION-REPORT-2026-07-25.md` F12 explicitly said a Deferred row with a revisit condition would.

**AD inventory:** `grep -c "^### AD-" ARCHITECTURE-SPINE.md` → `21`; ids AD-1..5, 11, 12, 16..29. No retired id (AD-6..10, AD-13..15) is reused.

---

## Recommended fixes, in priority order

1. **C1** — `UX-PB.2f and UX-PB.4c` → `UX-PB.2e and UX-PB.3g` at `:486` and `:1389`.
2. **H1** — add to the residuals row: AD-27's `Binds` now covers Stories 3.4 and 6.5, and `epics.md:1247` / `:1302` do not cite it. Replace the `grep -c AD-27` clearance evidence with the per-story check.
3. **H2** — add to the residuals row: `epics.md:265` and the DR-2 register row at `epics.md:310` both state the contrast guard does not exist; D36 / `a201fb0` landed it.
4. **H3** — add `Story UX-PB.5d` (and `UX-PB.3a`'s announcement clause, and `epics.md:265`'s manual VoiceOver pass) to the D37 restatement list by named heading.
5. **H4** — write the one clause the row already names onto AD-17's Activity rule, naming `⌘L`'s and `Esc`-rung-2's sinks, or gate the UX-PB.3x story that retires the drawer.
6. **M1** — restate the revision-9 tail counts as **10 / 20 / 29 / 15** from the four `*-v9` tally lines.
7. **M2** — retire the `macos-14` caveat at `:1363`.
8. **M4, M5** — add Story 3.2 to the inertness item; correct the AD-18 under-citation list to UX-PB.2c, 2d, 2f, 4b.

Items 1–4 and 6–8 are corrections to two tables and one rule and carry no new invariant. Item 5 is one clause on an existing rule, which the spine has already drafted in prose.
