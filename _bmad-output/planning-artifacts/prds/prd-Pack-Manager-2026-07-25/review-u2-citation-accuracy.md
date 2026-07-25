# Review — Citation Accuracy (Update pass 2)

**Target:** `_bmad-output/planning-artifacts/prds/prd-Pack-Manager-2026-07-25/prd.md`
**Axis:** citation accuracy only — every `AD-` id, every `docs/DECISIONS.md` D-number, every
file-plus-line-number citation, and over-claiming in the 13 `**Architecture binding:**` blocks.
**Method:** each cited location opened and read this session; every quoted string compared
byte-for-byte against the source; code claims checked against `HEAD` with `git grep` / `git
merge-base` rather than the working tree.
**Reviewed:** 2026-07-25. **Repo `HEAD`:** `1ac959e`.

---

## Overall verdict

**The architecture-citation layer is sound; the code-state layer is stale.**

All 13 `**Architecture binding:**` blocks plus the AD-12 narrowing note at `:544` are accurate.
Every quoted spine sentence I checked is literal, including the two multi-line quotes in §9.2 and
the AD-24 ellipsis at `:239`. No AD is over-claimed: where the PRD says an AD "binds" or "owns"
something, the spine's own `Binds` line or rule text covers it, and in five cases (AD-28 on FR-5's
ordering, on RP-2's accelerator map, on NFR-3's 101-rows sentence; AD-29 on NFR-4's transcript
asymmetry; AD-30 on FR-21's active set) the spine names the PRD requirement back by id. `prd.md:28`'s
claim holds: the eight AD ids cited (AD-12, 17, 23, 24, 27, 28, 29, 30) all exist in revision 10, and
every D-number cited (D1, D2, D3, D6, D10, D15, D19–D22, D25, D25a, D26–D33, D35–D37) exists in
`docs/DECISIONS.md`.

What fails is the other direction. Three passages assert that D36's contrast fix and its CI guard are
**uncommitted**, two commits after they landed — and the PRD cites D36 by name four lines from one of
them. One spine citation (`:1050`) has already rotted and now points at a blank line inside AD-25,
asserting an OPEN row that revision 10 marks **RESOLVED**. One reconciliation queue names two files
whose D37 work is already done. All three defects share one root cause: the PRD pins its code
verification to `HEAD` `5972109` while citing decisions recorded three commits later.

**Counts:** 1 critical, 2 high, 1 medium, 3 low.

---

## CRITICAL

### C-1. D36's contrast fix and its CI guard are claimed uncommitted; both are in `HEAD`, and D36 — cited on the facing page — says so

**Locations:** `prd.md:476` (FR-19), `prd.md:615` (NFR-6), `prd.md:673` (§7.1). Corroborating
defect in `addendum.md` §4 item 4.

**Verbatim offending text**

`prd.md:673`:
> **Not in this list, deliberately:** the automated contrast guard. The 4.5:1 assertion and the
> on-fill ink tokens that make it pass are **uncommitted working-tree changes**, absent from `HEAD`
> `5972109`. Until they land, contrast at release time is a by-eye check, and neither FR-19 nor
> NFR-6 may be read as CI-guaranteed on that axis.

`prd.md:615`:
> **The 4.5:1 contrast floor does not hold at `HEAD`** — three bright-fill sites still paint white
> ink, measuring 2.46:1, 2.30:1 and 2.15:1. The fix and its automated guard exist only as
> uncommitted working-tree changes.

`prd.md:476`:
> - Text contrast meets at least 4.5:1 on its surface. **Not met at `HEAD`** — see NFR-6.

**What the cited sources actually say**

`docs/DECISIONS.md` D36:
> All three sites move to `text-on-accent`; no `text-white` remains in `src/`.

and, in the same decision:
> So the fix carries a guard: `tests/e2e/browser-style-contract.spec.ts` now reads the rendered
> primary button's real computed foreground and background, applies the luminance formula, and
> fails below 4.5:1, plus a negative assertion against `text-white` returning. Green on Chromium
> and WebKit.

