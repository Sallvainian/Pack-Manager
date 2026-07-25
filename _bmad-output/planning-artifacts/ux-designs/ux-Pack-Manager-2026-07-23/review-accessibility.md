# Accessibility Review — Pack-Manager UX Spines

Reviewed: `DESIGN.md` (252 lines), `EXPERIENCE.md` (460 lines), both `status: final`,
`updated: 2026-07-24`. Re-verified against `docs/DECISIONS.md` D35, `src/styles/theme.css`,
`tests/e2e/browser-style-contract.spec.ts`, and the shipped components. This report replaces
the 2026-07-23 review in the same folder.

Note on one citation in the review brief: in WCAG 2.2, **2.4.11 is Focus Not Obscured
(Minimum)** (AA); the criterion named **Focus Appearance is 2.4.13** (AAA). This report cites
**1.4.11 Non-text Contrast** (AA) for focus-indicator contrast, which is the AA-level
obligation that actually bites here.

## Overall verdict

All six findings from the prior review are now closed in the spine text — the focus-ring
token, the Package Grid keyboard/ARIA model, the focus-transition matrix, the high-zoom
layout, the single announcement channel, and the explanatory-disabled rule are all written
down and specific. That is a real advance, and D35 implemented the focus half of it correctly.
What remains is a different class of problem: the spines were written before the ring→outline
migration and are **silent on the mechanism constraint that migration exists to fix**, silent
on focus-indicator contrast against adjacent colours, and carry one numeric self-contradiction
on checkbox target size that shipped code already lands on the wrong side of. Three high
findings; none critical.

## Status of prior review's findings

| Prior finding                                                              | Now                                                                                                                                                                                                                                                                                                                                            |
| -------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **1.** `borderStrong` too weak to serve as the focus indicator             | **Resolved.** A dedicated token exists (`DESIGN.md:23` `focusRing: "#F4F7FB"`; `DESIGN.md:119`), `DESIGN.md:252` now forbids the substitution — "Do not use `borderStrong` alone as keyboard focus" — and `EXPERIENCE.md:318` makes it normative. Recomputed: focusRing reaches **18.20:1** on `background` and **13.41:1** on `overlay`; the old `borderStrong` figures the prior review gave (2.67:1 / 2.33:1) reproduce exactly. Shipped at `src/styles/theme.css:19` and CI-pinned at `tests/e2e/browser-style-contract.spec.ts:101`. |
| **2.** Dense virtualized Package-table keyboard/VoiceOver model incomplete | **Resolved.** `EXPERIENCE.md:149` now specifies the ARIA grid pattern, `aria-rowcount`/`aria-rowindex`, and one roving Tab stop; `EXPERIENCE.md:283–288` gives the full key map; `EXPERIENCE.md:328` and `:330` add the 101-row reachability and acceptance obligations. The virtualization trap is answered directly at `EXPERIENCE.md:287`: "Virtualization never silently discards the focused identity."                                                                                     |
| **3.** Focus destinations undefined across dialog → Activity → Results     | **Resolved.** `EXPERIENCE.md:290–302` adds a nine-row Focus transitions matrix (`sed -n '294,302p' \| grep -c '^\|'` = 9) covering open, Change Plan, Escape/backdrop, final confirmation, row updates, Activity→Results, retry scope, `Done`, and replay. One transition it still omits is a *new* finding (F2 below), not this one.                                                                                                                                             |
| **4.** 900×600 and 200% zoom contracts cannot both hold                    | **Resolved.** `DESIGN.md:173` and `EXPERIENCE.md:329`/`:366` now define an explicit high-zoom mode below 720 usable CSS px.                                                                                                                                                                                                                     |
| **5.** Live announcements under-specified                                  | **Resolved.** `EXPERIENCE.md:322` names the exact announced events, polite-by-default priority, and the exclusion list; `:323` handles Brief Notification duplicate suppression.                                                                                                                                                                |
| **6.** "Explanatory-disabled" needs an explicit semantic rule              | **Resolved in the spine** (`EXPERIENCE.md:143`, `:326`) — **but not enforced, and shipped code violates it.** See F7.                                                                                                                                                                                                                            |

## Findings

### High

