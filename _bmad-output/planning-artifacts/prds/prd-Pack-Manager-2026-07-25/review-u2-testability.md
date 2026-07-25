# PRD Review — Testability of Consequences (Update pass 2, spine revision 10)

**Target:** `_bmad-output/planning-artifacts/prds/prd-Pack-Manager-2026-07-25/prd.md` (768 lines)
**Reference:** `_bmad-output/planning-artifacts/architecture/architecture-Pack-Manager-2026-07-23/ARCHITECTURE-SPINE.md` (1601 lines, revision 10)
**Also read this session:** `src-tauri/src/settings.rs`, `src/store/packages.ts`, `src/components/manager/PackageRow.tsx`, `src-tauri/src/logging.rs`, `review-testability.md` (prior pass)
**Lens:** every bullet under `**Consequences (testable):**` must be checkable by someone who can run the app and read the repo. Emphasis on text this Update added or rewrote.

---

## Overall verdict

The four requirement-defect corrections landed and are genuinely better than what they replaced. FR-6's tri-state denominator (`:231`) is now the most testable sentence in §4.2 — it names the set, the three states, and forbids storage. FR-14's OS-shutdown carve-out (`:381`) states an observable outcome ("It gets **no dialog**") and an observable invariant ("children never outlive the app"). FR-18's Planned — D29 bullet (`:464`) is checkable by opening the archive and diffing an entry against the on-disk journal.

The problem is the **Architecture binding** blocks. Thirteen were added, and they are not uniformly citations. Four of them state obligations that appear **nowhere else in the document**, and one of those four — FR-7's tombstone removal taxonomy — is the entire removal semantics of the Upgrade Plan, the product's central D27 feature, carried in a single clause using a term the PRD defines nowhere. Two more silently **narrow a consequence stated above them**, so the consequence as written admits a build the binding forbids. This is exactly the failure mode the review was scoped to look for, and it is present.

Two structural facts amplify it. First, eleven bindings are separate paragraphs (citation shape) but **two are bullets inside the `Consequences (testable):` list** — FR-7 `:274` and FR-14 `:382` — which is where the requirement-in-a-citation risk is highest, because the reader is told at `:28` that bindings are citations. Second, `RP-2` — which this pass added two normative paragraphs to — has **no `Consequences (testable):` block at all**, the only requirement in §4 without one (23 such blocks exist; RP-2 is not among them).

Counts: **1 critical, 6 high, 8 medium, 3 low.**

---

## CRITICAL

### C1 — The Upgrade Plan's entire removal semantics is a requirement hiding inside one Architecture binding bullet

**Location:** `prd.md:274` (FR-7, bullet inside `Consequences (testable):`)

> - **Architecture binding — `ARCHITECTURE-SPINE.md` AD-28:** the per-item `Remove` this FR requires on the sidecar is a single-ref removal under AD-28's closed removal taxonomy, so each use writes a tombstone that no later bulk expansion re-adds (AD-23). A scope-wide removal — the header checkbox, `⌘A` on an all-staged view, a Manager-wide remove, `Clear`, or undoing an `Update Everything` seed — instead clears membership *and* the tombstones of the refs whose membership it actually cleared, never of refs that held none.

**Note.** `grep -n "tombstone" prd.md` returns **exactly one line: 274**. The word is not in the Glossary (§3), not in FR-6, not in §7.2's D27 block. This bullet is the only place in the PRD where the reader learns that:

- a per-item `Remove` has a **persistent side effect beyond removing the item** (it vetoes future re-staging);
- `Update Everything` must **not** re-stage a deliberately removed Package;
- a scope-wide removal clears tombstones, but only for refs it actually cleared;
- controls named `Clear`, "a Manager-wide remove", and an "`Update Everything` seed" **exist at all** — none of the three appears anywhere else in the document.

FR-6's own consequence says only "every staged item is individually removable from the Upgrade Plan" (`:234`). A story writer building FR-6/FR-7 from the consequence lists builds a plain remove and satisfies every bullet. The user-visible defect that produces is concrete and is the one AD-28 `:1260`–`1271` was written to prevent: remove `foo` deliberately, click `Update Everything`, `foo` comes back.

Three further gaps make the clause untestable even for a reader who does open the spine:
1. **Tombstone lifetime is unstated.** AD-28 `:1262` calls them "session-permanent vetoes"; the PRD says nothing. Does a tombstone survive a refresh? A Manager switch? A relaunch? (AD-17 `:695` says the draft is session-scoped, which implies but does not state the answer.)
2. **No observable outcome is stated in product terms.** "writes a tombstone" is a mechanism. The testable form — "a Package removed individually is not re-added by a later `Update Everything` or header-checkbox add" — appears nowhere.
3. The three controls it names have no requirement defining them, so a tester cannot locate them.