`ARCHITECTURE-SPINE.md:1566` (revision 10, the revision this PRD stamps):
> **Extended 2026-07-25 by `docs/DECISIONS.md` D36 (commit `a201fb0`):** … All three now resolve
> `text-on-accent` and no `text-white` remains in `src/`. The fix carries its own guard in the same
> style-contract lane, computing real WCAG luminance from the rendered pair rather than trusting
> token names …

**Verified against `HEAD` `1ac959e`, not the working tree:**

| Command | Result |
| --- | --- |
| `git merge-base --is-ancestor a201fb0 HEAD` | `YES` — `a201fb0 fix(ui): use the palette's dark ink on bright accent fills` is an ancestor of `HEAD` |
| `git grep -n "text-white" HEAD -- src/` | no matches |
| `git grep -n "text-on-accent" HEAD -- src/` | `Button.tsx:7`, `Button.tsx:13`, `UpdateStatusItem.tsx:63` — the exact three sites D36 names |
| `git grep -n … HEAD -- tests/e2e/browser-style-contract.spec.ts` | `:226` `test("[P0] paints bright accent fills with ink that clears the 4.5:1 contrast floor"` and `:320` `expect(measured.ratio).toBeGreaterThanOrEqual(4.5);` |

**Note.** The PRD cites D36 as live authority at `:489` — "**Contrast** (D36's guard). It caught text
unreadable to anyone looking at the button" — while three other passages assert that guard does not
exist in the tree. Both cannot be true. D36 and the spine agree, and the repository agrees with them.

The damage is not cosmetic. `prd.md:673` puts the guard in an explicit *exclusion* list under
§7.1 "Shipping today", and `prd.md:615` demotes NFR-6 to Partial on that basis. Read literally by
`bmad-create-epics-and-stories`, it schedules a story to write a test that already exists and to
replace `text-white` occurrences that are already gone — which is exactly the failure the spine
flags against `epics.md` at `:1594`: "Commit `a201fb0` landed it. A builder reading either schedules
shipped work, which is the exact error AD-1's second rule forbids." The PRD warns against this same
habit itself at `:689` ("verify whether each already ships before scheduling it as new work") and
then commits it.

**Fix.**
1. `prd.md:476` — delete "**Not met at `HEAD`** — see NFR-6."; the consequence stands unqualified.
2. `prd.md:615` — retag NFR-6's contrast limb. The bright-fill measurements and the "uncommitted
   working-tree changes" sentence go; state instead that the floor is CI-asserted by
   `tests/e2e/browser-style-contract.spec.ts` under D36. If NFR-6 stays **Partial**, it should be
   Partial on the surviving limb only — the explanatory-disabled treatment on ineligible rows
   (FR-5), which is genuinely unbuilt.
3. `prd.md:673` — delete the "Not in this list, deliberately" paragraph and add the contrast guard
   to the §7.1 Shipping list beside the focus mechanism it lives next to in the same spec file.
4. `addendum.md` §4 item 4 carries the same false claim and should be struck or marked resolved by
   `a201fb0` in the same edit.

---

## HIGH

### H-1. `ARCHITECTURE-SPINE.md:1050` is cited as an OPEN row; the line is blank, and the row it means reads **RESOLVED**

**Location:** `prd.md:740`, §9.1 "Closed during this run".

**Verbatim offending text**
> - `ARCHITECTURE-SPINE.md:1050` still records this as OPEN and routes it to the owner. It is now
>   answered, and that row should be **retired** in the `bmad-architecture` Update that follows this
>   PRD.

**What is actually at the cited location.** `ARCHITECTURE-SPINE.md:1050` is blank. It sits between
AD-25's last rule (`:1046`–`:1049`, "Health and staleness presentation derive from the snapshot's
real timestamp…") and AD-26's heading at `:1051` ("### AD-26 — A native automation surface never
reaches release bits"). Nothing about transient selection, nothing OPEN, no owner routing.

**What the spine actually says about that question.** The row exists at `ARCHITECTURE-SPINE.md:1595`
and reads:
> | Transient selection has no owning invariant | **RESOLVED** | Closed by **AD-28**, written
> 2026-07-25 on the owner's decision: `EXPERIENCE.md`'s model wins, the checkbox *is* membership,
> and the transient selection plus `Add Selected` layer is eliminated.

