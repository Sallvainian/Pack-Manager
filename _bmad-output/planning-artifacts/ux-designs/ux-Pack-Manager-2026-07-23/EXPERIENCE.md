---
name: Pack-Manager Experience Contract
description: Interaction, information architecture, state, accessibility, and journey contract for Pack-Manager.
status: final
project: Pack-Manager
created: 2026-07-23
updated: 2026-07-24
sources:
  - _bmad-output/planning-artifacts/prds/prd-Pack-Manager-2026-07-22/prd.md
  - _bmad-output/planning-artifacts/prds/prd-Pack-Manager-2026-07-22/addendum.md
  - _bmad-output/planning-artifacts/architecture/architecture-Pack-Manager-2026-07-23/ARCHITECTURE-SPINE.md
  - _bmad-output/planning-artifacts/epics.md
  - _bmad-output/planning-artifacts/implementation-readiness-report-2026-07-23.md
  - _bmad-output/planning-artifacts/sprint-change-proposal-2026-07-24.md
---

# Foundation

Pack-Manager exists to replace the mental work of coordinating several command-line Package Managers with one understandable visual workflow. It should answer three questions immediately:

1. What is installed and is the information trustworthy?
2. What can be updated?
3. What will happen if I approve this plan?

The central experience object is the **Upgrade Plan**. Every Package update, Manager self-update, Manager-wide action, and `Update Everything` action is staged in the same reviewable plan. Nothing executes from an individual row or Manager header.

The product is a native, dark-only macOS desktop app built with Tauri, React, TypeScript, and Rust. It supports a minimum 900 × 600 window, large Package inventories, long-running operations, retained output, keyboard use, VoiceOver, and reduced motion. Requirements for exact commands, process ownership, execution, persistence, and safety remain authoritative in the source PRD, addendum, Architecture Spine, and epics. This document defines how users experience those requirements.

## Experience principles

- **Overview before operation.** The opening Dashboard summarizes reliability, update scope, and Manager health before presenting actions.
- **Plan before execution.** All update intent becomes a visible Upgrade Plan before a command can run.
- **Human meaning before terminal evidence.** Package names, version changes, status, and recovery come first; commands and output remain available as evidence.
- **Trust is earned through verification.** A successful process exit is not enough; affected state is refreshed and verified before the UI declares success.
- **One object across each attempt.** Draft Upgrade Plan → live Upgrade Activity → persistent Results Summary → exactly one History entry. A confirmed Retry is a new linked attempt.
- **Failure should teach.** Explain the known cause and safe next step before offering Retry.
- **Native restraint.** Keep controls compact, predictable, keyboard-accessible, and smooth without theatrical motion.

# Information Architecture

## Primary navigation

| Destination   | Purpose                                                                          | Important behavior                                                                                                         |
| ------------- | -------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------- |
| **Dashboard** | System overview, Update Everything entry point, and Manager cards                | No permanent Manager list; main content uses full width when the Upgrade Sidecar is absent                                 |
| **Managers**  | Disclosure control for detected Managers                                         | Collapsed by default; expanding reveals Managers, and selecting one opens its dedicated workspace                          |
| **Activity**  | Inspector for the currently executing confirmed plan                             | When idle, says there is no active upgrade; during replay, the live sidecar remains available with `Back to live activity` |
| **History**   | One record per confirmed execution attempt                                       | Opening an entry routes Activity into read-only replay mode; Retry entries link to their source attempt                    |
| **Settings**  | Upgrade safety, refresh, advanced operation limits, app updates, and diagnostics | One scrollable page; advanced sections are collapsed by default                                                            |

The Managers item is not a separate all-Managers page. Dashboard Manager cards provide the overview; the disclosure list is the direct switcher for dedicated Manager workspaces.

## Surface map

| Surface              | User question answered                                                | Relationship to the Upgrade Plan                                                                                          |
| -------------------- | --------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------- |
| Dashboard            | “What is the overall state?”                                          | Adds Update Everything or a Manager self-update; sidecar opens only after something is added                              |
| Manager workspace    | “What does this Manager own and what can change?”                     | Package Checkboxes add items to or remove them from the plan; `Update Manager` adds the Manager self-update               |
| Upgrade Plan sidecar | “What am I proposing to update?”                                      | Persists across Dashboard and Manager navigation, remains editable, and lets the user remove each item; hidden when empty |
| Confirmation Dialog  | “Am I sure, and what exact command will run?”                         | Final authorization; never visible before the blue confirm action is pressed                                              |
| Upgrade Activity     | “What is running now?”                                                | The same sidecar shows the live summary; full Activity shows deeper evidence from the same state model                    |
| Results Summary      | “What actually happened?”                                             | The same sidecar persists at terminal state until `Done`                                                                  |
| History              | “What happened in a prior attempt?”                                   | One entry reconstructs one confirmed attempt and links Retry relationships                                                |
| Activity replay      | “Which commands and items succeeded or failed in that prior attempt?” | Read-only reconstruction opened from History; a concurrent live sidecar remains clearly primary                           |
| Settings             | “How should Pack-Manager behave?”                                     | Stores confirmation, refresh, limits, app-update, and diagnostics preferences                                             |