**Fix.** Promote this to FR-6 consequences in observable form, e.g.:
- "Removing a staged Package individually — from its checkbox or the sidecar's `Remove` — prevents any later scope-wide addition (`Update Everything`, the header checkbox, `⌘A`) from re-staging it. Only re-staging it deliberately restores it."
- "A scope-wide removal clears that protection for exactly the Packages it removed, and for no others."
- "The protection is scoped to the session and dies with the draft."
Then define `Clear`, the Manager-wide remove, and the `Update Everything` seed as controls in FR-6/FR-7, and leave `:274` as a citation pointing at AD-28/AD-23 for the taxonomy's closure.

---

## HIGH

### H1 — FR-14's quit-guard trigger set exists only inside the AD-30 binding; the consequence above it says something narrower

**Location:** `prd.md:380` (consequence) vs `prd.md:382` (binding bullet)

> - **Not yet built:** a **user-initiated** quit with work in flight — a window-close request or ⌘Q — presents an explicit choice and does not silently discard it.

> - **Architecture binding — `ARCHITECTURE-SPINE.md` AD-30:** … AD-30 also fixes the guard's active set as `Queued` ∪ `Running` — **queued counts as running** — identical to FR-21's app-update guard, and the two may not drift apart.

**Note.** The only normative statement of *when the guard fires* is attributed to AD-30 and phrased as a report of what the spine says. The consequence's own trigger is "work in flight", which reads as running-only. A tester validating FR-14 from `:380` would queue nothing, run one Operation, press ⌘Q, see the dialog and pass. A build that quits silently with three Operations queued and none started passes `:380` and fails AD-30. §9.2 `:744` asserts "Both are now consequences of FR-14" — the OS-shutdown half is (`:381`); the `Queued` ∪ `Running` half is not.

Compare FR-21 `:524`, which states the same predicate **as its own consequence** and does not delegate: "Installation and relaunch are **refused** while any Package Operation is queued or running. Queued counts as active…". FR-14 should mirror that shape.

**Fix.** Add a consequence to FR-14: "The guard fires whenever any Package Operation is `Queued` or `Running`. Queued counts as running — admission has already committed to the work." Reduce `:382` to the citation and the cross-guard non-drift constraint.

### H2 — FR-9's "active" is narrowed from a citation, and the un-narrowed consequence permits a permanently wedged build

**Location:** `prd.md:300` (consequence) vs `prd.md:304` (binding paragraph)

> - **Planned — D30:** only one confirmed Plan Attempt may be active at a time. A second confirmation fails closed while an attempt is unterminated, **independent of lock-set overlap** …

> **Architecture binding:** … AD-29 also fixes what may count as *active* for the one-attempt rule: only an attempt the running process actually owns. A journal record read at launch is history, never liveness — the failure mode being prevented is a dead attempt resolving as live and refusing every subsequent confirmation, permanently.

**Note.** "unterminated" is the whole test in the consequence, and it is satisfiable by reading the journal at launch — which is precisely the implementation AD-29 `:1442`–`1448` forbids ("Getting this wrong does not degrade presentation — it wedges the product"). The correction is only in the binding, and it is framed as something *AD-29* fixes rather than something the product must do. The consequence and the binding also give a tester two different tests, and only the binding's is safe.

**Fix.** Rewrite `:300` to carry the qualifier: "…while an attempt **owned by the running process** is unterminated. An attempt record found on disk at launch is history, never liveness: a crash or force-quit never leaves the product unable to confirm a new plan." Keep `:304` as the citation.

### H3 — FR-15's "unreadable evidence" is a new user-facing outcome stated only in a binding

**Location:** `prd.md:400` (binding paragraph)

> AD-29 also narrows this FR's Interrupted test at attempt scope — `Interrupted` requires a **genuine** absence, and a terminal record that exists but is unreadable is reported as *unreadable evidence*, never silently reclassified as an unfinished attempt. Those are different facts and a reader must be able to tell them apart.

