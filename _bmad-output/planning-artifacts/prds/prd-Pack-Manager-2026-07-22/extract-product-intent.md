# Product Intent Extract — Pack-Manager

**Purpose:** Source extraction for a brownfield, launch-grade PRD. This is not a replacement for the PRD and does not assert that every requirement is already implemented.

**Primary sources:** `docs/SPEC.md` (authoritative product specification) and `docs/DECISIONS.md` (decision log).

**Precedence used in this extract:**

1. A later explicit decision overrides an earlier decision or stale text in the specification.
2. D23a overrides D23: `mas` is installed and verified against live captures; it is no longer an unverified adapter.
3. D25/D25a override D20 and the original F10/P2 delivery language: Pack-Manager now ships through a signed, notarized, stapled, auto-updating release pipeline.
4. D25a overrides D16's fixed five-event count with a sixth application-update event. This is a technical contract change, not a product capability count.
5. D26 creates one narrow exception to the otherwise byte-faithful transcript claim.

## 1. Product thesis

Pack-Manager gives a macOS user who maintains software across several command-line package managers one trustworthy desktop control plane for:

- seeing what is installed and what each manager says is outdated;
- understanding which manager owns and will update each tool;
- updating everything, a chosen subset, one package, or a manager itself;
- reviewing the exact commands before any bulk mutation;
- watching, cancelling, and recovering from long-running work; and
- reconstructing failures from durable local evidence.

The product's differentiator is not merely a graphical wrapper around update commands. Its value is **confidence across a mixed package-manager topology**: no surprise commands, no fabricated version judgments, no hidden privilege prompts, isolated failures, understandable routing, and a durable audit trail.

### Target user and job

**Source-supported primary user:** A macOS power user/developer with several of Homebrew, mise, npm, uv, rustup, and Mac App Store CLI (`mas`) installed, including tools delegated through other managers (for example, npm and uv through mise).

**Core job to be done:** “Help me keep my Mac's developer tools and packages current without memorizing every manager's syntax or accidentally updating the wrong thing, while preserving enough control and evidence to trust and debug every operation.”

**Context of use:**

- The app must work when launched from Finder or the Dock, where shell PATH behavior differs from a terminal.
- The user may also be running package-manager commands in a terminal.
- Network access may be unavailable or intermittently fail for one manager.
- Package versions are not reliably semantic versions and may be labels, hashes, or distribution-specific strings.

**Not established by the sources:** A named persona, secondary user roles, team/enterprise workflows, adoption goals, or commercial positioning.

### Desired product outcomes

1. **One-glance awareness:** The user can quickly see manager availability, ownership, health, package counts, and pending updates.
2. **Controlled mutation:** The user chooses what changes, sees the exact bulk commands, and explicitly authorizes installation or upgrade work.
3. **Correct routing:** Self-updates run through the manager that actually owns the tool, with the route explained in plain language.
4. **Resilient operation:** A failure, timeout, or offline condition in one manager does not erase or block useful information from the others.
5. **Actionable transparency:** Live output, queue state, error copy, history, transcripts, and diagnostics explain what happened and what to do next.
6. **Safe delivery:** The application itself can update through a signed/notarized flow, but installation/restart remains a user decision.

## 2. Product principles and behavioral invariants

These are product-level rules. A violation changes the promised user behavior and should be treated as a product defect.

### PI-1 — Manager truth is authoritative

- A package is outdated only when its package manager reports it as outdated.
- Version-delta styling is explanatory display only.
- Version strings render verbatim.
- When the latest version is unknown, the UI says “update available” and does not invent a version or severity.

### PI-2 — Ownership and self-update routing are discovered

- The product derives who manages a tool from the detected installation, rather than hardcoding a route.
- The user can inspect human-readable evidence for the ownership decision.
- Routes are reconsidered after fresh manager data because a manager's own outdated record can change the appropriate route.
- If the required executor is unavailable, the update action is disabled with a reason.

### PI-3 — Conflicting mutations do not run concurrently

- Pack-Manager never creates internal Homebrew contention.
- Operations that could mutate the same manager or delegated tool tree are serialized.
- Independent managers may run in parallel.
- Routed work tells the user when it is queued behind its executor.
- If Homebrew is busy because of an external terminal process, the product names that condition and does not retry automatically.

