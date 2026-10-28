# Reviewer Gate — adversary lens (paired-story divergence), spine revision 12

- **Spine:** `architecture-Pack-Manager-2026-07-23.md`, `artifact_revision: 12` (frontmatter line 12), 1633 lines, read in full.
- **Units attacked:** `tickets/epic-upgrade-plan-redesign/tickets/tickets.toml` (1a–5e, read in full) and `tickets/epic-shipped-behavior-gaps/tickets/tickets.toml` (66, 22, 25, 35, 31, 32, 34, 65, read in full).
- **Tree:** `main` at `ce86f2c`, clean. Code facts below were read from `src-tauri/src/` and `src/` at that commit.
- **Method:** for each finding, two real ticket entries that each satisfy every AD as written, and the place where they still build incompatibly. **Verified** = read in the file cited. **Inference** = my reasoning about what a compliant builder would do. Spine line numbers are this file at revision 12. (The spine's Citations convention forbids line numbers *inside* the spine. This review is not the spine, and the brief asked for line refs.)
- **Out of scope by rule:** D37 keyboard/VoiceOver items and the D33 readiness gate. Neither is raised.
- **Not re-raised:** rev10 C-1/C-2 (AD-28 taxonomy and predicate owner), H-1 (terminal-append owner), H-2 (export raw vs fold), H-3 (`PlanAttempt.state`), H-4 (narrow-width overlap), H-5 (`Cancelling`). All are closed in revision 12. A finding below that touches one of these is a residual the closure left behind, and it says so.

## Verdict

**Not safe to build past wave 1 without tightening.** I found 1 CRITICAL, 5 HIGH, 4 MEDIUM and 2 LOW findings, 12 in total. Every one is a pair of real entries that obey every AD and still diverge. Waves 2–4 hold most of them. That is where AD-29 and AD-30 left a closed enumeration or an unowned transition behind.

| # | Sev | Pair | Hole |
|---|---|---|---|
| C-1 | CRITICAL | 2b × 2e (× 3d) | Normal-completion terminal transition has no owner from wave 2 until 3d. The active slot never frees, or an unverified terminal record is written that can never be superseded |
| H-1 | HIGH | 66 × 3f / 2d / 3c | AD-30 lists the guard's active set by name (`Queued` ∪ `Running`) while AD-16 adds non-terminal states outside it. Quit and app-update go unguarded during `Interaction required` and pre-Operation verification |
| H-2 | HIGH | 2c × 4a × 4e | The fold has no rule for a terminal record with no admission record, and both nonfatal-append and compaction can produce one |
| H-3 | HIGH | 3c × 3f × 4b | "Durable, journaled" `Verifying` and `Interaction required` have no record shape in a start/finish Operation journal. Two record encodings are possible, and one replays a dead process as live |
| H-4 | HIGH | 1c × 35 | AD-28's snapshot token has no identity, owner or wave. The obvious candidate, the coordinator revision, rejects every batch during an active attempt |
| H-5 | HIGH | FR-23 route × every UX-PB story | `Run fix` (HealthFix) is an immediate-execution path the PRD routes through the plan. AD-16 does not retire it, PlanIntent has no member type for it, and no ticket owns it |
| M-1 | MEDIUM | 2c × 4a | Compacting through typed re-serialization (the discipline 2c is licensed to copy) erases unreadable terminal lines and unknown fields. That turns "unreadable evidence" into "genuine absence" |
| M-2 | MEDIUM | 32 × 1a / 1d | Greedy casks are classified as item-ineligible but depend on an intent-level option, and they ship as a composition exclusion. The toggle has no removal class and no provenance |
| M-3 | MEDIUM | 34 × 3b × 35 | `ActivityDrawer` retirement and the `⌘L` re-point have no owning ticket. Activity can end up with two homes |
| M-4 | MEDIUM | 34 × 2c / 3d | Verification refreshes are bound from live settings at mutation exit, so the reviewed snapshot AD-21 says covers them is not what runs |
| L-1 | LOW | 3a × 5d | Two tickets each build the below-720px safety indicator, and no convergence rule covers it |
| L-2 | LOW | 1a × 4d | Draft previews and retry previews share one 64-slot FIFO issued-plan store, so staging during a retry review can evict the retry `planId` |

---

## C-1 — CRITICAL — No ticket owns an attempt's normal-completion terminal transition from wave 2 until 3d

**Pair:** **2b** (atomic admission, one active attempt) × **2e** (terminal-record owner, via cancellation). **3d** is the far end.

**What each obeys (verified):**
- AD-16, spine 504–506: "Exactly one confirmed attempt may be active. A second confirmation fails closed with a typed already-active result."
- AD-29, spine 1371–1375: the terminal append "fires on the attempt's terminal transition inside the store … owned by whichever story first makes an attempt reachable terminal — **UX-PB.2e**, via cancellation".
- AD-29, spine 1474–1480: a record is never mistaken for liveness, because a dead attempt resolving as live "refuses **every** subsequent confirmation, permanently".
- 2b ACs (tickets.toml 269–277) cover minting and the second-confirmation refusal. They say nothing about when an attempt stops being active.
- 2e ACs (389–394) cover terminal *by cancellation only*.
- 3d (566–568) is the first ticket that makes an attempt terminal on normal completion, "when the required refresh verification … completes". It ships in wave 3 after 3a–3c.

**Why the interim build is real (verified):** AD-29 spine 1379–1386 justifies its own override on the ground that "each UX-PB story is a shippable slice". After 2b lands, the shipping `UpgradePlanSheet` still calls `execute_plan` (5a has not replaced it), so users confirm real attempts in waves 2–3.

**Divergence (inference):** the 2b builder has three moves, and none is lawful.
1. **No terminal transition.** The first attempt that finishes normally keeps the active slot, and every later confirmation fails closed until relaunch. That is the wedge AD-29 1478–1480 calls permanent.
2. **Terminal when all Operations are terminal, with a terminal record.** That record is written before verification exists. It claims an outcome that AD-16 511–513 says is "not successful until the required affected Manager refreshes complete". AD-29 1442–1444 makes any later terminal record "duplicate evidence and **never** a state change", so 3d can never correct it.
3. **Terminal with no record**, because 2e does not exist yet or its ACs cover only the cancel path. The attempt persists admission-only and folds to `Interrupted` (AD-29 1438–1441). This is rev10 H-1's defect, coming back through the normal-completion path instead of the cancel path.

Rev10 H-1 assumed normal completion is "completed by UX-PB.3d". That is true for the finished product. It leaves waves 2–3 with no lawful build.

**Tighten (AD-29 plus AD-16, owner decision needed on the outcome value):**
> An attempt leaves the active slot exactly when it reaches a terminal transition, and every terminal transition — cancellation, normal completion, hard-cap timeout — is made by the plan-attempt store and owned by **UX-PB.2b**, which first makes an attempt admissible. Until verification exists (UX-PB.3d), normal completion transitions to terminal with verification recorded as **not performed**. A terminal record never asserts verification it did not perform, and "not performed" is a distinct, durable outcome — never read later as success. UX-PB.2e adds the cancellation path to the same transition, and UX-PB.3d inserts `Verifying` before it.

The alternative is to resequence 3d's completion limb into wave 2. Either way the AD must name one owner for the transition, not only for the append.

---

## H-1 — HIGH — AD-30's guard set is a closed list of two states, while AD-16 adds non-terminal states outside it

**Pair:** **66** (quit guard) × **3f** (`Interaction required`), plus **2d** (pre-Operation verification refresh) and **3c / 3d** (`Verifying`).

**What each obeys (verified):**
- AD-30 spine 1489–1494: the guard fires on "**any quit that would orphan a live child process**".
- AD-30 spine 1495–1500 then fixes the set by name: "The guard's active set is `Queued` ∪ `Running`, identical to the app-update guard's, and the two may not drift apart."
- 66 ACs (shipped-gaps 28–32) restate it literally.
- Shipping code lists the same two states by name. `refuse_app_update_while_busy` matches `OpStatus::Queued | OpStatus::Running` (`commands.rs:772–780`), and `activeOps` filters `"queued" || "running"` (`src/store/operations.ts:134–137`).
- AD-16 spine 522–535 makes `Verifying`, `Skipped` and `Interaction required` new `OpStatus` variants. Today there are seven (`ipc.rs:99–107`).
- 3f (643–645) enters `Interaction required` on a **running** child that is waiting at a prompt.
- 2d (357): "A verification refresh has no `opId` until it becomes an Operation". So there is a window in which verification is pending and no Queued or Running Operation exists.

**Divergence (inference, grounded in the code above):** 66 ships the literal two-state set. 3f adds `InteractionRequired` and touches neither predicate, because no AD or AC tells it to. AD-3's atomic change covers the enum, the types, the guards *map* and the fixtures. It does not cover these two business predicates. The result:
- A quit while an Operation is in `Interaction required` gets no dialog, which breaks AD-30's first rule.
- An app-update restart is allowed at that moment. AGENTS.md says that is exactly what lets "an app update destroy in-flight work".
- The same happens during the attempt-level verification gap, which leaves the attempt admission-only and folding to `Interrupted`.

The one state AD-17 (813–822) names as safety-critical becomes the one state the quit guard ignores. AD-30's two rules contradict each other as soon as AD-16's states land, and neither says which wins.

**Tighten (AD-30, replacing the `Queued ∪ Running` sentence):**
> The guard's active set is **every non-terminal Operation status**, defined as the complement of a closed terminal set (`Succeeded`, `Failed`, `Cancelled`, `TimedOut`, `Interrupted`, `Skipped`), **plus any attempt that is non-terminal in the live process**, whether or not it currently has a Queued or Running Operation. Both predicates — `refuse_app_update_while_busy` and `activeOps` — are written as that complement, never as a positive list. An `OpStatus` addition must classify the new variant as terminal or non-terminal in the same AD-3 change.

---

## H-2 — HIGH — The attempt fold has no rule for a terminal record with no admission record, and two shipped mechanisms produce one

**Pair:** **2c** (admission record; "reconstructs the attempt only from durable plan-admission metadata that actually persisted", tickets.toml 316–318) × **4a** (fold to one History row, 732–735). **4e** (legacy labeling) and **4d** (Retry restricts "the source attempt's reviewed intent", 832) are involved too.

**What each obeys (verified):**
- AD-29 spine 1438–1446 gives the fold three cases: admission plus terminal, admission only, and a duplicate terminal. **Terminal-only is not defined.**
- AD-29 1422–1428 makes the admission append nonfatal, and 2c 312–313 restates it: "never fatal to admission". The attempt runs anyway, and 2e appends a terminal record. That produces a terminal-only attempt. This path is reachable with nothing but a transient write failure.
- AD-18 851–855: "The two journals share a retention policy … A record that loses its counterpart reads as legacy, never as corrupt." **No ticket owns this policy.** `grep -i "retention\|compact"` over both tickets.toml files finds only reader-side mentions (2c 297, 4b 756) and AD-25's unrelated "snapshot retention".
- The shipping Operation compaction counts Operations and knows nothing about attempts: `COMPACT_KEEP = 1000` and `compact()` (`journal.rs:19`, `:185–207`).
- AD-18 844 says "Existing retention bounds are unchanged". The attempt journal has no existing bound, so 2c picks one.
- A count- or line-based compaction of the attempt journal that 2c designs alone can drop an admission line and keep its terminal line.

**Divergence (inference):**
- 2c reads "only from durable plan-admission metadata" as "no admission, no attempt", so the attempt is invisible.
- 4a folds the terminal record into a History row with an outcome and no scope or commands.
- 4e follows AD-18's "loses its counterpart reads as legacy" and labels it a legacy entry. But "legacy" is defined only for Operations without `planAttemptId` (spine 709–711, 839–841), and this record carries one.
- 4b replay has nothing to reconstruct, and 4d Retry has no reviewed intent to restrict.

That gives three compliant renderings of one record, and 65 exports the raw line under whichever reading it picks.

**Tighten (AD-29 fold rule, plus AD-18 owner):**
> Terminal with no admission yields an attempt whose outcome is known and whose **reviewed scope is unavailable**. It is listed, it is not replayable, and it is not retryable, and it is never relabeled legacy. Legacy is reserved for records without a `planAttemptId`. The shared retention policy is owned by **UX-PB.2c**. It drops and keeps **whole attempts** in both journals together — never one record of a pair, and never an attempt's Operations without the attempt. It also states the attempt journal's bound.

---

## H-3 — HIGH — "Durable, journaled" `Verifying` and `Interaction required` have no record shape in a two-line Operation journal

**Pair:** **3c** (per-item `Verifying`, 529–535) × **3f** (`Interaction required`, 643–645). **4b** (replay must reconstruct what the user saw, 763–766) is the reader.

**What each obeys (verified):**
- AD-16 522–535: `Verifying`, `Skipped` and `Interaction required` "are journaled, exported in diagnostics, and replayed from History … replay must reconstruct what the user saw".
- AD-29 1412–1416 puts them in "the **Operation** journal … the level where transitions are recorded".
- AD-29 1393–1397 holds up the Operation journal as the model *because* it is "One line at op start, one at finish". The file header confirms it at `journal.rs:4`.
- The loader keeps the last finish line per `op_id` (`journal.rs:135–138`).
- `load_records` takes `status` from that finish line's `outcome` and maps only a *missing* finish to `Interrupted` (`journal.rs:151–155`).
- Spine 1611 warns that the `#[serde(untagged)]` `Line` enum (`journal.rs:50–55`) cannot tell overlapping shapes apart, but it warns for the **attempt** journal only.

**Divergence (inference):** the spine forces a per-transition record into a journal it describes as having none, which leaves two compliant encodings.
- **(a)** 3c writes `Verifying` as an intermediate `FinishRecord` with `outcome: Verifying`, and the loader's last-wins behavior makes this look harmless. A crash during verification then leaves the op's last durable status as `Verifying`. On relaunch `load_records` replays it as `Verifying`, not `Interrupted`. That contradicts AD-16 526–527 ("crash-reconstructed unfinished work stays `Interrupted`") and AD-29 1474–1477's liveness rule, and it feeds `deriveManagerPhase()` a Manager that looks busy forever.
- **(b)** 3f adds a third "transition" line kind for `Interaction required`. Under the untagged enum it deserializes as whichever shape fits first.

3c and 3f can each pick differently and both still comply. 4b then reconstructs from two encodings.

**Tighten (AD-29, Operation-level rule):**
> The Operation journal gains an explicit, discriminated **transition record** for each durable non-terminal state, `Verifying` and `InteractionRequired`. A `FinishRecord` carries only a terminal outcome. On relaunch, any op whose last durable record is non-terminal reconstructs as `Interrupted`, keeping the transition as replay evidence and never as current status. The record discriminator is added once, by whichever of UX-PB.3c or UX-PB.3f lands first, as one AD-3 change.

---

## H-4 — HIGH — AD-28's snapshot token has no identity, no owner and no wave, and the obvious candidate rejects every batch during an attempt

**Pair:** **1c** (header checkbox, Manager-wide, `Update Everything` — filter-wide interactions, wave 1, 111–126) × **35** (batch plus snapshot token, shipped-gaps 176–177).

**What each obeys (verified):**
- AD-28 1236–1243: a filter-wide interaction "is one membership operation … Adding the batch verb is one atomic surface change under AD-3".
- AD-28 1331–1334: the frontend submits identities "**plus the snapshot token it read**; Rust rejects a batch whose token is not its current snapshot".
- AD-28 1258–1264 contradicts this: a batch "is rejected outright **only** when the rebuild **errors**". A token mismatch is a second rejection that is not a rebuild error.
- No snapshot token or revision exists on the wire. `grep -i "snapshot_token\|snapshotToken\|generation\|revision"` over `ipc.rs` and `types.ts` returns nothing.
- `grep -i token` over the UX-PB tickets returns nothing. Only 35 carries the token, and 35 lands after 1a, 3.1 and 3.2.
- The only existing epoch is `PlanCoordinator.revision`. It bumps on every enqueue, plan-batch admission, cancel, op finish and refresh publication: `queue.rs:996`, `:1037`, `:1187`, `:1287`, `:1772`.
- AD-17 742–745 requires staging to keep working during a live attempt.

**Divergence (inference):**
- 1c ships the batch verb with no token, and that is compliant because 1c's ACs do not mention one. 35 then has to change the same verb, which breaks AD-28's "one atomic surface change".
- If either ticket picks `revision` as the token, every header-checkbox click during an active attempt races the attempt's own status churn and is spuriously rejected. Range staging becomes nondeterministic exactly when AD-17 promises it works.
- If 35 picks a registry-snapshot generation instead, that is a new epoch with no owner, and it must be projected together with the predicate result (1331–1342). That projection is itself unassigned: 1d builds the denominator, 31 reads the row predicate, 35 builds the payload.

**Tighten (AD-28):**
> The snapshot token is the **inventory epoch** — advanced only by a published Manager snapshot or a detection change, never by queue-record changes. It ships with the first batch verb (**UX-PB.1c**), together with the projected predicate result. A token mismatch is a **rejection that preserves the prior draft and preview**, and is listed beside rebuild error as the second, and only other, outright-rejection case.

---

## H-5 — HIGH — `Run fix` is an immediate-execution path the PRD routes through the plan, and no AD or ticket covers it

**Pair:** **1c / 1e / 3.1** (every staging entry point, "target is zero immediate-execution kinds", 1c notes 133) × **the FR-23 HealthFix route**, which no entry owns.

**What each obeys (verified):**
- AD-16 467–477: "No entry point executes" lists row, Manager-header, Manager-wide and `Update Everything`, and retires "**Three** shipping call sites": the row action plus both self-update paths. `Run fix` is absent.
- `prd.md` FR-23 (around line 367–368): "**Planned — D27–D30:** this path routes through the persistent Upgrade Plan and the confirmation gate like every other mutation, at which point the set above collapses to zero."
- `docs/RELEASE-CHECKLIST.md` (around lines 61–66) lists `HealthBanner.tsx:43` as a gate bypass that "D27–D30 routes … through the plan".
- The epic's Boundaries state "the target is zero kinds".
- The call is live: `HealthBanner.tsx:43` calls `runHealthFix` → `run_health_fix` (`client.ts:63–64`).
- `grep -i "healthbanner\|run fix\|healthfix\|health fix"` over both tickets.toml files returns **zero** hits.
- The normative PlanIntent (586–600) has only `packageUpdates` and `managerUpdates`.

**Divergence (inference):**
- Every UX-PB ticket can ship fully compliant while `Run fix` still executes immediately. AD-16's enumeration permits it, and that breaks the PRD and the epic boundary.
- If any ticket does route it, that ticket has to invent a third member kind, and a member-shape change outside AD-23's "lands the complete shape" rule (1011–1014) is an uncoordinated AD-3 change.
- During an active attempt, an immediate HealthFix also takes a mutating lock outside the attempt. 2b/5a admission then fails on "lock-set overlap" (AD-16 499–501), with no reason the user can trace.

**Tighten (AD-16 plus owner decision):** either
> `Run fix` is a fourth retired call site; `PlanIntent` gains `healthFixes: PlanMember<HealthFixRef>[]` as part of AD-23's atomic shape; owned by UX-PB.1c,

or, if the owner keeps it immediate,
> `Run fix` remains the single deliberate immediate-execution kind (FR-23's closed set, now size one), and FR-23's "Planned" limb and the epic's zero-kinds boundary are corrected to say so.

The spine must pick one of these. At present the PRD and the epic say one thing and the AD says another.

---

## M-1 — MEDIUM — Typed re-serializing compaction, which 2c may copy, erases what AD-29 says must stay distinguishable

**Pair:** **2c** (attempt-journal file discipline) × **4a** (fold must tell "unreadable" from "absent", 732–735).

**Verified:**
- Spine 1611: "UX-PB.2c may copy the file discipline from `journal.rs`".
- The shipping `compact()` re-parses through `load_entries()`, which silently skips any line that fails to deserialize (`journal.rs:140`, "lenient: skip garbage lines"). It then writes back only `serde_json::to_string(&start/&finish)` of the typed structs (`journal.rs:193–206`).
- `StartRecord` has no catch-all extra-fields slot (`journal.rs:26–38`).
- AD-29 1447–1456 requires a schema-failing terminal line to stay "terminal-evidence-unreadable".
- AD-19 873–881: unknown fields are tolerated, and a record is "never silently replaced".

**Divergence (inference):** a compliant 2c that copies the discipline turns every unreadable terminal line into a genuine absence on the first compaction. 4a then reads `Interrupted`, which is the reclassification AD-29 forbids. The same compaction strips any unknown field, including a field a newer build wrote, so AD-19's read tolerance becomes write-side erasure. The second half already holds for `operations.jsonl` today.

**Tighten (AD-18 or AD-19):**
> Compaction copies retained lines **byte-for-byte**; it never round-trips them through a typed model. Lines it cannot parse are retained (or dropped only with their whole attempt under H-2's policy) and counted.

## M-2 — MEDIUM — Greedy casks sit in two ineligibility classes, plus an intent-level scalar AD-23's own logic should have removed

**Pair:** **32** (greedy-only casks "enter a plan only through explicit opt-in", shipped-gaps 275–277) × **1a / 1d** (inertness for "excluded" rows, reason "excluded by your Settings", 157).

**Verified:**
- AD-16 668–669 lists "a non-opted-in greedy cask" as *item* ineligibility that "can never enter a `PlanIntent`".
- AD-16 698–703 says item ineligibility is "a property of the item alone", while composition exclusion "depends on what else is in the plan".
- A greedy cask's eligibility depends on `PlanIntent.includeGreedyCasks` (594), which is an intent-level boolean.
- The shipping code treats it as a composition exclusion: `ExclusionReason::GreedyCask` (`ipc.rs:142–147`), alongside `include_greedy_casks` on the plan wire (`ipc.rs:364`).
- A third locus, the `include_greedy_by_default` setting (`settings.rs:35`), is plan-determining under AD-21.

**Divergence (inference):**
- 32 builds greedy casks as item-ineligible: an inert row, outside the intent. Turning the opt-in on then **adds** members, but which origin do they get? None of AD-23's three scopes fits, and AD-28 1277–1288 has no removal class for turning it off.
- 1a keeps the shipping shape, where greedy casks are members excluded in the preview. Turning the opt-in on then enlarges what executes without adding a member, and AD-16 484–491 only polices membership.

**Tighten (AD-16 / AD-23):** decide one class. Then either replace `includeGreedyCasks` with per-member opt-in, or give the toggle a named taxonomy class and an origin.

## M-3 — MEDIUM — No ticket retires `ActivityDrawer` or re-points `⌘L`

**Pair:** **34** (removes "the Activity auto-open preference … from active Settings", shipped-gaps 312) × **3b** (Activity as a destination, 496–498). **35** removes `Esc`'s rungs (183).

**Verified:**
- AD-17 789–794: "The existing `ActivityDrawer` surface retires with the `autoOpenDrawer` setting; no story keeps it alive as a second home."
- AD-17 795–802: `⌘L` focuses the sidecar.
- `grep -i "drawer"` over both tickets.toml files finds only `autoOpenDrawer` (34, 5b). `grep "⌘L"` finds nothing.

**Divergence (inference):** "No story keeps it alive" is a prohibition with no one assigned to do the removal. 34 removes the setting, 3b adds the destination, and 35 deletes `Esc`'s drawer rung on the premise that the drawer is gone. None of them deletes the drawer. The shipped result has two homes for attempt status, and `⌘L` still toggles the drawer, which contradicts AD-17. RELEASE-CHECKLIST 9a explicitly expects this split only while the drawer is "shipping".

**Tighten (AD-17):** name **UX-PB.3b** as the owner of deleting `ActivityDrawer` and re-pointing `⌘L` to the focus move, in the same change that makes Activity a destination.

## M-4 — MEDIUM — The reviewed snapshot covers verification-refresh argv for drift, but the refresh is bound from live settings after admission

**Pair:** **34** (settings "persist before becoming active", 309–312) × **2c / 3d** (the reviewed "exact command snapshot", 306; verification refresh, 566–568). **22**'s Homebrew metadata toggle shapes refresh phases (79–82).

**Verified:**
- AD-21 913–919 counts verification-refresh argv and timeouts as plan-determining, "because the user reviewed a plan that runs under them".
- Mutating commands are bound at plan time (`queue.rs:86–104`).
- Post-mutation refreshes are built by `refresh_factory` when the mutation finishes (`queue.rs:1275–1285`), and therefore from whatever `run_brew_update_on_refresh` and `stall_after_secs` hold at that moment (`settings.rs:29–31`).

**Divergence (inference):** AD-21 only protects the *preview*. After admission, a 34-compliant settings change applies immediately, and the verification then runs argv the admission record never captured. That is evidence that is correct by AD-21's letter and wrong in the archive 65 exports.

**Tighten (AD-21):**
> Admission binds the verification refreshes' argv and timeouts into the reviewed snapshot; a settings change after admission never reaches an admitted attempt's remaining work, including its verification refreshes.

## L-1 — LOW — Two tickets each build the below-720px safety indicator

**Pair:** **3a** (464) × **5d** (1053–1055). Both ACs require "a persistent, non-occludable indicator". AD-17 779–788 states the indicator, and its "exactly one" convergence rule (823–827) covers announcement channels only (verified). 3a also builds the indicator before stall handoff (3b/3c) and `Interaction required` (3f) exist, and before 5d builds the stacked layout it appears in. **Tighten:** name one owner, presumably 5d, and extend AD-17's exactly-one rule to the indicator.

## L-2 — LOW — The retry preview can be evicted by staging

**Pair:** **1a** (every draft mutation issues a preview) × **4d** (retry preview under review). Verified: `ISSUED_PLAN_LIMIT = 64` with FIFO eviction (`state.rs:25`, `:156–170`), and an evicted `planId` fails closed (spine 482–483). AD-24 1040–1042 explicitly permits staging while a retry is under review. Enough staging evicts the retry's `planId`, and its Confirm then fails closed with no visible cause. AD-17's union also says "a derived intent under review" in the singular (749–751) but sets no cardinality for a second Retry. **Tighten (AD-24):** a derived intent's preview is held apart from the draft's eviction order, and at most one derived intent is under review at a time.

---

## Attacks run that found no hole

- **Draft store vs Rust canonical intent:** AD-17 720–723, AD-24 and AD-28 1230–1235 close it. The only residual is the ordering of out-of-order responses, which "Confirmation is unavailable while a rebuild is in flight" (660–667) contains.
- **`skipUpgradePlanConfirmation`, dialog rider vs Settings:** AD-21 926–927 and AD-22 960–977 close it. 34 is after 5b, so AD-21's "whichever lands first" (932–935) resolves to 5b.
- **Diagnostics raw lines vs fold:** closed by AD-29 1429–1437 (rev10 H-2). The residuals are H-2 and M-1 above.
- **App update vs Package plans (5e):** AD-16 573–579 closes it.

## Proposed AD changes, in priority order

1. **AD-29 / AD-16:** one owner for every terminal transition, plus a "verification not performed" outcome until 3d (C-1).
2. **AD-30:** active set = complement of a closed terminal set, plus any non-terminal attempt, and both predicates written that way (H-1).
3. **AD-29:** fold rule for terminal-only, and **AD-18:** a whole-attempt retention policy owned by 2c with a stated bound (H-2).
4. **AD-29:** a discriminated Operation transition record, and non-terminal-last reconstructs as `Interrupted` (H-3).
5. **AD-28:** define the token as the inventory epoch, owned by 1c, and list token mismatch as the second rejection case (H-4).
6. **AD-16:** decide `Run fix` (H-5). This is an owner decision.
7. **AD-18/19:** byte-preserving compaction (M-1). **AD-16/23:** one greedy class (M-2). **AD-17:** drawer/`⌘L` owner (M-3) and indicator owner (L-1). **AD-21:** bind verification argv at admission (M-4). **AD-24:** keep the derived preview out of the draft's eviction order (L-2).
