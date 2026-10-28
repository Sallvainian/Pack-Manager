# Reconcile review — `prd.md` (2026-07-25, status `final`) against `ARCHITECTURE-SPINE.md` revision 10

**Reviewer:** independent PRD reconciler
**Date:** 2026-07-25
**Input owned:** `_bmad-output/planning-artifacts/prds/prd-Pack-Manager-2026-07-25/prd.md` (709 lines, `wc -l`), `addendum.md` (81 lines, `wc -l`)
**Target:** `ARCHITECTURE-SPINE.md` — read at 1286 lines, **re-anchored at 1308 lines, mtime `2026-07-25 12:53:41`**, which is the version every line citation below refers to.

> **Note on a moving target.** The spine changed under this review (1286 → 1308 lines) while other reconcilers landed. Every quote below was re-verified against the 12:53 version; two findings I had drafted were withdrawn after re-reading, because the new Open row at `:1306` already surfaces them (see "Withdrawn" at the end). If the spine has moved again, re-anchor by the quoted phrase, not by the line number.

**Verdict: 1 CRITICAL, 4 HIGH, 2 MEDIUM, 3 LOW.** The spine tracks the PRD closely on the material the revision-10 note claims — AD-28 answers FR-6's membership, batching, `⌘A`, `Esc` and `⌘U` limbs almost clause for clause, and AD-29 cites NFR-4 correctly. The failures cluster in one place: **the D28 confirmation-opt-out path and the states around cancellation**, where the spine either contradicts a normative PRD sentence or states a rule whose phrasing reads as exhaustive while the PRD adds required limbs.

---

## CRITICAL

### C-1 — The spine mandates a `Cancelling` state on `OpStatus` that the PRD forbids by name

**Spine** (`ARCHITECTURE-SPINE.md:456`):

> - **Rule:** The same answer governs every new operation state the UX-PB stories
>   introduce, not only those two. `Cancelling` and `Interaction required` are
>   durable wire-level states on `OpStatus` as well — replay must reconstruct what
>   the user saw, and a transient flag following the `op:stalled` event precedent
>   cannot survive a crash or a replay. `OpStatus` ships seven variants today, so
>   every addition moves as one atomic AD-3 change across the Rust enum,
>   `src/lib/ipc/types.ts`, the guards, and `dev/fixtures/ipc/*.json`.

**PRD** (`prd.md:344`, FR-13, first consequence):

> - Queued, running, stalled, and terminal states are exposed with the exact command and live output visible. There is no distinct `cancelling` state: cancellation moves an Operation to its terminal state, and the 5-second SIGTERM grace window is not surfaced as its own status.

These cannot both be built. The spine instructs a builder to add a `Cancelling` variant to `OpStatus` as one atomic AD-3 contract change across the Rust enum, the TypeScript types, the guard map and the committed fixtures; the PRD — the declared requirements authority, whose FR-13 sentence is normative and not hedged — says that state must not exist and that the SIGTERM grace window is not surfaced as its own status.

This is not a `Planned`-vs-`Shipping` mismatch. Verified at `HEAD` `1ac959e`:

```
$ grep -rn -A14 "enum OpStatus" src-tauri/src/
src-tauri/src/ipc.rs:99:pub enum OpStatus {
    Queued, Running, Succeeded, Failed, Cancelled, TimedOut, Interrupted,
}
$ grep -rni "cancelling\|canceling" src/ src-tauri/src/
```
— seven variants, none of them `Cancelling`, and the only three `cancelling` hits in the tree are an `App.tsx` comment, a `queue.rs` test comment and a `tracing::info!` string. So the spine's "seven variants today" is right and its instruction is genuinely additive.

The conflict has a traceable origin, which matters for the remedy: `Cancelling` comes from the **Phase 3/UX layer**, not from the PRD.