### PI-4 — No privilege prompt path

- Pack-Manager never invokes `sudo`, requests a password, or opens an administrator authorization prompt.
- Commands cannot wait for input from the app.
- If a command appears to need input, the user can keep waiting, copy the command for a terminal, or cancel.
- If an application update cannot be installed without elevation, the product requires manual installation.

### PI-5 — Failures are isolated and useful data is retained

- A failed manager refresh does not blank other managers.
- The last successful snapshot for the failed manager remains browsable and is clearly marked stale.
- Offline and timeout states are shown per manager with a retry path.
- Parse failures are visible and actionable rather than crashing or silently presenting incomplete data.

### PI-6 — Bulk work is reviewable and exact

- Every bulk action presents the exact commands before execution.
- Bulk commands contain explicit package names; a bare, indiscriminate update command is not accepted as the plan.
- Excluded items and reasons are visible.
- A changed or stale plan never auto-executes. The UI presents a newly built plan and requires another confirmation.
- Altered, replayed, missing, or otherwise invalid plans enqueue nothing.
- A confirmed multi-group plan is admitted all-or-none; the product does not leave the user with a partially submitted bulk plan.

### PI-7 — Package exclusions preserve user intent

- Pinned Homebrew formulae are never upgradable in-app.
- Self-updating/greedy casks are excluded from selection and bulk updates by default and require explicit opt-in.
- When both mise and rustup report the same Rust toolchain in one plan, the plan updates it through rustup and visibly explains the mise exclusion.
- Cross-manager deduplication is not performed beyond this Rust rule.

### PI-8 — Mutations require user action

- Package upgrades, manager self-updates, health fixes, and application installation/restart require an explicit user action.
- Background application-update checking and downloading are allowed; background installation is not.
- The app does not perform unattended package auto-upgrades.

### PI-9 — Operations remain observable and reconstructible

- Every operation has live output, status, command visibility, a durable transcript, a history record, and correlated diagnostic evidence.
- A crash or forced quit must leave enough evidence to show an unfinished operation as `Interrupted` on the next launch.
- Journaled process identifiers are historical evidence only and are never acted upon after relaunch.
- Transcript content is faithful to child output except for D26's closed, literal allowlist of unterminated `mas` notices, where one readability newline may be inserted. No general heuristic rewriting is allowed.

### PI-10 — Cancellation is immediate and honest

- Cancel does not require a confirmation dialog.
- Cancellation applies to the whole spawned operation, escalates if it does not exit, and ends in an explicit terminal state.
- The product does not promise rollback of a partially completed package-manager command.

### PI-11 — Settings and trusted state do not partially apply

- A setting becomes active only after it is successfully persisted.
- If persistence fails, the previous persisted and in-memory setting remains authoritative.
- Stale UI continuations, dismissed dialogs, and superseded plan results cannot trigger later execution or state changes.

## 3. P0 launch capabilities

The source labels F1–F12 as MVP-required. D25 adds application self-update behavior that now belongs in launch scope even though it post-dates that numbered list.

### F1 — Detect supported managers at launch and on demand

**Product outcome:** The user immediately knows which supported managers are available and who owns them.

**Required behavior:**

- Support Homebrew, mise, npm, uv, rustup, and `mas`.
- Detect at launch and through Re-detect actions.
- For each present manager, show its path, version when available, ownership classification, and evidence.
- Treat absence as a normal state with a “Not installed” presentation and a copyable install hint where one is known.
- Include detection details in the Environment Report and diagnostics.
- Reflect D23a: `mas` has live-verified list and outdated behavior; the old “UNVERIFIED” label and synthetic-fixture limitation are obsolete.

### F2 — Refresh inventory and update status independently

**Product outcome:** The user gets a current, manager-specific inventory without one failure taking down the whole view.

**Required behavior:**

- Refresh each manager's installed inventory and outdated status.
- Refresh All starts work for every present manager, allowing independent managers to proceed concurrently.
- Show independent loading, phase, timeout, and error states.
- Retain and label a manager's previous successful data when its refresh fails.
- A successful package or manager update refreshes the affected manager data automatically.
- Homebrew metadata refresh also serves as Homebrew's self-update route.

