# Epics Consistency Review — Pack-Manager UX Spines vs Epic UX-PB

Lens: epics-consistency. Scope: `_bmad-output/planning-artifacts/epics.md` Epic UX-PB
against its binding inputs `DESIGN.md` and `EXPERIENCE.md` (this folder), plus
`docs/DECISIONS.md` D27–D30, D33, D35.

Counts are verbatim command output, not estimates:

- `grep -cE '^### Story UX-PB\.' _bmad-output/planning-artifacts/epics.md` → **28**
  (the stated 28 is confirmed).
- `grep -cE '^### Story ' _bmad-output/planning-artifacts/epics.md` → **34**
  (28 UX-PB + 6 surviving Epic 1–6 stories).
- `wc -l` → `epics.md` 1319, `docs/DECISIONS.md` 486, `DESIGN.md` 252,
  `EXPERIENCE.md` 460.

## Overall verdict

No. Twelve of the 28 stories can be built from the spines as written; fourteen are
partial (the spine gestures at the behavior but leaves a load-bearing choice open —
most often the presentation of a persistence/verification *failure*), and two are
fully uncovered — `grep -ci legacy` returns **0** in both spines, so UX-PB.2f and
UX-PB.4e have zero binding UX input. Worse than the gaps are three literal-label
conflicts where the spines contradict D27–D30 outright (`Confirm # updates` vs
D28's `Confirm N Updates`, `Skip confirmation for future upgrade plans` vs D28's
`Disable upgrade plan command execution confirmation`), and one behavioral
contradiction where `EXPERIENCE.md` tells a dev to do exactly what UX-PB.4d/AD-24
forbid. A dev agent building UX-PB.4d or UX-PB.5a/b/c from the spine alone would
ship the wrong thing, not merely an underspecified thing.

## UX-PB story coverage

