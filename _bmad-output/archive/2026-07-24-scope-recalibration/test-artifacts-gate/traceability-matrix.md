---
stepsCompleted:
  [
    "step-01-load-context",
    "step-02-discover-tests",
    "step-03-map-criteria",
    "step-04-analyze-gaps",
    "step-05-gate-decision",
  ]
lastStep: "step-05-gate-decision"
lastSaved: "2026-07-22"
courseCorrectionOverlay: "2026-07-24"
tempCoverageMatrixPath: "/tmp/tea-trace-coverage-matrix-2026-07-23T02-28-24Z.json"
gateEligible: true
collectionStatus: "COLLECTED"
gateDecision: "FAIL"
workflowType: "testarch-trace"
sourceCommit: "fe2881f3e48d26c0561857f72143c6570a5620fc"
sourceWorkingTree: "dirty"
inputDocuments:
  - "docs/SPEC.md"
  - "docs/DECISIONS.md"
  - "_bmad-output/project-context.md"
  - "_bmad-output/implementation-artifacts/spec-harden-command-trust-boundaries.md"
  - "_bmad-output/test-artifacts/automation-summary.md"
coverageBasis: "acceptance_criteria"
oracleConfidence: "high"
oracleResolutionMode: "formal_requirements"
oracleSources:
  - "docs/SPEC.md"
  - "docs/DECISIONS.md"
externalPointerStatus: "not_used"
---

# Traceability Matrix & Gate Decision - Pack-Manager

**Target:** Pack-Manager product and release requirements  
**Date:** 2026-07-22  
**Evaluator:** Codex, Master Test Architect  
**Source snapshot:** `fe2881f3e48d26c0561857f72143c6570a5620fc` plus the current uncommitted automation changes  
**Coverage oracle:** Acceptance criteria and normative product requirements  
**Oracle confidence:** High  
**Oracle sources:** `docs/SPEC.md`, amended by `docs/DECISIONS.md`

---

Note: This workflow analyzes existing tests and does not generate tests. Gaps can be addressed later with the ATDD or test-automation workflows.

> **2026-07-24 Correct Course overlay:** This matrix remains a historical
> planning snapshot of the older oracle. Decisions D27-D30 now require every
> Package/Manager update to enter one persistent Upgrade Plan. `AUT-003`
> demonstrates superseded immediate-row behavior and provides **no positive
> evidence** for revised `F5-AC3`. No status in this historical matrix is
> promoted or demoted here. Regenerate traceability only after the Product
> Behavior Prerequisite is implemented, the revision-2 map is mechanically
> reconciled and approved, and qualified evidence exists.

## Step 1: Coverage Oracle and Context

### Resolved Oracle

The coverage oracle is the formal product requirements in `docs/SPEC.md`, with later superseding decisions in `docs/DECISIONS.md` applied as an amendment overlay.

This is the strongest available oracle because the specification declares itself authoritative, defines prioritized P0 requirements F1-F12 and P1 requirements F13-F17, includes explicit acceptance statements for F1-F8, and supplies normative, testable behavior for the remaining product, architecture, test, and delivery requirements. A synthetic source-derived oracle is unnecessary.

The completed command-trust-boundary implementation spec and the current automation summary are supporting evidence. They refine implementation acceptance and describe newly added tests, but they do not replace the product-level authority of the specification and decisions.

### Oracle Metadata

- **Coverage basis:** `acceptance_criteria`
- **Resolution mode:** `formal_requirements`
- **Confidence:** `high`
- **External pointer status:** `not_used`
- **Why high confidence:** The primary specification is explicit, prioritized, test-oriented, and paired with a dated decision log that identifies superseded clauses.

### Artifacts Found

- **Primary formal requirements:** `docs/SPEC.md`
- **Current amendment and precedence record:** `docs/DECISIONS.md`
- **Persistent project and testing rules:** `_bmad-output/project-context.md`
- **Current implementation acceptance:** `_bmad-output/implementation-artifacts/spec-harden-command-trust-boundaries.md` (status `done`)
- **Current automation evidence:** `_bmad-output/test-artifacts/automation-summary.md`
- **Historical implementation reference:** `docs/IMPL_PLAN.md` exists but is not current authority.
- **Internal contract support:** Rust and TypeScript IPC types plus committed representative IPC fixtures exist. There is no standalone OpenAPI, GraphQL, protobuf, Pact, or HTTP service contract.
- **Formal artifacts not found:** No separate PRD, epic, story, acceptance-criteria file, or test-design document was found.
- **External requirements pointers:** None were found, so no external adapter or document resolution was needed.

### Authority and Drift Rules

When sources conflict, the later explicit decision or current normative specification controls:

1. **mas status:** D23a supersedes the earlier absent/unverified target-machine assumption; mas is live-verified.
2. **Packaging and updates:** D25 supersedes D20 and the original ad-hoc-only packaging clause; current releases are signed, notarized, and support a user-gated in-app update flow.
3. **Event surface:** D25a adds `appUpdate:status` as a justified sixth event, superseding the older five-event count without changing manager-state rules.
4. **Transcript fidelity:** D26 creates one closed literal exception for known unterminated mas notices; all other output remains faithful to the child stream.
5. **Command execution trust:** The current SPEC F4 wording and completed hardening spec govern one-use plan capabilities, coherent-state revalidation, atomic batch admission, and backend-only structured argv.
6. **Current implementation evidence:** Test inventories and gate calculations must be rebuilt from this checkout. Counts and conclusions from the older trace artifacts are not reused as current evidence.

### Knowledge Base Loaded

- `test-priorities-matrix.md`
- `risk-governance.md`
- `probability-impact.md`
- `test-quality.md`
- `selective-testing.md`

### Step 1 Outcome

The formal oracle is usable and sufficiently complete to proceed. Requirements will be normalized into atomic trace criteria during mapping, with their declared P0/P1 priority retained and superseded statements removed before coverage is calculated. Test discovery and requirement-to-test mapping have not yet been performed in this run, so this checkpoint makes no coverage percentage or gate decision.

## PHASE 1: REQUIREMENTS TRACEABILITY

### Step 2: Test Discovery and Catalog

#### Discovery Method

- Vitest's collector (`npx vitest list --json --includeTaskLocation`) identified every concrete frontend test, including cases generated from fixture loops, without executing test bodies.
- Cargo's harness listing (`cargo test --locked -- --list --format terse`) identified the canonical Rust set. It compiled and listed the harness but did not execute tests.
- Playwright's collector (`npx playwright test --list`) identified authored logical cases and their Chromium/WebKit execution variants without running them.
- Source scans supplied exact declaration lines, describe/module contexts, explicit priority markers, ignore attributes, and native-boundary evidence.

#### Inventory Summary

| Level     |  Active | Ignored |   Total | Notes                                                                             |
| --------- | ------: | ------: | ------: | --------------------------------------------------------------------------------- |
| Unit      |     301 |       9 |     310 | 56 Vitest unit tests plus 254 Rust unit tests; 245 Rust unit tests run by default |
| Component |      77 |       0 |      77 | React Testing Library/jsdom with fake IPC                                         |
| E2E       |       6 |       2 |       8 | Six logical Playwright cases; two ignored live-machine Rust smoke tests           |
| API       |       0 |       0 |       0 | No HTTP/service API exists                                                        |
| **Total** | **384** |  **11** | **395** | 133 Vitest + 256 Rust + 6 logical Playwright cases                                |

Additional inventory facts:

- **Test files:** 58 total: 23 Vitest, 4 Playwright, and 31 Rust source/integration files.
- **Playwright execution variants:** Six logical cases expand to 12 executions across Chromium and WebKit.
- **Execution flags:** Vitest and Playwright contain no authored skip, todo, pending, fixme, focused, or conditional-skip cases. Rust has 11 `#[ignore]` cases and one active expected-panic test.
- **Priority markers:** Five tests are explicitly P0: AUT-002, AUT-003, AUT-004, and the two package-factory verdict contracts. The remaining 390 tests are unmarked; requirement priority will be inherited from the formal oracle during mapping.
- **Level convention:** Internal Rust parser, scheduler, process, persistence, and command-harness tests are classified as Unit because the workflow's allowed levels do not include Integration. Their full module titles preserve their broader semantics.
- **Collection versus execution:** This step performed discovery only. The current automation summary records a prior green run of 133/133 Vitest, 245 default Rust tests with 11 ignored, and 12/12 Playwright project executions; execution evidence will be evaluated at the gate step rather than treated as discovery.

#### Coverage Heuristics Inventory

```yaml
coverage_heuristics:
  api_endpoint_coverage:
    applicability: not_applicable
    reason: "Pack-Manager has no REST, GraphQL, or server API."
    product_boundary: "local Tauri IPC"
    registered_commands: 20
    direct_native_tauri_invoke_test: false
    evidence:
      - "Rust validates 15 committed IPC JSON contracts byte-for-byte."
      - "Frontend type guards accept the same 15 committed fixtures."
      - "Rust command tests call the backend harness directly; Playwright uses a browser-local fake Tauri runtime."
    gap: "No automated test boots the real Tauri invoke handler and crosses JavaScript serialization into a registered Rust command/event path."
  authentication_authorization:
    applicability: not_applicable
    reason: "The app is a local single-user desktop tool with no accounts, sessions, roles, tokens, or logout."
    relevant_security_rule: "No sudo or password prompt."
    covered_examples:
      - "Stall handoff explicitly states that Pack-Manager never enters passwords."
      - "A non-writable app-update target surfaces manualInstallRequired instead of an administrator prompt."
  error_path_coverage:
    strength: strong_at_unit_and_component_partial_at_e2e
    covered:
      - "All internal PmError variants and stable IPC codes"
      - "Parser failures, expected/non-expected exits, timeouts, stalls, cancellation, and process-group termination"
      - "Refresh isolation and stale snapshots"
      - "Plan tamper, replay, eviction, drift, conflicting mutations, untrusted argv, and all-or-none admission"
      - "Settings persistence, journal corruption, diagnostics omissions, updater check/download/install failures"
      - "Stale-plan rebuild, failed rebuild/execute, out-of-order responses, listener failure, and persistent UI errors"
    gaps:
      - "Row-level build/execute rejection still lacks user-visible failure handling and a browser negative-path test."
      - "The 15 frontend IPC fixture guards are positive acceptance tests; malformed full-payload rejection is not broadly exercised."
      - "The browser upgrade journeys are happy-path only; backend negative paths remain lower-level evidence."
  ui_journey_coverage:
    playwright_covered:
      - "Dashboard shell and npm manager navigation"
      - "Package-name search"
      - "Selected-package plan review and confirmation"
      - "Immediate row-level upgrade without a plan dialog (superseded behavior; no revised credit)"
      - "Dark tokens, focus-visible treatment, and reduced-motion suppression"
    not_covered_end_to_end:
      - "History and Settings/Environment Report navigation"
      - "Activity drawer, streaming output, cancellation, and stall handoff"
      - "Refresh error/stale/absent states and manager self-update routing"
      - "Pinned/greedy exclusions, selection keyboard/range behavior, and stale-plan recovery"
      - "Diagnostics export, updater menu/install, packaged-app launch, signing, notarization, and stapling"
      - "P1 features F14-F17"
    boundary_note: "Playwright WebKit is cross-browser evidence, not the packaged Tauri WKWebView."
  ui_state_coverage:
    lower_level_strength: strong
    covered:
      - "Loading skeletons and manager re-detection transitions"
      - "Up-to-date/neutral and not-installed states"
      - "Refresh error, Retry, retained stale snapshot, and failed bootstrap recovery"
      - "Pending/failed/stale plan review gates"
      - "Updater idle, checking, determinate/indeterminate download, ready, error, and manual-install states"
    gaps:
      - "No direct empty-History test"
      - "No direct empty-package-list test separate from all-current state"
      - "No permission-denied UI state beyond updater manual-install fallback"
  network_isolation:
    claim: "Ordinary browser cross-origin requests are blocked under the default local E2E configuration."
    limitations:
      - "The explicit test exercises fetch only."
      - "WebSockets and service-worker traffic are not proven blocked."
      - "Same-origin traffic is allowed, and ALLOW_REMOTE_E2E=1 disables the guard."
```

#### Stable Identity Rules

- Vitest IDs are `VT-` plus the first 12 uppercase SHA-256 characters of `relative-file|line|full collector title`.
- Rust IDs are `RS-` plus the first eight uppercase SHA-256 characters of the exact Cargo test title.
- Playwright preserves authored AUT IDs where present; unmarked cases receive deterministic `PW-` IDs based on file and full title.
- Catalog state `A` means active with `skipped=false`, `pending=false`, and `fixme=false`; `I` means Rust `#[ignore]` with `skipped=true`, `pending=false`, and `fixme=false`.

#### Playwright Logical Test Catalog

| ID          | Priority | State | File                                         | Full title                                                                                                                   | Boundary                     |
| ----------- | -------- | ----- | -------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------- | ---------------------------- |
| AUT-004     | P0       | A     | `tests/e2e/browser-style-contract.spec.ts:8` | browser-rendered style contract › [P0] AUT-004 applies dark tokens, keyboard focus treatment, and reduced-motion suppression | Real browser CSS; fake Tauri |
| PW-416E2F1A | -        | A     | `tests/e2e/framework-contract.spec.ts:50`    | Playwright framework contract › drives the fake Tauri runtime through the real API wrappers                                  | Framework contract only      |
| PW-25B437A7 | -        | A     | `tests/e2e/framework-contract.spec.ts:158`   | Playwright framework contract › blocks outbound requests before they leave the browser                                       | Ordinary fetch guard only    |
| PW-46F8A8F6 | -        | A     | `tests/e2e/package-search.spec.ts:9`         | package search › filters a manager package list by its visible package name                                                  | Browser UI; fake Tauri       |
| AUT-002     | P0       | A     | `tests/e2e/upgrade-journeys.spec.ts:10`      | upgrade journeys › [P0] AUT-002 reviews the exact selected-package plan before execution                                     | Browser UI; fake Tauri       |
| AUT-003     | P0       | A     | `tests/e2e/upgrade-journeys.spec.ts:169`     | SUPERSEDED — executes a one-row Upgrade immediately without the required persistent plan                                     | Browser UI; fake Tauri       |

#### Vitest Catalog

Each group header is `[file|level|count]`; each row is `id|priority|state|line|full describe/title`.

