# Review — status-tag accuracy

**Artifact:** `_bmad-output/planning-artifacts/prds/prd-Pack-Manager-2026-07-25/prd.md`
(companion `addendum.md`)
**Scope:** the `Shipping` / `Partial` / `Planned` tag on each of FR-1…FR-22, RP-1, RP-2,
NFR-1…NFR-8, checked one requirement at a time against `src/` and `src-tauri/`.
**Baseline:** `HEAD` = `5972109`. Working tree carries three uncommitted files
(`src/components/primitives/Button.tsx`, `src/components/shell/UpdateStatusItem.tsx`,
`tests/e2e/browser-style-contract.spec.ts`); where that matters it is called out.
**Date:** 2026-07-25

## Method

Every claim below is grounded in a file opened this session and quoted literally as
`path:line "exact text"`. Counts come from a named command. Anything I could not settle
from the repository is marked **UNVERIFIED** rather than guessed at.

Out of scope by instruction and deliberately not reported: the retired D33 readiness
apparatus, the D37-removed keyboard/VoiceOver criteria, commercial-scale rigor, technical
mechanism mapped in `addendum.md` §1, and the four already-known defects (FR-14's
quit-guard consequence, §7.3's health fixes, §7.1's contrast guard, FR-6's extra
immediate-execution sites).

### Baseline check on the Planned marker

`prd.md:24` claims `planAttemptId`, `plan_attempt_id`, `Verifying`, `InteractionRequired`
and `skipUpgradePlanConfirmation` "return **zero** occurrences across `src/` and
`src-tauri/src/`".

```
for t in planAttemptId plan_attempt_id Verifying InteractionRequired \
         skipUpgradePlanConfirmation skip_upgrade_plan_confirmation; do
  grep -rn "$t" src src-tauri/src | wc -l
done
→ 0 0 0 0 0 0
```

Accurate. Every `Planned — D27–D30` tag rests on a true premise.

---

## Verdict table

| ID | PRD tag | Verdict |
| --- | --- | --- |
| FR-1 | Shipping | ✅ verified |
| FR-2 | Shipping | ✅ verified |
| FR-3 | Shipping | ❌ **conditional** — see F-4 |
| FR-4 | Shipping | ❌ **wrong** — see F-1 |
| FR-5 | Shipping | ❌ **wrong** (should be Partial) — see F-5 |
| FR-6 | Planned — D27 | ✅ verified as Planned |
| FR-7 | Partial | ⚠️ **incomplete** — unnamed missing limb, see F-3 |
| FR-8 | Shipping | ✅ verified |
| FR-9 | Shipping | ✅ verified |
| FR-10 | Planned — D27 | ✅ verified as Planned |
| FR-11 | Partial | ❌ **wrong limbs named** — see F-2 and F-1 |
| FR-12 | Shipping | ✅ verified |
| FR-13 | Partial | ⚠️ **incomplete** — unnamed missing limb, see F-6 |
| FR-14 | Partial | ✅ verified apart from known item (a) |
| FR-15 | Partial | ✅ verified |
| FR-16 | Shipping | ✅ verified |
| FR-17 | Partial | ✅ verified |
| FR-18 | Shipping | ✅ verified |
| FR-19 | Shipping (current nav model) | ❌ **wrong** on one consequence at HEAD — see F-7 |
| FR-20 | Shipping | ✅ verified |
| FR-21 | Shipping | ✅ verified |
| FR-22 | Shipping | ✅ verified for the pipeline; stale product copy, see F-10 |
| RP-1 | Shipping | ✅ verified |
| RP-2 | Shipping | ❓ **UNVERIFIED** — see F-8 |
| NFR-1 | Shipping | ✅ verified |
| NFR-2 | Shipping | ✅ verified |
| NFR-3 | Shipping | ⚠️ cites two Planned surfaces — see F-9 |
| NFR-4 | Partial | ✅ verified |
| NFR-5 | Shipping | ✅ verified |
| NFR-6 | Shipping | ❌ **wrong** at HEAD — see F-7 |
| NFR-7 | Shipping | ✅ verified |
| NFR-8 | Shipping | ✅ verified |

