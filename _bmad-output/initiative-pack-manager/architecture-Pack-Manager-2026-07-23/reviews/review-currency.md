# Currency & Reality-Check Review — ARCHITECTURE-SPINE.md (revision 4)

**Target:** `_bmad-output/planning-artifacts/architecture/architecture-Pack-Manager-2026-07-23/ARCHITECTURE-SPINE.md`
**Reviewed:** 2026-07-25
**Lens:** Verify every committed decision was web-researched or reality-checked rather than asserted from training data — current library/framework versions, that each named technology still exists and fits, and (brownfield) that the existing project is the reality check. Flag anything that could be out of date and wasn't confirmed against the web, the existing project, or the current starter.

**Verdict: PASS with one high finding.** Every Stack row matches the authoritative lockfiles exactly, every named package is a real, unyanked, published release confirmed against the live npm and crates.io registries, and every count and code-level assertion in the Verified Brownfield Baseline and the AD rules reproduces against the tree. One AD-11 rule asserts an automated accessibility check that does not exist in the repo and was not reality-checked; three lesser items are imprecisions.

---

## 1. Stack table vs. the authoritative lockfiles

The spine states at `ARCHITECTURE-SPINE.md:432-433`:

> "Verified against `package-lock.json` and `src-tauri/Cargo.lock` on 2026-07-25.
> A brownfield seed, not a version policy — the lockfiles own this."

That framing is correct and the table is faithful to it. Every row was resolved from the lockfile itself, not from the `package.json` / `Cargo.toml` ranges.

### 1a. npm — resolved from `package-lock.json`

Command used (per package, reading the `packages["node_modules/<name>"].version` field directly out of the lockfile):

```
node -e 'const l=require("./package-lock.json");console.log(l.packages[process.argv[1]].version)' node_modules/<pkg>
```

| Spine row | Spine value | `package-lock.json` resolves | Match |
| --- | --- | --- | --- |
| Application | 1.0.0 | `packages[""] = 1.0.0` | ✅ |
| Tauri JavaScript API | 2.11.1 | `@tauri-apps/api` 2.11.1 | ✅ |
| Tauri CLI | 2.11.4 | `@tauri-apps/cli` 2.11.4 | ✅ |
| React / React DOM | 19.2.8 | `react` 19.2.8, `react-dom` 19.2.8 | ✅ |
| TypeScript | 7.0.2 | `typescript` 7.0.2 | ✅ |
| Vite | 8.1.5 | `vite` 8.1.5 | ✅ |
| Tailwind CSS | 4.3.3 | `tailwindcss` 4.3.3 | ✅ |
| Zustand | 5.0.14 | `zustand` 5.0.14 | ✅ |
| TanStack React Virtual | 3.14.8 | `@tanstack/react-virtual` 3.14.8 | ✅ |
| Vitest | 4.1.10 | `vitest` 4.1.10 | ✅ |
| Playwright | 1.61.1 | `@playwright/test` 1.61.1, `playwright` 1.61.1 | ✅ |

Note the spine correctly resolved *through* the ranges rather than copying them. `package.json:36` declares `"typescript": "~7.0.2"` and `package.json:37` declares `"vite": "^8.1.5"`; `package.json:24` declares `"react": "^19.1.0"` while the lockfile resolves 19.2.8. The spine reports the resolved values, which is the correct discipline for this table.

### 1b. Rust — resolved from `src-tauri/Cargo.lock`

Command used: `grep -A1 '^name = "<crate>"$' src-tauri/Cargo.lock | grep '^version'`

| Spine row | Spine value | `src-tauri/Cargo.lock` | Match |
| --- | --- | --- | --- |
| Tauri Rust crate | 2.11.5 | `version = "2.11.5"` | ✅ |
| Tauri updater plugin | 2.10.1 | `version = "2.10.1"` | ✅ |
| Tauri opener plugin | 2.5.4 | `version = "2.5.4"` | ✅ |
| Tokio | 1.53.1 | `version = "1.53.1"` | ✅ |

Again resolved, not copied: `src-tauri/Cargo.toml:23` declares `tauri = { version = "2", features = [] }` and `src-tauri/Cargo.toml:44` declares `tauri-plugin-updater = "2"`. The spine reports 2.11.5 / 2.10.1 from the lock.

Rust edition — `src-tauri/Cargo.toml:6` `edition = "2021"`. Spine row "Rust edition | 2021" ✅.

### 1c. The two flagged-as-unusual versions are real and are current

TypeScript 7.0.2 and Vite 8.1.5 were called out as unusual. Both were confirmed against the live npm registry (not from training data):