**Note.** `grep -n "unreadable" prd.md` returns lines 400 and 489 only; 489 is about contrast. FR-15's consequences (`:392`–`:398`) define `Interrupted` at `:393` ("Work with a start record and no finish record surfaces as Interrupted on the next launch") and never mention a third outcome. So the PRD requires a **distinct History state the user can see** — with a name, presumably a label, and a distinct explanation — in a sentence that reads as a citation. `bmad-ux` source-extracting FR-15's consequences will not design it; `bmad-create-epics-and-stories` will not story it.

The Glossary (§3) defines `Interrupted` at `:122` and does not define this third state. §3's own rule — "Downstream workflows must use these terms exactly" — is not met by a state introduced in a binding.

**Fix.** Add a consequence under FR-15: "A terminal record that is present but unreadable is reported as unreadable evidence and is never shown as `Interrupted` — the two are distinguishable in History." Add the term to §3 alongside `Interrupted`.

### H4 — RP-2 received two new normative paragraphs and has no `Consequences (testable):` block

**Location:** `prd.md:563`–`573`

RP-2's entire body is four prose paragraphs (`:567`, `:569`, `:571`, `:573`). `grep -c "Consequences (testable)" prd.md` returns **23**; RP-2 has none. FR-12 (`:347`), FR-16 (`:408`), RP-1 (`:556`) — every other requirement in §4 has one, including the two other Shipping cross-cutting ones.

This pass added the ⌘L paragraph and the AD-28 binding to RP-2, so it now carries: a six-accelerator map, a re-pointing obligation for ⌘L, a no-op rule, a native-suppression rule, and a named shipping defect — all in prose. `:550` says these RPs "are validated through `docs/RELEASE-CHECKLIST.md`", so the consequence list is the artifact the checklist should mirror, and there isn't one.

The prior review raised this as its finding #1 ("RP-1 and RP-2 claim a validation route that does not exist, and carry no consequences"). RP-1 was fixed; RP-2 was not, and then grew.

**Fix.** Add `**Consequences (testable):**` to RP-2 and convert the prose: one bullet per accelerator with its observable outcome, one for Edit/Window submenu survival after `app.set_menu`, one for ⌘L's focus move, one for the no-op-when-hidden rule, one for the native-suppression rule.

### H5 — FR-17's target-set arithmetic contradicts the bullet immediately above it

**Location:** `prd.md:443`–`444`

> - **Planned — D28:** `skipUpgradePlanConfirmation` is added with a safe default of `false` and is reversible in Settings.
> - **Planned — D27–D30:** `autoOpenDrawer` **retires.** … The other seven fields are the target set.

**Note.** The shipping table (`:432`–`441`) has eight rows. Remove `autoOpenDrawer` → seven. Add `skipUpgradePlanConfirmation` from the bullet directly above → **eight**. "The other seven fields are the target set" is the last word in the FR and states a closed set that excludes the field the previous bullet adds. Two implementers reading `:444` as authoritative and `:443` as authoritative build different `Settings` structs, and `AD-3`'s atomic IPC-surface change makes that a contract divergence, not a cosmetic one. FR-17's status line at `:424` also says "all eight shipping fields work", so "eight" already means the shipping count — reusing "seven" for the target count without stating the addition invites the misread.

**Fix.** "The remaining seven shipping fields survive; with `skipUpgradePlanConfirmation` added by D28, the target set is eight."

### H6 — A consequence bullet in FR-6 states the behavior the FR removes, as if it were a consequence

**Location:** `prd.md:236`

> - The draft is transient dialog state discarded on close — pre-D27 behavior this FR removes.

**Note.** This sits in the `Consequences (testable):` list two bullets below its own negation at `:234` ("The draft persists while the user navigates between Managers and the Dashboard"). Every other bullet in the list is normative. A tester or a story generator processing the list bullet-by-bullet emits "draft is discarded on close" as an acceptance criterion — the exact opposite of the requirement, and a direct contradiction of AD-17 `:717`–`:728` (the sidecar "persists across `ActiveView` changes without losing membership or scroll identity").

FR-6 already has the right home for this: an `**Out of Scope:**` section three lines below at `:243`, which correctly holds the transient selection layer and ⌘U.

**Fix.** Move the sentence into `**Out of Scope:**` at `:243`, phrased as removal ("Transient dialog state for the draft, discarded on close — pre-D27 behavior this FR removes").

---

## MEDIUM

### M1 — "the bounded idle wait" has no bound

**Location:** `prd.md:381` (FR-14, OS-shutdown carve-out)

> The behavior is best-effort: run the existing kill hook — cancel every running Operation, then await the bounded idle wait, because cancellation only flips the tokens while the runner tasks perform the SIGTERM → grace → SIGKILL work, so a process that exits without awaiting may never poll them.