## Manager workspace structure

1. Manager Header with name, muted installed version, short role, executable path, Manager status, optional self-update delta, and Update Manager action. Once staged, it shows `IN PLAN` plus a visible `Remove` action.
2. Package Filter for All Packages, Updates, and Pinned, each with a count.
3. Package list ordered by update availability first, then pinned/excluded, then current.
4. Persistent editable Upgrade Sidecar when the draft plan is non-empty. Every staged Package and Manager self-update can be removed there.

All Packages is the default. Remember the selected filter separately for each Manager during the current session.

Visual reference: [Manager workspace and persistent Upgrade Plan](./mockups/manager-workspace.html).

## Lifecycle model

```text
No draft
  → user adds eligible Package or Manager
Draft Upgrade Plan
  → user reviews and confirms
Confirmation Dialog
  → final authorization
Live Upgrade Activity
  → every operation reaches a terminal state and affected state is refreshed
Results Summary
  → attempt is persisted
History attempt
  → user opens it
Read-only Activity replay
```

Removing the last draft item closes the Upgrade Sidecar. A draft never appears in Activity or History. Every confirmed execution attempt becomes exactly one immutable History entry, even when it contains multiple Managers, commands, Packages, failures, skips, or cancellations. Choosing Retry creates a new draft; if confirmed, that attempt receives a new History entry labeled `Retry of plan from <time>` and never rewrites the original result.

## Runtime clarifications

- Only one confirmed Upgrade Plan attempt may be active. A user may continue
  reviewing a draft, but it cannot be confirmed until the active attempt is
  terminal. Cross-Manager concurrency occurs inside the one active attempt.
- `Interaction required` is not inferred from arbitrary text or silence. It is
  shown only when a closed Manager-specific classifier or explicit native
  signal recognizes a trusted prompt. All unmatched null-input silence follows
  the ordinary stall path.
- Use `Cancel plan` when the consequence stops or skips the remaining work in
  the confirmed attempt. Use `Cancel operation` only for a deliberately
  Operation-scoped diagnostic action. Generic `Cancel` is reserved for closing
  a dialog or retry-scope editor without mutating running work.

# Voice and Tone

Use plain, calm, specific language. Prefer what happened, what is known, and what the user can safely do next.

- Use sentence case for headings, labels, and controls.
- Use lowercase supporting context in System Summary Cards: `across 3 managers`.
- Use compact uppercase only for Status Chips: `UPDATE AVAILABLE`, `NO UPDATES`, `REFRESH FAILED`, `PINNED`.
- Name the object that failed: `rustup refresh failed`, not `Something went wrong`.
- Distinguish saved from live information: `Showing snapshot from 10:42 AM`.
- Do not blame the user or imply certainty the system does not have.
- Do not call Update availability a System health problem. System health reflects reliability and operating state.
- Describe delegated ownership as `Managed through <Manager>`; reserve internal Route/owner terminology for technical evidence.

| Prefer                                                                                                     | Avoid                                                           |
| ---------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------- |
| `Warning · 5 managers current · 1 refresh failed`                                                          | `Attention` or `Ready` while a refresh has failed               |
| `This Package is pinned and cannot be updated. Unpin it, then refresh Pack-Manager to make it selectable.` | `Disabled` with no explanation                                  |
| `Homebrew is busy with another process. Wait for it to finish, then retry.`                                | `Retry failed`                                                  |
| `12 of 12 updates verified`                                                                                | `Command completed` as proof of success                         |
| `Show command output`                                                                                      | A terminal filling the primary surface by default               |
| `Restart to Update`                                                                                        | Silent restart or automatic interruption of active Package work |
| `Confirmation is off · Change in Settings`                                                                 | An immediate execution button that still says `Confirm`         |
| `10 of 12 verified · 2 failed`                                                                             | `10 of 12 updated` when verification did not succeed            |

# Component Patterns

The component names below match `DESIGN.md`. Implementations may compose them, but must preserve the stated behavior.

