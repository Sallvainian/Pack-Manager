# Epics 1-6 story triage — 2026-07-24 scope recalibration

Companion to `docs/DECISIONS.md` D33. Records the per-story reasoning behind the
37 Epic 1-6 entries as they stood before the rescope. The rescope was applied on
2026-07-25: the 19 merged and 12 retired stories moved to
`_bmad-output/archive/2026-07-24-scope-recalibration/planning/epics-1-6-triaged-out.md`,
Epics 1, 4, and 5 lost every story and were removed, and `sprint-status.yaml` now
carries only the 6 keeps (2.2, 3.1, 3.2, 3.4, 3.5, 6.5) alongside the 28 UX-PB stories.

**Method.** One pass per epic classified every story against a single test: *does this
make the app better for someone using it, or does it only produce paperwork about the
app?* Two adversarial passes followed — one hunting for real work being wrongly
discarded, one for ceremony being wrongly kept. The second overturned 14 of 20 initial
KEEP verdicts because the shipping Rust/TypeScript already implements the behavior and
the existing suite already tests it.

**Headline finding.** Most of what these stories describe is already built. Anyone
rescheduling them should verify against the source before assuming work remains.

**Totals:** 6 keep · 19 merge · 12 retire · 37 total

---

## Keep — real work with a verified gap

### Story 2.2 — Prove Refresh Phases and Per-Manager Timeouts

Each package manager needs its own time limit when refreshing. If one of them hangs, it gives up on its own, shows a clear message about what went wrong, and the other five keep going and finish normally instead of the whole refresh sitting there spinning. Also, when you turn off the Homebrew metadata refresh in settings, only that one step should disappear -- the remaining steps must still be named and reported correctly, not silently renamed or skipped. Dropped: the scenario-contract JSON and the rules about which test attempts "count".

<details><summary>Triage rationale</summary>

The title says "Prove", but the content is reliability behavior, not paperwork. `epics.md:1443-1444`: "**Then** the enabled path shows the required update/inventory/outdated phase order **And** the disabled path omits only the metadata-update phase without mislabeling later phases." And `epics.md:1448-1449`: "**Then** the correct Manager-specific terminal state and actionable detail appear **And** peers continue independently without real network access or wall-clock sleeps." A per-manager timeout is exactly the "more solid" work the owner asked for -- without it one hung manager stalls the whole refresh. `epics.md:1428` confirms the behavior itself is the deliverable: "missing or incorrect phase/timeout behavior creates Product Behavior work before test credit". Stripped: the third criterion's fail-closed admission rules at `epics.md:1451-1454` and the evidence artifact at `epics.md:1431`.

</details>

### Story 3.1 — Present Complete Package State and Manager Detail

Every package row shows its name, the version you have, the version available, and what state it's in — readable without depending on color alone, so it still makes sense in dark mode, at a glance, or with a screen reader. uv's list of executables can be opened up and searched. npm's own update shows up in the npm card at the top, not mixed in as if it were one of your regular packages, and pressing "Update Manager" there only adds it to the plan — it never starts an update on its own.

<details><summary>Triage rationale</summary>

The first two Acceptance Criteria are real screen behavior, not paperwork. `epics.md:1576` "name, installed/latest values, status text, eligibility, selection, and the row plan action that adds or removes the Package's stable identity in the one persistent editable draft Upgrade Plan without executing are complete and understandable without relying on color." and `epics.md:1580-1581` "uv executables are reachable and searchable **And** npm self state appears only in its Manager Card/Header — where `Update Manager` stages an independent, individually-removable self-update plan item surfaced as `IN PLAN` / `Remove` and never executes directly — while the four ordinary Package rows remain." A user notices immediately if a row can't be read without color or if uv's executables can't be found. STRIPPED: the whole Story Contract block (`epics.md:1555` "Primary readiness concern: Reusable Test Infrastructure", `epics.md:1562` "Versioned scenario contract", `epics.md:1564` "Expected evidence artifact: `b3-package-state-detail.json`") and the third AC `epics.md:1586` "a complete admitted first attempt makes all four criteria only **eligible for later FULL reassessment**". Note: the Manager Card / `IN PLAN` / `Remove` half already belongs to the promoted `UX-PB.1e`, which `epics.md:448` lists as "Blocks: Stories 3.1 and 5.2 and their affected evidence" — build it once there.

