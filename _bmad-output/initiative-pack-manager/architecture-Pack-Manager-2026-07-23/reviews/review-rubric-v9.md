# Rubric walk — ARCHITECTURE-SPINE.md revision 9

**Lens:** rubric walker (good-spine checklist), independent, read-only
**Target:** `_bmad-output/planning-artifacts/architecture/architecture-Pack-Manager-2026-07-23/ARCHITECTURE-SPINE.md`, `artifact_revision: 9`, uncommitted working-tree state
**Baseline for the diff:** `db92db7` (revision 8) — 78 insertions, 26 deletions, one file
**Tree state at review:** HEAD = `22ed41e`; `git status --short` shows only `.memlog.md`, `ARCHITECTURE-SPINE.md`, `DRIFT-NOTE.md` modified
**Date:** 2026-07-25

> **This file supersedes an earlier pass of the same lens run at HEAD `8d36cdf`,
> before the focus conversion was committed.** That pass is superseded rather than
> discarded: its findings were re-verified against the current tree and the ones
> still live are carried forward below with their evidence re-checked, marked
> *[carried]*. Three of its findings the spine has since closed and they are not
> repeated — the Focus-indicator row's three-vs-nine undercount (the row now reads
> "found **nine**", spine:1019), the "satisfies this today" false present tense (now
> "is the mechanism to reach for", spine:338), and the row's OPEN status (now
> RESOLVED). Two of its findings turn out to have been *understated* once HEAD moved
> to `22ed41e`; those are re-tiered upward.

---

## Verdict

**READY WITH FIXES**, with one qualification that matters more than the tier counts:
**both CRITICALs bite exactly the two stories revision 9 says it unblocked.** The
design-token row ends "UX-PB.1e and UX-PB.5d are unblocked" (spine:994), and C-1 and
C-2 are precisely the two ways a UX-PB.1e or UX-PB.5d builder can obey this spine and
still ship the defect it was written after. **Fix C-1 and C-2 before either story is
scheduled.** Everything else can follow.

What revision 9 got right is substantial, and a reader who skips to the findings will
misjudge the pass:

- **Every closure I sampled is real.** Four of the six closed rows verified against the
  tree, the commits, and — for the runner claim — the GitHub Actions API. One is closed
  with materially *better* evidence than the spine cites for itself (§4a).
- **The `epics.md` batch row is accurate item-by-item.** Eight independent counts, all
  matching.
- **The runner rule is now written as an invariant rather than a value.** "a named
  stable runner image, never `macos-latest`" (spine:299), with `macos-15` demoted to
  the current value of it, is the correct altitude and the single best edit in the
  revision.
- **The Styling-convention push-down was the right instinct.** Memlog 145 — "UX-PB.1e
  and UX-PB.5d read conventions, not the status table" — is exactly right. C-1 is that
  the same reasoning was not applied to the other half of the same decision.
- **Making the focus defect an invariant rather than a bug ticket is defensible.** The
  stated justification — a green cross-engine suite is not evidence against it, because
  the lane's sample did not represent its population — is the reasoning that
  distinguishes an invariant from a ticket, and it is well argued.

### Tally

| Tier | Count |
| --- | --- |
| CRITICAL | 2 |
| HIGH | 5 |
| MEDIUM | 8 |
| LOW | 5 |
| **Total** | **20** |

For comparison: revision 8's rubric lens returned 1 CRITICAL / 3 HIGH / 5 MEDIUM /
3 LOW (memlog 131). Revision 9 has a smaller change surface and a higher finding
density, and the reason is structural rather than careless: 78 inserted lines contain
two new invariants drafted against `8d36cdf`, and HEAD moved to `22ed41e` mid-run. Most
of the HIGH and MEDIUM findings share that one root cause.

### Rubric line-by-line

| Rubric criterion | Verdict |
| --- | --- |
| Fixes the real divergence points for the level below, misses none | **Partial** — the focus-paint divergence is identified correctly and routed to a place the diverging stories never read (C-1); two `epics.md` contradictions created by revision 9's own closures are missed (H-5) |
| Every `AD`'s Rule is enforceable and prevents its stated divergence | **Partial** — the paint rule prevents its *stated* divergence but permits a second one D35 had closed (C-2); "proven in WebKit" has no enforcement point (H-1) and does not establish what it claims (H-2) |
| Nothing under Deferred/Open could let two units diverge | **Pass with one reservation** — the Deferred rows are each fenced by an AD and the two new Open rows are records rather than licence; the reviewer-gate tail row is a bare count hiding six unresolved HIGHs (M-8) |
| Named tech is verified-current | **Pass** — `macos-15`, Playwright 1.61.1, `outline`/`outline-offset`, `appearance`, Tailwind v4 `ring-*`→`box-shadow` all verified against the tree |
| Ratifies rather than contradicts the brownfield codebase | **Fail** — AD-11's focus rule is materially weaker than `docs/DECISIONS.md` D35 and `docs/SPEC.md` §4.1 at HEAD (C-2), and quotes a §4.1 that no longer exists (H-3) |
| Every dimension the altitude owns is decided, deferred, or open | **Fail** — the E2E lane's browser engines and host platform are now load-bearing for a new invariant and are silent everywhere in the spine (H-2) |

---

## 1. AD-11's scope — is the title/scope mismatch a real defect?

**Yes, and it is worse than a title problem. It is a routing problem, and it is provable
from the spine's own text rather than from judgement.**

The assignment asks whether "a builder of a UI story would never read the release AD and
therefore miss the rule." The spine answers that question itself, in three places.

**(a) `Binds` is the routing key, and AD-11's does not include the stories.**

```
spine:281   - **Binds:** release
```

Compare every other AD that governs UX-PB work:

```
spine:184   - **Binds:** all IPC work; Epic UX-PB; Story 6.5                                  (AD-3)
spine:267   - **Binds:** persistence and lifecycle work; Stories 3.4, 6.5; UX-PB.1b, UX-PB.2c (AD-5)
spine:375   - **Binds:** D27–D30; Epic UX-PB (all 28 stories); Stories 3.1, 3.2, 3.5, 6.5     (AD-16)
spine:849   - **Binds:** Story 2.2; UX-PB.1e, UX-PB.2b, UX-PB.3d                              (AD-25)
```

AD-25 names UX-PB.1e explicitly. AD-11 does not name any UX-PB story, and revision 9 did
not touch its `Binds` line while adding two rules the revision header says govern
UX-PB.1e and UX-PB.5d.

**(b) The Capability → Architecture Map routes those stories away from AD-11.**

```
spine:979   | Draft Upgrade Plan and sidecar … (UX-PB.1a–1e) | … | AD-16, AD-17, AD-23, AD-24 |
spine:983   | Confirmation gate and its setting (UX-PB.5a–5e) | … | AD-16, AD-17, AD-19, AD-21, AD-22 |
spine:985   | Package state, eligibility, keyboard selection (Stories 3.1, 3.2, 3.5) | … | AD-4, AD-16, AD-17, AD-23 |
spine:988   | Packaged release, signing, updater | `release.yml` + `docs/RELEASE-CHECKLIST.md` | AD-11, AD-12 |
```

