# Reconciliation review — UX design vs. Architecture Spine revision 4

- **Run at:** 2026-07-25
- **Spine under review:** `_bmad-output/planning-artifacts/architecture/architecture-Pack-Manager-2026-07-23/ARCHITECTURE-SPINE.md` (`artifact_revision: 4`)
- **UX inputs read in full:** `DESIGN.md` (252 lines), `EXPERIENCE.md` (460 lines), `validation-report.md` (167 lines, partial)
- **Code read:** `src/styles/theme.css`, `src/store/ui.ts`, `src/hooks/useKeyboard.ts`, `src/components/shell/ToastHost.tsx`

Path abbreviations used below:

- `SPINE` = `_bmad-output/planning-artifacts/architecture/architecture-Pack-Manager-2026-07-23/ARCHITECTURE-SPINE.md`
- `DESIGN` = `_bmad-output/planning-artifacts/ux-designs/ux-Pack-Manager-2026-07-23/DESIGN.md`
- `EXP` = `_bmad-output/planning-artifacts/ux-designs/ux-Pack-Manager-2026-07-23/EXPERIENCE.md`

## Folder inventory

```
_bmad-output/planning-artifacts/ux-designs/ux-Pack-Manager-2026-07-23/
  .memlog.md              22062 bytes
  .working/               40 entries
  DESIGN.md               29880 bytes
  EXPERIENCE.md           57192 bytes
  imports/                empty
  mockups/                activity-results.html, history-replay.html,
                          manager-workspace.html, settings-app-updates.html
  review-accessibility.md 10325 bytes
  review-usability.md     10049 bytes
  validation-report.html  17385 bytes
  validation-report.md    10717 bytes
```

All four mockups named as authoritative visual references at `DESIGN:198` exist.

---

## Verdict