- `_bmad-output/planning-artifacts/epics.md:740` — "running work moves to `Cancelling` and escalates through the existing process-group mechanics"
- `_bmad-output/planning-artifacts/epics.md:903` — "changes still-running Operations bound to that `planAttemptId` to `Cancelling`"
- `EXPERIENCE.md:216` — "`Cancel plan` requires no second confirmation, changes running work to `Cancelling`"

Under `prd.md:16` ("Where this document and an older artifact disagree, this document wins") the PRD supersedes all three. The spine sided with `epics.md`/`EXPERIENCE.md` **silently** — nothing in AD-16 records that it is overriding FR-13, and the surrounding text presents the rule as a neutral application of the durability principle rather than as a decision against the requirements authority. Note the spine *does* know how to do this properly: AD-29 at `:1150` says "this **deliberately overrides UX-PB.2c's stated record contents**", and AD-22 at `:801` says "**This deliberately overrides UX-PB.5b's stated clause order**". `Cancelling` got no such sentence.

Compounding it, AD-16's own cancellation rule two bullets earlier (`:435`) is written *without* a `Cancelling` state and matches the PRD:

> - **Rule:** Primary cancellation targets `planAttemptId`: unstarted work becomes
>   `Skipped`, running process groups use the existing escalation, and every
>   terminal state stays durable.

So the spine currently contains both models, 20 lines apart. A builder implementing UX-PB.2f reads `:457` and ships a `Cancelling` variant; a builder implementing FR-13 reads the PRD and ships none. Both pass their own review, and the fixture-backed contract makes the collision a hard break rather than a cosmetic one.

**Remedy — pick one and state it in the spine's own text:**

- **(a) The PRD wins.** Strike `Cancelling` from the `:456` rule, leaving `Interaction required` (which FR-14 at `prd.md:362` does require as a shown state, so it is unaffected). Add a clause to the `:435` cancellation rule: "There is no `Cancelling` state — cancellation moves an Operation to its terminal state and the 5s SIGTERM grace window is not a status (`prd.md` FR-13). `epics.md` UX-PB.2f/3g and `EXPERIENCE.md:216` are the stale side and come out through `bmad-correct-course` and a `bmad-ux` Update." Add the corresponding item to the `epics.md` residuals row.
- **(b) The spine overrides.** Keep `Cancelling` and say so explicitly, in the AD-22/AD-29 house style: name FR-13, quote the sentence being overridden, and give the reason (replay must reconstruct what the user saw). Then the PRD needs a reconciliation-queue entry, because the requirements authority would be carrying a sentence the spine has knowingly refused.

(a) is the cheaper answer and the one the authority chain points to; but either is legitimate, and the defect is that neither was done.

---

## HIGH

### H-1 — FR-6 names three immediate-execution call sites; the spine names one, and its baseline records one

**PRD** (`prd.md:233`, FR-6):

> - Neither a checkbox nor a row action executes anything. **Three immediate-execution call sites are in scope for removal**, not one: the Package row action, and *both* direct Manager self-update paths — the Dashboard Manager card and the Manager workspace self-update card each invoke the self-update command directly today, bypassing the plan entirely. Scoping the D27 work to the row action alone would leave two unstaged mutation paths alive and breach SM-2.

**Spine** (`ARCHITECTURE-SPINE.md:401`, AD-16):

> - **Rule:** No entry point executes. A Package row action, a Manager-header
>   action, a Manager-wide action, and `Update Everything` all mutate the draft and
>   return. `execute_plan` is reachable only from the confirmed-attempt path below.
>   The shipping `ManagerPane.upgradeRow` → `executePlan` call site is retired by
>   this rule, not preserved by it (`docs/DECISIONS.md` D27).

**Spine baseline** (`ARCHITECTURE-SPINE.md:161`):

> - The Upgrade Plan is currently transient dialog state (`ui.dialog`
>   `{ kind: "upgradePlan" }`, discarded by `closeDialog`), a single-package row
>   action executes immediately, …

