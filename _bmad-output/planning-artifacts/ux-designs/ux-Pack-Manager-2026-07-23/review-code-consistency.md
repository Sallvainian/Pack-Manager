# Code Consistency Review — Pack-Manager UX Spines

Reviewed: `DESIGN.md`, `EXPERIENCE.md` (this folder) against `src/`, `src/styles/theme.css`,
`docs/SPEC.md`, `docs/DECISIONS.md` D35, `tests/e2e/browser-style-contract.spec.ts`.
Date of review: 2026-07-25.

## Overall verdict

The **colour layer is now in exact lockstep** — all 22 `DESIGN.md` colours are adopted into
`theme.css` with byte-identical values (verified programmatically, zero mismatches), and the
focus mechanism the spines demand is really shipped across 31 sites with a CI guard. The
remaining divergence is in the **non-colour layers**: radius, typography, and control sizing,
where `DESIGN.md` declares scales the code has no token for and, in two cases, defines the
same token at a different value. One divergence is a live accessibility defect: three shipped
components paint `text-white` on bright accent/danger fills at **2.46:1 and 2.30:1**, while
the `onAccent`/`onSuccess` tokens that exist precisely to prevent this have **zero consumers**.
That is a numerically false position against `EXPERIENCE.md`'s own 4.5:1 floor.

Second-order but structural: neither spine records the WebKit `box-shadow` constraint that
forced the `ring-*` → `outline` migration. Only `SPEC.md` and `DECISIONS.md` hold it. A UX
story generated from the spines alone can regress it.

---

## Findings

### CRITICAL

- **[critical] Shipped primary/danger buttons fail the spine's own 4.5:1 contrast floor.**
  `EXPERIENCE.md:317` "All ordinary text and controls target at least 4.5:1 contrast; large
  display text targets at least 3:1." `DESIGN.md:126` defines `| `onAccent` | `#07101D` |
  Text/icons on bright blue primary-action fills |` and `DESIGN.md:128` `| `onSuccess` |
  `#07140D` | Text/icons on bright green confirmation fills |`. The code paints white instead:
  - `src/components/primitives/Button.tsx:7` — `primary: "bg-accent text-white hover:bg-accent-hover disabled:bg-accent/40",`
  - `src/components/primitives/Button.tsx:11` — `danger: "bg-danger text-white hover:brightness-110 disabled:opacity-50",`
  - `src/components/shell/UpdateStatusItem.tsx:63` — `` className={`${CHIP} bg-accent text-white hover:bg-accent-hover`} ``

  Computed: `#FFFFFF` on `--color-accent #65A7FF` = **2.46:1**; on `--color-accent-hover
  #7DB3FF` (the hover state) = **2.15:1**; on `--color-danger #FF8793` = **2.30:1**. All three
  fail 4.5:1 *and* 3:1. Using the spine's own tokens instead: `#07101D` on `#65A7FF` =
  **7.74:1**; `#07101D` on `#FF8793` = **8.30:1**. `--color-on-accent` and `--color-on-success`
  exist in `src/styles/theme.css:30` and `:32` but `grep -rn "on-accent\|on-success" src/`
  returns matches **only in `theme.css` itself** — no component consumes either.
  *Which side is stale:* **code**. The spine is right; D35 added the tokens but did not
  repoint the fills.
  *Fix:* `text-white` → `text-on-accent` (Button primary, UpdateStatusItem chip) and
  `text-on-accent` or a dedicated `onDanger` (Button danger). Add a `text-white`-on-fill lint
  or extend `browser-style-contract.spec.ts` with a computed-colour assertion on the primary
  button, since nothing currently catches this.

### HIGH