```text
[src/__tests__/bootstrap.test.tsx|Component|2]
VT-274330D6FAC1|-|A|L34|launch_refresh_waits_for_detection > defers refresh_all until the first real detection:updated instead of racing it
VT-3672CCB30270|-|A|L64|bootstrap_failure_is_logged_and_recoverable > logs the real IpcError payload (not '[object Object]') and still runs the launch refresh

[src/__tests__/dashboard.test.tsx|Component|4]
VT-42F22198677A|-|A|L20|dashboard_fills_cards_independently > fills the npm card while brew is still a skeleton
VT-EEBA2D9C93AA|-|A|L35|mas_absent_renders_not_installed_with_hint > shows a muted Not installed card with the install command
VT-F44143F3727F|-|A|L44|manager_error_isolates_with_retry_and_stale_snapshot > shows an error card with Retry while other cards stay populated
VT-E39014F4807E|-|A|L77|all_clean_renders_up_to_date > shows the up-to-date state when nothing is outdated

[src/__tests__/events.test.ts|Unit|7]
VT-D8658959DCEF|-|A|L47|onSnapshot > stores the snapshot and clears any prior manager error
VT-3FA738A696CA|-|A|L56|onStatus — refresh isolation (SPEC §F2) > a failed refresh sets the manager error and marks its snapshot stale
VT-294503DC62A2|-|A|L76|onStatus — op record upsert > upserts a record for a mutating op without touching manager errors
VT-F421BC31E7C4|-|A|L84|onOutput > appends batches to the op ring buffer
VT-CEB335A5A522|-|A|L91|onStalled > opens the stalled dialog with the silent duration
VT-A022DF68D0BC|-|A|L108|subscribeEvents — no listener leak on partial failure > registers all five listeners and tears them all down
VT-66E2C351E535|-|A|L115|subscribeEvents — no listener leak on partial failure > unlistens the already-registered listeners when one listen() rejects

[src/__tests__/refreshRedetect.test.tsx|Component|2]
VT-33EC5E3AB248|-|A|L42|refresh_all_re_detects_managers_mid_session > transitions mas absent → present on the dashboard and sidebar from one Refresh All
VT-22082392E1E2|-|A|L100|refresh_all_re_detects_managers_mid_session > renders a manager absent again when a re-detect reports it gone

[src/__tests__/settings.test.tsx|Component|6]
VT-1BC2C8F7E79B|-|A|L20|settings form > renders the preference controls
VT-70364B262530|-|A|L27|settings form > persists a toggled setting via set_settings and applies the merged result
VT-110ACE171098|-|A|L44|settings form > persists the log level via a select
VT-D59F5E461AC7|-|A|L59|environment report > shows the PATH source and per-tool evidence
VT-0F530C5EB18D|-|A|L67|maintenance actions > Re-detect invokes detect_managers
VT-6DB3A4D43A8B|-|A|L74|maintenance actions > Export diagnostics and Open Logs Folder invoke their commands

[src/__tests__/sidebar.test.tsx|Component|4]
VT-5052CEDD1836|-|A|L19|sidebar present managers > lists present managers in order and shows the outdated count pill
VT-B2EB0AAB0551|-|A|L33|sidebar present managers > navigates to a manager pane on click
VT-A27106C58450|-|A|L39|sidebar present managers > routes Dashboard / History / Settings
VT-B18DC3484045|-|A|L51|sidebar absent disclosure > reveals the Not installed managers with their install hint

[src/__tests__/smoke.test.tsx|Component|1]
VT-23DA5FD88FC4|-|A|L26|App shell > renders the full shell against fake data and hydrates from get_state

[src/__tests__/store.test.ts|Unit|13]
VT-99E01C630A0B|-|A|L33|operations ring buffer > caps at LOG_CAP lines and counts the overflow
VT-BDCDB3C4FA01|-|A|L47|operations applyStatus > preserves queuedAt/packageIds across later status events
VT-46861EE61B7C|-|A|L78|deriveManagerPhase > is refreshing while a refresh op for the manager is live
VT-C4E47F283695|-|A|L84|deriveManagerPhase > is busy while a non-refresh op touches the manager
VT-BA11EAADDDC8|-|A|L91|deriveManagerPhase > is error when the manager has a last error, regardless of ops
VT-1878C3ED4881|-|A|L95|deriveManagerPhase > is idle with no live ops and no error
VT-2BFF90575AF5|-|A|L101|packages selection > toggles a package and sets the anchor
VT-73B7C5B7B7C8|-|A|L110|packages selection > selects an inclusive range from the anchor
VT-DD8695C22EB4|-|A|L119|packages selection > clears selection and anchor
VT-071F72E0E189|-|A|L128|packages setSnapshot > clears stale and defaults outdatedOnly ON when anything is outdated
VT-0BCEFFD1E242|-|A|L136|packages setSnapshot > drops selected ids that no longer exist after a refresh
VT-6A8DAE6A73D0|-|A|L150|outdated counting > counts the manager's outdated verdict but excludes greedy casks
VT-5C06EA2DDBC1|-|A|L160|managers store errors > sets and clears a per-manager error

[src/components/activity/dialogs.test.tsx|Component|2]
VT-1AF53546B5EA|-|A|L36|stall_dialog_keep_waiting_vs_cancel > renders the no-password handoff copy; Keep waiting dismisses, Cancel cancels
VT-42BECAA13CEE|-|A|L64|quit_guard_lists_ops_and_cancels_all > lists the running ops and cancels each on quit

[src/components/activity/keyboard.test.tsx|Component|7]
VT-4E19958FE4A5|-|A|L46|keyboard_map_dispatches_actions > Cmd+L toggles the drawer
VT-377DA7826C87|-|A|L55|keyboard_map_dispatches_actions > Cmd+A selects every visible selectable row for the current manager
VT-EF327861FFDF|-|A|L69|keyboard_map_dispatches_actions > Cmd+R refreshes the current manager
VT-B586ED40424B|-|A|L76|keyboard_map_dispatches_actions > Cmd+Shift+R refreshes all
VT-08D613607D9D|-|A|L82|keyboard_map_dispatches_actions > Esc clears a non-empty selection before closing the drawer
VT-09DBE3EB43E6|-|A|L90|keyboard_map_dispatches_actions > Cmd+Shift+U opens the plan sheet for everything
VT-57B193EAC22D|-|A|L101|keyboard_map_dispatches_actions > ignores shortcuts while typing in an input (except Escape)

[src/components/activity/liveLog.test.tsx|Component|7]
VT-86B761D03585|-|A|L39|log_view_appends_batches_pins_unpins_with_jump_chip > renders appended batches, and unpin surfaces the jump chip which re-pins
VT-29FFF28B94E5|-|A|L72|log_view_appends_batches_pins_unpins_with_jump_chip > collapses a carriage-return progress repaint to its final segment
VT-0A5F08A3B636|-|A|L80|log_view_appends_batches_pins_unpins_with_jump_chip > shows the ring-buffer cap banner when older lines overflowed to the file
VT-85DD2A3A37EE|-|A|L94|log_view_single_render_per_batch > commits exactly once per appended batch
VT-FD19DA096424|-|A|L113|activity_drawer_summary_and_expanded_panes > summarises running ops in the bar and shows the list + log when expanded
VT-61291967FF28|-|A|L132|focus_switch_repins_log_view > switching the focused op remounts the log pinned to the tail (no stale chip/scroll)
VT-804FDC5924E2|-|A|L179|cancel_flips_pill_on_event > re-renders the status pill Running -> Cancelled when the op:status event lands

[src/components/activity/opDisplay.test.ts|Unit|5]
VT-D81307CE9FA3|-|A|L14|durationLabel > returns '' when the op has not started
VT-5DFE9E67CB01|-|A|L18|durationLabel > ticks against `now` while running
VT-C96D93008899|-|A|L25|durationLabel > freezes finished ops at start→finish regardless of `now`
VT-8482B2EA6E28|-|A|L35|durationLabel > returns '' (→ em-dash) for an interrupted op with no finishedAt — never a ticker
VT-ACC57A5C4E32|-|A|L45|durationLabel > returns '' for EVERY terminal status lacking a finishedAt

[src/components/activity/opEffects.test.tsx|Component|3]
VT-945BEF349DE0|-|A|L51|drawer_auto_open_honors_setting > opens+focuses on a mutating start only when enabled, and never for refreshes
VT-F9E8898B02B8|-|A|L83|failure_toast_persists_and_view_log_focuses_op > raises a persistent failure toast whose View log focuses the op + opens the drawer
VT-EB14F746EEBC|-|A|L112|success and cancel toasts > raises an auto-dismissing success toast for a completed upgrade, but none for a refresh

[src/components/history/history.test.tsx|Component|3]
VT-B32E0901F128|-|A|L19|interrupted_ops_render_in_history > renders a start-without-finish record as Interrupted in the table
VT-695BBDD24EE7|-|A|L37|history filters and export > filters by status and exports diagnostics
VT-21877AA1D445|-|A|L59|history filters and export > expands a row to load the transcript tail

[src/components/manager/healthBanner.test.tsx|Component|2]
VT-7A9878F09808|-|A|L29|health_banner_renders_fix_command > renders the issue title, the copyable fix command, and wires Run fix
VT-27A50F858EBE|-|A|L43|health_banner_renders_fix_command > keeps an altered uv suggestion visible without exposing Copy or Run fix

[src/components/manager/managerPane.test.tsx|Component|4]
VT-5301E7F83768|-|A|L27|selection_shift_range_tri_state_and_toolbar_count > shift-click selects the inclusive range and drives the toolbar + tri-state
VT-16B9A758AFD2|-|A|L45|select_all_respects_filter_and_never_greedy_or_pinned > select-all picks only visible, outdated, non-pinned, non-greedy rows
VT-71EA86F08E57|-|A|L62|outdated_only_toggle_default_behavior > defaults to outdated-only and reveals up-to-date rows when toggled off
VT-F45377D5FFC4|-|A|L76|search_filters_names_and_executables > matches both package names and their executables

[src/components/manager/planSheet.test.tsx|Component|13]
VT-0BE5205EB6EA|-|A|L72|plan_sheet_renders_exact_command_previews_and_excluded_reasons > shows every command verbatim, the excluded reasons, warnings and notes
VT-D6D6DE8AEA1D|-|A|L90|greedy_toggle_off_by_default > defaults self-updates on and self-updating casks off
VT-B28DBF79D340|-|A|L98|confirm_calls_execute_plan_with_toggled_plan > rebuilds on toggle and executes the plan currently shown
VT-C889F6701836|-|A|L141|stale_plan_refresh_requires_renewed_confirmation > preserves non-default toggles and executes exactly the refreshed plan only after a second click
VT-6DCCCFDD046C|-|A|L198|stale_plan_refresh_requires_renewed_confirmation > does not rebuild or set dialog state when a stale execution returns after Cancel unmounts it
VT-0B35713C1EB0|-|A|L229|stale_plan_refresh_requires_renewed_confirmation > clears selections when a successful execution returns after Cancel unmounts it
VT-03DFF89FD48C|-|A|L287|plan_rebuild_readiness_gate > disables toggles and Upgrade while a rebuild is pending
VT-9C3238B4ACF6|-|A|L310|plan_rebuild_readiness_gate > keeps execution blocked after failure and makes Refresh plan rebuild only
VT-C6AF1588FD74|-|A|L343|plan_rebuild_readiness_gate > requires a fresh review after a non-stale execute error before allowing another click
VT-30AA59CD5E67|-|A|L389|plan_rebuild_readiness_gate > ignores an older rebuild response that arrives after the latest request
VT-5508460FBAF1|-|A|L453|all_packages_plan_request > preserves selection null when a toggle rebuilds the plan
VT-37110731A7E1|-|A|L477|all_packages_plan_request > preserves selection null through stale recovery and a manual retry
VT-049B769969DE|-|A|L541|upgrade_selected_dispatches_exact_ids_then_clears_on_success > builds a plan with exactly the checked ids, executes it, and clears selection

[src/components/manager/selfUpdate.test.tsx|Component|3]
VT-FB32EB19B2D9|-|A|L33|self_update_card_shows_routed_subtitle_and_queued_behind_executor > renders the routed command + why, and 'Queued behind Homebrew' when brew is busy
VT-43F45775CAC3|-|A|L46|npm_card_shows_mise_reset_note > shows the permanent mise-reset note on the npm self-update card
VT-E25E87F0D070|-|A|L56|self_update_disabled_when_executor_absent > disables the Update button when the routed executor is not installed

[src/components/manager/versionDelta.test.tsx|Unit|7]
VT-1AB93F38A393|-|A|L10|versionDelta > patch: last segment differs → patch, prefix + changed suffix
VT-97F16E975F77|-|A|L18|versionDelta > major: first segment differs → major, whole latest is the suffix
VT-C12D6FECB202|-|A|L26|versionDelta > minor: second segment differs → minor, separators preserved
VT-6343B1AEC827|-|A|L34|versionDelta > shorter installed than latest → patch on the added segment
VT-D64F9B3DCFA4|-|A|L42|versionDelta > plain: identical versions are not a delta
VT-5063C3521EF7|-|A|L46|versionDelta > plain: non-numeric changed segment never fabricates a delta
VT-FC31DA304745|-|A|L51|versionDelta > unknown: missing latest is not a delta

[src/components/manager/versionDelta.test.tsx|Component|4]
VT-E4A78A1F6B75|-|A|L58|version_delta_highlights_only_changed_segments > highlights only the changed patch segment
VT-7F8D454AE0AE|-|A|L67|version_delta_highlights_only_changed_segments > highlights the whole latest for a major bump
VT-B90D6952476E|-|A|L75|version_delta_highlights_only_changed_segments > renders plain (no highlight) for an up-to-date, non-comparable pair
VT-08C8C11CBEDE|-|A|L81|version_delta_highlights_only_changed_segments > shows 'update available' instead of a fabricated delta when latest is unknown

[src/components/shell/appUpdate.test.tsx|Component|10]
VT-86EA58C74180|-|A|L48|update_status_item_hidden_when_idle_and_up_to_date > renders nothing before a check, while checking, or when already current
VT-7BE42CA3CF03|-|A|L63|update_status_item_hidden_when_idle_and_up_to_date > keeps a failed check out of the chrome — offline must not leave a permanent warning
VT-DC56F3812BE7|-|A|L73|downloading_state_renders_progress > shows a percentage when the server sent a content length
VT-13E498B76945|-|A|L81|downloading_state_renders_progress > degrades to an indeterminate label rather than fabricating a percentage
VT-B68FA172C6B3|-|A|L93|ready_state_renders_restart_button_and_invokes_install > installs immediately when nothing is running
VT-EC68A6FD5550|-|A|L108|restart_with_running_ops_opens_quit_guard > confirms before killing in-flight operations, then installs on confirm
VT-FFEF0BFA11FC|-|A|L135|manual_install_required_is_surfaced_instead_of_prompting_for_a_password > explains why, rather than silently doing nothing
VT-A961AB9662D7|-|A|L151|manual_check_up_to_date_pushes_info_toast_automatic_does_not > stays silent for the 6h timer and speaks up for a menu-bar check
VT-70B9633858FB|-|A|L162|manual_check_up_to_date_pushes_info_toast_automatic_does_not > toasts a manual failure persistently but never an automatic one
VT-83665ECB3E8D|-|A|L174|manual_check_up_to_date_pushes_info_toast_automatic_does_not > does not re-toast when an identical manual result arrives twice

[src/lib/errors.test.ts|Unit|6]
VT-CFD5BFDCFBDD|-|A|L7|describeError > preserves the IpcError payload instead of '[object Object]'
VT-B7A6C352EBCE|-|A|L16|describeError > includes detail/manager/opId/logPath when present
VT-B93F5CB98D92|-|A|L33|describeError > keeps an Error's message (and stack when available)
VT-2EBD73226C0F|-|A|L39|describeError > JSON-serializes unknown plain values
VT-71A0D117D102|-|A|L44|describeError > falls back to String() for unserializable values
VT-02ADDEF170DE|-|A|L54|plan_stale copy > tells the user to review and confirm the refreshed plan

[src/lib/ipc/types.test.ts|Unit|16]
VT-96A9AA0A8B9D|-|A|L56|ipc_types_accept_contract_fixtures > covers exactly the committed fixture set
VT-564929CE1DDC|-|A|L62|ipc_types_accept_contract_fixtures > app_state.json passes isAppState
VT-5017E5EC65E2|-|A|L62|ipc_types_accept_contract_fixtures > detection_report.json passes isDetectionReport
VT-A8C30140E329|-|A|L62|ipc_types_accept_contract_fixtures > event_app_update_status.json passes isAppUpdateStatus
VT-BF0D72BBFBC8|-|A|L62|ipc_types_accept_contract_fixtures > event_op_output.json passes isOpOutputEvent
VT-C7D10C433C04|-|A|L62|ipc_types_accept_contract_fixtures > event_op_stalled.json passes isOpStalledEvent
VT-E37B59DFCAD8|-|A|L62|ipc_types_accept_contract_fixtures > event_op_status.json passes isOpStatusEvent
VT-B5340CE16EB0|-|A|L62|ipc_types_accept_contract_fixtures > event_snapshot_updated.json passes isSnapshotUpdatedEvent
VT-AADA9CCA52C0|-|A|L62|ipc_types_accept_contract_fixtures > ipc_error.json passes isIpcError
VT-DBA1B42643F9|-|A|L62|ipc_types_accept_contract_fixtures > manager_snapshot.json passes isManagerSnapshot
VT-5C72E2BD08BE|-|A|L62|ipc_types_accept_contract_fixtures > op_ref.json passes isOpRef
VT-C06FAA8EC610|-|A|L62|ipc_types_accept_contract_fixtures > operation_detail.json passes isOperationDetail
VT-346C179A228E|-|A|L62|ipc_types_accept_contract_fixtures > operation_record.json passes isOperationRecord
VT-D47E3BC19EA3|-|A|L62|ipc_types_accept_contract_fixtures > plan_request.json passes isPlanRequest
VT-F3C6C34E3B71|-|A|L62|ipc_types_accept_contract_fixtures > settings.json passes isSettings
VT-DDD19B28C9F5|-|A|L62|ipc_types_accept_contract_fixtures > upgrade_plan.json passes isUpgradePlan

[tests/support/fixtures/factories/pack-manager.test.ts|Unit|2]
VT-CBC1FBC10DD1|P0|A|L6|package factory outdated verdict > [P0] does not infer outdatedness from differing version strings
VT-B8086BC1EC9F|P0|A|L17|package factory outdated verdict > [P0] preserves an explicit package-manager verdict

<!-- END VITEST CATALOG -->
```

