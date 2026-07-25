# Currency / Reality-Check Review — ARCHITECTURE-SPINE.md revision 6

**Target:** `_bmad-output/planning-artifacts/architecture/architecture-Pack-Manager-2026-07-23/ARCHITECTURE-SPINE.md`
(revision 6, status `final`, `updated: "2026-07-25"`)
**Reviewed:** 2026-07-25
**Lens:** Verify every committed decision was web-researched or reality-checked
rather than asserted from training data — current library/framework versions,
that each named technology still exists and fits, and the live defaults of
anything the project leans on. Flag anything that could be out of date and was
not confirmed against the web, the existing project, or the current starter.

**Project is BROWNFIELD.** There is no starter. The authoritative reality check
is the repository itself: `package-lock.json` and `src-tauri/Cargo.lock` for
resolved versions, and the tree for every baseline claim. Every finding below is
grounded in a `path:line` citation with a literal quote, and every count states
the exact command that produced it.

**Verdict:** The spine is unusually well grounded — 47 of 49 checkable claims
verify exactly against the tree, including the two version rows most likely to
be "corrected" from stale memory. Two currency defects, both in the *provenance*
layer rather than the invariants: one falsified version claim carrying a
"Verified" banner, and one line-number citation that has already drifted.

---

## Findings

### HIGH-1 — The application version is 1.0.1, not 1.0.0, and the file the spine names as its authority says so

The spine asserts the version twice, under two explicit verification banners.

`ARCHITECTURE-SPINE.md:75`:

> `Verified against the tree on 2026-07-25. Starting conditions, not a claim of`

`ARCHITECTURE-SPINE.md:107`:

> `- Application version is 1.0.0; minimum supported macOS is 15.0 at`

`ARCHITECTURE-SPINE.md:573`:

> ``Verified against `package-lock.json` and `src-tauri/Cargo.lock` on 2026-07-25.``

`ARCHITECTURE-SPINE.md:578`:

> `| Application | 1.0.0 |`

Every version file in the tree disagrees:

- `package.json:4` — `  "version": "1.0.1",`
- `src-tauri/tauri.conf.json:4` — `  "version": "1.0.1",`
- `src-tauri/Cargo.toml:3` — `version = "1.0.1"`
- `.release-please-manifest.json` — `{".":"1.0.1"}`
- `package-lock.json`, root package entry — `1.0.1`
  (command: `node -e 'console.log(require("./package-lock.json").packages[""].version)'`)

`git show HEAD:src-tauri/tauri.conf.json | grep -n '"version"'` → `4:  "version": "1.0.1",`.
This is not an uncommitted local edit; `main` carries 1.0.1.

**Timeline.** The bump is commit `8a4cf6a`:

```
$ git log -1 --format='%H%n%ad%n%s' --date=local 8a4cf6a
8a4cf6a8ff3d2688f912c68438166ec838c6bb17
Sat Jul 25 02:46:52 2026
chore(main): release 1.0.1 (#34)
```

`ls -la` on the run folder reports `ARCHITECTURE-SPINE.md` last modified
`Jul 25 02:47`. The release landed roughly one minute before revision 6 was
written. `CHANGELOG.md:3` confirms the release is real:

> `## [1.0.1](https://github.com/Sallvainian/Pack-Manager/compare/v1.0.0...v1.0.1) (2026-07-25)`

`git diff` on the spine shows both lines 107 and 578 are **added** lines in this
working-tree revision, so the claim was authored — not merely inherited — after
1.0.1 was already on `main`.

**Why HIGH and not MEDIUM.** The number itself is inert: the Stack table
correctly disclaims itself at `ARCHITECTURE-SPINE.md:574` —

> ``A brownfield seed, not a version policy — the lockfile owns this.``

— and no invariant depends on the digit. What is damaged is the verification
contract. The spine names `package-lock.json` as its authority and then states
the one value that file most directly contradicts, in the table's very first
row, under a same-day "Verified against" banner. A builder who spot-checks that
row loses their reason to trust the other twenty. AD-12 additionally elevates
version drift to a governed concern (`ARCHITECTURE-SPINE.md:266`: `Prevents: a
hand-edited version drifting between the five files that must agree`), which
makes a stale version claim in the spine itself the wrong kind of error to
carry.

**Fix.** Change both to `1.0.1`. Better: drop the digit from line 107 and
replace row 578 with a pointer to `.release-please-manifest.json`. The version
is release-please-owned (AD-12), so it will be stale again on the next merge to
`main` — a spine that restates it has signed up to be wrong on a cadence it does
not control.

