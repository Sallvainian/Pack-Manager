# Reconciliation: `docs/SPEC.md` → PRD (2026-07-25)

**Reviewer lens:** `docs/SPEC.md`
**Reviewed:** `prd.md` (660 lines), `addendum.md` (70 lines), `docs/SPEC.md` (815 lines,
`wc -l < docs/SPEC.md` → `815`), against `src/` and `src-tauri/src/` at `HEAD`.

## Method and scope

Two questions only:

1. **Is `prd.md` §0.1's SPEC-defect table accurate and complete?** Each row was checked
   against both SPEC.md and the shipping code.
2. **What product substance in SPEC.md did the PRD drop that it should not have?**
   Technical mechanism listed in `addendum.md` §1 (PATH construction, ownership
   algorithm, adapter traits, parser regexes, scheduler internals, IPC wire shapes,
   transcript syntax, test seams and names, updater transport) is **not** treated as a
   gap. Neither is anything D33 retired or D37 removed.

Only user-visible behavior, product rules, error semantics, exclusion reasons, and
acceptance conditions are reported.

---

## 1. §0.1 defect table — verification

**All seven rows verified accurate. No row is wrong, overstated, or unsupported.**

| # | PRD claim (`prd.md:40–46`) | Verdict | Evidence |
| --- | --- | --- | --- |
| 1 | SPEC §5.9 lists 17 commands; `lib.rs` registers 20 | **ACCURATE** | `docs/SPEC.md:468–484` lists exactly 17 signatures. `src-tauri/src/lib.rs:232–253` registers 20, the extra three being `commands::get_app_update_state` (`:250`), `commands::check_for_app_update` (`:251`), `commands::install_app_update` (`:252`). |
| 2 | SPEC §5.9 lists 5 events; `events.rs` defines 6 | **ACCURATE** | SPEC table rows `docs/SPEC.md:646–650` = 5. `src-tauri/src/events.rs:77–82` defines 6 constants, the sixth `src-tauri/src/events.rs:82` `pub const EVENT_APP_UPDATE_STATUS: &str = "appUpdate:status";`. |
| 3 | §F11 and §5.9 `Settings` list 7 fields each, disagree with each other, and `Settings` ships 8 with `autoCheckForUpdates` in neither | **ACCURATE** | `docs/SPEC.md:112` names `runBrewUpdateOnRefresh`…`skipUpgradePlanConfirmation` (7, includes `skipUpgradePlanConfirmation`, omits `autoOpenDrawer` as active). `docs/SPEC.md:631–639` `interface Settings` has 7 (includes `autoOpenDrawer`, omits `skipUpgradePlanConfirmation`). `src-tauri/src/settings.rs:28–39` has 8, ending `src-tauri/src/settings.rs:38` `pub auto_check_for_updates: bool,`; `src-tauri/src/settings.rs:51` `auto_check_for_updates: true,`. `grep -c autoCheckForUpdates docs/SPEC.md` → 0. |
| 4 | §F11 states `skipUpgradePlanConfirmation` as current; zero occurrences in `src-tauri/src/` | **ACCURATE** | `docs/SPEC.md:112` `…and \`skipUpgradePlanConfirmation\` (default false)`. `grep -rn "skipUpgradePlanConfirmation\|skip_upgrade_plan_confirmation" src/ src-tauri/src/ \| wc -l` → `0`. |
| 5 | §1 P2 (line 128) says `notarized DMG` out of scope; line 108 says signed/notarized/stapled | **ACCURATE** | `docs/SPEC.md:128` `Light theme; menu-bar extra; scheduled refresh; cross-manager dedup beyond the rust rule; cargo-install support; notarized DMG.` vs `docs/SPEC.md:108` `Release delivery is signed, notarized, and stapled as required by D25/D25a; stale ad-hoc-only language is superseded.` |
| 6 | §0.1's supersession list never added F5 | **ACCURATE** | `docs/SPEC.md:43–46` `Existing immediate-row, direct self-update, Operation-row History, / Activity-drawer-only, global self-update toggle, and \`autoOpenDrawer\` / descriptions below are historical implementation detail, not target / behavior.` F5 is absent, and `docs/SPEC.md:80` still reads `\`Add N to Plan\` immediately adds the checked canonical identities to the persistent plan and clears the transient selection.` |
| 7 | 9 of 17 features carry no acceptance criterion, including four P0s | **ACCURATE** | `grep -c '^\*\*Acceptance:\*\*' docs/SPEC.md` → `8`, at lines 57, 62, 67, 76, 81, 88, 95, 100 — i.e. F1–F8 only. F9–F17 (9 features) have none; of those, F9/F10/F11/F12 are tagged `(P0)` at `docs/SPEC.md:102,106,110,114`. |

