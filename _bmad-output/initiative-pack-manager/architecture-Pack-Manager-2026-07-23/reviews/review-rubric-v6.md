# Review — `ARCHITECTURE-SPINE.md` revision 6 — rubric-walker lens

**Reviewed:** `_bmad-output/planning-artifacts/architecture/architecture-Pack-Manager-2026-07-23/ARCHITECTURE-SPINE.md`
(revision 6, `status: final`, 648 lines, `updated: "2026-07-25"`).
**Level below:** Epic UX-PB's 28 stories (`UX-PB.1a`–`UX-PB.5e`) plus surviving Stories 2.2, 3.1, 3.2, 3.4, 3.5, 6.5.
**Date:** 2026-07-25. **Lens:** good-spine checklist walk (divergence coverage, rule enforceability, deferrals, tech currency, brownfield ratification, source coverage, silent dimensions).

**Method.** The spine was read in full. Every factual claim reported below was
verified this session against the file it describes and is quoted verbatim with
`path:line`. Counts come from executed commands, not estimates:
`ls dev/fixtures/ipc/*.json | wc -l` → `15`; `grep -c 'commands::' src-tauri/src/lib.rs` → `21`
(one is the comment at `src-tauri/src/lib.rs:44`, leaving 20 inside the
`invoke_handler` block at `src-tauri/src/lib.rs:232-253`);
`grep -rn 'planAttemptId\|plan_attempt_id\|InteractionRequired\|skipUpgradePlanConfirmation' src/ src-tauri/src/ | wc -l` → `0`.

---

## Verdict

**READY WITH FIXES.**

