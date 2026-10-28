# Review — input reconciliation, ARCHITECTURE-SPINE.md revision 9

- **Lens:** input reconciliation. Does each load-bearing input land in the `AD`
  structure, and does the spine contradict an input without saying so?
- **Target:** `_bmad-output/planning-artifacts/architecture/architecture-Pack-Manager-2026-07-23/ARCHITECTURE-SPINE.md`,
  `artifact_revision: 9`.
- **Reviewed state:** 1019 lines, md5 `35c6d46c866faaae1b0558ecd59ba15b`, read
  2026-07-25. **The file was edited by another session during this review** — the
  `Focus-indicator remediation` row moved `OPEN` → `RESOLVED` and AD-11's
  focus-paint rule was rewritten mid-pass. All quotes below are from the state
  above; re-verify line numbers before acting.
- **Tree state:** `HEAD` = `22ed41e` ("fix(a11y): draw keyboard focus with an
  outline so it is visible in WKWebView"), which landed during this review.
- **Read this pass:** `docs/SPEC.md`, `docs/DECISIONS.md`,
  `docs/RELEASE-CHECKLIST.md`, `ux-.../DESIGN.md`, `ux-.../EXPERIENCE.md`,
  `_bmad-output/planning-artifacts/epics.md`,
  `_bmad-output/implementation-artifacts/deferred-work.md`,
  `.github/workflows/*.yml`, `playwright.config.ts`,
  `tests/e2e/browser-style-contract.spec.ts`, `src-tauri/src/commands.rs`.

## Verdict

**Revise before the next revision closes.** The five closures revision 9 claims
are each individually well-evidenced — I verified the `macos-15` pins, the
manual Release run, the Rust update guard, the `epics.md` batch, and the
deferred-work closure against the tree and against GitHub, and every one holds.
The failures are structural, and they cluster in the one place this revision
newly touched: **the new accessibility and focus invariants were folded into an
`AD` whose `Binds:` line is `release`, so they bind no story that builds a
control**, and the folded text **reopens two things `docs/DECISIONS.md` D35
closed on the record, without saying it is overriding them**.

Tally: **2 CRITICAL, 5 HIGH, 7 MEDIUM, 4 LOW — 18 findings.**

| Tier | Count |
| --- | --- |
| CRITICAL | 2 |
| HIGH | 5 |
| MEDIUM | 7 |
| LOW | 4 |
| **Total** | **18** |

---

## Direct answers to the five focus questions

### 1. D31 vs D34 — is the supersedence handled unambiguously?

**Substantively yes; structurally no.** The Decision Status table is exemplary:

> `ARCHITECTURE-SPINE.md:996` — "Note D31's own text still reads "CI therefore
> stays on `macos-14`" and its OPEN paragraph is unedited: D34 supersedes D31
> rather than rewriting it, so cite D34 for the closure and never D31 alone."

That is exactly right, and I verified the underlying facts rather than the
account of them:

- `.github/workflows/ci.yml:28` and `:70` and `.github/workflows/release.yml:63`
  all read `runs-on: macos-15`. No `runs-on` in `.github/workflows/` names
  `macos-14`. The spine's three-pin claim holds.
- The manual Release run is real, not asserted. `gh run view 30154432651`
  returns `{"conclusion":"success", "headSha":"419dc32762ca6d1f58588ddfabbb825e81c0150c",
  "createdAt":"2026-07-25T10:25:16Z"}`, and `git log -1 --format=%aI 419dc32`
  is `2026-07-25T06:25:05-04:00` — the same commit, run after it. D31 said the
  question "is settled by a manual Release workflow run"; it was. **This is the
  strongest verification in revision 9.**

**The reading under which the spine is wrong** is a builder who reads only the
normative text. The caveat lives *only* in the Decision Status table. AD-11's
`Rule` — the binding text — reads:

> `ARCHITECTURE-SPINE.md:297-298` — "Minimum supported macOS is 15.0 at
> `bundle.macOS.minimumSystemVersion` (`docs/DECISIONS.md` D31)."

Follow that citation and you land on `docs/DECISIONS.md:269-270`, "CI therefore
stays on `macos-14`", and on the unedited "**One question remains OPEN at the
time of writing**" paragraph — both contradicted by the same AD-11 rule four
lines later. AD-11 addresses the `notarytool` residual but never states that D34
supersedes D31 on the runner. See **M3**.

Separately, AD-11 attributes the "never `macos-latest`" constraint to D20, which
does not contain it and is itself superseded. See **M2**.

### 2. SPEC §4.1 vs DESIGN.md vs EXPERIENCE.md on focus — do they agree?

**On the focus indicator itself: yes, now.** Verbatim:

- `docs/SPEC.md:208` — "Focus: a real 2px `outline` in `--color-focus-ring` with
  `outline-offset`, on every interactive element — a dedicated indicator, never
  `--color-accent`. Use `outline-*`, **not** `ring-*` … Never add `outline-none`
  to a focusable element."
- `DESIGN.md:202` — "Keyboard focus uses a separated 2px `focusRing` outline;
  selection color never substitutes for focus."
- `EXPERIENCE.md:318` — "Every interactive element uses a separate
  `{colors.focusRing}` indicator that is at least 2px wide and visible against
  every surface. `{colors.borderStrong}` may indicate selection but never
  substitutes for focus; selected and focused states remain distinguishable."

Three sources, one answer: a dedicated `focusRing` token, at least 2px,
separated/offset, on every interactive element, distinguishable from selection.
`DESIGN.md` and `SPEC.md` additionally prescribe `outline`; `EXPERIENCE.md` is
mechanism-neutral. No conflict.

**They disagree on one thing, and the spine does not say which wins — and takes
the losing side.** The disagreement is about *disabled* interactive controls:

- `EXPERIENCE.md:326` — "Explanatory-disabled Package Checkboxes do not use
  native `disabled`. They expose `aria-disabled="true"`, retain focus, associate
  the persistent reason through an accessible description, remain inert on
  activation … Apply the same rule to pinned, current, excluded, and unavailable
  aligned controls."
- `DESIGN.md:203` — "Explanatory-disabled controls look unavailable but retain
  the same 2px `focusRing`."
- versus `docs/SPEC.md:272` — "pinned → disabled checkbox + tooltip "Pinned in
  Homebrew — run `brew unpin <name>` to upgrade"".

The spine adopts SPEC's side, silently:

> `ARCHITECTURE-SPINE.md:540-543` — "**Ineligible-item inertness.** An item that
> is pinned, already current, a non-opted-in greedy cask, or removed between
> staging and rebuild is inert: its control is **non-interactive to pointer and
> keyboard**, it carries a stated reason for assistive technology, and it can
> never enter a `PlanIntent`."

"Non-interactive to keyboard" is the native `disabled` behaviour: out of the tab
order, no `:focus-visible`, no focus indicator, and the "stated reason for
assistive technology" unreachable by the keyboard user it exists for. That is the
exact outcome `EXPERIENCE.md:326` forbids and `DESIGN.md:203` contradicts. See
**M1**. Note `docs/SPEC.md:66` is internally inconsistent on the same point — it
requires pinned rows to "explain **on attempted interaction**", which a natively
`disabled` control cannot receive.

### 3. `deferred-work.md` — was the closure claim accurate?

**Yes, verified.** Both entries were genuinely dischargeable and both are gone:
`git show 22ed41e -- _bmad-output/implementation-artifacts/deferred-work.md`
removes exactly the two `spec-adopt-design-tokens-and-focus-ring.md` blocks, and
`grep -n "outline\|ring-offset\|focus" deferred-work.md` now exits 1. Neither
survives.

One nuance the spine could state and does not: the first entry's *literal
remedy* was not performed, it was dissolved. The entry asked to

> "Add `ring-offset-*` to the 16 `focus-visible` ring sites …" and warned "each
> site also needs the correct `ring-offset-bg-*` for the surface it sits on, so
> it is a per-site visual decision rather than a mechanical sweep."

`outline-offset` needs no per-surface background token, so the per-site decision
the entry reserved evaporated rather than being taken. The *obligation* behind
the entry — `docs/SPEC.md` §4.1's offset requirement — is satisfied. The claim
holds; only the mechanism differs from what the entry anticipated.

The spine's own correction inside that row (three reported → **nine** found by
runtime audit) is the most useful sentence added in revision 9 and should not be
lost in a later cleanup.

