# Currency / reality review — ARCHITECTURE-SPINE.md revision 10

**Target:** `_bmad-output/planning-artifacts/architecture/architecture-Pack-Manager-2026-07-23/ARCHITECTURE-SPINE.md`
(revision 10, working-tree state — the file is modified and uncommitted:
`git diff --stat` → `1 file changed, 312 insertions(+), 56 deletions(-)`)

**Tree state:** branch `chore/restore-phase-2-prd`, HEAD `1ac959e`
(`ci: bump the Claude Code action model to opus-5`). `origin/main` tip is
`5972109` (`Clear the epics.md residual pile (#36)`).

**Date:** 2026-07-25. Read-only review — the target was not edited.

---

## Tally

| | Count |
| --- | --- |
| Checkable claims examined | **158** |
| Verified exactly | **145** |
| Failed (FALSE) | **8** |
| Failed (IMPRECISE / STALE) | **5** |
| UNVERIFIED | **0** |

By severity: **CRITICAL 3**, **HIGH 3**, **MEDIUM 4**, **LOW 3**.

The complete inventory of examined claims is in the appendix, grouped by
section, so the count above is a count and not an estimate.

**Headline:** the *technical* half of revision 10 is clean. Every row of the
Stack table resolves exactly against the lockfiles, every inline code citation
in every `AD` — including all of AD-28's and AD-29's new material — matches the
tree verbatim, and every external quotation in AD-26 matches the raw
`tauri.app/llms-full.txt` source word for word. The failures are concentrated in
one place: the **`epics.md` residuals row** of the Decision Status table
(line 1282) and the sentence at line 1280 that feeds it. That row's claims about
`epics.md` were carried forward from an earlier revision and are now contradicted
by `epics.md` at HEAD, which was updated by `5972109` (#36). Revision 10 states
these residuals as *newly urgent* — "the more urgent class, because these now
contradict rows above" — when the opposite is true: `epics.md` agrees with the
spine and the spine says it does not.

---

## CRITICAL

### C1 — "AD-27 is cited nowhere" is false; `epics.md` cites it on 32 lines

**Claim (stated twice):**

`ARCHITECTURE-SPINE.md:1282` — *"(5) **AD-27 is cited nowhere**, because this
revision created it — every story that renders a control needs it on its
Dependencies line, and per the spine's own standing instruction the citation is
by `AD` id and subject, never by rule ordinal."*

`ARCHITECTURE-SPINE.md:1280` — *"**AD-27 is the exception, and unavoidably so** —
this revision created it after the batch landed, so `epics.md` cannot yet cite
it; that is tracked as a residual below rather than counted as a failure of the
batch."*

**Command:**

```
grep -c 'AD-27' _bmad-output/planning-artifacts/epics.md
grep -n 'AD-27' _bmad-output/planning-artifacts/epics.md | head -40
```

**Result:** `32`. Sample lines, verbatim:

```
514:**Dependencies:** D27-D30; AD-16; AD-17; finalized UX spines; AD-27 (focus is a 2px `outline` in `--color-focus-ring`; never a `ring-*`/box-shadow, which WKWebView drops on native-appearance controls)
1109:**Dependencies:** UX-PB.5a; finalized focus and high-zoom contracts; FR-19; AD-27 (focus is a 2px `outline` in `--color-focus-ring`; never a `ring-*`/box-shadow, which WKWebView drops on native-appearance controls)
1272:- Governing invariants: AD-16, AD-17, AD-27 (focus is a 2px `outline` in `--color-focus-ring`; never a `ring-*`/box-shadow, which WKWebView drops on native-appearance controls)
```

`AD-27` appears on the **Dependencies line of every UX-PB story** (28 of them),
on the `Governing invariants` line of Stories 3.1/3.2/3.5, and inside the
Implementation-Entry register at line 308 — cited by `AD` id **and subject**,
exactly the form the residual demands. The citation is not missing; it is already
in the form the spine asks for.

**Verdict:** FALSE, and doubly so — both the residual and the line-1280 sentence
that justifies it.

**Remedy:** delete residual (5)'s AD-27 half and the line-1280 "AD-27 is the
exception" sentence. If anything survives, it is the *inverse* observation: the
AD-27 parenthetical is duplicated verbatim 30 times, which is a maintenance
hazard the next correct-course run may want to collapse to a single citation
form. Do **not** send a run to add citations that are already there.

---

### C2 — The design-token register row is claimed `OPEN`; `epics.md` reads `CLOSED`

**Claim:** `ARCHITECTURE-SPINE.md:1282` — *"(3) `epics.md`'s
Implementation-Entry register still lists the canonical design-token set as
`OPEN` and **"Blocks UX-PB.1e and UX-PB.5d"**, which D35 closed; those two
stories are unblocked and `epics.md` says otherwise."*

**Command:**

```
grep -n 'Blocks UX-PB.1e' _bmad-output/planning-artifacts/epics.md
grep -n 'Canonical design-token set' _bmad-output/planning-artifacts/epics.md
```

**Result:** `Blocks UX-PB.1e` → **no matches**. The register row reads, verbatim
at `epics.md:308`:

```
| Canonical design-token set | `CLOSED` — D35 | Resolved 2026-07-25 | Nothing blocked. `DESIGN.md`'s palette was adopted into `src/styles/theme.css`, focus gained a dedicated indicator, and the CI style contract moved with it in one change — see `ARCHITECTURE-SPINE.md` AD-27 and the *Canonical design-token set* row of its Decision Status table. …
```

and the row above it, `epics.md:307`, reads:

```
… nothing blocks starting it — the canonical design-token set that blocked UX-PB.1e and UX-PB.5d was decided and shipped (`docs/DECISIONS.md` D35), so both are startable.
```

**Verdict:** FALSE. `epics.md` does not say otherwise; it says exactly what the
spine says, and cites this spine by `AD` id and row title while doing so.

**Remedy:** delete residual (3).

---

### C3 — The `notarytool minos 15.0` question is claimed `OPEN`; `epics.md` records it `CLOSED`

**Claim:** `ARCHITECTURE-SPINE.md:1282` — *"(4) the same register still records
the `notarytool minos 15.0` question as OPEN, which D34 closed."*

**Command:** `grep -n 'notarytool' _bmad-output/planning-artifacts/epics.md`

**Result:** one match, `epics.md:309`, verbatim:

```
| DR-1 — minimum supported macOS | `CLOSED` — D31 | Resolved 2026-07-24 | None. 15.0 declared and shipped in v1.0.0. The `notarytool` `minos 15.0` question is CLOSED by `docs/DECISIONS.md` D34: CI and release moved to `macos-15`, so the build SDK is no longer behind the declared floor and the mismatch the question was about no longer exists. A manual Release run verified signing and notarization on the new image. |
```

**Verdict:** FALSE.

**Remedy:** delete residual (4). Note the corroborating negative:
`grep -c 'macos-14' _bmad-output/planning-artifacts/epics.md` → `0`.

---

## HIGH

### H1 — Six of seven cited commit hashes do not exist in `origin/main`'s history; five are on no branch at all

**Claims:** the spine cites commits as evidence in eight places —
`ARCHITECTURE-SPINE.md:1280` (`8d36cdf`), `:1258` and `:347` (`419dc32`),
`:1278` (`7cc7b5f`), `:1257` (`be1f0e6`), `:362`/`:1001`/`:1257` (`a201fb0`),
`:1286` (`22ed41e`), `:1285` (`5c8996f`).

**Command:**

```
for c in 8d36cdf 419dc32 7cc7b5f be1f0e6 a201fb0 22ed41e 5c8996f; do
  git merge-base --is-ancestor $c HEAD && a=ANCESTOR || a=NOT-ancestor
  git merge-base --is-ancestor $c origin/main && b=on-main || b=NOT-on-main
  echo "$c $a $b"; done
git branch -a --contains 22ed41e
```

**Result:**

```
8d36cdf  NOT-ancestor     NOT-on-origin/main
419dc32  NOT-ancestor     NOT-on-origin/main
7cc7b5f  NOT-ancestor     NOT-on-origin/main
be1f0e6  NOT-ancestor     NOT-on-origin/main
a201fb0  ANCESTOR-of-HEAD NOT-on-origin/main
22ed41e  NOT-ancestor     NOT-on-origin/main
5c8996f  ANCESTOR-of-HEAD NOT-on-origin/main
```

`git branch -a --contains 22ed41e` → **empty**. Five of the seven hashes are
unreferenced objects surviving only in the local object store / reflog; they will
disappear on `git gc`. All five were squash-merged into `c8c1f9a`
(`Reconcile the planning spine with reality, and fix what that surfaced (#35)`),
confirmed by content search:

```
git log --oneline -S 'session-scoped and never written to disk, so membership is never reconstructed' -- _bmad-output/planning-artifacts/epics.md
→ c8c1f9a Reconcile the planning spine with reality, and fix what that surfaced (#35)
git log --oneline -S 'macos-15' -- .github/workflows/ci.yml
→ c8c1f9a Reconcile the planning spine with reality, and fix what that surfaced (#35)
```

The **substance** of every cited commit is present in the tree and was verified
independently (see appendix rows B21, B24, AD11-3, AD27-*, DS-*). This finding is
about the citations, not the work.

**Verdict:** the claims about *what landed* are TRUE; the claims about *where it
landed* are unresolvable against the published history. This is the same class of
defect as citing a document by line number, which the spine itself names as a
recurring failure at line 1282.

**Remedy:** repoint every commit citation to the squash-merge commit and PR
number that actually carries it — `c8c1f9a` (#35) for `8d36cdf`, `419dc32`,
`7cc7b5f`, `be1f0e6`, `22ed41e`; `a201fb0` and `5c8996f` are still unmerged local
commits on `chore/restore-phase-2-prd` and should be cited as such, or restated
by content rather than hash, until they merge.

---

### H2 — Residual (1) says AD-25's rule "appears nowhere the builder reads"; UX-PB.3d states it verbatim in criterion text

**Claim:** `ARCHITECTURE-SPINE.md:1282` — *"(1) **UX-PB.3d cites AD-25 but never
states it** — its Dependencies line carries AD-25, yet its verification-failure
criterion says only that the item "stays `Verifying` until it resolves, then
reports verification failure with its evidence, and is never colored successful
on the strength of the exit code alone". AD-25's rule for that path — a failed or
timed-out verification *leaves the Last-good Snapshot in place* — appears nowhere
the builder reads."*

**Command:**

```
grep -n "stays \`Verifying\` until it resolves" _bmad-output/planning-artifacts/epics.md
grep -n "the Manager's Last-good Snapshot is left in place with its timestamp" _bmad-output/planning-artifacts/epics.md
```

**Result:** the quoted criterion is present at `epics.md:844` — and is
immediately followed at `epics.md:845` by:

```
**And** the Manager's Last-good Snapshot is left in place with its timestamp (AD-25) — a verification refresh that errors or times out never replaces or clears the snapshot it failed to refresh, so the surface keeps showing the last state it actually knows rather than blanking.
```

The Dependencies line at `epics.md:827` also carries the substance inline:
`AD-25 (a failed verification refresh leaves the Last-good Snapshot in place)`.

**Verdict:** FALSE. The residual quotes the criterion accurately and then asserts
the absence of the sentence directly beneath it.

**Remedy:** delete residual (1).

---

### H3 — Residual (2) says AD-21's substance "never reaches criterion text"; UX-PB.5b states it as its own `And` clause

**Claim:** `ARCHITECTURE-SPINE.md:1282` — *"(2) **AD-21's substance never reaches
criterion text**, surviving only as a parenthetical on UX-PB.5b's Dependencies
line, while AD-22's and AD-23's substance *is* restated in criterion prose."*

**Command:**
`grep -n 'skipUpgradePlanConfirmation\` is plan-inert (AD-21)' _bmad-output/planning-artifacts/epics.md`

**Result:** `epics.md:1066`, inside UX-PB.5b's acceptance criteria, verbatim:

```
**And** `skipUpgradePlanConfirmation` is plan-inert (AD-21) — it is not a plan-determining input, so writing it never advances the canonical revision and never invalidates the preview it rides on. Without that, this story's own save would expire the plan it just admitted and the safety opt-out would deterministically fail its own run.
```

That is AD-21's substance *and* its rationale, in criterion prose, one clause
after the AD-22 ordering clause the residual credits.

**Verdict:** FALSE.

**Remedy:** delete residual (2).

---

## MEDIUM

### M1 — Residual (5)'s "cite this spine by line number" is false for both rows

**Claim:** `ARCHITECTURE-SPINE.md:1282` — *"Both cite this spine by **line
number**, and those citations have already drifted — the same positional-reference
failure this run folder has now hit three times…"*

**Command:** `grep -n 'ARCHITECTURE-SPINE.md' _bmad-output/planning-artifacts/epics.md | grep -c ':[0-9]'` on the two register rows; read `epics.md:307-309`.

**Result:** `epics.md:308` cites *"`ARCHITECTURE-SPINE.md` AD-27 and the
*Canonical design-token set* row of its Decision Status table"* — by `AD` id and
row title. `epics.md:309` cites *"`docs/DECISIONS.md` D34"* — by decision id.
Neither carries a spine line number.

**Verdict:** FALSE. (Dependent on C2/C3: the rows it describes no longer exist in
the form described.)

**Remedy:** delete with residuals (3) and (4).

---

### M2 — Residual (6)'s `⌘U` limb: `epics.md` contains no `⌘U` anywhere

**Claim:** `ARCHITECTURE-SPINE.md:1282` — *"(6) **Story 3.5's criteria are
written on the selection model AD-28 abolishes** — toggle, shift-range,
tri-state, `⌘A`, Space, `⌘`-click, Clear and `Esc` against "the exact selectable
identities", plus a `Clear`/`Esc`-clears-selection rung AD-28 deletes and a `⌘U`
limb `prd.md` FR-6 drops."*

**Command:** `grep -n 'Cmd+U\|⌘U\|Cmd-U' _bmad-output/planning-artifacts/epics.md`

**Result:** **no matches.**

The rest of the residual verifies. `epics.md:1278-1279` reads, verbatim:

```
**When** toggle, shift-range, tri-state, Cmd+A, Space, Cmd-click, Clear, and Esc interactions execute
**Then** the exact selectable identities and visible filter semantics are preserved
```

**Verdict:** the residual is TRUE on the selection model and the `Clear`/`Esc`
rung, and FALSE on the `⌘U` limb — Story 3.5 has no `⌘U` limb to drop.

**Remedy:** strike ", and a `⌘U` limb `prd.md` FR-6 drops" from residual (6).
Keep the rest; it is the strongest surviving residual.

---

### M3 — Residual (8) asks for an inertness rule UX-PB.1d already carries

**Claim:** `ARCHITECTURE-SPINE.md:1282` — *"(8) UX-PB.1d's ineligibility
criterion needs AD-16's corrected inertness rule: **not** native `disabled`,
pointer-interactive, inert."*

**Command:**
`grep -n 'aria-disabled="true"\` rather than native' _bmad-output/planning-artifacts/epics.md`

**Result:** `epics.md:613`, verbatim:

```
**Then** it uses `aria-disabled="true"` rather than native `disabled`, keeps focus, announces its persistent reason as an accessible description, stays inert on activation, and retains focus when Escape closes its supplemental Tooltip/Popover.
```

"not native `disabled`" and "inert" are both already stated. What is *not* stated
is AD-16's newer framing — that the control must remain a **pointer-interaction
target** — and that clause currently sits under a `**When** a keyboard or
VoiceOver user reaches it` Given that D37 scopes out.

**Verdict:** IMPRECISE. The residual asks for two things that are present and one
that is not, and its wording implies all three are missing.

**Remedy:** narrow residual (8) to the pointer-interaction half and to moving the
`aria-disabled` clause off the keyboard/VoiceOver Given that D37 removes, so the
requirement does not disappear with the Given that hosts it.

---

### M4 — The design-token status row still says "All 22 `focus-visible` sites"; the tree has 31

**Claim:** `ARCHITECTURE-SPINE.md:1257` — *"All 22 `focus-visible` sites resolve
`--color-focus-ring`…"* (present tense).

**Command:**

```
grep -rno 'focus-visible:[a-z0-9-]*' src/ | sed 's/.*focus-visible:/focus-visible:/' | sort | uniq -c
```

**Result:**

```
  31 focus-visible:outline-2
  31 focus-visible:outline-focus-ring
  30 focus-visible:outline-offset-1
   1 focus-visible:outline-offset-2
```

31, not 22. The same document already knows this: `ARCHITECTURE-SPINE.md:1286`
says *"corroborated by the site count going from 22 to 31"*, and `docs/DECISIONS.md`
D37 says *"Merged across 31 sites and asserted in CI"*.

**Verdict:** IMPRECISE / STALE — a present-tense sentence describing D35's state
before `22ed41e`'s content landed, sitting two rows above the row that corrects
it. Internally contradictory as written.

**Remedy:** tense-shift to "All 22 `focus-visible` sites *at that commit*
resolved `--color-focus-ring`", or restate as 31.

---

## LOW

### L1 — NFR-3 quotation drops its leading "The"

`ARCHITECTURE-SPINE.md:1057-1058` — *"breaks NFR-3's "interface stays interactive
beyond 100 Packages, with correct actions reachable at 101 rows" (`prd.md` FR-6)"*.

`prd.md:556` (NFR-3): *"**The** interface stays interactive beyond 100 Packages,
with correct actions reachable at 101 rows."* — and `prd.md:235` (FR-6) quotes it
with the article intact.

IMPRECISE. The citation target (FR-6) is correct and FR-6 does attribute it to
NFR-3, so the attribution is sound. Restore the leading "The" so the quoted string
is greppable against both sources.

### L2 — AD-26's `tauri.app` quotations are verbatim; one apostrophe differs typographically

This is the failure mode revision 8 committed, so it was checked against the raw
source rather than a retrieval tool's summary:

```
curl -sL https://tauri.app/llms-full.txt -o /tmp/tauri-llms-full.txt   # 2,430,050 bytes, 67,412 lines
grep -n "no WKWebView driver tool" /tmp/tauri-llms-full.txt
grep -n "Windows, Linux, and macOS" /tmp/tauri-llms-full.txt
grep -n "embedded WebDriver server" /tmp/tauri-llms-full.txt
grep -n "tauri-plugin-wdio-webdriver" /tmp/tauri-llms-full.txt
grep -n "paid API key" /tmp/tauri-llms-full.txt
```

All five spine quotations match the raw source **word for word**:

- line 9532: *"Driven directly, only Windows and Linux are supported on desktop,
  as macOS has no WKWebView driver tool available (use the service's embedded
  WebDriver server for macOS)."* ✔ (AD-26, `:922-925`)
- line 9483: *"…the `@wdio/tauri-service`…, which works on **Windows, Linux, and
  macOS**."* ✔ (AD-26, `:926`)
- line 9485: *"By default the service runs an **embedded WebDriver server** inside
  your app…"* ✔ (AD-26, `:927`)
- line 9511: *"**`tauri-plugin-wdio-webdriver`** runs the embedded WebDriver
  server."* ✔ (AD-26, `:928`)
- line 9485: *"(a paid API key is required for macOS)"* ✔ (DS row, `:1267`)

Only difference across all five: the spine writes `service's` with a straight
apostrophe where the source has the typographic `service’s`. Typographic only —
no semantic drift. Optionally normalize so a literal grep against the source
succeeds.

### L3 — D37's own mention counts are wrong, and the spine's restatement is approximate

`ARCHITECTURE-SPINE.md:1282` — *"the counts in `docs/DECISIONS.md` D37 were copied
rather than measured and are wrong (`prd.md` addendum §3 re-measured them), and
two of this spine's eight keyword matches were material D37 protects."*

`docs/DECISIONS.md:557-558` says `ARCHITECTURE-SPINE.md` (3). `addendum.md:55`
says *"~8 lines, including the manual pass at 332 and 941"*. The spine's "eight"
is the addendum's "~8" stated as exact. The *substance* — that D37's counts are
wrong and that scoping by mention count is unsafe — verifies. LOW; the
approximate-stated-as-exact pattern is worth flagging only because this document
elsewhere insists counts come from a command.

---

## What verified exactly — appendix inventory

### A. Verified Brownfield Baseline (`:146-189`) — 31 claims, 31 verified

| # | Claim | Command | Result |
| --- | --- | --- | --- |
| B1 | 20 Tauri commands registered | `awk '/generate_handler!\[/,/\]\)/' src-tauri/src/lib.rs \| grep -o 'commands::[a-z_]*' \| sort -u \| wc -l` | `20` |
| B2 | six typed events | `grep -c 'pub const EVENT_' src-tauri/src/events.rs` | `6` (`detection:updated`, `snapshot:updated`, `op:status`, `op:output`, `op:stalled`, `appUpdate:status`) |
| B3 | `bridge.ts` sole frontend Tauri importer | `grep -rn '@tauri-apps/api' src/` | only `src/lib/ipc/bridge.ts` |
| B4 | startup subscribes before hydration | `src/App.tsx:56` | `// Subscribe BEFORE hydrating: 'detection:updated' is emitted only after the` |
| B5 | 15 committed contract fixtures | `ls dev/fixtures/ipc/ \| wc -l` | `15` |
| B6 | `ipc.rs` byte-compares against fixtures | `src-tauri/src/ipc.rs:4` | `//! 'ipc_contract_matches_committed_fixtures' (Rust) and` |
| B7 | structured argv | `src-tauri/src/process/runner.rs:299` region | `.env_clear()` on structured `Command` |
| B8 | cleared environment | `runner.rs:299` | `.env_clear()` |
| B9 | null stdin | `runner.rs:301` | `.stdin(Stdio::null()) // no sudo, no password entry, ever` |
| B10 | isolated process groups | `runner.rs:304` | `.process_group(0)` |
| B11 | timeout | `runner.rs:6` | `//! timeout, and SIGTERM → 5s grace → SIGKILL via 'nix::killpg'.` |
| B12 | SIGTERM→SIGKILL escalation | `runner.rs:261,270` | `killpg(pgid, Signal::SIGTERM)` … `killpg(pgid, Signal::SIGKILL)` |
| B13 | opener/reveal direct OS calls | `grep -n 'reveal_item_in_dir\|open_path' src-tauri/src/commands.rs` | `:672 tauri_plugin_opener::reveal_item_in_dir`, `:681 tauri_plugin_opener::open_path` |
| B14 | Plan is transient dialog state `{kind:"upgradePlan"}` | `grep -rn '"upgradePlan"' src/` | `ManagerPane.tsx:130`, `Sidebar.tsx:59`, `ManagerCard.tsx:61` all `openDialog({ kind: "upgradePlan", plan })` |
| B15 | discarded by `closeDialog` | `src/store/ui.ts:116` | `closeDialog: () => set({ dialog: { kind: "none" } })` |
| B16 | row action executes immediately | `src/components/manager/ManagerPane.tsx:145,152` | `async function upgradeRow(pkg: Package)` … `await executePlan(plan)` |
| B17 | durable token is a monotonic `revision` in `PlanCoordinator` | `src-tauri/src/state.rs:51,212` | `pub struct PlanCoordinator`, `pub plan_coordinator: Arc<Mutex<PlanCoordinator>>` |
| B18 | no `planAttemptId`/`Verifying`/`InteractionRequired` in `src/` or `src-tauri/src/` | `grep -rn 'planAttemptId\|plan_attempt_id\|InteractionRequired\|skipUpgradePlanConfirmation' src/ src-tauri/src/ \| wc -l` then same for `Verifying` | `0` and `0` |
| B19 | `autoOpenDrawer` still active | `grep -rn 'autoOpenDrawer' src/` | live at `SettingsView.tsx:134-135`, `useOperationEffects.ts:53` |
| B20 | `packages.ts` holds `selection` Set per Manager | `src/store/packages.ts:17` | `selection: Partial<Record<ManagerId, Set<string>>>;` |
| B21 | toggle / range / set / clear primitives + per-Manager `anchor` | `packages.ts:18,73,83,87,106,110` | all five present; `anchor: Partial<Record<ManagerId, string \| null>>` |
| B22 | predicate ships twice | `ManagerPane.tsx:92-107` (inline) and `useKeyboard.ts:35-53` (`visibleSelectableIds`) | both compute `caskGreedy` exclusion + search + `outdatedOnly` + `isSelectable` |
| B23 | comment says "mirrors ManagerPane filters" | `grep -rn 'mirrors' src/hooks/useKeyboard.ts` | `:34 /** Visible + selectable package ids for a manager (mirrors ManagerPane filters). */` |
| B24 | pinned row checkbox natively `disabled` with reduced opacity | `grep -n 'disabled\|opacity' src/components/manager/PackageRow.tsx` | `:92 disabled={checkboxDisabled}`, `:100 "disabled:cursor-not-allowed disabled:opacity-40"` |
| B25 | `settings.json` atomic replace | `src-tauri/src/settings.rs:139-145` | `let tmp = path.with_file_name(...)` → `f.sync_all()?` → `std::fs::rename(&tmp, path)` |
| B26 | `operations.jsonl` compacted to newest 1,000 | `grep -rn 'COMPACT_KEEP' src-tauri/src/` | `journal.rs:19 pub const COMPACT_KEEP: usize = 1000;` used at `state.rs:234` |
| B27 | compaction is temp + fsync + rename | `journal.rs:210` | `/// Write-to-temp + fsync + rename in the target's own directory` |
| B28 | export ships `report.json` | `diagnostics.rs:129` | `zip.start_file("report.json", ...)` |
| B29 | newest three app logs | `diagnostics.rs:240` | `assert_eq!(log_names.len(), 3, "last 3 app logs");` |
| B30 | newest 25 transcripts | `diagnostics.rs:23` | `pub const TRANSCRIPTS_INCLUDED: usize = 25;` |
| B31 | min macOS 15.0 + version release-please-owned, 1.0.1 | `grep -n minimumSystemVersion src-tauri/tauri.conf.json`; `cat .release-please-manifest.json` | `"minimumSystemVersion": "15.0"`; `{".":"1.0.1"}` |

### B. Stack table (`:1196-1217`) — 20 rows, 20 verified

Resolved versions read from lockfiles, never from declared ranges.
`node -e` over `package-lock.json` `packages[...]`.version (lockfileVersion 3),
and `grep -A1 '^name = "<crate>"$' src-tauri/Cargo.lock`:

| Row | Spine says | Lockfile resolves |
| --- | --- | --- |
| Application version | 1.0.1 | `.release-please-manifest.json` → `{".":"1.0.1"}` ✔ |
| Rust edition | 2021 | `src-tauri/Cargo.toml:6 edition = "2021"` ✔ |
| Tauri Rust crate | 2.11.5 | `2.11.5` ✔ |
| Tauri JavaScript API | 2.11.1 | `2.11.1` ✔ |
| Tauri CLI | 2.11.4 | `2.11.4` ✔ |
| Tauri updater plugin | 2.10.1 | `2.10.1` ✔ |
| Tauri opener plugin | 2.5.4 | `2.5.4` ✔ |
| Tokio | 1.53.1 | `1.53.1` ✔ |
| React / React DOM | 19.2.8 | `19.2.8` / `19.2.8` ✔ |
| TypeScript | **7.0.2** | `7.0.2` ✔ — flagged as suspect by training intuition, **correct here** |
| Vite | **8.1.5** | `8.1.5` ✔ — likewise correct |
| Tailwind CSS | **4.3.3** | `4.3.3` ✔ — likewise correct |
| Zustand | 5.0.14 | `5.0.14` ✔ |
| TanStack React Virtual | 3.14.8 | `3.14.8` ✔ |
| Vitest | 4.1.10 | `4.1.10` ✔ |
| Playwright | 1.61.1 | `@playwright/test` `1.61.1`, `playwright` `1.61.1` ✔ |
| Node in CI | 24 | `.nvmrc` → `24`; `ci.yml:57,76` and `release.yml:80` → `node-version: 24` ✔ |
| CI runner images | macos-15 (`ci.yml` rust, `ci.yml` build-smoke, `release.yml` build); ubuntu-latest elsewhere | `grep -rn 'runs-on:' .github/workflows/` → `ci.yml:28` (job `rust`), `ci.yml:70` (job `build-smoke`), `release.yml:63` (job `build`) are `macos-15`; all seven other jobs are `ubuntu-latest` ✔ |
| Minimum supported macOS | 15.0 | `tauri.conf.json:48` ✔ |
| Release automation | release-please action v5 | `release-please.yml:63,174 uses: googleapis/release-please-action@v5` ✔ |

### C. Inline code citations in the `AD`s — 62 claims, 62 verified

**AD-3** — `bridge.ts` re-exports exactly `invoke`, `listen`, `UnlistenFn` ✔
(`bridge.ts:9-11`); 20 commands / six events baseline ✔ (B1, B2);
`PM_UPDATE_CONTRACT=1 cargo test ipc_contract` matches
`ipc_contract_matches_committed_fixtures` ✔; TypeScript half asserts fixture set
equals guard map ✔ (`src/lib/ipc/types.test.ts:56` — `it("covers exactly the
committed fixture set", …)`, `:64` — the failure message literally names
`regenerate with PM_UPDATE_CONTRACT=1`).

**AD-4** — five ports exist and are exactly the five named ✔
(`grep -rn 'pub trait ' src-tauri/src/` → `EventSink` `events.rs:124`,
`ManagerAdapter` `managers/mod.rs:67`, `UpdateSource` `app_update.rs:41`,
`PendingRelease` `app_update.rs:48`, `CommandRunner` `process/runner.rs:26` —
five, no more); global concurrency cap of 4 ✔ (`queue.rs:48
pub const MAX_CONCURRENCY: usize = 4;`); 120s aging guard ✔ (`queue.rs:50
pub const AGING_GUARD: Duration = Duration::from_secs(120);`); no shell command
string ✔ (structured `Command` only); `env_clear`/null stdin/`process_group(0)` ✔
(B8–B10); mise-shim-before-canonicalize routing ✔ (`docs/SPEC.md` load-bearing
invariant 2, verbatim).

**AD-5** — diagnostics rejects symlinks both when selecting and when streaming ✔
(`diagnostics.rs:72-81` — `symlink_metadata`; regression test
`export_never_follows_symlinks_into_the_bundle` at `:274`).

**AD-11** — `minisign` verification of the base64-decoded detached signature
against the embedded pubkey ✔ (`release.yml:315-319`); `latest.json`
reachability/coherence assertion after upload ✔ (`release.yml:387-391`); both
`darwin-aarch64` and `darwin-x86_64` published ✔ (`release.yml:322-343`); all
three `macos-15` pins ✔ (Stack row above); `browser-style-contract.spec.ts` is the
inventory ✔ (323 lines; asserts dark tokens, focus treatment, reduced motion,
selection-vs-focus separation, and the 4.5:1 contrast floor); runs on every push
and PR to `main` ✔ (`test.yml:8-13` — `on: push: branches: [main]`,
`pull_request: branches: [main]`); reduced motion **is** automated ✔
(`browser-style-contract.spec.ts:120 expect(motion).toEqual({…})`); contrast guard
**does** now exist ✔ (`:226` `test("[P0] paints bright accent fills with ink that
clears the 4.5:1 contrast floor"…)`, `:320 expect(measured.ratio).toBeGreaterThanOrEqual(4.5)`).

**AD-11 / D37** — no VoiceOver or keyboard-navigation step on the checklist ✔
(`grep -in 'voiceover\|keyboard' docs/RELEASE-CHECKLIST.md` → only `:102`
*"**Keyboard navigation and screen-reader support are explicitly not release
criteria.**"*); `⌘X`/`⌘C`/`⌘V`/`⌘A` retained ✔ (`RELEASE-CHECKLIST.md:97`).

**AD-12** — seven release-please-owned files ✔ (five version files per
`CLAUDE.md` + `CHANGELOG.md` + `.release-please-manifest.json`);
`attach_to_tag` empty ⇒ publishes nothing ✔ (`release.yml:8`).

**AD-16** — `ManagerPane.upgradeRow` → `executePlan` call site ✔ (B16);
`OpStatus` ships seven variants ✔ (`ipc.rs:99-107` — `Queued, Running, Succeeded,
Failed, Cancelled, TimedOut, Interrupted`); pinned row is the defect, not the
reference ✔ (B24).

**AD-19** — persist-before-active-before-bump ✔ (`commands.rs:636-650`:
`save_to(...)?` → `*state.settings.write() = merged.clone()` →
`coordinator.bump_revision()`); the same call site bumps for every key ✔.

**AD-20** — `csp` is `null` ✔ (`tauri.conf.json:25`); one capability file granting
exactly `core:default`, `opener:default`, `core:window:allow-start-dragging` ✔
(`src-tauri/capabilities/default.json`, the only file in that directory).

**AD-21** — shipping call site bumps unconditionally ✔ (AD-19 above).

**AD-22** — *"No synchronous guard crosses an await"* ✔ **verbatim** at
`src-tauri/src/commands.rs:353`; `handle_plan_batch` performs the re-check ✔
(`queue.rs:1003-1015`: `if coordinator.revision() != expected_revision { … RevisionChanged }`);
`plan_coordinator` is a `std::sync::Mutex` ✔ (`state.rs:8
use std::sync::{Arc, Mutex, RwLock};`, `state.rs:212
pub plan_coordinator: Arc<Mutex<PlanCoordinator>>`); `save_to` fsyncs ✔ (B25);
`set_settings_core` legitimately holds the guard across its own save ✔.

**AD-25** — `ManagerAdapter::parse_recovery` takes `refresh_outputs` alongside
the failed command's output ✔ (`managers/mod.rs:89-94`:
`fn parse_recovery(&self, failed: &PlannedCommand, refresh_outputs: &[CommandOutput], out: &CommandOutput)`);
`docs/SPEC.md` load-bearing invariant 5 ✔ verbatim
(*"**One failing manager never blanks the others.**"*).

**AD-26** — five external quotations ✔ (L2 above); `src-tauri/Cargo.toml`
declares no `[profile.release]` ✔ (`grep -n 'profile' src-tauri/Cargo.toml` → no
match).

**AD-27** — `docs/DECISIONS.md` D37's protective sentence ✔ **verbatim**
(*"Deleting the rule would remove no work and would only un-guard working code
against the next `ring-*`"*, `DECISIONS.md:544`); `docs/SPEC.md` §4.1 ✔
**verbatim** (`SPEC.md:208` — *"a dedicated indicator, never `--color-accent`"*);
Tailwind 4 `outline-none` / `outline-hidden` rename ✔ (`SPEC.md:208` states the
same); 2px outline mechanism at 31 sites ✔ (M4 command); zero `ring-focus-ring`,
zero `ring-offset-*`, zero `outline-none` ✔ (all `grep -rn … src/ | wc -l` → `0`);
exactly one `ring-accent` survivor at `PackageRow.tsx:85` ✔
(`"ring-2 ring-inset ring-accent"`, no `focus-visible:` prefix); named samples are
a toolbar `<button>` and the package-row checkbox ✔
(`browser-style-contract.spec.ts:79` `refreshAll.focus()`, `:194` `checkbox.focus()`);
three-reported-vs-nine-actual ✔ (commit body of `22ed41e`: *"Nine controls had no
focus style at all, not the three a grep found."*); CI `webkit` is Playwright's
Linux WebKit on `ubuntu-latest` ✔ (`playwright.config.ts:85 name: "webkit"`;
`test.yml:56 runs-on: ubuntu-latest`); D36 contrast numbers ✔ **all six exact**
(`DECISIONS.md:498-502` — white 2.46 / 2.15 / 2.30, on-accent 7.74 / 8.87 / 8.30);
`no text-white remains in src/` ✔ (`grep -rn 'text-white' src/ | wc -l` → `0`);
tokens `--color-bg-base: #090C13`, `--color-accent: #65A7FF`,
`--color-focus-ring: #F4F7FB`, `--color-on-accent: #07101D`,
`--color-on-success: #07140D` ✔ (`src/styles/theme.css:8,19,27,30,32`); the guard
measures the rendered pair rather than token names ✔
(`browser-style-contract.spec.ts:289-304` — WCAG relative-luminance computed from
`style.color` / `style.backgroundColor`).

**AD-28** (revision 10 new material) — the live `selection` set in
`src/store/packages.ts` ✔ (B20); the anchor and its single-toggle fallback ✔
(`packages.ts:87-93` — *"// No usable anchor: behave like a single selection."*);
`EXPERIENCE.md` quotation ✔ **verbatim** (`EXPERIENCE.md:143` — *"On eligible
Package rows, selection immediately adds/removes Upgrade Plan membership."*);
`prd.md` §0.1 records F5's omission from SPEC §0.1's supersession list ✔
(`prd.md:45` — *"**F5 was never added to that list**"*); `docs/SPEC.md` F5 exists
and is the stale side ✔ (`SPEC.md:78 ### F5 (P0) Multi-select upgrade`); the
predicate ships twice ✔ (B22) with the comment quoted verbatim ✔ (B23);
`Esc` cascade reduces to close-dialog → close-drawer ✔ (`prd.md:237` — *"This FR
removes the middle rung only; Esc keeps close-dialog and close-drawer."*);
`⌘U` non-shift limb dropped, `⌘⇧U` unaffected ✔ (`prd.md:241`);
RP-2's surviving accelerator map is exactly `⌘R`, `⌘⇧R`, `⌘⇧U`, `⌘L`, `⌘F`,
`⌘1–9` plus `⌘A` as an Edit-menu action ✔ **verbatim** (`prd.md:534`);
**the shipping `⌘A` defect** ✔ — `useKeyboard.ts:160-162` is
`case "a": e.preventDefault(); selectAllVisible(); break;` and
`useKeyboard.ts:88-89` is `function selectAllVisible(): void { const ui = …; if
(ui.view.kind !== "manager") return;` — so on Dashboard, History and Settings the
native default is suppressed with nothing put in its place, exactly as AD-28
states.

**AD-29** (revision 10 new material) — `src-tauri/src/journal.rs` quotation ✔
**verbatim** (`journal.rs:4` — *"One line at op start, one at finish, flushed each
write."*); `StartRecord` and `FinishRecord` ✔ (`journal.rs:26`, `:43`);
start-without-finish reconstructs as `Interrupted` ✔ (`journal.rs:4-5` — *"Start-without-
finish renders `Interrupted` on the next launch."*); recorded pgids never signaled
✔ (`journal.rs:5`); `operations.jsonl` carries per-step detail ✔;
NFR-4's *"an unaudited command never starts"* ✔ (`prd.md:562` — *"Failure to create
an Operation transcript blocks the spawn — an unaudited command never starts."*);
a failed `Transcript::create` finishes the operation without spawning ✔
(`queue.rs:1499` — `let transcript = match Transcript::create(&op.log_path) {`).

### D. Consistency Conventions / Structural Seed / Capability Map — 9 claims, 9 verified

| # | Claim | Command | Result |
| --- | --- | --- | --- |
| D1 | 20 commands / six events is the current baseline | see B1, B2 | ✔ |
| D2 | `ErrorCode` is `snake_case` | `grep -n 'pub enum ErrorCode' -B4 src-tauri/src/error.rs` | `error.rs:67 #[serde(rename_all = "snake_case")]`; `ipc.rs:9` — *"`ErrorCode` is snake case"* ✔ |
| D3 | Package ids are `kind:name`, split on the first colon only | `grep -rn "split_once" src-tauri/src/` | `registry.rs:27 id.split_once(':').map(\|(_, name)\| name).unwrap_or(id)` ✔ |
| D4 | `mas` is the exception: its id segment is the numeric App Store id | `grep -rn 'numeric' src-tauri/src/managers/parse/mas.rs` | `parse/mas.rs:9-10` — *"unlike every other manager, the mas package id segment is the numeric App Store id (`app:497799835`), not the display name"* ✔ |
| D5 | Application Support holds `settings.json` (atomic replace) + append-only NDJSON journals compacted by temp + fsync + rename | see B25, B26, B27 | ✔ |
| D6 | The store may hold *where the user is*, never *what is staged* | `src/store/packages.ts:17-19` | `selection` **and** `anchor` both present today — the convention describes target state, correctly labelled as AD-17/AD-28 target ✔ |
| D7 | Styling: white on the three bright fills measures 2.15–2.46:1 | `docs/DECISIONS.md:498-500` | `2.46:1`, `2.15:1`, `2.30:1` — range stated correctly ✔ |
| D8 | Determinism: `CommandRunner`/`FakeRunner`, `EventSink`/`VecSink`, `bridge.ts`/`fakeIpc` | `ls src/test/` | `fakeIpc.ts`, `fixtures.ts`, `setup.ts`; traits at `runner.rs:26`, `events.rs:124` ✔ |
| D9 | Structural Seed — no draft file in Application Support | `grep -rn 'draft' src-tauri/src/ \| wc -l` | `0` — no draft path exists anywhere in Rust; consistent with AD-17 ✔ |

### E. Decision Status and Deferred Items — 36 claims, 23 verified, 13 failed

Verified in this section, notably:

- `macos-14` retirement — no `runs-on` in `.github/workflows/` names `macos-14` ✔
  (`grep -rn 'runs-on:' .github/workflows/` — full listing above), and
  `grep -c 'macos-14' _bmad-output/planning-artifacts/epics.md` → `0` ✔.
  `docs/SPEC.md:805` reads `rust (macos-15)` … `build-smoke (macos-15, main only)` ✔.
- The app-update guard is Rust and matches the frontend predicate exactly ✔ —
  `commands.rs:810 refuse_app_update_while_busy(&state.queue.records())?;` is the
  **first** statement of `install_app_update`, the helper filters
  `OpStatus::Queued | OpStatus::Running` (`:776-779`), and `activeOps` in
  `src/store/operations.ts:137` filters `o.status === "queued" || o.status ===
  "running"` ✔.
- Requirements authority — `prd.md` frontmatter `status: final`, and it restores
  FR-1…FR-22, RP-1/RP-2, NFR-1…NFR-8 ✔ (`prd.md:18`).
- Epic UX-PB is 28 stories ✔ —
  `grep -on 'UX-PB\.[0-9][a-g]' epics.md | sed 's/.*://' | sort -u | wc -l` → `28`.
- Residual (7) — `epics.md` FR-19 and NFR-6 still carry the D37-removed
  keyboard/VoiceOver and announcement obligations ✔ (`epics.md:89`, `:113`), and
  Story UX-PB.1d's heading still reads *"with keyboard, pointer, and VoiceOver
  explanation"* (`epics.md:596`) ✔. **This residual is TRUE and is the one that
  should survive.**
- Residual (9) — AD-28 and AD-29 are cited nowhere in `epics.md` ✔
  (`grep -o 'AD-28' epics.md | wc -l` → `0`; same for `AD-29`). **TRUE.**
- Retired ids AD-7/8/9/14 appear nowhere in `epics.md` ✔ (`0` each).
- `deferred-work.md` accessibility entries closed ✔ — the file now holds 10
  entries (`grep -c 'source_spec:'` → `10`) and none mentions focus,
  accessibility, contrast or keyboard (`grep -in 'a11y\|aria\|voiceover\|screen.reader\|focus\|contrast\|keyboard'`
  → no match). The two removed entries are visible in `git show 22ed41e -- …`.
  Minor imprecision: one of the two was a `ring-offset-*` entry rather than an
  "accessibility" entry per se.

Failed in this section: **C1, C2, C3, H1, H2, H3, M1, M2, M3, M4** (see above),
plus the two IMPRECISE items folded into H1's scope and M4's.

---

## Recommended edit set (one pass, all inside the Decision Status table)

1. **Rewrite the `epics.md` residuals row (`:1282`)** down to what survives:
   (6) minus its `⌘U` limb, (7), (8) narrowed to the pointer-interaction half, and
   (9). Delete (1), (2), (3), (4), and (5)'s AD-27 and line-number halves.
   Add the standing note that the row must be **re-verified against `epics.md` at
   HEAD** before each revision, because `5972109` (#36) already cleared most of it.
2. **Delete the "AD-27 is the exception" sentence at `:1280`.**
3. **Repoint all seven commit citations** to `c8c1f9a` (#35) / the current-branch
   commits, per H1.
4. **Tense-fix "All 22 `focus-visible` sites" at `:1257`** to 31, or scope it to
   D35's commit.
5. **Restore the leading "The"** in the NFR-3 quotation at `:1057`.

Nothing in `AD-1`…`AD-29`'s rule text needs to change. The invariants are sound
against the tree; only the status ledger drifted.