- **[high] `DESIGN.md` declares 6 radius values; `theme.css` defines 2, and both disagree in
  value.** `DESIGN.md:57-63`:
  ```
  rounded:
    control: 7px
    navigation: 9px
    card: 13px
    dialog: 16px
    window: 18px
    pill: 999px
  ```
  (`sed -n '58,63p' … | grep -c ":"` → `6`.) `src/styles/theme.css:46` is the entire shape
  block: `--radius-card: 10px; --radius-control: 6px;` (`grep -o "\-\-radius-[a-z]*" src/styles/theme.css | wc -l`
  → `2`). So `card` is 10px vs the spine's 13px, `control` is 6px vs 7px, and `navigation`,
  `dialog`, `window`, `pill` have no token at all. Consequences visible in the code: dialogs
  borrow the card radius — `src/components/dialogs/UpgradePlanSheet.tsx:205` "…overflow-hidden
  rounded-card border border-border-strong bg-bg-overlay shadow-2xl" (same at
  `QuitGuardDialog.tsx:57`, `StalledOperationDialog.tsx:42`) — and navigation borrows the
  control radius: `src/components/shell/Sidebar.tsx:27` "relative flex w-full items-center
  rounded-control px-2.5 py-1.5 text-left text-[13px]".
  *Which side is stale:* **both** — the two shared names are stale in the code; the four
  unimplemented names are aspirational in the spine.
  *Fix:* one decision. Either adopt the six values as `--radius-*` tokens (mirroring what D35
  did for colour), or amend `DESIGN.md`'s `rounded:` to the two the product actually uses. Do
  not leave a spine that declares six and a theme that defines two.

- **[high] Six checkbox sites hardcode an arbitrary radius the spine does not contain.**
  `grep -rho "rounded-[A-Za-z0-9_\[\]#.-]*" src/ | sort | uniq -c` → `27 rounded-control`,
  `16 rounded-card`, `8 rounded-full`, `6 rounded-[4px]`, `1 rounded-t-card`. The six:
  `src/components/primitives/Checkbox.tsx:36`, `src/components/manager/PackageTable.tsx:109`,
  `src/components/manager/PackageRow.tsx:98`, `src/components/manager/PackageToolbar.tsx:72`,
  `src/components/dialogs/UpgradePlanSheet.tsx:223` and `:234` — e.g.
  `Checkbox.tsx:36` "h-4 w-4 shrink-0 cursor-pointer rounded-[4px] border border-border-strong bg-bg-raised".
  `DESIGN.md:107` is explicit that this pattern is forbidden for colour — "Product components
  consume token names; they do not hardcode hex values." — but says nothing equivalent for
  shape, so `4px` is not a violation of any written rule, it is a rule the spine never wrote.
  *Which side is stale:* **spine** (missing a control-adjacent small radius) *and* **code**
  (six duplicated magic numbers).
  *Fix:* add the value to `rounded:` (e.g. `checkbox: 4px`) and a `--radius-checkbox` token, or
  fold the checkboxes onto `rounded-control`.

- **[high] `DESIGN.md`'s sidebar width contradicts both the code and `SPEC.md`.**
  `DESIGN.md:166` "2. A 190px left sidebar at ordinary zoom." The code:
  `src/components/shell/Sidebar.tsx:63` `<nav className="flex h-full w-60 shrink-0 flex-col border-r border-border bg-bg-surface">`
  — `w-60` is 15rem = 240px. `docs/SPEC.md:227` "│ Sidebar 240px │ MainView (Dashboard | ManagerPane |".
  *Which side is stale:* **spine**. Two independent sources say 240px.
  *Fix:* correct `DESIGN.md:166` to 240px, or open a decision if 190px was a deliberate
  redesign that was never implemented.

### MEDIUM

