# Pack-Manager — DECISIONS.md

Each entry: decision → rejected alternative(s) → why.

## D1. Base design: "quality", not "systems" or "ux"

Quality had the highest aggregate judge score (133 vs 128 vs 120) and was top-2 for all three judges; it is the only candidate that named and structurally fixed the mise-shim canonicalization trap. **Rejected:** systems as base (strongest engine, weakest UI — its scheduler and journal are grafted in instead); ux as base (best visuals but carried a latent classification bug — §D3 — and no dual-lock for routed self-updates; its signature visuals are grafted instead).

## D2. Managers' outdated verdict is authoritative; no local version comparison

Real data on this machine includes `2.0.14-1`, `1.6.2.dev0`, `stable`, and rustup commit hashes — semver math would produce wrong verdicts. The frontend's VersionDelta highlight and severity chips are pure string-segment display affordances. **Rejected:** ux's backend severity classification as data (risks disagreeing with the manager; moved to frontend, display-only); any "smart" comparison (rejected by default per quality R10).

## D3. Classify managed-by on the RAW resolved path BEFORE canonicalizing

Mise shims ARE symlinks to the mise binary (Judge 2 verified live: `~/.local/share/mise/shims/uv` canonicalizes to the brew-installed mise under `/opt/homebrew/`), so canonicalize-first would classify uv and npm as brew-managed and misroute their self-updates to `brew upgrade uv`. Evidence strings are stored and surfaced (chip tooltip, logs, diagnostics). Named regression test required. **Rejected:** ux's canonical-path classification (the latent bug); systems' correct-order-wrong-rationale ("shims are not symlinks" — factually wrong, corrected here).

## D4. Lock-set scheduler (from systems), not per-manager lanes with ordered multi-lane acquisition

Ops declare a lock set; a single scheduler task starts an op only when `locks ∩ held = ∅`. Expresses brew self-serialization, cross-manager parallelism, routed self-updates holding {executor, subject} (mise's binary is never replaced under a running mise op), and the shared-tree guards (npm/uv ops take the Mise lock when mise-managed — a concurrent `mise upgrade` could swap the node/uv tree mid `npm install -g`). Atomic acquisition inside one task → deadlock-free without ordinal ordering. Plus: Semaphore(4) global cap (16GB machine, from ux), skip-ahead FIFO with a 120s aging guard (systems' starvation note made concrete). **Rejected:** quality's ordinal multi-lane (equivalent for routed updates but couldn't express the npm↔mise tree hazard cleanly); ux's owner-queue-only model (no subject lock at all).

## D5. npm self-update routes in-band via the "appears in its own outdated list" override

npm's shim lives under mise's dir, so pure path classification would misroute it; the override (from systems) keeps routing fully dynamic — no hardcoding, satisfying the brief. Order: in-band override → delegated-to-detected-owner → native → unavailable. **Rejected:** ux/quality's asserted npm→self without a derivation rule (reads as hardcoding, which the brief forbids).

## D6. All bulk upgrades flow through a plan sheet previewing exact commands

`build_upgrade_plan` (pure) → sheet → `execute_plan`. Nothing runs that wasn't shown; the sheet also carries exclusions with reasons and staleness warnings. Grafted from ux per Judges 1 and 2. Row-level single-package upgrade executes immediately without the sheet (low blast radius, command still visible in drawer + transcript) to keep the common case one click. **Rejected:** quality's count-only confirm dialog (weaker trust device); sheet-for-every-single-row (needless friction).

## D7. Greedy-only casks = two-call set difference; cask JSON treated as unverified

`brew outdated --json=v2 --greedy` minus the plain run identifies self-updating casks; they're grouped separately and excluded from Upgrade All by default. Both captured brew fixtures have `"casks": []`, so the cask JSON shape is UNVERIFIED — serde is default-tolerant and the fixture-tested text form (`openusage (0.6.20) != 0.7.6`) is wired as recovery; capture task tracked. **Rejected:** quality's in-JSON "version looks like latest" heuristic (provably wrong: fixture casks carry concrete versions — Judge 2); including greedy casks in Upgrade All by default (churns self-managing apps).

## D8. JSON parsers with automatic text-parser recovery for mise/npm; synthetic fixtures are labeled and value-grounded

