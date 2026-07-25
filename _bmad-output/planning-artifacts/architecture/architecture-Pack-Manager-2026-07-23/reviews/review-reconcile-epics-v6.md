# Review — reconcile `epics.md` against ARCHITECTURE-SPINE revision 6

**Lens:** reconciliation reviewer (adversarial)
**Date:** 2026-07-25
**Subject:** `_bmad-output/planning-artifacts/epics.md` (1250 lines, `wc -l`), rewritten
2026-07-25 under `_bmad-output/planning-artifacts/sprint-change-proposal-2026-07-25.md`
**Question:** can `ARCHITECTURE-SPINE.md:648` — the `epics.md` retired register **Open**
row — now honestly be closed?

**Verdict: NO. The row cannot be closed as written.**

The four items the row names literally are substantially discharged. But the row's
operative claim — "It contradicts this spine and `docs/DECISIONS.md` D33" — is still
true, through a residue the rewrite left untouched and in fact strengthened
(`epics.md:244-249`). Two of the spine's own live citations into `epics.md` were also
invalidated by the rewrite and now land on the wrong section.

Closing this row is a bounded fix, not a re-run. See §8.

---

## 0. Method

Every count below comes from a command, quoted with it. Every factual claim is
anchored to `path:line` with literal text. The rewrite was verified by diffing the
working tree against `HEAD` (`git diff --numstat HEAD -- _bmad-output/planning-artifacts/epics.md`
→ `209	125	_bmad-output/planning-artifacts/epics.md`), not by trusting the proposal.

Documents read this session in full: `ARCHITECTURE-SPINE.md` (648 lines),
`epics.md` (1250), `docs/DECISIONS.md` (365), `docs/RELEASE-CHECKLIST.md` (104),
`_bmad-output/project-context.md` (157), `sprint-change-proposal-2026-07-25.md`
(742, §4–§5 in full), `_bmad-output/implementation-artifacts/sprint-status.yaml` (118),
`DRIFT-NOTE.md` (§1–§2).

---

## 1. Question 1 — are the retired identifiers gone as live requirements?

### 1.1 What is clean

Command:

```
grep -n "TIR-" epics.md
grep -nE "\bRE-([0-9]+)" epics.md
grep -n "72-criterion\|72 criterion" epics.md
grep -n "Candidate Identity Manifest\|Evidence Registrar\|contract-lock\|Evidence Index" epics.md
```

Results — every surviving occurrence is a retirement record, none is an obligation:

| id family | occurrences | verdict |
| --- | --- | --- |
| `TIR-1..TIR-8` | 1 (`:157`) | RETIREMENT RECORD |
| `RE-1..RE-11` | 2 (`:177`, `:191`) | RETIREMENT RECORD |
| 72-criterion controls | 2 (`:44`, `:123`) | RETIREMENT RECORD |
| Candidate Identity Manifest | 2 (`:178`, `:219`) | RETIREMENT RECORD |
| Evidence Registrar / Evidence Index | 2 (`:178-179`, `:220`) | RETIREMENT RECORD |
| `contracts/readiness/v1/contract-lock.json` | 2 (`:179`, `:220`) | RETIREMENT RECORD |
| register's own `AD-1..AD-15` | 1 table (`:203-222`) | RETIREMENT RECORD |
| `ASR-01/02/03` | 2 (`:210`, `:215`) | RETIREMENT RECORD |

Verbatim, `epics.md:157`:

> "TIR-1 through TIR-8 are retired by `docs/DECISIONS.md` **D33**, together with the
> evidence lanes they specified"

`epics.md:177-180`:

> "RE-1 through RE-11 are retired by `docs/DECISIONS.md` **D33**, together with the
> Candidate Identity Manifest, the append-only Evidence Index, the Evidence
> Registrar, `contracts/readiness/v1/contract-lock.json`, and the candidate-freeze
> machinery. No `contracts/` directory exists and none is to be created."

The diff confirms these replaced live obligations. `HEAD` carried, for example,
`- TIR-3: Provide a reusable real native lane crossing frontend invocation, Tauri
serialization/registration, Rust handlers, and representative events…` — a bare
imperative. That form is gone.

**This half of the reconciliation is real and well done.**

### 1.2 What is NOT clean — see CRITICAL-1

`ASR-04` and `ASR-05` are retired **nowhere**.

```
grep -rn "ASR-04\|ASR-05" --include="*.md" . | grep -v node_modules | grep -v "/archive/"
```