- **[medium] Neither spine records the WebKit constraint that dictates the focus mechanism —
  the single most regressible fact in this change.** `grep -n -i "focus\|ring\|box-shadow\|outline"`
  over `EXPERIENCE.md` returns **no** hit for `outline` or `box-shadow`; the strongest statement
  is `EXPERIENCE.md:318` "Every interactive element uses a separate `{colors.focusRing}`
  indicator that is at least 2px wide and visible against every surface." `DESIGN.md:202` says
  "Keyboard focus uses a separated 2px `focusRing` outline; selection color never substitutes
  for focus." — "outline" appears there only as an English noun, not as the CSS property, and
  `DESIGN.md:203` repeats "retain the same 2px `focusRing`". The actual rule lives outside the
  spines: `docs/SPEC.md:208` "Use `outline-*`, **not** `ring-*`: Tailwind's `ring-*` compiles to
  `box-shadow`, and WebKit does not paint `box-shadow` on native-appearance form controls
  (`<input type="checkbox">`, `<select>`). … Never add `outline-none` to a focusable element."
  and `docs/DECISIONS.md:425` "**Focus is drawn as a real `outline`, never `ring-*`.** This is
  the part most likely to be "modernised" back by someone who does not know why".
  *Which side is stale:* **spine**. A story generated from `DESIGN.md`/`EXPERIENCE.md` alone
  satisfies "2px focusRing indicator" perfectly well with `ring-2 ring-focus-ring` and ships
  invisible focus on every checkbox.
  *Fix:* add one sentence to the `EXPERIENCE.md` Accessibility Floor pinning the mechanism to
  `outline` + `outline-offset` and citing D35.

- **[medium] `DESIGN.md` declares 5 font weights and 8 font sizes; `theme.css` defines zero of
  either, and the code's actual scale differs from the spine's.**
  `DESIGN.md:42-56` declares `fontWeight: regular 400 / medium 550 / semibold 650 / bold 750 /
  heavy 850` and `fontSize: micro 10px / caption 11px / bodySmall 12px / body 13px / label 14px
  / headingSmall 16px / heading 20px / summaryValue 28px`. `grep -c "font-weight" src/styles/theme.css`
  → `0`; the file's whole `/* Fonts */` block is `theme.css:49-50` (`--font-sans`, `--font-mono`).
  So `font-medium` and `font-semibold` resolve to Tailwind's defaults 500 and 600, not 550/650.
  Sizes actually shipped (`grep -rho "text-\[[0-9]*px\]" src/ | sort | uniq -c`): `47 text-[12px]`,
  `38 text-[13px]`, `35 text-[11px]`, `15 text-[15px]`, `4 text-[20px]`, `2 text-[10px]`.
  `grep -ro "text-\[14px\]" src/ | wc -l` → `0`; same for `16px` → `0` and `28px` → `0`. The
  15px step the code uses 15 times is absent from `DESIGN.md` but present in `docs/SPEC.md:215`
  "| Section head | 15px/600 sans | Card titles, dialog titles |" — and SPEC's `600` matches the
  shipped `font-semibold`, where `DESIGN.md`'s `650` does not.
  *Which side is stale:* **spine**, on the evidence — `SPEC.md` and the code agree on 15px/600
  against `DESIGN.md`'s 14/16px + 650.
  *Fix:* reconcile `DESIGN.md`'s `typography:` with `SPEC.md §4.2`, then decide whether the
  scale earns tokens. Tailwind's arbitrary `text-[Npx]` covers sizes acceptably; the **weights
  are the real gap**, because 550/650 are non-default and cannot be reached without tokens.

- **[medium] `DESIGN.md` declares a `display` font family the code has no token for.**
  `DESIGN.md:40` `display: '-apple-system, BlinkMacSystemFont, "SF Pro Display", "Segoe UI", system-ui, sans-serif'`.
  `grep -c "font-display" src/styles/theme.css` → `0`. Only `--font-sans` and `--font-mono`
  exist (`theme.css:49-50`).
  *Which side is stale:* **spine** (the only sizes that would justify SF Pro Display — 20px and
  28px — are, per the count above, 4 sites and 0 sites respectively).
  *Fix:* drop `display` from `DESIGN.md`, or add `--font-display` and apply it to the two
  heading roles.

- **[medium] Standard control height diverges.** `DESIGN.md:202` "Minimum height: 28px for
  compact controls and 36px for standard controls." `src/components/primitives/Button.tsx:15-16`:
  `sm: "h-7 px-2.5 text-[12px]",` / `md: "h-8 px-3 text-[13px]",` — `h-7` = 28px (matches
  "compact"), `h-8` = 32px, and `md` is the component default (`Button.tsx:26` `size = "md",`).
  32px is neither of the spine's two numbers. `EXPERIENCE.md:327` sets the accessibility floor
  lower — "Hit targets are at least 28 × 28px for compact desktop controls and 36px where space
  permits." — so 32px clears the floor but misses the design contract.
  *Which side is stale:* **code**, unless 36px was rejected for density.
  *Fix:* `md` → `h-9` (36px), or amend `DESIGN.md:202` to 32px.

