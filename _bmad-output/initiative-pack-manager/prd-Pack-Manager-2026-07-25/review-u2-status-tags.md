# Review — status-tag truthfulness (Update pass 2)

**Artifact:** `_bmad-output/planning-artifacts/prds/prd-Pack-Manager-2026-07-25/prd.md` (769 lines, working tree — `git status` reports it ` M` against `HEAD`)
**Lens:** does every FR / RP / NFR status tag match what that requirement's consequences actually claim, verified against code at `HEAD` `1ac959e`
**Date:** 2026-07-25

---

## Method

Read `prd.md` in full. Extracted the tag of every requirement (22 FR, 2 RP, 8 NFR) with its line
number, then tested each tag against (a) §0's own definitions at `prd.md:22`–`24` and (b) the code
at real `HEAD`. Every claim below quotes a line I read this session and cites the file and line that
refutes or confirms it. Counts come from `grep`/`git grep`; the commands are named inline.

Two facts fix the baseline:

- `git rev-parse --short HEAD` → `1ac959e`. The PRD pins its verification baseline to `5972109`
  (`prd.md:673`, `prd.md:759`), which is **three commits behind** — `git log --oneline` shows
  `1ac959e`, `faa1a3e`, `3bd5b1a`, `5c8996f`, `a201fb0`, `5972109`.
- `git merge-base --is-ancestor a201fb0 HEAD` succeeds, so commit `a201fb0`
  ("fix(ui): use the palette's dark ink on bright accent fills") **is in `HEAD`**.

---

## Overall verdict

The tags are mostly honest, and the four tag decisions this Update made are individually defensible
— FR-17 and FR-18 are now correct, FR-14's split is correct. But the document applies **two
different rules to the same shape**: eight requirements were re-tagged `Partial` for exactly one
unbuilt limb (FR-4, FR-5, FR-7, FR-11, FR-13, FR-15, FR-17, FR-18) while three requirements keep
`Shipping` over an inline `Planned` limb (FR-9, RP-2) or a hybrid non-vocabulary tag (FR-19). §0
defines `Partial` as precisely that shape, so the inconsistency is not stylistic.

Separately, the single most consequential problem is not a tag at all: **three passages assert that
the 4.5:1 contrast floor fails and that its guard is uncommitted, and all three are false at `HEAD`.**
The fix landed in `a201fb0` at 12:31 today; the PRD was written at 15:14 against a SHA that was
already stale.

Direct answers to the four questions asked:

1. **Shipping tag over unbuilt behavior:** yes — FR-9 (`:293` vs `:300`), RP-2 (`:565` vs `:571`),
   NFR-3 (`:593` vs `:595`), and FR-19 (`:468` vs `:476`, where the consequence is *not* unbuilt but
   the FR says it is). The reverse direction — `Partial` over something that in fact ships — occurs
   at NFR-6, whose contrast limb now ships.
2. **`Partial` tags that do not name the unbuilt limb inline:** FR-7 (`installed → latest` in the
   plan is unbuilt and unnamed), FR-11 (the Manager-status badge and short description are unbuilt
   and unnamed), FR-5 (the second limb is named in the status line but the consequence that carries
   it, `:202`, has no inline marker while its two siblings do).
3. **RP-2:** not defensible as written. `:569` makes ⌘L's *meaning* part of the requirement, and the
   shipping handler does the one thing `:571` forbids. Should be `Partial`. Reasoning in F-5.
4. **FR-18:** `Partial` is correct and the D29 limb belongs in FR-18, not elsewhere. Reasoning in
   the "Confirmed correct" section.

**Counts:** 0 critical, 3 high, 6 medium, 1 low.

---

## Findings

### F-1 (high) — FR-19, NFR-6 and §7.1 all assert a contrast failure that does not exist at `HEAD`

**Location:** `prd.md:476` (FR-19 consequence), `prd.md:615` (NFR-6 status), `prd.md:673` (§7.1)

> `- Text contrast meets at least 4.5:1 on its surface. **Not met at `HEAD`** — see NFR-6.`
> — `prd.md:476`

