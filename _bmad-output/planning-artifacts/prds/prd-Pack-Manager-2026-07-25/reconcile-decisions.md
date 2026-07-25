# DECISIONS.md ↔ PRD reconciliation

**Reviewer lens:** `docs/DECISIONS.md` (D1–D37)
**Subject:** `prd.md` + `addendum.md` (`prd-Pack-Manager-2026-07-25/`)
**Date:** 2026-07-25
**Method:** every decision read in full, then matched against the PRD text that
claims it. Every claim below is a literal quote with `path:line`. Counts come
from `grep -c` / `grep -n`, never estimate.

## Scope exclusions honored

Per the review brief and the decisions themselves, the following are **not**
reported as gaps and were not looked for as such:

- D33's retired apparatus — the 72-criterion P0 gate, coverage percentages, the
  55 versioned scenario contracts, the evidence manifest, candidate freeze, the
  multi-host environment requirements, and `contracts/`. Their absence is
  correct.
- D37's removed criteria — keyboard operability of primary actions, VoiceOver
  operability, live-region announcements, deterministic focus restoration.
  `docs/DECISIONS.md:551` — "must not reinstate the removed criteria or report
  their absence as a gap."
- Commercial-launch expectations. `docs/DECISIONS.md:317` — "Pack-Manager is a
  personal, open-source macOS utility, not a commercial product."
- Planned-but-unimplemented D27–D30 state. Verified: `grep -rn
  "planAttemptId\|plan_attempt_id\|Verifying\|InteractionRequired\|skipUpgradePlanConfirmation"
  src/ src-tauri/src/ | wc -l` → `0`, exactly as `prd.md:24` states.

---

## Citation census

`grep -c "\bD<n>\b"` over each file:

| Decision | prd.md | addendum.md | Decision | prd.md | addendum.md |
| --- | --- | --- | --- | --- | --- |
| D1 | 1 | 0 | D20 | 2 | 0 |
| D2 | 1 | 1 | D21 | 1 | 0 |
| D3 | 1 | 2 | D22 | 1 | 1 |
| D4 | 0 | 1 | D23 | 0 | 0 |
| D5 | 0 | 0 | D23a | 0 | 0 |
| D6 | 1 | 0 | D24 | 0 | 0 |
| D7 | 0 | 2 | D25 | 1 | 2 |
| D8 | 0 | 0 | D25a | 4 | 1 |
| D9 | 0 | 1 | D26 | 1 | 0 |
| D10 | 2 | 0 | D27 | 16 | 1 |
| D11 | 0 | 1 | D28 | 6 | 0 |
| D12 | 0 | 1 | D29 | 7 | 0 |
| D13 | 0 | 1 | D30 | 10 | 1 |
| D14 | 0 | 0 | D31 | 2 | 0 |
| D15 | 1 | 1 | D32 | 2 | 1 |
| D16 | 0 | 0 | D33 | 7 | 3 |
| D17 | 0 | 0 | D34 | **0** | 1 |
| D18 | 0 | 0 | D35 | 3 | 1 |
| D19 | 1 | 0 | D36 | 1 | 1 |
| | | | D37 | 5 | 3 |

Nine decisions are cited in neither file: **D5, D8, D14, D16, D17, D18, D23,
D23a, D24**. Uncited is not the same as dropped — eight of the nine have their
substance carried without the citation, and the ninth (D23) is correctly gone
because D23a superseded it. Only **D34** is cited exclusively as a rejected
alternative (`addendum.md:40`) with none of its operative content anywhere; see
Finding 4.

---

## Per-decision verdicts

Legend: **✅ reflected** · **⚠️ partial / weakened** · **❌ contradicted** ·
**➖ correctly absent**

### D1 — Base design "quality"

**✅ reflected (by irrelevance).** A candidate-selection record for the original
design bake-off. `prd.md:654` cites the whole range — "`docs/DECISIONS.md`
D1–D37". No PRD sentence is owed.

### D2 — Managers' outdated verdict is authoritative

**✅ reflected.** `docs/DECISIONS.md:11` — "The frontend's VersionDelta highlight
and severity chips are pure string-segment display affordances."
`prd.md:156` — "Pack-Manager performs no version comparison to decide
outdatedness." `prd.md:158` — "VersionDelta styling and severity chips are
display affordances only." Also `prd.md:569` (§6 Non-Goals) — "**A version
oracle.** It never replaces a Manager's outdated verdict with its own
comparison." The rejected alternative is preserved at `addendum.md:30`.