- **[medium] The spine's 38px navigation row height is pinned nowhere in the code.**
  `DESIGN.md:204` "Cohesive outlined icons, 38px rows, active accent wash and left inset."
  The two navigation rows set no height class: `src/components/shell/Sidebar.tsx:27` "relative
  flex w-full items-center rounded-control px-2.5 py-1.5 text-left text-[13px]" and
  `src/components/shell/SidebarManagerItem.tsx:46` "group relative flex w-full items-center
  gap-2.5 rounded-control px-2.5 py-1.5 text-left text-[13px]". Height is whatever `py-1.5`
  (6px + 6px) plus the inherited line box produces. The rendered height is `[unverified]` — it
  cannot be established from source alone — but the *absence of any explicit height* is
  verifiable, and it means nothing holds the spine's 38px.
  *Which side is stale:* **code**.
  *Fix:* add `h-[38px]` (or a token) to both rows so the spine value is enforced rather than
  incidental.

### LOW

- **[low] Three `--color-sev-*` tokens exist in `theme.css` with no `DESIGN.md` counterpart —
  confirmed, and `DESIGN.md` should NOT own them, but should acknowledge the mirror.**
  Programmatic diff: 22 `DESIGN.md` colours, 25 `--color-*` in `theme.css`, extras =
  `sev-major`, `sev-minor`, `sev-patch`. They are owned by `SPEC.md`, which declares them
  verbatim at `docs/SPEC.md:189-193` and gives them semantics at `docs/SPEC.md:263`
  "render the common prefix in `--color-text-secondary` and the changed suffix of `latest` in
  the severity color (segment 0 → `--color-sev-major`, 1 → minor, ≥2 → patch)" — with
  `docs/SPEC.md:13` "The frontend's version-delta highlight and severity chips are display
  affordances computed by pure string segment-diff, never a source of truth." That is a
  behavioural contract, correctly SPEC-owned, and `DESIGN.md` should not absorb it. The
  residual risk is the coupling: `src/styles/theme.css:39-43` comments "Deliberately mirrors
  danger/warning/success." and the values are byte-identical to `danger`/`warning`/`success`,
  but nothing in `DESIGN.md` says a future change to those three must propagate. `D35` names
  the hazard — `docs/DECISIONS.md:411` "that mirror relationship is preserved under the new
  values so one table row cannot render two palettes."
  *Which side is stale:* **neither**; this is a missing cross-reference.
  *Fix:* one line in `DESIGN.md`'s Colors section noting that version-delta severity mirrors
  `danger`/`warning`/`success` and must move with them (see `SPEC §4.1`).

- **[low] `violet` is declared, documented as rare, and used nowhere.** `DESIGN.md:132`
  "| `violet` | `#B59CFF` | Rare secondary accent; never a core status |" and
  `src/styles/theme.css:36` `--color-violet: #B59CFF;   /* rare secondary accent; never a core status */`.
  `grep -rn "violet" src/ | wc -l` → `1`, i.e. the definition itself.
  *Which side is stale:* **neither** — a declared-but-unused reserve token is legitimate. Noted
  so a future audit does not "clean it up".