**Note.** The status is inverted, the line number is wrong, and the recommended action is already
discharged. `bmad-architecture` is the named consumer of §9; told to retire a row in "the Update
that follows this PRD", it would open revision 10, find the row already closed, and have to decide
whether the PRD is describing a spine it never read or a state that no longer exists. The same
paragraph is otherwise excellent — the `.memlog.md:75` quote at `prd.md:733` is verbatim and the
line is exact, and `review-reconcile-epics.md:166` at `prd.md:739` is exact.

**Fix.** Replace the bullet with a closure record rather than an instruction, citing by name:
"`ARCHITECTURE-SPINE.md`'s *Transient selection has no owning invariant* row is **RESOLVED** in
revision 10, closed by AD-28. No follow-up is owed to `bmad-architecture`." Do not re-cite a spine
line number (see L-3).

### H-2. The D37 reconciliation queue names four artifacts; two of them are already reconciled

**Location:** `prd.md:493`, FR-19 Notes.

**Verbatim offending text**
> `epics.md`, `ARCHITECTURE-SPINE.md`, `EXPERIENCE.md`, and `docs/RELEASE-CHECKLIST.md` still carry
> the removed obligations. The first three are workflow-owned and come out through
> `bmad-correct-course`, a `bmad-architecture` Update, and a `bmad-ux` Update — never a hand edit.
> The fourth is workflow-unowned and is a maintainer edit.

**What the cited sources actually say.**

`docs/RELEASE-CHECKLIST.md:102`–`:106` — read this session, already applied:
> **Keyboard navigation and screen-reader support are explicitly not release criteria.**
> Pack-Manager is a personal, single-user, mouse-driven utility. The former "Tab and arrow
> navigation reach every control" pass and the VoiceOver pass over the Upgrade Plan were
> removed deliberately on 2026-07-25 — they are not oversights, and a future
> regeneration or review should not reinstate them. Do not report their absence as a gap.

`git log --oneline -3 -- docs/RELEASE-CHECKLIST.md` → `5c8996f docs: record D36 and D37, and rescope
the release checklist`, an ancestor of `HEAD`.

`ARCHITECTURE-SPINE.md:1598` (revision 10):
> D37's "Not yet applied" counts were copied rather than measured and are wrong in all three
> columns; revision 10 **is** the `bmad-architecture` Update D37 names there, so the spine's limb is
> now discharged.

`ARCHITECTURE-SPINE.md:1600` names the survivors, and names only two:
> `epics.md` and `EXPERIENCE.md` still carry the removed obligations and come out through
> `bmad-correct-course` and a `bmad-ux` Update — never a hand edit.

**Note.** D37's own "Not yet applied" list — `epics.md` (10 mentions), `ARCHITECTURE-SPINE.md` (3),
`EXPERIENCE.md` (4) — never included the checklist, because D37 *is* the checklist edit. The PRD
added a fourth file that was already done and kept a third that revision 10 discharged. The instruction
to run a `bmad-architecture` Update for D37 removal is a request to re-run the pass that produced the
revision this PRD stamps at `:28` and `:759`. Per D37's own closing warning and the spine's
`:1600` retired-criteria row, "a regeneration or review pass that reports it as a gap is repeating a
mistake D37 names by example."

**Fix.** Reduce the list to `epics.md` and `EXPERIENCE.md`, with `bmad-correct-course` and a
`bmad-ux` Update as their two routes. Record the other two as done: "`ARCHITECTURE-SPINE.md`
revision 10 is the `bmad-architecture` Update D37 names, and `docs/RELEASE-CHECKLIST.md` was
rescoped in commit `5c8996f`." Mirror the change in `addendum.md` §3 if the queue is duplicated there.

---

## MEDIUM

### M-1. The stated code-verification baseline is three commits behind the decisions the PRD cites

**Locations:** `prd.md:673`, `prd.md:759`.

