# Reconciliation review — `epics.md` against ARCHITECTURE-SPINE.md revision 4

**Date:** 2026-07-25
**Spine:** `_bmad-output/planning-artifacts/architecture/architecture-Pack-Manager-2026-07-23/ARCHITECTURE-SPINE.md` (505 lines, revision 4)
**Input:** `_bmad-output/planning-artifacts/epics.md` (1166 lines)
**Scope:** Epic UX-PB (28 stories, lines 384–1009) and the six surviving stories 2.2, 3.1, 3.2, 3.4, 3.5, 6.5 (lines 1015–1166).

## Method

Both files read in full. Story count verified: `grep -c "^### Story UX-PB\." epics.md` → `28`, matching spine:13 `- Epic UX-PB (28 stories)`. Surviving-story headers verified: `grep -n "^### Story [0-9]"` → 2.2, 3.1, 3.2, 3.4, 3.5, 6.5 at lines 1015, 1043, 1066, 1088, 1112, 1141.

Every absence claim below was confirmed by grep against the spine. The terms **`hard cap`, `30-minute`, `inherited`, `Desktop`, `5,000`, `5000`, `720`, `900`, `zoom`, `Health`, `Done`, `Retry refresh`, `IN PLAN`, `phase order`, `log level`, `logLevel`, `grid`** return **zero hits** in `ARCHITECTURE-SPINE.md`. Where a term returns hits in a different sense, that is stated explicitly.

---

## Verdict

The spine's *domain* coverage of UX-PB is strong: AD-16, AD-17 and AD-18 govern plan identity, draft ownership, and durable attempt storage well. What the AD structure dropped is almost entirely the **presentational, accessibility, copy, ordering, and numeric-constant layer** — plus three places where an AD as written cannot be satisfied together with a story.

One contradiction is load-bearing enough to block UX-PB.3: **AD-17's sidecar rule and the UX-PB.3a/3d lifecycle are mutually unsatisfiable as written.**

---

## A. ADs that contradict a story's acceptance criteria

### A-1 [CRITICAL] AD-17's sidecar rule cannot satisfy UX-PB.3a and UX-PB.3d together

Spine, `ARCHITECTURE-SPINE.md:371-374`:

> "**Rule:** The sidecar is a single layout region whose visibility is driven by
> draft non-emptiness — not a `ui.dialog` kind and not a `DialogHost` child.
> Exactly one instance exists and it persists across `ActiveView` changes without
> losing membership or scroll identity. When hidden, the workspace reclaims its
> width with no reserved empty column. A confirmed attempt replaces the sidecar's
> content with live attempt status rather than opening a second surface."

Story UX-PB.3a, `epics.md:651-653`:

> "**When** the user keeps reviewing a draft or attempts a second confirmation
> **Then** only one confirmed Upgrade Plan attempt is active — the new draft stays in the Upgrade Plan and cannot be confirmed until the active attempt is terminal, and no second live summary is created."

Story UX-PB.3d, `epics.md:707`:

> "**Then** the attempt becomes terminal, the sidecar transforms in place into a persistent Results Summary that remains until `Done`, focus preserves the current viable node or moves to the Results heading, and one atomic outcome summary is announced (e.g. `12 of 12 updates verified` or `10 of 12 verified · 2 failed`)."

Three states must coexist in one region: a live confirmed attempt, a **new draft the user is still building** (3a), and a **persistent Results Summary that outlives the attempt until `Done`** (3d). AD-17 says exactly one instance exists, that its content is *replaced* by attempt status, and that its **visibility is a function of draft non-emptiness** — a predicate that is false during a Results Summary with an emptied draft, and that says nothing about where a new draft renders while the region is occupied by attempt status.

A builder following AD-17 literally will hide the Results Summary the moment the draft empties, and will have nowhere to put the new draft UX-PB.3a requires. `Done` — the action that terminates the Results Summary — returns **zero hits** in the spine.

### A-2 [HIGH] AD-16's inertness wording admits the build UX-PB.1d forbids

Spine, `ARCHITECTURE-SPINE.md:346-349`:

> "**Ineligible-item inertness.** An item that is pinned, already current, a
> non-opted-in greedy cask, or removed between staging and rebuild is inert: its
> control is non-interactive to pointer and keyboard, it carries a stated reason
> for assistive technology, and it can never enter a `PlanIntent`."

Story UX-PB.1d, `epics.md:484`:

> "**Then** it uses `aria-disabled=\"true\"` rather than native `disabled`, keeps focus, announces its persistent reason as an accessible description, stays inert on activation, and retains focus when Escape closes its supplemental Tooltip/Popover."

Native `disabled` is the most direct way to make a control "non-interactive to pointer and keyboard" — and it removes the control from the tab order, destroying the "keeps focus" and "announces its persistent reason" requirements UX-PB.1d states. AD-16 as written does not forbid the build UX-PB.1d explicitly forbids. `aria` returns no substantive hits in the spine (only incidental substring matches in words like "scope", "variable").

This becomes a live two-story conflict — see **B-1**.

### A-3 [HIGH] AD-19's "degrades to defaults" binds all persistence and contradicts UX-PB.2c / UX-PB.4b

Spine, `ARCHITECTURE-SPINE.md:402` and `:406-408`:

> "- **Binds:** UX-PB.5b; Story 3.4; all persistence"

> "**Rule:** Reading a persisted file tolerates unknown and retired fields and
> never fails the application. A corrupt file degrades to defaults with a visible
> notice — the shipping behavior for `settings.json`."

Story UX-PB.2c, `epics.md:575-577`:

