# Rubric walk — ARCHITECTURE-SPINE.md revision 9 (post-AD-27)

**Lens:** rubric walker (good-spine checklist), independent, read-only
**Intent:** Validate — report only. No file outside this one was modified.
**Target:** `_bmad-output/planning-artifacts/architecture/architecture-Pack-Manager-2026-07-23/ARCHITECTURE-SPINE.md`, `artifact_revision: 9`
**Size:** `wc -l ARCHITECTURE-SPINE.md` → `1052` (the final line carries no trailing newline, so the file reads as 1053 lines)
**Tree state:** HEAD = `5972109` "Clear the epics.md residual pile (#36)"; `git log --oneline -6` confirms `c8c1f9a`, `8a4cf6a`, `03e03fa` beneath it
**Date:** 2026-07-25

> **This pass reviews the spine *after* the AD-27 split.** `review-rubric-v9.md`
> reviewed the same `artifact_revision: 9` before AD-27 existed (its citations run to
> `spine:1019`; the file now runs to 1053) and recommended exactly this split at its
> §1. Its two CRITICALs are **closed** and are not repeated:
>
> - **v9 C-1** — the focus rules were unreachable by the stories that draw focus.
>   Fixed: `spine:899-901` now files them as `AD-27` with `Binds` reaching stories,
>   and `epics.md` cites AD-27 32 times (`grep -c "AD-27" epics.md` → `32`).
> - **v9 C-2** — the rule was weaker than D35. Fixed: `spine:907-912` now mandates
>   one mechanism and refuses the `appearance: none` escape by name, matching
>   `docs/SPEC.md:208` almost word for word.
> - **v9 H-3** (a `docs/SPEC.md` §4.1 quote that no longer existed) is closed —
>   `grep -n "offset against surface" ARCHITECTURE-SPINE.md` returns nothing.
> - **v9 M-4** (`RELEASE-CHECKLIST.md` contradicting the app-update closure) is
>   closed — `docs/RELEASE-CHECKLIST.md:86` now reads "`install_app_update` refuses
>   independently (`src-tauri/src/commands.rs:810`)".
> - **v9 H-5** (`epics.md` contradicting two RESOLVED rows) is closed by `5972109` —
>   `epics.md:308` now reads "| Canonical design-token set | `CLOSED` — D35 |".
>
> Where a finding below overlaps something already tracked, it says so in its first
> line and names the tracked row or prior finding.

---

## Verdict

**READY WITH FIXES.** The AD-27 split was the right surgery and it landed cleanly:
the rule is now enforceable at the altitude that owns it, its "named tech" claims
all verify against the installed toolchain, and it ratifies `docs/SPEC.md:208`
rather than contradicting it. The five closures the revision-9 header claims are
real, and I re-verified four of them independently.

The one finding that should gate work is **C-1**: the spine's two new
`OPEN — new architecture` rows record a divergence that can produce two
incompatible builds, and neither row carries a blocking relationship — while the
register the builders actually read says "nothing blocks starting it"
(`epics.md:307`). The first story in the primary build queue, UX-PB.1a, already
takes one side of the undecided question in its own acceptance criteria. Everything
else can follow.

### Tally

| Tier | Count | Of which REPEAT / tracked |
| --- | --- | --- |
| CRITICAL | 1 | 0 (builds on tracked row `spine:1050`, distinct finding) |
| HIGH | 5 | 1 sharpened repeat (H-4) |
| MEDIUM | 6 | 2 repeats (M-4, M-6) |
| LOW | 3 | 1 repeat (L-2) |
| **Total** | **15** | 4 |

### Rubric line-by-line

| Rubric criterion | Verdict |
| --- | --- |
| Fixes the real divergence points for the level below, misses none | **Partial** — AD-27 fixes the focus divergence at the right altitude but its `Binds` misses two bound stories that render controls (H-1); the transient-selection divergence is recorded and ungated (C-1) |
| Every `AD`'s Rule is enforceable and prevents its stated `Prevents` | **Partial** — AD-27's rules 1-4 are enforceable; rule 5 terminates in a checklist step that does not exist (H-4). AD-12's file rule is contradicted twice inside this same spine (H-2) |
| Nothing under Deferred could let two units diverge | **Fail** — `spine:1050` states in its own words that two bound stories "can each obey every existing `AD` and still build opposite models", and nothing gates either (C-1) |
| Named tech is verified-current | **Pass** — every Stack row matches the lockfiles; Tailwind 4.3.3's `outline-none` / `outline-hidden` behavior verified in the installed dist; the Playwright-WebKit-on-Linux claim verified in config and workflow. One imprecision (L-1) |
| Ratifies rather than contradicts the brownfield | **Partial** — AD-27 now matches `docs/SPEC.md:208` and `src/styles/theme.css`; but it mis-names the shipping control it samples (M-1), and AD-12 mis-describes the ownership of two files the spine itself tells you to edit (H-2) |
| Covers the driving specs' capabilities | **Pass** — no new capability gap found beyond what the tracked v6 tail already carries |
| Every dimension is decided, deferred, or open | **Fail** — the post-publish operational envelope (distribution host, artifact immutability, retraction) is silent in the spine (H-5), and two reviewer-gate tails are recorded nowhere (H-3) |

---

## What verified clean

Recorded so a later revision does not re-litigate these.

**Stack table, every row I could check.** `node -e` over `package-lock.json`
`packages` and `grep -A1` over `src-tauri/Cargo.lock`:

