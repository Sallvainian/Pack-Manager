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
Only `{}` clean captures exist for `mise outdated --json` and `npm outdated -g --json`; the populated text captures are real. Adapters expose `recovery_plan`: on ParseFailed, run the text command once and parse with the fixture-tested text parser. Synthetic JSON fixtures use the `_synthetic` suffix, copy values verbatim from the text captures, and are documented in `dev/fixtures/README.md` with retirement conditions. uv's populated `--outdated` format is unknown (0-byte capture): unknown suffixes degrade to `latest: null` and the UI shows "update available" — never a fabricated delta (Judge 2's mandate). **Rejected:** always running both JSON+text (doubles network cost); trusting assumed JSON shapes silently (fabrication).

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

## D24. Fixed-stack review (required by the brief)
No fixed decision is fatally flawed. Tauri 2 + Rust backend suits process orchestration; React/TS/Vite/Tailwind suits an event-driven table UI. Watch-items, not flaws: (a) Tauri/wry on macOS 27 beta WebKit — keep the UI free of exotic WebView APIs so regressions stay cosmetic; (b) Tailwind v4 CSS-first `@theme` is already in the scaffold, so tokens live in one CSS file either way.