> "**Given** a persisted attempt whose command snapshot is later read as corrupted or incomplete
> **When** the record is loaded
> **Then** the integrity failure is detected and the snapshot is refused as an execution source, blocking any display-to-executable round-trip so a damaged snapshot can never be silently re-run."

Story UX-PB.4b, `epics.md:821-823`:

> "**Then** the load failure states what could not be reconstructed, the History list stays intact and navigable, and no partial reconstruction is presented as a complete or trustworthy replay."

"Degrade to defaults" is correct for `settings.json` and **fabrication** for a plan-attempt record — a "default" History row or a "default" command snapshot is precisely the invented evidence UX-PB.2c, UX-PB.4a and UX-PB.4b exist to prevent. The trailing clause "— the shipping behavior for `settings.json`" reads as an example, not a scope limit, and the `Binds:` line says "all persistence". The rule needs an explicit carve-out: evidence records fail *visible-and-refused*, not *defaulted*.

---

## B. Two stories whose criteria can be satisfied incompatibly under the current ADs

### B-1 [HIGH] Story 3.2 says "disabled"; UX-PB.1d says never native `disabled`

`epics.md:1082` (Story 3.2):

> "**Then** pinned rows stay inert, add nothing to the draft Upgrade Plan, and are explained, disabled, and excluded from every plan with the correct reason."

`epics.md:484` (UX-PB.1d): "it uses `aria-disabled=\"true\"` rather than native `disabled`".

Both stories describe the same pinned Homebrew row. Under AD-16's "non-interactive to pointer and keyboard" (A-2), Story 3.2 can be closed with native `disabled` and pass its own criteria while breaking UX-PB.1d's. Nothing in the spine forces the two builds to converge.

### B-2 [HIGH] UX-PB.3d's item taxonomy has no slot for UX-PB.2e's un-stoppable work

`epics.md:709-711` (UX-PB.3d):

> "**Then** the overall outcome is exactly one of success, partial, failed, cancelled, timed out, or interrupted, and each item is verified, failed, cancelled, or skipped — mutation failure and verification failure are distinguished, `Skipped` marks only work that never started, and crash-reconstructed unfinished work reads as `Interrupted`."

`epics.md:614-617` (UX-PB.2e):

> "**Given** a plan cancellation where process-group escalation cannot stop some running work
> **When** the escalation partially fails
> **Then** the work that could not be stopped is reported honestly and never falsely marked cancelled, the successfully cancelled and skipped outcomes remain preserved
> **And** no terminal outcome is fabricated for work whose true state is unknown."

UX-PB.3d closes the per-item vocabulary at four values. UX-PB.2e requires a fifth condition that is explicitly *not* `cancelled` and explicitly *not* terminal. The spine adopts 3d's side implicitly — `ARCHITECTURE-SPINE.md:267` says "every terminal state stays durable" and the normative state list at `:312` is `state: admitted | running | verifying | terminal` — and never provides the honest-unknown state.

### B-3 [HIGH] UX-PB.3g requires a cancel-while-verifying transition the spine's state machine does not contain

`epics.md:779-781` (UX-PB.3g):

> "**Given** an attempt in the verifying window with processes exited and refresh verification pending (cancellation while verifying)
> **When** `Cancel plan` is issued
> **Then** cancellation is honored immediately for that `planAttemptId`, verifying items resolve to honest terminal outcomes (cancelled or skipped rather than falsely verified), and no item is reported successful because its exit preceded the cancel."

Spine state diagram, `ARCHITECTURE-SPINE.md:328-330`:

> ```
>     Running --> Verifying: processes exit, affected refreshes required
>     Verifying --> Terminal: Results distinguish mutation vs verification failure
>     Running --> Terminal: cancelled, unstarted work Skipped
> ```

There is a `Running --> Terminal: cancelled` edge and **no `Verifying --> Terminal: cancelled` edge**. The only exit from `Verifying` is via verification results. A builder implementing the diagram will either refuse cancellation during verification or fall through the verification-results path — both of which UX-PB.3g forbids. The diagram also has no edge expressing crash-reconstructed `Interrupted`, required by `epics.md:711` and `epics.md:804`.

### B-4 [HIGH] Story 2.2 requires a Setting that Story 3.4 declares does not exist

`epics.md:1029` and `:1023` (Story 2.2, which links **FR-17**, the Settings requirement):

> "**Given** Homebrew metadata refresh is enabled or disabled"

`epics.md:1102` (Story 3.4):

> "**Given** the retained editable stall threshold, hard cap, and log level plus `skipUpgradePlanConfirmation` (default `false`) as the configurable Settings — each with its default, valid bounds, invalid input, and a persistence failure …"

Story 3.4 enumerates a closed set of four "the configurable Settings" and does not include a Homebrew-metadata-refresh toggle. Story 2.2 requires that toggle to drive a rendered phase list. Story 3.4 can be closed with a four-control Settings surface that Story 2.2 cannot then exercise. The spine names only `skipUpgradePlanConfirmation` and `autoOpenDrawer` (`:409-410`) and no other setting — `hard cap`, `log level` and `logLevel` all return **zero hits** — so nothing arbitrates.

### B-5 [HIGH] Selection and draft membership are two layers; the spine models only one

`epics.md:1126-1129` (Story 3.5):

> "**Given** eligible, current, pinned, greedy, filtered, and range-addressable rows
> **When** toggle, shift-range, tri-state, Cmd+A, Space, Cmd-click, Clear, and Esc interactions execute
> **Then** the exact selectable identities and visible filter semantics are preserved
> **And** excluded rows never enter selection."

`epics.md:44` (FR-6): "…add exact identities to one persistent draft; keep the draft across Manager navigation; and never execute from selection or a row."

