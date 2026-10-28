# Review — internal coherence and glossary discipline

**Artifact:** `_bmad-output/planning-artifacts/prds/prd-Pack-Manager-2026-07-25/prd.md`
**Companion:** `.../addendum.md`
**Lens:** the PRD checked against itself — glossary discipline, cross-reference resolution, ID continuity, and prd/addendum agreement.
**Date:** 2026-07-25

Every claim below cites `path:line` and a literal quote. Counts come from a stated command.

---

## What passed

Recorded so a later reader does not re-run these.

**ID continuity is clean.**

- `grep -o "\bFR-[0-9]\+" prd.md | sort -u -V` → FR-1 … FR-22, contiguous, no gaps, no duplicates.
- `grep -n "^#### FR-" prd.md` → 22 headings, one per id, lines 135–477. No id is defined twice.
- `grep -o "\bNFR-[0-9]\+" prd.md | sort -u -V` → NFR-1 … NFR-8; `grep -n "^#### NFR-" prd.md` → 8 headings, lines 509–553.
- RP-1 / RP-2 defined at `prd.md:493` and `prd.md:499`. AJ-1 … AJ-6 all six referenced and all six defined in the §2.3 table. SM ids: SM-1 … SM-5, SM-C1, SM-C2 — all defined, none dangling.

**Every `D-n` reference resolves.** The 23 distinct ids cited (D1, D2, D3, D6, D10, D15, D19, D20, D21, D22, D25, D25a, D26, D27, D28, D29, D30, D31, D32, D33, D35, D36, D37) all have headings in `docs/DECISIONS.md` — e.g. `docs/DECISIONS.md:519 "## D37. Keyboard navigation and screen-reader support are not release criteria"`.

**The one `AD-` reference resolves.** `prd.md:434 "is governed by D35 and AD-27"`; AD-27 exists as of spine revision 9 — `ARCHITECTURE-SPINE.md`'s run log records `.memlog.md:162 "AD-27 created rather than AD-11 widened, and the id is the next free one"`.

**Every external line citation I checked lands on the text it claims.** `ux-Pack-Manager-2026-07-23/.memlog.md:75` is verbatim the sentence quoted at `prd.md:641`. `review-reconcile-epics.md:166` does contain the 100-round-trip cost cited at `prd.md:647`. `ARCHITECTURE-SPINE.md:1050` is the `"Transient selection has no owning invariant | **OPEN — new architecture, not this run's scope**"` row. `epics.md:53` is FR-1 and `epics.md:450` is FR-22, matching `prd.md:12`'s "lines 53–450". `epics.md:89` is FR-19 and `epics.md:113` is NFR-6, matching `addendum.md:52`. `EXPERIENCE.md:143` is the membership-model row, `EXPERIENCE.md:313` is `"# Accessibility Floor"`, `EXPERIENCE.md:373` is `"# Key Flows"`.

**§10 roundtrips on its central claim.** `grep -c "ASSUMPTION" prd.md` → 1, and that single occurrence is `prd.md:654 "No \`[ASSUMPTION]\` tags were needed."` — the claim is self-consistent.

**The §9 renumbering left no numeric orphan.** `grep -o "Q[0-9]" prd.md addendum.md` returns exactly one hit, `prd.md:639`. No reference to an old Q2–Q6 numbering survives in either file. (The *label* `Q1` is a problem, but for a different reason — finding C-3.)

**Already-known, confirmed, not re-reported as findings:** `prd.md:104 "See PI-1 in FR-2."` — `grep -rn "PI-[0-9]" prd.md addendum.md` returns that line only, and FR-2 (`prd.md:149-160`) defines no inner identifiers at all, so the pointer has nothing to resolve to even in principle. Also excluded per scope: the FR-14 quit-guard status, §7.3's health-fix deferral *as a code claim*, §7.1's contrast guard, and FR-6's immediate-execution site list.

---

## Findings

### C-1 [HIGH] §9 open question 1 asks a question FR-21 already answers normatively

`prd.md:629`:

> 1. **What happens on quit with work *queued* but not running?** The running-Operation quit guard is defined. Queued-only work, application-update installation during Package activity, and OS-initiated shutdown are not.