| Story | Needs from spines | Verdict | Evidence |
| --- | --- | --- | --- |
| UX-PB.1a | Row toggle stages, never executes; draft-rebuild-error presentation | **partial** | `EXPERIENCE.md:25` "Nothing executes from an individual row or Manager header."; `DESIGN.md:243` "Do not execute a Package or Manager update directly from a row or header." But `grep -ci "rebuild" DESIGN.md EXPERIENCE.md` → 0/0 — AC3's "the draft surfaces the specific error" has no spine surface. |
| UX-PB.1b | Sidecar open/persist/close; relaunch behavior | **partial** | `EXPERIENCE.md:96` "Removing the last draft item closes the Upgrade Sidecar."; `DESIGN.md:213` "Hidden when empty; transforms in place." `grep -ci "session-scoped" ` → 0/0; AC4 (draft never restored on relaunch) is spine-silent — `EXPERIENCE.md:221` covers only *confirmed* work: "After a crash or forced quit, reconstruct confirmed unfinished work as `Interrupted`". |
| UX-PB.1c | All four entry points feed one draft; per-item Remove; bulk provenance/tombstones | **partial** | `EXPERIENCE.md:181` "Every staged Package row has `Remove`; every staged Manager self-update has `Remove` in the Manager group heading." `grep -ci "tombstone\|provenance\|dedup"` → 0/0 — the "no later bulk expansion re-adds it" guarantee is uncommitted. |
| UX-PB.1d | Inert ineligible controls, exact reason strings, `aria-disabled` | **covered** | `EXPERIENCE.md:183` "Pinned: `This Package is pinned and cannot be updated. Unpin it, then refresh Pack-Manager to make it selectable.`" — matches `epics.md:608` verbatim; `EXPERIENCE.md:326` "expose `aria-disabled=\"true\"`, retain focus". |
| UX-PB.1e | Manager Header/Card standard presentation | **covered** | `DESIGN.md:232` "Show `34 managed packages · 8 package updates` in that order."; `EXPERIENCE.md:147` "Once staged, show `IN PLAN` and a separate visible `Remove` action with accessible name `Remove <Manager> update from Upgrade Plan`." |
| UX-PB.2a | Branded `planId`/`planAttemptId` types | **covered** (no UX input required) | Type-boundary work with no rendered surface. Note `grep -c planAttemptId` → 0 in both spines. |
| UX-PB.2b | Atomic admission; second confirmation fails closed | **covered** | `EXPERIENCE.md:199` "`Confirm # updates` admits the full plan atomically; partial silent admission is not allowed."; `:200` "If admission fails, nothing executes and the dialog explains why."; `:100` "Only one confirmed Upgrade Plan attempt may be active." |
| UX-PB.2c | Presentation of persistence failure / corrupted snapshot | **partial** | `grep -ci "corrupt\|unreadable\|unparse"` → 0/0. The spine explicitly defers: `EXPERIENCE.md:27` "Requirements for exact commands, process ownership, execution, persistence, and safety remain authoritative in the source PRD, addendum, Architecture Spine, and epics." |
| UX-PB.2d | Correlation by `planAttemptId` | **covered** (no UX input required) | Wire/store work; no rendered surface in the AC. |
| UX-PB.2e | `Cancel plan` mechanics; unstoppable-work honesty | **partial** | `EXPERIENCE.md:216` "`Cancel plan` requires no second confirmation, changes running work to `Cancelling`, prevents remaining unstarted attempt work from beginning, and states that rollback is not promised." AC2's "work that could not be stopped" has no state in `DESIGN.md:215`'s required-states list. |
| UX-PB.2f | Legacy Operation labeling | **uncovered** | `grep -ci legacy DESIGN.md EXPERIENCE.md` → **0 / 0**. `DESIGN.md:217` History Plan Row required states are "Success, partial, failed, cancelled, timed out, interrupted, retry-linked" — no legacy state exists. |
| UX-PB.3a | Sidecar transforms in place; focus target | **covered** | `EXPERIENCE.md:297` "Final confirmation \| Close the dialog and move focus to the programmatically focusable Upgrade Activity summary heading in the transformed sidecar." |
| UX-PB.3b | One shared state, two presentations; `View full Activity` | **covered** | `EXPERIENCE.md:209` "summarize the condition there and offer `View full Activity`; place `Keep waiting`, `Copy command`, `Cancel plan`, and expanded command evidence in full Activity rather than crowding the sidecar." |
| UX-PB.3c | Per-item states incl. `Verifying`; live-stream drop | **partial** | `EXPERIENCE.md:228` "A successful process exit remains `Verifying` until affected Manager state refreshes." — but neither required-state list contains it (`DESIGN.md:215`, `EXPERIENCE.md:155`). `grep -ci "disconnect\|reconnect\|stream"` → 0/0 for AC3. |
| UX-PB.3d | Outcome taxonomy; verification gate; terminal-write failure | **partial** | `EXPERIENCE.md:226` "Overall states: success, partial, failed, cancelled, timed out, interrupted."; `:227` "Per-item states: verified, failed, cancelled, skipped."; `DESIGN.md:216` "The single dismissal label is `Done`." AC4 (durable terminal write fails) has no spine presentation. |
| UX-PB.3e | `What happened` / `What to do next` | **covered** | `EXPERIENCE.md:156` "expands failures into `What happened`, `What to do next`, evidence, safe actions, and a secondary Retry."; `:120` "Name the object that failed: `rustup refresh failed`, not `Something went wrong`." |
| UX-PB.3f | Trusted classifier only | **covered** | `EXPERIENCE.md:217` "Only a closed Manager-specific classifier or explicit native signal may show `Interaction required`. That state includes a plain-language explanation plus `Copy command` and `Cancel plan`". |
| UX-PB.3g | Two cancel labels; cancel during verifying | **partial** | `EXPERIENCE.md:107` "Use `Cancel plan` when the consequence stops or skips the remaining work in the confirmed attempt. Use `Cancel operation` only for a deliberately Operation-scoped diagnostic action." AC3 (cancel issued inside the verifying window) is spine-silent. |
| UX-PB.4a | One immutable row per attempt; write-failure and unreadable-record presentation | **partial** | `EXPERIENCE.md:238` "Persist one immutable entry for every confirmed execution attempt."; `:242` "History summaries use verified outcome language such as `10 of 12 verified · 2 failed`". AC2/AC4 (unpersistable row; terminal record present-but-unparseable) have no spine state. |
| UX-PB.4b | Read-only replay; Retry offered *from a History entry* | **partial** | `EXPERIENCE.md:240` "Replay is read-only and clearly labeled." But the spine only ever places Retry inside Results (`EXPERIENCE.md:231`, `DESIGN.md:216` state "retry scope"); `epics.md:954`'s "one carve-out: the non-executing `Retry` affordance UX-PB.4d offers from a History entry" is a surface neither spine commits to. |
| UX-PB.4c | Live primary during replay | **covered** | `EXPERIENCE.md:210` "the sidecar remains visibly live and full Activity is labeled `Viewing past activity`; `Back to live activity` returns the main workspace to the active plan." |
| UX-PB.4d | Retry scope → separate reviewable object, not the draft | **partial — and conflicting** | `EXPERIENCE.md:231` "`Create new plan` deliberately replaces the sidecar with a new reviewable draft." vs `epics.md:990` "without ever writing to, merging with, or emptying the one persistent draft". See finding F1. |
| UX-PB.4e | Legacy History rows visibly distinct | **uncovered** | `grep -ci legacy` → **0 / 0**. `EXPERIENCE.md:157` states the opposite framing: "Represents one confirmed execution attempt, not one Package or command." |
| UX-PB.5a | Confirmation gate mechanics and button label | **partial** | Mechanics covered: `EXPERIENCE.md:196` "Pressing it opens the Confirmation Dialog. No safety checkbox appears in the base plan."; `:294` focus matrix. Label conflicts — see finding F2. |
| UX-PB.5b | Dialog-only opt-out; its literal label; migration | **partial** | `EXPERIENCE.md:154` "The skip-future checkbox exists only here and is accessibly described by its safety explanation." Label conflicts — see finding F3. Crash-interrupted preference write is spine-silent. |
| UX-PB.5c | Confirmation-off warning string and run action | **partial** | `EXPERIENCE.md:190` supplies the whole sentence but with a different placeholder than `epics.md:1096` — see finding F4. |
| UX-PB.5d | A11y + high-zoom of safety surfaces | **covered** | `EXPERIENCE.md:329` "At 150–200% zoom within the 900 × 600 minimum, the high-zoom layout collapses navigation and presents Plan/Activity/Results as a full-workspace or stacked surface."; `:318` the 2px `{colors.focusRing}` rule that D35 implements. |
| UX-PB.5e | App update separated from Package plans | **covered** | `EXPERIENCE.md:160` "Detailed state lives in Settings → Pack-Manager updates. One restrained application-level badge labeled `Pack-Manager Update Ready!` announces availability and links there."; `DESIGN.md:251` "Do not mix Pack-Manager application updates into Package Upgrade Plans, Activity, Results, or History." |