> `**Status:** Partial. Non-color cues, reduced motion, the focus indicator, the size and zoom floors, and display-only VersionDelta all ship. **The 4.5:1 contrast floor does not hold at `HEAD`** — three bright-fill sites still paint white ink, measuring 2.46:1, 2.30:1 and 2.15:1. The fix and its automated guard exist only as uncommitted working-tree changes. The explanatory-disabled treatment on ineligible rows is likewise unbuilt (FR-5).`
> — `prd.md:615`

> `**Not in this list, deliberately:** the automated contrast guard. The 4.5:1 assertion and the on-fill ink tokens that make it pass are **uncommitted working-tree changes**, absent from `HEAD` `5972109`. Until they land, contrast at release time is a by-eye check, and neither FR-19 nor NFR-6 may be read as CI-guaranteed on that axis.`
> — `prd.md:673`

**Note.** All three were true at `5972109` and are false at `HEAD` `1ac959e`. Commit `a201fb0` is an
ancestor of `HEAD` (`git merge-base --is-ancestor a201fb0 HEAD` succeeds) and its message reads
"White text on --color-accent measured 2.30:1 against the WCAG 4.5:1 floor … Point the three
bright-fill surfaces at it". At `HEAD`:

- `git grep -c "text-white" HEAD -- src/` returns **no matching lines** — the three sites the PRD
  says "still paint white ink" do not exist.
- The three sites now consume the token: `src/components/primitives/Button.tsx:7`
  `primary: "bg-accent text-on-accent hover:bg-accent-hover disabled:bg-accent/40",`,
  `Button.tsx:13` `danger: "bg-danger text-on-accent hover:brightness-110 disabled:opacity-50",`,
  and `src/components/shell/UpdateStatusItem.tsx:63`
  ``className={`${CHIP} bg-accent text-on-accent hover:bg-accent-hover`}``.
- The guard is committed. `git grep -n "4.5" HEAD -- tests/e2e/browser-style-contract.spec.ts`
  returns `HEAD:tests/e2e/browser-style-contract.spec.ts:226` and `:320`; `:320` is
  `expect(measured.ratio).toBeGreaterThanOrEqual(4.5);`, and `:226` names the case
  `"[P0] paints bright accent fills with ink that clears the 4.5:1 contrast floor"`.

The damage is directional and reaches three readers. `bmad-create-epics-and-stories` is told to
schedule work that is done. Whoever reads §7.1 is told release-time contrast is "a by-eye check"
when CI now asserts it in-page against real WCAG relative luminance. And NFR-6's `Partial` tag is
carried half by a limb that no longer exists — it stays `Partial` only because of the FR-5 limb.

**Fix.** Rewrite all three: FR-19's contrast consequence loses "**Not met at `HEAD`**"; NFR-6's
status keeps `Partial` but on the FR-5 limb alone, moving contrast into the shipping list and
citing `tests/e2e/browser-style-contract.spec.ts:226`; §7.1 moves the contrast guard *into* the
shipping list. Re-stamp the baseline SHA at `:673` and `:759` to the SHA actually verified against.

---

### F-2 (high) — FR-7 is `Partial` but its unbuilt limb is a third one the status line does not name: `installed → latest` never renders in the plan

**Location:** `prd.md:263` (status), `prd.md:268` (consequence)

> `**Status:** Partial. Exact command preview, exclusions, and warnings ship. The persistent editable sidecar and the separate confirmation dialog are Planned — D27, D28.`
> — `prd.md:263`

> `- Every staged Package and Manager update appears in the plan before execution, grouped by Manager, showing `installed → latest`.`
> — `prd.md:268`

**Note.** §0:23 defines the contract this breaks: "**Partial** — some limbs ship; the rest is named
inline." Two limbs are named; a third is not, and `:268` carries no marker while `:272`, `:273` and
`:275` each carry an explicit `**Planned — D27:**` / `**Planned — D28:**`. A reader takes the
unmarked bullet as shipping.

