# PRD ↔ Code Reconciliation

**Reviewer:** `src/` + `src-tauri/` at HEAD
**HEAD:** `5972109f46efe730d8b02c69cff3273aa80adfaa` (`git rev-parse HEAD`)
**Date:** 2026-07-25
**Subject:** `prd.md` + `addendum.md` in `_bmad-output/planning-artifacts/prds/prd-Pack-Manager-2026-07-25/`

## Method

Every Shipping / Partial / Planned tag was checked against the source at HEAD. Each
claim below is either confirmed with a `path:line "literal quote"`, or reported as a
gap. Counts come from named commands. Nothing here is estimated.

Per the review scope, the following are **out of scope and their absence is correct**:
the retired D33 readiness apparatus (72-criterion gate, coverage percentages, scenario
contracts, evidence manifest, candidate freeze, multi-host, `contracts/`), and the D37-
removed criteria (keyboard operability of primary actions, VoiceOver operability,
live-region announcements, deterministic focus restoration). Neither is reported.

---

## Part 1 — Structural anchors verified

| PRD claim | Verified at | Verdict |
| --- | --- | --- |
| 20 registered commands | `src-tauri/src/lib.rs:232-253` `generate_handler!` block; `commands::detect_managers` … `commands::install_app_update` | **20** — confirmed |
| 6 events | `src-tauri/src/events.rs:77-82` — `EVENT_DETECTION_UPDATED`, `EVENT_SNAPSHOT_UPDATED`, `EVENT_OP_STATUS`, `EVENT_OP_OUTPUT`, `EVENT_OP_STALLED`, `EVENT_APP_UPDATE_STATUS` | **6** — confirmed |
| 8 `Settings` fields | `src-tauri/src/settings.rs:28-39` | **8** — confirmed |
| `autoCheckForUpdates` default `true` | `settings.rs:51` `auto_check_for_updates: true,` | confirmed |
| All 8 defaults (FR-17, prd.md:394) | `settings.rs:43-52`: `run_brew_update_on_refresh: true`, `auto_refresh_on_launch: true`, `stall_after_secs: 120`, `upgrade_hard_cap_mins: 30`, `log_level: LogLevel::Debug`, `auto_open_drawer: true`, `include_greedy_by_default: false`, `auto_check_for_updates: true` | confirmed, all eight, exactly as PRD states |
| `MAX_CONCURRENCY` = 4 (FR-9) | `src-tauri/src/queue.rs:48` `pub const MAX_CONCURRENCY: usize = 4;` | confirmed |
| `ISSUED_PLAN_LIMIT` = 64 (FR-8) | `src-tauri/src/state.rs:25` `pub const ISSUED_PLAN_LIMIT: usize = 64;` | confirmed |
| Oldest-first eviction (FR-8) | `state.rs:39` `order: VecDeque<String>,`; `state.rs:165-166` `if let Some(oldest) = self.order.pop_front() { self.plans.remove(&oldest);` | confirmed |
| `MAX_PLAN_SELECTIONS` = 2048 | `queue.rs:400` `pub const MAX_PLAN_SELECTIONS: usize = 2_048;` | confirmed — **not stated in the PRD**; see Part 3, note A |
| `LOG_CAP` = 5000 (NFR-3) | `src/store/operations.ts:17` `export const LOG_CAP = 5000;` | confirmed |
| Flush at 50 ms / 64 lines / 8 KiB (NFR-3) | `events.rs:183` `BATCH_MAX_LINES: usize = 64`; `events.rs:185` `BATCH_MAX_BYTES: usize = 8 * 1024`; `events.rs:187` `BATCH_MAX_DELAY: Duration = Duration::from_millis(50)` | confirmed |
| Virtualization above 100 rows (NFR-3 "101 rows") | `src/components/manager/PackageTable.tsx:15` `const VIRTUALIZE_ABOVE = 100;` | confirmed |
| Log-view virtualization | `src/components/activity/LiveLogView.tsx:24` `const VIRTUALIZE_ABOVE = 200;` | confirmed — not a PRD claim, recorded for completeness |
| `minWidth` 900 / `minHeight` 600 (NFR-3, FR-19) | `src-tauri/tauri.conf.json:18-19` `"minWidth": 900,` `"minHeight": 600,` | confirmed |
| `minimumSystemVersion` 15.0 (NFR-7, §2.2) | `src-tauri/tauri.conf.json:48` `"minimumSystemVersion": "15.0"` | confirmed |
| Two-layer app-update refusal (FR-21) | Backend: `commands.rs:772` `fn refuse_app_update_while_busy(records: &[crate::ipc::OperationRecord]) -> Result<(), IpcError>`, called at `commands.rs:810`. Frontend: `src/components/shell/UpdateStatusItem.tsx:34-37`. `commands.rs:802-804` states the design: "The refusal below is the enforcement point — the frontend check stays as the path that explains itself to the user, and this one exists so no caller can bypass it." | confirmed, both layers |
| Queued counts as active (FR-21) | `commands.rs:776-779` `matches!(record.status, crate::ipc::OpStatus::Queued \| crate::ipc::OpStatus::Running)` | confirmed |
| Retention: 14d logs / 200 files / 90d transcripts / 1000 History (FR-15) | `logging.rs:26-28` `APP_LOG_RETENTION_DAYS: i64 = 14`, `TRANSCRIPT_RETENTION_DAYS: i64 = 90`, `TRANSCRIPT_MAX_FILES: usize = 200`; `journal.rs:19` `COMPACT_KEEP: usize = 1000` | confirmed, all four |
| Diagnostics: 3 logs / 25 transcripts / Desktop (FR-18) | `diagnostics.rs:22-23` `APP_LOGS_INCLUDED: usize = 3`, `TRANSCRIPTS_INCLUDED: usize = 25`; `diagnostics.rs:4` `~/Desktop/Pack-Manager-diagnostics-<YYYYMMDD-HHmmss>.zip` | confirmed |
| SIGTERM → 5s → SIGKILL over the process group (FR-14) | `process/runner.rs:57` `TERM_GRACE: Duration = Duration::from_secs(5);`; `runner.rs:261` `killpg(pgid, Signal::SIGTERM)`; `runner.rs:270` `killpg(pgid, Signal::SIGKILL)`; `runner.rs:304` `.process_group(0);` | confirmed |
| Focus indicator across 31 sites (FR-19 note 1) | `grep -ro 'focus-visible:outline-2' src/ --include='*.tsx' --include='*.ts' --include='*.css' \| wc -l` → **31**, across 20 files | confirmed exactly |
| CI-asserted focus + contrast (FR-19, §7.1) | `tests/e2e/browser-style-contract.spec.ts:96-102` asserts `outlineStyle === "solid"`, `outlineWidth === "2px"`, `outlineColor === "rgb(244, 247, 251)"` and `!== "rgb(101, 167, 255)"`; `:226` `test("[P0] paints bright accent fills with ink that clears the 4.5:1 contrast floor"`. Runs in CI: `.github/workflows/test.yml:88` `- name: Run Playwright shard` | confirmed |
| Two release-blocking checks (NFR-8) | `.github/workflows/release.yml:319-320` `minisign -V -p "$RUNNER_TEMP/updater.pub" … \|\| { echo "::error::updater signature does not verify against the configured pubkey"; exit 1; }`; `:387-391` fetches `latest.json` and `echo "::error::latest.json reports $PUBLISHED, expected $VERSION"; exit 1` | confirmed |
| Both darwin arch keys → one universal archive (FR-22) | `release.yml:341-342` `"darwin-aarch64": {signature: $sig, url: $url},` `"darwin-x86_64":  {signature: $sig, url: $url}`; `:192` `npm run tauri build -- --target universal-apple-darwin` | confirmed |
| Six Managers, closed set (§3 Glossary, FR-1) | `src-tauri/src/ipc.rs:35-42` `pub const ALL: [ManagerId; 6] = [Brew, Mise, Npm, Uv, Rustup, Mas]` | confirmed |
| Four Route kinds (§3 Glossary) | `ipc.rs:200-227` `SelfUpdateRoute` = `InBand`, `Routed`, `ViaRefresh`, `Unavailable` | confirmed — glossary wording matches variant-for-variant |
| Route precedence (FR-4) | `detect.rs:12-13` `//! resolves the self-update route with the SPEC §5.3 precedence:` / `//! in-band override → delegated-if-detected → native → unavailable.` | confirmed verbatim |
| Brew self-update is ViaRefresh (FR-3) | `managers/brew.rs:286-288` `// brew never lists itself; \`brew update\` (part of every refresh) IS` / `// its self-update (SPEC §5.3 native rule).` / `SelfUpdateRoute::ViaRefresh {` | confirmed |
| Refresh coalescing (FR-3) | `queue.rs:1068-1071` `// Duplicate refresh coalesces to the existing opId (SPEC §5.7).` … `tracing::debug!(subject = %sub.subject, op = %existing, "refresh coalesced");` | confirmed |
| Post-success auto-refresh (FR-3) | `queue.rs:1268` `// Successful upgrades auto-enqueue a refresh of the affected managers.` | confirmed |
| No shell, constructed env, null stdin (FR-12, NFR-5) | `process/runner.rs:299-301` `.env_clear()` / `.envs(spec.env.iter().cloned())` / `.stdin(Stdio::null()) // no sudo, no password entry, ever` | confirmed |
| Preview/argv byte-equality fails closed (FR-7, FR-12) | `queue.rs:272-276` `let expected_preview = ipc::command_preview(adapter_for(executor).binary_name(), args); if preview != expected_preview {` … `"self-update preview/argv mismatch for {subject}: expected \`{expected_preview}\`"`; re-derivation before execution at `commands.rs:387-393` `let mut fresh = canonical_plan(&issued.request, &current);` … `if fresh != issued.plan { return Err(plan_stale("package-manager state changed after preview")); }` | confirmed |
| Transcript failure blocks the spawn (NFR-4) | `queue.rs:1499-1510` `let transcript = match Transcript::create(&op.log_path) { … Err(e) => { … let _ = tx.send(Msg::Finished { op_id, status: OpStatus::Failed, …}); return; } };` | confirmed |
| Persist-before-publish settings (FR-17) | `commands.rs:643-649` `// Persist before publishing: a failed write leaves both the in-memory` / `// settings and the canonical plan revision unchanged.` / `merged.save_to(&state.settings_path).map_err(IpcError::from)?;` … `coordinator.bump_revision();` | confirmed |
| Atomic settings write (FR-17) | `settings.rs:140-146` temp file + `f.sync_all()?` + `std::fs::rename(&tmp, path)` | confirmed |
| `Interrupted` reconstruction (FR-15) | `journal.rs:147` `/// otherwise \`Interrupted\` (start-without-finish — SPEC F8).`; `journal.rs:154` `None => (OpStatus::Interrupted, None, None),` | confirmed |
| Recorded pgids never signalled (FR-15) | `journal.rs:5` `//! finish renders \`Interrupted\` on the next launch. Recorded pgids are NEVER` | confirmed |
| D26 unterminated-notice split (FR-15) | `process/runner.rs:86` `/// a verbatim string from a captured transcript, never a pattern. See D26.`; `:97` `fn split_unterminated_notices(piece: &str) -> Vec<String>` | confirmed |
| `ParseFailed` carries an excerpt (FR-2) | `error.rs:33-35` `/// \`excerpt\` = first 500 chars of the offending output.` / `ParseFailed { what: String, excerpt: String },` | confirmed |
| Rust dedup, exactly one rule (FR-5, §6) | `queue.rs:549` `// 3. rust-dedup (DECISIONS D10): one plan never races two upgrades of the`; user-facing note `src/components/dialogs/UpgradePlanSheet.tsx:30` `rustDedup: "handled by rustup",` | confirmed |
| Brew lock contention named distinctly, never retried (FR-9) | `error.rs:31` `BrewLockBusy { detail: String },`; `error.rs:6` `//! \`ParseFailed\` on refresh keeps the previous snapshot; \`BrewLockBusy\` is a` | confirmed; no retry path found |
| "Queued behind <executor>" (FR-9) | `src/components/manager/SelfUpdateCard.tsx:128` `<Chip tone="neutral">Queued behind {executorName}</Chip>` | confirmed |
| Stall dialog states no passwords (FR-14) | `src/components/dialogs/StalledOperationDialog.tsx:3` `* \`op:stalled\` event. Pack-Manager never enters passwords, so a silent command is`; `:52` `passwords. You can run it yourself in a terminal, keep waiting, or cancel.` | confirmed |
| Stall dialog offers Keep waiting / Copy / Cancel (FR-14) | `StalledOperationDialog.tsx:54` `<CopyableCommand …>`, `:57` `Keep waiting`, `:60` `Cancel operation` | confirmed |
| Refresh never auto-opens, never success-toasts (FR-13) | `src/components/activity/useOperationEffects.ts:48` `const mutating = op.kind !== "refresh";`; `:53` `if (mutating && ui.settings?.autoOpenDrawer)`; `:65` `if (mutating) ui.pushToast({ kind: "success", …})` | confirmed |
| First sightings are silent (FR-13) | `useOperationEffects.ts:94-95` `// Only act on transitions of already-known ops; first sightings are silent.` / `if (before === undefined \|\| before === op.status) continue;` | confirmed |
| Only manual checks notify; no re-notify (FR-20) | `src/lib/ipc/events.ts:123` `if (status.lastTrigger !== "manual") return;`; `:126` `if (previous?.state.kind === status.state.kind && previous.lastTrigger === "manual") return;` | confirmed |
| Launch + 6h checks (FR-20, RP-1) | `app_update.rs:27` `pub const AUTO_CHECK_INTERVAL: std::time::Duration = std::time::Duration::from_secs(6 * 60 * 60);`; `lib.rs:146-148` `tokio::time::interval(app_update::AUTO_CHECK_INTERVAL)` in a `loop { ticker.tick().await;` (first tick is immediate = launch check); menu trigger at `lib.rs:208-223` | confirmed |
| Non-writable install → manual-install-required (FR-21) | `app_update.rs:183` `AppUpdateState::ManualInstallRequired { version, reason },` | confirmed |
| Edit + Window submenus re-declared (RP-2) | `lib.rs:100-113` Edit submenu with `undo/redo/cut/copy/paste/select_all`; `:120-130` Window submenu; `lib.rs:55-56` `// notably the Edit submenu, without which ⌘X/⌘C/⌘V/⌘A stop working in the` / `// package search field and every CopyableCommand.` | confirmed |
| Reduced motion (FR-19, NFR-6) | `src/styles/theme.css:60-61` `/* prefers-reduced-motion disables all transitions (default 150ms ease). */` / `@media (prefers-reduced-motion: reduce) {` | confirmed |
| Aurora Control Deck palette + focus token (FR-19) | `theme.css:4` `a light theme is a value swap. Values are the approved "Aurora Control Deck"`; `:19` `--color-focus-ring:    #F4F7FB;`; `:27` `--color-accent:        #65A7FF;` | confirmed; focus token ≠ accent token |
| Status chips do not wrap (FR-19) | `src/components/primitives/Chip.tsx:25` `"inline-flex shrink-0 items-center gap-1 whitespace-nowrap rounded-full border px-2 py-0.5",` | confirmed |
| "Update Everything" goes through review (§4.2, AJ-2) | `src/components/shell/Sidebar.tsx:53-60` `updateEverything()` calls `buildUpgradePlan(...)` then `openDialog({ kind: "upgradePlan", plan })` | confirmed — it does not execute directly |