- **[high]** **Neither spine records the WebKit native-appearance constraint, and `DESIGN.md`
  still calls the indicator a "ring" five times.** The mechanism that D35 spent 31 sites
  fixing is nowhere in the normative documents.
  (`_bmad-output/.../EXPERIENCE.md:318`, `DESIGN.md:119`). Quote, `EXPERIENCE.md:318`:
  "Every interactive element uses a separate `{colors.focusRing}` indicator that is at least
  2px wide and visible against every surface." Quote, `DESIGN.md:119`: "`focusRing` | `#F4F7FB`
  | Dedicated high-contrast keyboard-focus **ring**". Counts: `grep -ci` for `box-shadow` = 0
  in both files; for `WebKit` = 0 in both files. `grep -oi "focus[- ]ring\|focusRing" | wc -l`
  = 5 in `DESIGN.md`, 1 in `EXPERIENCE.md`. Only one line in either spine names a mechanism,
  and only for one component — `DESIGN.md:202`: "Keyboard focus uses a separated 2px
  `focusRing` outline". `DESIGN.md:203` (Checkbox) drops the word: "retain the same 2px
  `focusRing`". The constraint itself is recorded only in `docs/DECISIONS.md:427–430`:
  "Tailwind's `ring-*` utilities compile to `box-shadow`, and **WebKit does not paint
  `box-shadow` on a native-appearance form control**". A spine that says "ring" and never says
  "not box-shadow" is an open invitation to regress the exact defect D35 closed — and the
  regression is invisible to `grep`, because it reappears as a *different* class name.
  *Fix:* add one normative line to the `EXPERIENCE.md` Accessibility Floor and to `DESIGN.md`'s
  colour section: focus is drawn as CSS `outline` + `outline-offset`, never `box-shadow` /
  Tailwind `ring-*`, because WKWebView discards `box-shadow` on native-appearance form
  controls; `outline-none` must never appear on a focusable element (Tailwind v4 makes it a
  real `outline-style: none`). Replace the word "ring" in `DESIGN.md:119` with "outline".

- **[high]** **Focus is undefined when the draft Upgrade Sidecar closes.** The sidecar's
  *appearance* is specified and its *transformations* are specified, but its disappearance —
  the one case where the focused control is itself destroyed — is not
  (`EXPERIENCE.md:181`, `EXPERIENCE.md:290–302`). Quote, `EXPERIENCE.md:181`: "Every staged
  Package row has `Remove`; every staged Manager self-update has `Remove` in the Manager group
  heading. Removing the final item closes the sidecar." Restated at `EXPERIENCE.md:418`:
  "Removing the last item closes the sidecar." Neither mentions focus. The contrasting case
  *is* covered — `EXPERIENCE.md:179`: "First eligible addition: sidecar opens with focus
  preserved on the source control" — which makes the omission asymmetric rather than
  deliberate. The nine matrix rows at `EXPERIENCE.md:294–302` contain no sidecar-close row.
  Pressing the final `Remove` from the keyboard therefore destroys the focused element with no
  specified destination; the browser default sends focus to `<body>`, and the user's next Tab
  restarts from the top of the document. Removing a non-final item has the same gap.
  *Fix:* add two matrix rows — "Remove a staged item" → move focus to the next surviving
  Remove control, else the plan heading; "Last item removed / sidecar closes" → restore focus
  to the Package row Checkbox or Manager action that staged it, else the workspace heading.

