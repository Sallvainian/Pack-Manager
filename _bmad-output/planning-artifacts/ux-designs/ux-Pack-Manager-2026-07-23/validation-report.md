# Validation Report — Pack-Manager

- **DESIGN.md:** `_bmad-output/planning-artifacts/ux-designs/ux-Pack-Manager-2026-07-23/DESIGN.md`
- **EXPERIENCE.md:** `_bmad-output/planning-artifacts/ux-designs/ux-Pack-Manager-2026-07-23/EXPERIENCE.md`
- **Run at:** 2026-07-25 (Validate intent, headless)
- **Lenses run:** rubric walker, code-consistency, epics-consistency, accessibility, usability

## Overall verdict

Mechanically these are among the denser spine pairs in the repo: exact 22/22/22 component
parity across `DESIGN.md` frontmatter, `DESIGN.md#Components` and `EXPERIENCE.md#Component
Patterns`; all 22 colour tokens carry hex; all six flows carry a named protagonist, numbered
steps and a climax; all four mockups are linked with no orphans; both files sit in canonical
section order. Against the *code* they are now in good standing — D35 adopted all 22 colours
into `src/styles/theme.css` with zero value mismatches, and the focus migration the spines
asked for landed at 31 sites.

They fail on **inheritance**. Seven of the eleven `sources:` paths across the two files do not
exist, and the three that matter were deliberately retired by `docs/DECISIONS.md` D33. This is
not bookkeeping: `EXPERIENCE.md:27` still delegates the authority for exact commands,
execution, persistence and safety to that retired PRD, so the most consequential UX in the
product currently points at nothing.

Beyond provenance, the pair carries a small number of genuine build-blockers — a sidecar that
two sections require to hold two different things at once, an irreversible-run stop control
with two different labels in one file, a `Verifying` state that is normative in one spine and
absent from the other, and a Retry behaviour that contradicts `epics.md`. **An Update run is
needed.** One item — transient selection — is an owner decision, not a doc edit.

## Category verdicts

| # | Category | Verdict |
|---|----------|---------|
| 1 | Flow coverage | adequate |
| 2 | Token completeness | adequate |
| 3 | Component coverage | strong |
| 4 | State coverage | thin |
| 5 | Visual reference coverage | strong |
| 6 | Bloat & overspecification | adequate |
| 7 | Inheritance discipline | **broken** |
| 8 | Shape fit | strong |

## Findings by severity

### Critical (5)

**[Inheritance] `sources:` frontmatter routes consumers into retired and non-existent documents**
(`DESIGN.md:9-13`, `EXPERIENCE.md:9-14`)
Seven of eleven paths do not exist. `DESIGN.md` cites 5, of which `prds/prd-Pack-Manager-2026-07-22/prd.md`,
its `addendum.md`, and `implementation-readiness-report-2026-07-23.md` are gone. `EXPERIENCE.md`
cites 6 and adds a fourth casualty, `sprint-change-proposal-2026-07-24.md`. All are in
`_bmad-output/archive/2026-07-24-scope-recalibration/`. `docs/DECISIONS.md:363-365` names this exact
hazard: "Also rejected: leaving the gate documents in place as aspirational, since BMAD skill runs
glob them back into the plan, re-entrenching what this record retires."
*Fix:* drop both PRD entries (do **not** repoint to the archive); decide drop-vs-repoint for the
readiness report (a live `implementation-readiness-report-2026-07-25.md` exists) and the sprint-change
proposal (successors `-2026-07-25.md`, `-2026-07-25-spine-rev8.md`); **add `docs/SPEC.md` and
`docs/DECISIONS.md`**, which neither spine cites today though `ARCHITECTURE-SPINE.md:15-27` does.

**[Inheritance] `EXPERIENCE.md` normatively delegates load-bearing UX to the retired PRD**
(`EXPERIENCE.md:27`)
> "Requirements for exact commands, process ownership, execution, persistence, and safety remain
> authoritative in the source PRD, addendum, Architecture Spine, and epics."

Also `:375` "The source acceptance journeys remain authoritative." and `:444` "the redaction and
retention requirements in the source contracts." AJ-1…AJ-6 do survive verbatim at `epics.md:148-153`,
so the journey names still resolve; the command/execution/safety delegation does not.
*Fix:* rewrite `:27` to name the surviving contracts (`docs/SPEC.md`, `docs/DECISIONS.md`,
`ARCHITECTURE-SPINE.md`, `epics.md`) rather than re-citing.

