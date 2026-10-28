# Reconcile — `docs/DECISIONS.md` against ARCHITECTURE-SPINE revision 10

**Lens:** decision reconciliation (D37 and D36 focus, D1–D37 sweep)
**Target (read-only):** `_bmad-output/planning-artifacts/architecture/architecture-Pack-Manager-2026-07-23/ARCHITECTURE-SPINE.md`

**Reviewed at:** `md5 c2d62373f807bfc9a192c07ba7a973a6`, **1343 lines**. The
target was being written concurrently — it changed five times during this review
(`96076f25…` → `9f010fcc…` → `3c74130d…` → `503a26b3…` → `c2d62373…` →
`3bf26693…` → `d34e9190…`, 1286 → 1395 lines). **Every spine line number below
was re-verified against `c2d62373`** and is also cited by `AD` id or row title,
which do not move; by `d34e9190` the Decision-Status rows had shifted about +50
lines, so prefer the titles. Findings were then re-tested against `d34e9190`:
**H-1, H-2, M-2, M-4, M-5, M-6 and all three LOWs are still live there; M-1 and
M-3 were fixed in flight** and are kept as records of what was fixed rather than
deleted.

**Repo state:** `HEAD = 1ac959e`. `git status --short` shows only `.memlog.md`
and `ARCHITECTURE-SPINE.md` modified, so every `docs/`, `src/`, `tests/` and
`.github/` quote below is HEAD content, not working-tree content.

**Verdict:** D37 is applied to the spine correctly and completely — all four
revision-9 sites carrying a removed obligation are gone, all three protected
items survive by name, and the scoping instruction is stated where a later pass
will hit it. D36 is applied correctly and every automation claim it rests on is
true at HEAD. The residue is at the seams: a live decision (D15) that contradicts
an AD rule with no supersession recorded anywhere, one false claim about what
`docs/RELEASE-CHECKLIST.md` contains, and four ledgers that have gone stale
around the two new decisions.