### F3 — Browse packages and update eligibility

**Product outcome:** The user can distinguish current, outdated, pinned, and self-updating packages without losing manager-supplied version detail.

**Required behavior:**

- Show package name, installed version, latest version when known, status, selection, and row action.
- Keep package-manager version strings verbatim.
- Use display-only version-delta treatment where values are comparable.
- Disable selection and upgrade for up-to-date or pinned packages, with an explanation.
- Group self-updating/greedy casks separately, collapsed by default, and exclude them from normal select-all and bulk plans.
- Preserve useful manager-specific detail, including uv executables and package kind.

### F4 — Review and confirm an “Update Everything” plan

**Product outcome:** The user can confidently approve a system-wide update because the app shows exactly what it will run and what it will omit.

**Required behavior:**

- “Update Everything” and per-manager “Upgrade all” open a plan sheet.
- The sheet groups work by manager and displays the exact commands.
- Manager self-updates are included by default.
- Self-updating/greedy casks are excluded by default.
- Exclusions, reasons, warnings, and possible stale data are visible.
- Confirmed work runs with independent managers parallelized and conflicting work serialized.
- A stale or otherwise invalid plan is replaced in the sheet and requires renewed confirmation.
- Dismissing the sheet invalidates late results and cannot cause execution.

### F5 — Upgrade a selected subset or one package

**Product outcome:** The user can update only the packages they intend.

**Required behavior:**

- Select outdated, eligible rows individually, by range, by toggle, or through filter-aware select-all.
- Show tri-state selection and a toolbar with count, upgrade, and clear actions.
- “Upgrade selected” opens a plan containing exactly the checked package identities.
- Clear selection after successful enqueue.
- A row-level Upgrade is an intentionally lower-friction immediate action; its command remains visible in operation output and the transcript.
- Pinned, up-to-date, and default-excluded greedy rows cannot enter the selection.

### F6 — Update package managers through an explained route

**Product outcome:** The user can update a manager without needing to know whether it updates itself or must be updated by another manager.

**Required behavior:**

- Each manager pane has a self-update card with installed/latest state, route explanation, and action state.
- Derived examples include Homebrew via refresh, mise via Homebrew when owned there, uv via mise when owned there, npm in-band when it reports itself outdated, and rustup in-band.
- A routed action explains both subject and executor in plain language.
- The manager's self record appears on the self-update card rather than duplicating a package-table row.
- The npm card permanently warns that upgrading the mise-managed Node runtime can reset npm and global packages.
- Queue state is understandable, such as “Queued behind Homebrew.”

### F7 — Monitor, cancel, and recover from stalled work

**Product outcome:** Long-running commands never become an opaque spinner.

**Required behavior:**

- Stream stdout and stderr into a live activity surface with the exact command visible.
- Auto-open activity for upgrades/self-updates according to the user's setting, but not for ordinary refreshes.
- Allow immediate cancellation without a confirmation dialog.
- After configurable silence (default 120 seconds), show a stalled state and explain that the app never enters passwords.
- Offer Keep waiting, Copy command, and Cancel.
- Enforce a configurable hard cap (default 30 minutes) and show a timed-out terminal state.

### F8 — Preserve operation history and transcripts

**Product outcome:** The user can answer what ran, what happened, and where to find the evidence after the fact.

**Required behavior:**

- Persist operation start and finish information and incremental transcripts.
- Show session and prior-history records with manager, status, search, and filters.
- Show start-without-finish work as `Interrupted` after relaunch.
- Let the user inspect the full command and transcript tail and reveal the transcript in Finder.
- Retain the newest 1,000 history records, and operation transcripts for the newest 200 files or 90 days.

### F9 — Export diagnostics without inherited-environment disclosure

**Product outcome:** The user can create one support bundle that explains detection, routing, settings, logs, and recent operations.

**Required behavior:**

- Export a timestamped diagnostics archive to the Desktop.
- Include app/OS/architecture data, resolved search path and source, detection evidence, settings, selected recent logs/transcripts, and history journal.
- Include only environment variables that Pack-Manager itself set; never dump the inherited environment.
- Treat diagnostics file selection and streaming as security-sensitive and do not follow symlink substitutions.

### F10 — Present a coherent, accessible macOS interface

