# Pack-Manager

macOS desktop app that auto-detects the package managers on the machine
(`brew`, `mise`, `npm`, `uv`, `rustup`, `mas`), lists every package with
installed/latest versions, and upgrades everything or a hand-picked selection —
with dynamically-derived self-update routing, live streaming output,
cancellation, and byte-faithful transcripts.

Authoritative design docs: `docs/SPEC.md`, `docs/DECISIONS.md`,
`docs/IMPL_PLAN.md`.

## Features

- **Manager auto-detection** — resolves each manager on a constructed search
  path (login-shell probe + static fallback, so Finder launches work), probes
  `--version`, and classifies who-manages-whom from paths with a
  human-readable evidence string ("resolved at ~/.local/share/mise/shims/uv").
  Absent managers (mas here) render as "Not installed" with a copyable
  install hint — never an error.
- **Refresh with per-manager isolation** — inventory + outdated overlay per
  manager, run in parallel with independent timeouts; one failing manager
  never blanks the others, and the previous snapshot stays browsable, marked
  stale. The manager's own outdated verdict is authoritative — Pack-Manager
  never computes version comparisons to decide outdatedness.
- **Package tables** — virtualized, with version-delta highlighting
  (display-only severity coloring), pinned-formula exclusion, greedy
  self-updating casks in their own opt-in section, uv executable expansion,
  search, and outdated-only filtering.
- **Upgrade plan sheet** — every bulk upgrade previews the EXACT commands
  (`brew upgrade dolt`, `mise upgrade deno ruby …`, `npm install -g
  typescript@latest`), with excluded rows and reasons (pinned / greedy /
  rust-dedup) and staleness warnings. Nothing runs that was not shown.
- **Dynamic self-update routing** — derived at detection, never hardcoded:
  brew→`brew update`, mise→via brew, npm→in-band (it reports itself
  outdated), uv→via mise, rustup→`rustup self update`. Routed updates hold
  both managers' locks so a binary is never replaced under a running op.
- **Lock-set scheduler** — all brew-binary operations serialize (Homebrew is
  never contended); different managers run in parallel; npm/uv ops take the
  mise lock when mise-managed (shared tree); FIFO with skip-ahead and a 120s
  aging guard.
- **Live streaming + cancellation + stall detection** — batched line
  streaming into a virtualized log view; cancel = SIGTERM → 5s → SIGKILL on
  the process group; silent commands trigger a stall dialog with a
  copy-to-terminal handoff (Pack-Manager never enters passwords — no sudo,
  ever; child stdin is /dev/null).
- **History + transcripts + crash-safe journal** — every operation writes a
  plain-text transcript (argv/PATH/env header, timestamped output, result
  footer) and start/finish journal records; ops interrupted by a crash
  surface as `Interrupted` on relaunch, and recorded pgids are never signaled
  on startup.
- **Diagnostics export** — one zip on the Desktop: report.json (detection with
  evidence, search path + source, settings), recent app logs, last 25
  transcripts, and the journal.

## Stack

- Backend: Tauri 2 + Rust (tokio, tracing)
- Frontend: React 19 + TypeScript + Vite 7 + Tailwind v4 (dark-only MVP)
- State: zustand · Virtualized tables: @tanstack/react-virtual
- Tests: cargo test (offline, fixture-grounded) + Vitest 4 (jsdom, RTL)

## Dev commands

```sh
npm install                # frontend deps
npm run tauri dev          # run the app (dev)
npm run tauri build        # ad-hoc-signed .app under src-tauri/target/release/bundle/macos/

npm test                   # Vitest suite
npx tsc --noEmit           # typecheck
cd src-tauri && cargo test # Rust suite (offline; live smoke is #[ignore])
```

Lint/format gates (CI runs all of these):

```sh
cd src-tauri && cargo fmt --check && cargo clippy --all-targets -- -D warnings
```

Live smoke tests (real commands against THIS machine — developer-run only,
never CI):

```sh
cd src-tauri && cargo test -- --ignored
```

IPC contract fixtures live in `dev/fixtures/ipc/`; regenerate with
`PM_UPDATE_CONTRACT=1 cargo test ipc_contract` (the Vitest type-guard test
checks the same files — both sides must move together).

App icon: `uv run --with pillow python3 dev/icon/generate_icon.py`, then
`npx tauri icon dev/icon/icon-1024.png`.

## Fixture capture

Parsers are grounded in real command output committed under `dev/fixtures/`
(provenance for every file in `dev/fixtures/README.md`). To re-capture the
offline-safe inventory fixtures (date-stamped, never clobbers an existing
capture):

```sh
dev/capture-fixtures.sh                    # offline-safe inventory/list commands
PM_CAPTURE_ONLINE=1 dev/capture-fixtures.sh  # + network-dependent outdated probes
```

Synthetic fixtures carry a `_synthetic` suffix, copy values verbatim from real
captures, and are listed with retirement conditions in the fixtures README.

## Log locations

| What | Where |
|---|---|
| App log (JSONL, daily rolling) | `~/Library/Logs/Pack-Manager/pack-manager.log.<YYYY-MM-DD>` |
| Operation transcripts | `~/Library/Logs/Pack-Manager/operations/` |
| Journal / history | `~/Library/Application Support/Pack-Manager/operations.jsonl` |
| Settings | `~/Library/Application Support/Pack-Manager/settings.json` |

Log filter precedence: `PACK_MANAGER_LOG` env (EnvFilter syntax) > Settings
logLevel > default `info,pack_manager_lib=debug`.

## Diagnostics

Settings → **Export diagnostics** (or the History footer) writes
`~/Desktop/Pack-Manager-diagnostics-<YYYYMMDD-HHmmss>.zip` containing
report.json (app/OS version, resolved search path + source, full detection
report with evidence, settings, log filter), the last 3 app-log files, the
last 25 transcripts, and `operations.jsonl`. Only env vars the app sets are
ever logged — never the inherited environment.

Debugging starts at the transcript: every operation's log file has the exact
program path, argv, PATH, and set env in its header and the outcome in its
footer; `grep <opId>` over the app log answers routing and lock-wait
questions.

## Known limitations

- **mas is unverified live** — the adapter ships fully implemented, but this
  machine doesn't have mas installed, so its parsers are tested only against
  labeled `_synthetic` fixtures. Parse failures degrade to an error card with
  an excerpt, never a crash (DECISIONS D23).
- **Notarization is out of scope** — `npm run tauri build` produces an
  ad-hoc-signed `.app` (the shipping deliverable). No Developer-ID signing,
  no notarized DMG (DECISIONS D20; macOS 27 beta + Xcode-beta).
- **Dark theme only** — tokens are structured for a future light theme, but
  MVP ships dark-only (DECISIONS D19).
- **Cask outdated JSON shape is unverified** — both captured fixtures have
  `"casks": []`; a fixture-tested text parser is wired as automatic recovery
  (DECISIONS D7).
- **Updates only** — no package install/uninstall, no auto-upgrades without
  user action, no telemetry.