### Planned tokens really are absent

`grep -rn <token> src/ src-tauri/src/ | wc -l`:

| Token | Count |
| --- | --- |
| `planAttemptId` | 0 |
| `plan_attempt_id` | 0 |
| `Verifying` | 0 |
| `InteractionRequired` | 0 |
| `skipUpgradePlanConfirmation` | 0 |
| `skip_upgrade_plan_confirmation` | 0 |

All six confirmed zero. **prd.md:24's assertion is exact.** FR-6, FR-10, and the
Planned limbs of FR-7, FR-11, FR-13, FR-14, FR-15, FR-17, FR-19 and NFR-4 are
correctly tagged: none of that state exists.

### The pre-D27 state FR-6 describes is real

- `src/components/manager/ManagerPane.tsx:145-152`:
  `async function upgradeRow(pkg: Package) {` /
  `// Single-package plan executes immediately — no sheet (SPEC §F5).` /
  … / `await executePlan(plan);`
- Transient selection layer: `src/store/packages.ts:17` `selection: Partial<Record<ManagerId, Set<string>>>;`
- Transient draft: `ManagerPane.tsx:130` `openDialog({ kind: "upgradePlan", plan });` — plan lives in dialog state, discarded on close.

prd.md:230 is accurate as far as it goes. See Finding 2 for what it omits.