returns zero hits in `epics.md` and zero hits in `ARCHITECTURE-SPINE.md`. The spine
retires only three:

`ARCHITECTURE-SPINE.md:638`:

> "| ASR-01 / ASR-02 / ASR-03 enabler framing | **RETIRED** | The enabler register
> belonged to the retired gate. …"

`epics.md:215-217` mirrors exactly that three-id scope. `ASR-04` (Release candidate
identity and attestation) and `ASR-05` (lane separation) were simply **deleted** from
`epics.md` by the rewrite — the diff shows `- AD-6 / ASR-05: …` and `- AD-7 / ASR-04: …`
removed with no replacement record — and are then **re-imported as still-owed** by
`epics.md:244-249`. That is CRITICAL-1.

---

## 2. Question 2 — is the `contracts/tauri-boundary/v1.json` set-equality
requirement gone?

**As literal text in a requirement position: yes. As a live obligation: no — it
survives one indirection.**

```
grep -c 'contracts/tauri-boundary' epics.md   →  1
```

The single occurrence is `epics.md:210`, inside the AD-collision table:

> "| AD-3 / ASR-01 | Exact set equality across the versioned
> `contracts/tauri-boundary/v1.json` catalog | The IPC surface changes atomically,
> proven by the committed fixtures in `dev/fixtures/ipc/`. "There is no separate
> versioned boundary-catalog file and none is to be created." |"

That is a retirement record. It states the retired rule in the left column and the
spine's replacement in the right. Correct form.

But `epics.md:244-249` reinstates the same obligation by reference. See CRITICAL-1.

*(Note: the proposal's own success criterion 2 — `sprint-change-proposal-2026-07-25.md:643`
"`grep -n 'contracts/tauri-boundary' epics.md` returns **zero** matches" — therefore
fails. Its criterion 1 at `:641-642` permits exactly this line. The two criteria
contradict each other; criterion 1 is the right standard. Logged as LOW-1, not as a
substantive defect.)*

---

## 3. Question 3 — does any `AD-n` id in `epics.md` still assert a rule that
contradicts the spine's rule under that same id?

**No. This was the sharpest problem and it is genuinely fixed.**

`epics.md:203-213` now handles the collision the correct way — it removes the
numbering rather than renumbering it, and tabulates the collisions explicitly:

> `:203-206` "The AD-1..AD-15 list this section previously carried was the **retired
> gate's own numbering, and it is not the spine's.** It is removed rather than
> renumbered, because the two schemes collide on the same ids with different
> meanings:"

Every `AD-n` occurrence in `epics.md` was then checked against the spine's rule under
that id (`grep -nE "AD-[0-9]+" epics.md`, 51 hits):

| `epics.md` | Cites | Asserts | Spine under that id | Verdict |
| --- | --- | --- | --- | --- |
| `:135` | AD-1 | behaviour-present-before-test-gap habit | `SPINE:121-123` same rule | AGREES |
| `:167` | AD-3 | fixtures prove shape, never delivery | `SPINE:167-172` same | AGREES |
| `:188` | AD-11, AD-12 | checklist + two automated checks | `SPINE:239-241`, `:264` same | AGREES |
| `:217` | AD-2..AD-5 surviving | — | `SPINE:638` same | AGREES |
| `:254`, `:282` | AD-12 | signing/notarization/updater | `SPINE:276-279` same | AGREES |
| `:271` | AD-2, AD-3 | native harness must satisfy them | `SPINE:641` same | AGREES |
| `:272` | AD-4 | no production shell surface | `SPINE:191-195` same | AGREES |
| `:273` | AD-18 | journal ownership/location/durability | `SPINE:493-498` same | AGREES |
| 28 story `Dependencies:` lines | AD-3, AD-16 | — | — | see HIGH-1 |

No id asserts a contradictory rule. **Question 3 passes cleanly.**

Two ancillary defects fall out of the same sweep — HIGH-1 (ordinals) and HIGH-2
(AD-17/19/20 never cited).

---

## 4. Question 4 — are the 28 UX-PB stories and the six survivors still
consistent with AD-16..AD-20, and was any acceptance criterion damaged?

### 4.1 No acceptance criterion was damaged. Verified, not assumed.

```
grep -cE "^[-+]\*\*(Given|When|Then|And)\*\*" <the full git diff>   →  0
```

Zero Given/When/Then/And lines appear on either side of the diff. Story headings and
user-story lines are likewise untouched. The only in-story edits are two `**Blocks:**`
metadata lines:

```
-**Blocks:** UX-PB.1b, UX-PB.1c; Story 3.5 and its affected evidence
+**Blocks:** UX-PB.1b, UX-PB.1c; Story 3.5
-**Blocks:** UX-PB.1d, UX-PB.1e; Stories 3.3 and 3.6 and their affected evidence
+**Blocks:** UX-PB.1d, UX-PB.1e
```

Structural counts hold: `grep -c "^### Story" epics.md` → 34;
`grep -c "^### Story UX-PB" epics.md` → 28; six survivors = 34 − 28.
`grep -c "affected evidence" epics.md` → 0 (13 occurrences removed).
`grep -c "Assignee\|Calendar date" epics.md` → 2, both inside retirement prose
(`:270`, `:353`), zero on any story.

**The proposal's promise at `:104` — "Story impact — no story's acceptance criteria
change" — is verified true.**

### 4.2 AD-16 consistency: pass on substance

Spot-checked the AC text against each of AD-16's 14 rules. Representative agreements:

- No-entry-point-executes (`SPINE:287-291`) ↔ `epics.md:480` "nothing executes",
  `:591` "the `Update Manager` action stages the self-update into the plan and never
  executes it", `:1143` "without executing", `:1217` "nothing is built, submitted,
  enqueued, or executed".
- Distinct `planId`/`planAttemptId` (`SPINE:305-308`) ↔ UX-PB.2a `:609-610` "distinct
  branded types… neither type is assignable to or substitutable for the other".
- Verification gate (`SPINE:322-332`) ↔ UX-PB.3d `:799` "it stays `Verifying` until it
  resolves… and is never colored successful on the strength of the exit code alone".
- `Skipped` semantics (`SPINE:333-338`) ↔ `:795` "`Skipped` marks only work that never
  started, and crash-reconstructed unfinished work reads as `Interrupted`".
- `skipUpgradePlanConfirmation` (`SPINE:342-345`) ↔ UX-PB.5b `:1017`, Story 3.4 `:1186-1189`.
- Interaction-required classifier (`SPINE:346-348`) ↔ UX-PB.3f `:833-843`.
- App update not a Package plan (`SPINE:349-355`) ↔ UX-PB.5e `:1087-1093`.

### 4.3 AD-17 consistency: **one real conflict** — HIGH-3

`SPINE:448-452` (AD-17):

> "**Rule:** The draft is session-scoped and is never written to disk. Every relaunch
> — after a clean quit, a crash, or a force-quit — starts with an empty draft and a
> hidden sidecar. This takes the second branch of UX-PB.1b's recovery criterion
> unconditionally: membership is never reconstructed, never partially restored, and
> never fabricated"

`epics.md:521-523` (UX-PB.1b), unchanged by the rewrite:

> "**Given** an in-progress draft when the app crashes or is force-quit
> **When** Pack-Manager relaunches
> **Then** the draft's canonical membership is reconstructed into the sidecar, or —
> if it cannot be recovered — the sidecar returns to empty with no fabricated
> membership and nothing executes"

The story still offers the first branch as its primary reading. AD-17 forbids it
outright and adds, `SPINE:454-457`: "A story that wants staging to survive a crash is
proposing a new decision, not implementing this one." The spine settled draft
durability on 2026-07-25 — the same day `epics.md` was rewritten — and the rewrite did
not carry the resolution down. `epics.md:46` ("the spine is upstream and wins")
mitigates but does not remove the ambiguity for anyone reading UX-PB.1b alone, which
is precisely the failure mode `epics.md:342-345` says the decomposition exists to
prevent.

### 4.4 AD-18 / AD-19 / AD-20 consistency

AD-18: consistent. UX-PB.2c `:648`, UX-PB.2d `:675-681`, Story 6.5 `:1243-1245` all
match `SPINE:496-518`.
AD-19: consistent on substance. UX-PB.5b `:1017`, `:1019-1025`, Story 3.4 `:1186-1189`
match `SPINE:527-540`.
AD-20: **not addressed at all** by any story — see HIGH-2.

### 4.5 The six survivors

2.2 (`:1099-1121`), 3.1 (`:1127-1148`), 3.2 (`:1150-1170`), 3.4 (`:1172-1194`),
3.5 (`:1196-1219`), 6.5 (`:1225-1249`) are byte-identical to `HEAD`. All read
consistently with AD-16/AD-17/AD-19. Story 3.1 `:1148` and Story 3.5 `:1217` both carry
the D27 no-immediate-execution rule locally, which is correct.