- **[low] The reduced-motion implementation is broader than the spine's wording, and the test
  does not cover the gap.** `EXPERIENCE.md:310` "Under `prefers-reduced-motion`, remove
  nonessential animation and use immediate state changes, text, and icons." and
  `EXPERIENCE.md:331` "Reduced-motion behavior removes continuous or sweeping animation."
  The code is unconditional — `src/styles/theme.css:61-66`:
  ```
  @media (prefers-reduced-motion: reduce) {
    *, *::before, *::after {
      transition: none !important;
      animation: none !important;
  ```
  This also stops `src/components/primitives/Spinner.tsx:13` "inline-block animate-spin rounded-full
  border-2 border-border-strong border-t-accent" and the six `animate-pulse` sites, which are
  arguably the "essential" indicators of `EXPERIENCE.md:308` "A running indeterminate indicator
  communicates activity, not percentage." This is safe — `EXPERIENCE.md:320` "Status is never
  conveyed only by color, meter length, animation, or icon." guarantees a text channel — but the
  spine says "nonessential" while the code removes all.
  *Which side is stale:* **spine** (imprecise) — the blanket rule is the better engineering.
  *Fix:* change `EXPERIENCE.md:310` to state that all transitions and animations are suppressed
  and status falls back to the text channel.

---

## Section F — reduced motion, what the test actually asserts

`tests/e2e/browser-style-contract.spec.ts:47` `await page.emulateMedia({ reducedMotion: "reduce" });`
then `:120-125`:
```
      expect(motion).toEqual({
        transitionProperty: "none",
        transitionDuration: "0s",
        animationName: "none",
        animationDuration: "0s",
      });
```
measured on one element only — `:106-109`, `const refreshAll = page.getByRole("button", { name: "Refresh All", exact: true });`.
So the assertion is real but narrow: it proves the `@media` block applies to a `<button>`, not
that any specific animated component (Spinner, the pulse dots) is suppressed. The spine's
motion durations at `EXPERIENCE.md:306` "Use 120–180ms for hover, focus, and selection feedback"
are consistent with the code, which sets no explicit duration
(`grep -rho "duration-[0-9]*" src/` → no matches) and uses Tailwind's 150ms default via the
three `transition-*` utilities (`grep -rho "transition-[a-z]*" src/ | sort | uniq -c` →
`2 transition-colors`, `1 transition-opacity`), matching `src/styles/theme.css:60`
"/* prefers-reduced-motion disables all transitions (default 150ms ease). */". The 180–260ms and
400ms bands at `EXPERIENCE.md:306` have no implementation — `[unverified]` whether the sidecar
transitions they describe are built yet.

---

## Contrast computation results

sRGB relative luminance per WCAG 2.x (`((c/255+0.055)/1.055)^2.4` above the 0.04045 knee),
`(L_lighter + 0.05) / (L_darker + 0.05)`. Script:
`/private/tmp/claude-501/-Users-sallvain-Projects-Pack-Manager/172afd72-073c-44a0-83e5-915c730beee0/scratchpad/contrast.py`.

### Load-bearing text pairs — all PASS

| Pair | Ratio | ≥4.5:1 | ≥3:1 |
| --- | ---: | --- | --- |
| textMuted `#8D99AA` on background `#090C13` | 6.77 | PASS | PASS |
| textMuted `#8D99AA` on surface `#151C2A` | 5.90 | PASS | PASS |
| textMuted `#8D99AA` on raised `#1B2434` | 5.39 | PASS | PASS |
| **textMuted `#8D99AA` on overlay `#202A3C`** | **4.99** | **PASS** | PASS |
| textMuted `#8D99AA` on inset `#070B12` | 6.82 | PASS | PASS |
| textMuted `#8D99AA` on accentSubtle `#172A46` | 4.99 | PASS | PASS |
| textSecondary `#AEB8C7` on background / surface / raised / overlay | 9.76 / 8.51 / 7.77 / 7.19 | PASS | PASS |
| textPrimary `#F4F7FB` on background / surface / raised / overlay | 18.20 / 15.87 / 14.49 / 13.41 | PASS | PASS |
| focusRing `#F4F7FB` on background / surface / raised / overlay | 18.20 / 15.87 / 14.49 / 13.41 | PASS | PASS |
| focusRing `#F4F7FB` on accentSubtle `#172A46` (focused control inside a selected row) | 13.42 | PASS | PASS |
| onAccent `#07101D` on accent `#65A7FF` | 7.74 | PASS | PASS |
| onSuccess `#07140D` on success `#72E6A0` | 12.15 | PASS | PASS |
| accent `#65A7FF` on background / surface | 7.94 / 6.92 | PASS | PASS |
| success `#72E6A0` on background / surface | 12.61 / 11.00 | PASS | PASS |
| warning `#F1C875` on background / surface | 12.35 / 10.77 | PASS | PASS |
| danger `#FF8793` on background / surface | 8.51 / 7.42 | PASS | PASS |
| info `#62E7D8` on background / surface | 12.99 / 11.32 | PASS | PASS |
| violet `#B59CFF` on background / surface | 8.54 / 7.44 | PASS | PASS |