**Product outcome:** The app feels like one focused macOS control surface rather than six disconnected command wrappers.

**Required behavior:**

- Ship a dark-only launch theme with a distinctive package/update identity.
- Use consistent status colors, text labels, mono treatment for commands/versions/logs, and one visual system.
- Remain usable at the minimum window size and with large package tables/logs.
- Honor reduced motion.
- Provide visible focus, keyboard operation, text/icon equivalents for color, completion announcements, and at least 4.5:1 text contrast.
- Launch correctly from Finder and the Dock.

**Delivery correction:** The original F10 ad-hoc-only `.app` requirement is obsolete. Current release behavior is defined in “Application release and self-update” below.

### F11 — Configure behavior and inspect the environment

**Product outcome:** The user can tune automation/timeout/logging preferences and diagnose discovery problems without editing files.

**Required settings and defaults:**

- Run Homebrew metadata update during refresh: on.
- Refresh automatically on launch: on.
- Stall threshold: 120 seconds.
- Upgrade hard cap: 30 minutes.
- App log level: project default is debug for the app's own code.
- Auto-open activity for mutation operations: on.
- Include greedy/self-updating casks by default: off.

**Required tools:** Read-only Environment Report with search-path source and per-tool details/evidence; Copy; Open Logs Folder; Export diagnostics; Re-detect.

### F12 — Meet deterministic launch quality gates

**Product outcome:** Launch behavior is repeatable and regressions in parsing, routing, scheduling, trust boundaries, accessibility-critical interactions, and frontend/backend contracts are caught before release.

**Required behavior:**

- Default Rust and frontend suites pass offline from a clean checkout.
- Default tests do not depend on real package-manager processes, network access, sleeps, or current machine state.
- Real-machine checks remain an explicit developer-run verification for compatibility-sensitive behavior.
- Production build, formatting, lint/type, contract, and unit/integration gates pass before release.

### F18 — Update Pack-Manager itself safely (D25)

**Product outcome:** The user receives application updates without reinstalling manually in the normal case, while retaining control over when the installed app changes.

**Required behavior:**

- Check for application updates on launch, every six hours, and on demand through “Pack-Manager → Check for Updates…”.
- Download an available update automatically in the background.
- Turn the Status Bar update indicator into the explicit install-and-relaunch action when the download is ready.
- Never silently install or restart.
- If the app's parent directory is not writable, enter a manual-install-required state rather than triggering an administrator prompt.
- Preserve standard macOS Edit and Window menu behavior, including common cut/copy/paste/select-all shortcuts.

## 4. Core user journeys and UX surfaces

### Journey A — Launch, detect, and refresh

1. The user launches Pack-Manager from Finder or the Dock.
2. The shell-independent environment is resolved and supported managers are detected.
3. The window renders immediately with skeleton/progressive states rather than a blank screen.
4. If auto-refresh is enabled, managers refresh independently.
5. The Dashboard and Sidebar progressively show availability, ownership, counts, health, and phase/error state.
6. Failed managers retain stale data and offer Retry/View log while other managers remain usable.

### Journey B — Review one manager

1. The user opens a manager from the Sidebar or Dashboard.
2. The Manager Pane shows ownership, version, last-refresh time, self-update route, and package counts.
3. The user searches or filters installed/outdated packages.
4. Version and status detail explains eligibility without overriding the manager's verdict.
5. Missing, pinned, unknown-version, self-updating, error, and clean states all have explicit, non-empty presentations.

### Journey C — Update everything

1. The user invokes “Update Everything.”
2. The plan sheet shows exact commands by manager, included manager self-updates, default-excluded greedy casks, other exclusions, and warnings.
3. The user changes supported toggles if desired.
4. The user confirms.
5. If state changed, the app replaces the plan and requires another confirmation.
6. Otherwise the whole plan is accepted and operations appear in the Activity Drawer.
7. Independent managers proceed concurrently; conflicting managers show useful queue state.
8. Completion triggers refreshed package state and success/failure feedback.

### Journey D — Update selected packages

1. The user filters the table and selects eligible outdated rows.
2. Tri-state and selection-toolbar feedback confirm the exact selection.
3. “Upgrade selected” opens the same exact-command plan sheet scoped to those package identities.
4. On successful enqueue, selection clears.
5. A single row can be updated directly for a one-click low-blast-radius path, with the command visible in activity/history.

