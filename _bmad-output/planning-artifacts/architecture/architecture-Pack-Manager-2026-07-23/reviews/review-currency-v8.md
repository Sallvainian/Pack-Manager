# Currency / Reality Review — ARCHITECTURE-SPINE.md revision 8

**Lens:** Verify every committed decision was web-researched or reality-checked
rather than asserted from training data: current library/framework versions, that
each named technology still exists and fits, and — greenfield — the live defaults
of any starter it leans on. Flag anything that could be out of date and wasn't
confirmed against the web, the existing project, or the current starter.

**Target:** `_bmad-output/planning-artifacts/architecture/architecture-Pack-Manager-2026-07-23/ARCHITECTURE-SPINE.md`
(revision 8, 888 lines, mtime 2026-07-25 04:23).

**Reviewed:** 2026-07-25. Brownfield macOS Tauri 2 + React 19 application.

> **Note on line numbers.** The spine was rewritten mid-review (04:19, 878 lines →
> 04:23, 888 lines). Every citation below was re-resolved against the 888-line
> version. Content of the cited passages was unchanged by that rewrite except
> AD-22, which gained a new `set_settings_core` claim — that new claim is checked
> and verified below (V68).

---

## Verdict

**PASS WITH ONE CRITICAL.**

The Stack table is exact — all 20 rows match the resolved lockfile versions, with
no drift and no "corrected-from-memory" errors. The Verified Brownfield Baseline
is exact — every count, path, symbol, and shipping-behavior claim I could check
resolved against the real files, including the surprising ones. Revision 8's new
reality claims about `set_settings_core`, the execute-path revision check,
`docs/SPEC.md` invariant 5, and reduced-motion CI automation are all **verified
exactly**, several of them quoting their sources word-for-word.

The document fails on exactly one axis, and it is the axis this lens exists to
catch: **AD-26 — the one new invariant built entirely on a web citation — quotes
a sentence that does not exist on the cited page, stamps it "verified
2026-07-25", and the real page says the opposite of what the quote says.**

A second, independent currency failure: the spine commits to `macos-14` as a
standing CI/release rule. GitHub began deprecating that runner label **on 2026-07-06
— nineteen days before this document's own "verified" date** — and retires it
2026-11-02, at which point `release.yml` terminates with an error. This was not
checked against the web.

**Counts.**

