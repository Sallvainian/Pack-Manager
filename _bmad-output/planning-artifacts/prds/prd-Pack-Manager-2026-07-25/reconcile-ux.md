# UX-spine reconciliation — `DESIGN.md` + `EXPERIENCE.md` vs `prd.md`

**Reviewer:** `DESIGN.md` + `EXPERIENCE.md`
**Date:** 2026-07-25
**Subject:** `_bmad-output/planning-artifacts/prds/prd-Pack-Manager-2026-07-25/prd.md` (659 lines) with `addendum.md` (69 lines)
**Sources:** `_bmad-output/planning-artifacts/ux-designs/ux-Pack-Manager-2026-07-23/DESIGN.md` (252 lines), `EXPERIENCE.md` (460 lines)

Line counts from `wc -l`. Every occurrence count below comes from `grep -c` / `grep -n` against the named file, cited inline.

Out of scope by instruction and deliberately not reported: the keyboard-navigation, VoiceOver, live-region-announcement, and focus-restoration obligations `docs/DECISIONS.md:519` D37 removed; the readiness apparatus D33 retired; commercial-launch expectations; and the Planned/unimplemented status of D27–D30 FRs.

---

## 0. The framing that makes qualitative loss non-hypothetical

The PRD asserts authority over the UX spine:

> `prd.md:16` — "This PRD is the requirements authority. `ARCHITECTURE-SPINE.md` and `epics.md` are reconciled *against* it, not the reverse."

> `prd.md:94` — "Where a journey beat and an FR here disagree, the FR wins and `EXPERIENCE.md` is reconciled — see §9."

and the addendum schedules the reconciliation:

> `addendum.md:54` — "| `EXPERIENCE.md` | 4 keyboard/VoiceOver mentions, plus the `Accessibility Floor` section (lines 313–332) whose announcement and 101-row VoiceOver-reachability obligations D37 removes. Its membership model (line 143) is **correct and survives** — only the accessibility limbs change. | `bmad-ux` (Update intent) |"

But the delegation the PRD grants back to the spine is narrower than the spine:

> `prd.md:28` — "**Upstream inputs already written; not duplicated here.** UX experience contract and interaction detail: `_bmad-output/planning-artifacts/ux-designs/ux-Pack-Manager-2026-07-23/EXPERIENCE.md` (journeys AJ-1…AJ-6, referenced by ID below) and `DESIGN.md` (the approved "Aurora Control Deck" palette, adopted by D35)."

`EXPERIENCE.md` is delegated as *journeys*. `DESIGN.md` is delegated as *a palette*. `EXPERIENCE.md` is 460 lines of which the journeys (`# Key Flows`, lines 373–460) are 88; `DESIGN.md` is 252 lines of which the colour table (lines 105–141) is 37. The remaining 372 and 215 lines — Information Architecture, Voice and Tone, Component Patterns, State Patterns, the lifecycle model, Inspiration & Anti-patterns, Brand & Style, Typography, Layout, Elevation, Do's and Don'ts — are neither carried in the PRD nor named in the delegation, and the addendum's §1 "Mechanisms deliberately excluded from the PRD narrative" table (`addendum.md:11–20`) lists eight rows, all of which are architecture (search path, ownership classification, adapters/parsers, lock sets, IPC, transcript format, test seams, updater transport). **Not one row delegates UX voice, IA, or brand.**

The consequence is concrete rather than theoretical: `addendum.md:54` routes `EXPERIENCE.md` through a `bmad-ux` Update run, and `prd.md:16` tells that run the PRD wins. Anything the PRD does not carry is at risk of being reconciled out of existence by a workflow doing exactly what these two documents instruct.

---

## 1. AJ-1…AJ-6 mapping audit

The PRD's §2.3 table (`prd.md:85–92`) is the mapping under review. I read each narrative at `EXPERIENCE.md:373–460` beat by beat and traced each beat to an FR consequence.

**Verdict: the ID reuse is the right call and the mapping is broadly accurate, but three rows name a beat no listed FR realizes, and two rows truncate the journey before its climax.**

### AJ-1 — Launch, detect, and refresh (`EXPERIENCE.md:377–388`) → FR-1…FR-5

| Beat | FR that realizes it |
| --- | --- |
| `:381` "launches Pack-Manager from Finder or the Dock" | FR-1 `:145` "Detection succeeds when the app is launched from Finder or the Dock"; NFR-7 `:551` ✅ |
| `:382` "window shell and navigation render immediately… independent loading states" | NFR-3 `:525` "State renders progressively without waiting for all Managers"; FR-3 `:170` "Each Manager shows its own loading, phase, timeout, and error state." ✅ |
| `:383` "name, version, role, path, Manager state, managed Package count, Package update count, and health" | FR-1 `:143`, FR-11 `:298` cover name/version/role/path/state. **Package counts and health: no FR.** `grep -ic "health meter" prd.md` = 0 ⚠️ |
| `:384` "Normal absence is labeled as not detected rather than failed" | FR-1 `:144` "Absence is a normal state… never an error." ✅ |
| `:385` "Each Manager refreshes independently" | FR-3 ✅ |
| `:386` **Climax** "the Dashboard settles into `Ready`… or `Warning` with exact failed-refresh context" | **No FR.** ❌ See Finding 8 |
| Failure path `:388` | FR-3 `:171`, FR-16 `:374` ✅ |