- `https://registry.npmjs.org/typescript/7.0.2` → exists, `"version":"7.0.2"`, tarball `https://registry.npmjs.org/typescript/-/typescript-7.0.2.tgz`, published by Microsoft. `https://registry.npmjs.org/typescript/latest` → `"version":"7.0.2"`. **7.0.2 is the current `latest` dist-tag**, not a typo for 5.x.
- `https://registry.npmjs.org/vite/8.1.5` → exists, `"version":"8.1.5"`, MIT, engines `^20.19.0 || >=22.12.0`. `https://registry.npmjs.org/vite/latest` → `"version":"8.1.5"`. **8.1.5 is the current `latest` dist-tag.**

Vite 8.1.5's declared engine floor (`^20.19.0 || >=22.12.0`) is satisfied by the pinned CI Node — see §1e. No conflict.

### 1d. Every other named package confirmed to exist upstream

npm (`https://registry.npmjs.org/<pkg>/<version>`), all returned the exact version:
`react@19.2.8` ✅ · `vitest@4.1.10` ✅ · `@playwright/test@1.61.1` ✅ · `tailwindcss@4.3.3` ✅ · `zustand@5.0.14` ✅ · `@tanstack/react-virtual@3.14.8` ✅ · `@tauri-apps/api@2.11.1` ✅ · `@tauri-apps/cli@2.11.4` ✅

crates.io (`https://crates.io/api/v1/crates/<crate>/<version>`), all exist with `"yanked": false`:
`tauri@2.11.5` (`created_at` `2026-07-01T13:56:38.810372Z`) ✅ · `tauri-plugin-updater@2.10.1` ✅ · `tauri-plugin-opener@2.5.4` ✅ · `tokio@1.53.1` ✅

Staleness check: `https://crates.io/api/v1/crates/tauri` returns `max_stable_version` `"2.11.5"` and `newest_version` `"2.11.5"` — **the pinned Tauri is the newest stable release**, published 24 days before this review. Nothing in the stack is a stale pin masquerading as current, and no named technology has been retired or renamed.

### 1e. Remaining Stack rows

| Row | Spine value | Repo evidence | Match |
| --- | --- | --- | --- |
| Node in CI | 24 | `.github/workflows/ci.yml:54` and `:73` `node-version: 24`; `.github/workflows/release.yml:79` `node-version: 24`; `.github/workflows/test.yml:40,72,127,171` `node-version-file: .nvmrc` with `.nvmrc` = `24` | ✅ |
| Minimum supported macOS | 15.0 | `src-tauri/tauri.conf.json` `"macOS": { "minimumSystemVersion": "15.0" }` | ✅ |
| Release automation | release-please action v5 + GitHub Actions | `.github/workflows/release-please.yml` `uses: googleapis/release-please-action@v5` (twice: the PR step and the `rp_after_merge` step) | ✅ |
| CI runner image | macos-14 | Partially — see Finding 2 | ⚠️ |

---

## 2. "Verified Brownfield Baseline" claims vs. the code

Every claim in `ARCHITECTURE-SPINE.md:67-102` was reproduced against the tree.

### 2a. 20 commands (`ARCHITECTURE-SPINE.md:72-73`)

Command: `sed -n '233,252p' src-tauri/src/lib.rs | grep -c "commands::"` → **20**

The `invoke_handler` block runs `src-tauri/src/lib.rs:232` `.invoke_handler(tauri::generate_handler![` through `:253` `])`, registering `detect_managers, get_state, refresh_manager, refresh_all, build_upgrade_plan, execute_plan, self_update_manager, run_health_fix, cancel_operation, get_operation, list_operations, get_settings, set_settings, reveal_operation_log, reveal_logs_dir, export_diagnostics, log_frontend_event, get_app_update_state, check_for_app_update, install_app_update`. **✅ 20.**

### 2b. Six typed events (`ARCHITECTURE-SPINE.md:73`)

`src-tauri/src/events.rs:75-80` declares exactly six name constants, and `:83` comments `/// One of the six events, name + typed payload.` The `AppEvent` enum at `:84-93` has six variants: `DetectionUpdated`, `SnapshotUpdated`, `OpStatus`, `OpOutput`, `OpStalled`, `AppUpdateStatus`. **✅ six.**

Matching frontend subscriptions: `src/lib/ipc/events.ts:149-156` inside `subscribeEvents()` passes six `listen<…>(…)` calls to `Promise.allSettled` — `EVENT_DETECTION_UPDATED`, `EVENT_SNAPSHOT_UPDATED`, `EVENT_OP_STATUS`, `EVENT_OP_OUTPUT`, `EVENT_OP_STALLED`, `EVENT_APP_UPDATE_STATUS`. **✅ six.** (A stale doc comment in that same file says "five" — Finding 4.)

Matching frontend wrappers: `src/lib/ipc/client.ts` has 20 `invoke<…>("<command>")` calls at lines 29, 34, 39, 44, 49, 54, 59, 64, 69, 74, 79, 83, 88, 93, 98, 103, 108, 117, 125, 133 — one per registered command. **✅ 1:1.**