</details>

### Story 3.2 — Enforce Pinned and Greedy Eligibility

If you pinned a package on purpose, nothing in the app can quietly un-pin it — the row stays unclickable, says why in plain words, and never sneaks into a plan through "update all" or "Update Everything". Separately, Homebrew casks that can only be upgraded by force-overwriting them stay switched off by default, sit in their own collapsed group, and only join a plan if you deliberately turn them on and see a warning explaining what that means.

<details><summary>Triage rationale</summary>

This is safety behavior, not evidence. `epics.md:1619` "pinned rows stay inert, add nothing to the draft Upgrade Plan, and are explained, disabled, and excluded from every plan with the correct reason." and `epics.md:1623` "greedy-only casks are the documented set difference, remain separate/collapsed/default-excluded, and enter a plan only through explicit opt-in with visible disclosure." The user-story line states the point plainly at `epics.md:1592`: "So that no plan silently overrides a pin or includes default-excluded work." That is exactly "don't run something the user didn't ask for". STRIPPED: the test-matrix framing at `epics.md:1618` ("exercised across every active filter"), the contract/artifact fields at `epics.md:1605-1607`, and the third AC `epics.md:1627` "both criteria become only **eligible for later FULL reassessment**". PARTIAL OVERLAP: the pinned-row half is already owned by promoted story `UX-PB.1d` (`epics.md:433` gives the exact wording "This Package is pinned and cannot be updated..."; `epics.md:423` "Blocks: Story 3.2 and its affected evidence"). The greedy-cask half has no promoted owner and is the genuinely new work here.

</details>

### Story 3.4 — Validate Every Settings Control and Environment Report

Settings actually stick. Type a bad number and the app refuses it instead of accepting it and misbehaving later. If saving fails, nothing changes — not what's running, not what's on disk — and you see that it failed rather than being told "Saved". You get a visible Saving / Saved / failed indicator on each control. Changing the log level only takes effect once it's genuinely been written down. The "skip the Upgrade Plan confirmation" option is a real, properly saved setting rather than a loose flag. The old "auto-open the Activity drawer" preference is gone from the screen, and if an old copy of it is still saved on your machine it's harmlessly ignored instead of switching something on behind your back. The Environment Report shows all the information it's supposed to, and Copy tells you whether it actually worked.

<details><summary>Triage rationale</summary>

This is the clearest "more solid" story in the epic and has no promoted owner anywhere in the UX-PB queue. `epics.md:1701` "valid values persist before becoming active, invalid values are rejected, save failure changes neither active nor persisted state, and log-level changes apply live only after persistence". `epics.md:1702` "`skipUpgradePlanConfirmation` is validated and persisted as a first-class control, the Activity auto-open preference is removed from active Settings while any old persisted `autoOpenDrawer` value is tolerated during migration without ever becoming active, the new value applies only after atomic persistence succeeds, and every control saves immediately and atomically with visible `Saving`/`Saved`/failure state." "save failure changes neither active nor persisted state" is literally the don't-corrupt-saved-state case named as protected. `epics.md:1706-1707` "every required field and evidence value is present **And** copy success and failure are visible and actionable" is ordinary usable behavior. STRIPPED: `epics.md:1687-1689` (scenario contract + `b3-settings-environment-report.json` artifact) and the third AC at `epics.md:1711` "both criteria become only **eligible for later FULL reassessment**".

</details>

### Story 3.5 — Preserve Exact Keyboard Selection and Row Plan Actions