The follow-on paragraph at `prd.md:48` is also accurate: `docs/SPEC.md:19` states invariant 7
(`The durable \`planAttemptId\` correlates reviewed intent, Operations, events, transcripts,
journal records, verification, Results, and Retry lineage.`) in present tense under
`docs/SPEC.md:11` `### Load-bearing invariants (violations are bugs)`, and it sits above
§0.1 at `docs/SPEC.md:21`. `grep -rn "planAttemptId\|plan_attempt_id" src/ src-tauri/src/ | wc -l`
→ `0`.

The PRD's FR-6 supporting claim also checks out: `prd.md:230` says
`Today \`ManagerPane.upgradeRow\` builds a single-Package plan and calls \`executePlan\`
immediately`. Code agrees — `src/components/manager/ManagerPane.tsx:145–153`, whose comment
reads `// Single-package plan executes immediately — no sheet (SPEC §F5).`

**Also verified as *not* drifted** (so nobody adds them to §0.1 later): SPEC's `ErrorCode`
list at `docs/SPEC.md:672` matches `src/lib/ipc/types.ts:68–81` exactly (12 codes); SPEC's
`OpStatus` at `docs/SPEC.md:501–508` matches `src-tauri/src/ipc.rs:99–107` exactly (7
variants); SPEC's `AppState`, `UpgradePlan`, and `PlanRequest` shapes at
`docs/SPEC.md:625–630, 578–596, 573–577` match `src/lib/ipc/types.ts:261–266, 204–212,
177–182`; SPEC §7.6 CI runners at `docs/SPEC.md:805` match `.github/workflows/ci.yml:28,52,70`.

---

## 2. The defect §0.1 missed

### D-12: SPEC §1 has no feature for Pack-Manager's own application update

Rows 1 and 2 of §0.1 record the *symptom* — three missing IPC commands and one missing
event — but not the cause. The entire application-update product area has **no feature
entry, no priority tag, and no acceptance criterion anywhere in SPEC §1**.

`grep -n "^### F" docs/SPEC.md` returns F1…F12 at lines 54–114 plus F13–F17 at 120–124.
None concerns updating Pack-Manager itself.
`grep -Ein 'updater|app update|application update|appUpdate|restart to update' docs/SPEC.md`
returns 4 hits; three (`:384`, `:528`, `:531`) are `SelfUpdateRoute`, i.e. *Manager*
self-update. The only genuine hit is `docs/SPEC.md:815`, and it sits inside §8, whose
opening words at `docs/SPEC.md:811` are `**Superseded for MVP.**`.

Why this matters more than the IPC-table rows: `prd.md:34` states that SPEC
`remains authoritative for UI specification, architecture detail, parser contracts, and the
test plan.` For the whole of PRD §4.5 (FR-20, FR-21, FR-22) and RP-1 — a shipping area
backed by `src-tauri/src/app_update.rs`, `src/store/appUpdate.ts`, three IPC commands, one
event, and the `autoCheckForUpdates` setting — that delegation points at nothing. A reader
who follows `prd.md:34` for UI or test detail on the update flow finds a superseded
packaging section. §0.1 should record the feature-level absence, not just the two table
diffs downstream of it.

---

## 3. Product substance the PRD dropped

### 3.1 Expected non-zero exits (HIGH)

`grep -ic exit prd.md addendum.md` → `0` and `0`. `grep -ic "non-zero" prd.md addendum.md`
→ `0` and `0`. The rule is entirely absent from the requirements authority.

SPEC states it three times, in three registers:

- `docs/SPEC.md:138` — `Verified command quirks: … \`npm outdated -g --json\` exits 1 when outdated`
- `docs/SPEC.md:399` — `\`npm outdated -g --json\` (120s; **exit 1 with parseable JSON object = success**; same rule for \`ls\`)`
- `docs/SPEC.md:672` — `Rules: \`ExpectedNonZero\` never becomes \`NonZeroExit\`.`

It ships and is load-bearing: `src-tauri/src/managers/npm.rs:187`
`Some(1) if cmd.argv.iter().any(|a| a == "--json") => ExitClass::ExpectedNonZero,`, consumed
at `src-tauri/src/queue.rs:1596`
`ExitClass::Success | ExitClass::ExpectedNonZero => outputs.push(out),`. `brew` uses it too
(`src-tauri/src/managers/brew.rs:304, 628`).