---

### MEDIUM-1 — The `epics.md:437-439` citation was correct when written and is already wrong against the working tree

`ARCHITECTURE-SPINE.md:646`, the Draft-durability row of the Deferred table:

> ``| Draft durability | **RESOLVED** | Fail-to-empty. The draft is session-scoped and never persisted; every relaunch starts empty. `epics.md:437-439` permits this branch explicitly. Decided 2026-07-25; closes the assumption revision 5 carried. |``

**The substance is true.** The permitting text exists and says what the spine
says it says (working tree, `_bmad-output/planning-artifacts/epics.md:523`):

> `**Then** the draft's canonical membership is reconstructed into the sidecar, or — if it cannot be recovered — the sidecar returns to empty with no fabricated membership and nothing executes; a draft is never surfaced as Activity or History.`

AD-17 at `ARCHITECTURE-SPINE.md:452` takes exactly that second branch —
`This takes the second branch of UX-PB.1b's recovery criterion unconditionally`
— so the decision is soundly grounded.

**The citation is not.** Against `HEAD`, lines 437-439 are precisely the
Given/When/Then block:

```
$ git show HEAD:_bmad-output/planning-artifacts/epics.md | sed -n '437,439p'
**Given** an in-progress draft when the app crashes or is force-quit
**When** Pack-Manager relaunches
**Then** the draft's canonical membership is reconstructed into the sidecar, or — if it cannot be recovered — the sidecar returns to empty with no fabricated membership and nothing executes; a draft is never surfaced as Activity or History.
```

Against the working tree, `epics.md` is modified (`git status`: ` M
_bmad-output/planning-artifacts/epics.md`) and the same block now sits at
lines 521-523. Line 437 of the current file is instead:

> `### Epic 6: Preserve State, Evidence, and Privacy Across Failure and Relaunch`

and line 439:

> `Users can reconstruct Operations after crashes, trust Settings and durable stores across failure, reveal native evidence safely, and export exact diagnostics without inherited-environment disclosure or hostile-path traversal.`

Anyone following the citation today lands on Epic 6's heading and goal
statement, finds nothing about draft durability, and has to decide whether the
spine is wrong or the epics file moved. The `Open` row at
`ARCHITECTURE-SPINE.md:648` already records that `epics.md` is in flux and
contradicts the spine, which makes a bare line-number reference into it the
least durable form of citation available.

**Fix.** Cite the story anchor rather than the offset — `epics.md`, Story
UX-PB.1b, crash/force-quit recovery criterion — or quote the clause inline. The
spine already cites `docs/DECISIONS.md` by decision id (D25, D27, D30–D33)
everywhere else; this is the one place it reaches for line numbers, and it is
the one place the target file is actively being edited.

---

### LOW-1 — `Node in CI 24` is true through two independent mechanisms, one of which is invisible to the spine

`ARCHITECTURE-SPINE.md:594`:

> `| Node in CI | 24 |`

Three jobs pin the literal:

- `.github/workflows/ci.yml:54` — `          node-version: 24`
- `.github/workflows/ci.yml:73` — `          node-version: 24`
- `.github/workflows/release.yml:79` — `          node-version: 24`

Four jobs resolve it indirectly:

- `.github/workflows/test.yml:40`, `:72`, `:127`, `:171` — `          node-version-file: .nvmrc`
- `.nvmrc` — `24`

The row is correct. The note is that editing `.nvmrc` alone moves four of the
seven jobs off 24 with no diff anywhere the spine points. Informational; a
half-line — "`.nvmrc` for the test lane" — would close it.

### LOW-2 — Two Stack rows sit behind major-only declared ranges

`package.json` declares `"@tauri-apps/api": "^2"` and `"@tauri-apps/cli": "^2"`
— the widest ranges in the manifest. The spine pins the resolved values
(`ARCHITECTURE-SPINE.md:580-581`, `| Tauri JavaScript API | 2.11.1 |` and
`| Tauri CLI | 2.11.4 |`) and those verify exactly. This is consistent with
`ARCHITECTURE-SPINE.md:574` (`the lockfile owns this`), so it is not a defect —
recorded only because these two rows are the likeliest in the table to move
under a routine `npm update` without any deliberate decision.

---

## Verified correct — no finding

Recorded so the next reviewer does not re-do the work, and so the two rows most
likely to be mistaken for typos are documented as confirmed.

