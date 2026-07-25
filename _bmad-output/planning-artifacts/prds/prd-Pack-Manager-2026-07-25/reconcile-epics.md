# Reconciliation — `epics.md` FR/NFR inventory vs. `prd-Pack-Manager-2026-07-25/prd.md`

**Reviewer:** epics.md FR/NFR block
**Date:** 2026-07-25
**Sources read in full this session:**

- `_bmad-output/planning-artifacts/epics.md` lines 40–459 (the FR/NFR inventory is lines 53–117; the surrounding blocks 119–454 were read for context but are Phase 3 material and out of scope for this diff).
- `_bmad-output/planning-artifacts/prds/prd-Pack-Manager-2026-07-25/prd.md` (all 659 lines).
- `_bmad-output/planning-artifacts/prds/prd-Pack-Manager-2026-07-25/addendum.md` (all 69 lines).
- Corroborating reads: `ux-Pack-Manager-2026-07-23/.memlog.md`, `EXPERIENCE.md`, `docs/SPEC.md` (F5/F11/§8), `src-tauri/src/settings.rs`, `src-tauri/src/logging.rs`, `src-tauri/src/commands.rs`.

## Method

One pass per requirement id. For each, the epics.md clause list is decomposed into
obligations and each obligation is matched against the PRD's same-numbered
requirement (and, where the PRD relocated it, against the FR that absorbed it).
An obligation is "carried" only when a literal PRD sentence states it.

Two differences were pre-declared correct by the review brief and are **not**
reported as findings, though they are recorded below so the diff is complete:

1. **FR-19 / NFR-6 lose their keyboard, VoiceOver, announcement, and
   focus-restoration limbs.** `docs/DECISIONS.md:519` — `## D37. Keyboard
   navigation and screen-reader support are not release criteria`. The PRD
   documents the removal itself at `prd.md:430`.
2. **FR-6 is rewritten from transient selection to direct Upgrade Plan
   membership control.** Owner decision recorded verbatim at
   `ux-Pack-Manager-2026-07-23/.memlog.md:75` — `"(decision) Package checkboxes
   directly control Upgrade Plan membership: checking an eligible Package
   immediately adds it, unchecking immediately removes it, and the header
   checkbox adds or removes all eligible visible Packages. Eliminate the separate
   temporary selection and Add Selected layer."`

## Per-requirement verdict