Selecting packages with the keyboard and mouse works the way a Mac list is expected to: click one, shift-click to grab a range, Cmd+A to select everything currently shown, Space to toggle, Esc or Clear to drop the selection, and a header checkbox that correctly shows "some selected" versus "all selected". Crucially, none of these shortcuts can pick up a row you're not allowed to update — pinned, already-current, or excluded packages stay out of the selection even when you hit Select All — and selecting only applies to the rows your current filter is actually showing.

<details><summary>Triage rationale</summary>

Half of this is already promoted work, but the multi-row selection half is not owned anywhere. AC1 at `epics.md:1743-1745`: "toggle, shift-range, tri-state, Cmd+A, Space, Cmd-click, Clear, and Esc interactions execute **Then** the exact selectable identities and visible filter semantics are preserved **And** excluded rows never enter selection." "excluded rows never enter selection" is the safety case — Cmd+A must not sweep a pinned package into a plan — and the range/select-all gestures are ordinary table usability a solo developer would want. STRIPPED / ALREADY OWNED: AC2 at `epics.md:1749-1750` ("exactly one eligible Package's canonical identity is added to (or removed from) the persistent draft Upgrade Plan, nothing is built, submitted, enqueued, or executed" and "ineligible, pinned, or current rows add nothing, stay inert with an explained reason") duplicates promoted `UX-PB.1a` (`epics.md:348`) and `UX-PB.1d` (`epics.md:433-434`), and epics.md says so at `epics.md:338`: "Blocks: UX-PB.1b, UX-PB.1c; Story 3.5 and its affected evidence". Also stripped: the evidence artifact at `epics.md:1732`, the `epics.md:1737` note "browser evidence is not packaged evidence", and AC3 at `epics.md:1755`.

</details>

### Story 6.5 — Export Exact Native Diagnostics and Visible Outcomes

The 'Export diagnostics' button produces a dated zip that actually contains what it should: the most recent app logs, the most recent command transcripts, the operations history, and the upgrade-plan records that tie them together. When it works it shows you where the file went; when it can't write there (no permission, bad folder) it shows a real error instead of failing quietly. 'Open Logs' behaves the same way — it opens the folder or tells you why it couldn't.

<details><summary>Triage rationale</summary>

Despite the framing at epics.md:2633 "Primary readiness concern: Reusable Test Infrastructure", this is a shipped user feature (epics.md:2632 "FR and requirement links: FR-18") with a button in the UI. epics.md:2652-2654 "Given documented default destination, alternate permission outcomes, and invocation from Settings and History When diagnostics export runs through the production native command Then the timestamped ZIP path and visible success/failure match the contract"; epics.md:2656-2658 "Then it contains report.json, the newest three app logs, newest 25 transcripts, operations.jsonl, and the durable plan-attempt records"; epics.md:2660-2662 "Given Export diagnostics and Open Logs actions ... Then the UI exposes actionable outcomes". A zip that is missing the logs, or a button that silently fails, is a broken feature the owner would notice the moment he needed it. STRIPPED: epics.md:2642 the "archive inventory/digests" evidence artifact, epics.md:2640 the scenario contract, epics.md:2648 "Preserve first ZIP and failure artifacts; runnerRetryCount = 0", and epics.md:2663 "all three criteria become only eligible for later FULL reassessment". Also note epics.md:2743-2745 in Story 6.7 repeats the export/Open Logs outcome behavior — build it once, here.

</details>

---

## Merge — real behavior, but owned by another story