This is not parser mechanism — `addendum.md:15` excludes `parser regexes`, and this is not
one. It is an **error-classification product rule with a visible failure mode**: on this
machine `npm outdated -g --json` exits 1 whenever anything is outdated, so an implementer
working from the PRD alone, wiring exit≠0 → failure, produces an npm error card on every
refresh that finds an update. The nearest PRD text points the *other* way — `prd.md:160`
`Output a parser cannot handle fails that Manager visibly, with an excerpt` — which is
`ParseFailed`, a different rule.

Note also that `addendum.md`'s exclusion table (`addendum.md:11–20`) has **no row for SPEC
§5.10 (error taxonomy)** at all. §5.10 is neither delegated nor absorbed; it simply fell
between the two documents.

**Fix:** add to FR-2 or FR-16: *"A Manager exit code that the Manager uses to signal a
normal condition is not a failure. Per-Manager expected-non-zero classification is
consulted before any exit code becomes a user-facing error; a usable payload with a
non-zero exit succeeds, and an unusable one fails as a parse failure, never as a
non-zero-exit failure."*

### 3.2 The `alreadyRunning` exclusion reason (MEDIUM)

The PRD enumerates three exclusion reasons — pinned (`prd.md:198`), self-updating casks
(`prd.md:199`), and the Rust dedup rule (`prd.md:203`). SPEC has four:
`docs/SPEC.md:592` `    reason: "pinned" | "greedyCask" | "rustDedup" | "alreadyRunning";`

The fourth ships and is user-visible copy:
`src-tauri/src/queue.rs:450` `/// \`alreadyRunning\` exclusions source: package ids of queued/running upgrades.`,
`src-tauri/src/queue.rs:539` `reason: ExcludeReason::AlreadyRunning,`,
`src/components/dialogs/UpgradePlanSheet.tsx:31` `alreadyRunning: "already running",`
rendered in the excluded list at `src/components/dialogs/UpgradePlanSheet.tsx:273`.
`grep -rn "already running\|alreadyRunning"` over the PRD directory → no matches.

This is a distinct behavior from anything the PRD states, and the difference is
behavioral, not cosmetic. `prd.md:261` describes *rejection*: `a lock-set overlap with any
pending or running Upgrade, SelfUpdate, or HealthFix rejects the submission without
enqueueing.` `alreadyRunning` is *exclusion at plan-build time*: the already-running
Package is dropped from the plan with a visible reason and the rest proceeds. Under the
PRD as written, `Update Everything` while one Package is upgrading would be read as
"reject the whole submission." Under shipping behavior it builds a plan minus that one
Package. FR-9's all-or-none guarantee (`prd.md:272`) then applies to what survived
exclusion — an important qualification the PRD does not make.

**Fix:** add to FR-5's eligibility list and to FR-9: *"A Package already inside a queued or
running Upgrade is excluded from a new Upgrade Plan with the reason stated, rather than
blocking the plan. Atomic admission applies to the plan that remains after exclusions."*

### 3.3 §0.1 delegates UI-specification authority to SPEC §4 without scoping it (MEDIUM)

`prd.md:34` — `It remains valuable and remains authoritative for UI specification,
architecture detail, parser contracts, and the test plan.` — in the same paragraph that
declares SPEC `materially out of date` (`prd.md:36`). The delegation is unscoped, and SPEC
§4 contains two classes of superseded content that SPEC's **own** supersession list
(`docs/SPEC.md:43–46`) does not fence:

1. **Pre-D27 transient selection UI.** `docs/SPEC.md:272`
   `Row states: hover \`bg-raised\`; selected \`accent-subtle\` wash;` plus the
   `SelectionToolbar` in the component tree (`docs/SPEC.md:252`), `Upgrade selected (n)` in
   the toolbar (`docs/SPEC.md:271`), and `docs/SPEC.md:288` `Esc clear selection`. This is
   the same defect §0.1 row 6 records for F5, but it recurs in §4.4, §4.8, §4.11, and
   §5.11 (`docs/SPEC.md:677` `per-manager selection \`Set<packageId>\` + shift-anchor`).
   §0.1 row 6 names only F5, so a reader who fixes F5 still finds four unfenced copies.

