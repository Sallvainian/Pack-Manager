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
- **In-app auto-update** — checks on launch, every 6h, and on demand from the
  macOS app menu (**Pack-Manager → Check for Updates…**). A found update
  downloads in the background with progress in the status bar, then that
  indicator becomes a **Restart to update** button. Installing is never
  automatic — the click is the gate. If the app bundle's directory isn't
  writable the updater would need an admin prompt, so it stops and says so
  instead (Pack-Manager never asks for a password). DECISIONS D25.

## Stack

- Backend: Tauri 2 + Rust (tokio, tracing)
- Frontend: React 19 + TypeScript + Vite 7 + Tailwind v4 (dark-only MVP)
- State: zustand · Virtualized tables: @tanstack/react-virtual
- Tests: cargo test (offline, fixture-grounded) + Vitest 4 (jsdom, RTL)

## Dev commands

```sh
npm install                          # frontend deps
npm run tauri dev                    # run the app (dev)
fnox exec -- npm run tauri build     # .app + .dmg + updater archive

npm test                   # Vitest suite
npx tsc --noEmit           # typecheck
cd src-tauri && cargo test # Rust suite (offline; live smoke is #[ignore])
```

Build output lands under `src-tauri/target/release/bundle/` — the app and its
updater archive in `macos/` (`Pack-Manager.app`, `Pack-Manager.app.tar.gz`,
`.sig`), the installer in `dmg/`.

`tauri build` needs the updater's minisign key, because `bundle.createUpdaterArtifacts`
is on and the CLI refuses to bundle when it finds a configured `pubkey` with no
private key ("A public key has been found, but no private key"). `fnox exec`
supplies `TAURI_SIGNING_PRIVATE_KEY` and its password; CI reads the same two
values from GitHub secrets of the same name. To build without signing at all,
add `--no-sign` (what `ci.yml`'s build-smoke job does).

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

## Releases

Fully automated by release-please — see `CLAUDE.md` for the rules that matter
when committing.

1. Push conventional commits to `main`; release-please opens a
   `chore(main): release X.Y.Z` PR that bumps the five version files in
   lockstep and writes `CHANGELOG.md`.
2. **Merging that PR is the release.** It tags `vX.Y.Z` and publishes the
   GitHub Release.
3. `release.yml` then builds a universal (arm64 + x86_64) app, signs it with
   Developer ID, notarizes and staples it, and attaches:

| Asset | For |
|---|---|
| `Pack-Manager-X.Y.Z.dmg` | first install |
| `Pack-Manager-X.Y.Z.zip` | archive |
| `Pack-Manager-X.Y.Z.app.tar.gz` + `.sig` | the in-app updater |
| `latest.json` | the updater endpoint |

Signing and notarization degrade gracefully when the secrets are absent (e.g.
on a fork): the build stays unsigned and the job still succeeds.

Never hand-edit a version — release-please owns `package.json`,
`package-lock.json`, `src-tauri/tauri.conf.json`, `src-tauri/Cargo.toml`, and
`src-tauri/Cargo.lock`, plus `CHANGELOG.md` and `.release-please-manifest.json`.

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
- **Updating from a read-only location** — the updater replaces the `.app` in
  place, so it needs write access to the bundle's parent directory. Run from
  `/Applications` (or `~/Applications`); launched straight off a mounted DMG,
  the in-app update stops at "needs a manual install" rather than raising an
  admin prompt (DECISIONS D25).
- **Dark theme only** — tokens are structured for a future light theme, but
  MVP ships dark-only (DECISIONS D19).
- **Cask outdated JSON shape is unverified** — both captured fixtures have
  `"casks": []`; a fixture-tested text parser is wired as automatic recovery
  (DECISIONS D7).
- **Updates only** — no package install/uninstall, no auto-upgrades without
  user action, no telemetry.

## License

MIT — see [LICENSE](LICENSE).