```
react 19.2.8            typescript 7.0.2        vite 8.1.5
tailwindcss 4.3.3       zustand 5.0.14          @playwright/test 1.61.1
vitest 4.1.10           @tauri-apps/api 2.11.1  @tanstack/react-virtual 3.14.8
tauri 2.11.5            tokio 1.53.1
```

All eleven match `spine:970-983` exactly.

**The five ports are five, and named correctly.** `spine:229-231` — "Five ports
exist today and are extended rather than bypassed: `CommandRunner`, `EventSink`,
`UpdateSource`, `PendingRelease`, and `ManagerAdapter`."
`grep -rn "pub trait CommandRunner\|pub trait EventSink\|pub trait UpdateSource\|pub trait PendingRelease\|pub trait ManagerAdapter" src-tauri/src/` returns exactly five
lines: `src-tauri/src/process/runner.rs`, `src-tauri/src/events.rs`,
`src-tauri/src/app_update.rs` (twice), `src-tauri/src/managers/mod.rs`.

**AD-27's Tailwind claim is verified-current, not remembered.**
`spine:909-911` — "under Tailwind 4 `outline-none` genuinely sets
`outline-style: none` (the v3 no-op was renamed `outline-hidden`)".
`grep -o 'outline-none.\{0,90\}' node_modules/tailwindcss/dist/lib.js` →
`outline-none",[["--tw-outline-style","none"],["outline-style","none"]])`, and
`grep -o '.\{80\}outline-hidden.\{160\}'` → `i.static("outline-hidden",()=>[o("--tw-outline-style","none"),o("outline-style","none"),B("@media","(forced-colors: active)",[o("outline","2px solid transparent")`.
Installed version confirmed 4.3.3. Both utilities exist and behave as the rule says.

**AD-27's WebKit honesty is verified.** `spine:936-937` — "CI's `webkit` project is
Playwright's **Linux** WebKit on `ubuntu-latest`, not WKWebView."
`playwright.config.ts:85-86` — `name: "webkit"`, `use: { ...devices["Desktop Safari"] }`;
`.github/workflows/test.yml:56` and `:117` — `runs-on: ubuntu-latest`. Correct as
written, and it is the honest framing AD-3 uses for the fixtures.

**AD-27's mechanism claim matches the tree exactly.** `spine:907-908` — "Focus is
drawn as a real 2px `outline` in `--color-focus-ring` with `outline-offset`".
`grep -rho "focus-visible:outline[^ \"'\`]*" src | sort | uniq -c`:

```
  31 focus-visible:outline-focus-ring
  31 focus-visible:outline-2
  30 focus-visible:outline-offset-1
   1 focus-visible:outline-offset-2
```

`grep -rn "outline-none\|outline-hidden" src` → no output. `src/styles/theme.css:19`
— `  --color-focus-ring:    #F4F7FB;`.

**The style contract samples the two controls AD-27 names.** `spine:930-932`.
`tests/e2e/browser-style-contract.spec.ts:74` — `const refreshAll = page.getByRole("button", {`;
`:186-189` — `page.getByTestId(\`row-${gemini.id}\`).getByRole("checkbox").first()`;
`:97` — `expect(focusTreatment.outlineWidth).toBe("2px")`; `:102` —
`expect(focusTreatment.outlineColor).not.toBe("rgb(101, 167, 255)")` — the negative
guard against the accent the design-token row claims. All present. (The *description*
of the second sample is wrong — M-1.)

**D35's adopted values are in the theme.** `spine:1025` claims
`--color-bg-base: #090C13` and `--color-accent: #65A7FF`.
`src/styles/theme.css:8` — `  --color-bg-base:       #090C13;   /* window background */`;
`:27` — `  --color-accent:        #65A7FF;   /* primary actions, active navigation, running */`.

**`docs/SPEC.md` is ratified, not contradicted.** `docs/SPEC.md:208` — "Focus: a real
2px `outline` in `--color-focus-ring` with `outline-offset`, on every interactive
element — a dedicated indicator, never `--color-accent`. Use `outline-*`, **not**
`ring-*`: Tailwind's `ring-*` compiles to `box-shadow`, and WebKit does not paint
`box-shadow` on native-appearance form controls". AD-27 is a faithful lift of this.

---

## CRITICAL

---

### C-1 — The two `OPEN — new architecture` rows record a divergence and gate nothing; the register the builders read says nothing blocks starting

**Where:** `spine:1050`, `spine:1051`, and `_bmad-output/planning-artifacts/epics.md:307`

**Relationship to tracked work:** the *divergence* is tracked — `spine:1050` is the
promotion of `reviews/review-divergence-v9.md` C-1, and `spine:1051` of its C-6. This
finding is not that divergence. It is that neither row establishes a **blocking
relationship**, and the artifact a builder consults before picking up a story
affirmatively says the opposite. That is a distinct, unreported defect.

The row states the hazard in its own words:

```
spine:1050  | Transient selection has no owning invariant | **OPEN — new architecture,
            not this run's scope** | … Story 3.5 (keyboard selection) and UX-PB.1a
            (staging) can each obey every existing `AD` and still build opposite
            models. Closing this means writing a new invariant … so it goes to the
            owner as a decision. |
```

The rubric criterion is "Nothing under Deferred could let two units diverge." This
row is the criterion's own failure case, written out. Compare how the spine gates a
comparable item when it means to:

```
spine:1025  | Canonical design-token set | **RESOLVED** | … UX-PB.1e and UX-PB.5d are
            unblocked. |
```

That row named the blocked stories while it was open, and `epics.md` carried the
block. Row 1050 names affected stories and no block. What the level below says:

```
epics.md:307  | Product Behavior Prerequisite UX-PB.1..UX-PB.5 | `APPROVED TARGET —
              NOT IMPLEMENTED` | … | Epic UX-PB is the primary build queue and runs
              first, and nothing blocks starting it — the canonical design-token set
              that blocked UX-PB.1e and UX-PB.5d was decided and shipped
              (`docs/DECISIONS.md` D35), so both are startable. … |
```

`grep -n "Transient selection\|transient selection\|writer identity\|record cardinality" _bmad-output/planning-artifacts/epics.md`
returns **one** line, `epics.md:827` — and it is about the journal row, not this one.
Neither row reaches `epics.md`'s Implementation-Entry Register at all
(`epics.md:298-322`; the register's open entries are the native harness, the
child-helper language, and the plan-attempt filename).

**This is not hypothetical for UX-PB.1a, which is the first story in the queue.** Its
own criteria already pick one side:

```
epics.md:514  **Dependencies:** D27-D30; AD-16; AD-17; finalized UX spines; AD-27 (…)
epics.md:515  **Blocks:** UX-PB.1b, UX-PB.1c; Story 3.5
epics.md:521  **When** I toggle its plan Checkbox by pointer, Enter/Space, or the grid Space key
epics.md:522  **Then** the Package's canonical identity is added to the one persistent
              draft Upgrade Plan, nothing executes, and Rust rebuilds the exact command
              from canonical intent.
```

There is one checkbox on a package row today, and it is a *selection* checkbox:

```
src/components/manager/PackageRow.tsx:6   * Selection interaction lives on the checkbox and reads modifier keys off the
src/components/manager/PackageRow.tsx:96          aria-label={`Select ${pkg.name}`}
```

