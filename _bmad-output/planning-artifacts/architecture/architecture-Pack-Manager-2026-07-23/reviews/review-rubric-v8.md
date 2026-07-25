# Rubric walk — ARCHITECTURE-SPINE.md revision 8

**Lens:** independent good-spine rubric, item by item
**Date:** 2026-07-25
**Target:** `_bmad-output/planning-artifacts/architecture/architecture-Pack-Manager-2026-07-23/ARCHITECTURE-SPINE.md`, revision 8

## Verdict

**READY WITH FIXES.**

The six new invariants are, with one exception, exactly the right shape: each names
two stories that obey every prior `AD` and still build incompatibly, states a rule
that reaches the divergence, and cites a shipping referent. AD-23, AD-24, AD-25 and
AD-26 are clean. AD-21 is nearly clean. **AD-22 is not** — its load-bearing rule
names a mechanism that contradicts the shipping admission path it claims to ratify,
and the naive implementation of it does not compile and would deadlock. That is the
one CRITICAL and it is a paragraph-level fix, not a redesign: the invariant AD-22
actually wants is already enforced by the code, under a different mechanism.

Beyond that, three HIGH findings: two dimensions the altitude owns are left silent
(new durable operation states beyond `Verifying`/`Skipped`; which design-token set
is canonical), and one epics.md divergence that AD-11's own correction created and
the correct-course Open row does not carry.

## Snapshot note — the target moved during this review

`ARCHITECTURE-SPINE.md` was edited while this lens was running (877 lines at 04:19,
903 at 04:24, 910 at 04:31), by the concurrent `*-v8` lenses. Three of my draft
findings were closed by those edits before I wrote them up and are **not** reported:

- AD-23 gained a tombstone-lifetime rule (`:697-701`), closing the "tombstones have
  no stated lifetime and AD-24's single-author rule does not reach them" gap.
- AD-26 was rewritten (`:775-807`), closing both a Rule-2/Rule-3 self-contradiction
  and a misattributed `tauri.app` quotation.
- AD-11 gained the `macos-14` retirement clock (`:284-289`).

Everything below is judged against the snapshot I froze:

```
sha256  05f5eb8b97b570295a5c4043f65f78e59fa727691875cc2b242d8d4ced4dae69
lines   910
mtime   2026-07-25 04:31
copy    /tmp/spine-rev8-rubric-snapshot.md
```

All `ARCHITECTURE-SPINE.md:N` citations resolve against that snapshot. Every finding
also quotes its text, so a later line shift does not orphan the citation.

---

## Rubric item 1 — does it fix the real divergence points, and does it miss any?

Mostly yes. The three revision-7 CRITICALs are genuinely closed, and I verified the
shipping premise behind each rather than taking the spine's word:

- **AD-21/AD-22 premise** — `src-tauri/src/commands.rs:649` `    coordinator.bump_revision();`
  inside `set_settings_core`, unconditional; and `src-tauri/src/commands.rs:372`
  `        if issued.revision != coordinator.revision() {`. The spine's claim at
  `:902` — "Verified as a shipping defect, not just a paper one" — holds.
- **AD-25 premise** — `src-tauri/src/managers/mod.rs:86-88`
  `    /// refresh plan (in \`refresh_plan\` order) — the inventory parsed from them` /
  `    /// must be merged with the recovered overlay, or every up-to-date package` /
  `    /// would vanish from the table whenever recovery fires.` The seam AD-25 cites
  at `:756-758` exists exactly as described.
- **AD-26 premise** — `src-tauri/Cargo.toml` declares no `[profile.release]`
  (`grep -n "profile" src-tauri/Cargo.toml` → no match), so `debug-assertions` is off
  in release today and `:792-797` is correct. `tauri-plugin-wdio-webdriver` and
  `@wdio/tauri-service` both exist and the embedded-server-inside-the-app claim is
  current.

Two real divergence points are **missed** — see **H1** and **H2** below.

## Rubric item 2 — is every Rule enforceable, and does it reach its Prevents?

One failure: **C1**, AD-22. Every other AD's Rules reach their stated Prevents. Two
partial cases are reported as **M1** (AD-21's Rule 1 promises more than its Rule 4
concedes) and **M4** (AD-11's new `macos-14` rule states a deadline but obliges
nothing).

## Rubric item 3 — could anything under Deferred let two units diverge?

No. All four Deferred rows (`:896`, `:897`, `:898`, `:901`) name a binding AD and a
constraint. `Crash/relaunch lifecycle controller` is the loosest — four stories each
build their own disposable-root setup — but the row states the constraint explicitly
and AD-5 binds all four. `Plan-attempt file name and serde shape` is safe because
UX-PB.2c blocks UX-PB.3, which blocks UX-PB.4; the one reader outside that chain
(Story 6.5) has its field list enumerated in `epics.md:1288`. No finding.

## Rubric item 4 — is named tech verified-current?

Yes. Every Stack row checks out against the lockfiles:

| Spine says | Verified from |
| --- | --- |
| Tauri Rust crate 2.11.5 | `src-tauri/Cargo.lock:3832-3833` |
| Tokio 1.53.1 | `src-tauri/Cargo.lock:4243-4244` |
| updater 2.10.1 / opener 2.5.4 | `src-tauri/Cargo.lock:3983-3984` / `:3961-3962` |
| React 19.2.8, TypeScript 7.0.2, Vite 8.1.5, Tailwind 4.3.3, Zustand 5.0.14, TanStack Virtual 3.14.8, Vitest 4.1.10, Playwright 1.61.1, Tauri API 2.11.1, CLI 2.11.4 | `package-lock.json` resolved versions |
| Rust edition 2021 | `src-tauri/Cargo.toml:6` `edition = "2021"` |
| Minimum supported macOS 15.0 | `src-tauri/tauri.conf.json:48` `      "minimumSystemVersion": "15.0"` |
| Application version 1.0.1 | `.release-please-manifest.json` `{".":"1.0.1"}` |
| Node in CI 24 | `.github/workflows/ci.yml:54`, `:73` |
| macos-14 pinned in both workflows | `.github/workflows/ci.yml:25`, `:67`; `.github/workflows/release.yml:62` |

AD-11's new external claim is current: the `macos-14` deprecation began 2026-07-06
and the images are fully unsupported by 2026-11-02, with workflows using the label
terminated with an error (`actions/runner-images` issue 13518). AD-26's named
components are current. No currency finding.

## Rubric item 5 — does it ratify rather than contradict the brownfield?

Every Verified Brownfield Baseline claim (`:102-134`) checks out:

- 20 registered commands (`src-tauri/src/lib.rs:232-253`), six event constants
  (`src-tauri/src/events.rs:77-82`).
- 15 fixtures (`ls dev/fixtures/ipc/*.json | wc -l` → `15`); byte-comparison and
  regeneration flag at `src-tauri/src/ipc.rs:545-546`, `:566`, `:788`; the TS half's
  set-equality at `src/lib/ipc/types.test.ts:56` `  it("covers exactly the committed fixture set", () => {`.
- Five ports and no sixth: `CommandRunner` (`process/runner.rs:26`), `EventSink`
  (`events.rs:124`), `UpdateSource` / `PendingRelease` (`app_update.rs:41`, `:48`),
  `ManagerAdapter` (`managers/mod.rs:67`).
- `ui.dialog` `{ kind: "upgradePlan" }` at `src/store/ui.ts:20`, discarded by
  `src/store/ui.ts:116` `  closeDialog: () => set({ dialog: { kind: "none" } }),`.
- Immediate row execution at `src/components/manager/ManagerPane.tsx:145` and `:152`.
- No `planAttemptId` / `Verifying` / `InteractionRequired` symbol anywhere in `src/`
  or `src-tauri/src/` (grep returns nothing).
- `autoOpenDrawer` still active at `src/components/settings/SettingsView.tsx:134-135`.
- Journal compaction to 1000 via temp + fsync + rename (`journal.rs:19`, `:179-180`,
  `:210`, `:222-223`); settings atomic replace (`settings.rs:124-127`); diagnostics
  3 logs / 25 transcripts (`diagnostics.rs:22-23`) and symlink rejection
  (`diagnostics.rs:72-75`).
- `csp: null` (`src-tauri/tauri.conf.json:25`) and exactly three permissions
  (`src-tauri/capabilities/default.json`).
- AD-11's reduced-motion correction is fully accurate — `src/styles/theme.css:51`
  `@media (prefers-reduced-motion: reduce) {`,
  `tests/e2e/browser-style-contract.spec.ts:46` `      await page.emulateMedia({ reducedMotion: "reduce" });`,
  `:110`/`:112` asserting `"0s"`, `:116-117` disclaiming contrast, and
  `.github/workflows/test.yml` triggering on `push`/`pull_request` to `main`.

One contradiction: **C1**. One under-scoped claim: **M1**.

## Rubric item 6 — does it cover its driving sources' capabilities?

`docs/SPEC.md`, `docs/DECISIONS.md` and `epics.md` are well covered. The two UX
spines are **not** — the spine cites `DESIGN.md` and `EXPERIENCE.md` only in its
`sources:` front matter (`:22-23`) and ratifies exactly two things from them: the
720px high-zoom rule (`:535-537`) and the single polite status channel (`:547-550`).
The visual-token contract, which is where those documents most directly contradict
`docs/SPEC.md` and the shipping code, is untouched. See **H2**.

I also checked one non-obvious source obligation and found it already shipping, so
correctly absent under AD-1: NFR-4's "block spawn when transcript creation fails" is
implemented at `src-tauri/src/queue.rs:1499-1511`, where `Transcript::create` failure
sends `status: OpStatus::Failed` and returns before any spawn.

## Rubric item 7 — is every dimension the altitude owns decided, deferred, or open?

I walked the operational/environmental envelope explicitly, because that is the one
most often silent:

| Dimension | Status in the spine |
| --- | --- |
| Deployment & environments | Decided — AD-11 (`:266-304`), AD-12 (`:311-321`), Stack CI rows |
| Build/release transport | Decided — AD-12; `release.yml` two blocking checks verified at `release.yml:316-318` (minisign) and `:340-342` (both `latest.json` target keys) |
| Provider strategy | Decided/deferred — GitHub Actions + Releases + Apple Developer ID (AD-12); secrets Deferred at `:901` |
| Runtime platform floor | Decided — macOS 15.0, with the `notarytool` residual Open at `:888` |
| Operations / observability | Covered — AD-5 disposable roots, AD-18 export payload and retention, AD-19 persistence tolerance; log prune windows are code-owned seed |
| Security boundary | Decided — AD-20; AD-26 extends it to automation surfaces |
| App-update safety guard | **Open**, correctly — `:906`; verified there is indeed no Rust guard at `src-tauri/src/commands.rs:770-796` |

No wholly silent dimension in the operational envelope. Two *product* dimensions are
silent: durable operation states beyond `Verifying`/`Skipped` (**H1**) and the design
token set (**H2**).

---

## Findings

### CRITICAL