---

## Part 2 — Findings

### F1 — CRITICAL: FR-14 tags the quit guard as shipping. It is entirely unwired.

**PRD says** (prd.md:340, status line):
> **Status:** Partial. The 120-second stall threshold, the 30-minute hard cap, immediate cancellation with process-group escalation, and explicit terminal states all ship. Attempt-wide `Cancel plan` and trusted interaction classification are Planned — D30.

That enumeration makes every other consequence in FR-14 a Shipping claim, including
(prd.md:349):
> - Quitting with work in flight presents an explicit choice and does not silently discard it.

**Code says** there is no quit path at all:

- `grep -rn 'close-requested\|CloseRequested\|onCloseRequested\|getCurrentWindow\|getCurrentWebviewWindow' src/ --include='*.ts' --include='*.tsx' | wc -l` → **0**
- `grep -rn 'on_window_event\|CloseRequested\|prevent_close\|ExitRequested\|prevent_exit' src-tauri/src/ --include='*.rs' | wc -l` → **0**

The component exists and says so itself, at `src/components/dialogs/QuitGuardDialog.tsx:10-12`:
> `* The window-close trigger (and the actual quit once operations are cancelled) is`
> `* host wiring outside this unit; this dialog owns the presentation, the per-op`
> `* cancellation, and dismissal.`