| Story | Title | Merge into | What must survive |
|---|---|---|---|
| 1.3 | Verify the Live Six-Manager Target-Mac Topology | 2.1 | Nothing separate to build — 2.1 already covers it. The one thing to carry over into 2.1: make sure `mas` is treated as a first-class manager alongside the other five, so the Mac App Store apps show up… |
| 1.4 | Capture the Real `mas` Correctness Oracle | 1.5 | Keep the real `mas` output that was saved from the Mac App Store as the sample the tests run against, instead of made-up examples. This is a step inside 1.5, not its own piece of work.… |
| 2.1 | Preserve Honest Absence and Complete Environment Evidence | MERGE | If a package manager isn't installed on the Mac, the app should not try to run it and should not show it as a failure. It should just look greyed out, say "Not installed", and tell you how to install … |
| 2.4 | Revalidate Stable Detection and Refresh State Truth | 2.3 | Two small checks folded into Story 2.3, not a story of its own. First: a tool installed through mise should be labeled as belonging to mise, judged by the path the app actually saw rather than the fil… |
| 3.3 | Build Plans from Every User Entry Point | UX-PB.1c | Carry one requirement into UX-PB.1c: if building the plan fails, the app says so on screen in a way you can act on, and it never leaves an out-of-date plan sitting there looking ready to confirm. Ever… |
| 3.6 | Revalidate Version Truth, mise Consequences, and Plan Defaults | 3.1 | This is a pure re-proving story with no new product behavior. Its user-story line is written for the auditor, not the user: `epics.md:1759-1761` "As a QA Lead, I want historically FULL display and pla… |
| 4.4 | Cross All-Six Native Refresh Ordering | 2.2 | When you hit Refresh All, each package manager reads its installed-package list before it checks what's outdated, so the outdated counts are never computed against a stale inventory. Managers that don… |
| 4.5 | Revalidate Native Refresh and Contract Equality | 2.2 | This story exists to re-run work that already passed. epics.md:1983-1984: "I want historical refresh and IPC-contract evidence rerun across the current real boundary, So that previous FULL status does… |
| 4.6 | Revalidate Reviewed Plans and Native Admission | UX-PB.2b | Nothing runs until you say yes on the final "Proceed with Upgrade Plan?" step, and what runs is exactly the list you confirmed -- the same packages and the same manager self-updates, with nothing quie… |
| 5.2 | Prove Dynamic Self-Update Routes and Manager Header/Card Plan State | MERGE | The app figures out, at the moment you look at it, how each manager updates itself — npm updates itself in place, mise and uv get updated through mise, some can only be updated by hand — instead of re… |
| 5.3 | Reject Unsafe Spawns and Hold Complete Locks | MERGE | The only command that ever runs is the exact command that was shown to you in the plan you confirmed — not a rebuilt or re-guessed version of it. If the plan is out of date, was edited, is being repla… |
| 5.4 | Preserve Native Output and Activity Boundaries | MERGE | Live output appears exactly as the tool printed it, in the right order, with regular output and error output not getting shuffled together wrongly and nothing silently dropped. A long update that prin… |
| 5.5 | Cancel, Stall, Time Out, and Shut Down Honestly | MERGE | Hitting Cancel stops everything right away — no second 'are you sure?' dialog. Anything still running is actually killed, including the helper processes it started, so nothing keeps chugging away invi… |
| 5.6 | Repair Only the Allowlisted Unterminated `mas` Notice | 5.4 | The one known malformed notice from the App Store tool (mas) gets a single line break added after it so the following output isn't glued onto the end of it and the log stays readable. If that notice a… |
| 5.8 | Revalidate Preview Bytes Against Spawned Bytes | 5.3 | Whatever command text you were shown before you hit confirm is character-for-character the command that actually runs. Nothing is rebuilt, re-resolved, or substituted in between. If the plan got stale… |
| 5.9 | Revalidate Scheduler Locks, Parallelism, and Capacity | 5.3 | Updates that would step on each other run one at a time; updates that have nothing to do with each other run side by side. No more than four run at once, so the machine doesn't get swamped. A job that… |
| 6.3 | Preserve Real Transcripts and Atomic Journals | MERGE | Everything an upgrade prints is written to a file as it runs, so after a crash or force-quit you can still read exactly what happened. If the app can't create that file in the first place, it refuses … |
| 6.4 | Reconstruct Interrupted Work Without Signaling History | MERGE | If the app is killed while an upgrade is running, reopening it shows that upgrade as 'Interrupted' with its output still attached — not stuck pretending to run, and not vanished. The app never sends a… |
| 6.7 | Preserve Settings and Native Utility Actions Across Failure | MERGE | Settings survive bad days: if the settings file is missing or damaged the app starts on sensible defaults instead of breaking, saves are written all-or-nothing so a crash mid-save can't garble the fil… |