Spine, `ARCHITECTURE-SPINE.md:363-365` (AD-17):

> "**Rule:** Rust owns the canonical `PlanIntent`. The Zustand draft store is a
> projection of the last authenticated rebuild — never the authority, never the
> author of executable text. Every mutation round-trips through Rust before the
> projection updates."

FR-6 and Story 3.5 describe a **transient selection layer** (shift-range, tri-state, Cmd+A, Clear, Esc) that is distinct from durable draft membership. The spine's normative domain minimum (`:293-313`) contains `PlanIntent`, `UpgradePlanPreview`, `PlanAttempt` — and no selection concept at all. If a builder maps selection onto draft membership to satisfy AD-17, then a shift-range across 100 rows becomes 100 Rust round-trips, against `epics.md:88` (NFR-3) "remain interactive with more than 100 Package rows". If a builder keeps selection frontend-local, nothing in the spine sanctions it. The spine's Capability map at `:484` names "keyboard selection (Stories 3.1, 3.2, 3.5)" and routes it to "AD-16, AD-17" — neither of which mentions selection.

### B-6 [MEDIUM] UX-PB.3b evicts from the sidecar exactly what UX-PB.3f requires there

`epics.md:669-671` (UX-PB.3b):

> "**Given** the compact sidecar while an Operation needs attention
> **When** the condition is summarized there
> **Then** the sidecar offers `View full Activity` and defers `Keep waiting`, `Copy command`, `Cancel plan`, and expanded command evidence to full Activity rather than crowding the summary."

`epics.md:753-755` (UX-PB.3f):

> "**Given** a running Operation that has gone silent
> **When** no trusted classifier matches the output at the 120-second threshold
> **Then** the Operation remains an ordinary stall presenting exactly `Keep waiting`, `Copy command`, and `Cancel plan`, never `Interaction required`."

3f says the stalled Operation presents **exactly** those three actions; 3b says the sidecar summarising that same condition presents **none** of them. Whether "the Operation" means its full-Activity representation or its sidecar summary is unresolved, and the spine's only placement rule is `:267` "`Cancel operation` is reserved for an explicitly Operation-scoped diagnostic action" — a naming rule, not a placement rule.

### B-7 [MEDIUM] Four verbatim ineligibility strings live in one story; three others require "a reason"

`epics.md:479` (UX-PB.1d) fixes the exact copy:

> "**Then** membership never changes and each exposes its plain-language reason — pinned `This Package is pinned and cannot be updated. Unpin it, then refresh Pack-Manager to make it selectable.`, excluded `This Package is excluded by your Settings. Change the setting, then refresh Pack-Manager.`, current `This Package is already current.`, and unavailable `An update target is not available. Refresh or view details.`"

Three other stories require the same reason without the wording: `epics.md:1082` (3.2) "explained, disabled, and excluded from every plan with the correct reason"; `epics.md:1134` (3.5) "stay inert with an explained reason"; `epics.md:409` (1a) "the now-ineligible item is dropped or flagged with what changed". The spine reduces all four to `:348` "it carries a stated reason for assistive technology". Three stories can ship three different strings and each pass.

### B-8 [MEDIUM] Bulk-add scope: filter-limited or all-eligible?

`epics.md:480` (UX-PB.1d): "**And** the bulk header Checkbox scope covers only eligible Packages matching the active filter and adds no ineligible identity."

`epics.md:453` (UX-PB.1c): "`Update Everything` seeds all eligible work while remaining editable".

`epics.md:1081` (Story 3.2): "**When** selection, row plan-add, per-Manager update-all, update-selected, and Update Everything draft-entry paths are exercised across every active filter".

Two bulk controls with different scoping rules, plus a third story exercising five entry paths "across every active filter". The spine's inertness rule (`:346-349`) covers only which items may never enter a `PlanIntent`; it says nothing about whether an active filter narrows a bulk add. Nothing named "filter" appears in the spine.

---

## C. Acceptance criteria genuinely uncovered by any AD

### C-1 [HIGH] No AD governs focus, announcement, or high-zoom layout — and the Capability map admits it

`ARCHITECTURE-SPINE.md:482`:

> "| Confirmation gate and its setting (UX-PB.5a–5e) | `DialogHost` modal + settings persistence | AD-17, AD-19 |"

UX-PB.5d is entirely a focus/announcement/zoom contract. AD-17 is the sidecar/draft-ownership AD; AD-19 is the persistence-tolerance AD. Neither mentions focus order, announcements, or zoom — `720`, `900` and `zoom` return **zero hits** in the spine.

Dropped criteria, each quoted:

- `epics.md:980` — "**Then** below 720 usable CSS pixels the layout enters high-zoom mode, navigation collapses to an accessible rail or temporary panel, and Plan/Confirmation/Activity/Results present as a full-workspace or stacked surface with a visible Back route, no overlapping panes, and no two-dimensional scrolling for the primary task, keeping every safety action reachable." This is also in tension with AD-17's persistent side-by-side layout region (`:371-374`), which has no provision for the sidecar taking over the workspace.
- `epics.md:903` (UX-PB.5a) — "focus moves to the dialog heading/command summary with `Change Plan` as the first actionable control so a final confirmation is never the accidental default for an unfocused Enter". A safety-critical focus-order rule; the spine's only dialog rule is `:376-378` "`DialogHost` remains the single mount point for modal surfaces and shows one dialog at a time."
- `epics.md:903` — "Escape/backdrop dismiss only while no command has begun and restore focus to the originating `Confirm N Updates` action."
- `epics.md:983-984` (UX-PB.5d) — "focus is restored to a defined fallback (the first staged Remove control or the plan heading) rather than lost to the document body, and focus is never stranded inside a closed dialog."