The middle clause is false against the PRD's own FR-21. `prd.md:471`:

> - Installation and relaunch are **refused** while any Package Operation is queued or running. Queued counts as active — admission has already committed to the work, and a restart would drop it unstarted.

and `prd.md:472`:

> - This refusal is enforced independently in two layers, and both must stay: the frontend quit guard explains it to the user, and the backend refuses on its own because the install path terminates every child process before relaunching.

FR-21 is tagged `prd.md:465 "**Status:** Shipping."` — so the PRD simultaneously states that application-update installation during Package activity is *defined, shipping, and doubly enforced*, and that it is an unresolved gap. `RP-1` says the same thing a third time: `prd.md:497 "application-update state stays separate from the Operation queue and History."`

**Why it matters:** §9's stated disposition is `prd.md:635 "each can be resolved during the epic that touches it."` The epic that touches this one will re-decide a requirement that is already binding and already implemented in two layers, and the cheapest wrong answer — "allow install, it's just an app update" — silently deletes a shipping safety property.

**Fix:** strike `application-update installation during Package activity, ` from `prd.md:629`, leaving the two clauses that genuinely are open (queued-only quit, OS-initiated shutdown).

---

### C-2 [HIGH] FR-6 quotes NFR-3 saying something NFR-3 does not say

`prd.md:231` closes the batch-membership requirement with a quoted appeal to NFR-3:

> …so a per-row mapping turns a shift-range across 100 rows into 100 round-trips and breaks NFR-3's "remain interactive with more than 100 Package rows."

NFR-3's actual text, `prd.md:525`:

> State renders progressively without waiting for all Managers. The interface stays interactive beyond 100 Packages, with correct actions reachable at 101 rows.

`grep -c "remain interactive with more than 100 Package rows" prd.md` → **1** — the quote at line 231 is the only occurrence in the file. The quoted string is verbatim from a *different* document: `epics.md:107 "NFR-3: Render progressive state without waiting for all Managers; remain interactive with more than 100 Package rows; prove reachability and correct actions at 101 rows…"`. It came in via `review-reconcile-epics.md:166`, which cites it correctly as `epics.md:88` — and the PRD re-attributed it to its own NFR-3 without checking that its own NFR-3 had been reworded.

**Why it matters:** this is the *sole* stated justification for the only new requirement the PRD invents (`prd.md:231 "This is a requirement, not an optimization"`, logged as new at `.memlog.md:26 "the NEW batch requirement"`). A downstream builder who follows the quote to NFR-3 finds different words and no "Package rows" unit at all, and the requirement loses its warrant at exactly the point someone would push back on it. It also makes the PRD cite epics.md as authority in the one document whose §0 says `prd.md:16 "This PRD is the requirements authority. \`ARCHITECTURE-SPINE.md\` and \`epics.md\` are reconciled *against* it, not the reverse."`

