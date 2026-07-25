# Currency & Reality-Check Review — ARCHITECTURE-SPINE.md revision 9

**Target:** `_bmad-output/planning-artifacts/architecture/architecture-Pack-Manager-2026-07-23/ARCHITECTURE-SPINE.md` (artifact_revision 9, 1015 lines).
**Lens:** every committed decision must have been web-researched or reality-checked, not asserted from training data. Every quotation mark in the spine is a claim checked byte-for-byte against its source.
**Date:** 2026-07-25.

## Method

- Repository claims were checked against **raw bytes at the commit the spine names**, using `git show <sha>:<path>` and `git grep <sha>`, never against the working tree unless the claim is about the working tree.
- Counts come from `grep -c` / `wc -l` / `git grep -o | wc -l`. No count is estimated.
- External claims were checked against **primary sources fetched this session**: `https://tauri.app/llms-full.txt`, WebKit source (`raw.githubusercontent.com/WebKit/WebKit/main/Source/WebCore/rendering/RenderTheme.{cpp,h}`, `RenderBox.cpp`, `BackgroundPainter.cpp`), and the installed `tailwindcss@4.3.3` package. Search-engine summaries were **not** accepted as sources for any claim.
- The load-bearing WebKit claim was additionally **reproduced empirically** by driving Playwright's WebKit and Chromium builds over the same elements (script retained at the session scratchpad; method described in HIGH/MEDIUM findings below).

### HEAD moved during this review — read this before acting on any finding

At the start of the review `HEAD` was **`8d36cdf`** — the commit the spine names throughout. Partway through, another agent committed **`22ed41e` "fix(a11y): draw keyboard focus with an outline so it is visible in WKWebView"**, which is the working-tree conversion the spine tracks as an Open row. Everything below states explicitly which tree it is about:

- Claims anchored to `8d36cdf` were verified at `8d36cdf`.
- Claims about "the working tree" were true of the working tree at authoring time and are now committed as `22ed41e`.

The spine was **right to timestamp** its focus claims to a named commit rather than to "today". That is the only reason those claims survive the tree moving under them.

## Verdict

**PASS with two HIGH findings.** This is the best-grounded revision of this spine to date. The specific failure mode the brief warned about — a retrieval tool's paraphrase stamped as a verbatim quote — **did not recur** in the load-bearing places: the four `tauri.app` quotations in AD-26 are byte-exact against `llms-full.txt`, and the WebKit/Tailwind claim in AD-11 is not only correct but reproduces under measurement. Every one of the eighteen Stack rows verifies exactly against the lockfiles. The four cited commits exist and did what the spine attributes to them.

The two HIGH findings are both **enumeration failures inherited from a secondary source instead of re-derived from the tree** — which is precisely the discipline the revision-9 preamble claims for itself ("each closure was verified against the committed tree rather than against the report of it", `ARCHITECTURE-SPINE.md:36-37`).

**Claims checked: 138. Verified exactly: 130.** Six produced findings; two could not be verified from any source available in this repository and are recorded as UNVERIFIED rather than as defects.

**Findings by severity: CRITICAL 0 · HIGH 2 · MEDIUM 5 · LOW 2 (9 total).**

---

## Findings

### HIGH-1 — "the three controls that render none" is false; there were nine, and the spine took the number from `deferred-work.md` without re-deriving it

`ARCHITECTURE-SPINE.md:1014` (Focus-indicator remediation row):

> the addition of an indicator to the three controls that render none (`PackageToolbar.tsx`, `ActivityDrawer.tsx`, `StatusBar.tsx`)

and, in the same row:

> When it lands it closes both accessibility entries in `_bmad-output/implementation-artifacts/deferred-work.md` — the missing `ring-offset-*` on 16 of 22 sites and the three absent indicators

The `16 of 22` half verifies exactly. The `three absent indicators` half does not.

At `8d36cdf`, `git grep -n "focus-visible" 8d36cdf -- src/` returns 22 lines across 16 files. Controls that carry **no** `focus-visible` class at all, and are keyboard-focusable, include at minimum:

- `src/components/dialogs/UpgradePlanSheet.tsx:217` and `:228` — two native checkboxes whose className is `"h-4 w-4 rounded-[4px] border border-border-strong bg-bg-raised accent-accent"` (lines 223 and 234), with no `focus-visible:` anything.
- `src/components/history/HistoryView.tsx:89`, `:102`, `:112` — two `<select>` filters and the search `<input>`; the file has **zero** `focus-visible` occurrences at `8d36cdf`.

That is five beyond the three named, and the spine's own enumeration omits two controls (`UpgradePlanSheet.tsx:217`, `:228`) that its *other* claim in the same paragraph — "all six native checkboxes" — necessarily covers. The two enumerations in one paragraph do not agree with each other.

The remediation commit `22ed41e`, written independently, states the same correction in its own message:

> Nine controls had no focus style at all, not the three a grep found. The other six were located by an audit that focused every focusable element in every reachable view and read computed style: both History select filters, the History search input, both Upgrade Plan sheet checkboxes, and the StatusBar Settings button. Absence of a class is invisible to grep.

The number `three` traces to `_bmad-output/implementation-artifacts/deferred-work.md:46` at `8d36cdf`:

> summary: Three keyboard-focusable controls render no focus indicator at all — the "Outdated only" checkbox, the activity-drawer toggle, and the status-bar "Open logs folder" button.

**Why HIGH:** the spine restated a count from a downstream artifact under a preamble that promises tree-verification, and the count was wrong by a factor of three. Absence of a class is not greppable, so the only sound method is the one `22ed41e` used. Corrected count: **31 focus sites at `22ed41e`, up from 22 at `8d36cdf`** (`git grep -o "focus-visible:outline-2" HEAD -- src/ | wc -l` → 31; `git grep -o "focus-visible:ring-2" HEAD -- src/ | wc -l` → 0).

**Remedy:** replace "the three controls that render none (`PackageToolbar.tsx`, `ActivityDrawer.tsx`, `StatusBar.tsx`)" with "the nine controls that render none" and cite `22ed41e`, or drop the enumeration entirely and cite the commit.

---

### HIGH-2 — revision 9 closes the design-token row but leaves `epics.md` asserting the opposite, and the "epics.md residuals" row claims exhaustively that only **two** things were left

`ARCHITECTURE-SPINE.md:989`:

> | Canonical design-token set | **RESOLVED** | … UX-PB.1e and UX-PB.5d are unblocked. |

`_bmad-output/planning-artifacts/epics.md:308` — unchanged at `8d36cdf` and still unchanged at current `HEAD` (`22ed41e`):

> | Canonical design-token set | `OPEN` — needs an owner decision | UX decides; Development implements | **Blocks UX-PB.1e and UX-PB.5d** (`ARCHITECTURE-SPINE.md:944`). …

and `epics.md:307`:

> Epic UX-PB is the primary build queue and runs first, and nothing blocks starting it **except UX-PB.1e and UX-PB.5d**, which are blocked on the canoni…

`epics.md` is, by the reconciliation commit's own account, the file the builder reads: `8d36cdf`'s message states "epics.md is the file bmad-create-story reads, so each would have flowed into a UX-PB story." A builder picking up UX-PB.1e today is told it is blocked; the spine says it is unblocked.

The spine's residuals row (`ARCHITECTURE-SPINE.md:1013`) opens:

> Two things the batch left, both found by verifying the batch rather than by reading the proposal.

That exhaustiveness claim is now false — this is a third live divergence, and it is one revision 9 itself created by closing the row that `8d36cdf` had just added to `epics.md` in its still-open form.

Secondary, same row: `epics.md:308` cites `ARCHITECTURE-SPINE.md:944` as the authority. At revision 9 line 944 is `| Tailwind CSS | 4.3.3 |` — a Stack row. The line citation has drifted, which is the recurrence of `review-currency-v6.md` MEDIUM-1 ("`epics.md:437-439` citation … is already wrong against the working tree").

**Why HIGH:** it is a direct spine-versus-bound-artifact contradiction about whether two stories may start, recorded in neither place.

**Remedy:** add a third numbered residual to `ARCHITECTURE-SPINE.md:1013` naming `epics.md:307-308` and the drifted `:944` citation, and change "Two things" to "Three things".

---

### MEDIUM-1 — "all six native checkboxes … have no visible focus state" is true, but the stated cause holds for only three of them

`ARCHITECTURE-SPINE.md:1014`:

> As of commit `8d36cdf` the shipping app draws focus with Tailwind `ring-*`, which WKWebView does not paint on native-appearance form controls, so all six native checkboxes — including the package-row plan-membership control — have no visible focus state in the distributed app.

The count is right. `git grep -o 'type="checkbox"' 8d36cdf -- src/ | wc -l` → **6**, at `UpgradePlanSheet.tsx:218`, `UpgradePlanSheet.tsx:229`, `PackageRow.tsx:90`, `PackageTable.tsx:100`, `PackageToolbar.tsx:68`, `Checkbox.tsx:29`. The conclusion is right. The causal chain is right for exactly three of them — `PackageRow`, `PackageTable`, `Checkbox` — which do carry `focus-visible:ring-2 focus-visible:ring-focus-ring`.

The other three (`UpgradePlanSheet.tsx:218`, `:229`, `PackageToolbar.tsx:68`) carry **no focus rule of any kind**, so the WKWebView box-shadow behaviour is not why they are invisible. "so" is doing causal work it cannot support across the whole set.

**Remedy:** "…so the three that draw a ring have no visible focus state in the distributed app, and the other three draw no focus state on any engine."

---

### MEDIUM-2 — the WebKit claim VERIFIES, but the mechanism is stronger than "does not paint": WebKit discards the declaration during style resolution

This is the claim the brief singled out as load-bearing. **It survives.** I did not find it overstated. I am recording the refinement because it makes the invariant *more* enforceable than the spine realises.

`ARCHITECTURE-SPINE.md:328-333`:

> Tauri ships WKWebView, which does not paint `box-shadow` on native-appearance form controls — so a Tailwind `ring-*` utility, which compiles to `box-shadow`, yields *no* visible focus state on a checkbox, radio, or select in the distributed app while looking correct in a Chromium preview.

**Tailwind half — VERIFIED against the installed package**, not against memory. `tailwindcss@4.3.3` (`package-lock.json` → `node_modules/tailwindcss`), `node_modules/tailwindcss/dist/lib.mjs`, at the first occurrence of `--tw-ring-shadow`:

```
let A=function(N){return`var(--tw-ring-inset,) 0 0 0 calc(${N} + var(--tw-ring-offset-width)) var(--tw-ring-color, ${C})`}
```

```
let o=["var(--tw-inset-shadow)","var(--tw-inset-ring-shadow)","var(--tw-ring-offset-shadow)","var(--tw-ring-shadow)","var(--tw-shadow)"].join(", ")
```

`ring-*` sets `--tw-ring-shadow` and composes it into the `box-shadow` property. Confirmed.

**WebKit half — VERIFIED by measurement and by source.** Measured with Playwright's WebKit and Chromium over identical elements, screenshot-hashing a clipped region with and without the ring to decide "painted":

| element | `appearance` | WebKit computed `box-shadow` | WebKit paints shadow | WebKit paints outline | Chromium paints shadow |
| --- | --- | --- | --- | --- | --- |
| `input[type=checkbox]` | auto | `none` | no | yes | yes |
| `input[type=radio]` | auto | `none` | no | yes | yes |
| `select` | auto | `none` | no | yes | yes |
| `button` | auto | `rgb(244, 247, 251) 0px 0px 0px 2px` | **yes** | yes | yes |
| `input[type=checkbox]` + `appearance:none` | none | `rgb(244, 247, 251) 0px 0px 0px 2px` | **yes** | yes | yes |

The declaration is not merely unpainted — `getComputedStyle(el).boxShadow` is the literal string `"none"`. Primary source, `Source/WebCore/rendering/RenderTheme.cpp` (fetched from `raw.githubusercontent.com/WebKit/WebKit/main`, lines 327-330):

```cpp
    if (!style.hasUsedAppearance())
        return;

    if (!supportsBoxShadow(style))
        style.setBoxShadow(CSS::Keyword::None { });
```

and `Source/WebCore/rendering/RenderTheme.h:170`:

```cpp
    virtual bool supportsBoxShadow(const Style::ComputedStyle&) const { return false; }
```

The base implementation returns `false`, so for any control whose used appearance survives (checkbox, radio, menulist), `box-shadow` is reset to `none` before layout. (`RenderBox::paintBoxDecorations` would otherwise have painted it: `BackgroundPainter::boxShadowShouldBeAppliedToBackground` returns `false` when `style.hasUsedAppearance()`, forcing the separate shadow paint. There is nothing left to paint.)

Everything the spine derives from the claim is correct, including the part that explains the green suite: `tests/e2e/browser-style-contract.spec.ts` at `8d36cdf` targets `page.getByRole("button", { name: "Refresh All", exact: true })`, and WebKit does paint `box-shadow` on a `<button>` — measured above. `22ed41e` reached the same conclusion independently: "computed box-shadow is the full ring in Chromium and the literal string \"none\" in WebKit".

**Why this matters, and why it is worth fixing the wording:** "does not paint" implies the failure is invisible to the DOM and therefore only catchable by pixels. It is not. Because WebKit zeroes the *computed value*, the exact assertion shape the spec already uses (`expect(focusTreatment.boxShadow).not.toBe("none")`) would have caught this the moment it was pointed at a checkbox. AD-11 should say so — it converts a "prove it visually" obligation into a cheap computed-style assertion.

**Remedy:** "…which discards `box-shadow` on native-appearance form controls during style resolution — the computed value is `none`, so a Tailwind `ring-*` utility … yields no visible focus state, and does so detectably: a computed-style assertion on the control catches it."

---

### MEDIUM-3 — a paraphrase is inside quotation marks in AD-26's CrabNebula sentence

`ARCHITECTURE-SPINE.md:998`:

> the CrabNebula fork alternative "requires a paid API key for macOS"

`https://tauri.app/llms-full.txt`, line 9485, verbatim:

> or [CrabNebula](https://crabnebula.dev)’s cross-platform fork of `tauri-driver` on all platforms (a paid API key is required for macOS).

The source reads **"a paid API key is required for macOS"**. The spine's quoted string is a re-voiced paraphrase. Substance is unchanged; the quotation marks are not earned. Given a prior revision of this spine was caught stamping a summary as a quote, this is worth correcting rather than waving through.

The three *other* `tauri.app` quotations in AD-26 are byte-exact and pass:

- `ARCHITECTURE-SPINE.md:878-880` ↔ `llms-full.txt:9532` — "Driven directly, only Windows and Linux are supported on desktop, as macOS has no WKWebView driver tool available (use the service's embedded WebDriver server for macOS)" (source uses a typographic apostrophe in "service's"; otherwise identical).
- `ARCHITECTURE-SPINE.md:882` ↔ `llms-full.txt:9483` — "works on **Windows, Linux, and macOS**".
- `ARCHITECTURE-SPINE.md:882-883` ↔ `llms-full.txt:9485` — "By default the service runs an **embedded WebDriver server** inside your app".
- `tauri-plugin-wdio-webdriver` as the embedded server ↔ `llms-full.txt:9511`.

---

### MEDIUM-4 — the Focus-indicator row's status label is false as of `22ed41e`

`ARCHITECTURE-SPINE.md:1014`:

> | Focus-indicator remediation | **OPEN — in flight, uncommitted at this revision** | … is in the working tree but not committed, so this spine does not yet describe it as shipping. |

`22ed41e` committed it. `git status --porcelain` now shows only the three spine-folder documents dirty; every `src/` file is committed.

I am rating this MEDIUM, not HIGH, because the spine did the right thing: it anchored its substantive claims to `"As of commit 8d36cdf"`, which remains true of `8d36cdf` forever. Only the status label and the phrase "uncommitted at this revision" decay. Note also that `22ed41e` moved `tests/e2e/browser-style-contract.spec.ts` in the same commit (from asserting `boxShadow` to asserting `outlineStyle`/`outlineWidth`/`outlineColor`), so the style-contract lane AD-11 depends on did not go red — the risk implied by "uncommitted" did not materialise.

**Remedy:** flip the row to RESOLVED citing `22ed41e`, and fold `22ed41e`'s finding ("nine controls, not three") in with it (see HIGH-1).

---

### MEDIUM-5 — AD-11 requires focus be "proven in WebKit", but the only WebKit the pinned lane runs is a Linux build

`ARCHITECTURE-SPINE.md:328-329`:

> A focus indicator must be drawn by a mechanism the **shipping engine actually paints**, and proven in WebKit rather than Chromium alone.

`playwright.config.ts` defines two projects, `chromium` and `webkit` (`use: { ...devices["Desktop Safari"] }`). `.github/workflows/test.yml:56` — the job that runs them — is `runs-on: ubuntu-latest`. All four `test.yml` jobs are `ubuntu-latest`.

The `macos-15` pins AD-11 correctly enumerates (`ci.yml` `rust` at `:28`, `ci.yml` `build-smoke` at `:70`, `release.yml` `build` at `:63`) belong to Rust, the bundle smoke build, and release — none of them runs Playwright. So the engine that discharges "proven in WebKit" in CI is the Linux WebKit port, whose form-control theming is not the macOS one, and the shipping engine is WKWebView on macOS. The evidence that actually settled this defect came from local macOS runs (`be1f0e6`: "Playwright 12/12 across chromium and webkit"; `22ed41e`: "Playwright 14/14 across chromium and webkit including the previously failing WebKit case").

The invariant is still correctly stated and the spine is careful elsewhere ("a green cross-engine suite is not evidence against it", `:336`). What is missing is that the lane cannot *discharge* it either, for macOS-specific theming.

**Remedy:** one clause in AD-11 stating that the CI WebKit is a Linux build, so a green CI run is not proof for native-appearance controls, and naming what is.

---

### LOW-1 — "a manual Release run built, signed, and notarized on the new image" is unverifiable from this repository

`ARCHITECTURE-SPINE.md:308-309`:

> and a manual Release run built, signed, and notarized on the new image (commit `419dc32`).

`419dc32` touches four files: `.github/workflows/ci.yml`, `.github/workflows/release.yml`, `docs/DECISIONS.md`, `docs/SPEC.md`. Its message asserts "Signing and notarization on the new image are proven by a manual Release run", and `docs/DECISIONS.md` D31 requires exactly that method. But no run id, log, or artifact reference exists in the tree, so the claim rests on the commit message alone. Recorded as UNVERIFIED, not as a defect — D31's stated settlement method was followed and the residual it closes is real.

**Remedy:** cite the workflow run URL alongside the sha.

---

### LOW-2 — the remaining reviewer-gate tail (`6 HIGH, 15 MEDIUM, 5 LOW`) is bookkeeping with no checkable source

`ARCHITECTURE-SPINE.md:1010`. The *input* number verifies exactly (see ledger below); the *remainder* is a running subtraction across revisions 7 and 8 with no per-finding resolution ledger in the tree, so it cannot be confirmed or refuted. Recorded as UNVERIFIED.

---

## Verification ledger — what was checked and what passed

### Commits (19 claims — 19 verified)

| Claim | Result |
| --- | --- |
| `419dc32` exists; `ci: move CI and release builds off the deprecated macos-14 runner` | VERIFIED |
| `419dc32` moved all three pins to `macos-15` | VERIFIED (touches `ci.yml`, `release.yml`) |
| `419dc32` recorded D34 | VERIFIED (`docs/DECISIONS.md:367` `## D34.`) |
| `419dc32` also updated `SPEC.md` §7.6 | VERIFIED (file in commit) |
| `be1f0e6` exists; `feat(ui): adopt the approved design palette and give focus its own ring` | VERIFIED |
| `be1f0e6` recorded D35 | VERIFIED |
| `be1f0e6` moved 22 `focus-visible` sites to `--color-focus-ring` | VERIFIED (22 lines, all `ring-focus-ring`) |
| One `ring-accent` deliberately survives at `PackageRow.tsx` | VERIFIED — `8d36cdf:src/components/manager/PackageRow.tsx:85` `highlighted ? "ring-2 ring-inset ring-accent" : ""`, no `focus-visible:` prefix |
| `src/store/ui.ts:45` is the cross-manager navigation referent | VERIFIED — `/** Cross-manager join navigation target: highlight a row in another manager. */` |
| `7cc7b5f` exists; app-update guard | VERIFIED |
| Enforcement point is Rust: `install_app_update` calls `refuse_app_update_while_busy(&state.queue.records())` before doing anything | VERIFIED — `commands.rs:810` is the first statement of the command |
| Helper refuses when any record is `Queued` or `Running` | VERIFIED — `commands.rs:776-779` |
| Matches the frontend predicate exactly | VERIFIED — `src/store/operations.ts:137` `o.status === "queued" \|\| o.status === "running"` |
| Split into a free function so it is unit-testable | VERIFIED — `fn refuse_app_update_while_busy` at `commands.rs:772` |
| Tests cover empty, all five terminal statuses, both active statuses, and a mixed set | VERIFIED — `commands.rs:862`, `:863-869` (5 terminals), `:879`, `:890` |
| Reuses `ErrorCode::SelfUpdateUnavailable` | VERIFIED — `commands.rs:787` |
| `8d36cdf` exists; seven-passage `epics.md` reconciliation | VERIFIED |
| `8d36cdf` landed under `sprint-change-proposal-2026-07-25-spine-rev8.md` | VERIFIED (file added in that commit) |
| HEAD was `8d36cdf` when revision 9 was written | VERIFIED |

### `.github/workflows/` (8 claims — 8 verified)

- All three macOS pins read `macos-15`: `ci.yml:28`, `ci.yml:70`, `release.yml:63`. VERIFIED.
- Those are jobs `rust` (`ci.yml:27`), `build-smoke` (`ci.yml:68`), `build` (`release.yml:60`). VERIFIED.
- No `runs-on` anywhere in `.github/workflows/` names `macos-14`. VERIFIED (`grep -rn "runs-on" .github/workflows/` — 11 lines, 3 `macos-15`, 8 `ubuntu-latest`).
- All other jobs are `ubuntu-latest`. VERIFIED.
- `test.yml` runs on push to `main` and pull_request to `main`. VERIFIED (`test.yml:9-13`).
- Node in CI is 24 through two mechanisms: `ci.yml:57`, `ci.yml:76`, `release.yml:80` literal `node-version: 24`; `test.yml` × 4 via `node-version-file: .nvmrc`, and `.nvmrc` contains `24`. VERIFIED.
- `release.yml` blocks on `minisign` verification of the base64-decoded detached signature against the embedded pubkey. VERIFIED — `release.yml:315-318`.
- `release.yml` asserts the published `latest.json` reachable and coherent after upload. VERIFIED — `release.yml:388-395` (`curl -fsSL … published.json`, `jq -r .version`, then `curl -fsIL` on `platforms["darwin-aarch64"].url`).
- `latest.json` carries both `darwin-aarch64` and `darwin-x86_64` pointing at one archive. VERIFIED — `release.yml:340-343`.

### `docs/DECISIONS.md` (9 claims — 9 verified)

**D34** (`docs/DECISIONS.md:367`, header `## D34. CI and release build on `macos-15`; the `macos-14` pin is retired`) says, verbatim:

> the three pins — `ci.yml` `rust`, `ci.yml` `build-smoke`, and `release.yml` `build` — were already exposed to intermittent unexplained failures, and after 2026-11-02 no signed, notarized release could be cut at all.

> This also closes the question D31 left open. D31 recorded that a deployment target above the build SDK is a floor annotation rather than an SDK requirement, and that whether `notarytool` accepts `minos 15.0` against SDK 14.5 was OPEN. On `macos-15` the build SDK is no longer behind the declared 15.0 floor, so the mismatch that question was about no longer exists.

> **Rejected:** `macos-latest`. It floats, so a future GitHub default change would move the signing and notarization environment without a commit — the opposite of what D20 wants.

Every characterisation the spine makes of D34 at `ARCHITECTURE-SPINE.md:300-309` and `:990` matches. VERIFIED.

**D31 handling — the spine gets the hard part right.** `ARCHITECTURE-SPINE.md:991`:

> Note D31's own text still reads "CI therefore stays on `macos-14`" and its OPEN paragraph is unedited: D34 supersedes D31 rather than rewriting it, so cite D34 for the closure and never D31 alone.

`docs/DECISIONS.md` D31 verbatim: "CI therefore stays\non `macos-14`." — the quoted fragment is byte-exact across the line wrap. And D31's paragraph beginning "**One question remains OPEN at the time of writing:** whether `notarytool` accepts `minos 15.0` against SDK 14.5." is present and unedited. The spine both **detected** the surviving contradiction and **instructed correctly** on it. VERIFIED, and worth calling out as the strongest single piece of work in this revision.

**D35** (`docs/DECISIONS.md`, `## D35. The approved design palette is adopted and focus gets its own ring`): confirms the stub-palette provenance (`69f37b0`, `a13738d`), the adoption into existing `--color-*` names, the five added tokens (`focusRing`, `shell`, `onAccent`, `onSuccess`, `violet`), "Focus now resolves `--color-focus-ring` (`#F4F7FB`), and the CI assertion moved with it, including a negative guard that the focus ring is not the accent", and the deliberate `PackageRow.tsx:85` survivor. Every spine characterisation at `:319-320` and `:989` matches. VERIFIED.

### Design tokens, SPEC, EXPERIENCE, style contract (13 claims — 13 verified)

- `src/styles/theme.css:8` `--color-bg-base:       #090C13;` VERIFIED.
- `src/styles/theme.css:27` `--color-accent:        #65A7FF;` VERIFIED.
- `src/styles/theme.css:19` `--color-focus-ring:    #F4F7FB;` VERIFIED.
- All 22 `focus-visible` sites at `8d36cdf` resolve `--color-focus-ring` (`ring-focus-ring`). VERIFIED.
- `src/styles/theme.css:61` `@media (prefers-reduced-motion: reduce) {`. VERIFIED.
- `docs/SPEC.md` §4.1 line 206 contains "offset against surface, on every interactive element" — VERIFIED verbatim.
- …and "a dedicated indicator, never `--color-accent`" — VERIFIED verbatim.
- `EXPERIENCE.md:318`: "Every interactive element uses a separate `{colors.focusRing}` indicator that is at least 2px wide and visible against every surface. `{colors.borderStrong}` may indicate selection but never substitutes for focus; selected and focused states remain distinguishable." Both spine quotations (`:349` full-fragment, `:989` with a marked ellipsis) are byte-exact substrings. VERIFIED.
- Spec emulates `{ reducedMotion: "reduce" }`. VERIFIED.
- Spec asserts transitions and animations resolve to `0s`. VERIFIED (`transitionDuration: "0s"`, `animationDuration: "0s"`, plus `transitionProperty: "none"`, `animationName: "none"`).
- **The focus assertion targets a `button`.** VERIFIED — `page.getByRole("button", { name: "Refresh All", exact: true })`. This is the spine's explanation for the green suite and it is correct.
- Negative guard against the accent. VERIFIED — `expect(focusTreatment.boxShadow).not.toContain("rgb(101, 167, 255)")`.
- The disclaimer quotation at `ARCHITECTURE-SPINE.md:326` — "It does not claim measured contrast compliance or validate the native Tauri package." — is byte-exact against the spec's trailing comment. VERIFIED.

### Verified Brownfield Baseline (16 claims — 16 verified)

| Claim | Evidence at `8d36cdf` |
| --- | --- |
| 20 registered commands | `lib.rs` `generate_handler!` — 20 `commands::` entries |
| Six typed events | `events.rs:77-82` — six `EVENT_*` consts |
| `bridge.ts` sole frontend Tauri importer | `git grep -l "@tauri-apps/api" 8d36cdf -- src/` → `src/lib/ipc/bridge.ts` only |
| re-exports exactly `invoke`, `listen`, `UnlistenFn` | `bridge.ts` — three export lines, nothing else |
| 15 committed contract fixtures | `ls dev/fixtures/ipc/ \| wc -l` → 15 |
| `PM_UPDATE_CONTRACT=1 cargo test ipc_contract` | `ipc.rs:546`, `:566`, `:576` |
| `ui.dialog { kind: "upgradePlan" }` | `src/store/ui.ts:20` |
| No `planAttemptId` / `Verifying` / `InteractionRequired` symbol in `src/` or `src-tauri/src/` | `git grep -in` → zero hits for all three |
| `autoOpenDrawer` still active | 5 files reference it |
| `ManagerPane.upgradeRow` → `executePlan` | `ManagerPane.tsx:145-152`, `await executePlan(plan)` |
| `operations.jsonl` compacted to newest 1,000 | `journal.rs:19` `pub const COMPACT_KEEP: usize = 1000;` |
| newest 25 transcripts | `diagnostics.rs:23` `pub const TRANSCRIPTS_INCLUDED: usize = 25;` |
| newest three app logs | `diagnostics.rs:197` "5 app logs → only the newest 3 ship." |
| Minimum macOS 15.0 at `bundle.macOS.minimumSystemVersion` | `src-tauri/tauri.conf.json:48` |
| Version read from `.release-please-manifest.json`, 1.0.1 | `{".":"1.0.1"}` |
| Global cap 4, 120s aging guard | `queue.rs:48` `MAX_CONCURRENCY: usize = 4`, `queue.rs:50` `AGING_GUARD: Duration = Duration::from_secs(120)` |

### Invariant-internal code claims (14 claims — 14 verified)

- Five ports exist as traits: `CommandRunner` (`process/runner.rs:26`), `EventSink` (`events.rs:124`), `UpdateSource` (`app_update.rs:41`), `PendingRelease` (`app_update.rs:48`), `ManagerAdapter` (`managers/mod.rs:67`). VERIFIED.
- `OpStatus` ships seven variants — `Queued, Running, Succeeded, Failed, Cancelled, TimedOut, Interrupted`. VERIFIED.
- AD-19 / AD-21: `set_settings_core` persists before publishing and bumps the revision for every key. VERIFIED — `commands.rs:636-650`: `save_to(...)?` then `*state.settings.write()… = merged` then `coordinator.bump_revision()`, unconditional.
- AD-22: `execute_issued_plan` "No synchronous guard crosses an await" — VERIFIED byte-exact at `commands.rs:353`.
- AD-22: `handle_plan_batch` performs the re-check — VERIFIED, `queue.rs:1006` takes `expected_revision`, `queue.rs:1011` `if coordinator.revision() != expected_revision`.
- AD-22: `plan_coordinator` is a `std::sync::Mutex` — VERIFIED, `state.rs:212` `Arc<Mutex<PlanCoordinator>>` and call sites use `.lock().expect("plan coordinator poisoned")`, the `LockResult` shape (a Tokio mutex would be `.lock().await`).
- AD-25: `ManagerAdapter::parse_recovery` takes `refresh_outputs` alongside the failed output — VERIFIED, `managers/mod.rs:89-94`, with the doc comment "the inventory parsed from them must be merged with the recovered overlay, or every up-to-date package would vanish from the table whenever recovery fires."
- AD-20: `csp` is `null` — VERIFIED, `tauri.conf.json:25`.
- AD-20: one capability file grants exactly `core:default`, `opener:default`, `core:window:allow-start-dragging` — VERIFIED, `src-tauri/capabilities/default.json` is the only file and lists exactly those three.
- AD-26: `src-tauri/Cargo.toml` declares no `[profile.release]` — VERIFIED (`grep -n "profile"` → no match).
- Rust edition 2021 — VERIFIED, `Cargo.toml:6`.

### Stack table (18 rows — 18 verified)

Resolved from `package-lock.json` (`lockfileVersion: 3`) and `src-tauri/Cargo.lock`:

| Spine row | Spine value | Lockfile | Result |
| --- | --- | --- | --- |
| Application version | 1.0.1 | `.release-please-manifest.json` `{".":"1.0.1"}` | VERIFIED |
| Rust edition | 2021 | `Cargo.toml:6` | VERIFIED |
| Tauri Rust crate | 2.11.5 | `Cargo.lock` `tauri` 2.11.5 | VERIFIED |
| Tauri JavaScript API | 2.11.1 | `@tauri-apps/api` 2.11.1 | VERIFIED |
| Tauri CLI | 2.11.4 | `@tauri-apps/cli` 2.11.4 | VERIFIED |
| Tauri updater plugin | 2.10.1 | `tauri-plugin-updater` 2.10.1 | VERIFIED |
| Tauri opener plugin | 2.5.4 | `tauri-plugin-opener` 2.5.4 | VERIFIED |
| Tokio | 1.53.1 | `tokio` 1.53.1 | VERIFIED |
| React / React DOM | 19.2.8 | both 19.2.8 | VERIFIED |
| TypeScript | 7.0.2 | 7.0.2 | VERIFIED |
| Vite | 8.1.5 | 8.1.5 | VERIFIED |
| Tailwind CSS | 4.3.3 | `tailwindcss` and `@tailwindcss/vite` both 4.3.3 | VERIFIED |
| Zustand | 5.0.14 | 5.0.14 | VERIFIED |
| TanStack React Virtual | 3.14.8 | 3.14.8 | VERIFIED |
| Vitest | 4.1.10 | 4.1.10 | VERIFIED |
| Playwright | 1.61.1 | `@playwright/test`, `playwright`, `playwright-core` all 1.61.1 | VERIFIED |
| Node in CI | 24 | `.nvmrc` = `24`; literal `node-version: 24` × 3 | VERIFIED |
| CI runner images | macos-15 ×3, ubuntu-latest otherwise | as enumerated above | VERIFIED |
| Minimum supported macOS | 15.0 | `tauri.conf.json:48` | VERIFIED |
| Release automation | release-please action v5 | `release-please.yml:63`, `:174` `googleapis/release-please-action@v5` | VERIFIED |

**Nothing in the Stack table is stale, and nothing was "corrected" from training-data priors.** TypeScript 7.0.2, Vite 8.1.5, Tailwind 4.3.3, Vitest 4.1.10 and React 19.2.8 are all ahead of common priors and all match the lockfile exactly.

### Counts and enumerations (9 claims — 7 verified, 1 falsified, 1 unverifiable)

| Claim | Result |
| --- | --- |
| Epic UX-PB has 28 stories | VERIFIED — 28 unique `UX-PB.Na` ids in `epics.md` |
| Six surviving Epics 1–6 stories: 2.2, 3.1, 3.2, 3.4, 3.5, 6.5 | VERIFIED — `epics.md:352` lists 3.1/3.2/3.4/3.5/6.5; Story 2.2 at `epics.md:1149` |
| 20 commands / six events / 15 fixtures | VERIFIED |
| `OpStatus` ships seven variants | VERIFIED |
| Eighteen live `AD` ids, every one cited in `epics.md` | VERIFIED — spine defines AD-1..5, 11, 12, 16..26 = 18; `epics.md` cites all 18 |
| AD-7/8/9/14 appear nowhere in `epics.md`; AD-6/10/13/15 only in the retired-id block | VERIFIED — no AD-7/8/9/14 hits; AD-6/10/13/15 only at `epics.md:208`, `:217`, `:219` |
| 22 focus sites, 16 without `ring-offset-*` | VERIFIED — 22 lines; 6 carry offset (`Button.tsx:38`, `Checkbox.tsx:38`, `Chip.tsx:40`, `CopyableCommand.tsx:36`, `PackageRow.tsx:99`, `PackageTable.tsx:110`) |
| Six native checkboxes | VERIFIED |
| "the three controls that render none" | **FALSIFIED — nine** (HIGH-1) |
| v6 lenses returned 44 findings: 5 CRITICAL, 14 HIGH, 18 MEDIUM, 7 LOW | **VERIFIED EXACTLY** — see below |
| Remaining tail 6 HIGH / 15 MEDIUM / 5 LOW | UNVERIFIABLE (LOW-2) |

The 44-finding arithmetic, reconstructed from the four review files:

| Review | C | H | M | L | Total | Source |
| --- | --- | --- | --- | --- | --- | --- |
| `review-currency-v6.md` | 0 | 1 | 1 | 2 | 4 | headings `HIGH-1`, `MEDIUM-1`, `LOW-1`, `LOW-2` |
| `review-divergence-v6.md` | 3 | 5 | 4 | 2 | 14 | `:27` "**Counts:** 3 CRITICAL, 5 HIGH, 4 MEDIUM, 2 LOW. 14 total." |
| `review-reconcile-epics-v6.md` | 1 | 4 | 4 | 0 | 9 | `####` headings from `:292` |
| `review-rubric-v6.md` | 1 | 4 | 9 | 3 | 17 | `:48` "**Finding count:** 1 CRITICAL, 4 HIGH, 9 MEDIUM, 3 LOW (17 total)." |
| **Total** | **5** | **14** | **18** | **7** | **44** | matches `ARCHITECTURE-SPINE.md:1010` exactly |

### `epics.md` divergence batch, items (a)–(g) plus three follow-ups (10 claims — 10 verified)

All checked against `8d36cdf:_bmad-output/planning-artifacts/epics.md`, not against the proposal.

- **(a)** UX-PB.1b, `epics.md:566`: "…the draft is session-scoped and never written to disk, so membership is never reconstructed, never partially restored, and never fabricated, and nothing executes on relaunch" — VERIFIED byte-exact against the spine's quotation.
- **(b)** UX-PB.1c, `epics.md:590`: "**And** no whole-intent `kind` is stored or converted — there is no `AllEligible` value to convert from…" — VERIFIED, and `AllEligible` occurs **exactly once** in the whole file (`grep -o … | wc -l` → 1), as claimed.
- **(c)** UX-PB.4d, `epics.md:984`: "…and takes that separate reviewable object straight to preview and confirmation without ever writing to, merging with, or emptying the one persistent draft" — VERIFIED; `RetryIntent` named at `:975` and `:984`.
- **(d)** Four native-harness locations cite AD-26: `epics.md:172`, `:285`, `:314`, and the Story 6.5 block (`:1294-1296`). VERIFIED — 7 `AD-26` occurrences across 4 locations, none calling the harness simply Deferred (`:172` reads "OPEN — owner Story 6.5; shape named, not yet adopted**, not as a bare deferral").
- **(e)** UX-PB.5b, `epics.md:1045`: "AD-22 (admit, then persist the rider)"; rejected-admission case present. VERIFIED.
- **(f)** Both accessibility passages corrected — `epics.md:265` (DR-2 restatement) and `:310` ("Reduced motion is already automated and runs in CI … automated 4.5:1 contrast does not exist"). VERIFIED.
- **(g)** UX-PB.4b carve-out, `epics.md:948`: "with exactly one carve-out: the non-executing `Retry` affordance UX-PB.4d offers from a History entry." VERIFIED.
- **Follow-up 1** AD-25 went from zero citations to four: `git show 7cc7b5f:…epics.md | grep -c "AD-25"` → **0**; at `8d36cdf` → **4** (`:622`, `:664`, `:827`, `:1159`). VERIFIED.
- **Follow-up 2** Story 3.2 restored to the surviving-story list at `epics.md:352`. VERIFIED.
- **Follow-up 3** Design-token blocker row added at `epics.md:307-308`. VERIFIED that it was added — but see **HIGH-2** for what it now says.

### The two disclosed residuals (2 claims — 2 verified byte-for-byte)

- UX-PB.3d, `epics.md:844`: "**Then** the item does not declare success — it stays `Verifying` until it resolves, then reports verification failure with its evidence, and is never colored successful on the strength of the exit code alone." The spine's quotation at `:1013` is byte-exact, and the AD-25 substance (Last-good Snapshot retention) genuinely does not appear in the criterion — only on the Dependencies line at `:827`. VERIFIED.
- AD-21, `epics.md:1045`: "AD-21 (`skipUpgradePlanConfirmation` is declared plan-inert)" is indeed the only appearance of AD-21 in `epics.md` (`grep -n "AD-21"` → one line), and it is on a Dependencies line, not in criterion prose. VERIFIED.

---

## What this revision did right, for the record

Three things are worth naming because they are the behaviours the previous currency reviews asked for and did not get:

1. **The D31/D34 supersession is handled correctly.** The spine noticed that D31's body still says "CI therefore stays on `macos-14`" and that its OPEN paragraph was never edited, quoted it exactly, and instructed builders to cite D34 and never D31 alone. That is the difference between reconciling and overwriting.
2. **Every Stack row is ahead of a plausible training-data prior and every one matches the lockfile.** Nothing was "corrected" downward from memory.
3. **The external quotations that carry architectural weight are real.** Four `tauri.app` quotations byte-match `llms-full.txt`; the WebKit/Tailwind claim reproduces under measurement and is backed by WebKit source. One paraphrase slipped inside quotation marks (MEDIUM-3), which is a real regression risk but a much smaller one than revision 6's.

## Severity tally

| Tier | Count |
| --- | --- |
| CRITICAL | 0 |
| HIGH | 2 |
| MEDIUM | 5 |
| LOW | 2 |
| **Total** | **9** |

**Claims checked: 138 · verified exactly: 130 · falsified or imprecise: 6 · unverifiable from any available source: 2.**