### 4. `epics.md` — are the two residuals right, and is there a third?

**Residual (2) is exactly right.** `epics.md:1045` carries "AD-21
(`skipUpgradePlanConfirmation` is declared plan-inert)" on the Dependencies line,
and none of UX-PB.5b's six criteria (`epics.md:1052-1075`) mention plan-inert
classification or the canonical revision. Verified.

**Residual (1) is right about the gap but wrong about the shape** — see **M4**.
`epics.md:827` reads "AD-25 (**a failed verification refresh leaves the Last-good
Snapshot in place**)", so the obligation *is* stated, in the Dependencies
parenthetical. The spine says it "appears nowhere the builder reads". It appears
in the same place AD-21's does. These are one failure mode, not two.

**There are at least three more residuals, and each contradicts a Decision Status
row this same revision closed.** See **H3**. Summary:

| `epics.md` | Says | Spine says |
| --- | --- | --- |
| `:307` | "nothing blocks starting it **except UX-PB.1e and UX-PB.5d**, which are blocked on the canonical design-token set" | `:994` "UX-PB.1e and UX-PB.5d are unblocked." |
| `:308` | "Canonical design-token set \| `OPEN` — needs an owner decision … **Blocks UX-PB.1e and UX-PB.5d** (`ARCHITECTURE-SPINE.md:944`)" | `:994` **RESOLVED**, D35 |
| `:309` | "Whether `notarytool` accepts `minos 15.0` against the CI SDK is **OPEN**" | `:996` closed by D34 |