---

## Findings

### F-1 (high) — FR-4 tagged Shipping: the D21 npm-inside-mise warning never renders in the one configuration it exists for

`prd.md:188`:

> `- The npm-inside-mise consequence — upgrading the mise-managed Node runtime resets npm and its global packages — appears permanently at the point of action (D21).`

FR-4's tag is `prd.md:178` `"**Status:** Shipping."`

The note is computed for mise-managed npm at `src-tauri/src/managers/npm.rs:149`:

> `        let note = (managed_by == ManagedBy::Mise).then(|| NPM_MISE_RESET_NOTE.to_string());`

…and then dropped on the branch that actually fires for mise-managed npm.
`src-tauri/src/managers/npm.rs:160-166`:

> ```
>         match managed_by {
>             ManagedBy::Mise => SelfUpdateRoute::routed(
>                 ManagerId::Mise,
>                 "mise",
>                 vec!["upgrade".into(), "npm".into()],
>                 "npm is managed by mise",
>             ),
> ```

`note` is passed only to the two `SelfUpdateRoute::in_band(...)` calls
(`npm.rs:154-158` and `npm.rs:173-177`). The `Routed` variant has nowhere to put it —
`src-tauri/src/ipc.rs:212-219`:

> ```
>     Routed {
>         executor: ManagerId,
>         command_preview: String,
>         /// Trusted argv excluding the executable (backend-only).
>         #[serde(skip)]
>         command_args: Vec<String>,
>         why: String,
>     },
> ```

…and the frontend only renders a note for the in-band case.
`src/components/manager/SelfUpdateCard.tsx:99`:

> `          {route.kind === "inBand" && route.note && (`

So the warning appears when npm is standalone, or when npm reports *itself* outdated
(the D5 in-band override at `npm.rs:153`), and is **absent** exactly when npm is
mise-managed and clean — which is the maintainer's own machine per the sample report at
`src-tauri/src/ipc.rs:604` `"binary_path: Some("/Users/sallvain/.local/share/mise/shims/npm".into()),"`.
The *ownership* is still explained (`SelfUpdateCard.tsx:83-88` renders `route.why`); the
*consequence* is not.

`grep -rn "NPM_MISE_RESET_NOTE\|resets npm\|global packages" src-tauri/src/ src/` returns
five hits, all in `npm.rs`, one fixture in `ipc.rs`, and one unrelated doc comment in
`registry.rs` — there is no second surface carrying it.

**Fix:** re-tag FR-4 `Partial` and name the limb: the D21 consequence renders only on the
in-band route; the mise-routed case has no `note` channel. The same sentence is repeated
at `prd.md:301` under FR-11 and needs the same treatment.

### F-2 (high) — FR-11 tagged Partial but the limb it names as shipping is the one that does not

`prd.md:293`:

> `**Status:** Partial. The Manager title area and Route explanation ship. Independent removable membership is Planned — D27.`

`prd.md:298`:

> `- Short description, executable path, installed version, and a Manager-status badge reading `NO UPDATES` or `UPDATE AVAILABLE`.`

Three of those four do not exist.

1. **Short description.** `ManagerInfo` has no such field —
   `src/lib/ipc/types.ts:103-115`:
   > ```
   > export interface ManagerInfo {
   >   id: ManagerId;
   >   displayName: string;
   >   status: ManagerStatus;
   >   binaryPath?: string;
   >   canonicalPath?: string;
   >   version?: string;
   >   managedBy: ManagedBy;
   >   evidence?: string;
   >   selfUpdate: SelfUpdateRoute;
   >   /** Absent managers only, e.g. "brew install mas". */
   >   installHint?: string;
   > }
   > ```

2. **Executable path in the identity area.** `SelfUpdateCard.tsx:66-78` renders the
   eyebrow, the display name and the `VersionDelta` only:
   > `          <div className="text-[11px] font-medium uppercase tracking-wide text-text-muted">Manager</div>`

   The path appears on the Dashboard card instead (`ManagerCard.tsx:150`
   `"          {info.binaryPath ? ` · ${info.binaryPath}` : ""}"`), not in the Manager pane's
   identity area FR-11 describes.