**One shared live region, six stories, no owner.** `epics.md:427` (1b) "focus stays on the source control that created it"; `:649` (3a) "focus moves to its programmatically focusable Upgrade Activity summary heading, and the status channel announces plan start"; `:685` (3c) "a row or status update never moves focus"; `:707` (3d) "focus preserves the current viable node or moves to the Results heading, and one atomic outcome summary is announced"; plus 5a and 5d above. All six address the same sidecar/dialog pair, which transforms in place three times. Nothing in the spine states that a single status channel exists or who owns focus at each transition.

**Accessibility is expressed only as a release obligation.** The spine's sole accessibility rule is AD-11, whose `Binds:` line at `:203` is "release":

> `:223-227` "**Rule:** Accessibility is product quality in the existing lanes — automated
> 4.5:1 text contrast and reduced-motion checks in the Playwright/Vitest lane,
> and one manual VoiceOver focus-order and completion-announcement pass on the
> release checklist. Broader WCAG or legal compliance is not implied
> (`docs/DECISIONS.md` D33, restating the former DR-2)."

That states where accessibility is *checked*, not what the product must *do*. It also sits awkwardly beside AD-1's own rule at `:115-117`: "Missing or incorrect behavior is product work, not test work." `epics.md:94` (NFR-6) states the product obligation directly — "deterministic dialog/sidecar focus restoration … announce plan progress, verification, cancellation, failure, and completion without noisy output narration" — and it has no AD.

### C-2 [HIGH] The spine's normative `PlanAttempt` minimum omits fields two stories require

Spine, `ARCHITECTURE-SPINE.md:305-313`:

> ```
> PlanAttempt
>   planAttemptId: durable PlanAttemptId
>   retryOfPlanAttemptId?: PlanAttemptId
>   reviewedIntent + reviewedCommandSnapshot
>   operationIds[]
>   state: admitted | running | verifying | terminal
>   verificationResults + resultSummary
> ```

`epics.md:564` (UX-PB.2c):

> "**Then** the append-only record stores the reviewed Manager/Package scope, Manager self-update identities, exact command snapshot, version evidence, timestamps, and result/verification state as immutable plan-admission metadata"

`epics.md:818` (UX-PB.4b):

> "**Then** Activity enters a clearly labeled read-only replay that reconstructs the attempt's Manager groups, Package/version changes, Manager self-updates, exact commands, Operation outcomes, errors, timings, and retained output"

**`version evidence`, `timestamps` and `timings` are absent from the normative minimum.** The spine explicitly defers field lists at `:503` ("the exact filenames and field lists belong to UX-PB.1a and UX-PB.2c") — but the block at `:288-291` is titled "Normative domain minimum" and says "the semantic separation is fixed", so builders will treat it as the floor. A `PlanAttempt` built to that floor cannot satisfy UX-PB.4b's replay of timings, or UX-PB.4d's `Retry of plan from <time>` History entry (`epics.md:855`).

### C-3 [MEDIUM] Every stall, timeout, flush and retention constant is missing

`hard cap`, `30-minute`, `5,000` and `5000` return **zero hits** in the spine. The only `120` hit is `:185` "the 120s aging guard, and duplicate-refresh coalescing are preserved" — that is AD-4's **scheduler aging guard**, a different mechanism from the stall threshold.

Dropped constants:

- `epics.md:754` (UX-PB.3f) "no trusted classifier matches the output at the 120-second threshold" and `epics.md:60` (FR-14) "the 120-second default stall threshold … the 30-minute default hard cap".
- `epics.md:88` (NFR-3) "flush live output at 50 milliseconds, 64 lines, or 8 KiB; retain the newest 5,000 live lines at 5,001 while preserving the complete transcript".

These are load-bearing shared constants: UX-PB.3b and 3c stream live output, UX-PB.4b replays "retained output", Story 3.4 makes the threshold and hard cap **editable Settings** with "valid bounds" (`epics.md:1102`). Two stories will pick different numbers.

Related dangling reference — spine `:281-282`:

> "Any unmatched null-stdin silence uses the ordinary stall contract."

"the ordinary stall contract" is referenced as though defined; it is defined nowhere in the spine. Same pattern at `:270-272`: "while preserving the Last-good Snapshot rules" — the only occurrence of "Last-good" in the spine, with no statement of the rules and no pointer. Those rules do exist in the input (`epics.md:38` FR-3 "retain and label Last-good Snapshots"; `epics.md:511` UX-PB.1e "retain the last-good snapshot with its timestamp").

### C-4 [MEDIUM] "Exclude inherited environment values" was dropped entirely

`inherited` returns **zero hits** in the spine. The input states it four times:

- `epics.md:68` (FR-18) "include app/OS/architecture, constructed ToolEnv and detection evidence, Settings, and log filter; **exclude inherited environment values**; and reject symlink substitution during selection and streaming."
- `epics.md:92` (NFR-5) "exclude inherited environment values from logs and diagnostics, and resist diagnostic symlink substitution."
- `epics.md:355` and `epics.md:1139` (Epic 6 statement, twice) "export exact diagnostics without inherited-environment disclosure or hostile-path traversal."

The spine kept the symlink half of the same sentence — `:198-199` "**Rule:** Diagnostics must reject symlinks both when selecting and when streaming files." — and dropped the privacy half. Story 6.5 is the story that would have to enforce it and its own ACs (`epics.md:1155-1166`) never restate it, so the obligation now exists only in the FR/NFR inventory.