---

## Retire — evidence ceremony, no product content

| Story | Title | Why |
|---|---|---|
| 1.1 | Restore Current `mas` and Release Truth | The AC is entirely document reconciliation plus a guard that the documents stay reconciled. `epics.md:1174-1177` "When authoritative and user-visible product sources are reconciled / Then they no longer describe `mas` as… |
| 1.2 | Qualify the Initial Split Evidence Lanes | This is the gate's plumbing and nothing else. The story itself says so: `epics.md:1198` "Criteria and historical baseline: None; ASR-05 enabler work does not add denominator rows" and `epics.md:1199` "No direct FR implem… |
| 1.5 | Enforce `mas` Provenance and Fixture Honesty | This is the one story in Epic 1 with behavior a user would feel. `epics.md:1351-1353` "Then the real capture proves ID, name, version, and padding behavior without stray whitespace / And synthetic inputs prove only crash… |
| 2.3 | Keep Offline Failures Isolated | This is the safety/reliability core of the epic and matches the epic's own stated outcome at `epics.md:1367`: "while peer Managers and Last-good Snapshots remain usable." `epics.md:1487-1489`: "**Then** each failure is l… |
| 4.1 | Establish the Versioned Production Boundary Contract | The acceptance criteria are catalog bookkeeping, not app behavior. epics.md:1836 "command and event entries use the architecture-defined closed fields, stable ordering, unique names/vectors, schema digests, and scenario-… |
| 4.2 | Deliver the Deterministic Process-Control Core | Two of the three criteria are test scaffolding, but the third is real command-execution safety. epics.md:1893-1894: "structured absolute argv, sanitized environment, null stdin, no shell/sudo/password path, and lock-set … |
| 4.3 | Cross Native Startup, Detection, and Re-detect | Underneath the "prove it through real Tauri" framing this is three concrete behaviors a user feels. epics.md:1927: "subscription precedes hydration, the real bridge/registration/serialization/handler/event path is crosse… |
| 5.1 | Refresh Every Routed Subject and Executor | epics.md:2098-2104 states real behavior, not evidence: "Given a successful in-band or same-Manager update / When terminal success is processed / Then the affected Manager refreshes exactly as specified without duplicate … |
| 5.7 | Prove the D26 Rule Cannot Become Heuristic | This story produces no product behavior — it produces a test corpus proving the 5.6 rule stays narrow. epics.md:2358-2360: "Given normally terminated, repeated, near-match, unrelated, and generic mid-line-marker inputs /… |
| 6.1 | Deliver ASR-02 Filesystem and Native-Utility Extensions | Pure test-harness plumbing for the retired gate. epics.md:2459 "Criteria and historical baseline: None; the ASR-02 Batch 6 extension adds no denominator row"; epics.md:2460 "No direct FR implementation; ASR-02/TIR-4 exte… |
| 6.2 | Deliver the Disposable Lifecycle Environment | Same category as 6.1 — infrastructure to run the retired gate's batches. epics.md:2503 "Criteria and historical baseline: None; ASR-03 enabler work adds no denominator row"; epics.md:2504 "No direct FR implementation; AS… |
| 6.6 | Reject Hostile or Private Diagnostic Inputs | The title reads like ceremony; the content is privacy behavior, and the epic itself already flags it as real product work: epics.md:2681 "Behavior-present handling: BP; missing/incorrect privacy behavior creates Product … |

---

## Verdicts overturned by adversarial review