**[Epics] `Create new plan` contradicts AD-24 as written** (`EXPERIENCE.md:231`)
> "`Create new plan` deliberately replaces the sidecar with a new reviewable draft."

`epics.md:990` requires the derived `RetryIntent` reach preview and confirmation "without ever
writing to, merging with, or emptying the one persistent draft". A dev following the binding spine
breaks the invariant.
*Fix:* restate `:231` as a separate reviewable object that does not touch the persistent draft.

**[Usability] The single Upgrade Sidecar has two mutually exclusive required occupants**
(`EXPERIENCE.md:62`, `:100-101`, `:179`)
`:62` commits the sidecar to Results "until `Done`"; `:100-101` permits "A user may continue
reviewing a draft, but it cannot be confirmed until the active attempt is terminal"; `:179` opens
the sidecar unconditionally on "First eligible addition". Adding an item while Results is pending
has no defined outcome. `epics.md:782` (UX-PB.3a) restates the collision rather than resolving it,
so UX-PB.1b and UX-PB.3a cannot both be built as written.
*Fix:* decide precedence — stacked regions, a Results dismissal requirement, or a draft-defers rule.

**[Usability] The irreversible-run stop control has two labels in one document**
(`EXPERIENCE.md:155` vs `:216`, `:427`)
`:155` "Stalled rows expose exactly `Keep waiting`, `Copy command`, and `Cancel`" against `:216`
and `:427` "exactly `Keep waiting`, `Copy command`, and `Cancel plan`". The file's own glossary at
`:110` reserves the generic form: "Generic `Cancel` is reserved for closing a dialog or retry-scope
editor without mutating running work." Building from the component table ships the worst possible
mislabel on plan-wide cancellation. `DESIGN.md` never contains the string `Cancel plan`.
*Fix:* `:155` → `Cancel plan`; add the label to `DESIGN.md`'s Activity Operation Row.

### High (8)

**[State] `Verifying` is normative in one spine and absent from the other**
(`EXPERIENCE.md:228`; `DESIGN.md:215`)
`EXPERIENCE.md:228` "A successful process exit remains `Verifying` until affected Manager state
refreshes." `grep -c 'Verifying' DESIGN.md` → 0, and the Activity Operation Row required-state list
at `DESIGN.md:215` ends "…cancelling, verified, failed, cancelled, skipped, timed out". It is
load-bearing downstream: `epics.md:818`, `:844`. So `DESIGN.md:236` "Verify refreshed state before
coloring an update successful" has no visual contract.
*Fix:* add `Verifying` to `DESIGN.md:215` and to `EXPERIENCE.md:155`'s enumeration.

**[Epics] Two live stories have zero contract on either spine** (UX-PB.2f, UX-PB.4e)
Both require legacy-Operation History labeling. `grep -ci legacy` → 0 in both spines, and
`DESIGN.md:217` gives History Plan Row only "Success, partial, failed, cancelled, timed out,
interrupted, retry-linked".

**[Epics] Quoted UI label literals diverge from the binding epic**
`epics.md` uses `Confirm N Updates` 10×, `docs/DECISIONS.md` 2×, `docs/SPEC.md` 1×; `EXPERIENCE.md`
uses `Confirm # updates` 6× and `Confirm N Updates` 0×. The opt-out checkbox carries two different
labels: `docs/SPEC.md:73` / `epics.md:1060` `Disable upgrade plan command execution confirmation`
vs `EXPERIENCE.md:251` `Skip confirmation for future upgrade plans`.

**[Accessibility] Checkbox target size contradicts the Accessibility Floor, and code takes the
smaller value** (`DESIGN.md:203` vs `EXPERIENCE.md:327`)
`DESIGN.md:203` "16–18px aligned control" against `EXPERIENCE.md:327` "Hit targets are at least
28 × 28px for compact desktop controls and 36px where space permits." Code lands on the smaller:
`src/components/manager/PackageRow.tsx:98` is `h-4 w-4` (16px). This is the control that governs
plan membership.

**[Accessibility] Neither spine records the WebKit native-appearance constraint**
`grep -ci 'box-shadow'` → 0 and `WebKit` → 0 in both files; `DESIGN.md:119` still calls the token a
"ring". The reason lives only in `docs/DECISIONS.md` D35 and AD-27. A story satisfying
`EXPERIENCE.md:318` "at least 2px wide" with `ring-2` reships the exact defect D35 closed at 31
sites — and it is invisible to grep, since it returns as a different class name.