### Journey E — Update a manager

1. The user reviews the manager's Self Update card.
2. The card explains whether the update runs in-band, through another manager, through refresh, or is unavailable.
3. Any topology-specific consequence is shown at the point of action, especially npm inside mise.
4. The user starts the update and sees executor-aware queued/running state.
5. The affected manager(s) refresh after success.

### Journey F — Handle a slow, failed, or externally blocked operation

1. Live output and current command appear in the Activity Drawer.
2. A silent operation becomes visibly stalled.
3. The user keeps waiting, copies the command to run manually in a terminal, or cancels.
4. Homebrew external-lock contention is named and asks the user to retry after the terminal operation finishes; the app does not auto-retry.
5. Failure feedback persists and links directly to the relevant log.

### Journey G — Investigate history and export support evidence

1. The user opens History and filters by manager, status, or search.
2. A row shows command, outcome, duration, exit state, and transcript tail.
3. The user reveals the full log in Finder.
4. For a broader problem, Settings or History exports one diagnostics zip.

### Journey H — Update the application

1. The app checks in the background or the user checks from the app menu.
2. When an update exists, it downloads without interrupting package work.
3. A Status Bar control tells the user it is ready.
4. The user explicitly chooses Restart to install and relaunch.
5. If the install location is not writable, the app explains that manual installation is required and never asks for administrator privileges.

### Primary surfaces

- **Sidebar:** Global refresh/update actions, Dashboard, detected managers, collapsed “Not installed” managers, History, Settings, per-manager counts/status.
- **Dashboard:** One-glance manager cards with ownership, version/path, update count, self-state, phase/error/absent/busy state, and manager-level actions.
- **Manager Pane:** Self Update card, optional Health banner, filters, package table, selection toolbar, row actions.
- **Upgrade Plan Sheet:** Exact commands, toggles, exclusions, warnings, stale-plan replacement, final confirmation.
- **Activity Drawer:** Queued/running/terminal operations, live logs, exact command, cancel, queue reason, reveal-log action.
- **Status Bar:** Last refresh, health, operation summary, logs/settings access, and application-update readiness/action.
- **History:** Durable operation list, filters, detail, transcript tail, reveal/export actions.
- **Settings:** Behavioral settings, Environment Report, logs, diagnostics, Re-detect.
- **Dialogs and notifications:** Stall handling, quit guard, plan confirmation, outcome toasts, native notification later.
- **macOS app menu:** On-demand application update check while preserving standard Edit and Window shortcuts.

### Keyboard and accessibility contract

- Refresh current/all, upgrade selected/everything, select visible, toggle row, clear/close, toggle activity, focus search, and navigate Sidebar are keyboard accessible.
- Tables use a roving focus model.
- Operation completion is announced through a live region.
- Interactive elements have visible focus.
- Color never carries status alone.
- Reduced-motion preference disables transitions.
- Text contrast target is at least 4.5:1.

## 5. Priorities beyond P0

### P1 — Ship-with polish after every P0 gate is green

1. **Health fixes:** Show uv broken-environment warnings. Only an exact recognized reinstall suggestion may become a copyable/runnable fix; altered, absent, or malformed suggestions remain visible but non-runnable.
2. **Snapshot cache:** Render prior snapshots immediately on launch with “stale — refreshing…”.
3. **Native notifications:** Promote relevant toasts to native notifications when the app is backgrounded.
4. **Package detail popover:** Show manager-specific details such as uv executables, mise source, npm dependency information, and Homebrew pinned version.
5. **Rust ownership note:** Show “Also managed by rustup” on mise's Rust row.

### P2 / deferred

- Light theme.
- Menu-bar extra.
- Scheduled package refresh.
- Cross-manager package deduplication beyond the one-plan Rust rule.
- `cargo install` support.

**Removed from P2 by D25:** A notarized DMG is no longer deferred; signed/notarized packaging is current release behavior.

### Explicit non-goals