| Story | First pass | Final | Reason |
|---|---|---|---|
| 1.5 | KEEP | RETIRE | Every clause of the surviving AC is already shipped and asserted. AC1 is `epics.md:1351-1353` "Then the real capture proves ID, name, version, and padding behavior without stray whitespace / And synth… |
| 2.1 | KEEP | MERGE | Merge the one live residual into Story 3.4; the rest is shipped. AC1 at `epics.md:1400` is "Then the Manager is not invoked, displays muted Not installed treatment, shows its known install hint, and e… |
| 2.3 | KEEP | RETIRE | The protected "one manager failing must not blank the others" rule is real — and it already works, end to end, with a test. AC1 at `epics.md:1487-1489` is "Then each failure is localized to that Manag… |
| 3.6 | RETIRE | MERGE into Story 3.1 | The rationale asserts "verbatim version display at epics.md:1788 is Story 3.1 (kept)" — that is not true of 3.1's actual text. 3.6 AC1 reads, `epics.md:1788`: "**Then** installed/latest strings remain… |
| 4.2 | KEEP | RETIRE | The triage kept 4.2 solely for AC3, `epics.md:1893-1894` "Then structured absolute argv, sanitized environment, null stdin, no shell/sudo/password path, and lock-set safety remain fail-closed". Every … |
| 4.3 | KEEP | RETIRE | This is a test-level upgrade, and the triage retired 4.4 and 4.5 on exactly that reasoning without applying it here. The story's own "So that" line is about test placement, not behavior: `epics.md:189… |
| 4.5 | RETIRE | MERGE into Story 2.2 | The rationale retires 4.5 on the claim that its "single real nugget ... is already covered by the refresh-ordering behavior folded into Story 2.2". That coverage does not exist. 4.5 AC1 is `epics.md:2… |
| 5.1 | KEEP | RETIRE | Both ACs are literally implemented in one block of the scheduler. AC2 at `epics.md:2102-2104` is "Given a successful routed self-update with different subject and executor / When terminal success is p… |
| 5.2 | KEEP | MERGE | Merge into UX-PB.1e; epics.md says so itself at `epics.md:448` — UX-PB.1e "**Blocks:** Stories 3.1 and 5.2 and their affected evidence". AC1 at `epics.md:2141-2142` ("Then the exact dynamic Route and … |
| 5.3 | KEEP | MERGE | The safety guarantee is real and must survive the D27-D30 rewrite — but it is already implemented today, and keeping it true through the rewrite is UX-PB.2a/UX-PB.5c's job, not a separate story. The s… |
| 5.4 | KEEP | MERGE | Split cleanly: AC1 is shipped, AC2/AC3 are verbatim UX-PB.3a/3b/3d — which `epics.md:673` already declares, listing UX-PB.3d as "**Blocks:** UX-PB.3e, UX-PB.3g; Stories 5.4, 6.5, 7.6 and their affecte… |
| 5.5 | KEEP | MERGE | Every honest-outcome behavior here is either shipped or is verbatim a promoted UX-PB story; `epics.md:745` already says UX-PB.3g "**Blocks:** Story 5.5 and its affected evidence". Shipped half of AC1 … |
| 6.3 | KEEP | MERGE | Another UNIT-ONLY→native-E2E re-prove (`epics.md:2545` "`F8-AC1` — `UNIT-ONLY`; `F8-AC2` — `UNIT-ONLY`", `epics.md:2549` "Required test level: Real native Tauri E2E"), and the crash-safety behavior is… |
| 6.4 | KEEP | MERGE | The pid-reuse safety rule the triage called "the single most important safety behavior in the epic" is already implemented, documented, and tested — so keeping the story protects nothing. AC1 at `epic… |
| 6.6 | KEEP | RETIRE | The privacy property is real and is structurally guaranteed today; there is nothing to build. AC1 at `epics.md:2696` is "Then only explicitly constructed values are eligible and inherited values never… |
| 6.7 | KEEP | MERGE | 6.7 is a three-way duplicate: AC1 is shipped, AC2 is Story 3.4 and UX-PB.5b word for word, AC3 is Stories 6.5 and 4.3. AC1 at `epics.md:2737` — "Then defaults and valid values behave as specified, wri… |
