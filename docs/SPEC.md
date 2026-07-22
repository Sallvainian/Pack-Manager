# Pack-Manager — SPEC.md (authoritative)

Status: FINAL. Synthesized from three design candidates and three judge reports. Base design: "quality" (consensus winner), with grafts from "systems" (lock-set scheduler, in-band routing override, crash-safe journal, cross-manager self-version join) and "ux" (version-delta signature, upgrade plan sheet, routing chips, phase labels, keyboard map). All fixture claims below were verified against the files in `/Users/sallvain/Projects/Pack-Manager/dev/fixtures/` on 2026-07-22.

---

## 0. Product statement

Pack-Manager is a macOS desktop app (Tauri 2 + Rust backend, React 19 + TypeScript + Vite 7 + Tailwind v4 frontend, dark mode) that auto-detects the package managers installed on the machine, lists every package each one manages with installed and latest versions, upgrades everything or a hand-picked selection, and updates the managers themselves via dynamically-derived routing (uv→mise, mise→brew, rustup→self, npm→self, brew→`brew update`). Every operation streams live output, is cancellable, and leaves a byte-faithful transcript on disk.

### Load-bearing invariants (violations are bugs)
1. **The manager's outdated verdict is authoritative.** Pack-Manager never computes version comparisons to decide outdatedness. Real data includes `2.0.14-1`, `1.6.2.dev0`, `stable`, and rustup commit hashes. The frontend's version-delta highlight and severity chips are display affordances computed by pure string segment-diff, never a source of truth.
2. **Who-manages-whom is derived from paths at detection time, never hardcoded.** Classification inspects the RAW resolved path against mise directories BEFORE canonicalizing — mise shims ARE symlinks to the mise binary (`~/.local/share/mise/shims/uv` canonicalizes to the brew-installed mise), so canonicalize-first would misroute uv/npm to brew.
3. **Homebrew is never contended by us.** Every operation declares a lock set; all brew-binary operations hold the Brew lock and therefore serialize. Different managers run in parallel.
4. **No sudo, no password entry, ever.** Child stdin is `/dev/null`. Silent processes trigger stall detection with a copy-the-command-to-terminal handoff.
5. **One failing manager never blanks the others.** Per-manager refresh isolation: independent operations, timeouts, error cards; prior snapshots are retained on failure.
6. **Nothing runs that was not shown.** Bulk upgrades go through a plan sheet that previews the exact commands (explicit package names, never bare `brew upgrade`).
7. **Every operation is reconstructible from disk.** Transcript with argv/PATH/env header + structured app log + journal, all correlated by opId.

---

## 1. Features

Priorities: **P0** = MVP-required. **P1** = ship-with polish, after all P0 tests pass. **P2** = out of MVP scope.

### F1 (P0) Manager auto-detection
Detect six adapters at launch and on demand: `brew`, `mise`, `npm`, `uv`, `rustup`, `mas`. Per manager: resolve the binary on the constructed search path (§5.2), probe `--version` (10s timeout), classify managed-by from paths (§5.3) with a human-readable `evidence` string. Absent managers (mas here — fixture `mas_outdated.txt` is literally `zsh: command not found: mas`) render as muted "Not installed" cards with copyable install hint (`brew install mas`); absence is a normal state, never an error. `detect_managers` also serves as Re-detect (Settings + dashboard overflow).
**Acceptance:** on this machine detection yields brew/mise/npm/uv/rustup present, mas absent; managedBy: brew→standalone, mise→brew, npm→mise, uv→mise, rustup→standalone; each with evidence string; results in Settings' Environment Report.

