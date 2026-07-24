---
name: Pack-Manager Aurora Control Deck
description: Dark-only native macOS design system for a calm, legible package-management control deck.
status: final
project: Pack-Manager
created: 2026-07-23
updated: 2026-07-24
sources:
  - _bmad-output/planning-artifacts/prds/prd-Pack-Manager-2026-07-22/prd.md
  - _bmad-output/planning-artifacts/prds/prd-Pack-Manager-2026-07-22/addendum.md
  - _bmad-output/planning-artifacts/architecture/architecture-Pack-Manager-2026-07-23/ARCHITECTURE-SPINE.md
  - _bmad-output/planning-artifacts/epics.md
  - _bmad-output/planning-artifacts/implementation-readiness-report-2026-07-23.md
colors:
  background: "#090C13"
  shell: "#0F1420"
  surface: "#151C2A"
  raised: "#1B2434"
  overlay: "#202A3C"
  inset: "#070B12"
  border: "#2A3548"
  borderStrong: "#465773"
  focusRing: "#F4F7FB"
  textPrimary: "#F4F7FB"
  textSecondary: "#AEB8C7"
  textMuted: "#8D99AA"
  accent: "#65A7FF"
  accentHover: "#7DB3FF"
  accentSubtle: "#172A46"
  onAccent: "#07101D"
  success: "#72E6A0"
  onSuccess: "#07140D"
  warning: "#F1C875"
  danger: "#FF8793"
  info: "#62E7D8"
  violet: "#B59CFF"
typography:
  fontFamily:
    sans: '-apple-system, BlinkMacSystemFont, "SF Pro Text", "Segoe UI", system-ui, sans-serif'
    display: '-apple-system, BlinkMacSystemFont, "SF Pro Display", "Segoe UI", system-ui, sans-serif'
    mono: 'ui-monospace, "SF Mono", "SFMono-Regular", Menlo, monospace'
  fontWeight:
    regular: 400
    medium: 550
    semibold: 650
    bold: 750
    heavy: 850
  fontSize:
    micro: 10px
    caption: 11px
    bodySmall: 12px
    body: 13px
    label: 14px
    headingSmall: 16px
    heading: 20px
    summaryValue: 28px
rounded:
  control: 7px
  navigation: 9px
  card: 13px
  dialog: 16px
  window: 18px
  pill: 999px
spacing:
  xxs: 4px
  xs: 8px
  sm: 12px
  md: 16px
  lg: 20px
  xl: 24px
  xxl: 32px
components:
  button: Blue primary, neutral secondary, text, and danger variants with visible focus and honest disabled states.
  checkbox: Direct plan-membership control with selected, unselected, disabled, and explanatory-disabled states.
  sidebar-navigation: Outlined-icon primary navigation with a collapsible Manager list.
  system-summary-card: Three-line system fact with a compact value and lowercase context.
  manager-card: Dashboard overview of one Manager, its reliability, self-update state, and Package health.
  manager-header: Dedicated Manager identity, version, status, path, self-update action, and filter context.
  package-filter: Compact count-bearing All Packages, Updates, and Pinned control.
  package-grid: Virtualized, keyboard-navigable Package inventory with stable row identity and bulk scope.
  package-row: Package status and version row whose checkbox directly controls Upgrade Plan membership.
  status-chip: Text-plus-color state label for Manager, Package, Activity, and result status.
  health-meter: Solid-color filled meter whose length and text describe the proportion of Packages current.
  upgrade-sidecar: Contextual right sidecar that stages a draft plan, then transforms into Activity and Results.
  confirmation-dialog: Modal final-authorization step with exact commands and an optional future-confirmation preference.
  activity-operation-row: Human-readable live or replayed Manager/Package operation state with trustworthy progress.
  results-summary: Persistent terminal outcome grouped by Manager with verification, diagnosis, and safe recovery.
  history-plan-row: One completed Upgrade Plan summary with result, scope, time, and duration.
  tooltip-popover: Anchored explanation for unavailable or safety-constrained actions.
  settings-section: Labeled preference group with explanation, saved state, and validation.
  application-update-status: Separate Pack-Manager application update state and explicit Restart to Update action.
  command-output-disclosure: Secondary reveal for exact commands, retained output, and diagnostic evidence.
  state-panel: Purpose-built empty, loading, offline, interrupted, or unrecoverable state.
  brief-notification: Optional transient reinforcement that never replaces persistent status.