### AJ-2 — Review and authorize Update Everything (`EXPERIENCE.md:390–404`) → FR-6, FR-7, FR-8, FR-9, FR-11

| Beat | FR that realizes it |
| --- | --- |
| `:394` "Sallvain selects Update Everything" | **No FR.** ❌ See Finding 6 |
| `:395` eligible items populate the sidecar, ineligible "explicitly omitted with reasons" | FR-7 `:247`, FR-5 `:200` ✅ |
| `:396` grouped by Manager, self-update deltas in group headings, Remove on every staged item | FR-7 `:245`, `:249`; FR-11 `:302` ✅ |
| `:397` "reviews Updates, Managers, and Commands counts" | `grep -n "Updates, Managers" prd.md` = no match. The three-count summary triad has no FR ⚠️ |
| `:398` single blue `Confirm # updates` | FR-7 `:250` ✅ |
| `:399` dialog + skip preference **and the confirmation-off path** | Dialog ✅ FR-7 `:250`. Confirmation-off compensations ❌ See Finding 4 |
| `:400` stale plan reconfirmation | FR-8 ✅ |
| `:401` **Climax** "atomically admits the whole plan **and the sidecar becomes live Upgrade Activity**" | Atomicity FR-9 ✅. Transformation ❌ See Finding 3 |
| `:402` "Activity becomes a persistent Results Summary and the execution becomes one History Plan" | FR-13 `:336`, FR-15 `:364` — **neither is listed in the AJ-2 row** ⚠️ |

### AJ-3 — Update a selected Package or Manager (`EXPERIENCE.md:406–418`) → FR-5, FR-6, FR-10, FR-11

| Beat | FR that realizes it |
| --- | --- |
| `:411` "workspace opens with All Packages selected and actionable updates sorted first" | Sort: FR-5 `:202` ✅. "All Packages" default and `EXPERIENCE.md:74` "Remember the selected filter separately for each Manager during the current session": no FR ⚠️ |
| `:412` checkbox / count-labeled header checkbox / Update Manager | FR-6 `:225–226`, FR-11 `:302` ✅ |
| `:413` immediate membership change, sidecar on first addition, Remove on all, persists across Managers | FR-6 `:229`, FR-7 `:249` ✅ |
| `:414` pinned "muted but readable; hover, click, or focus explains how to unpin and refresh it" | FR-5 `:198` ✅ (strong — it even keeps the disabled checkbox as an explanatory target) |
| `:415` same confirmation path as AJ-2 | FR-7 ✅ |
| `:416` **Climax** "verified Package rows show the single new current version, and the Results Summary confirms exactly what changed" | FR-13 `:336` (Planned) — **not listed in the AJ-3 row.** `EXPERIENCE.md:155` "Only verified rows collapse a version delta to the single new version" has no FR ⚠️ |

### AJ-4 — Slow, blocked, failed, cancelled, interrupted (`EXPERIENCE.md:420–432`) → FR-13, FR-14, FR-15, FR-16

The most faithful row in the table. Every beat lands:

- `:425` waiting reason → FR-9 `:274` "Queued behind Homebrew" ✅
- `:427` null input, 120 s, three actions, Homebrew never auto-retried → FR-12 `:320`, FR-14 `:345–346`, FR-9 `:275` ✅
- `:428` trusted classifier, cancel with no second confirm and no rollback promise, 30-min `Timed out` → FR-14 `:347–348`, `:350` ✅
- `:429` Climax "What happened" / "What to do next" → FR-16 `:375` ✅
- `:430` Retry lineage → FR-15 `:364` ✅
- `:432` Interrupted → FR-15 `:360` ✅

Two beats miss: `:425` "Running work uses indeterminate progress unless the adapter provides trustworthy measurable progress" (Finding 11), and `:430` "A repeated identical cause is called out before another attempt" — `grep -n "repeated" prd.md` returns only `:460`, which is about application-update notification payloads.

### AJ-5 — Diagnose and export support evidence (`EXPERIENCE.md:434–446`) → FR-15, FR-18

Accurate on the export half. Two gaps and one internal inconsistency:

- `:438` "searches or filters for a prior execution" and `EXPERIENCE.md:239` "Search and filters operate on plan time, result, Manager, Package, and relevant text" — `grep -n "search" prd.md` returns `:194` (Package search), `:409` (search *path*), `:503` (search field in menus). **History search/filter has no FR.** ⚠️
- `:440` "If work is currently live, its sidecar remains visible with `Back to live activity`" — no FR ⚠️
- `prd.md:382` says the §4.4 group (FR-17, FR-18, FR-19) "Realizes AJ-5", but the §2.3 AJ-5 row lists FR-15 and FR-18 only. FR-19 in fact underwrites AJ-1 through AJ-6, not AJ-5 specifically (Finding 13).

### AJ-6 — Install and update Pack-Manager (`EXPERIENCE.md:448–460`) → FR-20, FR-21, FR-22, RP-1

Accurate. The whole §4.5 group description (`prd.md:446`) restates the separation invariant well: "Application updates sit outside the Operation queue entirely — they are not Operations, hold no Manager lock, and never appear in History." Two IA details drop: `:453` "Detailed state lives in Settings → Pack-Manager updates, and one restrained application-level badge labeled `Pack-Manager Update Ready!` links there" — `grep -c "Update Ready" prd.md` = 0, and `grep -n "Pack-Manager updates" prd.md` returns only `:77`, an unrelated sentence. Folded into Finding 2.

---

## 2. What the PRD carries correctly (recorded so the Update run does not "fix" it)

- The membership model. `prd.md:236` resolves the F5 conflict in favour of `EXPERIENCE.md:143`, and `addendum.md:54` explicitly protects it: "Its membership model (line 143) is **correct and survives**."
- FR-6 `:231`'s batch-mutation requirement is *stronger* than the spine — `EXPERIENCE.md:182` states the bulk scope but not the round-trip cost. Genuine improvement.
- FR-5 `:198`'s explanatory-disabled pinned checkbox preserves the pointer-facing half of `EXPERIENCE.md:143` exactly as D37 requires.
- FR-19 `:425`'s `outline`-not-`ring-*` focus mechanism is more specific than `EXPERIENCE.md:318` and correctly framed as a code-quality guard, not an accommodation.
- Non-color status cues (`prd.md:422`, `:543`) match `DESIGN.md:134` "Status colors always travel with a word, icon, count, meter length, or version label."
- §6 Non-Goals absorbs `DESIGN.md:249–250` (no `sudo`, no automatic retries) and `EXPERIENCE.md:356` (silent restart) faithfully.

---

## 3. Findings

### F1 — HIGH — the entire `# Voice and Tone` section has no PRD representation and is not delegated

`EXPERIENCE.md:112–134` is 23 lines defining the product's voice. Counts against `prd.md`: `grep -ic "tone"` = **0**, `grep -ic "sentence case"` = **0**. Against `addendum.md`: `grep -ci "tone"` = **0**; its three "Voice" hits are all the string `VoiceOver` in the §3 reconciliation table.

The section is not architecture, so `addendum.md:11–20` does not delegate it, and `prd.md:28` delegates only "journeys AJ-1…AJ-6". It is simply gone.

Load-bearing items lost:

- `EXPERIENCE.md:114` — "Use plain, calm, specific language. Prefer what happened, what is known, and what the user can safely do next."
- `EXPERIENCE.md:120` — "Name the object that failed: `rustup refresh failed`, not `Something went wrong`."
- `EXPERIENCE.md:121` — "Distinguish saved from live information: `Showing snapshot from 10:42 AM`." (The PRD requires the last-good Snapshot be "labeled stale" at `:171` but never that the label carry its timestamp.)
- `EXPERIENCE.md:122` — "Do not call Update availability a System health problem. System health reflects reliability and operating state."
- `EXPERIENCE.md:134` — "`10 of 12 verified · 2 failed`" preferred over "`10 of 12 updated` when verification did not succeed", restated at `:242` as "History summaries use verified outcome language… never a generic completion ratio."

That last pair is not decoration. `prd.md:612` SM-2 makes an unreviewed mutation "a P0 defect, not a metric miss", and `prd.md:617` SM-4 stakes failure legibility on the transcript answering "what ran and what happened". Outcome wording that overstates verification is the exact failure both metrics exist to prevent, and the rule that forbids it now lives only in a document scheduled for regeneration.

**Fix:** add a short "Voice" subsection to §4.4 or a bullet block under FR-16, carrying at minimum: name the failed object; never state certainty the system does not have; verified-outcome wording for any success claim; uppercase reserved for status chips. Alternatively add a Voice/Tone row to `addendum.md` §1 delegating it to `EXPERIENCE.md` by name — but the §1 table's stated purpose is mechanisms, and a delegation row is the weaker option because `prd.md:16` still lets a `bmad-ux` Update override it.

---

### F2 — HIGH — the specific information architecture is reduced to a one-line enumeration

`EXPERIENCE.md:41–76` specifies the navigation model in three tables and a numbered workspace structure. The PRD's whole IA statement is:

> `prd.md:420` — "One coherent dark-only interface spans the Dashboard, Manager navigation and workspaces, the Upgrade Plan, Activity, History, Settings, status surfaces, and application menus."

That is a list of nouns. Counts against `prd.md`: `grep -c "disclosure"` = **0**, `grep -c "collapsed"` = **0**, `grep -c "no active upgrade"` = **0**, `grep -c "workspace"` = **1** (line 420 itself).

Decisions dropped:

- `EXPERIENCE.md:46` — "**Managers** | Disclosure control for detected Managers | Collapsed by default; expanding reveals Managers, and selecting one opens its dedicated workspace"
- `EXPERIENCE.md:51` — "The Managers item is not a separate all-Managers page. Dashboard Manager cards provide the overview; the disclosure list is the direct switcher for dedicated Manager workspaces."
- `EXPERIENCE.md:45` — "No permanent Manager list; main content uses full width when the Upgrade Sidecar is absent"
- `EXPERIENCE.md:47` — Activity "When idle, says there is no active upgrade", restated at `:162` "Idle Activity explicitly says no upgrade is active instead of displaying drafts or completed plans."
- `EXPERIENCE.md:49` — Settings "One scrollable page; advanced sections are collapsed by default", with the five-group structure at `:250`.
- `EXPERIENCE.md:67–74` — the four-part Manager workspace order, and "All Packages is the default. Remember the selected filter separately for each Manager during the current session."
- `EXPERIENCE.md:148` — "Counts always reflect the full current Manager dataset, not just rendered rows."
- `EXPERIENCE.md:453` — application-update detail lives in Settings → Pack-Manager updates behind one badge labeled `Pack-Manager Update Ready!`.

`EXPERIENCE.md:74`'s per-Manager filter memory is a plain functional requirement with no home anywhere in the PRD, and `EXPERIENCE.md:148`'s count-honesty rule is the same rule FR-6 `:226` already states for the header checkbox — the PRD kept one half and dropped the other.

**Fix:** give FR-19 a consequence block naming the five destinations and their load-bearing behaviors (Managers is a collapsed disclosure and not a page; no permanent Dashboard Manager list; idle Activity states that no upgrade is active; Settings is one scrollable page), and move the per-Manager filter memory and full-dataset counts into FR-5.

---

### F3 — HIGH — the lifecycle model and the one-surface continuity are gone

`EXPERIENCE.md:78–96` is an explicit state machine:

> `EXPERIENCE.md:81–93` — "No draft / → user adds eligible Package or Manager / Draft Upgrade Plan / → user reviews and confirms / Confirmation Dialog / → final authorization / Live Upgrade Activity / → every operation reaches a terminal state and affected state is refreshed / Results Summary / → attempt is persisted / History attempt / → user opens it / Read-only Activity replay"

restated as a principle at `EXPERIENCE.md:35` — "**One object across each attempt.** Draft Upgrade Plan → live Upgrade Activity → persistent Results Summary → exactly one History entry."

The PRD scatters the phases across FR-7 (draft, confirmation), FR-13 (Activity, Results) and FR-15 (History) and never states that they are one surface transforming in place. `grep -n "sidecar" prd.md` returns four hits: `:240` and `:249` (FR-7) and `:430`/`:659`, which are the D37 note. Nothing carries `EXPERIENCE.md:153` "It persists across Manager changes and transforms into Activity and Results" or `:225` "The sidecar becomes Results and remains until the user chooses `Done`" — `grep -c "Done" prd.md` = 1, and that hit is the heading "### 2.1 Jobs To Be Done".

Also lost: `EXPERIENCE.md:96` — "A draft never appears in Activity or History", which `DESIGN.md:245` restates as a Don't and `EXPERIENCE.md:352` as an anti-pattern.

A downstream epic writer reading only the PRD has FR-7 (build a sidecar), FR-13 (build Activity and Results) and FR-15 (build History) as three independent Planned deliverables with no continuity requirement. That is exactly how three surfaces get built where the design specifies one.

**Fix:** add the lifecycle chain as a stated consequence — either as an opening invariant to §4.2/§4.3 or as a new consequence on FR-13 — including "one surface transforms in place", "a draft never appears in Activity or History", and "Results persists until `Done`".

---

### F4 — CRITICAL — FR-7's confirmation opt-out drops all three compensating affordances

FR-7's Planned D28 block is:

> `prd.md:250` — "**Planned — D28:** `Confirm N Updates` opens a separate modal confirmation showing the exact commands, offering `Change Plan` and final confirmation. The opt-out checkbox appears *only* in that dialog and persists `skipUpgradePlanConfirmation`. The safe default is confirmation enabled (`false`), and Settings can restore it. The preference removes only the final dialog — never the draft review, the backend rebuild, or the stale-plan check."