#### Rust Catalog

Each group header is `[file|level|count]`; each row is `id|priority|state|line|exact Cargo title`.

Ignored reasons:

- Seven `process/runner.rs` cases are developer-run real-process/signal checks.
- Two `paths.rs` cases depend on this machine's real login/noisy shell behavior.
- Two `live_smoke.rs` cases execute real package-manager commands and are intentionally excluded from the default suite and CI.

```text
[src-tauri/src/app_update.rs|Unit|10]
RS-3A2F9E33|-|A|L463|app_update::tests::starts_idle_with_the_running_version
RS-E268B5B4|-|A|L473|app_update::tests::up_to_date_check_emits_checking_then_up_to_date
RS-AF81BCD5|-|A|L490|app_update::tests::failed_check_lands_in_error_with_the_message
RS-CC37163C|-|A|L510|app_update::tests::found_update_downloads_automatically_and_reports_progress
RS-37C1FD08|-|A|L560|app_update::tests::download_failure_lands_in_error_and_leaves_nothing_to_install
RS-1C2922C1|-|A|L580|app_update::tests::install_without_a_download_is_refused
RS-FE13F171|-|A|L589|app_update::tests::install_applies_the_downloaded_bytes
RS-88B37111|-|A|L601|app_update::tests::install_failure_surfaces_as_error_state
RS-E68B6612|-|A|L618|app_update::tests::bundle_root_finds_the_dot_app_ancestor
RS-2495AC18|-|A|L628|app_update::tests::bundle_root_outside_a_bundle_is_the_executables_directory

[src-tauri/src/commands.rs|Unit|19]
RS-BED9FE6C|-|A|L841|commands::tests::placeholder_report_carries_env_info
RS-69248718|-|A|L986|commands::tests::refresh_all_re_detects_and_includes_newly_installed_manager
RS-8B5400EB|-|A|L1047|commands::tests::refresh_all_re_detects_and_drops_removed_manager
RS-01A86540|-|A|L1095|commands::tests::refresh_manager_absent_in_cache_re_probes_then_submits
RS-9C43A196|-|A|L1289|commands::tests::issued_plan_executes_once_and_replay_submits_nothing
RS-6F4BF445|-|A|L1313|commands::tests::round_tripped_multi_group_plan_executes_routed_self_update_structurally
RS-B76051F9|-|A|L1376|commands::tests::later_group_missing_backend_argv_submits_no_partial_batch
RS-C9A0C9E1|-|A|L1421|commands::tests::every_round_tripped_plan_section_is_authenticated
RS-EF63F385|-|A|L1471|commands::tests::snapshot_drift_consumes_plan_and_submits_nothing
RS-39CA7FD5|-|A|L1490|commands::tests::issued_plan_echoes_deduplicated_request_and_rejects_oversized_selection
RS-E13303EA|-|A|L1532|commands::tests::redetection_revision_drift_consumes_plan_and_submits_nothing
RS-E1F18256|-|A|L1545|commands::tests::cancelled_pending_redetection_releases_revision_barrier
RS-2D8E12FF|-|A|L1584|commands::tests::plan_issued_during_active_refresh_is_rejected_without_upgrade
RS-E167FD26|-|A|L1634|commands::tests::plan_cannot_queue_behind_earlier_direct_mutation_on_same_lock
RS-EA358FC4|-|A|L1683|commands::tests::settings_change_after_issue_invalidates_plan_without_submission
RS-0B4177EB|-|A|L1708|commands::tests::failed_settings_persistence_changes_neither_memory_nor_revision
RS-71C691A0|-|A|L1734|commands::tests::oldest_plan_is_stale_after_bounded_cache_eviction
RS-2AC71F0E|-|A|L1745|commands::tests::overlapping_prebuilt_plans_serialize_validation_through_enqueue
RS-3E48ED57|-|A|L1789|commands::tests::fast_terminal_first_plan_still_invalidates_second_prebuilt_plan

[src-tauri/src/detect.rs|Unit|16]
RS-8662A9F0|-|A|L490|detect::tests::classify_mise_shim_path_is_mise_managed_without_canonicalizing
RS-2C422DCA|-|A|L519|detect::tests::classify_mise_installs_dir_is_mise_managed
RS-7267F9DC|-|A|L528|detect::tests::classify_opt_homebrew_canonical_is_brew
RS-7D97368B|-|A|L535|detect::tests::classify_cargo_bin_is_rustup
RS-005ED1F9|-|A|L542|detect::tests::classify_brew_itself_is_standalone
RS-EF78FFFC|-|A|L561|detect::tests::classify_unknown_is_standalone
RS-D11825C6|-|A|L571|detect::tests::classify_standalone_uv_in_local_bin_routes_in_band
RS-326828F3|-|A|L589|detect::tests::npm_in_band_override_wins_over_delegation
RS-07C3D1C3|-|A|L610|detect::tests::mise_routes_via_brew_when_brew_detected
RS-F6930B61|-|A|L626|detect::tests::mise_falls_through_to_in_band_when_brew_absent
RS-666EF700|-|A|L719|detect::tests::recheck_flips_npm_to_in_band_when_own_listing_reports_it_outdated
RS-62CB3B9C|-|A|L737|detect::tests::recheck_keeps_delegation_when_own_listing_is_clean_and_flips_back
RS-5BB2F871|-|A|L782|detect::tests::detect_all_classifies_routes_and_never_probes_absent_mas
RS-5A608597|-|A|L898|detect::tests::rustup_prefers_own_tree_binary_over_mise_shim
RS-DCA6ABB6|-|A|L963|detect::tests::probe_failure_keeps_manager_present_without_version
RS-5F104380|-|A|L997|detect::tests::extract_version_takes_first_numeric_token

[src-tauri/src/diagnostics.rs|Unit|4]
RS-A5E35D73|-|A|L190|diagnostics::tests::export_bundles_report_logs_transcripts_and_journal
RS-E52F713F|-|A|L274|diagnostics::tests::export_never_follows_symlinks_into_the_bundle
RS-9626DEFE|-|A|L338|diagnostics::tests::export_streams_file_bodies_byte_identically
RS-13DC43D9|-|A|L368|diagnostics::tests::export_tolerates_missing_sources

[src-tauri/src/error.rs|Unit|3]
RS-59ED297D|-|A|L188|error::tests::every_pm_error_variant_maps_to_its_wire_code
RS-9DED8655|-|A|L265|error::tests::brew_lock_busy_message_is_the_actionable_copy
RS-753A2385|-|A|L277|error::tests::plan_stale_uses_the_stable_snake_case_wire_code

[src-tauri/src/events.rs|Unit|7]
RS-4E79B6FD|-|A|L305|events::tests::vec_sink_records_in_order_and_names_are_stable
RS-ACDD4DA8|-|A|L323|events::tests::payload_json_uses_camel_case_fields
RS-EC601DD8|-|A|L358|events::tests::batch_500_lines_flush_in_le_64_line_batches_at_ge_50ms
RS-91EF42D5|-|A|L382|events::tests::batch_force_flushes_at_8kib
RS-18AEB1C1|-|A|L399|events::tests::batch_keeps_ops_separate_and_timer_flushes_each
RS-9CCF6C2E|-|A|L415|events::tests::batch_flush_op_flushes_immediately
RS-C95FE1B0|-|A|L428|events::tests::batch_drop_flushes_remaining_and_line_sink_feeds_batches

[src-tauri/src/ipc.rs|Unit|1]
RS-51C5E482|-|A|L788|ipc::tests::ipc_contract_matches_committed_fixtures

[src-tauri/src/journal.rs|Unit|8]
RS-9EAE1F4F|-|A|L258|journal::tests::journal_start_finish_roundtrip
RS-78DB125D|-|A|L280|journal::tests::start_only_record_surfaces_interrupted
RS-D4D62879|-|A|L294|journal::tests::journal_lines_use_camel_case_wire_fields
RS-2AD86557|-|A|L311|journal::tests::garbage_lines_and_orphan_finishes_are_skipped
RS-8D1BFC3D|-|A|L333|journal::tests::compact_keeps_newest_operations_with_their_finishes
RS-AE83619C|-|A|L363|journal::tests::compact_failure_leaves_the_original_journal_intact
RS-0B69DEC3|-|A|L391|journal::tests::compact_leaves_no_temp_file_behind
RS-74469A1C|-|A|L407|journal::tests::compact_is_a_noop_under_the_cap_and_for_missing_files

[src-tauri/src/logging.rs|Unit|6]
RS-A6775BD8|-|A|L247|logging::tests::directive_for_level_targets_own_crate
RS-0CC325E0|-|A|L260|logging::tests::resolve_directive_env_beats_settings_beats_default
RS-ABA8D593|-|A|L276|logging::tests::filename_date_parsers_are_strict
RS-8C017A7E|-|A|L295|logging::tests::prune_app_logs_removes_only_old_matching_files
RS-5CEDCD51|-|A|L312|logging::tests::prune_transcripts_by_age_then_newest_200
RS-AE439B98|-|A|L345|logging::tests::init_at_writes_json_log_and_reload_respects_env_override

[src-tauri/src/managers/brew.rs|Unit|10]
RS-65AF5E49|-|A|L361|managers::brew::tests::refresh_plan_starts_with_brew_update_and_gates_on_setting
RS-520314F1|-|A|L390|managers::brew::tests::brew_refresh_sequence_emits_phase_labels_and_parses_snapshot
RS-78801B89|-|A|L460|managers::brew::tests::refresh_timeout_propagates_as_error_for_an_error_snapshot
RS-E3B46A7E|-|A|L485|managers::brew::tests::greedy_only_casks_are_rekinded_in_the_merged_table
RS-FA2FB2FC|-|A|L537|managers::brew::tests::recovery_plan_wires_greedy_text_for_outdated_json_only
RS-CDCC6B5F|-|A|L554|managers::brew::tests::parse_recovery_merges_full_inventory_with_recovered_overlay
RS-FD1ABE94|-|A|L614|managers::brew::tests::brew_update_non_zero_is_expected_never_aborts_refresh
RS-6E111BDB|-|A|L651|managers::brew::tests::upgrade_plan_splits_by_kind_and_omits_empty
RS-AF59CCD5|-|A|L682|managers::brew::tests::brew_lock_stderr_maps_to_brew_lock_busy
RS-AFA1692A|-|A|L706|managers::brew::tests::absent_brew_plans_nothing

[src-tauri/src/managers/mas.rs|Unit|5]
RS-C7E7A4E7|-|A|L160|managers::mas::tests::absent_mas_plans_nothing_so_runner_is_never_called
RS-ADE81DCD|-|A|L170|managers::mas::tests::refresh_merges_list_and_outdated
RS-CFD56541|-|A|L207|managers::mas::tests::shell_error_degrades_to_parse_failed_never_a_crash
RS-E79E2F31|-|A|L216|managers::mas::tests::upgrade_plan_uses_numeric_ids
RS-4922928C|-|A|L226|managers::mas::tests::route_is_unavailable_unless_brew_managed

[src-tauri/src/managers/mise.rs|Unit|6]
RS-41B57E25|-|A|L207|managers::mise::tests::refresh_merges_inventory_with_clean_outdated
RS-B3ECE21E|-|A|L234|managers::mise::tests::refresh_overlay_marks_outdated_rows
RS-E39B82EC|-|A|L254|managers::mise::tests::recovery_fallback_runs_text_parser_on_bad_outdated_json
RS-492625BE|-|A|L313|managers::mise::tests::upgrade_plan_is_one_command_with_verbatim_names
RS-D41700C5|-|A|L340|managers::mise::tests::route_is_brew_when_brew_managed_else_in_band
RS-E7F8CAE4|-|A|L362|managers::mise::tests::absent_mise_plans_nothing

[src-tauri/src/managers/npm.rs|Unit|6]
RS-93B665FF|-|A|L244|managers::npm::tests::npm_exit_1_with_json_is_success
RS-C900B396|-|A|L290|managers::npm::tests::npm_exit_1_with_garbage_is_parse_failed
RS-4E9DA206|-|A|L306|managers::npm::tests::non_json_command_exit_1_is_failure
RS-4C106E06|-|A|L325|managers::npm::tests::recovery_runs_text_fallback_and_still_hoists_npm
RS-8D297E8A|-|A|L369|managers::npm::tests::upgrade_plan_is_one_install_g_command_with_latest_suffix
RS-8BE53F6D|-|A|L383|managers::npm::tests::in_band_override_wins_when_npm_reports_itself_outdated

[src-tauri/src/managers/parse/brew.rs|Unit|7]
RS-FABCB926|-|A|L215|managers::parse::brew::tests::brew_outdated_json_skips_leading_junk_line
RS-799E95A3|-|A|L229|managers::parse::brew::tests::brew_outdated_clean_json_parses_without_junk
RS-D3A41F5E|-|A|L238|managers::parse::brew::tests::brew_greedy_only_is_set_difference
RS-07F29DBB|-|A|L268|managers::parse::brew::tests::brew_text_where_json_expected_is_parse_failed_with_excerpt
RS-406CA698|-|A|L280|managers::parse::brew::tests::brew_list_versions_parses_inventory_and_last_version_wins
RS-65FBE502|-|A|L296|managers::parse::brew::tests::brew_cask_versions_parses_comma_version
RS-8F522B7D|-|A|L305|managers::parse::brew::tests::brew_formula_inventory_deduped_against_casks_is_243

[src-tauri/src/managers/parse/mas.rs|Unit|3]
RS-44E46A98|-|A|L107|managers::parse::mas::tests::mas_outdated_parses_real_capture
RS-6C25C101|-|A|L134|managers::parse::mas::tests::mas_list_parses_real_capture
RS-DE384A3B|-|A|L152|managers::parse::mas::tests::mas_shell_error_never_reaches_parser

[src-tauri/src/managers/parse/mise.rs|Unit|5]
RS-23A7691B|-|A|L168|managers::parse::mise::tests::mise_outdated_json_empty_object_means_clean
RS-78787BC3|-|A|L177|managers::parse::mise::tests::mise_outdated_text_parses_seven_rows_six_outdated
RS-FB26E133|-|A|L200|managers::parse::mise::tests::mise_outdated_json_populated_synthetic
RS-5B5C426C|-|A|L215|managers::parse::mise::tests::mise_ls_json_parses_eleven_tools_active_version
RS-E9D94548|-|A|L232|managers::parse::mise::tests::mise_ls_json_garbage_is_parse_failed_not_panic

[src-tauri/src/managers/parse/mod.rs|Unit|4]
RS-E77211A2|-|A|L171|managers::parse::tests::merge_patches_inventory_and_appends_overlay_only
RS-DDEC6787|-|A|L199|managers::parse::tests::extract_self_hoists_named_row_into_self_status
RS-9CE8E7B9|-|A|L214|managers::parse::tests::dedupe_removes_casks_from_formula_inventory
RS-2E9B3621|-|A|L230|managers::parse::tests::package_ids_use_camel_case_kind_prefix

[src-tauri/src/managers/parse/npm.rs|Unit|4]
RS-8AF7CA07|-|A|L166|managers::parse::npm::tests::npm_outdated_json_empty_object_means_clean
RS-88DEC828|-|A|L174|managers::parse::npm::tests::npm_outdated_text_parses_five_rows_hoists_npm_self
RS-9092B9BF|-|A|L196|managers::parse::npm::tests::npm_outdated_json_populated_synthetic
RS-A265A174|-|A|L214|managers::parse::npm::tests::npm_ls_json_parses_fifteen_global_deps

[src-tauri/src/managers/parse/rustup.rs|Unit|4]
RS-46575376|-|A|L122|managers::parse::rustup::tests::rustup_check_outdated_yields_toolchain_and_self
RS-C2E5E6C1|-|A|L139|managers::parse::rustup::tests::rustup_check_tolerates_both_colon_spacings
RS-4C0A3E74|-|A|L155|managers::parse::rustup::tests::rustup_toolchain_list_parses_names
RS-DD62E6A1|-|A|L164|managers::parse::rustup::tests::rustup_check_garbage_is_parse_failed_not_panic

[src-tauri/src/managers/parse/uv.rs|Unit|7]
RS-7356C2DB|-|A|L183|managers::parse::uv::tests::uv_tool_list_extracts_broken_env_warning_and_fix_command
RS-79EDA349|-|A|L213|managers::parse::uv::tests::altered_uv_reinstall_suggestion_remains_visible_but_is_not_runnable
RS-DF193999|-|A|L235|managers::parse::uv::tests::uv_warning_without_reinstall_suggestion_remains_visible
RS-D44794C1|-|A|L249|managers::parse::uv::tests::uv_warning_with_malformed_reinstall_suggestion_remains_visible
RS-8E7D4F63|-|A|L262|managers::parse::uv::tests::uv_tool_list_clean_collects_executables
RS-B2471CDE|-|A|L294|managers::parse::uv::tests::uv_outdated_empty_output_is_clean_not_error
RS-F8BA1824|-|A|L300|managers::parse::uv::tests::uv_outdated_unknown_suffix_degrades_to_null_latest

[src-tauri/src/managers/rustup.rs|Unit|7]
RS-B8B1CB26|-|A|L179|managers::rustup::tests::refresh_yields_toolchain_rows_and_rustup_self_status
RS-4A078C46|-|A|L200|managers::rustup::tests::clean_check_with_both_colon_spacings_is_up_to_date
RS-86C15E25|-|A|L212|managers::rustup::tests::inventory_only_toolchain_is_appended_with_unknown_versions
RS-D937584D|-|A|L229|managers::rustup::tests::upgrade_plan_is_rustup_update_with_toolchains
RS-D21B7924|-|A|L239|managers::rustup::tests::route_is_in_band_self_update
RS-37C43307|-|A|L249|managers::rustup::tests::refresh_plan_is_toolchain_list_then_check
RS-293F2215|-|A|L257|managers::rustup::tests::absent_rustup_plans_nothing

[src-tauri/src/managers/uv.rs|Unit|5]
RS-48C881C0|-|A|L185|managers::uv::tests::warning_line_becomes_health_issue_and_report_is_ok
RS-F8EE7F6F|-|A|L220|managers::uv::tests::unknown_outdated_suffix_keeps_latest_null
RS-9C47B481|-|A|L245|managers::uv::tests::upgrade_plan_is_one_tool_upgrade_command
RS-AD738ADD|-|A|L258|managers::uv::tests::routes_follow_managed_by_dynamically
RS-5C1AF4B3|-|A|L280|managers::uv::tests::no_recovery_wired

[src-tauri/src/ops.rs|Unit|7]
RS-5A0E4C2D|-|A|L332|ops::tests::transcript_golden_format_matches_spec_6_2
RS-C905C1E3|-|A|L387|ops::tests::footer_records_signal_paths_for_cancel_and_timeout
RS-8F806038|-|A|L407|ops::tests::transcript_file_name_matches_the_spec_pattern
RS-B4F6DB92|-|A|L425|ops::tests::command_line_joins_serial_specs
RS-0238187E|-|A|L441|ops::tests::transcript_writer_is_line_flushed_on_disk
RS-95CF0E68|-|A|L456|ops::tests::op_kind_maps_to_wire_and_carries_package_ids
RS-A2BB1DF5|-|A|L474|ops::tests::env_set_excludes_path

[src-tauri/src/paths.rs|Unit|13]
RS-7A1FAD0A|-|A|L324|paths::tests::sentinel_extraction_ignores_profile_noise
RS-5650740B|-|A|L335|paths::tests::sentinel_extraction_uses_last_start_marker
RS-A8EDD8EE|-|A|L342|paths::tests::sentinel_extraction_missing_markers_is_none
RS-67923087|-|A|L349|paths::tests::split_path_drops_empty_segments
RS-D4C9BAFD|-|A|L357|paths::tests::probe_failure_falls_back_to_static_with_shims_before_homebrew
RS-C59918D8|-|A|L387|paths::tests::merge_dedupes_preserving_order
RS-9D66102A|-|A|L402|paths::tests::probe_success_merges_with_source_merged
RS-AAFD4E3A|-|A|L414|paths::tests::child_env_is_constructed_not_inherited
RS-CB9C840F|-|A|L439|paths::tests::which_resolves_only_on_our_entries_and_require_reports_searched
RS-D98BFE73|-|A|L463|paths::tests::env_info_mirrors_the_wire_shape
RS-1E907455|-|A|L477|paths::tests::probe_timeout_kills_profile_spawned_descendants
RS-E3306255|-|I|L522|paths::tests::real_login_shell_probe_returns_entries
RS-7D301174|-|I|L532|paths::tests::real_probe_times_out_against_sleeping_shell

[src-tauri/src/process/fake.rs|Unit|10]
RS-A6AD0EAF|-|A|L565|process::fake::tests::fake_matches_on_basename_and_args_and_records_calls
RS-72D5DA5F|-|A|L591|process::fake::tests::fake_pops_successive_responses_then_repeats_last
RS-514590CA|-|A|L608|process::fake::tests::fake_fail_returns_the_scripted_error
RS-B47C9A0C|-|A|L633|process::fake::tests::fake_panics_on_unmatched
RS-07971583|-|A|L645|process::fake::tests::fake_fixture_loads_from_dev_fixtures
RS-7FEC088E|-|A|L663|process::fake::tests::fake_buffered_gate_cancel_returns_cancelled
RS-031ECAC3|-|A|L683|process::fake::tests::fake_streaming_emits_lines_and_waits_gate
RS-FB99E4D3|-|A|L729|process::fake::tests::fake_streaming_cancel_while_gated_returns_cancelled
RS-A968FE76|-|A|L754|process::fake::tests::fake_streaming_hard_cap_times_out_under_paused_time
RS-A72A4272|-|A|L783|process::fake::tests::fake_streaming_stall_notify_fires_and_rearms_on_output

[src-tauri/src/process/runner.rs|Unit|16]
RS-08A335D1|-|A|L437|process::runner::tests::strip_ansi_removes_csi_and_osc_sequences
RS-3322711C|-|A|L448|process::runner::tests::split_output_line_splits_carriage_return_repaints
RS-85A6D7EC|-|A|L462|process::runner::tests::split_output_line_breaks_after_an_unterminated_notice
RS-6B48FDD0|-|A|L478|process::runner::tests::split_output_line_leaves_a_terminated_notice_alone
RS-9A0CC26A|-|A|L507|process::runner::tests::split_output_line_is_lossy_on_invalid_utf8
RS-5E6A517B|-|A|L515|process::runner::tests::append_capped_stops_at_cap
RS-D58A912F|-|A|L529|process::runner::tests::phase_of_is_program_basename_plus_args
RS-22B0AB57|-|I|L557|process::runner::tests::real_spawns_captures_streams_and_exit_code
RS-5CB292C9|-|I|L582|process::runner::tests::real_splits_carriage_return_progress
RS-E2BA915B|-|I|L604|process::runner::tests::real_cancel_sigterm_kills_group
RS-5089E5C4|-|A|L631|process::runner::tests::real_cancel_escalates_to_sigkill_when_term_ignored
RS-6B3B1190|-|A|L659|process::runner::tests::real_completes_when_descendant_holds_pipes_open_after_exit
RS-AAFF7A40|-|I|L684|process::runner::tests::real_absolute_timeout_kills_and_reports
RS-C1402D8C|-|I|L702|process::runner::tests::real_stall_fires_at_threshold_and_rearms_on_output
RS-D083132F|-|I|L732|process::runner::tests::real_stall_hard_cap_times_out
RS-1BBE7C7A|-|I|L750|process::runner::tests::real_child_env_is_exactly_the_spec_env

[src-tauri/src/queue.rs|Unit|45]
RS-1ED7EB86|-|A|L2059|queue::tests::two_brew_ops_never_overlap_fifo
RS-06E072EE|-|A|L2110|queue::tests::brew_and_mise_run_concurrently
RS-6D78F285|-|A|L2156|queue::tests::routed_self_update_blocks_subject_lane
RS-FAC3A829|-|A|L2224|queue::tests::npm_op_blocks_mise_when_mise_managed
RS-758A8E10|-|A|L2275|queue::tests::skip_ahead_starts_unblocked_op
RS-D3240544|-|A|L2329|queue::tests::aging_guard_blocks_skip_ahead_after_120s
RS-F6C09169|-|A|L2398|queue::tests::semaphore_caps_concurrency_at_4
RS-FD80C771|-|A|L2440|queue::tests::duplicate_refresh_coalesces_to_same_opid
RS-F05D08B9|-|A|L2494|queue::tests::plan_batch_revision_mismatch_enqueues_all_or_none
RS-48FB7403|-|A|L2528|queue::tests::plan_batch_rejects_existing_mutation_with_intersecting_locks
RS-8E0C0CDF|-|A|L2575|queue::tests::plan_batch_rejects_running_self_update_with_intersecting_locks
RS-6D29D3A5|-|A|L2626|queue::tests::plan_batch_rejects_queued_health_fix_with_intersecting_locks
RS-B423E58A|-|A|L2695|queue::tests::npm_exit_1_with_json_is_success
RS-807C4347|-|A|L2729|queue::tests::npm_exit_1_with_garbage_is_parse_failed
RS-8170BE50|-|A|L2760|queue::tests::brew_lock_stderr_maps_to_brew_lock_busy
RS-182BFE67|-|A|L2791|queue::tests::recovery_plan_runs_text_fallback_on_json_parse_failure
RS-64840CF5|-|A|L2824|queue::tests::failure_aborts_remaining_specs
RS-C68B43C0|-|A|L2864|queue::tests::successful_upgrade_auto_enqueues_refresh
RS-4F174F00|-|A|L2926|queue::tests::cancel_sigterm_then_sigkill_marks_cancelled_finalizes_transcript
RS-36D258AE|-|A|L2968|queue::tests::cancel_running_refresh_marks_cancelled
RS-883BE45C|-|A|L3008|queue::tests::cancel_all_then_wait_until_idle_reaps_running_refresh
RS-A3EA73EE|-|A|L3040|queue::tests::brew_update_failure_degrades_to_local_snapshot
RS-470E77B7|-|A|L3078|queue::tests::refresh_invokes_route_recheck_with_parsed_snapshot
RS-14B56CEC|-|A|L3108|queue::tests::stall_fires_at_threshold_and_rearms_on_output
RS-D91D876D|-|A|L3154|queue::tests::hard_cap_times_out
RS-7FF9EB68|-|A|L3189|queue::tests::queued_op_cancel_removes_it_without_journal_lines
RS-06527E91|-|A|L3231|queue::tests::refresh_phase_labels_ride_op_status_events
RS-C67B7FC7|-|A|L3283|queue::tests::routed_self_update_holds_both_locks
RS-ED775BD7|-|A|L3292|queue::tests::npm_upgrade_with_mise_managed_node_locks_npm_and_mise
RS-4788C05B|-|A|L3510|queue::tests::plan_builder_previews_exact_argv_and_excludes_pinned_and_greedy
RS-F01E4929|-|A|L3561|queue::tests::plan_builder_greedy_opt_in_includes_the_cask
RS-E4C4984A|-|A|L3588|queue::tests::plan_builder_rust_dedup_drops_mise_rust_with_note
RS-BEC0F59B|-|A|L3641|queue::tests::canonical_request_deduplicates_before_rust_dedup_and_argv_planning
RS-3D1E9864|-|A|L3725|queue::tests::canonical_request_rejects_selection_and_package_id_limits
RS-B362A21F|-|A|L3752|queue::tests::plan_builder_selection_seeds_exact_ids
RS-3562B0C5|-|A|L3785|queue::tests::plan_builder_includes_routed_self_update_group_with_dual_locks
RS-65C62A35|-|A|L3831|queue::tests::plan_builder_warns_on_stale_manager_and_excludes_already_running
RS-73D9C15F|-|A|L3863|queue::tests::busy_and_stale_derivations_read_session_records
RS-4A595A5C|-|A|L3913|queue::tests::bind_commands_rebinds_stall_timeouts_to_settings
RS-0EF3D23A|-|A|L3968|queue::tests::self_update_submission_binds_routed_and_in_band_routes
RS-924BDD60|-|A|L4042|queue::tests::self_update_submission_preserves_structured_argument_boundaries
RS-B366B133|-|A|L4076|queue::tests::self_update_submission_fails_closed_without_matching_backend_argv
RS-068F52F6|-|A|L4122|queue::tests::health_fix_submission_binds_the_fix_command
RS-2D944CED|-|A|L4162|queue::tests::health_fix_submission_fails_closed_for_untrusted_or_mismatched_argv
RS-6974AB20|-|A|L4208|queue::tests::refresh_submission_binds_plan_env_and_meta

[src-tauri/src/registry.rs|Unit|7]
RS-997071F8|-|A|L175|registry::tests::cross_join_brew_formula_mise_patches_mise_self_status
RS-C61898A2|-|A|L206|registry::tests::cross_join_uv_from_mise_tool_uv_fixture_values
RS-F8D20DCC|-|A|L235|registry::tests::join_preserves_subjects_own_installed_version
RS-22457BC2|-|A|L265|registry::tests::upsert_without_matching_row_or_route_changes_only_itself
RS-A8300D77|-|A|L274|registry::tests::set_routes_from_extracts_routed_pairs
RS-BA32ACFF|-|A|L330|registry::tests::pkg_name_splits_on_first_colon_only
RS-DF991F6F|-|A|L337|registry::tests::all_returns_manager_id_order

[src-tauri/src/settings.rs|Unit|8]
RS-E1A89175|-|A|L188|settings::tests::defaults_match_spec_f11
RS-A0FC60CA|-|A|L200|settings::tests::save_then_load_round_trips
RS-5175C41E|-|A|L213|settings::tests::load_missing_or_corrupt_file_yields_defaults
RS-AF9E386E|-|A|L227|settings::tests::load_from_reporting_flags_corruption_but_not_missing_files
RS-E2E70539|-|A|L249|settings::tests::failed_save_leaves_existing_settings_intact
RS-9D72A9C5|-|A|L274|settings::tests::load_partial_file_fills_missing_fields_from_defaults
RS-C8508175|-|A|L285|settings::tests::apply_patch_touches_only_present_fields
RS-8F09DB39|-|A|L302|settings::tests::patch_deserializes_camel_case_partial_json

[src-tauri/src/state.rs|Unit|1]
RS-CAEF32DF|-|A|L415|state::tests::issued_plan_store_is_bounded_oldest_first_and_one_use

[src-tauri/tests/live_smoke.rs|E2E|2]
RS-A0098542|-|I|L28|live_detection_classifies_this_machine
RS-C8F379C5|-|I|L138|live_brew_outdated_json_round_trips

<!-- END RUST CATALOG -->
```