That host wiring was never written. The only non-type reference that opens the dialog
is the **app-update** path — `src/components/shell/UpdateStatusItem.tsx:36`:
> `openDialog({ kind: "quitGuard", opIds: running, reason: "update" });`

`grep -rn 'kind: "quitGuard"' src/ --include='*.tsx' --include='*.ts' | grep -v '\.test\.'`
returns exactly that line plus the type declaration at `src/store/ui.ts:26`.

Two further confirmations that the quit branch is dead code:

1. `QuitGuardDialog.tsx:39-48` — `cancelAll()` cancels the ops and closes the dialog;
   the quit itself only happens on the `updating` branch (`if (updating) { … void installAppUpdate() … }`).
   With `reason: "quit"` the button labelled `"Cancel operations and quit"`
   (`QuitGuardDialog.tsx:76`) never quits.
2. `src-tauri/src/lib.rs:291-295` handles `tauri::RunEvent::Exit` by calling
   `state.shutdown()` — the comment at `lib.rs:288-290` says
   `// Quit-guard kill hook: on exit, cancel every running op so child` /
   `// process groups are SIGTERMed and never outlive the app. The` /
   `// confirm dialog lives in the frontend (QuitGuardDialog, U8).`
   There is no `ExitRequested` arm and no `api.prevent_exit()`, so ⌘Q kills every child
   with no dialog and no choice.