---

## 5. Question 5 — do all `Blocks:` / `Dependencies:` references resolve?

**Yes, against `sprint-status.yaml`. This is fixed.**

`HEAD` carried 11 references to archived stories (3.3, 3.6, 4.1, 4.6, 5.2, 5.4, 5.5,
6.3, 6.4, 6.7) — verified by `git show HEAD:…/epics.md | grep -nE "^\*\*(Blocks|Dependencies):"`.
The working tree has none: every `Blocks:` target is either a live `UX-PB.*` id or one
of Stories 3.1, 3.2, 3.4, 3.5, 6.5.

Cross-checked against `sprint-status.yaml:72-99` (28 `ux-pb-*` keys) and `:104`,
`:109-112`, `:117` (2.2, 3.1, 3.2, 3.4, 3.5, 6.5). All 34 story ids in `epics.md` have
a matching `sprint-status.yaml` key; no key is orphaned.

Residual nits, all LOW: LOW-3 (bare group ids `UX-PB.3` and range `UX-PB.4a-4e` are not
story keys) and LOW-4 (three stories carry no `Blocks:` line at all). Neither is a
dangling reference.

One MEDIUM falls out of this section — MEDIUM-3, the "no inter-epic dependencies"
claim.

---

## 6. Question 6 — new contradictions or new requirements the spine does not carry

Four. CRITICAL-1, MEDIUM-1, MEDIUM-2, MEDIUM-4 below.

---

## 7. Findings

### CRITICAL

#### CRITICAL-1 — `epics.md:244-249` re-imports the retired gate's entire obligation set by reference, including the set-equality requirement the Open row names

`epics.md:244-249`, verbatim:

> "**R-001 through R-008 are NOT retired.** D33 retired the gate, not the risks. They
> remain open high risks: source/oracle drift (6), fake/native boundary gap (6),
> misleading UI state (6), process lifecycle uncertainty (6), persistence/diagnostics
> failure (6), updater integrity failure (6), invalid shipped artifact (9), and
> environmental dependency/contamination (6). **No mitigation is complete, waived, or
> accepted.**"

`R-001`..`R-008` are defined **nowhere live**. The only definition in the repository is
`_bmad-output/archive/2026-07-24-scope-recalibration/test-artifacts-gate/test-design-progress.md:182-189`
— a file `epics.md`'s own frontmatter lists at `:33` under `historicalInputDocuments`
and declares, at `:20-25`:

> "NOT authoritative: project-context.md states "Nothing under
> `_bmad-output/archive/2026-07-24-scope-recalibration/` is authoritative""