- Installing or uninstalling packages; Pack-Manager is update-only.
- Any `sudo`, password-entry, or administrator-prompt workflow.
- Unattended package or manager upgrades.
- Silent application installation/restart.
- Telemetry.
- Replacing manager-reported outdatedness with local version comparison.
- Forcing pinned Homebrew formulae to update.
- Automatically retrying external Homebrew lock contention.
- General-purpose terminal/shell command execution.
- Rollback of partially completed package-manager operations.

## 6. Product-level non-functional requirements

### Safety and trust

- Bulk execution is fail-closed: unreviewed, altered, replayed, stale, or partially admissible work runs nothing.
- Command previews and execution agree exactly for bulk work.
- Privilege escalation is categorically unavailable.
- User exclusions and package-manager protections are preserved.
- Display strings are never treated as a trusted source of executable arguments.
- Application-update download may be automatic; installation is explicitly gated by the user.

### Reliability and recovery

- Manager refreshes fail independently.
- Last-good data survives a later refresh failure.
- Duplicate refresh requests for the same manager coalesce rather than conflict.
- Conflicting mutations serialize; unrelated managers retain concurrency.
- Cancellation, timeouts, stalls, crashes, and partial command completion have explicit terminal/recovery states.
- Settings updates and critical journal rewrites are atomic from the user's perspective.
- A full disk or noncritical app-log failure does not hang package work; failure to create the operation transcript prevents an unaudited command from starting.

### Performance and responsiveness

- The app renders progressive detection/skeleton state instead of waiting for a full refresh.
- Independent manager work can proceed concurrently, with a global resource cap.
- Live output is delivered in near-real time while avoiding UI flooding.
- Package tables remain usable beyond 100 rows.
- The live view remains bounded while preserving earlier output in the on-disk transcript.

### Accessibility and interaction quality

- Complete keyboard paths exist for primary refresh, selection, update, search, navigation, and activity actions.
- Focus is visible and managed predictably.
- Motion can be disabled.
- Status is never color-only.
- Text contrast meets 4.5:1.
- Live operation completions are announced.
- Narrow-window behavior scrolls rather than allowing essential table content to collide.

### Privacy and security

- No telemetry.
- No inherited environment dump in logs or diagnostics.
- Only explicitly constructed environment values may be recorded.
- Diagnostic export resists symlink substitution.
- The app does not expose a generic shell-command surface.
- External-content, capability, and permission changes are security-sensitive because the current application has narrow native capabilities.

### Observability and supportability

- Every operation is correlated across status, output, transcript, structured log, and history.
- User-facing errors say what happened and suggest a next action.
- Error actions never link to a nonexistent operation log.
- Detection and routing evidence is visible in the UI and included in diagnostics.
- Retention bounds are explicit: app logs older than 14 days are pruned; transcripts keep the newest 200 or 90 days; history keeps the newest 1,000 records.

### Compatibility

- The product is a macOS desktop application and must launch correctly outside a terminal.
- The source design target is macOS 27 beta on Apple silicon, but the current release is universal for arm64 and x86_64.
- Package-manager output incompatibility fails visibly and per manager rather than crashing the app.
- Real-machine compatibility checks supplement, but do not contaminate, deterministic default tests.

### Testability and release confidence

- Normal test suites are deterministic, offline, and clean-checkout safe.
- Manager parsers are grounded in real captures; synthetic data can prove robustness but not real-format correctness.
- Frontend/backend contracts are checked from the same representative payloads.
- Concurrency, stale-plan, cancellation, timeout, parsing, error, accessibility-critical keyboard, and release-update paths require focused coverage.

## 7. Application release and self-update behavior

This section reflects D25/D25a and SPEC §8, not superseded D20/F10/P2 language.

### User-visible release contract

- Release artifacts support both Apple silicon and Intel Macs.
- The application, disk image, and updater payload are signed and notarized; relevant bundles are stapled.
- The installed application can discover a newer release, download it in the background, and offer an explicit install-and-relaunch action.
- The normal update path must not require Gatekeeper to contact Apple on first launch of the newly installed app.
- The app must launch from Finder/Dock, not only through development tooling or a terminal.
- If the current installation location cannot be written without elevation, the user receives a manual-install-required state.

### Current distribution behavior