| Id | Verdict | Note |
| --- | --- | --- |
| FR-1 | Carried in full | All six clauses present at `prd.md:141–147`; PRD additionally requires the result appear in the Environment Report and diagnostics export, and that a partial result never overwrite a complete one. |
| FR-2 | Carried in full | `prd.md:156–160`. PRD strengthens: "Pack-Manager performs no version comparison to decide outdatedness" and the "update available" wording for unknown latest. |
| FR-3 | Carried in full | `prd.md:169–174`. "safe cross-Manager concurrency" lands as "unrelated Managers proceed concurrently" plus FR-9's lock-set serialization. |
| FR-4 | Carried in full | `prd.md:183–188`. PRD adds the fixed Route precedence and the D21 npm-inside-mise consequence. |
| FR-5 | Carried in full | `prd.md:197–203`. The D10 Rust rule is named and bounded ("No broader cross-Manager deduplication is performed"). |
| FR-6 | Rewritten (expected) + **one obligation dropped** | Model change is the declared-correct difference. The *range* membership interaction is not carried — see Finding 3. |
| FR-7 | Carried in full | `prd.md:245–250`. Independent Manager removability lands via `prd.md:249` "offers Remove on every staged item" and `prd.md:302`. |
| FR-8 | Carried in full | `prd.md:259–263`. "missing authorization" lands as "unknown"; "require reconfirmation" as "returns the user to review with a newly issued plan". PRD adds the 64-capability bound. |
| FR-9 | Carried in full | `prd.md:272–275`. PRD names the concurrency cap (4) that epics left as "the global limit". |
| FR-10 | Carried in full | `prd.md:284–287`, clause for clause. |
| FR-11 | Carried in full | `prd.md:298–302`, including the "do not re-entrench a global toggle" warning epics did not have. |
| FR-12 | Carried in full | `prd.md:319–323`. PRD adds the constructed-environment rule. |
| FR-13 | **One obligation dropped** | Sidecar-as-live-progress-and-Results is not carried anywhere in the PRD — see Finding 1. Everything else present at `prd.md:332–336`. |
| FR-14 | Carried in full | `prd.md:345–350`, clause for clause, plus the never-enters-passwords statement and the SIGTERM→5s→SIGKILL escalation detail. |
| FR-15 | **One obligation dropped** | Durable per-attempt *command snapshot* — see Finding 4. Everything else present at `prd.md:359–364`, with retention numbers epics left as "as specified". |
| FR-16 | Carried in full | `prd.md:373–376`, clause for clause. |
| FR-17 | **One obligation dropped** | *Live* log level — see Finding 2. The eight shipping fields at `prd.md:394` match `src-tauri/src/settings.rs:29–38` exactly (verified: 8 fields, `auto_check_for_updates` present). |
| FR-18 | Carried in substance | `prd.md:407–411`. Named artifacts generalized — `report.json` → "an environment and detection report", `operations.jsonl` → "the History journal", "ZIP" → "archive", "constructed ToolEnv" → "the constructed search path and its source". Content mapping is 1:1; filenames are architecture per `addendum.md:18`. Not reported. |
| FR-19 | D37 limbs removed (expected) + **surface list narrowed** | The Confirmation Dialog and Results drop out of the coherent-interface span — see Finding 5. |
| FR-20 | Carried in full | `prd.md:457–461`. PRD adds the launch/6h/on-demand cadence and the manual-check-only notification rule. |
| FR-21 | Carried in full | `prd.md:470–475`, plus the two-layer refusal (frontend guard + backend refusal) epics did not state. |
| FR-22 | Carried in full | `prd.md:482–485`. "report success only after relaunch as the intended version" relocated to FR-21 (`prd.md:474`); "normal Finder/Dock launch" to FR-1 and NFR-7. Relocation is intact coverage. |
| RP-1 | Carried in full | `prd.md:497`; checklist validation at `prd.md:491`. |
| RP-2 | Carried in full | `prd.md:503`, with the D25a `app.set_menu` rationale added. |
| NFR-1 | Carried in full | `prd.md:513`. |
| NFR-2 | Carried in full | `prd.md:519` — same nine failure classes, plus the merge-not-replace rule. |
| NFR-3 | Carried in full | `prd.md:525` — 100/101 rows, 50 ms / 64 lines / 8 KiB, 5,000 lines, 900 × 600, 150–200%, all six surfaces. Verbatim-equivalent. |
| NFR-4 | Carried in full | `prd.md:531`. |
| NFR-5 | Carried in full | `prd.md:537`, plus the AD-20-adjacent security-sensitivity clause. |
| NFR-6 | D37 limbs removed (expected) | Surviving limbs all present at `prd.md:543`. "accessible ineligibility reasons" correctly narrowed to "pointer-accessible". |
| NFR-7 | Carried, with one clause correctly closed | The "declare the minimum supported macOS version before final candidate acceptance" clause is closed, not dropped: `prd.md:551` cites D31, and `docs/DECISIONS.md:245` is `## D31. Minimum supported macOS is 15.0`. Correct. |
| NFR-8 | Carried, with correct de-scoping | "one Release Candidate" → "one release" (`prd.md:557`); candidate-freeze machinery is retired by D33 per `prd.md:30`. PRD adds the two release-blocking checks. Correct. |

## Findings

### 1. FR-13 — the Upgrade Plan sidecar's post-confirmation life is not carried (medium)

`epics.md:77` requires: `"use the sidecar as live progress and Results; make
Activity a first-class destination"`. The PRD carries the second half and drops
the first.