| Component                     | Behavioral contract                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| ----------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| **Button**                    | Enter/Space activates. Primary actions state the consequence. Busy state prevents duplicate activation without removing the label. Danger styling is reserved for destructive or cancel consequences, not ordinary navigation.                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| **Checkbox**                  | On eligible Package rows, selection immediately adds/removes Upgrade Plan membership. The header Checkbox applies to every eligible Package identity matching the active filter, including off-screen virtualized rows; it announces the exact count and uses `mixed` when only some are staged. An explanatory-disabled control never uses native `disabled`: expose `aria-disabled="true"`, attach its persistent reason as an accessible description, keep activation inert, and preserve focus for its supplemental Tooltip/Popover.                                                                                                                                                   |
| **Sidebar Navigation**        | Arrow/Tab navigation follows native expectations. Managers expands/collapses without changing workspace; choosing a child Manager changes workspace while preserving any draft sidecar.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| **System Summary Card**       | Reads in the order name, compact value, context. System health becomes `Warning` when any Manager refresh has failed even if a last-good snapshot keeps the app usable.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| **Manager Card**              | Opens the Manager workspace. The Status Chip reports Manager self-state only; Package update counts and Health Meter report managed-Package state separately. A refresh failure retains the last-good snapshot, timestamp, exact failure summary, and Retry refresh.                                                                                                                                                                                                                                                                                                                                                                                                                       |
| **Manager Header**            | Update Manager adds the self-update to the Upgrade Plan and never executes it. Once staged, show `IN PLAN` and a separate visible `Remove` action with accessible name `Remove <Manager> update from Upgrade Plan`. Its version delta remains beneath `UPDATE AVAILABLE`; Package health stays in the Package area.                                                                                                                                                                                                                                                                                                                                                                        |
| **Package Filter**            | Filters All Packages, Updates, or Pinned without changing plan membership. Counts always reflect the full current Manager dataset, not just rendered rows. Changing the filter preserves stable Package identity and moves focus to the first surviving row or the grid heading.                                                                                                                                                                                                                                                                                                                                                                                                           |
| **Package Grid**              | Use an ARIA grid pattern with persistent column headers, `aria-rowcount`, stable Package identity, `aria-rowindex`, and one roving row Tab stop. Up/Down moves one row; Page Up/Down moves one viewport; Home/End reaches first/last; Space toggles the active eligible Package; Shift+Up/Down extends a contiguous membership range from the anchor; the header Checkbox adds/removes all eligible identities matching the active filter. Virtualization scrolls the focused row into view and never removes the active identity without moving focus deliberately. Tab leaves the row set for the visible Upgrade Sidecar; F6 cycles primary navigation, main grid, and sidecar regions. |
| **Package Row**               | Shows Package name, installed version, target version where relevant, and explicit state. Current, pinned, excluded, and unavailable rows cannot enter the plan and each exposes a plain-language reason. Use `Managed through mise` for delegated ownership. Package version comparisons are display evidence, never the source of truth for outdatedness.                                                                                                                                                                                                                                                                                                                                |
| **Status Chip**               | Combines text with semantic color. Status text remains available to assistive technology and never relies on hue alone.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| **Health Meter**              | Fill length equals current Packages ÷ managed Packages. The fill is one solid color selected from the health scale. The accessible name states exact counts. Unknown or failed refresh uses text instead of invented health.                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| **Upgrade Sidecar**           | Hidden when empty. Draft view groups changes by Manager, summarizes Updates/Managers/Commands, and gives every staged Package and Manager self-update a visible Remove control. It persists across Manager changes and transforms into Activity and Results. When confirmation is off, commands are expanded, a persistent `Confirmation is off` warning links to Settings, and the immediate action is `Run # updates`.                                                                                                                                                                                                                                                                   |
| **Confirmation Dialog**       | Opens only from `Confirm # updates`, dims the background, traps focus, shows exact commands, and offers `Change Plan` plus final confirmation. The skip-future checkbox exists only here and is accessibly described by its safety explanation. Escape/backdrop dismiss only when no command has begun. `Change Plan` returns to the first editable plan item or plan heading.                                                                                                                                                                                                                                                                                                             |
| **Activity Operation Row**    | Shows queued, waiting, running, stalled, interaction required, cancelling, verified, failed, cancelled, skipped, or timed out in human terms. Use indeterminate motion unless a trustworthy percentage exists. Stalled rows expose exactly `Keep waiting`, `Copy command`, and `Cancel`; an unexpected prompt never accepts input and uses the same blocked handoff. Only verified rows collapse a version delta to the single new version.                                                                                                                                                                                                                                                |
| **Results Summary**           | Remains visible until `Done`. Groups outcomes by Manager, declares success only after refresh verification, and expands failures into `What happened`, `What to do next`, evidence, safe actions, and a secondary Retry. Retry first shows the failed-item scope inline; `Create new plan` deliberately replaces Results with a draft while the immutable result stays in History with `View previous result`.                                                                                                                                                                                                                                                                             |
| **History Plan Row**          | Represents one confirmed execution attempt, not one Package or command. Summaries use verified wording such as `10 of 12 verified · 2 failed`. A Retry attempt links to its source. Opening a row launches read-only Activity replay with correlated commands, outcomes, errors, timestamps, and retained output.                                                                                                                                                                                                                                                                                                                                                                          |
| **Tooltip/Popover**           | Opens on hover, click, or keyboard focus; remains long enough to read; is dismissible; and does not contain the only path to a required action.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| **Settings Section**          | Uses immediate per-control persistence: after activation, show `Saving`; change the active value only after the write succeeds, then show `Saved`. On failure, retain/revert to the prior value and expose an inline error. Upgrade safety contains the same confirmation preference used by the dialog.                                                                                                                                                                                                                                                                                                                                                                                   |
| **Application Update Status** | Detailed state lives in Settings → Pack-Manager updates. One restrained application-level badge labeled `Pack-Manager Update Ready!` announces availability and links there. The update card heading is simply `Pack-Manager`; the installed-to-target version delta stays on one line, with the installed version in warning yellow and target version in success green. Background check/download may occur, but installation requires explicit Restart to Update, is blocked while Package work is active, and falls back to manual-install guidance without privilege escalation.                                                                                                      |
| **Command Output Disclosure** | Exact commands are available before execution and command/output evidence remains available afterward. Expanded content is selectable and copyable; long retained output offers a secondary log reveal.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| **State Panel**               | States the condition, its impact, and the safest next action. Idle Activity explicitly says no upgrade is active instead of displaying drafts or completed plans.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| **Brief Notification**        | May visually reinforce a verified success, warning, or failure. It never replaces persistent status and must not repeat an announcement already emitted by the single Activity/Results status channel.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |

# State Patterns

## Loading and refresh

- Render the application shell immediately on launch.
- Detect Managers progressively; one slow or absent Manager does not block the others.
- Show Manager-level loading states independently.
- A normally absent Manager is not an error. Explain how to re-detect after installation.
- On refresh failure, retain the last-good snapshot, show its timestamp, set System health to `Warning`, and provide Retry refresh plus the specific known failure.
- Never replace trusted saved data with an empty table because a live refresh failed.

## Draft Upgrade Plan

- Empty: no sidecar.
- First eligible addition: sidecar opens with focus preserved on the source control.
- Membership changes: the `Updates`, `Managers`, and `Commands` counts, Manager groups, and button label update immediately.
- Every staged Package row has `Remove`; every staged Manager self-update has `Remove` in the Manager group heading. Removing the final item closes the sidecar.
- The bulk Checkbox scope is every eligible Package matching the active filter, including virtualized off-screen rows; its accessible label states the exact add/remove count and its mixed state reflects partial plan membership.
- Pinned: `This Package is pinned and cannot be updated. Unpin it, then refresh Pack-Manager to make it selectable.`
- Excluded: `This Package is excluded by your Settings. Change the setting, then refresh Pack-Manager.`
- Current: `This Package is already current.`
- Unavailable/unknown target: `An update target is not available. Refresh or view details.`
- Delegated ownership uses `Managed through <Manager>` and explains that the update is grouped and executed through that Manager.
- Manager self-update appears in the Manager group heading, not as a Package row; the Manager Header simultaneously shows `IN PLAN` and `Remove`.
- With confirmation enabled, exact commands are hidden by default behind `Show update command` and the action is `Confirm # updates`.
- With confirmation disabled, exact commands automatically expand before the action is enabled; show `Confirmation is off. Changes will run immediately when you choose Run # updates. Change in Settings.` and label the immediate action `Run # updates`.
- Stale plan: replace invalidated details, explain what changed, and require a fresh confirmation.

## Confirmation

- With confirmation enabled, the base plan footer contains one blue `Confirm # updates` button.
- Pressing it opens the Confirmation Dialog. No safety checkbox appears in the base plan.
- The dialog presents exact commands and final scope before authorization.
- `Change Plan` closes the dialog and returns focus to the first staged item’s Remove control, or the plan heading if no such control survives.
- `Confirm # updates` admits the full plan atomically; partial silent admission is not allowed.
- If admission fails, nothing executes and the dialog explains why.
- With confirmation disabled, the dialog does not open; the visible `Run # updates` action atomically admits the command-expanded plan.

## Live Activity

Visual reference: [live Upgrade Activity and completed Results](./mockups/activity-results.html).