**[Accessibility] Focus destination undefined when the draft sidecar closes**
(`EXPERIENCE.md:181`; matrix `:294-302`)
`:181` "Removing the final item closes the sidecar" names no focus destination, and the nine-row
Focus transitions matrix has no such row — asymmetric, since the opening case *is* covered at `:179`.
Pressing the last `Remove` from the keyboard drops focus to `<body>`.

**[Token] Non-text contrast has no stated target and the selection tokens fall below 3:1**
Computed from the spine's own hex: `borderStrong` vs `surface` = 2.33:1, vs `overlay` = 1.97:1;
`accentSubtle` vs `surface` = 1.18:1 — all below WCAG 1.4.11's 3:1 floor, with no non-text target
stated anywhere in either spine. `EXPERIENCE.md:318` permits `borderStrong` to indicate selection.
All *text* pairs pass 4.5:1, including `DESIGN.md:122`'s claim for `textMuted` (4.99:1 through
overlay — true, with only 0.49 of margin). This matters because `ARCHITECTURE-SPINE.md:328` confirms
automated contrast does not exist; the by-eye checker has no reference values.

**[Usability] The 900×600 minimum does not fit the specified content**
Arithmetic from the authoritative mockup's own CSS: sidebar 190 + padding 32 + gap 14 + sidecar 370
leaves **294px** for a `.grid-row` declaring `min-width: 560px`, pushing Version, Status and Source
behind horizontal scroll. That contradicts `DESIGN.md:171`'s promise to "preserve the sidebar, the
primary action, status words, and version evidence" at minimum size. The high-zoom rule cannot
rescue it: its 720px trigger measures the window (900), not the pane.

### Medium (7)

**[Code] Radius tokens diverge and most are undefined in code** — `DESIGN.md:57-63` declares 6
values; `src/styles/theme.css:46` defines 2, and both shared names disagree (`--radius-card: 10px`
vs `card: 13px`; `--radius-control: 6px` vs `control: 7px`). Dialogs borrow `rounded-card`.

**[Code] Sidebar width** — `DESIGN.md:166` "A 190px left sidebar at ordinary zoom" vs
`src/components/shell/Sidebar.tsx:63` `w-60` (240px), and `docs/SPEC.md:227`.

**[Visual reference] The "authoritative" mockups are largely off-palette** — `DESIGN.md:198` calls
the four `mockups/*.html` "Authoritative visual references". They contain 75 distinct hexes; only 19
are in the 21-hex `DESIGN.md` palette — **56 are not**, including off-palette accents `#4F77FF`,
`#529CFF`, `#76B4FF` beside the real `#65A7FF`. Now that the code paints the real palette, the
mockups are the least accurate reference in the folder. `DESIGN.md:198` does state
spines-win-on-conflict, which contains the damage.

**[Token] Health-meter 80–99% band has no colour value** — `DESIGN.md:138` "80–99% current:
interpolate through yellow-green while maintaining contrast" names no token and no hex, while
`:134` requires "one solid fill color—not a gradient fill" and `:246` forbids "a gradient inside the
Package Health Meter". Implementable only by invention.

**[State] `offline` and `no Managers` are required states with no behavioral coverage** —
`DESIGN.md:222` requires State Panel "Loading, empty, offline, no Managers, no active upgrade,
interrupted, fatal"; `grep -ic offline EXPERIENCE.md` → 0.

**[Inheritance] `EXPERIENCE.md:330` claims verification the release process does not perform** —
"Packaged acceptance verifies focus, final-row reachability, selection scope, completion
announcements, and no overlap at 100%, 150%, and 200% zoom with VoiceOver and 101 Package rows."
`docs/RELEASE-CHECKLIST.md:90-93` carries one VoiceOver pass and nothing else on that list.

**[Owner decision] Transient selection has no owning invariant** — `ARCHITECTURE-SPINE.md:1050`
records the conflict as OPEN: `docs/SPEC.md` F5 has Esc clear the transient selection while
`EXPERIENCE.md:143` has "selection immediately adds/removes Upgrade Plan membership", and
`src/store/packages.ts:17` ships a live `selection` set `PlanIntent` cannot represent. The UX side is
deliberate — `.memlog.md:75` records "Eliminate the separate temporary selection and Add Selected
layer" — so `docs/SPEC.md` F5 is the stale side, and F5 was never added to §0.1's supersession list.
But `reviews/review-reconcile-epics.md:166` flags a real cost to the spine's model: a shift-range
across 100 rows becomes 100 Rust round-trips against NFR-3 (`epics.md:88`). **Not fixable by editing
the spine** — it needs the owner decision the architecture spine asked for.

