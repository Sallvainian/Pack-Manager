# Pack-Manager

macOS desktop app that auto-detects the package managers on the machine
(`brew`, `mise`, `npm`, `uv`, `rustup`, `mas`), lists every package with
installed/latest versions, and upgrades everything or a hand-picked selection —
with dynamically-derived self-update routing, live streaming output,
cancellation, and byte-faithful transcripts.

Authoritative design docs: `docs/SPEC.md`, `docs/DECISIONS.md`,
`docs/IMPL_PLAN.md`.

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

IPC contract fixtures live in `dev/fixtures/ipc/`; regenerate with
`PM_UPDATE_CONTRACT=1 cargo test ipc_contract` (the Vitest type-guard test
checks the same files — both sides must move together).

App icon: `uv run --with pillow python3 dev/icon/generate_icon.py`, then
`npx tauri icon dev/icon/icon-1024.png`.

## Log locations

| What | Where |
|---|---|
| App log (JSONL, daily rolling) | `~/Library/Logs/Pack-Manager/pack-manager.log.<YYYY-MM-DD>` |
| Operation transcripts | `~/Library/Logs/Pack-Manager/operations/` |
| Journal / history | `~/Library/Application Support/Pack-Manager/operations.jsonl` |
| Settings | `~/Library/Application Support/Pack-Manager/settings.json` |

Log filter precedence: `PACK_MANAGER_LOG` env (EnvFilter syntax) > Settings
logLevel > default `info,pack_manager_lib=debug`.

_README is completed in the final integration pass (features, screenshots,
diagnostics, fixture-capture how-to, known limitations)._