`grep -n "AD-11"` over the whole spine returns six hits: three in the revision-9 header
prose, the AD's own heading, the Capability-map release row, and two Decision-Status
rows. **AD-11 is reachable from exactly one capability row, and it is the release row.**
A UX-PB.1e builder following the map lands on AD-16/17/23/24; a UX-PB.5d builder lands on
AD-16/17/19/21/22. Neither path passes through AD-11.

**(c) The run knew the principle and applied it to only half of the decision.**

Memlog 145 states it, correctly:

> "the convention now carries the rule a builder actually reads … Without that, the row's
> own complaint — 'The Styling convention fixes where tokens live and never says which' —
> would still be true of the convention after the status table said RESOLVED, and
> **UX-PB.1e and UX-PB.5d read conventions, not the status table.**"

The convention was duly updated:

```
spine:928   | Styling | … Focus resolves `--color-focus-ring` and never `--color-accent`,
            so selection and focus stay distinguishable. |
```

That is the **token** half of D35. The **mechanism** half — outline vs. `ring-*`,
WKWebView, `appearance` — appears nowhere in the Consistency Conventions table. The run
correctly diagnosed that the status table is not where builders read, pushed the token
rule down, and then filed the mechanism rule in an AD whose `Binds` is `release` and whose
only capability-map entry is the release row. Two halves of one decision, at two
different reachability levels.

### Recommendation: split into `AD-27`. Do not widen the title.

I considered both options the assignment names and recommend the split, for four reasons.

1. **`Prevents` does not stretch.** AD-11's single `Prevents` clause is:

   ```
   spine:282-283   - **Prevents:** a release shipping without the checks whose failure is
                     silent and simultaneous across every installed client
   ```

   An unpainted focus ring is not a release-check failure and is not "simultaneous across
   every installed client" in the sense that clause means — it is a property of the build,
   not of the publication. Widening the title leaves two rules with no `Prevents`, which
   in this document's format is how a rule loses its reason and gets dropped by a later
   pass. A new AD gets its own: *prevents an interactive affordance shipping with a focus
   indicator the shipping engine does not paint, and prevents a green cross-engine suite
   being read as evidence that every control has one.*

2. **Widening the title fixes neither `Binds` nor the capability map**, which are the
   actual routing mechanisms. If both must be edited anyway, that is 80% of a split.

3. **No id constraint blocks it.** Memlog 137 records the owner's constraint as "AD ids
   stay stable; AD-6..AD-10 and AD-13..AD-15 remain retired and are never reused." AD-26
   exists (spine:875), so AD-27 is the next free id and reuses nothing retired. AD-26 was
   given its own id for a *narrower* concern than this one.

4. **It fixes M-1 (AD-11 overload) at the same time.** See §3.

**Concrete shape:**

```
### AD-27 — A focus indicator is drawn by a mechanism the shipping engine paints

- **Binds:** all UI work; Epic UX-PB (all 28 stories); Stories 3.1, 3.2, 3.5
- **Prevents:** an interactive control shipping with a focus indicator WKWebView does not
  paint, and a green cross-engine suite being read as proof that every control has one
- **Rule:** [spine:329-345, corrected per C-2 and M-4]
- **Rule:** [spine:346-354, corrected per H-3 and H-4]
```

Plus: add `AD-27` to capability-map rows 979, 983, 985; add a one-line mechanism clause to
the Styling convention at 928 so the convention carries both halves of D35.

---

## 2. Are the two new rules enforceable? Is "proven in WebKit" owned by anyone?

**The paint rule is enforceable in principle and unenforced in practice. The
coverage-limit rule is a disclaimer rather than a rule, and its one affirmative
obligation is unowned.**

### The paint rule (spine:329-345)

As a *design* rule it is well-built. Naming `appearance` as the discriminator rather than
listing element types is the right call and memlog 149 gives the right reason — "a rule
pinned to 'checkbox, radio, select' silently fails on the next native-appearance control
someone adds." The discriminator claim is correct against the tree: at `8d36cdf`,
`src/components/primitives/Checkbox.tsx:38`, `src/components/manager/PackageRow.tsx:99`,
and `src/components/manager/PackageTable.tsx:110` each carried
`focus-visible:ring-2 focus-visible:ring-focus-ring`, and `docs/DECISIONS.md:431-434`
records the measurement: ":focus-visible matched `true` in both engines, while the
computed `box-shadow` was the full ring in Chromium and the literal string `"none"` in
WebKit."

But the *proof* half — "and proven in WebKit rather than Chromium alone" — names no lane,
no gate, no artifact, and no owner. Three places could carry it, and none does:

- **The style-contract lane.** Disclaimed by AD-11's very next rule: "It does not assert
  that a given interactive element has one" (spine:347-348). The spine explicitly removes
  the automated lane from the role of proving this.
- **The release checklist.** `docs/RELEASE-CHECKLIST.md:86` reads "**Keyboard and
  accessibility pass.** Tab and arrow navigation reach every control." That is
  *reachability*, not *indicator visibility*. `grep -in "focus"` over the whole checklist
  returns one hit, at line 36, and it is a description of what CI runs, not a manual step.
  Revision 9 added nothing here.
- **"the story that adds it."** spine:350-351 — "An element gaining an interactive
  affordance is verified by the story that adds it." This names an owner and no evidence.
  Compare AD-3, which names both the enforcing mechanism and the artifact a story must
  produce:

  ```
  spine:194-196   The enforcing mechanism is the shipping contract test —
                  `src-tauri/src/ipc.rs` byte-compares each serialized model against its
                  committed fixture…
  spine:213-215   Any story adding a field to an event payload (AD-18's `planAttemptId` on
                  `op:status`, `op:output`, and the attention path) owns fixture coverage
                  of the shape…
  ```

  AD-3 says *what the story must produce*. AD-11's parallel clause says only that the
  story is responsible. That is the difference between an obligation and an aspiration.

**This is not hypothetical.** The nine-controls discovery is the proof. Three were found
by grep; six were found only by "a runtime audit that focused every focusable element in
every reachable view and read computed style" (`22ed41e` commit body). Nothing in the
spine requires, schedules, or even names that audit. The single technique that found two
thirds of the defect is absent from the invariant written about the defect.

**Fix.** Name the deliverable: *a story adding or changing an interactive affordance
either adds a WebKit-project assertion on that element's computed focus style, or records
a runtime tab-order audit of the view it touches.* That converts a responsibility into
something reviewable.

### The coverage-limit rule (spine:346-354)

Structurally this is the strongest idea in the revision — generalizing AD-3's
sample-versus-population limit to the style lane is genuinely correct, and memlog 147's
reasoning for it is sound. But as written it is three negations plus one under-specified
positive; two of the negations are wrong at HEAD (H-4) and the positive is unowned
(above). A rule made entirely of "you may not conclude X from Y" prevents a *misreading*.
It does not prevent a *divergence*: two builders can both correctly decline to read the
green run as proof and still ship different things.

---

## 3. Does AD-11 now carry too many concerns to be one invariant?

**Yes.** AD-11 is 76 lines (spine:279-354) carrying **seven Rules over five unrelated
concerns**, under one `Binds` and one `Prevents`:

| Rule | Lines | Concern |
| --- | --- | --- |
| 1 | 284-286 | Release readiness is a manual checklist, not a computed verdict |
| 2 | 287-290 | Two `release.yml` checks block publication |
| 3 | 291-296 | Universal build; both updater target keys; Intel unverified |
| 4 | 297-310 | macOS floor + named stable runner image + D31 notarytool closure |
| 5 | 311-328 | Accessibility lane staging; reduced motion covered; contrast not; focus token |
| 6 | 329-345 | Focus paint mechanism / WKWebView / `appearance` |
| 7 | 346-354 | What the style-contract lane does not assert |

Rules 1-4 share the `Prevents` clause. Rules 6-7 do not, and they are the only rules in
AD-11 a non-release builder must obey. Rule 4 is CI policy, not release acceptance.

The diagnostic the rest of the spine uses is `Binds`: an AD whose binding cannot honestly
be enumerated is carrying more than one invariant. AD-11's is one word. A second
diagnostic: `epics.md` cites `AD-11` six times, and after this revision the citation no
longer identifies which of seven obligations is meant. Compare AD-3, which is also large
but single-subject — everything in it is "the IPC surface changes atomically."

Splitting rules 6-7 into AD-27 leaves AD-11 at five rules over four release-shaped
concerns, which is defensible. Rule 4 is arguably still misfiled (AD-12 already owns
"the release framework and transport"), but that is a pre-existing structure, not
something revision 9 introduced, and I am not tiering it.

---

## 4. Do the newly-RESOLVED rows close something not actually closed, or overstate evidence?

I sampled four of the six closures and verified each against the tree, `git log`, and —
for the runner claim — the GitHub Actions API. **All four are genuinely closed. None is
fabricated. One overstates by a stale count; one is contradicted by a source the spine
itself makes normative; one rests on a premise nothing pins.**

### 4a. `macos-14` runner retirement — CLOSED, and better-evidenced than the spine claims

```
$ grep -rn "runs-on" .github/workflows/    # macOS pins only
.github/workflows/ci.yml:28:    runs-on: macos-15
.github/workflows/ci.yml:70:    runs-on: macos-15
.github/workflows/release.yml:63:    runs-on: macos-15
$ grep -rn "macos-14" .github/workflows/
.github/workflows/release.yml:61:    # … macos-14 is        ← comment
.github/workflows/ci.yml:10:# … That was macos-14 until …   ← comment
```

Zero `runs-on` pins on `macos-14`. Matches spine:995 exactly.

The spine's strongest claim is spine:309-310: "a manual Release run built, signed, and
notarized on the new image (commit `419dc32`)". The spine cites only the commit, and
`git log -1 --format=%B 419dc32` shows that claim originates in the commit body — so on
the spine's own citation it is a report of the tree, not the tree, inside a revision whose
headline discipline is the opposite. **I checked the run itself and the claim holds:**

```
$ gh run view 30154432651 --json headSha,conclusion,createdAt
  conclusion: success   createdAt: 2026-07-25T10:25:16Z
  headSha: 419dc32762ca6d1f58588ddfabbb825e81c0150c
```

Steps: `8 success Import Developer ID certificate`, `9 success Configure notarization`,
`10 success Build app`, `11 success Package`, **`12 success Verify signature &
notarization`**. That last step is what makes this dispositive, because `release.yml:16-18`
documents that "Signing/notarization degrade gracefully when the secrets are absent … and
the job still succeeds" — a bare `success` conclusion would *not* have proven notarization.
Step 12 does. D31's requirement ("It is settled by a manual Release workflow run",
`DECISIONS.md:273`) is genuinely met.

**No finding against the closure.** Recommendation only: cite run `30154432651` rather
than the commit, which makes the claim first-hand and removes the one place the blanket
"verified against the committed tree" header claim does not hold on its own citations.

### 4b. Minimum supported macOS residual — CLOSED, correctly attributed, premise unpinned

`docs/DECISIONS.md:381-385` states the closure in its own words and the spine paraphrases
it accurately. The supersedence caveat is exactly right and is the kind of note that stops
a later reviewer re-opening a closed row:

```
spine:996   Note D31's own text still reads "CI therefore stays on `macos-14`" and its OPEN
            paragraph is unedited: D34 supersedes D31 rather than rewriting it, so cite D34
            for the closure and never D31 alone.
```

Verified: `DECISIONS.md:268-269` still reads "CI therefore stays on `macos-14`" and
`:271-276` still carries the OPEN paragraph. The closure is sound; see M-6 for the
unpinned premise.

### 4c. Canonical design-token set — CLOSED, one stale count (M-3)

```
$ grep -n "color-bg-base\|color-accent\|color-focus-ring" src/styles/theme.css
8:  --color-bg-base:       #090C13;
19:  --color-focus-ring:    #F4F7FB;
27:  --color-accent:        #65A7FF;
```

Matches spine:994. `docs/DECISIONS.md:392` is D35. `EXPERIENCE.md`'s quoted sentence exists
(`grep -c` = 1). The `ring-accent` survivor at `PackageRow.tsx:85` exists.

**But** spine:994 asserts "All 22 `focus-visible` sites resolve `--color-focus-ring`":

```
$ git grep -c focus-visible be1f0e6 -- 'src/**/*.tsx' | awk -F: '{s+=$NF} END {print s}'   → 22
$ git grep -c focus-visible 8d36cdf -- 'src/**/*.tsx' | awk -F: '{s+=$NF} END {print s}'   → 22
$ grep -rn focus-visible src --include='*.tsx' | wc -l                                     → 31
```

22 was true at `be1f0e6`. It is false at HEAD. See M-3.

### 4d. App-update safety guard — CLOSED in code, contradicted by a normative source (M-4)

The Rust side is exactly as described:

```
src-tauri/src/commands.rs:810    refuse_app_update_while_busy(&state.queue.records())?;
src-tauri/src/commands.rs:772    fn refuse_app_update_while_busy(records: &[crate::ipc::OperationRecord]) -> Result<(), IpcError> {
src-tauri/src/commands.rs:777-779        crate::ipc::OpStatus::Queued | crate::ipc::OpStatus::Running
src/store/operations.ts:137              (o.status === "queued" || o.status === "running")
```

The sets are identical, which is the point, and the helper carries the invariant in a doc
comment: "The status set matches `activeOps` in `src/store/operations.ts` exactly." The
closure is real and well-made. But see M-4 — `docs/RELEASE-CHECKLIST.md:84`, a document
AD-11 makes normative, still says the opposite.

### 4e. `epics.md` divergence batch — CLOSED, accurate on every item sampled

Eight counts against the committed `epics.md`, all matching spine:1016:

| Claim | Expected | Actual |
| --- | --- | --- |
| `reconstructed into the sidecar` gone (item a) | 0 | 0 |
| `AllEligible` survives once as a negation (item b) | 1 | 1 |
| `new reviewable draft` gone (item c) | 0 | 0 |
| `awaiting the deferred native harness` gone (item d) | 0 | 0 |
| `Neither automated check exists yet` gone (item f) | 0 | 0 |
| `AD-25` now cited (follow-up) | 4 | 4 |
| `AD-26` cited at the harness sites (item d) | ≥4 | 6 |
| `AD-21` cited | ≥1 | 1 |