Revision 6 is a materially stronger document than the revision the earlier
`review-rubric.md` attacked: AD-16 now carries the headline `Rule` ("No entry
point executes"), `Verifying`/`Skipped` are stated as durable wire states,
AD-17's sidecar visibility is a three-way union, AD-4 names the five ports that
actually exist, AD-20 closes the previously-silent webview dimension, and
`DRIFT-NOTE.md` now exists. The paradigm, the domain minimum under AD-16, and
the persistence rules under AD-18/AD-19 are the right altitude and are
enforceable as written.

What blocks a clean READY is not the invariants that are present — it is three
classes of defect in what surrounds them:

1. Two `Deferred` rows assert "no live consumer" facts that the bound stories
   contradict (findings 1 and 4). One of them makes Story 6.5 — one of only six
   surviving Epic 1–6 stories — unbuildable from the spine.
2. One whole dimension the initiative owns is silent while three bound stories
   depend on it: per-Manager failure isolation and Last-good Snapshot retention
   (finding 2), which AD-16 references as though it were defined somewhere in
   this document.
3. The operational envelope is thin rather than absent: runtime logging,
   transcript-creation failure, network egress/telemetry, and the app-update
   safety guard are each either unowned or unstated (findings 5, 9, 10).

None of these require re-architecting. All are additions or corrections to
existing ADs and to the Decision Status table.

**Finding count:** 1 CRITICAL, 4 HIGH, 9 MEDIUM, 3 LOW (17 total).

---

## CRITICAL

### C1. The "porting is Deferred with no live consumer" claim is falsified by Story 6.5's acceptance criteria

**Rubric item:** *Nothing under Deferred could let two units diverge* / *it fixes
the real divergence points for the level below*.

The spine defers porting the direct OS calls and grounds the deferral on a
factual claim about the level below:

- `ARCHITECTURE-SPINE.md:645` — "| Porting opener, reveal, restart, current-executable, writability, and remaining path/time call sites | **Deferred** | Direct calls today. **No live story needs them controllable**; AD-4 binds whoever ports them. |"
- `ARCHITECTURE-SPINE.md:186-190` (AD-4) — "Opener, reveal, restart, current-executable, bundle-parent writability, and the remaining path/time call sites are direct calls today; that is recorded brownfield state, not a violation, and **porting them is Deferred with no live consumer**."

A live story needs exactly that:

- `_bmad-output/planning-artifacts/epics.md:1247-1249` (Story 6.5) — "**Given** Export diagnostics and Open Logs actions / **When** native command/**opener success and failure are controlled** / **Then** the UI exposes actionable outcomes".

"Open Logs" is `commands::reveal_logs_dir`, and both reveal paths are the
un-ported direct calls the row names:

- `src-tauri/src/commands.rs:672` — "    tauri_plugin_opener::reveal_item_in_dir(PathBuf::from(&record.log_path)).map_err(|e| {"
- `src-tauri/src/commands.rs:681` — "    tauri_plugin_opener::open_path(crate::logging::logs_dir(), None::<&str>).map_err(|e| {"

You cannot control opener failure without a seam. So Story 6.5 must either
(a) introduce an opener port — which the spine says is deferred and unowned, or
(b) drive a real Finder/opener failure in a native harness — which is the other
deferred item (finding 4), or (c) quietly weaken its own AC. Three builders will
pick three different answers, and (c) is the cheapest.

This is CRITICAL rather than HIGH because the deferral is justified by a stated
fact about the bound stories, and that fact is false. A wrong `Prevents` is a
weak rule; a wrong premise under a `Deferred` row silently removes the decision
from anyone's queue.

**Fix.** Change the row to name Story 6.5 as the live consumer of an opener/reveal
seam, and either decide the seam here (a sixth port alongside the five in AD-4)
or move it to an Open question with Story 6.5 as its owner. AD-4's sentence at
:190 must lose "with no live consumer".

---

## HIGH

### H1. The native Tauri E2E harness is Deferred with a named live consumer, and the spine forbids the substitutes

**Rubric item:** *Nothing under Deferred could let two units diverge.*

- `ARCHITECTURE-SPINE.md:641` — "| Native Tauri E2E harness and runner | **Deferred** | Story 6.5 is the only live consumer (\"Real native Tauri E2E plus artifact inspection\"). Any choice must satisfy AD-2 and AD-3. |"
- `_bmad-output/planning-artifacts/epics.md:1234` (Story 6.5 Story Contract) — "- Required test level: Real native Tauri E2E plus artifact inspection"
- `ARCHITECTURE-SPINE.md:170-172` (AD-3) — "proving delivery itself waits on the native harness Deferred below, whose only live consumer is Story 6.5. **No story may claim delivery coverage from a fixture or from the browser double.**"

The spine is admirably honest here — it names the consumer and closes the two
escape hatches. But the net effect is that a story in the live backlog cannot be
completed as specified until an undecided, unowned architectural choice is made,
and the spine assigns no owner, no decision trigger, and no shape (in-process
`tauri::test`? a driven `.app` bundle? WebDriver?). The `Deferred` row's own
constraint — "Any choice must satisfy AD-2 and AD-3" — is a filter, not a
decision.

Contrast the two rows that are correctly deferred: "Controlled child-helper
language" (`:642`) and "Crash/relaunch lifecycle controller" (`:643`) both
assert no live consumer. This one asserts the opposite and still sits in the
same table, which reads as an oversight rather than a decision.

**Fix.** Promote to an Open question with Story 6.5 as owner, or fix the harness
shape here. If it stays deferred, Story 6.5's `Required test level` must be
renegotiated in `epics.md` in the same change — otherwise the level below
carries an obligation the level above declined to enable.

### H2. Per-Manager failure isolation and Last-good Snapshot retention is a silent dimension — and AD-16 references it as if it were stated

**Rubric item:** *Every dimension the altitude owns is decided, deferred, or an
open question* / *it covers the capabilities of its driving sources.*

AD-16 relies on a rule set that this spine never states:

- `ARCHITECTURE-SPINE.md:323-325` — "The attempt explicitly enters `Verifying`, and Results distinguish mutation failure from verification failure **while preserving the Last-good Snapshot rules**."

Searching the spine for the referent returns that one line and nothing else
(`grep -n -i 'last-good\|isolat\|snapshot' ARCHITECTURE-SPINE.md`: `:325` for
"Last-good Snapshot", `:86` for "isolated process groups" — an unrelated use —
and `:379` for `reviewedCommandSnapshot`). No AD states that a failing Manager
must not blank its peers or destroy its prior snapshot.

That invariant is load-bearing in three driving sources:

- `docs/SPEC.md:17` — "5. **One failing manager never blanks the others.** Per-manager refresh isolation: independent operations, timeouts, error cards; prior snapshots are retained on failure." (under `docs/SPEC.md:11` "### Load-bearing invariants (violations are bugs)")
- `_bmad-output/planning-artifacts/ux-designs/ux-Pack-Manager-2026-07-23/EXPERIENCE.md:173` — "On refresh failure, retain the last-good snapshot, show its timestamp, set System health to `Warning`, and provide Retry refresh plus the specific known failure."
- `_bmad-output/planning-artifacts/epics.md:105` — "NFR-2: Isolate and recover from detection, refresh, parse, network, update, crash, cancellation, timeout, and persistence failures without blanking another Manager or destroying a Last-good Snapshot."

And it is directly exercised by three bound stories:

- `epics.md:1118-1121` (Story 2.2) — "**Then** the correct Manager-specific terminal state and actionable detail appear / **And** peers continue independently without real network access or wall-clock sleeps."
- `epics.md:593-595` (UX-PB.1e) — "**Then** they retain the last-good snapshot with its timestamp, state the exact failure summary with `Retry refresh`, and use text rather than an invented Health Meter value."
- AD-16's own verification rule at `:323-325`, above.

The Capability map routes Story 2.2 to AD-4 alone
(`ARCHITECTURE-SPINE.md:624` — "| Detection, refresh phases, timeouts (Story 2.2) | Manager adapters behind runtime ports | AD-4 |"),
and AD-4 contains no failure-isolation rule — its rules cover ports, the safety
floor, lock order, the outdated verdict, routing, output fidelity, and
scheduling.

The concrete divergence: a verification refresh that fails (UX-PB.3d's
"verification-refresh failure/timeout" path, `epics.md:797-799`) must not
replace the Manager's snapshot with nothing, and the shipping merge rule —
"`parse_recovery` must MERGE the recovered overlay into the inventory already
parsed from the successful refresh outputs" (`_bmad-output/project-context.md:121`)
— is the kind of rule two stories will re-derive differently.

**Fix.** Either add a rule to AD-4 ("A Manager failure is contained to that
Manager: peers keep running, the prior snapshot is retained and labeled stale,
and no refresh path may replace a good snapshot with an empty or overlay-only
one"), or state the Last-good Snapshot rules where AD-16 :325 points. A dangling
reference in a `status: final` spine is worse than silence, because it reads as
though the rule exists.

### H3. "Crash/relaunch lifecycle controller — no live story requires one" is contradicted by four bound stories

**Rubric item:** *Nothing under Deferred could let two units diverge.* (Carried
over unresolved from `review-rubric.md` §9.)

- `ARCHITECTURE-SPINE.md:643` — "| Crash/relaunch lifecycle controller | **Deferred** | **No live story requires one.** AD-5 binds whoever builds it. |"

Four bound stories have crash-and-relaunch acceptance criteria:

- `epics.md:521-523` (UX-PB.1b) — "**Given** an in-progress draft when the app crashes or is force-quit / **When** Pack-Manager relaunches / **Then** the draft's canonical membership is reconstructed into the sidecar, or — if it cannot be recovered — ..."
- `epics.md:655-657` (UX-PB.2c) — "**Given** a `planAttemptId` was minted but its durable record was lost to a crash or forced quit mid-admission / **When** Pack-Manager relaunches / **Then** it reconstructs the attempt only from durable plan-admission metadata that actually persisted..."
- `epics.md:886-888` (UX-PB.4a) — "**Given** a confirmed attempt was admitted but the app crashed or relaunched before the attempt reached a terminal row / **When** History reconciles on the next launch ..."
- `epics.md:1023-1025` (UX-PB.5b) — "**Given** an interrupted atomic write of the confirmation preference across a crash and relaunch / **When** the app relaunches / **Then** the setting reconstructs to exactly one coherent value, old or new and never partial..."

AD-17 removed one of these from the queue by deciding fail-to-empty for the
draft (`ARCHITECTURE-SPINE.md:447-452`), which is real progress. The other three
remain, and each needs the same thing: a way to enter "the process died here,
now relaunch" deterministically. Without one rule about where relaunch
reconstruction happens and how it is exercised, UX-PB.2c, UX-PB.4a, and
UX-PB.5b will each invent their own — three different crash simulations, three
different reconstruction entry points, and no single answer to "who reconciles
in-flight attempts at launch".

**Fix.** Correct the row's premise and name the three consumers, or state a rule
(likely under AD-5 or AD-19) fixing a single launch-time reconciliation pass
that all three stories extend.

### H4. The one safety rule on the application-update path — "blocked while Package work is active" — has no stated enforcement point, and the shipping guard is frontend-only

**Rubric item:** *Every AD's Rule is enforceable and actually prevents its stated
divergence* / *it ratifies rather than contradicts the brownfield codebase.*

AD-16's app-update rule enumerates surfaces and one shared code path but never
mentions the active-work block:

- `ARCHITECTURE-SPINE.md:349-355` — "The application's own update is not a Package plan. It never enters a `PlanIntent`, draft, confirmed attempt, Results, or plan-attempt History, and it holds no manager lock. Its surfaces are its own StatusBar badge, the Settings Updates card, and the macOS app menu's `Check for Updates…` item. The menu handler and the IPC command share one code path behind the `UpdateSource`/`PendingRelease` seam — a second entry point never grows its own state transitions".

The sources require the block, and a bound story must render it:

- `EXPERIENCE.md:160` — "Background check/download may occur, but installation requires explicit Restart to Update, **is blocked while Package work is active**, and falls back to manual-install guidance without privilege escalation."
- `epics.md:93` (FR-21) — "refuse install/relaunch while a Package Operation is queued or running"
- `epics.md:1087-1089` (UX-PB.5e) — "**Given** active or historical Package work / **When** application-update state changes (checking, available, downloading, ready to restart, **blocked by active work**, or error)"

The shipping code has the hole:

- `src-tauri/src/commands.rs:770-774` — "pub async fn install_app_update( / app: tauri::AppHandle, / state: State<'_, AppState>, / ) -> Result<(), IpcError> { / state.app_update.install().map_err(|detail| {" — no queue check precedes the install.
- `_bmad-output/project-context.md:127` — "**Known gap:** refusing an update while a package operation is running is enforced by frontend convention only — the Rust `install_app_update` command has no guard."

AD-16's own rule says the menu handler and the IPC command "share one code
path" specifically so "a second entry point never grows its own state
transitions" — which is exactly the divergence that a frontend-only guard
already permits today (`src/components/shell/UpdateStatusItem.tsx:40` and
`src/components/dialogs/QuitGuardDialog.tsx:44` both call `installAppUpdate()`).
The spine's `Prevents` is real; its `Rule` does not reach the case.

**Fix.** Add a rule to AD-16: the active-work refusal is enforced in Rust at the
`install_app_update`/menu code path (fail-closed with a typed result), and the
frontend state is presentation of that verdict, never the guard. Then say
whether closing the shipping gap is UX-PB.5e's work or a separate item —
`epics.md:405` currently routes FR-21 to "Release checklist", so no live story
owns it.

---

## MEDIUM

### M1. The `epics.md` retired register is still recorded as `Open`, and the description is now false

`ARCHITECTURE-SPINE.md:648` — "| `epics.md` retired register | **Open** | `_bmad-output/planning-artifacts/epics.md` **still carries TIR-1..TIR-8, RE-1..RE-11, the 72-criterion controls, and a set-equality requirement against `contracts/tauri-boundary/v1.json`.** It contradicts this spine and `docs/DECISIONS.md` D33; reconciling it was out of scope for this run. See `DRIFT-NOTE.md`. |"

That was true of the committed file (`git show HEAD:…/epics.md` still has
"TIR-1: Begin every provisionally test-only gap…" at its line 123 and the
`contracts/tauri-boundary/v1.json` set-equality clause at its line 150). It is
false of the working-tree file the stories now live in — modified at
`2026-07-25 03:33:50`, after the spine's `02:47`:

- `epics.md:157-158` — "TIR-1 through TIR-8 are retired by `docs/DECISIONS.md` **D33**, together with the evidence lanes they specified"
- `epics.md:177-178` — "RE-1 through RE-11 are retired by `docs/DECISIONS.md` **D33**, together with the Candidate Identity Manifest…"
- `epics.md:123-124` — "The 72-criterion P0 readiness gate this section carried is retired by `docs/DECISIONS.md` **D33**."
- `epics.md:210` — the collision table now records the spine's position verbatim: "\"There is no separate versioned boundary-catalog file and none is to be created.\""

A builder reading the spine's last row will conclude the binding story document
contradicts the architecture, when the two now agree.

**Fix.** Close the row (`RESOLVED — reconciled 2026-07-25`, pointing at
`_bmad-output/planning-artifacts/sprint-change-proposal-2026-07-25.md`), and
note that the reconciliation is uncommitted so it survives a checkout.

### M2. The Stack table and Brownfield baseline report an application version the tree no longer carries

**Rubric item:** *Named tech is verified-current.*

- `ARCHITECTURE-SPINE.md:107-108` — "Application version is 1.0.0; minimum supported macOS is 15.0 at `bundle.macOS.minimumSystemVersion`."
- `ARCHITECTURE-SPINE.md:573` — "Verified against `package-lock.json` and `src-tauri/Cargo.lock` on 2026-07-25."
- `ARCHITECTURE-SPINE.md:578` — "| Application | 1.0.0 |"

The tree says otherwise, in all three release-please-owned files:

- `package.json:4` — "  \"version\": \"1.0.1\","
- `src-tauri/tauri.conf.json:4` — "  \"version\": \"1.0.1\","
- `src-tauri/Cargo.toml:3` — "version = \"1.0.1\""
- `git log -1 --format='%H %ad %s' --date=iso 8a4cf6a` → "8a4cf6a8ff3d2688f912c68438166ec838c6bb17 2026-07-25 06:46:52 +0000 chore(main): release 1.0.1 (#34)"

Every other row I checked is correct: React/React DOM 19.2.8, TypeScript 7.0.2,
Vite 8.1.5, Tailwind 4.3.3, Zustand 5.0.14, TanStack React Virtual 3.14.8,
Vitest 4.1.10, Playwright 1.61.1, `@tauri-apps/api` 2.11.1, `@tauri-apps/cli`
2.11.4 (all from `package-lock.json`); tauri 2.11.5, tauri-plugin-opener 2.5.4,
tauri-plugin-updater 2.10.1, tokio 1.53.1 (`src-tauri/Cargo.lock`); edition 2021
(`src-tauri/Cargo.toml:6`); `macos-14` at `.github/workflows/ci.yml:25`, `:67`
and `.github/workflows/release.yml:62`, `ubuntu-latest` elsewhere;
`node-version: 24` at `ci.yml:54`, `:73` and `release.yml:79`;
`googleapis/release-please-action@v5` at `release-please.yml:63`, `:174`;
`"minimumSystemVersion": "15.0"` at `src-tauri/tauri.conf.json:48`.

So this is one stale cell, not a pattern — but it is the cell a `status: final`
document uses to claim same-day verification.

**Fix.** Set both to 1.0.1, or drop the application version from the Stack table
entirely: the spine already says "A brownfield seed, not a version policy — the
lockfiles own this" (`:574`) and AD-12 forbids hand-editing it, so restating a
release-please-owned number guarantees recurring drift.

### M3. AD-2's heading claims controlled adapters never reach release bits; one of them is compiled into them, and no rule fixes the gating mechanism

- `ARCHITECTURE-SPINE.md:125` — "### AD-2 — [ADOPTED] One composition root; **controlled adapters never reach release bits**"
- `ARCHITECTURE-SPINE.md:131-133` — "Any controlled adapter is a construction-time dependency of a non-distributable target."

The tree has two different precedents:

- `src-tauri/src/process/fake.rs:9` — "#![cfg(any(test, feature = \"test-util\"))]" — `FakeRunner` is compiled out of release builds.
- `src-tauri/src/events.rs:128-130` — "/// Test sink: records every emitted event in order. / #[derive(Debug, Default)] / pub struct VecSink {" — no `cfg` attribute; `VecSink` is an ordinary `pub` item in the shipping crate (the file's only `#[cfg(test)]` is at `events.rs:300`, below it).

Under the rule's literal words `VecSink` complies (nothing constructs it in
production composition); under the heading it does not (it reaches release
bits). AD-4 requires new ports for the live queue — "the filesystem access
AD-18's attempt journal requires, and the clock any verification or staleness
deadline reads" (`:183-185`) — so UX-PB.2c and UX-PB.3d will each add a
controlled adapter and each pick a precedent. That is the divergence AD-2
exists to prevent.

**Fix.** State the mechanism: new controlled adapters are gated
`#![cfg(any(test, feature = "test-util"))]` like `src-tauri/src/process/fake.rs`,
and record `VecSink` as known brownfield state in the Verified Brownfield
Baseline rather than leaving the heading overstated.

### M4. AD-3's sole-importer rule has no enforcing mechanism, and the AD does not say so

(Carried over unresolved from `review-rubric.md` §16.)

- `ARCHITECTURE-SPINE.md:144-146` — "**Rule:** `src/lib/ipc/bridge.ts` is the only frontend module that imports Tauri APIs, re-exporting exactly `invoke`, `listen`, and `UnlistenFn`."
- `_bmad-output/project-context.md:56` — "`src/lib/ipc/bridge.ts` is the only frontend module that imports Tauri APIs… **The rule is convention-only — no lint enforces it.**"
- `_bmad-output/project-context.md:90` — "There is no configured Prettier or ESLint script."

The tree currently complies — `grep -rn "from ['\"]@tauri-apps" src/` returns
three hits, all in `src/lib/ipc/bridge.ts`. The point is that AD-3 names an
enforcing mechanism for its other rule ("The enforcing mechanism is the shipping
contract test", `:151`) and none for this one, while AD-20 makes the same
boundary security-relevant ("Adding a permission, a plugin, a window, or a
second capability is a security-sensitive change… never folded into a feature
story as a side effect", `:552-555`). 28 frontend stories are about to be
written against an unenforced convention.

**Fix.** Either name the enforcement (a Vitest/Node assertion over `src/**` is
~10 lines and satisfies AD-1 because it adds no product behavior), or say
explicitly that the rule is review-enforced so no story assumes a guard exists.

### M5. "Send no telemetry" — the runtime network-egress posture is silent

**Rubric item:** *A whole dimension left SILENT is a finding.*

- `epics.md:111` — "NFR-5: **Send no telemetry**, expose no generic shell surface, exclude inherited environment values from logs and diagnostics, and resist diagnostic symlink substitution."

Three of NFR-5's four clauses are covered: no shell (`:193-195`), no inherited
environment in the export (`:505-510`), symlink rejection (`:231-232`). The
telemetry/egress clause has no rule. The nearest statements are about other
things:

- `ARCHITECTURE-SPINE.md:569` — "| Determinism | Default suites are offline and deterministic… No real processes, network, sleeps, or host state. |" — a *test* rule.
- `ARCHITECTURE-SPINE.md:547-550` (AD-20) — "The application loads only its own bundled assets. `csp` is `null` today… Any change that introduces remote content… must set a real CSP in the same change" — a *webview asset-loading* rule.

Neither constrains the Rust side, which is where outbound traffic actually
happens (the updater polls a published `latest.json`, and manager `outdated`
probes hit the network). Nothing in the spine says "the application makes
network requests only to (a) the update endpoint and (b) through manager
processes; it emits no analytics, crash, or usage payload."

**Fix.** One rule in AD-20 (or a new sibling of AD-11) fixing the allowed egress
set. It is one sentence and it closes a dimension a personal-tool spine should
be proud to state.

### M6. The operations/observability envelope is thin: runtime logging, log level, and transcript-failure semantics are unowned

**Rubric item:** *operational envelope: deployment & environments, infra/provider
strategy, operations.*

Deployment and provider strategy are covered (AD-11, AD-12, the Stack table's CI
rows). Operations is not. The spine mentions logs and transcripts only as export
payload and directory shape:

- `ARCHITECTURE-SPINE.md:502-504` (AD-18) — "Diagnostics export carries both journals as distinct entries alongside `report.json`, the newest three app logs, and the newest 25 transcripts."
- `ARCHITECTURE-SPINE.md:605-613` (Structural Seed) — "  transcripts/           # newest 25 exported" / "~/Library/Logs/<bundle-id>/ … # newest 3 exported"

The sources put behavioral requirements on that surface that no AD carries:

- `epics.md:109` — "NFR-4: … **block spawn when transcript creation fails**; and keep later noncritical logging failures from hanging Package work."
- `epics.md:85` (FR-17) — "support editable thresholds/**live log level**" — bound to Story 3.4, which the Capability map routes to AD-19 alone (`:626`).
- `epics.md:107` (NFR-3) — "flush live output at 50 milliseconds, 64 lines, or 8 KiB; retain the newest 5,000 live lines at 5,001 while preserving the complete transcript" — bound to UX-PB.3b/3c, which render the same output in two surfaces.

AD-18's "an append failure is nonfatal to package operations" (`:497-498`)
covers journals only. The asymmetry that actually matters — transcript creation
blocks spawn, later transcript writes are best-effort
(`_bmad-output/project-context.md:126`) — is stated nowhere in the spine, and
UX-PB.2d adds `planAttemptId` to "transcript metadata" (`epics.md:675`) without
a rule about what happens when that write fails.

**Fix.** Extend AD-18 (or AD-5) with one rule covering the durable-evidence
failure taxonomy: transcript creation blocks spawn; journal, log, and later
transcript writes are nonfatal and counted; log level is a persisted setting
under AD-19 that takes effect only after persistence.

### M7. AD-11 leaves the surviving accessibility obligation without an owner, and it is not in the Decision Status table

- `ARCHITECTURE-SPINE.md:257-262` — "Automated 4.5:1 text-contrast and reduced-motion checks belong in the Playwright/Vitest lane — **neither exists yet, so this is an obligation on whichever story adds them**, not a description of current coverage. One manual VoiceOver pass sits on the release checklist."

"Whichever story adds them" is not an owner. The one bound accessibility story
does not require them:

- `epics.md:1058-1060` (UX-PB.5d) — "**Then** the dialog traps focus, exposes meaningful names, roles, and states, honors reduced motion, and every safety action … has an accessible name and a reachable focus order." — behavioral criteria, no automated-check obligation.
- `epics.md:240` — "DR-2 is RESTATED by **D33**… **Neither automated check exists yet, so this is an obligation on whichever story adds them**, not a description of current coverage." — the same unowned phrasing, one level down.

An obligation both levels describe as belonging to somebody else belongs to
nobody. It is also absent from the Decision Status and Deferred table, so it is
neither decided, deferred, nor open — the rubric's failure case.

**Fix.** Add a Decision Status row (`Automated contrast + reduced-motion checks
| Open | owner: UX-PB.5d`) or assign it in `epics.md`. Either is fine; silence
in the table is not.

### M8. The Retry → sidecar-region transition is unstated, and it collides with AD-17's accumulation rule

**Rubric item:** *fixes the real divergence points for the level below.*

AD-17 fixes what the region shows in three states and what happens to
membership staged during an attempt:

- `ARCHITECTURE-SPINE.md:459-463` — "While an attempt is non-terminal the region is owned by attempt status, and **new membership staged during that attempt accumulates in the canonical draft without displacing it — surfacing in the region only once the attempt's Results are dismissed.**"
- `ARCHITECTURE-SPINE.md:467-470` — "Its visibility is a three-way union: a non-empty draft, a non-terminal attempt, or undismissed Results… Results remain until dismissed even though the draft behind them is empty."

Two stories then act on the terminal state in ways the union does not resolve:

- `epics.md:791` (UX-PB.3d) — "the sidecar transforms in place into a persistent Results Summary **that remains until `Done`**"
- `epics.md:939` (UX-PB.4d) — "`Create new plan` **rebuilds current canonical intent into a new reviewable draft**, and confirming that draft creates a new attempt with a fresh `planAttemptId`"
- `EXPERIENCE.md:156` — "Retry first shows the failed-item scope inline; `Create new plan` **deliberately replaces Results with a draft** while the immutable result stays in History with `View previous result`."

Unanswered by the spine: does `Create new plan` dismiss Results (UX-PB.3d says
they persist until `Done`)? And does the retry scope *replace* or *merge with*
the membership AD-17 says accumulated in the canonical draft during the attempt?
Two builders, two answers; one of them silently drops user-staged work.

**Fix.** One clause in AD-17: Retry's `Create new plan` is a Results dismissal
that seeds the draft, and its interaction with accumulated membership is
(replace | merge) — pick one.

### M9. Whether verification-refresh Operations are members of the attempt is unstated

AD-16 fixes the freshness and coalescing semantics of verification refreshes
(`:327-332`) but not their identity:

- `ARCHITECTURE-SPINE.md:309-310` — "Admission is atomic and all-or-none. The complete derived operation / set enters the scheduler together or nothing does" — verification refreshes are created after admission, so they are not in that set.
- `ARCHITECTURE-SPINE.md:380`, `:382` (domain minimum) — "  operationIds[]" and "  verificationResults + resultSummary" as separate fields.
- `epics.md:679-681` (UX-PB.2d) — "crash-journal start/finish records, diagnostics, and **verification refreshes carry the same `planAttemptId` where applicable**" — "where applicable" defers the question back up.

Three stories need the answer: UX-PB.2e/3g (does `Cancel plan` cancel a pending
verification refresh? `epics.md:863-865` asks exactly this), UX-PB.3d (does the
`12 of 12` count include them?), UX-PB.4a (do they appear in the History row's
nested evidence?). Today the shipping code auto-enqueues a post-upgrade refresh
as a *separate* operation
(`_bmad-output/project-context.md:124` — "a successful Upgrade/SelfUpdate/HealthFix
auto-enqueues a refresh of the affected subject and executor as a **separate**
operation (`queue.rs`)"), so the spine also does not say whether that existing
operation *is* the verification refresh or is superseded by one.

**Fix.** State it in AD-16: verification refreshes carry the `planAttemptId`,
are attempt-scoped for cancellation, and are excluded from (or included in) the
Results item count — and say whether the existing auto-enqueued refresh becomes
the verification refresh.

---

## LOW

### L1. `Binds:` lists are under-inclusive for AD-17 and AD-18

- `ARCHITECTURE-SPINE.md:438` — "- **Binds:** UX-PB.1a–1e, UX-PB.3a; Stories 3.1, 3.2, 3.5"

AD-17 governs stories not listed: the Results-in-place transform (UX-PB.3d,
`epics.md:791`), the live/replay coexistence in the same region (UX-PB.4c,
`epics.md:919-921`), the confirmation-off sidecar rendering (UX-PB.5c,
`epics.md:1037-1039`), and — most concretely — the 720px rule at
`ARCHITECTURE-SPINE.md:472-474` is the architectural half of UX-PB.5d's
"**below 720 usable CSS pixels the layout enters high-zoom mode**"
(`epics.md:1064`), yet UX-PB.5d is not bound.

- `ARCHITECTURE-SPINE.md:491` — "- **Binds:** UX-PB.2c, UX-PB.2d, UX-PB.2f, UX-PB.4a, UX-PB.4b, UX-PB.4e; Story 6.5"

AD-18 governs UX-PB.3d's "Results persistence failure" path (`epics.md:801-803`)
and UX-PB.4d's `retryOfPlanAttemptId` lineage (`epics.md:946-948`), neither
listed. Since `epics.md` stories cite AD ids to find their constraints, an
under-inclusive `Binds` is a real (if low-blast-radius) miss.

### L2. The draft-durability citation now points at the wrong lines

- `ARCHITECTURE-SPINE.md:646` — "Fail-to-empty. The draft is session-scoped and never persisted; every relaunch starts empty. **`epics.md:437-439` permits this branch explicitly.**"

That was correct against the committed file (`git show HEAD:…/epics.md` line 439
is the UX-PB.1b recovery `Then`). In the working-tree file, `epics.md:437-439`
is the Epic 6 heading — "### Epic 6: Preserve State, Evidence, and Privacy
Across Failure and Relaunch" — and the criterion now lives at
`epics.md:521-523`. Re-anchor it (and prefer quoting the clause over citing line
numbers in a file that is being edited concurrently).

### L3. Live-stream disconnect/reconnect resync is unowned

(Carried over unresolved from `review-rubric.md` §17.)

- `epics.md:775-777` (UX-PB.3c) — "**Given** an attempt in progress (live-state stream disconnect/reconnect) / **When** the per-item progress source drops mid-attempt and later reconnects / **Then** each item keeps its last known honest state and is never silently shown complete…"

AD-3 fixes subscribe-before-hydrate at startup
(`ARCHITECTURE-SPINE.md:163-165` — "Startup subscribes to native events before
`get_state` hydration, and a real detection report is never clobbered by the
pre-detection placeholder") but says nothing about mid-attempt resync, which is
the same clobbering hazard with a live attempt behind it. One sentence
generalizing the startup rule to any re-hydration would cover it.

---

## Checks that pass

Verified this session, all correct as written:

- **20 commands / six events.** `src-tauri/src/lib.rs:232-253` registers exactly
  20 `commands::` entries; `src-tauri/src/events.rs:77-82` declares six event
  names ending "pub const EVENT_APP_UPDATE_STATUS: &str = \"appUpdate:status\";".
  The spine's framing — "a baseline, not a fixed count" (`:160-162`) — is the
  right call and is consistent with `epics.md:210`.
- **15 committed fixtures**, byte-compared both ways. `ls dev/fixtures/ipc/*.json | wc -l`
  → 15; `src-tauri/src/ipc.rs:545-546` — "// Contract test (SPEC §7.4) — byte-equality
  against dev/fixtures/ipc/*.json. / // Regenerate with `PM_UPDATE_CONTRACT=1 cargo test ipc_contract`.";
  `src/lib/ipc/types.test.ts:56` — "it(\"covers exactly the committed fixture set\"".
  AD-3's enforcing-mechanism sentence is accurate.
- **Five ports exist.** `CommandRunner` (`process/runner.rs:26`), `EventSink`
  (`events.rs:124`), `UpdateSource` (`app_update.rs:41`), `PendingRelease`
  (`app_update.rs:48`), `ManagerAdapter` (`managers/mod.rs:67`). AD-4 Rule 1 now
  matches the tree — the defect the earlier review found is fixed.
- **The target-state absence claim.** `planAttemptId`, `Verifying`,
  `InteractionRequired`, `skipUpgradePlanConfirmation`: zero occurrences across
  `src/` and `src-tauri/src/`. The Brownfield Baseline at `:95-98` is exact.
- **Persistence numbers.** `src-tauri/src/journal.rs:19` — "pub const COMPACT_KEEP: usize = 1000;";
  `src-tauri/src/diagnostics.rs:22-23` — "pub const APP_LOGS_INCLUDED: usize = 3;" /
  "pub const TRANSCRIPTS_INCLUDED: usize = 25;".
- **AD-20 is exact.** `src-tauri/tauri.conf.json:25` — "\"csp\": null";
  `src-tauri/capabilities/default.json` grants exactly `core:default`,
  `opener:default`, `core:window:allow-start-dragging` to `["main"]`. (Note the
  spine is *more* current here than `_bmad-output/project-context.md:136`, which
  still lists two permissions.)
- **Release rules.** `minisign -V -p "$RUNNER_TEMP/updater.pub" -x …`
  (`.github/workflows/release.yml:318`) and the published-`latest.json`
  assertion (`release.yml:386-390`) are both present and both block publication;
  `attach_to_tag` gating (`release.yml:363`, `:382`) matches AD-12's claim.
- **AD-16's headline rule.** "No entry point executes… The shipping
  `ManagerPane.upgradeRow` → `executePlan` call site is retired by this rule,
  not preserved by it" (`:287-291`) — this closes the earlier review's CRITICAL,
  names the retiring call site, and correctly frames it as target state.
- **AD-17's draft-durability decision** is a genuine decision with its cost
  stated: "the cost of that choice is a lost draft after a crash, and it was
  accepted deliberately" (`:456-457`), plus a door-closing rule (`:454-457`)
  that stops a later story reopening it by accident. This is what a good spine
  entry looks like.

---

## Summary table

| # | Severity | Finding | Anchor |
| --- | --- | --- | --- |
| C1 | CRITICAL | "No live story needs them controllable" is false — Story 6.5 requires controlled opener success/failure | `ARCHITECTURE-SPINE.md:645`, `epics.md:1247-1249` |
| H1 | HIGH | Native Tauri E2E harness Deferred with a named live consumer and no owner or shape | `ARCHITECTURE-SPINE.md:641`, `epics.md:1234` |
| H2 | HIGH | Failure isolation / Last-good Snapshot dimension silent; AD-16 references rules that do not exist | `ARCHITECTURE-SPINE.md:325`, `docs/SPEC.md:17` |
| H3 | HIGH | "Crash/relaunch lifecycle controller — no live story requires one" contradicted by 4 bound stories | `ARCHITECTURE-SPINE.md:643`, `epics.md:655-657` |
| H4 | HIGH | App-update "blocked while Package work is active" has no stated enforcement point; Rust guard missing | `ARCHITECTURE-SPINE.md:349-355`, `src-tauri/src/commands.rs:770-774` |
| M1 | MEDIUM | `epics.md` retired-register row still `Open`; its description is now false | `ARCHITECTURE-SPINE.md:648`, `epics.md:157-158` |
| M2 | MEDIUM | Application version stated as 1.0.0; tree is 1.0.1 | `ARCHITECTURE-SPINE.md:107`, `:578`, `package.json:4` |
| M3 | MEDIUM | AD-2 heading vs ungated `VecSink`; no gating mechanism for new controlled adapters | `ARCHITECTURE-SPINE.md:125`, `src-tauri/src/events.rs:128-130` |
| M4 | MEDIUM | AD-3 sole-importer rule is unenforced and the AD does not say so | `ARCHITECTURE-SPINE.md:144`, `project-context.md:56` |
| M5 | MEDIUM | Runtime network egress / no-telemetry posture silent | `epics.md:111`, `ARCHITECTURE-SPINE.md:547-550` |
| M6 | MEDIUM | Operations envelope thin: logging, log level, transcript-failure semantics unowned | `epics.md:109`, `ARCHITECTURE-SPINE.md:502-504` |
| M7 | MEDIUM | Automated contrast/reduced-motion obligation has no owner and no Decision Status row | `ARCHITECTURE-SPINE.md:257-262` |
| M8 | MEDIUM | Retry → sidecar-region transition unstated; collides with AD-17 accumulation rule | `ARCHITECTURE-SPINE.md:459-463`, `epics.md:939` |
| M9 | MEDIUM | Verification-refresh Operations' attempt membership and cancellability unstated | `ARCHITECTURE-SPINE.md:309-310`, `epics.md:863-865` |
| L1 | LOW | AD-17 / AD-18 `Binds:` lists under-inclusive | `ARCHITECTURE-SPINE.md:438`, `:491` |
| L2 | LOW | `epics.md:437-439` citation now points at the Epic 6 heading | `ARCHITECTURE-SPINE.md:646`, `epics.md:521-523` |
| L3 | LOW | Live-stream disconnect/reconnect resync unowned | `epics.md:775-777` |
