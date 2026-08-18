<!-- bmad:context -->
<!-- Verified 2026-08-18 against c8302e2. Managed by bmad-project-context; edits inside this block are replaced on refresh. Keep anything you want preserved outside the markers. -->

## Pack-Manager

macOS GUI for package managers (Tauri 2 + React 19 + TypeScript + Vite; Rust backend in `src-tauri/`). One native app bundle — no HTTP API, no database. Product behavior lives in `docs/SPEC.md`, the decision log in `docs/DECISIONS.md`, architecture invariants in the spine (see *Where things are*).

## Policy

- Never hand-edit the five version-lockstep files (`package.json`, `package-lock.json`, `src-tauri/tauri.conf.json`, `src-tauri/Cargo.toml`, `src-tauri/Cargo.lock`), `CHANGELOG.md`, or `.release-please-manifest.json` — release-please owns all seven.
- Never spawn shell command strings for a package-manager operation — structured argv only (`env_clear()`, explicit env, null stdin, `process_group(0)`); no sudo or password-prompt path exists anywhere in this app.
- Never move files out of `_bmad-output/archive/2026-07-24-scope-recalibration/` — the formal 72-criterion release-readiness gate it holds was deliberately retired (D33); `docs/RELEASE-CHECKLIST.md` is the sole release-readiness authority now.
- Keyboard navigation and screen-reader support are explicitly not release criteria (D37, retired 2026-07-25 for this single-user mouse-driven app) — don't report their absence as a gap.

## Where things are

- Doc authority by role: `docs/SPEC.md` governs product behavior; `docs/DECISIONS.md` is the dated decision log (a later entry only overrides an earlier one when it says so explicitly — no implicit last-write-wins; re-count with `grep -c '^## D' docs/DECISIONS.md` rather than trusting a cached number); the architecture spine (`_bmad-output/planning-artifacts/architecture/architecture-Pack-Manager-2026-07-23/ARCHITECTURE-SPINE.md`) is the sole authority for architecture invariants and `AD-` ids (read its `artifact_revision:` frontmatter yourself); `docs/IMPL_PLAN.md` is history only.
- High-risk trust boundaries needing focused tests and review: `queue.rs`, `process/runner.rs`, `ops.rs`, `ipc.rs`, `journal.rs`/`settings.rs`, `diagnostics.rs`, `app_update.rs`.
- Per-manager phase (idle/refreshing/busy/error) is never stored — it's derived from operation records in `deriveManagerPhase()` (`src/store/index.ts`).
- `DialogHost` (`src/components/dialogs/`) is the single mount point for modal surfaces; navigation is the discriminated `ActiveView` union in `src/store/ui.ts` — don't add a router.
- `src/lib/ipc/bridge.ts` is the sole frontend importer of `@tauri-apps/api` (convention only, nothing enforces it).
- `@seontechnologies/playwright-utils` is an installed devDependency (added for TEA) but is not imported or wired into any fixture, spec, or config — don't assume Playwright tests use it. `@seontechnologies/pactjs-utils` isn't installed at all, despite BMAD config declaring `tea_use_pactjs_utils: true`.

## Running and verifying

- Push conventional commits to `main` — that's the entire release trigger:

  | Prefix | Effect |
  | --- | --- |
  | `fix: …` | patch bump |
  | `feat: …` | minor bump |
  | `feat!: …` / `BREAKING CHANGE:` footer | major bump |
  | `chore:`, `docs:`, `ci:`, `refactor:`, `test:` | no release |

  A `Release-As: X.Y.Z` footer (own paragraph) forces a specific version.