#### Step 2 Outcome

The current checkout has a large deterministic lower-level suite and six logical browser cases. The new Playwright tests close the previously recorded absence of browser automation for selected-package and row-level upgrade journeys, but they do not erase the native Tauri transport, packaged-app, release-signing, or P1 feature gaps. The catalog is complete enough to map atomic criteria without reusing the stale matrix's counts.

## Step 3: Requirement-to-Test Traceability

### Mapping Rules

- The matrix retains the 80 normalized acceptance criteria from the resolved SPEC/DECISIONS oracle: 72 P0 and 8 P1.
- FULL means the criterion has sufficient executable evidence at the appropriate level; PARTIAL means material behavior or a boundary is still missing; UNIT-ONLY and INTEGRATION-ONLY identify a level gap; NONE means no qualifying executable evidence.
- Every mapped ID resolves to the Step 2 catalog, which preserves the test title, file, declaration line, level, active/ignored state, and explicit priority flag. This relational reference avoids duplicating or drifting those stable identity fields.
- Reuse across rows is intentional when one test protects a shared invariant. Coverage totals deduplicate stable test IDs before export.
- Browser cases run the real React UI in Chromium/WebKit but use fake Tauri IPC. They are not evidence of native JavaScript-to-Rust transport or packaged WKWebView behavior.