**Note.** `grep -n "idle wait" prd.md` returns line 381 only; the spine (`:1471`, `:1597`) uses the same phrase and also gives no number. Every other duration in FR-14 is stated: "default 120 seconds" (`:376`), "default 30 minutes" (`:378`), "5s grace" (`:379`). This one is the only bound the invariant at the end of the same bullet depends on — "**children never outlive the app**" is true or false depending on whether the wait outlasts SIGTERM + 5s grace + SIGKILL. Two implementers pick 1s and 10s; one of them ships orphaned `brew` processes after every logout, and both pass the bullet as written.

**Fix.** State it: "…then await a bounded idle wait long enough to cover the SIGTERM → 5s grace → SIGKILL escalation (default N seconds)." If the value is genuinely an architecture choice, say "at least the grace window plus the escalation, and the value is stated where it is configured."

### M2 — The header checkbox's behavior from the `mixed` state is undefined

**Location:** `prd.md:231` (FR-6, rewritten this pass)

> - The header checkbox adds or removes every eligible Package matching the active filter, including off-screen virtualized rows, and reports the exact count it will affect. **Its tri-state denominator is that same set** … It is **unchecked** when none of that set is staged, **mixed** when some, and **checked** when all.

**Note.** The denominator is now unambiguous — that is the fix this pass made, and it worked. What is still open is the **transition**: "adds or removes" tells a tester which two acts exist but not which state produces which. From `unchecked` it clearly adds; from `checked` it clearly removes. From `mixed` — the state the sentence takes the trouble to define — it is 50/50, and the two builds differ visibly: one click stages the remaining Packages, the other clears the ones already staged. AD-28 `:1311`–`:1316` defines the same three states and is likewise silent on the transition, so opening the spine does not resolve it. The removal taxonomy at AD-28 `:1248`–`:1259` establishes only that the header checkbox *can* remove.

**Fix.** One clause: "From `mixed` it adds the remainder; only from `checked` does it remove."

### M3 — "the backend rejects a batch whose token is not its current snapshot" states no observable outcome

**Location:** `prd.md:238` (FR-6, ⌘A predicate consequence, rewritten this pass)

> The frontend submits concrete identities **plus the snapshot token it read**, and the backend rejects a batch whose token is not its current snapshot.

**Note.** "rejects" is the whole outcome. What the user sees is unstated: is the click silently dropped, does the projection refresh and the batch re-apply, does an error surface, does the header checkbox snap back? FR-8 `:285` shows the standard this document sets for exactly this shape — "enqueues **nothing** and returns the user to review with a newly issued plan" — a rejection plus its user-facing resolution. Here there is no resolution, and the failure is reachable on every refresh that lands between render and click, which is the ordinary case on a machine with auto-refresh on launch.

Secondary: "snapshot token" is a new term used at `:238` and `:257` and defined nowhere; §3 defines `Snapshot` but not the token.

**Fix.** Add the outcome: "A rejected batch stages nothing, and the user is shown the re-projected list with the count the new snapshot produces." Add "snapshot token" to §3 or gloss it inline.

### M4 — FR-17's settings table names one of eight persisted fields, so it cannot be checked against the build it claims to describe

**Location:** `prd.md:430`–`441`

> The settings and their defaults **as the current build ships them** are below. This table is the shipping inventory, not the target set …
> | Run Homebrew metadata update during refresh | on |
> | Auto-open the activity surface for mutations — **shipping only; retires with the drawer** (`autoOpenDrawer`) | on |

**Note.** The table is explicitly a claim about the current build, so it is verifiable in principle — `src-tauri/src/settings.rs:28`–`:39` declares eight fields and `:41`–`:53` their defaults. But only the retiring row carries its field name. The other seven are prose labels a tester must map by inference (`run_brew_update_on_refresh`, `auto_refresh_on_launch`, `stall_after_secs`, `upgrade_hard_cap_mins`, `log_level`, `include_greedy_by_default`, `auto_check_for_updates`). Everywhere else the PRD names settings by field — `skipUpgradePlanConfirmation` at `:273` and `:443`, `autoCheckForUpdates` at `:42`, `autoOpenDrawer` throughout — so the table is the outlier.

The log-level row is additionally unassertable: "debug, for the app's own code" is not a value. `settings.rs:48` stores `LogLevel::Debug` and `logging.rs:55` `directive_for_level` turns it into a directive; the table describes the directive's effect, not the stored default, and a tester cannot tell which is being asserted.