### Stack table — Rust (`src-tauri/Cargo.lock`)

Command: `grep -n -A1 '^name = "<crate>"$' src-tauri/Cargo.lock`

| Spine row | Claimed | Cargo.lock | Line |
| --- | --- | --- | --- |
| Tauri Rust crate | 2.11.5 | `version = "2.11.5"` | 3833 |
| Tokio | 1.53.1 | `version = "1.53.1"` | 4244 |
| Tauri updater plugin | 2.10.1 | `version = "2.10.1"` | 3984 |
| Tauri opener plugin | 2.5.4 | `version = "2.5.4"` | 3962 |

`src-tauri/Cargo.toml:6` — `edition = "2021"` — matches `| Rust edition | 2021 |`.

### Stack table — npm (`package-lock.json`, `lockfileVersion: 3`)

Command:
`node -e 'const l=require("./package-lock.json"); for (const n of [...]) console.log(n, l.packages[n].version, l.packages[n].resolved)'`

| Spine row | Claimed | Resolved | Registry |
| --- | --- | --- | --- |
| Tauri JavaScript API | 2.11.1 | 2.11.1 | registry.npmjs.org |
| Tauri CLI | 2.11.4 | 2.11.4 | registry.npmjs.org |
| React / React DOM | 19.2.8 | 19.2.8 / 19.2.8 | registry.npmjs.org |
| **TypeScript** | **7.0.2** | **7.0.2** | `https://registry.npmjs.org/typescript/-/typescript-7.0.2.tgz` |
| **Vite** | **8.1.5** | **8.1.5** | `https://registry.npmjs.org/vite/-/vite-8.1.5.tgz` |
| Tailwind CSS | 4.3.3 | 4.3.3 | registry.npmjs.org |
| Zustand | 5.0.14 | 5.0.14 | registry.npmjs.org |
| TanStack React Virtual | 3.14.8 | 3.14.8 | registry.npmjs.org |
| Vitest | 4.1.10 | 4.1.10 | registry.npmjs.org |
| Playwright | 1.61.1 | 1.61.1 (`@playwright/test` and `playwright`) | registry.npmjs.org |

**TypeScript 7.0.2 and Vite 8.1.5 are CONFIRMED**, resolved from the public npm
registry with real tarball URLs, and matched by the declared ranges
(`"typescript": "~7.0.2"`, `"vite": "^8.1.5"`). A reviewer working from an older
mental model will want to revise these to 5.x / 5.x. **Do not.** The lockfile is
the authority and it says 7.0.2 and 8.1.5.

`| Release automation | release-please action v5 + GitHub Actions |` —
`.github/workflows/release-please.yml:63` and `:174`, both
`        uses: googleapis/release-please-action@v5`.

`| CI runner images | macos-14 (ci.yml build/test, release.yml); ubuntu-latest (all other jobs) |`
— `ci.yml:25`, `ci.yml:67`, `release.yml:62` are `runs-on: macos-14`;
`ci.yml:49`, all four `test.yml` jobs, `release-please.yml:40`, `claude.yml:20`,
`claude-code-review.yml:16` are `runs-on: ubuntu-latest`.
(Command: `grep -rn "runs-on:" .github/workflows/`)

### Verified Brownfield Baseline

**20 commands** (`ARCHITECTURE-SPINE.md:78`).
Command: `sed -n '232,253p' src-tauri/src/lib.rs | grep -c 'commands::'` → `20`.
`src-tauri/src/lib.rs:232` — `        .invoke_handler(tauri::generate_handler![`,
entries at lines 233-252, closing `        ])` at 253.

**Six typed events** (`ARCHITECTURE-SPINE.md:79`).
Command: `grep -c '^pub const EVENT_' src-tauri/src/events.rs` → `6`.
`src-tauri/src/events.rs:77-82`: `detection:updated`, `snapshot:updated`,
`op:status`, `op:output`, `op:stalled`, `appUpdate:status`.

**15 committed contract fixtures** (`ARCHITECTURE-SPINE.md:81`).
Command: `ls dev/fixtures/ipc/*.json | wc -l` → `15`.

**`bridge.ts` is the sole frontend Tauri importer** (`ARCHITECTURE-SPINE.md:80`,
AD-3 at `:144-146`).
Command: `grep -rn "@tauri-apps" src/` returns five hits, all in
`src/lib/ipc/bridge.ts` (three of them comment lines). The three exports are
exactly what AD-3 requires:

- `src/lib/ipc/bridge.ts:9` — `export { invoke } from "@tauri-apps/api/core";`
- `src/lib/ipc/bridge.ts:10` — `export { listen } from "@tauri-apps/api/event";`
- `src/lib/ipc/bridge.ts:11` — `export type { UnlistenFn } from "@tauri-apps/api/event";`

**`planAttemptId` / `Verifying` / `InteractionRequired` do not exist**
(`ARCHITECTURE-SPINE.md:96-97`).
Command:
`grep -rn --include='*.ts' --include='*.tsx' --include='*.rs' -F "<term>" src/ src-tauri/src/ | wc -l`
→ `0` for each of `planAttemptId`, `plan_attempt_id`, `Verifying`,
`InteractionRequired`. `Skipped` is also absent (0 hits), consistent with AD-16
treating it as a new durable state. `Interrupted` **does** exist —
`src-tauri/src/ipc.rs:106` — as AD-16 and AD-5 assume.

**Two release-blocking checks in `release.yml`** (`ARCHITECTURE-SPINE.md:104-106`,
AD-11 at `:243-246`). Both present.

1. minisign verification against the embedded pubkey —
   `.github/workflows/release.yml:315`:
   `          jq -r '.plugins.updater.pubkey' src-tauri/tauri.conf.json | base64 -d > "$RUNNER_TEMP/updater.pub"`
   and `:318-319`:
   `          minisign -V -p "$RUNNER_TEMP/updater.pub" -x "$RUNNER_TEMP/updater.minisig" -m "$UPDATER_TGZ" \`
   `            || { echo "::error::updater signature does not verify against the configured pubkey"; exit 1; }`
2. published-endpoint reachability and coherence —
   `.github/workflows/release.yml:381`: `      - name: Verify published updater endpoint`,
   with the version assertion at `:389-391` and the reachability check at
   `:393-394`:
   `          curl -fsIL --retry 5 --retry-delay 3 "$ASSET" >/dev/null \`
   `            || { echo "::error::updater asset URL is not reachable: $ASSET"; exit 1; }`

**Minimum supported macOS 15.0** (`ARCHITECTURE-SPINE.md:108`, `:596`, AD-11 at
`:252-253`) — `src-tauri/tauri.conf.json:48` — `      "minimumSystemVersion": "15.0"`.

**Persistence** (`ARCHITECTURE-SPINE.md:99-102`) —
`src-tauri/src/journal.rs:19` — `pub const COMPACT_KEEP: usize = 1000;`;
`src-tauri/src/diagnostics.rs:22` — `pub const APP_LOGS_INCLUDED: usize = 3;`;
`src-tauri/src/diagnostics.rs:23` — `pub const TRANSCRIPTS_INCLUDED: usize = 25;`;
`src-tauri/src/diagnostics.rs:129` — `zip.start_file("report.json", SimpleFileOptions::default())`;
`src-tauri/src/diagnostics.rs:144` — `add_file(&mut zip, "operations.jsonl", journal_path)?;`.

**Process-runner safety floor** (`ARCHITECTURE-SPINE.md:85-87`, AD-4 at
`:191-195`) — `src-tauri/src/process/runner.rs:299` `.env_clear()`, `:301`
`.stdin(Stdio::null()) // no sudo, no password entry, ever`, `:304`
`.process_group(0);`, `:261` `killpg(pgid, Signal::SIGTERM)`, `:270`
`killpg(pgid, Signal::SIGKILL)`.

**Current Upgrade Plan shape** (`ARCHITECTURE-SPINE.md:93-98`) —
`src/components/manager/ManagerPane.tsx:130`
`openDialog({ kind: "upgradePlan", plan });`; the row action executes
immediately at `src/components/manager/ManagerPane.tsx:145-152`
(`async function upgradeRow(pkg: Package)` … `await executePlan(plan);`),
exactly the call site AD-16 retires at `ARCHITECTURE-SPINE.md:290-291`;
`src-tauri/src/state.rs:51-52` `pub struct PlanCoordinator {` / `    revision: u64,`;
`autoOpenDrawer` is live at `src/components/settings/SettingsView.tsx:134-135`
and `src/components/activity/useOperationEffects.ts:53`;
`src/components/activity/ActivityDrawer.tsx` exists.