### Coverage Summary

| Status           | Criteria | Meaning                                             |
| ---------------- | -------: | --------------------------------------------------- |
| FULL             |       14 | Sufficient evidence at the appropriate level        |
| PARTIAL          |       38 | Meaningful evidence with a behavior or boundary gap |
| UNIT-ONLY        |       16 | Only unit-level evidence                            |
| INTEGRATION-ONLY |        2 | Only component/workflow integration evidence        |
| NONE             |       10 | No qualifying executable evidence                   |
| **Total**        |   **80** | **238 unique mapped tests**                         |

- **Strict full coverage:** 14/80 (17.5%)
- **P0 strict full coverage:** 14/72 (19.4%)
- **P1 strict full coverage:** 0/8 (0.0%)

### Detailed Traceability Matrix

| Criterion | Priority | Coverage         | Atomic requirement                                                                                                                             | Mapped stable test IDs                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   | Levels               | Validation                                                                                                                                                                                   |
| --------- | -------- | ---------------- | ---------------------------------------------------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | -------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| F1-AC1    | P0       | PARTIAL          | Startup detection yields entries for all six adapters.                                                                                         | <code>RS-5BB2F871</code><br><code>VT-274330D6FAC1</code><br><code>VT-3672CCB30270</code><br><code>VT-23DA5FD88FC4</code>                                                                                                                                                                                                                                                                                                                                                                                                                                 | Unit, Component      | Hermetic detection plus launch hydration/error paths; real native startup/Tauri boundary is untested.                                                                                        |
| F1-AC2    | P0       | PARTIAL          | Detection can rerun on demand from the required UI entry points.                                                                               | <code>RS-69248718</code><br><code>RS-8B5400EB</code><br><code>RS-01A86540</code><br><code>VT-33EC5E3AB248</code><br><code>VT-22082392E1E2</code><br><code>VT-0F530C5EB18D</code>                                                                                                                                                                                                                                                                                                                                                                         | Unit, Component      | Add/remove/still-absent and Settings paths covered; dashboard overflow entry and real transport are missing.                                                                                 |
| F1-AC3    | P0       | UNIT-ONLY        | ToolEnv resolves only from the constructed static/login-shell merged path with fallback.                                                       | <code>RS-D4C9BAFD</code><br><code>RS-9D66102A</code><br><code>RS-CB9C840F</code><br><code>RS-1E907455</code>                                                                                                                                                                                                                                                                                                                                                                                                                                             | Unit                 | Fallback, merge, restricted resolution, and timeout cleanup covered; real probes RS-147/RS-148 are ignored.                                                                                  |
| F1-AC4    | P0       | UNIT-ONLY        | Present managers are version-probed with timeout and probe failure does not become absence.                                                    | <code>RS-5BB2F871</code><br><code>RS-DCA6ABB6</code><br><code>RS-5F104380</code>                                                                                                                                                                                                                                                                                                                                                                                                                                                                         | Unit                 | Success, timeout, present-without-version, and extraction covered without real binaries or UI missing-version state.                                                                         |
| F1-AC5    | P0       | FULL             | managedBy uses raw mise paths before canonicalization and emits human-readable evidence.                                                       | <code>RS-8662A9F0</code><br><code>RS-7267F9DC</code><br><code>RS-7D97368B</code><br><code>RS-005ED1F9</code><br><code>RS-EF78FFFC</code><br><code>RS-5A608597</code><br><code>VT-D59F5E461AC7</code>                                                                                                                                                                                                                                                                                                                                                     | Unit, Component      | All classification families, precedence, fallback, and representative evidence rendering have active tests.                                                                                  |
| F1-AC6    | P0       | PARTIAL          | Absent managers are normal, are not run, and show Not installed plus install hint.                                                             | <code>RS-5BB2F871</code><br><code>RS-C7E7A4E7</code><br><code>VT-EEBA2D9C93AA</code><br><code>VT-B18DC3484045</code>                                                                                                                                                                                                                                                                                                                                                                                                                                     | Unit, Component      | Absence/no-run/card/disclosure content covered; muted styling and copy action are not asserted.                                                                                              |
| F1-AC7    | P0       | NONE             | Target-machine manager ownership matches the current required topology.                                                                        | —                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        | —                    | No active test proves the current-machine topology after D23a. RS-A0098542 is ignored and still asserts the now-false claim that mas is absent.                                              |
| F1-AC8    | P0       | PARTIAL          | Environment Report exposes ToolEnv source/path and evidence for every manager.                                                                 | <code>VT-D59F5E461AC7</code><br><code>RS-8662A9F0</code><br><code>RS-7267F9DC</code><br><code>RS-7D97368B</code><br><code>RS-005ED1F9</code><br><code>RS-EF78FFFC</code><br><code>RS-5A608597</code>                                                                                                                                                                                                                                                                                                                                                     | Component, Unit      | Representative report/evidence covered, not completeness across all six managers or clipboard behavior.                                                                                      |
| F2-AC1    | P0       | UNIT-ONLY        | Each present manager refresh runs inventory before outdated checks.                                                                            | <code>RS-65AF5E49</code><br><code>RS-ADE81DCD</code><br><code>RS-41B57E25</code><br><code>RS-B8B1CB26</code><br><code>RS-37C43307</code><br><code>RS-48C881C0</code><br><code>RS-470E77B7</code>                                                                                                                                                                                                                                                                                                                                                         | Unit                 | Adapter coverage is broad but command-order assertions are uneven and no UI/native integration covers all six.                                                                               |
| F2-AC2    | P0       | FULL             | Inventory and outdated overlay merge semantics preserve, patch, and append rows correctly.                                                     | <code>RS-E77211A2</code><br><code>RS-CDCC6B5F</code><br><code>RS-ADE81DCD</code><br><code>RS-B3ECE21E</code><br><code>RS-B8B1CB26</code>                                                                                                                                                                                                                                                                                                                                                                                                                 | Unit                 | Pure merge, recovery, clean/outdated, and overlay-only behavior are directly active-tested.                                                                                                  |
| F2-AC3    | P0       | UNIT-ONLY        | Brew refresh honors the update setting and emits required phase labels.                                                                        | <code>RS-65AF5E49</code><br><code>RS-520314F1</code><br><code>RS-06527E91</code><br><code>RS-A3EA73EE</code>                                                                                                                                                                                                                                                                                                                                                                                                                                             | Unit                 | Plan order, phases, parsing, and metadata failure degradation covered; visible phase labels are not component-tested.                                                                        |
| F2-AC4    | P0       | FULL             | Refresh All uses the fresh manager set and runs non-conflicting manager operations in parallel.                                                | <code>RS-69248718</code><br><code>RS-8B5400EB</code><br><code>RS-06E072EE</code><br><code>VT-33EC5E3AB248</code>                                                                                                                                                                                                                                                                                                                                                                                                                                         | Unit, Component      | Re-detection, fresh-set fan-out, disjoint-manager concurrency, conflict serialization, duplicate-refresh coalescing, and the visible transition are all exercised.                           |
| F2-AC5    | P0       | FULL             | Per-manager loading is isolated so populated and loading managers coexist.                                                                     | <code>VT-42F22198677A</code>                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             | Component            | Component assertion directly covers npm populated while brew remains a skeleton.                                                                                                             |
| F2-AC6    | P0       | PARTIAL          | Refresh operations enforce timeout/error behavior per manager.                                                                                 | <code>RS-78801B89</code><br><code>RS-D91D876D</code>                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     | Unit                 | Generic hard-cap behavior and brew timeout/error handling are covered, but there is no network/offline timeout matrix for every adapter.                                                     |
| F2-AC7    | P0       | FULL             | A manager failure retains stale data, shows actionable Retry, and does not blank other managers.                                               | <code>VT-F44143F3727F</code><br><code>VT-3FA738A696CA</code><br><code>VT-D8658959DCEF</code><br><code>RS-78801B89</code>                                                                                                                                                                                                                                                                                                                                                                                                                                 | Component, Unit      | Failure, stale retention, unaffected manager, Retry, and later error clearing are covered.                                                                                                   |
| F2-AC8    | P0       | PARTIAL          | Offline failures degrade independently without blanking the application.                                                                       | <code>RS-78801B89</code><br><code>RS-A3EA73EE</code><br><code>VT-F44143F3727F</code>                                                                                                                                                                                                                                                                                                                                                                                                                                                                     | Unit, Component      | One brew timeout plus isolated UI failure covered; no offline case for every network-dependent manager.                                                                                      |
| F2-AC9    | P0       | PARTIAL          | Successful upgrades automatically enqueue refresh for affected managers.                                                                       | <code>RS-C68B43C0</code>                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 | Unit                 | A successful same-subject upgrade queues refresh, but a routed self-update requiring both subject and executor refreshes is not directly asserted.                                           |
| F3-AC1    | P0       | PARTIAL          | Package table exposes required columns, statuses, selection, VersionDelta, and row Upgrade.                                                    | <code>VT-5301E7F83768</code><br><code>VT-16B9A758AFD2</code><br><code>VT-E4A78A1F6B75</code><br><code>PW-46F8A8F6</code>                                                                                                                                                                                                                                                                                                                                                                                                                                 | Component, E2E       | A real-browser case proves package-name filtering and row rendering; component tests cover selection and deltas. The complete column/status/verbatim matrix is not asserted together.        |
| F3-AC2    | P0       | PARTIAL          | uv rows expand to show executables.                                                                                                            | <code>RS-8E7D4F63</code><br><code>VT-F45377D5FFC4</code>                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 | Unit, Component      | Executable parsing and search covered; expansion interaction and chips are untested.                                                                                                         |
| F3-AC3    | P0       | PARTIAL          | Pinned brew formulae show pinned state, disable selection, explain why, and never enter plans.                                                 | <code>VT-16B9A758AFD2</code><br><code>RS-4788C05B</code>                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 | Component, Unit      | Selection and plan exclusion covered; badge, tooltip copy, and every entry path are not.                                                                                                     |
| F3-AC4    | P0       | PARTIAL          | Greedy-only casks use set difference, separate collapsed UI, default exclusion, and explicit opt-in.                                           | <code>RS-E3B46A7E</code><br><code>RS-D3A41F5E</code><br><code>VT-16B9A758AFD2</code><br><code>VT-D6D6DE8AEA1D</code><br><code>RS-4788C05B</code><br><code>RS-F01E4929</code>                                                                                                                                                                                                                                                                                                                                                                             | Unit, Component      | Classification/exclusion/opt-in covered; collapsed disclosure and explainer are untested.                                                                                                    |
| F3-AC5    | P0       | FULL             | Installed/latest versions remain verbatim, including non-semver and non-comparable values.                                                     | <code>RS-78787BC3</code><br><code>RS-88DEC828</code><br><code>RS-46575376</code><br><code>RS-8E7D4F63</code><br><code>VT-C12D6FECB202</code><br><code>VT-B90D6952476E</code><br><code>VT-CBC1FBC10DD1</code><br><code>VT-B8086BC1EC9F</code>                                                                                                                                                                                                                                                                                                             | Unit, Component      | Parser/component coverage preserves non-semver text, and the new P0 factory contracts prove differing versions never override the manager-provided outdated verdict.                         |
| F3-AC6    | P0       | PARTIAL          | npm self row is hoisted so four package rows remain and self status moves to its card.                                                         | <code>RS-88DEC828</code><br><code>RS-DDEC6787</code><br><code>VT-42F22198677A</code>                                                                                                                                                                                                                                                                                                                                                                                                                                                                     | Unit, Component      | Parser/hoist/count covered; table absence and card presence are not asserted together.                                                                                                       |
| F3-AC7    | P0       | FULL             | mise yields six outdated rows while stable rust remains clean.                                                                                 | <code>RS-78787BC3</code><br><code>VT-6A8DAE6A73D0</code><br><code>VT-71EA86F08E57</code><br><code>VT-CBC1FBC10DD1</code><br><code>VT-B8086BC1EC9F</code>                                                                                                                                                                                                                                                                                                                                                                                                 | Unit, Component      | Real fixture parsing, filter behavior, and the new P0 authoritative-verdict contracts cover the six-outdated/clean-rust result.                                                              |
| F3-AC8    | P0       | PARTIAL          | Status presentation distinguishes update, clean, pinned, and self-updating rows.                                                               | <code>VT-16B9A758AFD2</code><br><code>VT-71EA86F08E57</code><br><code>VT-E4A78A1F6B75</code><br><code>VT-B90D6952476E</code>                                                                                                                                                                                                                                                                                                                                                                                                                             | Component            | Underlying states are indirect-tested; StatusBadge lacks the full label matrix.                                                                                                              |
| D23a-AC1  | P0       | PARTIAL          | Current mas is installed and live-verified; stale absent/unverified assumptions are superseded.                                                | <code>RS-69248718</code><br><code>VT-33EC5E3AB248</code><br><code>RS-44E46A98</code><br><code>RS-6C25C101</code>                                                                                                                                                                                                                                                                                                                                                                                                                                         | Unit, Component      | Real mas 7.0.0 captures and present-state transitions are covered, but the only current-machine detection test is ignored and asserts the obsolete absent state.                             |
| D23a-AC2  | P0       | PARTIAL          | Real mas list/outdated captures form the correctness basis and replace synthetic fixtures.                                                     | <code>RS-ADE81DCD</code><br><code>RS-44E46A98</code><br><code>RS-6C25C101</code>                                                                                                                                                                                                                                                                                                                                                                                                                                                                         | Unit                 | The active parser and merge tests consume the real dated captures. No automated provenance check prevents a future return to synthetic correctness claims.                                   |
| D23a-AC3  | P0       | FULL             | mas parsing handles real ID/name/version padding without retaining whitespace.                                                                 | <code>RS-44E46A98</code><br><code>RS-6C25C101</code>                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     | Unit                 | The real captures directly assert right-aligned IDs, padded names, and trimmed versions inside padded parentheses.                                                                           |
| D23a-AC4  | P0       | NONE             | The obsolete UNVERIFIED label and README limitation remain removed.                                                                            | —                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        | —                    | The requirement is currently violated: SPEC text, Rust comments, the fixture README, and the ignored live smoke still say mas is absent, unverified, or synthetic.                           |
| D23a-AC5  | P0       | UNIT-ONLY        | Synthetic fixtures prove robustness only; correctness claims require real captures.                                                            | <code>RS-44E46A98</code><br><code>RS-6C25C101</code><br><code>RS-CFD56541</code><br><code>RS-DE384A3B</code>                                                                                                                                                                                                                                                                                                                                                                                                                                             | Unit                 | Parser robustness and real-capture correctness are separated in unit tests, but no policy test enforces that distinction.                                                                    |
| F4-AC1    | P0       | PARTIAL          | Global and per-manager update actions build all-outdated selection and open the plan sheet.                                                    | <code>VT-09DBE3EB43E6</code><br><code>AUT-002</code>                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     | Component, E2E       | Keyboard entry and the new browser selected-upgrade journey reach the review sheet. Dashboard and per-manager bulk buttons plus build-failure UI remain uncovered.                           |
| F4-AC2    | P0       | FULL             | Plan sheet shows exact commands, exclusions, warnings, notes, and serialization behavior.                                                      | <code>VT-0BE5205EB6EA</code><br><code>RS-4788C05B</code><br><code>RS-65C62A35</code><br><code>RS-6F4BF445</code><br><code>AUT-002</code>                                                                                                                                                                                                                                                                                                                                                                                                                 | Component, Unit, E2E | Backend plan construction and round-trip execution bind exact structured argv; AUT-002 verifies that the browser submits the reviewed plan unchanged.                                        |
| F4-AC3    | P0       | FULL             | Self-update/greedy defaults, toggle rebuild, rust dedup, and confirm-current-plan behavior are correct.                                        | <code>VT-D6D6DE8AEA1D</code><br><code>VT-B28DBF79D340</code><br><code>RS-F01E4929</code><br><code>RS-E4C4984A</code><br><code>RS-3562B0C5</code><br><code>VT-C889F6701836</code><br><code>VT-5508460FBAF1</code><br><code>RS-9C43A196</code><br><code>RS-C9A0C9E1</code><br><code>RS-39CA7FD5</code><br><code>RS-3D1E9864</code><br><code>RS-BEC0F59B</code><br><code>RS-CAEF32DF</code>                                                                                                                                                                 | Component, Unit      | Canonicalization, bounds, deduplication, selection:null, one-use capability storage, tamper authentication, toggles, and renewed confirmation all have active tests.                         |
| F4-AC4    | P0       | FULL             | execute_plan revalidates and spawns only commands byte-identical to the preview.                                                               | <code>AUT-002</code><br><code>RS-9C43A196</code><br><code>RS-6F4BF445</code><br><code>RS-B76051F9</code><br><code>RS-C9A0C9E1</code><br><code>RS-EF63F385</code><br><code>RS-E13303EA</code><br><code>RS-2D8E12FF</code><br><code>RS-E167FD26</code><br><code>RS-EA358FC4</code><br><code>RS-71C691A0</code><br><code>RS-2AC71F0E</code><br><code>RS-3E48ED57</code><br><code>RS-F05D08B9</code><br><code>RS-48FB7403</code><br><code>RS-8E0C0CDF</code><br><code>RS-6D29D3A5</code>                                                                     | E2E, Unit            | Round-trip, replay, tamper, drift, active-refresh, direct-mutation, overlapping-plan, fast-terminal, and all-or-none admission tests close the prior race and partial-submit gaps.           |
| F4-AC5    | P0       | FULL             | Lock sets serialize conflicts, permit safe parallelism, and cap global concurrency at four.                                                    | <code>RS-1ED7EB86</code><br><code>RS-06E072EE</code><br><code>RS-F6C09169</code><br><code>RS-F05D08B9</code><br><code>RS-48FB7403</code><br><code>RS-8E0C0CDF</code><br><code>RS-6D29D3A5</code>                                                                                                                                                                                                                                                                                                                                                         | Unit                 | Deterministic scheduler tests cover serial conflicts, safe parallelism, the four-operation cap, and atomic conflict rejection; real package managers are intentionally excluded.             |
| F5-AC1    | P0       | PARTIAL          | Selection eligibility, toggle/range, filter-aware tri-state, Cmd+A, and Esc follow the spec.                                                   | <code>VT-2BFF90575AF5</code><br><code>VT-73B7C5B7B7C8</code><br><code>VT-DD8695C22EB4</code><br><code>VT-377DA7826C87</code><br><code>VT-08D613607D9D</code><br><code>VT-5301E7F83768</code><br><code>VT-16B9A758AFD2</code><br><code>AUT-002</code>                                                                                                                                                                                                                                                                                                     | Unit, Component, E2E | Core range, tri-state, Cmd+A, Esc, exclusions, and one browser checkbox flow are covered. Cmd-click, active-search select-all, tooltips, and browser keyboard/range behavior remain missing. |
| F5-AC2    | P0       | FULL             | Upgrade selected passes exact PackageRefs, executes returned plan, and clears after successful enqueue.                                        | <code>VT-049B769969DE</code><br><code>RS-B362A21F</code><br><code>AUT-002</code>                                                                                                                                                                                                                                                                                                                                                                                                                                                                         | Component, Unit, E2E | The browser and component paths both submit exact PackageRefs, execute the returned plan unchanged, and clear selection after enqueue.                                                       |
| F5-AC3    | P0       | PARTIAL          | Historical criterion: row Upgrade created one-package plan and executed immediately. Revised criterion requires add-to-plan without execution. | <code>AUT-003</code> (superseded; no revised credit)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     | E2E                  | Historical status retained only. AUT-003 proves behavior now prohibited by D27 and cannot support the revised criterion.                                                                     |
| F6-AC1    | P0       | PARTIAL          | Self-update routes follow dynamic precedence and required manager commands.                                                                    | <code>RS-D11825C6</code><br><code>RS-326828F3</code><br><code>RS-07C3D1C3</code><br><code>RS-F6930B61</code><br><code>RS-666EF700</code><br><code>RS-62CB3B9C</code><br><code>RS-5BB2F871</code>                                                                                                                                                                                                                                                                                                                                                         | Unit                 | Strong unit precedence coverage; brew ViaRefresh and active whole-machine boundary remain incomplete.                                                                                        |
| F6-AC2    | P0       | UNIT-ONLY        | Self rows are hoisted and routed cross-joins patch latest without replacing installed.                                                         | <code>RS-DDEC6787</code><br><code>RS-997071F8</code><br><code>RS-C61898A2</code><br><code>RS-F8D20DCC</code>                                                                                                                                                                                                                                                                                                                                                                                                                                             | Unit                 | Data behavior covered; card/table rendering and real event transport are absent.                                                                                                             |
| F6-AC3    | P0       | INTEGRATION-ONLY | SelfUpdateCard renders route, queued, npm-reset, and unavailable-executor states.                                                              | <code>VT-FB32EB19B2D9</code><br><code>VT-43F45775CAC3</code><br><code>VT-E25E87F0D070</code>                                                                                                                                                                                                                                                                                                                                                                                                                                                             | Component            | Component integration covers UI states with fake data, without backend or Tauri transport.                                                                                                   |
| F6-AC4    | P0       | UNIT-ONLY        | Update action binds resolved routed/in-band command or rejects unavailable routes.                                                             | <code>RS-0EF3D23A</code><br><code>RS-924BDD60</code><br><code>RS-B366B133</code>                                                                                                                                                                                                                                                                                                                                                                                                                                                                         | Unit                 | Structured routed and in-band argv binding and fail-closed behavior are unit-tested; no frontend click crosses the real Tauri command boundary.                                              |
| F6-AC5    | P0       | UNIT-ONLY        | Routed and mise-managed operations hold complete executor/subject/shared-tree locks.                                                           | <code>RS-6D78F285</code><br><code>RS-C67B7FC7</code><br><code>RS-ED775BD7</code><br><code>RS-6F4BF445</code>                                                                                                                                                                                                                                                                                                                                                                                                                                             | Unit                 | Exact dual-lock sets and scheduler blocking are deterministic Rust evidence only; no native concurrent-manager execution is covered.                                                         |
| F7-AC1    | P0       | UNIT-ONLY        | Output batching preserves streams and flushes at line/time/size/drop boundaries per operation.                                                 | <code>RS-EC601DD8</code><br><code>RS-91EF42D5</code><br><code>RS-18AEB1C1</code><br><code>RS-9CCF6C2E</code><br><code>RS-C95FE1B0</code>                                                                                                                                                                                                                                                                                                                                                                                                                 | Unit                 | Deterministic batching is strong; real process evidence RS-166/RS-167 is ignored and Tauri events are untested.                                                                              |
| F7-AC2    | P0       | PARTIAL          | Activity log appends, pins, bounds memory, handles repaint, and exposes operation context.                                                     | <code>VT-F421BC31E7C4</code><br><code>VT-99E01C630A0B</code><br><code>VT-86B761D03585</code><br><code>VT-29FFF28B94E5</code><br><code>VT-0A5F08A3B636</code><br><code>VT-85DD2A3A37EE</code><br><code>VT-FD19DA096424</code>                                                                                                                                                                                                                                                                                                                             | Unit, Component      | Core states covered; stderr styling, virtualization, Copy/Reveal, empty state, and real WebView events are missing.                                                                          |
| F7-AC3    | P0       | PARTIAL          | Cancel terminates process group, escalates, finalizes state/journal/transcript, and waits on shutdown.                                         | <code>RS-C905C1E3</code><br><code>RS-5089E5C4</code><br><code>RS-4F174F00</code><br><code>RS-36D258AE</code><br><code>RS-883BE45C</code><br><code>VT-804FDC5924E2</code>                                                                                                                                                                                                                                                                                                                                                                                 | Unit, Component      | Escalation and terminal state covered; ordinary real SIGTERM is ignored and row Cancel/no-confirm path is untested.                                                                          |
| F7-AC4    | P0       | PARTIAL          | Stall/hard-cap logic and no-password dialog support keep-waiting, copy, cancel, and timeout.                                                   | <code>RS-A968FE76</code><br><code>RS-A72A4272</code><br><code>RS-14B56CEC</code><br><code>RS-D91D876D</code><br><code>VT-CEB335A5A522</code><br><code>VT-1AF53546B5EA</code>                                                                                                                                                                                                                                                                                                                                                                             | Unit, Component      | Stall/timeout/dialog covered; clipboard, explicit null stdin, and real stall boundary are missing.                                                                                           |
| F8-AC1    | P0       | UNIT-ONLY        | Every operation transcript has canonical name/header/metadata, line flush, and terminal footer.                                                | <code>RS-5A0E4C2D</code><br><code>RS-8F806038</code><br><code>RS-B4F6DB92</code><br><code>RS-0238187E</code><br><code>RS-95CF0E68</code><br><code>RS-A2BB1DF5</code>                                                                                                                                                                                                                                                                                                                                                                                     | Unit                 | Formatting/flush/minimization covered without end-to-end crash/process reconstruction.                                                                                                       |
| F8-AC2    | P0       | UNIT-ONLY        | Journal writes/reads/compacts atomically and tolerates corruption and pre-start cancellation.                                                  | <code>RS-9EAE1F4F</code><br><code>RS-D4D62879</code><br><code>RS-2AD86557</code><br><code>RS-8D1BFC3D</code><br><code>RS-AE83619C</code><br><code>RS-0B69DEC3</code><br><code>RS-74469A1C</code><br><code>RS-7FF9EB68</code>                                                                                                                                                                                                                                                                                                                             | Unit                 | Corruption, atomic failure, retention, and queued-cancel paths covered; real crash boundary is absent.                                                                                       |
| F8-AC3    | P0       | PARTIAL          | Start-without-finish becomes Interrupted and startup never signals recorded pgids.                                                             | <code>RS-78DB125D</code><br><code>VT-8482B2EA6E28</code><br><code>VT-B32E0901F128</code>                                                                                                                                                                                                                                                                                                                                                                                                                                                                 | Unit, Component      | Interrupted data/UI covered; startup hydration and never-signal-pgid rule have no direct test.                                                                                               |
| F8-AC4    | P0       | PARTIAL          | History combines durable/session records and supports filters, detail, transcript, and Reveal.                                                 | <code>VT-695BBDD24EE7</code><br><code>VT-21877AA1D445</code>                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             | Component            | Status filter and success detail covered; other filters, states, Reveal, and native Finder boundary are missing.                                                                             |
| D26-AC1   | P0       | UNIT-ONLY        | Only exact closed-list unterminated notices split when bytes are glued after the literal.                                                      | <code>RS-85A6D7EC</code>                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 | Unit                 | Positive byte shape covered; no real mas transcript boundary.                                                                                                                                |
| D26-AC2   | P0       | UNIT-ONLY        | Terminated/repeated notices behave deterministically and unrelated output is not heuristically rewritten.                                      | <code>RS-6B48FDD0</code>                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 | Unit                 | Required literal cases covered; near-match/property and real mas cases are absent.                                                                                                           |
| F9-AC1    | P0       | PARTIAL          | Diagnostics export uses dated ZIP naming/default destination and is available from Settings/History.                                           | <code>VT-6DB3A4D43A8B</code><br><code>VT-695BBDD24EE7</code><br><code>RS-A5E35D73</code>                                                                                                                                                                                                                                                                                                                                                                                                                                                                 | Component, Unit      | Mock UI and temporary destination covered; real Desktop/native command/permission/error boundary is not.                                                                                     |
| F9-AC2    | P0       | UNIT-ONLY        | ZIP contains report, three newest logs, 25 newest transcripts, and operation journal.                                                          | <code>RS-A5E35D73</code>                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 | Unit                 | Bundle composition covered in temporary filesystem; full report schema and command boundary are not.                                                                                         |
| F9-AC3    | P0       | UNIT-ONLY        | Export is private/resilient: no inherited env dump or symlink traversal; bytes and missing sources are safe.                                   | <code>RS-E52F713F</code><br><code>RS-9626DEFE</code><br><code>RS-13DC43D9</code><br><code>RS-A2BB1DF5</code>                                                                                                                                                                                                                                                                                                                                                                                                                                             | Unit                 | Strong Rust evidence; exhaustive exclusion of every inherited environment value is not proven.                                                                                               |
| F9-AC4    | P0       | PARTIAL          | Export/Open Logs UI exposes visible success and failure outcomes.                                                                              | <code>VT-6DB3A4D43A8B</code><br><code>VT-695BBDD24EE7</code>                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             | Component            | Entry points and export success covered; native opener/export and error UI are not.                                                                                                          |
| F10-AC1   | P0       | PARTIAL          | Product uses the specified dark-only tokenized visual and accessibility treatment.                                                             | <code>AUT-004</code>                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     | E2E                  | AUT-004 covers dark tokens, visible keyboard focus, and reduced-motion behavior in Chromium and WebKit; contrast and packaged WKWebView rendering are not certified.                         |
| F10-AC2   | P0       | NONE             | Generated 1024px icon and required icon set are committed and packaged.                                                                        | —                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        | —                    | Script/config/assets are static evidence only; no automated verification.                                                                                                                    |
| F10-AC3   | P0       | NONE             | macOS app bundle builds and launches normally from Finder/Dock.                                                                                | —                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        | —                    | CI configuration builds a debug artifact but no current run or Finder/WebView launch test was inspected.                                                                                     |
| F10-AC4   | P0       | NONE             | Superseding release contract produces universal signed, notarized, stapled updater assets.                                                     | —                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        | —                    | Release workflow is static intent only; no run, asset, signature, ticket, or updater archive was verified.                                                                                   |
| F11-AC1   | P0       | UNIT-ONLY        | Settings defaults and durable atomic persistence handle missing/corrupt/partial/failing files.                                                 | <code>RS-E1A89175</code><br><code>RS-A0FC60CA</code><br><code>RS-5175C41E</code><br><code>RS-E2E70539</code><br><code>RS-C8508175</code>                                                                                                                                                                                                                                                                                                                                                                                                                 | Unit                 | Strong temporary-filesystem coverage; actual Application Support path and command boundary are absent.                                                                                       |
| F11-AC2   | P0       | PARTIAL          | Every Settings control persists valid values, bounds numeric input, and applies live log level.                                                | <code>VT-1BC2C8F7E79B</code><br><code>VT-70364B262530</code><br><code>VT-110ACE171098</code><br><code>RS-0CC325E0</code><br><code>RS-C8508175</code>                                                                                                                                                                                                                                                                                                                                                                                                     | Component, Unit      | Representative controls and precedence covered; full control matrix, numeric validation, save failure, and live runtime are not.                                                             |
| F11-AC3   | P0       | PARTIAL          | Environment Report shows required fields/evidence and supports Copy.                                                                           | <code>VT-D59F5E461AC7</code><br><code>RS-D98BFE73</code>                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 | Component, Unit      | UI and wire shape partly covered; completeness, clipboard, and failure state are missing.                                                                                                    |
| F11-AC4   | P0       | PARTIAL          | Re-detect, Export Diagnostics, and Open Logs invoke native actions with visible outcomes.                                                      | <code>VT-0F530C5EB18D</code><br><code>VT-6DB3A4D43A8B</code>                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             | Component            | Mocked invocations covered; native filesystem/opener and failure outcomes are not.                                                                                                           |
| F12-AC1   | P0       | PARTIAL          | Rust and frontend quality suites pass deterministically/offline from a clean checkout.                                                         | —                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        | —                    | The latest recorded local runs and current head CI are green, but they are not a fresh clean-checkout, forced-offline proof; 11 environment/live tests remain ignored.                       |
| F12-AC2   | P0       | PARTIAL          | Default tests use deterministic seams/fixtures and exclude live machine/network probes.                                                        | <code>VT-96A9AA0A8B9D</code><br><code>RS-51C5E482</code><br><code>RS-A6AD0EAF</code><br><code>RS-07971583</code><br><code>RS-A968FE76</code><br><code>RS-A72A4272</code><br><code>PW-416E2F1A</code><br><code>PW-25B437A7</code>                                                                                                                                                                                                                                                                                                                         | Unit, E2E            | Deterministic seams, fake-Tauri transport, an outbound-fetch guard, and dedicated Playwright CI exist. No suite-wide proof covers WebSockets, service workers, or all machine dependencies.  |
| F12-AC3   | P0       | FULL             | Representative IPC payload fixtures are byte-checked in Rust and runtime-guarded in TypeScript.                                                | <code>RS-51C5E482</code><br><code>VT-96A9AA0A8B9D</code><br><code>VT-564929CE1DDC</code><br><code>VT-A8C30140E329</code><br><code>VT-5017E5EC65E2</code><br><code>VT-BF0D72BBFBC8</code><br><code>VT-C7D10C433C04</code><br><code>VT-E37B59DFCAD8</code><br><code>VT-B5340CE16EB0</code><br><code>VT-AADA9CCA52C0</code><br><code>VT-DBA1B42643F9</code><br><code>VT-5C72E2BD08BE</code><br><code>VT-C06FAA8EC610</code><br><code>VT-346C179A228E</code><br><code>VT-D47E3BC19EA3</code><br><code>VT-F3C6C34E3B71</code><br><code>VT-DDD19B28C9F5</code> | Unit                 | Two-sided fixture-set and payload validation directly guard contract drift.                                                                                                                  |
| D25-AC1   | P1       | PARTIAL          | Update checks run at launch, every six hours, and on menu demand under saved policy.                                                           | <code>VT-A961AB9662D7</code><br><code>VT-70B9633858FB</code>                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             | Component            | Manual/automatic result policy covered; launch callback, timer, menu, setting, and runtime boundary are not.                                                                                 |
| D25-AC2   | P0       | PARTIAL          | Updater state machine safely exposes check/current/download/progress/error states.                                                             | <code>RS-3A2F9E33</code><br><code>RS-E268B5B4</code><br><code>RS-AF81BCD5</code><br><code>RS-CC37163C</code><br><code>RS-37C1FD08</code><br><code>VT-86EA58C74180</code><br><code>VT-7BE42CA3CF03</code><br><code>VT-DC56F3812BE7</code><br><code>VT-13E498B76945</code><br><code>VT-70B9633858FB</code><br><code>VT-83665ECB3E8D</code>                                                                                                                                                                                                                 | Unit, Component      | Backend and UI states covered separately; real endpoint/plugin/event/download integrity are untested.                                                                                        |
| D25-AC3   | P0       | PARTIAL          | Install requires explicit click and guards active operations before restart/install.                                                           | <code>RS-1C2922C1</code><br><code>RS-FE13F171</code><br><code>RS-88B37111</code><br><code>VT-B68FA172C6B3</code><br><code>VT-EC68A6FD5550</code>                                                                                                                                                                                                                                                                                                                                                                                                         | Unit, Component      | Refusal, fake install, error, action, and quit guard covered; real installation/relaunch/OS process boundary is not.                                                                         |
| D25-AC4   | P0       | PARTIAL          | Non-writable bundle never prompts for admin and becomes manualInstallRequired.                                                                 | <code>RS-E68B6612</code><br><code>RS-2495AC18</code><br><code>VT-FFEF0BFA11FC</code>                                                                                                                                                                                                                                                                                                                                                                                                                                                                     | Unit, Component      | Bundle-root logic and UI copy covered; actual access preflight and absence of OS authorization prompt are not.                                                                               |
| D25-AC5   | P1       | PARTIAL          | appUpdate status contract/rehydration preserves state, errors, progress, and trigger semantics.                                                | <code>VT-86EA58C74180</code><br><code>VT-7BE42CA3CF03</code><br><code>VT-DC56F3812BE7</code><br><code>VT-13E498B76945</code><br><code>VT-B68FA172C6B3</code><br><code>VT-EC68A6FD5550</code><br><code>VT-FFEF0BFA11FC</code><br><code>VT-A961AB9662D7</code><br><code>VT-70B9633858FB</code><br><code>VT-83665ECB3E8D</code><br><code>VT-A8C30140E329</code><br><code>RS-51C5E482</code>                                                                                                                                                                 | Component, Unit      | Fixture and component states covered; real emission/transport/listener ordering/rehydration are not.                                                                                         |
| D25A-AC1  | P1       | NONE             | Custom macOS menu preserves required Edit and Window actions.                                                                                  | —                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        | —                    | Implementation exists but no native menu assertion or runtime test.                                                                                                                          |
| D25A-AC2  | P0       | INTEGRATION-ONLY | Updater release assets are authentic/consistent and CI build smoke uses the required no-sign behavior.                                         | —                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        | —                    | CI and release workflows encode --no-sign smoke plus signing/notarization/updater packaging, but no collected test boots or updates a published signed artifact.                             |
| F13-AC1   | P1       | PARTIAL          | Surfaced health fix uses the exact trusted parser-provided command.                                                                            | <code>VT-7A9878F09808</code><br><code>RS-7356C2DB</code><br><code>RS-48C881C0</code><br><code>RS-068F52F6</code><br><code>VT-27A50F858EBE</code><br><code>RS-79EDA349</code><br><code>RS-DF193999</code><br><code>RS-D44794C1</code><br><code>RS-2D944CED</code>                                                                                                                                                                                                                                                                                         | Component, Unit      | Exact, altered, missing, and malformed uv suggestions plus backend argv binding are tested. The real frontend-to-Tauri-to-queue path and failure feedback remain uncovered.                  |
| F14-AC1   | P1       | NONE             | Latest snapshots persist and render instantly as stale/refreshing after relaunch.                                                              | —                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        | —                    | No durable snapshot-cache implementation or test was found.                                                                                                                                  |
| F15-AC1   | P1       | NONE             | Backgrounded app sends native completion/failure notifications.                                                                                | —                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        | —                    | No native notification integration or test was found.                                                                                                                                        |
| F16-AC1   | P1       | NONE             | Package-detail popover presents uv, mise, npm, and brew metadata.                                                                              | —                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        | —                    | Required popover/cross-manager presentation is not implemented or tested.                                                                                                                    |
| F17-AC1   | P1       | NONE             | mise rust row shows Also managed by rustup when applicable.                                                                                    | —                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        | —                    | No matching implementation or test was found.                                                                                                                                                |