3. **`NO UPDATES` / `UPDATE AVAILABLE` badge.**
   `grep -rni "no updates\|update available" src/ --include='*.tsx' | grep -v test`
   returns four hits, none of them a Manager-status badge:
   `VersionDelta.tsx:34` and `:64` (a per-Package label), and `StatusBadge.tsx:37`
   > `    return <Chip tone="warning">Update available</Chip>;`
   which is the per-**Package** row pill (`StatusBadgeProps` takes `pkg: Package`,
   `StatusBadge.tsx:14-17`).

Installed version is the only one that ships (`SelfUpdateCard.tsx:69-77`).

This is the exact failure mode the audit was aimed at: a `Partial` tag that names one
missing limb and asserts the rest ships, when three further limbs are missing.

**Fix:** rewrite FR-11's status line to name the identity-area gaps alongside the D27
membership gap, or drop the unimplemented identity elements from the consequence list if
they are no longer wanted.

### F-3 (high) — FR-7 tagged Partial: `installed → latest` in the plan is a third unnamed missing limb

`prd.md:240`:

> `**Status:** Partial. Exact command preview, exclusions, and warnings ship. The persistent editable sidecar and the separate confirmation dialog are Planned — D27, D28.`

`prd.md:245`:

> `- Every staged Package and Manager update appears in the plan before execution, grouped by Manager, showing `installed → latest`.`

The plan payload carries no version data at all. `src/lib/ipc/types.ts:189-196`:

> ```
> export interface PlanGroup {
>   subject: ManagerId;
>   executor: ManagerId;
>   locks: ManagerId[];
>   commands: PlanCommand[];
>   packageIds: string[];
>   selfUpdate: boolean;
> }
> ```

`PlanCommand` is `{ argvPreview, label }` (`types.ts:184-187`) and `UpgradePlan`
(`types.ts:204-212`) adds only `planId`, `request`, `groups`, `excluded`, `notes`,
`warnings`. The backend builder agrees — `src-tauri/src/queue.rs:591-598` constructs each
group from `commands`, `package_ids`, `locks` and nothing else.

The sheet therefore renders commands, not versions.
`src/components/dialogs/UpgradePlanSheet.tsx:258`:

> `                      {cmd.argvPreview}`

The excluded list shows a bare name (`UpgradePlanSheet.tsx:275`
`"                    <span className="font-mono">{packageName(e.packageId)}</span>"`).
Nowhere in the file is `installed` or `latest` referenced.

**Fix:** add "per-package `installed → latest` in the plan" to FR-7's Planned list, or
delete the clause. As written the tag tells a downstream reader this limb ships.

### F-4 (medium) — FR-3 tagged Shipping: Homebrew's self-update route silently no-ops under a user-toggleable setting, and never enters the Upgrade Plan

`prd.md:174`:

> `- Homebrew's metadata refresh doubles as Homebrew's self-update Route.`

FR-3's tag is `prd.md:164` `"**Status:** Shipping."`

Two problems.

**(a) The route is gated on a preference.** `src-tauri/src/managers/brew.rs:86`:

> `        if settings.run_brew_update_on_refresh {`

with the adapter's own test proving the consequence at `brew.rs:380-386`:

> ```
>         let off = Settings {
>             run_brew_update_on_refresh: false,
>             ..Settings::default()
>         };
>         let plan = adapter.refresh_plan(&present(), &off);
>         assert_eq!(plan.len(), 4);
>         assert_eq!(plan[0].argv, vec!["list", "--versions"]);
> ```

`self_update_manager` for brew reuses that same plan —
`src-tauri/src/commands.rs:507-518`:

> ```
>             // brew: `brew update` IS the self-update — enqueue a refresh.
>             SelfUpdateRoute::ViaRefresh { .. } => {
> ```
> `                queue::make_refresh_submission(args.manager_id, &status, &settings, &env)`

