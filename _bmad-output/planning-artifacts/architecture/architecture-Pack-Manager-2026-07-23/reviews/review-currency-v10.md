# Reviewer Gate — currency / reality-check lens, revision 10

**Target:** `_bmad-output/planning-artifacts/architecture/architecture-Pack-Manager-2026-07-23/ARCHITECTURE-SPINE.md`
(1053 lines, `artifact_revision: 9`, `updated: "2026-07-25"`).
**Intent:** Validate — report only. This file is the only thing this lens wrote.
**Date:** 2026-07-25. **Prior lenses read:** `review-currency-v6.md`,
`review-currency-v8.md`, `review-currency-v9.md`.

## Method

Every claim below was checked against a file Read this session or a command run
this session, and every finding carries a literal quote. Counts come from
`grep`/`wc`/`sed` with the command stated. Where a claim's source is the web, the
source was downloaded and grepped locally rather than summarized by a fetch tool
— see the note in the AD-26 section, which is the reason this review's first
attempt at AD-26 was wrong.

Given as repo state and therefore not re-derived or reported: the retired PRD in
`_bmad-output/archive/**`; `_bmad-output/project-context.md`'s mid-update
self-contradiction; `docs/*.md` being generated output regenerated today; the
five landings merged to `main` today; `lint_spine.py` returning zero findings.

## Verdict

**The Stack table is exact. All eight counted Brownfield-Baseline claims are
exact. Every cited symbol and file path exists and still says what the spine
says it says. AD-26's four web quotations are byte-exact against the live
source. AD-26's compile-time premise holds.** 21/21 Stack rows, 8/8 counted
baseline claims, 5/5 cited commits, 32/32 code-symbol citations, 4/4 web
quotations verified.

What remains is one **HIGH** and five smaller findings, and the HIGH is the
characteristic failure of this run folder rather than a new one: a *positional /
enumerative* claim that was true when written and has already decayed. Revision
9 wrote a "Caveat for a future currency check" naming which files still carry the
retired `macos-14` pin. That enumeration is now wrong in both directions — two of
the three named files no longer say `macos-14`, and the one file that does say it
in present tense, in a hand-maintained authority AD-11 cites by name, is absent
from the list.

No CRITICAL. Nothing in revision 9's new material (AD-27) is factually wrong
about the tree; AD-27 is the best-grounded new invariant this folder has
produced. Its one defect is a mislabeled sample name (MEDIUM-2).

---

## Findings

### HIGH-1 — the `macos-14` caveat is wrong in both directions: it names two files that no longer say `macos-14`, and omits `docs/DECISIONS.md` D20, which does

`ARCHITECTURE-SPINE.md:1026`:

> **Caveat for a future currency check:** `docs/SPEC.md` §7.6 moved with the
> change, but `docs/development-guide.md`, `docs/index.md`, and
> `_bmad-output/project-context.md` still say `macos-14`. Those are generated
> workflow output — they need a `bmad-document-project` /
> `bmad-generate-project-context` regeneration, not a hand edit, and they are not
> evidence that this row reopened.

**(a) The omission — this is the load-bearing half.** `docs/DECISIONS.md:83`,
inside **D20**, which AD-11 cites as authority for the named-stable-runner rule:

> macOS 27 beta + Xcode-beta codesign/notarization drift is the top platform risk
> (all judges); this is a personal tool with no distribution requirement. CI
> build-smoke runs on stable macos-14 runners; beta-specific issues are diagnosed
> on-machine.

Command: `grep -n "macos-14" docs/DECISIONS.md` → three lines, `83`, `269`, `367`.
`grep -c "macos-14" docs/DECISIONS.md` → `3`. Same three at HEAD
(`git show HEAD:docs/DECISIONS.md | grep -n "macos-14"`). The spine already
acknowledges `:269` — `ARCHITECTURE-SPINE.md:1027` "Note D31's own text still
reads \"CI therefore stays on `macos-14`\"" (verified verbatim across
`docs/DECISIONS.md:268-269`, which wrap as "CI therefore stays\non `macos-14`.").
`:367` is D34's own title. **`:83` is acknowledged nowhere**, and it is the one
that matters:

- It is **present tense** — "CI build-smoke **runs** on stable macos-14 runners"
  — not a superseded historical record like D31's.
- It is **hand-maintained**, so the caveat's blanket disposition ("generated
  workflow output — they need a regeneration, not a hand edit") is the wrong
  remedy for it. Nothing regenerates `docs/DECISIONS.md`.