The UX spine specifies what replaces the dialog when it is off:

> `EXPERIENCE.md:190` — "With confirmation disabled, exact commands automatically expand before the action is enabled; show `Confirmation is off. Changes will run immediately when you choose Run # updates. Change in Settings.` and label the immediate action `Run # updates`."

> `EXPERIENCE.md:153` — "When confirmation is off, commands are expanded, a persistent `Confirmation is off` warning links to Settings, and the immediate action is `Run # updates`."

> `DESIGN.md:213` — Upgrade Sidecar required states include "confirmation off"; "When confirmation is off, show a persistent warning and `Run # updates`."

> `EXPERIENCE.md:133` — Prefer "`Confirmation is off · Change in Settings`" / Avoid "An immediate execution button that still says `Confirm`".

Counts against `prd.md`: `grep -c "Confirmation is off"` = **0**, `grep -c "Run #"` = **0**.

All three compensations — auto-expanded commands, the persistent warning linking back to Settings, and the relabel from `Confirm` to `Run` — are absent. Built from the PRD as written, the skip preference produces a button still labeled `Confirm N Updates` that executes without a dialog, with commands still collapsed behind a reveal. `EXPERIENCE.md:133` names that outcome as the thing to avoid, and `prd.md:612` SM-2 calls a single unreviewed mutation "a P0 defect, not a metric miss".

This is the highest-consequence drop in the review: the one FR where removing a safety gate was authorized, and the compensations that were the price of that authorization did not come across.

**Fix:** extend FR-7's D28 bullet — when `skipUpgradePlanConfirmation` is true, the plan auto-expands exact commands before the action is enabled, shows a persistent `Confirmation is off` notice linking to Settings, and the action relabels to `Run N updates`. The label must not remain `Confirm`.

---

### F5 — HIGH — "one active attempt at a time" appears only in §7.2 scope prose, and FR-8's admission test does not imply it

The invariant:

> `EXPERIENCE.md:100–102` — "Only one confirmed Upgrade Plan attempt may be active. A user may continue reviewing a draft, but it cannot be confirmed until the active attempt is terminal. Cross-Manager concurrency occurs inside the one active attempt."

`grep -n "active attempt\|One active" prd.md` returns exactly one line, and it is not in the requirement body:

> `prd.md:593` — "- One active attempt at a time, a `Verifying` state before success is declared, Activity as a first-class destination, a terminal Results summary, attempt-wide `Cancel plan`, and trusted-classifier-only `Interaction required` (D30)."

That is §7.2 "Decided, not yet implemented", a scope inventory. No FR or NFR states it. And FR-8's admission test does not produce it:

> `prd.md:261` — "An in-progress state update, revision drift, an active refresh, or a lock-set overlap with any pending or running Upgrade, SelfUpdate, or HealthFix rejects the submission without enqueueing."

Two confirmed attempts with **disjoint** lock sets — say one Homebrew-only and one rustup-only — have no lock-set overlap and would pass FR-8 as written. The lock-set rule prevents conflicting *Operations*; it does not prevent a second concurrent *attempt*. Under D29/D30 the attempt is the unit of Activity, Results, and History (`prd.md:114`), so two live attempts would mean two live Activity surfaces and two Results — precisely what `EXPERIENCE.md:35`'s "One object across each attempt" is built to prevent.

Downstream artifacts cite FR/NFR IDs. An invariant that exists only in a scope bullet will not survive the `bmad-architecture` Update the addendum schedules.

**Fix:** add "at most one confirmed Plan Attempt is active; a further plan may be drafted and reviewed but cannot be confirmed until the active attempt reaches a terminal state" as a consequence of FR-8 or FR-9.

---

### F6 — HIGH — `Update Everything` is named in the Vision and in AJ-2 step 1 but has no FR

`grep -n "Update Everything" prd.md` returns exactly two lines, and neither is normative:

- `prd.md:88` — the AJ-2 table row, which is a pointer to `EXPERIENCE.md`.
- `prd.md:212` — the §4.2 group description: "Every path to a mutation — a Package row, a Manager header, a Manager-wide action, `Update Everything` — converges on one reviewable Upgrade Plan".

The spine treats it as a primary product surface:

> `EXPERIENCE.md:45` — "**Dashboard** | System overview, Update Everything entry point, and Manager cards"

> `EXPERIENCE.md:57` — Dashboard "Adds Update Everything or a Manager self-update; sidecar opens only after something is added"

> `EXPERIENCE.md:394` — "1. Sallvain selects Update Everything."

and the PRD's own Vision promises it: `prd.md:54` — "updates everything, a chosen subset, a single Package, or a Manager itself."

FR-6 `:226` covers a header checkbox scoped to "every eligible Package matching the active filter" — that is per-Manager, per-filter. FR-10 covers the single-Package path. Nothing covers a single action that stages every eligible update across every Manager. The one action the product is named after has an FR for its narrow case and its narrowest case, and none for its widest.