---

# Brand & Style

Pack-Manager should feel like a quiet native control deck: technically capable, visually calm, and always honest about what it knows. The selected direction is **Aurora Control Deck**—a dark blue-graphite workspace with restrained glass depth, crisp blue controls, compact information density, and status colors that remain legible without becoming decorative noise.

The visual personality borrows only general qualities observed in Kanban Pro: layered dark surfaces, compact outlined controls, visible hierarchy, a 4/8-based rhythm, and smooth short motion. Pack-Manager must remain an original design. Do not copy Kanban Pro code, assets, branding, layouts, or distinctive component treatments.

The interface is dark-only for this release. Native macOS conventions—traffic-light window controls, system typography, expected focus behavior, and restrained motion—take priority over web-dashboard decoration.

# Colors

All product colors must be exposed through semantic Tailwind tokens in `src/styles/theme.css`. Product components consume token names; they do not hardcode hex values.

| Token           |     Value | Purpose                                                               |
| --------------- | --------: | --------------------------------------------------------------------- |
| `background`    | `#090C13` | Deep application canvas                                               |
| `shell`         | `#0F1420` | Window chrome and stable navigation surfaces                          |
| `surface`       | `#151C2A` | Cards, table bodies, and sidecar groups                               |
| `raised`        | `#1B2434` | Hovered rows, inputs, and elevated panels                             |
| `overlay`       | `#202A3C` | Dialogs, popovers, and strongly elevated surfaces                     |
| `inset`         | `#070B12` | Command and output wells                                              |
| `border`        | `#2A3548` | Default structural dividers                                           |
| `borderStrong`  | `#465773` | Selected or emphasized boundaries                                     |
| `focusRing`     | `#F4F7FB` | Dedicated high-contrast keyboard-focus ring                           |
| `textPrimary`   | `#F4F7FB` | Headings and primary values                                           |
| `textSecondary` | `#AEB8C7` | Descriptions and ordinary metadata                                    |
| `textMuted`     | `#8D99AA` | Tertiary labels with 4.5:1 minimum contrast through the overlay layer |
| `accent`        | `#65A7FF` | Primary actions, active navigation, and running state                 |
| `accentHover`   | `#7DB3FF` | Hovered primary action                                                |
| `accentSubtle`  | `#172A46` | Selected-row or active-destination wash                               |
| `onAccent`      | `#07101D` | Text/icons on bright blue primary-action fills                        |
| `success`       | `#72E6A0` | Verified current or successfully completed                            |
| `onSuccess`     | `#07140D` | Text/icons on bright green confirmation fills                         |
| `warning`       | `#F1C875` | Outdated, waiting, stale, or recoverable caution                      |
| `danger`        | `#FF8793` | Failure, cancellation, destructive consequence                        |
| `info`          | `#62E7D8` | Neutral live status and supporting information                        |
| `violet`        | `#B59CFF` | Rare secondary accent; never a core status                            |

Status colors always travel with a word, icon, count, meter length, or version label. The Package-health scale selects one solid fill color—not a gradient fill:

- 0–49% current: `danger`
- 50–79% current: `warning`
- 80–99% current: interpolate through yellow-green while maintaining contrast
- 100% current: `success`

The meter length represents the proportion current. Its accessible label repeats the exact current and total counts.

# Typography

Use native system faces so the desktop app feels at home on macOS and remains fast.