- AD-11 sends a builder straight to it. `ARCHITECTURE-SPINE.md:306-308`:
  "CI and release build on a **named stable runner image, never `macos-latest`**
  — a floating label would move the signing and notarization environment without
  a commit (`docs/DECISIONS.md` D20, D34)." A builder who follows the D20
  citation to check the runner rule reads "stable macos-14 runners" and finds the
  cited authority contradicting the spine — the exact failure mode
  `review-currency-v8.md` H3 and `review-currency-v9.md` established, arriving a
  third time through the one file the caveat forgot.

**(b) The over-claim.** Two of the three named files no longer say `macos-14`:

| File | `grep -c macos-14` (worktree) | `grep -c macos-15` |
| --- | --- | --- |
| `docs/index.md` | `0` | `2` |
| `docs/development-guide.md` | `1` | `1` |
| `docs/DECISIONS.md` | `3` | `3` |

`docs/index.md:85` now reads:

> - macOS and Apple command-line build tools. The shipped bundle declares a
>   `15.0` floor; CI builds on `macos-15` (D34).

`docs/development-guide.md:9` now reads:

> - macOS for the supported desktop target and real package-manager smoke tests.
>   The shipped bundle declares `bundle.macOS.minimumSystemVersion` `15.0`
>   (`DECISIONS.md` D31); CI builds on `macos-15` (D34), which moved all three
>   runner pins off the `macos-14` images GitHub began deprecating on 2026-07-06.

Its single surviving `macos-14` is a historical clause inside a sentence that
states the current pin — not the spine's "still say `macos-14`".

**In fairness to revision 9:** the caveat was true at HEAD.
`git show HEAD:docs/index.md | grep -c 'macos-14'` → `1`, and
`git show HEAD:docs/development-guide.md | grep -n "macos-1"` → line 9 "CI builds
on `macos-14` because a deployment target above the build SDK is a floor
annotation, not an SDK requirement." The `bmad-document-project` regeneration the
caveat *prescribes* is what invalidated it, and it is sitting uncommitted in the
tree (`git status --porcelain docs/` → ` M docs/index.md`, ` M
docs/development-guide.md`). So the caveat was correct, was acted on, and decayed
in the same day — which is precisely why an enumeration of *which files are
stale* does not belong in a spine.

**Severity HIGH, not MEDIUM,** for one reason: the caveat's whole purpose is to
brief the next currency check, and it misdirects that check away from the only
residual that a regeneration will never clear.

**Remedy.** Drop the file enumeration. Replace with the durable statement: D20
and D31 both name `macos-14` in text D34 supersedes without rewriting; cite D34
for the runner, never D20 or D31 alone. That claim cannot decay.

---

### MEDIUM-1 — the design-token row still says "All 22 `focus-visible` sites", present tense, while the table's own next-but-one row records the move to 31

`ARCHITECTURE-SPINE.md:1025`:

> All 22 `focus-visible` sites resolve `--color-focus-ring`, which is what
> `EXPERIENCE.md`'s "a separate `{colors.focusRing}` indicator … selected and
> focused states remain distinguishable" requires

`ARCHITECTURE-SPINE.md:1052`:

> corroborated by the site count going from 22 to 31.

Commands: `grep -rn "focus-visible" src/ | wc -l` → `31`.
`grep -rno "focus-visible:outline[a-z0-9-]*" src/ | sed 's/.*://' | sort | uniq -c` →
`31 outline-2`, `31 outline-focus-ring`, `30 outline-offset-1`, `1
outline-offset-2`. So every one of the 31 sites carries `outline-2` +
`outline-focus-ring` + an offset, exactly as AD-27 requires — the *substance* of
the row is stronger than it claims. Only the number is stale, and the same table
contradicts it 27 lines later.

`22` was the count at `8d36cdf`/`be1f0e6`; `22ed41e` took it to 31.
`review-currency-v9.md` HIGH-1 established that correction and revision 9 folded
it into the Focus-indicator row (`:1052`) — but left `:1025` unedited. **Partial
REPEAT: this is the un-swept residual of v9 HIGH-1, not a new count error.**

The row is otherwise exact. Verified: `src/styles/theme.css:8`
`--color-bg-base:       #090C13;`, `:27` `--color-accent:        #65A7FF;`, `:19`
`--color-focus-ring:    #F4F7FB;`; the five added tokens at `:9`
`--color-bg-shell`, `:19` `--color-focus-ring`, `:30` `--color-on-accent`, `:32`
`--color-on-success`, `:36` `--color-violet`; `DESIGN.md:15` `background:
"#090C13"`, `:23` `focusRing: "#F4F7FB"`, `:27` `accent: "#65A7FF"`;
`EXPERIENCE.md:318` "Every interactive element uses a separate
`{colors.focusRing}` indicator that is at least 2px wide and visible against
every surface. `{colors.borderStrong}` may indicate selection but never
substitutes for focus; selected and focused states remain distinguishable.";
`docs/SPEC.md:208` "a dedicated indicator, never `--color-accent`"; the sole
surviving `ring-accent` at `src/components/manager/PackageRow.tsx:85`
`highlighted ? "ring-2 ring-inset ring-accent" : "",`.