The *rule* is general enough ("No entry point executes"), but both places where the spine converts the rule into concrete shipping scope name exactly one call site, and the two it omits do not execute `execute_plan` at all — they invoke a different command, `selfUpdateManager`, so a builder grepping for the named `executePlan` call site finds one hit and closes the story.

All three verified at `HEAD` `1ac959e`:

| Call site | Verbatim |
| --- | --- |
| `src/components/manager/ManagerPane.tsx:145-152` | `async function upgradeRow(pkg: Package) {` … `// Single-package plan executes immediately — no sheet (SPEC §F5).` … `await executePlan(plan);` |
| `src/components/manager/SelfUpdateCard.tsx:116` | `onClick={() => void selfUpdateManager(managerId)}` |
| `src/components/dashboard/ManagerCard.tsx:128` | `void selfUpdateManager(info.id);` |

The PRD's claim is accurate and the spine's is incomplete. This is the exact failure the PRD's own sentence predicts: "Scoping the D27 work to the row action alone would leave two unstaged mutation paths alive and breach SM-2" — and SM-2 (`prd.md:658`) is "Zero unreviewed mutations… A single violation is a P0 defect."

Note this also interacts with AD-16's `:401` enumeration "A Package row action, a Manager-header action, a Manager-wide action, and `Update Everything`" — the Dashboard Manager card's overflow-menu `Self-update` item is none of those four by name, so even the rule's enumeration under-covers the surface.

**Remedy:** extend the `:404` sentence to name all three: "The shipping `ManagerPane.upgradeRow` → `executePlan` call site is retired by this rule, and so are **both** direct `selfUpdateManager` call sites — `SelfUpdateCard` and the Dashboard `ManagerCard` overflow menu — which bypass the plan entirely (`prd.md` FR-6)." Correct the baseline bullet at `:161` to record all three, since it is what a builder reads for starting conditions.

### H-2 — FR-7's three D28 compensations are mandatory and load-bearing; the spine has none of them, and its opt-out rule reads as exhaustive

**PRD** (`prd.md:260`, FR-7):

> - **Planned — D28, and load-bearing:** when the opt-out is active, three compensations replace the dialog and all three are required. The plan **auto-expands the exact commands** before the action is enabled; a **persistent `Confirmation is off` notice** is shown and links to Settings; and the primary action **relabels from `Confirm` to `Run N updates`**. Removing the gate without them produces a button still reading `Confirm` that executes immediately with commands collapsed behind a reveal — the outcome `EXPERIENCE.md` names as an anti-pattern, and a direct breach of SM-2. The compensations are the price of the opt-out, not a nicety attached to it.

**Spine** (`ARCHITECTURE-SPINE.md:469`, AD-16):

> - **Rule:** Settings replace active `autoOpenDrawer` behavior with
>   `skipUpgradePlanConfirmation`, default `false`. A confirmation opt-out skips
>   only the final modal — never draft review, the Rust rebuild, stale detection,
>   or the explicit confirmation action.

Command run: `grep -n -iE "Run N updates|Confirmation is off|auto-expand|compensat|relabel|opt-out" ARCHITECTURE-SPINE.md` → **3 hits, none of them a compensation**: `:470` (the rule above), `:750` (AD-21's "the confirmation opt-out deterministically failing the admission it rides on"), `:811` (AD-22's "an unsaved opt-out costs one extra confirmation"). Zero occurrences of the three compensations anywhere in the spine.

The problem is not silence — it is that `:469` is phrased as a closed enumeration of what the opt-out does and does not remove ("skips **only** the final modal — never draft review, the Rust rebuild, stale detection, or the explicit confirmation action"). A builder implementing UX-PB.5a/5b against that sentence has been told precisely what the opt-out preserves, and the list does not include the auto-expansion, the notice or the relabel. They ship exactly the anti-pattern the PRD names — the gate removed, commands still collapsed behind a reveal, button still reading `Confirm` — and they can defend it against every word of AD-16.

