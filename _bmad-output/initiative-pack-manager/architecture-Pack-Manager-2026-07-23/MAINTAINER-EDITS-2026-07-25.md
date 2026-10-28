# Maintainer edits — 2026-07-25

Ready-to-apply drafts for six edits to files **no BMAD workflow owns**:
`docs/DECISIONS.md`, `docs/RELEASE-CHECKLIST.md`, `docs/SPEC.md`.

**Edits 1–5** came from `ARCHITECTURE-SPINE.md` revision 10's Open row. **Edit 6** was added
later, on 2026-07-25, from the `bmad-prd` Reviewer Gate.

> **Before applying Edit 4, re-read its quoted `prd.md` text.** It quotes RP-2 as
> "**⌘L** (toggle the activity surface)" and proposes a checklist line carrying the same
> phrase. `prd.md` has since been corrected: RP-2 now reads "**⌘L** (move focus into the
> Upgrade Plan sidecar region)" and RP-2's status moved from Shipping to Partial. Edit 4's
> `prd.md` line anchors have also shifted. Applying it verbatim would write the retiring
> `ActivityDrawer` behaviour into the release checklist.

## Status: all six applied 2026-07-25 (maintainer)

Edits 1, 2, 3, 5a, 5b, and 6 were applied as drafted.

**Edit 4 was applied with an amendment, and the amendment is the point.** The warning
above is correct that the drafted text is wrong, but substituting AD-17's wording would
have been wrong too. `docs/RELEASE-CHECKLIST.md` verifies the **shipping** build, and at
`HEAD` ⌘L still calls `toggleDrawer()` (`src/hooks/useKeyboard.ts:166`). AD-17's focus-move
is target state and Epic UX-PB is unbuilt. The applied 9a therefore tells the verifier to
check the drawer toggle, names AD-17's replacement behaviour, and says explicitly not to
report the difference as a failure — the same shipping-vs-target split step 5 already uses.
Writing either behaviour alone would have produced a step that fails every release or gets
waved through.

`docs/DECISIONS.md` **D38** was written and appended in the same pass; the exclusion note
at the end of this file records why it could not be drafted here.

Source: `ARCHITECTURE-SPINE.md` revision 10, Open row *"Maintainer edits this
spine cannot make"*. Every "Current text" block below was re-read against the
file at `HEAD` while writing this, and every count was measured with the command
shown — not copied.

Nothing here has been applied. This file is the only thing written.

---

## Edit 1 — `docs/DECISIONS.md` D37: the "Not yet applied" counts are wrong, and the spine's limb is discharged

**Current text**

`docs/DECISIONS.md:556-560`

```
**Not yet applied:** `epics.md` (10 mentions, including FR-19, NFR-6, and Story
UX-PB.1d), `ARCHITECTURE-SPINE.md` (3), and `EXPERIENCE.md` (4) still carry the
removed obligations. Those are workflow-owned and come out through
`bmad-correct-course`, a `bmad-architecture` Update, and a `bmad-ux` Update
respectively — never a hand edit.
```

**Measurement.** Run from the repo root:

```
grep -ciE "voiceover|keyboard" _bmad-output/planning-artifacts/epics.md \
  _bmad-output/planning-artifacts/architecture/architecture-Pack-Manager-2026-07-23/ARCHITECTURE-SPINE.md \
  _bmad-output/planning-artifacts/ux-designs/ux-Pack-Manager-2026-07-23/EXPERIENCE.md
```

Result on 2026-07-25: `epics.md` **13** lines (21 raw occurrences),
`ARCHITECTURE-SPINE.md` **14** (32 raw), `EXPERIENCE.md` **11** (13 raw).
D37 records 10 / 3 / 4. All three are wrong.

**Replacement text** (for `:556-560`)