**Totals: 12 covered · 14 partial · 2 uncovered (= 28).**

## Findings

- **[critical] F1 — `EXPERIENCE.md` instructs the Retry behavior UX-PB.4d and AD-24 forbid** (`EXPERIENCE.md:231`). Quote: "`Create new plan` deliberately replaces the sidecar with a new reviewable draft." `epics.md:990` requires the opposite: "`Create new plan` composes a derived `RetryIntent` in Rust … and takes that separate reviewable object straight to preview and confirmation **without ever writing to, merging with, or emptying the one persistent draft**". A dev agent following the binding spine would author the persistent draft from Retry and break AD-24's single-author invariant. *Fix:* amend `EXPERIENCE.md:231` (and `:430`) to say `Create new plan` opens a *separate reviewable retry object* that goes straight to preview and confirmation, leaving the persistent draft untouched.

- **[high] F2 — the primary confirm button has two different literal labels** across the decision of record and the binding spine. `docs/DECISIONS.md:184` "The persistent Upgrade Plan has one primary `Confirm N Updates` action." `EXPERIENCE.md:195` "the base plan footer contains one blue `Confirm # updates` button." Counts: `grep -c 'Confirm N Updates'` → epics.md **10**, DECISIONS.md **2**, EXPERIENCE.md **0**; `grep -c 'Confirm # updates'` → EXPERIENCE.md **6**, epics.md **0**. Both are backticked literals a test will assert. *Fix:* pick one placeholder convention and normalize the spine to D28's `Confirm N Updates` (D28 is the decision; the spine predates it).

- **[high] F3 — the opt-out checkbox has two different literal labels.** `docs/DECISIONS.md:190` "Selecting `Disable upgrade plan command execution confirmation` and then confirming persists `skipUpgradePlanConfirmation: true`." `EXPERIENCE.md:251` "`Skip confirmation for future upgrade plans` is off by default and reversible in Upgrade safety." `grep -c 'Disable upgrade plan command execution confirmation' epics.md` → **3**; `grep -c 'Skip confirmation for future' epics.md` → **0**. UX-PB.5b (`epics.md:1060`) asserts the D28 string. *Fix:* replace the `EXPERIENCE.md:251` label with D28's.

- **[high] F4 — the confirmation-off warning sentence differs by one token, and a third variant exists.** `EXPERIENCE.md:190` "show `Confirmation is off. Changes will run immediately when you choose Run # updates. Change in Settings.`" vs `epics.md:1096` "`Confirmation is off. Changes will run immediately when you choose Run N Updates. Change in Settings.`" A third, shorter form sits in the voice table at `EXPERIENCE.md:133`: "`Confirmation is off · Change in Settings`". *Fix:* one canonical string; mark `:133` explicitly as an abbreviated example or delete it.