**Fix:** add a consequence to FR-6 (or a short FR-6b) — a Dashboard-level `Update Everything` action stages every eligible Package and Manager self-update across all detected Managers into the Upgrade Plan draft, subject to every eligibility and exclusion rule in FR-5, and executes nothing.

---

### F7 — HIGH — five of the ten "Anti-patterns to reject" have no PRD home

`EXPERIENCE.md:345–356` lists 10 anti-patterns (`grep -c '^- '` over that span = 10). §6 Non-Goals (`prd.md:561–575`) is their natural destination. Traced individually:

| Anti-pattern (`EXPERIENCE.md`) | PRD home |
| --- | --- |
| `:347` "Copying proprietary Kanban Pro code, assets, branding, exact layouts, or distinctive visuals." | **none** — `grep -ic "kanban" prd.md` = 0, `grep -ci "kanban" addendum.md` = 0 |
| `:348` "A permanent redundant Manager list on the Dashboard." | **none** (see F2) |
| `:349` "Terminal-first Upgrade Plan or Activity presentation." | **none.** §6 `:568` "**A terminal.** No general shell surface, no arbitrary command execution, no user-supplied argument vectors" is about execution surface, not presentation. `EXPERIENCE.md:33`'s "Human meaning before terminal evidence" principle is unrepresented |
| `:350` "Immediate row-level execution." | FR-6 `:230`, FR-10 `:287` ✅ |
| `:351` "A permanently visible empty right drawer." | FR-7 `:249` "hidden when empty" ✅ |
| `:352` "A queue of draft plans in Activity." | **none** (see F3) |
| `:353` "One History row per Package or command." | FR-15 `:364` ✅ |
| `:354` "A gradient-filled Package-health bar or fabricated progress." | **none** (see F11) |
| `:355` "Unexplained route/owner jargon." | FR-4 `:187`, FR-11 `:300` ✅ |
| `:356` "Silent retries, silent restart, or requests for administrator credentials." | §6 `:566`, `:572`; FR-21 `:470` ✅ |

`:347` is the sharpest of the five. `DESIGN.md:101` states it as a design constraint with legal shape:

> `DESIGN.md:101` — "The visual personality borrows only general qualities observed in Kanban Pro: layered dark surfaces, compact outlined controls, visible hierarchy, a 4/8-based rhythm, and smooth short motion. Pack-Manager must remain an original design. Do not copy Kanban Pro code, assets, branding, layouts, or distinctive component treatments."

An originality constraint on an open-source project that borrows from a named proprietary product is a product-level boundary, and §6 is a list of product-level boundaries. It appears in neither the PRD nor the addendum.

**Fix:** add the five unhomed items to §6 Non-Goals. Four are one line each; `:347` deserves its own bullet naming Kanban Pro and the originality requirement.

---

### F8 — MEDIUM — AJ-1's climax (Dashboard `Ready` / `Warning` system health) is named in the mapping table but realized by no FR

The PRD's own AJ-1 row promises it:

> `prd.md:87` — "…and settles into `Ready` or `Warning` with exact failed-refresh context. | FR-1, FR-2, FR-3, FR-4, FR-5 |"

`grep -n "System health" prd.md` = **0 matches**. The only `Ready`/`Warning` occurrence in the whole PRD is line 87 itself. FR-3 carries the per-Manager half — `prd.md:171` "A failed refresh retains the Last-good Snapshot, keeps it browsable, and labels it stale" — but nothing aggregates to a system verdict.

The spine states it three times, twice as a prohibition:

> `EXPERIENCE.md:145` — "System health becomes `Warning` when any Manager refresh has failed even if a last-good snapshot keeps the app usable."

> `EXPERIENCE.md:122` — "Do not call Update availability a System health problem. System health reflects reliability and operating state."

> `DESIGN.md:247` — "Do not call the system `Ready` when any Manager refresh has failed; use `Warning` with the exact failure count."

Both prohibitions guard against the same failure: a Dashboard that reads `Ready` while data is stale, or that turns "you have updates" into "your system is unhealthy". Neither survives.

**Fix:** add a consequence to FR-3 or FR-19 — the Dashboard aggregates a single system state that is `Warning` whenever any detected Manager's most recent refresh failed, with the exact failure count; update availability never affects it.

---

### F9 — MEDIUM — `DESIGN.md` is reduced to a palette; the brand direction and its constraints are dropped

Both PRD references treat `DESIGN.md` as a colour list:

> `prd.md:28` — "`DESIGN.md` (the approved "Aurora Control Deck" palette, adopted by D35)"

> `prd.md:421` — "The visual identity is the approved "Aurora Control Deck" palette in `DESIGN.md`, adopted by D35, with a recognizable package/update application icon."