`epics.md:308` also still describes the conflict as "`docs/SPEC.md`'s
accent-coloured ring", which `docs/SPEC.md:208` has not said since `be1f0e6`, and
points at `ARCHITECTURE-SPINE.md:944`, which is now a row of the Stack table.

### 5. Input requirements on accessibility, focus, or CI the `AD` structure still does not carry

The full list is **C1, C2, H1, H2, H5, M1, M5, M6, M7, L1, L2, L3**. The single
most consequential is **C1**: the structure carries the rules but binds nobody to
them. The quietest is **H5**: `EXPERIENCE.md` explicitly delegates its
accessibility *method* to this document, and this document never supplies one.

---

## CRITICAL

### C1 — The new focus and accessibility invariants bind `release`, so they reach no story that builds a control

Revision 9's own header states the folding decision:

> `ARCHITECTURE-SPINE.md:42-43` — "One new invariant, folded into AD-11 rather
> than given its own id: **a focus indicator must be drawn by a mechanism the
> shipping engine actually paints** …"

AD-11's binding line has not moved:

> `ARCHITECTURE-SPINE.md:279-281` — "### AD-11 — [ADOPTED] Release acceptance is
> the checklist plus two automated checks / - **Binds:** release"

And the Capability → Architecture Map routes AD-11 to exactly one row:

> `ARCHITECTURE-SPINE.md:988` — "| Packaged release, signing, updater |
> `release.yml` + `docs/RELEASE-CHECKLIST.md` | AD-11, AD-12 |"

No other map row cites AD-11. Every UX-PB row cites AD-16/17/18/19/21/22/23/24/25;
`Confirmation gate and its setting (UX-PB.5a–5e)` — the row containing
**UX-PB.5d, "Accessibility and responsiveness of the confirmation and safety
surfaces"** — cites AD-16, AD-17, AD-19, AD-21, AD-22 and not AD-11. So this
sentence, which is the entire remedy the second new rule proposes, addresses an
empty set:

> `ARCHITECTURE-SPINE.md:350-351` — "An element gaining an interactive affordance
> is verified by the story that adds it".

Confirmed downstream: `epics.md:1102`, UX-PB.5d's Dependencies, reads
"UX-PB.5a; **finalized focus and high-zoom contracts**; FR-19" — an unresolved
placeholder, no AD cited, and none of its four criteria (`epics.md:1108-1122`)
mention a focus indicator's mechanism, width, or presence.

The invariant is stated and unenforced. Either widen AD-11's `Binds` and add it
to the UI rows of the map, or give the focus rules their own `AD` id bound to
`all frontend work` — the reason it was folded (economy of ids) is not worth the
binding it costs.

### C2 — AD-11 reopens two things D35 closed on the record, and does not say it is overriding them

AD-11's rewritten focus rule reads:

> `ARCHITECTURE-SPINE.md:337-341` — "`outline` with `outline-offset` satisfies the
> rule everywhere and is the mechanism to reach for; **stripping a control to
> `appearance: none` and styling it fully is the only other way to earn
> `box-shadow`**. The requirement is that focus be painted where the user runs
> it, **not that any particular property is used**."

Both halves contradict a decision the same revision cites as authority.