2. **The D37-removed criteria.** `docs/SPEC.md:288` ends
   `Roving tabindex in tables; live region announces op completions; all color states carry
   text/icon equivalents; text contrast ≥4.5:1 on its surface.` `prd.md:440` lists the
   carriers as `\`epics.md\`, \`ARCHITECTURE-SPINE.md\`, and \`EXPERIENCE.md\` … (10, 3, and 4
   mentions respectively)` — mirroring `docs/DECISIONS.md:554–556`, which lists the same
   three. `docs/SPEC.md` is a fourth carrier and appears in neither list, while being the
   one document the PRD names authoritative for UI. `addendum.md:56` recommends exactly two
   SPEC edits (`a header pointing requirements authority at this PRD, and **adding F5 to the
   §0.1 supersession list**`); neither covers this.

   *Honest qualification:* `docs/DECISIONS.md:562–564` rejects deleting the shipped focus and
   ARIA affordances, so the shipped code is fine. The problem is narrower — a *criterion*
   stated in a document the PRD delegates UI authority to, with no fence around it.

**Fix:** scope the delegation at `prd.md:34` (e.g. "authoritative for UI specification
**except §§4.4, 4.8, 4.10, 4.11's selection and accessibility clauses**, architecture
detail, parser contracts, and the test plan"), and add the two unfenced classes to §0.1 —
they are exactly the kind of stale-source reconciliation §0.1 exists to prevent.

### 3.4 The keyboard map the PRD says survives is stated nowhere (MEDIUM)

`prd.md:435` asserts, as one of three things that explicitly stay:
`**⌘X / ⌘C / ⌘V / ⌘A and the ⌘R / ⌘A / ⌘L map** (RP-2).` It mirrors
`docs/DECISIONS.md:543` `**⌘X / ⌘C / ⌘V / ⌘A and the ⌘R/⌘A/⌘L keyboard map.**`

RP-2 does not contain that map. `prd.md:503` in full:
`Standard Edit and Window menu actions — including cut, copy, paste, and select-all in the
search field and in every copyable command surface — are preserved.` ⌘R (refresh) and ⌘L
(toggle the activity surface) are neither Edit nor Window menu actions. The cross-reference
dangles, and the PRD therefore states no requirement for a shipping surface it names as
protected.

The map is real and shipping: `docs/SPEC.md:288`
`Cmd+R refresh current manager (Dashboard: all) · Cmd+Shift+R refresh all · Cmd+U upgrade
selected (opens sheet) · Cmd+Shift+U Update Everything (sheet) · Cmd+A select all visible
selectable rows · Space toggle focused row · Esc clear selection / close sheet / close
drawer · Cmd+L toggle drawer · Cmd+F focus search · Cmd+1..9 sidebar jump.`, implemented in
`src/hooks/useKeyboard.ts:150–176` (`case "r"`, `"u"`, `"a"`, `"l"`, `"f"`, and
`if (/^[1-9]$/.test(key))`), whose own header comment at `src/hooks/useKeyboard.ts:2` reads
`useKeyboard — the global keyboard map (SPEC §4.11).`

**This is not a request to reinstate anything D37 removed.** D37 removed keyboard
*navigation and screen-reader support as release criteria*; it kept this map by name and
classified it as functional. The fix is to make RP-2 say what the PRD already claims it
says.

**Fix:** extend RP-2 to name the application accelerators that must survive
`app.set_menu` — at minimum ⌘R, ⌘L, and ⌘F alongside the Edit/Window standards — and drop
the ⌘A ambiguity (Edit select-all vs. select-all-rows) by naming them separately. Note that
per FR-6, ⌘A's row semantics change from "select all visible" to "stage all eligible
matching the active filter", so RP-2 should state the accelerator and defer the semantics
to FR-6.

### 3.5 A pending or failed plan rebuild must block confirmation (LOW)

`docs/SPEC.md:284` — `A pending or failed rebuild disables Upgrade; any execute error
consumes the attempted capability and requires a fresh plan, and late execute/rebuild
results are ignored after dismissal.`

The PRD carries the second and third clauses (`prd.md:262` and `prd.md:263`) but not the
first. It ships: `src/components/dialogs/UpgradePlanSheet.tsx:343`
`disabled={submitting || planReadiness !== "ready" || !hasCommands}`, with the module
comment at `src/components/dialogs/UpgradePlanSheet.tsx:7` reading
`so a pending or failed rebuild can never execute an older preview.`

This is a fail-closed acceptance condition of the same class the PRD kept elsewhere. The
nearest text, `prd.md:248` (`Commands are rebuilt by the backend whenever the draft changes
and again before execution.`), says rebuilds happen but not that confirmation is blocked
while one is in flight — which is the part that prevents confirming a stale preview.

**Fix:** add to FR-7 or FR-8: *"Final confirmation is unavailable while a rebuild is in
flight or after a rebuild failure; the user cannot confirm a preview the backend has not
just re-derived."*

### 3.6 A Manager's own row is not listed as one of its Packages (LOW)

`docs/SPEC.md:556` — `  packages: Package[]; // excludes the manager's own self row` — and
`docs/SPEC.md:405` — `the manager's own row (npm) is extracted into \`selfPackage\`.`
Ships: `src-tauri/src/managers/npm.rs:195`
`// The npm row lives only on the SelfUpdateCard (SPEC §5.4, F6).`

`grep -ic "own row\|self row" prd.md addendum.md` → `0` and `0`. FR-5 governs the Package
table and FR-11 governs the Manager identity area, but nothing says a Manager's own entry
belongs only to the latter. Implemented literally from the PRD, npm appears twice — once as
a Package row and once as the Manager — and under FR-11's Planned change
(`prd.md:302`, `the Manager update action adds *independent, individually removable* plan
membership`) the user could stage the same update twice by two routes.

**Fix:** one clause in FR-5 or FR-11: *"A Manager's own entry in its own inventory is
surfaced as that Manager's self-update state, not as a Package row."*

### 3.7 Plan-request input bounds (LOW)

`docs/SPEC.md:454` — `receives a canonical request whose explicit selection has at most
2,048 entries, package IDs of at most 512 bytes, and exact duplicate manager/package pairs
removed first-seen-order`. Ships: `src-tauri/src/queue.rs:400`
`pub const MAX_PLAN_SELECTIONS: usize = 2_048;` and `src-tauri/src/queue.rs:401`
`pub const MAX_PLAN_PACKAGE_ID_BYTES: usize = 512;`.

FR-8 states the capability bound (`prd.md:259`, `at most 64 unconsumed capabilities are
retained per session`) but not the request bounds. By `addendum.md:22`'s own admission
criterion — technical detail that `directly constrains observable trust` — bounded,
deduplicated plan input belongs with the bounded capability it already kept. Low severity
because the only input source is the app's own UI.

**Fix:** one clause in FR-8: *"A submitted plan request is bounded in size and deduplicated
before it is expanded; an over-large or malformed request is refused rather than
truncated."*

---

## 4. Minor notes (not reported as findings)

- **Search over executables.** `docs/SPEC.md:271` `SearchInput (240px, filters name +
  executables, 200ms debounce)`, ships at `src/hooks/useKeyboard.ts:48`. `prd.md:194` says
  only `browse, search, filter`. Reachable via FR-5's `uv executables` clause
  (`prd.md:202`); noted, not filed.
- **`Skipped` as the terminal state of queued Operations under `Cancel plan`.**
  `docs/SPEC.md:94` `queued Operations for that \`planAttemptId\` become \`Skipped\``. The PRD
  reaches it obliquely through FR-13's Results bullet (`prd.md:336`, `skipped work`).
  Planned state; adequate for now.
- **SPEC §8 omits the two release-blocking checks** that NFR-8 requires
  (`prd.md:557`). SPEC:815 documents only the notarization-ticket assertion. A gap in SPEC,
  not in the PRD.
- **SPEC:138's target machine is `macOS 27.0 beta`** while D31 sets the floor at 15.0
  (`prd.md:551`). SPEC states a development machine, not a support floor — not a conflict.

## 5. Ruled out — do not re-file

Checked and confirmed *correctly* excluded or correctly covered, listed so a later pass
does not re-raise them: SPEC §5.2 PATH construction, §5.3 classification algorithm,
§§5.4–5.5 adapter/parser contracts, §5.7 scheduler internals and the aging guard, §5.9 wire
shapes, §5.11 store shape, §6 transcript/log format, §7 test names — all in
`addendum.md:11–20`. SPEC's seven load-bearing invariants (`docs/SPEC.md:13–19`) all map:
1→FR-2, 2→FR-4 + `prd.md:131`, 3→FR-9, 4→FR-12, 5→NFR-2, 6→§4.2, 7→FR-15 (Planned). SPEC's
five explicit non-features (`docs/SPEC.md:132`) are all in PRD §6. SPEC P1 F13–F17 are all
in PRD §7.3 (`prd.md:599`); SPEC P2 (`docs/SPEC.md:128`) is in `prd.md:600–601`. F1–F8
acceptance criteria all have FR homes. Nothing D33 retired or D37 removed is reported as a
gap anywhere above.
