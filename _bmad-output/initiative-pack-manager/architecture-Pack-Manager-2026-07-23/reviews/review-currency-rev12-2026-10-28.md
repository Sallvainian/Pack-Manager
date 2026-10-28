# Reviewer Gate — Currency / Reality-Check Lens (revision 12)

**Target:** `_bmad-output/initiative-pack-manager/architecture-Pack-Manager-2026-07-23/architecture-Pack-Manager-2026-07-23.md`, `artifact_revision: 12` (`updated: "2026-08-11"`)
**Lens charter:** "Verify every committed decision was web-researched or reality-checked rather than asserted from training data: current library/framework versions, that each named technology still exists and fits, and — greenfield — the live defaults of any starter it leans on. Flag anything that could be out of date and wasn't confirmed against the web, the existing project, or the current starter."
**Run date:** 2026-10-28
**Repo state at review:** branch `main`, `HEAD` = `ce86f2c`, clean tree
**Mode:** Validate. The spine was not modified. Line numbers are the spine's own (`:N`).
**Web access:** available. Sources fetched raw (curl / `gh api` / npm and crates.io registries) unless marked as a search-result summary.

---

## Verdict

The **quotations and technology-existence claims hold up three months later.**
All five AD-26 quotations are still byte-verbatim in today's `tauri.app/llms-full.txt`,
including the U+2019 apostrophe that revision 10's gate flagged. `tauri-driver` still has
no macOS path, and `@wdio/tauri-service`, `tauri-plugin-wdio-webdriver`, release-please
action v5, minisign, `macos-15` and Node 24 all still exist and are supported. AD-27's
Tailwind claims re-compile identically against the installed 4.3.3. AD-11's
`cfg!(target_arch)` claim holds in the locked `tauri-plugin-updater` 2.14.0 source.

Two things fail. **First (HIGH):** AD-26 and the harness row say "a compliant composition
does exist" and that Story 6.5 "is therefore buildable". That conclusion comes from one
source, and the service's own setup guide contradicts it. The guide has said since June
that a *second* plugin, `tauri-plugin-wdio`, is **required on every provider**, and that
setup also needs `withGlobalTauri: true`, a capability grant and a frontend import. None
of those can be gated at compile time. **Second (MEDIUM):** the Stack table has drifted.
12 of its 20 rows no longer match the lockfiles, one of them across a major version
(Vitest 4 → 5). No revision has re-checked the table since 2026-07-25, although
Dependabot now auto-merges major bumps.

---

## Tally