### C-5 [MEDIUM] The plan-attempt journal has no retention bound, but attempts must be immutable and lineage must resolve

Spine `:386-389` (AD-18) mandates compaction without stating a bound:

> "**Rule:** Confirmed attempts persist to their own append-only NDJSON journal in
> the same Application Support directory as `operations.jsonl`, under the same
> discipline: an append failure is nonfatal to package operations, and compaction
> is temp file + fsync + rename, never truncate-in-place."

The verified baseline gives `operations.jsonl` a bound (`:93-94` "compacted to the newest 1,000 records") and Story 6.5 exercises it (`epics.md:1159` "1,000 journal records"). The plan-attempt journal gets none. Meanwhile `epics.md:793-796` (UX-PB.4a) requires "exactly one immutable History row … for that `planAttemptId`", and `epics.md:862-864` (UX-PB.4d) requires:

> "**When** the source is missing, the link is dangling or orphaned, or the original would be mutated by the Retry
> **Then** the original attempt's History row and result remain immutable and are never overwritten, the lineage is surfaced honestly including when its source cannot be resolved, and no fabricated or repaired lineage is presented as valid."

Compaction is the mechanism that *creates* dangling `retryOfPlanAttemptId` links. Whoever picks the bound decides how often UX-PB.4d's failure path fires, and nothing constrains them.

### C-6 [MEDIUM] Story 2.2's phase **order** and per-Manager timeout boundaries are unowned

`epics.md:1029-1032`:

> "**Given** Homebrew metadata refresh is enabled or disabled
> **When** a Brew refresh is planned and rendered
> **Then** the enabled path shows the required update/inventory/outdated phase order
> **And** the disabled path omits only the metadata-update phase without mislabeling later phases."

`epics.md:1034` — "**Given** each of the six Manager adapters and its documented timeout boundary".

The spine routes this at `:483`: "| Detection, refresh phases, timeouts (Story 2.2) | Manager adapters behind runtime ports | AD-4 |". AD-4 covers port coverage, the safety floor, lock order and the scheduler; it contains no phase vocabulary, no ordering rule, and no timeout values. `phase order` returns zero hits. The nearest spine statement is `:426` "Per-manager phase is derived, never stored" — a state-storage rule, not an ordering rule. The named order `update/inventory/outdated` and the "omits **only** the metadata-update phase" constraint are exactly the quiet ordering clauses an AD structure loses.

### C-7 [MEDIUM] UX-PB.1e's Manager presentation contract is unowned

The spine routes UX-PB.1e at `:478`: "| Persistent draft Upgrade Plan and sidecar (UX-PB.1a–1e) | Rust plan services + Zustand projection + layout region | AD-16, AD-17 |". Nothing in AD-16 or AD-17 addresses any of these:

- `epics.md:502` — "Package counts in `34 managed packages · 8 package updates` **order**, and the self-update delta beneath the Manager status" (an explicit ordering constraint with a literal separator).
- `epics.md:503` — "**And** Manager self-state stays separate from managed-Package health, and update availability is **never colored as a system-health problem**." The spine's only colour rule is `:427` "Color states always carry a text or icon equivalent" — redundancy, not semantics.
- `epics.md:511` — "use text rather than an invented **Health Meter** value". `Health` returns zero hits in the spine.
- `epics.md:507` — "it shows `IN PLAN` plus a separate visible `Remove` action named `Remove <Manager> update from Upgrade Plan`, keeps no separate self-update row". `IN PLAN` returns zero hits. Story 3.1 independently depends on this exact symbol (`epics.md:1064` "surfaced as `IN PLAN` / `Remove` and never executes directly"), so it is a genuine two-story shared contract.
- `epics.md:511` — "state the exact failure summary with `Retry refresh`". `Retry refresh` returns zero hits.

### C-8 [MEDIUM] UX-PB.3e's failure-guidance contract is unowned, while three other stories demand "actionable" errors

The spine routes UX-PB.3a–3g at `:480` to "AD-4, AD-16". Neither says anything about error copy. UX-PB.3e is entirely error copy:

`epics.md:733` — "**Then** it presents `What happened` and `What to do next` with evidence and safe contextual actions before a secondary Retry, and it names the object that failed (e.g. `rustup refresh failed`) rather than a generic message."

`epics.md:737` — "**Then** it is not framed as likely fixed by repeated retries; a repeated identical failure says it repeated and emphasizes resolving the known cause before Retry, and an unknown non-zero exit shows evidence **without inventing advice**."

Three other live stories require actionable failure presentation without defining its shape: `epics.md:1036` (2.2) "the correct Manager-specific terminal state and actionable detail appear"; `epics.md:1165` (6.5) "the UI exposes actionable outcomes"; `epics.md:64` (FR-16) "provide actionable error feedback". The "without inventing advice" clause in particular is a never-do-X that no AD carries.

### C-9 [MEDIUM] Live-stream disconnect/reconnect is uncovered

`epics.md:691-693` (UX-PB.3c):

> "**Given** an attempt in progress (live-state stream disconnect/reconnect)
> **When** the per-item progress source drops mid-attempt and later reconnects
> **Then** each item keeps its last known honest state and is never silently shown complete, the interruption to the live stream is surfaced rather than guessed, and reconnection resumes correlated `planAttemptId` state without fabricating progress."

The spine's only event-ordering rule is AD-3 `:157-158`: "**Rule:** Startup subscribes to native events before `get_state` hydration, and a real detection report is never clobbered by the pre-detection placeholder." That covers *startup* subscription, not mid-attempt loss and resubscription. This is an architectural concern (event delivery guarantees across the single IPC boundary AD-3 owns) sitting in a UI story with no AD.