`git status --short` on `epics.md` and on the proposal is clean, so the spine's decision to
drop the planned working-tree qualifier (memlog 148) is correct. **No finding against the
batch row.** It is the best-verified closure in the revision. What the row misses is that
revision 9's *other* closures made two different `epics.md` passages stale — see H-5.

---

## 5. Claims about the tree in revision 9's new text — sample verification

Every factual assertion in the two new AD-11 rules, plus the header sentence and the
focus row:

| Claim (spine line) | Verdict |
| --- | --- |
| Tailwind `ring-*` compiles to `box-shadow` (333) | **True** — `DECISIONS.md:427`; corroborated by the pre-fix classes |
| `appearance` is the discriminator, not the element (331-332) | **True** — `DECISIONS.md:434-436`, `22ed41e` body |
| The same utility paints on a `button` (336) | **True** — the spec asserted a button ring in WebKit for months |
| "the style contract's own focus assertion targets a `button`" (343) | **STALE** — true at `8d36cdf`, false at HEAD (H-4) |
| The lane "does not assert that a given interactive element has one" (347-348) | **FALSE at HEAD** (H-4) |
| `docs/SPEC.md` §4.1 says "offset against surface, on every interactive element" (352) | **FALSE** — `grep -c` = 0 (H-3) |
| `EXPERIENCE.md` "Every interactive element uses a separate `{colors.focusRing}` indicator" (353-354) | **True** — `grep -c` = 1 |
| "six native checkboxes had no visible focus at all" (46) | **Misattributed** (M-2) |
| Focus row: 31 sites, 0 `ring-focus-ring`, 0 `ring-offset-*`, 0 `outline-none`, 1 `ring-accent` (1019) | **True** — verified; `ring-accent` = 3 grep hits, 1 production (L-2) |
| Focus row: "Playwright 14/14" (1019) | **True** — `npx playwright test --list` → "Total: 14 tests in 4 files" |
| Stack: CI runner images, three named jobs (955) | **True** |

---

## Findings

### CRITICAL

---

#### C-1 — AD-11's two new rules are unreachable by the two stories they govern

**Where:** spine:281 (`Binds: release`), spine:979/983/985 (capability map), spine:928
(Styling convention), spine:329-354 (the new rules) *[carried, re-verified]*

**Claim.** The revision-9 header says the new invariant governs product-side focus work,
and the design-token row ends "UX-PB.1e and UX-PB.5d are unblocked" (spine:994). Every
routing mechanism in the spine sends those two stories somewhere other than AD-11:
`Binds` says `release`; the capability map lists AD-16/17/23/24 for UX-PB.1a–1e and
AD-16/17/19/21/22 for UX-PB.5a–5e; AD-11 appears in the map only on the "Packaged
release, signing, updater" row. The Styling convention — the one place memlog 145
correctly identifies as where these two stories read — received the token half of D35 and
not the mechanism half.