| Role                       | Size / line height |       Weight | Notes                                                     |
| -------------------------- | -----------------: | -----------: | --------------------------------------------------------- |
| Window title / micro label |     10–11px / 14px |      650–750 | Uppercase only for short category labels and Status Chips |
| Compact metadata           |        12px / 17px |      400–650 | Paths, counts, timestamps, and helper text                |
| Body                       |        13px / 19px |      400–550 | Default dense desktop copy                                |
| Control label              |        14px / 20px |          650 | Buttons and important row actions                         |
| Section heading            |        16px / 22px |      650–750 | Card groups and sidecar sections                          |
| Page heading               |        20px / 26px |          750 | One per workspace                                         |
| System summary value       |        28px / 31px |      750–850 | `Ready`, `Warning`, `12`, `76`; never oversized           |
| Commands / versions        |        12px / 18px | 400–650 mono | Exact evidence only                                       |

Use sentence case for controls, headings, and explanatory copy. The third line of every System Summary Card is lowercase supporting context. Uppercase is reserved for compact state chips such as `NO UPDATES`, `UPDATE AVAILABLE`, `REFRESH FAILED`, and `PINNED`.

# Layout & Spacing

The spacing system is based on 4px and 8px increments: 4, 8, 12, 16, 20, 24, and 32px. Prefer 16px internal card padding, 20–24px between page sections, and 8–12px between related metadata.

The native desktop window supports a minimum content size of **900 × 600**. The default shell consists of:

1. A compact native title bar.
2. A 190px left sidebar at ordinary zoom.
3. A flexible main workspace.
4. A contextual 340–380px Upgrade Sidecar only when a draft, active execution, or Results Summary exists.

When the sidecar is hidden, the main workspace reclaims its width. Do not reserve an empty column. At the minimum window size and ordinary zoom, preserve the sidebar, the primary action, status words, and version evidence; reduce secondary spacing before truncating meaningful copy. Tables and long output scroll inside their own regions instead of increasing the window minimum.

At 150–200% zoom, or whenever the usable CSS width drops below 720px, switch to a high-zoom layout: collapse the sidebar into an accessible navigation rail or temporary panel and present the Upgrade Plan, Activity, or Results as a full-workspace/stacked surface instead of retaining a fixed sidecar. Required headings, status, versions, actions, focus order, and a visible route back must remain available without overlapping panes or two-dimensional scrolling for the primary task.

Use two Manager Card columns when space permits and one column when the main workspace becomes too narrow. The Package Grid should support more than 100 rows through virtualization or equivalent efficient rendering. Command output may retain up to 5,000 lines without forcing the rest of the page to reflow.

# Elevation & Depth

Depth is restrained and functional:

- The window uses a deep shell with a subtle border, blur, and one broad shadow.
- Cards use translucent `surface`/`raised` layers and a faint inner highlight.
- The Upgrade Sidecar is separated by a strong border and slight shadow, not a floating detached drawer.
- The Confirmation Dialog uses `overlay`, a visible boundary, a dark scrim, and focus containment.
- Tooltips and popovers use the highest local elevation and must not be translucent enough to harm readability.
- Inset command/output regions use `inset` with a strong border and mono text.

No glass effect may reduce text contrast. Avoid stacked shadows, neon glows on status colors, and decorative blur behind dense tables.

# Shapes

Use 7px radii for controls, 9px for navigation and compact rows, 13px for cards, 16px for dialogs, and 18px for the outer window. Pills are reserved for Status Chips, compact filters, and small count indicators.

Outlined icons use rounded line caps and joins at a consistent 1.8–2px stroke. Do not mix outline, filled, letter, and emoji icon styles in primary navigation.

# Components

Authoritative visual references: [Manager workspace and Upgrade Plan](./mockups/manager-workspace.html), [live Activity and Results](./mockups/activity-results.html), [read-only Activity replay](./mockups/history-replay.html), and [Settings and application updates](./mockups/settings-app-updates.html). `DESIGN.md` and `EXPERIENCE.md` remain authoritative whenever a mock omits a state or conflicts with either spine.