**The `DESIGN.md:122` claim is numerically TRUE.** "| `textMuted` | `#8D99AA` | Tertiary labels
with 4.5:1 minimum contrast through the overlay layer |" — measured 4.99:1 on `overlay #202A3C`,
which is the worst of the six surfaces. The margin is 0.49; a one-step darkening of `textMuted`
or a lightening of `overlay` breaks it, so it is a genuine constraint, not slack.

### FAILURES — shipped `text-white` fills

| Pair | Ratio | ≥4.5:1 | ≥3:1 |
| --- | ---: | --- | --- |
| **`#FFFFFF` on accent `#65A7FF`** (`Button.tsx:7`, `UpdateStatusItem.tsx:63`) | **2.46** | **FAIL** | **FAIL** |
| **`#FFFFFF` on accentHover `#7DB3FF`** (`Button.tsx:7` hover) | **2.15** | **FAIL** | **FAIL** |
| **`#FFFFFF` on danger `#FF8793`** (`Button.tsx:11`) | **2.30** | **FAIL** | **FAIL** |

### Non-text boundaries — below 3:1, judged not-a-violation as currently used

| Pair | Ratio | ≥3:1 |
| --- | ---: | --- |
| border `#2A3548` on background / surface / raised / overlay | 1.58 / 1.38 / 1.26 / 1.17 | FAIL |
| borderStrong `#465773` on background / surface / raised / overlay | 2.67 / 2.33 / 2.13 / 1.97 | FAIL |

WCAG 1.4.11 (3:1 for non-text) applies to boundaries **required to identify a component or its
state**. `border` is a structural divider — `DESIGN.md:117` "| `border` | `#2A3548` | Default
structural dividers |" — and is exempt. `borderStrong` is the exposure: `DESIGN.md:118` calls it
"Selected or emphasized boundaries" and `EXPERIENCE.md:318` permits "`{colors.borderStrong}` may
indicate selection but never substitutes for focus". At a maximum of **2.67:1** it cannot carry
selection state on its own. The current code does not ask it to — selection is the accent wash,
`src/components/manager/PackageRow.tsx:84` `selected ? "bg-accent-subtle" : "hover:bg-bg-raised",`
(`grep -rn "accent-subtle" src/` → 3 hits: the token, `Chip.tsx:7`, `PackageRow.tsx:84`) — so
this is latent, not live. **Recommend `EXPERIENCE.md:318` be tightened** from "may indicate
selection" to "may reinforce selection but is never its only indicator", because at 2.67:1 the
permission the spine currently grants is one a future story can take and fail 1.4.11 with.

---

## Verified-consistent (no action)

- **All 22 `DESIGN.md` colours adopted at exact values.** Programmatic diff of `DESIGN.md:15-36`
  `colors:` against every `--color-*` in `src/styles/theme.css`: 22 declared, **0 missing, 0
  value mismatches**. The `# Colors` table at `DESIGN.md:109-132` agrees with the frontmatter.
  D35 delivered what it claimed.
- **`--color-focus-ring` is a real dedicated token, not the accent.** `src/styles/theme.css:19`
  `--color-focus-ring:    #F4F7FB;` under `:17-18` "Keyboard focus — a dedicated indicator,
  deliberately not the accent, so focused and selected states stay distinguishable." satisfying
  `EXPERIENCE.md:318` and `DESIGN.md:119` "| `focusRing` | `#F4F7FB` | Dedicated high-contrast
  keyboard-focus ring |". CI holds the negative guard:
  `tests/e2e/browser-style-contract.spec.ts:102` `expect(focusTreatment.outlineColor).not.toBe("rgb(101, 167, 255)");`