### 2c. `bridge.ts` is the sole frontend Tauri importer (`ARCHITECTURE-SPINE.md:74-75`, restated as a rule at `:138-139`)

Command: `grep -rl "@tauri-apps" src/ --include="*.ts" --include="*.tsx" | wc -l` → **1**, and it is `src/lib/ipc/bridge.ts`.

The file is 11 lines and re-exports exactly the three named symbols:
- `src/lib/ipc/bridge.ts:9` `export { invoke } from "@tauri-apps/api/core";`
- `:10` `export { listen } from "@tauri-apps/api/event";`
- `:11` `export type { UnlistenFn } from "@tauri-apps/api/event";`

Its own header at `:2` says `* bridge.ts — the SINGLE importer of \`@tauri-apps/api\` in the whole frontend.` **✅** — including AD-3's stricter wording "re-exporting exactly `invoke`, `listen`, and `UnlistenFn`".

### 2d. Startup subscribes before hydration (`ARCHITECTURE-SPINE.md:75-76`, restated at `:157-158`)

`src/App.tsx:56-57`:
> "// Subscribe BEFORE hydrating: `detection:updated` is emitted only after the
> // backend stores detection, so with listeners registered first a `get_state`"

and `src/App.tsx:26` `// \`get_state\` was in flight) with the pre-detection placeholder.` — which also grounds the spine's "a real detection report is never clobbered by the pre-detection placeholder" (`:158-159`). **✅**

### 2e. 15 committed contract fixtures (`ARCHITECTURE-SPINE.md:76-78`)

Command: `ls dev/fixtures/ipc/ | wc -l` → **15**; `ls dev/fixtures/ipc/*.json | wc -l` → **15** (all 15 are `.json`, no stragglers).

Files: `app_state, detection_report, event_app_update_status, event_op_output, event_op_stalled, event_op_status, event_snapshot_updated, ipc_error, manager_snapshot, op_ref, operation_detail, operation_record, plan_request, settings, upgrade_plan` (`.json`). **✅ 15.**

Byte-compare + round-trip, exactly as the spine describes at `:77-78` ("byte-compares each serialized model against its fixture and round-trips the committed bytes back through `Deserialize`"):
- `src-tauri/src/ipc.rs:560-561` `/// Serializes \`value\` pretty + trailing newline and compares byte-for-byte` / `/// with the committed fixture; also proves the fixture deserializes back.`
- `:573-576` the `assert_eq!(committed, rendered, "IPC contract drift for {name} …")`
- `:580-582` `// The committed bytes must also round-trip through Deserialize.` / `let _: T = serde_json::from_str(&committed)`

**✅ both halves.**

### 2f. Process-runner safety floor (`ARCHITECTURE-SPINE.md:79-83`, restated as AD-4's rule at `:171-176`)

`src-tauri/src/process/runner.rs:3-6` header:
> "`RealRunner`: `tokio::process::Command` with `.process_group(0)`,
> `.stdin(Stdio::null())`, line readers with `\r` split, ANSI stripping and
> … timeout, and SIGTERM → 5s grace → SIGKILL via `nix::killpg`."

Concretely: `:299` `.env_clear()`, `:301` `.stdin(Stdio::null()) // no sudo, no password entry, ever`, `:304` `.process_group(0)`, `:261` `let _ = killpg(pgid, Signal::SIGTERM);`, `:270` `let _ = killpg(pgid, Signal::SIGKILL);`. Bounded output: `src-tauri/src/queue.rs:52` `pub const RING_CAP: usize = 5000;`. **✅ every element the spine lists.**

### 2g. Test-topology claim (`ARCHITECTURE-SPINE.md:84-86`)

- "Rust tests construct handlers below Tauri" — `src-tauri/src/commands.rs:892` `let plan_coordinator = Arc::new(Mutex::new(crate::state::PlanCoordinator::default()));` builds the handler state directly in the test harness. **✅**
- "browser tests replace the Tauri bridge with an in-browser double" — `src/test/fakeIpc.ts` exists; `tests/e2e/framework-contract.spec.ts:21,36,102` each do `const bridgeModulePath = "/src/lib/ipc/bridge.ts";` and dynamically import `invoke` / `listen` from it in-page. **✅**
- "No native harness exists" — `grep -rn "tauri-driver\|webdriver\|WebDriver" package.json .github/workflows/ tests/` → **0 hits**. **✅**

### 2h. Upgrade Plan is transient dialog state (`ARCHITECTURE-SPINE.md:87-92`)

