# Reconciliation review — ARCHITECTURE-SPINE.md revision 8

**Lens:** independent input reconciliation. For each load-bearing input, what did
the spine fail to carry, carry wrongly, or invert — with particular attention to
whether revision 8's in-place amendments dropped something the earlier wording
held.

**Target as finally reviewed:** `ARCHITECTURE-SPINE.md`, **939 lines, md5
`09dbb79e1d646667a222807081747d8c`**. Every `SPINE:n` anchor below is against that
file.

> **The target moved four times during this review** (877 → 890 → 910 → 939 lines).
> Seven of the sixteen findings raised were resolved in flight and are recorded in
> **§ Resolved during review** rather than dropped, so the fixes are auditable and
> nobody re-raises them. Nine remain open. If the file has moved again, the quotes
> below are verbatim and greppable even when the line numbers have shifted.

**Inputs reconciled against:** `docs/SPEC.md` (§0 load-bearing invariants 1–7,
§5.6, §5.7, §5.10, §7.6, F4/F6/F7/F9/F11); `docs/DECISIONS.md` D22, D25, D25a, D27,
D28, D29, D30, D31, D32, D33; `_bmad-output/project-context.md`;
`ux-.../DESIGN.md`; `ux-.../EXPERIENCE.md`; `_bmad-output/planning-artifacts/epics.md`
(read only). Spine factual claims were additionally checked against the tree:
`src-tauri/src/commands.rs`, `queue.rs`, `state.rs`, `app_update.rs`,
`src-tauri/Cargo.toml`, `src-tauri/capabilities/default.json`,
`src/styles/theme.css`, `tests/e2e/browser-style-contract.spec.ts`,
`.github/workflows/test.yml`, `.github/workflows/ci.yml`,
`docs/RELEASE-CHECKLIST.md`.

---

## Verdict

**Holds, with one HIGH outstanding.** As finally reviewed, the spine reconciles
with its inputs on everything load-bearing. The two CRITICAL defects this review
found — AD-22 mandating a lock discipline that would deadlock, and
"admission empties the draft" being unqualified so a confirmed Retry destroyed the
membership AD-24 exists to protect — are both fixed, correctly and at the right
altitude. So are three of the five HIGHs.

What remains is a set of **quiet requirements that the AD structure never picked
up**, which is the class this review was asked to hunt. The one HIGH (H-2) is the
sharpest instance: AD-17 fixed the status-announcement channel as *polite*, which
silently forecloses the assertive escalation `EXPERIENCE.md` reserves for an
immediate safety action — so the accessibility fix landed with its priority policy
flattened out of it. Below that, four decisions/promises have no home at all:
D22's no-automatic-retry rule, D25a's admin-prompt pre-flight enforcement point,
SPEC §5.7's plan-request bounds, and the question of whether the debug bundle CI
publishes is in scope for AD-26's compile-time exclusion.

SPEC invariants 1–7, D27–D33, and AD-11's corrected accessibility claims all
verify. AD-21 and the rewritten AD-22 and AD-26 are clean.

## Counts

| Severity | Raised | Resolved in flight | **Open** |
| --- | --- | --- | --- |
| CRITICAL | 2 | 2 | **0** |
| HIGH | 5 | 4 | **1** |
| MEDIUM | 6 | 1 | **5** |
| LOW | 3 | 0 | **3** |
| **Total** | **16** | **7** | **9** |

---

# OPEN — HIGH

## H-2 — AD-17 fixes the announcement channel as "polite", dropping EXPERIENCE's assertive escalation for immediate safety actions

`ARCHITECTURE-SPINE.md:558-561`:

> - **Rule:** There is exactly one polite status-announcement channel for plan and
>   attempt progress, owned alongside the sidecar region. Stories announce through
>   it; none adds a second live region for the same information. Two live regions
>   narrating one attempt is a defect, not additive coverage.

`EXPERIENCE.md:322`:

> - One atomic Activity/Results status channel announces plan start, a changed
>   waiting reason, an action-required failure, each Manager's completion summary,
>   and the final plan outcome. **It uses polite priority by default and assertive
>   priority only for an immediate safety action**; it never announces queued rows,
>   progress ticks, or command-output lines.

`grep -c assertive` over the spine returns **0**.

"Polite" is doing load-bearing work here: it is the only priority the spine
permits, and adding a second live region is declared a defect in the same rule. A
story implementing `Interaction required` (UX-PB.3f) or the 120-second stall
handoff (UX-PB.3c) — both immediate safety actions, where the user must act before
a running process is abandoned — has no compliant way to escalate. It either
violates "exactly one polite channel" or builds the second region the rule calls a
defect.