**Fix:** requote from this PRD's own NFR-3 — `breaks NFR-3's "The interface stays interactive beyond 100 Packages, with correct actions reachable at 101 rows."`

---

### C-3 [HIGH] "Q1" now names two different questions

`prd.md:639` opens the closed-question record with the label:

> **Q1 — Does a Package checkbox mutate Upgrade Plan membership directly, or is there a separate transient selection? CLOSED 2026-07-25: directly.**

But §9 was renumbered when that question left the list — `.memlog.md:30 "9 renumbered: Q1 removed from the open list and recorded under a new 9.1 Closed during this run."` — so the surviving open list now starts at `prd.md:629 "1. **What happens on quit with work *queued* but not running?**"`.

The result: "Q1" resolves to the membership question in §9.1 and to the quit question in §9, in the same document, with no disambiguator. `prd.md:635 "All five are non-blocking"` confirms the open list is 1–5, so item 1 is a live, citable id.

**Why it matters:** §9.1 exists precisely so downstream artifacts stop relitigating the membership model, and `addendum.md:53` routes a spine row to be retired on the strength of `"see \`prd.md\` §9.1"`. A reconciliation run told to "close Q1" has a 50% chance of closing the wrong one. This is the same positional-reference failure the spine run folder already recorded three times — `ARCHITECTURE-SPINE.md:1050 "the same positional-reference failure this run folder has now hit three times (rule ordinals, \`epics.md\` line numbers, and now spine line numbers)."`

**Fix:** relabel §9.1's entry to something outside the open-list namespace — e.g. **"Closed: Package-checkbox membership model"** — or retire numeric Q ids from §9 in favour of titles.

---

### C-4 [MEDIUM] Three FRs appear in no §2.3 journey row, and AJ-3's row contradicts §4.1's group header

`prd.md:100`-anchored discipline aside, the §2.3 table is the PRD's only FR→journey mapping. Collecting every FR id cited in the table body (`prd.md:87`–`92`) yields FR-1…FR-11, FR-13…FR-16, FR-18, FR-20…FR-22. **FR-12, FR-17, and FR-19 appear in no row.**

That is not an artifact of grouping. `prd.md:308` puts FR-12 inside the group that claims `"Realizes AJ-4, AJ-5."`, yet `prd.md:90` (AJ-4) lists only `"FR-13, FR-14, FR-15, FR-16"` and `prd.md:91` (AJ-5) only `"FR-15, FR-18"`. Same for §4.4: `prd.md:382 "Realizes AJ-5."` while AJ-5's row omits both FR-17 and FR-19.

The disagreement runs the other way too. `prd.md:89` maps AJ-3 to `"FR-5, FR-6, FR-10, FR-11"`, but FR-5 lives in §4.1, whose header claims only `prd.md:129 "Realizes AJ-1."`

**Why it matters:** `prd.md:83` justifies mirroring the AJ namespace rather than minting a new one — `"This PRD mirrors those IDs rather than creating a parallel set"` — which only pays off if the mapping is total and the two directions agree. `bmad-create-epics-and-stories` reads both the table and the group headers; FR-12 (the no-privilege boundary) and FR-19 (the whole interface) landing in neither direction is the kind of hole that produces an epic with no story covering them.

**Fix:** add FR-12 to AJ-4 (it is the "no password prompt" beat the stall surface depends on — `prd.md:347 "The stall surface states that Pack-Manager never enters passwords."`), add FR-17 and FR-19 to AJ-5 or state explicitly that they are cross-journey, and add AJ-3 to §4.1's Realizes line.

**Same shape in §8.** `prd.md:612 "**SM-2: Zero unreviewed mutations.** No Package or Manager update ever runs that the user did not see staged first… Validates FR-7, FR-8, FR-10."` — SM-2's own verb is "staged", which is FR-6's: `prd.md:225 "- Checking an eligible Package immediately adds it to the Upgrade Plan draft"` and `prd.md:230 "- Neither a checkbox nor a row action executes anything."` FR-6 is the requirement that *removes* immediate execution and is the one SM-2 measures, and it is the one FR SM-2 does not list. FR-9's atomic admission (`prd.md:271 "- All derived groups enqueue together, or none do."`) is likewise unlisted. Add FR-6 and FR-9 to SM-2's Validates line.

---

### C-5 [MEDIUM] The antonym of the glossary-defined "Outdated" is written four ways, none of them defined

`prd.md:100` states the rule: `"Downstream workflows must use these terms exactly. No synonyms appear anywhere else in this document."` `prd.md:104` defines `**Outdated**`. Its complement — the state that determines Upgrade Plan eligibility — is never defined, and appears as four distinct strings:

- `prd.md:197 "- Current, Outdated, Pinned, self-updating, unknown-version, and error states are visually distinct…"`
- `prd.md:200 "- Up-to-date and otherwise ineligible Packages cannot enter the Upgrade Plan…"`
- `prd.md:202 "- Packages with updates sort first by default, with a filter that shows all."`
- `prd.md:519 "…replacing a Snapshot with an outdated-only overlay would make every up-to-date Package vanish."`

The membership rule is stated twice with two different vocabularies for the same exclusion set. `prd.md:200` excludes `"Up-to-date and otherwise ineligible Packages"`; `prd.md:227` excludes `"Current, Pinned, and default-excluded Packages"`.

**Why it matters:** FR-6 is Planned, so a builder implements the exclusion predicate from this text. Two enumerations of the same set with no shared term is how "Current" and "Up-to-date" end up as two different enum variants, or how "default-excluded" gets read as broader than `prd.md:199`'s self-updating casks. `EXPERIENCE.md:143` and `.memlog.md:76` both use "current", which suggests `Current` is the intended canonical form.

**Fix:** add `**Current**` to §3 as the defined complement of Outdated, and rewrite `prd.md:200`, `prd.md:202`, and `prd.md:519` onto it. Also define the ineligible set once (Current ∪ Pinned formulae ∪ default-excluded self-updating casks ∪ unavailable-executor) and reference it from FR-5 and FR-6 rather than re-enumerating.

---

### C-6 [MEDIUM] "sidecar" is an undefined synonym for the Upgrade Plan surface

`prd.md:112` defines the term: `"**Upgrade Plan** — the reviewable set of Package and Manager updates… Under D27 it is one persistent editable draft; see FR-7."` FR-7 then names the same thing something else, twice:

- `prd.md:240 "The persistent editable sidecar and the separate confirmation dialog are Planned — D27, D28."`
- `prd.md:249 "- **Planned — D27:** the sidecar is hidden when empty, appears on first addition, persists across navigation, and offers Remove on every staged item."`

`grep -n -i "sidecar" prd.md` returns four hits; the other two (`prd.md:430`, `prd.md:659`) are quotations of `epics.md` NFR-6's removed clause, so lines 240 and 249 are the PRD speaking in its own voice. §3 contains no entry for "sidecar".

**Why it matters:** line 249 carries four testable consequences that attach to "the sidecar" and to nothing else in the document. A reader who takes §3 at its word — `prd.md:100 "No synonyms appear anywhere else in this document"` — will look for a distinct object and not find one, and the Upgrade Plan's own entry never mentions that it has a persistent surface at all.

**Fix:** either add `**Upgrade Plan sidecar**` to §3 as the named surface of the Upgrade Plan, or replace both uses with "the Upgrade Plan surface". Same choice applies to the Activity/activity-surface/live-surface trio — `prd.md:334 "Refresh Operations never auto-open a live surface"` vs `prd.md:394 "auto-open the activity surface for mutations (on)"` vs `prd.md:336 "Activity as a first-class navigation destination"` — which is three names across two FRs for one thing.

---

### C-7 [MEDIUM] The app-update refusal scope is written three ways and none is defined

`prd.md:107` defines the only relevant term: `"**Operation** — one queued unit of work: Refresh, Upgrade, SelfUpdate, or HealthFix."` The refusal that gates application-update installation is then scoped with three undefined phrases:

- `prd.md:471 "…refused while any **Package Operation** is queued or running."`
- `prd.md:461 "**Package work** remains understandable and uninterrupted throughout."` and `prd.md:583 "…the two-layer refusal while **Package work** is in flight."`
- `prd.md:629 "…application-update installation during **Package activity**…"`
- `prd.md:92 "…refusal while **Package work** is active…"`

**Why it matters:** the question these phrases fail to answer is load-bearing and testable: *does a running Refresh block an application-update install?* Under §3, Refresh **is** an Operation. Under "Package Operation" it plausibly is not. FR-21 is tagged Shipping with `prd.md:472 "enforced independently in two layers"`, so an acceptance test has to pick one reading, and picking the narrow one weakens a shipping guard while picking the broad one makes the app un-updatable during the auto-refresh that `prd.md:394 "refresh automatically on launch (on)"` fires at every launch.

**Fix:** define one term in §3 — e.g. `**Package Operation** — any Operation other than an Application update; i.e. Refresh, Upgrade, SelfUpdate, or HealthFix` (or exclude Refresh explicitly, if that is the intent) — and use it at 92, 461, 471, 531, 583, and 629.

---

### C-8 [MEDIUM] The reconciliation queue's counts are VoiceOver-only but labelled "keyboard/VoiceOver"

`prd.md:440`:

> `epics.md`, `ARCHITECTURE-SPINE.md`, and `EXPERIENCE.md` still carry the removed obligations (10, 3, and 4 mentions respectively).

`addendum.md:52` `"…still carry the D37-removed keyboard/VoiceOver and announcement obligations; 10 mentions total."`; `addendum.md:53` `"3 keyboard/VoiceOver mentions"`; `addendum.md:54` `"4 keyboard/VoiceOver mentions"`.

Counted:

```
grep -o "VoiceOver" <file> | wc -l   → epics.md 10 · ARCHITECTURE-SPINE.md 3 · EXPERIENCE.md 4
grep -oi "keyboard" <file> | wc -l   → epics.md 11 · ARCHITECTURE-SPINE.md 6 · EXPERIENCE.md  9
```

The stated numbers reproduce exactly — but only as **VoiceOver** occurrences. The keyword the labels put first is not counted at all, and it is the larger half in every file.

**Why it matters:** `addendum.md:48` makes these rows executable instructions — `"Three downstream artifacts contain statements it supersedes. All three are workflow-owned — **none of them may be hand-edited.**"` — each routed to a named workflow. A `bmad-correct-course` run that treats "10 mentions total" as its done-condition on `epics.md` stops with 11 keyboard obligations still standing, including `epics.md:1262 "### Story 3.5: Preserve Exact Keyboard Selection and Row Plan Actions"` — a whole story built on a criterion D37 removed. Same shape in the other two files.

**Fix:** restate as "10 `VoiceOver` and 11 `keyboard` occurrences" (and 3/6, 4/9), or drop the numbers and name the sections instead — `EXPERIENCE.md`'s `Accessibility Floor` (lines 313–332) and `epics.md` Story 3.5 / UX-PB.1d are the load-bearing sites, not the raw counts.

---

### C-9 [MEDIUM] FR-19's surface list omits two surfaces FR-19's own status line and NFR-3 both require

`prd.md:420`:

> - One coherent dark-only interface spans the Dashboard, Manager navigation and workspaces, the Upgrade Plan, Activity, History, Settings, status surfaces, and application menus.

`prd.md:415` (FR-19's own Status):

> **Status:** Shipping for the current navigation model. The D30 navigation changes — Activity as a first-class destination, **the Results surface**, and one-plan-per-row History — are Planned.

`prd.md:525` (NFR-3):

> Navigation, the plan, **confirmation**, Activity, **Results**, and recovery all remain usable at 900 × 600 and at 150–200% zoom.

So Results and the confirmation dialog are surfaces the PRD requires elsewhere (`prd.md:250 "\`Confirm N Updates\` opens a separate modal confirmation"`, `prd.md:336 "a terminal Results summary"`) but are missing from the one consequence that enumerates what the coherent interface has to span. `epics.md:89`'s FR-19, which this one restates, does list both: `"…persistent Upgrade Plan, separate Confirmation Dialog, Activity, Results, one-plan-per-row History…"`.

**Why it matters:** FR-19 is the visual-consistency contract. Two surfaces that are new in the D27–D30 block — precisely the ones with no existing styling to inherit — are the two the contract forgets to cover.

**Fix:** add "the separate confirmation dialog" and "Results" to `prd.md:420`, matching NFR-3's enumeration.

---

### C-10 [MEDIUM] §7.3 defers HealthFix while three Shipping-tagged requirements treat it as live

Separate from the code question already logged: the PRD contradicts itself on health work *internally*.

`prd.md:599` (§7.3 Deferred): `"- **Health fixes** (uv broken tool environments, with only an exactly-recognized reinstall suggestion becoming runnable)…"`

Against three places that treat HealthFix as a present Operation kind:

- `prd.md:107` (§3 Glossary): `"**Operation** — one queued unit of work: Refresh, Upgrade, SelfUpdate, or HealthFix."`
- `prd.md:173` (FR-3, `**Status:** Shipping`): `"- A successful Upgrade, SelfUpdate, or HealthFix refreshes every affected subject and executor."`
- `prd.md:261` (FR-8, `**Status:** Shipping`): `"- An in-progress state update, revision drift, an active refresh, or a lock-set overlap with any pending or running Upgrade, SelfUpdate, or HealthFix rejects the submission without enqueueing."`

`prd.md:118` also defines `"**Health issue**"` in the glossary — a defined domain noun for a capability §7.3 says does not exist yet.

**Why it matters:** FR-3 and FR-8 are Shipping and their consequences are written as acceptance criteria. A test author writing FR-8's lock-overlap case has to enqueue a HealthFix; §7.3 says there is none to enqueue. §7.3's own `prd.md:599 "\`[NOTE FOR PM]\` — these were P1 in \`docs/SPEC.md\`, but D33's surviving habit applies: verify whether each already ships"` is the right instinct and is not applied to this row.

**Fix:** resolve §7.3's health row against FR-3/FR-8/§3 in one direction. If HealthFix Operations ship and only the *uv broken-tool-environment fix* is deferred, say that — the current wording defers the whole category.

---

### C-11 [LOW] Two glossary entries are defined and never used; two more are defined capitalized and used lowercase

`prd.md:100` states the glossary is mandatory downstream vocabulary. Four entries do not hold up:

- `prd.md:111 "- **Managed by** — the ownership relationship derived from the detected installation path, surfaced with human-readable evidence."` — `grep -n "Managed by\|managed by" prd.md` returns line 111 and `prd.md:599`'s `"**\"also managed by rustup\"** note on mise's Rust row"`, a quoted UI string about a deferred feature. The term is never used as a term. FR-4, which owns the concept, says `prd.md:180 "which Manager owns each tool"` and `prd.md:183 "Ownership and Route are derived from current detection"` instead.
- `prd.md:118 "- **Health issue** — a Manager-reported warning about a broken Package or tool environment."` — used nowhere else (see C-10).
- `prd.md:108 "- **Executor** — the Manager whose binary actually runs an Operation."` and `prd.md:109 "- **Subject** — the Manager whose state an Operation changes."` are capitalized as terms but appear only lowercase in the body: `prd.md:173 "…refreshes every affected subject and executor."`, `prd.md:186 "- A routed action names both subject and executor in plain language."`, `prd.md:187 "- When the required executor is absent…"`. Line 107 lowercases them too, inside the glossary itself.

**Fix:** use "Managed by" in FR-4's consequences or drop it; use "Health issue" in whichever FR owns the concept or drop it; capitalize Executor/Subject at 107, 173, 186, 187.

---

### C-12 [LOW] `prd.md:94` routes EXPERIENCE.md reconciliation to §9, which contains none

`prd.md:94`:

> Full narratives: `EXPERIENCE.md` lines 373–460. Where a journey beat and an FR here disagree, the FR wins and `EXPERIENCE.md` is reconciled — see §9.

§9 (`prd.md:627`–`649`) contains five open questions and one closed one; no reconciliation of `EXPERIENCE.md` appears in it. §9.1 mentions `EXPERIENCE.md` only to say it was *right*: `prd.md:641 "\`EXPERIENCE.md\` is correct and \`docs/SPEC.md\` F5 is the stale side."` The actual route is two places §9 does not point at: `prd.md:440 "…come out through \`bmad-correct-course\`, a \`bmad-architecture\` Update, and a \`bmad-ux\` Update — never a hand edit."` and `addendum.md:54`'s `EXPERIENCE.md` row.

Contrast `prd.md:236`, which routes correctly — `"and §9 for the reconciliation this triggers in \`ARCHITECTURE-SPINE.md\`"` — and does land on `prd.md:648`.

**Fix:** repoint `prd.md:94` at `addendum.md` §3 (or FR-19's notes), which is where the `EXPERIENCE.md` route actually lives.

---

### C-13 [LOW] The addendum presents two quoted strings as PRD requirement text; neither exists in the PRD

`addendum.md:13`: `"The requirement is \"detection works when launched from Finder\" (FR-1)."`
`addendum.md:15`: `"FR-2's \"fail visibly rather than invent state\" is the requirement; parser shape is implementation."`

```
grep -c "detection works when launched from Finder" prd.md   → 0
grep -c "fail visibly rather than invent state" prd.md        → 0
```

The real text is `prd.md:145 "- Detection succeeds when the app is launched from Finder or the Dock, not only from a terminal."` and `prd.md:160 "- Output a parser cannot handle fails that Manager visibly, with an excerpt, rather than presenting incomplete data as complete."` — both stronger than the paraphrase (line 145 adds the Dock; line 160 adds the excerpt and the *scoping to that Manager*).

**Why it matters:** `addendum.md:9` frames the table as the boundary marker — `"this section exists so nobody re-derives them into the PRD later"` — so these quotes are what a downstream reader compares against when deciding whether something is already covered. Quoting the weaker paraphrase invites re-derivation of the parts it dropped.

**Fix:** requote verbatim from `prd.md:145` and `prd.md:160`, or drop the quotation marks and mark them as summaries.

---

### C-14 [LOW] §10's ID-preservation entry drops RP-1/RP-2, which §0 includes

`prd.md:18`: `"Requirement IDs are **preserved from the prior artifacts** — FR-1…FR-22, RP-1, RP-2, NFR-1…NFR-8 — because \`epics.md\` and \`ARCHITECTURE-SPINE.md\` already cite them"`.

`prd.md:658` restates the same judgment call in the Assumptions Index and omits the RPs: `"- **Requirement IDs were preserved rather than renumbered.** \`epics.md\` and \`ARCHITECTURE-SPINE.md\` already cite FR-1…FR-22 and NFR-1…NFR-8; renumbering would have broken every downstream reference."`

RP-1 and RP-2 are real ids with headings at `prd.md:493` and `prd.md:499` and are cited in the §2.3 table (`prd.md:92`). Since §10 is the roundtrip record a reviewer reads to check the PRD's own decisions, the omission makes the RP namespace look unclaimed.

**Fix:** add "RP-1, RP-2" to `prd.md:658`.

---

### C-15 [LOW] §4.6 is the only feature group with no `Realizes AJ-n` line, though AJ-6 claims RP-1

`grep -n "Realizes" prd.md` → 5 hits: `prd.md:129` (AJ-1), `212` (AJ-2, AJ-3), `308` (AJ-4, AJ-5), `382` (AJ-5), `446` (AJ-6). §4.6 (`prd.md:489`) has only `prd.md:491 "These two are mandatory prerequisites rather than product features. They are validated through \`docs/RELEASE-CHECKLIST.md\`."`

Meanwhile `prd.md:92` maps AJ-6 to `"FR-20, FR-21, FR-22, RP-1"`, so the table asserts a journey link that the group header does not return. RP-2 is mapped to no journey at all in either direction, which is defensible — but it is only defensible if stated.

**Fix:** add `Realizes AJ-6 (RP-1).` to `prd.md:491`, and note that RP-2 is journey-independent.

---

### C-16 [LOW] §1 uses two out-of-glossary names for the closed Manager set

`prd.md:102` fixes the set and its members: `"- **Manager** — one of Homebrew (\`brew\`), mise, npm, uv, rustup, or \`mas\`. Exactly six; the set is closed."`

`prd.md:54` names the same set differently and renames one member: `"…a machine whose software arrives through six different package managers — Homebrew, mise, npm, uv, rustup, and the Mac App Store CLI."`

`prd.md:78` reuses the lowercase form for the complement, which is at least unambiguous: `"- **Users of package managers outside the supported six.**"`

`prd.md:100`'s rule admits no exception for the Vision section.

**Fix:** `prd.md:54` → "six different Managers — Homebrew, mise, npm, uv, rustup, and `mas`". The Mac App Store CLI expansion belongs in §3's Manager entry if it is wanted at all.

---

## Not findings — checked and deliberately not reported

- Absence of the 72-criterion gate, coverage percentages, versioned scenario contracts, evidence manifest, candidate freeze, multi-host environments, `contracts/` (D33). Correctly absent; `prd.md:30` records the exclusion explicitly.
- Absence of keyboard operability, VoiceOver operability, live-region announcements, and focus restoration (D37). `prd.md:430` and `prd.md:659` record these as decisions with owner confirmation; the three survivors D37 protects — focus indicator (`prd.md:425`), the Cmd map (`prd.md:499` RP-2), 4.5:1 contrast (`prd.md:423`) — are all present.
- Zero occurrences of `planAttemptId`, `Verifying`, `InteractionRequired`, `skipUpgradePlanConfirmation` in the codebase. `prd.md:24` states this outright; Planned tags are correct.
- Absence of PATH construction, parser regexes, adapter traits, IPC shapes, scheduler internals, transcript syntax, test names. Mapped at `addendum.md:11`–`20`.
- No adoption metric, rollout plan, sign-off, SLA, or ROI. `prd.md:607 "Adoption metrics would be theater."`