- `ui.dialog` `{ kind: "upgradePlan" }` — `src/store/ui.ts:20` `| { kind: "upgradePlan"; plan: UpgradePlan }`. **✅**
- "discarded by `closeDialog`" — `src/store/ui.ts:116` `closeDialog: () => set({ dialog: { kind: "none" } }),`. **✅**
- "the durable token is a monotonic `revision` in `PlanCoordinator`" — `src-tauri/src/state.rs:51` `pub struct PlanCoordinator {`, `:52` `revision: u64,`, `:58-60` `pub fn revision(&self) -> u64 { self.revision }`, `:62-67` `pub fn bump_revision(&mut self)` using a checked increment that `.expect("plan revision exhausted")`. **✅ monotonic.**
- "no `planAttemptId`, `Verifying`, or `InteractionRequired` symbol exists in `src/` or `src-tauri/src/`" — counted with `grep -rc … | awk -F: '{s+=$2} END{print s+0}'` across both trees:

  | Symbol | `src/` | `src-tauri/src/` |
  | --- | --- | --- |
  | `planAttemptId` / `plan_attempt_id` | 0 | 0 |
  | `Verifying` | 0 | 0 |
  | `InteractionRequired` | 0 | 0 |

  **✅ all zero.** (Also 0 for the case variants `interactionRequired` and `"Interaction required"`.)
- "`autoOpenDrawer` is still an active setting" — 13 hits in `src/`, 9 in `src-tauri/src/`. Live product paths, not just tests: `src/components/settings/SettingsView.tsx:134` `checked={settings.autoOpenDrawer}` and `:135` `onChange={(v) => void patch({ autoOpenDrawer: v })}`; `src/components/activity/useOperationEffects.ts:53` `if (mutating && ui.settings?.autoOpenDrawer) {`; `src/lib/ipc/types.ts:274` `autoOpenDrawer: boolean;`; `src-tauri/src/settings.rs:34` `pub auto_open_drawer: bool,` with `:49` `auto_open_drawer: true,` as the default. It is also in the committed wire contract at `dev/fixtures/ipc/settings.json:7` and `dev/fixtures/ipc/app_state.json:130`. **✅ active.**

### 2i. Persistence (`ARCHITECTURE-SPINE.md:93-96`)

- `settings.json` atomic replace — `src-tauri/src/settings.rs:124` `/// Atomic: temp file + fsync + rename, so a crash or full disk mid-write`, implemented at `:139-145` (`with_file_name(format!("{file_name}.tmp"))` → `File::create` → `sync_all()` → `rename`). **✅**
- `operations.jsonl` append-only, compacted to the newest 1,000 — `src-tauri/src/journal.rs:18-19` `/// Operations kept by startup compaction.` / `pub const COMPACT_KEEP: usize = 1000;`. **✅ 1,000 exactly.**
- "temp file + fsync + rename" — `src-tauri/src/journal.rs:213-215` `/// Write-to-temp + fsync + rename in the target's own directory (rename is` / `/// atomic on the same filesystem). On failure the temp file is best-effort` / `/// removed and the original is left untouched.`, implemented `:222-226`. `:179-183` explicitly rejects truncate-in-place. **✅** — this also grounds AD-18's rule at `:387-389`.
- Diagnostics ships `report.json`, newest three app logs, newest 25 transcripts, `operations.jsonl` — `src-tauri/src/diagnostics.rs:22` `pub const APP_LOGS_INCLUDED: usize = 3;`, `:23` `pub const TRANSCRIPTS_INCLUDED: usize = 25;`, `:129` `zip.start_file("report.json", …)`, `:144` `add_file(&mut zip, "operations.jsonl", journal_path)?;`. Asserted in-test at `:240` `assert_eq!(log_names.len(), 3, "last 3 app logs");` and `:251` `assert_eq!(transcript_names.len(), 25, "last 25 transcripts");`. **✅ 3 and 25 exactly.**

### 2j. `release.yml`'s two blocking checks (`ARCHITECTURE-SPINE.md:97-100`, restated as AD-11's rule at `:209-212`)

Check 1 — minisign verification against the embedded pubkey:
- `.github/workflows/release.yml:314` `base64 -d < "$UPDATER_TGZ.sig" > "$RUNNER_TEMP/updater.minisig"`
- `:315` `jq -r '.plugins.updater.pubkey' src-tauri/tauri.conf.json | base64 -d > "$RUNNER_TEMP/updater.pub"`
- `:318-319` `minisign -V -p "$RUNNER_TEMP/updater.pub" -x "$RUNNER_TEMP/updater.minisig" -m "$UPDATER_TGZ" \` / `|| { echo "::error::updater signature does not verify against the configured pubkey"; exit 1; }`

The spine's precise wording "base64-decoded and verified with `minisign` against the public key the shipping app embeds" matches the code including the base64 unwrap step, which `:305-312` documents as the trap that shipped v1.0.0 with zero assets. **✅**

Check 2 — published `latest.json` reachable and coherent:
- `.github/workflows/release.yml:382` `- name: Verify published updater endpoint` (guarded `if: inputs.attach_to_tag != ''`)
- `:386-387` fetches `https://github.com/${GITHUB_REPOSITORY}/releases/latest/download/latest.json`
- `:389-391` `if [ "$PUBLISHED" != "$VERSION" ]; then echo "::error::latest.json reports $PUBLISHED, expected $VERSION"; exit 1; fi`
- `:392-394` asserts the `darwin-aarch64` asset URL is reachable