`docs/DECISIONS.md` D35 rejects `appearance: none` explicitly, by name, in its
Rejected list:

> "**Rejected:** `appearance: none` on the checkboxes, which does make
> `box-shadow` paint in WebKit but strips the native checkmark, **trading an
> invisible focus state for an invisible checked state**."

and rejects the property-agnostic reading, with its reason:

> "The rule is nonetheless applied uniformly to all focusable elements, because a
> mixed codebase is a trap — the next person adding a checkbox copies the `ring-`
> from the button beside it and ships an invisible focus state that no test
> catches. **One mechanism, `outline` + `outline-offset`, everywhere.**"

`docs/SPEC.md:208` states the same prohibition normatively — "Use `outline-*`,
**not** `ring-*`" and "**Never add `outline-none` to a focusable element**" —
and neither prohibition appears anywhere in the spine.

This is the quiet requirement the `AD` structure dropped. D35's uniformity is not
a stylistic preference; it is the *enforcement mechanism*, chosen because a
scoped rule cannot be enforced by any test the project has. AD-11 restates the
scoped version and re-offers the rejected escape hatch, so a story could comply
with AD-11, strip a checkbox to `appearance: none`, ship an invisible checkmark,
and cite the spine for it. If the spine means to permit that, it must say it is
overriding D35 and take D35's stated cost. It currently says neither.

---

## HIGH

### H1 — "proven in WebKit" points at a lane that does not run the shipping engine

> `ARCHITECTURE-SPINE.md:329-330` — "A focus indicator must be drawn by a
> mechanism the **shipping engine actually paints**, and proven in WebKit rather
> than Chromium alone."

The lane AD-11 names for this (`ARCHITECTURE-SPINE.md:318-319`,
"`.github/workflows/test.yml`") runs entirely on Linux:

- `.github/workflows/test.yml:30`, `:56`, `:117`, `:161` — every job is
  `runs-on: ubuntu-latest`.
- `playwright.config.ts:85-86` — `name: "webkit", use: { ...devices["Desktop Safari"] }`.

So the "WebKit" that proves the rule is Playwright's bundled Linux WebKit build,
not the macOS WKWebView the rule is *about* — and the defect class the rule names
is *native-appearance form-control painting*, which is precisely the
platform-specific behaviour that does not transfer between WebKit ports.

The spine applies this exact scepticism everywhere else and withholds it here:

> `ARCHITECTURE-SPINE.md:212-215` (AD-3) — "no current suite crosses the complete
> JavaScript-to-Tauri-to-Rust transport … No story may claim delivery coverage
> from a fixture or from the browser double."