```
**Not yet applied:** `epics.md` and `EXPERIENCE.md` still carry the removed
obligations. In `epics.md` they are FR-19, NFR-6, Story UX-PB.1d, Story
UX-PB.5d, UX-PB.3a's plan-start announcement, and the DR-2 restatement's claim
that a manual VoiceOver pass joins the release checklist. In `EXPERIENCE.md`
they are the `Keyboard` and `Package Grid keyboard model` sections and the
`Accessibility Floor`. Those come out through `bmad-correct-course` and a
`bmad-ux` Update respectively — never a hand edit.

`ARCHITECTURE-SPINE.md` is **done**: revision 10 (2026-07-25) is the
`bmad-architecture` Update named here, and it applied D37 through AD-11, AD-16,
AD-17 and AD-27.

**Scope those runs by the named sections above, never by a mention count.**
A count is measurable — `grep -ciE "voiceover|keyboard" <file>` — but it is not
the scope: applying D37 means *recording* what was retired, and the recording
uses the same words. The spine's own count rose from 8 to 14 while it was
applying D37 correctly.
```

**Why.** The counts were copied rather than measured and are wrong in all three
columns; a run that stops after fixing *n* mentions leaves the file half
converted. Naming the sections survives the drift that a count cannot.

---

## Edit 2 — `docs/DECISIONS.md` D36: the title over-claims

**Current text**

`docs/DECISIONS.md:488`

```
## D36. Bright fills use the palette's dark ink; D35's on-fill tokens get consumers
```

**Verification.** `grep -rn "on-success" src/ tests/` returns exactly two lines,
neither a consumer: `src/styles/theme.css:32` (the definition itself) and
`tests/e2e/browser-style-contract.spec.ts:317` (a `//` comment). By contrast
`grep -rn "on-accent" src/` shows three real consumers —
`src/components/primitives/Button.tsx:7` (primary), `:13` (danger), and
`src/components/shell/UpdateStatusItem.tsx:63`.

**Replacement title**

```
## D36. Bright fills use the palette's dark ink; `--color-on-accent` gets consumers
```

**Plus one line**, appended to the paragraph ending `docs/DECISIONS.md:506`
("…not a blocker for the fix."):

```
`--color-on-success` remains consumer-less: no bright success fill carries text
today, so D35's second token is still defined and unused.
```

**Why.** "Tokens" plural reads as both tokens getting consumers; only
`--color-on-accent` did. The added line keeps the unused token visible so a
later reader does not treat its absence as a regression from D36.

---

## Edit 3 — `docs/RELEASE-CHECKLIST.md` step 5: describes D27 target state as if verifiable today

**Current text**

`docs/RELEASE-CHECKLIST.md:49-51`

```
5. **The bulk paths do not execute without explicit confirmation.** Row checkboxes,
   Manager headers, and Update Everything all stage into the draft plan and reach the
   confirmation gate — verify each still does.
```

**Verification, both halves.** A row checkbox writes a transient selection set,
not a plan: `src/store/packages.ts:17` declares
`selection: Partial<Record<ManagerId, Set<string>>>;`, and the store's header
comment at `:5` reads "The store keeps *primitive* selection ops (toggle, range,
set, clear)." A row action executes immediately:
`src/components/manager/ManagerPane.tsx:145` `async function upgradeRow(pkg: Package) {`,
`:146` `// Single-package plan executes immediately — no sheet (SPEC §F5).`,
`:152` `await executePlan(plan);`. There is no draft plan at `HEAD`.