**Verbatim offending text** (`prd.md:759`)
> Every requirement in this document traces to a named source — `docs/SPEC.md`, `docs/DECISIONS.md`
> D1–D37, `epics.md` FR/NFR lines 53–450, `ARCHITECTURE-SPINE.md` revision 10, `EXPERIENCE.md`,
> `DESIGN.md`, or verification against `src/` and `src-tauri/` at `HEAD` `5972109`.

**What the repository says.** `git log --oneline` gives, newest first: `1ac959e`, `faa1a3e`,
`3bd5b1a`, `5c8996f`, `a201fb0`, `5972109`. The stated baseline is the sixth commit back. Two of the
commits after it are load-bearing for this document: `a201fb0` landed the contrast fix (C-1 above),
and `5c8996f` is where **D36 and D37 were recorded** — the two decisions the PRD leans on hardest at
`:483`, `:489` and `:615`. The PRD therefore reads decisions from a commit its code baseline
predates, which is the mechanism that produced C-1.

Independently, `faa1a3e chore(planning): restore the Phase 2 PRD and archive completed specs` — the
commit that introduced this file — is itself later than both, so the document was committed into a
tree where its own baseline claim was already stale.

**Note.** Not every code claim rotted: `planAttemptId`, `plan_attempt_id`, `Verifying`,
`InteractionRequired` and `skipUpgradePlanConfirmation` still return zero files across `src/` and
`src-tauri/src/` (`prd.md:24`, re-run this session), `settings.rs` still has exactly 8 `Settings`
fields with `auto_check_for_updates: true`, `lib.rs` still registers 20 commands, and `events.rs`
still defines 6 events. The three `useKeyboard.ts` / `ManagerPane.tsx` citations are all still
correct at `HEAD`. The baseline is stale in one axis only — but it is the axis two status tags
depend on.

**Fix.** Re-stamp both places to `HEAD` `1ac959e` and re-run the two claims that move with it
(contrast, per C-1). If the intent was to freeze verification at a point in time, say which commits
were excluded and why — a bare stale SHA reads as current.

---

## LOW

### L-1. The AD-28 self-contradiction quote is cited to a line range that stops one line short of it

**Location:** `prd.md:753`, §9.2.

**Verbatim offending text**
> `ARCHITECTURE-SPINE.md` AD-28 contradicts itself on the `Esc` cascade inside a single bullet:
> `:1272`–`1273` says "the cascade drops from three rungs to two and keeps close-dialog"

**What is at the cited location.** The quoted sentence is verbatim, but it spans `:1273`–`:1274`, not
`:1272`–`:1273`. `:1272` reads "- **Rule:** `Esc` never touches membership. Its clear-selection rung
is **deleted,"; `:1273` reads "not re-pointed** — the cascade drops from three rungs to two and
keeps"; the word `close-dialog` is the first token of `:1274`. The companion citation in the same
sentence, `:1276`–`1278`, is exact, and the AD-17 citation `:774` is exact.

**Note.** The substance is right — AD-28 does contradict itself in that bullet, and "FR-6 is correct
and stays as written" is the correct disposition. Only the range is off by one at both ends.

**Fix.** Cite by subject: "AD-28's `Esc` rule contradicts itself inside one bullet — its first clause
says … while its own correction four lines later says …". See L-3.

### L-2. `useKeyboard.ts:70`–`78` is cited for a three-rung cascade whose first rung is outside the range

**Location:** `prd.md:239`, FR-6.

**Verbatim offending text**
> The shipping cascade is close-dialog → clear-selection → close-drawer
> (`src/hooks/useKeyboard.ts:70`–`78`).

**What is at the cited location.** `handleEscape` spans `:64`–`:79`. The close-dialog rung is
`:66`–`:69` (`if (ui.dialog.kind !== "none") { ui.closeDialog(); return; }`) — outside the cited
range. `:70`–`:77` is the clear-selection rung and `:78` is `if (ui.drawerOpen)
ui.setDrawerOpen(false);`. The range names two of the three rungs it enumerates.