It does not ship. The plan payload carries no version data:
`src-tauri/src/queue.rs:583`–`:598` builds each group from `commands`, `package_ids` and `locks`,
with each command mapped to `argv_preview` plus a `label` (`queue.rs:586`–`:589`), and the
self-update groups add only `label: format!("Self-update {}", m.display_name)` (`queue.rs:622`).
The sheet renders that and nothing else — `src/components/dialogs/UpgradePlanSheet.tsx:258`
`{cmd.argvPreview}`, with the excluded list showing a bare name at `:275`. `grep -n
"installed\|latest\|→" src/components/dialogs/UpgradePlanSheet.tsx` returns **zero lines**.

This limb is not cosmetic for this product: `installed → latest` in the review surface is the thing
that lets the user judge a staged change before admitting it, which is SM-2's subject.

**Fix.** Either add "per-item `installed → latest` in the plan" to FR-7's status line and mark `:268`
inline, or split `:268` so the grouped-by-Manager half stays unmarked and the version half carries a
`**Planned:**` prefix.

---

### F-3 (high) — FR-11 is `Partial` and certifies the Manager title area as shipping; two of the four elements it enumerates exist nowhere in `src/`

**Location:** `prd.md:322` (status), `prd.md:327` (consequence)

> `**Status:** Partial. The Manager title area and Route explanation ship. Independent removable membership is Planned — D27.`
> — `prd.md:322`

> `- Short description, executable path, installed version, and a Manager-status badge reading `NO UPDATES` or `UPDATE AVAILABLE`.`
> — `prd.md:327`

**Note.** `grep -rni "no updates" src/` returns **zero lines**. The only badge carrying that
vocabulary is the per-*Package* chip — `src/components/manager/StatusBadge.tsx:37`
`return <Chip tone="warning">Update available</Chip>;`, whose own header comment at `:2` calls it
"the per-row status pill (SPEC §4.8)". No Manager-scoped `NO UPDATES` / `UPDATE AVAILABLE` badge
exists. Nor does a short description: the Manager identity area is
`src/components/manager/SelfUpdateCard.tsx:66`–`:77` (an uppercase "Manager" label, the display
name, then either a `VersionDelta` or the bare version) and the workspace header
`src/components/manager/ManagerPane.tsx:157`–`:166` (name, `ManagedByChip`, version, "Refreshed …",
Refresh button). The executable path renders on the *dashboard* card
(`src/components/dashboard/ManagerCard.tsx:148`–`:150`) and in Settings
(`src/components/settings/SettingsView.tsx:242`–`:243`), not in the Manager title area this
consequence is about.

The sentence is target state lifted from `docs/SPEC.md:85`: "Each Manager title area shows a short
standardized description, executable path, installed version beside the name, and a Manager-status
badge reading `NO UPDATES` or `UPDATE AVAILABLE`." The PRD's own §0.1 exists to stop exactly this —
reconciling *back* to SPEC and inheriting its unbuilt claims as shipping.

**Fix.** Name the badge and the short description in FR-11's status line as unbuilt, or mark `:327`
inline. The installed-version and Route halves genuinely ship and should stay unmarked.

---

### F-4 (medium) — FR-9 is tagged `Shipping` and carries a `**Planned — D30:**` consequence; §0 calls that `Partial`

**Location:** `prd.md:293` (status), `prd.md:300` (consequence)

> `**Status:** Shipping.`
> — `prd.md:293`

> `- **Planned — D30:** only one confirmed Plan Attempt may be active at a time. A second confirmation fails closed while an attempt is unterminated, **independent of lock-set overlap** — two attempts touching disjoint Managers are still refused. Cross-Manager concurrency continues to occur *inside* the single active attempt. FR-8's lock-overlap rejection is a different test and does not imply this one.`
> — `prd.md:300`

**Note.** §0:22 defines `Shipping` as "implemented and verifiable in the current build", and §0:23
defines `Partial` as "some limbs ship; the rest is named inline" — which is literally FR-9's shape.
The unbuilt half is verifiable as unbuilt: §0:24's own inventory says `planAttemptId` returns zero
occurrences, and I confirmed it —
`grep -rn "planAttemptId\|plan_attempt_id\|Verifying\|InteractionRequired\|skipUpgradePlanConfirmation" src/ src-tauri/src/ | wc -l`
returns `0`. The shipping half is real (`src-tauri/src/queue.rs:47` documents the global cap and
`queue.rs:2398` is the test `semaphore_caps_concurrency_at_4`), so this is a two-limb requirement.