### C-10 [MEDIUM] The diagnostics destination and archive naming are absent from the Structural Seed

`Desktop` returns **zero hits** in the spine. `epics.md:68` (FR-18): "Export one timestamped diagnostics ZIP to the documented **Desktop** path". Story 6.5 asserts against it at `epics.md:1155-1157`: "**Given** documented default destination, alternate permission outcomes, and invocation from Settings and History … **Then** the timestamped ZIP path and visible success/failure match the contract."

The spine's Structural Seed (`:463-472`) maps Application Support and `~/Library/Logs/` and stops. AD-5 `:192-194` names "diagnostics destination" as one of the injected roots — so the *port* is fixed and the *production default* is not. Story 6.5's "match the contract" has no contract to match.

### C-11 [MEDIUM] UX-PB.2e's cancellation qualifiers did not land

`epics.md:611` — "no second confirmation is required, rollback is not promised"; restated at `epics.md:773` (3g) "it requires no second confirmation … promises no rollback, and never delays cancellation behind a dialog."

Spine `:265-268` carries the mechanics but neither qualifier:

> "**Rule:** Primary cancellation targets `planAttemptId`: unstarted work becomes
> `Skipped`, running process groups use the existing escalation, and every
> terminal state stays durable. `Cancel operation` is reserved for an explicitly
> Operation-scoped diagnostic action."

Two stories agree on "no second confirmation" and "no rollback promise"; the AD that governs them says neither. Given AD-17 makes `DialogHost` the single modal mount point, a builder could reasonably add a confirm dialog to `Cancel plan`.

### C-12 [LOW–MEDIUM] Smaller drops, each quoted

- **Outcome-wording rule.** `epics.md:795` (4a) "its summary uses verified-outcome wording such as `10 of 12 verified · 2 failed` **rather than a generic completion ratio**" — a never-do-X, agreed with 3d (`:707`), absent from the spine.
- **Read-only replay guarantee.** `epics.md:819` (4b) "**And** no control in the replay can mutate, re-run, or execute anything." The spine mentions replay only as an addressing concern: `:316-318` "The active-attempt lookup, cancel command, History query, Activity replay, and diagnostic export address `planAttemptId`."
- **Replay/live coexistence.** `epics.md:837` (4c) "full Activity is labeled `Viewing past activity`, `Back to live activity` is offered" and `:841` "the concurrent replay never suppresses, delays, or overwrites live status." Uncovered; interacts with A-1.
- **`Cancel` reservation.** `epics.md:777` (3g) "generic `Cancel` is reserved for closing a dialog or retry-scope editor without mutating running work." The spine carries the `Cancel plan` / `Cancel operation` split only.
- **Confirmation-off warning copy.** `epics.md:955` (5c) the exact string "`Confirmation is off. Changes will run immediately when you choose Run N Updates. Change in Settings.`" plus "links to Settings". The spine's `:276-279` covers the semantics ("skips only the final modal") and not the disclosure.
- **App-update presentation.** `epics.md:1001` (5e) "the update card heading is simply `Pack-Manager` and the installed-to-target version delta stays on one unbroken line with the installed version in warning yellow and the target version in success green" and the exact badge `Pack-Manager Update Ready!`. The spine covers only the separation (`:283-286`).
- **Event names.** `epics.md:591` (2d) names "the `op:status`/`op:output`/attention events"; the spine paraphrases at `:396-398` as "Operation status, output and **stall** events". "attention" vs "stall" is a naming drift across a typed boundary AD-3 governs.
- **VersionDelta.** `epics.md:70` (FR-19) "preserve VersionDelta as display-only" and `epics.md:689` (3c) "only that verified row collapses its `old → new` delta to the single new current version". No spine rule.
- **uv sub-entities.** `epics.md:1063` (3.1) "uv executables are reachable and searchable". No spine rule; FR-5 (`epics.md:42`) "retain useful Manager details".
- **UX-PB.1a's test-rewrite obligation.** `epics.md:411-413`: "**Given** the committed end-to-end suite asserts that a single-row upgrade executes immediately without a plan dialog **When** this story lands **Then** that assertion is rewritten to expect draft membership with nothing executing, because Decision D27 supersedes the behavior it encodes." The spine's baseline acknowledges the behaviour (`:88-89` "a single-package row action executes immediately") but not the required test rewrite. Worth a line in AD-1, which is the AD that talks about the product/test boundary.
- **Story 6.5's last AC appears truncated.** `epics.md:1165` ends "**Then** the UI exposes actionable outcomes" with no terminating period and no `And` clause, unlike every other AC in the file.

### C-13 [LOW] AD-16 owns `skipUpgradePlanConfirmation` but does not bind the story that validates it

Spine `:248` — "**Binds:** D27–D30; Epic UX-PB (all 28 stories); Stories 3.1, 3.2, 3.5, 6.5" — omits Story 3.4, yet AD-16 `:276-279` is the rule that creates the setting, and Story 3.4 (`epics.md:1105`) is the story that validates it: "`skipUpgradePlanConfirmation` is validated and persisted as a first-class control".

---

## D. Places the spine asserts something about the stories that the stories do not say

### D-1 [MEDIUM] "scroll identity" appears nowhere in the input

Spine `:372-373`: "Exactly one instance exists and it persists across `ActiveView` changes without losing membership or **scroll identity**."

`grep -n "scroll" epics.md` returns exactly one hit, `epics.md:980`, and it is about the opposite concern: "no two-dimensional scrolling for the primary task". The story that owns sidecar persistence, UX-PB.1b `epics.md:431`, says only:

> "**Then** the sidecar and its exact membership persist unchanged across navigation, and when hidden the main workspace reclaims its width with no reserved empty column."

"Membership" and the width-reclaim clause both trace. "Scroll identity" is spine-originated. Not harmful, but it is an unsourced requirement presented as derived.

### D-2 [MEDIUM] "carrying no inter-epic dependencies" is contradicted by 25 `Blocks:` lines

Spine `:499`:

> "| Epics 1–6 | **RESCOPED** | Six stories survive — 2.2, 3.1, 3.2, 3.4, 3.5, 6.5 — **carrying no inter-epic dependencies**. Epics 1, 4, and 5 were removed; 31 stories archived. `docs/DECISIONS.md` D33. |"

This faithfully echoes `epics.md:374` ("so the survivors carry no inter-epic dependencies") — but the story bodies in the same file contradict it. `grep -n "^\*\*Blocks:\*\*" epics.md` returns 25 lines; five name surviving stories as **blocked by UX-PB work**:

- `epics.md:388` (UX-PB.1a) — "**Blocks:** UX-PB.1b, UX-PB.1c; **Story 3.5** and its affected evidence"
- `epics.md:471` (UX-PB.1d) — "**Blocks:** **Story 3.2** and its affected evidence"
- `epics.md:494` (UX-PB.1e) — "**Blocks:** **Stories 3.1** and 5.2 and their affected evidence"
- `epics.md:699` (UX-PB.3d) — "**Blocks:** UX-PB.3e, UX-PB.3g; Stories 5.4, **6.5**"
- `epics.md:917` (UX-PB.5b) — "**Blocks:** UX-PB.5c; **Stories 3.4** and 6.7 and their affected evidence"

Five of the six survivors are declared blocked by UX-PB stories. A builder reading the spine's decision table will believe 3.1/3.2/3.4/3.5/6.5 can start immediately; the story bodies say four of five cannot. The spine should either restate the dependency or record the contradiction.

### D-3 [MEDIUM] The spine's drift note under-reports what is stale in `epics.md`

Spine `:505`:

> "| `epics.md` retired register | **Open** | `_bmad-output/planning-artifacts/epics.md` still carries TIR-1..TIR-8, RE-1..RE-11, the 72-criterion controls, and a set-equality requirement against `contracts/tauri-boundary/v1.json`. It contradicts this spine and `docs/DECISIONS.md` D33; reconciling it was out of scope for this run. See `DRIFT-NOTE.md`. |"

**Every element of that claim verifies.** TIR-1..TIR-8 at `epics.md:123-130`; RE-1..RE-11 at `epics.md:134-144`; the 72-criterion control at `epics.md:104` ("Preserve exactly 72 P0 criteria from `readiness-coverage-map.md`"); the catalog set-equality at `epics.md:150` ("Accept by Batch 4 exit through exact set equality across the versioned `contracts/tauri-boundary/v1.json` catalog").

It is **incomplete** in one way that matters for the build queue: ten archived stories are still named as blocked by live UX-PB stories — **3.3, 3.6, 4.1, 4.6, 5.2, 5.4, 5.5, 6.3, 6.4, 6.7** (from `epics.md:445, 494, 517, 536, 583, 623, 699, 765, 787, 810, 917`). Those are dangling exit criteria on stories that will be worked; they belong in the same note.

### D-4 [LOW] AD-17 binds two stories that contain none of its subject matter

Spine `:356` — "**Binds:** UX-PB.1a–1e, UX-PB.3a; Stories 3.1, 3.2, 3.5". UX-PB.1d (`epics.md:467-488`) is entirely about ineligible-control inertness and assistive-technology explanation; UX-PB.1e (`epics.md:490-511`) is entirely Manager Header/Card presentation. Neither contains a criterion about draft ownership, the Rust round-trip, sidecar visibility, or `DialogHost`. The over-broad binding is why C-7's presentation contract looks governed when it is not.

### D-5 [OK — noted as good practice] The AD-17 `[ASSUMPTION]` is correctly scoped

Spine `:366-369`:

> "**Rule:** The draft is durable. It is written to Application Support under the
> same atomic-replace discipline settings use, and reconstructed at launch. A
> missing, unreadable, or incoherent draft file yields an empty draft — never a
> partial or inferred membership. A draft is never surfaced as Activity or
> History. `[ASSUMPTION]` Durable persistence is the reading taken from
> UX-PB.1b, which also permits an always-empty-on-relaunch fallback."

`epics.md:437-439` does permit both readings — "the draft's canonical membership is reconstructed into the sidecar, **or** — if it cannot be recovered — the sidecar returns to empty with no fabricated membership". The spine narrows to the stricter reading and labels the narrowing. This is the right handling and the only place in the spine where a reading choice is marked.

---

## Appendix: per-story disposition

Legend: **A** governed by an AD · **B** pure implementation detail · **C** uncovered · **X** contradiction or cross-story incompatibility.