| | |
| --- | --- |
| Checkable claims checked | **82** |
| Verified exactly against the cited source | **80** |
| Failed verification | **1** (AD-26's verbatim quote) |
| Verified-as-written but stale against the world | **1** (`macos-14`) |

**Findings by severity:** CRITICAL **1** · HIGH **3** · MEDIUM **2** · LOW **2**
(8 total).

---

## CRITICAL

### C1 — AD-26's load-bearing verbatim quote does not exist on the cited page, and the real page states the opposite

`ARCHITECTURE-SPINE.md:767-770`:

> - **Rule:** Pack-Manager ships macOS, and Tauri's own WebDriver route does not
>   cover it — "Support is available on Windows and Linux, but not macOS due to the
>   lack of a WKWebView driver tool" (`tauri.app`, verified 2026-07-25). Every
>   macOS-viable route therefore embeds an automation surface *in the application*:

Repeated at `ARCHITECTURE-SPINE.md:876`:

> Tauri's WebDriver route excludes macOS ("not macOS due to the lack of a WKWebView
> driver tool", `tauri.app`, verified 2026-07-25)

**I fetched the cited source twice — the rendered page and the raw doc source.**

`https://v2.tauri.app/develop/tests/webdriver/` and its upstream
`https://raw.githubusercontent.com/tauri-apps/tauri-docs/v2/src/content/docs/develop/Tests/WebDriver/index.mdx`.

Asked directly whether the quoted sentence appears in the file, the raw source
returns: **"No, that exact sentence does not appear in the file."**

The actual current text is:

> "Driven directly, only Windows and Linux are supported on desktop, as macOS has
> no WKWebView driver tool available (use the service's embedded WebDriver server
> for macOS)."

and

> "by default the service runs an **embedded WebDriver server** inside your app, so
> no external driver is needed on any platform — and this is how macOS is supported."

Three separate defects in one citation:

1. **The quote is fabricated relative to its source.** The sentence in quotation
   marks is not on the page. It reads like an older revision of the docs or a
   training-data recollection. It carries an explicit "verified 2026-07-25" stamp,
   which asserts a verification that the page contradicts.
2. **It inverts the material fact.** The spine's quote says macOS is *not*
   supported. The current docs say macOS **is** supported — via the embedded
   WebDriver server, which is the documented default. The scoping the real
   sentence carries ("Driven directly…") is exactly the qualifier the spine's
   version drops, and it is the qualifier that changes the meaning.
3. **It is the entire evidentiary basis of a new AD.** AD-26 exists because of
   this sentence, and the Decision Status row at `:876` re-derives "OPEN — blocks
   Story 6.5" from it. A builder who follows the citation to check the constraint
   will not find the sentence and will find text saying macOS is supported.

**What survives.** AD-26's *architectural conclusion* — that every macOS route
embeds an automation surface in the application — is **correct**, and I confirmed
it independently. It is simply not what the cited quote says, and the spine cites
none of the evidence that actually supports it:

- Tauri's own docs: the macOS route *is* the embedded server, i.e. in-app.
- WebdriverIO's Tauri platform-support page documents exactly two macOS routes,
  and **both require a plugin compiled into the application binary**: the embedded
  provider requires `tauri-plugin-wdio-webdriver` registered in Rust; the
  CrabNebula route requires `tauri-plugin-automation`.
- `danielraffel/tauri-webdriver` (an independent macOS WebDriver for Tauri):
  "The plugin (tauri-plugin-webdriver-automation) runs inside your Tauri app in
  debug builds… This two-hop design means the plugin must be compiled into your
  debug build—it's not optional external attachment."

**On the specific CRITICAL condition I was asked to test** — whether any macOS
route exists that does *not* embed an automation surface in the application: **I
found none.** Every documented Tauri-webview route embeds. I checked one
non-WebDriver alternative (Appium's `Mac2Driver`, XCTest/Accessibility-based,
externally driven); its documentation states only system-level requirements
(macOS 10.15+, Xcode 12+, Accessibility access for Xcode Helper) and **does not
state** whether an unmodified third-party app can be driven. I am not asserting
either way — it is `[unverified]`, and it is recorded in M2 below as research
AD-26 owes but has not done.

So AD-26's rule stands. Its citation does not.

**Fix.** Replace the quoted sentence with the real one, re-scope the derived
claim (H1), and cite the WebdriverIO/CrabNebula/`tauri-plugin-*` evidence that
actually establishes the "every route embeds" premise.

---

## HIGH

### H1 — "Tauri's WebDriver route excludes macOS" is false as stated

`:767` "Tauri's own WebDriver route does not cover it" and `:876` "Tauri's
WebDriver route excludes macOS" are both **wrong** against the current docs.
What excludes macOS is `tauri-driver` — the *external* driver, "Driven directly".
Tauri's WebDriver *route* explicitly covers macOS via the embedded server, which
the docs call the recommended path ("this is how macOS is supported").

This matters beyond pedantry: the spine's framing tells a builder there is no
Tauri-supported macOS route at all, when there is a documented, recommended,
first-party one. The correct statement — "the only Tauri-supported macOS route
embeds a WebDriver server in the app" — is *stronger* support for AD-26 than the
false one, because it removes the "maybe an external driver will appear" escape.

### H2 — The Open row asserts undecidability that the cited page family already resolves

`ARCHITECTURE-SPINE.md:876`:

> What remains undecided is whether a compliant non-distributable composition
> exists at all. Until one does, Story 6.5's "Real native Tauri E2E plus artifact
> inspection" test level is not buildable from this spine

AD-26's second rule (`:774-778`) defines compliance as: the automation surface is
"a construction-time dependency of a non-distributable target only. No release
build carries an automation plugin, an embedded driver, a capability granting one,
or any flag, variable, or argument that could activate one."

The WebdriverIO Tauri platform-support docs describe the embedded provider's
registration as occurring **only within `#[cfg(debug_assertions)]` conditional
compilation, limiting it to debug builds**. `danielraffel/tauri-webdriver`
likewise: the plugin "runs inside your Tauri app **in debug builds**."

A `#[cfg(debug_assertions)]`-gated plugin registration is compile-time exclusion
from release bits — not a flag, not an environment variable, not a runtime
selector. That is, on its face, a candidate for exactly the composition the row
declares undecided. Whether it fully satisfies AD-2 (e.g. whether `debug_assertions`
is the right gate versus an explicit non-default Cargo feature, and whether the
`opener:default`-only capability set at `src-tauri/capabilities/default.json` would
need widening) is a real open question — but "whether one exists at all" is not.

The row blocks a story on an unknown that one web fetch narrows. That is the
failure mode this lens targets: an assertion of ignorance substituted for
research.

### H3 — `macos-14` entered deprecation before this document's own verification date and is retired 2026-11-02

The spine commits to `macos-14` twice, as a rule and as a stack fact:

`ARCHITECTURE-SPINE.md:279-283`:

> - **Rule:** Minimum supported macOS is 15.0 at
>   `bundle.macOS.minimumSystemVersion` (`docs/DECISIONS.md` D31). CI stays on
>   `macos-14`: a deployment target above the build SDK is a floor annotation, not
>   an SDK requirement.

`ARCHITECTURE-SPINE.md:828`:

> | CI runner images | macos-14 (ci.yml build/test, release.yml); ubuntu-latest (all other jobs) |

The claim is **accurate as a description of the repository** — I verified every
job:

| Workflow | Job | `runs-on` |
| --- | --- | --- |
| `ci.yml:24-25` | `rust` | `macos-14` |
| `ci.yml:48-49` | `web` | `ubuntu-latest` |
| `ci.yml:65-67` | `build-smoke` | `macos-14` |
| `release.yml:60-62` | `build` | `macos-14` |
| `release-please.yml:39-40` | `release-please` | `ubuntu-latest` |
| `test.yml:28,53,113,157` | `lint`, `test`, `burn-in`, `report` | `ubuntu-latest` (all) |
| `claude.yml:14-20`, `claude-code-review.yml:8-16` | — | `ubuntu-latest` |

It is **stale against the world.** `actions/runner-images` issue #13518:

> "Deprecation: July 6th, 2026"
> "Retirement: November 2nd, 2026"
> "Workflows using the `macos-14`, `macos-14-large`, `macos-14-xlarge` image labels
> will be terminated with an error."

with brownouts through October 2026 (Oct 5-6, 12-13, 16-17, 19-20, 23-24, 26-27,
29-31, each 14:00-00:00 UTC). Recommended replacements: `macos-latest`,
`macos-15`, `macos-26`.

Deprecation began **2026-07-06 — nineteen days before the spine's stated
verification date of 2026-07-25.** This was checkable on the day the document
claimed to check its stack and was not checked.

Consequences the spine should absorb rather than discover:

- `release.yml` is the **release** path. After 2026-11-02 the product cannot be
  built or published at all. That is a release-blocking hard failure, not a
  warning.
- AD-11's reasoning for staying on `macos-14` — "a deployment target above the
  build SDK is a floor annotation, not an SDK requirement" — is a defensible
  argument for a *lower* SDK. It is not an argument for a *retired image*, and the
  rule as written ("CI stays on `macos-14`") reads as a standing instruction that
  will be obeyed straight into breakage.
- The forced migration interacts with the one thing AD-11 and the Decision Status
  row both flag as still OPEN: "Whether `notarytool` accepts `minos 15.0` against
  that SDK is still open and is settled by a manual Release run, never by
  assertion" (`:282-283`). Moving to `macos-15` or `macos-26` changes the build
  SDK and therefore changes that open question's answer. The migration and that
  residual should be settled by the same manual Release run.

---

## MEDIUM

### M1 — AD-26's "therefore" does not follow from its cited premise

`:769-771`: "Every macOS-viable route **therefore** embeds an automation surface
*in the application*."

Even taking the quote at face value, "tauri-driver does not support macOS" does
not entail "every macOS-viable route embeds a surface in the app." The universal
is derived from a statement about one tool. The conclusion happens to be true (I
confirmed it against three independent sources — see C1), but the spine cites
none of them, so the strongest invariant in AD-26 rests on a non-sequitur over a
misquote. Cite the WebdriverIO platform-support page and the plugin requirements
directly; then the universal is supported by evidence rather than inference.

### M2 — AD-26 names no concrete candidate, so the Open row cannot be actioned

AD-26 speaks of "an embedded WebDriver server, or an automation plugin" in the
abstract, and the Open row asks whether a compliant composition exists — but the
spine names not one candidate. The live ecosystem as of 2026-07-25 has at least
four, each with a different answer to AD-2 and AD-20:

- `@wdio/tauri-service` + `tauri-plugin-wdio-webdriver` (embedded provider; the
  WebdriverIO-documented default for macOS; `#[cfg(debug_assertions)]`-gated)
- `@crabnebula/tauri-driver` + `tauri-plugin-automation` (macOS only)
- `danielraffel/tauri-webdriver` / `tauri-plugin-webdriver-automation`
  (external `tauri-wd` CLI + in-app plugin, debug builds)
- CrabNebula Cloud (commercial hosted)

Plus the `[unverified]` non-WebDriver alternative noted in C1 (Appium
`Mac2Driver`, XCTest/Accessibility, externally driven) — whose application-side
requirements I could not establish from its documentation and which AD-26 should
either evaluate or explicitly exclude with a reason. Whoever picks up Story 6.5
has to redo all of this discovery from zero.

---

## LOW

### L1 — The application version literal appears twice; both are disclaimed and both are currently correct

I was asked to check that the version is not restated as a literal anywhere it
claims to be authoritative. **It is not.** Both occurrences explicitly point at
`.release-please-manifest.json` as the owner and both carry a date stamp:

`:131-134`:

> The application version is release-please-owned across five files and is read
> from `.release-please-manifest.json`, never restated here — it was 1.0.1 as of
> 2026-07-25 and will move without this document changing.

`:811`:

> | Application version | release-please-owned; see `.release-please-manifest.json` (1.0.1 on 2026-07-25) |

Verified accurate at review time — all six sources agree:

| File | Value |
| --- | --- |
| `.release-please-manifest.json` | `{".":"1.0.1"}` |
| `package.json:4` | `"version": "1.0.1"` |
| `package-lock.json` (root) | `1.0.1` |
| `src-tauri/tauri.conf.json:4` | `"version": "1.0.1"` |
| `src-tauri/Cargo.toml:3` | `version = "1.0.1"` |
| `src-tauri/Cargo.lock:2619-2620` | `name = "pack-manager"` / `version = "1.0.1"` |

AD-12's seven-file list (`:308-311`) is also correct — all seven exist and are
release-please-owned.

Compliant as written. The only residual is that the **Stack table** is the place
a builder is likeliest to read a row as authoritative, and a row literally titled
"Application version" invites that even with the disclaimer. Optional: drop the
parenthetical from `:811` and keep it only in the prose at `:133`, where the
"will move without this document changing" caveat sits adjacent.

### L2 — Stack table names CI jobs that do not exist under those names

`:828` says "macos-14 (ci.yml build/test, release.yml)". `ci.yml`'s two macOS
jobs are named `rust` (`ci.yml:24`) and `build-smoke` (`ci.yml:65`); there is no
job named `build` or `test` in `ci.yml`. The `test` job name exists in `test.yml`
(`test.yml:53`) and runs `ubuntu-latest`. The runner mapping is right; the job
labels are not. Cosmetic, but it is a citation a reader will try to follow.

---

## Verified-exactly log

Everything below resolved against the real file at the cited location. This is
the 80.

### Stack table — 20/20 exact

Method: resolved versions extracted programmatically from `package-lock.json`
(`lockfileVersion 3`, via each `node_modules/<pkg>.version`) and by
`name`/`version` pair from `src-tauri/Cargo.lock`. **No version was corrected
from memory.**

| Spine row | Claimed | Lockfile says | Source |
| --- | --- | --- | --- |
| Tauri Rust crate | 2.11.5 | 2.11.5 | `Cargo.lock:3832-3833` |
| Tauri JavaScript API | 2.11.1 | 2.11.1 | `package-lock.json` `@tauri-apps/api` |
| Tauri CLI | 2.11.4 | 2.11.4 | `package-lock.json` `@tauri-apps/cli` |
| Tauri updater plugin | 2.10.1 | 2.10.1 | `Cargo.lock:3983-3984` |
| Tauri opener plugin | 2.5.4 | 2.5.4 | `Cargo.lock:3961-3962` |
| Tokio | 1.53.1 | 1.53.1 | `Cargo.lock:4243-4244` |
| React / React DOM | 19.2.8 | 19.2.8 / 19.2.8 | `package-lock.json` |
| **TypeScript** | **7.0.2** | **7.0.2** | `package-lock.json` |
| **Vite** | **8.1.5** | **8.1.5** | `package-lock.json` |
| Tailwind CSS | 4.3.3 | 4.3.3 | `package-lock.json` |
| Zustand | 5.0.14 | 5.0.14 | `package-lock.json` |
| TanStack React Virtual | 3.14.8 | 3.14.8 | `package-lock.json` |
| Vitest | 4.1.10 | 4.1.10 | `package-lock.json` |
| Playwright | 1.61.1 | 1.61.1 | `@playwright/test` and `playwright` both |
| Rust edition | 2021 | `edition = "2021"` | `src-tauri/Cargo.toml:6` |
| Node in CI | 24 | `24` | `.nvmrc`; `ci.yml:54,73`; `release.yml:79`; `test.yml` uses `node-version-file: .nvmrc` |
| Minimum supported macOS | 15.0 | `"minimumSystemVersion": "15.0"` | `src-tauri/tauri.conf.json:48` |
| Release automation | release-please action v5 | `googleapis/release-please-action@v5` | `release-please.yml:63` |
| CI runner images | see H3 table | matches | all six workflows |
| Application version | 1.0.1, release-please-owned | 1.0.1 in all six | see L1 |

The two rows flagged as surprising — **TypeScript 7.0.2** and **Vite 8.1.5** —
are **confirmed correct** by the lockfile. I did not "correct" them.

### Verified Brownfield Baseline — exact

- **"Production registers 20 Tauri commands"** (`:102`) — counted from
  `src-tauri/src/lib.rs` `invoke_handler`/`generate_handler!`: `detect_managers`,
  `get_state`, `refresh_manager`, `refresh_all`, `build_upgrade_plan`,
  `execute_plan`, `self_update_manager`, `run_health_fix`, `cancel_operation`,
  `get_operation`, `list_operations`, `get_settings`, `set_settings`,
  `reveal_operation_log`, `reveal_logs_dir`, `export_diagnostics`,
  `log_frontend_event`, `get_app_update_state`, `check_for_app_update`,
  `install_app_update` = **20**. ✓
- **"six typed events"** (`:103`) — `src-tauri/src/events.rs:77-82`:
  `EVENT_DETECTION_UPDATED` `"detection:updated"`, `EVENT_SNAPSHOT_UPDATED`
  `"snapshot:updated"`, `EVENT_OP_STATUS` `"op:status"`, `EVENT_OP_OUTPUT`
  `"op:output"`, `EVENT_OP_STALLED` `"op:stalled"`, `EVENT_APP_UPDATE_STATUS`
  `"appUpdate:status"` = **6**. ✓
- **"`src/lib/ipc/bridge.ts` is the sole frontend Tauri importer"** (`:104-105`) —
  `grep -rln "@tauri-apps/api" src/` returns exactly one path:
  `src/lib/ipc/bridge.ts`. Its own header says: "bridge.ts — the SINGLE importer
  of `@tauri-apps/api` in the whole frontend." Re-exports exactly `invoke`,
  `listen`, and `type UnlistenFn` — matching AD-3's rule at `:171`. ✓
- **"startup subscribes before hydration"** (`:105`) — `src/App.tsx:56-57`:
  "Subscribe BEFORE hydrating: `detection:updated` is emitted only after the
  backend stores detection…". ✓
- **"a real detection report is never clobbered by the pre-detection placeholder"**
  (AD-3, `:196`) — `src/App.tsx:26-29` guards `setDetection` against the
  placeholder; `src/lib/ipc/events.ts:55`: "A real detection report — the
  pre-detection placeholder has no managers." ✓
- **"`dev/fixtures/ipc/` holds 15 committed contract fixtures"** (`:106`) —
  `ls dev/fixtures/ipc/*.json | wc -l` = **15**. ✓
- **"byte-compares … round-trips the committed bytes back through `Deserialize`"**
  (`:106-108`) — `src-tauri/src/ipc.rs:545` "byte-equality against
  dev/fixtures/ipc/*.json"; `:560` "compares byte-for-byte"; `:578` "The committed
  bytes must also round-trip through Deserialize"; test
  `ipc_contract_matches_committed_fixtures` at `:788`. ✓
- **`PM_UPDATE_CONTRACT=1 cargo test ipc_contract`** (AD-3, `:181`) —
  `src-tauri/src/ipc.rs:546` and `:566`, `:576`. ✓
- **Process runner floor** (`:109-111`, AD-4 `:219-222`) —
  `src-tauri/src/process/runner.rs:299` `.env_clear()`, `:301` `.stdin(Stdio::null())`
  with the inline comment "no sudo, no password entry, ever", `:304`
  `.process_group(0)`, `:261` `killpg(pgid, Signal::SIGTERM)` → `:270`
  `killpg(pgid, Signal::SIGKILL)` with a 5s grace (`:56`). ✓
- **Opener/reveal/restart are direct calls** (`:111-113`, `:880`) —
  `src-tauri/src/commands.rs:672` `tauri_plugin_opener::reveal_item_in_dir(...)`,
  `:681` `tauri_plugin_opener::open_path(...)`, `:795` `app.restart()`. Both
  reveal paths un-ported exactly as `:880` states. ✓
- **`ui.dialog { kind: "upgradePlan" }`, discarded by `closeDialog`** (`:117-118`) —
  `src/store/ui.ts:20` `| { kind: "upgradePlan"; plan: UpgradePlan }`; `:116`
  `closeDialog: () => set({ dialog: { kind: "none" } })`. ✓
- **"a single-package row action executes immediately"** (`:118-119`) and AD-16's
  "The shipping `ManagerPane.upgradeRow` → `executePlan` call site" (`:327`) —
  `src/components/manager/ManagerPane.tsx:145` `async function upgradeRow(pkg: Package)`
  → `:152` `await executePlan(plan);`, wired at `:276`. ✓
- **"no `planAttemptId`, `Verifying`, or `InteractionRequired` symbol exists in
  `src/` or `src-tauri/src/`"** (`:120-121`) — grep for
  `planAttemptId|plan_attempt_id|InteractionRequired|Verifying` across both trees
  returns **zero** matches. ✓
- **"`autoOpenDrawer` is still an active setting"** (`:122`) — live consumer at
  `src/components/activity/useOperationEffects.ts:53`
  `if (mutating && ui.settings?.autoOpenDrawer)`, plus the Settings control at
  `src/components/settings/SettingsView.tsx:134-135`. ✓
- **`settings.json` atomic replace** (`:123`) — `src-tauri/src/settings.rs:127`
  `save_to`, `:139` writes `{file_name}.tmp` then renames. ✓
- **`operations.jsonl` append-only, newest 1,000, temp + fsync + rename**
  (`:124-125`) — `src-tauri/src/journal.rs:19` `pub const COMPACT_KEEP: usize = 1000;`,
  `:179-180` "content goes to a sibling temp file (fsynced), then `rename` replaces
  `operations.jsonl` in one step", `:210` "Write-to-temp + fsync + rename",
  `:222` `f.sync_all()`, `:223` `std::fs::rename(&tmp, path)`. Regression test
  `compact_failure_leaves_the_original_journal_intact` at `:363`. ✓
- **Diagnostics ships `report.json` + newest 3 logs + newest 25 transcripts +
  `operations.jsonl`** (`:125-126`, AD-18 `:559-561`) —
  `src-tauri/src/diagnostics.rs:22` `APP_LOGS_INCLUDED: usize = 3`, `:23`
  `TRANSCRIPTS_INCLUDED: usize = 25`, `:129` `zip.start_file("report.json", ...)`,
  `:144` `add_file(&mut zip, "operations.jsonl", journal_path)`. Tests assert
  exactly 3 (`:240`) and exactly 25 (`:251`). ✓
- **AD-5 "Diagnostics must reject symlinks both when selecting and when streaming"**
  (`:259`) — `src-tauri/src/diagnostics.rs:72-75` `symlink_metadata` for the
  selection filter, `:81` re-checks "regular file (not a symlink) at read time"
  before `io::copy`. Regression test
  `export_never_follows_symlinks_into_the_bundle` at `:274`. ✓
- **AD-5 "Historical PGIDs … never signaled after a restart"** (`:255-257`) —
  `src-tauri/src/journal.rs:6` "signaled on startup (pid reuse)". ✓
- **`release.yml` two blocking checks** (`:127-130`, AD-11 `:269-272`) —
  `release.yml:316-318` installs and runs
  `minisign -V -p "$RUNNER_TEMP/updater.pub" -x "$RUNNER_TEMP/updater.minisig" -m "$UPDATER_TGZ"`,
  with `:305-312` documenting the base64-decode-first trap; `:386-390` fetches the
  published `latest.json` and fails on a version mismatch
  (`echo "::error::latest.json reports $PUBLISHED, expected $VERSION"; exit 1`). ✓
- **AD-12 "publishes nothing only when `attach_to_tag` is empty"** (`:314-315`) —
  `release.yml:362-363` `Attach assets to release (release-please)` /
  `if: inputs.attach_to_tag != ''`; `:382` same guard on the `latest.json` check.
  Header comment `release.yml:8-10` states the manual-empty path attaches to the
  run only. ✓

### AD-11 revision-8 accessibility claims — exact, and the correction is right

This is the claim revision 8 reversed, so I checked every clause.

- **"the product honors it at `src/styles/theme.css` (`@media (prefers-reduced-motion: reduce)`)"**
  (`:287-288`) — `src/styles/theme.css:50-56`:
  `/* prefers-reduced-motion disables all transitions (default 150ms ease). */`
  `@media (prefers-reduced-motion: reduce) { *, *::before, *::after { transition: none !important; animation: none !important; } }`. ✓
- **"`tests/e2e/browser-style-contract.spec.ts` emulates `{ reducedMotion: "reduce" }`"**
  (`:289-290`) — `tests/e2e/browser-style-contract.spec.ts:46`
  `await page.emulateMedia({ reducedMotion: "reduce" });`. ✓
- **"asserts transitions and animations resolve to `0s`"** (`:290-291`) — same
  file `:110` `transitionDuration: "0s",` and `:112` `animationDuration: "0s",`. ✓
- **"running in CI on every push and pull request to `main` via
  `.github/workflows/test.yml`"** (`:291-292`) — `test.yml:8-12`:
  `on:` / `push:` / `branches: [main]` / `pull_request:` / `branches: [main]`.
  The spec runs in the `test` job (`test.yml:53`), gated by `Enforce quality gate`
  which exits 1 unless `TEST_RESULT == success`. ✓
- **"Automated 4.5:1 text contrast does not exist; that same spec disclaims it —
  'It does not claim measured contrast compliance or validate the native Tauri
  package.'"** (`:292-294`) — `tests/e2e/browser-style-contract.spec.ts:116-117`:
  `// This is a browser DOM/CSS contract only. It does not claim measured` /
  `// contrast compliance or validate the native Tauri package.` **Quote is
  word-for-word exact.** ✓
- **"One manual VoiceOver pass and a by-eye contrast check sit on the release
  checklist"** (`:296-297`) — `docs/RELEASE-CHECKLIST.md:89` "One VoiceOver pass
  over the Upgrade Plan announces state changes and completion."; `:95` "Contrast
  (4.5:1) is **not** automated — check it by eye here." The checklist independently
  corroborates the reversal at `:91`: "Reduced motion is covered automatically and
  needs no manual step". ✓

**AD-11's revision-8 correction is correct in both directions.** Reduced motion is
automated and does run in CI; contrast is not automated. The release checklist and
the spec agree with the spine and with each other.

### AD-19 / AD-21 / AD-22 `set_settings_core` and revision drift — exact

- **AD-19 (`:594-596`): "A settings patch is persisted before it becomes active in
  memory or advances the canonical revision; a failed save changes neither — the
  shipping order at `src-tauri/src/commands.rs` `set_settings_core`."** —
  `src-tauri/src/commands.rs:636-651`. Order is exactly as stated:
  `:645-647` `merged.save_to(&state.settings_path).map_err(IpcError::from)?;`
  (early-returns on failure), then `:648` `*state.settings.write()... = merged.clone();`,
  then `:649` `coordinator.bump_revision();`. The inline comment at `:643-644`
  states the invariant in the same words: "Persist before publishing: a failed
  write leaves both the in-memory settings and the canonical plan revision
  unchanged." Corroborating test at `:1708`
  `failed_settings_persistence_changes_neither_memory_nor_revision`. ✓
- **AD-21 (`:638-639`) and `:883`: "the same call site currently bumps it for every
  key" / "`set_settings_core` bumps for every key"** — `commands.rs:649`
  `coordinator.bump_revision();` sits on the unconditional path with no key
  inspection anywhere in the function body. ✓
- **`:883`: "the execute path rejects on `issued.revision != coordinator.revision()`"**
  — `src-tauri/src/commands.rs:372`: `if issued.revision != coordinator.revision() {`.
  **Character-for-character exact.** ✓ The defect AD-21/AD-22 close is real, not
  paper: a `set_settings` write of any key bumps the revision that `:372` tests.
- **AD-22's new revision-8 claim (`:652-655`): "`set_settings_core` already holds
  the coordinator across its atomic `save_to`, so a persist inside the section is
  an existing pattern, not a new blocking-IO-under-mutex hazard."** —
  `commands.rs:637-640` acquires `state.plan_coordinator.lock()` as the function's
  first act; `save_to` at `:646` executes while that guard is live; the guard is
  not released until the function returns at `:650`. **Verified exactly** — this
  is the one new claim the 04:23 rewrite introduced and it holds. ✓
- **AD-4's coordinator-first lock order (`:223-226`)** — consistent with the above;
  `commands.rs:305` and `:460` both read `coordinator.revision()` under the held
  lock. ✓

### AD-25 — exact

- **"(`docs/SPEC.md` load-bearing invariant 5)"** (`:743`) — `docs/SPEC.md:11`
  `### Load-bearing invariants (violations are bugs)`; item **5** at `docs/SPEC.md:17`:
  "**One failing manager never blanks the others.** Per-manager refresh isolation:
  independent operations, timeouts, error cards; prior snapshots are retained on
  failure." **The citation is exact — right file, right list, right ordinal**, and
  the SPEC text independently supports both halves of AD-25's containment rule
  (peers keep running; prior snapshots retained). ✓
- **Merge-not-replace (`:745-749`)** — the merge semantics AD-25 fixes are the
  ones already shipping per `docs/SPEC.md:61`: "merges into a `ManagerSnapshot`
  (inventory rows get `latest = installed`, `outdated = false`; the outdated
  overlay patches `latest` and flips `outdated`; overlay-only rows are appended)",
  and `docs/SPEC.md:748` "Adapter merge: inventory + overlay → statuses;
  overlay-only row appended". AD-25's extension of this to *recovered-parse* output
  is new decision content, correctly framed as a rule rather than a tree claim. The
  parse-failure containment it builds on is real: `docs/SPEC.md:408` "Any parse
  failure → `PmError::ParseFailed { what, excerpt }` (first 500 chars), previous
  snapshot retained, manager error card — loud but never fatal." ✓

### AD-4 ports and scheduling — exact

- **"Five ports exist today"** (`:205-207`) — all five traits located:
  `CommandRunner` `src-tauri/src/process/runner.rs:26`; `EventSink`
  `src-tauri/src/events.rs:124`; `UpdateSource` `src-tauri/src/app_update.rs:41`;
  `PendingRelease` `src-tauri/src/app_update.rs:48`; `ManagerAdapter`
  `src-tauri/src/managers/mod.rs:67`. **5/5.** ✓
- **"global concurrency cap of 4, the 120s aging guard, and duplicate-refresh
  coalescing"** (`:244-245`) — `src-tauri/src/queue.rs:48`
  `pub const MAX_CONCURRENCY: usize = 4;`; `:50`
  `pub const AGING_GUARD: Duration = Duration::from_secs(120);`; `:1068`
  "Duplicate refresh coalesces to the existing opId (SPEC §5.7)"; header `:6-7`
  "Global `Semaphore(4)`. Duplicate `refresh_manager` submissions coalesce to the
  existing opId." Tests `aging_guard_blocks_skip_ahead_after_120s` (`:2329`) and
  `duplicate_refresh_coalesces_to_same_opid` (`:2440`). ✓
- **Unterminated-notice list is "a closed set of verbatim strings — never
  patterns"** (`:236-240`) — `src-tauri/src/process/runner.rs:97-113`
  `split_unterminated_notices` iterates a fixed `UNTERMINATED_NOTICES` const and
  uses `rest.find(notice)` — literal substring search, no regex, no pattern
  compilation. The "only place Pack-Manager inserts a line break the child never
  printed" claim matches `:5` "an unterminated-notice split (D26)". ✓
- **"The manager's own `outdated` verdict is the only authority"** (`:227-229`) —
  `docs/SPEC.md:13` invariant 1, verbatim support including "The frontend's
  version-delta highlight and severity chips are display affordances computed by
  pure string segment-diff, never a source of truth." ✓
- **"Classification inspects the RAW resolved path … BEFORE canonicalizing"**
  (`:231-234`) — `docs/SPEC.md:14` invariant 2 and `docs/SPEC.md:342`, both stating
  the canonicalize-first misroute of uv/npm to brew. ✓
- **"Route precedence is fixed: in-band override, then delegated-if-detected, then
  native, then unavailable"** (`:234-235`) — `docs/SPEC.md:351` in-band, `:352`
  delegated, `:353` native/unavailable, in that order. ✓
- **"no `sudo`, password, or administrator route exists"** (`:221-222`) —
  `docs/SPEC.md:16` invariant 4 "**No sudo, no password entry, ever.** Child stdin
  is `/dev/null`", matching `runner.rs:301`. ✓

### AD-20 — exact

- **"`csp` is `null` today"** (`:607`) — `src-tauri/tauri.conf.json:25` `"csp": null`. ✓
- **"One capability file grants the `main` window exactly `core:default`,
  `opener:default`, and `core:window:allow-start-dragging`"** (`:610-612`) —
  `src-tauri/capabilities/default.json` is the only file under
  `src-tauri/capabilities/`; `"windows": ["main"]`; `"permissions": ["core:default",
  "opener:default", "core:window:allow-start-dragging"]` — **exactly three, exactly
  those**. ✓

### Bindings and cross-document citations — exact

- **"Epic UX-PB (28 stories)"** (frontmatter `:13`, AD-16 `:319`) — distinct ids in
  `_bmad-output/planning-artifacts/epics.md`: UX-PB.1a–1e (5), 2a–2f (6), 3a–3g (7),
  4a–4e (5), 5a–5e (5) = **28**. ✓
- **`:880` quoting `epics.md` Story 6.5: "native command/opener success and failure
  are controlled"** — `_bmad-output/planning-artifacts/epics.md:1291`: "**When**
  native command/opener success and failure are controlled". **Exact.** ✓
- **`:876` quoting Story 6.5's test level "Real native Tauri E2E plus artifact
  inspection"** — `epics.md:1276` "- Required test level: Real native Tauri E2E
  plus artifact inspection". **Exact.** ✓
- **Open-row item (d): the `DEFERRED` register row "still cites only AD-2 and
  AD-3"** — `epics.md:308`: "| Native Tauri E2E harness and runner | `DEFERRED` |
  … Any choice must satisfy `ARCHITECTURE-SPINE.md` AD-2 and AD-3. |" **Exactly
  two ADs cited, exactly those.** ✓
- **All `docs/DECISIONS.md` ids referenced by the spine exist** — D25 (`:118`),
  D25a (`:132`), D27 (`:161`), D30 (`:218`), D31 (`:245`), D32 (`:282`), D33
  (`:310`). ✓
- **`:882` "`install_app_update` has no Rust guard"** — `src-tauri/src/commands.rs:770-795`:
  the body calls `state.app_update.install()`, `state.shutdown()`, sets
  `RELAUNCH_FOCUS_ENV`, and `app.restart()`. **No queue or in-progress check
  anywhere in the function.** The OPEN row is accurate. ✓

---

## Method / scope

- Every npm version was read from `package-lock.json`'s resolved
  `node_modules/<pkg>.version` field, extracted programmatically, not from a
  declared range in `package.json` and not from memory.
- Every Rust crate version was read from the `name`/`version` pair in
  `src-tauri/Cargo.lock`.
- Counts came from `wc -l`, `ls | wc -l`, `grep -c`, or explicit enumeration —
  never estimated.
- The AD-26 web claim was checked against **both** the rendered page
  (`v2.tauri.app`) and the upstream raw source
  (`raw.githubusercontent.com/tauri-apps/tauri-docs/v2/.../WebDriver/index.mdx`),
  plus three corroborating sources (WebdriverIO Tauri platform-support,
  `danielraffel/tauri-webdriver`, `tauri-apps/tauri` issue #7068 context).
- The `macos-14` lifecycle was checked against `actions/runner-images` issue
  #13518 directly, not a summary.
- Appium `Mac2Driver`'s application-side requirements are marked `[unverified]` —
  its documentation states system requirements only and does not address whether
  an unmodified third-party app can be driven. I did not resolve it and do not
  assert it either way.
- Not in scope for this lens and not assessed: internal consistency between ADs,
  story-to-AD reconciliation, prose quality, and the 26 open tail findings from the
  revision-6 gate.

## Sources

- [WebDriver | Tauri](https://v2.tauri.app/develop/tests/webdriver/)
- [tauri-docs WebDriver/index.mdx (v2, raw)](https://github.com/tauri-apps/tauri-docs/blob/v2/src/content/docs/develop/Tests/WebDriver/index.mdx)
- [Platform Support | WebdriverIO](https://webdriver.io/docs/desktop-testing/tauri/platform-support/)
- [danielraffel/tauri-webdriver](https://github.com/danielraffel/tauri-webdriver)
- [Choochmeque/tauri-webdriver](https://github.com/Choochmeque/tauri-webdriver)
- [[feat] MacOSX Support for tauri-driver · tauri-apps/tauri#7068](https://github.com/tauri-apps/tauri/issues/7068)
- [Mac2 | Appium](https://appium.github.io/appium.io/docs/en/drivers/mac2/)
- [[macOS] macOS 14 runner images deprecation · actions/runner-images#13518](https://github.com/actions/runner-images/issues/13518)
- [Upcoming changes to macOS hosted runners — GitHub Changelog](https://github.blog/changelog/2025-07-11-upcoming-changes-to-macos-hosted-runners-macos-latest-migration-and-xcode-support-policy-updates/)