Corroborated by `project-context.md:135` and `:86` ("treat nothing under
`_bmad-output/archive/` as authoritative").

The mitigation column of that archived table **is the retired machinery, verbatim**:

| id | archived mitigation (`test-design-progress.md`) |
| --- | --- |
| R-001 `:182` | "Complete PC-1 before recurrence credit; preserve capture provenance; run serialized, dated topology evidence; reject unreported host drift." |
| R-002 `:183` | "**Deliver ASR-01 and AD-3 set-equality checks; round-trip every catalog command and dispatch every catalog event through real Tauri with controlled state.**" |
| R-007 `:188` | "**Deliver ASR-04**; inspect exact candidate artifacts; prove trust and launch on Apple silicon and **physical Intel**; **replay the complete Evidence Index** before decision." |
| R-008 `:189` | "**Deliver ASR-05**; bind provenance depth; deny cross-lane substitution; …" |

The score mapping is unambiguous — `epics.md:247` "invalid shipped artifact (9)" is the
only 9 in the table and is R-007 (`:188` "| 3 | 3 | 9 |").

**Consequences, each independently disqualifying:**

1. **The set-equality requirement is not retired.** R-002's mitigation is literally
   "Deliver ASR-01 and AD-3 set-equality checks… round-trip every catalog command…
   through real Tauri". `epics.md:244` asserts it is not retired and `:249` asserts it
   is not complete, waived, or accepted. That is the exact obligation
   `ARCHITECTURE-SPINE.md:648` cites as the reason the row is Open, and the exact rule
   `SPINE:160-162` forbids: "There is no separate versioned boundary-catalog file and
   none is to be created."
2. **ASR-04 and ASR-05 are resurrected.** No document retires them (§1.2). `epics.md`
   deleted their text and then asserted their mitigations are still owed.
3. **It contradicts D32 eight lines above itself.** `epics.md:241` — "DR-3 is NARROWED
   by **D32**: … the obligation to verify on physical Intel hardware is dropped." Then
   `epics.md:244-249` says R-007 is not retired and its mitigation is not waived — and
   R-007's mitigation is "prove trust and launch on Apple silicon and physical Intel".
   `DECISIONS.md:294-300` calls that obligation one that "could never be discharged".
4. **The rewrite strengthened it.** `HEAD` read "No mitigation is complete, waived, or
   accepted **through planning**" — a scoped statement about planning. The rewrite
   dropped the qualifier and added bold "**are NOT retired**". This is the one place the
   rewrite made the residue worse.
5. **The spine carries no risk register at all.** `grep -n "R-00" ARCHITECTURE-SPINE.md`
   → zero. So this is also a requirement `epics.md` carries that the spine does not
   (Question 6).

**Note for whoever closes this:** `.memlog.md:94` already records "epics.md retired
register CLOSED", asserting that "TIR-1..TIR-8, RE-1..RE-11, the register's own
AD-1..AD-15 and ASR-01..ASR-05, … and the `contracts/tauri-boundary/v1.json`
set-equality requirement no longer appear as obligations." That entry is true of the
*text* and false of the *effect*. A Validate → Update run resuming from the memlog will
inherit the wrong conclusion unless this finding is carried in.

**Fix:** either (a) delete the sentence "No mitigation is complete, waived, or
accepted" and restate the eight risks in their own words with no dependency on the
archived mitigation column, or (b) delete `:244-249` entirely, since no live document
defines R-001..R-008 and D33 does not preserve them. (a) is preferable — the risks
themselves are legitimate; the archived mitigations are not.

---

### HIGH

#### HIGH-1 — six `AD-16 rule N` ordinal citations mis-resolve against revision 6

AD-16 carries exactly 14 rules
(`awk 'NR>=281 && NR<=356 && /^- \*\*Rule:\*\*/' ARCHITECTURE-SPINE.md | wc -l` → `14`,
at lines 287, 292, 298, 305, 309, 315, 318, 322, 327, 333, 339, 342, 346, 349).

| `epics.md` | Story | Cites | Rule N actually says (`SPINE:line`) | Correct rule |
| --- | --- | --- | --- | --- |
| `:760` | UX-PB.3c *Per-item live progress states* | rule 4 | `:305` "`execute_plan` returns a newly generated durable `planAttemptId`" | 10 (`:333`, `Verifying`/`Skipped` durable states) |
| `:782` | UX-PB.3d *Verification-gated Results* | rules 6-7 | `:315` one active attempt; `:318` cancellation | 8-9 (`:322`, `:327`) |
| `:848` | UX-PB.3g *Two labeled cancellation scopes* | rules 8, 10 | `:322` verification gate; `:333` durable states | 7 (`:318`) + 10 |
| `:870` | UX-PB.4a *One immutable History row* | rules 2 and 5 | `:292` draft holds `PlanIntent`; `:309` atomic admission | 4 (`:305`) |
| `:930` | UX-PB.4d *Retry scope + linked attempt* | rule 5 | `:309` atomic admission | 11 (`:339`, Retry) |
| `:953` | UX-PB.4e *Legacy Operation labeling* | rule 9 | `:327` verification refresh freshness | the "Legacy honesty" bullet at `:432-435` (not a numbered rule) |

There is no consistent offset, so this is not a single shift that a `+N`/`−N` correction
would repair. These citations predate the rewrite (`git show HEAD:…/epics.md | grep -nE "AD-16 rule"`
returns the same six), but the rewrite's stated purpose was reconciliation with
revision 6 and it left them. An implementer following `epics.md:199-200` — "Stories cite
its AD ids and no others" — will be sent to the wrong rule six times.

**Fix:** replace ordinals with the rule's subject, e.g. `AD-16 (Verifying/Skipped are
durable wire states)`. Ordinals into an unnumbered bullet list break on every spine
revision; the spine has revised three times in two days.

#### HIGH-2 — `AD-17`, `AD-19`, and `AD-20` are cited zero times in `epics.md`, though the spine binds all three to named stories

```
grep -c "AD-17\|AD-19\|AD-20" epics.md   →  0
```

The spine binds them explicitly:

- `SPINE:438` AD-17 **Binds:** "UX-PB.1a–1e, UX-PB.3a; Stories 3.1, 3.2, 3.5"
- `SPINE:522` AD-19 **Binds:** "UX-PB.5b; Story 3.4; all persistence"
- `SPINE:544` AD-20 **Binds:** "all frontend work; any new plugin, permission, window, or remote asset"

Nine named stories are bound by an invariant none of them references. AD-20 in
particular has no representation anywhere in `epics.md` — no story, no FR, no NFR
mentions CSP, capability files, or webview permissions, and `SPINE:549-555` makes
widening the boundary "a security-sensitive change reviewed on its own terms and never
folded into a feature story as a side effect". A UX-PB story adding a plugin would hit
nothing in `epics.md` telling it to stop.

This directly undercuts the reconciliation's own claim at `epics.md:199-200`:

> "Stories cite its AD ids and no others."

They cite only AD-3 and AD-16 (plus AD-18 once, at `:273`, outside any story).

**Fix:** add AD-17 to the `Dependencies:` lines of UX-PB.1a–1e and UX-PB.3a; AD-19 to
UX-PB.5b; and either a story or an explicit line in the Architecture Invariants section
acknowledging AD-20 binds all frontend work.

#### HIGH-3 — UX-PB.1b still permits the draft-reconstruction branch AD-17 forbids

Detailed in §4.3 above. `epics.md:521-523` vs `SPINE:448-457`.

**Fix:** rewrite the criterion's `Then` to the unconditional form AD-17 settled — "the
sidecar returns to empty with no reconstructed, partially restored, or fabricated
membership, and nothing executes" — and note that this is `SPINE` AD-17's resolution
dated 2026-07-25. This is an acceptance-criterion edit, so it is out of scope for the
proposal that produced this rewrite (`:104` "no story's acceptance criteria change") and
needs its own authorization.

#### HIGH-4 — the rewrite invalidated the spine's own two live citations into `epics.md`

`ARCHITECTURE-SPINE.md:646`:

> "| Draft durability | **RESOLVED** | Fail-to-empty. … `epics.md:437-439` permits this
> branch explicitly. Decided 2026-07-25; closes the assumption revision 5 carried. |"

`DRIFT-NOTE.md:76-78` repeats the same citation.

What is at `epics.md:437-439` today:

```
437: ### Epic 6: Preserve State, Evidence, and Privacy Across Failure and Relaunch
438:
439: Users can reconstruct Operations after crashes, trust Settings and durable stores…
```

The UX-PB.1b recovery criterion moved to `:521-523` when the rewrite added 84 net lines
above it. The spine's **RESOLVED** row for draft durability — the row that closed
revision 5's open assumption — now rests on a citation that points at an unrelated epic
heading. `grep -n "epics.md" ARCHITECTURE-SPINE.md` returns exactly three lines: `:21`
(sources), `:646`, `:648`. Two of the three are affected by this rewrite; one is the row
under review.

**Fix:** repoint `SPINE:646` and `DRIFT-NOTE.md:76` to `epics.md:521-523` in the same
change that closes the Open row. Closing the row while leaving a broken citation two
rows above it in the same table would be a regression in the same document.

---

### MEDIUM

#### MEDIUM-1 — `epics.md:328-329` keeps a live-shaped instruction referencing ids from the retired oracle

> "`AUT-003` is retained as historical evidence of superseded behavior and must
> not support revised `F5-AC3`."

`F5-AC3` is a criterion id from the 72-criterion coverage map that `epics.md:126`
declares nonexistent ("no `readiness-coverage-map.md` oracle"). It resolves only into
the archive (`readiness-coverage-map.md:114`, `traceability-matrix.md:820`).
`grep -rn "F5-AC3"` outside the archive returns `epics.md:329` and the proposal only.

`AUT-003` is real and live —
`tests/e2e/upgrade-journeys.spec.ts:169` `test("[P0] AUT-003 executes a one-row Upgrade
immediately without a plan dialog"` — and its rewrite is already owned by an acceptance
criterion, `epics.md:495-497`. So the sentence's live half is redundant and its dead
half points at a retired oracle. The proposal considered this at `:606-618` and
deliberately left it. Placement makes it worse: it sits *after* the historical table, so
`:306`'s "not a live instruction" disclaimer does not obviously cover it, and it is
phrased "must not".

**Fix:** either delete the sentence (UX-PB.1a `:495-497` already owns the work) or move
it inside the table's scope with an explicit "historical" marker.

#### MEDIUM-2 — `epics.md:305-316` contradicts itself about whether the amendment table is live

`:306`: "**This table is a historical revision record, not a live instruction.**"
`:316`, ten lines later, immediately above the same table:
"Where older story text conflicts, these replacements are binding:"

Both cannot be true. The table's seven rows name Stories 3.3, 3.6, 4.1, 4.6, 5.2, 5.4,
5.5, 6.3, 6.4, 6.7, 7.6, 7.7, 7.10, 8.7 — all archived, as `:310-313` itself states. The
proposal's Change Set D (`:601-604`) asked to "add one sentence marking it explicitly
historical" and that sentence was added at `:306`, but the pre-existing "are binding"
lead-in at `:316` was not removed.

**Fix:** change `:316` to past tense — "Where older story text conflicted, these were
the replacements applied on 2026-07-24:".

#### MEDIUM-3 — "the six surviving Epic 1-6 stories carry no inter-epic dependencies" is contradicted by `epics.md`'s own `Blocks:` edges

Asserted three times: `:261`, `:415`, `:457-458`. Mirrored in `SPINE:640` and
`project-context.md:135`.

But `epics.md`'s own metadata creates Epic-UX-PB → Epic-2/3/6 edges for five of the six:

| `epics.md` | Story | `Blocks:` |
| --- | --- | --- |
| `:472` | UX-PB.1a | `UX-PB.1b, UX-PB.1c; Story 3.5` |
| `:555` | UX-PB.1d | `Story 3.2` |
| `:578` | UX-PB.1e | `Story 3.1` |
| `:783` | UX-PB.3d | `UX-PB.3e, UX-PB.3g; Story 6.5` |
| `:931` | UX-PB.4d | `Story 6.5` |
| `:1001` | UX-PB.5b | `UX-PB.5c; Story 3.4` |

Only Story 2.2 is genuinely unblocked. The edges are also **asymmetric**: Story 6.5's
own contract (`:1236`) reads "Dependencies: disposable logs/transcripts/journal" and
never names UX-PB.3d or UX-PB.4d, so a reader starting from 6.5 sees no prerequisite
while two UX-PB stories claim to block it. Same for 3.1, 3.2, 3.4, 3.5.

The charitable reading of "inter-epic" is "among Epics 1–6", which is true. But `:415`
places the claim immediately after "Epic UX-PB is the primary build queue and runs
first", where it reads as a scheduling statement — and as a scheduling statement it is
false.

**Fix:** say what is actually true — "the six survivors carry no dependencies on each
other beyond Epic 3's internal 3.1 → 3.2 → 3.5 chain; five of them are blocked by named
Epic UX-PB stories" — and add the reciprocal `Dependencies:` entries to the five
survivors.

#### MEDIUM-4 — AJ-1..AJ-6 survive as unowned "Prove …" obligations

`epics.md:146-153` retains six Product Acceptance Journeys, each phrased as an
obligation ("AJ-1: Prove normal Finder/Dock launch, …"). They resolve to a live
authority — `EXPERIENCE.md:377-460` — so they are not dangling. But nothing assigns
them: the FR Coverage Map (`:356-411`) allocates every FR and RP and never mentions AJ;
`docs/RELEASE-CHECKLIST.md` (read in full, 104 lines) never mentions AJ; the spine
carries no AJ. They are the last "Prove X" construct left in a document whose gate was
retired, and the spine does not carry them — Question 6's "requirement the spine does
not carry" applies.

**Fix:** either map each AJ to the checklist item or story that discharges it, or mark
the section as the UX journey index it now is rather than an acceptance obligation.

---

### LOW

**LOW-1 — the proposal's own success criteria 1 and 2 contradict each other.**
`sprint-change-proposal-2026-07-25.md:641-643`: criterion 1 permits retired identifiers
in lines that mark them retired; criterion 2 demands `grep -n 'contracts/tauri-boundary'
epics.md` return zero. `epics.md:210` satisfies 1 and violates 2. Criterion 1 is the
correct standard; criterion 2 was over-specified. No file change needed — but do not
treat criterion 2's failure as a defect when closing.

**LOW-2 — success criterion 4 reads 29, not 28.**
`sprint-change-proposal-2026-07-25.md:646`: "`grep -c 'Primary concern' epics.md` still
returns **28**." Actual: 29. The 29th is prose the rewrite itself introduced at
`epics.md:137` ("The `Primary concern` label retained…"). The 28 story labels are intact
at `:470`–`:1076`. Cosmetic.

**LOW-3 — bare group ids remain in dependency metadata.**
`epics.md:640` and `:687` both read `**Blocks:** UX-PB.3 (on UX-PB.2 completion)`;
`:809`, `:827`, `:849` read `**Blocks:** UX-PB.4a-4e`. Neither `UX-PB.3` nor
`UX-PB.4a-4e` is a key in `sprint-status.yaml`. Change Set C rule 3
(`sprint-change-proposal-2026-07-25.md:575-576`) asked for expansion to concrete ids;
partially applied. `bmad-create-story` reads these lines.

**LOW-4 — three stories carry no `Blocks:` line at all.**
UX-PB.4e (`:952-953`), UX-PB.5d (`:1051-1052`), UX-PB.5e (`:1076-1077`) have
`Primary concern:` and `Dependencies:` but no `Blocks:`, while the other 25 UX-PB
stories have all three. UX-PB.4c uses the explicit form `**Blocks:** No dependent
sub-story (leaf of the UX-PB.4 spine)` (`:913`). Inconsistent; use the explicit form.

**LOW-5 — UX-PB.1b's sidecar-close criterion is stated unconditionally.**
`epics.md:517-519`: "**When** I remove the last item **Then** the sidecar closes".
`SPINE:466-471` makes visibility a three-way union — non-empty draft **or** non-terminal
attempt **or** undismissed Results — and `:469-470` says "Results remain until dismissed
even though the draft behind them is empty". The AC is scoped to a draft-only scenario
so it is not wrong, but read literally it would close a sidecar holding live Results.

---

## 8. What must change before the Open row can be closed

Blocking, in order:

1. **CRITICAL-1** — remove the archived-mitigation import at `epics.md:244-249`. This is
   the finding that keeps the row's own words true.
2. **HIGH-2** — cite AD-17 and AD-19 in the stories the spine binds them to; give AD-20
   a home. `epics.md:199-200` currently claims something the file does not do.
3. **HIGH-4** — repoint `ARCHITECTURE-SPINE.md:646` and `DRIFT-NOTE.md:76` from
   `epics.md:437-439` to `epics.md:521-523`, in the same change that closes the row.
4. **HIGH-1** — replace the six `AD-16 rule N` ordinals with subject references.

Should be fixed but arguably separable (each needs its own authorization because it
touches acceptance-criterion or governance text):

5. **HIGH-3** — narrow UX-PB.1b `:521-523` to AD-17's settled branch.
6. **MEDIUM-1, MEDIUM-2, MEDIUM-3, MEDIUM-4.**

If items 1–4 land, the row can be rewritten as **RESOLVED**, with the recommended note:

> "Reconciled 2026-07-25 under `sprint-change-proposal-2026-07-25.md`. TIR-1..TIR-8,
> RE-1..RE-11, ASR-01..ASR-05, the register's own AD-1..AD-15, the 72-criterion
> controls, the Candidate Identity Manifest, the Evidence Registrar,
> `contracts/readiness/v1/contract-lock.json`, and the
> `contracts/tauri-boundary/v1.json` set-equality requirement appear only as
> retirement records. No `AD-n` id in `epics.md` asserts a rule differing from this
> spine's under the same id. Residual: UX-PB.1b `epics.md:521-523` still offers the
> draft-reconstruction branch AD-17 forbids."

---

## 9. Observations outside this review's scope

Surfaced because the brief named these files as governing authorities. Neither affects
the Open row; neither is `epics.md`'s defect.

**OBS-1 — `docs/RELEASE-CHECKLIST.md:91-92` contradicts the spine and `epics.md`.**
The checklist states: "Automated contrast (4.5:1) and reduced-motion checks run in the
Playwright/Vitest lane and need no manual step." `SPINE:257-260` (AD-11) says: "neither
exists yet, so this is an obligation on whichever story adds them, not a description of
current coverage." `epics.md:240` agrees with the spine verbatim. The checklist is the
outlier and describes coverage that does not exist — which is exactly the failure mode
D33 retired the gate to avoid.

**OBS-2 — `project-context.md:136` understates the capability grant.**
It says the capability file "grants only `core:default` and `opener:default` to the
`main` window". `src-tauri/capabilities/default.json` grants three:
`"core:default"`, `"opener:default"`, `"core:window:allow-start-dragging"` — the third
added by commit `03e03fa fix: restore window dragging from the title bar (#33)`.
`SPINE:551-553` (AD-20) has all three and is correct. `project-context.md` is stale
here; since AD-20 is the invariant that guards permission widening, the authority most
agents read should not undercount it.