| Component                     | Visual contract                                                                                                                                                                                                                                                                                                                                                                                                                      | Required states                                                                                                                                         |
| ----------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ | ------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **Button**                    | Blue primary, quiet neutral secondary, underlined text action, or consequence-colored danger text. Minimum height: 28px for compact controls and 36px for standard controls. Keyboard focus uses a separated 2px `focusRing` outline; selection color never substitutes for focus.                                                                                                                                                   | Default, hover, pressed, focus-visible, disabled, busy                                                                                                  |
| **Checkbox**                  | 16–18px aligned control. Checked state uses accent and a visible mark. Explanatory-disabled controls look unavailable but retain the same 2px `focusRing`.                                                                                                                                                                                                                                                                           | Unchecked, checked, mixed, focus-visible, disabled, explanatory-disabled                                                                                |
| **Sidebar Navigation**        | Cohesive outlined icons, 38px rows, active accent wash and left inset. Managers is a disclosure with indented detected Managers.                                                                                                                                                                                                                                                                                                     | Default, hover, active, focus-visible, expanded, collapsed                                                                                              |
| **System Summary Card**       | Three lines: name, 28px value, lowercase context. Status dot accompanies text.                                                                                                                                                                                                                                                                                                                                                       | Ready, Warning, no Managers, loading                                                                                                                    |
| **Manager Card**              | Name plus muted installed version; short role; executable path; Manager Status Chip; optional self-update delta; managed Package count before Package updates; Health Meter or refresh recovery.                                                                                                                                                                                                                                     | Current, self-update available, refresh failed with last-good snapshot, unavailable                                                                     |
| **Manager Header**            | Manager identity, version, short role, path, status, self-update delta/action, and Package Filter context. No separate self-update row. A staged self-update shows `IN PLAN` plus a visible `Remove` action.                                                                                                                                                                                                                         | Current, update available, in plan/removable, refresh failed, unavailable                                                                               |
| **Package Filter**            | Compact count-bearing segmented control: All Packages, Updates, Pinned.                                                                                                                                                                                                                                                                                                                                                              | Default, selected, hover, focus-visible, zero-count                                                                                                     |
| **Package Grid**              | Virtualized inventory with persistent column headers, a count-bearing bulk Checkbox, one visibly focused row, and stable visual identity while rows recycle.                                                                                                                                                                                                                                                                         | Loading, empty, filtered, partial/mixed, 101+ rows, keyboard focus, high zoom                                                                           |
| **Package Row**               | Direct plan Checkbox, name, current/target versions, status, and relevant metadata. Actionable updates sort first.                                                                                                                                                                                                                                                                                                                   | Update available, in plan, pinned, excluded, current, unavailable                                                                                       |
| **Status Chip**               | Short uppercase state text, 1px border, subtle tinted fill, semantic foreground.                                                                                                                                                                                                                                                                                                                                                     | Neutral, info, success, warning, danger                                                                                                                 |
| **Health Meter**              | Dark track and one solid fill. Fill length equals proportion current; fill color follows health scale.                                                                                                                                                                                                                                                                                                                               | 0–100%, unknown, refresh failed                                                                                                                         |
| **Upgrade Sidecar**           | Grouped human-readable Manager/Package changes with Updates, Managers, and Commands counts. Every staged Package and Manager self-update has a visible remove control. Hidden when empty; transforms in place. When confirmation is off, show a persistent warning and `Run # updates`.                                                                                                                                              | Draft editable, command revealed, confirmation off, confirming, executing, results                                                                      |
| **Confirmation Dialog**       | `Proceed with Upgrade Plan?`, exact command well, neutral `Change Plan`, green final confirm, optional skip-future checkbox and warning.                                                                                                                                                                                                                                                                                             | Open, checkbox off/on, confirming, plan stale, error                                                                                                    |
| **Activity Operation Row**    | Package or Manager operation with state text, version evidence, time, trustworthy progress, and safe action area when stalled or interaction-blocked.                                                                                                                                                                                                                                                                                | Queued, waiting, running indeterminate, running determinate, stalled, interaction required, cancelling, verified, failed, cancelled, skipped, timed out |
| **Results Summary**           | Persistent terminal outcome grouped by Manager. Verified rows compact; failed rows expose explanation, guidance, evidence, and secondary Retry. The single dismissal label is `Done`.                                                                                                                                                                                                                                                | Success, partial, failed, cancelled, timed out, interrupted, retry scope                                                                                |
| **History Plan Row**          | One confirmed execution attempt per row with time, overall result, verified/total wording, Manager count, duration, and optional source-retry link.                                                                                                                                                                                                                                                                                  | Success, partial, failed, cancelled, timed out, interrupted, retry-linked                                                                               |
| **Tooltip/Popover**           | Opaque anchored explanation with pointer alignment and keyboard availability.                                                                                                                                                                                                                                                                                                                                                        | Hover, focus, click-pinned, dismissed                                                                                                                   |
| **Settings Section**          | Clear group title, short explanation, aligned editable controls, and per-control immediate-save feedback. A value becomes active only after persistence succeeds. Advanced durations use labeled numeric/unit controls, and Application log level uses a labeled selector rather than static text.                                                                                                                                   | Default, editing, saving, saved, validation error, save failed/reverted, unavailable                                                                    |
| **Application Update Status** | Detailed app-version state lives in Settings → Pack-Manager updates; one restrained application-level badge labeled `Pack-Manager Update Ready!` links there. The update card names `Pack-Manager` without repeating the target version in its heading, then shows one unbroken version delta with the installed version in warning yellow and target version in success green. It never appears in Package Upgrade Plan or History. | Checking, current, available, downloading, ready to restart, blocked by active work, manual install, error                                              |
| **Command Output Disclosure** | Bold underlined reveal label followed by inset mono evidence.                                                                                                                                                                                                                                                                                                                                                                        | Collapsed, expanded, loading, unavailable, truncated-with-log                                                                                           |
| **State Panel**               | Purpose-specific title, explanation, and one safe next action where available.                                                                                                                                                                                                                                                                                                                                                       | Loading, empty, offline, no Managers, no active upgrade, interrupted, fatal                                                                             |
| **Brief Notification**        | Small transient reinforcement with text and optional icon. Never sole confirmation.                                                                                                                                                                                                                                                                                                                                                  | Informational, success, warning, failure                                                                                                                |