**Note.** Behaviour described is exactly right; this is a range that should have started at `:64`.
The neighbouring code citations in the same FR are precise: `:35`–`:53` is exactly
`visibleSelectableIds`, the comment at `:34` reads verbatim "mirrors ManagerPane filters",
`ManagerPane.tsx:92`–`107` covers `anyOutdated`/`outdatedOnly`/`matchesSearch`/`visibleMain`/
`orderedSelectable` in that order, and `ManagerPane.tsx:138`–`141` is `upgradeAll()` filtering
`mainPackages` rather than `visibleMain` — the divergent third copy, correctly characterised.

**Fix.** Change to `src/hooks/useKeyboard.ts:64`–`78` (or cite `handleEscape` by name).

### L-3. Four line-number citations into the spine violate the spine's own Citations convention, and one has already rotted

**Locations:** `prd.md:740` (`:1050`), `prd.md:753` (`:1272`–`1273`, `:1276`–`1278`, `:774`).

**What the cited source says.** `ARCHITECTURE-SPINE.md:1496`, the Citations convention row:
> Cite by **name**, never by position. No line numbers into a document under edit, no `AD` rule
> ordinals, no bare counts in a status row, and no pre-squash commit SHAs. Each of those has rotted
> here — rule ordinals when a rule was inserted, `epics.md` line numbers twice, this spine's own
> line numbers, a count that went stale inside the revision that wrote it, and commit hashes that
> resolve locally but sit on no branch after a squash merge.

**Note.** H-1 is this convention's prediction coming true inside one revision. The remaining three
are correct today but sit in a document that is explicitly still under edit — the spine's own
`epics.md` residuals row records `grep -c 'ARCHITECTURE-SPINE.md[:#]*[0-9]'` returning **0** for
`epics.md` as clearance evidence that line-number citations were removed there; this PRD reintroduces
four. The stale `HEAD` SHA in M-1 is the same convention's "pre-squash commit SHAs" clause.