**Fix.** Add the wire field name to each row (the table already proves the convention works), and state the log-level row as "`logLevel` | `debug` (applied as a filter directive scoped to the app's own crate)".

### M5 — FR-13's binding introduces `Skipped` as a durable Operation state that no consequence names

**Location:** `prd.md:367` (binding paragraph)

> Per-Operation `Verifying` and `Skipped` are durable states in the **Operation** journal; verification outcomes and the Results summary ride the attempt's **terminal** record, so Results is served by one read.

**Note.** `grep -n "Skipped\|skipped" prd.md` returns 365 and 367 only. Line 365 has "skipped work" as a **category in the Results summary** — a lowercase noun phrase, not a state. Line 367 makes it a capital-S durable Operation state, which is a different and stronger obligation (it must be journaled, it must survive a relaunch, it must be distinguishable from `failed` in History). Nothing in FR-13's consequences, FR-15's, or §3 establishes when an Operation becomes `Skipped` — which is the part an implementer actually needs.

**Fix.** Either add a consequence to FR-13 defining `Skipped` (what causes it, how it renders, that it is durable), or restrict `:367` to `Verifying` and let AD-16/AD-29 own `Skipped` without the PRD asserting it.

### M6 — FR-5's binding is internally two-directional about where row ordering lives

**Location:** `prd.md:208` (binding paragraph)

> **Architecture binding:** the unbuilt outdated-first ordering is constrained by `ARCHITECTURE-SPINE.md` **AD-28**: row ordering is presentation and may change freely, but it may never change how a batch is derived — a range is an anchor and a target over the **ordered filtered set the projection holds**, including off-screen virtualized rows, and never the rendered DOM window.

**Note.** "row ordering is presentation and may change freely" points an implementer at a frontend sort. "the **ordered filtered set the projection holds**" points at ordering being part of the backend projection — because FR-6 `:238` makes the predicate the backend's, and a range over "the ordered filtered set the projection holds" therefore needs the backend to hold the order. Both readings satisfy the sentence. They produce different systems: one where FR-5's Planned outdated-first sort (`:205`) is a `.sort()` in `ManagerPane`, and one where it is a field on the projected result. A shift-range across a re-sorted list then covers different Packages depending on which was built.

**Fix.** Say which tier owns the order: "the order the projection carries is the order a range resolves against; changing the sort is changing the projection, not the DOM."

### M7 — Range selection is presupposed by three passages and required by none

**Location:** `prd.md:237`, `:208`, `:597`

> - **Membership mutation must accept a batch.** A range or filter-wide interaction submits one membership operation covering every affected Package identity, not one per row.

**Note.** "range" appears at `:208`, `:237`, `:241`, `:597`, `:739`, `:753`. Every occurrence constrains it; none establishes it. No consequence in FR-5 or FR-6 says the user can shift-click to select a span, and the named filter-wide interactions (header checkbox, ⌘A) are separately specified. A story writer reading only the consequences builds the batch verb, the header checkbox and ⌘A, and never builds a range — losing a behavior that ships today (`src/components/manager/PackageRow.tsx:64` `if (e.shiftKey) onRangeSelect(pkg.id);`, `src/store/packages.ts:19` "Shift-range anchor per manager"). The anchor semantics that make it testable live only in AD-28 `:1201`–`:1206` and `:1242`–`:1247`.

**Fix.** Add a consequence to FR-6: "Shift-clicking an eligible row stages the inclusive span from the current anchor to the clicked row, as one batch. The anchor is frontend state; membership is never." Then `:208` and `:597` are citations again.

### M8 — FR-18's Planned bullet anchors to a journal name the list above does not use

**Location:** `prd.md:460` vs `:464`

> - It contains an environment and detection report, the newest three application logs, the newest 25 transcripts, and the History journal.

> - **Planned — D29:** the archive also carries the **plan-attempt journal**, as a second journal distinct from the Operation journal above and entered separately.

**Note.** "the Operation journal above" has no referent — the list above says "the History journal". A tester verifying "a second journal distinct from the Operation journal above" has to decide whether the shipping archive's History journal *is* the Operation journal (it is — `:398` calls attempt-less records "Legacy Operation records", and AD-29 `:1360` names `operations.jsonl`), but the document never says so. §3 defines neither term, and §3's rule is "No synonyms appear anywhere else in this document."

The bullet is otherwise the best-tested Planned limb in the document: "raw lines — never a synthesized record" is diffable, and "the two-record *set* … admission plus terminal" is countable.