### Low (2)

**[Motion] Reduced motion freezes the indeterminate spinner** — `src/styles/theme.css:61-66` kills
all animation; `src/components/primitives/Spinner.tsx:13` is `animate-spin`. `EXPERIENCE.md:310`
asks to "remove nonessential animation", but a running indicator is essential state. The automated
test asserts only an idle button.

**[Voice] Sidecar `Cancel` scoping** — `EXPERIENCE.md:110`'s three-way `Cancel` / `Cancel plan` /
`Cancel operation` glossary is correct but not mirrored in `DESIGN.md`, which never names any of the
three.

## Code defects surfaced by this run (not spine defects — the spine is right)

These belong in a code fix, not an Update run.

**Critical — shipped text/background contrast failures.** `src/components/primitives/Button.tsx:7`
`primary: "bg-accent text-white hover:bg-accent-hover disabled:bg-accent/40"` and `:11`
`danger: "bg-danger text-white hover:brightness-110 disabled:opacity-50"`, plus
`src/components/shell/UpdateStatusItem.tsx:63`. Computed: white on accent `#65A7FF` = **2.46:1**,
on hover `#7DB3FF` = **2.15:1**, on danger `#FF8793` = **2.30:1** — all fail 4.5:1 *and* 3:1 against
`EXPERIENCE.md:317`. The tokens that exist to prevent this have zero consumers:
`grep -rn 'on-accent\|on-success' src/` matches only `theme.css`. Using them gives 7.74:1 and
12.15:1. D35 adopted the palette but not the on-fill pairs, and contrast is by-eye at release.

**High — explanatory-disabled checkboxes use native `disabled`.** `EXPERIENCE.md:326` requires
`aria-disabled="true"` and explicitly forbids native `disabled`; `src/components/manager/PackageRow.tsx:92`
is `disabled={checkboxDisabled}` and `grep -rn aria-disabled src/ | wc -l` → 0. The pinned-Package
explanation is keyboard-unreachable today.

## What is verified clean

- **Colour parity:** all 22 `DESIGN.md` colours present in `theme.css` — 0 missing, 0 value mismatches.
- **Focus migration:** 31 production sites, all `outline-2`; zero `ring-*` focus utilities; zero
  `outline-none`; the one `ring-accent` at `PackageRow.tsx:85` is the navigation highlight D35 exempts.
- **Component parity:** 22 components in `DESIGN.md` frontmatter, 22 rows in `DESIGN.md#Components`,
  22 rows in `EXPERIENCE.md#Component Patterns` — names identical across all three.
- **Flow coverage:** AJ-1…AJ-6 each carry a named protagonist, numbered steps, a climax beat and a
  failure path; names match `epics.md:148-153` verbatim.
- **Visual references:** all four `mockups/*.html` linked inline at the relevant section; `imports/`
  empty; no orphans; spines-win-on-conflict stated once at `DESIGN.md:198`.
- **Shape fit:** `DESIGN.md` sections in canonical order; all eight `EXPERIENCE.md` defaults present
  plus two justified triggered sections (Inspiration & Anti-patterns, Responsive & Platform).
- **Token refs resolve:** both `{colors.focusRing}` and `{colors.borderStrong}` at `EXPERIENCE.md:318`
  resolve to `DESIGN.md` frontmatter.
- **Control heights:** `DESIGN.md:202`'s 28px/36px clear WCAG 2.5.8's 24×24 with margin.
- **`EXPERIENCE.md` tracks the *new* plan model**, not the retired sheet — `:153`, `:154`, `:188`,
  `:256` all agree with `docs/SPEC.md:28-30`; it is `docs/SPEC.md:251`/`:284` that are stale.

## Recommended sequencing

1. **Owner decision** on transient selection (`ARCHITECTURE-SPINE.md:1050`) — blocks UX-PB.1a and
   Story 3.5, and may change `EXPERIENCE.md#Component Patterns`. Do this first.
2. **`bmad-ux` Update run** for everything in Critical/High/Medium above except the owner decision.
3. **Separate code fix** for the contrast failures and `aria-disabled` — no spec change needed.
4. **`docs/SPEC.md` pass** (§11 component tree, F5, the §0.1 supersession list) — outside `bmad-ux` scope.

## Reviewer files

- `review-rubric.md`
- `review-code-consistency.md`
- `review-epics-consistency.md`
- `review-accessibility.md`
- `review-usability.md`