- Merging the release-please PR **is** the release — `release-please.yml` squash-merges it automatically the moment it's opened, with no human approval step. Keep unfinished work off `main`.
- The release PR must be squash-merged, never a plain merge commit — a merge commit repeats the message in its body and release-please double-counts the fix in the changelog (happened across 0.2.0–0.2.3).
- Frontend gates (`ci.yml` `web`): `npx tsc --noEmit`, `npx vitest run`, `npm run build`. Rust gates (`ci.yml` `rust`, from `src-tauri/`): `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, `cargo test --locked`. Browser e2e is a separate required gate (`test.yml`): `npm run test:e2e` + `test:e2e:typecheck` — `vitest.config.ts` excludes `tests/e2e/**` and the root `tsconfig.json` only includes `src/`, so neither `tsc` nor the Vitest/build gates ever see anything under `tests/`.
- `npm run tauri build` fails locally with no args — `tauri.conf.json` sets an updater pubkey with no matching `TAURI_SIGNING_PRIVATE_KEY`. Build unsigned with `-- --no-sign` (what CI's build-smoke job does) or signed with `fnox exec -- npm run tauri build`.
- After an intentional IPC change, regenerate contract fixtures with `PM_UPDATE_CONTRACT=1 cargo test ipc_contract` from `src-tauri/`, then update `src/lib/ipc/types.ts` — never set that var during a normal verification run, it overwrites fixtures instead of asserting.
- Node, the macOS runner label, and the dev-server port are each pinned in 3 places and must move together or something breaks silently: Node in `.nvmrc` + `ci.yml:57,76` + `release.yml:80`; `macos-15` in `ci.yml:28,70` + `release.yml:63` (never `macos-latest` — D34); port `1420` in `vite.config.ts`, `tauri.conf.json`'s `devUrl`, and `playwright.config.ts`.
- Action pinning is per-workflow: `release.yml`/`test.yml` SHA-pin every third-party action; `ci.yml`/`release-please.yml`/`claude*.yml` float tags. Match the file you're editing.
- Actions → Release → Run workflow builds/signs/notarizes and uploads to the run only, unless `attach_to_tag` names an existing tag, in which case it re-uploads to that GitHub Release with `--clobber` (the repair path for a release whose build failed after the tag was cut).

## Conventions that differ from defaults

- No ESLint or Prettier is configured anywhere — match surrounding style by eye; run `cargo fmt` on modified Rust before committing (`cargo fmt --check` is CI-enforced; there's no frontend format gate).
- No path aliases anywhere — use relative imports and `import type` for type-only imports.
- Naming: components/exports PascalCase, hooks `useX`, Zustand stores `useXStore`, helpers lowerCamelCase, constants UPPERCASE.
- IPC enum wire casing is per-enum, not inferable from variant shape — read the `#[serde(rename_all = ...)]` attribute on each one. Exactly five use `lowercase` (`ManagerId`, `ManagedBy`, `ManagerStatus`, `HealthSeverity`, `StreamKind`); `ErrorCode` uses `snake_case`; everything else is `camelCase`. The two struct-variant enums (`SelfUpdateRoute`, `AppUpdateState`) additionally need `rename_all_fields = "camelCase"` or their fields silently ship snake_case with no compile error.
- Focus is always a real `outline` + `outline-offset` in `--color-focus-ring`, never `ring-*` or `outline-none` — WKWebView doesn't paint `box-shadow` on native-appearance controls (checkbox/select), so a ring-based focus state is invisible there even though `:focus-visible` still matches. One deliberate `ring-accent` survivor exists (`PackageRow.tsx:85`, a nav highlight, not focus) — don't repoint it or reuse `ring-*` for focus elsewhere.
- Use narrow Zustand selectors (`useXStore(s => s.field)`); outside React use `useXStore.getState()`. The deliberate exception is derived-array logic (`ManagerPane`, `StatusBar`, `UpdateStatusItem`, `SelfUpdateCard`), which subscribes to a whole store — don't "fix" those into selectors, Zustand 5 has no default shallow equality and it becomes a re-render loop.
- Route caught frontend `unknown` errors through `describeError()` before logging — a failed `invoke()` rejects with a plain object that `String(e)` collapses to `"[object Object]"`. There are zero `console.*` calls in `src/`; use `logFrontendEvent()` instead.

## Known pitfalls

- D27–D30 (a persistent Upgrade Plan, `planAttemptId`, a `Verifying` status, an "Interaction required" UX) are decided but unimplemented — every `epic-ux-pb` story is still `backlog`. `planAttemptId`, `Verifying`, `InteractionRequired`, `skipUpgradePlanConfirmation`, and `PlanIntent` do not exist anywhere in source. A single-package upgrade still executes immediately with no confirmation sheet; the durable coherence token today is only `PlanCoordinator.revision`.
- Window dragging depends on three independent, unguarded pieces that fail silently with no failing test: `titleBarStyle: Overlay` + `data-tauri-drag-region`, the `core:window:allow-start-dragging` capability grant, and two hardcoded `38px` strips in separate files (`AppLayout.tsx`, `Sidebar.tsx`) that must stay equal.
- `refresh_all`/`refresh_manager` must re-run detection first and fan out from the fresh result, never the cached `DetectionOutcome` — this is the fix for a named regression (`brew install mas` then Refresh All left mas stuck "Not installed").
- Plan building applies rust-dedup (D10): if rustup contributes any package, mise's `tool:rust` is removed from the plan — one plan never contains both.
- `HOMEBREW_NO_AUTO_UPDATE=1` is added per-command (every brew command except the explicit `brew update`), not in the base environment.
- `parse_recovery` must merge the recovered overlay into the already-parsed inventory — replacing it drops every up-to-date row from the table.
- `refuse_app_update_while_busy`'s status set (`commands.rs`) must exactly mirror `activeOps` in `src/store/operations.ts` — it's enforced independently in two layers (frontend quit guard + backend command), and changing one without the other lets an app update destroy in-flight work.
- Editing a `theme.css` token value is a coordinated change — `tests/e2e/browser-style-contract.spec.ts` pins several tokens as literal `rgb()` strings. A token edit alone leaves `tsc`/`vitest`/`build` green and only reddens the separate e2e lane.
- `**/.memlog.md` is deliberately tracked, not ignored — it's a BMAD workflow's append-only working memory with no edit/delete tooling. Never rewrite, prune, or gitignore one. The working tree also gains/loses files mid-session from concurrent BMAD workflows — stage by explicit path, never `git add -A`.
- `dev/fixtures/README.md` and `live_smoke.rs` still say this machine has "mas absent" — that's stale documentation, not a code defect; real mas 7.0.0 fixtures exist.

<!-- /bmad:context -->