So with `runBrewUpdateOnRefresh` off — a first-class toggle exposed at
`src/components/settings/SettingsView.tsx:120` `"                  label="Run brew update on refresh""` —
"Update Homebrew" enqueues a listing refresh that never runs `brew update`. It reports
success without self-updating.

**(b) Brew self-update is dropped from the plan without a trace.**
`src-tauri/src/queue.rs:648-649`:

> ```
>                 // brew updates via refresh; unavailable routes have no command.
>                 _ => {}
> ```

Neither a `PlanGroup` nor an `ExcludedPackage` is emitted, so FR-7's
`prd.md:245` "Every staged Package and **Manager update** appears in the plan before
execution" and `prd.md:247` "Exclusions, their reasons, warnings, and staleness notices are
visible" are both silently untrue for Homebrew.

**Fix:** qualify FR-3's consequence with the setting dependency, and record brew's absence
from the plan surface under FR-7 (either as a shipping exclusion-with-reason gap or as
intended behaviour stated explicitly).

### F-5 (medium) — FR-5 tagged Shipping: two consequences do not ship

FR-5's tag is `prd.md:192` `"**Status:** Shipping."`

**(a) No sorting exists anywhere.** `prd.md:202`:

> `- Packages with updates sort first by default, with a filter that shows all.`

`grep -rn "sort" src-tauri/src/` returns 9 hits — `diagnostics.rs:67`, `logging.rs:220`,
`commands.rs:47`, `events.rs:407`, plus four doc comments. None orders packages.
`grep -rn "\.sort(" src/components src/store src/lib` returns hits only in `*.test.tsx`
and `StatusBar.tsx:25`. The pane filters, it does not sort —
`src/components/manager/ManagerPane.tsx:103-105`:

> ```
>   const visibleMain = mainPackages.filter(
>     (p) => matchesSearch(p) && (!outdatedOnly || p.outdated),
>   );
> ```

The default-on outdated-only filter (`ManagerPane.tsx:93`
`"  const outdatedOnly = storedOutdatedOnly ?? anyOutdated;"`) masks this until the user
clicks "Show all packages" (`ManagerPane.tsx:257`), at which point outdated rows are
interleaved in snapshot order.

**(b) mise source path is captured but never rendered.** `prd.md:201`:

> `- Manager-specific detail is preserved where useful — uv executables, mise source path, Package kind, Homebrew pinned version.`

`PackageMeta.source` exists (`src/lib/ipc/types.ts:126` `"  source?: string;"`) and the mise
parser populates it (`src-tauri/src/managers/parse/mise.rs:151`
`"fn build_meta(requested: Option<String>, source: Option<String>) -> Option<PackageMeta> {"`).
But `grep -rn "meta?\.\|meta\." src/components --include='*.tsx' | grep -v test` shows the
only `pkg.meta` reads in the whole UI are `meta?.executables`
(`PackageRow.tsx:53`, `ManagerPane.tsx:98`). `source`, `requested`, `wanted` and
`dependedBy` are never displayed.

**Fix:** re-tag FR-5 `Partial` and name both limbs.

### F-6 (medium) — FR-13 tagged Partial: there is no `cancelling` state, and the tag does not name it

`prd.md:332`:

> `- Queued, running, stalled, cancelling, and terminal states are exposed with the exact command and live output visible.`

FR-13's Planned list (`prd.md:336`) names `planAttemptId` correlation, `Verifying`,
Activity-as-destination and Results — not `cancelling`.

The status enum has no such member. `src-tauri/src/ipc.rs:99-107`:

> ```
> pub enum OpStatus {
>     Queued,
>     Running,
>     Succeeded,
>     Failed,
>     Cancelled,
>     TimedOut,
>     Interrupted,
> }
> ```

Neither does the display map — `src/components/activity/opDisplay.ts:35-52` covers
`queued | running | succeeded | failed | cancelled | timedOut | interrupted` and nothing
else. `grep -rni "cancelling\|canceling" src/ src-tauri/src/` returns three hits, all
prose comments (`src/App.tsx:17`, `src-tauri/src/state.rs:375`, `src-tauri/src/queue.rs:2963`).