- **[high] F5 — UX-PB.2f and UX-PB.4e have zero binding UX input.** `grep -ci legacy` returns **0** in both `DESIGN.md` and `EXPERIENCE.md`. `epics.md:1012` requires legacy rows to be "explicitly labeled as legacy Operation entries, are visibly distinct from plan-attempt History rows", but `DESIGN.md:217` gives History Plan Row only "Success, partial, failed, cancelled, timed out, interrupted, retry-linked" and `EXPERIENCE.md:157` asserts the row "Represents one confirmed execution attempt, not one Package or command." A dev must invent the legacy row's visual identity, its detail surface, and its filter/search behavior. *Fix:* add a Legacy Operation entry row to `DESIGN.md`'s component table with required states, and a History-and-replay bullet in `EXPERIENCE.md` covering legacy detail vs plan replay.

- **[high] F6 — `Verifying` is normative in one prose line but absent from both required-state lists.** `EXPERIENCE.md:228` "A successful process exit remains `Verifying` until affected Manager state refreshes." `DESIGN.md:215` Activity Operation Row required states: "Queued, waiting, running indeterminate, running determinate, stalled, interaction required, cancelling, verified, failed, cancelled, skipped, timed out" — no verifying. `EXPERIENCE.md:155` repeats the same list, also without it. UX-PB.3c (`epics.md:814`) requires the state to render. *Fix:* add `verifying` to both state lists and give it a visual treatment distinct from `verified`.

- **[high] F7 — the spines' `sources:` frontmatter points at four paths that no longer exist**, and one of them is load-bearing for the persistence gaps above. `DESIGN.md:9` and `EXPERIENCE.md:9` cite `_bmad-output/planning-artifacts/prds/prd-Pack-Manager-2026-07-22/prd.md`; `EXPERIENCE.md:14` cites `_bmad-output/planning-artifacts/sprint-change-proposal-2026-07-24.md`. `ls` on all six source paths: PRD **MISSING**, addendum **MISSING**, implementation-readiness-report **MISSING**, sprint-change-proposal **MISSING**; only ARCHITECTURE-SPINE.md and epics.md exist. D33 (`docs/DECISIONS.md:353`) explains why: "Retired planning artifacts are archived under `_bmad-output/archive/2026-07-24-scope-recalibration/`, not deleted." The consequence is normative, not cosmetic: `EXPERIENCE.md:27` delegates the exact gaps behind UX-PB.2c / 3d-AC4 / 4a-AC2 to that dead document — "Requirements for exact commands, process ownership, execution, persistence, and safety remain authoritative in the source PRD, addendum, Architecture Spine, and epics." *Fix:* repoint `sources:` at the live set (`docs/DECISIONS.md`, `ARCHITECTURE-SPINE.md`, `epics.md`) and rewrite `:27` to defer to `docs/DECISIONS.md` D27–D30 and the Architecture Spine only.

- **[medium] F8 — no spine presentation exists for any durable-write failure.** `grep -ci "corrupt\|unreadable\|unparse"` → 0/0. Four stories carry an AC that requires one: UX-PB.2c (`epics.md:698` "the failure is surfaced, no partial attempt record is left behind"), UX-PB.3d (`epics.md:849` "the failure to persist is surfaced honestly"), UX-PB.4a (`epics.md:930`, `:938` "reported as unreadable evidence rather than silently reclassifying a finished attempt as unfinished"), UX-PB.4b (`epics.md:958`). `DESIGN.md:222` State Panel states are "Loading, empty, offline, no Managers, no active upgrade, interrupted, fatal" — no "evidence unreadable". *Fix:* one shared spine pattern for "the outcome is real but its record is not trustworthy", with required states added to State Panel and History Plan Row.

- **[medium] F9 — live-stream interruption has no spine treatment.** `grep -ci "disconnect\|reconnect\|stream"` → 0/0, against `epics.md:822` "the interruption to the live stream is surfaced rather than guessed". *Fix:* add a Live Activity bullet for progress-source loss and resumption.

- **[medium] F10 — cancellation edge states are unspecified.** `EXPERIENCE.md:216` covers the ordinary `Cancel plan`, but neither spine addresses `Cancel plan` during the verifying window (`epics.md:911`) nor escalation that fails to stop running work (`epics.md:745`). *Fix:* add both to the Live Activity state list.