The PRD's only sidecar obligation stops at confirmation —
`prd.md:249` `"**Planned — D27:** the sidecar is hidden when empty, appears on
first addition, persists across navigation, and offers Remove on every staged
item."` FR-13's Planned bullet then jumps straight to
`prd.md:336` `"Activity as a first-class navigation destination; a terminal
Results summary with successes, failures, skipped work, verification outcomes,
and Retry where appropriate."` The word "sidecar" appears three times in the
whole PRD (lines 240, 249, 430) and never after confirmation.

The upstream obligation is explicit and load-bearing:
`EXPERIENCE.md:153` `"It persists across Manager changes and transforms into
Activity and Results."` and `EXPERIENCE.md:208` `"The sidecar and full Activity
are two presentations of one shared live state, never separate executions. The
sidecar remains the compact live summary while full Activity shows detailed
evidence."`

**Why it matters.** The PRD asserts authority — `prd.md:16` `"This PRD is the
requirements authority. \`ARCHITECTURE-SPINE.md\` and \`epics.md\` are reconciled
*against* it, not the reverse."` — and `addendum.md:52` routes epics.md through
`bmad-correct-course`. A correct-course pass that reconciles epics.md to this
PRD has no anchor for the sidecar→Activity→Results continuity and can drop it.
Read from the PRD alone, "Activity as a first-class navigation destination" reads
as *navigate away to see progress*, which is the opposite of the approved design.

**Fix:** add one consequence to FR-13's Planned bullet: the Upgrade Plan surface
transforms into the live plan summary on confirmation and into the terminal
Results summary at completion; full Activity is a second presentation of the same
state, not a separate execution.

### 2. FR-17 — the log level's *live* application is dropped (medium)

`epics.md:85` requires: `"support editable thresholds/live log level"`. The PRD
lists the field but not its behavior —
`prd.md:394` `"application log level (debug for the app's own code)"` — and no
other PRD sentence states that a log-level change takes effect without a restart.

This is shipping behavior with an explicit code contract:
`src-tauri/src/logging.rs:85` `"/// Applies a Settings logLevel change live.
Returns \`false\` (no-op) while"` and
`src-tauri/src/commands.rs:626` `"let applied = handle.apply_settings_level(merged.log_level);"`.
`docs/SPEC.md:112` states the weaker form: `"Install threshold, operation hard
cap, and application log level are editable."`

**Why it matters.** FR-17 is tagged Partial with the shipping limbs called out,
and Shipping/Partial tags are what protect working code from silent regression.
With the liveness obligation gone, a refactor that made log-level changes require
a relaunch would violate no requirement in the authority document.

**Fix:** append to the FR-17 consequence list: a log-level change applies to the
running process without relaunch (noting the documented no-op when the
`PACK_MANAGER_LOG` environment override is active).

### 3. FR-6 — range membership mutation is no longer required, yet the batch rule is justified by it (medium)

`epics.md:63` requires: `"Support exact selection of eligible Outdated Packages
through individual, range, toggle, filter-aware select-all, tri-state, and clear
interactions"`. This is separate from the model rewrite: individual, select-all,
and tri-state all survive the rewrite intact and are restated at
`prd.md:225–226`. **Range does not.**

The PRD nonetheless makes range the sole motivating case for a requirement it
calls non-negotiable — `prd.md:231` `"**Membership mutation must accept a batch.**
A range or filter-wide interaction submits one membership operation covering
every affected Package identity, not one per row. This is a requirement, not an
optimization … so a per-row mapping turns a shift-range across 100 rows into 100
round-trips"`. Nothing in the PRD requires that shift-range to exist.

No live upstream artifact supplies it either. The pointer form survives only
inside the superseded transient-selection paragraph —
`docs/SPEC.md:80` `"header tri-state over visible selectable rows, shift-click
range, Cmd-click toggle, and Cmd+A select all visible. \`Add N to Plan\`
immediately adds the checked canonical identities to the persistent plan and
clears the transient selection."` — which is exactly the F5 text the owner
decision retires. The only other form is keyboard, which D37 removes:
`EXPERIENCE.md:285` `"Shift+Up/Down extends a contiguous membership range from
the current anchor while respecting pinned, current, excluded, and unavailable
eligibility."`