### Coverage-Logic Validation

- **P0/P1:** Every criterion was reviewed. A mapping is not upgraded merely because implementation source exists; static code/workflow declarations are called out separately from executable test evidence.
- **Duplicate coverage:** Shared tests recur only where they protect cross-cutting invariants such as authoritative outdatedness, plan authentication, atomic admission, lock ownership, or IPC contract synchronization. Stable IDs allow unique-test deduplication.
- **Error and alternate paths:** Lower-level coverage is strong for parse failures, timeouts, cancellation, stale plans, tamper/replay, persistence failure, updater failure, and retained stale state. Browser journeys remain mostly happy-path and do not exercise native error transport.
- **API and authorization:** HTTP endpoint and account/role authentication checks are not applicable. The relevant local trust boundary is Tauri IPC plus backend-owned argv; it has strong separate frontend/backend tests but no real invoke-handler crossing.
- **UI journeys and states:** This historical snapshot covered search, selected upgrade review/confirm, superseded immediate row upgrade, dark tokens, focus, and reduced motion. Persistent plan membership, separate confirmation, plan Activity/Results/History/Retry, Settings, refresh failures, diagnostics, native updater/menu, high zoom, and packaged launch require revised evidence.
- **Oracle drift:** D23a is contradicted by current repository text and the ignored live smoke: mas is installed and verified, while several locations still say absent/unverified/synthetic. Settings also retains obsolete copy that notarization is out of scope despite D25/D25a.