**✅** — and the spine is careful to say "asserted reachable and coherent **after upload**" (`:212`), which matches `:377-379`'s comment that `latest.json` is written before the asset it references is uploaded. The spine did not overstate this as a pre-publication gate.

### 2k. Version and macOS floor (`ARCHITECTURE-SPINE.md:101-102`)

`package.json:3` `"version": "1.0.0"`, `src-tauri/Cargo.toml:3` `version = "1.0.0"`, `src-tauri/tauri.conf.json` `"version": "1.0.0"`, `.release-please-manifest.json` `{".":"1.0.0"}`. All four agree. `src-tauri/tauri.conf.json` `"minimumSystemVersion": "15.0"`. **✅**

---

## 3. Code-level assertions inside the AD rules

| AD | Assertion (`ARCHITECTURE-SPINE.md` line) | Evidence | Verdict |
| --- | --- | --- | --- |
| AD-3 | `bridge.ts` sole Tauri importer, re-exports exactly three symbols (`:138-140`) | §2c | ✅ |
| AD-3 | "Argument-taking commands wrap payloads as `{ args: ... }`; no-argument commands omit the payload entirely" (`:141-142`) | `src/lib/ipc/client.ts` — 11 arg-taking calls all use `{ args: … }` (`:39,44,49,54,59,64,69,74,79,88,93,108`); 9 no-arg calls pass no second parameter (`:29,34,44→refresh_all,83,98,103,117,125,133`). `src/lib/ipc/client.ts:7` documents it: "`args` key: `invoke(\"refresh_manager\", { args: { managerId } })`" | ✅ |
| AD-3 | "The enforcing mechanism is the shipping contract test … the TypeScript half asserts its fixture set exactly equals its guard map" (`:145-148`) | `src/lib/ipc/types.test.ts:57-60` `it("covers exactly the committed fixture set", () => { const found = Object.keys(FIXTURES).map(baseName).sort(); expect(found).toEqual(Object.keys(GUARDS).sort()); });` — set equality, not subset | ✅ |
| AD-3 | "Regenerate only with `PM_UPDATE_CONTRACT=1 cargo test ipc_contract`" (`:149-150`) | `src-tauri/src/ipc.rs:546` `// Regenerate with \`PM_UPDATE_CONTRACT=1 cargo test ipc_contract\`.`; `:566` `if std::env::var("PM_UPDATE_CONTRACT").is_ok() {` → writes the fixture and returns; test fn `:788` `fn ipc_contract_matches_committed_fixtures()`. Cross-referenced in `README.md:120`, `docs/development-guide.md:154`, `docs/SPEC.md:774` | ✅ |
| AD-3 | "There is no separate versioned boundary-catalog file and none is to be created" (`:155-156`); front-matter `:37-38` "a `contracts/` directory that has never existed" | `ls -d contracts` → `No such file or directory`; `find . -type d -name contracts -not -path "./node_modules/*"` → 0 hits. Corroborated by `docs/DECISIONS.md:328-329` "against a `contracts/` directory that does not exist and that no story creates" | ✅ |
| AD-4 | "`CommandRunner`, `EventSink`, `UpdateSource`, and `PendingRelease` are existing ports" (`:169-170`) | `src-tauri/src/process/runner.rs:26` `pub trait CommandRunner: Send + Sync {`; `src-tauri/src/events.rs:124` `pub trait EventSink: Send + Sync {`; `src-tauri/src/app_update.rs:41` `pub trait UpdateSource: Send + Sync {`; `:48` `pub trait PendingRelease: Send + Sync {` | ✅ all four |
| AD-4 | Coordinator-first lock order (`:177-180`) | `src-tauri/src/commands.rs:74` `/// Caller must hold \`state.plan_coordinator\`; every writer follows the same`; `src-tauri/src/state.rs:46` `/// issuance, and atomic plan submission all take this mutex. The revision is` | ✅ |
| AD-4 | "concurrency cap of 4, the 120s aging guard, and duplicate-refresh coalescing" (`:184-185`) | `src-tauri/src/queue.rs:48` `pub const MAX_CONCURRENCY: usize = 4;`; `:50` `pub const AGING_GUARD: Duration = Duration::from_secs(120);`; `:6-7` "waited longer than the aging guard (120s). Global `Semaphore(4)`. / Duplicate `refresh_manager` submissions coalesce to the existing opId." Tests: `:2398` `async fn semaphore_caps_concurrency_at_4()`, `:2329` `async fn aging_guard_blocks_skip_ahead_after_120s()`, `:2440` `async fn duplicate_refresh_coalesces_to_same_opid()` | ✅ |
| AD-4 | "plus the Mise lock when a mise-managed npm or uv is the executor" (`:183-184`) | `src-tauri/src/queue.rs:10` `//! subject; npm/uv ops add \`Mise\` when the executor binary is mise-managed.`; `:60` `/// mise-managed (shared-tree guard).` | ✅ |
| AD-5 | "Historical PGIDs … are data only and are never signaled after a restart … An unfinished start is reconstructed as Interrupted instead" (`:195-197`) | `src-tauri/src/journal.rs:32-33` `/// Informational only — never signaled on startup (pid reuse). \`0\` when`; `:5` `//! finish renders \`Interrupted\` on the next launch. Recorded pgids are NEVER`; `:147` `/// otherwise \`Interrupted\` (start-without-finish — SPEC F8).`; `:154` `None => (OpStatus::Interrupted, None, None),` | ✅ |
| AD-5 | "Diagnostics must reject symlinks both when selecting and when streaming files" (`:198-199`) | Selecting: `src-tauri/src/diagnostics.rs:50-51` `/// Only REGULAR files qualify: \`DirEntry::file_type()\` does not follow / /// symlinks.`, `:63` `let is_regular = e.file_type().map(|t| t.is_file()).unwrap_or(false);`. Streaming: `:72` `/// \`true\` only for a REGULAR file at \`path\` itself (symlinks excluded —`, `:75` `std::fs::symlink_metadata(path)`, `:81` `/// path is a regular file (not a symlink) at read time, and \`io::copy\``. Regression test `:274` `fn export_never_follows_symlinks_into_the_bundle()` | ✅ both sides |
| AD-11 | "Minimum supported macOS is 15.0 at `bundle.macOS.minimumSystemVersion`" (`:219-220`) | `src-tauri/tauri.conf.json` `"minimumSystemVersion": "15.0"`; `docs/DECISIONS.md:245` `## D31. Minimum supported macOS is 15.0` | ✅ |
| AD-11 | "CI stays on `macos-14`" (`:221`) | Partially — Finding 2 | ⚠️ |
| AD-11 | "the x86_64 key is never dropped … `latest.json` publishes both `darwin-aarch64` and `darwin-x86_64`" (`:213-216`) | `.github/workflows/release.yml:339-341` emits both keys pointing at the same `$ASSET_URL`; `:321-325` documents exactly the `cfg!(target_arch)` reasoning the spine gives. `docs/DECISIONS.md:289-292` (D32) corroborates | ✅ |
| AD-11 | Automated 4.5:1 contrast + reduced-motion checks in the Playwright/Vitest lane (`:223-225`) | **Contrast does not exist** — Finding 1 | ❌ |
| AD-11 | "one manual VoiceOver focus-order and completion-announcement pass on the release checklist" (`:225-226`) | Partially — Finding 3 | ⚠️ |
| AD-12 | "Seven files are release-please-owned" (`:237-240`) | `release-please-config.json` `"release-type": "node"` (owns `package.json` + `package-lock.json`), `"changelog-path": "CHANGELOG.md"`, and three `extra-files`: `src-tauri/tauri.conf.json`, `src-tauri/Cargo.toml`, `src-tauri/Cargo.lock`. `.release-please-manifest.json` is the action's default manifest (`.github/workflows/release-please.yml:64-65` "Config + last-released version live in release-please-config.json and / .release-please-manifest.json (the action's default filenames)"). **7 files.** Matches `CLAUDE.md`'s "five files" plus the two release-please-owned outputs | ✅ |
| AD-12 | "A conventional commit reaching `main` enters release automation with no later human gate" (`:234-236`) | `.github/workflows/release-please.yml:14-17` "That PR is squash-merged automatically by the step below — there is NO / human gate. Any `fix:` or `feat:` reaching main ships a public, / signed, notarized release without further approval." | ✅ |
| AD-12 | "Apple Developer ID signing and notarization are required for a published release" (`:241-242`) | `docs/DECISIONS.md:81` D20 says notarization is *out of scope* — but it is explicitly superseded at `docs/DECISIONS.md:144-147`: "**D20 is superseded.** It said notarization was out of scope; `release.yml` has notarized / since, and the updater depends on it — an un-stapled auto-update would install an app / that phones Apple on first launch." The spine took the current position, not the stale record | ✅ |
| AD-12 | "The manual workflow-dispatch path publishes nothing only when `attach_to_tag` is empty" (`:242-244`) | `.github/workflows/release.yml:363` and `:382` both gate on `if: inputs.attach_to_tag != ''`; `:379-380` "Manual runs / publish nothing, so there is no endpoint for them to check." Matches `CLAUDE.md`'s "it never touches a GitHub Release" | ✅ |
| AD-17 | "`DialogHost` remains the single mount point for modal surfaces" (`:376-377`) | `src/components/dialogs/DialogHost.tsx:15` `case "upgradePlan":`; `src/components/dialogs/UpgradePlanSheet.tsx:9` `* Mounted by DialogHost (U8) for \`ui.dialog.kind === "upgradePlan"\`, receiving` | ✅ |
| AD-19 | "A corrupt file degrades to defaults with a visible notice — the shipping behavior for `settings.json`" (`:406-407`) | `src-tauri/src/settings.rs:27` `#[serde(rename_all = "camelCase", default)]` — `default` is what tolerates unknown/missing fields on read | ✅ |