This is not excluded by `addendum.md` §1. That table (`addendum.md:11-20`) assigns search-path construction, ownership classification, adapter signatures, lock-set structures, IPC signatures, transcript syntax, test seams and updater transport to other artifacts. The confirmation compensations appear nowhere in it, and `addendum.md:22` explicitly lists what *did* earn a place in the PRD, including "exact preview/execution agreement" — which is what the auto-expansion enforces.

Nor is it "should say more". The spine already spends three rules on this exact path (AD-16 `:469`, AD-21 `:765` classifying the key plan-inert, AD-22 `:808` on the rider's commit ordering), so the opt-out is squarely in the spine's scope; the safety compensations are the one part of it that fell out.

**Remedy:** add one clause to the `:469` rule — "When the opt-out is active, the three compensations FR-7 requires are not optional: the plan auto-expands the exact commands before the action is enabled, a persistent `Confirmation is off` notice links to Settings, and the primary action relabels from `Confirm` to `Run N updates`. They are the price of removing the gate (`prd.md` FR-7); shipping the opt-out without them breaches SM-2." That is one sentence and it closes the divergence without restating UX.

### H-3 — FR-8 requires confirmation be unavailable during and after a failed rebuild; the spine preserves a confirmable preview and states no in-flight rule

**PRD** (`prd.md:274`, FR-8, last consequence):

> - Final confirmation is unavailable while a plan rebuild is in flight and after a rebuild failure. The user can never confirm a preview the backend has not just re-derived — during a pending rebuild the displayed commands still belong to the previous options, so a confirmation in that window would execute something other than what is on screen while passing every other check in this FR.

**Spine** (`ARCHITECTURE-SPINE.md:558`, "Draft-mutation convergence" under AD-16):

> - **Draft-mutation convergence.** Every draft mutation resolves against a Rust
>   canonical rebuild. If the rebuild errors or rejects, the prior coherent draft
>   and its last authenticated preview are preserved unchanged and nothing is
>   admitted.

Command run: `grep -n -iE "in flight|unavailable|disabled while|until the rebuild" ARCHITECTURE-SPINE.md` → 4 hits, none about confirmation availability: `:290` (route precedence "then unavailable"), `:447` (verification refresh coalescing), `:883` (retry members "unavailable"), `:1299` (a Decision Status row). **The spine has no rule governing the confirm action while a rebuild is in flight.**

Two distinct problems:

1. **The gap.** FR-8's stated failure mode — the displayed commands still belong to the previous options, so a confirmation in that window executes something other than what is on screen "while passing every other check in this FR" — is precisely the case that survives every other rule the spine does carry (AD-16's revision drift, stale-plan rejection and one-use `planId` all pass, because the *previous* preview is genuinely valid). Nothing in the spine closes it.

2. **The contradiction.** On rebuild failure the spine says the last authenticated preview is "preserved unchanged", where FR-8 says confirmation is unavailable "after a rebuild failure". A builder reading "preserved unchanged" implements a still-confirmable preview; the PRD forbids exactly that. The spine is also in tension with itself here — AD-16 `:406` says the `planId` "expires on mutation, staleness, execution attempt, or eviction", so a mutation whose rebuild then fails has both expired the capability (`:410`) and preserved it (`:560`).

I can see the intent behind `:558` — rollback semantics, where a failed mutation leaves the draft untouched so the prior preview still describes it truthfully. That reading is coherent. But it is not stated, it collides with the expiry rule, and it lands on the opposite side of a fail-closed requirement (NFR-1, `prd.md:544`: "Unreviewed, stale, altered, replayed, partially admissible, or privilege-seeking work never runs").