**Counts:** 0 CRITICAL · 2 HIGH · 6 MEDIUM · 3 LOW — of which **9 are still live
at the newest hash checked (`d34e9190`)** and 2 (M-1's spine half, M-3) were
fixed in flight while this review was being written.

---

## What was verified clean (so it is not re-raised next revision)

**D37, direction (a) — a removed obligation still stated as live in the spine:
none.** Revision 9 carried four; all four are gone. Candidate sites were found
with `grep -nEic "voiceover|screen[- ]reader|keyboard|aria|announce|assistive|live region"`
against `git show HEAD:…/ARCHITECTURE-SPINE.md > /tmp/spine-rev9-HEAD.md`
(`artifact_revision: 9`, 29 matching lines), then read one by one:

| rev-9 site | rev-9 text | rev-10 disposition |
| --- | --- | --- |
| `:332` | `"One manual VoiceOver pass and a by-eye"` | AD-11 `:366` — `"No manual VoiceOver pass, keyboard-navigation pass, or by-eye contrast sweep sits on docs/RELEASE-CHECKLIST.md"` |
| `:526-527` | `"control is non-interactive to pointer and keyboard, it carries a stated reason for assistive technology"` | AD-16 `:566-567`, `:575-577` — `"the control stays a pointer-interaction target that states the reason"` … `"Exposing the reason to assistive technology is not a release criterion"` |
| `:610-617` | `"There is exactly one status-announcement channel …"` (an obligation, at polite priority, for a VoiceOver user) | AD-17 — `"No story is obliged to build a status-announcement channel. If one exists there is exactly one"` |
| `:941` | `"the manual VoiceOver-and-keyboard pass on docs/RELEASE-CHECKLIST.md"` as AD-27's fallback | AD-27 `:1025-1026` — `"D37 removed the manual keyboard-and-VoiceOver pass this rule previously named as the fallback"` |
| `:1015` | Capability Map row `"Package state, eligibility, keyboard selection (Stories 3.1, 3.2, 3.5)"` | now `"Package state, eligibility, and membership control — the checkbox, header tri-state, and shift-range"` |

**D37, direction (b) — the three items D37 protects by name:** all present.

- *Focus indicator.* AD-27 `:963` is `[ADOPTED]`, binds every story that renders a
  control, and refuses its own deletion at `:971`: `"**This AD survives D37, which
  preserves the focus indicator by name**"`, with the scoping instruction
  `"Scope D37 by named heading, never by mention count"`. True at HEAD:
  `grep -rn "focus-visible:" src/ | wc -l` → **31**; `grep -rn "outline-none" src/
  | wc -l` → **0**; the only product-code `ring-` is
  `src/components/manager/PackageRow.tsx:85` `"highlighted ? \"ring-2 ring-inset
  ring-accent\" : \"\""` — the navigation highlight D35 keeps deliberately.
- *Clipboard + accelerator map.* Kept, but mis-described — H-2.
- *Contrast.* AD-27 `:1008-1018` and the Styling convention row.

**D36 — every "what is automated" statement is true at HEAD.**
`grep -rn "text-white" src/ | wc -l` → **0**.
`grep -rn "on-accent\|on-success" src/` → `theme.css:30`, `theme.css:32`,
`UpdateStatusItem.tsx:63`, `Button.tsx:7`, `:11`, `:13`.
`git log --oneline -1 a201fb0` → `fix(ui): use the palette's dark ink on bright
accent fills`, touching `Button.tsx`, `UpdateStatusItem.tsx`, and
`tests/e2e/browser-style-contract.spec.ts` (+98 lines). The guard measures
rendered values, as AD-27 `:1012-1014` claims:
`tests/e2e/browser-style-contract.spec.ts:289` `"// WCAG 2.1 relative luminance,
then the (L1+0.05)/(L2+0.05) ratio."`, `:320`
`expect(measured.ratio).toBeGreaterThanOrEqual(4.5);`, `:319`
`expect(measured.color).not.toBe("rgb(255, 255, 255)");`.
AD-11 `:349-350`'s `"is the inventory of what is automated"` holds —
`grep -rln "outlineColor\|reducedMotion\|prefers-reduced-motion\|luminance" tests/`
matches **only** that spec. Its CI claim holds — `.github/workflows/test.yml:8-12`
`on: push: branches: [main]` / `pull_request: branches: [main]`. AD-27
`:1019-1020`'s proxy claim holds — `test.yml:56` `runs-on: ubuntu-latest`,
`playwright.config.ts:85` `name: "webkit"`. The rule lands where a builder reads
(AD-27 `:1008` and the Styling convention row), not only in a status row — and
the status row `:1311` says so itself: `"The rule a builder reads lives in the
Styling convention and in AD-27, not in this row."`

**AD-11's release-side claims, verified at HEAD:** three `macos-15` pins and no
`macos-14` (`grep -rn "runs-on:" .github/workflows/` → `release.yml:63`,
`ci.yml:28`, `ci.yml:70`); `minisign -V` at `release.yml:319`; the `latest.json`
reachability check at `release.yml:387-391`. `docs/RELEASE-CHECKLIST.md` carries
no VoiceOver, keyboard or by-eye-contrast step — `grep -nEi "voiceover|Tab|arrow"`
returns only `:103-104`, the paragraph recording their deliberate removal. All
six commits the spine cites resolve: `a201fb0`, `22ed41e`, `be1f0e6`, `8d36cdf`,
`419dc32`, `7cc7b5f`.

**Supersession order, D1–D37.** Citations in the spine
(`grep -oE "\bD[0-9]{1,2}[a-z]?\b" | sort | uniq -c`): D20 ×2, D25 ×1, D25a ×4,
D27 ×5, D30 ×3, D31 ×8, D32 ×3, D33 ×8, D34 ×8, D35 ×9, D36 ×7, D37 ×26. No
superseded decision is cited alone: D33 always with D37 (`:365`), D31 with D34
(`:344-347`, and the *Minimum supported macOS* row's `"cite D34 for the closure
and never D31 alone"`), D20 with D34 (`:338`), D6 and D18 never cited. Decisions
restated without citation (D2, D3, D4, D5, D10, D12, D13, D14, D26, D28, D29)
were checked clause by clause and none is misstated — e.g. D4's `"Semaphore(4)
global cap"` / `"120s aging guard"` against AD-4's `"global concurrency cap of 4,
the 120s aging guard"`; D5's `"in-band override → delegated-to-detected-owner →
native → unavailable"` against AD-4's `"in-band override, then
delegated-if-detected, then native, then unavailable"`.

---

## HIGH

### H-1 — D15 mandates the native `disabled` checkbox AD-16 forbids, and no document records a supersession

`docs/DECISIONS.md:61-63` (D15 — live; `grep -n "D15" docs/DECISIONS.md` returns
exactly one hit, its own heading, so no later decision touches it):

> ## D15. Pinned brew formulae are never upgradable in-app
>
> **Disabled checkbox + tooltip with the `brew unpin` command**; excluded from
> every plan.

`ARCHITECTURE-SPINE.md:566-575` (AD-16, *Ineligible-item inertness*):

> Its control is **inert, not inactive** … It therefore **may not use the native
> `disabled` state** — a natively disabled control cannot receive the pointer
> interaction the reason requires … The shipping row is the defect this names,
> not the reference — it is natively `disabled` with reduced opacity today.

The shipping row is not drift; it is D15 implemented literally.
`src/components/manager/PackageRow.tsx:69-71`:

> `const checkboxTitle = pkg.pinned`
> `  ? \`Pinned in Homebrew — run \\\`brew unpin ${pkg.name}\\\` to upgrade\``

with `:92` `disabled={checkboxDisabled}` and `:100`
`"disabled:cursor-not-allowed disabled:opacity-40"` — a disabled checkbox plus a
tooltip carrying the `brew unpin` command, precisely D15's two clauses.

AD-16 cites the newer authority and that authority does say this —
`prd.md:200`: `"rendered in a *visually* disabled treatment that **must not be
the native disabled state**"`. What is missing is the record that it *overrides a
decision*. This repo supersedes rather than rewrites; D15's mechanism clause has
no supersession note, and AD-16 never names D15 (`grep -n "D15"` over the spine:
zero hits).

**Failure it produces:** UX-PB.1d removes `disabled` under AD-16; a reviewer or a
regeneration citing D15 — still live, and still the only place the `brew unpin`
tooltip text is specified — restores it. D15's *policy* (never upgradable,
excluded from every plan) is untouched by AD-16 and must survive; only the
mechanism clause conflicts.

**Remedy:** AD-16's inertness bullet cites `docs/DECISIONS.md` D15 as superseded
*on the mechanism* and still governing *on the policy and the tooltip content*.
Owner-side, D15 gains a supersession line pointing at `prd.md` FR-5.

### H-2 — AD-11 says the checklist carries the application accelerator map. It does not.

`ARCHITECTURE-SPINE.md:369-372` (AD-11):

> What the checklist *does* still carry is `⌘X`/`⌘C`/`⌘V`/`⌘A` **and the
> application accelerator map**, as a functional copy-paste concern under D25a
> rather than an accessibility check (`prd.md` RP-2).

`docs/RELEASE-CHECKLIST.md:97-100`, the entirety of step 9's action:

> 9. **Clipboard and menus still work.** ⌘X / ⌘C / ⌘V / ⌘A work in the package
>    search field and in every `CopyableCommand`. These die if the Edit and Window
>    submenus aren't re-declared, per `DECISIONS.md` D25a…

`grep -nE "⌘|Cmd" docs/RELEASE-CHECKLIST.md` returns one accelerator line, `:97`.
The map RP-2 enumerates (`prd.md:534`: `"**⌘R** … **⌘⇧R** … **⌘⇧U** … **⌘L** …
**⌘F** … **⌘1–9**"`) is not in the checklist and never was:
`git show 5c8996f -- docs/RELEASE-CHECKLIST.md` shows the removed step 9 read
`"**Keyboard and accessibility pass.** Tab and arrow navigation reach every
control. ⌘X / ⌘C / ⌘V / ⌘A work…"` — four clipboard keys then, four now.

Two things compound it. The Capability Map routes builders to AD-11 for exactly
this question — `:1301`: `"| Application menus and the accelerator map (RP-2) |
`app.set_menu` re-declaration + the global key handler | **AD-28** (which key owns
the native default), **AD-11 (what the checklist still carries)** |"`. And no AD
states the underlying invariant: `grep -nE "set_menu|Edit and Window"` over the
spine returns that Capability Map cell and nothing else, so the Edit/Window
re-declaration requirement D37 protects by name
(`docs/DECISIONS.md:543-546`: `"Per D25a these break silently if the Edit and
Window submenus are not re-declared — a functional regression in copy/paste, not
an accessibility check"`) exists in this spine only inside a false sentence.

The rule three lines above is what makes this material: `:357-359` — `"This rule
states a *claim boundary*, not a coverage inventory … Four consecutive revisions
restated the inventory and four were wrong"`. This is a fifth restated inventory
item, in the same AD, and it is wrong the same way.

**Remedy:** AD-11 says what the checklist actually carries — the four clipboard
accelerators, in the search field and in `CopyableCommand` — and states that
RP-2's wider map has **no** release-time step and no automated coverage, making
it story-owned product work under AD-1. If the owner wants it release-verified,
the hand-owned checklist gains the step; the spine must not claim it first.

---

## MEDIUM

### M-1 — `docs/SPEC.md` §4.11 is now named in the spine, but no ledger schedules it *(overtaken in part, mid-review)*

Raised as CRITICAL against `md5 503a26b3` (AD-28 then disposed of only
`docs/SPEC.md` F5). Between that read and `c2d62373` the spine gained the
disposition, so the spine-side half is **closed**. Recorded rather than deleted
because the remaining half is real and because the fix should not be silently
re-opened.

`docs/SPEC.md:288` (§4.11, still live, quoted for the four conflicts):

> Cmd+R refresh current manager (Dashboard: all) · Cmd+Shift+R refresh all ·
> **Cmd+U upgrade selected (opens sheet)** · … · **Esc clear selection** / close
> sheet / close drawer · … **Roving tabindex in tables; live region announces op
> completions**; all color states carry text/icon equivalents; text contrast
> ≥4.5:1 on its surface.

Now correctly dispositioned at `ARCHITECTURE-SPINE.md:1111-1116` (AD-28):

> `docs/SPEC.md` §4.11 is a second stale side alongside F5: it still lists
> "`Cmd+U` upgrade selected" and "`Esc` clear selection", both removed here, plus
> a roving tabindex and a completion-announcing live region that D37 removes. Its
> contrast floor survives. `prd.md` §0.1 catalogues SPEC's defects but does not
> reach §4.11, so this rule is where that is recorded.

What remains open: **no ledger schedules the SPEC edit.** `docs/DECISIONS.md:556`
lists `epics.md`, `ARCHITECTURE-SPINE.md` and `EXPERIENCE.md` as still carrying
removed obligations and names the workflow that clears each; `docs/SPEC.md` is
absent. `prd.md` §0.1's table (`prd.md:38-46`) lists §5.9, §F11, §1 P2, §0.1 and
§F1–F17 — not §4.11. SPEC is hand-written and workflow-unowned, so nothing will
clear it on its own, and `prd.md:45` diagnoses this exact mechanism as the cause
of the last live conflict: `"**F5 was never added to that list**, so SPEC's
transient-selection-plus-`Add N to Plan` model reads as current when D27
superseded it. This omission is the entire reason the FR-6 conflict stayed live"`.

**Remedy (owner, both hand-owned files):** add `docs/SPEC.md` §4.11 to D37's
*Not yet applied* list with "hand edit — no workflow owns SPEC", and add the
§4.11 row to `prd.md` §0.1's table. No further spine change needed.

### M-2 — The spine states two focus-site counts, and the smaller one is false at HEAD

`ARCHITECTURE-SPINE.md:1311` (*Canonical design-token set* row), present tense:

> `src/styles/theme.css` now reads `--color-bg-base: #090C13` … **All 22
> `focus-visible` sites resolve `--color-focus-ring`**

`ARCHITECTURE-SPINE.md:1343` (*Focus-indicator remediation* row):

> **31 sites, with zero `ring-focus-ring`**, zero `ring-offset-*`, and zero
> `outline-none` remaining … corroborated by the site count going from 22 to 31.

Measured: `grep -rn "focus-visible:" src/ | wc -l` → **31**. The 22 is D35's
pre-remediation population left in a present-tense sentence after `22ed41e` added
nine. Two rows of one document give two live numbers for one population.

**Remedy:** the design-token row's clause goes past tense and D35-scoped ("the 22
sites then existing"), or drops the count and defers to the remediation row.

### M-3 — AD-16 retires one immediate-execution call site; three exist *(fixed in flight at `d34e9190`)*

Raised against `c2d62373`; the live file already carries the fix at
`d34e9190`, AD-16 `:414-421` — `"**Three shipping call sites are retired by this
rule, not one:** the Package row action (`ManagerPane.upgradeRow` →
`executePlan`) **and both direct Manager self-update paths** … Scoping the work
to the row action alone would leave two unstaged mutation paths alive, which is a
breach of SM-2"`. Kept as the record of what was wrong and what closed it; no
action remains. The finding as raised:

`ARCHITECTURE-SPINE.md:404-405` (AD-16, at `c2d62373`):

> The shipping `ManagerPane.upgradeRow` → `executePlan` call site is retired by
> this rule, not preserved by it (`docs/DECISIONS.md` D27).

`prd.md:233` (FR-6), the requirements authority this spine is reconciled against:

> **Three immediate-execution call sites are in scope for removal**, not one: the
> Package row action, and *both* direct Manager self-update paths — the Dashboard
> Manager card and the Manager workspace self-update card each invoke the
> self-update command directly today, bypassing the plan entirely. **Scoping the
> D27 work to the row action alone would leave two unstaged mutation paths alive
> and breach SM-2.**

Verified at HEAD — `grep -rn "selfUpdateManager" src/` (excluding the client
wrapper): `src/components/dashboard/ManagerCard.tsx:128`
`void selfUpdateManager(info.id);` and
`src/components/manager/SelfUpdateCard.tsx:116`
`onClick={() => void selfUpdateManager(managerId)}`, alongside
`src/components/manager/ManagerPane.tsx:145-152`. `docs/RELEASE-CHECKLIST.md:53-56`
independently names two of the three as today's deliberate bypasses. AD-16's
generic clause covers them in principle; the one sentence naming *shipping code*
names one, and FR-6 says in terms that doing exactly that breaches SM-2.

**Remedy:** AD-16's rule names all three shipping call sites by path, or cites
`prd.md` FR-6's three-site sentence rather than naming one.

### M-4 — D37's own "Not yet applied" paragraph is stale about the spine, and its counts were never measured

`docs/DECISIONS.md:556-559`:

> **Not yet applied:** `epics.md` (10 mentions, including FR-19, NFR-6, and Story
> UX-PB.1d), **`ARCHITECTURE-SPINE.md` (3)**, and `EXPERIENCE.md` (4) still carry
> the removed obligations. Those are workflow-owned and come out through
> `bmad-correct-course`, a `bmad-architecture` Update, and a `bmad-ux` Update
> respectively — never a hand edit.

Revision 10 **is** that `bmad-architecture` Update and it discharged the spine's
limb (see the clean-verification table). The counts were never measurements:
`grep -nEic "voiceover|screen[- ]reader|keyboard|aria|announce|live region|assistive|tab (order|and arrow)"`
→ `EXPERIENCE.md` **24** matching lines (D37 says 4), `epics.md` **29** (D37 says
10), rev-9 spine **29** (D37 says 3). The spine already flags this — `:1342`
residual (9): `"the counts in `docs/DECISIONS.md` D37 were copied rather than
measured and are wrong"` — but the decision record still asserts them, and it is
what a future regeneration reads first.

**Remedy (owner; `docs/DECISIONS.md` is the hand-written record, so a hand edit
is correct here):** strike the parenthetical counts, record that the
`ARCHITECTURE-SPINE.md` limb was discharged by revision 10, and leave `epics.md`,
`EXPERIENCE.md` and (per M-1) `docs/SPEC.md` §4.11 outstanding.

### M-5 — `prd.md` still says the contrast guard is uncommitted; D36 landed it

Adjacent to this lens (currency owns `prd.md`), recorded because it contradicts
D36 at HEAD and two spine rules. `prd.md:628`:

> **Not in this list, deliberately:** the automated contrast guard. The 4.5:1
> assertion and the on-fill ink tokens that make it pass are **uncommitted
> working-tree changes**, absent from `HEAD` `5972109`. Until they land, contrast
> at release time is a by-eye check, and neither FR-19 nor NFR-6 may be read as
> CI-guaranteed on that axis.

`prd.md:447` (FR-19): `"Text contrast meets at least 4.5:1 on its surface. **Not
met at `HEAD`** — see NFR-6."`

Both are false at `HEAD = 1ac959e`: `a201fb0` is an ancestor, `text-white` in
`src/` is 0, and the assertion is `tests/e2e/browser-style-contract.spec.ts:320`.
The spine is the correct side (AD-11 `:361-362`, AD-27 `:1012-1015`), and
`docs/RELEASE-CHECKLIST.md:120-121` already says `"No by-eye contrast sweep is
required at release."` A builder reading FR-19 today is told the opposite of what
AD-27 tells them.

**Remedy:** owner refreshes FR-19's contrast limb and §6's "not in this list"
paragraph against `HEAD` `1ac959e`. No spine change.

### M-6 — `RELEASE-CHECKLIST.md` step 5 describes HEAD in vocabulary the spine says does not exist yet

`docs/RELEASE-CHECKLIST.md:49-51`:

> 5. **The bulk paths do not execute without explicit confirmation.** Row
>    checkboxes, Manager headers, and Update Everything all **stage into the draft
>    plan** and reach the confirmation gate — verify each still does.

`ARCHITECTURE-SPINE.md:161-165` (*Verified Brownfield Baseline*):

> The Upgrade Plan is currently transient dialog state (`ui.dialog`
> `{ kind: "upgradePlan" }`, discarded by `closeDialog`), a single-package row
> action executes immediately … and no `planAttemptId`, `Verifying`, or
> `InteractionRequired` symbol exists in `src/` or `src-tauri/src/`.

There is no draft plan at HEAD to stage into: a row checkbox writes `selection`
in `src/store/packages.ts`, and nothing reaches a plan until a sheet is opened
(`src/hooks/useKeyboard.ts:93-104`, `upgradeSelected` → `openPlan` →
`openDialog({ kind: "upgradePlan" })`). The step contradicts itself seven lines
later — `:58` `"D27–D30 routes them through the plan too, but Epic UX-PB is
unbuilt, so that is target state"` — inside the document AD-11 makes the release
authority.

**Remedy:** owner restates step 5 in shipping vocabulary (the plan *sheet*), or
tags the "draft plan" phrasing as target state the way the rest of the step does.
No spine change.

---

## LOW

### L-1 — AD-16 cites the `EXPERIENCE.md` Checkbox contract without the staleness qualifier it uses elsewhere

`ARCHITECTURE-SPINE.md:571` cites `"(`prd.md` FR-5, `EXPERIENCE.md` Checkbox
contract)"`. That contract, at
`_bmad-output/planning-artifacts/ux-designs/ux-Pack-Manager-2026-07-23/EXPERIENCE.md:143`,
carries D37-removed obligations in the sentence being borrowed from: `"it
**announces the exact count** and uses `mixed` when only some are staged. An
explanatory-disabled control never uses native `disabled`: expose
`aria-disabled=\"true\"`, attach its persistent reason as an accessible
description…"`. The spine warns globally at `:1342`
(`"`epics.md` and `EXPERIENCE.md` still carry the removed obligations"`), so a
thorough reader is covered; one following the inline pointer is not.

**Remedy:** append "— its native-`disabled` clause only; the announcement limbs
are D37-removed and await a `bmad-ux` Update" to that citation.

### L-2 — D36's heading over-claims: `--color-on-success` still has zero consumers

`docs/DECISIONS.md:488` — `"## D36. Bright fills use the palette's dark ink;
D35's on-fill tokens get consumers"`. At HEAD `--color-on-success` is matched only
by its own definition (`src/styles/theme.css:32`); all three fixed sites use
`text-on-accent`. The spine is accurate — AD-27 `:1009` names only
`--color-accent`, `--color-accent-hover` and `--color-danger` — so this is a
defect in D36's title, not in the spine.

**Remedy:** none for the spine. Owner either points something at
`--color-on-success` or narrows D36's title to the accent tokens.

### L-3 — AD-27 names its sample in target-state terms

`ARCHITECTURE-SPINE.md:1002-1004` — `"today a toolbar `<button>` and the
package-row **plan-membership** checkbox"`. At HEAD that checkbox drives the
transient `selection` set in `src/store/packages.ts`, which AD-28 abolishes; it
becomes membership only after UX-PB.1a. "Today" sits in front of a name that is
true tomorrow. The sample *choice* is right — it is the `appearance`-discriminator
control the WebKit rule needs, and
`tests/e2e/browser-style-contract.spec.ts:186-197` confirms it is keyboard-driven
(`"a pointer click sets :focus but not :focus-visible"`).

**Remedy:** "the package-row checkbox (membership under AD-28; the `selection`
toggle today)".

---

## Method

- Every quote was read from the file in this session. Nothing is paraphrased from
  a prior review, a summary, or memory.
- Counts came from named commands, reproduced at each use: `grep -rn … | wc -l`,
  `grep -nEic …`, `grep -oE … | sort | uniq -c`, `wc -l`, `git show`,
  `git log --oneline -1 <sha>`.
- The revision-9 comparison used `git show HEAD:…/ARCHITECTURE-SPINE.md`, so "what
  D37 removed" is measured against the committed predecessor rather than a
  description of it.
- D37 was scoped **by named heading** — AD-11, AD-16, AD-17, AD-27, AD-28, the
  Styling convention, the Capability Map, and the two Decision-Status rows. Keyword
  greps were used only to locate candidate sites, never to conclude anything, per
  D37's own instruction and AD-27 `:971`.
- Nothing in the target was edited. This review is the only file written.