### D3 — Classify managed-by on the RAW resolved path

**✅ reflected.** `prd.md:131` — "ownership is classified from the **raw**
resolved path before canonicalization (because mise shims are symlinks to the
mise binary, so canonicalizing first misroutes uv and npm to Homebrew — D3)".
Evidence-string requirement lands at `prd.md:143` — "a human-readable evidence
string". Algorithm deliberately routed out at `addendum.md:14`.

*Not carried:* `docs/DECISIONS.md:15` — "Named regression test required." Test
strategy, routed away at `addendum.md:19`. Not a finding.

### D4 — Lock-set scheduler

**✅ reflected at outcome level.** `docs/DECISIONS.md:19` — "Semaphore(4) global
cap"; `prd.md:273` — "independent Managers run concurrently within a global
concurrency cap of 4." Lock-set overlap surfaces at `prd.md:261`. Mechanism
(lock structures, aging guard, skip-ahead FIFO) is deliberately excluded at
`addendum.md:16` — "FR-9 requires conflict serialization, concurrency, and
understandable queue states. The scheduler is how."

### D5 — npm self-update in-band override

**✅ reflected, uncited.** `docs/DECISIONS.md:23` — "Order: in-band override →
delegated-to-detected-owner → native → unavailable." `prd.md:184` — "Route
precedence is fixed: in-band override → delegated-if-detected → native →
unavailable." The anti-hardcoding rule lands at `prd.md:183` ("No Route is
hardcoded") and the re-derivation trigger at `prd.md:185` — "Routes are
reconsidered after every fresh Snapshot, because a Manager's own Outdated row
can change the correct Route."

### D6 — All bulk upgrades flow through a plan sheet

**✅ reflected, correctly as the *superseded* side.** `prd.md:289` — "This FR
supersedes D6's immediate single-Package exception. D6's command-trust and
stale-plan protections remain fully in force." That matches
`docs/DECISIONS.md:176-178` exactly. No PRD sentence asserts D6's row-executes-
immediately exception as current.

### D7 — Greedy-only casks = two-call set difference

**✅ reflected.** `prd.md:199` — "Self-updating casks are excluded by default and
grouped separately; including them requires explicit opt-in." Setting default at
`prd.md:394` — "include self-updating casks by default (off)". The provably-wrong
alternative is preserved at `addendum.md:32`.

### D8 — JSON parsers with text-parser recovery; no fabricated deltas