**Remedy:** add to the convergence bullet — "While a rebuild is in flight, and after a rebuild that errored or was rejected, the final confirmation action is unavailable: the user may never confirm a preview the backend has not just re-derived (`prd.md` FR-8). 'Preserved unchanged' means the draft and its display survive the failure, **not** that the expired preview stays confirmable — a mutation expires the `planId` under the rule above, and recovery is a fresh rebuild."

### H-4 — The spine asserts the contrast guard shipped; the PRD asserts three times that it has not, and the spine never records that it supersedes it

**PRD** — three normative status statements:

- `prd.md:447` (FR-19): "Text contrast meets at least 4.5:1 on its surface. **Not met at `HEAD`** — see NFR-6."
- `prd.md:572` (NFR-6): "**The 4.5:1 contrast floor does not hold at `HEAD`** — three bright-fill sites still paint white ink, measuring 2.46:1, 2.30:1 and 2.15:1. The fix and its automated guard exist only as uncommitted working-tree changes."
- `prd.md:628` (§7.1): "**Not in this list, deliberately:** the automated contrast guard. The 4.5:1 assertion and the on-fill ink tokens that make it pass are **uncommitted working-tree changes**, absent from `HEAD` `5972109`. Until they land, contrast at release time is a by-eye check, and neither FR-19 nor NFR-6 may be read as CI-guaranteed on that axis."

Also `addendum.md:70`: "**The contrast guard and its on-fill ink tokens are uncommitted.** … absent from `HEAD` `5972109`. Until they land, D36's guarantee is not enforced by CI."

**Spine** (`ARCHITECTURE-SPINE.md:997`, AD-27):

> - **Rule:** Text on a bright fill takes the palette's **dark ink**, never white.
>   `--color-on-accent` is the ink for `--color-accent`, `--color-accent-hover`
>   and `--color-danger` alike; white on those measures 2.46:1, 2.15:1 and 2.30:1
>   against a 4.5:1 floor … (`docs/DECISIONS.md` D36, commit `a201fb0`). The guard in the same style-contract
>   lane measures the **rendered** foreground and background of a named sample and
>   fails below 4.5:1 …

**The spine is right and the PRD is stale.** Verified:

```
$ git log --oneline -1
1ac959e ci: bump the Claude Code action model to opus-5
$ git log --oneline 5972109..HEAD | wc -l
5
$ git show --stat --oneline a201fb0
a201fb0 fix(ui): use the palette's dark ink on bright accent fills
 src/components/primitives/Button.tsx      |  6 +-
 src/components/shell/UpdateStatusItem.tsx |  2 +-
 tests/e2e/browser-style-contract.spec.ts  | 98 +++++++++++++++++++++++++++++++
$ git grep -c "text-white" HEAD -- src/
(no output — zero occurrences)
$ git grep -n "4.5" HEAD -- tests/e2e/browser-style-contract.spec.ts
226:  test("[P0] paints bright accent fills with ink that clears the 4.5:1 contrast floor", …
320:      expect(measured.ratio).toBeGreaterThanOrEqual(4.5);
```

The PRD pinned itself to `HEAD` `5972109`; `HEAD` is now `1ac959e`, five commits later, and `a201fb0` landed both the ink fix and the guard. So this is a **stale input claim**, reported as such per the brief.

The finding against the spine is narrower but real, and it is a direction-3 problem. AD-11 `:357` does record the correction, but only as historiography about its own revisions:

> - **Rule:** This rule states a *claim boundary*, not a coverage inventory … revision 9 corrected the reduced-motion half,
>   and revision 10 found the contrast half false again because the check landed
>   (`docs/DECISIONS.md` D36, commit `a201fb0`).

It cites D36 and its own prior revisions. **It never names the PRD claim it is overriding**, and the PRD is the declared requirements authority whose status tags are normative (`prd.md:26`: "The tags are implementation status, not requirement strength"). Three separate PRD sentences currently tell a builder that contrast is not CI-guaranteed and that FR-19/NFR-6 "may not be read as CI-guaranteed on that axis"; the spine tells them the opposite. Nothing tells a reader which is current.