| Story | Disposition | Findings |
| --- | --- | --- |
| UX-PB.1a | A (draft/rebuild) + C (grid keyboard, test rewrite) | B-5, C-12 |
| UX-PB.1b | A (sidecar, durability) + C (counts trio, focus) | C-1, D-1 |
| UX-PB.1c | A (intent kind, convergence, no `includeSelfUpdates`) + C (filter scope) | B-8 |
| UX-PB.1d | X (`aria-disabled`) + C (four strings, `Managed through <Manager>`) | A-2, B-1, B-7 |
| UX-PB.1e | C (entire presentation contract) | C-7, D-4 |
| UX-PB.2a | A — fully governed by AD-16 / Consistency "Plan identity" | — |
| UX-PB.2b | A — atomic admission, one-active, fail-closed all in AD-16 | — |
| UX-PB.2c | C (version evidence, timestamps) + X (corrupt-record handling) | A-3, C-2 |
| UX-PB.2d | A (AD-18 correlation) + C-low (event names) | C-12 |
| UX-PB.2e | X (un-stoppable work) + C (no-second-confirm, no-rollback) | B-2, C-11 |
| UX-PB.2f | A — AD-16 "Legacy honesty" + AD-18 | — |
| UX-PB.3a | X (sidecar cannot hold draft + attempt) + C (status channel) | **A-1**, C-1 |
| UX-PB.3b | X (control placement) | B-6 |
| UX-PB.3c | C (item state vocabulary, disconnect/reconnect, focus) | C-9, C-1 |
| UX-PB.3d | X (closed taxonomy; Results-until-`Done`) + C (wording rule) | **A-1**, B-2, C-12 |
| UX-PB.3e | C (entire copy contract) | C-8 |
| UX-PB.3f | A (classifier) + C (120s threshold, exactly-three actions) | C-3, B-6 |
| UX-PB.3g | X (no `Verifying --> cancelled` edge) + C (`Cancel` reservation) | B-3, C-11, C-12 |
| UX-PB.4a | A (AD-18) + C (wording rule, persistence-failure honesty) | C-12 |
| UX-PB.4b | C (timings, read-only guarantee, retained-output bound) + X (corrupt replay) | A-3, C-2, C-3, C-12 |
| UX-PB.4c | C (replay/live coexistence) | C-12 |
| UX-PB.4d | A (retry lineage) + C (retention bound behind dangling links) | C-5 |
| UX-PB.4e | A — AD-16 + AD-18 | — |
| UX-PB.5a | A (gate semantics) + C (focus order, Escape conditions) | C-1 |
| UX-PB.5b | A — AD-19 covers atomicity, default, `autoOpenDrawer` tolerance | — |
| UX-PB.5c | A (safety semantics) + C (warning copy) | C-12 |
| UX-PB.5d | C (no governing AD at all) | **C-1** |
| UX-PB.5e | A (separation) + C (exact strings, colour tokens) | C-12 |
| 2.2 | C (phase order, timeouts) + X (settings set) | B-4, C-6 |
| 3.1 | A (row plan action, non-colour) + C (`IN PLAN`, uv search) | C-7, C-12 |
| 3.2 | X (native `disabled`) + C (greedy set-difference presentation) | B-1, B-7, B-8 |
| 3.4 | X (settings set) + C (Environment Report contents) | B-4, C-3 |
| 3.5 | X (selection layer vs AD-17 round-trip) + C (keyboard model) | B-5, B-7 |
| 6.5 | A (AD-18 export contents) + C (Desktop path, inherited env) | C-2, C-4, C-10 |

**Story 6.5 test-level note (not a defect):** `epics.md:1150` declares "- Required test level: Real native Tauri E2E plus artifact inspection", and the spine records the blocker honestly at `:500`: "| Native Tauri E2E harness and runner | **Deferred** | Story 6.5 is the only live consumer (\"Real native Tauri E2E plus artifact inspection\"). Any choice must satisfy AD-2 and AD-3. |" Story 6.5 cannot be accepted at its declared level until that is resolved. The spine handles this correctly; flagged for queue planning only.

---

## Recommended spine edits, ordered

1. **AD-17** — rewrite the sidecar rule to model three occupancy states (draft / live attempt / Results-until-`Done`), say where a new draft renders while an attempt is live, and replace "visibility is driven by draft non-emptiness" with a predicate that covers all three. Add the below-720-CSS-px full-workspace mode. *(A-1, C-1)*
2. **AD-19** — scope "degrades to defaults" to configuration files and add the opposing rule for evidence records: detected-and-refused, never defaulted, never partially reconstructed. *(A-3)*
3. **AD-16** — replace "non-interactive to pointer and keyboard" with the focus-retaining formulation, and state that native `disabled` is forbidden for explanatory-ineligible controls. *(A-2, B-1)*
4. **New AD** — accessibility and focus as *product* invariants: one status channel, focus ownership at each sidecar transition, dialog focus order with the destructive action never default, focus-restoration fallback, reduced motion, 4.5:1. Bind UX-PB.1b/1d/3a/3c/3d/5a/5d and Story 3.5. *(C-1)*
5. **AD-16 normative minimum** — add `timestamps` and version evidence to `PlanAttempt`; add the `Verifying --> Terminal: cancelled` edge and an honest-unknown per-item state; add the crash `Interrupted` edge. *(B-2, B-3, C-2)*
6. **New AD or Consistency row** — the shared constants: 120s stall threshold, 30-minute hard cap, 50 ms / 64 lines / 8 KiB flush, 5,000-line live retention, plan-attempt journal retention bound; and the closed list of configurable Settings. *(B-4, C-3, C-5)*
7. **AD-5** — restore "exclude inherited environment values from logs and diagnostics" alongside the symlink rule; add the diagnostics Desktop destination to the Structural Seed. *(C-4, C-10)*
8. **Resolve or state the dangling references** — "the ordinary stall contract" (`:282`) and "the Last-good Snapshot rules" (`:272`). *(C-3)*
9. **Decision table `:499` and `:505`** — correct or footnote "carrying no inter-epic dependencies", and add the ten archived stories still named in live `Blocks:` lines. *(D-2, D-3)*
10. **AD-17 `:372`** — drop "scroll identity" or source it. **AD-16 `:248`** — add Story 3.4. *(D-1, C-13)*