The inconsistency is what makes it a finding: this Update re-tagged FR-18 `Partial` for exactly one
inline `Planned — D29` limb (`prd.md:454`), and the prior pass re-tagged FR-4 and FR-5 `Partial` for
one limb each. FR-9 has the same shape and keeps `Shipping`. A downstream reader filtering by tag
gets a different answer for identical documents.

**Fix.** `**Status:** Partial. Atomic all-or-none admission, the lock-set scheduler and the
Homebrew-contention rule ship. The one-active-attempt rule is Planned — D30.`

---

### F-5 (medium) — RP-2: keeping `Shipping` with an inline `Planned` ⌘L limb misleads, and the tag should be `Partial`

**Location:** `prd.md:565` (status), `prd.md:569` (enumeration), `prd.md:571` (⌘L paragraph)

> `**Status:** Shipping.`
> — `prd.md:565`

> `… and must survive the same menu replacement: **⌘R** (refresh current, or all from the Dashboard), **⌘⇧R** (refresh all), **⌘⇧U** (Update Everything), **⌘L** (move focus into the Upgrade Plan sidecar region), **⌘F** (focus search), and **⌘1–9** (navigation jump).`
> — `prd.md:569`

> `**⌘L is a focus move, not a toggle. Planned — D27–D30 for the behavior; the registration ships.** … ⌘L moves focus into the region, and when the region is hidden it is a **no-op** — it must not conjure the region into existence. The shipping handler instead toggles the `ActivityDrawer` (`src/hooks/useKeyboard.ts:164`–`166`, `toggleDrawer()`) …`
> — `prd.md:571`

**Note.** The citation is exact — `src/hooks/useKeyboard.ts:164`–`167` reads
`case "l": / e.preventDefault(); / useUiStore.getState().toggleDrawer(); / break;`. So the code is
correctly described; the question is only whether `Shipping` survives it. It does not, for three
reasons:

1. **The requirement is not "a key is registered".** `:569` defines ⌘L's meaning inside the
   enumeration — "(move focus into the Upgrade Plan sidecar region)" — and `:571` states the
   behavior normatively, including a prohibition ("it must not conjure the region into existence").
   The shipping handler both fails the requirement and does the prohibited thing to the surface it
   currently owns. Under §0:22 ("implemented and verifiable in the current build") that limb is not
   Shipping.
2. **The document already has the word for it.** §0:23 — "some limbs ship; the rest is named inline"
   — describes RP-2 exactly. Every other requirement with one inline-named unbuilt limb was moved to
   `Partial`; RP-2 and FR-9 are the two exceptions, and nothing in the document explains the
   difference.
3. **The `Shipping` tag hides the work from the only place it would be scheduled.** §7.2 calls
   itself "the live build queue" (`prd.md:685`) and its five bullets (`prd.md:679`–`:683`) never
   mention ⌘L or the drawer's retirement. So the ⌘L re-point exists in exactly one place in this
   PRD — a paragraph under a `Shipping` tag, inside §4.6, whose preamble says these two "are
   validated through `docs/RELEASE-CHECKLIST.md`" (`prd.md:550`). Tag-first readers see a checklist
   item, not a story.

The narrow defense — that RP-2's subject is "accelerators survive `app.set_menu`", which does ship —
is undercut by `:569` itself: if only survival mattered, the parenthetical meanings would not be
part of the requirement text.

**Fix.** `**Status:** Partial. The Edit and Window submenus, and every accelerator's registration,
survive `app.set_menu`. ⌘L's behavior is Planned — D27–D30: the shipping sink toggles the
`ActivityDrawer`, which retires under AD-17.` Keep `:571` verbatim — it is the best-argued paragraph
in §4.6 and the fix is only to the tag above it. Optionally add the ⌘L re-point to §7.2 so it
appears in the build queue.