**Remedy.** `:1025` → "All focus sites resolve `--color-focus-ring` (22 at
`be1f0e6`; 31 after `22ed41e`)", or drop the number.

---

### MEDIUM-2 — AD-27 calls its second style-contract sample "the package-row plan-membership checkbox"; the shipping control is a *selection* checkbox, and the spine's own Open row says that distinction is unmodeled

`ARCHITECTURE-SPINE.md:929-931`:

> - **Rule:** The style contract proves the mechanism on **named samples, not a
>   sweep**: today a toolbar `<button>` and the package-row plan-membership
>   checkbox, chosen because they sit on opposite sides of the `appearance`
>   discriminator.

The control the spec actually focuses is the row-selection checkbox.
`tests/e2e/browser-style-contract.spec.ts:191`:

> await test.step("When the row is selected from the keyboard, leaving that checkbox focused", async () => {

and the element it renders, `src/components/manager/PackageRow.tsx:96`:

> aria-label={`Select ${pkg.name}`}

There is no plan-membership control in the tree to sample.
`grep -rn "planAttemptId\|PlanAttemptId\|Verifying\|InteractionRequired" src/ src-tauri/src/`
→ **no matches**, which is the spine's own baseline claim at `:143-145`. And the
spine's own Open row, `ARCHITECTURE-SPINE.md:1050`:

> No `AD` models the relationship between transient row selection and
> `PlanIntent` membership … `src/store/packages.ts` ships a live `selection` set
> that `PlanIntent` cannot represent.

Verified at `src/store/packages.ts:17` `selection: Partial<Record<ManagerId,
Set<string>>>;`.

**Why this is more than a naming nit.** AD-27's next sentence is "No story may
read a green run as proof that the element *it* added has a visible focus state."
A UX-PB.1a builder reads that the plan-membership checkbox is already a named
sample and concludes the control *they* are adding is covered. The mislabel
invites the exact misread the rule exists to forbid, in the same rule.

The first sample name is correct — `tests/e2e/browser-style-contract.spec.ts:74-77`
targets `page.getByRole("button", { name: "Refresh All", exact: true })`, and its
guard is exact: `expect(focusTreatment.outlineStyle).toBe("solid")`,
`.outlineWidth).toBe("2px")`, `.outlineColor).toBe("rgb(244, 247, 251)")`,
`.outlineColor).not.toBe("rgb(101, 167, 255)")` — which discharges AD-11's "a
negative guard against the accent" (`#65A7FF` = `rgb(101, 167, 255)`) exactly.

**Remedy.** "the package-row **selection** checkbox", plus one clause stating
that no plan-membership control exists yet, so the first story to add one owns
its own runtime focus verification.

---

### LOW-1 — the Stack table's preamble attributes to two lockfiles six rows neither lockfile contains

`ARCHITECTURE-SPINE.md:963`:

> Verified against `package-lock.json` and `src-tauri/Cargo.lock` on 2026-07-25.

Fifteen of the 21 rows do come from those two files and all fifteen are exact
(ledger below). Six do not, and all six verify against their real sources:

| Row | Real source | Verified value |
| --- | --- | --- |
| Application version | `.release-please-manifest.json` | `{".":"1.0.1"}` |
| Rust edition | `src-tauri/Cargo.toml:6` | `edition = "2021"` |
| Node in CI | `.github/workflows/ci.yml:57`, `:76`, `release.yml:80` (`node-version: 24`) + `.nvmrc` (`24`) | 24 |
| CI runner images | `ci.yml:28`, `ci.yml:70`, `release.yml:63` | `runs-on: macos-15` |
| Minimum supported macOS | `src-tauri/tauri.conf.json:48` | `"minimumSystemVersion": "15.0"` |
| Release automation | `.github/workflows/release-please.yml:63`, `:174` | `uses: googleapis/release-please-action@v5` |

The row *bodies* name their sources correctly; only the preamble over-claims.
`review-currency-v6.md` LOW-1 raised this for the Node row specifically —
**REPEAT, widened to the other five.** No factual error, so LOW.

**Remedy.** "Verified against the lockfiles, the workflows, and
`tauri.conf.json` on 2026-07-25", or move the two-file attribution into the rows
it actually covers.

---

### LOW-2 (REPEAT of `review-currency-v9.md` LOW-1, sharpened) — "proven by a manual Release run" is attributed to D34, and D34 closes the question by reasoning instead

`ARCHITECTURE-SPINE.md:1026`:

> Signing and notarization on the new image were proven by a manual Release
> workflow run rather than asserted.

`ARCHITECTURE-SPINE.md:1027`:

> is closed by D34, and closed the way D31 required, by a manual Release run
> rather than by assertion.

D34's own text closes it by *inference*, not by a run. `docs/DECISIONS.md:381-385`:

> This also closes the question D31 left open. D31 recorded that a deployment
> target above the build SDK is a floor annotation rather than an SDK
> requirement, and that whether `notarytool` accepts `minos 15.0` against SDK
> 14.5 was OPEN. On `macos-15` the build SDK is no longer behind the declared
> 15.0 floor, so the mismatch that question was about no longer exists.

No run, log, or artifact is cited anywhere in D34. The manual-run claim exists in
exactly one place, `git log -1 --format=%B 419dc32`:

> Verified: all six workflow files still parse as YAML; vitest 23 files / 133
> tests pass. Signing and notarization on the new image are proven by a manual
> Release run, which is what D31 said was the only way to settle it.

So the claim **is** sourced — v9 was right to call it UNVERIFIED rather than
false — but the spine attributes to D34 a settlement method D34 does not use.
Both mechanisms are individually sound (D34's reasoning does close the SDK-vs-floor
mismatch; the commit body does assert a run), and the spine merges them into one
sentence that neither source supports on its own.

**Remedy.** Split: "D34 closes the mismatch by reasoning — on `macos-15` the SDK
is no longer behind the floor. `419dc32`'s commit body additionally reports a
manual Release run; no run id is recorded in the tree."

---

### LOW-3 — AD-27's Tailwind-4 parenthetical calls `outline-hidden` "the v3 no-op"; in the installed Tailwind it is not a no-op

`ARCHITECTURE-SPINE.md:909-911`:

> **`ring-*` is forbidden, and `outline-none` is never added to a focusable
> element** — under Tailwind 4 `outline-none` genuinely sets `outline-style:
> none` (the v3 no-op was renamed `outline-hidden`), so it actively suppresses
> the indicator.

**The load-bearing half verifies exactly** against the installed Tailwind 4.3.3
(`node_modules/tailwindcss/package.json` version `4.3.3`; located in
`node_modules/tailwindcss/dist/lib.mjs` at byte offset 123308 via Python
`str.find`, because the file is minified and single-line):

> i.static("outline-hidden",()=>[a("--tw-outline-style","none"),a("outline-style","none"),B("@media","(forced-colors: active)",[a("outline","2px solid transparent"),a("outline-offset","2px")])]),t("outline-none",[["--tw-outline-style","none"],["outline-style","none"]])

`outline-none` → `outline-style: none`. Confirmed, and `grep -rn "outline-none"
src/` → **no matches**, so the rule is obeyed.

The parenthetical is the imprecise part: v4's `outline-hidden` also sets
`outline-style: none`, and adds a `forced-colors: active` fallback (`outline: 2px
solid transparent`). It is the *successor* to v3's `outline-none`, not a
transplanted no-op. Substance unaffected; the reason to note it is that this
folder has twice been caught stating a library behavior more confidently than the
source supports.

**Remedy.** "(v3's `outline-none` behavior now lives under `outline-hidden`,
which is not a no-op either — it sets `outline-style: none` plus a forced-colors
fallback)".

---

## AD-26 — verified exact, and a note on how

All four web quotations are **byte-exact** against the live source, fetched
today. Method matters here: a WebFetch of `https://tauri.app/llms-full.txt`
returned "**no content** about WebDriver, tauri-driver, @wdio/tauri-service, or
tauri-plugin-wdio-webdriver" — which is false, and would have produced a
CRITICAL finding had I trusted it. The file is 2,430,050 bytes
(`curl -sIL https://tauri.app/llms-full.txt | grep -i content-length` →
`content-length: 2430050`) and the fetch tool summarized a truncated prefix. I
downloaded it and grepped:

| Spine | Source | Text |
| --- | --- | --- |
| `:866-869` | `llms-full.txt:9532` | "Driven directly, only Windows and Linux are supported on desktop, as macOS has no WKWebView driver tool available (use the service's embedded WebDriver server for macOS)." — exact; source uses a typographic apostrophe in "service's" |
| `:870` | `llms-full.txt:9483` | "which works on **Windows, Linux, and macOS**" — exact |
| `:871-872` | `llms-full.txt:9485` | "By default the service runs an **embedded WebDriver server** inside your app" — exact |
| `:872` | `llms-full.txt:9511` | "**`tauri-plugin-wdio-webdriver`** runs the embedded WebDriver server." — exact |
| `:1034` | `llms-full.txt:9485` | "(a paid API key is required for macOS)" — exact; **`review-currency-v9.md` MEDIUM-3 is RESOLVED**, the paraphrase-in-quotes is gone |

Commands: `grep -c -F "no WKWebView driver tool available" llms-full.txt` → `2`;
`"Windows, Linux, and macOS"` → `2`; `"embedded WebDriver server"` → `3`;
`"tauri-plugin-wdio-webdriver"` → `1`; `"paid API key is required for macOS"` →
`1`.

**Judgment on currency:** still current. The page's own framing is unchanged from
what `review-currency-v8.md` established, and the load-bearing distinction —
`tauri-driver` driven directly excludes macOS; the wdio service covers macOS only
by embedding a server in the app — is stated twice on the page
(`:9532` and `:10235`) and is the basis of AD-26's trust-boundary reasoning. No
newer macOS route appeared.

**AD-26's compile-time premise holds.** `ARCHITECTURE-SPINE.md:883-884`:
"`src-tauri/Cargo.toml` declares no `[profile.release]`, so `debug-assertions` is
off in release builds today and the gate holds." Command:
`grep -n "edition\|\[profile" src-tauri/Cargo.toml` → one line only, `6:edition =
"2021"`. There is no `[profile` stanza of any kind in the file.

---

## Verification ledger

### Stack table — 21/21 rows exact

npm rows read from `package-lock.json` via `python3` (`packages[…]["version"]`):
`@tauri-apps/api` 2.11.1 · `@tauri-apps/cli` 2.11.4 · `react` 19.2.8 ·
`react-dom` 19.2.8 · `typescript` 7.0.2 · `vite` 8.1.5 · `tailwindcss` 4.3.3 ·
`zustand` 5.0.14 · `@tanstack/react-virtual` 3.14.8 · `vitest` 4.1.10 ·
`@playwright/test` 1.61.1 (`playwright` 1.61.1 also present).

Cargo rows read from `src-tauri/Cargo.lock` via `grep -n -A2 '^name = "…"$'`:
`tauri` `:3832-3833` version `2.11.5` · `tauri-plugin-updater` `:3983-3984`
`2.10.1` · `tauri-plugin-opener` `:3961-3962` `2.5.4` · `tokio` `:4243-4244`
`1.53.1`.

Non-lockfile rows: see LOW-1's table — all six verified against their real
sources. Job names verified to exist: `ci.yml:27` `rust:`, `ci.yml:68`
`build-smoke:`, `release.yml:60` `build:` (`review-currency-v8.md` L2 is
RESOLVED). `.release-please-manifest.json` = `{".":"1.0.1"}`;
`src-tauri/tauri.conf.json:4` `"version": "1.0.1",`; `package-lock.json` root
`"version": "1.0.1"` — all three agree, so `:968` and `:157-158` are consistent.

### AD-11 / D34 runner claims — exact

`grep -rn "runs-on" .github/workflows/`: `ci.yml:28` `runs-on: macos-15`,
`ci.yml:70` `runs-on: macos-15`, `release.yml:63` `runs-on: macos-15`; the other
eight jobs are `ubuntu-latest` (`ci.yml:52`, `test.yml:30/56/117/161`,
`release-please.yml:40`, `claude-code-review.yml:16`, `claude.yml:20`).
`grep -rn "macos-14" .github/workflows/` → two hits, both comments
(`ci.yml:10`, `release.yml:61`); **zero `runs-on` names `macos-14`**. `docs/SPEC.md:805`
now reads "rust (macos-15)" and "build-smoke (macos-15, main only)".

### Verified Brownfield Baseline — 8/8 counted claims exact

| Claim | Command | Result |
| --- | --- | --- |
| 20 Tauri commands | `sed -n '233,252p' src-tauri/src/lib.rs \| grep -c "commands::"` | `20` |
| six typed events | `src-tauri/src/events.rs:77-82`, six `pub const EVENT_*`; `:84` "/// One of the six events, name + typed payload." | 6 |
| 15 contract fixtures | `ls dev/fixtures/ipc/*.json \| wc -l` | `15` |
| newest 1,000 records | `src-tauri/src/journal.rs:19` `pub const COMPACT_KEEP: usize = 1000;` | 1000 |
| newest three app logs | `src-tauri/src/diagnostics.rs:22` `pub const APP_LOGS_INCLUDED: usize = 3;` | 3 |
| newest 25 transcripts | `src-tauri/src/diagnostics.rs:23` `pub const TRANSCRIPTS_INCLUDED: usize = 25;` | 25 |
| min macOS 15.0 | `src-tauri/tauri.conf.json:48` `"minimumSystemVersion": "15.0"` | 15.0 |
| app version 1.0.1 | `.release-please-manifest.json` | 1.0.1 |

Also exact: `grep -rn "@tauri-apps" src/ | grep -v bridge.ts` → **empty**, so
`src/lib/ipc/bridge.ts` is the sole frontend importer, and it re-exports exactly
what `:194-196` says (`export { invoke } from "@tauri-apps/api/core";`,
`export { listen } from "@tauri-apps/api/event";`,
`export type { UnlistenFn } from "@tauri-apps/api/event";`).
`src-tauri/src/ipc.rs:545-546` "// Contract test (SPEC §7.4) — byte-equality
against dev/fixtures/ipc/*.json." / "// Regenerate with `PM_UPDATE_CONTRACT=1
cargo test ipc_contract`."; the TS half at `src/lib/ipc/types.test.ts:56`
`it("covers exactly the committed fixture set", () => {`.
`src/store/ui.ts:20` `| { kind: "upgradePlan"; plan: UpgradePlan }` and `:116`
`closeDialog: () => set({ dialog: { kind: "none" } }),`.
`src-tauri/src/state.rs:51` `pub struct PlanCoordinator {`, `:58` `pub fn
revision(&self) -> u64 {`. `src/components/settings/SettingsView.tsx:134`
`checked={settings.autoOpenDrawer}`. `src-tauri/src/settings.rs:124` "/// Atomic:
temp file + fsync + rename"; `src-tauri/src/journal.rs:179-180` "/// The rewrite
is ATOMIC: content goes to a sibling temp file (fsynced), then / `rename`
replaces `operations.jsonl` in one step."

Both release gates exist and both are live on the automated path:
`release.yml:319` `minisign -V -p "$RUNNER_TEMP/updater.pub" -x
"$RUNNER_TEMP/updater.minisig" -m "$UPDATER_TGZ" \` and `release.yml:382`
`- name: Verify published updater endpoint`. That second step is gated
`if: inputs.attach_to_tag != ''` — I checked whether that skips it on the real
release path, and it does not: `release-please.yml:192-194` calls
`uses: ./.github/workflows/release.yml` `with: attach_to_tag: ${{
needs.release-please.outputs.tag_name }}`. **Not a finding.**

### Cited symbols and paths — 32/32 exist and still say what the spine says

`src-tauri/src/commands.rs:636` `fn set_settings_core(state: &AppState, patch:
&SettingsPatch) -> Result<Settings, IpcError> {` (AD-19, AD-21) ·
`src/store/operations.ts:137` `.filter((o): o is OpView => !!o && (o.status ===
"queued" || o.status === "running"));` (the `activeOps` predicate `:1045` claims
matches the Rust guard) · `commands.rs:772` `fn
refuse_app_update_while_busy(records: &[crate::ipc::OperationRecord]) ->
Result<(), IpcError> {`, `:806` `pub async fn install_app_update(`, `:810`
`refuse_app_update_while_busy(&state.queue.records())?;` — **the spine's `:772`
and `:810` line numbers are both exact** · `src-tauri/src/error.rs:76`
`SelfUpdateUnavailable,` (the `ErrorCode` variant `:1045` says was reused) ·
`src-tauri/src/managers/mod.rs:67` `pub trait ManagerAdapter` with
`parse_recovery` implemented in `npm.rs:107`, `mas.rs:91`, `rustup.rs:99`,
`brew.rs:193`, and called at `queue.rs:1686` `match
adapter.parse_recovery(&failed_planned, &outputs, &out) {` — three arguments,
`refresh_outputs` among them, exactly as AD-25 states ·
`src/store/packages.ts:17` `selection: Partial<Record<ManagerId, Set<string>>>;`
· five ports all present: `process/runner.rs:26` `pub trait CommandRunner`,
`events.rs:124` `pub trait EventSink`, `app_update.rs:41` `pub trait
UpdateSource`, `app_update.rs:48` `pub trait PendingRelease`,
`managers/mod.rs:67` `pub trait ManagerAdapter` · `queue.rs:48` `pub const
MAX_CONCURRENCY: usize = 4;`, `:50` `pub const AGING_GUARD: Duration =
Duration::from_secs(120);` · `commands.rs:353` "/// enqueue the complete batch.
No synchronous guard crosses an await." and `queue.rs:1003` `fn
handle_plan_batch(` / `:1011` `if coordinator.revision() != expected_revision {`
— AD-22's quoted comment and re-check both exact · `state.rs:212` `pub
plan_coordinator: Arc<Mutex<PlanCoordinator>>,` (AD-22's "not reentrant" premise)
· `src/components/manager/ManagerPane.tsx:145` `async function upgradeRow(pkg:
Package) {` / `:152` `await executePlan(plan);` (the call site AD-16 retires) ·
`src-tauri/src/ipc.rs:99-107` `pub enum OpStatus` with exactly seven variants ·
`src-tauri/capabilities/default.json` grants exactly `"core:default"`,
`"opener:default"`, `"core:window:allow-start-dragging"` ·
`src-tauri/tauri.conf.json:25` `"csp": null` · `src-tauri/src/commands.rs:672`
`tauri_plugin_opener::reveal_item_in_dir(…)` and `:681`
`tauri_plugin_opener::open_path(…)` — both un-ported direct calls as `:1038`
says · `src/store/index.ts` exists (cross-store derived state) ·
`src-tauri/src/registry.rs:27` `id.split_once(':').map(|(_, name)| name)` (split
on the first colon only) · `docs/RELEASE-CHECKLIST.md:93` "One VoiceOver pass
over the Upgrade Plan announces state changes and completion."

### Accessibility lane — exact, and AD-27's WebKit caveat is right

`tests/e2e/browser-style-contract.spec.ts:47` `await page.emulateMedia({
reducedMotion: "reduce" });` and `:122`/`:124` `transitionDuration: "0s"` /
`animationDuration: "0s"`; `src/styles/theme.css:61` `@media
(prefers-reduced-motion: reduce) {`; the disclaimer at `:128-129` "This is a
browser DOM/CSS contract only. It does not claim measured contrast compliance or
validate the native Tauri package." — quoted correctly at `:329-330`.
`.github/workflows/test.yml:9-12` `push: branches: [main]` / `pull_request:
branches: [main]`, so AD-11's "on every push and pull request to `main`" is
exact. AD-27's final rule verifies: `playwright.config.ts:81` `name: "chromium"`,
`:85` `name: "webkit"` with `use: { ...devices["Desktop Safari"] }`, and the job
that runs them is `test.yml:56` `runs-on: ubuntu-latest` — Linux WebKit, exactly
as AD-27 says. **`review-currency-v9.md` MEDIUM-5 is RESOLVED by AD-27's
existence.**

`grep -rn "outline-none" src/` → none. `grep -rn "ring-" src/` → six hits, one
product focus-adjacent (`PackageRow.tsx:85` `ring-2 ring-inset ring-accent`), two
its unit-test assertions (`managerPane.test.tsx:114`, `:118`), three unrelated
ring-*buffer* prose. `grep -rn "outline-focus-ring" src/ | grep -v
"focus-visible:outline-focus-ring"` → one hit, `managerPane.test.tsx:115`
`expect(highlighted.className).not.toContain("outline-focus-ring");` — a negative
assertion, so the "exactly one `ring-accent` survivor, deliberately not a focus
state" claim at `:1052` is exact and now test-pinned.

### Commits — 5/5 resolve, subjects match

`git log --oneline -1 <sha>`: `be1f0e6 feat(ui): adopt the approved design
palette and give focus its own ring` · `419dc32 ci: move CI and release builds
off the deprecated macos-14 runner` · `22ed41e fix(a11y): draw keyboard focus
with an outline so it is visible in WKWebView` · `8d36cdf docs(planning):
reconcile the last seven epics.md passages against the spine` · `7cc7b5f
fix(update): refuse an app update while package operations are in flight`.
`:1052`'s test numbers verify against `git log -1 --format=%B 22ed41e`:
"Verified: vitest 134/134, tsc --noEmit clean, test:e2e:typecheck clean,
Playwright 14/14 across chromium and webkit including the previously failing
WebKit case".

`docs/DECISIONS.md` D34 at `:367` and D35 at `:392` both exist under the titles
the spine cites; D34's `macos-latest` rejection (`:387-389`) matches `:1026`'s
account verbatim in substance: "**Rejected:** `macos-latest`. It floats, so a
future GitHub default change would move the signing and notarization environment
without a commit — the opposite of what D20 wants."

### Prior-lens dispositions

| Prior finding | Status now |
| --- | --- |
| v9 HIGH-1 (three vs nine controls) | RESOLVED in AD-27 `:926-927` and `:1052`; **residual count at `:1025` is MEDIUM-1** |
| v9 HIGH-2 (`epics.md` asserts the opposite of D35) | Tracked by the spine itself at `:1049` items (3)(4); out of a Validate lens's remit |
| v9 MEDIUM-2 (WebKit mechanism) | Folded into AD-27 `:914-920`; the `appearance` discriminator is now stated as the cause |
| v9 MEDIUM-3 (paraphrase in quotes, CrabNebula) | **RESOLVED** — `:1034` now quotes `llms-full.txt:9485` verbatim |
| v9 MEDIUM-4 (Focus row label false) | **RESOLVED** — `:1052` is `**RESOLVED**` citing `22ed41e` |
| v9 MEDIUM-5 (CI WebKit is Linux) | **RESOLVED** — AD-27's last rule states it |
| v9 LOW-1 (manual Release run unverifiable) | **REPEAT — LOW-2 above**, sharpened: the spine attributes the method to D34, which does not use it |
| v9 LOW-2 (reviewer-gate tail bookkeeping) | Still UNVERIFIED at `:1046`; unchanged, not re-reported |
| v8 C1 / H1 (fabricated AD-26 quote) | **RESOLVED** — all four quotations byte-exact |
| v8 H2 (undecidability over-claimed) | **RESOLVED** — `:1034` now names the compliant composition and the residual procurement question |
| v8 H3 (`macos-14` clock) | **RESOLVED** by `419dc32`/D34; **its documentation residual is HIGH-1 above** |
| v8 L2 (Stack names nonexistent CI jobs) | **RESOLVED** — `rust`, `build-smoke`, `build` all exist |
| v6 LOW-1 (Node in CI has two mechanisms) | **REPEAT, widened — LOW-1 above** |

---

## What revision 9 got right, for the record

- Every counted claim it added or kept is exact. The eight Brownfield-Baseline
  numbers, the 20/6 IPC baseline, the seven `OpStatus` variants, the three
  capability permissions, the concurrency cap and aging guard, the 31 focus
  sites at `:1052`, `134/134` and `14/14` — none estimated, all checkable.
- It fixed all four AD-26 citation defects from v8 and v9, including the
  paraphrase-in-quotation-marks that a lesser pass would have waved through.
- AD-27 is the first invariant in this folder whose evidence is a *runtime*
  observation with its own stated limits ("no grep, and no green suite,
  substitutes for that"), and it correctly refuses to let the CI `webkit`
  project stand in for WKWebView.
- `:1049` names its own positional-reference failure explicitly — "the same
  positional-reference failure this run folder has now hit three times (rule
  ordinals, `epics.md` line numbers, and now spine line numbers)". HIGH-1 is
  that failure a fourth time, in a form the note did not anticipate: an
  enumeration of *stale files* rather than of line numbers.

## Severity tally

| Severity | Count |
| --- | --- |
| CRITICAL | 0 |
| HIGH | 1 |
| MEDIUM | 2 |
| LOW | 3 |
| **Total** | **6** |

Of the six, two are explicit REPEATs (LOW-1 of v6 LOW-1; LOW-2 of v9 LOW-1) and
one is the un-swept residual of a v9 finding (MEDIUM-1 of v9 HIGH-1). Claims
checked this session: 21 Stack rows + 8 counted baseline claims + 32 symbol/path
citations + 4 web quotations + 5 commits = **70 checked, 66 verified exactly, 4
carrying the findings above**.

## Sources

Repo files Read or grepped this session: `ARCHITECTURE-SPINE.md`,
`package-lock.json`, `src-tauri/Cargo.lock`, `src-tauri/Cargo.toml`,
`src-tauri/tauri.conf.json`, `.release-please-manifest.json`,
`release-please-config.json`, `.nvmrc`, `src-tauri/capabilities/default.json`,
`.github/workflows/{ci,test,release,release-please,claude-code-review,claude}.yml`,
`src-tauri/src/{lib,commands,events,ipc,state,queue,journal,settings,diagnostics,error,registry}.rs`,
`src-tauri/src/managers/{mod,npm,brew,mas,rustup}.rs`,
`src-tauri/src/{app_update.rs,process/runner.rs}`,
`src/lib/ipc/{bridge.ts,types.test.ts}`,
`src/store/{ui,packages,operations,index}.ts`,
`src/components/manager/{PackageRow,ManagerPane}.tsx`,
`src/components/settings/SettingsView.tsx`, `src/styles/theme.css`,
`tests/e2e/browser-style-contract.spec.ts`, `playwright.config.ts`,
`docs/{DECISIONS,SPEC,RELEASE-CHECKLIST,index,development-guide,deployment-guide}.md`,
`_bmad-output/planning-artifacts/ux-designs/ux-Pack-Manager-2026-07-23/{DESIGN,EXPERIENCE}.md`,
`node_modules/tailwindcss/dist/lib.mjs`.

Web: `https://tauri.app/llms-full.txt` (downloaded 2026-07-25, 2,430,050 bytes,
grepped locally); `https://v2.tauri.app/develop/tests/webdriver/` (cross-check,
same wording).