`DESIGN.md:99` defines it as a direction, not a palette:

> `DESIGN.md:99` — "Pack-Manager should feel like a quiet native control deck: technically capable, visually calm, and always honest about what it knows. The selected direction is **Aurora Control Deck**—a dark blue-graphite workspace with restrained glass depth, crisp blue controls, compact information density, and status colors that remain legible without becoming decorative noise."

> `DESIGN.md:103` — "Native macOS conventions—traffic-light window controls, system typography, expected focus behavior, and restrained motion—take priority over web-dashboard decoration."

The PRD gets closest at `:417` — "The app reads as one focused macOS control surface, not six command wrappers in a window" — which captures the coherence but not the *feel*: calm, restrained, honest, dense-but-breathing. Related losses in the same class:

- Motion. `grep -in "motion" prd.md` returns four hits: `:68`, `:388` (both the word "emotional"/"demotion"), `:424` "The reduced-motion preference disables transitions", `:543`. The positive spec is gone — `EXPERIENCE.md:306` "Use 120–180ms for hover, focus, and selection feedback; 180–260ms for sidecar/layout transitions; up to 400ms for significant state transforms" and `:311` "Do not use celebratory confetti, bouncing cards, or continuous decorative glow". The PRD requires motion be *disableable* without ever requiring it be *restrained*.
- Depth. `DESIGN.md:188` — "No glass effect may reduce text contrast. Avoid stacked shadows, neon glows on status colors, and decorative blur behind dense tables." This is a contrast guard, and D37 explicitly preserves contrast (`docs/DECISIONS.md` D37, "**Contrast.** D36's guard stays"). It is unrepresented.

**Fix:** in FR-19, replace "palette" with the direction — cite `DESIGN.md`'s Brand & Style section, not just its colours — and add one consequence carrying restraint: no celebratory or continuous decorative motion, and no depth or glass treatment that reduces text contrast.

---

### F10 — MEDIUM — Glossary "Managed by" conflicts with the required user-facing string "Managed through", with no reconciliation entry

The Glossary is normative and exclusive:

> `prd.md:100` — "Downstream workflows must use these terms exactly. No synonyms appear anywhere else in this document."

> `prd.md:111` — "- **Managed by** — the ownership relationship derived from the detected installation path, surfaced with human-readable evidence."

The spine mandates a different string, in the user's copy, three times:

> `EXPERIENCE.md:123` — "Describe delegated ownership as `Managed through <Manager>`; reserve internal Route/owner terminology for technical evidence."

> `EXPERIENCE.md:150` — "Use `Managed through mise` for delegated ownership."

> `EXPERIENCE.md:187` — "Delegated ownership uses `Managed through <Manager>` and explains that the update is grouped and executed through that Manager."

`grep -n "Managed through" prd.md` = **0 matches**. The two terms are not interchangeable: `EXPERIENCE.md:123` distinguishes the user-facing phrasing from internal ownership vocabulary, which is exactly the distinction "No synonyms appear anywhere else" collapses. Neither `prd.md` §9 nor `addendum.md:54` lists this as a conflict to reconcile, so a `bmad-ux` Update taking the PRD as authority will most likely rewrite the UI string to match the Glossary and lose the distinction.

**Fix:** either state in the Glossary that `Managed by` is the internal concept and `Managed through <Manager>` is its user-facing rendering, or add the conflict to `addendum.md` §3's `EXPERIENCE.md` row so the Update run resolves it deliberately.

---

### F11 — MEDIUM — the honesty rules for progress and Package health are dropped, though they are the same class as FR-2

FR-2 is the PRD's strongest requirement — `prd.md:153` "A Package is Outdated when and only when its Manager says so", with `:159` "An unknown latest version stays unknown; the UI says 'update available' rather than fabricating a version or a severity." The spine applies the identical rule to two more surfaces, and neither survives:

> `EXPERIENCE.md:212` — "Running progress is indeterminate unless the adapter provides a trustworthy measurable total."

> `EXPERIENCE.md:155` — "Use indeterminate motion unless a trustworthy percentage exists."

> `EXPERIENCE.md:152` — Health Meter: "Fill length equals current Packages ÷ managed Packages. The fill is one solid color selected from the health scale… Unknown or failed refresh uses text instead of invented health."

> `DESIGN.md:246` — "Do not use a gradient inside the Package Health Meter, fabricate progress percentages, or use color as the only signal."

Counts against `prd.md`: `grep -ic "health meter"` = **0**; `grep -n "indeterminate"` = **0 matches**. The only progress language is `prd.md:336` "plan-level progress correlated by `planAttemptId`", which says nothing about determinacy.

"Fabricated progress" is the same defect as a fabricated version number, and the PRD's Non-Goals already name the version case — `:569` "**A version oracle.** It never replaces a Manager's outdated verdict with its own comparison."

