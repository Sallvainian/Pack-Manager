# PRD Review — Done-ness and Testability

**Reviewer lens:** done-ness and testability
**Target:** `_bmad-output/planning-artifacts/prds/prd-Pack-Manager-2026-07-25/prd.md` (659 lines) with `addendum.md` (69 lines)
**Date:** 2026-07-25
**Verified against:** `HEAD` `5972109`

## Scope honored

Per the review brief, this pass does **not** report: the retired D33 readiness apparatus
(72-criterion gate, coverage percentages, scenario contracts, evidence manifest,
candidate-freeze, multi-host, `contracts/`); the D37-removed keyboard/VoiceOver/live-region/
focus-restoration criteria; commercial-scale rigor (adoption metrics, rollout, sign-off, SLA);
Planned-but-unimplemented D27–D30 state as a defect; or mechanism deliberately mapped to
`addendum.md` §1. The four already-known findings (FR-14 quit-guard mis-tag, §7.3 health-fix
deferral, §7.1 contrast-guard claim, FR-6's incomplete immediate-execution site list) are not
restated; three findings below are deliberately adjacent to them and say so.

## Verification performed

Every numeric bound in the PRD was checked against source. **All of them hold**, and that is
worth recording because it is the part of this document that is most testable:

| PRD claim | Source | Result |
| --- | --- | --- |
| stall 120s, hard cap 30 min | `src-tauri/src/settings.rs:46` `stall_after_secs: 120`, `:47` `upgrade_hard_cap_mins: 30` | ✅ |
| eight shipping settings fields | `src-tauri/src/settings.rs:29-38` (`Settings` struct) | ✅ 8 |
| SIGTERM → 5s → SIGKILL | `src-tauri/src/process/runner.rs:57` `pub const TERM_GRACE: Duration = Duration::from_secs(5);` | ✅ |
| 64 unconsumed capabilities | `src-tauri/src/state.rs:25` `pub const ISSUED_PLAN_LIMIT: usize = 64;` | ✅ |
| concurrency cap 4 | `src-tauri/src/queue.rs:48` `pub const MAX_CONCURRENCY: usize = 4;` | ✅ |
| 50 ms / 64 lines / 8 KiB flush | `src-tauri/src/events.rs:185` `pub const BATCH_MAX_BYTES: usize = 8 * 1024;`, `:358` `batch_500_lines_flush_in_le_64_line_batches_at_ge_50ms` | ✅ |
| newest 5,000 live lines | `src-tauri/src/queue.rs:52` `pub const RING_CAP: usize = 5000;`, `src/store/operations.ts:17` `export const LOG_CAP = 5000;` | ✅ |
| logs 14d, transcripts 200/90d | `src-tauri/src/logging.rs:26-28` | ✅ |
| History compacted to 1,000 | `src-tauri/src/journal.rs:19` `pub const COMPACT_KEEP: usize = 1000;` | ✅ |
| newest 3 logs, newest 25 transcripts, Desktop | `src-tauri/src/diagnostics.rs:22-23`, `:4` | ✅ |
| six-hour app-update check | `src-tauri/src/app_update.rs:27` `AUTO_CHECK_INTERVAL … from_secs(6 * 60 * 60)` | ✅ |
| focus indicator across 31 sites | `grep -rn "focus-visible:outline\|focus:outline" src/ \| wc -l` → `31` | ✅ |
| 28 stories under `epic-ux-pb` | `grep -c "^  ux-pb-" _bmad-output/implementation-artifacts/sprint-status.yaml` → `28` | ✅ |
| 900 × 600 floor | `src-tauri/tauri.conf.json:18-19` `"minWidth": 900,` `"minHeight": 600,` | ✅ |
| "complete transcript … on disk" | `src-tauri/src/process/runner.rs:171-172` `return; // cap reached — transcripts still carry the full output` | ✅ (512 KiB cap is in-memory only) |

Structural counts:

```
grep -c '^#### FR-'              prd.md → 22
grep -c '^#### NFR-'             prd.md → 8
grep -c '^#### RP-'              prd.md → 2
grep -c 'Consequences (testable)' prd.md → 22
grep -c '^\*\*Status:\*\*'        prd.md → 32
```

All 22 FRs carry a `Consequences (testable)` block. **Zero** of the 8 NFRs and **zero** of the
2 RPs do. That asymmetry is the root of findings 1, 6, and 8.

---

## Findings

### 1. HIGH — RP-1 and RP-2 claim a validation route that does not exist, and carry no consequences

`prd.md:491`:

> "These two are mandatory prerequisites rather than product features. They are validated through `docs/RELEASE-CHECKLIST.md`."

`grep -n "RP-1\|RP-2" docs/RELEASE-CHECKLIST.md` returns **no output**. Walking RP-1's five
clauses (`prd.md:497`) against the checklist's nine steps:

> "Launch, six-hour, and app-menu update checks are preserved; in-process update state survives supported UI recreation; the saved trigger policy survives a normal relaunch; failed or interrupted downloads never present as Ready; and application-update state stays separate from the Operation queue and History."

- launch / six-hour / app-menu triggers — no checklist step
- "in-process update state survives supported UI recreation" — no checklist step, **and the
  term is undefined**: nothing in the PRD says what recreation is "supported" (React remount?
  window close/reopen? webview reload?), so there is no observation to make
- "saved trigger policy survives a normal relaunch" — no checklist step
- "failed or interrupted downloads never present as Ready" — no checklist step
- "application-update state stays separate from the Operation queue and History" — no checklist step

Zero of five. The nearest steps are 8/8a/8b (`docs/RELEASE-CHECKLIST.md:76-95`), which validate
FR-21's install/relaunch/refusal/no-admin-prompt behavior, not RP-1.

RP-2 fares better but not cleanly: `docs/RELEASE-CHECKLIST.md:97` covers "⌘X / ⌘C / ⌘V / ⌘A work
in the package search field and in every `CopyableCommand`", while `prd.md:503` scopes RP-2 to
"Standard **Edit and Window** menu actions". No step exercises the Window menu.

**Why it matters:** §4.6 designates these as *mandatory prerequisites*. A prerequisite whose
stated verification method doesn't mention it, written as a five-obligation semicolon run-on with
no `Consequences (testable)` block, cannot become an acceptance criterion. Downstream story
creation will either skip RP-1 entirely or invent its criteria.

**Fix:** give RP-1 and RP-2 the same `Consequences (testable)` treatment as the 22 FRs, one bullet
per obligation; define "supported UI recreation" by naming the recreations in scope; and either
add the corresponding steps to `docs/RELEASE-CHECKLIST.md` or replace §4.6's claim with the
actual route (CI, manual, or unverified).

---

### 2. HIGH — `Verifying` is required but never defined, and Retry has no precondition

`prd.md:336` (FR-13, the only place either appears in a requirement):

> "**Planned — D29/D30:** plan-level progress correlated by `planAttemptId` with nested Operations by `opId`; a `Verifying` state before success is declared; Activity as a first-class navigation destination; a terminal Results summary with successes, failures, skipped work, verification outcomes, and Retry where appropriate."

Nothing in the PRD says what verification *checks*, what counts as verified, or what the state
machine does when verification contradicts the Operation's exit status. "Retry **where
appropriate**" gives the Retry affordance no precondition at all — yet Retry is load-bearing:
`prd.md:364` requires "Retry creates a **new linked attempt** and never overwrites the first
failure."

This is not deferred mechanism. `addendum.md:11-20` enumerates seven excluded mechanism rows
(search-path construction, ownership classification, adapter/parser shape, scheduler internals,
IPC signatures, transcript syntax, test seams, updater transport). Verification is on none of them.

The decision record is **more specific than its own requirements authority**.
`docs/DECISIONS.md:224-225`:

> "While it runs, the plan sidecar becomes live status. Completed items fill their progress treatment and collapse from `old -> new` to the verified current version."

and `docs/DECISIONS.md:205` names "verification refreshes" as a correlated artifact. The PRD drops
both, keeping only the state name.

**Why it matters:** this FR feeds the 28 `epic-ux-pb` stories. `ux-pb-2b-…` and the Results
stories will be written against "a `Verifying` state before success is declared" with no testable
pass/fail condition.

**Fix:** state what verification does (re-refresh the affected subject and compare against the
Manager's post-upgrade verdict), what a failed verification produces in Results, and replace
"Retry where appropriate" with the enumerated outcome classes that expose Retry.

---

### 3. HIGH — "One active attempt at a time" is a D30 invariant that no FR carries

`grep -n "active at a time\|One active" prd.md` returns exactly one line — `prd.md:593`, inside
§7.2 *Decided, not yet implemented*:

> "- One active attempt at a time, a `Verifying` state before success is declared, Activity as a first-class destination, a terminal Results summary, attempt-wide `Cancel plan`, and trusted-classifier-only `Interaction required` (D30)."

`docs/DECISIONS.md:220` states it normatively: "Only one confirmed Upgrade Plan attempt may be
active at a time."

§7.2 is a scope inventory, not a requirement. The FRs that would have to enforce it say nothing:
FR-8 (`prd.md:261`) rejects on "a lock-set overlap with any pending or running Upgrade, SelfUpdate,
or HealthFix" — a lock-set test, not an attempt-count test; FR-9 (`prd.md:273`) says only
"Conflicting work serializes; independent Managers run concurrently within a global concurrency
cap of 4", which permits two concurrent attempts over disjoint lock sets; FR-13 and FR-14 assume a
single attempt without requiring one.

**Why it matters:** this is the constraint that makes plan-level progress, attempt-wide `Cancel
plan`, and a single terminal Results surface coherent. Implemented from FR-8/FR-9 as written, two
disjoint attempts can run concurrently and the rest of D30 collapses.

**Fix:** add the single-active-attempt admission rule as a consequence of FR-8 or FR-9, tagged
Planned — D30, so §7.2 summarizes a requirement instead of being the only home for it.

---

### 4. HIGH — SM-2 is the only status-unqualified absolute in a document built on Current-vs-Target

`prd.md:612`:

> "- **SM-2: Zero unreviewed mutations.** No Package or Manager update ever runs that the user did not see staged first. A single violation is a P0 defect, not a metric miss. Validates FR-7, FR-8, FR-10."

FR-10 is `prd.md:279` "**Status:** Planned — D27." FR-6 records the shipping behavior at
`prd.md:230`: "Today `ManagerPane.upgradeRow` builds a single-Package plan and calls `executePlan`
immediately". `docs/RELEASE-CHECKLIST.md:53` instructs the release tester the opposite way:

> "Two paths deliberately **bypass** the gate today and must not be reported as failures"

Every FR in §4 carries Shipping / Partial / Planned per `prd.md:20-24`. SM-2 carries none, so read
literally the 1.0.1 build fails the PRD's primary success metric by design — while the release
checklist forbids reporting exactly that. A metric that the current release is required to fail
and forbidden to flag cannot be measured.

*Adjacent to known finding (d), not a restatement:* (d) concerns FR-6's incomplete enumeration of
immediate-execution call sites. This concerns SM-2 carrying no Current/Target split.

**Fix:** qualify SM-2 the way the FRs are qualified — "Target — D27: zero unreviewed mutations.
Today two paths (`Update Package`, Manager self-update) bypass the gate by design; the metric
becomes enforceable when Epic UX-PB lands."

---

### 5. HIGH — FR-16's "per error class" has no class enumeration anywhere

`prd.md:375`:

> "- Errors state what happened and what to do next, in plain language, per error class."

The PRD contains no error-class list; `addendum.md` §1 does not route one to another artifact
(the seven exclusion rows cover path construction, ownership, parsers, scheduler, IPC, transcript
format, test seams, updater transport — not the error taxonomy). Two unfalsifiable terms stack:
"in plain language" and "per error class".

A closed 12-code taxonomy already ships — `src/lib/ipc/types.ts:68-81`:

```
export const ERROR_CODES = [
  "tool_not_found", "spawn_failed", "timeout", "non_zero_exit",
  "brew_lock_busy", "parse_failed", "cancelled", "self_update_unavailable",
  "plan_stale", "env_capture_failed", "io", "internal",
] as const;
```

with per-code copy at `src/lib/errors.ts:13-25` (`ERROR_TITLES`) and `:31-48` (`ERROR_FALLBACKS`).

**Why it matters:** FR-16 is tagged Shipping, so a story derived from it becomes a regression test.
"Errors are in plain language per error class" is not assertable; "each of the 12 `ErrorCode`
values maps to a title and a next-action message" is.

**Fix:** cite the taxonomy by name and count in FR-16, or route it explicitly through
`addendum.md` §1 with its owning artifact. Replace "in plain language" with the observable:
each class states what happened **and** a next action.

---

### 6. MEDIUM — FR-19 is tagged Shipping while meeting the document's own definition of Partial

`prd.md:23` defines the tag:

> "- **Partial** — some limbs ship; the rest is named inline."

`prd.md:415`:

> "**Status:** Shipping for the current navigation model. The D30 navigation changes — Activity as a first-class destination, the Results surface, and one-plan-per-row History — are Planned."

Some limbs ship; the rest is named inline. That is Partial, tagged Shipping with a qualifier the
tag vocabulary does not have. FR-7, FR-11, FR-13, FR-14, FR-15, and FR-17 all use `Partial` for
precisely this shape (e.g. `prd.md:293` "**Status:** Partial. The Manager title area and Route
explanation ship. Independent removable membership is Planned — D27.").

`prd.md:26` says "The tags are implementation status, not requirement strength" — which makes the
tag the field a reader filters on to answer "what ships today". FR-19 answers wrong under that
filter.

**Fix:** retag FR-19 as Partial with the same inline split it already carries.

---

### 7. MEDIUM — FR-21's "Every update-stage failure is actionable" names no stages and no test

`prd.md:475`:

> "- Every update-stage failure is actionable."

Neither "update stage" nor "actionable" is bounded. The PRD never enumerates the stages (check /
download / verify / install / relaunch are inferable from FR-20's "Checking, available,
downloading, ready, and failure states are visible" at `prd.md:459`, but that is a *state* list on a
different FR, and it omits install and relaunch). "Actionable" has no definition — no requirement
that the failure names a next action, offers a retry, or links a log.

Contrast the two consequences immediately above it, which are testable: `prd.md:473` "A
non-writable install location produces manual-install-required, never an administrator prompt."
(shipping, `src-tauri/src/app_update.rs:180-183`, and validated at
`docs/RELEASE-CHECKLIST.md:80-87`) and `prd.md:474` "The app relaunches as the intended version,
and success is reported only after that."

**Fix:** enumerate the stages, and define actionable the way FR-16 should — states what happened
and one next action.

---

### 8. MEDIUM — The 900×600 / zoom / >100-Packages obligation is stated four times in three unmeasurable words

Four separate homes, no shared definition:

- `prd.md:206` (FR-5 feature NFR) — "The Package list stays **usable and responsive** beyond 100 Packages."
- `prd.md:427` (FR-19) — "The interface remains **usable** at 900 × 600, at 150–200% zoom, with more than 100 Packages, and with long command output. Narrow widths scroll rather than letting essential content collide."
- `prd.md:525` (NFR-3) — "The interface stays **interactive** beyond 100 Packages, with correct actions reachable at 101 rows. … Navigation, the plan, confirmation, Activity, Results, and recovery all remain **usable** at 900 × 600 and at 150–200% zoom."
- `prd.md:543` (NFR-6) — "**usability** at 900 × 600 and 150–200% zoom, and with more than 100 Packages"

"Usable", "responsive", "interactive", and "usability" are four words for one property, and none
of them carries a threshold — no interaction latency, no frame budget, no scroll-jank ceiling.
Exactly one clause in the set is checkable: "correct actions reachable at 101 rows"
(`prd.md:525`), which is a *reachability* assertion, not a responsiveness one. "Long command
output" (`prd.md:427`) has no bound either, and "essential content" is undefined.

The viewport floor itself is real and grounded (`src-tauri/tauri.conf.json:18-19` `"minWidth":
900, "minHeight": 600`), so the geometry half is fine; only the quality adjective is unfalsifiable.

**Fix:** state the property once with a threshold ("interaction feedback within N ms at 500 rows"
or, if no budget is wanted at this scale, "no visible scroll jank and every action reachable at
101 rows — verified by eye at release"), and have the other three sites cite it rather than
re-word it.

---

### 9. MEDIUM — NFR-6 restates FR-19's consequences and adds nothing

`prd.md:543`:

> "Non-color status cues and pointer-accessible ineligibility reasons; at least 4.5:1 text contrast; reduced motion honored; a visible focus indicator on every interactive element (see FR-19 for the mechanism and why it is not an accessibility obligation); usability at 900 × 600 and 150–200% zoom, and with more than 100 Packages; VersionDelta display-only."

Every clause has a one-to-one FR-19 source:

| NFR-6 clause | FR-19 source |
| --- | --- |
| non-color status cues | `prd.md:422` "All color states carry text or icon equivalents; status chips do not wrap." |
| 4.5:1 contrast | `prd.md:423` "Text contrast meets at least 4.5:1 on its surface." |
| reduced motion honored | `prd.md:424` "The reduced-motion preference disables transitions." |
| visible focus indicator | `prd.md:425` |
| 900×600 / zoom / 100 Packages | `prd.md:427` |
| VersionDelta display-only | `prd.md:426` |

Six of six. The only content NFR-6 adds is the D37 restatement at `prd.md:545`, which is
reconciliation bookkeeping, not a requirement.

**Why it matters:** `bmad-create-epics-and-stories` reads FRs and NFRs as separate inputs. Two
sources for one AC set produces duplicate stories or divergent criteria the moment one side is
edited — and the wording already diverges ("usability" vs "remains usable", per finding 8).

**Fix:** reduce NFR-6 to a pointer at FR-19 plus the D37 note, or move the six clauses out of
FR-19 into NFR-6 and have FR-19 cite it. One home either way.

---

### 10. MEDIUM — The PRD takes three positions on whether *queued* work blocks a quit

- `prd.md:349` (FR-14, consequence): "Quitting with work **in flight** presents an explicit choice and does not silently discard it."
- `prd.md:471` (FR-21, consequence): "Installation and relaunch are **refused** while any Package Operation is queued or running. **Queued counts as active** — admission has already committed to the work, and a restart would drop it unstarted."
- `prd.md:629` (Open Question 1): "**What happens on quit with work *queued* but not running?** The running-Operation quit guard is defined. Queued-only work, application-update installation during Package activity, and OS-initiated shutdown are not."

FR-21 settles the identical question for the install path with an explicit rationale that applies
verbatim to quit ("a restart would drop it unstarted"). OQ1 then declares the same question open
for quit. FR-14's "in flight" resolves to neither reading. Separately, FR-14 requires "an explicit
choice" without naming the options or what each does — an engineer cannot build the dialog from it,
and `prd.md:635` marks all five open questions non-blocking, so nothing forces the resolution.

*Adjacent to known finding (a), not a restatement:* (a) is that no `CloseRequested`/quit handler
exists in `src/` or `src-tauri/src/`. This is that the PRD contradicts itself about the guard's
trigger condition and leaves its option set unspecified, so even a correctly-tagged FR-14 could not
be implemented unambiguously.

**Fix:** apply FR-21's "queued counts as active" rule to FR-14, close OQ1, and enumerate the
choice's options and their effects.

---

### 11. MEDIUM — FR-14 reserves `Cancel operation` for a feature the PRD never defines

`prd.md:350`:

> "The primary cancellation label becomes `Cancel plan` when the whole attempt is affected; `Cancel operation` is reserved for deliberately Operation-scoped diagnostics."

`grep -n "diagnostic" prd.md` returns 10 hits: AJ-5 (`:91`), FR-1's export mention (`:147`), this
line (`:350`), the §4.4 heading (`:380`), FR-17 (`:395`), FR-18's heading and body (`:400`, `:531`,
`:537`), §7.1 (`:583`), OQ5 (`:633`). Every one of them is the **diagnostics export bundle**
(FR-18). No Operation-scoped diagnostic action exists anywhere in the requirement set.

The label rule therefore has an empty domain: a story implementing it cannot determine when
`Cancel operation` should ever appear. The same wording is inherited from
`docs/DECISIONS.md:233-234` ("an Operation-level cancel label is reserved for an explicitly
Operation-scoped diagnostic action"), so the gap predates this PRD — but the PRD is now the
requirements authority and is where it has to be closed.

**Fix:** either name the Operation-scoped diagnostic surface (Activity's per-Operation view is the
likely candidate, given `prd.md:364` "Operation evidence nested inside it"), or state that
`Cancel operation` appears only in that surface, or drop the reservation.

---

### 12. MEDIUM — HealthFix is a first-class Operation type that no FR owns

`prd.md:107` (Glossary):

> "- **Operation** — one queued unit of work: Refresh, Upgrade, SelfUpdate, or **HealthFix**. Has an **executor** (whose binary runs), a **subject** (whose data it changes), and a lock set."

It is then relied on inside two Shipping FRs' consequences — `prd.md:173` "A successful Upgrade,
SelfUpdate, or HealthFix refreshes every affected subject and executor" and `prd.md:261` "a
lock-set overlap with any pending or running Upgrade, SelfUpdate, or HealthFix rejects the
submission without enqueueing" — and defined nowhere. `prd.md:118` glosses the input ("**Health
issue** — a Manager-reported warning about a broken Package or tool environment. Only a narrowly
recognized fix may become runnable") without stating a requirement, and "narrowly recognized" is
itself unbounded.

FR-1…FR-22 contain no requirement for detecting a health issue, surfacing it, gating which fixes
are runnable, or executing one. It ships — `src-tauri/src/ops.rs:31` `HealthFix { issue_id:
String },`, `src-tauri/src/commands.rs:536` `pub async fn run_health_fix(`,
`src/components/manager/HealthBanner.tsx`.

*Adjacent to known finding (b), not a restatement:* (b) is that §7.3 defers a shipping feature.
This is that the FR set has no requirement for it at all, so even correcting §7.3 leaves FR-3's
and FR-8's HealthFix clauses pointing at undefined behavior.

**Fix:** add an FR for health-issue detection and the recognized-fix gate, tagged Shipping, with
the recognition rule stated (which fixes qualify, and what "narrowly recognized" excludes).

---

### 13. MEDIUM — FR-18's "documented Desktop path" is a dangling reference

`prd.md:407`:

> "- A timestamped archive is written to the documented Desktop path."

The PRD never states the path and never says where it is documented. `addendum.md` §1 does not
route it. The other four FR-18 consequences are fully testable by contrast — `prd.md:408` names
exact counts ("the newest three application logs, the newest 25 transcripts, and the History
journal"), all verified above.

The real value is `src-tauri/src/diagnostics.rs:4`:

> "`~/Desktop/Pack-Manager-diagnostics-<YYYYMMDD-HHmmss>.zip` containing:"

**Why it matters:** the archive path and filename pattern are user-observable output, not
mechanism — the addendum's exclusion rationale doesn't reach them. An AC written from this bullet
can only assert "a file appeared on the Desktop".

**Fix:** state the path template inline, as the counts already are.

---

### 14. MEDIUM — FR-20's closing consequence is unfalsifiable

`prd.md:461`:

> "- Package work remains understandable and uninterrupted throughout."

"Understandable" has no bound and no surrounding text supplying one. "Uninterrupted" is nearly
testable — the observable is presumably that a background app-update download neither pauses,
cancels, nor reorders queued or running Operations — but the PRD does not say so, and FR-20's
other four consequences are all crisply testable by comparison (`prd.md:457` "Checks run at
launch, every six hours, and on demand from the application menu."; `prd.md:460` "Only a *manual*
check may surface a notification.").

**Fix:** replace with the observable — background application-update checking and downloading
neither blocks, delays, nor alters the state of any queued or running Operation, and emits no
surface that overlays Package work.

---

### 15. LOW — FR-22 is the only FR with no requirement statement

`prd.md:20` sets the rule:

> "Every FR therefore states the requirement normatively and carries one of:"

`prd.md:477-481`:

```
#### FR-22: Launch normally and accept only authorized updates

**Status:** Shipping.

**Consequences (testable):**
```

Status goes straight to Consequences. All 21 other FRs place a normative sentence between them
(e.g. FR-21 at `prd.md:467` "Nothing about the installed application changes without a deliberate
user action."). FR-22's four consequences are individually strong; the requirement they are
consequences *of* exists only as a heading, so nothing states the property that
"universal + signed/notarized/stapled + authorized-payload-only" is meant to guarantee.

**Fix:** add the missing sentence.

---

### 16. LOW — NFR-5's last clause is a classification, not a requirement

`prd.md:537`:

> "No telemetry. No generic shell surface. Inherited environment values are excluded from logs and diagnostics. Diagnostic export resists symlink substitution. **Any external-content, capability, or permission change is security-sensitive.**"

The first four clauses are testable prohibitions. The fifth asserts a property of a class of future
changes and imposes no obligation — no review requirement, no approval gate, no test. There is
nothing to verify and nothing an engineer would do differently.

**Fix:** state the obligation the classification is meant to trigger (e.g. "changes to Tauri
capabilities, CSP, or filesystem scopes require an explicit decision record entry"), or drop it.

---

### 17. LOW — RP-2's normative text omits the ⌘R/⌘A/⌘L map that FR-19's note says RP-2 covers

`prd.md:435` (FR-19 notes, item 2):

> "2. **⌘X / ⌘C / ⌘V / ⌘A and the ⌘R / ⌘A / ⌘L map** (RP-2). Copy, paste, and refresh are things a mouse user does constantly…"

`prd.md:503` (RP-2 itself):

> "Standard Edit and Window menu actions — including cut, copy, paste, and select-all in the search field and in every copyable command surface — are preserved."

RP-2 mentions no ⌘R, ⌘A-as-refresh-scope, or ⌘L. A story generator reading RP-2 in isolation —
which is the normal path, since RP-2 is the numbered requirement and `prd.md:435` is a note under a
different FR — scopes it to clipboard only.

The shipping map is also larger than either statement. `src/hooks/useKeyboard.ts:7-10`:

> "Map: Cmd+R refresh current (Dashboard: all) · Cmd+Shift+R refresh all · Cmd+U upgrade selected (plan sheet) · Cmd+Shift+U Update Everything (sheet) · Cmd+A select all visible selectable rows · Esc clear selection / close sheet / close drawer · Cmd+L toggle drawer · Cmd+F focus search · Cmd+1..9 sidebar jump."

Nine bindings; RP-2 names four, FR-19's note names seven.

**This is not a D37 finding.** The review brief and `docs/DECISIONS.md:543-546` both keep the
Cmd-key map explicitly in scope; the defect is that the PRD states RP-2's extent twice,
inconsistently, so "preserved" has no fixed referent to regression-test against.

**Fix:** state RP-2's covered set once, in RP-2, and say whether it is the four clipboard actions
or the shipped map.

---

### 18. LOW — Residual unfalsifiable phrases with no surrounding bound

Collected because each is small but each becomes an untestable AC:

- `prd.md:160` (FR-2) — "Output a parser cannot handle **fails that Manager visibly, with an excerpt**, rather than presenting incomplete data as complete." No excerpt bound. Ships as `src/components/primitives/ErrorState.tsx:16` `maxLines = 10,`.
- `prd.md:201` (FR-5) — "Manager-specific detail is preserved **where useful** — uv executables, mise source path, Package kind, Homebrew pinned version." The em-dash list may be an enumeration or examples; as written a reviewer cannot tell whether omitting one is a defect.
- `prd.md:281` (FR-10) — "The user has a **low-friction** path to update one Package". No click or step budget; and `prd.md:285` "It follows **the common confirmation path**" names a term that is not in the Glossary (§3) and appears nowhere else in the document.
- `prd.md:364` (FR-15) — "Legacy Operation records without an attempt identity stay **visibly legacy**". No treatment specified.
- `prd.md:186` (FR-4) and `prd.md:300` (FR-11) — "in plain language". Tolerable here because both bullets pair it with a concrete obligation ("names both subject and executor", "near the Manager identity, not as unexplained metadata"); noted only so the phrase's use in FR-16 (finding 5) is not mistaken for the same shape.
- `prd.md:118` (Glossary, Health issue) — "Only a **narrowly recognized** fix may become runnable." See finding 12.

**Fix:** bound the excerpt; make FR-5's list closed or explicitly exemplary; give FR-10 a concrete
step count or drop "low-friction"; define "the common confirmation path" in §3 or replace it with
"FR-7's draft review and FR-8's capability check"; specify the legacy treatment.

---

## What is strong

Recorded so a revision doesn't damage it:

- **Every numeric bound in the document is correct against `HEAD`.** 15 of 15 checked (table above). This is unusual and is the document's main testability asset.
- **FR-7's `prd.md:246`** — "The exact commands are revealable on demand and are **byte-equal** to the commands actually spawned." Exactly the right shape: an unambiguous predicate.
- **FR-8** — all five consequences are falsifiable, with the capability bound stated (64, oldest-evicted) and the enqueue outcome stated as **nothing**.
- **FR-15's retention bullet (`prd.md:363`)** — three retention rules, three units, all verified.
- **FR-6's batch requirement (`prd.md:231`)** — states the requirement, the reason it is a requirement rather than an optimization, and the NFR it protects, with the cost cited to a specific line.
- **§9's five open questions** are genuinely open rather than disguised gaps, and `prd.md:635` correctly states none blocks a phase.
- **The Current/Target status discipline** (`prd.md:20-26`) is the right answer to a decided-but-unbuilt backlog. Findings 4 and 6 are about the two places it wasn't applied, not about the mechanism.