**Remedy:** one clause on AD-27's ink rule or the design-token row — "`prd.md` FR-19, NFR-6 and §7.1 record this as uncommitted and absent from `HEAD` `5972109`; that was true when written and is now stale — `a201fb0` landed both the ink and the guard, `HEAD` is `1ac959e`, and no `text-white` remains in `src/`. The spine's status supersedes the PRD's on this axis." And flag it for the PRD's own reconciliation queue, since `addendum.md` §3 does not currently list `prd.md` as needing correction against a moving `HEAD`.

---

## MEDIUM

### M-1 — RP-2's Edit/Window submenu re-declaration obligation has no invariant, and the Capability Map routes it to two ADs that do not state it

**PRD** (`prd.md:532`, RP-2):

> Standard Edit and Window menu actions — including cut, copy, paste, and select-all in the search field and in every copyable command surface — are preserved. This is a functional requirement, not an accessibility one: `app.set_menu` replaces Tauri's default menu wholesale, so these submenus must be re-declared or the shortcuts die silently (D25a).

**Spine Capability Map** (`ARCHITECTURE-SPINE.md:1268`):

> | Application menus and the accelerator map (RP-2) | `app.set_menu` re-declaration + the global key handler | **AD-28** (which key owns the native default), AD-11 (what the checklist still carries) |

The map routes RP-2 to AD-28 and AD-11. Neither states the obligation:

- **AD-28** `:1115` governs only *suppression of the native default* by an accelerator that shadows an Edit-menu action ("suppresses the native default **only on surfaces where it performs its own action**"), and `:1096` enumerates the surviving accelerator map. Both are about the key handler, not the menu.
- **AD-11** `:370` says only what the checklist carries: "What the checklist *does* still carry is `⌘X`/`⌘C`/`⌘V`/`⌘A` and the application accelerator map, as a functional copy-paste concern under D25a" — a release-acceptance claim, and AD-11 `Binds: release`, so no story reads it.

Command run: `grep -n -iE "set_menu|Edit menu|Edit-menu|Window submenu|Window menu|submenu" ARCHITECTURE-SPINE.md` → 4 hits (`:1100`, `:1115`, `:1121`, `:1268`). None states that replacing the default menu obliges re-declaring the Edit and Window submenus. The **Window** menu is not mentioned anywhere in the spine.

The divergence: a story that touches `app.set_menu` — adding an item, reordering, changing the app menu — has no invariant telling it that omitting a submenu silently kills cut/copy/paste, and RP-2 is a *release prerequisite* (`prd.md:513`: "These two are mandatory prerequisites rather than product features"). D25a's whole point is that the failure is silent.

**Remedy:** add a clause to AD-28 (which already owns the accelerator map and already binds "the application accelerator map (`prd.md` RP-2)" at `:1032`): "`app.set_menu` replaces Tauri's default menu wholesale, so the standard **Edit and Window** submenus are re-declared in the same change or cut/copy/paste/select-all die silently (`docs/DECISIONS.md` D25a, `prd.md` RP-2). A story that edits the menu owns this." Then the Capability Map row is true as written.

### M-2 — FR-14's quit guard is a requirement with no enforcement-point invariant, where its sibling FR-21 got one

**PRD** (`prd.md:361`, FR-14):

> - **Not yet built:** quitting with work in flight presents an explicit choice and does not silently discard it. The dialog exists and is rendered by the shared dialog host, but nothing listens for a quit — its only caller is the application-update path, so the *restart* case is guarded (FR-21) and the *quit* case is not. Do not read this consequence as shipping.

Corroborated by `addendum.md:68`: "**The quit guard is unwired.** `QuitGuardDialog` exists and the dialog host renders it, but no close-requested handler exists in either process."

