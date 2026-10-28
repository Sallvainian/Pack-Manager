# Reconciliation review — ARCHITECTURE-SPINE.md (revision 4) vs docs/DECISIONS.md + docs/SPEC.md

Date: 2026-07-25
Reviewer scope: the four questions posed by the team lead.
Sources read in full this session: `ARCHITECTURE-SPINE.md` (505 lines),
`docs/DECISIONS.md` (365 lines), `docs/SPEC.md` (805 lines). Code and workflow
greps are cited inline.

Authority order applied: `docs/SPEC.md` governs product behavior; a later
explicit `docs/DECISIONS.md` entry overrides an earlier decision.

**Verdict:** the spine is substantially reconciled. The D33 cleanup is complete —
no live readiness-gate rule survives. Two live decisions are contradicted
(D25's update surfaces, D31's open sub-question), and five decisions with real
architectural consequence are uncovered. No SPEC load-bearing invariant is
inverted; two are simply absent.

---

## 1. Spine rules that CONTRADICT a live decision

### F1 [MEDIUM-HIGH] AD-16's app-update surface list excludes a surface D25 names and the app ships

The spine, `ARCHITECTURE-SPINE.md:284-286`:

> "- **Rule:** The application's own update is not a Package plan. It never enters a
>   `PlanIntent`, draft, confirmed attempt, Results, or plan-attempt History; it
>   holds no manager lock and surfaces only through its own badge and Settings card
>   (`docs/DECISIONS.md` D25)."

D25, `docs/DECISIONS.md:119-122`:

> "Checks run
> on launch, every 6h, and on demand from the macOS app menu ("Pack-Manager → Check for
> Updates…"); a found update downloads automatically in the background, and the bottom-left
> StatusBar indicator turns into the button that installs it and relaunches."

The shipping code has that menu item — `src-tauri/src/lib.rs:70`:

> `                "Check for Updates…",`

and `src-tauri/src/lib.rs:26`:

> `/// Menu item id for the app menu's "Check for Updates…".`

D25a treats the menu as load-bearing enough to record its cost,
`docs/DECISIONS.md:135-138`:

> "- **The app menu is rebuilt by hand.** `app.set_menu` replaces Tauri's default wholesale,
>   so `lib.rs` re-declares the Edit and Window submenus; without them ⌘X/⌘C/⌘V/⌘A die in
>   the package search field and every `CopyableCommand`."

The word **only** in the spine rule makes the list exclusive. It omits the macOS
app menu, which is both a D25-named trigger and a shipped, hand-rebuilt menu whose
absence would silently regress clipboard shortcuts. A builder enforcing AD-16
literally would be entitled to delete it.

The "Settings card" half of the spine's list is correct —
`src/components/settings/SettingsView.tsx:182-183`:

> `            <section aria-label="Updates">`
> `              <h2 className="mb-2 text-[15px] font-semibold text-text-primary">Updates</h2>`

and `src/components/settings/SettingsView.tsx:192`:

> `                <Button variant="secondary" size="sm" onClick={() => void checkForAppUpdate()}>`

**Fix:** make the list "its own badge, the Settings Updates card, and the app
menu's `Check for Updates…` item", or drop "only" and state the exclusion
positively (it never enters plan domains).

### F2 [MEDIUM] The spine records D31 as RESOLVED; D31 explicitly declines to close it

Spine status table, `ARCHITECTURE-SPINE.md:493`:

> `| Minimum supported macOS | **RESOLVED** | 15.0 at `bundle.macOS.minimumSystemVersion`. `docs/DECISIONS.md` D31. |`

and AD-11, `ARCHITECTURE-SPINE.md:219-222`:

> "- **Rule:** Minimum supported macOS is 15.0 at
>   `bundle.macOS.minimumSystemVersion` (`docs/DECISIONS.md` D31). CI stays on
>   `macos-14`: a deployment target above the build SDK is a floor annotation, not
>   an SDK requirement."

That rule text is a faithful restatement of `docs/DECISIONS.md:268-270`. But D31
carries a residual it does not permit anyone to assert away,
`docs/DECISIONS.md:271-276`:

> "**One question remains OPEN at the time of writing:** whether `notarytool`
> accepts `minos 15.0` against SDK 14.5. Nothing here asserts that it does. It
> is settled by a manual Release workflow run — which builds, signs, and
> notarizes, and uploads to the workflow run only, never touching a GitHub
> Release. If notarization rejects the floor, this record changes rather than
> the pipeline absorbing a surprise later."

The spine states **RESOLVED** with no residual and no pointer to the settling
run. It does describe the mechanism elsewhere, AD-12 `ARCHITECTURE-SPINE.md:242-244`:

> "The manual
>   workflow-dispatch path publishes nothing only when `attach_to_tag` is empty —
>   that is the safe way to test pipeline changes."

— but never joins it to the open question. A reader of the status table
concludes the macOS floor carries no remaining release risk; D31 says the
opposite in bold. This is the spine asserting a closure its cited source
refuses to assert.

**Fix:** change the note to "15.0 declared; `notarytool` acceptance against the
CI SDK is settled by a manual Release run (D31, open)" — or mark the row
**RESOLVED (one residual)**.

---

## 2. Live decisions with architectural consequence the spine does NOT cover

### F3 [MEDIUM] D10's rust-dedup exclusion has no home in the new draft/preview domain

D10, `docs/DECISIONS.md:43`:

> "the plan builder drops mise's `tool:rust` with reason `rustDedup` and a visible note whenever the same plan contains rustup toolchain targets — one plan never races two upgrades of the same toolchain."

`grep -n -iE 'dedup' ARCHITECTURE-SPINE.md` returns exactly one hit, and it is a
different sense — `ARCHITECTURE-SPINE.md:344`:

> "  converge to one deduplicated membership set with a single authenticated
>   rebuild."

The spine's exclusion enumeration, `ARCHITECTURE-SPINE.md:346-349`:

> "- **Ineligible-item inertness.** An item that is pinned, already current, a
>   non-opted-in greedy cask, or removed between staging and rebuild is inert: its
>   control is non-interactive to pointer and keyboard, it carries a stated reason
>   for assistive technology, and it can never enter a `PlanIntent`."

rustDedup is missing, and it is the **only** exclusion whose trigger is
cross-item — it depends on what else is in the plan, not on the item. So the
spine's rule as written is actively wrong for it: "it can never enter a
`PlanIntent`" cannot hold for `tool:rust`, which is legitimately stageable until
a rustup toolchain joins the same plan. Under the new model membership is durable
and rebuilt, which makes the question sharper than it was under D6's one-shot
sheet: does a rustDedup item stay in `PlanIntent` and get dropped at preview, or
is it evicted from intent?

The reason code is live in shipping code and in the IPC contract —
`src/lib/ipc/types.ts:65`:

> `export const EXCLUDE_REASONS = ["pinned", "greedyCask", "rustDedup", "alreadyRunning"] as const;`

and `src-tauri/src/queue.rs:394`:

> `pub const RUST_DEDUP_NOTE: &str = "rust toolchains are handled by rustup in this plan";`

Note that `alreadyRunning` is likewise absent from the spine's list.

**Fix:** add one clause to the inertness rule distinguishing item-level
ineligibility (barred from `PlanIntent`) from plan-composition exclusions
(admitted to `PlanIntent`, surfaced in `UpgradePlanPreview.exclusions`). The
domain minimum already has the slot — `ARCHITECTURE-SPINE.md:300` "exclusions / warnings".

### F4 [MEDIUM] D2 and D3 — SPEC load-bearing invariants 1 and 2 — get no spine rule, though Story 2.2 is bound

`grep -n -iE 'canonicaliz|classif|managed.?by|outdated verdict|version compar' ARCHITECTURE-SPINE.md`
returns no hit on any of these terms in the product sense.

D3, `docs/DECISIONS.md:15`:

> "Mise shims ARE symlinks to the mise binary (Judge 2 verified live: `~/.local/share/mise/shims/uv` canonicalizes to the brew-installed mise under `/opt/homebrew/`), so canonicalize-first would classify uv and npm as brew-managed and misroute their self-updates to `brew upgrade uv`. Evidence strings are stored and surfaced (chip tooltip, logs, diagnostics). Named regression test required."

SPEC states it as a load-bearing invariant, `docs/SPEC.md:14`:

> "2. **Who-manages-whom is derived from paths at detection time, never hardcoded.** Classification inspects the RAW resolved path against mise directories BEFORE canonicalizing — mise shims ARE symlinks to the mise binary (`~/.local/share/mise/shims/uv` canonicalizes to the brew-installed mise), so canonicalize-first would misroute uv/npm to brew."

D2 / SPEC invariant 1 is likewise absent, `docs/SPEC.md:13`:

> "1. **The manager's outdated verdict is authoritative.** Pack-Manager never computes version comparisons to decide outdatedness."

The spine binds a live story to this area with no governing rule —
`ARCHITECTURE-SPINE.md:483`:

> `| Detection, refresh phases, timeouts (Story 2.2) | Manager adapters behind runtime ports | AD-4 |`

AD-4's port list touches the adjacent capability but not the ordering rule,
`ARCHITECTURE-SPINE.md:166-168`:

> "- **Rule:** The runtime-port set covers process spawn/output/exit/stdin/signals,
>   monotonic and wall time, executable discovery and `ToolEnv`, application and
>   log roots, filesystem and permissions, symlink metadata, opener and reveal,"

This is the single highest-blast-radius invariant in the product (it silently
misroutes self-updates rather than failing), it is bound to a live story, and it
is exactly the class of thing a "spine of invariants" exists to hold. Its absence
is defensible only if the spine's altitude deliberately excludes detection
semantics — but the spine already descends to `#[serde(rename_all = ...)]`
casing (`ARCHITECTURE-SPINE.md:421`) and lock ordering
(`ARCHITECTURE-SPINE.md:177-180`), which is lower.

**Fix:** one rule under AD-4, or a Consistency Conventions row: classification
reads the RAW resolved path before canonicalization; the manager's `outdated`
field is the only authority on outdatedness; frontend delta/severity is display
only.

### F5 [MEDIUM] D30 moves Activity out of the drawer; the spine settles the sidecar's shape but not Activity's

D30, `docs/DECISIONS.md:230-232`:

> "Activity is a first-class navigation destination for the active attempt and
> for replaying a completed History entry. A queued draft remains in the Upgrade
> Plan, not Activity."

and its supersession, `docs/DECISIONS.md:241-242`:

> "**Supersedes:** D18's flat Manager navigation and drawer-only Activity model."

AD-17 exists precisely to prevent this class of ambiguity —
`ARCHITECTURE-SPINE.md:357-359`:

> "- **Prevents:** UX-PB.1a building a frontend-only draft that UX-PB.1b cannot
>   reconstruct after a crash, and separate stories disagreeing on whether the
>   sidecar is a dialog, a drawer, or a layout region"

but it resolves that question for the sidecar only. The spine's only Activity
rules are `ARCHITECTURE-SPINE.md:367` ("A draft is never surfaced as Activity or
History") and `ARCHITECTURE-SPINE.md:316` ("The active-attempt lookup, cancel
command, History query, Activity replay, and diagnostic export address
`planAttemptId`"). Neither states whether Activity is a new `ActiveView`
destination, nor what becomes of the existing `ActivityDrawer`.

The drawer is live in the verified baseline, `ARCHITECTURE-SPINE.md:90-92`:

> "  `InteractionRequired` symbol exists in `src/` or `src-tauri/src/`.
>   `autoOpenDrawer` is still an active setting."

and AD-19 retires only the *setting*, not the surface,
`ARCHITECTURE-SPINE.md:409-410`:

> "  observed by product code. An old persisted `autoOpenDrawer` value is tolerated
>   on read and inert once `skipUpgradePlanConfirmation` exists."

So a UX-PB.3x story and a UX-PB.4x story can currently disagree on whether
Activity is a route, a drawer that no longer auto-opens, or a sidecar mode —
the exact failure AD-17 was written to prevent for the sidecar.

**Fix:** extend AD-17 with one rule fixing Activity's surface (destination vs
drawer) and the disposition of the existing `ActivityDrawer` component.

### F6 [LOW] D25's "menu handler and IPC command share one code path" rule is not stated

D25, `docs/DECISIONS.md:126-129`:

> "The state machine lives in Rust (`app_update.rs`) behind an `UpdateSource`/`PendingRelease` seam
> mirroring `CommandRunner`/`FakeRunner`, so the menu handler and the IPC command share
> one code path and the whole flow is testable offline."

The spine names the ports but not the convergence rule,
`ARCHITECTURE-SPINE.md:168-170`:

> "  `CommandRunner`, `EventSink`, `UpdateSource`, and
>   `PendingRelease` are existing ports and are extended rather than bypassed."

"Extended rather than bypassed" does not forbid a second entry point growing its
own state transitions. Given F1 already understates the menu surface, the two
omissions compound.

### F7 [LOW] D26's output-mutation carve-out has no anchor under AD-4

D26, `docs/DECISIONS.md:157`:

> "`runner.rs` carries `UNTERMINATED_NOTICES`, a const list of **verbatim strings, never patterns**, and breaks the line after any entry that has output glued behind it."

and `docs/DECISIONS.md:159`:

> "This is the only place Pack-Manager inserts a line break the child never printed, which is why the rule is deliberately the least clever one available."

AD-4's safety floor is exhaustively about the *request* side —
`ARCHITECTURE-SPINE.md:172-176`:

> "- **Rule:** Ports may not weaken the settled safety floor. Process requests stay
>   structured argv against resolved absolute executables, with `env_clear`, an
>   explicit environment, null stdin, and `process_group(0)`. No shell command
>   string is ever run, no display text is ever split back into arguments, and no
>   `sudo`, password, or administrator route exists."

Nothing constrains the *output* side. Since `runner.rs` sits behind
`CommandRunner`, a builder extending that port under AD-4 has no signal that
child-output rewriting is a closed, verbatim-list-only operation — and SPEC
still promises byte fidelity at `docs/SPEC.md:9` ("leaves a byte-faithful
transcript on disk"), narrowed only by D26's accepted cost.

### F8 [LOW] `execute_plan`'s all-or-none batch admission is not stated

`docs/SPEC.md:443`:

> "`execute_plan` consumes the capability, compares submitted/issued/fresh plans within one coherent revision, rejects active refreshes, and sends the complete derived operation set to the scheduler as one atomic all-or-none batch."

The spine's nearest rule, `ARCHITECTURE-SPINE.md:258-259`:

> "- **Rule:** `execute_plan` returns a newly generated durable `planAttemptId` plus
>   the admitted Operation identities."

"Admitted" implies but does not require atomicity. Under D30's one-active-attempt
model a partial admission would produce an attempt whose membership silently
differs from the reviewed intent — precisely what AD-16 is defending.

---

## 3. Residue of the retired readiness gate (D33)

**None found in the rule set.** `grep -n -iE 'TIR-|RE-[0-9]|ASR-|contracts/|72-criterion|coverage|evidence manifest|candidate.freeze|wave|scenario contract|P0 gate|DR-[0-9]'`
over the spine returns six hits. Every one is a retirement record or a negation:

1. `ARCHITECTURE-SPINE.md:36` — revision note, describing what was removed:
   > "which bound builders to a `contracts/` directory that has never existed. Adds"
2. `ARCHITECTURE-SPINE.md:207` — AD-11, a negation:
   > "  checklist, not a computed verdict, coverage percentage, or gate decision. A"
3. `ARCHITECTURE-SPINE.md:227` — AD-11, correct D33 citation:
   > "  (`docs/DECISIONS.md` D33, restating the former DR-2)."
   Matches `docs/DECISIONS.md:339-342` ("DR-2's substance survives without its gate framing... Accessibility here is product quality, not evidence ceremony.").
4. `ARCHITECTURE-SPINE.md:495` — status table, explicit RETIRED:
   > "| Readiness gate policy | **RETIRED** | The 72-criterion gate, coverage percentages, scenario contracts, evidence manifests, and candidate-freeze machinery are dissolved. `docs/DECISIONS.md` D33. AD-6..AD-10 and AD-13..AD-15 are retired ids and are never reused. |"
5. `ARCHITECTURE-SPINE.md:496` — status table, explicit RETIRED:
   > "| Boundary catalog file | **RETIRED** | `contracts/tauri-boundary/v1.json` is not created. The atomic-change obligation moved to AD-3's committed contract fixtures. |"
6. `ARCHITECTURE-SPINE.md:497` — status table, explicit RETIRED:
   > "| ASR-01 / ASR-02 / ASR-03 enabler framing | **RETIRED** | The enabler register belonged to the retired gate. The surviving obligations are AD-2, AD-3, AD-4, and AD-5. |"

One further hit is a deliberate drift report about a *different* file, correctly
labelled — `ARCHITECTURE-SPINE.md:505`:

> "| `epics.md` retired register | **Open** | `_bmad-output/planning-artifacts/epics.md` still carries TIR-1..TIR-8, RE-1..RE-11, the 72-criterion controls, and a set-equality requirement against `contracts/tauri-boundary/v1.json`. It contradicts this spine and `docs/DECISIONS.md` D33; reconciling it was out of scope for this run. See `DRIFT-NOTE.md`. |"

This is not spine residue: it names the retired vocabulary only to flag it as
living somewhere else. It is the correct handling. The remaining work is in
`epics.md`, not here.

Positive confirmations of D33 compliance in the rule set:

- The surviving habit is carried as a rule, `ARCHITECTURE-SPINE.md:115-117`:
  > "- **Rule:** Missing or incorrect behavior is product work, not test work. Before
  >   scheduling anything described as a test gap, verify whether the behavior is
  >   already present in the shipping code (`docs/DECISIONS.md` D33)."
  matching `docs/DECISIONS.md:344-346`.
- The replacement acceptance model matches D33 exactly. Spine `ARCHITECTURE-SPINE.md:209-212`
  vs `docs/DECISIONS.md:335-337`.
- The rescope arithmetic checks out: spine `ARCHITECTURE-SPINE.md:499` says
  "Six stories survive... 31 stories archived"; `docs/DECISIONS.md:350-351` says
  "6 keep, 19 merge, 12 retire across the 37 stories" — 19 + 12 = 31, 6 + 31 = 37.
- Spine `binds: Epic UX-PB (28 stories)` (`ARCHITECTURE-SPINE.md:13`) matches
  `docs/DECISIONS.md:324-325` ("28 real product stories (the D27–D30 Upgrade Plan redesign)").

No `contracts/` obligation, no coverage threshold, no evidence manifest, no
candidate freeze, no batch wave, and no TIR-n/RE-n/ASR-n enabler survives as a
live rule.

---

## 4. SPEC.md invariants stated in a way that inverts or weakens them

**No inversion found.** Checked all seven load-bearing invariants
(`docs/SPEC.md:13-19`) against the spine:

| SPEC invariant | Spine | Assessment |
| --- | --- | --- |
| 1. Manager's outdated verdict authoritative (`SPEC:13`) | — | **Absent** (F4). Not inverted; simply unstated. |
| 2. Who-manages-whom from RAW path (`SPEC:14`) | — | **Absent** (F4). |
| 3. Homebrew never contended (`SPEC:15`) | AD-4 `:181-185` | Faithful. |
| 4. No sudo, no password (`SPEC:16`) | AD-4 `:172-176` | Faithful, slightly stronger. |
| 5. One failing manager never blanks the others (`SPEC:17`) | AD-16 `:271-272` | Referenced only, as "the Last-good Snapshot rules". |
| 6. Nothing runs that was not staged and shown (`SPEC:18`) | AD-16 `:252-261` | Faithful. |
| 7. Every confirmed attempt reconstructible from disk (`SPEC:19`) | AD-18 `:386-398` | Faithful. |

Detail on the two that warrant a note:

**Invariant 4 is strengthened, not weakened.** `docs/SPEC.md:16`:

> "4. **No sudo, no password entry, ever.** Child stdin is `/dev/null`. Silent processes trigger stall detection with a copy-the-command-to-terminal handoff."

Spine `ARCHITECTURE-SPINE.md:175-176`:

> "string is ever run, no display text is ever split back into arguments, and no
>   `sudo`, password, or administrator route exists."

The spine's "no administrator route exists" additionally absorbs D25a's
pre-flight requirement, `docs/DECISIONS.md:139-143`:

> "- **No admin prompt, ever.** The plugin's macOS installer falls back to AppleScript `with
> administrator privileges` when the bundle's parent directory is not writable. That would
>   break SPEC §1 invariant 4, so `app_update.rs` pre-flights with `access(2)` and parks in
>   `manualInstallRequired` instead of letting the prompt appear."

and AD-4's port list carries the mechanism (`ARCHITECTURE-SPINE.md:169`,
"current bundle and writability"). Correct.

**Invariant 5 is cited rather than stated.** The spine's only reference,
`ARCHITECTURE-SPINE.md:271-272`:

> "  Results distinguish mutation failure from verification failure while preserving
>   the Last-good Snapshot rules."

"the Last-good Snapshot rules" is treated as an external given. That is
acceptable delegation to SPEC, but it means per-manager refresh isolation
(`docs/SPEC.md:17`, "prior snapshots are retained on failure") has no spine
anchor at all, while a live story depends on it — Story 2.2, routed to AD-4
alone (`ARCHITECTURE-SPINE.md:483`). Same structural gap as F4; not a separate
finding.

**Supersession chains verified clean.** The spine correctly reflects, without
reviving superseded text:

- **D23a supersedes D23** — no "UNVERIFIED" mas label anywhere in the spine.
  `grep -iE '\bmas\b|unverified|synthetic'` finds only
  `ARCHITECTURE-SPINE.md:218` (D32's Intel wording) and `:422` ("`mas` is the
  exception: its id segment is the numeric App Store id"). Correct.
- **D25a supersedes D20** — the spine never mentions ad-hoc signing; AD-12
  `:241-242` requires "Apple Developer ID signing and notarization... for a
  published release", matching `docs/DECISIONS.md:144-147`. Correct. The spine's
  six-event baseline (`:73`) also correctly reflects D25a's sixth event
  (`docs/DECISIONS.md:133-136`) over D16's five.
- **D27 supersedes D6's immediate single-Package exception** — the spine records
  the immediate row action only as brownfield fact (`:87-88`, "a single-package
  row action executes immediately"), never as a rule, and AD-16 `:252-257`
  carries forward D6's surviving command-trust and staleness protections per
  `docs/DECISIONS.md:177-178`. Correct.
- **D29 supersedes D12 partially** — AD-18 `:386-392` gives attempts their own
  journal while keeping `operations.jsonl`'s record shape, matching
  `docs/DECISIONS.md:214-216`. Correct.
- **D30 supersedes D18 partially** — the navigation half is uncovered (F5); the
  cancellation half is correct, AD-16 `:265-268` vs `docs/DECISIONS.md:242-243`.

---

## 5. Secondary observations (not findings)

- `.github/workflows/release.yml:61` justifies the runner with a superseded
  decision: `    # ci.yml stays on stable macos-14 per DECISIONS D20; same here.`
  D20 is superseded (`docs/DECISIONS.md:144`, "**D20 is superseded.**"); the live
  citation is D31 (`docs/DECISIONS.md:270`, "CI therefore stays on `macos-14`").
  The spine cites D31 correctly at `:220-221`. Only the workflow comment is stale.
- `settings.autoCheckForUpdates` exists in shipping code
  (`src/components/settings/SettingsView.tsx:200`,
  `                  checked={settings.autoCheckForUpdates}`) but is not in
  `docs/SPEC.md:112`'s settings list. SPEC drift, not spine drift; AD-19's
  tolerate-unknown-fields rule already covers the read path.
- `ARCHITECTURE-SPINE.md:482` routes UX-PB.5a–5e to "AD-17, AD-19", but the
  governing rule for `skipUpgradePlanConfirmation` lives in AD-16
  (`:276-279`). Cosmetic; the capability map should cite AD-16 too.
- `docs/SPEC.md:9` says "Vite 7"; the spine's stack table says
  `| Vite | 8.1.5 |` (`:448`). The spine states it is lockfile-derived
  (`:432-434`) and explicitly "not a version policy", so this is SPEC staleness,
  not a spine contradiction.

---

## Recommended edits, ranked

1. **AD-16 `:284-286`** — add the app menu to the update-surface list, or remove "only". (F1)
2. **Status table `:493`** — record D31's open `notarytool` residual. (F2)
3. **AD-16 `:346-349`** — distinguish item-level ineligibility from plan-composition exclusions (`rustDedup`, `alreadyRunning`). (F3)
4. **AD-4 or Consistency Conventions** — add the RAW-path-before-canonicalize and manager-verdict-authoritative rules. (F4)
5. **AD-17** — fix Activity's surface shape and the `ActivityDrawer`'s disposition. (F5)
6. Lower priority: F6 (one code path for the updater), F7 (output-fidelity floor), F8 (all-or-none admission).