Only `{}` clean captures exist for `mise outdated --json` and `npm outdated -g --json`; the populated text captures are real. Adapters expose `recovery_plan`: on ParseFailed, run the text command once and parse with the fixture-tested text parser. Synthetic JSON fixtures use the `_synthetic` suffix, copy values verbatim from the text captures, and are documented in `dev/fixtures/README.md` with retirement conditions. The 2026-07-23 uv 0.11.30 capture verifies an observed populated `--outdated` shape: parent rows use `name vINSTALLED [latest: LATEST]` and are followed by `- exe` child rows that are not packages. Unknown but valid parent suffixes still degrade to `latest: null`; unmatched lines are ignored, and the UI never receives a fabricated delta (Judge 2's mandate). **Rejected:** always running both JSON+text (doubles network cost); trusting assumed shapes silently (fabrication); treating uv executable child rows as independently upgradable packages.

## D9. Inventory fixtures are captured live BEFORE inventory parsers are written

`brew list --versions`, `brew list --cask --versions`, `mise ls --json`, `npm ls -g --depth=0 --json`, `rustup toolchain list` were never probed in recon, but they are local/offline commands — implementation happens on the target machine, so IMPL_PLAN U3 captures them first and parsers are grounded in real data from day one. The capture also settles the exact `mise ls` flags. **Rejected:** writing those parsers from guessed shapes (all three candidates' weakness, flagged by every judge).

## D10. rust double-report: show both, dedup only inside a single plan

mise reports `rust stable stable stable` while rustup reports the toolchain independently. Policy: both render under their managers (per-manager truth, no magic dedup); the plan builder drops mise's `tool:rust` with reason `rustDedup` and a visible note whenever the same plan contains rustup toolchain targets — one plan never races two upgrades of the same toolchain. **Rejected:** ux's cross-queue dependency edge (complex, effectively untestable — Judge 1); ignoring it (Judges 2/3 flagged the race); global auto-dedup (hides per-manager truth; the mise row is rarely outdated anyway since current==latest).

## D11. Login-shell PATH probe: `-lc` with sentinels, static list as base, merged

Sentinel wrapping (`__PM_S__`/`__PM_E__`, from quality) survives profile noise; non-interactive `-l` only (from systems/ux) because `-i` can hang on TTY-dependent rc files — Judge 2 explicitly rejected quality's `-ilc`. 5s timeout; failure → static fallback, WARN, visible in Environment Report. Children get a constructed env by absolute-path spawn, never inherited PATH. **Rejected:** `-ilc` (hang risk); trusting inherited PATH (the category's #1 failure mode); login-shell-only without static base (probe failure would blind the app).

## D12. One `operations.jsonl` serves as both crash journal and history store

Start records flushed at spawn, finish records at terminal state; start-without-finish renders Interrupted; recorded pgids are never signaled on relaunch (pid reuse). **Rejected:** quality's separate `history.jsonl` + systems' `operations.jsonl` side by side (two files, one purpose); auto-killing orphan pgids at startup (pid-reuse hazard, all judges).

## D13. Cancellation is immediate (SIGTERM→5s→SIGKILL on the process group), no confirmation dialog

Cancel is already a deliberate small click on a specific op; a confirm adds friction to the time-critical action. Partial-state risk is documented in the transcript footer; no rollback attempted. **Rejected:** ux's confirm-on-mutation-cancel (safety benefit judged smaller than the cost; not grafted by any judge).

## D14. No sudo ever: stdin=/dev/null + stall detection + copy-to-terminal handoff

Default silence threshold 120s (configurable), hard cap 30min; the stall dialog offers Keep waiting / Copy command / Cancel and states that Pack-Manager never enters passwords. The handoff is a designed, tested flow (Judge 3's demand), not an afterthought. **Rejected:** ux's 60s default (too eager for slow-but-honest builds); any privilege prompting (hard requirement).

## D15. Pinned brew formulae are never upgradable in-app

Disabled checkbox + tooltip with the `brew unpin` command; excluded from every plan. `brew upgrade` of a pinned formula errors, and force-upgrading would silently defeat the user's pin. **Rejected:** ux's includePinned plan toggle (would require unpin-or-force semantics we refuse to run implicitly).

## D16. Event surface minimized to 5 events; health rides inside snapshots; manager phase is derived

`detection:updated`, `snapshot:updated`, `op:status` (with queued/phase/finish states), `op:output` (batched 50ms/64/8KiB), `op:stalled`. Frontend derives per-manager phase from op records. **Rejected:** systems' separate `manager:status` and `health:changed` events (redundant state channels invite drift).

## D17. Contract drift guard: committed representative payload fixtures checked by BOTH sides

A Rust test asserts serialization byte-equality against `dev/fixtures/ipc/*.json` (regenerate via env flag); a Vitest test runs TS type guards over the same files. Offline, clean-checkout safe. **Rejected:** hand-synced types with no guard (ux); runtime schema export during tests writing outside the tree.

## D18. Navigation: Dashboard cards + per-manager panes + History + Settings (quality), with ux's visual signature grafted on

Version-delta typography, routing chips with plain-language tooltips, phase labels, and the plan sheet carry the crafted identity; the card grid gives the one-glance overview. **Rejected:** shipping BOTH quality's dashboard AND ux's separate "updates inbox" view (scope bloat, two homes for the same data — only Judge 3 hinted at pairing and only as complement).

## D19. Dark-only MVP; tokens structured for a future light theme

Requirement says dark default; a second theme doubles visual QA on a beta OS for zero required value. **Rejected:** shipping light mode at MVP.

## D20. Ad-hoc-signed `.app` is the shipping deliverable; notarization out of scope

macOS 27 beta + Xcode-beta codesign/notarization drift is the top platform risk (all judges); this is a personal tool with no distribution requirement. CI build-smoke runs on stable macos-14 runners; beta-specific issues are diagnosed on-machine. The final gate launches the built bundle via `open` (Finder path) to prove the PATH machinery works where it matters. **Rejected:** notarized DMG at MVP (time sink chasing beta drift).

## D21. npm-inside-mise reset semantics get permanent UI copy at the point of action

The npm SelfUpdateCard always shows: "npm and all global packages live inside the mise-managed node — upgrading node via mise resets them." (Judge 3: prominent at the moment of self-update, not buried.) Functionally the Mise lock (D4) already prevents the mid-flight race; the copy prevents the "my upgrade disappeared" mystery later. **Rejected:** copy-only in Settings/docs; no copy.

## D22. External Homebrew lock contention is detected, named, and never retried automatically

Our serialization prevents self-contention only; a user's terminal brew can still hold the lock. Distinct `brew_lock_busy` error code from the stderr signature, copy "Homebrew is busy in another terminal. Retry when it finishes." **Rejected:** blind retry loops (both judges warned); folding into generic NonZeroExit (loses the actionable message).

## D23. mas adapter ships fully implemented but labeled UNVERIFIED

Detection returns Absent on this machine (verified `zsh: command not found: mas`); parsers are regex-lenient, tested only against `_synthetic` fixtures, and fail as ParseFailed-with-excerpt, never a crash. Installing mas later requires zero code changes. **Rejected:** omitting the adapter (graceful-absence requirement includes future presence); claiming its parsing is verified.

### D23a. Resolved 2026-07-22 — mas is verified live

mas 7.0.0 is installed; `mas list` / `mas outdated` are captured as
`dev/fixtures/mas_list_2026-07-22.txt` and `mas_outdated_2026-07-22.txt` (one
refresh, so the pair is self-consistent) and both `_synthetic` fixtures are
retired. The UNVERIFIED label is withdrawn and the README limitation deleted.

D23's bet paid off — "installing mas later requires zero code changes" held
exactly. But the synthetic fixtures were **wrong about the format** in three
ways, and only regex leniency hid it: mas right-aligns the app id (9-digit ids
carry a leading space), pads the name column to the widest entry rather than
single-spacing, and pads the version column _inside_ the parens, so
`(4.2.2  -> 4.3.0)` leaves trailing spaces on the installed version. The
adapter parsed real output on the first live run, but by luck rather than by
test. **Read as:** a `_synthetic` fixture validates that a parser doesn't
panic; it cannot validate that the parser is right.

## D24. Fixed-stack review (required by the brief)

No fixed decision is fatally flawed. Tauri 2 + Rust backend suits process orchestration; React/TS/Vite/Tailwind suits an event-driven table UI. Watch-items, not flaws: (a) Tauri/wry on macOS 27 beta WebKit — keep the UI free of exotic WebView APIs so regressions stay cosmetic; (b) Tailwind v4 CSS-first `@theme` is already in the scaffold, so tokens live in one CSS file either way.

## D25. Pack-Manager updates itself in-app: check → auto-download → user-clicked restart

`tauri-plugin-updater` against a static `latest.json` on the GitHub Release. Checks run
on launch, every 6h, and on demand from the macOS app menu ("Pack-Manager → Check for
Updates…"); a found update downloads automatically in the background, and the bottom-left
StatusBar indicator turns into the button that installs it and relaunches. Downloading
automatically is safe; installing is not, so the click is the gate — SPEC §1's "No
auto-upgrades without user action" is about mutating the machine, which only install does.
The state machine lives in Rust (`app_update.rs`) behind an `UpdateSource`/`PendingRelease`
seam mirroring `CommandRunner`/`FakeRunner`, so the menu handler and the IPC command share
one code path and the whole flow is testable offline. **Rejected:** the `@tauri-apps/plugin-updater`
JS API (a second direct-to-Tauri surface next to `bridge.ts`, and it would have needed two
events instead of one); a Homebrew cask (no in-app prompt); silent auto-install (never).

### D25a. Consequences accepted

- **A sixth event.** `appUpdate:status` breaks D16's five-event surface. Justified: it is
  not manager state, and folding it into `op:status` would put a non-`Operation` into
  History and the queue.
- **The app menu is rebuilt by hand.** `app.set_menu` replaces Tauri's default wholesale,
  so `lib.rs` re-declares the Edit and Window submenus; without them ⌘X/⌘C/⌘V/⌘A die in
  the package search field and every `CopyableCommand`.
- **No admin prompt, ever.** The plugin's macOS installer falls back to AppleScript `with
administrator privileges` when the bundle's parent directory is not writable. That would
  break SPEC §1 invariant 4, so `app_update.rs` pre-flights with `access(2)` and parks in
  `manualInstallRequired` instead of letting the prompt appear.
- **D20 is superseded.** It said notarization was out of scope; `release.yml` has notarized
  since, and the updater depends on it — an un-stapled auto-update would install an app
  that phones Apple on first launch. The Package step asserts the archived `.app` is
  stapled rather than trusting bundler ordering.
- **CI build-smoke needs `--no-sign`.** With a `pubkey` configured, the CLI refuses to
  bundle without `TAURI_SIGNING_PRIVATE_KEY` ("A public key has been found, but no private
  key"), and a throwaway key fails too because the CLI checks it against the configured
  pubkey.

## D26. One closed, literal list of unterminated notices may be split — nothing else

`mas` 7.0.0 prints `Update progress cannot be displayed` per app during `mas upgrade` with **no line terminator at all** — not `\n`, not `\r`. Verified at the byte level: `od -c` over a captured transcript shows `d i s p l a y e d = = > U \n`, so the notice and the following `==> Updated …` arrive in one `read_until(b'\n')` buffer and render as one jammed line. The existing `\r` split (progress repaints) cannot help; there is nothing to split on.

`runner.rs` carries `UNTERMINATED_NOTICES`, a const list of **verbatim strings, never patterns**, and breaks the line after any entry that has output glued behind it. A notice sitting at the end of a buffer was terminated normally and is left alone.

This is the only place Pack-Manager inserts a line break the child never printed, which is why the rule is deliberately the least clever one available. **Accepted cost:** transcripts are no longer byte-identical to the child's stream for these lines — a break appears where the producer emitted none. Judged worth it: the alternative is an unreadable line every time an App Store app upgrades, and the inserted break is exactly the one the producer should have sent. **Rejected:** splitting before any mid-line `==>` (would corrupt legitimate output containing `==>`); regex/heuristic detection of "a new message probably started here" (unbounded false positives, and it would silently rewrite output from managers that are behaving correctly); doing nothing (the display bug is real and recurs on every `mas upgrade`).

## D27. Every Package and Manager update is staged in one persistent Upgrade Plan

Resolved 2026-07-24 through the approved Correct Course proposal.

The Upgrade Plan is the product's central review and organization layer. A
Package row action, Manager-header action, Manager-wide action, and
`Update Everything` all add eligible work to the same editable draft. No row
or Manager-header update executes immediately. The sidecar appears only after
the first item is added and persists while the user changes Manager views.
Pinned, current, excluded, or otherwise ineligible items cannot be added and
must explain why.

The draft stores canonical intent rather than executable strings. Rust rebuilds
the exact commands whenever the draft changes and again before execution.

**Supersedes:** D6's immediate single-Package exception and its rejection of a
review step for row updates. D6's command-trust and stale-plan protections
remain in force.

## D28. Confirmation is a separate final safety gate with a reversible preference

Resolved 2026-07-24 through the approved Correct Course proposal.

The persistent Upgrade Plan has one primary `Confirm N Updates` action. By
default that action opens a modal confirmation dialog over a dimmed
background. The dialog asks `Proceed with Upgrade Plan?`, shows the exact
commands that will run, and offers `Change Plan` and `Confirm N Updates`.

Only this dialog contains the opt-out checkbox. Selecting
`Disable upgrade plan command execution confirmation` and then confirming
persists `skipUpgradePlanConfirmation: true`. The Settings view exposes the
same preference so it can be reversed. The safe default is `false`; the
obsolete `autoOpenDrawer` setting becomes inactive legacy input.

The preference removes only the final dialog. It never bypasses the persistent
Upgrade Plan, native plan rebuild, stale-plan check, or explicit user action.

## D29. A confirmed plan attempt is the durable Activity, Results, and History unit

Resolved 2026-07-24 through the approved Correct Course proposal.

The existing one-use preview `planId` remains a short-lived capability and must
not be reused as history identity. Every confirmed execution receives a new
durable `planAttemptId`. Its Operations, events, transcripts, journal records,
verification refreshes, terminal Results, and optional `retryOfPlanAttemptId`
are correlated to that identity.

History contains one immutable entry per confirmed plan attempt, with
Operation-level evidence nested inside it. A retry creates a new attempt and
links back to the failed attempt; it never overwrites the first failure.
Legacy Operation records that lack an attempt identity remain visibly legacy
Operations and are never fabricated into plans.

**Supersedes:** D12 only where it equates one Operation journal with the
complete user-facing History model. `operations.jsonl` may remain crash
evidence, but plan-attempt persistence and reconstruction are now required.

## D30. One active plan, explicit Activity and Results, and trusted interaction classification

Resolved 2026-07-24 through the approved Correct Course proposal.

Only one confirmed Upgrade Plan attempt may be active at a time.
Cross-Manager concurrency still occurs inside that attempt under D4's lock-set
scheduler. While it runs, the plan sidecar becomes live status. Completed
items fill their progress treatment and collapse from `old -> new` to the
verified current version. Terminal state becomes a Results summary with
successes, failures, skipped work, verification outcomes, actionable error
explanations, and Retry where appropriate.

Activity is a first-class navigation destination for the active attempt and
for replaying a completed History entry. A queued draft remains in the Upgrade
Plan, not Activity. Plan-level cancellation stops queued work for that attempt
and marks it honestly; an Operation-level cancel label is reserved for an
explicitly Operation-scoped diagnostic action.

`Interaction required` may be shown only when a closed, Manager-specific
classifier or explicit native signal recognizes a known prompt. Unknown
silence with null stdin follows D14's ordinary stall path. This avoids
inventing prompt meaning from arbitrary output.

**Supersedes:** D18's flat Manager navigation and drawer-only Activity model.
D13 remains valid for immediate cancellation mechanics, but the primary
user-facing action is now `Cancel plan` when the whole attempt is affected.

## D31. Minimum supported macOS is 15.0

Resolved 2026-07-24 through the approved Correct Course proposal. Closes
`DR-1` ("Minimum supported macOS version — OPEN, implementation-entry
blocker"), which lived in the retired PRD gate-governance register.

`src-tauri/tauri.conf.json` now declares `bundle.macOS.minimumSystemVersion`
= `"15.0"`. Until now the repo declared no floor anywhere and inherited
Tauri's documented default of `10.13`. That is why shipped v0.2.9 carries
`LSMinimumSystemVersion 10.13` while its arm64 slice is compiled `minos
11.0` — arm64 macOS did not exist before Big Sur, so 11.0 is a hard rustc
floor and every universal Tauri build on defaults produces this same split.
Declaring the floor replaces an inherited default nobody chose with a chosen
one, and removes the plist/binary inconsistency regardless of value.

15.0 clears the arm64 11.0 clamp and, as of 2026-07, sits inside Apple's
security-update window and is Homebrew Tier 1 — which matters because this
app is primarily a Homebrew GUI. Both are external vendor facts, sourced from
documentation rather than this repo, and both will age. The floor moves when
Apple drops security support for it, not on a fixed schedule.

A deployment target above the build SDK is a floor annotation, not an SDK
requirement: measured `rc=0` with the expected `minos` for targets 15.0,
26.0, and 30.0 against SDK 27.0, via both clang and rustc. CI therefore stays
on `macos-14`.

**One question remains OPEN at the time of writing:** whether `notarytool`
accepts `minos 15.0` against SDK 14.5. Nothing here asserts that it does. It
is settled by a manual Release workflow run — which builds, signs, and
notarizes, and uploads to the workflow run only, never touching a GitHub
Release. If notarization rejects the floor, this record changes rather than
the pipeline absorbing a surprise later.

**Rejected:** leaving the floor undeclared (ships an inherited default and
keeps the plist/binary mismatch); 11.0 (matches the arm64 clamp but sits
outside Apple's security-update window).

## D32. The universal build is retained; verification is Apple silicon only

Resolved 2026-07-24 through the approved Correct Course proposal. Narrows
`DR-3` ("Intel execution evidence — APPROVED"), which lived in the retired
PRD gate-governance register.

`release.yml` is unchanged. It continues to build `universal-apple-darwin`
and to publish both `darwin-aarch64` and `darwin-x86_64` in `latest.json`,
both pointing at the single universal archive. One archive, one signature,
both keys already resolve against it — so retaining Intel costs nothing and
strands nobody.

What is dropped is the obligation to physically verify on Intel hardware.
DR-3 required a physical Intel fresh-install, Finder/Dock launch, and
prior-public-version update run, and held that universal-binary inspection
could not substitute. The maintainer does not own an Intel Mac and does not
intend to acquire one, so that obligation could never be discharged; it
blocked 3 stories directly and 11 more transitively.

The promise is narrowed to match what is actually verified: the release
builds universal; verification is Apple silicon only. Intel remains
best-effort and unverified.

**Rejected:** dropping the x86_64 slice (the updater resolves its target from
`cfg!(target_arch)`, so removing the `darwin-x86_64` key would strand any
installed Intel user with no signal); keeping the physical-Intel requirement
(unsatisfiable, and permanently blocking).

## D33. The formal readiness gate is retired; the plan is rescoped

Resolved 2026-07-24 through the approved Correct Course proposal. Dissolves
`DR-4` ("P1 gate policy — PROPOSED, gate-approval blocker") and restates
`DR-2` ("Packaged accessibility evidence — APPROVED"). Both lived in the
retired PRD gate-governance register.

Pack-Manager is a personal, open-source macOS utility, not a commercial
product. Observable evidence: 1 star, 0 forks, 12 releases inside a 48-hour
window, and 3 lifetime `.dmg` downloads. The other 27 of the 30 recorded
downloads are the app's own updater traffic — 17 × `latest.json` polling and
10 × `.app.tar.gz` payloads, both automatic under D25.

The planning stack built for it did not match that reality. 83 planned
stories split into 28 real product stories (the D27–D30 Upgrade Plan
redesign) and 55 evidence-production stories carrying exactly one versioned
scenario contract each — all 55 unassigned, against a `contracts/` directory
that does not exist and that no story creates. The evidence schema bound even
manual scenarios to GitHub Actions run provenance, which nothing in the plan
provisioned.

Retired: the 72-criterion P0 gate, all P0/P1/overall coverage percentages,
the 55 scenario contracts, the evidence-manifest and candidate-freeze
machinery, the multi-host environment requirements, and Epics 7–8. They are
replaced by `docs/RELEASE-CHECKLIST.md` plus two automated checks added to
`release.yml`: the updater's detached signature is verified against the
public key the shipping app embeds, and the published `latest.json` is
asserted reachable and coherent after upload.

DR-2's substance survives without its gate framing. Automated 4.5:1 contrast
and reduced-motion checks belong in the existing Playwright/Vitest lane; one
manual VoiceOver pass joins the release checklist. Accessibility here is
product quality, not evidence ceremony.

One habit survives from the retired gate: before scheduling work described as
a test gap, verify whether the behavior is already present in the shipping
code. The Epics 1–6 triage found most of it was — an adversarial pass
overturned 14 of 20 initial keeps for exactly that reason.

Retired planning artifacts are archived under
`_bmad-output/archive/2026-07-24-scope-recalibration/`, not deleted.

**Rejected:** completing the gate. Its two rules fail for different reasons,
and conflating them obscures both. The legacy P1 rule fails by construction:
5 of its 8 rows are declared out of scope, capping achievable coverage at
3/8 = 37.5% against an 80% minimum, so no amount of work can pass it. The
72-criterion P0 rule is achievable in principle — 100% of criteria you intend
to deliver is just work — but discharging it requires the evidence
infrastructure described above, which was never built and is disproportionate
to a tool with 3 lifetime installs. Also rejected: leaving the gate documents
in place as aspirational, since BMAD skill runs glob them back into the plan,
re-entrenching what this record retires.