| Metric | Count |
| --- | --- |
| Claims examined | **52** |
| Verified exactly | **36** |
| Stale (true when written, false against today's lockfiles) | **13** |
| Unsupported / contradicted by a primary source | **2** |
| Unverifiable this pass | **1** |

| Severity | Count |
| --- | --- |
| CRITICAL | 0 |
| HIGH | 1 |
| MEDIUM | 1 |
| LOW | 3 |

---

# Findings

## HIGH-1 — AD-26's "compliant composition does exist" was never checked against the service's own setup guide, which requires changes that cannot be compile-time gated

**Spine text.**

- `:1099-1101`: "the paid alternative trades money for not widening the boundary."
- `:1103-1107`: "The automation surface is excluded at **compile time**, never by a runtime selector. The reference shape is the plugin registered under `#[cfg(debug_assertions)]` … A feature flag, environment variable, capability toggle, or any other runtime route that could activate it is forbidden by AD-2".
- `:1608` (Native Tauri E2E harness row): "A compliant composition does exist: the plugin registered under `#[cfg(debug_assertions)]` is excluded from release bits at compile time, and this repo declares no `[profile.release]`, so the gate holds today. Story 6.5 is therefore buildable".
- `:1102`: "All quotations verified against raw `tauri.app/llms-full.txt` on 2026-07-25".

**What the spine checked.** One source: the Tauri docs page. Its quotations are accurate
(see ledger §B). That same page links out to the WebdriverIO setup guide ("See [Plugin
Setup](https://webdriver.io/docs/desktop-testing/tauri/plugin-setup) for the full steps",
`llms-full.txt:9580`), and the spine never followed the link.

**What the primary source says.** File:
`webdriverio/desktop-mobile` → `packages/tauri-service/docs/plugin-setup.md`. I fetched
`main` on 2026-10-28, and also the 2026-06-24 revision (`7ed7bc8e`), which is the
version that was live when the spine did its research on 2026-07-25. Both say the same
things:

- "The `tauri-plugin-wdio` is a **required** Tauri plugin". Its table lists it as
  required **"Yes"** for all providers, and lists `tauri-plugin-wdio-webdriver` only
  "For `embedded`, the default".
- "The `@wdio/tauri-service` explicitly checks for the plugin and will fail if it's not
  available".
- Step 3 says to add `"wdio:default"` to `src-tauri/capabilities/default.json`.
- Step 4 says: "Make sure `withGlobalTauri` is enabled in `src-tauri/tauri.conf.json`".
- Step 5 says to import the frontend plugin "in your main frontend entry file".
- The embedded-server plugin also says to add `"wdio-webdriver:default"` to the
  capabilities. The guide adds: "listing it ensures Tauri loads the plugin's ACL
  manifest".
- `tauri-plugin-wdio` provides "Mocking Support - Mock Tauri backend commands" through
  "invoke interception (`window.__wdio_mocks__`)".

The Tauri page instead says the two plugins are "both optional depending on your
requirements" (`llms-full.txt:9576`). The upstream sources therefore **disagree** about
whether `tauri-plugin-wdio` is needed. The spine resolved that disagreement without
noticing it existed.

**Why this matters.**

1. **The paid-route claim at `:1099-1101` is contradicted.** The WDIO guide requires
   `tauri-plugin-wdio` on *every* provider, CrabNebula included. On that reading, the
   paid route still puts a plugin, a capability grant and the global API inside the app.
   It does not "trade money for not widening the boundary".
2. **The "compliant composition" in `:1608` covers only the Rust registration step.**
   - `withGlobalTauri` lives in `tauri.conf.json`, so it ships in release builds.
   - A capability file is static JSON. AD-20 (`:901-903`) fixes that file at exactly
     three permissions, and AD-26 (`:1106`) forbids a "capability toggle" by name.
   - A frontend plugin import would sit beside `bridge.ts`. AD-3 (`:258-259`) makes
     `bridge.ts` the sole Tauri importer.
   - Invoke-interception mocking is exactly what AD-26 `:1116-1121` says "proves nothing"
     about delivery.
   - None of these is excluded by `#[cfg(debug_assertions)]`.
3. **There is a build-time consequence (inference, UNVERIFIED).** The guide's
   webdriver-plugin setup puts the dependency under
   `[target.'cfg(debug_assertions)'.dependencies]` (June revision). If a static
   capability then names `wdio-webdriver:default`, the release build may fail
   capability validation, or it may ship a grant for a plugin that isn't there. Either
   way the "gate holds today" statement describes Rust registration, not the whole
   adoption.

A compliant composition may still exist. Tauri 2 can add a capability at runtime from
Rust under `cfg`, a separate config merge file can be used for test builds, and the
service may run with reduced features without `tauri-plugin-wdio`. **The spine has not
shown that it does.** It asserts the conclusion from a source that does not discuss
these steps.

**Severity.** HIGH, not CRITICAL. The row is marked `OPEN — … not yet adopted`, and
adoption already sits behind an AD-20 security review (`:1608`), so nothing ships on
this claim alone. But the row's purpose is to tell Story 6.5 that it is buildable, and
that feasibility claim is unverified.

**Suggested repair (owner's call, via Validate → Update, not a hand edit):**

- Name `tauri-plugin-wdio` and the three non-Rust setup steps in AD-26.
- State whether any of them is admissible under AD-2, AD-3 and AD-20, or how each would
  be confined to a non-distributable target.
- Strike or qualify "the paid alternative … not widening the boundary".
- Change `:1608` from "Story 6.5 is therefore buildable" to a conditional until a spike
  shows a composition that keeps `tauri.conf.json`, `capabilities/default.json` and
  `src/` unchanged in release.

---

## MEDIUM-1 — The Stack table is stale on 12 of 20 rows, including one major version, and nothing has re-verified it since 2026-07-25

**Spine text.** `:1533-1534`: "Verified against `package-lock.json` and
`src-tauri/Cargo.lock` on 2026-07-25. A brownfield seed, not a version policy — the
lockfiles own this." The table is `:1536-1557`. Revisions 11 and 12 (dated 2026-08-06
and 2026-08-11) did not touch it.

**Command and output.** Each package's locked version was read from
`package-lock.json` `packages["node_modules/<p>"].version` and from the `Cargo.lock`
`[[package]]` blocks. The "Upstream latest" column comes from registry.npmjs.org and the
crates.io API on 2026-10-28.

| Spine row (line) | Spine says | Locked today | Upstream latest | Status |
| --- | --- | --- | --- | --- |
| Application version (`:1538`) | 1.0.1 on 2026-07-25 | 1.0.2 (`.release-please-manifest.json`) | — | snapshot stale; row correctly defers to manifest |
| Tauri Rust crate (`:1540`) | 2.11.5 | 2.12.2 | 2.12.5 | **STALE** |
| Tauri JavaScript API (`:1541`) | 2.11.1 | 2.12.4 | 2.12.4 | **STALE** |
| Tauri CLI (`:1542`) | 2.11.4 | 2.12.4 | 2.12.4 | **STALE** |
| Tauri updater plugin (`:1543`) | 2.10.1 | 2.14.0 | 2.14.1 | **STALE** (four minors; see LOW-2) |
| Tauri opener plugin (`:1544`) | 2.5.4 | 2.7.2 | 2.7.2 | **STALE** |
| Tokio (`:1545`) | 1.53.1 | 1.54.1 | 1.54.1 | **STALE** |
| React / React DOM (`:1546`) | 19.2.8 | 19.3.1 | 19.3.1 | **STALE** |
| Vite (`:1548`) | 8.1.5 | 8.3.5 | 8.3.5 | **STALE** |
| Zustand (`:1550`) | 5.0.14 | 5.0.16 | 5.0.16 | **STALE** |
| TanStack React Virtual (`:1551`) | 3.14.8 | 3.14.22 | 3.14.23 | **STALE** |
| Vitest (`:1552`) | 4.1.10 | **5.0.5** | 5.0.5 | **STALE — major** (`12f4422`, #60) |
| Playwright (`:1553`) | 1.61.1 | 1.64.1 | 1.64.1 | **STALE** |
| Rust edition, TypeScript 7.0.2, Tailwind 4.3.3, Node 24, runners, macOS 15.0, release-please v5 | — | match | — | verified |

**Why this matters.** The disclaimer at `:1534` is honest, but it makes the table weaker
than a pointer to the lockfiles. A reader gets twelve wrong numbers with a "verified"
date attached. The spine already learned this lesson for AD-11 (`:411-413`): "A rule
that names what ships goes stale on the next commit; a rule that names where to look
does not." The Stack table names what ships.

The repo has also changed in a way that speeds up the drift. Commit `eb7391c` ("ci:
auto-merge Dependabot major updates too") and
`.github/workflows/dependabot-auto-merge.yml` now merge major bumps straight to `main`
behind CI alone. Since 2026-07-25, majors that landed this way include Vitest 4→5,
jsdom 29→30, `dirs` 6→7 and `actions/setup-node` 4→7. No architecture re-check runs when
a stack component crosses a major. That matters most for the components AD-11 and AD-26
lean on: the updater plugin, the Tauri core, and Playwright's WebKit.

**Suggested repair:** reduce the table to the rows that are policy rather than
inventory: edition, minimum macOS, runner pins, Node major, and release automation. For
the version rows, say "see the lockfiles". Or keep the table, but re-verify it on every
revision and give it a revisit trigger for major bumps of `tauri`,
`tauri-plugin-updater` and `@playwright/test`.

---

## LOW-1 — The proxy AD-27 promoted runs on `ubuntu-latest`, and that label is moving to Ubuntu 26.04 right now

**Spine text.**

- `:1188-1189`: "CI's `webkit` project is Playwright's **Linux** WebKit on `ubuntu-latest`".
- `:1203-1208`: "D37 therefore **promoted the proxy** … it is the only automated thing
  standing between a `ring-*` on a native control and a release".
- AD-11 `:375-377` rejects floating labels for macOS because "a floating label would
  move the … environment without a commit".

**Evidence.**

- GitHub changelog, 2026-09-17 (fetched): "This migration will roll out gradually
  between October 19 and November 19, 2026", and "pin your workflows to `ubuntu-24.04`
  to stay on the current image".
- `actions/runner-images` README (raw, 2026-10-28) still lists `ubuntu-latest` as
  24.04, with 26.04 published alongside it.
- All four `test.yml` jobs use `runs-on: ubuntu-latest`.

**Assessment.** The spine is correct today and contradicts nothing. But it never applies
D20/D34's own reasoning to the lane it calls the last guard. During the window, any run
may land on either image. The likely impact is small, and that part is verified: the
installed `playwright-core` 1.64.1 knows `ubuntu26.04-x64` as a host platform, and
Playwright pins its own WebKit build per release. What can still move is the host
system libraries under `--with-deps` (inference). Recorded as an observation: the spine
could state whether that lane deliberately floats.

---

## LOW-2 — `tauri-plugin-updater` moved four minors under AD-11, and its client-side semantics changed. The rules still hold, but the post-publish row does not know about the new primitives

**Spine text.**

- AD-11 `:367-372` (the x86_64 key, `cfg!(target_arch)`).
- `:1615`, Post-publish operational envelope: "Deferred — revisit on the first bad
  published release, or on any updater-key change". Its undecided questions include
  retraction, immutability and `pubkey` rotation.

**Evidence.** I extracted the locked crate, `tauri-plugin-updater-2.14.0.crate` from
static.crates.io.

- `src/updater.rs:1769-1782` still resolves the architecture with
  `cfg!(target_arch = "x86_64" | "aarch64" …)`. AD-11's claim **verified**.
- `:753-761` now tries `{os}-{arch}-{installer}` before `{os}-{arch}`. Verified: the
  `darwin-aarch64` / `darwin-x86_64` keys still resolve through the fallback.
- `CHANGELOG.md` since the spine's 2.10.1:
  - **2.12.0 (breaking):** `allowDowngrades` was removed from the webview `check`
    command and is now config-only.
  - **2.13.2:** a failed macOS install no longer deletes the installed app; bundles are
    swapped atomically.
  - **2.13.3:** `minisign-verify` 0.3 "rejects signatures with a non-canonical Ed25519
    `S` value".
  - **2.14.0:** `Update::install` re-verifies package bytes, and there is a new optional
    `requireSignedVersion` that binds the signature's trusted comment to the announced
    version.

**Assessment.** None of these changes falsifies an AD-11 rule. The `release.yml`
signature check still runs `minisign -V` before upload (`release.yml:347` precedes
`:363-376`). But the deferred `:1615` row lists retraction, downgrade and key-rotation
questions as undecided. Upstream now has primitives that bear on them: config-level
downgrade policy, and signature-to-version binding. The row's revisit trigger is
event-based, so it will not fire on this drift. Observation only.

---

## LOW-3 — Node 24 and `macos-15` are both now N-1, and the spine records no revisit trigger for either

**Spine text.** `:1554` "Node in CI | 24". `:375-383`, `:1555`: `macos-15` is the named
stable image.

**Evidence.**

- `nodejs/Release` `schedule.json` (raw): v24 `"maintenance":"2026-10-20"`,
  `"end":"2028-04-30"`. v26 `"lts":"2026-10-28"`. `nodejs.org/dist/index.json` shows
  v26.13.0 released 2026-10-28 with `lts=Lithium`.
- `actions/runner-images` README (raw): `macos-latest` now maps to `macos-26`. `macos-15`
  carries no deprecated badge, and its image readme was refreshed to version
  `20260907.0337.1` (macOS 15.7.9, SDKs `macosx15.0`…`15.5`). That supports the spine's
  "build SDK no longer behind the floor" claim (`:378-383`).
- `macos-14` shows the deprecated badge with "fully unsupported by November 2nd", which
  verifies `:1599`'s 2026-11-02 date.

**Assessment.** Both pins are supported. Node 24 is supported until 2028-04-30. No
`macos-15` deprecation date is published; one secondary source says "Fall 2027", which is
**UNVERIFIED**. Neither pin is a defect. The spine's `macos-14` episode shows the cost of
learning about a deprecation from a deadline. A one-line revisit condition would prevent
a repeat: "when GitHub announces `macos-15` deprecation" and "Node 24 EOL 2028-04-30".

---

# Claim ledger

## §A. Stack table (`:1536-1557`) — 20 claims: 7 verified, 12 stale, 1 snapshot stale

See the MEDIUM-1 table. Verified rows:

- Rust edition 2021 (`Cargo.toml:6`).
- TypeScript 7.0.2 (locked, and npm `latest`).
- Tailwind 4.3.3 (locked, and npm `latest`).
- Node 24 (`.nvmrc`, `ci.yml:57,76`, `release.yml:80`).
- Runner images (`ci.yml:28,70`, `release.yml:63` = `macos-15`; every other job is
  `ubuntu-latest`).
- macOS 15.0 (`tauri.conf.json` `bundle.macOS.minimumSystemVersion`).
- release-please action v5 (`release-please.yml:63,202`; `googleapis/release-please-action`
  latest release `v5.0.0`, 2026-04-22).

## §B. AD-26 quotations (`:1086-1098`) — 5 claims, 5 verbatim

Checked with `grep -c -F` against today's `tauri.app/llms-full.txt` (2,859,733 bytes).
Each count is 1:

- "Driven directly, only Windows and Linux are supported on desktop, as macOS has no
  WKWebView driver tool available (use the service’s embedded WebDriver server for
  macOS)" (line 9599). The U+2019 apostrophe now matches, so revision 10's LOW-1 is fixed.
- "works on **Windows, Linux, and macOS**"
- "By default the service runs an **embedded WebDriver server** inside your app"
- "on all platforms (a paid API key is required for macOS)"
- "You can skip it if you want to use the `external` or `crabnebula` provider instead"

## §C. AD-26 technology status — 4 claims, 4 verified

- `tauri-driver` has no macOS support. crates.io `max_stable=2.1.0` (2026-09-26). Its
  `src/main.rs` is gated `#[cfg(any(target_os = "linux", windows))]`, and the README lists
  "**[Todo]** macOS via Appium Mac2 Driver".
- `@wdio/tauri-service` exists, npm `latest` 1.5.1 (2026-10-24), and its default provider
  is `embedded`. Breaking changes since the spine's research (`driverProvider: 'official'`
  removed, Tauri floor `^2.10`) do not touch anything the spine names.
- `tauri-plugin-wdio-webdriver` exists (crates.io 1.5.1). Its upstream reference
  registration is exactly `#[cfg(debug_assertions)] let builder = builder.plugin(tauri_plugin_wdio_webdriver::init());`.
- The `crabnebula` provider exists, with a paid key for macOS (§B).

## §D. AD-26 / `:1608` derived claims — 3 claims: 1 verified, 2 unsupported

- No `[profile.release]` in `src-tauri/Cargo.toml`: verified (`grep -n '\[profile'` returns nothing).
- "A compliant composition does exist … Story 6.5 is therefore buildable": **unsupported** (HIGH-1).
- "the paid alternative trades money for not widening the boundary": **contradicted** by
  the WDIO setup guide; the Tauri page disputes the guide (HIGH-1).

## §E. AD-11 / release — 7 claims, 7 verified

- `cfg!(target_arch)` target resolution, in the locked updater 2.14.0 source (LOW-2).
- `minisign -V` runs before upload: `release.yml:317-319` in the step at `:347`, which
  precedes `gh release upload` at `:376`.
- The `latest.json` reachability check runs after upload: `release.yml:382-391`.
- The three `macos-15` pins: `ci.yml:28,70`, `release.yml:63`.
- No `runs-on: macos-latest` anywhere in `.github/workflows/`.
- The `macos-15` image SDK ≥ 15.0 (runner-images `macos-15-arm64-Readme.md`).
- `macos-14` is fully unsupported on 2026-11-02 (runner-images announcement #13518).

## §F. AD-12 — 2 claims, 2 verified

- `release-please-config.json` `extra-files` has exactly three entries: `$.version`,
  `$.package.version`, and the `pack-manager` lock entry.
- The updater `pubkey` sits under `tauri.conf.json` `plugins.updater` and is in no
  `extra-files` path.

## §G. AD-20 — 2 claims, 2 verified

- `app.security.csp` is `null`.
- `src-tauri/capabilities/default.json` is the only capability file and grants exactly
  `core:default`, `opener:default` and `core:window:allow-start-dragging`.

## §H. AD-27 engine and lane claims — 5 claims: 4 verified, 1 unverifiable this pass

These three were compiled with the installed `tailwindcss` 4.3.3 `compile()` API:

- `outline-none` → `outline-style: none`. Verified.
- `outline-hidden` → `outline-style: none` plus
  `@media (forced-colors: active) { outline: 2px solid transparent; outline-offset: 2px; }`.
  Verified.
- `ring-2` → `box-shadow: …`. Verified.

The other two:

- The `webkit` project is `devices["Desktop Safari"]` (`playwright.config.ts:85-86`) on
  `ubuntu-latest` (`test.yml`). Verified (LOW-1).
- "WebKit does not paint `box-shadow` on native-appearance controls" (`:1153-1160`):
  **UNVERIFIED this pass.** It needs a WKWebView run, and no macOS Tauri build was
  exercised here. In particular it has not been re-checked on macOS 26's WebKit, which
  users above the 15.0 floor will run. The rule's remedy, `outline`, is safe in either
  case, so the conclusion does not depend on this.

## §I. Verified Brownfield Baseline counts (`:174-180`) — 3 claims, 3 verified

- 20 commands in `lib.rs` `generate_handler!`.
- 6 `EVENT_*` constants in `events.rs:77-82`.
- 15 files in `dev/fixtures/ipc/*.json`.

## §J. Node pin consistency — 1 claim, 1 verified

`.nvmrc` = `24`; `ci.yml:57,76` = `node-version: 24`; `release.yml:80` =
`node-version: 24`. `test.yml` and `claude-code-review.yml` read `.nvmrc`.

---

# What this lens did *not* find

- **No dead or renamed technology.** Every named tool, crate, package, action and runner
  label still exists and is maintained:
  - `tauri-driver`, `@wdio/tauri-service`, `tauri-plugin-wdio-webdriver`
  - CrabNebula
  - minisign (Homebrew 0.12, not deprecated)
  - release-please-action v5
  - `macos-15`, `ubuntu-latest`
  - Node 24
- **No fabricated quotation.** All five external quotations are verbatim in today's source.
- **No version claim that is wrong at the time it was made.** Every stale row in §A was
  accurate on 2026-07-25. The failure is that nothing re-checked them.
- **The D37 absence is not reported as a gap**, per `AGENTS.md` and `:413-419`.

---

# Reproduction

```sh
# Locked versions
node -e 'const l=require("./package-lock.json").packages; for (const p of ["react","typescript","vite","tailwindcss","zustand","@tanstack/react-virtual","vitest","@playwright/test","@tauri-apps/api","@tauri-apps/cli"]) console.log(p, l["node_modules/"+p].version)'
cd src-tauri && for c in tauri tauri-plugin-updater tauri-plugin-opener tokio; do awk -v n="$c" '$0=="name = \""n"\""{getline; print n, $0}' Cargo.lock; done

# Upstream latest
curl -s https://registry.npmjs.org/<pkg> | jq -r '."dist-tags".latest'
curl -s -A ua https://crates.io/api/v1/crates/<crate> | jq -r .crate.max_stable_version

# AD-26 quotations
curl -sL https://tauri.app/llms-full.txt -o llms.txt && grep -c -F '<quote>' llms.txt

# HIGH-1 primary source (current and the version live on 2026-07-25)
curl -sL https://raw.githubusercontent.com/webdriverio/desktop-mobile/main/packages/tauri-service/docs/plugin-setup.md
gh api 'repos/webdriverio/desktop-mobile/contents/packages/tauri-service/docs/plugin-setup.md?ref=7ed7bc8e' --jq .content | base64 -d

# LOW-2 updater source
curl -sL https://static.crates.io/crates/tauri-plugin-updater/tauri-plugin-updater-2.14.0.crate | tar xz
grep -n 'target_arch' tauri-plugin-updater-2.14.0/src/updater.rs

# LOW-1 / LOW-3 runner + Node status
curl -sL https://raw.githubusercontent.com/actions/runner-images/main/README.md | grep -i 'macos\|ubuntu'
curl -sL https://raw.githubusercontent.com/nodejs/Release/main/schedule.json | jq '.v24,.v26'
```

Sources (web): [tauri.app/llms-full.txt](https://tauri.app/llms-full.txt) ·
[WDIO Tauri plugin setup](https://github.com/webdriverio/desktop-mobile/blob/main/packages/tauri-service/docs/plugin-setup.md) ·
[GitHub changelog: Ubuntu 26 GA and latest migration](https://github.blog/changelog/2026-09-17-ubuntu-26-generally-available-and-latest-migration) ·
[GitHub changelog: upcoming image migrations](https://github.blog/changelog/2026-05-14-github-actions-upcoming-image-migrations) ·
[actions/runner-images README](https://github.com/actions/runner-images) ·
[Node.js release schedule](https://github.com/nodejs/Release) ·
[crates.io](https://crates.io) · [npm registry](https://registry.npmjs.org)