**✅ reflected, uncited.** `docs/DECISIONS.md:35` — "the UI never receives a
fabricated delta (Judge 2's mandate)." `prd.md:159` — "An unknown latest version
stays unknown; the UI says 'update available' rather than fabricating a version
or a severity." `prd.md:160` — "Output a parser cannot handle fails that Manager
visibly, with an excerpt, rather than presenting incomplete data as complete."

### D9 — Inventory fixtures captured live before parsers

**✅ reflected by exclusion.** Implementation ordering; routed at
`addendum.md:15` ("parser shape is implementation").

### D10 — rust double-report

**✅ reflected.** `prd.md:203` — "The one permitted cross-Manager deduplication is
the Rust rule (D10): a single Upgrade Plan never contains both mise's
`tool:rust` and rustup toolchains; the mise entry is excluded with a visible
reason. No broader cross-Manager deduplication is performed." Reinforced as a
non-goal at `prd.md:574`.

### D11 — Login-shell PATH probe

**✅ reflected at outcome level.** `prd.md:145` — "Detection succeeds when the app
is launched from Finder or the Dock, not only from a terminal."
`docs/DECISIONS.md:47` — "failure → static fallback, WARN, visible in
Environment Report" lands at `prd.md:147` and `prd.md:409` ("the constructed
search path and its source"). Probe form routed out at `addendum.md:13`.

### D12 — One `operations.jsonl` as journal and history

**✅ reflected as partially superseded.** `prd.md:354` tags FR-15 "Partial ...
Plan Attempt as the History unit and Retry lineage are Planned — D29", which
matches `docs/DECISIONS.md:214-216`. The never-signal rule survives verbatim in
substance at `prd.md:361` — "Recorded process group identifiers are historical
evidence only and are **never** signaled after relaunch — process IDs are
reused."

### D13 — Cancellation is immediate, no confirmation

**✅ reflected.** `prd.md:348` — "Cancellation requires no confirmation dialog,
escalates SIGTERM → 5s grace → SIGKILL over the process group, and promises no
rollback of partially completed work." D30's amendment (label becomes `Cancel
plan`) is correctly tagged Planned at `prd.md:350`.

### D14 — No sudo ever; stall detection; copy-to-terminal handoff

**✅ reflected, uncited.** Every limb lands: `prd.md:345` (120s default, Keep
waiting / Copy command / Cancel), `prd.md:346` ("The stall surface states that
Pack-Manager never enters passwords"), `prd.md:347` (30-minute hard cap),
`prd.md:566` (§6 — "No `sudo`, no password entry, no administrator prompt,
anywhere, for any reason").

### D15 — Pinned formulae never upgradable in-app

**✅ reflected, and strengthened.** `prd.md:198` — "Pinned formulae cannot enter
the Upgrade Plan. The row keeps a visible disabled checkbox for table alignment
and remains an explanatory interaction target: activating it never changes
membership, and it states how to unpin and refresh." Non-goal at `prd.md:571`.

### D16 — Five-event surface

**✅ reflected, uncited, and correctly updated by D25a.** `prd.md:41` — "`events.rs`
defines **6** — `appUpdate:status` is missing from SPEC (D25a accepted it
explicitly)". The PRD records the six-event reality, not D16's stale five.

### D17 — Contract drift guard checked by both sides

**⚠️ thin but defensible.** `prd.md:398` — "It is recorded here because the
byte-equality contract fixtures already carry it" is the only trace. Test seams
are deliberately routed out at `addendum.md:19`. Not a finding: D17 is a test
mechanism, and the PRD does not contradict it.

### D18 — Navigation: Dashboard cards + panes + History + Settings

**✅ reflected as partially superseded by D30.** `prd.md:415` — "Shipping for the
current navigation model. The D30 navigation changes — Activity as a
first-class destination, the Results surface, and one-plan-per-row History —
are Planned." D18's rejected "updates inbox" second home is not reintroduced.

### D19 — Dark-only MVP

**✅ reflected.** `prd.md:420` — "One coherent dark-only interface". `prd.md:600` —
"**Light theme** — the tokens live in one file specifically so this is a value
swap, but dark-only is the v1 commitment (D19)."

### D20 — Ad-hoc-signed `.app`; notarization out of scope

**✅ reflected as superseded on the notarization axis; ⚠️ incomplete on the
runner axis.** `prd.md:483` — "This supersedes D20 and `docs/SPEC.md`'s stale
'notarized DMG is out of scope' line." Correct per `docs/DECISIONS.md:144-147`.
D20's other operative clause, `docs/DECISIONS.md:83` — "CI build-smoke runs on
stable macos-14 runners" — was amended by D34 and is unrepresented. See
**Finding 4**.

### D21 — npm-inside-mise copy at the point of action

**✅ reflected twice.** `prd.md:188` — "The npm-inside-mise consequence —
upgrading the mise-managed Node runtime resets npm and its global packages —
appears permanently at the point of action (D21)." Restated at `prd.md:301`.

### D22 — Homebrew lock contention never retried

**✅ reflected.** `prd.md:275` — "External Homebrew lock contention is detected,
named distinctly, and **never** retried automatically (D22)." Non-goal at
`prd.md:572`; rejected alternative at `addendum.md:33`.

### D23 — mas ships fully implemented but labeled UNVERIFIED

**➖ correctly absent.** Superseded by D23a. `grep -ni "UNVERIFIED" prd.md`
returns no mas-related hit; the only "unverified" in the PRD is `prd.md:485` —
"Intel remains best-effort and unverified (D32)", which is a different subject.

### D23a — mas verified live

**✅ reflected by correct silence.** The PRD makes no unverified-mas claim, and
`prd.md:102` lists `mas` in the closed six with no caveat. D23a's transferable
lesson (`docs/DECISIONS.md:111-112` — "a `_synthetic` fixture validates that a
parser doesn't panic; it cannot validate that the parser is right") is a
fixture-strategy note, routed out at `addendum.md:15`.

### D24 — Fixed-stack review

**➖ correctly absent from a requirements document.** The PRD names no stack. Its
one WebView-facing constraint (`docs/DECISIONS.md:116` — "keep the UI free of
exotic WebView APIs") is functionally covered by `prd.md:425`'s WebKit rule.

### D25 — In-app self-update: check → auto-download → user-clicked restart

**✅ reflected.** `prd.md:457` — "Checks run at launch, every six hours, and on
demand from the application menu." `prd.md:458` — "Automatic **download** is
required behavior, not an optional outcome — installation is the machine
mutation, and that stays user-gated." `prd.md:470` — "A downloaded update
installs only after the user chooses Restart to update." `prd.md:567` restates
the boundary as a non-goal.

### D25a — Consequences accepted

**✅ all five reflected or correctly excluded.**

| D25a consequence | PRD |
| --- | --- |
| Sixth event `appUpdate:status` | `prd.md:41` |
| App menu rebuilt by hand | `prd.md:503` — "`app.set_menu` replaces Tauri's default menu wholesale, so these submenus must be re-declared or the shortcuts die silently (D25a)." |
| No admin prompt ever | `prd.md:473` — "A non-writable install location produces manual-install-required, never an administrator prompt." Also `prd.md:323`. |
| D20 superseded | `prd.md:483` |
| CI build-smoke `--no-sign` | Correctly excluded; `addendum.md:20` routes signing environment to architecture. |

### D26 — One closed literal list of unterminated notices

**✅ reflected.** `prd.md:362` — "Transcript content is faithful to child output,
with exactly one exception: D26's closed, literal list of unterminated `mas`
notices, after which one readability newline may be inserted. No general
heuristic rewriting." "Closed, literal list" carries `docs/DECISIONS.md:156` —
"verbatim strings, never patterns".

### D27 — One persistent Upgrade Plan; no immediate execution

**✅ reflected, thoroughly (16 citations).** Each mandate maps:

| D27 mandate | PRD |
| --- | --- |
| Every path adds to the same editable draft | `prd.md:226`, `prd.md:284`, `prd.md:302` |
| "No row or Manager-header update executes immediately" (`:167`) | `prd.md:230`, `prd.md:287` |
| Sidecar appears after first item, persists across views | `prd.md:249` |
| Ineligible items cannot be added and must explain why | `prd.md:200`, `prd.md:227` |
| "canonical intent rather than executable strings" (`:173`) | `prd.md:248` |
| Supersedes D6's exception | `prd.md:289` |

### D28 — Separate final confirmation gate with reversible preference

**✅ reflected.** `prd.md:250` — "`Confirm N Updates` opens a separate modal
confirmation showing the exact commands, offering `Change Plan` and final
confirmation. The opt-out checkbox appears *only* in that dialog and persists
`skipUpgradePlanConfirmation`. The safe default is confirmation enabled
(`false`)". Settings reversal and `autoOpenDrawer` demotion at `prd.md:396`. The
"removes only the final dialog" guarantee (`docs/DECISIONS.md:195-196`) survives
verbatim in substance at the end of `prd.md:250`.

### D29 — Confirmed plan attempt is the durable unit

**✅ reflected.** Glossary separates capability from identity — `prd.md:113`
("Plan Capability — a bounded, one-use backend authorization") vs `prd.md:114`
("Plan Attempt — one confirmed execution of an Upgrade Plan, durably identified
by `planAttemptId`"), which carries `docs/DECISIONS.md:202-204`'s must-not-reuse
rule structurally. `prd.md:364` — "one immutable History row per confirmed Plan
Attempt ... Retry creates a **new linked attempt** and never overwrites the
first failure. Legacy Operation records without an attempt identity stay visibly
legacy and are never fabricated into plans." Verification refreshes → `prd.md:336`
"a `Verifying` state before success is declared". NFR-4 tagged Partial at
`prd.md:529`.

### D30 — One active plan; explicit Activity and Results; trusted classification

**⚠️ two mandates weakened.** Correctly carried: `Verifying`, Activity as a
destination, Results summary, `Cancel plan` vs `Cancel operation`
(`prd.md:336`, `prd.md:350`), and the interaction classifier (`prd.md:350` —
"`Interaction required` is shown **only** when a closed Manager-specific
classifier or an explicit native signal recognizes a trusted prompt").

Weakened:
1. `docs/DECISIONS.md:222` — "Only one confirmed Upgrade Plan attempt may be
   active at a time." reaches only `prd.md:593`, a §7 scope bullet. See
   **Finding 1**.
2. `docs/DECISIONS.md:224` — "While it runs, the plan sidecar becomes live
   status." and `docs/DECISIONS.md:231-232` — "A queued draft remains in the
   Upgrade Plan, not Activity." are both absent. See **Finding 5**.

### D31 — Minimum supported macOS is 15.0

**✅ reflected on the floor; ⚠️ on its residual open question.** `prd.md:76` and
`prd.md:551` both state 15.0 with the `tauri.conf.json` declaration.
`prd.md:551` — "D31 closed this; it is no longer an open prerequisite for
candidate acceptance." True for DR-1. But D31's own residual —
`docs/DECISIONS.md:271` — "**One question remains OPEN at the time of writing:**"
— was closed by D34, which the PRD never cites. See **Finding 4**.

### D32 — Universal build retained; verification Apple silicon only

**✅ reflected.** `prd.md:482` — "Update metadata publishes **both** architecture
keys, both pointing at the single universal archive; dropping the Intel key
would strand every installed Intel user with no signal (D32)." `prd.md:485` —
"Verification is Apple-silicon only; Intel remains best-effort and unverified
(D32)." Rejected alternative at `addendum.md:41`.

### D33 — Formal readiness gate retired; plan rescoped

**✅ reflected, with one dropped pointer.** `prd.md:30` enumerates the
not-carried-forward apparatus and matches `docs/DECISIONS.md:332-337` item for
item, including the replacement — `prd.md:557` — "the updater's detached
signature is verified against the public key the shipping app embeds, and the
published update metadata is asserted reachable and coherent after upload."
Scale evidence is quoted accurately at `prd.md:71` and `prd.md:607` against
`docs/DECISIONS.md:318-322`. DR-2's surviving substance (contrast, reduced
motion) lands at `prd.md:423-424`.

Dropped: `docs/DECISIONS.md:350` — "Read it before rescheduling any of them."
See **Finding 6**.

### D34 — CI and release build on `macos-15`

**⚠️ operative content absent.** `grep -c "\bD34\b" prd.md` → `0`. Its only
appearance in either file is `addendum.md:40`, a rejected-alternative bullet
about `macos-latest`. See **Finding 4**.

### D35 — Approved palette adopted; focus gets its own ring

**✅ reflected, with one recorded exception dropped.** `prd.md:421` — "the
approved 'Aurora Control Deck' palette in `DESIGN.md`, adopted by D35".
`prd.md:425` carries the focus mechanism, the WebKit rationale, the
`outline-none` prohibition, and the never-the-accent rule. `prd.md:434` carries
the 31-site count matching `docs/DECISIONS.md:541`.

Dropped: `docs/DECISIONS.md:459` — "Exactly one `ring-accent` use deliberately
survives". See **Finding 7**.

### D36 — Bright fills use the palette's dark ink

**✅ reflected.** `prd.md:423` — "Text contrast meets at least 4.5:1 on its
surface." `prd.md:436` — "**Contrast** (D36's guard). It caught text unreadable
to anyone looking at the button — product quality, not accommodation." The CI
guard is recorded at `prd.md:583` — "a CI-asserted focus mechanism and contrast
guard". Rejected alternative at `addendum.md:39`.

### D37 — Keyboard navigation and screen-reader support are not release criteria

**✅ reflected on the removals; ⚠️ on the two protective clauses.** The PRD is
scrupulous about the removals — `prd.md:79`, `prd.md:430`, `prd.md:545`,
`prd.md:659` — and does not reinstate a single removed criterion. The three
survivals at `prd.md:432-436` match `docs/DECISIONS.md:540-548` exactly.

Weakened: D37's two protective clauses are the parts that did not make it.
`docs/DECISIONS.md:564` — "**Rejected:** deleting the shipped focus and ARIA
affordances from `src/`" (**Finding 2**) and `docs/DECISIONS.md:560` —
"**Story UX-PB.1d is not to be deleted**" (**Finding 3**).

---

## Findings

### Finding 1 — HIGH · D30's one-active-attempt invariant never reaches an FR

`docs/DECISIONS.md:222` — "Only one confirmed Upgrade Plan attempt may be active
at a time."

In the PRD this survives only at `prd.md:593`, inside the §7.2 scope list:
"One active attempt at a time, a `Verifying` state before success is declared,
Activity as a first-class destination, a terminal Results summary, attempt-wide
`Cancel plan`, and trusted-classifier-only `Interaction required` (D30)."

No FR states it. The three candidates all omit it:

- `prd.md:261` (FR-8) — "An in-progress state update, revision drift, an active
  refresh, or a lock-set overlap with any pending or running Upgrade,
  SelfUpdate, or HealthFix rejects the submission without enqueueing."
- `prd.md:273` (FR-9) — "Conflicting work serializes; independent Managers run
  concurrently within a global concurrency cap of 4."
- `prd.md:336` (FR-13) — "**Planned — D29/D30:** plan-level progress correlated
  by `planAttemptId` with nested Operations by `opId` ..."

FR-8's lock-set overlap is **not** equivalent. Two confirmed attempts with
disjoint lock sets — say an npm-only attempt and a rustup-only attempt — pass
every condition at `prd.md:261` and admit concurrently, which D30 forbids
outright.

This matters because of the PRD's own routing. `prd.md:14` — "the BMAD workflows
that consume Phase 2 output — `bmad-architecture`, `bmad-ux`, and
`bmad-create-epics-and-stories`" — and `prd.md:18` — "Requirement IDs are
**preserved from the prior artifacts** ... because `epics.md` and
`ARCHITECTURE-SPINE.md` already cite them". Downstream consumes FR ids. A
concurrency invariant parked in a scope bullet has no id to cite and will not
reach a story.

**Fix:** add a consequence to FR-9 (atomic admission is where attempt admission
already lives), tagged Planned — D30: "Only one confirmed Plan Attempt may be
active at a time; a submission is rejected while another attempt is
unterminated, independent of lock-set overlap."

---

### Finding 2 — HIGH · D37's protection of shipped ARIA in `src/` is dropped, and the PRD's drop-list reads as a licence to strip it

D37 states its scope twice, and the PRD carries only the first half.

`docs/DECISIONS.md:536-537` — "This is a decision about **release gates and
unbuilt scope**, not a licence to strip shipped behavior."
`docs/DECISIONS.md:564-566` — "**Rejected:** deleting the shipped focus and ARIA
affordances from `src/`. They cost nothing to keep, and removing working code to
satisfy a scope decision about *plans* would be a regression bought for no
saving."

The PRD's reconciliation note is `prd.md:430`: "**This PRD restates FR-19 and
NFR-6 without those obligations.** Specifically dropped: keyboard operability of
primary actions, VoiceOver operability, live-region announcements of plan
progress/verification/cancellation/failure/completion, and NFR-6's deterministic
dialog/sidecar focus restoration."

`prd.md:432` — "Three things explicitly stay, and none of them is kept as an
accessibility obligation:" — then lists the focus indicator, the ⌘-key map, and
contrast. Shipped ARIA is not among them, and `docs/DECISIONS.md:536`'s
not-a-licence-to-strip sentence appears nowhere in either file.

The affordances in question are live. `grep -rln "aria-" src/ | wc -l` → `25`.
`src/components/shell/ToastHost.tsx:65` — `aria-live="polite"`.
`src/components/primitives/ErrorState.tsx:25` — `role="alert"`.
`src/components/dialogs/UpgradePlanSheet.tsx:286` — `role="alert"`.

`prd.md:16` — "This PRD is the requirements authority." A builder reading
"Specifically dropped: ... live-region announcements" as authority, with no
counter-clause anywhere, deletes `aria-live` from `ToastHost.tsx` — precisely
the regression D37 rejected by name. The PRD protects the focus indicator
explicitly at `prd.md:434` and leaves its sibling unguarded.

**Fix:** append to `prd.md:432`'s preamble, sourced to `docs/DECISIONS.md:564`:
"Dropping these as *release criteria* is not a licence to delete shipped
behavior. The ARIA affordances already in `src/` stay; removing working code to
satisfy a scope decision about plans is a regression bought for no saving."

---

### Finding 3 — MEDIUM · D37's "Story UX-PB.1d is not to be deleted" is missing from the reconciliation queue that will act on `epics.md`

`docs/DECISIONS.md:560-562` — "**Story UX-PB.1d is not to be deleted**: its
pointer-hover explanation of why a Package is ineligible is mouse-facing
behavior, and only its keyboard and VoiceOver limbs are in scope here."

The addendum owns the routing. `addendum.md:52` — "FR-19 (line 89) and NFR-6
(line 113) still carry the D37-removed keyboard/VoiceOver and announcement
obligations; 10 mentions total. FR-17 also still describes
`skipUpgradePlanConfirmation` and `autoOpenDrawer` without status
qualification." Route: `bmad-correct-course`. The carve-out is absent.

The risk is concrete, because the story's *title* is two-thirds removed
criteria. `_bmad-output/planning-artifacts/epics.md:596` — "### Story UX-PB.1d:
Ineligible-control inertness with keyboard, pointer, and VoiceOver explanation".
A correct-course run told to strip D37's ten `epics.md` mentions, reading only
the PRD's reconciliation queue, has every reason to retire the whole story.

The PRD does carry the substance — `prd.md:438` — "Pointer-facing explanations
of *why* a Package is ineligible also stay; only their keyboard and
screen-reader limbs are out of scope" — and the requirement itself survives at
`prd.md:200`. What is missing is the story-level instruction in the table the
executing workflow actually reads.

**Fix:** add to `addendum.md:52`'s `epics.md` row: "Story UX-PB.1d is **not** to
be deleted (`docs/DECISIONS.md:560`) — only its keyboard and VoiceOver limbs are
in scope; its pointer-hover ineligibility explanation is FR-5 behavior that
survives."

---

### Finding 4 — MEDIUM · D34 is invisible, so the PRD credits D31 with a closure D31 itself leaves open

`grep -c "\bD34\b" prd.md` → `0`. Its sole appearance is `addendum.md:40`, a
rejected-alternative bullet: "**`macos-latest` for CI and release runners**
(D34)." None of D34's operative content appears anywhere.

Two consequences.

**(a) A citation that points at an open question.** `prd.md:551` — "**Minimum
supported macOS is 15.0**, declared in `src-tauri/tauri.conf.json` — D31 closed
this; it is no longer an open prerequisite for candidate acceptance." A reader
following that citation lands on `docs/DECISIONS.md:271` — "**One question
remains OPEN at the time of writing:** whether `notarytool` accepts `minos 15.0`
against SDK 14.5." It was closed by `docs/DECISIONS.md:381` — "This also closes
the question D31 left open." — which the PRD never names, while asserting at
`prd.md:635` that "This document carries **no phase-blockers**."

`epics.md` already does this correctly, which is the standard the PRD should
meet. `_bmad-output/planning-artifacts/epics.md:309` — "The `notarytool` `minos
15.0` question is CLOSED by `docs/DECISIONS.md` D34: CI and release moved to
`macos-15`, so the build SDK is no longer behind the declared floor".

**(b) An incomplete supersession record for D20.** `prd.md:483` records only the
notarization half: "This supersedes D20 and `docs/SPEC.md`'s stale 'notarized
DMG is out of scope' line." D20 has a second operative clause —
`docs/DECISIONS.md:83` — "CI build-smoke runs on stable macos-14 runners" —
amended by `docs/DECISIONS.md:377` — "All three move to `macos-15`. D20's
constraint is unchanged and still governs: the runner stays on a **stable**
image, never a beta one". The surviving stable-image constraint is the kind of
delivery-integrity rule NFR-8 exists for, and it is stated nowhere.

**Fix:** cite D34 at `prd.md:551` alongside D31, and add the stable-runner
constraint to NFR-8 (`prd.md:557`) or to `prd.md:483`.

---

### Finding 5 — MEDIUM · D30's Activity / Upgrade Plan ownership boundary is dropped

D30 states which surface owns which state:

`docs/DECISIONS.md:224` — "While it runs, the plan sidecar becomes live status."
`docs/DECISIONS.md:231-232` — "A queued draft remains in the Upgrade Plan, not
Activity."

Neither appears in the PRD. FR-13's D30 limb is `prd.md:336` — "**Planned —
D29/D30:** plan-level progress correlated by `planAttemptId` with nested
Operations by `opId`; a `Verifying` state before success is declared; Activity
as a first-class navigation destination; a terminal Results summary with
successes, failures, skipped work, verification outcomes, and Retry where
appropriate." §7.2's summary at `prd.md:593` names Activity and Results but not
the boundary either.

This is not visual polish — it is the rule that stops the same state existing in
two places, and the PRD states comparable surface-ownership rules elsewhere
rather than deferring them: `prd.md:334` — "Refresh Operations never auto-open a
live surface and never emit a success notification." Without it, epics building
the Upgrade Plan sidecar and epics building Activity can each render a queued
draft and both be conformant.

**Fix:** add to FR-13's Planned bullet: "A queued draft stays in the Upgrade
Plan and never appears in Activity; once the attempt is confirmed and running,
the plan sidecar itself becomes live status."

---

### Finding 6 — LOW · D33's mandatory triage pointer is dropped from §7.3, the exact section it governs

`docs/DECISIONS.md:348-351` — "That triage is recorded per story, with rationale
and `epics.md` citations, in
`_bmad-output/planning-artifacts/story-triage-2026-07-24.md`: 6 keep, 19 merge,
12 retire across the 37 stories. Read it before rescheduling any of them."

`prd.md:599` invokes the habit and reproduces the supporting number, but names
no file: "`[NOTE FOR PM]` — these were P1 in `docs/SPEC.md`, but D33's surviving
habit applies: verify whether each already ships before scheduling it as new
work. An adversarial triage pass overturned 14 of 20 initial keep verdicts for
exactly that reason."

§7.3 Deferred is the rescheduling surface D33's imperative was written for. The
file exists (`ls _bmad-output/planning-artifacts/story-triage-2026-07-24.md` →
present) and carries per-story verdicts a PM would otherwise re-derive.

**Fix:** name the file in `prd.md:599`.

---

### Finding 7 — LOW · D35's one deliberate `ring-accent` survivor is unrecorded, and the addendum's "everywhere" overshoots it

`docs/DECISIONS.md:459-464` — "Exactly one `ring-accent` use deliberately
survives, at `src/components/manager/PackageRow.tsx:85`. It is a cross-manager
navigation highlight (`src/store/ui.ts:45`), not a focus state, and it has no
`focus-visible:` prefix. Repointing it would have made a navigated-to row
indistinguishable from a focused control, which is the exact confusion the
accessibility floor forbids."

It is still live and CI-pinned:
`src/components/manager/PackageRow.tsx:85` — `highlighted ? "ring-2 ring-inset
ring-accent" : "",`
`src/components/manager/managerPane.test.tsx:114` —
`expect(highlighted.className).toContain("ring-accent");`

`prd.md:425` is scoped correctly and does not contradict it — "Every interactive
element carries a visible focus indicator, drawn as a real `outline` — never a
`ring-*` box-shadow" — the subject is the focus indicator. The addendum is
looser. `addendum.md:38` ends: "One mechanism — `outline` + `outline-offset` —
everywhere." Read as written, "everywhere" sweeps in the navigation highlight
D35 preserved on purpose.

Also minor: `prd.md:425` says "drawn as a real `outline`" and omits
`outline-offset`, which `docs/DECISIONS.md:447-450` ties to a closed SPEC gap —
"`outline-offset` is also precisely what SPEC §4.1's 'offset against surface'
was already asking for, so this closed that gap rather than deferring it."

**Fix:** narrow `addendum.md:38` to "everywhere focus is drawn", and add the
exception: "One `ring-accent` survives at `PackageRow.tsx:85` as a cross-manager
navigation highlight, not a focus state (D35)." Add `outline-offset` to
`prd.md:425`.

---

## Summary

37 decisions reviewed. **Zero contradictions** — no PRD statement asserts a
superseded position as current. Every supersession chain the brief flagged is
handled correctly in the PRD's own text:

| Chain | PRD handling |
| --- | --- |
| D23a > D23 | ✅ no unverified-mas claim survives |
| D25a > D20 | ✅ `prd.md:483` (notarization axis; runner axis → Finding 4) |
| D27 > D6 | ✅ `prd.md:289` |
| D29 > D12 | ✅ `prd.md:354`, `prd.md:364` |
| D30 > D18 | ✅ `prd.md:415` |
| D34 > D31's open question | ⚠️ closure asserted, D34 uncited → Finding 4 |
| D35 > stub palette | ✅ `prd.md:421` |
| D37 > D33's VoiceOver clause | ✅ `prd.md:430` |

All seven findings are **silent drops**, not conflicts — mandates a decision
makes that the PRD does not carry. Two are high: D30's one-active-attempt
invariant never reaching an FR id (Finding 1), and D37's protection of shipped
ARIA being absent while its removal list is stated with full authority
(Finding 2). Neither is a phase-blocker; both are single-paragraph edits.