- Updates are sourced from release metadata attached to GitHub Releases.
- The release includes `.dmg`, `.zip`, `.app.tar.gz`, signature, and `latest.json` artifacts needed by direct download and in-app update clients.
- Application update checks occur on launch, every six hours, and through an explicit app-menu command.
- Download is automatic; install/restart is user-triggered.

### Launch release gates implied by the sources

- Offline Rust/frontend suites and production builds are green.
- Contract fixtures agree across Rust and TypeScript.
- The built app launches from Finder/Dock.
- Universal architecture, signature, notarization, stapling, updater archive/signature, and `latest.json` are mutually consistent.
- The update flow is testable without network access through an application-update seam, while the real release path is verified separately.
- No release behavior can introduce an administrator prompt.

## 8. Product terms

- **Manager:** One of Homebrew (`brew`), mise, npm, uv, rustup, or `mas`.
- **Package:** One manager-owned update unit, such as a formula, cask, tool, global package, toolchain, or App Store app.
- **Outdated:** A package-manager verdict that an update is available. It is not inferred by Pack-Manager.
- **Snapshot:** The latest merged installed/outdated view for one manager.
- **Stale snapshot:** The last successful snapshot retained after a newer refresh failed or while cached data refreshes.
- **Operation:** One queued unit of refresh, package upgrade, manager self-update, or health fix.
- **Subject:** The manager whose state the operation changes.
- **Executor:** The manager/binary that actually runs the operation.
- **Route:** The discovered way a manager updates: in-band, routed through another manager, via refresh, or unavailable.
- **Managed by:** The ownership relationship inferred from the detected installation path and shown with evidence.
- **Plan / Upgrade Plan:** The exact, user-reviewable set of commands, exclusions, notes, and warnings for a bulk update.
- **Plan capability (`planId`):** A one-use authorization bound to the reviewed plan and current state. If it is invalid or stale, no operation starts.
- **All-outdated intent:** A request to build a plan from all currently outdated, eligible items rather than a frozen explicit selection.
- **Pinned formula:** A Homebrew package the user has intentionally pinned; never updated in-app.
- **Self-updating/greedy cask:** A Homebrew cask that normally manages its own update behavior; excluded from bulk work unless explicitly included.
- **Health issue:** A manager-reported warning about a package/tool environment. Only narrowly recognized fixes can become runnable.
- **Interrupted:** An operation with a durable start but no durable finish after a prior app session ended.
- **Stalled:** An operation that has produced no output for the configured silence threshold but has not yet timed out.
- **Application update:** An update to Pack-Manager itself, distinct from package/manager operations and their History queue.

## 9. Technical mechanisms to keep out of the PRD's product narrative

The following source material supports the product requirements but belongs primarily in architecture, implementation planning, or an addendum:

- Tauri/Rust/React/TypeScript/Vite/Tailwind/Zustand dependency choices and repository layout.
- Exact PATH search-list order, login-shell probe command, sentinel strings, and constructed environment-variable list.
- Raw-path-before-canonicalization algorithm details. The PRD should retain the outcome: accurate discovered ownership, evidence, and dynamic routing.
- Adapter trait signatures, command arrays, parser regexes, fallback-parser implementation, fixture file names, and synthetic-fixture mechanics.
- Tokio scheduler task design, lock-set data structures, semaphore implementation, queue-aging algorithm, and process-group system calls. The PRD should retain conflict serialization, useful queue states, concurrency, cancellation, and timeout outcomes.
- IPC command signatures, serialized field casing, payload structs, event subscription architecture, command/event counts, and frontend store shape.
- Exact transcript header syntax, log library configuration, span names, and source-module placement.
- Fake runner/event/time seams and individual unit-test names.
- Tauri updater API choice, Rust update-state module, GitHub metadata transport, signing environment variables, and workflow step implementation.

Technical details that **do** earn a place in a launch PRD because they directly constrain observable trust or acceptance include:

- exact bulk preview/execution agreement;
- stale-plan reconfirmation and zero-enqueue failure behavior;
- no shell strings or privilege path;
- atomic all-or-none bulk admission;
- last-good snapshot retention;
- interruption/history durability;
- diagnostic privacy;
- signed/notarized/stapled universal delivery; and
- explicit restart installation for application updates.

## 10. Conflicts, stale statements, and unresolved ambiguities

### Resolved by explicit precedence