**Failure scenario.** UX-PB.1e adds a plan-membership control in the sidecar. Its builder
reads AD-16, AD-17, AD-23, AD-24 and the Styling convention. The convention says focus
resolves `--color-focus-ring` and never `--color-accent` — satisfied by
`focus-visible:ring-2 focus-visible:ring-focus-ring`, which is what every site in the
repo looked like until `22ed41e` and what `DESIGN.md:202` ("Keyboard focus uses a
separated 2px `focusRing` outline") describes as a *visual* rather than a CSS mechanism.
Every rule the builder was routed to is satisfied. The control ships with no visible focus
in WKWebView — the exact defect, on the exact story, that the invariant was written after.

**Fix.** Split into `AD-27` (§1), add it to capability-map rows 979/983/985, and put the
mechanism clause into the Styling convention at 928.

---

#### C-2 — AD-11's focus rule is weaker than D35 and `SPEC.md` §4.1, and silently reopens a decided question

**Where:** spine:337-341 vs. `docs/DECISIONS.md:425-483` and `docs/SPEC.md:206`

**Claim.** The spine says:

```
spine:337-341   `outline` with `outline-offset` satisfies the rule everywhere and is the
                mechanism to reach for; stripping a control to `appearance: none` and
                styling it fully is the only other way to earn `box-shadow`. The
                requirement is that focus be painted where the user runs it, not that any
                particular property is used.
```

The brownfield says the opposite, in ADOPTED sources the spine cites as its own
authorities:

```
DECISIONS.md:425      **Focus is drawn as a real `outline`, never `ring-*`.**
DECISIONS.md:441-445  The rule is nonetheless applied uniformly to all focusable elements,
                      because a mixed codebase is a trap — the next person adding a checkbox
                      copies the `ring-` from the button beside it and ships an invisible
                      focus state that no test catches. One mechanism, `outline` +
                      `outline-offset`, everywhere.
DECISIONS.md:481-483  **Rejected:** `appearance: none` on the checkboxes, which does make
                      `box-shadow` paint in WebKit but strips the native checkmark…
SPEC.md:206           Use `outline-*`, **not** `ring-*` … but the rule is uniform so no
                      control can be given an invisible focus state by following its
                      neighbours.
```

So the spine (a) declines to require the uniform mechanism D35 mandates, (b) explicitly
blesses `appearance: none` as one of two legitimate routes when D35 rejects it **by name**,
and (c) states "not that any particular property is used" against a §4.1 that names the
property and forbids the alternative. The spine never acknowledges that D35's uniformity
rule exists, so a reader cannot see that it is being overridden.

**Failure scenario.** UX-PB.5d adds a confirmation-gate control as a `<button>` with
`focus-visible:ring-2 focus-visible:ring-focus-ring`. It paints in WKWebView — buttons are
not native-appearance form controls. AD-11 is fully satisfied: "The requirement is that
focus be painted where the user runs it, not that any particular property is used." The
codebase now has two focus mechanisms. The next story adds a checkbox beside it, copies
the `ring-` classes, and ships an invisible focus state. That is verbatim the trap
`DECISIONS.md:441-445` was written to close — reopened by the spine that was supposed to
ratify it, and reopened *invisibly*, because the spine outranks D35 for a builder and
never mentions the conflict.

The rubric criterion this fails is "ratifies rather than contradicts the brownfield
codebase," and it fails it in the harder direction: the spine is permitting a divergence
the codebase had already closed.

**Why the phrasing happened, and why the reason does not survive.** Memlog 149 explains:
writing "focus is an outline" would have been "a false present tense the moment it was
typed," because `22ed41e` was uncommitted. That instinct — state the decision, not the tree
state — is right, and it produced the excellent runner rule. But it was applied one level
too high. The *decision* is not "focus must be painted"; the decision, made in D35 and
recorded before revision 9 was written, is "one mechanism, `outline` + `outline-offset`,
everywhere, because a mixed codebase is a trap." That decision was already durable and
already tree-independent — it would not have gone stale. The spine abstracted past it and
landed on a requirement general enough to re-admit the case D35 rejected.

**Fix.** Restate as the two-part rule the sources actually carry (this also absorbs M-1's
`outline-none` gap):

> **Rule:** A focus indicator must be drawn by a mechanism the shipping engine actually
> paints, and the mechanism is uniform across the codebase: `outline` with
> `outline-offset`, never `ring-*` (`docs/DECISIONS.md` D35, `docs/SPEC.md` §4.1).
> WKWebView does not paint `box-shadow` on a control still rendering with its native
> appearance — the discriminator is `appearance`, not the element and not the property —
> so a Tailwind `ring-*` utility, which compiles to `box-shadow`, yields no visible focus
> on a native-appearance checkbox, radio, or select in the distributed app while looking
> correct in a Chromium preview. It paints correctly on a `button`, which is exactly why
> the rule is uniform rather than scoped: a mixed codebase lets the next control inherit
> the invisible mechanism from its neighbour. `appearance: none` plus full restyling would
> also earn `box-shadow` and is rejected — it strips the native checkmark. Never add
> `outline-none` to a focusable element: Tailwind v4 makes it genuinely set
> `outline-style: none` (v3's no-op was renamed `outline-hidden`).

---

### HIGH

---

#### H-1 — "proven in WebKit rather than Chromium alone" has no enforcement point and no owner

**Where:** spine:330, spine:350-351

Full argument in §2. Summary: the automated lane is disclaimed by AD-11's own next rule
(spine:347-348); the release checklist has no focus-visibility step
(`RELEASE-CHECKLIST.md:86` covers reachability only; `grep -in "focus"` over the file
returns one hit, the CI description at line 36); and "verified by the story that adds it"
names an owner without a deliverable. AD-3's structurally identical rule names both an
enforcing mechanism (spine:194-196) and the artifact a story owes (spine:213-215).

**Failure scenario.** UX-PB.3f adds a `<select>` filter to Results. The builder adds
`focus-visible:outline-2 focus-visible:outline-focus-ring`, runs the suite green, ships.
Nobody checks the shipping engine for that element: no lane asserts it, no checklist step
covers it, the story owed no artifact. If the class had been mistyped, or an inherited
`outline-none` suppressed it, nothing would catch it — which is exactly how six of the
nine indicator-less controls survived until a manual runtime audit the spine does not
require.

**Fix.** Add a deliverable clause (§2).

---

#### H-2 — The WebKit the lane runs is Linux WebKit on `ubuntu-latest`, not WKWebView — and the spine declares neither the engine set nor the gap

**Where:** spine:330, spine:953 (Stack), spine:955 (CI runner images) *[carried, re-verified and re-tiered]*

Two problems, one root.

**(i) The proof engine is not the shipping engine, and the spine asserts the equivalence.**
The rule says focus must be painted by "a mechanism the **shipping engine actually
paints**, and proven in WebKit". Verified:

```
playwright.config.ts:82-87        { name: "chromium", … }, { name: "webkit", use: { …devices["Desktop Safari"] } }
.github/workflows/test.yml:56     runs-on: ubuntu-latest      ← the E2E shard job
.github/workflows/test.yml:117    runs-on: ubuntu-latest      ← the burn-in job
package.json:18                   "test:e2e:install:ci": "playwright install --with-deps chromium webkit"
```

That is Playwright's **Linux** WebKit build. The defect the rule is about — `box-shadow`
not painting on a *native-appearance* form control — is precisely the area where WebKit
ports diverge from one another, because it depends on the platform form-control theme.
Nothing in the repo establishes that Linux WebKit reproduces WKWebView here. Empirically
it did this time (`DECISIONS.md:431-434`), which is fortunate rather than guaranteed, and
the spec's own new comment conflates the two: "drawn as an outline so it paints on a
native checkbox in WebKit/WKWebView too"
(`tests/e2e/browser-style-contract.spec.ts:215-216`).

The spine already knows how to write this honestly. AD-3 does it: "The fixtures prove
payload shape on both sides. They do not dispatch anything through Tauri, so real event
*delivery* is unproven by construction" (spine:211-213), and "No story may claim delivery
coverage from a fixture or from the browser double" (spine:216-217). AD-11 asserts the
substitution AD-3 forbids by name, in the same document — and AD-11's own sibling rule
quotes the spec disclaiming that it "validate[s] the native Tauri package" (spine:322-324).

**(ii) The engine set is undeclared and unprotected.** `grep -n "WebKit\|Chromium\|webkit\|
chromium"` over the whole spine returns six hits, **all inside AD-11's new prose or the
revision-9 header**, plus one unrelated mention at spine:883 about `tauri-driver`. The
Stack table lists `| Playwright | 1.61.1 |` with no engines. The CI runner row lists
images, not browser projects. `docs/RELEASE-CHECKLIST.md:36` knows the fact — "`test.yml`
runs Playwright on Chromium and WebKit" — and the spine does not.

**Failure scenario.** A CI-tuning pass drops the `webkit` project or scopes CI to
`--project=chromium` to halve E2E runtime. Every spine rule still passes. AD-11's paint
rule becomes unprovable and silently unenforceable, and nothing records that a capability
was lost — the same silence AD-11 was written to close. Under the rubric this is a whole
dimension the altitude owns (which engines the browser lane must exercise, on which host)
left silent while a new invariant was built on top of it.

**Secondary, and worth a sentence in the rule:** revision 9 discovered one member of a
class — a CSS mechanism Chromium paints and WKWebView does not. The class has other
members a UX-PB story can reach (`backdrop-filter`, scrollbar styling, sticky/overflow
interaction, native `<select>` rendering, `-webkit-` prefixed behavior). The rule is
scoped to focus indicators, so the next member is unbound and will be found the same way.

**Fix.** Declare the engine set as load-bearing and non-narrowing, the way AD-2 says
release bits may not gain a runtime selector; and scope the proof claim — *Playwright
WebKit on Linux is the closest available automated proxy for WKWebView, not WKWebView
itself; the native package remains unvalidated (AD-3), and shipping-engine proof today is
a manual pass in the built app.* Optionally state the general obligation with focus as its
first named instance.

---

#### H-3 — AD-11 cites a `docs/SPEC.md` §4.1 quotation that no longer exists, and quotes the one clause §4.1 dropped

**Where:** spine:352-353

```
spine:352-353   `docs/SPEC.md` §4.1 ("offset against surface, on every interactive element")
                plus `EXPERIENCE.md` … are the floor it is verified against.
```

```
$ grep -c "offset against surface" docs/SPEC.md
0
```

`22ed41e` rewrote §4.1. The old sentence — "Focus: 2px `--color-focus-ring` ring, offset
against surface, on every interactive element" — was replaced at `docs/SPEC.md:206` by:

> "Focus: a real 2px `outline` in `--color-focus-ring` with `outline-offset`, on every
> interactive element — a dedicated indicator, never `--color-accent`. Use `outline-*`,
> **not** `ring-*` … Never add `outline-none` to a focusable element."

Two consequences, and the second is worse.

1. The spine designates §4.1 as "the floor it is verified against" and then quotes a floor
   that is not there. A builder following the citation to check the authority finds it says
   something else — the exact failure memlog 144 says "the currency lens caught twice
   already."
2. The quotation the spine *chose* is the one clause §4.1 no longer has, while the two
   clauses §4.1 gained — "not `ring-*`", "never `outline-none`" — are the two that would
   have prevented C-2 and M-1. The spine quoted the obsolete half of its own floor and
   dropped the half that carries the rule.

Note the design-token row (spine:994) quotes §4.1 *correctly* — "a dedicated indicator,
never `--color-accent`" — so the same revision cites the same section accurately in one row
and inaccurately in an AD, which locates the cause: the AD text was drafted against
`8d36cdf` and not re-checked after `22ed41e`.

**Fix.** Requote against HEAD, and quote the clauses that carry weight.

---

#### H-4 — AD-11's coverage-limit rule and its supporting evidence both describe `8d36cdf`, not HEAD

**Where:** spine:342-345, spine:346-348 *[carried, re-tiered upward now that `22ed41e` is committed]*

**(i) The evidence sentence.**

```
spine:342-345   the style contract's own focus assertion targets a `button`, whose
                `box-shadow` WebKit *does* paint, so the suite passed on both engines while
                the package-row plan-membership checkbox had no visible focus at all.
```

At HEAD the spec has **two** focus assertions. The first still targets the Refresh All
button (`browser-style-contract.spec.ts:74-103`). The second is a whole new test,
`"keeps a selected row and a focused control inside it distinguishable"` (`:132-224`),
which focuses the package-row checkbox by keyboard and asserts its computed outline:

```
browser-style-contract.spec.ts:186-189   const checkbox = page.getByTestId(`row-${gemini.id}`).getByRole("checkbox").first();
browser-style-contract.spec.ts:219-222   expect(focus.outlineStyle).toBe("solid");
                                         expect(focus.outlineWidth).toBe("2px");
                                         expect(focus.outlineColor).toBe("rgb(244, 247, 251)");
```

The very gap the sentence cites as live evidence was closed by the commit the spine records
one screen further down as RESOLVED.

**(ii) The coverage-limit rule's first negation is now false.**

```
spine:347-348   It does not assert that a given interactive element has one…
```

The second test asserts exactly that, for the package-row plan-membership checkbox — the
single control the paint rule names as its motivating case. (The other two clauses, offset
and contrast, do survive: neither test reads `outlineOffset` and neither computes contrast.)

**Failure scenario.** A refactor deletes or renames the second test. Nobody objects: the
spine says the lane does not assert element-has-indicator, so removing an assertion of
element-has-indicator looks like removing something that was never load-bearing. The one
automated regression guard on the exact control the invariant exists for disappears with
the spine's blessing.

**Fix.** Move (i) to past tense anchored to `8d36cdf` — it is history, not current state.
Reword (ii) to the AD-3 shape rather than an enumeration that decays every time the lane
grows: *a story claiming coverage from this lane must point at the specific assertion that
provides it; the lane asserts presence on only the two elements it names, and asserts
neither offset nor contrast anywhere.* The durable half — "No story may read a green run as
evidence of those three" — is a real prohibition and should stay.

---

#### H-5 — `epics.md` now contradicts two of revision 9's own RESOLVED rows, and the new residuals row does not list it

**Where:** spine:994, spine:996, spine:1018 vs. `_bmad-output/planning-artifacts/epics.md:307`, `:308`, `:311` *[carried, re-verified against the committed file]*

The spine closes the design-token row and states "UX-PB.1e and UX-PB.5d are unblocked."
`epics.md` at HEAD still says the opposite, twice:

```
epics.md:307   … nothing blocks starting it **except UX-PB.1e and UX-PB.5d**, which are
               blocked on the canonical design-token set in the row below.
epics.md:308   | Canonical design-token set | `OPEN` — needs an owner decision | UX decides;
               Development implements | **Blocks UX-PB.1e and UX-PB.5d**
               (`ARCHITECTURE-SPINE.md:944`). …
```

And the `notarytool` residual the spine just closed:

```
epics.md:311   | DR-1 — minimum supported macOS | `CLOSED` — D31 | … Whether `notarytool`
               accepts `minos 15.0` against the CI SDK is OPEN and is settled by a manual
               Release run, never by assertion. |
```

The new residuals row (spine:1018) opens "Two things the batch left" and lists only
UX-PB.3d/AD-25 and AD-21's thin citation. **These two are not batch residuals — they are
new staleness created by revision 9's own closures**, which is exactly the class the spine
is uniquely placed to catch and the reason that row exists. Note the design-token blocker
row at `epics.md:308` was *added by the batch* (memlog 142: "the design-token blocker row
added at :308") and invalidated by the closure in the same revision.

**Failure scenario.** A builder consulting `epics.md` — the artifact directly below this
altitude, and the one a sprint-planning or dev-story run reads — does not start UX-PB.1e,
because its own implementation-entry register says the story is blocked. Separately, the
next currency lens re-raises the `notarytool` question from `epics.md:311` and the closure
is re-litigated.

**Fix.** Add both to the residuals row so the next `bmad-correct-course` run picks them up.

---

### MEDIUM

---

#### M-1 — AD-11 omits the `outline-none` prohibition, which was the actual suppressor

**Where:** spine:329-345

The spine names `outline` + `outline-offset` and says nothing about `outline-none`. Both
authoritative sources treat it as load-bearing:

```
DECISIONS.md:447-450   Tailwind v4's `outline-none` genuinely sets `outline-style: none`
                       (v3's no-op was renamed `outline-hidden`), so `outline-none` on a
                       focusable element actively suppresses the indicator and must never
                       be added.
SPEC.md:206            Never add `outline-none` to a focusable element.
```

Not theoretical — every ring site at `8d36cdf` carried it:

```
Checkbox.tsx:38      "focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-focus-ring …"
PackageRow.tsx:99    "focus-visible:outline-none focus-visible:ring-2 …"
PackageTable.tsx:110 "focus-visible:outline-none focus-visible:ring-2 …"
```

`22ed41e`'s commit body: "that is what had been suppressing the one mechanism that works."

**Failure scenario.** A builder follows AD-11, adds `focus-visible:outline-2
focus-visible:outline-focus-ring`, and keeps or inherits a neighbouring `outline-none`.
Under Tailwind v4 the indicator is suppressed. AD-11 gave no warning; the lane does not
assert presence on their element (H-4); nothing catches it.

**Fix.** Absorbed into the C-2 rewrite.

---

#### M-2 — The revision-9 header misattributes three of the six checkboxes to the wrong defect

**Where:** spine:44-46 *[carried, re-verified]*

```
spine:44-46   WKWebView does not paint `box-shadow` on native-appearance form controls, so
              the style contract stayed green on both engines while six native checkboxes
              had no visible focus at all.
```

The "so" makes the paint failure the cause for all six. Verified at `8d36cdf`, only
**three** of the six carried a focus class at all:

| Site (at `8d36cdf`) | Pre-fix focus classes | Defect |
| --- | --- | --- |
| `src/components/primitives/Checkbox.tsx:38` | `focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-focus-ring …` | ring unpainted by WebKit |
| `src/components/manager/PackageRow.tsx:99` | same | ring unpainted by WebKit |
| `src/components/manager/PackageTable.tsx:110` | same | ring unpainted by WebKit |
| `src/components/manager/PackageToolbar.tsx:68` | `h-4 w-4 rounded-[4px] border border-border-strong bg-bg-raised accent-accent` | **no focus class at all** |
| `src/components/dialogs/UpgradePlanSheet.tsx:218` | same as above | **no focus class at all** |
| `src/components/dialogs/UpgradePlanSheet.tsx:229` | same as above | **no focus class at all** |

Confirmed by the `22ed41e` diff, which *adds* `focus-visible:outline-*` to the two
UpgradePlanSheet checkboxes rather than converting a ring, and by the commit body listing
"both Upgrade Plan sheet checkboxes" among the nine controls that "had no focus style at
all."

Two distinct defects, two distinct causes, two distinct detection methods (grep finds one;
only a runtime audit finds the other) merged into one causal sentence. A builder who
accepts the framing converts `ring-*` → `outline` and reasonably believes the floor is met.
It also obscures that the *harder* class is the absent one — which is the point the
coverage-limit rule is trying to make.

**Fix.** "…while three native checkboxes had an unpainted ring and three more had no focus
style at all — two defects, one of which no text search can find."

---

#### M-3 — The design-token RESOLVED row bakes in a count HEAD has already falsified

**Where:** spine:994

"All 22 `focus-visible` sites resolve `--color-focus-ring`" — present tense; 22 at
`be1f0e6` and `8d36cdf`, **31 at HEAD** (verified three ways, §4c). The focus-remediation
row seven lines later says "the site count going from 22 to 31" (spine:1019), so the same
table presents 22 and 31 as current in adjacent rows, with only the second flagging the
first as historical.

This is the failure memlog 148 identified and legislated against **in this same run**:

> "a bare number in a status row rots the same way the rule ordinals and line citations
> did, so it is replaced with 'every live AD id' rather than corrected to eighteen."

The lesson was applied to the `epics.md` retired-register row and not to the row above it.

**Fix.** "Every `focus-visible` site resolves `--color-focus-ring`" — no number.

---

#### M-4 — `docs/RELEASE-CHECKLIST.md` still contradicts the app-update closure the spine ratifies

**Where:** spine:1014 vs. `docs/RELEASE-CHECKLIST.md:82-84`

Spine (RESOLVED): "The enforcement point is now Rust: `install_app_update` calls
`refuse_app_update_while_busy(&state.queue.records())` before doing anything."

Checklist, unchanged at HEAD:

```
RELEASE-CHECKLIST.md:82-84   - **8b. An update is refused while an operation is running.**
                             *(~45 s)* … This is currently enforced by frontend convention
                             only; the Rust command has no guard.
```

AD-11 rule 1 makes this document normative: "Release readiness is
`docs/RELEASE-CHECKLIST.md` — a manual checklist" (spine:284-285). The spine therefore
simultaneously declares the row closed and ratifies as the release authority a document
asserting it is open. Unlike the D31 case — where the spine wrote an explicit supersedence
note (spine:996) and a generated-docs caveat (spine:995) — this contradiction is
unacknowledged. `RELEASE-CHECKLIST.md` is one of the spine's declared `sources`
(spine:18), and recording source drift was plainly in scope: the run did exactly that for
D31 and for the three generated docs.

**Fix.** Add a caveat clause to spine:1014 in the same shape as the D34 row's.

---

#### M-5 — `DRIFT-NOTE.md` §2f contradicts the spine's own status table on two points

**Where:** `DRIFT-NOTE.md` §2f (added by revision 9) vs. spine:1019

DRIFT-NOTE:

> "The remediation — converting every focus site from `ring-*` to `outline` and giving
> **the three** indicator-less controls one — was **in the working tree and uncommitted**
> while this revision was being written … and the conversion itself is tracked as an Open
> row that says plainly it is not committed yet."

Spine:1019: "**RESOLVED** … Closed 2026-07-25 by commit `22ed41e`" and "a runtime audit of
the real tab order found **nine**."

Two contradictions — OPEN vs. RESOLVED, three vs. nine — inside the same revision's own
deliverable set, and the spine points at this file as its change record ("Change record:
`DRIFT-NOTE.md`", spine:50). `22ed41e` landed at 06:49 and the run-folder mtimes are
06:50-06:51, so §2f simply was not re-read after the row flipped.

**Fix.** Bring §2f forward: committed, nine, RESOLVED-and-retained-as-rationale.

---

#### M-6 — "the build SDK is no longer behind the floor" is asserted in three places and pinned in none

**Where:** spine:305-309, spine:995, spine:996 *[carried, re-verified]*

The premise that dissolves D31's open question is a claim about GitHub's image contents:
"on `macos-15` the build SDK is no longer behind the floor, so the mismatch the question
was about no longer exists."

Verified: `release.yml` has no `xcode-select` pin, no `SDKROOT`, and no assertion that the
build SDK is ≥ `minimumSystemVersion` (`src-tauri/tauri.conf.json:48` `"minimumSystemVersion":
"15.0"`). The only trace is an explanatory comment at `ci.yml:13` ("aligns the build SDK
with the 15.0 deployment floor declared in D31"). A future image change, or a future floor
raise, silently re-opens the question and nothing fails.

This is a real closure — the manual Release run proved notarization *on today's image*
(§4a) — but the *rule that keeps it closed* does not exist. If the closure argument is "the
mismatch no longer exists," the rule preserving that state belongs in AD-11 alongside the
named-stable-image rule it sits next to.

**Fix.** One clause: *the runner image must not fall behind `bundle.macOS.minimumSystemVersion`;
raising the floor above the image's SDK reopens D31's notarization question.*

---

#### M-7 — The new residuals row overstates residual (1)

**Where:** spine:1018 vs. `epics.md:827` *[carried, re-verified]*

The row says of UX-PB.3d:

> "AD-25's actual rule for that path — a failed or timed-out verification *leaves the
> Last-good Snapshot in place* — **appears nowhere the builder reads**"

`epics.md:827` is UX-PB.3d's own Dependencies line:

```
epics.md:827   **Dependencies:** UX-PB.3c; D29-D30; AD-16 (verification-gated success;
               post-exit fresh acquisition); AD-25 (a failed verification refresh leaves
               the Last-good Snapshot in place)
```

The substance is present, in the same parenthetical form the row treats as
present-but-thin for AD-21 in residual (2). The genuine residual — the *criterion prose*
omits the snapshot obligation — survives. "Appears nowhere" does not, and the row's own
next paragraph applies the opposite standard to AD-21.

**Fix.** Restate (1) and (2) as one finding: three of the new `AD`s reach `epics.md` only
as Dependencies-line parentheticals and never as criterion prose.

---

#### M-8 — The reviewer-gate tail Open row remains a bare count, untouched for a third revision

**Where:** spine:1015

```
spine:1015   | Reviewer-gate tail (revision 6) | **Open** | … The remaining tail is **6 HIGH,
             15 MEDIUM, 5 LOW** across `reviews/review-divergence-v6.md`,
             `review-rubric-v6.md`, `review-reconcile-epics-v6.md`, and
             `review-currency-v6.md`. Each finding names its own affected stories. |
```

Revision 9 closed six rows, opened two, and did not touch this one. Twenty-six findings —
six of them HIGH — are carried as a count plus a pointer to four files, with no per-finding
disposition. Under "nothing under Deferred/Open could let two units diverge," this row
cannot be audited from the table: a divergence pair could be sitting inside those six HIGHs
and the spine would look identical. "Each finding names its own affected stories" moves the
burden to the reader without moving the risk.

Carried from revision 8 rather than introduced here, which is why it is MEDIUM. But it is
now the only Open row whose contents cannot be inspected from the table, and it is aging.

**Fix.** Enumerate the six HIGHs as their own rows — the treatment revision 7 gave the five
promoted findings — or state a disposition date. A count is not a status.

---

### LOW

---

#### L-1 — `DRIFT-NOTE.md`'s header states a memlog entry count that is already wrong, and forward-declares reviews that do not exist

```
DRIFT-NOTE.md:4-5   **Memlog:** `.memlog.md` (148 entries by `grep -c '^- ('`; … entry 137
                    opens revision 9, so … 137–148 are revision 9)
$ grep -c '^- (' .memlog.md
150
```

Also: "four `*-v9.md` lenses against revision 9" — `ls reviews/` shows fourteen files, none
`*-v9` (this file is the first). The second is arguably a deliberate forward declaration.
But the paragraph immediately above the count correction in §2f calls a stale count "the
same class as the stale entry count and the rotted line citations before it," so the file
names the error class and commits it in its own header.

---

#### L-2 — "exactly one `ring-accent` survivor" is true of production and false of the repo

spine:1019: "exactly one `ring-accent` survivor at `src/components/manager/PackageRow.tsx`".

```
$ grep -rn ring-accent src --include='*.tsx'
src/components/manager/managerPane.test.tsx:114:    expect(highlighted.className).toContain("ring-accent");
src/components/manager/managerPane.test.tsx:118:    expect(other.className).not.toContain("ring-accent");
src/components/manager/PackageRow.tsx:85:        highlighted ? "ring-2 ring-inset ring-accent" : "",
```

One production site, three occurrences. The two extras are the unit test `22ed41e` added to
pin the survivor — a good thing, and worth naming in the row rather than being contradicted
by it. Wording only.

---

#### L-3 — The focus-remediation row is an implementation log inside a decision table, carrying six volatile numbers

spine:1019 records 31 sites, 0 `ring-focus-ring`, 0 `ring-offset-*`, 0 `outline-none`, 1
`ring-accent`, vitest 134/134, Playwright 14/14. **I verified all of them and they are
correct today** (`npx playwright test --list` → "Total: 14 tests in 4 files"). But *Decision
Status and Deferred Items* is where decisions live, and every one of these changes on the
next UI commit. The row's stated justification — "it records why AD-11's focus-paint rule
exists" — is rationale, and rationale belongs with the rule or in `DRIFT-NOTE.md`, both of
which already carry it. Same class as M-3, hence LOW: the numbers are right, the location
guarantees they will not stay right.

---

#### L-4 — AD-11's runner rule cites D20, whose own text still names `macos-14`, without the caveat it gives D31

**Where:** spine:301 *[carried, re-verified]*

AD-11 cites "(`docs/DECISIONS.md` D20, D34)". `docs/DECISIONS.md:83` (D20) reads: "CI
build-smoke runs on stable macos-14 runners; beta-specific issues are diagnosed
on-machine." The spine carefully flags D31's stale text (spine:996) and does not flag D20's,
while citing D20 *inside a Rule* as the authority for the never-`macos-latest` principle.
D34 confirms the principle survives ("D20's constraint is unchanged and still governs …
Only which image is the stable one has changed", `DECISIONS.md:377-379`), so the citation is
correct in substance and stale in letter.

**Fix.** One-line caveat, the same shape as the D31 note.

---

#### L-5 — `epics.md:308` cites `ARCHITECTURE-SPINE.md:944`, which is now the Tauri-opener stack row

```
$ sed -n '944p' ARCHITECTURE-SPINE.md
| Tauri opener plugin | 2.5.4 |
```

A line-number citation into a living document, already rotted one revision after it was
written. Not the spine's edit — but the residuals row is the ledger for `epics.md`
staleness, so it belongs there alongside H-5. It is also a concrete instance of the rule
memlog 148 derived ("a bare number in a status row rots the same way the rule ordinals and
line citations did"), arriving from the artifact below rather than from within.

---

## Answers to the assignment's focus questions, in brief

**1. AD-11's scope.** Real defect, provable from the spine's own routing rather than from
judgement: `Binds: release` (281) plus a capability map that sends UX-PB.1a–1e and
UX-PB.5a–5e to five other ADs (979, 983) means the two named stories never reach the rule.
**Recommendation: split into `AD-27`, not widen the title** — AD-11's single `Prevents`
clause cannot cover a focus-paint rule; widening the title still leaves `Binds` and the
capability map to fix; no id constraint blocks AD-27; and the split fixes the overload
finding at the same time. Also push a mechanism clause into the Styling convention at 928,
where memlog 145 already established these two stories actually read.

**2. Enforceability.** The paint rule is well-constructed as a design rule — naming
`appearance` as the discriminator is the right abstraction — but it is not enforced. **There
is no enforcement point and no owner for "proven in WebKit":** the automated lane is
disclaimed by AD-11's own next rule, the release checklist has no focus-visibility step, and
"verified by the story that adds it" names an owner without a deliverable. Worse, the
WebKit that would do the proving is Playwright's Linux build on `ubuntu-latest`, not
WKWebView, and the spine neither declares the engine set nor scopes the claim (H-2).

**3. Too many concerns?** Yes — seven rules, five concerns, one `Binds`, and a `Prevents`
that covers four of the seven. The `Binds` line is the diagnostic: an AD whose binding
cannot be honestly enumerated is more than one invariant. A second diagnostic: `epics.md`
cites AD-11 six times and the citation no longer identifies which obligation is meant.

**4. Overstated closures?** No fabricated closures. All four sampled rows are genuinely
closed, and the `epics.md` batch row is accurate on all eight claims checked. The runner
closure is in fact *stronger* than the spine claims — manual Release run `30154432651` on
`headSha 419dc32`, step 12 "Verify signature & notarization" succeeding, which matters
because `release.yml:16-18` documents that signing degrades gracefully and a bare `success`
would not have proven it. Three blemishes: the design-token row's "All 22" is falsified by
HEAD (M-3); the app-update closure is contradicted by `RELEASE-CHECKLIST.md:84`, which
AD-11 makes normative (M-4); and the SDK premise behind the D31 closure is asserted three
times and pinned nowhere (M-6). Separately, two closures created *new* staleness in
`epics.md` that the residuals row does not list (H-5).

**5. New tree claims.** Eleven checked (table in §5). Three wrong: the `SPEC.md` §4.1
quotation does not exist at HEAD (H-3); "the lane does not assert that a given interactive
element has one" is false at HEAD (H-4); and "six native checkboxes" attributes three
absent-class defects to the box-shadow paint failure (M-2). One stale: "All 22
focus-visible sites" (M-3). The root cause of all four is one thing — the new AD-11 text was
drafted against `8d36cdf` and not re-verified after `22ed41e` landed mid-run, which is the
identical failure mode the run diagnosed for itself twice (memlog 146, 149) and guarded
against successfully in the runner rule. A single re-verification pass over the two new
rules against HEAD closes H-3, H-4, and M-2 together.