**AD-17's sidecar model is corroborated in substance and wrong in its stated
predicate.** Every structural claim AD-17 makes — layout region not dialog, one
instance, persists across navigation, no reserved empty column, transforms in
place rather than opening a second surface — is independently stated in the UX
design. But the one *mechanical* sentence AD-17 adds ("visibility is driven by
draft non-emptiness") is narrower than the UX's visibility rule and, taken
literally, destroys Results.

Nine further findings follow: five things the UX requires that no AD covers,
and four places an AD and a stated UX behavior disagree.

---

## 1. Does the UX corroborate AD-17's sidecar model?

### Corroborated — quote by quote

AD-17 says:

```
SPINE:370-375
- **Rule:** The sidecar is a single layout region whose visibility is driven by
  draft non-emptiness — not a `ui.dialog` kind and not a `DialogHost` child.
  Exactly one instance exists and it persists across `ActiveView` changes without
  losing membership or scroll identity. When hidden, the workspace reclaims its
  width with no reserved empty column. A confirmed attempt replaces the sidecar's
  content with live attempt status rather than opening a second surface.
```

| AD-17 clause | UX corroboration |
| --- | --- |
| layout region, not a dialog/drawer | `DESIGN:183` "The Upgrade Sidecar is separated by a strong border and slight shadow, not a floating detached drawer." |
| a region of the default shell | `DESIGN:164-169` "The default shell consists of: … 4. A contextual 340–380px Upgrade Sidecar only when a draft, active execution, or Results Summary exists." |
| exactly one instance | `EXP:208` "The sidecar and full Activity are two presentations of one shared live state, never separate executions." |
| persists across `ActiveView` changes | `EXP:59` "Persists across Dashboard and Manager navigation, remains editable, and lets the user remove each item; hidden when empty" |
| persists across `ActiveView` changes | `EXP:144` "choosing a child Manager changes workspace while preserving any draft sidecar." |
| persists across `ActiveView` changes | `EXP:413` "the sidecar opens on the first addition, provides Remove on every staged item, and persists while Sallvain visits other Managers." |
| workspace reclaims width, no empty column | `DESIGN:171` "When the sidecar is hidden, the main workspace reclaims its width. Do not reserve an empty column." |
| no reserved empty column | `EXP:351` (anti-pattern) "A permanently visible empty right drawer." |
| replaces content rather than opening a second surface | `DESIGN:213` "Hidden when empty; transforms in place." |
| replaces content rather than opening a second surface | `EXP:153` "It persists across Manager changes and transforms into Activity and Results." |
| the confirmation dialog is a separate modal | `EXP:154` "Opens only from `Confirm # updates`, dims the background, traps focus, shows exact commands, and offers `Change Plan` plus final confirmation." |

Nothing in either UX document calls the sidecar a dialog, a sheet, or a modal.
The `not a ui.dialog kind` half of AD-17 is a clean win: it overturns the
verified baseline (`SPINE:87-88` "The Upgrade Plan is currently transient dialog
state (`ui.dialog` `{ kind: "upgradePlan" }`, discarded by `closeDialog`)"),
which the tree confirms at `src/store/ui.ts:20` `| { kind: "upgradePlan"; plan: UpgradePlan }`.

### Contradicted — the visibility predicate

**FINDING 1 — CRITICAL. `visibility is driven by draft non-emptiness` is a
one-term predicate where the UX states three, and the missing term is Results.**

The UX makes visibility a disjunction:

> `DESIGN:169` "A contextual 340–380px Upgrade Sidecar only when a draft, active execution, or Results Summary exists."

The third term is load-bearing and stated twice more:

> `EXP:62` "The same sidecar persists at terminal state until `Done`"

> `EXP:225` "The sidecar becomes Results and remains until the user chooses `Done`."

**Failure scenario.** UX-PB.2c confirms a plan. `execute_plan` mints a
`planAttemptId` and the draft is consumed, so the draft is empty. A builder
implementing `SPINE:370-371` literally unmounts the sidecar at the instant of
confirmation — taking live Activity and the Results Summary with it, and
breaking `EXP:401-402` ("final confirmation atomically admits the whole plan and
the sidecar becomes live Upgrade Activity … At terminal state, Activity becomes
a persistent Results Summary"). The very next AD-17 sentence
(`SPINE:374-375` "A confirmed attempt replaces the sidecar's content with live
attempt status") rescues the *running* case by implication, but there is no
sentence anywhere in the spine that keeps the sidecar mounted after the attempt
reaches a terminal state and before the user presses `Done`.

The charitable reading — visible when `draft non-empty OR attempt present` —
still fails, because a terminal attempt is *finished*; what keeps the sidecar up
is an unacknowledged-Results flag the domain model does not contain.
`SPINE:307-314` (`PlanAttempt`) has `state: admitted | running | verifying | terminal`
and no acknowledgement field, and `EXP:301` makes `Done` a real user action with
a defined focus consequence ("Results `Done` | Restore focus to the most relevant
surviving page action").

**Recommended repair:** state the predicate as the disjunction the UX states, and
add the acknowledgement term to the `PlanAttempt` minimum under AD-16, e.g.
`resultsAcknowledged` / `dismissedAt`. Also decide explicitly whether admission
clears the draft — `EXP:96` only defines the removal path ("Removing the last
draft item closes the Upgrade Sidecar"), never the admission path.

---

## 2. What the UX requires that no AD covers

### FINDING 2 — HIGH. No AD owns the single atomic status-announcement channel, and the tree already has competing live regions.

The UX specifies one cross-cutting singleton with an explicit event-filtering
policy and a cross-component dedupe contract:

> `EXP:322` "One atomic Activity/Results status channel announces plan start, a changed waiting reason, an action-required failure, each Manager's completion summary, and the final plan outcome. It uses polite priority by default and assertive priority only for an immediate safety action; it never announces queued rows, progress ticks, or command-output lines."

> `EXP:323` "Brief Notifications suppress duplicate speech when the status channel already announced the same event and never move focus."

> `EXP:163` "It never replaces persistent status and must not repeat an announcement already emitted by the single Activity/Results status channel."

> `EXP:324` "Opening/closing dialogs and transformations follow the Focus transitions matrix; Results receives one accessible summary announcement."

This is architecture, not styling: it is a mounted-once region that subscribes to
the same high-frequency stream AD-18 defines (`SPINE:396-398` "Operation status,
output and stall events, transcript metadata, and journal start/finish records
carry `planAttemptId`") and *suppresses* almost all of it, plus a suppression
protocol another component must honour.

No AD names a live region, an announcement channel, announcement priority, or
dedupe. AD-11 schedules only *verification* of the outcome
(`SPINE:225-226` "one manual VoiceOver focus-order and completion-announcement pass
on the release checklist"), never ownership. The Capability map assigns
UX-PB.3a–3g to "Rust event dispatch + React attempt views" (`SPINE:480`) —
plural views, no singleton.

**Verified against the tree:** there is no such channel today, and there are at
least five uncoordinated announcers, including a nested pair inside the Brief
Notification component itself — `src/components/shell/ToastHost.tsx:65`
`aria-live="polite"` wrapping `src/components/shell/ToastHost.tsx:30`
`role="status"`, plus `src/components/primitives/Spinner.tsx:11`,
`src/components/manager/HealthBanner.tsx:29`,
`src/components/shell/SidebarManagerItem.tsx:62`, and
`src/components/dialogs/UpgradePlanSheet.tsx:304`.

**Divergence risk:** each of UX-PB.3a–3g adds `aria-live` to its own surface,
producing exactly the per-row progress chatter `EXP:322` forbids, and no story
owns removing the existing ones.

### FINDING 3 — HIGH. The UX adds an `Activity` primary destination that is parameterized live-vs-replay; no AD covers the navigation model, and AD-17 leans on `ActiveView` as though it were unchanged.

`EXP:43-49` defines five primary destinations. Two of them do not exist:

> `EXP:47` "**Activity** | Inspector for the currently executing confirmed plan | When idle, says there is no active upgrade; during replay, the live sidecar remains available with `Back to live activity`"

> `EXP:48` "**History** | One record per confirmed execution attempt | Opening an entry routes Activity into read-only replay mode; Retry entries link to their source attempt"

The tree has no Activity destination — `src/store/ui.ts:12-16`:

```
export type ActiveView =
  | { kind: "dashboard" }
  | { kind: "manager"; managerId: ManagerId }
  | { kind: "history" }
  | { kind: "settings" };
```

AD-17 cites `ActiveView` as a fixed quantity ("persists across `ActiveView`
changes", `SPINE:372`) without noting the type must gain a member, and the
member it must gain carries a parameter:

> `EXP:210` "If History replay opens during a live plan, the sidecar remains visibly live and full Activity is labeled `Viewing past activity`; `Back to live activity` returns the main workspace to the active plan."

> `EXP:302` "History replay opens during live work | Keep live-sidecar focus available; main replay receives focus at its heading and exposes `Back to live activity`."

**Divergence risk:** the main workspace and the sidecar must simultaneously bind
to *two different plan attempts* — the sidecar pinned to the active attempt, the
main region showing an arbitrary historical `planAttemptId`. Nothing in AD-16,
AD-17, or AD-18 says Activity is parameterized by `planAttemptId`. A builder who
implements one "current attempt" store slot (the obvious reading of
`SPINE:316-318` "The active-attempt lookup, cancel command, History query,
Activity replay, and diagnostic export address `planAttemptId`") will have replay
clobber the live binding and `Back to live activity` will have nothing to return
to.

### FINDING 4 — HIGH. The existing bottom ActivityDrawer region has no retirement owner; AD-16 retires only the *setting*.

AD-16 retires the preference:

```
SPINE:276-277
- **Rule:** Settings replace active `autoOpenDrawer` behavior with
  `skipUpgradePlanConfirmation`, default `false`.
```

The UX retires the same preference (`EXP:256` "Remove any `Auto-open Activity
drawer` preference; the sidecar always transforms automatically after
confirmation") — but it also removes the drawer *region* by omission: the Surface
map at `EXP:55-65` lists nine surfaces and none is a bottom drawer, and `EXP:47`
promotes Activity to a primary destination instead.

The drawer is a live, geometried region in the tree —
`src/store/ui.ts:41-43`:

```
export const DRAWER_MIN = 0.25;
export const DRAWER_MAX = 0.6;
export const DRAWER_DEFAULT = 0.4;
```

with `drawerOpen`, `drawerHeight`, and `focusedOpId` in `UiState`
(`src/store/ui.ts:53-55`) and the store's own header calling it "the
ActivityDrawer geometry" (`src/store/ui.ts:2`).

**Divergence risk:** a builder who reads AD-16 as retiring only the setting keeps
the resizable bottom drawer and adds the sidecar, leaving two concurrent
live-status surfaces. AD-17's guard against that ("rather than opening a second
surface", `SPINE:375`) is scoped to the sidecar's *own* transitions and does not
reach the drawer. No AD, and no line in the "Decision Status and Deferred Items"
table (`SPINE:491-505`), mentions the drawer.

### FINDING 5 — HIGH. `PlanIntent.kind: Explicit | AllEligible` has no scope, but the UX defines three differently-scoped bulk actions.

The normative domain minimum is a single unscoped flag:

```
SPINE:294-296
PlanIntent
  kind: Explicit | AllEligible      # durable; a removal converts AllEligible -> Explicit
```

and the rule that depends on it:

```
SPINE:336-339
- **Intent kind.** `PlanIntent` distinguishes explicitly chosen membership from
  bulk `AllEligible` membership, durably: a bulk-added item the user removes
  stays removed across a rebuild, and an explicit item is never silently absorbed
  into a later bulk action.
```

The UX has at least three bulk scopes:

> `EXP:394` "Sallvain selects Update Everything." — all Managers.

> `EXP:286` "The header Checkbox adds or removes every eligible Package matching the active filter, not merely rendered rows. Its label names the exact consequence (`Add all 8 updates`), and its mixed state is announced." — one Manager, one filter.

> `EXP:285` "Shift+Up/Down extends a contiguous membership range from the current anchor while respecting pinned, current, excluded, and unavailable eligibility." — a contiguous range.

**Failure scenario.** The user opens Homebrew, sets the filter to `Updates`, and
presses the header Checkbox. Does `kind` become `AllEligible`? If yes, the next
Rust rebuild absorbs newly-eligible packages from *every* Manager and every
filter, and `Update Everything` becomes indistinguishable from a per-Manager
bulk. If no, then `AllEligible` applies only to `Update Everything` and the
header Checkbox is a bulk `Explicit` write — at which point
`SPINE:337-338` ("a bulk-added item the user removes stays removed across a
rebuild") has no mechanism for the header-Checkbox case, because
`Explicit` removals are just removals.

`DESIGN:252` makes the scope question explicit as a prohibition ("Do not … treat
only rendered virtual rows as a bulk-selection scope"), and
`validation-report.md:40` records it as a resolved *usability* high finding
("#### Usability — Header Checkbox scope is ambiguous"). That resolution landed
in the UX; it did
not land in the domain minimum. UX-PB.1a and Stories 3.2/3.5 will resolve it
differently.

### FINDING 6 — HIGH. `EXP:315` defers the accessibility method to the spine; AD-11's method is narrower than what `EXP:330` asserts is verified.

The UX defers:

> `EXP:315` "Pack-Manager must meet the packaged-app accessibility method approved by the Architecture Spine."

then asserts a verification scope:

> `EXP:330` "Packaged acceptance verifies focus, final-row reachability, selection scope, completion announcements, and no overlap at 100%, 150%, and 200% zoom with VoiceOver and 101 Package rows."

AD-11 contains neither final-row reachability, nor selection scope, nor zoom:

```
SPINE:223-227
- **Rule:** Accessibility is product quality in the existing lanes — automated
  4.5:1 text contrast and reduced-motion checks in the Playwright/Vitest lane,
  and one manual VoiceOver focus-order and completion-announcement pass on the
  release checklist. Broader WCAG or legal compliance is not implied
  (`docs/DECISIONS.md` D33, restating the former DR-2).
```

The deferral is circular: `EXP` points at the spine for the method, the spine
covers two automated checks plus one manual pass, and three of the five things
`EXP:330` claims are verified have no lane, no owner, and no acceptance step.
The 101-row and zoom cases are the expensive ones (they need a seeded fixture and
a zoom harness), so they are exactly the ones that silently do not get built.

### FINDING 7 — HIGH. High-zoom mode requires the sidecar to stop being a side region; AD-17 as written makes both implementations non-compliant.

> `DESIGN:173` "At 150–200% zoom, or whenever the usable CSS width drops below 720px, switch to a high-zoom layout: collapse the sidebar into an accessible navigation rail or temporary panel and present the Upgrade Plan, Activity, or Results as a full-workspace/stacked surface instead of retaining a fixed sidecar."

> `EXP:366` "Below 720 usable CSS pixels—such as 150–200% zoom at the minimum window—enter high-zoom mode: collapse navigation to an accessible rail or temporary panel and present Plan/Activity/Results as a full-workspace or stacked surface with a clear Back route."

> `EXP:364-365` "At ordinary zoom and usable width of at least 720 CSS pixels, keep the 190px sidebar stable. At ordinary zoom, when the Upgrade Sidecar opens, allocate 340–380px and allow the main workspace to become one Manager-card column before hiding required information."

AD-17 has no responsive term at all, and its two absolute clauses conflict at the
breakpoint:

- "Exactly one instance exists" (`SPINE:372`) forbids rendering a separate
  full-workspace variant.
- "persists across `ActiveView` changes without losing membership or scroll
  identity" (`SPINE:372-373`) forbids the alternative — re-parenting the single
  instance into the main region, which in React remounts the subtree and drops
  scroll position and focus unless the builder knows to hoist state or use a
  portal.

**Divergence risk:** builder A conditionally renders two sidecar variants
(violates "exactly one instance"); builder B re-parents one (violates "without
losing … scroll identity" unless portal/state-hoisting is specified). AD-17 needs
to say which, and needs a term for the breakpoint. `DESIGN:252` also forbids the
lazy option ("retain fixed panes when zoom makes them overlap").

Related and also uncovered: `EXP:277` "F6 cycles the primary navigation, main
Package Grid/workspace, and Upgrade Sidecar regions without changing selection"
and `EXP:274` "Tab order follows visual reading order: sidebar → page
header/actions → filters → table/Manager cards → Upgrade Sidecar" together fix
the sidecar's DOM position *after* the main workspace and require a global region
registry. In high-zoom mode the sidecar is no longer a third region, so what F6
cycles changes. `grep -rn "F6" src --include="*.tsx" --include="*.ts"` returns
only a SPEC section reference in `src/components/manager/SelfUpdateCard.tsx:3`
and `:80`; no F6 handler exists.

---

## 3. Where an AD contradicts a stated UX behavior

### FINDING 8 — MEDIUM. AD-17's durable draft creates a relaunch state the UX never specifies, and the assumption is flagged while its consequence is not.

```
SPINE:364-369
- **Rule:** The draft is durable. It is written to Application Support under the
  same atomic-replace discipline settings use, and reconstructed at launch. A
  missing, unreadable, or incoherent draft file yields an empty draft — never a
  partial or inferred membership. A draft is never surfaced as Activity or
  History. `[ASSUMPTION]` Durable persistence is the reading taken from
  UX-PB.1b, which also permits an always-empty-on-relaunch fallback.
```

The assumption is honestly marked. The consequence is not covered anywhere in
the UX:

- `DESIGN:213` enumerates the sidecar's required states as "Draft editable,
  command revealed, confirmation off, confirming, executing, results" — there is
  no restored-from-previous-session state.
- The only launch-time reconstruction the UX defines is for *confirmed* work:
  `EXP:221` "After a crash or forced quit, reconstruct confirmed unfinished work
  as `Interrupted` on the next launch." and `EXP:432` repeats it.
- A restored draft is stale by construction, because detection re-runs at launch
  (`EXP:169-172`), and `EXP:191` requires an explanation when that happens:
  "Stale plan: replace invalidated details, explain what changed, and require a
  fresh confirmation." Nothing says whether a restored sidecar opens announcing
  staleness, opens silently, or is suppressed until the user acts.
- `EXP:180` "First eligible addition: sidecar opens with focus preserved on the
  source control." — on a restored draft there is no source control, so the
  defined focus behaviour has no referent.

Either the spine adopts the `always-empty-on-relaunch` fallback it already names
(cheap, contradicts nothing), or UX-PB.1b owes a restored-draft state with copy,
staleness handling, and a focus rule.

### FINDING 9 — MEDIUM. `EXP:155` labels the stalled-row action bare `Cancel`, which AD-16 makes a different command against a different target.

AD-16 makes the distinction load-bearing:

```
SPINE:266-269
- **Rule:** Primary cancellation targets `planAttemptId`: unstarted work becomes
  `Skipped`, running process groups use the existing escalation, and every
  terminal state stays durable. `Cancel operation` is reserved for an explicitly
  Operation-scoped diagnostic action.
```

The UX states the rule correctly twice and then breaks it once:

> `EXP:107-110` "Use `Cancel plan` when the consequence stops or skips the remaining work in the confirmed attempt. Use `Cancel operation` only for a deliberately Operation-scoped diagnostic action. Generic `Cancel` is reserved for closing a dialog or retry-scope editor without mutating running work."

> `EXP:216` "At the 120-second silence threshold, the stalled Operation presents exactly `Keep waiting`, `Copy command`, and `Cancel plan`."

> `EXP:155` "Stalled rows expose exactly `Keep waiting`, `Copy command`, and `Cancel`; an unexpected prompt never accepts input and uses the same blocked handoff."

This is a UX-internal inconsistency, but AD-16 is what makes it consequential: a
builder implementing the Activity Operation Row from `EXP:155` wires a row-level
control to the Operation-scoped cancel (`opId`), which under
`SPINE:266-267` does *not* skip unstarted attempt work — the opposite of the
behavior `EXP:216` promises ("prevents remaining unstarted attempt work from
beginning"). Fix the label at `EXP:155` to `Cancel plan`.

### FINDING 10 — MEDIUM. The Styling convention points at `theme.css`, but `theme.css` does not contain the DESIGN token set — five required token *names* are absent, including the focus ring.

Both documents agree on the location:

```
SPINE:427
| Styling | Design tokens live in `src/styles/theme.css`; the product is dark-only and adds no hardcoded hex elsewhere. Color states always carry a text or icon equivalent. |
```

> `DESIGN:107` "All product colors must be exposed through semantic Tailwind tokens in `src/styles/theme.css`. Product components consume token names; they do not hardcode hex values."

`src/styles/theme.css:5-41` defines no `focusRing`, `onAccent`, `onSuccess`,
`shell`, or `violet`. The focus-ring omission is the one that matters, because
the UX makes a *dedicated* focus token a hard accessibility requirement:

> `DESIGN:119` "| `focusRing` | `#F4F7FB` | Dedicated high-contrast keyboard-focus ring |"

> `DESIGN:202` "Keyboard focus uses a separated 2px `focusRing` outline; selection color never substitutes for focus."

> `EXP:318` "Every interactive element uses a separate `{colors.focusRing}` indicator that is at least 2px wide and visible against every surface."

(`EXP:318` also ships an unresolved template placeholder, `{colors.focusRing}`,
in the final document.)

`theme.css:22` instead makes one token do three jobs —
`--color-accent:        #4F8CFF;   /* primary actions, focus, running */` — and
the tree follows it uniformly, e.g.
`src/components/primitives/Button.tsx:38`
`"transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-accent focus-visible:ring-offset-2 focus-visible:ring-offset-bg-base"`,
with the same `focus-visible:ring-accent` at 21 further call sites across 15
further files (`grep -ro "focus-visible:ring-accent" src --include="*.tsx" --include="*.ts" | wc -l` → `22`).

**Divergence risk is a silent one.** In Tailwind 4, an unknown utility emits
nothing — a builder who writes `focus-visible:ring-focus-ring` against
`DESIGN:202` gets *no focus ring at all*, and the AD-11 automated lane checks
text contrast and reduced motion (`SPINE:224`), not focus-indicator presence. No
AD and no story in the Capability map (`SPINE:476-487`) owns reconciling
`theme.css` to the DESIGN token set.

**Explicitly not part of this finding:** the hex *values* differ throughout
(`accent` `#65A7FF` vs `#4F8CFF`, `success` `#72E6A0` vs `#3FB96B`, radii
`control 7px/card 13px` vs `--radius-control: 6px/--radius-card: 10px`, and the
whole surface ramp). Those are a value swap, cannot cause two builders to
diverge architecturally, and are correctly out of the spine's scope.

### FINDING 11 — MEDIUM. AD-18 expands what diagnostics exports; the UX calls that export privacy-preserving, and no contract anywhere carries a redaction requirement.

```
SPINE:393-395
- **Rule:** Diagnostics export carries both journals as distinct entries
  alongside `report.json`, the newest three app logs, and the newest 25
  transcripts. Existing retention bounds are unchanged.
```

The UX asserts a property of that export:

> `EXP:255` "Diagnostics includes Environment Report, Copy, Re-detect, Open Logs, and privacy-preserving export."

> `EXP:436` "**Goal:** Reconstruct a prior plan and share useful evidence without exposing unnecessary private data."

> `EXP:444` "**Climax:** the exported evidence is useful for support while respecting the redaction and retention requirements in the source contracts."

AD-5's only diagnostics rule is `SPINE:198-199` "Diagnostics must reject symlinks
both when selecting and when streaming files." The deferral target is empty:
`grep -rn -iE "redact|privacy|sanitiz" docs/*.md` returns no matches, and the
only redaction text in `_bmad-output/` outside `EXPERIENCE.md` is the archived
PRD extract recording it as a *known gap* —
`_bmad-output/archive/2026-07-24-scope-recalibration/planning/prds/prd-Pack-Manager-2026-07-22/extract-product-intent.md:602`
"**Diagnostics consent/redaction:** Contents are enumerated and inherited
environment is excluded, but there is no preview/redaction step or explicit
sharing workflow."

So AD-18 adds a new file to the export bundle while the property the UX claims
about that bundle is owned by nobody. Story 6.5 is the only live consumer
(`SPINE:486`).

### FINDING 12 — LOW. Session-scoped per-Manager view state has no home, and AD-19 could wrongly capture it.

> `EXP:74` "All Packages is the default. Remember the selected filter separately for each Manager during the current session."

The Frontend-state convention covers derived state and immutability
(`SPINE:426` "Per-manager phase is derived, never stored.") but says nothing
about session-scoped per-Manager view state, and AD-19 governs everything that
reaches a persisted file. A builder who routes the remembered filter through
settings makes it durable across launches, contradicting "during the current
session"; a builder who keeps it in component state loses it on navigation,
contradicting "Remember … separately for each Manager". One sentence in the
Frontend-state convention resolves it.

### FINDING 13 — LOW. The global reduced-motion reset is total and un-overridable; the UX distinguishes essential from nonessential motion.

`src/styles/theme.css:50-56`:

```
/* prefers-reduced-motion disables all transitions (default 150ms ease). */
@media (prefers-reduced-motion: reduce) {
  *, *::before, *::after {
    transition: none !important;
    animation: none !important;
  }
}
```

> `EXP:310` "Under `prefers-reduced-motion`, remove nonessential animation and use immediate state changes, text, and icons."

> `EXP:212` "Running progress is indeterminate unless the adapter provides a trustworthy measurable total."

Under the reset, an indeterminate CSS indicator freezes and reads as a stalled
bar — the one thing `EXP:155` ("Use indeterminate motion unless a trustworthy
percentage exists") is meant to prevent. `EXP:310` names the substitute ("text,
and icons") and `EXP:331` supports the total reset ("Reduced-motion behavior
removes continuous or sweeping animation"), so this is not a hard contradiction —
but nothing owns providing the text substitute, and `*` + `!important` means a
builder cannot opt one indicator out. Low severity because the UX arguably wants
the indicator gone; flagged because AD-11's automated reduced-motion check
(`SPINE:224`) passes trivially against this reset and will never detect the
resulting dead indicator.

### FINDING 14 — LOW. Three of the six declared sources of `DESIGN.md`/`EXPERIENCE.md` no longer exist at their declared paths.

`EXP:8-14` and `DESIGN:8-13` declare sources including
`_bmad-output/planning-artifacts/prds/prd-Pack-Manager-2026-07-22/prd.md`,
`.../addendum.md`,
`_bmad-output/planning-artifacts/implementation-readiness-report-2026-07-23.md`,
and `_bmad-output/planning-artifacts/sprint-change-proposal-2026-07-24.md`.
`ls` confirms none of those paths exists; `_bmad-output/planning-artifacts/`
now contains only `architecture`, `epics.md`, `story-triage-2026-07-24.md`, and
`ux-designs`. The PRD is at
`_bmad-output/archive/2026-07-24-scope-recalibration/planning/prds/prd-Pack-Manager-2026-07-22/`.

This matters because the UX defers substantive authority to them:

> `EXP:27` "Requirements for exact commands, process ownership, execution, persistence, and safety remain authoritative in the source PRD, addendum, Architecture Spine, and epics."

The remaining live deferral target, `epics.md`, is the one the spine already
flags as contradicting itself (`SPINE:505` "It contradicts this spine and
`docs/DECISIONS.md` D33; reconciling it was out of scope for this run"). The UX's
authority chain therefore terminates in an archived document and a document the
spine disowns. Bookkeeping, not architecture — but it undermines Finding 11's
deferral in particular.

---

## 4. Dark-only / tokens / reduced-motion claim — CONFIRMED

The spine claims:

```
SPINE:427
| Styling | Design tokens live in `src/styles/theme.css`; the product is dark-only and adds no hardcoded hex elsewhere. Color states always carry a text or icon equivalent. |
```

Each clause, verified:

| Claim | Status | Evidence |
| --- | --- | --- |
| tokens live in `src/styles/theme.css` | **CONFIRMED** | `theme.css:5` `@theme {` opens the token block; `find src -name "*.css"` returns exactly one file, `src/styles/theme.css`. |
| the product is dark-only | **CONFIRMED** | `theme.css:3` "Design tokens (SPEC §4.1). Dark-only in MVP; tokens live in this one file so"; `theme.css:43` "/* Base surface: the window is always the dark ramp. */"; `grep -rn -iE "prefers-color-scheme\|data-theme" src/` returns no matches. |
| dark-only, per the UX | **CONFIRMED** | `DESIGN:103` "The interface is dark-only for this release."; `DESIGN` frontmatter `name: Pack-Manager Aurora Control Deck` / `description: Dark-only native macOS design system…` (`DESIGN:2-3`); `EXP:27` "The product is a native, dark-only macOS desktop app built with Tauri, React, TypeScript, and Rust." |
| no hardcoded hex elsewhere | **CONFIRMED** | `grep -rlE '#[0-9a-fA-F]{6}' src --exclude=theme.css \| wc -l` → `0`. |
| global `prefers-reduced-motion` reset | **CONFIRMED** | `theme.css:51-56`, quoted in full under Finding 13. |
| reduced-motion, per the UX | **CONFIRMED** | `EXP:310` "Under `prefers-reduced-motion`, remove nonessential animation and use immediate state changes, text, and icons."; `EXP:331` "Reduced-motion behavior removes continuous or sweeping animation."; `DESIGN:237` "Use visible focus, non-color status text, and reduced-motion behavior everywhere." |
| color states carry a text/icon equivalent | **CONFIRMED** | `DESIGN:134` "Status colors always travel with a word, icon, count, meter length, or version label."; `EXP:320` "Status is never conveyed only by color, meter length, animation, or icon." |

**The claim as written is accurate.** The two caveats are Finding 10 (the token
*set* in `theme.css` is not the DESIGN token set — five names missing, focus ring
among them) and Finding 13 (the reset is total where the UX distinguishes
essential motion). Neither refutes the claim; both qualify it.

---

## 5. Explicit non-findings

Recorded so a later pass does not re-litigate them. Each is a real difference
between the documents that cannot cause two builders to diverge architecturally.

- **Palette hex values.** `DESIGN:109-132` and `theme.css:6-33` disagree on
  essentially every value. A value swap in one file, exactly as `theme.css:3-4`
  anticipates. Not architecture. (The missing token *names* are Finding 10.)
- **Typography scale, radii, spacing, elevation.** `DESIGN:147-194` specifies a
  full type ramp, six radii, a 4/8 spacing scale, and shadow/blur treatment; the
  spine says nothing. Presentational; no story disagrees on structure because of
  it.
- **Motion durations.** `EXP:306` "Use 120–180ms for hover, focus, and selection
  feedback; 180–260ms for sidecar/layout transitions; up to 400ms for significant
  state transforms" vs `theme.css:50` "(default 150ms ease)". Tuning.
- **Component required-states enumerations.** `DESIGN:200-223` lists required
  states per component. These are acceptance detail for the UX-PB stories, not
  cross-cutting invariants; the spine correctly declines to restate them.
- **Empty states.** AD-17 already covers the only architecturally significant one
  (the sidecar is hidden when the draft is empty, `SPINE:370-371`). The remaining
  `DESIGN:222` State Panel variants ("Loading, empty, offline, no Managers, no
  active upgrade, interrupted, fatal") are component-level, and `interrupted` is
  already governed by AD-5 (`SPINE:196-197` "An unfinished start is reconstructed
  as Interrupted instead") and `EXP:221`.
- **Voice and tone, Do's and Don'ts, Inspiration/Anti-patterns.** `EXP:112-134`,
  `DESIGN:225-253`, `EXP:334-356`. Copy and taste. The one anti-pattern with
  architectural force ("A permanently visible empty right drawer", `EXP:351`) is
  already in AD-17.
- **Sidecar is not a dialog.** Fully corroborated; see the table in §1. No
  finding.
- **Confirmation opt-out semantics.** `SPINE:277-279` ("A confirmation opt-out
  skips only the final modal — never draft review, the Rust rebuild, stale
  detection, or the explicit confirmation action") and `EXP:201` ("With
  confirmation disabled, the dialog does not open; the visible `Run # updates`
  action atomically admits the command-expanded plan") agree. No finding.
- **`Interaction required` classification.** `SPINE:280-282` and `EXP:104-106`,
  `EXP:217` are near-verbatim agreement. No finding.
- **App update separation.** `SPINE:283-286` and `EXP:453`, `DESIGN:251` agree.
  No finding.
- **`Keep waiting` needs a new IPC command.** Real, but AD-3 already licenses it:
  `SPINE:154-156` "The verified 20 commands and six events are a baseline, not a
  fixed count." No finding.

---

## Summary table

| # | Sev | Finding | Anchor |
| --- | --- | --- | --- |
| 1 | **critical** | AD-17's `draft non-emptiness` visibility predicate drops Results-until-`Done`; a literal build unmounts the sidecar at confirmation | `SPINE:370-371` vs `DESIGN:169`, `EXP:62`, `EXP:225` |
| 2 | high | No AD owns the single atomic status-announcement channel; five uncoordinated live regions exist today | `EXP:322-323` vs `SPINE:480`, `ToastHost.tsx:30,65` |
| 3 | high | UX adds a live/replay-parameterized `Activity` destination; no AD covers it and `ActiveView` lacks the member | `EXP:47-48,210` vs `SPINE:372`, `src/store/ui.ts:12-16` |
| 4 | high | The bottom ActivityDrawer region has no retirement owner; AD-16 retires only the setting | `SPINE:276-277`, `EXP:55-65,256`, `src/store/ui.ts:41-43` |
| 5 | high | `kind: Explicit \| AllEligible` is unscoped; UX defines three differently-scoped bulk actions | `SPINE:294-296,336-339` vs `EXP:285-286,394` |
| 6 | high | `EXP:315` defers to a spine accessibility method narrower than what `EXP:330` claims is verified | `SPINE:223-227` vs `EXP:315,330` |
| 7 | high | High-zoom mode (<720px) makes both readings of AD-17's "exactly one instance" non-compliant | `SPINE:372-373` vs `DESIGN:173`, `EXP:366` |
| 8 | medium | AD-17's durable draft creates a restored-draft state the UX never specifies | `SPINE:364-369` vs `DESIGN:213`, `EXP:191,221` |
| 9 | medium | `EXP:155` says bare `Cancel` on stalled rows; AD-16 makes that a different command and target | `SPINE:266-269` vs `EXP:107-110,155,216` |
| 10 | medium | `theme.css` lacks five DESIGN token names incl. the focus ring; unknown Tailwind utility = no focus ring, silently | `SPINE:427`, `DESIGN:119,202`, `EXP:318` vs `theme.css:22`, `Button.tsx:38` |
| 11 | medium | AD-18 adds a journal to the export; "privacy-preserving" has no redaction owner anywhere | `SPINE:393-395` vs `EXP:255,444` |
| 12 | low | Session-scoped per-Manager filter memory has no home; AD-19 could wrongly capture it | `EXP:74` vs `SPINE:426` |
| 13 | low | The reduced-motion reset is total and `!important`; kills the indeterminate indicator with no owned substitute | `theme.css:50-56` vs `EXP:212,310` |
| 14 | low | Three of six declared UX sources no longer exist at their declared paths | `EXP:8-14` vs `ls _bmad-output/planning-artifacts/` |