- **[low] F11 — `autoOpenDrawer` is framed as removal in the spine, tolerance in the decision.** `EXPERIENCE.md:256` "Remove any `Auto-open Activity drawer` preference; the sidecar always transforms automatically after confirmation." `docs/DECISIONS.md:193` "the obsolete `autoOpenDrawer` setting becomes inactive legacy input." These are reconcilable (remove the control, tolerate the stored value) but `epics.md:1074` states the migration rule and the spine does not. *Fix:* one clause in `EXPERIENCE.md:256` noting the stored value is tolerated as inactive.

- **[low] F12 — the Package Grid keyboard models do not match.** `EXPERIENCE.md:284-288` specifies Page Up/Down, Home/End, Shift+Up/Down, F6, `aria-rowcount`; `grep -c` in `epics.md`: `F6` **0**, `Shift+Up` **0**, `aria-rowcount` **0**. Conversely Story 3.5 (`epics.md:1278`) exercises "toggle, shift-range, tri-state, Cmd+A, Space, Cmd-click, Clear, and Esc" — `Cmd+A`, `Cmd-click`, `Clear`, and `Esc` appear in neither spine. *Fix:* reconcile into one key map; this is Story 3.5's scope, not UX-PB's, but the spine is its binding input too.

## Spine decisions with no implementing story

Verified by `grep -c` against `epics.md` (all 34 stories, not just UX-PB). These are
spine commitments nothing in the live plan will build. Caveat: "no story" is not
"not implemented" — several exist in `src/` under different names (e.g. the spine's
Health Meter vs `src/components/manager/HealthBanner.tsx`, Status Chip vs
`StatusBadge.tsx`, State Panel vs `EmptyState.tsx`/`ErrorState.tsx`), so this list is
a planning gap, not necessarily a code gap.

- **Brief Notification** — `DESIGN.md:223`, `EXPERIENCE.md:163` ("must not repeat an announcement already emitted by the single Activity/Results status channel"). `grep -ci "brief notification\|notification" epics.md` → **0**.
- **State Panel** — `DESIGN.md:222`, `EXPERIENCE.md:162`. `grep -c "State Panel"` → **0**.
- **Command Output Disclosure** — `DESIGN.md:221`, `EXPERIENCE.md:161`. `grep -c` → **0**.
- **Status Chip / Package Filter / Sidebar Navigation / System Summary Card / Settings Section** as named components — `grep -c` each in `epics.md` → **0**.
- **Health Meter colour scale** — `DESIGN.md:134-141` ("0–49% current: `danger` … 80–99% current: interpolate through yellow-green"). The single `Health Meter` hit in `epics.md` is at `:640`, and it is a *negative*: "use text rather than an invented Health Meter value." Nothing builds the scale.
- **Package Filter per-Manager session memory** — `EXPERIENCE.md:74` "Remember the selected filter separately for each Manager during the current session." `grep -ci "remember" epics.md` → **0**.
- **Managers disclosure collapsed by default** — `EXPERIENCE.md:46`. `grep -c "collapsed by default" epics.md` → **0**.
- **History search and filters** — `EXPERIENCE.md:239` "Search and filters operate on plan time, result, Manager, Package, and relevant text." The only History-search reference in `epics.md` is the journey line `:152` and the legacy-labeling AC `:1015`; no story owns the search surface.
- **Motion contract** — `EXPERIENCE.md:306` "Use 120–180ms for hover, focus, and selection feedback; 180–260ms for sidecar/layout transitions; up to 400ms for significant state transforms." `grep -c "120-180\|180-260\|400ms" epics.md` → **0**.
- **Layout geometry** — `DESIGN.md:164-175` / `EXPERIENCE.md:364-365` (190px sidebar, 340–380px sidecar, two Manager-card columns). `grep -c "190px\|340" epics.md` → **0**.
- **Application Update Status full state set** — `DESIGN.md:220` requires "Checking, current, available, downloading, ready to restart, blocked by active work, manual install, error"; UX-PB.5e (`epics.md:1140-1150`) only asserts the badge and the separation. `grep -c "Restart to Update" epics.md` → **0**, against `EXPERIENCE.md:455` and `:456` "If Package operations are active, restart is refused with a clear explanation."
- **Typography / radii / elevation scales** — `DESIGN.md:147-194`. D35 (`docs/DECISIONS.md:403`) adopted only "the `DESIGN.md` `colors:` block into the existing `--color-*` names"; `grep -ci "aurora" epics.md` → **0** and no story adopts the type ramp, radius set, or elevation rules.