- The confirmed plan is the only live Activity object.
- The sidecar and full Activity are two presentations of one shared live state, never separate executions. The sidecar remains the compact live summary while full Activity shows detailed evidence.
- Keep the compact sidecar readable: when an Operation stalls or requires trusted-classified interaction, summarize the condition there and offer `View full Activity`; place `Keep waiting`, `Copy command`, `Cancel plan`, and expanded command evidence in full Activity rather than crowding the sidecar.
- If History replay opens during a live plan, the sidecar remains visibly live and full Activity is labeled `Viewing past activity`; `Back to live activity` returns the main workspace to the active plan.
- Operations waiting because of resource or Manager locks show `Waiting` and the reason within the active plan.
- Running progress is indeterminate unless the adapter provides a trustworthy measurable total.
- A Manager may show exact completed/total item counts even when the current command has indeterminate progress.
- Homebrew contention is never automatically retried.
- Planned commands run non-interactively with null input. Pack-Manager never presents an inline prompt or accepts a password.
- At the 120-second silence threshold, the stalled Operation presents exactly `Keep waiting`, `Copy command`, and `Cancel plan`. `Keep waiting` rearms monitoring. `Copy command` hands control to the user in Terminal and does not alter the running process. `Cancel plan` requires no second confirmation, changes running work to `Cancelling`, prevents remaining unstarted attempt work from beginning, and states that rollback is not promised.
- Only a closed Manager-specific classifier or explicit native signal may show `Interaction required`. That state includes a plain-language explanation plus `Copy command` and `Cancel plan`; Pack-Manager never accepts the response. Output that does not match a trusted classifier remains an ordinary stall.
- At the 30-minute hard cap, the Operation ends as `Timed out`.
- `Skipped` is not a generic user action: it is a terminal outcome for work that never started because the plan stopped, was cancelled, or became ineligible.
- Cancel, timeout, skip, and failure each end in honest terminal states.
- After a crash or forced quit, reconstruct confirmed unfinished work as `Interrupted` on the next launch.

## Results and recovery

- The sidecar becomes Results and remains until the user chooses `Done`.
- Overall states: success, partial, failed, cancelled, timed out, interrupted.
- Per-item states: verified, failed, cancelled, skipped.
- A successful process exit remains `Verifying` until affected Manager state refreshes.
- Known failures use curated cause-specific guidance. Unknown non-zero exits show evidence without invented advice.
- A repeated identical failure says it repeated and emphasizes resolving the known cause before Retry.
- Retry is always user-controlled and never automatic. It first reveals the failed-item retry scope inside Results with `Cancel` and `Create new plan`; Cancel closes the scope and returns focus to the Retry action. `Create new plan` deliberately replaces the sidecar with a new reviewable draft. The completed result remains immutable in History with a `View previous result` path.
- If the new draft is confirmed, it creates a separate History entry labeled `Retry of plan from <time>`.

## History and replay

Visual reference: [read-only Activity replay with concurrent live Activity](./mockups/history-replay.html).

- Persist one immutable entry for every confirmed execution attempt. A confirmed Retry becomes a new linked entry; it never changes the original.
- Search and filters operate on plan time, result, Manager, Package, and relevant text.
- Replay is read-only and clearly labeled.
- Correlate each command with its affected Manager/Packages and outcome.
- History summaries use verified outcome language such as `10 of 12 verified · 2 failed`, never a generic completion ratio.
- During a live execution, replay remains secondary: the live sidecar stays visible and offers `Back to live activity`.
- External log reveal is secondary to readable inline evidence.

## Settings and application updates

Visual reference: [Settings and Pack-Manager application updates](./mockups/settings-app-updates.html).

- Settings is one scrollable surface grouped into Upgrade safety, Refresh behavior, Advanced operation limits, Pack-Manager updates, and Advanced diagnostics.
- `Skip confirmation for future upgrade plans` is off by default and reversible in Upgrade safety.
- Self-updating/greedy Package behaviors are off by default and include a caution.
- Auto-refresh at launch and Homebrew metadata behavior are explicit preferences.
- Stall threshold, Operation hard cap, and Application log level are editable controls under Advanced. The two duration controls validate permitted numbers and units; log level uses an explicit choice control. Each follows the same immediate `Saving` → `Saved` persistence contract as every other preference.
- Diagnostics includes Environment Report, Copy, Re-detect, Open Logs, and privacy-preserving export.
- Remove any `Auto-open Activity drawer` preference; the sidecar always transforms automatically after confirmation.
- Every Settings control saves immediately and atomically: show `Saving`, activate the new value only after persistence succeeds, then show `Saved`. On failure, retain or restore the prior value and show an inline error.
- Settings → Pack-Manager updates is the canonical detailed application-update location. One restrained title/status-area badge labeled `Pack-Manager Update Ready!` links there when action is available.
- In the detailed update card, identify the app as `Pack-Manager` without repeating the target version in the heading. Keep the installed → target version delta on one line, using warning yellow for the installed version and success green for the target version.
- Application update checking/downloading may run in the background. Restart is always explicit and refused during active Package work.

# Interaction Primitives

## Pointer

- Single click selects navigation, expands disclosure, toggles eligible plan membership, or invokes a clearly labeled action.
- Hover may reveal supporting information but never the sole required meaning.
- Rows that navigate show a consistent hover treatment; rows with Checkboxes do not use whole-row click to surprise-toggle membership.
- Activating an explanatory-disabled Checkbox never changes membership; it may open supplemental help while the persistent reason remains associated with the control.
- Backdrop click may close an idle Confirmation Dialog, never a running operation.

## Keyboard