### Step 3 Outcome

All 80 normalized criteria are mapped to the current stable test catalog. The strongest area is deterministic backend behavior, including the newly hardened one-use plan capability and atomic-admission races. The largest residual gaps are native Tauri transport, packaged/release behavior, current-machine mas acceptance, broad browser failure journeys, and the unimplemented P1 F14-F17 surfaces. Coverage gaps and gate impact are analyzed in the next step.

## Step 4: Coverage Gap Analysis

### Execution Mode

- **Requested:** `auto` from TEA configuration; the user supplied no override.
- **Resolved:** `agent-team` because agent-team capability probing succeeded.
- **Parallel work:** gap classification, heuristic extraction, and statistics verification ran independently; recommendation synthesis and JSON assembly ran after those results converged.

### Gap Distribution

| Gap class               | Count | IDs                                                    |
| ----------------------- | ----: | ------------------------------------------------------ |
| P0 uncovered (critical) |     5 | `F1-AC7`, `D23a-AC4`, `F10-AC2`, `F10-AC3`, `F10-AC4`  |
| P1 uncovered (high)     |     5 | `D25A-AC1`, `F14-AC1`, `F15-AC1`, `F16-AC1`, `F17-AC1` |
| Partial                 |    38 | See the detailed matrix and Phase 1 JSON               |
| Unit-only               |    16 | See the detailed matrix and Phase 1 JSON               |
| Integration-only        |     2 | `F6-AC3`, `D25A-AC2`                                   |