**Five ports** (AD-4, `ARCHITECTURE-SPINE.md:179-181`) — all five are declared
traits: `CommandRunner` `src-tauri/src/process/runner.rs:26`, `EventSink`
`src-tauri/src/events.rs:124`, `UpdateSource` `src-tauri/src/app_update.rs:41`,
`PendingRelease` `src-tauri/src/app_update.rs:48`, `ManagerAdapter`
`src-tauri/src/managers/mod.rs:67`.

**Scheduler constants** (AD-4, `ARCHITECTURE-SPINE.md:215-218`) —
`src-tauri/src/queue.rs:48` `pub const MAX_CONCURRENCY: usize = 4;`,
`src-tauri/src/queue.rs:50` `pub const AGING_GUARD: Duration = Duration::from_secs(120);`.

**Contract-test mechanics** (AD-3, `ARCHITECTURE-SPINE.md:152-156`) —
`src-tauri/src/ipc.rs:788` `fn ipc_contract_matches_committed_fixtures()`,
`:566` `if std::env::var("PM_UPDATE_CONTRACT").is_ok() {`; the TypeScript half
asserts set equality at `src/lib/ipc/types.test.ts:56` —
`  it("covers exactly the committed fixture set", () => {`.

**AD-20 webview boundary** — `src-tauri/tauri.conf.json:25` `      "csp": null`;
`src-tauri/capabilities/default.json` is the only capability file
(`ls src-tauri/capabilities/` → `default.json`) and grants exactly the three
claimed permissions at `:7-9`: `"core:default"`, `"opener:default"`,
`"core:window:allow-start-dragging"`.

**AD-12's seven release-please-owned files** (`ARCHITECTURE-SPINE.md:272-275`) —
`git show --stat --format='' 8a4cf6a` touched exactly those seven and nothing
else: `.release-please-manifest.json`, `CHANGELOG.md`, `package-lock.json`,
`package.json`, `src-tauri/Cargo.lock`, `src-tauri/Cargo.toml`,
`src-tauri/tauri.conf.json`. `release-please-config.json` confirms the three
`extra-files` entries that carry `tauri.conf.json`, `Cargo.toml`, and
`Cargo.lock`.

**Retired boundary catalog** (`ARCHITECTURE-SPINE.md:637`) — `ls -d contracts`
→ absent. Correctly described as not created.

**All nine frontmatter `sources:` paths exist** — `docs/SPEC.md`,
`docs/DECISIONS.md`, `docs/RELEASE-CHECKLIST.md`, `docs/architecture.md`,
`_bmad-output/project-context.md`,
`_bmad-output/planning-artifacts/epics.md`, and the three workflow files.

**All seven cited `docs/DECISIONS.md` ids exist** — D25 (`:118`), D25a (`:132`),
D27 (`:161`), D30 (`:218`), D31 (`:245`), D32 (`:282`), D33 (`:310`).

### Third-party behavioral claim — verified against the pinned crate source

AD-11 (`ARCHITECTURE-SPINE.md:248-250`) makes a load-bearing assertion about
upstream internals:

> ``The x86_64 key is never dropped — `tauri-plugin-updater` resolves its target from `cfg!(target_arch)`, so removing it strands every installed Intel user with no signal.``

This is the one place the spine relies on third-party implementation detail
rather than its own tree, so it is exactly what this lens exists to catch. It is
**correct**, verified against the pinned 2.10.1 source in the local cargo
registry —
`~/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/tauri-plugin-updater-2.10.1/src/updater.rs:1338-1346`:

```
    if cfg!(target_arch = "x86") {
    } else if cfg!(target_arch = "x86_64") {
    } else if cfg!(target_arch = "arm") {
    } else if cfg!(target_arch = "aarch64") {
    } else if cfg!(target_arch = "riscv64") {
```

Target selection is compile-time `cfg!(target_arch)`, so an Intel-built client
requests `darwin-x86_64` and finds no key if the entry is dropped. The
consequence AD-11 states follows.

---

## Counts by severity

| Severity | Count |
| --- | --- |
| CRITICAL | 0 |
| HIGH | 1 |
| MEDIUM | 1 |
| LOW | 2 |

## Method note

Brownfield, so no starter defaults were consulted and no web lookup was needed
for version currency — the lockfiles are the ground truth and they were read
directly. Resolved versions came from `package-lock.json` `packages[].version`
and `src-tauri/Cargo.lock` `[[package]] version`, never from `package.json` or
`Cargo.toml` ranges. Every count in this review states its command. No claim
here is paraphrased; each is a literal quote from a file read during this
review.