**Fix:** add to FR-13 — progress is indeterminate unless the executing adapter supplies a trustworthy measurable total; a percentage is never synthesized. Add the Package-health proportion display to FR-5 or FR-19 with the "text instead of invented health" rule for unknown/failed refresh.

---

### F12 — MEDIUM — the high-zoom layout mode is replaced by "narrow widths scroll", which is weaker and partly contrary

The PRD's statement:

> `prd.md:427` — "The interface remains usable at 900 × 600, at 150–200% zoom, with more than 100 Packages, and with long command output. Narrow widths scroll rather than letting essential content collide."

The spine specifies a layout *mode change*, in two places outside the Accessibility Floor section (so this is not a D37-removed obligation — the PRD itself keeps 150–200% zoom at `:427`, `:525`, and `:543`):

> `EXPERIENCE.md:366` — "Below 720 usable CSS pixels—such as 150–200% zoom at the minimum window—enter high-zoom mode: collapse navigation to an accessible rail or temporary panel and present Plan/Activity/Results as a full-workspace or stacked surface with a clear Back route."

> `DESIGN.md:173` — "At 150–200% zoom, or whenever the usable CSS width drops below 720px, switch to a high-zoom layout: collapse the sidebar into an accessible navigation rail or temporary panel and present the Upgrade Plan, Activity, or Results as a full-workspace/stacked surface instead of retaining a fixed sidecar. Required headings, status, versions, actions, focus order, and a visible route back must remain available without overlapping panes or two-dimensional scrolling for the primary task."

> `DESIGN.md:252` — "…or retain fixed panes when zoom makes them overlap."

`grep -c "rail" prd.md` = **0**, `grep -c "stack" prd.md` = **0**. "Narrow widths scroll" is not merely thinner than the spine — `DESIGN.md:173` forbids "two-dimensional scrolling for the primary task", so an implementer building to the PRD's sentence can satisfy it while violating the design. The specific mechanism (collapse the 190px sidebar, stack the sidecar, provide a Back route) has no PRD home.

**Fix:** restate `:427`'s last sentence as the mode change — below roughly 720 usable CSS pixels the sidebar collapses and the Plan/Activity/Results surface becomes full-workspace with a visible route back; panes never overlap and the primary task never requires two-dimensional scrolling.

---

### F13 — LOW — the §2.3 mapping table truncates two journeys and §4.4's "Realizes AJ-5" does not match it

Two small inconsistencies in an otherwise sound table:

- `prd.md:88` ends AJ-2 at "final confirmation atomically admits the whole plan", but the narrative continues two beats further — `EXPERIENCE.md:401` "the sidecar becomes live Upgrade Activity" and `:402` "Activity becomes a persistent Results Summary and the execution becomes one History Plan". The FRs that realize those (FR-13, FR-15) are assigned exclusively to AJ-4 and AJ-5 in the table, so a reader auditing AJ-2 for completeness never reaches them.
- `prd.md:89` ends AJ-3 at "no row or header action executes immediately", omitting its climax at `EXPERIENCE.md:416` — "verified Package rows show the single new current version, and the Results Summary confirms exactly what changed" — again FR-13.
- `prd.md:382` says the §4.4 group "Realizes AJ-5", but the §2.3 AJ-5 row (`:91`) lists only FR-15 and FR-18, not FR-17 or FR-19. FR-19 in fact underwrites every journey; claiming it for AJ-5 alone is misleading in both directions.

**Fix:** add FR-13 and FR-15 to the AJ-2 and AJ-3 rows; change §4.4's line to "Realizes AJ-5; FR-19 underwrites all journeys."

---

## 4. Summary of the queue this creates

Nothing here is a hand edit to a workflow-owned file. The PRD is the artifact under review and the changes belong in it:

| Finding | Destination in `prd.md` |
| --- | --- |
| F4 confirmation-off compensations | FR-7, D28 bullet |
| F1 voice | new block in §4.4 or under FR-16 |
| F2 information architecture | FR-19 consequences; FR-5 for filter memory and counts |
| F3 lifecycle continuity | §4.2/§4.3 invariant or FR-13 |
| F5 one active attempt | FR-8 or FR-9 |
| F6 `Update Everything` | FR-6 consequence or new FR-6b |
| F7 anti-patterns | §6 Non-Goals |
| F8 system health | FR-3 or FR-19 |
| F9 brand direction | FR-19, §0 line 28 |
| F10 Managed by / through | §3 Glossary, or `addendum.md` §3 |
| F11 progress and health honesty | FR-13, FR-5 |
| F12 high-zoom mode | FR-19 line 427 |
| F13 mapping table | §2.3, §4.4 |

If only one is taken: **F4**. It is the sole finding where the omission produces a shipped behaviour the design explicitly names as forbidden, on the one requirement whose whole purpose is to remove a safety gate.