**Why it matters.** Internal incoherence in the authority document: FR-6's batch
rule reads as over-engineering because its trigger is unstated, and an
implementer building strictly from the PRD ships per-row checkbox plus header
checkbox only.

**Fix:** either add "a pointer range interaction adds or removes every eligible
Package between the anchor and the clicked row in one membership operation" as an
FR-6 consequence, or restate the batch rule's justification on the header
checkbox alone (which already spans off-screen virtualized rows). Do not leave
the rule motivated by an interaction no requirement mandates.

### 4. FR-15 — the durable per-attempt command snapshot is dropped (low)

`epics.md:81` requires: `"Durably correlate each confirmed Plan Attempt's reviewed
intent, command snapshot, Operations, verification, Results, and Retry lineage
through \`planAttemptId\`"`. The PRD's two restatements both omit the command
snapshot: `prd.md:364` `"**Planned — D29:** one immutable History row per
confirmed Plan Attempt, with Operation evidence nested inside it and Activity
replay from the row."` and `prd.md:592` `"A durable \`planAttemptId\` correlating
reviewed intent, Operations, events, transcripts, journal records, verification,
Results, and Retry lineage"`. `grep -n -i "command snapshot"` over `prd.md` and
`addendum.md` returns zero lines.

"Reviewed intent" is not a substitute: `prd.md:248` `"The draft stores canonical
intent, never executable display strings. Commands are rebuilt by the backend
whenever the draft changes and again before execution."` A snapshot of the
commands *as reviewed and as spawned* is what lets History answer "what actually
ran" for an attempt without re-deriving it from a changed machine.

**Fix:** add "command snapshot" to the FR-15 Planned correlation list, or state
explicitly that per-Operation transcripts are the durable command record and no
attempt-level snapshot is required.

### 5. FR-19 — the Confirmation Dialog and Results leave the coherent-interface span (low)

`epics.md:89` requires: `"Preserve one coherent dark-only macOS interface across
Dashboard, expandable Manager navigation, Manager workspaces, persistent Upgrade
Plan, separate Confirmation Dialog, Activity, Results, one-plan-per-row History,
Settings, status, and app menus"`. The PRD restates it as
`prd.md:420` `"One coherent dark-only interface spans the Dashboard, Manager
navigation and workspaces, the Upgrade Plan, Activity, History, Settings, status
surfaces, and application menus."` — Confirmation Dialog and Results are out of
the list; "expandable" and "one-plan-per-row" are dropped as qualifiers.

Both surfaces exist elsewhere in the PRD (`prd.md:250` for the modal, `prd.md:415`
naming "the Results surface" as Planned), so this is a scope narrowing of the
visual-coherence obligation rather than a lost surface. It still means the two
newest surfaces in the product are the two not covered by the requirement that
everything look like one app.

**Fix:** restore both to the FR-19 span with a Planned qualifier.

## Non-findings recorded for completeness

- **FR-18 artifact names** (`report.json`, `operations.jsonl`, "ZIP") generalized
  to prose. Content mapping is 1:1 and `addendum.md:18` places transcript/log
  format with architecture. Not a gap.
- **FR-22's relocated clauses.** "report success only after relaunch" and "normal
  Finder/Dock launch" moved to FR-21 and FR-1/NFR-7. Coverage intact.
- **NFR-7's macOS-declaration clause and NFR-8's "Release Candidate" framing.**
  Both correctly closed/de-scoped by D31 and D33 respectively, with the decision
  cited in the PRD text.
- **Planned-tag sanity check.** `planAttemptId`, `plan_attempt_id`,
  `skipUpgradePlanConfirmation`, `skip_upgrade_plan_confirmation`, and
  `InteractionRequired` each return 0 matching files across `src/` and
  `src-tauri/src/` (`grep -rIl … | wc -l`). The PRD's claim at `prd.md:24` holds
  and no FR is mis-tagged on that basis. `src-tauri/src/settings.rs` declares
  exactly the 8 fields FR-17 lists.