1. **`mas` absent/unverified vs live-verified:** SPEC F1/F2 acceptance examples, target-machine facts, parser text, and several tests still describe `mas` as absent or synthetic/unverified. D23a supersedes those machine-specific statements. The generic “absent manager is normal” behavior remains required.
2. **Ad-hoc `.app` vs signed/notarized delivery:** SPEC F10, original P2, and D20 say notarization is out of scope. D25/D25a and SPEC §8 supersede them. Notarized delivery and in-app update are current requirements.
3. **Five events vs six:** D16 and SPEC's exact event list omit application updates. D25a explicitly accepts a sixth app-update status event. This should not be framed as a product conflict.
4. **No auto-upgrades vs auto-download:** D25 resolves the apparent conflict: automatic download is allowed because installation is the machine mutation; installation/restart still requires a click.
5. **Scheduled refresh out of scope vs six-hour checks:** P2 defers scheduled **package-manager refresh**. D25 schedules **application-update checks**. They are different jobs.
6. **Byte-faithful transcripts vs inserted newline:** D26 permits one closed, literal exception for known unterminated `mas` notices. The stronger accurate statement is “faithful except for this allowlisted readability repair.”

### Product ambiguities requiring PRD or UX reconciliation

1. **Primary persona and launch audience:** The sources describe one power-user target machine and a personal-tool origin, but not a named persona, breadth of supported macOS users, or whether public distribution is now intended.
2. **Supported macOS range:** The design target is macOS 27 beta and the bundle is universal, but the minimum supported macOS version is not stated.
3. **Success measures:** No adoption, task-time, failure-rate, update-completion, supportability, or user-confidence metrics are defined.
4. **Onboarding/first run:** The expected first-run explanation, permissions, empty state, and guidance for users with no supported managers are not fully specified.
5. **Duplicate global actions:** SPEC §4.5 places Refresh All/Update Everything in the Sidebar, while §4.7 also places them in the Dashboard header. The product need is clear; whether both entry points are required is not.
6. **Single-row trust timing:** D6 allows an immediate row-level upgrade and says the command is visible in the drawer/transcript. The load-bearing slogan “Nothing runs that was not shown” is explicitly framed around bulk work, but the sources do not state whether a row command is displayed before spawn or only immediately after.
7. **Quit behavior:** A quit guard is specified for running operations, but behavior for queued-only work, app-update installation during package activity, or OS shutdown is not fully defined.
8. **Application-update failure UX:** Manual-install-required is defined, but signature/download/check failure messages, retry rules, and whether a downloaded update survives relaunch are not described at product level.
9. **Release cadence/rollback:** Delivery artifacts and update checks are defined, but phased rollout, rollback, downgrade, skipped releases, and update-channel behavior are not.
10. **Diagnostics consent/redaction:** Contents are enumerated and inherited environment is excluded, but there is no preview/redaction step or explicit sharing workflow.
11. **History management:** Automatic retention is defined, but user-facing clear/delete behavior is not.
12. **Accessibility verification:** Requirements are strong, but assistive-technology targets and a manual accessibility release gate are not named.
13. **P1 promotion status:** The source priority remains P1, but a brownfield PRD must verify whether Health Fixes, snapshot caching, notifications, detail popovers, or the Rust ownership note have already shipped before treating them as future scope.
14. **Current implementation vs source acceptance examples:** Several acceptance examples are intentionally machine-specific and now stale after D23a. Brownfield launch acceptance should be written as topology-independent behavior plus separate current-machine verification.

## 11. PRD drafting guidance from this extraction

- Write the PRD around user confidence, controlled updates, isolated failure, and evidence—not around the Tauri/Rust implementation.
- Preserve F1–F12 as the P0 capability backbone and add application self-update from D25 as a current launch capability.
- Turn the behavioral invariants into globally numbered functional requirements; do not bury them as architecture notes.
- Keep the concrete journeys because the product has meaningful multi-step trust, failure, and recovery UX.
- Treat exact source commands and routing examples as acceptance examples, not fixed product routes.
- Remove superseded absence, unverified-`mas`, ad-hoc-signing, notarization-out-of-scope, and five-event claims.
- Carry unresolved product questions into the PRD rather than inventing personas, metrics, OS support, or rollout policy.