### Scope/binding claims

- "Epic UX-PB (28 stories)" (front-matter `:13`, AD-16 `:248`) — `grep -oh "UX-PB\.[0-9][a-z]" … | sort -u | wc -l` → **28** distinct ids (`1a–1e`, `2a–2f`, `3a–3g`, `4a–4e`, `5a–5e` = 5+6+7+5+5 = 28), each with a `### Story UX-PB.…` heading in `_bmad-output/planning-artifacts/epics.md:384-1008`. **✅**
- "Six stories survive — 2.2, 3.1, 3.2, 3.4, 3.5, 6.5" and "Epics 1, 4, and 5 were removed; 31 stories archived" (`:499`) — `epics.md` contains headings for exactly Epic UX-PB (`:378`), Epic 2 (`:1011`), Epic 3 (`:1039`), Epic 6 (`:1137`) and exactly six non-UX-PB stories: `2.2` (`:1015`), `3.1` (`:1043`), `3.2` (`:1066`), `3.4` (`:1088`), `3.5` (`:1112`), `6.5` (`:1141`). `docs/DECISIONS.md:352-354` gives "6 keep, 19 merge, 12 retire across the 37 stories" — 37 − 6 = **31 archived**. **✅ arithmetic and membership both check out.**
- The `epics.md` retired register, flagged Open at `:505` — `grep -oh "TIR-[0-9]*" … | sort -uV` → `TIR-1 … TIR-8` (**8**); `grep -oh "\bRE-[0-9]*" … | sort -uV` → `RE-1 … RE-11` (**11**); `grep -c "tauri-boundary/v1.json"` → **1**, at `epics.md:150` "exact set equality across the versioned `contracts/tauri-boundary/v1.json` catalog"; the 72-criterion controls survive at `epics.md:104, 105, 165`. **✅ the spine's open-drift entry is exact, including the id ranges.**
- `DRIFT-NOTE.md` — referenced at `:39` and `:505`. Present: `_bmad-output/planning-artifacts/architecture/architecture-Pack-Manager-2026-07-23/DRIFT-NOTE.md` (8,781 bytes). **✅**
- Retired ids AD-6..AD-10, AD-13..AD-15 are absent from the document body; the surviving set is AD-1..AD-5, AD-11, AD-12, AD-16..AD-19. **✅ no id reuse.**