**Why this matters.** FR-14 is the requirement that separates "honest terminal states"
from "an indefinite spinner". A downstream `bmad-create-epics-and-stories` run reads
FR-14's Partial tag, sees the quit guard is not in the Planned list, and schedules zero
work. The product ships claiming an explicit quit choice that does not exist, and the
existing `QuitGuardDialog` stays reachable only from the update button — so the bug is
invisible to anyone testing the update flow.

**Compounding**: prd.md:629 (§9 Q1) reinforces the false reading —
> **What happens on quit with work *queued* but not running?** The running-Operation quit guard is defined. Queued-only work, application-update installation during Package activity, and OS-initiated shutdown are not.

"defined" is true of `docs/SPEC.md`; it is not true of the build. The PRD's entire
status-tag apparatus exists to keep those two apart, and here it collapsed them. The
open question also scopes the gap to *queued-only* work, which frames an implementation
hole as a specification hole.

**Fix.** Move "Quitting with work in flight presents an explicit choice and does not
silently discard it" into FR-14's Planned/unbuilt set, naming the missing wiring
explicitly: a `tauri://close-requested` (or Rust `RunEvent::ExitRequested` +
`prevent_exit`) handler that opens `QuitGuardDialog` with `reason: "quit"`, and a quit
call on the `cancelAll` quit branch. Add it to §7.2 as build-queue work. Rewrite §9 Q1
so it asks only what it actually asks — the *policy* for queued-only work and
OS-initiated shutdown — and drop the "is defined" clause. Add the quit guard to the
§7.1/§7.2 boundary correctly: it is not in §7.1 today, and that is right; the defect is
FR-14's consequence list.

---

### F2 — HIGH: FR-11 mis-describes the current Manager self-update path, so the D27 work as scoped leaves three unstaged mutation paths alive.

**PRD says** (prd.md:302):
> - **Planned — D27:** the Manager update action adds *independent, individually removable* plan membership. Today it is a single global all-or-nothing `include_self_updates` toggle — do not re-entrench a global toggle when implementing this.

**Code says** the global toggle is a *different, additional* path. The Manager update
action itself is an immediate IPC call that never touches the Upgrade Plan:

- `src/components/manager/SelfUpdateCard.tsx:116`
  > `onClick={() => void selfUpdateManager(managerId)}`