The window is real, not theoretical: `src-tauri/src/process/runner.rs:57`

> `pub const TERM_GRACE: Duration = Duration::from_secs(5);`

so after Cancel the row keeps reading `Running` (`opDisplay.ts:40`
`"      return { label: "Running", tone: "accent", pulse: true };"`) for up to five seconds
before flipping to `Cancelled`.

**Fix:** move `cancelling` into FR-13's Planned list, or drop it from the consequence.

### F-7 (medium) — NFR-6 tagged Shipping and FR-19's contrast consequence are false at HEAD

*Shares a root cause with known item (c) but is a distinct artifact: (c) concerns §7.1's
prose claim of a "contrast guard"; this concerns two load-bearing status tags that
downstream reconciliation reads as truth.*

`prd.md:541` `"**Status:** Shipping."` for NFR-6, whose body at `prd.md:543` includes
`"at least 4.5:1 text contrast"`. FR-19 restates it at `prd.md:423`:

> `- Text contrast meets at least 4.5:1 on its surface.`

At HEAD this is false in three places. `git grep -n "text-white" HEAD -- src/`:

> ```
> HEAD:src/components/primitives/Button.tsx:7:  primary: "bg-accent text-white hover:bg-accent-hover disabled:bg-accent/40",
> HEAD:src/components/primitives/Button.tsx:11:  danger: "bg-danger text-white hover:brightness-110 disabled:opacity-50",
> HEAD:src/components/shell/UpdateStatusItem.tsx:63:        className={`${CHIP} bg-accent text-white hover:bg-accent-hover`}
> ```

The uncommitted fix quantifies the failure in its own comment —
`src/components/primitives/Button.tsx:11-12` (working tree):

> ```
>   // --color-on-accent is the palette's dark ink for bright fills; on --color-danger
>   // it measures 8.30:1. White measured 2.30:1 here and failed the 4.5:1 floor.
> ```

The `--color-on-accent` token itself *is* committed (`src/styles/theme.css:30`
`"  --color-on-accent:     #07101D;   /* text/icons on bright blue fills */"`); only its
consumers are not. The asserting e2e test is likewise uncommitted — `git status --porcelain`
lists `tests/e2e/browser-style-contract.spec.ts` as modified, and the third test in that
file (`browser-style-contract.spec.ts:226` `"[P0] paints bright accent fills with ink that clears the 4.5:1 contrast floor"`)
is part of that diff.