### F2 (P0) Refresh: inventory + outdated overlay, parallel, isolated
Per manager, refresh runs an inventory command then outdated command(s) (§5.4), merges into a `ManagerSnapshot` (inventory rows get `latest = installed`, `outdated = false`; the outdated overlay patches `latest` and flips `outdated`; overlay-only rows are appended). brew refresh starts with `brew update` (also brew's self-update; phase-labeled in UI). Refresh All fans out one operation per present manager; each has its own loading skeleton, absolute timeouts, and error isolation. Successful upgrades auto-enqueue a refresh of the affected manager(s).
**Acceptance:** with a fake brew failure, brew shows an error card with stderr tail + Retry while other tables are fully populated and the previous brew snapshot stays browsable, marked stale. Offline: each network-dependent manager degrades to its own Timeout/error card; nothing blanks.

### F3 (P0) Package table: name, installed, latest, status
Columns: checkbox, name, installed, latest (version-delta treatment §4.6), status badge, row Upgrade. uv rows expand to show executables. Pinned brew formulae (JSON `pinned` field) show `Pinned`, checkbox disabled with tooltip, excluded from every plan. Greedy-only casks (two-call diff, §5.4) sit in a collapsed "Self-updating casks" section, excluded from select-all and Upgrade All unless opted in. Versions render verbatim.
**Acceptance:** npm fixture data renders 4 package rows (self `npm` row hoisted to the SelfUpdateCard); mise fixture renders 6 outdated rows (`rust stable stable stable` is not outdated); pinned/greedy exclusions hold under select-all.

### F4 (P0) Upgrade All via plan sheet
Dashboard "Update Everything" and per-manager "Upgrade all (k)" open the Upgrade Plan Sheet: per-manager sections showing the EXACT commands (`brew upgrade dolt`, `mise upgrade deno ruby fnox ruff npm:prettier uv`, `npm install -g typescript@latest …`), toggles `Include manager self-updates (n)` (default on) and `Include self-updating casks (n)` (default off), an `excluded` list with reasons, and warnings (e.g. stale check). Each returned `planId` is a bounded, one-use backend capability. Before issuance, an explicit selection is bounded to 2,048 entries and 512 bytes per package ID, then exact `(managerId, packageId)` duplicates are removed first-seen-order; `selection: null` remains the distinct “all outdated” intent. Confirm calls `execute_plan`, which authenticates the exact round-tripped plan, rebuilds it from current state, and enqueues only if both plans match; a stale plan is replaced in the sheet and requires another confirmation. Dismissing the sheet while confirmation is pending invalidates every late continuation. Managers run in parallel, serialized per lock set.
**Acceptance:** the sheet's command strings are byte-equal to what the backend spawns (asserted via plan-builder tests + transcript header); rust-dedup rule (§5.7) applies; nothing runs that the sheet did not display.

### F5 (P0) Multi-select upgrade
Checkbox per selectable row (outdated only; pinned and up-to-date disabled with tooltips), header tri-state over visible selectable rows (filter-aware, never greedy casks), shift-click range, Cmd-click toggle, Cmd+A select all visible. SelectionToolbar slides up: "N selected · Upgrade selected · Clear (Esc)". "Upgrade selected" opens the plan sheet seeded with exactly the checked ids; selection clears after enqueue. Row-level Upgrade button executes a single-package plan immediately (no sheet; command visible in drawer + transcript).
**Acceptance:** frontend test asserts `build_upgrade_plan` is called with exactly the selected `PackageRef`s and `execute_plan` with the returned plan.

### F6 (P0) Manager self-update with dynamic routing
Each ManagerPane has a SelfUpdateCard: manager's installed version, latest when known (cross-manager join §5.8), route in plain language, Update button. Routes on this machine (derived, not hardcoded): brew→ViaRefresh (`brew update`); mise→Routed(brew, `brew upgrade mise`); npm→InBand (`npm install -g npm@latest`) via the in-band override (npm reports itself in its own outdated list — fixture row `npm 11.16.0 12.0.1 12.0.1`); uv→Routed(mise, `mise upgrade uv`); rustup→InBand (`rustup self update`). Routed copy pattern: "uv is managed by mise — this runs `mise upgrade uv` on the mise queue." Executor absent → button disabled with reason. npm's card carries a persistent note: "npm and all global packages live inside the mise-managed node — upgrading node via mise resets them." Routed self-updates hold BOTH manager locks (§5.7). The manager's own row is removed from its package table and lives only on the card.
**Acceptance:** route precedence tests (§7.2); frontend test asserts the routed subtitle and that clicking invokes `self_update_manager`; queued-behind-executor state renders ("Queued behind Homebrew").

### F7 (P0) Live streaming + cancellation + stall detection
Every operation streams stdout/stderr line-batched (≤50ms/64 lines, 8KiB force-flush) to the ActivityDrawer's virtualized LiveLogView. Cancel: SIGTERM to process group → 5s grace → SIGKILL; no confirmation dialog; final transcript line records the signal. Stall: no output for `stallAfterSecs` (default 120, configurable) → `op:stalled` → amber pill + dialog: "No output for 2m. This command may be waiting for input Pack-Manager cannot provide — it never enters passwords." Buttons: Keep waiting (re-arms), Copy command, Cancel operation. Hard cap 30min → TimedOut + group kill.
**Acceptance:** scheduler tests with paused time (§7.3); frontend stall-dialog and cancel tests.

### F8 (P0) Operation history + persistent transcripts + journal
Every operation writes a transcript (§6) and start/finish records to the crash-safe journal `operations.jsonl` (flushed immediately). HistoryView lists session + journal records (filter by manager/status/search); interrupted ops (start-without-finish) surface as `Interrupted` on next launch — recorded pgids are NEVER signaled on startup (pid reuse). Row detail: full command, transcript tail, Reveal in Finder.
**Acceptance:** journal round-trip tests; simulated start-only record renders Interrupted.

### F9 (P0) Diagnostics export
`export_diagnostics` builds `~/Desktop/Pack-Manager-diagnostics-<YYYYMMDD-HHmmss>.zip`: `report.json` (app/OS version, arch, resolved search path + source, full DetectionReport with evidence, settings, log filter), last 3 app-log files, last 25 transcripts, `operations.jsonl`. Only env vars we set are ever logged — never the inherited environment.

### F10 (P0) Dark-mode UI, app icon, build
Dark theme is the only MVP theme (tokens §4.1, structured for a future light theme). Icon: programmatic 1024px PNG via `dev/icon/generate_icon.py` (Pillow) piped into `npx tauri icon`; generated set committed under `src-tauri/icons/`. `npm run tauri build` produces an ad-hoc-signed `.app` — that is the shipping deliverable; notarized DMG is explicitly out of scope (macOS 27 beta + Xcode-beta).

### F11 (P0) Settings
Persisted at `~/Library/Application Support/Pack-Manager/settings.json`: `runBrewUpdateOnRefresh` (default true), `autoRefreshOnLaunch` (true), `stallAfterSecs` (120), `upgradeHardCapMins` (30), `logLevel` ('debug' for own crate), `autoOpenDrawer` (true), `includeGreedyByDefault` (false). A patch is persisted before in-memory settings and the canonical-state revision advance; failed persistence leaves both unchanged. Plus read-only Environment Report (search path + source, per-tool path/version/managedBy/evidence, Copy button) and Open Logs Folder / Export diagnostics / Re-detect.

### F12 (P0) Deterministic test suite
`cargo test` and `npm test` pass offline on a clean checkout. Full plan in §7.

### P1 (after all P0 green)
- **F13 Health fixes:** uv broken tool environments (fixture warning line) render as HealthBanner. Only an exact recognized `uv tool install <name> --reinstall` suggestion receives a copyable fix command and backend-only argv; altered, missing, or malformed suggestions remain warning detail with no copy/run affordance. The "Run fix" button enqueues the structured argv on the uv lane (`run_health_fix`).
- **F14 Snapshot cache:** persist last snapshots; render instantly on launch with "stale — refreshing…".
- **F15 Toasts→native notifications** when backgrounded.
- **F16 Package detail popover** (uv executables, mise source path, npm dependedBy, brew pinned version).
- **F17 "Also managed by rustup"** informational note on mise's `rust` row.

### P2 (out of scope)
Light theme; menu-bar extra; scheduled refresh; cross-manager dedup beyond the rust rule; cargo-install support; notarized DMG.

### Explicit non-features
No sudo/password entry. No auto-upgrades without user action. No package install/uninstall (updates only). No telemetry. No local version-comparison authority.

---

## 2. Target-machine facts designed against
macOS 27.0 beta, arm64 M4, 16GB. Xcode-beta toolchain. brew at `/opt/homebrew/bin/brew`; mise installed via brew; node/npm via mise; uv via mise; rustup standalone in `~/.cargo/bin`; mas absent. Verified command quirks: brew prints junk to stdout before JSON; `npm outdated -g --json` exits 1 when outdated; `mise outdated --json` returns `{}` when clean; `uv tool list` can emit `warning:` lines; `rustup check` has inconsistent colon spacing; both captured brew JSON fixtures have `"casks": []` (cask JSON shape is UNVERIFIED — see §5.5 brew).

---

## 3. Definitions
- **Manager**: one of `brew | mise | npm | uv | rustup | mas` (`ManagerId`).
- **Operation (op)**: one queued unit of work (Refresh, Upgrade{packageIds}, SelfUpdate{subject}, HealthFix{issueId}) with an `executor` (whose binary runs), a `subject` (whose data it affects), a lock set, and 1..n serial `CommandSpec`s. Ids are UUIDv7 (time-sortable).
- **Snapshot**: the merged inventory+outdated view of one manager.
- **Lane/lock**: one lock per ManagerId; an op runs only when its whole lock set is free.

---

## 4. UI specification

### 4.1 Design tokens (Tailwind v4 `@theme` in `src/styles/theme.css`)
```css
@theme {
  /* Surfaces — dark blue-graphite ramp */
  --color-bg-base:       #0B0E14;   /* window background */
  --color-bg-surface:    #11151D;   /* sidebar, cards, table header */
  --color-bg-raised:     #171C26;   /* hover rows, drawer, inputs */
  --color-bg-overlay:    #1D2330;   /* dialogs, sheets, popovers */
  --color-bg-inset:      #07090D;   /* log viewer background */
  --color-border:        #232A36;
  --color-border-strong: #303948;

  /* Text */
  --color-text-primary:   #E6E9EF;
  --color-text-secondary: #9AA3B2;
  --color-text-muted:     #5C6675;

  /* Accent + status */
  --color-accent:        #4F8CFF;   /* primary actions, focus, running */
  --color-accent-hover:  #6BA0FF;
  --color-accent-subtle: #4F8CFF1F; /* 12% — selected-row wash */
  --color-success:       #3FB96B;
  --color-warning:       #E5A53A;
  --color-danger:        #E5564F;
  --color-info:          #38BDF8;
  /* Badges/banners use the status color at 12% alpha backgrounds. */

  /* Version-delta severity (DISPLAY ONLY — never decides outdatedness) */
  --color-sev-major: #E5564F;
  --color-sev-minor: #E5A53A;
  --color-sev-patch: #3FB96B;

  /* Shape */
  --radius-card: 10px; --radius-control: 6px;

  /* Fonts */
  --font-sans: -apple-system, BlinkMacSystemFont, "SF Pro Text", system-ui, sans-serif;
  --font-mono: ui-monospace, "SF Mono", Menlo, monospace;
}
```
Dark-only in MVP; tokens live in one file so a light theme is a value swap. `prefers-reduced-motion` disables all transitions (default 150ms ease). Focus: 2px `--color-accent` ring, offset against surface, on every interactive element.

### 4.2 Typography
| Role | Spec | Usage |
|---|---|---|
| Page title | 20px/600 sans | Pane headers |
| Section head | 15px/600 sans | Card titles, dialog titles |
| Body | 13px/400 sans | Default text, table cells |
| Mono data | 12px/400 mono, `tabular-nums` | versions, deltas, commands, paths, logs |
| Meta | 11px/500 sans uppercase tracking-wide, secondary | labels, chips, timestamps |
Line height 1.45; log lines 1.6. Spacing: 4px base (4/8/12/16/24/32); cards `p-4`; table cell `px-3 py-2`.

### 4.3 Window & layout
Window: 1200×800 default (existing scaffold), min 900×600, `titleBarStyle: "Overlay"`, `hiddenTitle: true`; sidebar top has 38px padding + `data-tauri-drag-region` to clear traffic lights.
```
┌───────────────┬──────────────────────────────────────────┐
│ Sidebar 240px │ MainView (Dashboard | ManagerPane |       │
│               │           History | Settings)             │
│               ├──────────────────────────────────────────┤
│               │ ActivityDrawer (collapsed 32px / 40%)     │
├───────────────┴──────────────────────────────────────────┤
│ StatusBar 28px                                            │
└──────────────────────────────────────────────────────────┘
```

### 4.4 Component tree (exact file names)
```
App
└─ AppLayout
   ├─ Sidebar ── SidebarManagerItem×n
   ├─ MainView (store-routed, no router lib)
   │   ├─ DashboardView ── ManagerCard×n ── ManagedByChip
   │   ├─ ManagerPane ── SelfUpdateCard · HealthBanner · PackageToolbar
   │   │                 · PackageTable ── PackageRow ── VersionDelta · StatusBadge
   │   │                 · SelectionToolbar
   │   ├─ HistoryView
   │   └─ SettingsView
   ├─ ActivityDrawer ── OperationList ── OperationRow · LiveLogView
   ├─ StatusBar
   ├─ DialogHost ── UpgradePlanSheet · StalledOperationDialog · QuitGuardDialog
   └─ ToastHost
primitives/: Button, Checkbox, Chip, Tooltip, Spinner, EmptyState, ErrorState,
             CopyableCommand, SkeletonRows
```

### 4.5 Sidebar
Header: app glyph + "Pack-Manager" (15/600). Two full-width buttons: `Refresh All` (secondary), `Update Everything` (primary accent, shows total outdated count, disabled at 0 with tooltip). Items: **Dashboard**, divider, one `SidebarManagerItem` per present manager (order: Homebrew, mise, npm, uv, rustup, mas) — 20px monochrome glyph, name, right-aligned amber count pill (hidden at 0), 6px status dot (gray idle / accent pulsing while refreshing or busy / red on error / amber when health issues), then a collapsed **Not installed** disclosure (mas, 60% opacity, install-hint tooltip), divider, **History**, **Settings** pinned bottom. Selected item: `bg-raised` + 2px accent left rail. While detection is in flight: 5 shimmer rows. Cmd+1..9 jumps items.

### 4.6 VersionDelta (the visual signature)
`VersionDelta` renders `installed → latest` in mono. Pure frontend helper `lib/versionDelta.ts`: split both on `.`/`-`; find first differing segment; render the common prefix in `--color-text-secondary` and the changed suffix of `latest` in the severity color (segment 0 → `--color-sev-major`, 1 → minor, ≥2 → patch). Non-comparable pairs (`stable`, hashes, missing latest) render plain with no highlight and the row shows "update available" instead of a fabricated delta (mandated for uv until its outdated format is captured). A small severity chip (`major/minor/patch`) accompanies comparable deltas; chips carry text labels, not only color. Display-only: `outdated` always comes from the manager.

### 4.7 DashboardView
Header: "Packages", right: `Refresh All`, `Update Everything`. Grid of `ManagerCard`s (`repeat(auto-fill, minmax(320px,1fr))`, gap 16). Card: glyph, name, `ManagedByChip` (tooltip = evidence string, e.g. "Resolved at ~/.local/share/mise/shims/uv → managed by mise"; clicking a `via brew` chip navigates to Homebrew and highlights the `mise` row), version line (mono, `4.5.2 · /opt/homebrew/bin/brew`, middle-truncated), big outdated numeral (28/700, amber >0, green check + "Up to date" at 0), self-version delta when known (join §5.8), footer: `Refresh` (ghost), `Upgrade all` (secondary, hidden at 0), overflow `⋯`: Self-update (route subtitle), View packages, Reveal last log, Re-detect. States: loading skeleton; error (red left border, one-line summary + ≤2 stderr lines mono + Retry + View log); absent (60% opacity + `CopyableCommand brew install mas`); busy (thin indeterminate accent bar on top, conflicting buttons disabled with tooltip). During brew refresh the card cycles phase labels: "Updating Homebrew metadata…" → "Listing installed…" → "Checking outdated…".

### 4.8 ManagerPane
Row 1: name (20/600), ManagedByChip, version (mono), "Refreshed 2m ago" (live meta), spacer, `Refresh`. Row 2: `SelfUpdateCard` — "Manager" label, installed → latest (VersionDelta when latest known), route copy (exact strings per §F6), `Update <name>` button (accent when update known; "Updating…" spinner while running; "Queued behind Homebrew" chip when the executor lane is busy); npm's mise-reset note lives here permanently. Row 3 (conditional): `HealthBanner` — warning-tinted, per issue: "Tool `aider-chat` environment is broken." + `CopyableCommand uv tool install aider-chat --reinstall` + `Run fix` (P1). Row 4: `PackageToolbar` — SearchInput (240px, filters name + executables, 200ms debounce), `OutdatedOnlyToggle` (default ON when any outdated), right meta "14 installed · 7 outdated", `Upgrade selected (n)` (hidden at 0), `Upgrade all (k)`.
`PackageTable`: virtualized (@tanstack/react-virtual) beyond 100 rows. Columns: ☐ 36px | Name flex (kind sub-label 11px muted for brew formula/cask and rustup toolchain; uv chevron expands executables as mono chips) | Installed 100px mono (truncates, full value on hover) | Latest 216px (VersionDelta; fits `2025.10.1 → 2025.11.2` + severity chip at 210px, longer versions truncate with the chip pinned) | Status 150px (`StatusBadge`: Update available / Up to date / Pinned / Self-updating / Upgrading… / Queued — the widest, "Update available", measures 129px) | action 90px (row `Upgrade` ghost, hidden for up-to-date/pinned). Chips never wrap (`whitespace-nowrap`); table floor `min-w-[790px]` so a narrow window scrolls horizontally instead of columns colliding. Row states: hover `bg-raised`; selected `accent-subtle` wash; running op → spinner replaces button, checkbox locks; pinned → disabled checkbox + tooltip "Pinned in Homebrew — run `brew unpin <name>` to upgrade". Greedy casks: collapsed section "Self-updating casks (3)" with explainer, excluded from header select-all.
Pane states: loading (header immediate from detection + 8 skeleton rows); clean (`EmptyState` green check "Everything up to date" + counts; when OutdatedOnly hides all rows, a "Show all packages" link); error (`ErrorState`: taxonomy-mapped title, ≤10 stderr lines mono collapsible, Retry, View log; stale snapshot below, dimmed, "Showing last successful data from <time>"); absent (mas: EmptyState + install hint + Re-detect).

### 4.9 ActivityDrawer, StatusBar, HistoryView, SettingsView
**Drawer** collapsed 32px bar: "2 operations running · 1 queued" or "Idle", pulsing dot, chevron. Auto-opens when an upgrade/self-update starts (Settings toggle; never for refreshes). Expanded 40% (drag 25–60%): left `OperationList` (280px) — rows: status pill (Queued gray / Running accent pulsing / Succeeded green / Failed red / Cancelled gray / Timed out red / Stalled? amber / Interrupted gray), title ("Upgrade 3 · npm", "Self-update: mise · via brew"), duration ticker, queue position ("Waiting for brew · 1 ahead"), Cancel icon-button. Right `LiveLogView`: `bg-inset`, mono 12/1.6, virtualized, stdout `text-secondary`, stderr with amber 2px left gutter, `\r` repaints collapse in place, auto-scroll pinned with "Jump to latest ↓" chip on unpin, in-memory cap 5000 lines/op with "earlier output in log file" pinned first row; header strip: exact command (mono, copyable) + `Reveal log file`. Stall banner per F7.
**StatusBar**: left "Last full refresh 2m ago"; middle health icon (jump to first manager with issues); right gear + logs-folder icon; running summary mirrors the drawer.
**HistoryView**: filter bar (manager, status, search); table time/manager/kind/summary/duration/pill/exit code; row detail: full command, transcript tail (`get_operation`), Reveal in Finder; footer `Export diagnostics`.
**SettingsView**: per F11.

### 4.10 Dialogs & toasts
`UpgradePlanSheet` (overlay, 560px): title "Upgrade N packages", per-manager mono command blocks, toggles (self-updates on / greedy off), excluded list with reasons, warnings, footer note "Managers run in parallel; each manager runs one command at a time.", Cancel / `Upgrade` (primary). A pending or failed rebuild disables Upgrade; any execute error consumes the attempted capability and requires a fresh plan, and late execute/rebuild results are ignored after dismissal. `StalledOperationDialog` per F7. `QuitGuardDialog`: quitting with running ops lists them; "Cancel operations and quit" (danger) / "Keep running". `ToastHost` (top-right, stack ≤3): success auto-dismiss 4s ("brew: 1 package upgraded"); failure persists with `View log` (opens drawer to that op); info ("Refresh complete — 12 updates available"); routed self-update enqueue ("Update uv queued via mise").

### 4.11 Keyboard map
Cmd+R refresh current manager (Dashboard: all) · Cmd+Shift+R refresh all · Cmd+U upgrade selected (opens sheet) · Cmd+Shift+U Update Everything (sheet) · Cmd+A select all visible selectable rows · Space toggle focused row · Esc clear selection / close sheet / close drawer · Cmd+L toggle drawer · Cmd+F focus search · Cmd+1..9 sidebar jump. Roving tabindex in tables; live region announces op completions; all color states carry text/icon equivalents; text contrast ≥4.5:1 on its surface.

### 4.12 App icon
`dev/icon/generate_icon.py` (Pillow, run via `uv run --with pillow python3 dev/icon/generate_icon.py`) draws 1024×1024: rounded-square tile `#11151D`, 1px inner border `#303948`, three stacked isometric package boxes in a `#4F8CFF → #7B6CF6` vertical gradient, small upward chevron `#3FB96B` rising off the top box. Output `dev/icon/icon-1024.png` → `npx tauri icon dev/icon/icon-1024.png` → committed `src-tauri/icons/` (replaces the scaffold's default Tauri icons).

---

## 5. Architecture

### 5.1 Repository layout (builds on existing scaffold, commit d857cf5)
```
Pack-Manager/
├── dev/
│   ├── fixtures/                  # real captures (existing) + new + *_synthetic + README.md + ipc/
│   ├── capture-fixtures.sh        # re-captures live outputs, date-stamped
│   └── icon/generate_icon.py, icon-1024.png
├── src/                           # frontend (§4.4 component tree)
│   ├── main.tsx  App.tsx  styles/theme.css
│   ├── lib/ipc/{bridge.ts, client.ts, events.ts, types.ts}
│   ├── lib/{errors.ts, versionDelta.ts}
│   ├── store/{managers.ts, packages.ts, operations.ts, ui.ts, index.ts}
│   ├── hooks/useKeyboard.ts
│   ├── components/{shell,dashboard,manager,activity,history,settings,dialogs,primitives}/
│   └── test/{setup.ts, fakeIpc.ts, fixtures.ts}
├── src-tauri/
│   ├── Cargo.toml (package pack-manager, lib pack_manager_lib)  tauri.conf.json  icons/
│   ├── tests/live_smoke.rs        # all #[ignore]
│   └── src/
│       ├── main.rs  lib.rs  commands.rs  state.rs  settings.rs
│       ├── ipc.rs                 # ALL serde IPC payload types (camelCase)
│       ├── error.rs  events.rs    # EventSink trait + batching emitter
│       ├── logging.rs  paths.rs  detect.rs  registry.rs
│       ├── queue.rs  ops.rs  journal.rs  diagnostics.rs
│       ├── process/{mod.rs, runner.rs, fake.rs}
│       └── managers/{mod.rs, brew.rs, mise.rs, npm.rs, uv.rs, rustup.rs, mas.rs,
│                     parse/{mod.rs, brew.rs, mise.rs, npm.rs, uv.rs, rustup.rs, mas.rs}}
└── .github/workflows/ci.yml
```
Rust deps to add: `tokio` (rt-multi-thread, process, io-util, time, sync, macros), `tokio-util` (CancellationToken), `thiserror`, `async-trait`, `tracing`, `tracing-subscriber` (env-filter, json, registry), `tracing-appender`, `nix` (signal, process), `which`, `regex`, `dirs`, `uuid` (v7, serde), `time` (formatting, serde-well-known), `zip`, `tauri-plugin-opener`; dev: `tempfile`. Frontend deps to add: `zustand`, `@tanstack/react-virtual`.

### 5.2 ToolEnv — PATH resolution (`paths.rs`) — the #1 failure mode
Finder-launched apps get `PATH=/usr/bin:/bin:/usr/sbin:/sbin`. Built once at startup, rebuilt on Re-detect:
1. **Static list** (order matters; mise shims first so uv/npm/node resolve to shims): `$HOME/.local/share/mise/shims`, `/opt/homebrew/bin`, `/opt/homebrew/sbin`, `$HOME/.cargo/bin`, `$HOME/.local/bin`, `/usr/local/bin`, `/usr/bin`, `/bin`, `/usr/sbin`, `/sbin`.
2. **Login-shell probe** (best-effort, non-fatal, 5s timeout, 64KiB output cap): `$SHELL` (fallback `/bin/zsh`) with args `["-l","-c","echo __PM_S__; printf %s \"$PATH\"; echo; echo __PM_E__"]` — sentinels survive profile noise; `-i` is deliberately NOT used (interactive rc files can block on TTY). Extract between sentinels, split on `:`.
3. **Merge**: static list first, probe entries appended deduped. Probe failure → `source: staticFallback`, WARN logged, surfaced in Environment Report.
4. `ToolEnv { path: String, entries: Vec<PathBuf>, home: PathBuf, source: PathSource }` in AppState; full contents INFO-logged at startup (the single record that answers "it can't find brew").
5. Tools resolve to ABSOLUTE paths at detection (`which::which_in`); children are spawned by absolute path, never PATH lookup.
6. **Child env is constructed, never inherited**: `PATH` (ours), `HOME`, `USER`, `LOGNAME`, `TMPDIR`, `LANG=en_US.UTF-8`, `NO_COLOR=1`, `TERM=dumb`, `GIT_TERMINAL_PROMPT=0`, `HOMEBREW_COLOR=0`, `HOMEBREW_NO_EMOJI=1`, `HOMEBREW_NO_ENV_HINTS=1`, `HOMEBREW_NO_INSTALL_CLEANUP=1`; `HOMEBREW_NO_AUTO_UPDATE=1` on every brew command EXCEPT the explicit `brew update` spec.

### 5.3 Detection & managed-by classification (`detect.rs`)
```rust
pub enum ManagedBy { Brew, Mise, Rustup, Standalone }
pub enum DetectStatus { Absent { reason: String },
  Present { binary_path: PathBuf, canonical_path: PathBuf, version: Option<String>,
            managed_by: ManagedBy, evidence: String } }
```
`classify_managed_by(resolved: &Path, home: &Path) -> (ManagedBy, String)` is a **pure function**. Ordered rules:
1. RAW resolved path under `{home}/.local/share/mise/shims/` or `{home}/.local/share/mise/installs/` → `Mise`. **Checked BEFORE canonicalization** — mise shims are symlinks to the mise binary itself; canonicalizing `…/shims/uv` yields the brew-installed mise under `/opt/homebrew/`, which would misclassify uv (and npm) as brew-managed and misroute their self-updates to `brew upgrade uv`. The shim location is the evidence.
2. Else canonicalize; under `/opt/homebrew/`, `/usr/local/Cellar/`, or `/usr/local/Homebrew/` → `Brew`.
3. Under `{home}/.cargo/bin/` → `Rustup`.
4. If the classified owner IS the manager being classified (brew under /opt/homebrew) → `Standalone`.
5. Else `Standalone`.
Evidence strings ("resolved at ~/.local/share/mise/shims/uv") are stored, shown in ManagedByChip tooltips, logged, and included in diagnostics.

**Self-update route precedence** (resolved at detection, re-checked each refresh):
1. **In-band override**: the manager reports ITSELF in its own outdated listing → in-band. npm's case (fixture `npm_outdated_g_text_2026-07-21.txt` row `npm 11.16.0 12.0.1 12.0.1`) → `npm install -g npm@latest`. Without this rule npm would misroute to mise.
2. **Delegated**: `managed_by` is another *detected* manager → Routed to that executor (mise→`brew upgrade mise`, uv→`mise upgrade uv`). Executor absent → fall through.
3. **Native**: brew→ViaRefresh(`brew update`); rustup→`rustup self update`; mise standalone→`mise self-update`; uv standalone→`uv self update`; mas→Unavailable unless brew-managed (then `brew upgrade mas`).

### 5.4 ManagerAdapter trait (`managers/mod.rs`) — pure plans, generic execution
```rust
#[async_trait]
pub trait ManagerAdapter: Send + Sync {
    fn id(&self) -> ManagerId;
    fn display_name(&self) -> &'static str;
    fn binary_name(&self) -> &'static str;
    fn detection_candidates(&self) -> &'static [&'static str];   // fixed-path fallbacks
    /// PURE: ordered refresh commands (run serially inside one Refresh op).
    fn refresh_plan(&self, det: &DetectStatus, settings: &Settings) -> Vec<PlannedCommand>;
    /// PURE, fixture-tested: outputs in refresh_plan order -> snapshot.
    fn parse_refresh(&self, outputs: &[CommandOutput]) -> Result<ManagerSnapshot, PmError>;
    /// PURE: one recovery command when a given spec's output failed to parse (mise/npm/brew-casks text fallback).
    fn recovery_plan(&self, failed: &PlannedCommand) -> Option<PlannedCommand>;
    fn parse_recovery(&self, failed: &PlannedCommand, out: &CommandOutput) -> Result<ManagerSnapshot, PmError>;
    /// PURE: exact argv for upgrading the given package ids.
    fn upgrade_plan(&self, pkgs: &[PackageId], opts: &PlanOptions) -> Vec<PlannedCommand>;
    fn self_update_route(&self, managed_by: ManagedBy, own_outdated_row: Option<&Package>) -> SelfUpdateRoute;
    /// Non-zero exits that still mean success for a command (npm outdated -> 1 with JSON).
    fn classify_exit(&self, cmd: &PlannedCommand, out: &CommandOutput) -> ExitClass; // Success | ExpectedNonZero | Failure
}
pub struct PlannedCommand { pub label: &'static str, pub argv: Vec<String>,
    pub timeout: Timeout, pub extra_env: Vec<(String,String)>, pub phase_label: Option<String> }
pub enum Timeout { Absolute(Duration), Stall { silence: Duration, hard_cap: Duration } }
```

**Per-manager command surface (exact):**
| Manager | refresh_plan (Absolute timeouts) | upgrade argv | recovery |
|---|---|---|---|
| brew | `brew update` (600s, if setting on; phase "Updating Homebrew metadata…") → `brew list --versions` (60s) → `brew list --cask --versions` (60s) → `brew outdated --json=v2` (120s) → `brew outdated --json=v2 --greedy` (120s) | `brew upgrade <formulae…>`; `brew upgrade --cask <tokens…>`; opted-in greedy: `brew upgrade --cask --greedy <tokens…>` (split by kind, omit empty) | cask JSON parse failure → `brew outdated --greedy` text (`name (installed) != latest`) |
| mise | `mise ls --json` (60s) → `mise outdated --json` (120s; `{}` = clean, verified) | `mise upgrade <tool…>` (names verbatim incl. `npm:prettier`) | outdated JSON failure → `mise outdated` text (fixture-tested) |
| npm | `npm ls -g --depth=0 --json` (60s) → `npm outdated -g --json` (120s; **exit 1 with parseable JSON object = success**; same rule for `ls`) | `npm install -g <name>@latest …` (one command) | outdated JSON failure → `npm outdated -g` text (fixture-tested) |
| uv | `uv tool list` (60s) → `uv tool list --outdated` (120s; empty stdout = clean, per 0-byte capture) | `uv tool upgrade <name…>` | — |
| rustup | `rustup toolchain list` (30s) → `rustup check` (120s) | `rustup update <toolchain…>` | — |
| mas | `mas list` (60s) → `mas outdated` (120s) | `mas upgrade <id…>` | — (UNVERIFIED live; labeled) |
All upgrades use `Timeout::Stall { silence: settings.stallAfterSecs, hard_cap: settings.upgradeHardCapMins }`. The exact `mise ls` flags are confirmed during the mandatory fixture-capture step (IMPL_PLAN U3) before the parser is written.
**Greedy-only casks = set difference**: casks in the `--greedy` result minus casks in the plain result → `kind: caskGreedy`. Never an in-JSON heuristic (fixture casks carry concrete versions like `0.6.20`, not "latest" — an in-JSON rule is provably wrong).
**Snapshot assembly**: inventory rows → `latest = installed`, `outdated = false`; outdated overlay patches `latest`/`outdated = true`; overlay-only rows appended; the manager's own row (npm) is extracted into `selfPackage`.

### 5.5 Parser contracts (`managers/parse/*` — PURE functions, fixture-grounded)
All parsers read **stdout only** for data (uv additionally scans stderr for `warning:` lines). Verified fixture facts drive these contracts:
- **brew outdated JSON**: skip leading lines until the first line whose trim starts with `{` — fixture `brew_outdated.json` line 1 is `✔︎ JSON API packages.arm64_golden_gate.jws.json` before the `{`. Schema per fixture: `formulae[] { name, installed_versions: [String], current_version, pinned: bool, pinned_version }` (dolt: `2.2.1` → `2.2.2`, pinned false); `casks` — **both captured fixtures have `"casks": []`; the cask JSON shape is UNVERIFIED** — deserialize with `#[serde(default)]` + unknown-field tolerance, and the fixture-tested greedy-text parser (`openusage (0.6.20) != 0.7.6` etc.) is wired as recovery. `pinned: true` → pinned flag.
- **brew list --versions**: `name ver1 [ver2…]` per line, last version wins. (Fixture captured in U3 before implementation.)
- **mise outdated JSON**: `{}` = clean (verified). Populated shape UNVERIFIED → `*_synthetic` fixture with values copied verbatim from the text capture; text parser is the wired recovery. Text format (verified, NO header row in fixture): whitespace columns `tool requested current latest source` — 7 rows; `rust stable stable stable` → NOT outdated (current == latest); `npm:prettier` name preserved verbatim (split ids on first `:` only); source path → `meta.source`. A first line starting with `Tool` is skipped if present.
- **npm outdated JSON**: `{}` = clean (verified); populated shape `{ name: { current, wanted, latest, location, dependent } }` UNVERIFIED → synthetic fixture from the verified text capture (header `Package Current Wanted Latest Location Depended by`, 5 rows). Display uses `latest`; `wanted` → meta. The `npm` entry becomes `selfPackage`.
- **uv tool list**: `warning:` lines from either stream → `HealthIssue` (extract tool name between backticks independently of the optional reinstall suffix; the fixture line `warning: Tool \`aider-chat\` environment not found (run \`uv tool install aider-chat --reinstall\` to reinstall)` is runnable only because its suggestion exactly matches the allowlisted command derived from the safe tool name). Altered, missing, or malformed suggestions remain in `detail` but set neither `fixCommand` nor backend fix argv. Tool lines `^(\S+) v(\S+)$`; `- exe` lines accumulate into `meta.executables` (verified: 12 tools in each capture; `claude-code-tools` has 17 executables in `uv_tool_list.txt`; `serena-agent v1.6.2.dev0` proves non-semver versions). `--outdated`: empty output = clean (0-byte capture); any `(vX available)`-style suffix captured leniently as `latest`, unknown suffixes degrade to `latest: null` → UI shows "update available" without a fabricated delta. WARN-log on non-empty until the format is captured.
- **rustup check**: line regex `^(?P<name>\S+)\s+-\s+(?:Update available\s*:\s*(?P<from>\S+).*?->\s*(?P<to>\S+)|up to date\s*:\s*(?P<cur>\S+))` — the flexible `\s*:\s*` is load-bearing: verified fixtures contain BOTH `up to date: 1.97.1 (8bab26f4f 2026-07-14)` and `rustup - up to date : 1.29.0` within one file. Toolchain rows → packages; the `rustup` row → self-update state (`1.28.2 → 1.29.0` in the outdated fixture). Hashes/dates → meta.
- **rustup toolchain list**: `name [(default)]` per line. (Fixture captured in U3.)
- **mas**: list `^(\d+)\s+(.+?)\s+\((\S+)\)$`; outdated `^(\d+)\s+(.+?)\s+\((\S+) -> (\S+)\)$` — from docs, UNVERIFIED live (mas absent), synthetic fixtures labeled; failure mode is ParseFailed-with-excerpt, never a crash. `zsh: command not found: mas` never reaches a parser (detection gates it); a defensive test documents this.
Any parse failure → `PmError::ParseFailed { what, excerpt }` (first 500 chars), previous snapshot retained, manager error card — loud but never fatal.

### 5.6 Process execution (`process/`)
```rust
#[async_trait]
pub trait CommandRunner: Send + Sync {
    async fn run(&self, spec: &CommandSpec) -> Result<CommandOutput, PmError>;              // buffered (refresh/detection)
    async fn run_streaming(&self, spec: &CommandSpec, sink: LineSink, cancel: CancellationToken)
        -> Result<CommandOutput, PmError>;                                                  // upgrades/self-updates
}
pub struct CommandSpec { pub program: PathBuf /* absolute */, pub args: Vec<String>,
    pub env: Vec<(String,String)> /* full constructed env */, pub timeout: Timeout,
    pub purpose: CmdPurpose /* Detection|Refresh|Upgrade|SelfUpdate|HealthFix — drives logging/UI */ }
pub struct CommandOutput { pub exit_code: Option<i32>, pub stdout: String, pub stderr: String, pub duration: Duration }
pub struct LogLine { pub stream: StreamKind /* Out|Err */, pub line: String, pub ts_ms: u64 }
```
`RealRunner`: `tokio::process::Command`, `.process_group(0)`, `.stdin(Stdio::null())`, piped stdout/stderr with two reader tasks (`read_until(b'\n')`, `from_utf8_lossy`, additional split on `\r` for progress repaints, ANSI escapes stripped). Lines go to the sink AND the transcript AND capped (512KiB/stream) retention for `CommandOutput`. Stall watchdog resets per line; silence ≥ threshold → stalled notification (op continues); hard cap → group kill + Timeout. Absolute timeout → group kill + Timeout. Cancel → `killpg(pgid, SIGTERM)` → 5s grace → `SIGKILL`. `FakeRunner` (`cfg(any(test, feature = "test-util"))`): canned outputs keyed by (program basename, args), scripted line streams, `tokio::sync::Notify` gates for deterministic ordering, records every call; unmatched invocation panics with the spec.

### 5.7 Operation scheduler (`queue.rs`) — lock-set model
```rust
pub struct Operation { pub id: Uuid /* v7 */, pub kind: OpKind, pub executor: ManagerId,
    pub subject: ManagerId, pub locks: BTreeSet<ManagerId>, pub specs: Vec<CommandSpec>,
    pub cancel: CancellationToken, pub log_path: PathBuf }
pub enum OpKind { Refresh, Upgrade { package_ids: Vec<String> }, SelfUpdate, HealthFix { issue_id: String } }
```
Lock rules:
- Base: `{ executor }` — all brew-binary ops serialize (Homebrew's global lock never contended by us).
- Routed self-update adds the subject: `brew upgrade mise` holds `{Brew, Mise}` — no mise op runs while its binary is replaced.
- Shared-tree guards: npm ops add `Mise` when npm is mise-managed (`npm -g` writes inside mise's node tree); uv ops add `Mise` when uv is mise-managed. Correctness over throughput on exactly this machine's topology.
Scheduler: single tokio task owning `pending: VecDeque`, `held: BTreeSet<ManagerId>`, `running` map. On submit/completion, scan front-to-back and start every op whose `locks ∩ held = ∅` (FIFO with skip-ahead), bounded by a global `Semaphore(4)` (16GB headroom). Lock acquisition is atomic inside the single scheduler task → no deadlock possible; no ordered acquisition needed. **Starvation guard**: skip-ahead is disabled past any op that has waited >120s. Duplicate `refresh_manager` for a manager with a queued/running refresh coalesces (returns the existing opId). Spec execution is serial per op; a `Failure` classification aborts remaining specs. Terminal state → `op:status finished`, journal finish record, transcript footer; successful Upgrade/SelfUpdate/HealthFix auto-enqueues Refresh for `subject` (and `executor` if different).
**Plan builder** (`queue::build_upgrade_plan`, pure): receives a canonical request whose explicit selection has at most 2,048 entries, package IDs of at most 512 bytes, and exact duplicate manager/package pairs removed first-seen-order (`null` remains all-outdated). It expands that request into per-manager groups with exact argv previews; excludes pinned (reason `pinned`) and greedy casks unless opted in (reason `greedyCask`); **rust-dedup rule**: a single plan never contains both mise's `tool:rust` and any rustup `toolchain:*` — the mise entry is dropped with reason `rustDedup` and a sheet note ("rust toolchains are handled by rustup in this plan"); warns when a manager's last check errored ("list may be stale"). The IPC handler stores at most 64 issued plans, each bound to a monotonic canonical-state revision covering detection/routes, snapshots, queue busy/stale state, settings, and ToolEnv. `execute_plan` consumes the capability, compares submitted/issued/fresh plans within one coherent revision, rejects active refreshes, and sends the complete derived operation set to the scheduler as one atomic all-or-none batch. Scheduler admission rechecks the expected revision and rejects the batch if any incoming lock intersects an already queued/running Upgrade, SelfUpdate, or HealthFix; overlaps among groups in the same incoming batch remain valid and serialize normally. Successful admission advances the revision, so every other prebuilt plan is invalidated even if the first batch finishes immediately.
**Crash safety (`journal.rs`)**: append-only `~/Library/Application Support/Pack-Manager/operations.jsonl` — one line at start `{opId, kind, executor, subject, commandLine, pgid, startedAt}`, one at finish `{opId, outcome, exitCode, finishedAt}`, flushed each write; compacted to newest 1000 at startup. Start-without-finish → `Interrupted` in History. Recorded pgids are NEVER signaled on startup (pid reuse). This one file is both the crash journal and the History source.

### 5.8 Registry & cross-manager self-version join (`registry.rs`)
`RwLock<HashMap<ManagerId, ManagerSnapshot>>`. After any snapshot update, run the join: for each manager with a Routed self-update, look up its row in the executor's snapshot (mise's latest ← brew's `formula:mise`; uv's latest ← mise's `tool:uv` — fixture row `uv latest 0.11.26 0.11.30`) and patch the subject's `selfStatus`, then emit `snapshot:updated` for the subject. npm's selfStatus comes from its own listing; rustup's from `rustup check`'s rustup line; brew has no latest concept (ViaRefresh).

### 5.9 IPC contract (exact)

Conventions: commands use snake_case and return `Result<T, IpcError>`; serialized struct fields use lowerCamelCase. Enum wire spellings are explicit—ordinary values are lowercase or lowerCamelCase as defined by the type, while stable `ErrorCode` values use snake_case. TypeScript mirrors serialized fields in `src/lib/ipc/types.ts` (backend-only `#[serde(skip)]` fields are omitted); drift is guarded by the contract test (§7.4). Event names use Tauri-legal charset.

**Commands** (TS signatures in `client.ts`):
```ts
detect_managers(): Promise<DetectionReport>                       // also Re-detect; rebuilds ToolEnv
get_state(): Promise<AppState>                                    // rehydration on mount / dev reload
refresh_manager(args: { managerId: ManagerId }): Promise<OpRef>   // coalesces duplicates
refresh_all(): Promise<{ opIds: string[] }>
build_upgrade_plan(args: PlanRequest): Promise<UpgradePlan>       // issues one-use trust-device preview
execute_plan(args: { plan: UpgradePlan }): Promise<{ opIds: string[] }>
self_update_manager(args: { managerId: ManagerId }): Promise<OpRef>  // errors self_update_unavailable
run_health_fix(args: { managerId: ManagerId, issueId: string }): Promise<OpRef>
cancel_operation(args: { opId: string }): Promise<void>
get_operation(args: { opId: string }): Promise<OperationDetail>   // record + ring-buffer replay
list_operations(args: { limit: number }): Promise<OperationRecord[]>  // session + journal (Interrupted)
get_settings(): Promise<Settings>
set_settings(args: { patch: Partial<Settings> }): Promise<Settings>
reveal_operation_log(args: { opId: string }): Promise<void>       // tauri-plugin-opener
reveal_logs_dir(): Promise<void>
export_diagnostics(): Promise<{ zipPath: string }>
log_frontend_event(args: { level: 'warn'|'error', message: string }): Promise<void>
```

**Types** (TS mirrors serialized Rust fields; backend-only `#[serde(skip)]` fields are intentionally omitted):
```ts
type ManagerId = 'brew'|'mise'|'npm'|'uv'|'rustup'|'mas';
type ManagedBy = 'brew'|'mise'|'rustup'|'standalone';
type PackageKind = 'formula'|'cask'|'caskGreedy'|'tool'|'globalPackage'|'toolchain'|'app';
type OpKind = 'refresh'|'upgrade'|'selfUpdate'|'healthFix';
type OpStatus = 'queued'|'running'|'succeeded'|'failed'|'cancelled'|'timedOut'|'interrupted';

interface DetectionReport {
  managers: ManagerInfo[];
  env: { path: string; entries: string[]; source: 'loginShell'|'staticFallback'|'merged'; home: string };
}
interface ManagerInfo {
  id: ManagerId; displayName: string; status: 'present'|'absent';
  binaryPath?: string; canonicalPath?: string; version?: string;
  managedBy: ManagedBy; evidence?: string;
  selfUpdate: SelfUpdateRoute; installHint?: string;           // absent: e.g. "brew install mas"
}
type SelfUpdateRoute =
  | { kind: 'inBand';  commandPreview: string; note?: string }               // rustup; npm (note = mise-reset warning)
  | { kind: 'routed';  executor: ManagerId; commandPreview: string; why: string }
  | { kind: 'viaRefresh'; note: string }                                     // brew
  | { kind: 'unavailable'; reason: string };
interface Package {
  id: string;                    // `${kind}:${name}`, name verbatim; split on FIRST ':' only ("tool:npm:prettier")
  name: string; kind: PackageKind;
  installed: string | null; latest: string | null;             // verbatim; null = unknown (no fabricated deltas)
  outdated: boolean;                                           // the manager's verdict — authoritative
  pinned: boolean;
  meta?: { executables?: string[]; requested?: string; source?: string;
           wanted?: string; dependedBy?: string; pinnedVersion?: string };
}
interface ManagerSnapshot {
  managerId: ManagerId; refreshedAt: string;                   // RFC3339
  packages: Package[];                                         // excludes the manager's own self row
  selfStatus?: { installed: string | null; latest: string | null; updateAvailable: boolean };
  health: HealthIssue[];
}
interface HealthIssue { id: string; managerId: ManagerId; severity: 'warning'|'error';
  title: string; detail: string; fixCommand?: string; fixable: boolean }
interface PlanRequest {
  selection: { managerId: ManagerId; packageId: string }[] | null;   // null = all outdated, all managers
  includeSelfUpdates: boolean; includeGreedyCasks: boolean;
}
interface UpgradePlan {
  planId: string;
  request: PlanRequest;        // UI stale refresh only; execution trusts the cached request
  groups: { subject: ManagerId; executor: ManagerId; locks: ManagerId[];
            commands: { argvPreview: string; label: string }[];
            packageIds: string[]; selfUpdate: boolean }[];
  excluded: { managerId: ManagerId; packageId: string;
              reason: 'pinned'|'greedyCask'|'rustDedup'|'alreadyRunning' }[];
  notes: string[]; warnings: string[];
}
interface OpRef { opId: string }
interface OperationRecord {
  opId: string; kind: OpKind; executor: ManagerId; subject: ManagerId; status: OpStatus;
  commandLine: string; packageIds: string[];
  queuedAt: string; startedAt: string | null; finishedAt: string | null;
  exitCode: number | null; error: IpcError | null; logPath: string;
}
interface OperationDetail { record: OperationRecord; lines: LogLine[]; truncated: boolean }  // ring buffer, cap 5000
interface LogLine { stream: 'out'|'err'; line: string; tsMs: number }
interface AppState { detection: DetectionReport; snapshots: ManagerSnapshot[];
  operations: OperationRecord[]; settings: Settings }
interface Settings { runBrewUpdateOnRefresh: boolean; autoRefreshOnLaunch: boolean;
  stallAfterSecs: number; upgradeHardCapMins: number;
  logLevel: 'error'|'warn'|'info'|'debug'|'trace'; autoOpenDrawer: boolean;
  includeGreedyByDefault: boolean }
```

**Events** (subscribed once in `events.ts`; components never call `listen`; `bridge.ts` is the single mock seam):
| Event | Payload |
|---|---|
| `detection:updated` | `DetectionReport` |
| `snapshot:updated` | `{ managerId: ManagerId; snapshot: ManagerSnapshot }` (health rides in the snapshot) |
| `op:status` | `{ opId, kind, executor, subject, status: OpStatus, queuePosition?: number, phaseLabel?: string, commandLine, exitCode?: number, error?: IpcError, startedAt?, finishedAt?, logPath }` — emitted on enqueue (queued), start, phase change, finish |
| `op:output` | `{ opId, batch: LogLine[] }` — flushed every 50ms or 64 lines or 8KiB, whichever first |
| `op:stalled` | `{ opId, silentForSecs: number }` |
Backend emits through an `EventSink` trait (`events.rs`) so core logic never touches `tauri::AppHandle`; tests use `VecSink`. Frontend derives per-manager phase (idle/refreshing/busy/error) from op records — no separate status event.

### 5.10 Error taxonomy (`error.rs`)
```rust
#[derive(thiserror::Error, Debug)]
pub enum PmError {
    ToolNotFound { tool: String, searched: Vec<String> },
    SpawnFailed { program: String, detail: String },
    Timeout { after_secs: u64, phase: String },
    NonZeroExit { code: i32, stderr_tail: String },      // last 20 lines; consulted AFTER classify_exit
    BrewLockBusy { detail: String },                      // stderr matches "Another active Homebrew process"
    ParseFailed { what: String, excerpt: String },        // first 500 chars
    Cancelled,
    SelfUpdateUnavailable { reason: String },
    EnvCaptureFailed { detail: String },                  // login-shell probe failed (non-fatal, reported)
    Io { detail: String },
    Internal { detail: String },
}
```
Serialized as `IpcError { code, message, detail?, managerId?, opId?, logPath? }` with codes `tool_not_found | spawn_failed | timeout | non_zero_exit | brew_lock_busy | parse_failed | cancelled | self_update_unavailable | plan_stale | env_capture_failed | io | internal`. `plan_stale` covers unknown, evicted, replayed, altered, or current-state-mismatched bulk plans and requires review of a newly issued plan. `logPath` is always populated for op-scoped errors — "View log" never dangles. Rules: `ExpectedNonZero` never becomes `NonZeroExit`. `ParseFailed` on refresh keeps the previous snapshot. `BrewLockBusy` is a distinct user-facing state — "Homebrew is busy in another terminal. Retry when it finishes." — with NO automatic retry. User-facing copy per code lives in `src/lib/errors.ts` (state what happened + next action, e.g. timeout → "Homebrew refresh timed out after 600s. Check your network and retry."); offline degrades to per-manager timeout/error cards with snapshots retained.

### 5.11 Frontend state (zustand, `src/store/`)
- `managers`: DetectionReport; derived phase per manager from ops; lastError per manager.
- `packages`: snapshots by managerId; per-manager selection `Set<packageId>` + shift-anchor; filters (search, outdatedOnly).
- `operations`: records byId + ordered; per-op log ring buffer (5000 lines, overflow counter).
- `ui`: activeView, drawer open/height, focused opId, dialog state, toasts, settings copy.
`client.ts` is typed wrappers; `bridge.ts` re-exports invoke/listen from `@tauri-apps/api` and is the ONLY importer of it; `events.ts` wires events → store in one subscription from App mount. Derived selectors (`outdatedCount`, `totalOutdated`) memoized.

### 5.12 Startup sequence
1. `logging::init()` (first) → prune old logs. 2. Load settings → build ToolEnv → detection → load journal (mark Interrupted) → registry. 3. Window shows; frontend `get_state()` → render → `refresh_all()` if `autoRefreshOnLaunch`. No blank window: skeletons render from detection immediately.

---

## 6. Logging design

### 6.1 Sinks
| Sink | Path | Format | Retention |
|---|---|---|---|
| App log | `~/Library/Logs/Pack-Manager/pack-manager.log.<YYYY-MM-DD>` | JSONL via `tracing` + `tracing-appender` daily rolling | prune >14 days at startup |
| Operation transcripts | `~/Library/Logs/Pack-Manager/operations/<YYYY-MM-DDTHH-mm-ss>_<opId8>_<manager>_<kind>.log` | plain text | keep newest 200 files or 90 days |
| Journal/History | `~/Library/Application Support/Pack-Manager/operations.jsonl` | JSONL | compact to newest 1000 at startup |
| Dev console | stderr, pretty | `cfg(debug_assertions)` only | — |

### 6.2 Transcript format (written incrementally from spawn; line-flushed — survives crash)
```
=== Pack-Manager operation ===
op_id:      01914f2e-…            kind: upgrade      executor: npm   subject: npm
queued_at:  2026-07-22T14:03:11Z  started_at: 2026-07-22T14:03:11Z
command:    /Users/sallvain/.local/share/mise/shims/npm install -g typescript@latest
cwd:        /Users/sallvain
PATH:       /Users/sallvain/.local/share/mise/shims:/opt/homebrew/bin:…
env_set:    NO_COLOR=1 TERM=dumb GIT_TERMINAL_PROMPT=0 HOMEBREW_NO_AUTO_UPDATE=1 …
timeout:    stall 120s / hard cap 1800s        pgid: 51234
=== output ===
14:03:12.104 [out] added 1 package in 4s
14:03:12.371 [err] npm warn deprecated …
=== result ===
status: succeeded   exit_code: 0   duration: 5.2s   finished_at: 2026-07-22T14:03:16Z
```
Cancelled/timed-out footers record the signal path (`cancelled (SIGTERM→exit)`, `timed_out (SIGKILL after 5s grace)`).

### 6.3 Levels & filtering
Default directive: `info,pack_manager_lib=debug`. Precedence: env `PACK_MANAGER_LOG` (EnvFilter syntax) > Settings logLevel > default; Settings changes apply live via `tracing_subscriber::reload`. Every op runs inside `info_span!("op", op_id, executor, subject, kind)`; commands inside `debug_span!("cmd", program, purpose)` — everything greppable by opId.
| Level | Policy |
|---|---|
| ERROR | op failures (post-classify NonZeroExit, SpawnFailed, Timeout), ParseFailed with excerpt, journal/IO errors |
| WARN | login-shell probe failure (StaticFallback), stall fired, ExpectedNonZero exits, non-empty uv `--outdated` (under-verified shape), health issues found, BrewLockBusy, event-buffer overflow, frontend errors (target `pack_manager_lib::frontend`) |
| INFO | app start (version/OS), ToolEnv result (full PATH + source + per-tool resolved/canonical paths — the record that answers most PATH reports), detection + routing decisions ("uv managedBy=mise → routed: mise upgrade uv; evidence=~/.local/share/mise/shims/uv"), op lifecycle (queued/started/finished, duration, exit), snapshot summaries, settings changes, prune stats |
| DEBUG | every CommandSpec before spawn (program, argv, timeout kind; env as overridden keys only — NEVER a full env dump), scheduler decisions ("op X waits: locks {brew} held by op Y"), classify_exit outcomes, parser summaries ("brew outdated: 1 formula, 0 casks, 1 junk line skipped"), cross-join patches, batch flush stats |
| TRACE | raw child lines mirrored into the app log (off by default; transcripts already have them — TRACE exists to interleave concurrent ops in one stream) |
Hygiene: parsers never log (pure); no secrets exist in this domain but full env dumps are banned on principle; all writers are non-blocking — a full disk drops log lines, never hangs an upgrade.

### 6.4 Debugging walkthrough (the design target)
1. Toast "npm upgrade failed (exit 1)" → **View log** → drawer shows the full transcript; usually sufficient. 2. Op row → **Reveal log file** → header has absolute program path, argv, PATH, pgid; footer has exit/outcome. 3. Routing question ("why did updating uv run mise?") → `grep <opId>` the app log: INFO shows the resolved route with evidence; DEBUG shows lock waits. 4. Finder-launch "command not found" → Settings → Environment Report (PATH source `staticFallback` means the probe failed; WARN has why). 5. Crash mid-upgrade → History shows Interrupted + transcript-so-far; journal has the start record incl. pgid. 6. Anything else → Settings→debug → reproduce → **Export diagnostics** → one zip.

---

## 7. Test plan

Principles: every nondeterministic input has exactly one seam — processes → `CommandRunner`; time → `tokio::test(start_paused = true)` / `vi.useFakeTimers()`; IPC → `bridge.ts`; paths → constructor params. No real processes, no network, no sleeps in default suites. `cargo test` and `npm test` pass offline on a clean checkout. Fixtures load via `concat!(env!("CARGO_MANIFEST_DIR"), "/../dev/fixtures/…")`.

### 7.1 Rust parser tests (names are the spec; all against real fixtures unless marked synthetic)
- `brew_outdated_json_skips_leading_junk_line` — line 1 `✔︎ JSON API packages.arm64_golden_gate.jws.json`; yields dolt 2.2.1→2.2.2, pinned false, 0 casks.
- `brew_outdated_clean_json_parses_without_junk` (`brew_outdated_greedy.json`).
- `brew_greedy_only_is_set_difference` — greedy minus plain; text recovery parses `openusage (0.6.20) != 0.7.6`, `syncthing-app (2.0.14-1) != 2.1.2-1`, `transmission (4.1.1) != 4.1.3`.
- `brew_text_where_json_expected_is_parse_failed_with_excerpt` — never a panic.
- `mise_outdated_json_empty_object_means_clean` (`{}`).
- `mise_outdated_text_parses_seven_rows_six_outdated` — `rust stable stable stable` dropped (current==latest); `npm:prettier` verbatim; `uv 0.11.26→0.11.30`; source path in meta.
- `npm_outdated_json_empty_object_means_clean`; `npm_outdated_text_parses_five_rows_hoists_npm_self` — self `11.16.0→12.0.1`; 4 package rows; wanted/location→meta.
- `npm_outdated_json_populated_synthetic` — values copied verbatim from the text capture.
- `uv_tool_list_extracts_broken_env_warning_and_fix_command` — HealthIssue{aider-chat, `uv tool install aider-chat --reinstall`}; 12 tools parsed.
- `uv_tool_list_clean_collects_executables` — 12 tools, claude-code-tools 17 exes, `serena-agent v1.6.2.dev0` verbatim.
- `uv_outdated_empty_output_is_clean_not_error` (0-byte fixture); `uv_outdated_unknown_suffix_degrades_to_null_latest`.
- `rustup_check_outdated_yields_toolchain_and_self` — `stable-aarch64-apple-darwin 1.94.0→1.97.1`; self `1.28.2→1.29.0`.
- `rustup_check_tolerates_both_colon_spacings` — `up to date:` and `up to date :` in one file.
- `mas_outdated_synthetic_parses`; `mas_shell_error_never_reaches_parser` (documents detection gating).
- Inventory parsers tested against fixtures captured in U3 (`brew list --versions`, `mise ls`, `npm ls -g --json`, `rustup toolchain list`).
- Adapter merge: inventory + overlay → statuses; overlay-only row appended; self row extracted.

### 7.2 Rust pure-logic tests
- `classify_mise_shim_path_is_mise_managed_without_canonicalizing` (THE regression test), `classify_opt_homebrew_canonical_is_brew`, `classify_cargo_bin_is_rustup`, `classify_brew_itself_is_standalone`, `classify_unknown_is_standalone` — plus a counterfactual (uv standalone in `~/.local/bin` → inBand) proving routing is dynamic.
- Route precedence: npm managedBy=mise + self in own listing → inBand override wins; mise with brew detected → routed; mise with brew ABSENT → inBand `mise self-update`.
- Lock sets: routed self-update `{brew, mise}`; npm upgrade with mise-managed node `{npm, mise}`.
- Plan builder: exact argv previews; pinned/greedy exclusions; rust-dedup drops `tool:rust` when rustup toolchains present, with reason; stale-check warning.
- Cross-join: brew snapshot `formula:mise a→b` → mise selfStatus.latest == b; uv from mise `tool:uv`.
- ToolEnv: sentinel extraction ignores profile noise; probe failure → StaticFallback with shims-before-homebrew ordering; merge dedupes preserving order.

### 7.3 Rust scheduler/process tests (FakeRunner + gates + paused time — zero sleeps)
- `two_brew_ops_never_overlap_fifo` (call-log intervals); `brew_and_mise_run_concurrently`.
- `routed_self_update_blocks_subject_lane` — queued `mise upgrade deno` waits during `brew upgrade mise`; unrelated rustup starts.
- `npm_op_blocks_mise_when_mise_managed`; `skip_ahead_starts_unblocked_op`; `aging_guard_blocks_skip_ahead_after_120s`.
- `semaphore_caps_concurrency_at_4`; `duplicate_refresh_coalesces_to_same_opid`.
- `npm_exit_1_with_json_is_success`; `npm_exit_1_with_garbage_is_parse_failed`; `brew_lock_stderr_maps_to_brew_lock_busy`.
- `recovery_plan_runs_text_fallback_on_json_parse_failure` (mise).
- `cancel_sigterm_then_sigkill_marks_cancelled_finalizes_transcript`; `stall_fires_at_threshold_and_rearms_on_output`; `hard_cap_times_out`.
- `failure_aborts_remaining_specs`; `successful_upgrade_auto_enqueues_refresh`.
- `journal_start_finish_roundtrip`; `start_only_record_surfaces_interrupted`; event batching `500_lines_flush_in_le_64_line_batches_at_ge_50ms`.
- `src-tauri/tests/live_smoke.rs`: all `#[ignore]` — real detection finds brew/mise/npm/uv/rustup with expected classification; real `brew outdated --json=v2` round-trips. Developer-run only, never CI/default.

### 7.4 Contract sync
Committed representative payload JSON per IPC type in `dev/fixtures/ipc/`. Rust test `ipc_contract_matches_committed_fixtures` constructs the same values, serializes, asserts byte-equality (regenerate with `PM_UPDATE_CONTRACT=1`). Vitest `ipc_types_accept_contract_fixtures` imports the same JSON and passes it through TS type guards. Both offline.

### 7.5 Frontend (Vitest + RTL + jsdom; seam = `vi.mock('@/lib/ipc/bridge')` via `test/fakeIpc.ts` exposing `respond/emit/calls`; fixtures in `test/fixtures.ts` hand-derived from real dev/fixtures values)
- `dashboard_fills_cards_independently` (emit npm snapshot; brew still skeleton).
- `manager_error_isolates_with_retry_and_stale_snapshot` (brew fails; mise table intact; Retry re-invokes).
- `mas_absent_renders_not_installed_with_hint`; `all_clean_renders_empty_state`.
- `outdated_only_toggle_default_behavior`; `search_filters_names_and_executables`.
- `selection_shift_range_tri_state_and_toolbar_count`; `select_all_respects_filter_and_never_greedy_or_pinned`.
- `plan_sheet_renders_exact_command_previews_and_excluded_reasons`; `confirm_calls_execute_plan_with_toggled_plan`; `greedy_toggle_off_by_default`.
- `upgrade_selected_dispatches_exact_ids_then_clears_on_success`.
- `self_update_card_shows_routed_subtitle_and_queued_behind_executor`; `npm_card_shows_mise_reset_note`; `self_update_disabled_when_executor_absent`.
- `version_delta_highlights_only_changed_segments` (`2.2.1→2.2.2` last segment patch; `6.0.3→7.0.2` major; `stable→stable` plain; missing latest → "update available").
- `log_view_appends_batches_pins_unpins_with_jump_chip` (fake timers); `stall_dialog_keep_waiting_vs_cancel`; `cancel_flips_pill_on_event`.
- `failure_toast_persists_and_view_log_focuses_op`; `health_banner_renders_fix_command`.
- `keyboard_map_dispatches_actions` (Cmd+R/A/L/Esc etc.).
- `interrupted_ops_render_in_history`.

### 7.6 CI (`.github/workflows/ci.yml`)
rust (macos-14): `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, `cargo test --locked`. web (ubuntu): `npm ci`, `tsc --noEmit`, `vitest run`, `npm run build`. build-smoke (macos-14, main only): `npm run tauri build -- --debug`, upload `.app`. Caches; no job touches the network beyond dependency install. Beta-OS-specific issues are diagnosed on-machine by design.

---

## 8. Packaging & delivery
**Superseded for MVP.** This section described the original ad-hoc-signed deliverable; delivery has since moved to a signed, notarized, auto-updating release pipeline. DECISIONS D20 (notarization out of scope) is superseded by D25, which records why and what it cost. The current pipeline is documented in README "Releases" and implemented in `.github/workflows/release.yml`.

What still holds from the original text: the app must launch from Finder/Dock (the PATH machinery in §5.2 exists precisely for this) — the final gate verifies via `open` of the built bundle, not just `tauri dev`.

What changed: `release.yml` builds a universal (arm64 + x86_64) bundle, signs it with a Developer ID Application certificate, notarizes and staples both the `.app` and the `.dmg`, and attaches `.dmg`, `.zip`, `.app.tar.gz` + `.sig`, and `latest.json` to the GitHub Release. The `.app.tar.gz` and `latest.json` are what `tauri-plugin-updater` consumes for the in-app update flow (D25); the Package step asserts the archived `.app` carries a notarization ticket rather than trusting bundler ordering, because an un-stapled auto-update would install an app that has to phone Apple on first launch.