**This is a residual of a fix, not an old gap.** The rule was added to close
`reviews/review-reconcile-ux.md` FINDING 2 ("No AD owns the single atomic
status-announcement channel"), which quoted `EXP:322` in full — *including* the
assertive clause. The clause did not survive the transfer.

Also dropped in the same transfer: the cross-component suppression protocol at
`EXPERIENCE.md:323` — "Brief Notifications suppress duplicate speech when the
status channel already announced the same event and never move focus." The spine
forbids a second *region* but says nothing about a second *voice*, which is exactly
what a toast is, and `DESIGN.md:223` keeps Brief Notification as a live component.

**Remedy direction.** Restate as "exactly one status-announcement channel … polite
by default, assertive only for an immediate safety action", and add one clause
binding Brief Notifications to suppress what the channel already announced.

---

# OPEN — MEDIUM

## M-1 — No spine rule carries "external Homebrew contention is never retried automatically"

Five inputs assert it; the spine asserts it nowhere. `grep -in "automatic"` over
the spine returns **0** hits, and the same grep over `reviews/` returns nothing, so
this is not sitting in the v6 tail either.

- `docs/DECISIONS.md:89` — "## D22. External Homebrew lock contention is detected,
  named, and never retried automatically"
- `docs/SPEC.md:661` — "`BrewLockBusy` is a distinct user-facing state — "Homebrew
  is busy in another terminal. Retry when it finishes." — with NO automatic retry."
- `EXPERIENCE.md:214` — "- Homebrew contention is never automatically retried."
- `DESIGN.md:250` — "- Do not hide failure evidence behind automatic retries or
  imply a retry will solve a deterministic problem."
- `epics.md:69` (FR-9) — "name external Homebrew contention without automatic retry."

AD-25 gives failure a "Retry affordance" (`ARCHITECTURE-SPINE.md:780-781`) but never
says the retry must be user-initiated. This matters more in revision 8 than it did
in revision 7, because AD-16's verification path introduces an *automatic*
re-execution mechanism — `ARCHITECTURE-SPINE.md:367-368`, "A mutating attempt is not
successful until the required affected Manager refreshes complete" — so the spine
now has automatic reruns with no stated boundary against the one rerun the product
forbids. The concrete case a builder will hit: a verification refresh that returns
`brew_lock_busy`. Nothing in the spine stops them from retrying it in a loop, and
UX-PB.3d (verification-gated Results) is where it will be written.

## M-2 — D25a's no-administrator-prompt pre-flight has an invariant but no enforcement point, while the spine schedules that exact call site for porting

`docs/SPEC.md:16` (load-bearing invariant 4):

> 4. **No sudo, no password entry, ever.** Child stdin is `/dev/null`. …

`docs/DECISIONS.md:140-143` (D25a):

> - **No admin prompt, ever.** The plugin's macOS installer falls back to AppleScript `with
> administrator privileges` when the bundle's parent directory is not writable. That would
>   break SPEC §1 invariant 4, so `app_update.rs` pre-flights with `access(2)` and parks in
>   `manualInstallRequired` instead of letting the prompt appear.

The mechanism is real and verified: `src-tauri/src/app_update.rs:180` calls
`install_target_writable()`; `:183` parks in
`AppUpdateState::ManualInstallRequired { version, reason }`; `:241` is the
`nix::unistd::access(parent, nix::unistd::AccessFlags::W_OK)` call.

The spine states the invariant absolutely — `ARCHITECTURE-SPINE.md:221-222`, "no
`sudo`, password, or administrator route exists" — but inside AD-4's rule about
*process requests through `CommandRunner`* (`:218-220`, "Process requests stay
structured argv against resolved absolute executables…"), which is not the path the
updater's AppleScript fallback takes.
`reviews/review-reconcile-decisions.md:381-388` judged that clause sufficient to
absorb D25a and marked it "Correct." I am not reopening that; the invariant is
stated.

What is missing is narrower and still real: the **enforcement point is unnamed, and
the spine simultaneously schedules it for change.**
`ARCHITECTURE-SPINE.md:929` (Decision Status):

> | Porting opener, reveal, restart, current-executable, **writability**, and remaining path/time call sites | **OPEN — owner Story 6.5** | Direct calls today. …

Story 6.5 is told to port the writability call behind a port, with nothing anywhere
recording that this specific call is the only thing standing between the product
and an AppleScript admin prompt. `ARCHITECTURE-SPINE.md:112` lists it as ordinary
brownfield ("bundle-parent writability, and some path/time behavior are still direct
OS" calls) with no safety annotation.

Compare the spine's treatment of the *sibling* app-update guard, which gets its own
row — `ARCHITECTURE-SPINE.md:936`: "App-update safety guard enforcement point |
**OPEN** | The rule that an update is refused while Package work is queued or
running has no stated enforcement point, and the shipping guard is frontend
convention only". The writability pre-flight is the same class of problem, with a
higher consequence, and gets no row.

## M-3 — The plan-request bounds are absent from AD-16's normative minimum, which revision 8 rewrote

`docs/SPEC.md:443`:

> receives a canonical request whose explicit selection has at most **2,048 entries**,
> package IDs of at most **512 bytes**, and exact duplicate manager/package pairs
> removed first-seen-order … The IPC handler stores at most **64 issued plans** …

`_bmad-output/project-context.md:115`:

> At most 64 unconsumed plan capabilities are retained per session
> (`ISSUED_PLAN_LIMIT`, oldest-first eviction); an evicted `planId` must fail closed
> exactly like an unknown or already-replayed one …

`grep -n "2,048\|512\|64 issued\|ISSUED_PLAN"` over the spine returns **0**. It
carries the *eviction semantics* (`ARCHITECTURE-SPINE.md:338-339`, "an evicted
`planId` fails closed exactly like an unknown or replayed one") and none of the
three bounds.

That is in-altitude for this spine, which elsewhere fixes the concurrency cap of 4,
the 120s aging guard, newest-1,000 journal records, newest-25 transcripts, and 720
CSS pixels. And it matters more after revision 8, because the rewritten `PlanIntent`
(`ARCHITECTURE-SPINE.md:410-416`) replaced flat `PackageRef[]` with
`PlanMember<PackageRef>[]` plus a `removed: unique Ref[]` tombstone set — a strictly
larger per-member structure whose bounding the spine now describes only
qualitatively, at `:729`: "Growth is therefore bounded by one session's draft
activity."

UX-PB.1c is the story that seeds membership in bulk from `Update Everything`, and it
has no stated ceiling to build against.

## M-4 — AD-17 hides the accumulating draft until Results are dismissed; EXPERIENCE says the user may keep reviewing it

`ARCHITECTURE-SPINE.md:526-529`:

> While an attempt is non-terminal the region is owned by attempt status, and new
>   membership staged during that attempt accumulates in the canonical draft
>   without displacing it — **surfacing in the region only once the attempt's Results
>   are dismissed.**

`EXPERIENCE.md:100-101`:

> - Only one confirmed Upgrade Plan attempt may be active. **A user may continue
>   reviewing a draft**, but it cannot be confirmed until the active attempt is
>   terminal.

The UX restriction is on *confirming*, not on *seeing*. The spine's precedence makes
the accumulating draft invisible for the whole attempt plus the whole
undismissed-Results window — so a user staging work during a long multi-Manager
upgrade gets no confirmation that anything was staged, and `EXPERIENCE.md:180`
("Membership changes: the `Updates`, `Managers`, and `Commands` counts … update
immediately") cannot hold during that window.

This survived the AD-17 rewrite that fixed H-1: the visibility union went back to
three-way and the retry scope was correctly demoted, but this clause was not
revisited. It binds UX-PB.3a.

## M-5 — AD-26's compile-time gate is open by design in the debug bundle CI publishes as an artifact

AD-26's rewrite is good and its claims verify: `src-tauri/Cargo.toml` declares no
`[profile.*]` section at all, so `ARCHITECTURE-SPINE.md:821-826` is correct that
`debug-assertions` is off in release builds today.

But the gate it chose is `#[cfg(debug_assertions)]` —
`ARCHITECTURE-SPINE.md:815-820`:

> - **Rule:** The automation surface is excluded at **compile time**, never by a
>   runtime selector. The reference shape is the plugin registered under
>   `#[cfg(debug_assertions)]`, so release builds do not contain it at all. …

— and this project builds and **publishes** a debug bundle.
`.github/workflows/ci.yml:88-94`:

> ```
>       - name: tauri build (debug)
>         run: npm run tauri build -- --debug --no-sign
>       - name: Upload .app
>         uses: actions/upload-artifact@v7
>         with:
>           name: Pack-Manager-debug-app
>           path: src-tauri/target/debug/bundle/macos/Pack-Manager.app
> ```

`.github/workflows/ci.yml:5` describes the job as "build-smoke (macos-14, main
only): debug .app bundle, uploaded as artifact", and `docs/SPEC.md:794` records the
same. Under AD-26's reference shape, that artifact would carry
`tauri-plugin-wdio-webdriver` — an embedded WebDriver server — inside a downloadable
bundle of an open-source app (`docs/DECISIONS.md:313-314`, "a personal,
open-source macOS utility").

AD-26's title and rules speak only of "release bits", so nothing is strictly
contradicted. But AD-2's underlying rule is broader —
`ARCHITECTURE-SPINE.md:157-159`: "Any controlled adapter is a construction-time
dependency of a **non-distributable target**." Whether a CI-published debug `.app`
is a non-distributable target is precisely the question AD-26 should answer and does
not. Note that AD-26's third rule already anticipates the *neighbouring* hazard
(`:821-826`, "Enabling `debug-assertions` in a release profile … silently re-admits
the automation surface") while missing the one that exists in the repo today.

**Remedy direction.** Either state that the build-smoke artifact is in scope for the
exclusion — which makes `debug_assertions` the wrong gate, since a dedicated
non-default feature would be needed — or state explicitly that the CI debug artifact
is an accepted carrier, and why.

---

# OPEN — LOW

## L-1 — `project-context.md:136` understates the capability list; the spine is right

`_bmad-output/project-context.md:136`: "the single capability file grants **only
`core:default` and `opener:default`** to the `main` window."

`ARCHITECTURE-SPINE.md:627-629` says three, and the tree agrees —
`src-tauri/capabilities/default.json` lists `core:default`, `opener:default`, and
`core:window:allow-start-dragging`. Input drift, spine correct. Recorded so the next
project-context regeneration picks it up rather than "correcting" the spine to match.

## L-2 — `project-context.md:30` and two `epics.md` lines still say v1.0.0; the spine is right

`_bmad-output/project-context.md:30`: "App version 1.0.0." `epics.md:259` and `:303`
likewise say "shipped in v1.0.0". `ARCHITECTURE-SPINE.md:131-134` and `:861` handle
this correctly by refusing to restate the version ("release-please-owned … it was
1.0.1 as of 2026-07-25 and will move without this document changing"). Input drift;
the spine's approach is the fix.

## L-3 — AD-16's "Member provenance" domain rule and AD-23 overlap

`ARCHITECTURE-SPINE.md:467-473` states the provenance *requirement* in full, then
says "AD-23 fixes the shape this requires"; `ARCHITECTURE-SPINE.md:697-732` states
requirement and shape together. The division (requirement under AD-16, shape under
AD-23) is defensible, but both now carry the "stays removed" guarantee in their own
words, so a future edit can move one without the other. Minor.

---

# Resolved during review

Raised against an earlier copy of revision 8 and fixed before this review closed.
Recorded so the fixes are auditable and nobody re-raises them.

### C-1 (CRITICAL) — AD-22's unbroken critical section deadlocked on the shipping mutex — **FIXED**

The rule required a confirming action to hold `state.plan_coordinator` "once,
unbroken, across validation, admission, and any side effect", and justified it with
"This ratifies the shipping lock discipline rather than extending it". Three
problems: admission re-acquires the same non-reentrant `std::sync::Mutex`
(`src-tauri/src/state.rs:210-212`, "Queue batch admission shares this exact mutex";
`queue.rs:1009-1011`), so the hold self-deadlocks; the guard would have to cross the
`submit_plan_batch` await (`queue.rs:814`), which `_bmad-output/project-context.md:51`
forbids and `clippy -D warnings` rejects; and the shipping path does the opposite of
what the rule claimed — `src-tauri/src/commands.rs:460-465` releases the guard and
hands off `expected_revision` to a recheck-at-admission.

Now correct. AD-22 rule 1 states the invariant as atomicity **against the canonical
revision** rather than against one mutex hold, and rule 2 names the impossibility
outright: "A rule demanding one unbroken hold would not compile, and would deadlock
if it did." AD-22's rules 3–4 (fixed ordering, rider commits only on successful
admission) were sound throughout and survive.

### C-2 (CRITICAL) — "Admission empties the draft" was unqualified, so a confirmed Retry destroyed the membership AD-24 exists to protect — **FIXED**

AD-17, AD-23, AD-24 rule 1, and the normative state diagram all asserted that
admission empties the draft; a confirmed retry mints a `planAttemptId` and reached
`Admitted` only through the `Preview --> Admitted: … draft emptied atomically` arrow.
Result: confirming a retry destroyed work staged during the attempt — verbatim the
outcome AD-24's **Prevents** clause names ("one discarding membership staged during
the attempt").

Now correct. `ARCHITECTURE-SPINE.md:519` reads "Admission transfers custody **of what
it admitted**", `:522` carves out the derived intent, and the diagram gained a
separate `RetryPreview --> Admitted: confirmed, planAttemptId minted, draft NOT
emptied`.

### H-1 (HIGH) — AD-17's four-way visibility union made the retry review hide Results — **FIXED**

The amendment ranked "an open retry review" above "undismissed Results" with "Higher
precedence hides lower content", against `EXPERIENCE.md:231` ("reveals the
failed-item retry scope **inside Results** … Cancel closes the scope and returns
focus to the Retry action"), `EXPERIENCE.md:300`, `DESIGN.md:216` (retry scope is a
required *state of Results Summary*), and `epics.md:976` ("**inline**"). It would
have removed the `Retry` control that `Cancel` must return focus to.

Now correct. `ARCHITECTURE-SPINE.md:533` is back to a three-way union, `:538` states
"The retry scope is **not** a fourth member of that union", AD-24`:751` says it "is
revealed inline in the surface Retry was invoked" from, and the diagram uses
`Terminal --> RetryScope: Retry reveals failed-item scope inside Results`.

### H-3 (HIGH) — the corrected AD-11 created a sixth `epics.md` divergence — **FIXED (recorded)**

`epics.md:260` and `:304` still assert "Neither automated check exists yet" /
"which do not exist yet", which the corrected AD-11 contradicts — and AD-11's claims
verify against the tree exactly: `src/styles/theme.css:51-55`;
`tests/e2e/browser-style-contract.spec.ts:46` (`emulateMedia({ reducedMotion:
"reduce" })`) with `:110`/`:112` asserting `"0s"`; `.github/workflows/test.yml:8-12`
(push + pull_request on `main`); disclaimer quoted identically at
`docs/RELEASE-CHECKLIST.md:95-96`. Left unrecorded, a story would schedule building
a check that runs in CI on every PR — the exact trap AD-1 names.

Now carried as batch item **(f)** at `ARCHITECTURE-SPINE.md:938`. `epics.md` itself
is correctly left unedited this run.

### H-4 (HIGH) — batch item (e) understated: UX-PB.5b states the *reverse* of AD-22's ordering — **FIXED (recorded)**

`epics.md:1050` reads "written atomically, the new value takes effect only after
persistence succeeds, and the plan is admitted" — persist-then-admit, against
AD-22's "validate, admit and mint `planAttemptId`, then persist the rider. A rider
never precedes the admission it rides on." The row had recorded only a missing case.
Item (e) now says "not merely a missing case: its criterion states the *reverse* of
AD-22's fixed ordering".

### H-5 (HIGH) — batch item (b) understated: the `AllEligible` **seed** is a live-predicate intent — **FIXED (recorded)**

`epics.md:580` seeds the draft "as an `AllEligible` intent" — a stored predicate,
which AD-16's rewritten frozen-expansion rule forbids ("the scope predicate never
runs a second time"). The row addressed only the `Then` conversion clause and
asserted "The observable outcome survives", which is true of the conversion and
false of the seed. Item (b) now says "Both halves need work, not just the
conversion" and "Only the conversion half's observable outcome survives".

### M-6 (MEDIUM) — batch item (d) named one of four `epics.md` locations — **FIXED (recorded)**

Besides the register row at `epics.md:308`, the deferral survives at `epics.md:170-171`
(which attributes it to *this spine*: "The native Tauri harness is Deferred
**there**"), at `epics.md:280`, and in Story 6.5's own contract line at `epics.md:1277`
(which cites neither AD-2 nor AD-26). Item (d) now says "four locations still frame
the harness as simply Deferred".

---

# Batch-row accuracy audit

The row is `ARCHITECTURE-SPINE.md:938`. **As finally reviewed, all six items are
accurate and none understates its text.** For the record, items (b), (d), (e) were
corrected during this review (see above) and (f) was added; (a) and (c) were accurate
throughout:

| Item | Verdict |
| --- | --- |
| (a) UX-PB.1b crash/relaunch | Accurate. `epics.md:558-560` does offer both branches; AD-17 takes "the second branch … unconditionally". |
| (b) UX-PB.1c `AllEligible` | Accurate as revised — now covers both the seed and the conversion. |
| (c) UX-PB.4d "a new reviewable draft" | Accurate. `epics.md:976` carries the distinction in an indefinite article, exactly as stated. The related spine-side defect (C-2) is fixed. |
| (d) Story 6.5 / native harness | Accurate as revised — now names four locations. |
| (e) UX-PB.5b | Accurate as revised — now names the inverted ordering, not just the missing case. |
| (f) `epics.md` D33 restatement | Accurate as added. Two locations carry it (`epics.md:260`, `:304`); the row quotes the first. |

**No `epics.md` divergence was found outside the row.** Checked and clean:
UX-PB.2a/2b identity types, UX-PB.2e cancellation scope, UX-PB.3c `Verifying`,
UX-PB.3d verification-gated Results, UX-PB.3f interaction classifier, UX-PB.4a
one-row-per-attempt, UX-PB.4e legacy honesty, UX-PB.5a confirmation gate, UX-PB.5c
bypass, and Stories 2.2, 3.1, 3.2, 3.4, 3.5 — all reconcile with their cited ADs.

---

# What reconciled cleanly

Recorded so the next reviewer does not redo it.

**`docs/SPEC.md` load-bearing invariants** — 1 (`ARCHITECTURE-SPINE.md:236-238`),
2 (`:232-235`), 3 (`:241-245`), 5 (`:775-778`, cited explicitly), 6 (AD-16's "No
entry point executes", `:329-333`), 7 (AD-18). Invariant 4 is stated (`:221-222`)
with the enforcement-point caveat at M-2.

**`docs/DECISIONS.md`** — D25, D25a, D27, D28, D29, D30, D31, D32, D33 all carried
with correct citations. D22 is the one decision with no home (M-1).

**AD-11's corrected accessibility claims** — every one verified against the tree
(quoted under H-3 above). This correction was right to make, and the release
checklist already matches it.

**AD-21** — the closed set matches AD-4's coordinator-first list exactly; the
fail-closed classification default is the safe direction and correctly leaves
`stallAfterSecs` / `upgradeHardCapMins` plan-determining by omission; the
"splitting the revision does not split the lock" rule preserves AD-4 while
narrowing only the bump. AD-21 contradicts and weakens nothing — the cleanest of
the six new invariants, and the only one that needed no repair during this review.

**AD-25** — a real gap closed; it is now the referent AD-16's verification rule
cites, and the merge-not-replace rule matches `_bmad-output/project-context.md:121`
and `docs/SPEC.md:408`.

**AD-23** — the tombstone-lifetime rule closed two findings this review had opened
against it (unbounded growth; fate at admission).

**AD-26 (rewritten)** — its factual claims verify: no `[profile.*]` in
`src-tauri/Cargo.toml`, so the `debug_assertions` gate holds for release builds. Its
rules now cite AD-2 rather than restating it. The remaining gap is the debug bundle
CI publishes (M-5), not the release path.

**Redundancy check.** Answering the question directly: no new AD (21–26) now
restates something already covered. AD-26's second rule did duplicate AD-2's second
rule nearly verbatim in an earlier copy; the rewrite replaced the duplication with a
citation. The only remaining overlap is L-3, between AD-16's domain rule and AD-23.

---

## Verification pass — fixes applied

**Target:** `ARCHITECTURE-SPINE.md`, **948 lines, md5
`5d337a4e3646982c385eaa8b7238be23`**. Scope: only the four fix areas named by the
owner, checked against `DESIGN.md`, `EXPERIENCE.md`, `docs/SPEC.md`,
`docs/DECISIONS.md`, `_bmad-output/project-context.md` — plus the tree, where a fix
cites it.

| Fix | Verdict |
| --- | --- |
| 1. C-2 — custody transfer scoped to what was admitted | **HOLDS** |
| 2. H-1 — visibility reverted to a three-way union | **WEAKENED** (V-1 HIGH, V-2 MEDIUM) |
| 3. C-1 — AD-22 rewritten over the canonical revision | **HOLDS** (V-3 MEDIUM) |
| 4. Batch rows (b) / (d) / (f) | (b) **HOLDS**; (d) **WEAKENED** (V-4 LOW); (f) **WEAKENED** (V-5 LOW) |

New findings this pass: **1 HIGH, 2 MEDIUM, 2 LOW.**

---

### 1. C-2 fix — custody transfer scoped to what was admitted — **HOLDS**

All four sites now agree, and each states the same rule from its own side rather
than deferring to the others.

- `ARCHITECTURE-SPINE.md:527-533` (AD-17): "Admission transfers custody **of what it
  admitted**. Admitting the draft's own preview empties the draft atomically with the
  mint of `planAttemptId` … Admitting a derived intent — a retry scope (AD-24) —
  consumes that intent and leaves the draft and its tombstones untouched, because
  the draft was never its source. **Minting a `planAttemptId` is not by itself the
  trigger; being the admitted intent's source is.**"
- `ARCHITECTURE-SPINE.md:749-753` (AD-24 rule 1): "Admission of the draft's own
  preview empties it as custody transfer (AD-17) … **A confirmed retry does not
  empty the draft** — it admits a derived intent, not the draft, so staged
  membership and its tombstones survive it."
- `ARCHITECTURE-SPINE.md:728-732` (AD-23): "an admission that empties the draft
  carries them off with it, while a retry admission leaves both untouched (AD-24)."
- Diagram `:460` / `:469` / `:470`: `Preview --> Admitted: … draft emptied
  atomically`, `RetryPreview --> Admitted: … draft NOT emptied`,
  `RetryPreview --> Terminal: admission rejected, original result unchanged`.

**No site still makes the mint the trigger.** I checked every occurrence of
`mint` / `empt` / `custody` / `tombstone` in the file (`:348`, `:460`, `:469`,
`:527-533`, `:730`, `:750`, `:769`, `:854`, Structural Seed, Capability map). The
only remaining "emptied atomically" is on the draft-preview arrow, where it belongs.
`:769-772` states the retry mint with no emptying claim attached.

**Against the inputs.** AD-24 rule 5's rejected-retry behaviour ("nothing is
admitted, the original result stays immutable, and the persistent draft is
unchanged", `:775-778`) matches `epics.md:979-981` and `EXPERIENCE.md:231` ("The
completed result remains immutable in History with a `View previous result` path").
`docs/DECISIONS.md:209-211` (D29) is satisfied: "A retry creates a new attempt and
links back to the failed attempt; it never overwrites the first failure."

### 2. H-1 fix — three-way union with the retry scope as a content state — **WEAKENED**

The core of the fix is right. `ARCHITECTURE-SPINE.md:541-545` is back to the
three-way union, and `:546-553` states the retry scope is "**not** a fourth member
of that union. It is a content state *inside* the surface the user invoked Retry
from … so the failure detail the user is deciding against stays on screen and
`Cancel` has a Retry action to return focus to."

That matches `EXPERIENCE.md:231` ("reveals the failed-item retry scope inside
Results with `Cancel` and `Create new plan`; Cancel closes the scope and returns
focus to the Retry action"), `EXPERIENCE.md:300` (focus matrix), and `DESIGN.md:216`
(Results Summary carries `retry scope` as a required state). The focus-return
contract is now explicitly the stated reason for the rule, which is stronger than
what revision 7 had.

Two problems remain, both in the new material.

#### V-1 — HIGH. Removing the fourth union member left `RetryPreview` with no rendering home, and contradicts EXPERIENCE's "replaces Results"

The spine's own diagram makes `RetryPreview` a distinct lifecycle state —
`ARCHITECTURE-SPINE.md:468`:

> ```
>     RetryScope --> RetryPreview: Create new plan, derived intent rebuilt
> ```

The visibility union has no member that covers it. `ARCHITECTURE-SPINE.md:541-545`:

> Its
>   visibility is a three-way union: a non-empty draft, a non-terminal attempt, or
>   undismissed Results. … When all three are false the region is hidden
>   and the workspace reclaims its width with no reserved empty column.

`RetryPreview` is none of the three: it is not the draft (AD-24 rule 2, `:754-758`,
"never merges with the persistent draft in either direction"), and it is not yet an
attempt. That leaves "undismissed Results" as the only candidate — but three
separate places in `EXPERIENCE.md` say `Create new plan` **removes** Results:

> `EXPERIENCE.md:156` — "Retry first shows the failed-item scope inline; `Create new
> plan` **deliberately replaces Results with a draft** while the immutable result
> stays in History with `View previous result`."

> `EXPERIENCE.md:231` — "`Create new plan` **deliberately replaces the sidecar** with
> a new reviewable draft."

> `EXPERIENCE.md:430` — "`Create new plan` **replaces Results** with a fresh
> reviewable draft."

The spine says the opposite at `ARCHITECTURE-SPINE.md:542-544`: "Results remain
until dismissed even though the draft behind them is empty" — with `Done` as the
only dismissal (`DESIGN.md:216`, "The single dismissal label is `Done`").

So: follow `EXPERIENCE` and Results are replaced, all three union members go false,
and the region hides — leaving the derived retry plan the user is being asked to
confirm nowhere on screen. Follow the spine and Results persist, contradicting three
`EXPERIENCE` lines and leaving `RetryPreview` rendering inside a surface the inputs
say it replaced.

The old four-way union covered this by accident: "an open retry review" spanned both
the scope and the preview. The fix correctly demoted the *scope* and dropped the
*preview* with it. UX-PB.4d and UX-PB.3d cannot both be built from the current text.

**Remedy direction.** Either add the derived-intent-under-review state to the union
as a fourth member ranked *below* Results (which is not the precedence H-1
objected to — the objection was to it outranking and hiding Results), or state that
`Create new plan` replaces Results in the region while the immutable result remains
reachable via `View previous result`, which is what `EXPERIENCE.md:156` actually
describes.

#### V-2 — MEDIUM. Placing the retry scope inside a read-only History replay contradicts UX-PB.4b and is unsupported by DESIGN/EXPERIENCE

Both new rules extend the retry scope to History replay —
`ARCHITECTURE-SPINE.md:547-549` and `:759-762` (AD-24): "revealed inline in the
surface Retry was invoked from — **Results, or a read-only History replay**".

`epics.md:940` (UX-PB.4b) forbids exactly that:

> **And** no control in the replay can mutate, re-run, or execute anything.

And the UX sources put Retry only in Results: `DESIGN.md:216` gives Results Summary
the `retry scope` state, while `DESIGN.md:217` gives History Plan Row only an
"optional **source-retry link**" and the state `retry-linked` — a lineage pointer,
not an action. `EXPERIENCE.md:156`/`:231` both scope Retry to Results.

The one input that supports the spine is `epics.md:974` (UX-PB.4d): "**Given** a
terminal Results **or History entry** with failed items and Retry available". So
`epics.md` contradicts itself between 4b and 4d, and the spine has silently taken
4d's side.

Taking a side is the spine's job — recording it is too. This is a sixth `epics.md`
divergence (UX-PB.4b's read-only criterion needs an explicit carve-out for the
non-executing Retry affordance, or UX-PB.4d's History origin needs to go), and it is
not in the batch row at `ARCHITECTURE-SPINE.md:947`.

### 3. C-1 fix — AD-22 rewritten over the canonical revision — **HOLDS**

The rewrite is correct and its shipping citations verify.

- `ARCHITECTURE-SPINE.md:678-685` (rule 1): "The confirming action is atomic against
  the **canonical revision**, not against one mutex hold. Validation reads under
  `state.plan_coordinator`; the guard is released before admission; the scheduler
  re-checks the same `expected_revision` under its own acquisition and enqueues
  all-or-none or nothing." Both citations check out: `src-tauri/src/commands.rs:353`
  literally reads "No synchronous guard crosses an await." and
  `src-tauri/src/queue.rs:1003` is `fn handle_plan_batch(`, which performs the
  re-check at `queue.rs:1009-1011`.
- `:686-692` (rule 2) names the impossibility outright — not reentrant, guard not
  `Send` across the admission await, scheduler takes the same lock — which is
  exactly the defect C-1 raised.
- `:693-698` (rule 3) fixes admit-then-persist and **labels the override honestly**:
  "**This deliberately overrides UX-PB.5b's stated clause order** (persist, activate,
  then admit); see the rider rule below for why, and the `epics.md` batch row for the
  criterion that must be restated." That is the right form — it names the input it
  overrides, gives the reason, and points at where the input gets fixed.

**Nothing in the inputs contradicts admit-then-persist.** Every persistence rule in
the inputs constrains the *internal* order of the save (disk before memory), not its
position relative to admission, so all are satisfied by persisting the rider after
admission returns:

- `docs/SPEC.md:112` (F11) — "A patch is persisted before in-memory settings and the
  canonical-state revision advance; failed persistence leaves both unchanged."
- `_bmad-output/project-context.md:126` — "Persist a settings patch before publishing
  it in memory or advancing the canonical revision; a failed save changes neither."
- `epics.md:85` (FR-17) — same.
- `EXPERIENCE.md:159` — "after activation, show `Saving`; change the active value
  only after the write succeeds".
- `docs/DECISIONS.md:189-191` (D28) — "Selecting `Disable upgrade plan command
  execution confirmation` and then confirming persists `skipUpgradePlanConfirmation:
  true`" — sequences the persist *after* the confirm, which is the spine's order.

AD-21's plan-inert classification of `skipUpgradePlanConfirmation`
(`ARCHITECTURE-SPINE.md:645-650`) closes the loop: persisting the rider after
admission cannot bump the revision and so cannot invalidate the attempt it just rode
on. Coherent.

#### V-3 — MEDIUM. Rule 2's blanket persist ban contradicts the shipping settings path that AD-19 ratifies

`ARCHITECTURE-SPINE.md:691-692`:

> Nothing may be persisted under a held coordinator guard
>   either — the settings path acquires that lock itself and `save_to` fsyncs.

The reasoning is sound *for a confirming action* (calling `set_settings_core` under
a held guard would deadlock on re-acquisition). But "Nothing" is unqualified, and the
shipping settings path does precisely this — `src-tauri/src/commands.rs:637-647`
takes the coordinator guard and then persists under it:

> ```
>     let mut coordinator = state
>         .plan_coordinator
>         .lock()
> …
>     merged
>         .save_to(&state.settings_path)
>         .map_err(IpcError::from)?;
> ```

AD-19 ratifies that exact sequence as correct — `ARCHITECTURE-SPINE.md:617-620`: "a
failed save changes neither — **the shipping order at `src-tauri/src/commands.rs`
`set_settings_core`**". So AD-22 rule 2, read literally, condemns the code AD-19
holds up as the reference. A builder reconciling the two has to guess which one is
scoped.

This is the mirror of the original C-1 error: that version cited `set_settings_core`
to license too much, this one bans the pattern too broadly. **Remedy:** scope the
sentence — "No confirming action may persist under a held coordinator guard", which
is what the rule means and leaves AD-19's reference intact.

### 4. Batch rows (b), (d), (f)

Checked against the `epics.md` text each describes. The row is
`ARCHITECTURE-SPINE.md:947`.

**(b) — HOLDS.** "Both halves need work, not just the conversion: the *seed* clause
treats `AllEligible` as a live-predicate intent, which AD-16's frozen-expansion rule
already forbade, and the *conversion* clause is the whole-intent kind AD-23 removes.
Only the conversion half's observable outcome survives". Accurate and complete
against `epics.md:580-582`, and the remedy it gives ("restate the seed as a frozen
bulk expansion and the conversion as member removal plus a tombstone") covers both
halves.

#### V-4 — LOW. Item (d) says "four locations" but enumerates three, omitting `epics.md:280`

The row names "the `DEFERRED` register row" (`epics.md:308`), "the narrative line
attributing the deferral to *this spine*" (`epics.md:170-171`), and "Story 6.5's own
contract lines" (`epics.md:1276-1277`). The fourth is unnamed —
`epics.md:280`, in the Governance and Risks table:

> | Suites green while the real command/event boundary is broken | AD-3 (committed contract fixtures; delivery coverage explicitly unproven and **awaiting the deferred native harness**) |

The count is right; a correct-course run working from the enumeration will miss one.

#### V-5 — LOW. Item (f) quotes one of the two locations carrying the DR-2 error

The row quotes `epics.md:260` ("Neither automated check exists yet…"). The same
false claim is also at `epics.md:304`, in the Implementation-Entry Register:

> | DR-2 — packaged accessibility method | `RESTATED` — D33 | Existing Playwright/Vitest lane + release checklist | None. An obligation on whichever story adds the two automated checks, **which do not exist yet**. |

Same failure mode as V-4: accurate as far as it goes, incomplete as an instruction.

---

## H-2 — my call, asked for plainly

**It is a real defect, and it should not block revision 8 — but it should be fixed
now rather than deferred, because the fix is smaller than the tracking.**

Why it is real, not pedantry: `EXPERIENCE.md:322` reserves assertive priority for "an
immediate safety action", and the product has exactly such actions — the 120-second
stall handoff and `Interaction required`, both of which ask the user to intervene
before a running process is abandoned or a command is handed to Terminal. The spine
permits one channel and fixes it as *polite*. A polite live region is, by
specification, announced only when the user is otherwise idle; a VoiceOver user
working elsewhere in the window can miss a safety prompt entirely.

Why it should not block: it is not a story-collision. Unlike C-1 and C-2, no two
stories build incompatibly from it — every story builds the same single channel, and
the only defect is that one of them cannot set the right priority. Nothing is
architecturally foreclosed; no other AD has to move; no code shape depends on it.
Revision 8 is materially better than revision 7 with this outstanding.

Why deferral is nonetheless the wrong call here: the cost asymmetry runs the other
way. The fix is roughly ten words in a rule that is already being edited. Deferring
it means carrying an Open row, and the failure mode if it is forgotten is
*silent* — the spine's neighbouring sentence, "Two live regions narrating one attempt
is a defect, not additive coverage" (`ARCHITECTURE-SPINE.md:568-569`), actively pushes
an implementer away from the correct escalation, so the wrong build looks compliant
and surfaces only in a manual VoiceOver pass on the release checklist, if at all.
That is the profile of a defect that should be closed at the altitude where it is
cheap, not scheduled.

Concretely, this is the whole fix:

> - **Rule:** There is exactly one status-announcement channel for plan and attempt
>   progress, owned alongside the sidecar region. It announces at polite priority by
>   default and assertive priority only for an immediate safety action
>   (`EXPERIENCE.md:322`). Stories announce through it; none adds a second live
>   region for the same information, and Brief Notifications suppress speech the
>   channel has already emitted (`EXPERIENCE.md:323`). Two live regions narrating one
>   attempt is a defect, not additive coverage.

If the owner prefers to defer it regardless, the deferral is legitimate **only if
recorded** — an Open row naming UX-PB.3c and UX-PB.3f as the consumers. Deferring it
silently is not, because the current wording does not read as an open question; it
reads as a settled answer.