# Do’s and Don’ts

## Do

- Make the current state, next action, and consequence understandable without reading a terminal command.
- Keep Manager self-update status separate from the health of its Packages.
- Put installed Manager versions beside Manager names and self-update deltas beneath the Manager status.
- Show `34 managed packages · 8 package updates` in that order.
- Use short Manager descriptions that fit: `macOS package manager`, `Runtime version manager`, `JavaScript package manager`, `Python tool manager`, `Rust toolchain manager`, and `Mac App Store CLI manager`.
- Keep exact commands available before execution and retained afterward as evidence.
- Preserve last-good data after refresh failure and label its age.
- Verify refreshed state before coloring an update successful.
- Use visible focus, non-color status text, and reduced-motion behavior everywhere.
- Make every staged Upgrade Plan item visibly removable and show the exact scope of bulk Package actions.

## Don’t

- Do not show a permanent Manager list on the Dashboard.
- Do not execute a Package or Manager update directly from a row or header.
- Do not display raw commands as the primary description of an Upgrade Plan.
- Do not show an empty Upgrade Sidecar or list unconfirmed plans in Activity.
- Do not use a gradient inside the Package Health Meter, fabricate progress percentages, or use color as the only signal.
- Do not call the system `Ready` when any Manager refresh has failed; use `Warning` with the exact failure count.
- Do not use `Prompts` as a plan summary or expose internal route/owner language without plain-language context.
- Do not ask for `sudo`, passwords, or administrator elevation.
- Do not hide failure evidence behind automatic retries or imply a retry will solve a deterministic problem.
- Do not mix Pack-Manager application updates into Package Upgrade Plans, Activity, Results, or History.
- Do not use `borderStrong` alone as keyboard focus, treat only rendered virtual rows as a bulk-selection scope, or retain fixed panes when zoom makes them overlap.