**Replacement text** (for `:49-51`; the existing "Two paths deliberately
**bypass** the gate today" paragraph at `:53-58` stays unchanged)

```
5. **The bulk paths do not execute without explicit confirmation.** Today: a row
   checkbox builds a transient selection (`src/store/packages.ts:17`, the
   `selection` set) and `Add N to Plan`, a Manager header action, and
   `Update Everything` each open `UpgradePlanSheet` with the exact commands
   before anything runs — verify each still reaches that sheet, and that
   dismissing it runs nothing.

   **Target state, not checkable yet:** D27–D30 replace this with a persistent
   draft plan where the checkbox *is* membership (`ARCHITECTURE-SPINE.md` AD-28)
   and a separate confirmation dialog is the gate. When Epic UX-PB ships, this
   step becomes "row checkboxes, Manager headers and Update Everything all stage
   into the draft plan and reach the confirmation gate."
```

**Why.** As written the step asks the verifier to confirm behavior that does not
exist, so it either fails every release or gets waved through — and a waved-
through step stops guarding the thing it does cover.

---

## Edit 4 — `docs/RELEASE-CHECKLIST.md`: RP-2's accelerator half has no validation step

**Verification.** `prd.md:515` — "They are validated through
`docs/RELEASE-CHECKLIST.md`." `prd.md:534` enumerates the accelerators:
"**⌘R** (refresh current, or all from the Dashboard), **⌘⇧R** (refresh all),
**⌘⇧U** (Update Everything), **⌘L** (toggle the activity surface), **⌘F** (focus
search), and **⌘1–9** (navigation jump)."

`grep -ncE "⌘R|⌘⇧R|⌘⇧U|⌘L|⌘F|⌘1|Cmd\+R|Cmd\+Shift" docs/RELEASE-CHECKLIST.md`
returns **0**. The file's only `⌘` line is `:97`, which covers ⌘X / ⌘C / ⌘V / ⌘A
alone. RP-2's second half is validated nowhere.

**Addition** — a new sub-item under step 9, immediately after the `:97-100`
paragraph and before the "Keyboard navigation and screen-reader support" note:

```
   - **9a. The application accelerators still fire.** *(~60 s)* ⌘R (refresh
     current, or all from the Dashboard), ⌘⇧R (refresh all), ⌘⇧U (Update
     Everything), ⌘L (toggle the activity surface), ⌘F (focus search), and
     ⌘1–9 (navigation jump). Same failure mode as ⌘X/⌘C/⌘V/⌘A above:
     `app.set_menu` replaces Tauri's default menu wholesale (`DECISIONS.md`
     D25a), so an accelerator dropped from the re-declaration dies silently with
     no error anywhere. `prd.md` §4.6 makes this checklist RP-2's validation
     route, and RP-2 enumerates exactly these. This is a functional regression
     check on shipped shortcuts, **not** an accessibility pass — D37 retired
     those and did not retire this.
```

**Why.** `prd.md` §4.6 routes RP-2's validation here and the checklist validates
only its Edit-menu half; the accelerators can be lost in a menu refactor with no
signal. Framing it as functional regression matters because D37 retired the
keyboard *accessibility* criteria, and an accessibility-framed step would be
deleted on sight by the next reader applying D37.

---

## Edit 5 — `docs/SPEC.md`: two omissions, both recording retirement

`prd.md:34` — "**It is no longer the requirements authority.**" Both edits
therefore *record that something is retired*. Neither adds a requirement.

### 5a — §0.1's supersession list never named F5

**Current text**

`docs/SPEC.md:43-46`

```
- Existing immediate-row, direct self-update, Operation-row History,
  Activity-drawer-only, global self-update toggle, and `autoOpenDrawer`
  descriptions below are historical implementation detail, not target
  behavior.
```

F5 is at `docs/SPEC.md:78-81` and still describes the superseded model —
`:80` reads "`Add N to Plan` immediately adds the checked canonical identities
to the persistent plan and clears the transient selection."

**Addition** — one new bullet at the end of §0.1's list, after `:46`:

```
- **F5's transient-selection-plus-`Add N to Plan` model is superseded.** A
  Package checkbox *is* Upgrade Plan membership; there is no separate selection
  set and no submit step (`ARCHITECTURE-SPINE.md` AD-28, `docs/DECISIONS.md`
  D27). F5's row `Update Package` action and its "Neither action executes" line
  remain correct as target behavior; only the two-step selection model is
  historical.
```

**Why.** `prd.md:45` records this omission as "the entire reason the FR-6
conflict stayed live" — it is the sole mechanical reason the membership conflict
survived into an architecture Open row. One bullet closes it.

### 5b — §4.11's keyboard map carries three retired limbs and one survivor

**Current text**

`docs/SPEC.md:288` (§4.11 heading at `:286`)

```
Cmd+R refresh current manager (Dashboard: all) · Cmd+Shift+R refresh all · Cmd+U upgrade selected (opens sheet) · Cmd+Shift+U Update Everything (sheet) · Cmd+A select all visible selectable rows · Space toggle focused row · Esc clear selection / close sheet / close drawer · Cmd+L toggle drawer · Cmd+F focus search · Cmd+1..9 sidebar jump. Roving tabindex in tables; live region announces op completions; all color states carry text/icon equivalents; text contrast ≥4.5:1 on its surface.
```

**Addition** — a superseded marker inserted after `:288`. The line itself is
left intact.

```
> **Superseded in part (2026-07-25).** Retired: `Cmd+U upgrade selected` and the
> `Esc` *clear selection* rung — `ARCHITECTURE-SPINE.md` AD-28 removes both
> (`Esc` closes a dialog and nothing else; membership is never cleared by a
> keystroke). Also retired: the roving tabindex and the completion-announcing
> live region — `docs/DECISIONS.md` D37 removes keyboard navigation and
> screen-reader support as obligations. **Surviving from this line:** the ≥4.5:1
> text contrast floor, now CI-asserted (D36), and the non-color status cues. The
> surviving accelerator map is `prd.md` RP-2: ⌘R, ⌘⇧R, ⌘⇧U, ⌘L, ⌘F, ⌘1–9, plus
> ⌘A as an Edit-menu action.
```

**Why.** A marker, not a rewrite: rewriting the line would re-state
requirements in a file `prd.md` §0.1 has demoted. `prd.md` §0.1 catalogues
SPEC's defects but does not reach §4.11, so nothing else in the tree records
this.

---

## Edit 6 — `docs/RELEASE-CHECKLIST.md` step 5: the bypass list undercounts by one

*Added 2026-07-25 from the `bmad-prd` Reviewer Gate. Verified against `HEAD` `1ac959e` while writing.*

**Current text**

`docs/RELEASE-CHECKLIST.md:53-58`

```
   Two paths deliberately **bypass** the gate today and must not be reported as failures:
   a row's own `Update Package` action (`src/components/manager/ManagerPane.tsx:145`,
   commented "Single-package plan executes immediately — no sheet (SPEC §F5)") and a
   Manager's self-update button (`src/components/manager/SelfUpdateCard.tsx:116`). Both
   run one known command against one named target. D27–D30 routes them through the plan
   too, but Epic UX-PB is unbuilt, so that is target state — not something to check here.
```

**Verification.** There is a third. `src/components/manager/HealthBanner.tsx:43` renders a
`Run fix` button whose handler is `onClick={() => void runHealthFix(managerId, issue.id)}` —
immediate execution, no staging, same shape as the other two. It is a real Operation:
`src-tauri/src/ipc.rs:93` declares `HealthFix`, `src-tauri/src/queue.rs:374` uses
`CmdPurpose::HealthFix`, and `src-tauri/src/lib.rs:240` registers `commands::run_health_fix`.

**Replacement text** (for `:53-58`)

```
   Three paths deliberately **bypass** the gate today and must not be reported as failures:
   a row's own `Update Package` action (`src/components/manager/ManagerPane.tsx:145`,
   commented "Single-package plan executes immediately — no sheet (SPEC §F5)"), a
   Manager's self-update button (`src/components/manager/SelfUpdateCard.tsx:116`), and a
   Health issue's `Run fix` button (`src/components/manager/HealthBanner.tsx:43`). All three
   run one known command against one named target, and `Run fix` additionally requires the
   Manager's suggestion to be byte-equal to the product's own canonical command before the
   button exists at all (`prd.md` FR-23). D27–D30 routes all three through the plan, but
   Epic UX-PB is unbuilt, so that is target state — not something to check here.
```

**Why.** A verifier reading "two paths" and finding a third executing button has to decide
on the spot whether it is the defect step 5 exists to catch. Naming it — and naming the gate
that makes it safe — is the difference between a known-safe bypass and an unexplained one.
`prd.md` FR-23 now states this requirement PRD-side; this edit is the checklist half.

---

## What is deliberately NOT proposed

- **No requirement is added to `docs/SPEC.md`.** Both SPEC edits record
  retirement only. `prd.md:34` makes SPEC non-authoritative for requirements;
  adding one here would recreate the two-authority problem `prd.md` §0.1 exists
  to close.
- **`docs/DECISIONS.md` D15's supersession is excluded.** D15's mechanism clause
  at `docs/DECISIONS.md:63` — "Disabled checkbox + tooltip with the `brew unpin`
  command" — is overridden by AD-16's inertness rule and nothing records the
  supersession. Only a decision can supersede a decision, and the owner is
  drafting that themselves as **D38**. It is named here so it is not mistaken
  for an oversight.