Command run: `grep -n -iE "quit|close-requested|closeRequested" ARCHITECTURE-SPINE.md` → 2 hits, neither about the guard: `:603` (AD-17, "after a clean quit, a crash, or a force-quit — starts with an empty draft") and `:1293` (the crash/relaunch lifecycle Deferred row).

The contrast with FR-21 is what makes this a finding rather than an observation. The PRD's other refusal-while-busy requirement (`prd.md:496`) says "This refusal is enforced independently in two layers, and both must stay: the frontend quit guard explains it to the user, and the backend refuses on its own", and the spine gave that one a full Decision Status row (`:1299`) recording the Rust enforcement point, the exact helper, and the verification that the frontend predicate matches — "so the guard the user sees and the guard that actually holds cannot drift apart, which was the defect".

The quit case has the same shape and the same drift risk, and nothing in the spine assigns it a layer. A builder wiring `onCloseRequested` frontend-only reproduces exactly the defect `:1299` exists to record as fixed.

I am not claiming the spine must specify the dialog. The divergence is narrower: **which process refuses**, and whether the two agree.

**Remedy:** either a one-clause addition to AD-16's application-update rule or a Decision Status row — "The quit guard is unbuilt (`prd.md` FR-14). When it lands it takes the same two-layer shape as the app-update refusal: the frontend explains, and Rust refuses on its own against the same `Queued`/`Running` predicate. `prd.md` §9 Q1 leaves the queued-but-not-running and OS-shutdown cases open; those are owner decisions, not builder ones."

---

## LOW

### L-1 — The spine names `rustDedup` without naming which side is excluded

**PRD** (`prd.md:206`, FR-5):

> - The one permitted cross-Manager deduplication is the Rust rule (D10): a single Upgrade Plan never contains both mise's `tool:rust` and rustup toolchains; the mise entry is excluded with a visible reason.

**Spine** (`ARCHITECTURE-SPINE.md:581`):

> A plan-composition exclusion depends on what else is in the plan — `rustDedup`
> when rustup toolchains join a plan also containing mise's `tool:rust`, and
> `alreadyRunning` — so the item stays in `PlanIntent` and is surfaced with its
> reason in the preview's exclusions.

The spine names the trigger but not which side loses, and then says "removing the other side of the conflict restores the item" — where "the item" is undefined. The PRD fixes it (the mise entry), and shipping code agrees: `src-tauri/src/queue.rs:557` removes `tool:rust` from the mise list and pushes `ExcludedPackage { manager_id: ManagerId::Mise, package_id: "tool:rust", reason: ExcludeReason::RustDedup }`.

Rated LOW because the behavior is fixture-backed (`src-tauri/src/ipc.rs:858-862` carries the same shape as a committed contract fixture), so a builder inverting it fails the contract test rather than shipping it. **Remedy:** insert "the mise `tool:rust` entry is the excluded side" at `:582`.

### L-2 — FR-19's reduced-motion requirement exists in the spine only inside a revision note

**PRD** (`prd.md:448`, FR-19): "- The reduced-motion preference disables transitions." Also NFR-6 (`prd.md:574`): "reduced motion honored".

Command run: `grep -n -iE "reduced.motion|prefers-reduced" ARCHITECTURE-SPINE.md` → 2 hits, both in the revision-history header block: `:88` ("AD-11's accessibility rule is corrected: reduced motion **is** automated and runs in CI") and `:361` ("revision 9 corrected the reduced-motion half"). Neither is a `Rule`, and the Styling convention at `:1209` — which does carry the dark-only, token, focus and on-accent-ink rules — omits it.

Rated LOW: the behavior ships and is CI-asserted, so this is un-guarded working code rather than a divergence, and AD-11's claim-boundary rule (`:357`) deliberately refuses to inventory what is automated. But it is the one FR-19 limb with neither a rule nor a convention entry, and D37 explicitly did *not* touch it (`docs/DECISIONS.md:533`: "D33's reduced-motion and contrast positions are unaffected") — so unlike the keyboard limbs, its absence is not a decision. **Remedy:** one clause in the Styling convention, alongside the existing colour rules.