- `src/components/dashboard/ManagerCard.tsx:128` (the card's ⋯ menu → "Self-update")
  > `void selfUpdateManager(info.id);`
- Backed by a registered command: `src-tauri/src/lib.rs:239` `commands::self_update_manager,`

The global toggle the PRD names is a separate control on the plan sheet —
`src/components/dialogs/UpgradePlanSheet.tsx:218-225`, `checked={includeSelfUpdates}` /
`aria-label="Include manager self-updates"`. Both exist today.

There is a **fourth** immediate-mutation path the PRD never mentions:
`src/components/manager/HealthBanner.tsx:43`
> `onClick={() => void runHealthFix(managerId, issue.id)}`
(command registered at `lib.rs:241` `commands::run_health_fix,`).

The complete current inventory, from
`grep -rn 'executePlan\|selfUpdateManager\|runHealthFix' src/components src/store --include='*.tsx' --include='*.ts' | grep -v '\.test\.' | grep -v import`:

| Site | Call | Reviewed first? | Named in the PRD? |
| --- | --- | --- | --- |
| `ManagerPane.tsx:152` | `await executePlan(plan)` (from `upgradeRow`) | no | **yes** — prd.md:230 |
| `SelfUpdateCard.tsx:116` | `void selfUpdateManager(managerId)` | no | no |
| `ManagerCard.tsx:128` | `void selfUpdateManager(info.id)` | no | no |
| `HealthBanner.tsx:43` | `void runHealthFix(managerId, issue.id)` | no | no |
| `UpgradePlanSheet.tsx:154` | `await executePlan(plan)` | yes (sheet) | n/a |
| `Sidebar.tsx:89` | `updateEverything()` → sheet | yes (sheet) | n/a |

**Why this matters.** prd.md:212 states the group's central promise:
> **Description:** This is where the product's central promise lives: *nothing runs that was not staged and shown.* Every path to a mutation — a Package row, a Manager header, a Manager-wide action, `Update Everything` — converges on one reviewable Upgrade Plan …

and prd.md:612 makes a violation a P0:
> - **SM-2: Zero unreviewed mutations.** No Package or Manager update ever runs that the user did not see staged first. A single violation is a P0 defect, not a metric miss.

An implementer working from FR-6 + FR-11 as written removes `upgradeRow` and replaces a
checkbox. The three `selfUpdateManager` / `runHealthFix` buttons survive, and the
product ships with the central promise broken in three places, none of which any story
covers. This is precisely the failure mode the PRD's Current-vs-Target split exists to
prevent — it just got the "Current" wrong.

**Fix.** Rewrite FR-11's Planned note to state the actual current behavior: "Today the
Manager update action calls `self_update_manager` directly from `SelfUpdateCard` and
from `ManagerCard`'s overflow menu, executing with no plan review at all; a separate
global `include_self_updates` toggle on the plan sheet is the only staged path. D27
removes both." Extend FR-6's out-of-scope block, or add a consequence to FR-10, that
names all four immediate-execution call sites by path so no story misses one. If health
fixes are meant to stay outside the plan (a defensible call — a HealthFix is not an
Upgrade), say so explicitly under FR-16 or a new FR, because §4.2's "every path to a
mutation" currently sweeps them in and the glossary at prd.md:107 lists `HealthFix` as
an Operation.

---

### F3 — HIGH: §7.3 schedules Health fixes as deferred work. They ship in full, including the exact constraint §7.3 describes.

**PRD says** (prd.md:599):
> - **Health fixes** (uv broken tool environments, with only an exactly-recognized reinstall suggestion becoming runnable), **snapshot cache**, **native notifications when backgrounded**, **Package detail popover**, and the **"also managed by rustup"** note on mise's Rust row. `[NOTE FOR PM]` — these were P1 in `docs/SPEC.md`, but D33's surviving habit applies: verify whether each already ships before scheduling it as new work.

The `[NOTE FOR PM]` asks for a verification that was never done. Doing it:

| §7.3 item | Verdict | Evidence |
| --- | --- | --- |
| **Health fixes** | **SHIPS** | `lib.rs:241` `commands::run_health_fix,`; `HealthBanner.tsx:43` `onClick={() => void runHealthFix(managerId, issue.id)}`; `ipc.rs:340` `pub struct HealthIssue`, `:350-351` `pub fix_args: Option<Vec<String>>,` / `pub fixable: bool,`; the narrow-recognition rule at `managers/parse/uv.rs:82-83` `let fixable = SAFE_TOOL_NAME_RE.is_match(&name)` / `&& suggested_fix.as_deref() == Some(canonical_fix.as_str());` with `uv.rs:90-92` `// The full warning remains in \`detail\`, but an altered` / `// suggestion is neither runnable nor presented as a copyable` / `// fix command.` |
| snapshot cache | deferred, correctly | no snapshot cache found; `queue.rs:397-404` "cache" refers to the 64-entry issued-plan store |
| native notifications when backgrounded | deferred, correctly | `grep -rn 'notification\|Notification' src-tauri/src/ src/ src-tauri/capabilities/ src-tauri/tauri.conf.json` → 0 |
| Package detail popover | deferred, correctly | `grep -rn 'popover\|Popover' src/` → 1 hit, a CSS comment at `theme.css:12`. `PackageRow` has row expansion for `pkg.meta?.executables` (`PackageRow.tsx:53-54`), which is not the popover |
| "also managed by rustup" note on mise's Rust row | deferred, correctly | `grep -rn 'rustup' src/components/` → `ManagedByChip.tsx:12` and `UpgradePlanSheet.tsx:30` `rustDedup: "handled by rustup",` — that is the *plan* exclusion note (FR-5's D10 rule), not a note on the mise Rust row |

So **1 of 5 §7.3 items already ships**, and it is the largest of them.

This also makes the PRD internally inconsistent, in four places that all treat HealthFix
as existing:

- prd.md:107 (Glossary) — `**Operation** — one queued unit of work: Refresh, Upgrade, SelfUpdate, or HealthFix.`
- prd.md:118 (Glossary) — `**Health issue** — a Manager-reported warning about a broken Package or tool environment. Only a narrowly recognized fix may become runnable.` (this is `uv.rs:82-83`, described in the present tense)
- prd.md:173 (FR-3, Shipping) — `- A successful Upgrade, SelfUpdate, or HealthFix refreshes every affected subject and executor.`
- prd.md:261 (FR-8, Shipping) — `… a lock-set overlap with any pending or running Upgrade, SelfUpdate, or HealthFix rejects the submission without enqueueing.`

**Why this matters.** This is the exact error the task brief calls "D33's surviving
habit" and that the PRD itself warns about one sentence later. A sprint-planning run
that reads §7.3 schedules a feature that is already built, tested, and wired end to
end — burning a story on rediscovery, and risking a reimplementation that loses the
`fixable` guard at `uv.rs:82-83` (the one thing standing between a parsed Manager string
and a spawned command).

**Fix.** Move health fixes out of §7.3 into §7.1's Shipping list — §7.1 (prd.md:583)
currently omits them entirely, so both scope sections are wrong in the same direction.
Drop the `[NOTE FOR PM]` and replace it with the four resolved verdicts above so nobody
re-runs the check. Then decide FR-2's open question from F2: health fixes ship *without*
plan review, so either §4.2's "every path to a mutation" needs a carve-out or the
HealthBanner button needs staging.

---

### F4 — MEDIUM: FR-5 is tagged Shipping, but "Packages with updates sort first by default" is not implemented anywhere.

**PRD says** (prd.md:202, under FR-5 **Status:** Shipping):
> - Packages with updates sort first by default, with a filter that shows all.

**Code says** nothing sorts packages, in either process.

- `grep -rn 'sort' src-tauri/src/ --include='*.rs'` → 9 lines total, none of them package
  ordering: `events.rs:407` `batches.sort();` (a test), `diagnostics.rs:67`
  `files.sort_by(|a, b| b.0.cmp(&a.0));` (log file selection), `logging.rs:220`
  `kept.sort_by(|a, b| b.0.cmp(&a.0));` (retention), `commands.rs:47`
  `records.sort_by(|a, b| b.queued_at.cmp(&a.queued_at));` (operation records).
- `grep -rn 'sort' src/components/manager/*.tsx src/store/packages.ts` → 2 hits, both
  inside `planSheet.test.tsx`.
- `PackageTable` renders `rows` in array order: `PackageTable.tsx:126`
  `<div>{rows.map(renderRow)}</div>`, and `ManagerPane.tsx:103-105` builds `visibleMain`
  by `.filter(...)` only — no reordering.

What actually ships is the *filter* half, defaulted on: `src/store/packages.ts:56-59`
> `// Default the outdated-only toggle ON when anything is outdated.`
> `const outdatedOnly = { ...s.outdatedOnly };`
> `if (outdatedOnly[id] === undefined) {`
> `  outdatedOnly[id] = snapshot.packages.some((p) => p.outdated);`

with the "show all" escape at `ManagerPane.tsx:252-257` (`Show all packages`).

The sort requirement traces to `EXPERIENCE.md:411`:
> `2. The Manager workspace opens with All Packages selected and actionable updates sorted first.`

so it is a real, sourced requirement — it just isn't built.

**Why this matters.** FR-5 Shipping means no story. The requirement then survives in the
PRD as a false statement of current behavior, and the next reader who checks it against
the app finds the PRD wrong about the one thing it is supposed to be authoritative on.
Materially smaller than F1–F3 (one consequence out of seven, and the default filter
delivers most of the user-visible benefit), but it is a Shipping tag over absent code.

**Fix.** Split the consequence: keep "a filter that shows all, defaulted to outdated-only
when anything is outdated" as Shipping, and move "Packages with updates sort first" to a
**Planned** limb on FR-5 with a status line naming it, the way FR-7 and FR-11 already do
for their split limbs. Alternatively drop it if the default filter is deemed to
supersede it — but then `EXPERIENCE.md:411` needs the same reconciliation, and it should
join addendum.md §3's queue.

---

### F5 — LOW: FR-5's "plain-language reason on pointer interaction" has a hole for outdated self-updating casks.

**PRD says** (prd.md:200, FR-5 Shipping):
> - Up-to-date and otherwise ineligible Packages cannot enter the Upgrade Plan and expose a plain-language reason on pointer interaction. Ineligibility never relies on gray styling alone.