So UX-PB.1a — unblocked, first, and citing no dependency on the open question —
converts the selection checkbox into plan membership, which is the `EXPERIENCE.md`
branch `spine:1050` says is one of two live readings. Story 3.5 ("Preserve **Exact**
Keyboard Selection and Row Plan Actions") then lands afterward and is contractually
obliged to preserve the other. The row predicted this precisely; nothing stops it.

**Fix (report-only recommendation).** Two edits, neither of them new architecture:
add the blocking relationship to both rows (`Blocks UX-PB.1a and Story 3.5` /
`Blocks UX-PB.3d and UX-PB.4a`), and route them into `epics.md`'s
Implementation-Entry Register on the next `bmad-correct-course` run so
`epics.md:307`'s "nothing blocks starting it" becomes true or is qualified. Deciding
the invariant is separate and correctly out of this run's scope; recording that it
blocks is not.

---

## HIGH

---

### H-1 — AD-27's `Binds` is a closed story list that omits the two bound stories whose criteria are about rendering controls, and `epics.md` reproduced the omission

**Where:** `spine:901-903`, `spine:907`, `spine:1016`, `spine:1017`

This is `review-rubric-v9.md` C-1's failure mode — the right rule filed where the
diverging story never reads it — recurring one revision later for two different
stories. It is a new finding: v9 C-1 was about UX-PB.1e and UX-PB.5d, both of which
AD-27 now reaches.

The rule's scope and its `Binds` disagree:

```
spine:901-903  - **Binds:** every story that renders an interactive control — all of Epic UX-PB,
                 Stories 3.1, 3.2, 3.5 — plus any change to `src/styles/theme.css` or the style
                 contract
spine:907      - **Rule:** Focus is drawn as a real 2px `outline` in `--color-focus-ring` with
                 `outline-offset`, on every interactive element.
```

"every story that renders an interactive control" is then narrowed by an em-dash into
an enumeration of four. The spine binds six non-UX-PB stories
(`spine:14` — "Stories 2.2, 3.1, 3.2, 3.4, 3.5, 6.5"). Two of the missing two render
controls by their own acceptance criteria:

```
epics.md:1237  ### Story 3.4: Validate Every Settings Control and Environment Report
epics.md:1247  - Governing invariants: AD-4, AD-5, AD-19
epics.md:1253  **Then** valid values persist before becoming active, invalid values are rejected …
epics.md:1254  **And** … every control saves immediately and atomically with visible
               `Saving`/`Saved`/failure state.
```

```
epics.md:1292  ### Story 6.5: Export Exact Native Diagnostics and Visible Outcomes
epics.md:1301  - Governing invariants: AD-3, AD-4, AD-5, AD-16, AD-18, AD-26
epics.md:1318  **Given** Export diagnostics and Open Logs actions
```

The spine's own AD-19 confirms Story 3.4 is a control-rendering story:

```
spine:671  … Every control saves immediately and atomically with visible `Saving` /
           `Saved` / failure state.
```

The Capability map repeats the gap rather than backstopping it:

```
spine:1016  | Settings and Environment Report (Story 3.4) | Settings persistence + detection state | AD-19, AD-21 |
spine:1017  | Diagnostics export (Story 6.5) | `diagnostics.rs` through the production native command | AD-5, AD-18, AD-26 |
```

Every other UX-facing row in that table gained **AD-27** in this revision
(`spine:1009-1013`, `:1018`). These two did not. And `epics.md` propagated the
omission faithfully — `grep -n "AD-27" epics.md` returns 32 lines, covering all 28
UX-PB stories plus Stories 3.1 (`:1200`), 3.2 (`:1224`), 3.5 (`:1272`), and none
inside Story 3.4's block (`epics.md:1237-1261`) or Story 6.5's (`:1292-1319`).

**Failure scenario.** A Story 3.4 builder adds the `skipUpgradePlanConfirmation`
toggle in Settings. Nothing they read mentions AD-27 — not the story's
`Governing invariants`, not the Capability map row, not AD-27's `Binds`. They copy
the nearest neighbour. If that neighbour predates the conversion, or if they reach for
`ring-` out of habit, they ship a checkbox with no visible focus in WKWebView — the
exact defect AD-27 exists to prevent, on a native form control, which is the one place
it is invisible.

**Fix.** Delete the enumeration, or extend it to Stories 3.4 and 6.5 and add AD-27 to
both Capability map rows. The rule text is already universal; only the routing is
narrow.

---

### H-2 — AD-12's "seven files … never hand-edited" is whole-file where release-please ownership is key-scoped, and this spine instructs edits to two of those files

**Where:** `spine:348-351`, contradicted at `spine:303-304` and `spine:883-884`

```
spine:348-351  - **Rule:** Seven files are release-please-owned and never hand-edited:
                 `package.json`, `package-lock.json`, `src-tauri/tauri.conf.json`,
                 `src-tauri/Cargo.toml`, `src-tauri/Cargo.lock`, `CHANGELOG.md`, and
                 `.release-please-manifest.json`.
```

`review-currency.md:220` verified the *count* as ✅ and stopped there. The granularity
was never checked. `release-please-config.json` scopes ownership to one key per file:

```
release-please-config.json  "extra-files": [
                              { "type": "json", "path": "src-tauri/tauri.conf.json",
                                "jsonpath": "$.version" },
                              { "type": "toml", "path": "src-tauri/Cargo.toml",
                                "jsonpath": "$.package.version" },
                              { "type": "toml", "path": "src-tauri/Cargo.lock",
                                "jsonpath": "$.package[?(@.name.value==\"pack-manager\")].version" }
                            ]
```

Read literally, the rule forbids the only mechanism the spine itself relies on twice:

```
spine:303-304  - **Rule:** Minimum supported macOS is 15.0 at
                 `bundle.macOS.minimumSystemVersion` (`docs/DECISIONS.md` D31).
```

```
spine:883-884  `src-tauri/Cargo.toml` declares no `[profile.release]`, so `debug-assertions`
               is off in release builds today and the gate holds. Enabling `debug-assertions`
               in a release profile … is a security-sensitive change under AD-20
```

`bundle.macOS.minimumSystemVersion` lives in `src-tauri/tauri.conf.json`;
`[profile.release]` would live in `src-tauri/Cargo.toml`
(`grep -n "profile" src-tauri/Cargo.toml` → no output). AD-11 tells a builder where to
declare the floor and AD-26 contemplates a reviewed change to the release profile,
while AD-12 says both files are "never hand-edited". A builder obeying AD-12 literally
cannot execute AD-11 or AD-26.

**The most consequential instance is not in the spine at all.** The same file carries
the updater identity:

```
src-tauri/tauri.conf.json:30  "pubkey": "dW50cnVzdGVkIGNvbW1lbnQ6IG1pbmlzaWduIHB1YmxpYyBrZXk6IEY0OUQ5NzE3QkZCM0Q2QzEK…"
src-tauri/tauri.conf.json:31-33  "endpoints": [
                                   "https://github.com/Sallvainian/Pack-Manager/releases/latest/download/latest.json"
                                 ]
```

Rotating the updater keypair means hand-editing `pubkey` in a file AD-12 declares
untouchable — and `release.yml` already documents that getting this wrong is a
silent, fleet-wide failure:

```
.github/workflows/release.yml:298-305  # Prove the detached signature validates against the pubkey the
                                       # shipping app actually embeds. Nothing else checks this: a rotated
                                       # or mismatched TAURI_SIGNING_PRIVATE_KEY still builds, signs with
                                       # Apple, notarizes, staples and publishes green, and every installed
                                       # client then fails verification and stops updating. app_update.rs
                                       # logs that failure at WARN so offline laptops do not nag every six
                                       # hours, which means nobody is ever told.
```

AD-11 correctly makes the *check* an invariant (`spine:293-296`). The rotation
*procedure* it protects has no home, and AD-12 forbids its only mechanism.

**Fix.** One clause: release-please owns the **version field** in
`src-tauri/tauri.conf.json`, `src-tauri/Cargo.toml`, and `src-tauri/Cargo.lock`, and
the whole of `package.json`'s version, `package-lock.json`, `CHANGELOG.md`, and
`.release-please-manifest.json`; every other key in those files is ordinary product
configuration, and the security-sensitive ones (`updater.pubkey`,
`updater.endpoints`, `bundle.macOS.minimumSystemVersion`, `[profile.release]`) change
only under AD-11/AD-20/AD-26 review.

---

### H-3 — The reviewer-gate tails for revisions 8 and 9 are recorded nowhere; only the revision-6 tail has a row, and it is still a bare count

**Where:** `spine:1046`; absence verified by grep

```
spine:1046  | Reviewer-gate tail (revision 6) | **Open** | The four `*-v6` lenses returned
            44 findings: 5 CRITICAL, 14 HIGH, 18 MEDIUM, 7 LOW. … The remaining tail is
            **6 HIGH, 15 MEDIUM, 5 LOW** across `reviews/review-divergence-v6.md`,
            `review-rubric-v6.md`, `review-reconcile-epics-v6.md`, and
            `review-currency-v6.md`. Each finding names its own affected stories. |
```

That is the only tail row. `grep -n "review-rubric-v8\|review-rubric-v9\|review-divergence-v8\|review-divergence-v9\|review-currency-v8\|review-currency-v9\|review-reconcile-v8\|review-reconcile-v9" ARCHITECTURE-SPINE.md`
returns four lines — `:1026`, `:1034`, `:1050`, `:1051` — and every one of them cites a
v8/v9 review as the *source of a single promoted item*. None is a tail.

What the two gates returned, from the review files' own tallies:

- `review-rubric-v8.md:680` heading `## Summary table`; its finding headings
  (`grep -n "^#\{4\} " review-rubric-v8.md`) are `C1`, `H1`-`H3`, `M1`-`M5`, `L1`-`L3`
  — 12 findings.
- `review-rubric-v9.md:51-59` — "### Tally | Tier | Count | … | CRITICAL | 2 | HIGH | 5 |
  MEDIUM | 8 | LOW | 5 | **Total** | **20** |", and at `:61-62` — "revision 8's rubric
  lens returned 1 CRITICAL / 3 HIGH / 5 MEDIUM / 3 LOW (memlog 131)".

That is 32 findings from the rubric lens alone across two gates, before counting
`review-divergence-v8/v9`, `review-currency-v8/v9`, and `review-reconcile-v8/v9`. Some
are closed — I confirmed six of them at the top of this file. The rest are neither
closed nor recorded. Under the rubric's "decided, deferred, or an open question" test,
they are none of the three: they exist only inside review files that the spine's own
status table does not point at.

**This is worse than `review-rubric-v9.md` M-8**, which said the v6 row "remains a bare
count". A bare count is at least a pointer. Revisions 8 and 9 have neither.

**Fix.** Either enumerate the surviving findings, or add two rows in the shape of
`spine:1046` naming the files and the residual counts. The v6 row's own defect —
`Each finding names its own affected stories` in place of naming them — should not be
copied forward.

---

### H-4 — AD-27's only shipping-engine enforcement point is a release-checklist step that does not check focus visibility

**Where:** `spine:940-942`

**REPEAT, sharpened.** `review-rubric-v9.md` H-1 (`:552-571`) established that
"`RELEASE-CHECKLIST.md:86` covers reachability only". That finding is **not** in the
spine's status table (see H-3), and revision 9's fix made it worse rather than better:
AD-27 now *names* the checklist as the rule's terminus, converting an unowned
obligation into a false claim about a specific artifact.

```
spine:940-942  Verification in the real WKWebView remains the manual
               VoiceOver-and-keyboard pass on `docs/RELEASE-CHECKLIST.md` until a
               native harness exists under AD-26.
```

`grep -n "focus\|indicator\|outline" docs/RELEASE-CHECKLIST.md` → **no output.** The
file does not contain the word. What the pass actually asks:

```
docs/RELEASE-CHECKLIST.md:91  9. **Keyboard and accessibility pass.** Tab and arrow navigation reach every control.
docs/RELEASE-CHECKLIST.md:95     One VoiceOver pass over the Upgrade Plan announces state changes and completion.
```

Reachability and announcement. The nine indicator-less controls AD-27 was written
after were all reachable and all announced — that is precisely why the defect survived:

```
spine:926-928  It was found twice here by walking the real tab order and reading computed
               style, and the first count was wrong by a factor of three — three
               reported, **nine** actual.
```

So AD-27's chain of evidence is: the automated lane is a proxy and disclaimed by
AD-27's own rule 5; the story-level obligation is real (`spine:927-928` — "A story
adding an interactive control verifies that control's focus state at runtime") but owes
no artifact; and the one step named as covering the shipping engine does not look at
the indicator. Rule 5's `Prevents` — "a control shipping a focus state that matches
`:focus-visible` but is invisible to the user in the app they actually run"
(`spine:904-906`) — is not reached by any of the three.

**Fix.** One clause on `docs/RELEASE-CHECKLIST.md` item 9: *tab the full order in the
built app and confirm every stop draws a visible outline* — and, per AD-3's precedent
(`spine:213-215`), name the artifact the adding story owes.

---

### H-5 — The post-publish operational envelope is silent in the spine: distribution host, artifact immutability, and retraction

**Where:** absence, verified by grep; the invariant-shaped statements live only in generated docs

**Relationship to tracked work:** adjacent to `review-rubric-v6.md` M5 (runtime egress)
and M6 (operations/observability), both still in the tracked v6 tail at `spine:1046`.
Neither of those covers what follows — M5 is about outbound *runtime* traffic, M6 about
logging and transcripts. This is the distribution and post-publish limb, and no prior
lens raises it (`grep -rn "rollback\|yank\|retract\|unpublish" reviews/` returns only
`review-currency.md:73` about crates.io yanked flags and
`review-reconcile-epics.md:351` about UX-PB.2e's no-rollback promise).

```
$ grep -n "rollback\|retract\|rotat\|unpublish" ARCHITECTURE-SPINE.md
(no output)
$ grep -n "endpoint\|github.com\|GitHub Releases\|latest.json" ARCHITECTURE-SPINE.md
154:  `latest.json`.
295:  the shipping app embeds, and the published `latest.json` is asserted reachable
297:- **Rule:** The build stays universal and `latest.json` publishes both
```

The spine names `latest.json` three times and never names where it lives. The single
point of failure for every installed client's update path is
`src-tauri/tauri.conf.json:32` —
`"https://github.com/Sallvainian/Pack-Manager/releases/latest/download/latest.json"`
— and the spine's Stack table lists "Release automation | release-please action v5 +
GitHub Actions" (`spine:987`) without naming GitHub Releases as the *distribution*
provider. AD-11's `Prevents` reasons explicitly about fleet-wide blast radius
(`spine:265-267` — "the checks whose failure is silent and simultaneous across every
installed client") and then leaves the host unowned.

Two invariant-shaped rules exist in the repo, in generated output rather than in the
spine:

```
docs/deployment-guide.md:189  The repository does not define an automated rollback workflow. If a published build must be withdrawn:
docs/deployment-guide.md:195  Do not reuse or overwrite an existing version tag/artifact; ship a new version so the updater and installed application can reason about ordering correctly.
```

`docs/deployment-guide.md:195` is a genuine invariant — reusing a published version
breaks the updater's ordering for every client that already has it — and it lives in a
`bmad-document-project` artifact, which per this project's standing rule is
regenerated, not authoritative. The release checklist hands the problem off and stops:

```
docs/RELEASE-CHECKLIST.md:61-63  Items 7–9 are **post-publish smoke tests, not gates**. By the time they can run, the
                                 release is live and installed clients have already begun polling. They catch a bad
                                 release; they cannot prevent one.
```

It says the steps catch a bad release. Nothing anywhere says what to do when one is
caught, and the spine — which owns the invariants — is silent on both immutability and
retraction. This is the rubric's "whole dimension left SILENT", in the envelope the
brief singles out.

**Mitigating.** release-please's monotonic bump makes accidental version reuse
unlikely in practice, which is why this is HIGH and not CRITICAL.

**Fix.** One rule under AD-11 or AD-12: name GitHub Releases as the distribution
provider and the `releases/latest/download/latest.json` endpoint; state that a
published version is immutable and never overwritten; state that withdrawal is
forward-only (ship a higher version), never a re-tag.

---

## MEDIUM

---

### M-1 — AD-27 names its own sample as "the package-row plan-membership checkbox"; the shipping control is a selection checkbox, and the conflation pre-answers the question `spine:1050` says is open

**Where:** `spine:930-932`

```
spine:930-932  - **Rule:** The style contract proves the mechanism on **named samples, not a
                 sweep**: today a toolbar `<button>` and the package-row plan-membership
                 checkbox, chosen because they sit on opposite sides of the `appearance`
                 discriminator.
```

"today" is doing real work in that sentence, and it is false about the control's
identity. The shipping control:

```
src/components/manager/PackageRow.tsx:6    * Selection interaction lives on the checkbox and reads modifier keys off the
src/components/manager/PackageRow.tsx:96           aria-label={`Select ${pkg.name}`}
```

And the test that samples it asserts selection semantics, not membership:

```
tests/e2e/browser-style-contract.spec.ts:191  await test.step("When the row is selected from the keyboard, leaving that checkbox focused", async () => {
tests/e2e/browser-style-contract.spec.ts:200  await test.step("Then selection reads as the accent wash and focus as the dedicated ring", async () => {
```

A plan-membership checkbox cannot exist yet — the spine's own baseline says so:

```
spine:143-145  … a single-package row action executes immediately, the durable token is a
               monotonic `revision` in `PlanCoordinator`, and no `planAttemptId`,
               `Verifying`, or `InteractionRequired` symbol exists in `src/` or
               `src-tauri/src/`.
```

Two consequences. The small one: a builder looking for the named sample will not find
it under that name. The larger one: this is the same conflation `spine:1050` records as
**undecided** — whether the row checkbox is selection or plan membership. AD-27 answers
it in passing, in the affirmative, in a rule about CSS. If C-1 is decided the other way
(`docs/SPEC.md` F5's reading, where Esc clears selection and writes no membership),
AD-27's sample description becomes wrong in a second way.

**Fix.** Call it what it is — "the package-row selection checkbox" — and let C-1's
decision rename it if it renames it. The `appearance`-discriminator reasoning, which is
the point of the rule, is unaffected.

---

### M-2 — The `epics.md` residuals row is stale: commit `5972109` cleared it, and the row still reads OPEN with five items

**Where:** `spine:1049`

```
spine:1049  | `epics.md` residuals for the next `bmad-correct-course` run | **OPEN — record
            only; do not edit `epics.md` here** | Five items, in three classes. …
            (3) `epics.md`'s Implementation-Entry register still lists the canonical
            design-token set as `OPEN` and **"Blocks UX-PB.1e and UX-PB.5d"**, which D35
            closed … (4) the same register still records the `notarytool minos 15.0`
            question as OPEN, which D34 closed. |
```

`5972109` "Clear the epics.md residual pile (#36)" landed after revision 9. Both
register rows now read the opposite of what the residual describes:

```
epics.md:308  | Canonical design-token set | `CLOSED` — D35 | Resolved 2026-07-25 | Nothing blocked. …
epics.md:309  | DR-1 — minimum supported macOS | `CLOSED` — D31 | … The `notarytool` `minos 15.0` question is CLOSED by `docs/DECISIONS.md` D34 …
```

Residual (1) is also addressed — `epics.md:827` now carries
"AD-25 (a failed verification refresh leaves the Last-good Snapshot in place)" on
UX-PB.3d's Dependencies line. Residual (2) is partly addressed — AD-21's substance now
reaches criterion prose at `epics.md:1066`: "**And** `skipUpgradePlanConfirmation` is
plan-inert (AD-21) — it is not a plan-determining input". (Residual (5) is out of scope
for this review per the assignment.)

The row's own instruction — "do not edit `epics.md` here" — is now aimed at a pile that
no longer exists, and a `bmad-correct-course` run acting on it would be re-doing
`5972109`.

**Fix.** Retire or rewrite the row against `5972109`. See also M-5 for the one part of
residual (2) that genuinely survives.

---

### M-3 — `epics.md:827` states a rule *about* AD-18 that AD-18 does not contain, so the journal-cardinality fence lives at story altitude

**Where:** `spine:1051` (the OPEN row), `spine:1048` (the invariant it breaks), `epics.md:827`

`spine:1051` records the gap as open architecture. The level below has already fenced it
unilaterally, and says so:

```
epics.md:827  **Dependencies:** UX-PB.3c; D29-D30; AD-16 (verification-gated success;
              post-exit fresh acquisition); AD-18 (the plan-attempt journal's home, format
              and durability — note AD-18 does not itself name a writer or a record
              cardinality, so the terminal-write ownership below is stated here and belongs
              in AD-18 when it is next amended); UX-PB.4a owns the single durable terminal
              write and this story never writes one; …
```

That is the right *content* — it is exactly the fold rule `spine:1051` says is missing —
in the wrong *place*. It collides with the invariant the spine claims for `epics.md`:

```
spine:1048  … No `AD-n` id in `epics.md` asserts a rule differing from this spine's under
            that id, and every live `AD` id is cited there.
```

`epics.md:827` asserts, under the id AD-18, a rule AD-18 does not carry. It is
additive rather than contradictory, so it is not a live divergence today. But the fence
now lives in one story's Dependencies prose: a routine edit to UX-PB.3d deletes the
only statement that UX-PB.4a owns the single terminal write, and `spine:1051`'s
double-replay hazard reopens with nothing recording that it did. And a UX-PB.4a builder
reading AD-18 directly sees no cardinality rule at all — `spine:620-649` contains none.

**Fix.** Lift the sentence into AD-18 (the row already says it "belongs in AD-18 when it
is next amended") or, at minimum, record in `spine:1051` that the interim fence exists
at `epics.md:827` and that deleting it reopens the row.

---

### M-4 — The design-token RESOLVED row still bakes in a site count HEAD falsified

**Where:** `spine:1025`

**REPEAT** of `review-rubric-v9.md` M-3 (`:846`), unfixed, and not in the status table
(H-3).

```
spine:1025  … All 22 `focus-visible` sites resolve `--color-focus-ring`, which is what
            `EXPERIENCE.md`'s "a separate `{colors.focusRing}` indicator … selected and
            focused states remain distinguishable" requires …
```

`grep -rn "focus-visible" src | wc -l` → **31**. The spine knows the real number two
rows down:

```
spine:1052  … Every focus site now draws with `outline` plus `outline-offset`: 31 sites …
            corroborated by the site count going from 22 to 31.
```

So the same table asserts 22 and 31 for the same population, eight rows apart. The
substantive claim (all of them resolve the focus-ring token, none the accent) is true;
only the count is stale.

**Fix.** Drop the number — "every `focus-visible` site resolves `--color-focus-ring`" is
the invariant, and a count in a decision table is a maintenance liability
(see `review-rubric-v9.md` L-3, also unfixed).

---

### M-5 — AD-21 binds Story 3.4, and Story 3.4 does not cite AD-21

**Where:** `spine:692`, `epics.md:1247`

```
spine:692  - **Binds:** UX-PB.2b, UX-PB.5b, UX-PB.5c; Story 3.4; all settings work
spine:717-718  - **Rule:** The shipping call site bumps unconditionally for every key
               (`src-tauri/src/commands.rs` `set_settings_core`). Narrowing it is product
               work owned by whichever of UX-PB.5b or Story 3.4 lands first, not a test
               concern (AD-1).
```

AD-21 names Story 3.4 twice, once as a `Binds` target and once as a possible *owner of
the narrowing work*. `grep -n "AD-21" _bmad-output/planning-artifacts/epics.md` returns
exactly two lines, `:1051` and `:1066`, both inside UX-PB.5b. Story 3.4's contract:

```
epics.md:1247  - Governing invariants: AD-4, AD-5, AD-19
```

The Capability map does carry it (`spine:1016` — "AD-19, AD-21"), so the spine is
internally consistent; the routing to the story is what is missing. This is the
surviving half of `spine:1049`'s residual (2): `5972109` restated AD-21's substance for
UX-PB.5b (`epics.md:1066`) and left Story 3.4 untouched. If Story 3.4 lands first, its
builder narrows — or fails to narrow — `set_settings_core` with no knowledge of the
plan-determining classification AD-21 requires at the definition site
(`spine:705-710`).

**Fix.** Add AD-21 to Story 3.4's `Governing invariants` on the next
`bmad-correct-course` run.

---

### M-6 — The browser engine set is now stated in AD-27's prose but no rule makes it non-narrowing, and the Stack table still lists Playwright without engines

**Where:** `spine:936-937`, `spine:983`, `spine:985`

**REPEAT, partial.** `review-rubric-v9.md` H-2 had two limbs. Limb (i) — the spine
asserting Linux WebKit as evidence about WKWebView — is **closed** by
`spine:936-940`, correctly and well. Limb (ii) is not.

```
spine:983  | Playwright | 1.61.1 |
spine:985  | CI runner images | macos-15 (`ci.yml` rust, `ci.yml` build-smoke, `release.yml` build); ubuntu-latest (all other jobs) |
```

Neither row names the browser projects. AD-27 states the fact in prose but no rule
protects it: a CI-tuning pass that scopes E2E to `--project=chromium` to halve runtime
violates nothing in this spine, and AD-27's rules 4 and 5 both silently lose the only
automated engine that catches the defect class. Compare AD-2's treatment of a
comparable "must not narrow" property (`spine:184-186` — "Release builds contain no
feature flag, environment variable, CLI option, hidden command, or runtime selector
that can activate a controlled adapter"), which is written as a prohibition rather than
as a description.

**Fix.** One Stack row (`Playwright browser projects | chromium, webkit`) plus a clause
in AD-27: the engine set is load-bearing and narrowing it is a change to this AD, not a
CI-tuning detail.

---

## LOW

---

### L-1 — "the v3 no-op" mis-describes Tailwind v3's `outline-none`

**Where:** `spine:909-911`

```
spine:909-911  under Tailwind 4 `outline-none` genuinely sets `outline-style: none` (the v3
               no-op was renamed `outline-hidden`), so it actively suppresses the indicator.
```

The substantive half is verified (see "What verified clean"). The parenthetical is
imprecise: v3's `outline-none` was not a no-op — it set a transparent 2px outline so
forced-colors mode kept an indicator, which is why v4's `outline-hidden` still emits it:

```
node_modules/tailwindcss/dist/lib.js  i.static("outline-hidden",()=>[o("--tw-outline-style","none"),
                                      o("outline-style","none"),B("@media","(forced-colors: active)",
                                      [o("outline","2px solid transparent"),o("outline-offset","2…
```

Calling it a no-op invites someone to treat `outline-hidden` as safe-to-delete when it
is the forced-colors-preserving variant. One word: "the v3 spelling", not "the v3
no-op".

---

### L-2 — "exactly one `ring-accent` survivor" is true of production and false of the repo

**Where:** `spine:1025`, `spine:1052`

**REPEAT** of `review-rubric-v9.md` L-2 (`:1016`), unfixed.
`grep -rn "ring-" src | grep -v "focus-ring"` returns the production survivor plus two
assertions on it:

```
src/components/manager/PackageRow.tsx:85         highlighted ? "ring-2 ring-inset ring-accent" : "",
src/components/manager/managerPane.test.tsx:114  expect(highlighted.className).toContain("ring-accent");
src/components/manager/managerPane.test.tsx:118  expect(other.className).not.toContain("ring-accent");
```

The claim is correct if scoped to product code and wrong as a repo-wide count. Say
"one in product code".

---

### L-3 — AD-27 fixes the outline width and leaves the offset free, while claiming one mechanism everywhere

**Where:** `spine:907-912`

```
spine:907-908  - **Rule:** Focus is drawn as a real 2px `outline` in `--color-focus-ring` with
                 `outline-offset`, on every interactive element.
spine:911-912  One mechanism everywhere (`docs/DECISIONS.md` D35, `docs/SPEC.md` §4.1).
```

The width is pinned; the offset is required but unvalued, and the tree already has two
values:

```
$ grep -rho "outline-offset[^ \"'\`]*" src | sort | uniq -c
  30 outline-offset-1
   1 outline-offset-2
```

```
src/components/primitives/Button.tsx:40  "transition-colors focus-visible:outline-2 focus-visible:outline-focus-ring focus-visible:outline-offset-2",
```

Both are compliant, so this is not a violation — but the shared primitive
(`Button.tsx`) is the outlier, so the next control built on it inherits `offset-2`
while its hand-rolled neighbours use `offset-1`. Either pin the offset or say
explicitly that the offset is a per-context choice and only the width, color, and
property are the mechanism.

---

## Method note

Every quotation above was read from the file this session; no claim rests on memory or
on another review's account of a file. Counts and absences come from the commands shown
inline — `wc -l`, `grep -c`, `grep -n`, `grep -rho | sort | uniq -c`, and
`node -e` over `package-lock.json`. Where a command returned nothing, that is stated as
"no output" rather than paraphrased.

Per the assignment I did not re-run `lint_spine.py`, did not re-derive the given repo
state (`macos-15`, D35, the 31-site focus conversion, the `install_app_update` guard,
`5972109`'s AD-27 citations, app version 1.0.1), and did not treat the archived PRD's
absence or `_bmad-output/project-context.md`'s mid-update state as findings.

`docs/RELEASE-CHECKLIST.md` was read in its working-tree state (`git status` shows it
modified). H-4's grep result — no occurrence of "focus", "indicator", or "outline" —
is therefore a statement about the working tree, which is the version a release would
be run from.