**Fix.** Convert all four to AD id plus rule subject (e.g. "AD-17's `Esc` rule", "AD-28's `Esc`
rule and its correction in the same bullet"). Where a status row is meant, cite the row title.

---

## Confirmed — checked and correct

Recorded so a later pass does not re-open them.

**All 13 `**Architecture binding:**` blocks plus the AD-12 narrowing note.** Each was checked
against the full AD text, not the PRD's framing:

| PRD | AD | Verdict |
| --- | --- | --- |
| `:208` FR-5 ordering | AD-28 | Correct. AD-28 `:1221`–`:1228` names FR-5's unbuilt ordering by id and states the ordered-filtered-set-vs-DOM-window rule the PRD reproduces. |
| `:241` FR-6 | AD-28 | Correct on all eight limbs listed. AD-28's title is quoted verbatim from `:1181`. |
| `:274` FR-7 removal taxonomy | AD-28 | Correct, including the five-item scope-wide list (`:1248`–`:1259`) and the "never of refs that held none" clause. |
| `:304` FR-9 | AD-29 | Correct. Mint-and-admit-then-append `:1385`; append gates nothing `:1390`–`:1396`; liveness rule `:1442`–`:1448`. |
| `:367` FR-13 | AD-29 | Correct. Operation-journal `Verifying`/`Skipped` `:1380`; terminal record carries results `:1369`–`:1376`; exactly two records `:1359`. |
| `:382` FR-14 | AD-30 | Correct. One enforcement point `:1457`–`:1462`; `Queued` ∪ `Running` `:1463`–`:1468`; no-rollback `:1476`–`:1479`. |
| `:400` FR-15 | AD-29 | Correct. Idempotent fold keyed by `planAttemptId` `:1425`–`:1430`; genuine-absence rule `:1415`–`:1424`. |
| `:464` FR-18 D29 limb | AD-29 | Correct. Raw lines, record *set*, derived-view-as-additional-entry all at `:1397`–`:1405`. |
| `:530` FR-21 | AD-30 | Correct. "the two may not drift apart" `:1467`–`:1468`; "children never outlive the app" `:1475`. |
| `:542` FR-22 | AD-12 | Correct. `attach_to_tag` clause verbatim at `:428`–`:429`. |
| `:544` FR-22 narrowing | AD-12 | Correct. Field list matches `:411`–`:418`; "**Everything else in those files stays maintainer-owned.**" verbatim `:419`; "unable to rotate its own signing key" verbatim `:420`–`:421`. |
| `:573` RP-2 | AD-28 | Correct. Binding list `:1183`–`:1184`; complete accelerator map `:1281`–`:1285`; native-default rule `:1317`–`:1324`. |
| `:597` NFR-3 | AD-28 | Correct. AD-28 `:1211`–`:1213` quotes the 101-rows sentence by name. |
| `:605` NFR-4 | AD-29 | Correct. Transcript/append asymmetry `:1390`–`:1396` (cites NFR-4 by name); no persisted `PlanAttempt.state` `:1431`–`:1441`. |
| `:633` NFR-8 | AD-12 | Correct. `pubkey` in no `extra-files` path `:423`–`:426`. |

**Quoted spine and decision text — all literal.** AD-24 at `prd.md:239` ("Admission of the draft's
own preview empties it as custody transfer … no other path adds, replaces, or clears membership") is
verbatim across `:994`–`:996` with a legitimate ellipsis. AD-17 at `prd.md:448` and `prd.md:571` is
verbatim `:763`–`:764`. AD-30 at `prd.md:744` ("**Queued counts as running.** This is not an open
question") is verbatim `:1463`. AD-28's second `Esc` clause at `prd.md:753` is verbatim
`:1276`–`:1278`.

**`docs/DECISIONS.md` citations.** `:193` is exactly "obsolete `autoOpenDrawer` setting becomes
inactive legacy input" (`prd.md:448`). D25a's sixth-event and superseded-D20 clauses back
`prd.md:41` and `prd.md:44`. D33 supplies "28 real product stories", "1 star, 0 forks … 3 lifetime
`.dmg` downloads", "the other 27 of the 30 recorded downloads", and "overturned 14 of 20 initial
keeps" — all verbatim at `prd.md:71`, `:685`, `:689`, `:698`. D36's three ratios and D37's three
protected items back `prd.md:487`–`:489`. No D-number cited is absent from the decision record.

**Other file-and-line citations.** `ux-Pack-Manager-2026-07-23/.memlog.md:75` (`prd.md:733`) is
verbatim and on the exact line. `review-reconcile-epics.md:166` (`prd.md:739`) is exact.
`epics.md:89` is FR-19 with "keyboard/VoiceOver operable", `epics.md:113` is NFR-6 with "announce
plan progress" and "deterministic dialog/sidecar focus restoration" — both as `prd.md:483`
describes. `EXPERIENCE.md` `373`–`460` opens on "# Key Flows" and closes on the AJ-6 manual-install
paragraph. `docs/SPEC.md` `:19`, `:108`, `:128` all say what the §0.1 table says they say.

**Code claims other than contrast.** Verified at `HEAD`: 20 registered commands, 6 events
(`appUpdate:status` present), 8 `Settings` fields with the exact defaults tabulated at `prd.md:432`;
`QuitGuardDialog`'s only caller is `UpdateStatusItem.tsx:36` with `reason: "update"`, confirming
`prd.md:380`; both direct self-update paths exist — `dashboard/ManagerCard.tsx:128` and
`manager/SelfUpdateCard.tsx:116` — confirming `prd.md:235`'s "three immediate-execution call sites";
`PackageRow.tsx:92` uses native `disabled` and `:100` `disabled:opacity-40`, confirming
`prd.md:200`; 31 `focus-visible` sites and the CI assertion in
`tests/e2e/browser-style-contract.spec.ts`, confirming `prd.md:487`; 28 `ux-pb-*` entries in
`sprint-status.yaml`, confirming `prd.md:685`.

**`prd.md:28`'s numbering claim holds.** Every `AD-` id cited anywhere in the PRD — AD-12, AD-17,
AD-23, AD-24, AD-27, AD-28, AD-29, AD-30 — has a heading in `ARCHITECTURE-SPINE.md` at
`artifact_revision: 10`. No invented, retired (AD-6..10, AD-13..15), or misnumbered id appears.