---

### F-6 (medium) — FR-19's status is not one of §0's three tags, and reads `Shipping` over a consequence the FR itself marks unmet

**Location:** `prd.md:468`

> `**Status:** Shipping for the current navigation model. The D30 navigation changes — Activity as a first-class destination, the Results surface, and one-plan-per-row History — are Planned.`

**Note.** §0:20 says every FR "carries one of" the three tags at `:22`–`:24`. This is a fourth,
scope-qualified variant. Mechanically it *is* `Partial` — some limbs ship, the rest is named inline —
and the qualifier adds nothing that `Partial` plus the same sentence would not. FR-13 handles the
identical D29/D30 split correctly (`prd.md:356`, "**Status:** Partial. Live streaming, exact command
visibility, and `opId` correlation ship. Plan-level state, `planAttemptId` correlation, Activity as a
first-class destination, and the Results summary are Planned — D29, D30."), so the document contains
both treatments of the same fact.

Compounding it, FR-19's consequence at `:476` says the contrast floor is "**Not met at `HEAD`**"
under a status whose first word is "Shipping" — the exact Shipping-over-unbuilt shape §0 forbids.
(That the underlying fact is now false is F-1; the tag shape is a separate defect and survives F-1's
fix only if the status is also normalized.)

Verification that the Planned half is genuinely Planned: `src/store/ui.ts:13`–`:16` defines the whole
navigation union as `dashboard | manager | history | settings` — no Activity destination — and
`ui.ts:19`–`:26` defines the whole dialog union as `none | upgradePlan | stalled | quitGuard` — no
Results surface.

**Fix.** `**Status:** Partial.` followed by the same two sentences.

---

### F-7 (medium) — NFR-3 is tagged `Shipping` while promising the responsiveness of three surfaces that do not exist

**Location:** `prd.md:593` (status), `prd.md:595` (text)

> `**Status:** Shipping.`
> — `prd.md:593`

> `… Navigation, the plan, confirmation, Activity, Results, and recovery all remain usable at 900 × 600 and at 150–200% zoom.`
> — `prd.md:595`

**Note.** "Results" does not exist — `grep -rn "Results" src/` (excluding tests) returns nothing, and
`src/store/ui.ts:19`–`:26` bounds the dialog union at `none | upgradePlan | stalled | quitGuard`.
"Activity" exists only as the `ActivityDrawer` that AD-17 retires, not as the destination FR-13 and
FR-19 tag Planned. The separate "confirmation" is FR-7's `**Planned — D28:**` limb (`prd.md:273`).
The rest of NFR-3 verifies: `src/store/operations.ts:17` `export const LOG_CAP = 5000;` matches the
5,000-line claim, and `src/components/manager/PackageTable.tsx:15` `const VIRTUALIZE_ABOVE = 100;`
(used at `:123`) backs the 101-rows clause.

This was raised in the pre-Update gate (`review-status-tags.md:453`, F-9, rated low) and was not
applied. It is worth re-raising at medium now that FR-9, FR-19 and RP-2 show the same pattern: four
requirements tagged `Shipping` over Planned surfaces is no longer an isolated nit but the document's
de-facto second convention.

**Fix.** Either scope the sentence to shipping surfaces and move the rest into a `Planned — D30`
clause, or tag NFR-3 `Partial` naming those three.

---

### F-8 (medium) — FR-5's second unbuilt limb is named in the status line but the consequence carrying it has no inline marker, while both its siblings do

**Location:** `prd.md:194` (status), `prd.md:202` (consequence)

> `**Status:** Partial. Browsing, search, filtering, and every eligibility rule ship. Two limbs do not: outdated-first ordering, and the explanatory-disabled treatment on ineligible rows — both named inline below.`
> — `prd.md:194`

> `- Up-to-date and otherwise ineligible Packages cannot enter the Upgrade Plan and expose a plain-language reason on pointer interaction. Ineligibility never relies on gray styling alone.`
> — `prd.md:202`

**Note.** The status line promises both limbs are "named inline below". Two bullets carry markers —
`:200` ends with "**Planned:** the shipping row uses native `disabled` plus reduced opacity, which
both blocks the explanation and leans on gray styling alone", and `:205` opens with "**Planned:**"
for the ordering. `:202` carries none, yet it states the same explanatory-disabled requirement over
a *wider* row set (up-to-date and otherwise ineligible, not just Pinned), and NFR-6 confirms it is
unbuilt: `prd.md:615` "The explanatory-disabled treatment on ineligible rows is likewise unbuilt
(FR-5)."

The code shows one mechanism serving both bullets, and it is the one `:200` rules out.
`src/components/manager/PackageRow.tsx:69`–`:75` computes
`const checkboxTitle = pkg.pinned ? … : !pkg.outdated ? "Already up to date" : …` and hangs it on the
checkbox at `:95` `title={checkboxTitle}` — a control rendered `disabled={checkboxDisabled}` at `:92`
with `"disabled:cursor-not-allowed disabled:opacity-40"` at `:100`. By `:200`'s own argument ("a
natively disabled control cannot receive the pointer interaction this consequence requires"), the
`:202` explanation is blocked by the same defect.

A story author generating acceptance criteria bullet-by-bullet — which is what
`bmad-create-epics-and-stories` does — will mark `:200` and `:205` as D27 work and `:202` as already
satisfied.

**Fix.** Add a `**Planned:**` clause to `:202` pointing at the same treatment `:200` requires, or
merge the two bullets so one marker governs both.

---

### F-9 (medium) — FR-6's `Consequences (testable)` list contains a bullet stating pre-D27 behavior in requirement voice, contradicting the bullet two lines above it

**Location:** `prd.md:234` and `prd.md:236`

> `- The draft persists while the user navigates between Managers and the Dashboard, and every staged item is individually removable from the Upgrade Plan.`
> — `prd.md:234`

> `- The draft is transient dialog state discarded on close — pre-D27 behavior this FR removes.`
> — `prd.md:236`

**Note.** Both sit in the same list under the heading `**Consequences (testable):**` (`prd.md:229`)
in an FR tagged `**Status:** Planned — D27.` (`prd.md:225`). Read as written they are a direct
contradiction: the draft persists across navigation, and the draft is discarded on close. The
trailing clause on `:236` is the only thing marking it as an anti-requirement, and it is the sole
place in the document where a removed behavior is stated in the same voice and list position as a
requirement — everywhere else the convention is a leading `**Planned:**` / `**Out of Scope:**`
marker, and FR-6 itself has an `**Out of Scope:**` block at `:243` that is where this belongs.

`src/store/packages.ts:17` still declares `selection: Partial<Record<ManagerId, Set<string>>>;` — the
set the PRD calls "pre-D27 code" (`prd.md:735`) — so the sentence is a true description of today. It
is just not a consequence of FR-6.

**Fix.** Move `:236` into FR-6's `**Out of Scope:**` block at `:243`–`:245`, whose first bullet
(`:244`) already reads "A transient selection distinct from draft membership, and any `Add Selected`
submit step. Both are eliminated."

---

### F-10 (low) — the document's verification baseline is a SHA that is no longer `HEAD`, and it is cited as `HEAD` twice

**Location:** `prd.md:673` and `prd.md:759`

> `… are **uncommitted working-tree changes**, absent from `HEAD` `5972109`.`
> — `prd.md:673`

> `… or verification against `src/` and `src-tauri/` at `HEAD` `5972109`.`
> — `prd.md:759`

**Note.** `git rev-parse --short HEAD` returns `1ac959e`. `5972109` is three commits back, and one of
the intervening commits (`a201fb0`) invalidates a claim the PRD makes in three places — see F-1. The
pinning itself is good practice; the defect is calling a non-`HEAD` SHA "`HEAD`", which makes every
"at `HEAD`" assertion in the document silently time-bound without saying so.

**Fix.** Write "verified at `5972109`" rather than "`HEAD` `5972109`", and re-verify the contrast
passages against the SHA actually current when the Update ran.

---

## Confirmed correct (recorded so the next reviewer does not repeat the work)

**FR-18 is correctly `Partial`, and the D29 limb belongs here — not elsewhere.** (`prd.md:454`,
`prd.md:464`.) Three reasons:

1. The shipping half of the status line is accurate. `src-tauri/src/diagnostics.rs:7` describes the
   bundle as "files, the last 25 transcripts, and `operations.jsonl`"; `:144` adds
   `add_file(&mut zip, "operations.jsonl", journal_path)?;`, and the test at `:190`
   (`export_bundles_report_logs_transcripts_and_journal`) asserts `:251`
   `assert_eq!(transcript_names.len(), 25, "last 25 transcripts");` and `:238` that the archive
   contains `operations.jsonl`. So "every content enumerated below ship" holds.
2. The D29 limb is a genuinely separate requirement, not a restatement of FR-15's. FR-15 owns
   whether attempt records *exist*; FR-18 owns what the *bundle* carries and in what form — and
   `:464`'s substantive content ("raw lines", "the two-record *set*", "a folded attempt view may be
   added as an **additional** entry, marked as derived") is a constraint on the export that no other
   FR states. Moving it to FR-15 would put an export rule in a History requirement.
3. It matches how the document handles the same journal everywhere else: FR-13 (`prd.md:356`) and
   FR-15 (`prd.md:387`) are both `Partial` with D29 limbs for the same unbuilt identity. Leaving
   FR-18 `Shipping` would have been the outlier, and §9.2 records why that was a defect: "an
   implementer could ship an archive with no plan-attempt records and believe the requirement met"
   (`prd.md:751`).

**FR-17's re-tag and its retirement wording are correct.** `src-tauri/src/settings.rs:28`–`:39`
defines exactly eight fields — `run_brew_update_on_refresh`, `auto_refresh_on_launch`,
`stall_after_secs`, `upgrade_hard_cap_mins`, `log_level`, `auto_open_drawer`,
`include_greedy_by_default`, `auto_check_for_updates` — matching the eight-row table at
`prd.md:432`–`:441` and its defaults at `settings.rs:43`–`:52`.

**FR-14's split is correct.** `prd.md:380`'s "**Not yet built:**" is verified: `QuitGuardDialog`
exists (`src/components/dialogs/QuitGuardDialog.tsx:29`) and is routed by `DialogHost.tsx:20`, but
`grep -rn "QuitGuard\|onCloseRequested\|close-requested\|CloseRequested" src/ src-tauri/src/` shows
its only caller is `src/components/shell/UpdateStatusItem.tsx:33` — the app-update path, exactly as
the consequence says.

**FR-8 `Shipping` is correct**, including the consequence added in the prior pass. `prd.md:289`'s
"Final confirmation is unavailable while a plan rebuild is in flight and after a rebuild failure"
is implemented — `src/components/dialogs/UpgradePlanSheet.tsx:343`
`disabled={submitting || planReadiness !== "ready" || !hasCommands}`, with the file header at `:5`–`:7`
stating "Confirm calls `execute_plan` with the currently-displayed, ready … so a pending or failed
rebuild can never execute an older preview."

**FR-6's two corrected passages check out against code.** The Esc citation `prd.md:239`
(`src/hooks/useKeyboard.ts:70`–`78`) is exact — `:66`–`:69` is the close-dialog rung, `:70`–`:77` the
clear-selection rung, `:78` `if (ui.drawerOpen) ui.setDrawerOpen(false);`. The three-site divergence
table at `prd.md:253`–`:255` is exact — `useKeyboard.ts:34` does read
`/** Visible + selectable package ids for a manager (mirrors ManagerPane filters). */`.

**FR-13's `cancelling` removal is correct.** `grep -rn "cancelling" src/ src-tauri/src/` (excluding
tests) returns only prose comments — no status value.

**Unchallenged tags** (spot-verified, no defect found): FR-1, FR-2, FR-3, FR-10, FR-12, FR-14,
FR-15, FR-16, FR-17, FR-20, FR-21, FR-22, RP-1, NFR-1, NFR-2, NFR-4, NFR-5, NFR-7, NFR-8.