### Release-Critical Findings

1. **Release artifacts are not proven.** No executable check establishes universal architecture, bundle contents/icon, Developer ID signature, notarization/stapling, updater metadata/signature consistency, Finder launch, or updater install/relaunch (`F10-AC2` through `F10-AC4`, `D25A-AC2`).
2. **The current-machine acceptance oracle is stale.** `mas` 7.0.0 is installed and real captures are tested, but the ignored live smoke still expects absence and several SPEC/comments/README locations retain the superseded absent/unverified/synthetic claim (`F1-AC7`, `D23a-AC4`). This is a repository correctness problem as well as a test gap.
3. **The native application boundary is untested.** All 20 registered Tauri commands are tested only on one side or the other; no test crosses real JavaScript serialization through the native invoke handler, and no representative native event round-trip exists.
4. **Updater safety is split across seams.** The Rust state machine and React states are strong, but the real endpoint/plugin transport, native menu, deterministic non-writable preflight, downloaded artifact integrity, install, and relaunch remain unproven (`D25-AC2` through `D25-AC4`, `D25A-AC1`, `D25A-AC2`).
5. **Reliability journeys remain below the native/E2E boundary.** Cancellation/stall handling, crash recovery, offline refresh isolation, diagnostics privacy/export, and settings persistence have meaningful lower-level evidence but incomplete integrated proof.
6. **Four P1 feature surfaces are not implemented.** Snapshot cache (`F14`), background notifications (`F15`), package-detail popover (`F16`), and the rustup note (`F17`) are product gaps, not merely missing tests.

### Heuristic Blind Spots

| Heuristic                                                  | Count | Interpretation                                                                                                           |
| ---------------------------------------------------------- | ----: | ------------------------------------------------------------------------------------------------------------------------ |
| HTTP/API endpoints without tests                           |     0 | Not applicable; Pack-Manager has no service API                                                                          |
| Native Tauri commands without a real JS→Rust boundary test |    20 | Representative native command/event integration is absent                                                                |
| Auth/authz negative-path gaps                              |     0 | Not applicable; there are no accounts, roles, sessions, or tokens                                                        |
| Happy-path-only criteria                                   |     9 | Upgrade errors, History/diagnostics failures, settings rollback, clipboard denial, and health-fix failure feedback       |
| UI journey families without E2E                            |     6 | History/Settings, operation lifecycle, refresh/self-update, selection edges, native maintenance/release, and P1 surfaces |
| Missing UI states                                          |     3 | Empty History, truly empty package table, and permission-denied maintenance actions                                      |

### Recommended Order

1. Add release-artifact verification and a packaged Finder/Dock launch smoke.
2. Correct all D23a drift, update the live detection expectation to `mas` present, and run the current-machine smoke explicitly.
3. Add a native Tauri integration harness for representative read, plan/execute, settings, diagnostics, updater, and event paths.
4. Prove updater install/relaunch and the no-admin-prompt preflight against a controlled non-writable target.
5. Cover cancellation/stall, crash/relaunch, offline manager isolation, diagnostics privacy, and settings failure paths.
6. Add red-path Playwright/component coverage for row/bulk upgrade errors and the missing History/Settings/UI states.
7. Decide whether F14-F17 ship now; implement and automate them or explicitly defer them from the gate scope.
8. Run `bmad-testarch-test-review` after the critical coverage work.

### Phase 1 Output

- **Coverage matrix:** `/tmp/tea-trace-coverage-matrix-2026-07-23T02-28-24Z.json`
- **Strict full coverage:** 14/80 (18% rounded)
- **P0 strict full coverage:** 14/72 (19% rounded)
- **P1 strict full coverage:** 0/8 (0%)
- **Mapped test inventory:** 238 active unique tests: 177 Unit, 55 Component, 6 E2E
- **Discovered suite inventory:** 395 cases: 384 active and 11 ignored

### Step 4 Outcome

Phase 1 is complete. The gap analysis, heuristic findings, recommendations, full requirement mappings, deduplicated mapped-test inventory, and full discovered-suite inventory are valid JSON at the recorded temporary path. No gate decision has been made in this phase.

## PHASE 2: QUALITY GATE DECISION

### Gate Decision: FAIL

**Gate type:** Release  
**Decision mode:** Deterministic  
**Collection status:** COLLECTED  
**Gate eligible:** Yes  
**Oracle confidence:** High

**Rationale:** P0 coverage is 19% (required: 100%). Five critical requirements are completely uncovered.

This is a coverage-gate failure, not a statement that the existing tests are failing. The latest recorded active suites are green; the release gate fails because required behavior lacks sufficient executable proof and two requirements are contradicted or unimplemented in the current repository.

### Decision Criteria

| Criterion                    |                     Required |      Actual | Status  |
| ---------------------------- | ---------------------------: | ----------: | ------- |
| P0 strict full coverage      |                         100% | 19% (14/72) | NOT MET |
| P1 strict full coverage      | 80% minimum; 90% PASS target |    0% (0/8) | NOT MET |
| Overall strict full coverage |                  80% minimum | 18% (14/80) | NOT MET |

The first deterministic rule already yields `FAIL`: P0 coverage is below 100%. The P1 and overall thresholds also fail independently.

### Execution Evidence

- **Vitest:** 133/133 passed across 23 files in the latest recorded automation run.
- **Rust:** 245 active tests passed; 11 live/environment tests were ignored by design.
- **Playwright:** 12/12 project executions passed, representing six logical cases across Chromium and WebKit.
- **Build and static checks:** frontend build, E2E typecheck/discovery/format checks, and the recorded CI checks passed.
- **Important boundary:** Playwright uses fake Tauri IPC, so these green results do not prove the native JS→Rust boundary or packaged application behavior.

### Critical P0 Blockers

| ID         | Blocking issue                                                                                                                 | What closes it                                                                                                                                                     |
| ---------- | ------------------------------------------------------------------------------------------------------------------------------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `F10-AC4`  | Published release authenticity and updater compatibility are not executable evidence.                                          | Inspect produced release assets for architectures, signatures, notarization/stapling, archive contents, metadata URLs, updater signature, and version consistency. |
| `F10-AC3`  | No packaged `.app` launch smoke proves Finder/Dock startup, embedded resources, WKWebView, entitlements, or GUI PATH behavior. | Build the release candidate and launch it with a native macOS smoke test.                                                                                          |
| `F1-AC7`   | No active test proves current-machine manager topology; the ignored live smoke incorrectly expects `mas` absent.               | Update the live expectation to `mas` 7.0.0 present and execute the explicit live smoke on the target Mac.                                                          |
| `D23a-AC4` | SPEC text, Rust comments, fixture README, and the ignored smoke retain obsolete absent/unverified/synthetic `mas` claims.      | Remove every superseded claim and add a focused guard against regression.                                                                                          |
| `F10-AC2`  | Icon source, generated set, and bundle inclusion have static configuration only.                                               | Add an asset/bundle verification test for the 1024px source, required icon outputs, and packaged resources.                                                        |

### High-Priority and Cross-Cutting Risks

- `D25A-AC1` has no native test that the hand-built macOS menu preserves Edit/Window shortcuts.
- `F14-AC1`, `F15-AC1`, `F16-AC1`, and `F17-AC1` are unimplemented P1 product requirements.
- All 20 registered Tauri commands lack a real frontend-to-native handler crossing; a representative native event round-trip is also absent.
- Updater download/install/relaunch and deterministic no-admin-prompt behavior are only proven in separate fakes/components.
- Cancellation/stall, crash recovery, offline isolation, diagnostics, and settings have strong lower-level tests but incomplete native or packaged journeys.

### Required Remediation Order

1. Verify real release artifacts and packaged Finder/Dock launch.
2. Correct D23a documentation/code-comment drift and run the corrected live `mas` detection smoke.
3. Add representative native Tauri command/event integration coverage.
4. Prove updater install/relaunch and non-writable-bundle behavior without an administrator prompt.
5. Cover the critical reliability and red-path UI journeys.
6. Implement or explicitly defer F14-F17, then rerun `bmad-testarch-trace`.

### Machine-Readable Outputs

- `e2e-trace-summary.json`: portable coverage, risk, recommendation, and gate summary.
- `gate-decision.json`: slim downstream gate signal with `gate_status: FAIL`.
- Phase 1 source matrix: `/tmp/tea-trace-coverage-matrix-2026-07-23T02-28-24Z.json`.

### Final Outcome

**Release blocked by the deterministic traceability gate.** Existing active tests are green, but the required coverage thresholds are not met. Address the five P0 blockers first and rerun this workflow; no waiver was requested or applied.