#### C1 — AD-22's "one unbroken critical section" contradicts the shipping admission path it claims to ratify; implemented literally it does not compile and deadlocks

`ARCHITECTURE-SPINE.md:655-661`:

> - **Rule:** A confirming action holds `state.plan_coordinator` once, unbroken,
>   across validation, admission, and any side effect it carries. No
>   release-and-reacquire: that window is precisely what admits an interleaved
>   writer and re-opens the drift AD-21 just closed. This ratifies the shipping lock
>   discipline rather than extending it — `set_settings_core` already holds the
>   coordinator across its atomic `save_to`, so a persist inside the section is an
>   existing pattern, not a new blocking-IO-under-mutex hazard.

The ratification claim is half true. The `set_settings_core` half is real —
`src-tauri/src/commands.rs:637-649` does hold the coordinator across `save_to`. The
**validation-through-admission** half is the exact opposite of what ships.

The lock is a `std::sync::Mutex`:

- `src-tauri/src/state.rs:8` `use std::sync::{Arc, Mutex, RwLock};`
- `src-tauri/src/state.rs:212` `    pub plan_coordinator: Arc<Mutex<PlanCoordinator>>,`

The confirming path deliberately **releases and reacquires**, and says so:

- `src-tauri/src/commands.rs:351-353`
  `/// Consumes and re-validates a backend-issued plan against one canonical` /
  `/// revision, then asks the scheduler to atomically re-check that revision and` /
  `/// enqueue the complete batch. No synchronous guard crosses an await.`
- `src-tauri/src/commands.rs:358` `    let (subs, expected_revision) = {` — the guard's scope opens
- `src-tauri/src/commands.rs:460-461` `        (subs, coordinator.revision())` / `    };` — the guard drops here
- `src-tauri/src/commands.rs:462-465` `    let op_ids = state` / `        .queue` /
  `        .submit_plan_batch(subs, expected_revision)` / `        .await`
- `src-tauri/src/queue.rs:1009-1011` — admission **re-locks** in the scheduler task:
  `        let coordinator = self.deps.plan_coordinator.clone();` /
  `        let mut coordinator = coordinator.lock().expect("plan coordinator poisoned");` /
  `        if coordinator.revision() != expected_revision {`

**Failure scenario.** UX-PB.5b's builder implements AD-22 literally: acquire
`state.plan_coordinator`, validate, admit, then persist the rider — one guard, no
release. Two things break, in order:

1. Holding a `std::sync::MutexGuard` across `submit_plan_batch(...).await` makes the
   future `!Send`, so `#[tauri::command] pub async fn execute_plan` fails to compile.
   The developer's only in-place workaround is to release the guard — which AD-22
   forbids in the same sentence.
2. If they instead reach the "any side effect it carries" clause by calling the
   named existing pattern, `set_settings_core` re-locks the same non-reentrant
   `std::sync::Mutex` at `src-tauri/src/commands.rs:637-640` while the confirming
   action already holds it. That is a self-deadlock: the app hangs on confirm, with
   the plan coordinator poisoned for every other reader.

Meanwhile UX-PB.2b's builder, reading "This ratifies the shipping lock discipline
rather than extending it", leaves `execute_issued_plan` exactly as it is — releasing
the guard at `:461`. The two stories now hold incompatible readings of the same rule,
which is precisely the condition AD-22 was added to eliminate.

**What makes this CRITICAL rather than HIGH.** AD-22 binds UX-PB.5b, UX-PB.2b and
UX-PB.5c (`:651`), and UX-PB.2b is a hub — `epics.md:657` `**Blocks:** UX-PB.2c, UX-PB.2d, UX-PB.2e`.
The Decision Status table also books the closure on this rule: `:902` records
"Settings write vs. revision drift | **RESOLVED** | Closed by **AD-21** … and
**AD-22** (one unbroken critical section…)". A rule that cannot be implemented as
described cannot close a row.

**The fix is small, because the invariant is already enforced.** What AD-22 wants —
no interleaved writer between validation and admission — is achieved today by
carrying `expected_revision` from `:460` through to the scheduler's re-check at
`queue.rs:1011`, with `handle_plan_batch` holding the coordinator across the whole
admission (`queue.rs:1010-1037`). Restating Rule 1 as *"the revision validated against
is the revision admitted against; the admission re-checks it inside the coordinator
and rejects on drift — no confirming action may admit on a revision it did not
validate"* preserves the Prevents, ratifies the code truthfully, and leaves Rule 2's
ordering and Rule 3's asymmetry untouched. The rider persist then sits after
`submit_plan_batch` returns `Ok`, in its own `set_settings_core` acquisition, which
is both compilable and correct because admission has already bumped the revision
(`queue.rs:1037` `        coordinator.bump_revision();`).

---

### HIGH

#### H1 — AD-16 fixes `Verifying` and `Skipped` as durable wire states and leaves `Cancelling` and `Interaction required` undecided, though the same stories need them

`ARCHITECTURE-SPINE.md:378-381`:

> - **Rule:** `Verifying` and `Skipped` are durable wire-level operation states, not
>   presentation states derived in React. They are journaled, exported in
>   diagnostics, and replayed from History, so a derived state could not survive a
>   crash or a replay. Adding them is one atomic contract change under AD-3.

The spine correctly identifies that "durable wire state vs. React-derived" is a real,
non-obvious call. It then answers it for exactly two of the four new states its own
sources require.

Shipping wire enum — `src-tauri/src/ipc.rs:99-107`:

```
pub enum OpStatus {
    Queued,
    Running,
    Succeeded,
    Failed,
    Cancelled,
    TimedOut,
    Interrupted,
}
```

Two more states are required by bound stories and are nowhere in the spine:

- **`Cancelling`** — `epics.md:732` "running work moves to `Cancelling` and escalates
  through the existing process-group mechanics" (UX-PB.2e); `epics.md:894` "changes
  still-running Operations bound to that `planAttemptId` to `Cancelling`" (UX-PB.3g);
  `DESIGN.md:215` lists `cancelling` among the Activity Operation Row's required
  states. The spine's cancellation rule (`:363-366`) names only `Skipped` and "the
  existing escalation" and never mentions `Cancelling`.
- **`Interaction required`** — the spine's only rule is `:393-395`
  "**Rule:** `Interaction required` is emitted only from a closed Manager-specific
  classifier or an explicit typed native signal." *Emitted* points at an event; the
  sibling rule at `:378` says `Verifying`/`Skipped` are states, not events. The
  shipping precedent is genuinely ambiguous — stalls are an event, not a status
  (`src-tauri/src/events.rs:81` `pub const EVENT_OP_STALLED: &str = "op:stalled";`,
  with no `Stalled` variant in `OpStatus`).

**Failure scenario.** UX-PB.3f builds `Interaction required` as a transient event
flag, following the `op:stalled` precedent, and journals nothing. UX-PB.4b then has
to satisfy `epics.md:939` — replay must reconstruct "the attempt's Manager groups,
Package/version changes, Manager self-updates, exact commands, Operation outcomes,
errors, timings, and retained output" — and finds no durable interaction state to
replay, exactly the failure mode `:380` describes ("a derived state could not survive
a crash or a replay"). Separately, UX-PB.2e adds `Cancelling` as an eighth wire
variant without an AD-3 atomic-change obligation naming it, so the fixture set,
`src/lib/ipc/types.ts` and `dev/fixtures/ipc/*.json` can land out of step with the
Rust enum — the one thing `:175-182` exists to prevent.

The asymmetry is the finding: having decided the durability question once, the spine
made its silence on the two neighbouring cases read as deliberate.

#### H2 — No invariant decides which design-token set is canonical, and the two UX sources contradict `docs/SPEC.md`, the shipping theme, and a CI-enforced assertion

The spine's only statement is a Consistency Convention, `ARCHITECTURE-SPINE.md:822`:

> | Styling | Design tokens live in `src/styles/theme.css`; the product is dark-only and adds no hardcoded hex elsewhere. Color states always carry a text or icon equivalent. |

That fixes *where* tokens live and says nothing about *which* tokens. Three sources
the spine binds disagree on the values and on the focus mechanism:

Shipping / `docs/SPEC.md`:
- `src/styles/theme.css:7` `  --color-bg-base:       #0B0E14;   /* window background */`
- `src/styles/theme.css:21` `  --color-accent:        #4F8CFF;   /* primary actions, focus, running */`
- `docs/SPEC.md:197` "Focus: 2px `--color-accent` ring, offset against surface, on every interactive element."
- There is no `focusRing` token in `theme.css`.

`DESIGN.md` (a bound source, `:22`):
- `DESIGN.md:111` `| `background`    | `#090C13` | Deep application canvas                                               |`
- `DESIGN.md:123` `| `accent`        | `#65A7FF` | Primary actions, active navigation, and running state                 |`
- `DESIGN.md:119` `| `focusRing`     | `#F4F7FB` | Dedicated high-contrast keyboard-focus ring                           |`
- `DESIGN.md:107` "All product colors must be exposed through semantic Tailwind tokens in `src/styles/theme.css`. Product components consume token names; they do not hardcode hex values."

`EXPERIENCE.md` makes the `focusRing` token normative, `:318`:

> - Every interactive element uses a separate `{colors.focusRing}` indicator that is at least 2px wide and visible against every surface. `{colors.borderStrong}` may indicate selection but never substitutes for focus; selected and focused states remain distinguishable.

And there is a CI-enforced assertion on the shipping values —
`tests/e2e/browser-style-contract.spec.ts:66-67`:

```
        backgroundColor: "rgb(11, 14, 20)",
        color: "rgb(230, 233, 239)",
```

`rgb(11, 14, 20)` is `#0B0E14`. That spec runs on every push and PR to `main`
(`.github/workflows/test.yml`) — the same lane AD-11 at `:293-298` now relies on.

**Failure scenario.** UX-PB.1e ("standardized Manager workspace presentation",
`epics.md:611`) and UX-PB.5d ("accessibility and responsiveness of the confirmation
and safety surfaces", `epics.md:1086`) both build directly from `DESIGN.md` /
`EXPERIENCE.md`, and `epics.md:340` binds them to do so ("navigation, high zoom,
keyboard, focus, VoiceOver, Manager cards, … follow `DESIGN.md`, `EXPERIENCE.md`, and
`validation-report.md`"). Whichever lands first either (a) swaps `--color-bg-base` to
`#090C13` and introduces a `focusRing` token, breaking the CI style contract on
`main` and silently invalidating AD-11's reduced-motion regression surface, or
(b) keeps the SPEC values and ships accent-coloured focus rings, which
`EXPERIENCE.md:318` forbids and `UX-PB.5d`'s VoiceOver/focus criteria will be
reviewed against. The other story then builds against the opposite choice.

This passes the spine's own inclusion test cleanly — two units one level down choose
incompatibly, the call is non-obvious, and it is a real trade-off (adopt Aurora and
re-baseline the CI contract, or keep SPEC §4.1 and amend `EXPERIENCE.md`'s
accessibility floor). It needs a one-line ruling in the Styling convention or a
Decision Status row, not a new AD.

#### H3 — `epics.md` still schedules the reduced-motion check AD-11 now says already ships, and the divergence is absent from the Open row that exists to carry exactly these

AD-11's correction is the headline of revision 8 (`:43-45`) and it is accurate.
`ARCHITECTURE-SPINE.md:293-302`:

> **Reduced motion is covered today**: the product honors it at
> `src/styles/theme.css` (`@media (prefers-reduced-motion: reduce)`), and
> `tests/e2e/browser-style-contract.spec.ts` emulates
> `{ reducedMotion: "reduce" }` and asserts transitions and animations resolve to
> `0s`, running in CI on every push and pull request to `main` via
> `.github/workflows/test.yml`. … reduced motion is a regression surface to preserve,
> not a gap to schedule (AD-1).

`epics.md` still says the opposite, in two places:

- `epics.md:260` — "DR-2 is RESTATED by **D33**: … Neither automated check exists yet,
  so this is an obligation on whichever story adds them, not a description of current
  coverage."
- `epics.md:304` — `| DR-2 — packaged accessibility method | `RESTATED` — D33 | Existing Playwright/Vitest lane + release checklist | None. An obligation on whichever story adds the two automated checks, which do not exist yet. |`

`docs/RELEASE-CHECKLIST.md` was already corrected for this (`:91-96` now reads
"Reduced motion is covered automatically and needs no manual step — AUT-004 in
`tests/e2e/browser-style-contract.spec.ts` …" and "Contrast (4.5:1) is **not**
automated"), so `epics.md` is now the sole outlier.

The spine's `epics.md` divergence batch Open row at `:908` enumerates five items —
`(a) **UX-PB.1b**`, `(b) **UX-PB.1c**`, `(c) **UX-PB.4d**`, `(d) **Story 6.5**`,
`(e) **UX-PB.5b**` — and this is not among them.

**Failure scenario.** The next `bmad-correct-course` run, which the row explicitly
scopes ("**OPEN — owner runs this once, after revision 8**"), fixes five items and
leaves `epics.md:260`/`:304` asserting that reduced-motion automation does not exist.
A builder then schedules "add automated reduced-motion coverage" and reimplements
AUT-004 — the exact behaviour AD-1 at `:147-149` exists to stop ("Before scheduling
anything described as a test gap, verify whether the behavior is already present in
the shipping code"). Revision 8 created this divergence by correcting AD-11 without
adding the corresponding epics row, so it is the spine's to record.

---

### MEDIUM

#### M1 — AD-21 Rule 1 says the revision advances only on a *change*; the coordinator advances it on every publication and lease, and Rule 4 concedes only one of at least four such sites

`ARCHITECTURE-SPINE.md:628-633`:

> - **Rule:** The canonical revision AD-16 tests for drift advances only on a
>   change to a **plan-determining input**: the closed set of state whose change
>   can alter a preview's membership, its exclusions, or the argv Pack-Manager
>   would construct. That set is `detection`, `registry`, `queue.records()`,
>   `tool_env`, and the plan-determining subset of `settings`. A change outside it
>   is not drift and does not expire a preview.

The closed set is right — I checked every bump site and all fall inside it. The
problem is "a change to". The coordinator is a sequence counter, not a change
detector, and says so at `src-tauri/src/state.rs:46-49`:

> /// issuance, and atomic plan submission all take this mutex. The revision is
> /// deliberately monotonic for the process lifetime: even if an operation
> /// finishes before another prebuilt plan is validated, the earlier batch's
> /// enqueue permanently invalidated that second capability.

Bump sites that fire without any value change:

- `src-tauri/src/queue.rs:1772` `    coordinator.bump_revision();` in
  `publish_refresh_snapshot`, which bumps whether or not the snapshot differs — its
  own doc comment at `:1758-1759` says "every plan issued / before this refresh
  becomes stale before the operation reports finished."
- `src-tauri/src/state.rs:74` and `:82` — `begin_state_update` and
  `finish_state_update` each call `self.bump_revision();`, so a detection or refresh
  lease bumps twice regardless of outcome.
- `src-tauri/src/queue.rs:1287` `        coordinator.bump_revision();` on every
  operation finish.

AD-21's Rule 4 (`:644-647`) names one site only: "The shipping call site bumps
unconditionally for every key (`src-tauri/src/commands.rs` `set_settings_core`).
Narrowing it is product work owned by whichever of UX-PB.5b or Story 3.4 lands first".

**Failure scenario.** The user reviews a plan, presses `Refresh All`, and every
Manager returns byte-identical data. UX-PB.2b's builder, reading "advances only on a
change", expects the preview to survive; four bumps have already fired and admission
returns `plan_stale("canonical state revision changed after preview")`
(`src-tauri/src/commands.rs:373`). Story 3.4's builder, following Rule 4, narrows
`set_settings_core` only and never touches the refresh path. The two ship different
answers to "does a no-op refresh invalidate my reviewed plan?", and nothing in the
spine adjudicates. Either add "advance on write, not on value comparison" to Rule 1,
or extend Rule 4's concession to the refresh and lease sites.

#### M2 — AD-17's content precedence places the retry review above Results; both UX sources and `epics.md` place the retry scope *inside* Results

`ARCHITECTURE-SPINE.md:525-532`:

> Its
> visibility is a four-way union: a non-terminal attempt, an open retry review
> (AD-24), undismissed Results, or a non-empty draft — and that is also their
> content precedence, highest first. … Higher
> precedence hides lower content, never destroys it.

Every source says the retry scope is a disclosure *within* Results, not a surface
that replaces it:

- `EXPERIENCE.md:231` — "Retry is always user-controlled and never automatic. It
  first reveals the failed-item retry scope inside Results with `Cancel` and
  `Create new plan`; Cancel closes the scope and returns focus to the Retry action.
  `Create new plan` deliberately replaces the sidecar with a new reviewable draft."
- `epics.md:976` — "**Then** it first reveals the proposed failed-item scope inline
  with `Cancel` and `Create new plan`"
- `DESIGN.md:216` — Results Summary: "failed rows expose explanation, guidance,
  evidence, and secondary Retry."

**Failure scenario.** UX-PB.4d implements Retry per AD-17 as its own precedence level,
so invoking Retry hides the Results the user was reading the failure evidence in — and
`Cancel`, which `EXPERIENCE.md:231` says "closes the scope and returns focus to the
Retry action", has no visible Retry action to return focus to. UX-PB.3d, built from
`EXPERIENCE.md`, renders the scope inline and never enters AD-17's third precedence
state, so `epics.md:1128-1130`'s replay-during-live assertions and AD-17's precedence
rule describe different surfaces. The two stories cannot both be right about whether
the region has three content states or four.

#### M3 — AD-26's compile gate is `debug_assertions`, and this repo already bundles and uploads a `--debug` `.app`, so the automation surface would ship in a distributed non-release bundle the rules do not reach

`ARCHITECTURE-SPINE.md:786-791`:

> - **Rule:** The automation surface is excluded at **compile time**, never by a
>   runtime selector. The reference shape is the plugin registered under
>   `#[cfg(debug_assertions)]`, so release builds do not contain it at all. A
>   feature flag, environment variable, capability toggle, or any other runtime
>   route that could activate it is forbidden by AD-2 and is not an acceptable
>   substitute.

The gate correctly distinguishes release from debug. It does not distinguish
*distributed* from *not distributed*, and this repository already distributes a debug
bundle:

- `.github/workflows/ci.yml:89` `        run: npm run tauri build -- --debug --no-sign`
- `.github/workflows/ci.yml:93-94` `          name: Pack-Manager-debug-app` /
  `          path: src-tauri/target/debug/bundle/macos/Pack-Manager.app`

That is a launchable `.app` uploaded on every push to `main`, downloadable by anyone
with repository read access. With the reference shape adopted, it carries
`tauri-plugin-wdio-webdriver` and its embedded WebDriver server, which listens when
the app runs.

AD-2's rule (`:160-162`) is scoped to "Release builds", and AD-26's Prevents (`:772-774`)
is scoped to "the distributed application", so nothing in the spine forbids this —
but nothing addresses it either, and the spine went to the trouble of enumerating the
subtler `[profile.release]` hazard at `:792-797`.

**Failure scenario.** Story 6.5 adopts the reference shape. The build-smoke artifact
begins shipping a WebDriver server. A maintainer hands `Pack-Manager-debug-app` to
someone for a quick check — a normal use of a build-smoke artifact, and the reason
`docs/RELEASE-CHECKLIST.md` treats launchability as evidence — and an automation
surface runs on that machine. AD-20 would have caught this had the change been framed
as a permission widening; framed as `#[cfg(debug_assertions)]`, it passes every rule
as written.

#### M4 — AD-11's new `macos-14` retirement clock names deadline-bearing, release-blocking work with no owner, no target runner, and no Decision Status row

`ARCHITECTURE-SPINE.md:284-289`:

> **`macos-14` is now on a retirement clock** — GitHub's
> runner-images deprecation began 2026-07-06 and the images are "fully
> unsupported by November 2nd, 2026", after which workflows using the label "will
> be terminated with an error". `ci.yml` and `release.yml` both pin it, so the
> runner move is release-blocking work with a deadline, and it changes the build
> SDK underneath the open `notarytool` question rather than leaving it untouched.

The fact is correct and current, and the pin is real (`ci.yml:25`, `ci.yml:67`,
`release.yml:62`). But this is stated as a **Rule** and obliges nothing: it names no
target image, no owner, and no acceptance condition. It is a status, and the spine has
a place for statuses — the Decision Status and Deferred Items table — where it does not
appear. Compare the `notarytool` residual, which does get a row (`:888`), and the
app-update guard, which does (`:906`).

**Failure scenario.** No story owns the runner move. `epics.md` has no release-lane
story — `epics.md:438-448` routes every release FR to "Release checklist" — and
`docs/RELEASE-CHECKLIST.md` is a manual pass, not a work queue. On 2026-11-02 both
workflows begin failing with an error, and the item that AD-11 correctly identified as
release-blocking was never tracked anywhere a planner reads. It also interacts with an
existing Open row: moving to `macos-15`/`macos-26` changes the build SDK under the open
`notarytool minos 15.0` question at `:888`, which that row does not mention.

#### M5 — the Capability → Architecture Map omits AD-25 from two rows and AD-23 from one, and `epics.md` cannot backstop the omission

The ADs' own `Binds` lines:

- `ARCHITECTURE-SPINE.md:743` — `- **Binds:** Story 2.2; UX-PB.1e, UX-PB.2b, UX-PB.3d; the verification path`
- `ARCHITECTURE-SPINE.md:676` — `- **Binds:** UX-PB.1a, UX-PB.1c, UX-PB.1d, UX-PB.2a; Story 3.2`

The map:

- `:873` — `| Draft Upgrade Plan and sidecar … (UX-PB.1a–1e) | … | AD-16, AD-17, AD-23, AD-24 |`
  — no AD-25, though AD-25 binds UX-PB.1e.
- `:874` — `| Plan attempts, admission, cancellation (UX-PB.2a–2f) | … | AD-3, AD-4, AD-16, AD-18, AD-21, AD-22 |`
  — no AD-23 (binds UX-PB.2a) and no AD-25 (binds UX-PB.2b).

**Failure scenario.** UX-PB.1e's builder uses the map to find governing invariants —
it is the only table in the spine organised by story — misses AD-25, and re-derives
Last-good-Snapshot merge-versus-replace independently of Story 2.2. That is verbatim
the divergence AD-25 was added to close (`:744-745`: "two stories
re-deriving merge-versus-replace differently on the recovery path"). `epics.md` cannot
catch it: UX-PB.1e's own contract at `epics.md:614` reads
`**Dependencies:** UX-PB.1c; D27-D30; AD-16; AD-17` and names neither AD-25 nor AD-23,
and `epics.md:198-201` makes the spine the single authority precisely so stories do not
restate invariants.

---

### LOW

#### L1 — AD-3's "the attention path" has no defined referent and does not resolve to AD-18's "stall events"

`ARCHITECTURE-SPINE.md:194-196`:

> Any story adding a field to an event payload (AD-18's `planAttemptId` on
> `op:status`, `op:output`, and the attention path) owns fixture coverage of the
> shape

AD-18's corresponding rule, `:579-581`, is precise where AD-3 is not:

> - **Rule:** Operation status, output and stall events, transcript metadata, and
>   journal start/finish records carry `planAttemptId` when one exists

"The attention path" appears nowhere else in the spine and has no shipping referent.
The event surface is closed at `src-tauri/src/events.rs:77-82`, and the plausible
readings differ: `op:stalled` alone; `op:stalled` plus a new interaction-required
event (which H1 shows is itself undecided); or including `appUpdate:status`, which
`:396-398` explicitly excludes from plans. Fixture coverage under AD-3 is then
incomplete for whichever reading was not taken. Replacing the phrase with AD-18's
enumeration costs nothing.

#### L2 — three distinct new seams are each obliged, and one is called "a sixth port"

AD-4 obliges two new ports, `:209-211`:

> - **Rule:** Effects the live build queue newly introduces go through a port from
>   the start — specifically the filesystem access AD-18's attempt journal requires,
>   and the clock any verification or staleness deadline reads.

The Deferred/Open row then obliges a third and numbers it, `:899`:

> Story 6.5 must introduce an opener/reveal seam as a sixth port under AD-4

Against the five at `:205-206`, three new seams cannot all be the sixth. Cosmetic, but
the ordinal reads as an ownership or ordering claim, and the clock port in particular
has no named owner anywhere — no Decision Status row, and `epics.md`'s UX-PB.3d
contract names only AD-16.

#### L3 — AD-17 does not state whether Results dismissal is durable, so its relaunch rule and its four-way union have to be reconciled by inference

`ARCHITECTURE-SPINE.md:506-508`:

> - **Rule:** The draft is session-scoped and is never written to disk. Every
>   relaunch — after a clean quit, a crash, or a force-quit — starts with an empty
>   draft and a hidden sidecar.

`ARCHITECTURE-SPINE.md:529-530`:

> Results remain until dismissed even
> though the draft behind them is empty

"A hidden sidecar" on relaunch is only consistent with "Results remain until
dismissed" if dismissal state is session-scoped too — which is almost certainly the
intent, and which AD-17 never says. UX-PB.3d owns Results persistence
(`epics.md:838-840` requires a terminal outcome that "cannot be written" to be
surfaced honestly) and UX-PB.4a owns relaunch reconciliation
(`epics.md:923-925`). A builder could reasonably persist dismissal to satisfy the
first and restore an undismissed Results panel on relaunch, contradicting the second.
One clause — "Results dismissal is session state; a relaunched attempt reaches the
user through History (AD-18), never the region" — closes it.

---

## Checks that pass — recorded so a later revision does not re-litigate them

- **AD-6..AD-10 and AD-13..AD-15 absence.** Correct and correctly explained at `:890`.
  Not a gap.
- **AD-24's completeness.** The single-author rule now covers the case I expected to be
  missing — staging while a retry review is open — at `:728-731`: "Staging while a retry
  review is / open behaves identically — the item joins the persistent draft behind the
  / review, never the derived intent". Clean.
- **AD-16's verification rule ratifies the code.** `:370-371` — "'Affected' is the
  executor and the subject of each / mutating operation in the attempt" — matches the
  shipping auto-enqueue at `src-tauri/src/queue.rs:1276-1279`
  (`                let mut targets = vec![info.subject];` /
  `                if info.executor != info.subject {` /
  `                    targets.push(info.executor);`). The post-exit requirement is
  satisfied by the same site firing on `status == OpStatus::Succeeded`
  (`src-tauri/src/queue.rs:1269`).
- **The ineligibility/exclusion split is genuinely closed.** `:478-490` gives two closed
  lists, so "is a stale-snapshot Manager's outdated row eligible for `Update Everything`?"
  is decided (yes, with a warning) rather than left to UX-PB.1c and UX-PB.1e.
- **AD-25's snapshot honesty under failed verification.** `:759-763` combined with
  `:764-767` yields the right outcome — a successful mutation whose verification fails
  shows the pre-mutation snapshot at its own real timestamp, labelled degraded. Honest,
  not misleading. No finding.
- **Reviewer-gate tail arithmetic** at `:907` is internally consistent: the four `*-v6`
  lenses do total 5 CRITICAL / 14 HIGH / 18 MEDIUM / 7 LOW (rubric 1/4/9/3 at
  `review-rubric-v6.md:48`; divergence 3/5/4/2 at `review-divergence-v6.md:27`;
  currency 0/1/1/2 at `review-currency-v6.md:382-390`; reconcile-epics 1/4/4/0), and
  44 − 12 − 5 − 1 = 26 = 6 + 15 + 5. Both non-promoted CRITICALs are genuinely closed:
  `review-rubric-v6.md` C1 became the Open row at `:899`, and
  `review-reconcile-epics-v6.md` CRITICAL-1 is closed by `epics.md:264-271`
  ("**The `R-001`..`R-008` register is retired, and its ids must not be reimported.**").
- **NFR-4's transcript rule** is already shipping (`src-tauri/src/queue.rs:1499-1511`),
  so its absence from the spine is correct under AD-1, not a miss.

---

## Summary table

| # | Severity | Finding | Anchor |
| --- | --- | --- | --- |
| C1 | CRITICAL | AD-22's unbroken-critical-section rule contradicts the shipping admission path it claims to ratify; implemented literally it fails to compile and self-deadlocks via `set_settings_core` | `ARCHITECTURE-SPINE.md:655-661` × `src-tauri/src/commands.rs:351-353,460-465,637-640` × `src-tauri/src/queue.rs:1009-1011` |
| H1 | HIGH | `Verifying`/`Skipped` are fixed as durable wire states; `Cancelling` and `Interaction required` are left undecided though the same stories need them | `ARCHITECTURE-SPINE.md:378-381,393-395` × `src-tauri/src/ipc.rs:99-107` × `epics.md:732,872,894` |
| H2 | HIGH | No invariant decides which design-token set is canonical; `DESIGN.md`/`EXPERIENCE.md` contradict `docs/SPEC.md`, the shipping theme, and a CI-enforced assertion | `ARCHITECTURE-SPINE.md:822` × `DESIGN.md:119,123` × `EXPERIENCE.md:318` × `tests/e2e/browser-style-contract.spec.ts:66-67` |
| H3 | HIGH | `epics.md` still schedules the reduced-motion check AD-11 now says already ships, and the divergence is missing from the correct-course Open row | `ARCHITECTURE-SPINE.md:293-302,908` × `epics.md:260,304` |
| M1 | MEDIUM | AD-21 says the revision advances only on a *change*; it advances on every publication and lease, and Rule 4 concedes only one of four sites | `ARCHITECTURE-SPINE.md:628-633,644-647` × `src-tauri/src/queue.rs:1772,1287` × `src-tauri/src/state.rs:74,82` |
| M2 | MEDIUM | AD-17 puts the retry review above Results in precedence; every source puts the retry scope inside Results | `ARCHITECTURE-SPINE.md:525-532` × `EXPERIENCE.md:231` × `epics.md:976` |
| M3 | MEDIUM | AD-26's `debug_assertions` gate separates release from debug, not distributed from not-distributed; CI already uploads a `--debug` `.app` | `ARCHITECTURE-SPINE.md:786-791` × `.github/workflows/ci.yml:89,93-94` |
| M4 | MEDIUM | AD-11's `macos-14` retirement clock names deadline-bearing release-blocking work with no owner, target runner, or Decision Status row | `ARCHITECTURE-SPINE.md:284-289` × `.github/workflows/ci.yml:25,67` × `.github/workflows/release.yml:62` |
| M5 | MEDIUM | The Capability → Architecture Map omits AD-25 from two rows and AD-23 from one; `epics.md` cannot backstop it | `ARCHITECTURE-SPINE.md:743,676,873,874` × `epics.md:614` |
| L1 | LOW | AD-3's "the attention path" has no defined referent and does not resolve to AD-18's "stall events" | `ARCHITECTURE-SPINE.md:194-196,579-581` × `src-tauri/src/events.rs:77-82` |
| L2 | LOW | Three distinct new seams are obliged; one is called "a sixth port", and the clock port has no owner | `ARCHITECTURE-SPINE.md:205-211,899` |
| L3 | LOW | AD-17 never states whether Results dismissal is durable, leaving its relaunch rule and its four-way union to be reconciled by inference | `ARCHITECTURE-SPINE.md:506-508,529-530` × `epics.md:838-840,923-925` |

**Counts:** 1 CRITICAL, 3 HIGH, 5 MEDIUM, 3 LOW. 12 total.

**Verdict: READY WITH FIXES.** C1 blocks — AD-22 binds UX-PB.2b, which blocks three
further stories in the primary build queue — but the fix is a restatement of one rule
in terms of the revision-equality mechanism the code already enforces, not a redesign.
H1 and H2 should land in the same pass, because both are silences that the level below
will otherwise resolve twice, incompatibly. H3, M4 and M5 are bookkeeping against
tables the spine already owns.

## Method note

Every claim above was checked against a file read in this session; counts come from
`wc -l`, `grep -c`, or `ls | wc -l` rather than estimation. External facts (the
`macos-14` deprecation window, Tauri's macOS WebDriver situation,
`tauri-plugin-wdio-webdriver`) were fetched during this review rather than recalled.
The two external fetches went through a summarising intermediary, so their wording is
reported as corroboration of substance, not as byte-exact page text — that limitation
affects only the "verified-current" verdict in rubric item 4, and both claims were
independently corroborated. `reviews/review-*-v6.md` were read for tally verification
only; revision 8 was judged on its own text.