AD-11's new rule needs the same disclaimer, or it launders a Linux browser result
into evidence about WKWebView — the failure mode it was written to prevent. The
real proof of the defect (`docs/DECISIONS.md` D35: "Measured on the same element,
same keyboard interaction") was a local measurement, not this lane.

### H2 — AD-11 quotes `docs/SPEC.md` §4.1 from a superseded revision of it

> `ARCHITECTURE-SPINE.md:352` — "and `docs/SPEC.md` §4.1 ("**offset against
> surface, on every interactive element**") plus `EXPERIENCE.md` … are the floor
> it is verified against."

`docs/SPEC.md:208` does not contain the string "offset against surface". That
phrasing was replaced by `22ed41e`; the current text is "a real 2px `outline` in
`--color-focus-ring` with `outline-offset`, on every interactive element". The
spine names SPEC §4.1 as its floor and quotes a version of it that no longer
exists — and the current version contains two prohibitions (`not ring-*`, `never
outline-none`) that the spine's quote elides. Requote from the committed source.

### H3 — the `epics.md` residuals row undercounts, and the three it misses contradict rows this revision just closed

> `ARCHITECTURE-SPINE.md:1018` — "| `epics.md` residuals after the revision-8
> batch | **OPEN — record for the next `bmad-correct-course` run, do not edit
> here** | **Two things the batch left** …"

Three more survive, listed in the table under question 4 above. The severity is
not the count — it is that the row is the *handoff mechanism*: it is explicitly
the record the next `bmad-correct-course` run works from, and it forbids editing
`epics.md` here. Anything absent from it will not be fixed.

The consequence is concrete. A builder reading `epics.md:307` today is told
UX-PB.1e and UX-PB.5d are blocked on an open owner decision. The spine says at
`:994` they are unblocked. `epics.md` is the build queue; the spine is not read
per-story. The revision that closed the blocker is the revision that should have
recorded that its closure never reached the queue.

`epics.md:309` is the same shape for `notarytool`: it still calls the question
`OPEN` two rows after the spine declares it closed by D34.

### H4 — `docs/RELEASE-CHECKLIST.md` still denies the guard the spine just closed

> `docs/RELEASE-CHECKLIST.md:83-84` — "**8b. An update is refused while an
> operation is running.** *(~45 s)* … This is currently enforced by **frontend
> convention only; the Rust command has no guard**."

versus

> `ARCHITECTURE-SPINE.md:1014` — "The enforcement point is now Rust:
> `install_app_update` calls `refuse_app_update_while_busy(&state.queue.records())`
> before doing anything".

The spine's row is correct — `src-tauri/src/commands.rs:772` defines the helper
and `:810` calls it, with unit tests at `:862-890`. The checklist is stale.

This matters more than an ordinary stale doc, because AD-11 makes that file the
definition of release readiness: "Release readiness is `docs/RELEASE-CHECKLIST.md`"
(`ARCHITECTURE-SPINE.md:284`). The spine's own governing document contradicts the
spine's own closure claim, and revision 9 — a reconciliation pass that touched
that closure — does not note it. The `macos-14` row models the right handling
("**Caveat for a future currency check:** … `docs/development-guide.md`,
`docs/index.md`, and `_bmad-output/project-context.md` still say `macos-14`");
the app-update row needs the same caveat, and unlike those three, `RELEASE-CHECKLIST.md`
is hand-maintained, not generated, so it can simply be corrected.

### H5 — `EXPERIENCE.md` delegates its accessibility *method* to this spine, and the spine never supplies one

> `EXPERIENCE.md:315` — "Pack-Manager must meet the **packaged-app accessibility
> method approved by the Architecture Spine**."
> `EXPERIENCE.md:330` — "**Packaged acceptance verifies** focus, final-row
> reachability, selection scope, completion announcements, and no overlap at
> 100%, 150%, and 200% zoom with VoiceOver and 101 Package rows."

The input names the spine as the owner of the method. AD-11 supplies only:

> `ARCHITECTURE-SPINE.md:326-327` — "One manual VoiceOver pass and a by-eye
> contrast check sit on the release checklist."

And `docs/RELEASE-CHECKLIST.md:86-89` (item 9) covers only tab/arrow reach,
⌘X/⌘C/⌘V/⌘A, and "One VoiceOver pass over the Upgrade Plan". Nothing covers
100/150/200% zoom, final-row reachability at 101 rows, or selection scope. And
nothing covers *packaged*: AD-3 says no current suite crosses the transport,
AD-26 leaves the native harness OPEN with Story 6.5 as owner, and UX-PB.5d
(`epics.md:1099-1122`) asserts the zoom criteria without any statement that they
are verified on the packaged app.

`docs/DECISIONS.md` D33 legitimately narrowed DR-2 ("Automated 4.5:1 contrast and
reduced-motion checks belong in the existing Playwright/Vitest lane; one manual
VoiceOver pass joins the release checklist"), but D33 never addresses
`EXPERIENCE.md:330`'s packaged-acceptance list. So the obligation is neither
discharged, nor scheduled, nor retired — it is pointed at a document that does not
answer it. Either AD-11 states the packaged-app method (even if the answer is
"the release checklist and nothing more, and `EXPERIENCE.md:330` is narrowed to
that"), or the narrowing goes in `docs/DECISIONS.md` and the spine cites it.

---

## MEDIUM

### M1 — AD-16's inertness rule contradicts `EXPERIENCE.md` and `DESIGN.md` on disabled controls, unflagged

Detailed under question 2 above. `ARCHITECTURE-SPINE.md:542` "non-interactive to
pointer and keyboard" versus `EXPERIENCE.md:326` "do not use native `disabled` …
**retain focus**" and `DESIGN.md:203` "retain the same 2px `focusRing`". Three
inputs disagree; the spine picks one without saying so, and picks the one that
removes the focus indicator this same revision elevated to an invariant, and
makes the "stated reason for assistive technology" keyboard-unreachable. AD-16
predates revision 9, but revision 9 is the reconciliation pass with the
accessibility lens, and this is the sharpest accessibility contradiction in the
document.

### M2 — AD-11 cites D20 for a constraint D20 does not state, and D20 is superseded

> `ARCHITECTURE-SPINE.md:299-301` — "build on a **named stable runner image, never
> `macos-latest`** — a floating label would move the signing and notarization
> environment without a commit (`docs/DECISIONS.md` **D20**, D34)."

`docs/DECISIONS.md` D20's only runner sentence is "CI build-smoke runs on stable
macos-14 runners; beta-specific issues are diagnosed on-machine." Its "stable"
means *not a beta OS*, which is how D34 reads it — "the runner stays on a
**stable** image, **never a beta one**". The floating-vs-pinned rationale is
D34's alone, in its Rejected paragraph. Two further problems: D20's headline is
"notarization out of scope", which `docs/DECISIONS.md` D25a explicitly overturns
("**D20 is superseded.** It said notarization was out of scope"), and AD-12
requires notarization. The spine takes care to flag D31's partial supersedence
and takes none over D20's fuller one. Cite D34; drop D20 or mark it superseded
where cited.

### M3 — the D31 supersedence caveat is in the status table, not in the binding rule

Detailed under question 1. `ARCHITECTURE-SPINE.md:996` says it perfectly;
`ARCHITECTURE-SPINE.md:297-298` — the `Rule` a builder is bound by — cites D31
bare. Move one clause of the caveat into the rule.

### M4 — residual (1) is mis-diagnosed; it is the same failure mode as residual (2)

> `ARCHITECTURE-SPINE.md:1018` — "AD-25's actual rule for that path … **appears
> nowhere the builder reads**, so the citation points at an obligation the story
> text does not carry."

`epics.md:827` reads: "**Dependencies:** UX-PB.3c; D29-D30; AD-16 (verification-gated
success; post-exit fresh acquisition); **AD-25 (a failed verification refresh
leaves the Last-good Snapshot in place)**". The obligation *is* stated — in the
Dependencies parenthetical, which is exactly where residual (2) says AD-21's
lives. The real residual, in both cases, is "the substance never reaches criterion
prose". Stating it as two different problems will cause the next
`bmad-correct-course` run to fix them two different ways.

### M5 — the Styling convention carries token identity and nothing else

> `ARCHITECTURE-SPINE.md:928` — "| Styling | … Focus resolves `--color-focus-ring`
> and never `--color-accent`, so selection and focus stay distinguishable. |"

That row is where a builder looks for styling law. Absent from it: the ≥2px
minimum (`EXPERIENCE.md:318`, `DESIGN.md:202`, `docs/SPEC.md:208`), the
separation/offset (`DESIGN.md:202` "a **separated** 2px `focusRing` outline"),
"visible against every surface" (`EXPERIENCE.md:318`), and the two prohibitions
at `docs/SPEC.md:208` (`not ring-*`, `never outline-none`). The convention table
is the only part of the spine most stories read end to end; the focus law should
be complete there or explicitly point at AD-11.

### M6 — `docs/SPEC.md` §7.6 does not contain the CI lane AD-11's accessibility rules depend on

> `docs/SPEC.md:805` (§7.6) — "rust (macos-15): `cargo fmt --check`, `cargo clippy
> --all-targets -- -D warnings`, `cargo test --locked`. web (ubuntu): `npm ci`,
> `tsc --noEmit`, `vitest run`, `npm run build`. build-smoke (macos-15, main
> only) …"

§7.6 was correctly updated for `macos-15`, but it describes `ci.yml` only. It
never names `.github/workflows/test.yml`, Playwright, Chromium, or WebKit. AD-11
rests reduced-motion coverage, the focus-token negative guard, and the entire
WebKit-proof rule on that unlisted lane. `docs/RELEASE-CHECKLIST.md:36` knows
about it ("`test.yml` runs Playwright on Chromium and WebKit"); the CI spec does
not. Revision 9 verified §7.6's runner change and did not notice the lane it
depends on is missing from it.

### M7 — no `AD` carries D35's "the CI assertion moves in the same change" atomicity

> `docs/DECISIONS.md` D35 — "**Rejected:** changing the token values without
> moving the CI assertion in the same commit, which would simply turn the
> style-contract lane red — that lane is **also what AD-11 relies on for
> reduced-motion coverage, so it must stay green on every push**."

That is an atomic-change obligation of exactly AD-3's shape — theme values,
`tests/e2e/browser-style-contract.spec.ts`, and `docs/SPEC.md` §4.1 move
together, because splitting them takes down a lane another invariant depends on.
The spine has AD-3 for the IPC surface and nothing for this pair. The Styling
convention only forbids *different* values; it does not say how a sanctioned
change lands. `ARCHITECTURE-SPINE.md:994` mentions the property descriptively
("with the CI assertion moving in the same change") but as a report of what
happened, not as a rule.

---

## LOW

### L1 — the 3:1 large-text contrast tier is dropped

`EXPERIENCE.md:317` — "All ordinary text and controls target at least 4.5:1
contrast; **large display text targets at least 3:1**." AD-11 names only "4.5:1
text contrast" (`ARCHITECTURE-SPINE.md:322`). A story implementing the obligation
AD-11 assigns has no basis for the second tier.

### L2 — the focus indicator's own non-text contrast obligation has no home

AD-11's contrast sentence is scoped to *text*. `EXPERIENCE.md:318` ("visible
against **every surface**") and `DESIGN.md:119` ("Dedicated **high-contrast**
keyboard-focus ring") are non-text contrast requirements on the indicator itself,
and AD-11 explicitly says the lane "does not measure contrast"
(`ARCHITECTURE-SPINE.md:348-349`). So the requirement is stated by two inputs,
disclaimed by the lane, and assigned to nobody.

### L3 — AD-11 flattens the checklist's own gate/smoke-test tiering

`docs/RELEASE-CHECKLIST.md:61-63` — "Items 7–9 are **post-publish smoke tests,
not gates**. By the time they can run, the release is live and installed clients
have already begun polling. They catch a bad release; **they cannot prevent
one**." The accessibility pass AD-11 leans on is item 9 — post-publish. AD-11's
headline, "Release acceptance is the checklist plus two automated checks", reads
as though the whole checklist is preventive. The checklist's three-tier
structure (blocking automated / pre-merge preventive / post-publish detective) is
load-bearing and the `AD` does not carry it.

### L4 — internal inconsistency: 22 focus sites vs 31, in one revision

`ARCHITECTURE-SPINE.md:994` — "All **22** `focus-visible` sites resolve
`--color-focus-ring`" versus `:1019` — "**31** sites, with zero
`ring-focus-ring` … the site count going from 22 to 31". Both are true of their
own moment (`be1f0e6` and `22ed41e`), but the first row does not date its number
and reads as current. Add "as of `be1f0e6`" to the first, or reconcile.

---

## What revision 9 got right (verified, not taken on trust)

Recording these so a later pass does not re-litigate them:

- **`macos-15`.** All three pins verified in the workflow files; no `macos-14`
  anywhere in `.github/workflows/`.
- **The `notarytool` closure is evidenced, not asserted.** Run `30154432651`,
  conclusion `success`, `headSha` = `419dc32`. This is what D31 demanded and it
  actually happened.
- **The Rust app-update guard.** `src-tauri/src/commands.rs:772` / `:810`, with
  the frontend-parity claim checkable against `src/store/operations.ts`.
- **The deferred-work closure.** Both entries removed by `22ed41e`; neither
  survives.
- **The seven-item `epics.md` batch.** Spot-checked (b), (d), (g): `AllEligible`
  survives exactly once at `epics.md:590` and only as the negation; AD-26 is
  cited six times across the harness locations; UX-PB.5b restates AD-22's
  ordering verbatim in criterion prose at `epics.md:1058`.
- **The "three → nine" correction** at `ARCHITECTURE-SPINE.md:1019` is a genuine
  methodological finding (grep cannot find an absent class) and is worth keeping
  in the document permanently.

## Suggested order of work

1. **C1** — widen AD-11's `Binds` and its map rows, or split the focus rules into
   their own id. Nothing else in this list matters until the rules bind someone.
2. **C2** — either restore D35's uniform-mechanism rule and its `outline-none`
   prohibition verbatim, or state the override and accept D35's stated cost.
3. **H3** — add the three missed `epics.md` residuals to `ARCHITECTURE-SPINE.md:1018`
   before the next `bmad-correct-course` run; they are the closures this revision
   made that never reached the build queue.
4. **H4** — correct `docs/RELEASE-CHECKLIST.md:84` (hand-maintained, one line).
5. **H1, H2** — disclaim the Linux-WebKit lane's limit; requote SPEC §4.1.
6. **H5, M1** — the two places an input asks the spine a question it does not
   answer, and the place it answers against the input without saying so.