---

## Findings

### Finding 1 — HIGH — AD-11 asserts an automated 4.5:1 contrast check that does not exist

`ARCHITECTURE-SPINE.md:223-225`:

> "- **Rule:** Accessibility is product quality in the existing lanes — automated
>   4.5:1 text contrast and reduced-motion checks in the Playwright/Vitest lane,
>   and one manual VoiceOver focus-order and completion-announcement pass on the
>   release checklist."

The reduced-motion half is real. `tests/e2e/browser-style-contract.spec.ts:8` is `test("[P0] AUT-004 applies dark tokens, keyboard focus treatment, and reduced-motion suppression"`, and `:46` does `await page.emulateMedia({ reducedMotion: "reduce" });` before asserting suppression at `:108-113`.

The contrast half does not exist anywhere in the repo:

```
grep -rn "contrastRatio\|contrast_ratio\|4\.5:1\|luminance" tests/ src/   →  0 hits
```

And the one test that touches the styling contract explicitly disclaims it — `tests/e2e/browser-style-contract.spec.ts:116-117`:

> "    // This is a browser DOM/CSS contract only. It does not claim measured
>     // contrast compliance or validate the native Tauri package."

The spine inherited this from a source document that is itself wrong. `docs/RELEASE-CHECKLIST.md:91-92`:

> "   Automated contrast (4.5:1) and reduced-motion checks run in the Playwright/Vitest lane
>    and need no manual step."

So the chain is: the checklist tells the release operator contrast needs no manual step → AD-11 restates that as the architectural rule for where accessibility lives → nothing anywhere measures contrast. This is precisely the failure this lens exists to catch: an assertion about existing tooling carried forward from a document rather than reality-checked against the code. It is load-bearing because AD-11 is the release-acceptance rule, and `docs/DECISIONS.md:341-343` (D33) phrased the same content *prescriptively* — "Automated 4.5:1 contrast and reduced-motion checks **belong in** the existing Playwright/Vitest lane" — which the spine converted into a description of what is there.