- Tab order follows visual reading order: sidebar → page header/actions → filters → table/Manager cards → Upgrade Sidecar.
- Enter/Space activates Buttons, Checkboxes, disclosure controls, and selectable rows.
- Escape dismisses popovers and an uncommitted Confirmation Dialog, returning focus to its trigger.
- F6 cycles the primary navigation, main Package Grid/workspace, and Upgrade Sidecar regions without changing selection.
- Opening the Upgrade Sidecar does not steal focus from the Package or Manager action that created it.
- Full Activity replay and long output support keyboard scrolling, selection, and copy.

## Package Grid keyboard model

- The grid exposes persistent column headers, one roving row Tab stop, total filtered row count, and each virtual row’s stable position.
- Up/Down moves one Package; Page Up/Down moves one rendered viewport; Home/End reaches the first/final Package and scrolls it into view.
- Space toggles the active eligible Package’s Upgrade Plan membership. Shift+Up/Down extends a contiguous membership range from the current anchor while respecting pinned, current, excluded, and unavailable eligibility.
- The header Checkbox adds or removes every eligible Package matching the active filter, not merely rendered rows. Its label names the exact consequence (`Add all 8 updates`), and its mixed state is announced.
- Filter changes retain focus on the same Package when it survives; otherwise focus moves to the first available row or the grid heading. Virtualization never silently discards the focused identity.
- Tab from the active row exits the grid and reaches the visible Upgrade Sidecar; Shift+Tab returns to the grid controls.

## Focus transitions

| Transition                            | Required focus behavior                                                                                                                                                                                      |
| ------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| Open Confirmation Dialog              | Move focus to the programmatically focusable dialog heading/command summary. `Change Plan` is the first actionable control; final confirmation is never the accidental default for an unfocused Enter press. |
| Change Plan                           | Move focus to the first staged Remove control, or the editable plan heading when no item control survives.                                                                                                   |
| Escape or backdrop dismissal          | Restore focus to the originating `Confirm # updates` action.                                                                                                                                                 |
| Final confirmation                    | Close the dialog and move focus to the programmatically focusable Upgrade Activity summary heading in the transformed sidecar.                                                                               |
| Activity row/status update            | Never move focus. Preserve the focused Operation or user control.                                                                                                                                            |
| Activity transforms to Results        | Preserve the current viable focus. If that node is removed, move focus to the Results heading and announce one atomic outcome summary.                                                                       |
| Retry scope opens                     | Move focus to the retry-scope heading; cancelling returns to the Retry action. Creating the plan moves focus to the new draft heading.                                                                       |
| Results `Done`                        | Restore focus to the most relevant surviving page action; if it no longer exists, use the current page heading.                                                                                              |
| History replay opens during live work | Keep live-sidecar focus available; main replay receives focus at its heading and exposes `Back to live activity`.                                                                                            |

## Motion and feedback

- Use 120–180ms for hover, focus, and selection feedback; 180–260ms for sidecar/layout transitions; up to 400ms for significant state transforms.
- Animate opacity and transform where possible; avoid layout thrash.
- A running indeterminate indicator communicates activity, not percentage.
- Verified completion uses a short fill/color transition, then settles.
- Under `prefers-reduced-motion`, remove nonessential animation and use immediate state changes, text, and icons.
- Do not use celebratory confetti, bouncing cards, or continuous decorative glow.

# Accessibility Floor

Pack-Manager must meet the packaged-app accessibility method approved by the Architecture Spine.

- All ordinary text and controls target at least 4.5:1 contrast; large display text targets at least 3:1.
- Every interactive element uses a separate `{colors.focusRing}` indicator that is at least 2px wide and visible against every surface. `{colors.borderStrong}` may indicate selection but never substitutes for focus; selected and focused states remain distinguishable.
- Primary navigation, Manager disclosure, Package Grid, Upgrade Sidecar, Confirmation Dialog, Activity, Results, History, and Settings expose meaningful names, roles, states, and relationships to VoiceOver.
- Status is never conveyed only by color, meter length, animation, or icon.
- Version changes are read in an understandable order: Package name, installed version, target version.
- One atomic Activity/Results status channel announces plan start, a changed waiting reason, an action-required failure, each Manager’s completion summary, and the final plan outcome. It uses polite priority by default and assertive priority only for an immediate safety action; it never announces queued rows, progress ticks, or command-output lines.
- Brief Notifications suppress duplicate speech when the status channel already announced the same event and never move focus.
- Opening/closing dialogs and transformations follow the Focus transitions matrix; Results receives one accessible summary announcement.
- Tooltips/popovers are available on keyboard focus/click, not hover only.
- Explanatory-disabled Package Checkboxes do not use native `disabled`. They expose `aria-disabled="true"`, retain focus, associate the persistent reason through an accessible description, remain inert on activation, and keep focus when Escape closes a supplemental Tooltip/Popover. Apply the same rule to pinned, current, excluded, and unavailable aligned controls.
- Hit targets are at least 28 × 28px for compact desktop controls and 36px where space permits.
- The Package Grid exposes total and row-position metadata, scrolls the focused virtual row into view, preserves stable Package identity, and guarantees keyboard/VoiceOver reachability of the final row and actions with at least 101 Packages.
- At 150–200% zoom within the 900 × 600 minimum, the high-zoom layout collapses navigation and presents Plan/Activity/Results as a full-workspace or stacked surface. Primary tasks have no overlapping panes or two-dimensional scrolling.
- Packaged acceptance verifies focus, final-row reachability, selection scope, completion announcements, and no overlap at 100%, 150%, and 200% zoom with VoiceOver and 101 Package rows.
- Reduced-motion behavior removes continuous or sweeping animation.
- Commands and logs are selectable, copyable, and not rendered solely as inaccessible canvas content.