- **[high]** **The checkbox target size contradicts the Accessibility Floor and falls below
  WCAG 2.2 SC 2.5.8.** (`DESIGN.md:203` vs `EXPERIENCE.md:327`). Quote, `DESIGN.md:203`:
  "16–18px aligned control." Quote, `EXPERIENCE.md:327`: "Hit targets are at least 28 × 28px
  for compact desktop controls and 36px where space permits." 16–18px cannot satisfy 28×28,
  and it is under SC 2.5.8 Target Size (Minimum)'s 24×24 CSS px floor. The shipped code lands
  on the smaller number: `src/components/manager/PackageRow.tsx:98` and
  `src/components/primitives/Checkbox.tsx:36` both begin `"h-4 w-4 shrink-0 ..."` — 16×16 — and
  the wrapper at `PackageRow.tsx:88` is `"flex w-9 shrink-0 justify-center pt-0.5"`, a layout
  cell, not a label, so it does not enlarge the target. This is the control that governs
  Upgrade Plan membership — the product's core interaction. SC 2.5.8 does admit a Spacing
  exception (an undersized target passes if a 24px-diameter circle centred on it meets no
  other target's circle), but neither spine invokes it or states the spacing that would make it
  hold.
  *Fix:* reconcile the two numbers in one direction. Either raise the checkbox to a ≥24×24
  activation area (a padded `<label>` wrapper keeps the 16–18px visual control while enlarging
  the target), or state the Spacing exception explicitly in `DESIGN.md:203` with the required
  row pitch, and amend `EXPERIENCE.md:327` so the 28×28 floor no longer contradicts it.

### Medium

- **[medium]** **No focus-indicator contrast rule against adjacent colours; the spine's only
  contrast floor is about text.** (`EXPERIENCE.md:317`, `EXPERIENCE.md:318`). Quote,
  `EXPERIENCE.md:317`: "All ordinary text and controls target at least 4.5:1 contrast; large
  display text targets at least 3:1." Quote, `EXPERIENCE.md:318`: "…visible against every
  surface." "Visible" is not a ratio, and "every surface" is not "every adjacent colour."
  Computed (WCAG relative-luminance formula, sRGB): focusRing `#F4F7FB` against the dark ramp
  is excellent — background 18.20:1, shell 17.13:1, surface 15.87:1, raised 14.49:1, overlay
  13.41:1, accentSubtle 13.42:1. Against *filled controls* it is not: **accent `#65A7FF`
  2.29:1**, **danger `#FF8793` 2.14:1**, success `#72E6A0` 1.44:1, warning `#F1C875` 1.47:1,
  info `#62E7D8` 1.40:1 — all below SC 1.4.11 Non-text Contrast's 3:1. A checked Checkbox is
  accent-filled (`DESIGN.md:203`: "Checked state uses accent and a visible mark"), so an
  outline drawn at zero offset on a checked checkbox, or on a blue primary button, would sit
  at 2.29:1. The implementation escapes this only through offset — `grep -c "outline-offset"`
  over `src/` = 31, i.e. every one of the 31 focus sites — but the spine requires offset for
  exactly one component (`DESIGN.md:202` "separated"), and omits it for the Checkbox
  (`DESIGN.md:203`).
  *Fix:* make offset normative in `EXPERIENCE.md:318` and state the obligation in adjacency
  terms: the focus indicator must reach ≥3:1 against **both** adjacent colours (the control
  fill on the inside and the surface on the outside), which for `#F4F7FB` on filled accent,
  danger, success, warning, or info controls is achievable only with a non-zero
  `outline-offset` that exposes the surface on both edges.

- **[medium]** **Version-delta severity is a pure colour signal that neither spine defines at
  all.** `grep -ci` for `severity`, `major`, `minor`, and `patch` returns **0 in both
  `DESIGN.md` and `EXPERIENCE.md`** — yet three severity tokens ship
  (`src/styles/theme.css:41–43`: `--color-sev-major: #FF8793; --color-sev-minor: #F1C875;
  --color-sev-patch: #72E6A0;`) and colour a version string by semver severity
  (`src/components/manager/VersionDelta.tsx:19–21`, applied at `:50`). `DESIGN.md:134` claims
  "Status colors always travel with a word, icon, count, meter length, or version label" — but
  a version label such as `1.2.0 → 1.3.0` states the *versions*, never the *severity*, so hue
  is the only carrier of "this is a major bump." The shipped component does redeem it with a
  text chip (`VersionDelta.tsx:57` renders `{delta.severity}` inside a `Chip`), which means the
  code is currently correct **and entirely unprotected** — nothing in either spine obliges that
  chip, so removing it would violate no written contract.
  *Fix:* add version-delta severity to `DESIGN.md`'s colour section as a named signal, and add
  one Accessibility Floor line requiring the severity word to accompany the severity colour.

- **[medium]** **Reduced motion has no rule for the indeterminate activity indicator, and the
  automated test does not reach one.** (`EXPERIENCE.md:308`, `EXPERIENCE.md:310`). Quote,
  `EXPERIENCE.md:308`: "A running indeterminate indicator communicates activity, not
  percentage." Quote, `EXPERIENCE.md:310`: "Under `prefers-reduced-motion`, remove nonessential
  animation and use immediate state changes, text, and icons." An indeterminate spinner is by
  definition *essential* animation — motion is the entire signal — and the spine gives no
  substitute. The implementation resolves the ambiguity in the harmful direction:
  `src/styles/theme.css:61–66` applies `animation: none !important` to `*, *::before, *::after`,
  and `src/components/primitives/Spinner.tsx:13` is `"inline-block animate-spin rounded-full
  border-2 border-border-strong border-t-accent"`. Under reduced motion that spinner freezes
  into a static ring indistinguishable from decoration. VoiceOver users are fine
  (`Spinner.tsx:11–12` carries `role="status"` and `aria-label`); sighted reduced-motion users
  lose the signal. The automated coverage does not catch it: `browser-style-contract.spec.ts:105–126`
  asserts `transitionProperty: "none"` and `animationName: "none"` on a single idle
  `Refresh All` **button**, so it confirms suppression works and asserts nothing about whether
  a running Operation still reads as running.
  *Fix:* `EXPERIENCE.md` should state what replaces continuous motion under reduced motion —
  a visible `Running`/`Waiting` text state and a stepped or static-but-distinct indicator —
  and `EXPERIENCE.md:330`'s acceptance list should name a running Operation row under
  `prefers-reduced-motion`, not a static control.

- **[medium]** **The explanatory-disabled contract is written but absent from the acceptance
  list, and shipped code already violates it.** Quote, `EXPERIENCE.md:326`: "Explanatory-disabled
  Package Checkboxes do not use native `disabled`. They expose `aria-disabled=\"true\"`, retain
  focus, associate the persistent reason through an accessible description, remain inert on
  activation…" Restated at `EXPERIENCE.md:143`. The code does the opposite:
  `src/components/manager/PackageRow.tsx:92` is `disabled={checkboxDisabled}`, and
  `grep -rn "aria-disabled" src/ | wc -l` = **0**. The reason is carried by the native `title`
  attribute (`PackageRow.tsx:69–75`, applied at `:94`), which is hover-only and therefore also
  breaks `EXPERIENCE.md:158`: "Opens on hover, click, or keyboard focus" (and SC 1.4.13 Content
  on Hover or Focus). A native `disabled` checkbox is unfocusable, so the pinned-Package
  explanation is unreachable by keyboard today. The spine is not at fault for the code — it is
  at fault for `EXPERIENCE.md:330`, which lists what packaged acceptance verifies ("focus,
  final-row reachability, selection scope, completion announcements, and no overlap at 100%,
  150%, and 200% zoom") and omits this entirely.
  *Fix:* add explanatory-disabled reachability to `EXPERIENCE.md:330`'s acceptance list — a
  pinned Package Checkbox must be Tab-reachable, announce its reason, and remain inert — so the
  existing implementation gap is caught rather than re-litigated.

- **[medium]** **Async Settings save results are never announced.** (`EXPERIENCE.md:257`,
  `EXPERIENCE.md:322`). Quote, `EXPERIENCE.md:257`: "Every Settings control saves immediately
  and atomically: show `Saving`, activate the new value only after persistence succeeds, then
  show `Saved`. On failure, retain or restore the prior value and show an inline error." Every
  verb there is visual. The one announcement channel the Floor defines is explicitly scoped
  elsewhere — `EXPERIENCE.md:322`: "One atomic **Activity/Results** status channel announces
  plan start, a changed waiting reason, an action-required failure, each Manager's completion
  summary, and the final plan outcome." A VoiceOver user who toggles `Skip confirmation for
  future upgrade plans` gets no confirmation that it persisted, and no notice when it silently
  reverts — a safety-preference failure that is invisible to the user it matters most to
  (SC 4.1.3 Status Messages).
  *Fix:* extend the Floor to require that each Settings control's `Saving` → `Saved` → error
  outcome is exposed as a status message scoped to that control, and state that it does not
  move focus.

### Low

- **[low]** **No general accessible-name rule for icon-only controls.** The spines name exactly
  two controls' accessible text — `EXPERIENCE.md:147` ("accessible name `Remove <Manager>
  update from Upgrade Plan`") and `EXPERIENCE.md:182` (bulk Checkbox label) — plus the Health
  Meter (`DESIGN.md:141`, `EXPERIENCE.md:152`). Meanwhile `DESIGN.md:204` builds primary
  navigation from icons ("Cohesive outlined icons, 38px rows…") and `DESIGN.md:194` governs
  icon style generally. `EXPERIENCE.md:319` requires surfaces to "expose meaningful names,
  roles, states, and relationships to VoiceOver", but names no rule for a control whose only
  visible content is a glyph — e.g. disclosure carets, the row expander at
  `src/components/manager/PackageRow.tsx:114–121`.
  *Fix:* one Floor line: every control whose visible content is an icon alone carries a text
  accessible name stating its action and object, and the icon is `aria-hidden`.

- **[low]** **Progress semantics are described visually and never mapped to assistive
  technology.** `DESIGN.md:215` lists "running indeterminate, running determinate" as required
  states and `EXPERIENCE.md:212` says "Running progress is indeterminate unless the adapter
  provides a trustworthy measurable total", but `grep -ci progressbar` = 0 in both files, and
  neither spine says how a determinate percentage reaches VoiceOver — particularly awkward
  against `EXPERIENCE.md:322`, which forbids the status channel from announcing "progress
  ticks". With no role committed, determinate progress is currently announced by nothing.
  *Fix:* state that determinate progress uses a progress role carrying current/max, that
  indeterminate progress omits the value rather than faking one, and that neither is routed
  through the announcement channel.

- **[low]** **F6 is the only skip-navigation equivalent, and no landmark structure is
  required.** Quote, `EXPERIENCE.md:277`: "F6 cycles the primary navigation, main Package
  Grid/workspace, and Upgrade Sidecar regions without changing selection." Region cycling is
  the right desktop answer to skip-to-content, and `grep -ci "skip to"` = 0 is not itself a
  defect. Two gaps: `grep -ci landmark` = 0 in both files, so nothing requires those three
  regions to be exposed as landmarks — which is what makes them reachable via the VoiceOver
  rotor rather than only via the app's own key handler; and bare `F6` is a Windows/web
  convention that the spine never reconciles with macOS Full Keyboard Access, on a product
  whose stated premise is "Native macOS conventions… take priority" (`DESIGN.md:103`). The
  exact macOS system binding for panel cycling is `[unverified]` here.
  *Fix:* require the three F6 regions to be landmark regions with accessible names, and confirm
  the F6 binding against macOS Full Keyboard Access before the spine freezes it.

- **[low]** **The 80–99% health band has no token and no contrast number.** Quote,
  `DESIGN.md:138`: "80–99% current: interpolate through yellow-green while maintaining
  contrast." This conflicts with the spine's own rule at `DESIGN.md:107`: "All product colors
  must be exposed through semantic Tailwind tokens in `src/styles/theme.css`. Product
  components consume token names; they do not hardcode hex values." No interpolation token
  exists in `src/styles/theme.css` (66 lines, read in full). The three fixed bands are
  comfortably safe as graphical objects against the `inset` track — danger 8.58:1, warning
  12.44:1, success 12.71:1 — but "maintaining contrast" names no threshold, so the interpolated
  band is unverifiable by inspection or by test.
  *Fix:* either define the intermediate stops as tokens, or state the numeric floor the
  interpolation must hold (≥3:1 against the meter track, per SC 1.4.11).

- **[low]** **The navigated-to row highlight is a colour-only state that appears in neither
  spine.** `src/components/manager/PackageRow.tsx:85` renders
  `highlighted ? "ring-2 ring-inset ring-accent" : ""`, deliberately preserved by
  `docs/DECISIONS.md:459–464` ("a cross-manager navigation highlight… not a focus state").
  `grep -ci navigated` = 0 and `grep -ci highlight` = 0 in `EXPERIENCE.md` (the single
  `DESIGN.md` hit is `:182`, "a faint inner highlight", about card elevation). So a row state
  that exists in the product is carried by accent colour alone, with no text, icon, or
  announcement, and no spine contract at all — the one live counterexample to `DESIGN.md:134`'s
  claim that colour always travels with another signal.
  *Fix:* define the navigated-to row state in both spines with a non-colour companion (a
  programmatic focus move to that row, or a "jumped from <Manager>" affordance), or drop it.

## Verified-adequate (no action)

- **Focus token, value, and separation from selection.** `DESIGN.md:23`/`:119`,
  `EXPERIENCE.md:318`, `DESIGN.md:252`. Implemented at `src/styles/theme.css:19` and guarded
  both positively and negatively in CI — `browser-style-contract.spec.ts:101–102` asserts
  `outlineColor` is `rgb(244, 247, 251)` and **not** `rgb(101, 167, 255)`, and `:214–222`
  proves a selected row (`accent-subtle`) and a focused checkbox inside it stay distinct.
- **The keyboard contract is complete on every axis except sidecar close (F2).** Tab order
  `EXPERIENCE.md:274`; dialog focus containment `:154` ("traps focus"); restoration `:296`;
  Escape `:276`; roving focus and full arrow/Home/End/Page/Shift-range map `:149`, `:283–288`;
  region cycling `:277`; sidecar appearance preserves source focus `:179`; in-place
  transformation `:297`, `:299`.
- **The virtualized-row focus trap is answered directly.** `EXPERIENCE.md:287`: "Filter changes
  retain focus on the same Package when it survives; otherwise focus moves to the first
  available row or the grid heading. Virtualization never silently discards the focused
  identity." Reinforced at `:149` and `:328`.
- **Announcement policy.** `EXPERIENCE.md:322–324` — named events, polite default, assertive
  only for immediate safety, explicit exclusion of queued rows/progress ticks/output lines,
  duplicate suppression for Brief Notifications, and no focus movement.
- **Count-bearing bulk checkbox scope.** `EXPERIENCE.md:286`: "Its label names the exact
  consequence (`Add all 8 updates`), and its mixed state is announced" — with the off-screen
  scope pinned at `:143` and `:182`.
- **Health Meter accessible label.** `DESIGN.md:141` and `EXPERIENCE.md:152`; length + counts +
  colour, with `:152` refusing invented health on unknown/failed refresh.
- **Text contrast across the whole ramp.** All computed AA-passing: textMuted 6.77:1 on
  background, 5.90:1 on surface, 5.39:1 on raised, 4.99:1 on overlay (the floor case, and it
  clears 4.5:1); textSecondary 7.19–9.76:1; textPrimary 13.41–18.20:1; onAccent on accent
  7.74:1; onSuccess on success 12.15:1; status colours ≥7.42:1 on surface. Note that
  `EXPERIENCE.md:317`'s "at least 4.5:1" is verified by eye at release, not automated —
  `browser-style-contract.spec.ts:128–129` says so plainly: "This is a browser DOM/CSS contract
  only. It does not claim measured contrast compliance."
- **28px / 36px minimum control heights (excluding the checkbox, F3).** `DESIGN.md:202`,
  `EXPERIENCE.md:327`. **This is defensible.** 28px clears WCAG 2.2 SC 2.5.8's 24×24 CSS px
  floor with margin while staying consistent with dense native macOS control metrics; 36px for
  standard controls is generous for a desktop app. No change needed to these two numbers.
- **Zoom and reflow, carried in both spines.** `DESIGN.md:173` sets the ≥150% / <720px trigger
  and the required survivors; `EXPERIENCE.md:329` and `:366–367` carry the matching behavioural
  contract, and `:330` puts 100/150/200% into packaged acceptance.
- **Colour independence at Status Chip, Health Meter, and Activity rows.** `DESIGN.md:211` +
  `EXPERIENCE.md:151` ("never relies on hue alone"); `DESIGN.md:212` + `EXPERIENCE.md:152`;
  `EXPERIENCE.md:155` (eleven named text states); `DESIGN.md:205` ("Status dot accompanies
  text"); `DESIGN.md:246` ("or use color as the only signal" under Don'ts). The two exceptions
  are F5 (version-delta severity) and the last low finding (navigated-to row).
- **The app-update version delta.** `DESIGN.md:220` / `EXPERIENCE.md:259` assign warning yellow
  to installed and success green to target. Those two hues are near-identical in luminance
  (1.02:1 against each other), so hue alone would not separate them for every user — but the
  `installed → target` ordering and arrow carry the meaning independently, so the colour is
  redundant rather than load-bearing. Adequate as written.
- **Reduced-motion suppression itself.** `src/styles/theme.css:61–66` covers `*, *::before,
  *::after`; asserted at `browser-style-contract.spec.ts:120–125`. The gap is scope (F6), not
  the mechanism.
- **Commands and logs remain real text.** `EXPERIENCE.md:332`: "Commands and logs are
  selectable, copyable, and not rendered solely as inaccessible canvas content."