Everything else in NFR-6 does hold at HEAD: non-color cues (`StatusBadge.tsx:19-40`),
reduced motion (`src/styles/theme.css:61-66`), the focus indicator (31 production sites —
`grep -rc "outline-focus-ring" src/` totals 32 across 21 files, of which one is
`managerPane.test.tsx:1`, so the PRD's "31 sites" at `prd.md:434` is **accurate**), and
900 × 600 (`src-tauri/tauri.conf.json` `"minWidth": 900, "minHeight": 600`).

**Fix:** the tag has to move with the commit. Either land the Button/UpdateStatusItem
change before the PRD is frozen, or tag NFR-6 `Partial` naming contrast as the outstanding
limb.

### F-8 (low) — RP-2 tagged Shipping is not verifiable from this repository: ⌘A is claimed on both sides of the map

`prd.md:503`:

> `Standard Edit and Window menu actions — including cut, copy, paste, and select-all in the search field and in every copyable command surface — are preserved.`

`prd.md:435` puts ⌘A on the other side of the same map:

> `2. **⌘X / ⌘C / ⌘V / ⌘A and the ⌘R / ⌘A / ⌘L map** (RP-2).`

The menu is declared (`src-tauri/src/lib.rs:100-113` re-declares Edit with
`undo/redo/cut/copy/paste/select_all`; `lib.rs:120-130` re-declares Window). But the app
also binds ⌘A globally — `src/hooks/useKeyboard.ts:160-162`:

> ```
>         case "a":
>           e.preventDefault();
>           selectAllVisible();
>           break;
> ```

Editable fields are protected (`useKeyboard.ts:142` `"      if (isEditable(e.target)) return;"`,
with `isEditable` covering `INPUT | TEXTAREA | SELECT | contentEditable` at
`useKeyboard.ts:26-31`), so RP-2's "in the search field" half is safe. A "copyable command
surface" is not an editable field — `CopyableCommand` is a `<button>` wrapping `<code>`
(`src/components/primitives/CopyableCommand.tsx:29` `"    <button"`, `:41`
`"      <code className="whitespace-pre">{command}</code>"`), so ⌘A there hits
`selectAllVisible()` and `preventDefault()`.

Whether AppKit's `performKeyEquivalent:` consumes ⌘A before the WKWebView `keydown` fires
is not decidable from this repository, so I will not assert a defect. What I can assert is
that RP-2's tag cannot be confirmed from code, and that the PRD assigns ⌘A two
incompatible meanings one page apart without noting the collision.

**Fix:** either state which ⌘A wins in a copyable-command surface, or scope RP-2's
select-all clause to text-entry fields.

### F-9 (low) — NFR-3 tagged Shipping asserts usability of two surfaces the PRD tags Planned

`prd.md:523` `"**Status:** Shipping."`, and `prd.md:525`:

> `Navigation, the plan, confirmation, Activity, Results, and recovery all remain usable at 900 × 600 and at 150–200% zoom.`

"Results" is Planned per FR-13 (`prd.md:336` `"a terminal Results summary with successes, failures, skipped work, verification outcomes, and Retry where appropriate."`)
and §7.2 (`prd.md:593`). The separate "confirmation" dialog is Planned per FR-7
(`prd.md:250` `"**Planned — D28:** `Confirm N Updates` opens a separate modal confirmation..."`).
Neither exists: `src/store/ui.ts:18-26` defines the whole dialog union as
`none | upgradePlan | stalled | quitGuard`, and `ui.ts:12-16` defines the whole navigation
union as `dashboard | manager | history | settings`.

The rest of NFR-3 is verified: progressive render, virtualization above 100 rows
(`PackageTable.tsx:15` `"const VIRTUALIZE_ABOVE = 100;"`), the flush thresholds
(`src-tauri/src/events.rs:183-187` — `BATCH_MAX_LINES = 64`, `BATCH_MAX_BYTES = 8 * 1024`,
`BATCH_MAX_DELAY = Duration::from_millis(50)`), and the 5,000-line cap
(`src/store/operations.ts:17` `"export const LOG_CAP = 5000;"`).

**Fix:** a Shipping NFR should not promise responsiveness of surfaces that do not exist.
Either scope the sentence to shipping surfaces or tag NFR-3 `Partial`.

### F-10 (low) — FR-22 tagged Shipping, but the shipping app still displays the retired notarization claim

`prd.md:483`:

> `- The application, disk image, and updater payload are signed and notarized, and the relevant bundles are stapled ... This supersedes D20 and `docs/SPEC.md`'s stale "notarized DMG is out of scope" line.`

The pipeline half is verified: `.github/workflows/release.yml:316-320` verifies the
detached updater signature against the pubkey the app embeds, `:341-342` publishes both
`darwin-aarch64` and `darwin-x86_64` keys pointing at one archive, and `:387-391` asserts
the published `latest.json` is reachable and reports the expected version.

But the PRD's §0.1 table records the stale claim as living in `docs/SPEC.md` only
(`prd.md:44`). It also lives in the shipping product —
`src/components/settings/SettingsView.tsx:272`:

> `              mas is unverified; notarization is out of scope for the MVP.`

**Fix:** add the in-app string to §0.1's "Code says" row so the reconciliation queue picks
it up. The FR-22 tag itself stays `Shipping`.

---

## Tags verified correct (evidence summary)

Recorded so the next reviewer does not repeat the work.

- **FR-1 / FR-2** — six-manager probe with Re-detect (`commands.rs:169`,
  `SettingsView.tsx:267`, `ManagerCard.tsx:141`); absence is a normal state with a copyable
  hint (`ManagerCard.tsx:74-82`); no local version math — `VersionDelta.tsx:33-35` returns
  `"update available"` when `latest == null`, and `:62-65` again for non-comparable pairs.
- **FR-6 / FR-10 correctly Planned** — the transient layer is still live:
  `src/store/packages.ts:17` `"  selection: Partial<Record<ManagerId, Set<string>>>;"`, and
  `ManagerPane.tsx:145-153` `upgradeRow` builds a single-package plan and calls
  `executePlan` immediately.
- **FR-8** — `src-tauri/src/state.rs:25` `"pub const ISSUED_PLAN_LIMIT: usize = 64;"`;
  one-use consumption and every rejection reason at `commands.rs:363-393`; no speculative
  retry at `UpgradePlanSheet.tsx:174-183`.
- **FR-9** — `src-tauri/src/queue.rs:48` `"pub const MAX_CONCURRENCY: usize = 4;"`;
  all-or-none admission via `submit_plan_batch` (`commands.rs:462-480`); "Queued behind"
  at `SelfUpdateCard.tsx:128`; D22 no-retry via `BREW_LOCK_SIGNATURE`
  (`managers/brew.rs:24`) mapped to a distinct `ErrorCode::BrewLockBusy` (`error.rs:73`).
- **FR-12** — `process/runner.rs:299-301` `.env_clear()` and
  `.stdin(Stdio::null()) // no sudo, no password entry, ever`; preview/argv agreement at
  `commands.rs:344-349`; manual-install fallback at `app_update.rs:180-187`.
- **FR-14 (apart from known item (a))** — 120 s watchdog (`queue.rs:101`,
  `queue.rs:1443-1463`), 30 min cap, SIGTERM → 5 s → SIGKILL over the process group
  (`runner.rs:57`, `:261-271`, `:304` `.process_group(0)`), the password promise at
  `StalledOperationDialog.tsx:51-52`, no confirm on cancel (`:59-61`).
- **FR-15** — transcript-blocks-spawn at `queue.rs:1499-1510`; pgid recorded as evidence
  only (`journal.rs:32-34`); D26's literal-list newline split at `runner.rs:86-97`;
  retention `APP_LOG_RETENTION_DAYS = 14`, `TRANSCRIPT_RETENTION_DAYS = 90`,
  `TRANSCRIPT_MAX_FILES = 200` (`logging.rs:26-28`), `COMPACT_KEEP = 1000` (`journal.rs:19`).
- **FR-16** — post-success refresh of subject *and* executor at `queue.rs:1268-1286`;
  `View log` gated on `opId` everywhere (`ManagerCard.tsx:166`, `ManagerPane.tsx:216`,
  `ToastHost.tsx:38`).
- **FR-17** — persist-before-publish at `commands.rs:645-649`; all eight fields present and
  defaulted exactly as `prd.md:394` states (`settings.rs:29-38`, `:43-52`); the Notes claim
  at `prd.md:398` checks out — `dev/fixtures/ipc/settings.json` carries
  `"autoCheckForUpdates": true`.
- **FR-18** — Desktop archive naming (`diagnostics.rs:101-111`), 3 logs / 25 transcripts
  (`diagnostics.rs:22-23`), symlink rejection at selection (`:63`) and at streaming (`:89`).
- **FR-20 / FR-21 / RP-1** — `AUTO_CHECK_INTERVAL = 6 * 60 * 60` (`app_update.rs:27`),
  launch + heartbeat + menu triggers (`lib.rs:146-164`, `:208-223`), trigger recorded for
  the frontend's toast decision (`app_update.rs:97`), two-layer install refusal
  (`commands.rs:772-795` backend, `UpdateStatusItem.tsx:34-38` frontend), state held in
  Rust so it survives UI recreation (`app_update.rs:63`).
- **NFR-2** — `merge_inventory_overlay` (`managers/mas.rs:81`, `managers/mise.rs:73`) and
  the recovery contract at `managers/mod.rs:84-89`.
- **NFR-7** — `src-tauri/tauri.conf.json` `bundle.macOS.minimumSystemVersion: "15.0"`.
- **NFR-8** — the two release-blocking checks at `release.yml:316-320` and `:387-391`.