# Inspiration & Anti-patterns

## Inspiration to preserve

- Layered dark graphite surfaces with restrained blue/cyan light.
- A clear blue “control plane” for primary actions and current navigation.
- Compact outlined tools with native macOS rhythm.
- Shallow glass depth, crisp borders, and readable hierarchy.
- Smooth 120–400ms motion that explains changes in state.
- Dense information that still provides breathing room and obvious grouping.

## Anti-patterns to reject

- Copying proprietary Kanban Pro code, assets, branding, exact layouts, or distinctive visuals.
- A permanent redundant Manager list on the Dashboard.
- Terminal-first Upgrade Plan or Activity presentation.
- Immediate row-level execution.
- A permanently visible empty right drawer.
- A queue of draft plans in Activity.
- One History row per Package or command.
- A gradient-filled Package-health bar or fabricated progress.
- Unexplained route/owner jargon.
- Silent retries, silent restart, or requests for administrator credentials.

# Responsive & Platform

Pack-Manager is a native desktop product, not a mobile-responsive website.

- Minimum supported content size: 900 × 600.
- Preferred comfortable size: approximately 1200–1440 × 760–900.
- At ordinary zoom and usable width of at least 720 CSS pixels, keep the 190px sidebar stable.
- At ordinary zoom, when the Upgrade Sidecar opens, allocate 340–380px and allow the main workspace to become one Manager-card column before hiding required information.
- Below 720 usable CSS pixels—such as 150–200% zoom at the minimum window—enter high-zoom mode: collapse navigation to an accessible rail or temporary panel and present Plan/Activity/Results as a full-workspace or stacked surface with a clear Back route.
- At narrow/high-zoom widths, stack secondary metadata but preserve name, state, versions, counts, primary action, error/recovery, focus order, and status announcements.
- Long Package lists and output use internal scrolling.
- Use macOS system typography, traffic-light window controls, expected keyboard behavior, and native menu/quit conventions.
- No `sudo`, password sheet, or elevation prompt is part of any supported interaction.
- Pack-Manager application restart/update follows native explicit-restart expectations and never interrupts active Package work.

# Key Flows

The source acceptance journeys remain authoritative. These maps define their visible experience without restating every technical acceptance criterion.

## AJ-1 — Launch, detect, and refresh

**Goal:** Open Pack-Manager and understand current system reliability and update state.

1. Sallvain launches Pack-Manager from Finder or the Dock.
2. The window shell and navigation render immediately; System Summary Cards and Manager Cards show independent loading states.
3. Detected Managers appear progressively with name, version, role, path, Manager state, managed Package count, Package update count, and health.
4. Normal absence is labeled as not detected rather than failed.
5. Each Manager refreshes independently.
6. **Climax:** the Dashboard settles into `Ready` when all detected Managers are trustworthy, or `Warning` with exact failed-refresh context when one or more rely on a last-good snapshot.

**Failure path:** A refresh failure retains the prior snapshot and timestamp, states what failed, and offers Retry refresh. Other Managers remain usable.

## AJ-2 — Review and authorize Update Everything

**Goal:** Safely approve a complete multi-Manager update without reasoning through CLI commands.

1. Sallvain selects Update Everything.
2. Eligible Manager and Package updates populate the Upgrade Sidecar; pinned, excluded, current, unavailable, and otherwise ineligible items are explicitly omitted with reasons.
3. The plan groups Package version changes by Manager, shows Manager self-update deltas in group headings, and provides a visible Remove action for every staged Package and Manager self-update.
4. Sallvain reviews Updates, Managers, and Commands counts and optionally reveals exact commands.
5. With confirmation enabled, Sallvain presses the single blue `Confirm # updates` action.
6. The Confirmation Dialog dims the background and shows the exact commands, `Change Plan`, final confirmation, and the optional skip-future preference. With confirmation disabled, the plan instead shows `Confirmation is off`, automatically expands commands, and uses the immediate `Run # updates` action.
7. If the plan became stale, Pack-Manager replaces the affected preview, explains the change, and requires reconfirmation.
8. **Climax:** final confirmation atomically admits the whole plan and the sidecar becomes live Upgrade Activity.
9. At terminal state, Activity becomes a persistent Results Summary and the execution becomes one History Plan.