### L-3 — FR-5's outdated-first ordering has no presence in the spine

**PRD** (`prd.md:205`, FR-5): "- **Planned:** within the list, Packages with updates sort first. No ordering is applied anywhere today — the default filter delivers most of the benefit, but the ordering requirement is real and unbuilt."

Command run: `grep -n -iE "outdated-first|sort|ordering" ARCHITECTURE-SPINE.md` → 3 hits, all unrelated (`:799` AD-22 clause ordering, `:1166` AD-29 append ordering, `:1283` a Decision Status row).

Rated LOW and reported only because ordering touches AD-28's range semantics: a shift-range is defined by an anchor and a target over the *rendered* order, so changing the order changes which identities a range covers. AD-28 already handles the correctness half — `:1061` requires "concrete canonical identities computed from the **snapshot the user is looking at**" — so no divergence follows, which is why this is LOW rather than MEDIUM. Recorded so a reader does not mistake the silence for a decision that ordering is out of scope. **Remedy:** none required; optionally note under AD-28 that ordering is a presentation concern that must not change the batch's derivation.

---

## Checked and clean

Recorded so a later pass does not re-derive them. Each was checked clause-by-clause against the spine:

- **FR-6 membership, batching, `⌘A`, `Esc`, `⌘U`, header tri-state, canonical identities, one predicate** — AD-28 `:1038-1122` covers every limb, and covers the `⌘A` `preventDefault()` defect (`:1115`) that `addendum.md` §4.1 raises. Verified the defect is live at `HEAD`: `src/hooks/useKeyboard.ts:160-162` — `case "a": e.preventDefault(); selectAllVisible();`.
- **FR-6's batch requirement / NFR-3** — AD-28 `:1053` quotes NFR-3 verbatim and gets the citation right.
- **NFR-4's transcript precondition** — AD-29 `:1171` cites `prd.md` NFR-4 correctly and states the asymmetry with the journal append.
- **FR-5's explanatory-disabled treatment** — AD-16's inertness bullet `:564-577` is stronger than the PRD's, forbids native `disabled` *and* opacity-alone, and correctly names the shipping row as the defect (verified: baseline `:175` matches `src/components/manager/PackageRow.tsx`).
- **FR-15 evidence, retention, PGID rule, `mas` newline exception** — AD-5 `:310`, AD-4 `:291`, AD-18 `:682-705`, AD-29's fold rule.
- **FR-19 focus mechanism / D37 scope** — AD-27 `:963` and the D37 Decision Status row are correct against `docs/DECISIONS.md:519-560`, including the "Story UX-PB.1d is not to be deleted" carve-out.
- **RP-2's accelerator enumeration** — AD-28 `:1096` matches `prd.md:534` exactly (`⌘R`, `⌘⇧R`, `⌘⇧U`, `⌘L`, `⌘F`, `⌘1–9`, `⌘A`).
- **FR-9's one-active-attempt rule** — AD-16 `:432` carries it as a typed already-active result, separate from the lock-overlap test at `:424`, which is the separation `prd.md:285` insists on.

## Withdrawn after re-anchoring

Two drafted findings were withdrawn when the spine moved from 1286 to 1308 lines mid-review:

- **`Esc`'s surviving second rung** (FR-6 calls it close-drawer; AD-17 retires the `ActivityDrawer`) — now surfaced by the spine itself at `:1088-1095` and given an Open row at `:1306`.
- **`⌘L`'s sink** (RP-2 requires it survive as "toggle the activity surface"; AD-17 moves Activity to `ActiveView`) — same Open row at `:1306`, which routes both to the owner as outside revision 10's authorized scope.

Both are correctly handled; no action needed from this review.