**Fix.** Use one name. Either rename `:460`'s entry to "the Operation journal (History)" or make `:464` say "distinct from the History journal above".

---

## LOW

### L1 — RP-2 is tagged `Shipping.` while containing a `Planned — D27–D30` limb

**Location:** `prd.md:565` vs `:571`

> **Status:** Shipping.

> **⌘L is a focus move, not a toggle. Planned — D27–D30 for the behavior; the registration ships.**

**Note.** Not a complaint that ⌘L is unimplemented — that is fine and stated. The issue is that `:20`–`:24` defines `Partial` as exactly this shape ("some limbs ship; the rest is named inline") and FR-4, FR-5, FR-7, FR-11, FR-13, FR-14, FR-15, FR-17, FR-18 all use it for the same situation. A release-checklist author scanning statuses treats RP-2 as fully verifiable against 1.0.1 and passes ⌘L on the toggle behavior this RP now forbids.

**Fix.** `**Status:** Partial. The menu declarations and the accelerator registrations ship; ⌘L's sink is Planned — D27–D30.`

### L2 — RP-2's ⌘L consequence names no focus target and no way to observe the move

**Location:** `prd.md:571`

> ⌘L moves focus into the region, and when the region is hidden it is a **no-op** — it must not conjure the region into existence.

**Note.** The no-op half is testable (empty draft, press ⌘L, region stays hidden). The positive half is not: "into the region" does not say *onto what* — the region container, its heading, the first staged row, the first `Remove` button — and the four choices are visibly different. FR-19 `:478` requires a focus indicator on "every interactive element", and a region container is not one, so a build that focuses the container produces no visible change at all and a tester cannot distinguish it from a no-op. AD-17 `:766`–`:773` adds only that it is "the direct jump to the third of those" F6 stops.

**Fix.** Name the target and its observability: "⌘L moves focus to the sidecar region's first focusable control, which shows the standard focus indicator."

### L3 — Three binding clauses are unfalsifiable prose inside consequence lists

**Locations and quotes:**

- `prd.md:304` — "an attempt-journal write failure is surfaced, never fatal, so a full disk may not turn an all-or-none admission into 'none'." (also `:605`, "surfaced rather than fatal"). "Never fatal" is testable — fill the disk, confirm a plan, work still runs. "**Surfaced**" is not: no channel, no wording, no persistence is stated, and `grep` finds the word only in these two bindings.
- `prd.md:530` — "A change to the refusal predicate above is now an AD-30 change, not a local one." A governance rule about how future edits are classified; nothing a tester can check against a build.
- `prd.md:464` — "A folded attempt view may be added as an **additional** entry, marked as derived; it never replaces the raw lines." Permissive: the first half creates no obligation, so only the "never replaces" half is checkable, and only if someone chose to build the optional half.

**Fix.** For `:304`/`:605`, state the surfacing: "…is reported to the user as a degraded-evidence warning and recorded in the application log." For `:530` and `:464`, these are legitimately non-testable and belong in prose outside the `Consequences (testable):` list — `:530` already is; `:464`'s permissive clause could move to FR-18's Notes.

---

## What holds up

Recorded so a later pass does not re-open it:

- **FR-6 `:231` tri-state denominator.** Names the set, includes off-screen rows, forbids storage, defines all three states. This closed the ambiguity AD-28 `:1314` predicted ("two stories would pick differently"). Only the `mixed` transition is left (M2).
- **FR-14 `:381` OS-shutdown carve-out.** "It gets **no dialog**" is a pass/fail observation; "children never outlive the app" is checkable with `ps` after a logout; the rationale is marked as rationale rather than smuggled in as a requirement. Only the missing bound is a defect (M1).
- **FR-6 `:238` last clause and RP-2 `:573`.** "⌘A must also stop suppressing the native select-all on surfaces that have no Package list" names three surfaces (Dashboard, History, Settings) and an observable outcome. Directly testable, and the shipping defect is cited with a file and line.
- **FR-18 `:464`.** Raw-lines-not-synthesized is diffable; the two-record set is countable; the derived-view carve-out is explicitly additive.
- **FR-22 `:542` and NFR-8 `:633`.** Both AD-12 bindings are true citations — they narrow the space of causes and add no obligation the FRs do not already carry. `:544`'s "Nothing above changes" is exactly the right thing for a reconciliation note to say.
- **FR-21 `:530`'s first half.** Taking FR-21's shipping predicate as AD-30's source of truth, rather than inventing a second one, is the correct direction of authority and matches `:16`.