**Recommendation:** either (a) reword AD-11 to mark contrast as an obligation not yet discharged, and correct `docs/RELEASE-CHECKLIST.md:91-92` so the operator knows contrast is currently unchecked, or (b) treat the missing check as product work under AD-1's second rule. Do not leave the checklist claiming "no manual step" for a check that does not run.

### Finding 2 — MEDIUM — "CI runner image: macos-14" describes 3 of 8 CI jobs; the other 5 run on `ubuntu-latest`

Stack row at `ARCHITECTURE-SPINE.md:454` `| CI runner image | macos-14 |`, restated as an AD-11 rule at `:221-222` "CI stays on `macos-14`".

Actual runner distribution (`grep -n "runs-on:" .github/workflows/ci.yml .github/workflows/test.yml`):

| Workflow:line | Job | Runner |
| --- | --- | --- |
| `ci.yml:25` | `rust` | `macos-14` |
| `ci.yml:49` | `web` | `ubuntu-latest` |
| `ci.yml:67` | `build-smoke` | `macos-14` |
| `test.yml:30` | `lint` | `ubuntu-latest` |
| `test.yml:56` | `test` | `ubuntu-latest` |
| `test.yml:117` | `burn-in` | `ubuntu-latest` |
| `test.yml:161` | `report` | `ubuntu-latest` |
| `release.yml:62` | build | `macos-14` |

The claim is true of every macOS lane and false as a description of CI overall. A single-valued Stack row invites a builder to assume the Playwright/Vitest lane — the very lane AD-11 assigns the accessibility checks to — runs on macos-14, when it runs on Ubuntu. That interaction with Finding 1 is why this is medium rather than low.

**Recommendation:** make the row "macOS lanes: macos-14; web/test lanes: ubuntu-latest", and scope AD-11's rule to "the macOS build lanes stay on `macos-14`".

### Finding 3 — LOW — AD-11 attributes "focus-order" to a VoiceOver checklist item that covers only announcements

`ARCHITECTURE-SPINE.md:225-226` says "one manual VoiceOver **focus-order and completion-announcement** pass on the release checklist."

`docs/RELEASE-CHECKLIST.md:89` reads:

> "   One VoiceOver pass over the Upgrade Plan announces state changes and completion."

Focus order is covered — but by a separate, non-VoiceOver sentence at `docs/RELEASE-CHECKLIST.md:86`: "**Keyboard and accessibility pass.** Tab and arrow navigation reach every control." The spine merges two checklist obligations into one and attributes both to the VoiceOver pass. Harmless to the architecture; misleading if someone edits the checklist against the spine.

### Finding 4 — LOW — stale "five backend events" comment in `src/lib/ipc/events.ts` (source-code drift, not a spine defect)

The spine's "six typed events" is correct and `src/lib/ipc/events.ts:149-156` subscribes six. But that file's own header at `:4-5` says:

> " * `subscribeEvents()` is called once from App mount; it wires the five backend
>  * events to the stores and returns an unlisten."

`appUpdate:status` was added (per `docs/DECISIONS.md:118` D25) without updating the count in the comment. Noted so the AD-3 "one deliberate surface change moves as one change" rule (`:143-145`) picks it up on the next IPC change — the spine is right and the code comment is wrong.

---

## What I could not confirm

Nothing material. Two notes on limits of this pass:

1. I verified package existence and version resolution against the live npm and crates.io registries, and lockfile agreement — I did not install or build. `cargo test --locked` / `npm ci` were not run, so "the lockfile resolves X" is a lockfile fact, not a build-succeeds fact.
2. `ARCHITECTURE-SPINE.md:69-70` scopes the baseline honestly — "Starting conditions, not a claim of completeness" — and `:80-83` volunteers what is *not* yet behind a port ("Opener, reveal, restart, current-executable, bundle-parent writability, and some path/time behavior are still direct OS calls"). I spot-checked that this is a real limitation rather than a hedge and found it accurate; I did not exhaustively enumerate every remaining direct OS call.

## Summary

The spine's version discipline is sound and demonstrably was reality-checked rather than recalled: every Stack row resolves through the lockfiles rather than copying the `package.json`/`Cargo.toml` ranges (React reported as 19.2.8 against a `^19.1.0` range; Tauri as 2.11.5 against a `"2"` range), the two versions flagged as suspicious — TypeScript 7.0.2 and Vite 8.1.5 — are both real and are both the current `latest` dist-tag, and the pinned Tauri crate is the newest stable release on crates.io. Every count in the Verified Brownfield Baseline reproduces exactly (20 commands, 6 events, 15 fixtures, 1000/3/25 retention bounds, cap 4, 120s), and every absence claim is genuinely zero. The one substantive failure is AD-11's automated-contrast assertion, which was carried forward from `docs/RELEASE-CHECKLIST.md` without being checked against a codebase that contains no contrast assertion at all and a test that explicitly disclaims one.