**Failure path:** Admission failure executes nothing. Execution failures remain correlated to the responsible Manager/Package and receive diagnosis, evidence, and safe recovery.

## AJ-3 — Update a selected Package or Manager

**Goal:** Update a limited selection through the same safe plan workflow.

1. Sallvain expands Managers and chooses a Manager.
2. The Manager workspace opens with All Packages selected and actionable updates sorted first.
3. Sallvain checks an eligible Package, uses the count-labeled header Checkbox for every eligible Package matching the active filter—including off-screen rows—or chooses Update Manager.
4. Each action immediately changes Upgrade Plan membership; the sidecar opens on the first addition, provides Remove on every staged item, and persists while Sallvain visits other Managers.
5. A pinned Package remains muted but readable; hover, click, or focus explains how to unpin and refresh it.
6. Sallvain reviews the combined plan and follows the same confirmation path as AJ-2.
7. **Climax:** verified Package rows show the single new current version, and the Results Summary confirms exactly what changed.

**Failure path:** Unchecking removes draft membership. Removing the last item closes the sidecar. No row or header action executes immediately.

## AJ-4 — Handle slow, blocked, failed, cancelled, or interrupted work

**Goal:** Understand and safely recover from abnormal execution.

1. After confirmation, the sidecar and Activity show the active plan with exact Manager/item counts.
2. Waiting work states the lock or ownership reason. Running work uses indeterminate progress unless the adapter provides trustworthy measurable progress.
3. Sallvain may expand command/output evidence without losing the human-readable state.
4. Planned commands run with null input and never request a password. At 120 seconds of silence, the stalled Operation presents `Keep waiting`, `Copy command`, and `Cancel plan`; Homebrew contention never retries automatically.
5. A prompt recognized by the trusted Manager-specific classifier becomes `Interaction required` with `Copy command` and `Cancel plan`, never an inline input field. Unrecognized silence remains a stall. Cancel requires no second confirmation, does not promise rollback, and stops unstarted remainder work; the 30-minute hard cap ends as `Timed out`.
6. **Climax:** Results explains `What happened` and `What to do next` for each failed item, with safe contextual actions, retained evidence, and secondary Retry.
7. Retry first reveals the failed-item scope; `Create new plan` replaces Results with a fresh reviewable draft. If confirmed, it creates a new History entry linked as `Retry of plan from <time>`. A repeated identical cause is called out before another attempt.

**Interrupted path:** After a crash or forced quit, the next launch reconstructs confirmed unfinished work as Interrupted and preserves available evidence.

## AJ-5 — Diagnose and export support evidence

**Goal:** Reconstruct a prior plan and share useful evidence without exposing unnecessary private data.

1. Sallvain opens History and searches or filters for a prior execution.
2. One History Plan Row summarizes the entire attempt by date/time, outcome, verified/total, Managers, duration, and optional Retry source.
3. Opening the row routes Activity into clearly labeled read-only replay. If work is currently live, its sidecar remains visible with `Back to live activity`.
4. Replay reconstructs Manager groups, Package/version changes, Manager updates, command timing, outcomes, errors, and retained output.
5. Sallvain expands Command Output Disclosure or reveals the external log only when deeper evidence is needed.
6. From Settings → Advanced → Diagnostics, Sallvain opens the Environment Report or exports privacy-preserving diagnostics.
7. **Climax:** the exported evidence is useful for support while respecting the redaction and retention requirements in the source contracts.

**Failure path:** Missing or truncated evidence is labeled honestly and points to any retained external log; the UI never invents command output.

## AJ-6 — Install and update Pack-Manager

**Goal:** Install, launch, and explicitly update the application without confusing app updates with Package work.

1. Sallvain installs the trusted packaged application and launches it normally from Finder or the Dock.
2. Application Update Status checks in the background and remains separate from Package Upgrade Plans, Activity, Results, and History. Detailed state lives in Settings → Pack-Manager updates, and one restrained application-level badge labeled `Pack-Manager Update Ready!` links there when action is available.
3. When an update is available, Pack-Manager downloads or presents the approved download state without interrupting Package work.
4. When ready, it presents `Restart to Update`.
5. If Package operations are active, restart is refused with a clear explanation.
6. Sallvain explicitly restarts; the application relaunches and shows the intended new version.
7. **Climax:** Application Update Status returns to current after verified relaunch.

**Manual-install path:** If the installation location is not writable or the updater cannot proceed, present a clear manual-install path. Never request `sudo`, a password, or privilege escalation.
