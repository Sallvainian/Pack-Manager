# Reviewer Gate — Currency / Reality-Check Lens

**Target:** `_bmad-output/planning-artifacts/architecture/architecture-Pack-Manager-2026-07-23/ARCHITECTURE-SPINE.md`, `artifact_revision: 10`
**Lens charter:** "Verify every committed decision was web-researched or reality-checked rather than asserted from training data: current library/framework versions, that each named technology still exists and fits, and the live defaults of any starter it leans on. Flag anything that could be out of date and wasn't confirmed against the web, the existing project, or the current starter."
**Run date:** 2026-07-25
**Repo state at review:** branch `chore/restore-phase-2-prd`, `HEAD` = `1ac959e`

---

## Verdict

Revision 10 is the most reality-checked revision of this spine so far. Every
Stack row resolves exactly against the lockfiles, all seven cited commits exist
and say what the spine says they say, the two engine claims that revision 9
asserted (`ring-*` → `box-shadow`, Tailwind 4's `outline-none`) now verify
against the *installed compiler* and Tailwind's own upgrade guide, and the
AD-26 quotations that revision 8 fabricated are this time genuine text from
`tauri.app/llms-full.txt`. Nothing rises to CRITICAL or HIGH.

What survives is one **FALSE** claim (a stale file list in a row whose stated
purpose is to be currency-accurate), two derived claims narrower or wronger than
their own sources, and three typographic deviations in text presented inside
quotation marks.

---

## Tally

| Metric | Count |
| --- | --- |
| Claims examined | **94** |
| Verified exactly | **86** |
| Failed (FALSE) | **1** |
| Imprecise (true in substance, wrong in a stated detail) | **6** |
| Unverifiable | **1** |

| Severity | Count |
| --- | --- |
| CRITICAL | 0 |
| HIGH | 0 |
| MEDIUM | 4 |
| LOW | 4 |

---

# Findings

## MEDIUM-1 — FALSE: the D34 currency caveat names a file that is already clean and omits three that are not

**Spine text.** `ARCHITECTURE-SPINE.md:1307`:

> "**Caveat for a future currency check:** `docs/SPEC.md` §7.6 moved with the change, but `docs/development-guide.md`, `docs/index.md`, and `_bmad-output/project-context.md` still say `macos-14`."

**Command and output.**

```
$ grep -rln "macos-14" docs/ _bmad-output/project-context.md
docs/architecture.md
docs/deployment-guide.md
docs/project-scan-report.json
docs/development-guide.md
docs/contribution-guide.md
docs/.archive/project-scan-report-2026-07-25T091107.json
docs/DECISIONS.md
docs/.archive/project-scan-report-2026-07-25T002425.json
_bmad-output/project-context.md

$ for f in docs/architecture.md docs/deployment-guide.md docs/development-guide.md \
    docs/contribution-guide.md docs/DECISIONS.md _bmad-output/project-context.md; do
    printf "%s: " "$f"; grep -c "macos-14" "$f"; done
docs/architecture.md: 1
docs/deployment-guide.md: 1
docs/development-guide.md: 1
docs/contribution-guide.md: 1
docs/DECISIONS.md: 3
_bmad-output/project-context.md: 3

$ ls -la docs/index.md
-rw-r--r--+ 1 sallvain 10405 Jul 25 09:22 docs/index.md
$ grep -c "macos-14" docs/index.md
0
```

**Verdict: FALSE in part.** `docs/index.md` exists and contains **zero**
occurrences of `macos-14`. Three files that do carry it —
`docs/architecture.md`, `docs/deployment-guide.md`, `docs/contribution-guide.md`
— are unnamed.

**Root cause — this is the lens's own failure mode.** The list is inherited
verbatim from the commit message of `419dc32` rather than re-checked against the
tree:

```
$ git log -1 --format=%B 419dc32 | grep -n "index.md"
28:docs/development-guide.md, docs/index.md, and project-context.md
```

That commit landed 2026-07-25 06:25:05. Commit `3bd5b1a` ("docs: regenerate
project documentation and context") landed afterwards and changed which
generated docs carry the stale label. The spine restated a report of the tree
instead of the tree — the exact substitution revision 9's opening paragraph says
it stopped doing ("each closure was verified against the committed tree rather
than against the report of it", `ARCHITECTURE-SPINE.md:55-56`).

**Why MEDIUM, not LOW.** The row's own words label it a "Caveat for a future
currency check". Its only job is to tell the next currency pass which files still
need regeneration. It sends that pass to a clean file and hides three dirty ones.

---

## MEDIUM-2 — AD-26's derived conclusion is narrower than the source it fetched: a macOS route exists that does *not* embed a server in the app

**Spine text.** `ARCHITECTURE-SPINE.md:933-939`:

> "The route that does cover macOS is `@wdio/tauri-service`, which "works on **Windows, Linux, and macOS**" because "By default the service runs an **embedded WebDriver server** inside your app". That server is `tauri-plugin-wdio-webdriver`. So the macOS route puts an automation surface *inside the application*…"

**Command and output.** Raw source fetched, not a retrieval-tool paraphrase:

```
$ curl -sL https://tauri.app/llms-full.txt -o /tmp/tauri-llms-full.txt
$ wc -c /tmp/tauri-llms-full.txt
2430050 /tmp/tauri-llms-full.txt

$ grep -n -F 'embedded WebDriver server' /tmp/tauri-llms-full.txt
9485:By default the service runs an **embedded WebDriver server** inside your app, so no external driver is needed on any platform — and this is how macOS is supported. It can also drive the platform's native WebDriver through [`tauri-driver`](https://crates.io/crates/tauri-driver) on Windows and Linux, or [CrabNebula](https://crabnebula.dev)'s cross-platform fork of `tauri-driver` on all platforms (a paid API key is required for macOS). Whichever route you choose, the service detects your application binary, and on the `tauri-driver` route it keeps the Edge WebDriver in sync on Windows for you.
9511:* **`tauri-plugin-wdio-webdriver`** runs the embedded WebDriver server. It's required for the `embedded` provider (the default) — the service drives your app through it, with no external driver, and it's how macOS is supported. You can skip it if you want to use the `external` or `crabnebula` provider instead.
```

**Verdict: IMPRECISE.** The source names **two** macOS-capable routes under
`@wdio/tauri-service`, not one:

1. the `embedded` provider (default) — requires `tauri-plugin-wdio-webdriver`
   *inside the app*; and
2. the `crabnebula` provider — CrabNebula's cross-platform fork of
   `tauri-driver`, "on all platforms (a paid API key is required for macOS)",
   for which line 9511 says explicitly "You can skip it [the plugin]".

The spine says "**the** macOS route puts an automation surface *inside the
application*". That is true of the default route and false of the other, and the
disqualifying sentence sits **26 lines** from the sentence the spine did quote,
on the page it fetched.

**Why this matters and why it is not HIGH.** AD-26's *rules* survive intact —
they govern what happens if an in-app automation surface is adopted, and the
default route does adopt one. But the Rule's stated premise ("Which route is
chosen decides this, so name it") is exactly a route-enumeration claim, and it
enumerates incompletely. A story evaluating Story 6.5's native harness would read
AD-26 and conclude the only macOS option costs a trust-boundary decision, when a
paid-key option exists that does not. Same failure mode as revision 8's inverted
claim — derived rather than read — at lower severity because the direction is
right this time.

**Suggested repair (owner's call, not a hand edit):** state that the *default*
macOS route embeds the server, name the `crabnebula` provider as the documented
alternative with its cost (paid API key), and keep the rule scoped to "any route
that embeds an automation surface".

---

## MEDIUM-3 — AD-27 names its own named sample by the wrong surface: "a toolbar `<button>`" is a Sidebar button

**Spine text.** `ARCHITECTURE-SPINE.md:1002-1005`:

> "The style contract proves the mechanism on **named samples, not a sweep**: today a toolbar `<button>` and the package-row plan-membership checkbox, chosen because they sit on opposite sides of the `appearance` discriminator."

**Command and output.**

```
$ grep -n "Refresh All" tests/e2e/browser-style-contract.spec.ts
74:      const refreshAll = page.getByRole("button", {
75:        name: "Refresh All",

$ grep -rn "Refresh All" src/components/
src/components/shell/Sidebar.tsx:80:          Refresh All
src/components/dashboard/DashboardView.tsx:13:      {/* Refresh All / Update Everything live in the Sidebar only. They were

$ grep -rn "Toolbar" src/ | grep -v "\.test\."
src/components/manager/ManagerPane.tsx:32:import { PackageToolbar } from "./PackageToolbar";
src/components/manager/ManagerPane.tsx:33:import { SelectionToolbar } from "./SelectionToolbar";
src/components/manager/PackageToolbar.tsx:2: * PackageToolbar — search, outdated-only toggle, counts, and the bulk-upgrade
src/components/manager/SelectionToolbar.tsx:2: * SelectionToolbar — the slide-up bar shown while rows are selected
```

**Verdict: IMPRECISE.** The button sample is `Refresh All`, rendered by
`src/components/shell/Sidebar.tsx:80` inside a `<Button variant="secondary">`.
`DashboardView.tsx:13` states outright that it "live[s] in the Sidebar only".
The codebase *does* have components called `PackageToolbar` and
`SelectionToolbar` — neither contains this sample. The word "toolbar" therefore
points a reader at a real, wrong place.

The substance is correct: `Refresh All` is a `<button>`, i.e. on the
non-native-`appearance` side of the discriminator, which is the property the rule
depends on. Only the surface name is wrong.

**Why MEDIUM.** The Rule's entire instruction to a story author is *read the
named samples* (AD-11:349-350 repeats it: "read it for the current set rather
than trusting a list here"). A wrong location in the one sentence that names them
defeats that instruction.

---

## MEDIUM-4 — `prd.md`, which this spine's frontmatter names as its requirements authority, carries two contrast claims that commit `a201fb0` falsified

**Spine text.** `ARCHITECTURE-SPINE.md:16` lists
`_bmad-output/planning-artifacts/prds/prd-Pack-Manager-2026-07-25/prd.md` as a
source, and `ARCHITECTURE-SPINE.md:36-38` calls it "the requirements authority
this spine is reconciled *against*".

**Command and output.**

```
$ grep -n "Not met at \`HEAD\`\|uncommitted working-tree changes" \
    _bmad-output/planning-artifacts/prds/prd-Pack-Manager-2026-07-25/prd.md
447:- Text contrast meets at least 4.5:1 on its surface. **Not met at `HEAD`** — see NFR-6.
628:**Not in this list, deliberately:** the automated contrast guard. The 4.5:1 assertion and the on-fill ink tokens that make it pass are **uncommitted working-tree changes**, absent from `HEAD` `5972109`. Until they land, contrast at release time is a by-eye check, and neither FR-19 nor NFR-6 may be read as CI-guaranteed on that axis.

$ git log -1 --format="%h %ad %s" 5972109
5972109 Sat Jul 25 08:45:58 2026 -0400 Clear the epics.md residual pile (#36)

$ git log -1 --format="%h %ad %s" a201fb0
a201fb0 Sat Jul 25 12:31:30 2026 -0400 fix(ui): use the palette's dark ink on bright accent fills

$ grep -rn "text-white" src/ | wc -l
0
$ grep -n "toBeGreaterThanOrEqual(4.5)" tests/e2e/browser-style-contract.spec.ts
320:      expect(measured.ratio).toBeGreaterThanOrEqual(4.5);
```

**Verdict: the spine is CORRECT; its cited authority is stale.** The spine gets
this right and says so explicitly at `ARCHITECTURE-SPINE.md:359-363`:

> "revision 5 asserted both automated checks existed, revision 8 asserted neither did, revision 9 corrected the reduced-motion half, and revision 10 found the contrast half false again because the check landed (`docs/DECISIONS.md` D36, commit `a201fb0`)."

The PRD, however, still asserts the guard does not exist and pins that assertion
to `HEAD 5972109`, which is now three commits behind. A reader who follows the
spine's own frontmatter to the requirements authority reads the opposite of the
truth on this one axis.

**Logged here, not as a spine defect.** The remedy is a PRD update through
`bmad-prd`, not a hand edit and not a spine change. Flagged because the charter
asks for "anything that could be out of date", and a stale authority is a
currency risk the spine inherits by citation.

---

## LOW-1 — AD-26's first quotation is not byte-verbatim: the source apostrophe is U+2019, the spine's is ASCII

**Spine text.** `ARCHITECTURE-SPINE.md:930-933`:

> "Driven directly, only Windows and Linux are supported on desktop, as macOS has no WKWebView driver tool available (use the service's embedded WebDriver server for macOS)" (`tauri.app/llms-full.txt`, verified 2026-07-25)

**Command and output.**

```
$ grep -n -F 'Driven directly' /tmp/tauri-llms-full.txt
9532:If you are not using Node.js, prefer [Selenium](/develop/tests/webdriver/example/selenium/), or are integrating WebDriver into a custom test harness, you can drive [`tauri-driver`](https://crates.io/crates/tauri-driver) directly instead of using the service. Driven directly, only Windows and Linux are supported on desktop, as macOS has no WKWebView driver tool available (use the service's embedded WebDriver server for macOS).

$ python3 -c "...print(repr(frag))"   # line 9532, from 'Driven directly'
'Driven directly, only Windows and Linux are supported on desktop, as macOS has no WKWebView driver tool available (use the service’s embedded WebDriver server for macOS).\n'

$ sed -n '929,937p' ARCHITECTURE-SPINE.md | cat -A | grep -n "service"
4:  available (use the service's embedded WebDriver server for macOS)"$
```

**Verdict: VERIFIED, with one character deviation.** The source has `service’s`
(U+2019 RIGHT SINGLE QUOTATION MARK); the spine has `service's` (U+0027 ASCII
APOSTROPHE). Every other character, including word order and parenthetical,
matches exactly. The sentence, its meaning, and its attribution are all genuine —
this is a typographic normalization, not a fabrication.

Reported because the charter says "Report any quotation that does not appear
verbatim, however minor", and because a mechanical `grep -F` against the source
— which is how a future gate would re-check this — returns **zero hits** on the
spine's spelling. That is the practical cost: the quote is unverifiable by the
obvious method.

**Note:** the other two AD-26 quotations are byte-exact.
`grep -n -F 'which works on **Windows, Linux, and macOS**'` and
`grep -n -F 'By default the service runs an **embedded WebDriver server** inside your app'`
both hit (lines 9483 and 9485), bold markers and all.

---

## LOW-2 — AD-29's `journal.rs` quotation silently lowercases its first word

**Spine text.** `ARCHITECTURE-SPINE.md:1157-1159`:

> "This mirrors the shipping Operation journal rather than inventing a discipline — `src-tauri/src/journal.rs` is "one line at op start, one at finish, flushed each write", with a `StartRecord` and a `FinishRecord`."

**Command and output.**

```
$ sed -n '1,7p' src-tauri/src/journal.rs
//! Crash-safe journal `operations.jsonl` (SPEC §5.7, F8, DECISIONS D12) —
//! implemented by U5.
//!
//! One line at op start, one at finish, flushed each write. Start-without-
//! finish renders `Interrupted` on the next launch. Recorded pgids are NEVER
//! signaled on startup (pid reuse). Compacted to the newest 1000 operations at
//! startup. This one file is both the crash journal and the History source.

$ python3 -c "<whitespace-normalize journal.rs, test both casings>"
'One line at op start, one at finish, flushed each write' FOUND
'one line at op start, one at finish, flushed each write' NOT FOUND

$ grep -n "StartRecord\|FinishRecord" src-tauri/src/journal.rs | head -4
26:pub struct StartRecord {
43:pub struct FinishRecord {
```

**Verdict: VERIFIED, with one character deviation.** Source reads `One`; the
spine reads `one`. Standard prose convention for a quote spliced mid-sentence,
and the substance is exact — including that the doc comment spans a line wrap the
spine correctly normalized. `StartRecord` (line 26) and `FinishRecord` (line 43)
both exist as claimed.

---

## LOW-3 — Calling Tailwind v3's `outline-none` a "no-op" is shorthand its own docs contradict

**Spine text.** `ARCHITECTURE-SPINE.md:982-985`:

> "under Tailwind 4 `outline-none` genuinely sets `outline-style: none` (the v3 no-op was renamed `outline-hidden`), so it actively suppresses the indicator."

**Command and output.** Compiled against the *installed* Tailwind, not from
memory:

```
$ node -e 'console.log(require("./node_modules/tailwindcss/package.json").version)'
4.3.3

# compile ['ring-2','ring','outline-none','outline-hidden','outline-2', ...]
# through tailwindcss's own compile() API:
  .outline-none {
    --tw-outline-style: none;
    outline-style: none;
  }
  .outline-hidden {
    --tw-outline-style: none;
    outline-style: none;
    @media (forced-colors: active) {
      outline: 2px solid transparent;
      outline-offset: 2px;
    }
  }
```

And from Tailwind's own v4 upgrade guide (`https://tailwindcss.com/docs/upgrade-guide`):

> "Renamed utilities … v3 `outline-none` → v4 `outline-hidden`"
>
> "The outline-none utility previously didn't actually set `outline-style: none`, and instead set an invisible outline that would still show up in forced colors mode for accessibility reasons. To make this more clear we've renamed this utility to `outline-hidden` and added a new `outline-none` utility that actually sets `outline-style: none`."

**Verdict: VERIFIED in substance; "no-op" is imprecise.** Both halves of the
claim hold: the rename happened, and v4's `outline-none` genuinely sets
`outline-style: none`. But v3's `outline-none` was not a no-op — it set
`outline: 2px solid transparent`, which is invisible in normal rendering and
*visible* in forced-colors mode. The shorthand is defensible for the rule's
purpose (it did nothing to draw a focus ring) and would only mislead someone
reasoning about forced-colors behavior, which this product does not.

This claim was **checked against the web, not asserted** — the source it needed
is exactly the source that confirms it.

---

## LOW-4 — "blocks publication" overstates the second release check; AD-11 states it correctly

**Spine text.** `ARCHITECTURE-SPINE.md:182-185` (Verified Brownfield Baseline):

> "`release.yml` builds universal signed artifacts and blocks publication on two checks: `minisign` verification of the detached updater signature against the embedded pubkey, and a reachability/coherence assertion on the published `latest.json`."

**Command and output.**

```
$ grep -n "Attach assets to release\|Verify published updater endpoint\|minisign -V" \
    .github/workflows/release.yml
319:          minisign -V -p "$RUNNER_TEMP/updater.pub" -x "$RUNNER_TEMP/updater.minisig" -m "$UPDATER_TGZ" \
363:      - name: Attach assets to release (release-please)
382:      - name: Verify published updater endpoint
```

`release.yml:378-381` says so itself:

> "# latest.json is written before the asset it references is uploaded, so
>  # nothing until now has confirmed the published endpoint is reachable,
>  # names this release, and points at a file that exists."

**Verdict: IMPRECISE in the baseline, CORRECT in the AD.** The `minisign` check
(line 319) runs *before* upload and genuinely blocks publication. The
`latest.json` check (line 382) runs *after* `gh release upload` at line 363 — it
fails the workflow, but the assets are already attached. AD-11 gets this exactly
right at `ARCHITECTURE-SPINE.md:324-327`: "the published `latest.json` is
asserted reachable and coherent **after upload**." Only the baseline bullet
compresses both into "blocks publication".

Both checks were verified to exist, to be wired to `exit 1`, and to do what the
spine says: the signature is base64-decoded before verification
(`base64 -d < "$UPDATER_TGZ.sig"`, line 315) against the pubkey read out of the
shipping config (`jq -r '.plugins.updater.pubkey' src-tauri/tauri.conf.json`,
line 316).

---

# Claim ledger

Every claim below was checked by running a command. Nothing is estimated.

## 1. Stack table (`ARCHITECTURE-SPINE.md:1245-1266`) — 20 rows, 20 verified

Header at `:1242-1243`: "Verified against `package-lock.json` and
`src-tauri/Cargo.lock` on 2026-07-25."

```
$ node -e 'const l=require("./package-lock.json"); for (const k of [...]) console.log(k,"=>",l.packages[k]?.version)'
node_modules/@tauri-apps/api          => 2.11.1
node_modules/@tauri-apps/cli          => 2.11.4
node_modules/react                    => 19.2.8
node_modules/react-dom                => 19.2.8
node_modules/typescript               => 7.0.2
node_modules/vite                     => 8.1.5
node_modules/tailwindcss              => 4.3.3
node_modules/zustand                  => 5.0.14
node_modules/@tanstack/react-virtual  => 3.14.8
node_modules/vitest                   => 4.1.10
node_modules/@playwright/test         => 1.61.1
lockfileVersion 3 name pack-manager version 1.0.1

$ for p in tauri tauri-plugin-updater tauri-plugin-opener tokio; do grep -A2 "^name = \"$p\"$" src-tauri/Cargo.lock; done
name = "tauri"                 version = "2.11.5"
name = "tauri-plugin-updater"  version = "2.10.1"
name = "tauri-plugin-opener"   version = "2.5.4"
name = "tokio"                 version = "1.53.1"
```

| # | Spine row (`:line`) | Resolved | Verdict |
| --- | --- | --- | --- |
| 1 | `:1247` Application version → `.release-please-manifest.json` (1.0.1) | `cat .release-please-manifest.json` → `{".":"1.0.1"}` | ✅ |
| 2 | `:1248` Rust edition 2021 | `grep -n "^edition" src-tauri/Cargo.toml` → `6:edition = "2021"` | ✅ |
| 3 | `:1249` Tauri Rust crate 2.11.5 | Cargo.lock | ✅ |
| 4 | `:1250` Tauri JavaScript API 2.11.1 | package-lock | ✅ |
| 5 | `:1251` Tauri CLI 2.11.4 | package-lock | ✅ |
| 6 | `:1252` Tauri updater plugin 2.10.1 | Cargo.lock (no JS package present — Rust crate is the referent) | ✅ |
| 7 | `:1253` Tauri opener plugin 2.5.4 | Cargo.lock | ✅ |
| 8 | `:1254` Tokio 1.53.1 | Cargo.lock | ✅ |
| 9 | `:1255` React / React DOM 19.2.8 | package-lock (both) | ✅ |
| 10 | `:1256` TypeScript **7.0.2** | package-lock — resolved, not intuited | ✅ |
| 11 | `:1257` Vite **8.1.5** | package-lock — resolved, not intuited | ✅ |
| 12 | `:1258` Tailwind CSS **4.3.3** | package-lock **and** `node_modules/tailwindcss/package.json` | ✅ |
| 13 | `:1259` Zustand 5.0.14 | package-lock | ✅ |
| 14 | `:1260` TanStack React Virtual 3.14.8 | package-lock | ✅ |
| 15 | `:1261` Vitest 4.1.10 | package-lock | ✅ |
| 16 | `:1262` Playwright 1.61.1 | `@playwright/test`, `playwright`, `playwright-core` all 1.61.1 | ✅ |
| 17 | `:1263` Node in CI 24 | `ci.yml:57,76` + `release.yml:80` `node-version: 24`; `test.yml` uses `node-version-file: .nvmrc`, `cat .nvmrc` → `24` | ✅ |
| 18 | `:1264` CI runner images | see §6 | ✅ |
| 19 | `:1265` Minimum supported macOS 15.0 | `grep -n minimumSystemVersion src-tauri/tauri.conf.json` → `48: "minimumSystemVersion": "15.0"` | ✅ |
| 20 | `:1266` release-please action v5 | `grep -n "uses:" .github/workflows/release-please.yml` → `63,174: googleapis/release-please-action@v5` | ✅ |

**Note on rows 10–12.** These were flagged in the charter as looking wrong from
training-data intuition. They are correct. TypeScript 7.0.2, Vite 8.1.5 and
Tailwind 4.3.3 are all present in `package-lock.json` and installed in
`node_modules/`. No "correction from memory" was applied.

## 2. AD-26 external quotations (`:923-961`) — 6 claims, 4 verified exactly, 2 imprecise

Source fetched raw (`curl -sL https://tauri.app/llms-full.txt`, 2 430 050 bytes,
67 412 lines) and grepped with `grep -F` for literal sentences. No retrieval-tool
paraphrase was used or accepted.

| # | Claim | Source line | Verdict |
| --- | --- | --- | --- |
| 21 | `:930-933` Q1 "Driven directly, only Windows and Linux are supported…" | 9532 | ⚠️ LOW-1 (U+2019 → `'`) |
| 22 | `:934` Q2 "works on **Windows, Linux, and macOS**" | 9483 | ✅ byte-exact |
| 23 | `:935-936` Q3 "By default the service runs an **embedded WebDriver server** inside your app" | 9485 | ✅ byte-exact |
| 24 | `:936` "That server is `tauri-plugin-wdio-webdriver`." | 9511: "**`tauri-plugin-wdio-webdriver`** runs the embedded WebDriver server." | ✅ supported |
| 25 | `:936-939` derived: "So the macOS route puts an automation surface *inside the application*" | 9485, 9511 | ⚠️ MEDIUM-2 |
| 26 | `:946-948` "`src-tauri/Cargo.toml` declares no `[profile.release]`" | `grep -n "profile" src-tauri/Cargo.toml` → **no output, exit 1** | ✅ |

**Unverifiable (1).** `:941-942` "The reference shape is the plugin registered
under `#[cfg(debug_assertions)]`". No such plugin exists in the tree to check
against; the repo uses the macro form (`cfg!(debug_assertions)` at
`src-tauri/src/lib.rs:142`, `logging.rs:118`) and the attribute form only at
`main.rs:2`. The sentence is **prescriptive**, describing a shape a future
harness must take, not a citation of shipping code — so this is UNVERIFIED rather
than false, and is not a defect.

## 3. AD-27 engine and lane claims (`:963-1039`) — 11 claims, 9 verified, 2 imprecise

| # | Claim (`:line`) | Evidence | Verdict |
| --- | --- | --- | --- |
| 27 | `:987` "Tailwind's `ring-*` compiles to `box-shadow`" | Compiled `ring-2` through `tailwindcss@4.3.3`'s own `compile()` API → `box-shadow: var(--tw-inset-shadow), …, var(--tw-ring-shadow), var(--tw-shadow);` | ✅ |
| 28 | `:987-991` "WebKit does not paint `box-shadow` on a control still rendering with its **native appearance** — the discriminator is `appearance`" | `git log -1 --format=%B 22ed41e`: "computed box-shadow is the full ring in Chromium and the literal string \"none\" in WebKit… the same checkbox paints the moment appearance: none is applied. So the discriminator is appearance, not focus". Corroborated by `docs/DECISIONS.md:428-439` and `docs/SPEC.md:208`. **Measured, not asserted.** | ✅ |
| 29 | `:982-985` Tailwind 4 `outline-none` sets `outline-style: none`; v3's was renamed `outline-hidden` | Compiled: `.outline-none { --tw-outline-style: none; outline-style: none; }`; `.outline-hidden` adds the `forced-colors` fallback. Confirmed against `tailwindcss.com/docs/upgrade-guide`. | ⚠️ LOW-3 ("no-op") |
| 30 | `:1019-1020` "CI's `webkit` project is Playwright's **Linux** WebKit on `ubuntu-latest`, not WKWebView" | `playwright.config.ts:84-87` `{ name: "webkit", use: {...devices["Desktop Safari"]} }`; `test.yml:30,56,117,161` all `runs-on: ubuntu-latest` | ✅ |
| 31 | `:1002-1005` named samples: "a toolbar `<button>` and the package-row plan-membership checkbox" | spec:74-77 `Refresh All` (Sidebar), spec:186-189 `getByTestId(row-…).getByRole("checkbox")` | ⚠️ MEDIUM-3 |
| 32 | `:1010-1011` white measures 2.46:1, 2.15:1, 2.30:1 | Computed WCAG 2.1 from `theme.css` tokens `#65A7FF`, `#7DB3FF`, `#FF8793`: **2.46, 2.15, 2.30** | ✅ exact |
| 33 | `:1011` on-accent measures 7.74:1, 8.87:1, 8.30:1 | Same computation with `#07101D`: **7.74, 8.87, 8.30** | ✅ exact |
| 34 | `:1011-1015` guard measures **rendered** fg/bg and fails below 4.5:1 | `browser-style-contract.spec.ts:284-320` computes relative luminance from `getComputedStyle`, `expect(measured.ratio).toBeGreaterThanOrEqual(4.5)` and `expect(measured.color).not.toBe("rgb(255, 255, 255)")` | ✅ |
| 35 | `:994-995` `appearance: none` rejected — destroys the native checkmark (D35) | `docs/DECISIONS.md:436` "`appearance: none` also destroys the native checkmark, so it is not the fix"; `:481` "**Rejected:** `appearance:` …" | ✅ |
| 36 | `:974-976` D37 quote "Deleting the rule would remove no work and would only un-guard working code against the next `ring-*`" | `docs/DECISIONS.md:541-543` (line-wrapped); whitespace-normalized match → FOUND | ✅ verbatim |
| 37 | `:998-1000` "three reported, **nine** actual" | `git log -1 --format=%B 22ed41e`: "Nine controls had no focus style at all, not the three a grep found." | ✅ |

## 4. AD-28 / AD-29 code citations — 14 claims, 14 verified

| # | Claim (`:line`) | Command → output | Verdict |
| --- | --- | --- | --- |
| 38 | `:1157-1159` `journal.rs` "one line at op start, one at finish, flushed each write" | `journal.rs:4` (capital `One`) | ⚠️ LOW-2 |
| 39 | `:1159` `StartRecord` and `FinishRecord` | `journal.rs:26`, `journal.rs:43` | ✅ |
| 40 | `:1211-1214` "every start-shaped line pushes a new entry and only the finish half dedups, against the latest index write" | `journal.rs:132-141`: `Ok(Line::Start(s)) => { index.insert(s.op_id.clone(), entries.len()); entries.push((s, None)); }` / `Ok(Line::Finish(f)) => { if let Some(&i) = index.get(&f.op_id) { entries[i].1 = Some(f); } }` | ✅ exact |
| 41 | `:1193-1194` "the shipping start-without-finish semantics" → `Interrupted` | `journal.rs:154-156`: `None => (OpStatus::Interrupted, None, None)` | ✅ |
| 42 | `:1053` "The live `selection` set in `src/store/packages.ts`" | `packages.ts:17` `selection: Partial<Record<ManagerId, Set<string>>>;` with toggle/range/set/clear at `:73,83,87,106` | ✅ |
| 43 | `:1058-1059` "the **anchor survives**" | `packages.ts:18-19` `/** Shift-range anchor per manager. */ anchor: …`; `:110 setAnchor` | ✅ |
| 44 | `:1115-1117` predicate ships **twice** | `useKeyboard.ts:35-53` `visibleSelectableIds`; `ManagerPane.tsx:92-107` inline (`anyOutdated`/`outdatedOnly`/`matchesSearch`/`caskGreedy`/`isSelectable`) — same five filters, same order | ✅ |
| 45 | `:1116-1117` comment "mirrors ManagerPane filters" | `useKeyboard.ts:34` "/** Visible + selectable package ids for a manager (mirrors ManagerPane filters). */" | ✅ verbatim |
| 46 | `:1129-1131` "`⌘A` calls `preventDefault()` before its helper early-returns" | `useKeyboard.ts:160-163` `case "a": e.preventDefault(); selectAllVisible();` → `:88-89` `if (ui.view.kind !== "manager") return;` | ✅ |
| 47 | `:1099-1101` Esc cascade drops three rungs to two | `useKeyboard.ts:64-79` `handleEscape` = closeDialog → clearSelection → setDrawerOpen(false); `prd.md:237` "Esc collapses from three rungs to two… removes the middle rung only" | ✅ |
| 48 | `:1109-1111` RP-2 map `⌘R`, `⌘⇧R`, `⌘⇧U`, `⌘L`, `⌘F`, `⌘1–9`, `⌘A` | `prd.md:534` enumerates exactly that set | ✅ |
| 49 | `:1051-1052` `EXPERIENCE.md` "On eligible Package rows, selection immediately adds/removes Upgrade Plan membership" | `EXPERIENCE.md:143` | ✅ verbatim |
| 50 | `:1068-1070` NFR-3 "interface stays interactive beyond 100 Packages, with correct actions reachable at 101 rows" (`prd.md` FR-6) | `prd.md:235` (inside FR-6) and `prd.md:556` (NFR-3) | ✅ verbatim substring |
| 51 | `:1054-1057` SPEC F5 stale; `prd.md` §0.1 records F5 never entered the supersession list | `prd.md:45` "**F5 was never added to that list**… This omission is the entire reason the FR-6 conflict stayed live" | ✅ |

## 5. Verified Brownfield Baseline (`:141-189`) — 18 claims, 17 verified, 1 imprecise

| # | Claim (`:line`) | Command → output | Verdict |
| --- | --- | --- | --- |
| 52 | `:146-147` "20 Tauri commands (`src-tauri/src/lib.rs`, `invoke_handler`)" | `awk '/generate_handler!\[/,/\]/' src-tauri/src/lib.rs \| grep -c "commands::"` → **20** | ✅ |
| 53 | `:147` "six typed events (`src-tauri/src/events.rs`)" | `events.rs:77-82` — `detection:updated`, `snapshot:updated`, `op:status`, `op:output`, `op:stalled`, `appUpdate:status` = **6** | ✅ |
| 54 | `:147-148` "matching frontend wrappers and subscriptions" | `grep -c "^export async function\|^export function" src/lib/ipc/client.ts` → **20**; `src/lib/ipc/types.ts:319-324` mirrors all six event names | ✅ |
| 55 | `:148-149` "`src/lib/ipc/bridge.ts` is the sole frontend Tauri importer" | `grep -rn "@tauri-apps/api" src/ \| grep -v "\.test\."` → **only** `bridge.ts:2,5,9,10,11` | ✅ |
| 56 | `:149` "startup subscribes before hydration" | `src/App.tsx:56-57` "// Subscribe BEFORE hydrating: `detection:updated` is emitted only after the // backend stores detection…" | ✅ |
| 57 | `:150` "`dev/fixtures/ipc/` holds 15 committed contract fixtures" | `ls dev/fixtures/ipc/*.json \| wc -l` → **15** | ✅ |
| 58 | `:150-152` "`src-tauri/src/ipc.rs` byte-compares each serialized model against its fixture and round-trips" | `ipc.rs:788 fn ipc_contract_matches_committed_fixtures`, `:566 std::env::var("PM_UPDATE_CONTRACT")`, `:546` regeneration comment | ✅ |
| 59 | `:153-156` runner: structured argv, cleared env, null stdin, isolated process groups, bounded output, timeout, SIGTERM→SIGKILL | `src-tauri/src/process/runner.rs:299 .env_clear()`, `:301 .stdin(Stdio::null()) // no sudo, no password entry, ever`, `:304 .process_group(0)`, header `:3-4` | ✅ |
| 60 | `:156-158` opener, reveal, restart, current-exe, bundle-parent writability still direct OS calls | `src-tauri/src/paths.rs:118-129` direct `Command` with `Stdio::null()`/`process_group(0)`; no port wraps them | ✅ |
| 61 | `:161-163` Upgrade Plan is transient dialog state, discarded by `closeDialog` | `src/store/ui.ts:116` `closeDialog: () => set({ dialog: { kind: "none" } })` | ✅ |
| 62 | `:163-165` "no `planAttemptId`, `Verifying`, or `InteractionRequired` symbol exists in `src/` or `src-tauri/src/`" | `grep -rn "planAttemptId\|plan_attempt_id\|Verifying\|InteractionRequired\|interactionRequired" src/ src-tauri/src/ \| wc -l` → **0** | ✅ |
| 63 | `:165` "`autoOpenDrawer` is still an active setting" | `src/components/settings/SettingsView.tsx:134-135` renders and patches it | ✅ |
| 64 | `:175-177` pinned row's checkbox natively `disabled` with reduced opacity | `PackageRow.tsx:92 disabled={checkboxDisabled}`, `:100 "disabled:cursor-not-allowed disabled:opacity-40"` | ✅ |
| 65 | `:178-180` `settings.json` atomic replace; `operations.jsonl` compacted to newest 1 000 via temp + fsync + rename | `settings.rs:124` "Atomic: temp file + fsync + rename", `:144-145 f.sync_all()? / std::fs::rename`; `journal.rs:19 COMPACT_KEEP: usize = 1000`, `:210-223` temp+`sync_all`+`rename` | ✅ |
| 66 | `:180-181` diagnostics ships `report.json`, newest 3 app logs, newest 25 transcripts, `operations.jsonl` | `diagnostics.rs:5-7` header; `:23 TRANSCRIPTS_INCLUDED: usize = 25`; tests `:240` "last 3 app logs", `:251` "last 25 transcripts" | ✅ |
| 67 | `:182-185` `release.yml` blocks publication on two checks | see LOW-4 | ⚠️ LOW-4 |
| 68 | `:186-189` min macOS 15.0; version release-please-owned across five files, 1.0.1 on 2026-07-25 | `tauri.conf.json:48`; `.release-please-manifest.json` `{".":"1.0.1"}`; `package-lock.json` `version 1.0.1` | ✅ |
| 69 | `:158-160` "No native harness exists" | No `tauri-driver`, `@wdio/tauri-service`, or `tauri-plugin-wdio-webdriver` in `package-lock.json` or `Cargo.lock` | ✅ |

## 6. CI runner pins and release-blocking checks — 5 claims, 5 verified

`ARCHITECTURE-SPINE.md:337-339`: "That image is `macos-15`, and all three pins
moved together: `ci.yml` `rust`, `ci.yml` `build-smoke`, and `release.yml`
`build`."

```
$ grep -n "^  [a-z-]*:\|runs-on" .github/workflows/ci.yml
27:  rust:
28:    runs-on: macos-15
51:  web:
52:    runs-on: ubuntu-latest
68:  build-smoke:
70:    runs-on: macos-15

$ grep -n "^  [a-z-]*:\|runs-on:" .github/workflows/release.yml
60:  build:
63:    runs-on: macos-15

$ grep -rn "runs-on" .github/workflows/ | grep -v macos-15
.github/workflows/test.yml:30,56,117,161:    runs-on: ubuntu-latest
.github/workflows/claude.yml:20:    runs-on: ubuntu-latest
.github/workflows/claude-code-review.yml:16:    runs-on: ubuntu-latest
.github/workflows/release-please.yml:40:    runs-on: ubuntu-latest
```

| # | Claim | Verdict |
| --- | --- | --- |
| 70 | `ci.yml` `rust` on `macos-15` | ✅ `:27-28` |
| 71 | `ci.yml` `build-smoke` on `macos-15` | ✅ `:68,70` |
| 72 | `release.yml` `build` on `macos-15` | ✅ `:60,63` |
| 73 | every other job on `ubuntu-latest`; no `runs-on` names `macos-14` | ✅ |
| 74 | `:348-353` style contract "runs in CI on every push and pull request to `main` via `.github/workflows/test.yml`" | ✅ `test.yml:8-12` `push: branches: [main]`, `pull_request: branches: [main]` (plus a weekly `schedule`, which is additive) | ✅ |

## 7. Cited commit hashes — 7 claims, 7 verified

```
$ for h in a201fb0 5c8996f 22ed41e be1f0e6 419dc32 7cc7b5f 8d36cdf; do
    git log -1 --format="%h %ad %s" "$h"; done
a201fb0  Sat Jul 25 12:31:30 2026  fix(ui): use the palette's dark ink on bright accent fills
5c8996f  Sat Jul 25 12:31:39 2026  docs: record D36 and D37, and rescope the release checklist
22ed41e  Sat Jul 25 06:49:46 2026  fix(a11y): draw keyboard focus with an outline so it is visible in WKWebView
be1f0e6  Sat Jul 25 06:32:35 2026  feat(ui): adopt the approved design palette and give focus its own ring
419dc32  Sat Jul 25 06:25:05 2026  ci: move CI and release builds off the deprecated macos-14 runner
7cc7b5f  Sat Jul 25 06:37:59 2026  fix(update): refuse an app update while package operations are in flight
8d36cdf  Sat Jul 25 06:37:59 2026  docs(planning): reconcile the last seven epics.md passages against the spine
```

| # | Spine claim | Verdict |
| --- | --- | --- |
| 75 | `:362,1012` `a201fb0` = D36 bright-fill ink | ✅ subject matches; `grep -rn "text-white" src/ \| wc -l` → 0 |
| 76 | `:1337` `5c8996f` applied D37 to the checklist | ✅ `git show --stat 5c8996f` touches exactly `docs/DECISIONS.md` and `docs/RELEASE-CHECKLIST.md`; checklist `:102-104` carries the removal |
| 77 | `:1338` `22ed41e` = focus-indicator remediation | ✅ subject and body match every number the row cites |
| 78 | `:1306` `be1f0e6` = D35 palette + focus ring | ✅ subject matches |
| 79 | `:347,1307` `419dc32` = `macos-15` move, manual Release run notarized | ✅ body: "Signing and notarization on the new image are proven by a manual Release run" |
| 80 | `:61,1328` `7cc7b5f` = Rust enforcement point | ✅ `commands.rs:772 fn refuse_app_update_while_busy`, `:810` called from `install_app_update`, `:778` matches `Queued \| Running`; frontend `operations.ts:137` filters `"queued" \|\| "running"` — **identical predicate**, as the row claims |
| 81 | `:57,1331` `8d36cdf` = `epics.md` seven-item batch | ✅ subject matches |

## 8. Decision-status rows and remaining AD citations — 13 claims, 12 verified, 1 failed

| # | Claim (`:line`) | Evidence | Verdict |
| --- | --- | --- | --- |
| 82 | `:1307` D34 caveat file list | see MEDIUM-1 | ❌ **FALSE** |
| 83 | `:1306` "All 22 `focus-visible` sites resolve `--color-focus-ring`" (at D35) | `git grep -n "focus-visible:" be1f0e6 -- src/ \| wc -l` → **22** | ✅ |
| 84 | `:1338` "31 sites … zero `ring-focus-ring`, zero `ring-offset-*`, zero `outline-none`" | `git grep -n "focus-visible:outline" HEAD -- src/ \| wc -l` → **31**; `grep -rn "ring-focus-ring\|ring-offset-" src/ \| wc -l` → **0**; `grep -rn "outline-none" src/ \| wc -l` → **0** | ✅ |
| 85 | `:1338` "exactly one `ring-accent` survivor at `src/components/manager/PackageRow.tsx`… deliberately not a focus state" | `PackageRow.tsx:85` `highlighted ? "ring-2 ring-inset ring-accent" : ""` — **no** `focus-visible:` prefix; pinned by `managerPane.test.tsx:114,118` | ✅ |
| 86 | `:1338` "the site count going from 22 to 31" | 22 (`be1f0e6`) → 31 (`HEAD`) | ✅ |
| 87 | `:386-389` (AD-12) "Seven files are release-please-owned" | `release-please-config.json`: `release-type: node` (→ `package.json`, `package-lock.json`) + `extra-files` (`tauri.conf.json`, `Cargo.toml`, `Cargo.lock`) + `changelog-path: CHANGELOG.md` + `.release-please-manifest.json` = **7** | ✅ |
| 88 | `:748-751` (AD-20) capability grants exactly `core:default`, `opener:default`, `core:window:allow-start-dragging` | `src-tauri/capabilities/*.json` → exactly those three, `"windows": ["main"]` | ✅ |
| 89 | `:744-745` (AD-20) "`csp` is `null` today" | `grep -n csp src-tauri/tauri.conf.json` → `25: "csp": null` | ✅ |
| 90 | `:298-300` (AD-4) "global concurrency cap of 4, the 120s aging guard" | `queue.rs:48 MAX_CONCURRENCY: usize = 4`, `:50 AGING_GUARD: Duration = Duration::from_secs(120)` | ✅ |
| 91 | `:458-459` (AD-16) "`OpStatus` ships seven variants today" | `ipc.rs:99-107` — Queued, Running, Succeeded, Failed, Cancelled, TimedOut, Interrupted = **7** | ✅ |
| 92 | `:795-797` (AD-22) `execute_issued_plan` "No synchronous guard crosses an await"; `handle_plan_batch` re-checks | `commands.rs:353` (doc comment on `pub async fn execute_issued_plan` at `:354`) — **verbatim**; `queue.rs:1003 fn handle_plan_batch`, dispatched at `:959` with `expected_revision` | ✅ |
| 93 | `:909-912` (AD-25) "`ManagerAdapter::parse_recovery` takes `refresh_outputs` alongside the failed command's output" | `managers/mod.rs:89-94` `fn parse_recovery(&self, failed: &PlannedCommand, refresh_outputs: &[CommandOutput], out: &CommandOutput)`; doc at `:85-88` states the merge requirement | ✅ |
| 94 | `:404-405` (AD-16) "The shipping `ManagerPane.upgradeRow` → `executePlan` call site" | `ManagerPane.tsx:145 async function upgradeRow(pkg: Package)`, `:152 await executePlan(plan)` | ✅ |

---

# What this lens did *not* find

Recorded so a future gate does not re-litigate:

- **No fabricated quotation.** Every string the spine puts in quotation marks was
  located in its cited source. Revision 8's failure mode (a Context7 paraphrase
  stamped as verbatim from `tauri.app`) has not recurred: the AD-26 quotes are
  real sentences at `llms-full.txt:9483`, `:9485`, `:9532`.
- **No stale version.** All 20 Stack rows resolve. The three the charter singled
  out as intuition-suspect are correct as written.
- **No dead technology.** `tauri-driver`, `@wdio/tauri-service` and
  `tauri-plugin-wdio-webdriver` all exist and are documented on Tauri's current
  site. Playwright, Vitest, Tailwind, Vite, TypeScript, React, Zustand, TanStack
  Virtual, Tokio and the Tauri crates are all installed at the stated versions.
- **No wrong commit.** All seven hashes resolve and their subjects match the
  spine's characterization.
- **No unverified engine assertion.** AD-27's two engine claims — the trap that
  motivated the whole AD — are backed by measurements recorded in `22ed41e`'s
  body and reproducible from the installed compiler here.
- **The D37 absence is not reported as a gap.** `docs/DECISIONS.md:519-566` and
  `docs/RELEASE-CHECKLIST.md:102-106` both instruct a review pass not to. This
  review does not.

---

# Reproduction

```bash
cd /Users/sallvain/Projects/Pack-Manager

# Stack
node -e 'const l=require("./package-lock.json");for(const k of ["node_modules/typescript","node_modules/vite","node_modules/tailwindcss"])console.log(k,l.packages[k].version)'
grep -A2 '^name = "tauri"$' src-tauri/Cargo.lock

# AD-26 quotations (raw source, not a retrieval tool)
curl -sL https://tauri.app/llms-full.txt -o /tmp/tauri-llms-full.txt
grep -n -F 'Driven directly' /tmp/tauri-llms-full.txt
grep -n -F 'embedded WebDriver server' /tmp/tauri-llms-full.txt

# AD-27 Tailwind claims (compiled from the installed version)
#   compile ['ring-2','outline-none','outline-hidden'] via tailwindcss compile() API

# Contrast
node -e '<WCAG 2.1 relative luminance over #65A7FF/#7DB3FF/#FF8793 vs #FFFFFF and #07101D>'

# MEDIUM-1
grep -rln "macos-14" docs/ _bmad-output/project-context.md
grep -c "macos-14" docs/index.md   # → 0

# Focus site counts
git grep -n "focus-visible:" be1f0e6 -- src/ | wc -l   # → 22
git grep -n "focus-visible:outline" HEAD -- src/ | wc -l  # → 31
```

No file in the repository was modified by this review.