- **Zero `ring-*` focus utilities remain in `src/`.** `grep -rn "ring-" src/ | wc -l` → `6`, and
  every one is accounted for: 2 are the words "ring-buffer" in prose comments
  (`src/lib/ipc/types.ts:254`, `src/lib/ipc/client.ts:72`), 1 is a test name
  (`src/components/activity/liveLog.test.tsx:80`), 2 are test assertions
  (`src/components/manager/managerPane.test.tsx:114`, `:118`), and the single production use is
  `src/components/manager/PackageRow.tsx:85` `highlighted ? "ring-2 ring-inset ring-accent" : "",`
  — a cross-manager navigation highlight with **no `focus-visible:` prefix**, exactly the
  exception `docs/DECISIONS.md:459` reserves: "Exactly one `ring-accent` use deliberately
  survives, at `src/components/manager/PackageRow.tsx:85`." Its sibling assertion
  `managerPane.test.tsx:115` `expect(highlighted.className).not.toContain("outline-focus-ring");`
  pins the separation.
- **Focus outline width is 2px at every site**, matching `DESIGN.md:202` "a separated 2px
  `focusRing` outline" and `EXPERIENCE.md:318` "at least 2px wide".
  `grep -rho "outline-[A-Za-z0-9_\[\]#.-]*" src/ | sort | uniq -c` → `32 outline-focus-ring`,
  `31 outline-2`, `30 outline-offset-1`, `1 outline-offset-2`. `grep -rn "focus-visible:" src/ | wc -l`
  → `31`; the 32nd `outline-focus-ring` is the test assertion above, so **31 production focus
  sites**, matching D35's claim exactly. Enforced at runtime by
  `browser-style-contract.spec.ts:97` `expect(focusTreatment.outlineWidth).toBe("2px");` and
  again on a native checkbox at `:220`.
- **No `outline-none` / `outline-hidden` anywhere.** `grep -r "outline-hidden" src/ | wc -l` → `0`,
  and no file matches `outline-none`. `docs/SPEC.md:208` "Never add `outline-none` to a focusable
  element." holds. (Note: "separated" in `DESIGN.md:202` is never quantified, so
  `outline-offset-1` at 30 sites vs `outline-offset-2` at 1 (`Button.tsx:38`) cannot be called a
  violation — the spine under-specifies rather than the code diverging.)
- **Selection and focus are separate channels in both the code and CI**, per `DESIGN.md:252`
  "Do not use `borderStrong` alone as keyboard focus". `browser-style-contract.spec.ts:214-222`
  asserts the row background is `rgb(23, 42, 70)` (`accentSubtle`) while the focused checkbox
  inside it carries a `rgb(244, 247, 251)` outline.
- **The nine previously-unstyled controls are covered.** Reconstructed from the utility counts:
  every one of the 31 `focus-visible:` sites carries the full triple
  `focus-visible:outline-2 focus-visible:outline-focus-ring focus-visible:outline-offset-{1,2}`,
  including the two `<select>` filters (`src/components/history/HistoryView.tsx:90`, `:103`) and
  both Upgrade Plan sheet checkboxes (`src/components/dialogs/UpgradePlanSheet.tsx:223`, `:234`)
  that `docs/DECISIONS.md:456` names.
- **Body surface tokens are asserted in CI.** `browser-style-contract.spec.ts:66-70` pins
  `backgroundColor: "rgb(9, 12, 19)"` (`#090C13`) and `color: "rgb(244, 247, 251)"` (`#F4F7FB`),
  matching `theme.css:8` and `:22`.
- **`DESIGN.md:107`'s no-hardcoded-hex rule holds.** No component in `src/` contains a hex colour
  literal; all colour comes through `bg-*`/`text-*`/`border-*` token utilities. (The `text-white`
  finding above is a Tailwind default keyword, not a hex literal — it evades this rule, which is
  part of why it survived.)