**Code says** the per-row reason covers three of four ineligibility causes.
`src/components/manager/PackageRow.tsx:69-75`:
> `const checkboxTitle = pkg.pinned`
> `  ? \`Pinned in Homebrew — run \\\`brew unpin ${pkg.name}\\\` to upgrade\``
> `  : !pkg.outdated`
> `    ? "Already up to date"`
> `    : upgrading`
> `      ? "Upgrade in progress"`
> `      : undefined;`

Ineligibility is defined at `src/store/packages.ts:125-126`:
> `export function isSelectable(pkg: Package): boolean {`
> `  return pkg.outdated && !pkg.pinned && pkg.kind !== "caskGreedy";`

An **outdated** `caskGreedy` therefore falls through every branch to `undefined`: its
checkbox is disabled (`PackageRow.tsx:68` `const checkboxDisabled = !selectable || upgrading;`)
with no per-row reason.

Partially mitigated: greedy casks render in their own collapsible group with a
group-level explanation at `PackageTable.tsx:145-147`:
> `<div className="px-3 pb-2 text-[12px] text-text-muted">`
> `  These casks update themselves and are excluded from Upgrade All unless opted in.`

So the information exists on screen; it is just not on the control the FR points at.

**Fix.** Either add a `caskGreedy` branch to `checkboxTitle`, or amend FR-5 to say the
self-updating-cask reason is carried by the group header rather than the row — the
grouping is already a separate consequence one line up (prd.md:199). One sentence
either way; recorded so the FR and the code agree on which mechanism is the contract.

---

## Part 3 — Notes (not findings)

**A. `MAX_PLAN_SELECTIONS` is unstated.** FR-8 (prd.md:259) states the capability bound
— "at most 64 unconsumed capabilities are retained per session" — but the PRD never
states the plan-size bound, `queue.rs:400` `pub const MAX_PLAN_SELECTIONS: usize = 2_048;`,
enforced at `queue.rs:410-413`. Not a defect: an FR-6 batch membership operation over
100 rows is nowhere near 2,048, and FR-6's batch requirement (prd.md:231) is about
round-trips, not size. Recorded because the D27 draft becomes persistent and long-lived,
and 2,048 is the ceiling any implementer of the batch mutation will hit first.

**B. `refuse_app_update_while_busy` and `activeOps` are declared coupled.**
`commands.rs:769` `/// The status set matches \`activeOps\` in \`src/store/operations.ts\` exactly.`
FR-21's "enforced independently in two layers, and both must stay" (prd.md:472) is
correct, but the two layers share a status definition by convention with no test binding
them. Architecture's concern, not the PRD's — noted for `bmad-architecture`.

**C. Automatic update checks are compiled out of debug builds.** `lib.rs:142-145`
> `if cfg!(debug_assertions) {`
> `    tracing::debug!("debug build: automatic update checks disabled");`
> `    return;`
> `}`
FR-20 and RP-1 describe release behavior, which is correct. Recorded only so nobody
"reproduces" a missing launch check in `npm run tauri dev` and files it as a defect.

**D. §0.1's SPEC-vs-code table is accurate.** Every row re-verified:
17→20 commands (`lib.rs:232-253`), 5→6 events (`events.rs:77-82`), 7→8 settings fields
(`settings.rs:28-39`), `skipUpgradePlanConfirmation` absent (0 occurrences),
notarization live (`release.yml:347` `- name: Verify signature & notarization`). No
correction needed.

**E. FR-19's "31 sites" is exact, not approximate.** `grep -ro 'focus-visible:outline-2'`
over `src/` returns 31 across 20 files. The only `ring-*` remaining is
`PackageRow.tsx:85` `highlighted ? "ring-2 ring-inset ring-accent" : ""` — a cross-Manager
*join-target highlight*, not focus, so D35's rule is intact.

---

## Summary

The PRD's structural claims are unusually well grounded: every count, constant,
threshold, default and CI gate it asserts checks out against HEAD, and the six "do not
go looking for them" Planned tokens really do return zero. The status-tag apparatus
works — where it fails, it fails on the *Current* side, not the Target side.

Three failures matter. FR-14 asserts a quit guard that has no host wiring at all
(critical: it would ship as a false Shipping claim with zero scheduled work). FR-11
mis-states what the Manager update action does today, and in doing so leaves three
unstaged mutation call sites outside the D27 removal scope, against the PRD's own P0
metric SM-2 (high). §7.3 schedules health fixes as new work when they ship complete,
including the narrow-fix guard §7.3 itself describes — the D33 habit the PRD warns about
one sentence later, in a `[NOTE FOR PM]` that was never resolved (high). FR-5's "sort
first by default" is a Shipping tag over code that does not exist (medium), and FR-5's
ineligibility-reason mechanism has one uncovered case (low).
