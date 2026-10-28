# Divergence review — ARCHITECTURE-SPINE.md revision 9

**Lens:** adversarial divergence. Construct pairs of stories one level down that
each obey every `AD` to the letter and still build incompatibly.

**Target:** `_bmad-output/planning-artifacts/architecture/architecture-Pack-Manager-2026-07-23/ARCHITECTURE-SPINE.md`,
`artifact_revision: 9`, read 2026-07-25.

**Units one level down:** `_bmad-output/planning-artifacts/epics.md` — Epic UX-PB's
28 stories (UX-PB.1a–5e) plus Stories 2.2, 3.1, 3.2, 3.4, 3.5, 6.5.

**Read this session (every quote below is from one of these):**
`ARCHITECTURE-SPINE.md`, `epics.md`, `docs/SPEC.md`, `docs/DECISIONS.md`,
`_bmad-output/planning-artifacts/ux-designs/ux-Pack-Manager-2026-07-23/DESIGN.md`,
`.../EXPERIENCE.md`, `tests/e2e/browser-style-contract.spec.ts`,
`playwright.config.ts`, `.github/workflows/test.yml`, `src/styles/theme.css`,
`src/store/packages.ts`, `src/components/primitives/Checkbox.tsx`,
`src/components/primitives/Button.tsx`, `src/components/manager/PackageRow.tsx`,
`src/components/shell/ToastHost.tsx`, `src/components/manager/managerPane.test.tsx`.

**Note on a moving target.** The spine changed under me mid-review (1014 → 1019
lines; AD-11 moved from line 278 to 279 and its focus-paint rule gained the
`appearance`-based discriminator). I re-read AD-11 verbatim at the end and all
quotes below are from the later text. Line numbers elsewhere may have shifted by
±5.

---

## Verdict

**Revision 9's new material introduces more divergence than it closes.** The
two new AD-11 rules are correct as engineering statements and wrong as spine
rules: they are *product* invariants filed under an AD whose `Binds:` line reads
`release`, cited by zero stories, absent from `epics.md`'s "four invariants bind
every story" list, and enforced by a lane the same rules disclaim. The focus
rule now names **two** compliant mechanisms and declines to fix offset, so two
builders can obey it to the letter and ship visibly different focus on two
checkboxes — and the working tree already does.

Separately, the largest hole in the spine is not focus at all. **The spine has
no model of *selection*.** `PlanIntent` under AD-16 has membership and
tombstones; the shipping store has a live `selection: Partial<Record<ManagerId,
Set<string>>>`; `docs/SPEC.md` F5 and `EXPERIENCE.md` give opposite answers on
whether the row checkbox *is* membership or feeds it. Six live stories touch
that control.

**Tally: 27 findings — 6 CRITICAL, 8 HIGH, 9 MEDIUM, 4 LOW.**

| Tier | Count | Ids |
| --- | --- | --- |
| CRITICAL | 6 | C-1 … C-6 |
| HIGH | 8 | H-1 … H-8 |
| MEDIUM | 9 | M-1 … M-9 |
| LOW | 4 | L-1 … L-4 |

Ranking is by **reachability** — whether the two named stories, building only
from their own criteria plus every `AD` that binds them, actually arrive at the
divergence. Hypotheticals are tiered down.

---

## CRITICAL

### C-1 — The spine has no model of selection, and `PlanIntent` cannot represent one
**Stories: 3.5 and UX-PB.1a.** (Also reached by Story 3.2 and UX-PB.1d.)

AD-16's "Normative domain minimum" fixes `PlanIntent`, `PlanMember`,
`RetryIntent`, `UpgradePlanPreview`, `PlanAttempt`. There is no selection set
anywhere in it, and the Frontend-state convention says only "Narrow Zustand
selectors in components … Per-manager phase is derived, never stored."

The two authoritative UX sources give opposite answers:

- `docs/SPEC.md` F5: "Checkbox per eligible outdated row, header tri-state over
  visible selectable rows, shift-click range, Cmd-click toggle, and Cmd+A select
  all visible. `Add N to Plan` **immediately adds the checked canonical
  identities to the persistent plan and clears the transient selection**." — two
  states, selection is transient and separate.
- `EXPERIENCE.md:143`: "On eligible Package rows, **selection immediately
  adds/removes Upgrade Plan membership**." — one state, the checkbox *is*
  membership.

And `epics.md` carries both:

- **UX-PB.1a**: "**When** I toggle its plan Checkbox by pointer, Enter/Space, or
  the grid Space key **Then** the Package's canonical identity is added to the
  one persistent draft Upgrade Plan" — one state.
- **Story 3.5**: "**When** toggle, shift-range, tri-state, Cmd+A, Space,
  Cmd-click, Clear, and Esc interactions execute **Then** the exact selectable
  identities and visible filter semantics are preserved **And** excluded rows
  never enter selection." — a `selection` noun that is not membership, plus a
  separate criterion for "the single-row plan action".
- **Story 3.2** enumerates them as distinct paths: "selection, row plan-add,
  per-Manager update-all, **update-selected**, and Update Everything draft-entry
  paths".

`src/store/packages.ts:17` ships the second model today:
`selection: Partial<Record<ManagerId, Set<string>>>` with `toggle`, range,
`set`, `clear` primitives.

**Why both builders are compliant.** No `AD` names selection, so neither story
violates one. UX-PB.1a's builder deletes the selection store because AD-17 says
"Rust owns the canonical `PlanIntent`. The Zustand draft store is a projection"
and a second client-side set of package ids reads as exactly the unauthorized
projection AD-17 forbids. Story 3.5's builder keeps and extends it because their
criteria require range, tri-state, anchor, Clear, and Esc semantics that
`PlanIntent` has no shape for.

**What breaks.** Under the one-state reading, `Esc` (SPEC §4.11: "Esc clear
selection") mass-removes plan membership and, under AD-23, writes a tombstone
per removed ref that "no bulk expansion re-adds". Under the two-state reading
`Esc` is free. Shift-range on membership produces N `Explicit` members; on
selection it produces none. `update-selected` has a source in one model and no
source in the other.

**Close it.** A new `AD` that either (a) declares transient selection a
first-class, session-only frontend concept that is *not* `PlanIntent`, names
which actions bridge selection → membership, and states that clearing selection
never writes a tombstone; or (b) declares selection abolished, the checkbox is
membership, and restates Story 3.5's range/tri-state/Clear/Esc criteria in
membership terms — including what `Esc` does to tombstones. AD-23's tombstone
lifetime rule makes this urgent: under (b), one `Esc` can permanently poison a
draft against `Update Everything`.

---

### C-2 — AD-11's focus rule names two compliant mechanisms and refuses to fix offset
**Stories: UX-PB.1a and UX-PB.5b.**

AD-11 (revision 9) now reads: "`outline` with `outline-offset` satisfies the rule
everywhere and is the mechanism to reach for; **stripping a control to
`appearance: none` and styling it fully is the only other way to earn
`box-shadow`**. The requirement is that focus be painted where the user runs it,
not that any particular property is used."

The very next rule: "It does not assert that a given interactive element has one,
**does not assert offset**, and does not measure contrast."

The Styling convention pins only colour: "Focus resolves `--color-focus-ring` and
never `--color-accent`."

So two stories that each add a native checkbox can both be fully compliant and
visibly different:

- **UX-PB.1a** adds the package-row plan-membership checkbox. Ships
  `outline` + `outline-offset-1`, `--color-focus-ring`. Compliant.
- **UX-PB.5b** adds the `Disable upgrade plan command execution confirmation`
  checkbox — "only this dialog contains" it, so it is a new control this story
  owns. Ships `appearance: none` + `ring-2 ring-focus-ring` (or
  `outline-offset-2`). Also compliant, by the sentence AD-11 added this revision.

**This is not hypothetical — the tree already diverges.** With the in-flight
remediation applied: `src/components/primitives/Button.tsx:38`
`"…focus-visible:outline-2 focus-visible:outline-focus-ring
focus-visible:outline-offset-2"` versus `src/components/primitives/Checkbox.tsx`
`"focus-visible:outline-2 focus-visible:outline-focus-ring
focus-visible:outline-offset-1"`. 30 sites at offset-1, 1 at offset-2
(`grep -rho "focus-visible:[a-z0-9:.\[\]/-]*" src/ | sort | uniq -c`). `DESIGN.md`
requires the opposite — Button "Keyboard focus uses a separated 2px `focusRing`
outline"; Checkbox "Explanatory-disabled controls look unavailable but retain
**the same** 2px `focusRing`" — but "the same" is `DESIGN.md`'s word, and the
Styling convention adopts `DESIGN.md`'s ***values***, not its component-state
table.

**Worse: AD-11 contradicts the floor it names.** AD-11's last rule makes
`docs/SPEC.md` §4.1 "the floor it is verified against". `docs/SPEC.md` §4.1 says:
"Use `outline-*`, **not** `ring-*`: … **the rule is uniform so no control can be
given an invisible focus state by following its neighbours**." SPEC bans `ring-*`
for focus categorically; AD-11 permits it on an `appearance: none` control. A
UX-PB.1a builder citing SPEC and a UX-PB.5b builder citing AD-11 are each
citing an authority the spine hands them.

**Close it.** Tighten AD-11 to one mechanism (`outline` + a **stated**
`outline-offset` value) for every focus indicator in the product, or move the
mechanism out of AD-11 into a styling `AD` that fixes property, width, colour
token, *and* offset. If `appearance: none` + `box-shadow` is genuinely to stay
legal, say which surfaces may use it and why the uniformity SPEC §4.1 demands is
being given up.

---

### C-3 — AD-11 `Binds: release`, so the two stories that will ship the focus indicator never read it
**Stories: UX-PB.1a and UX-PB.5d.**

AD-11's header is unchanged: "**Binds:** release". Its title is "Release
acceptance is the checklist plus two automated checks". The
Capability → Architecture Map routes AD-11 to exactly one row: "Packaged
release, signing, updater | `release.yml` + `docs/RELEASE-CHECKLIST.md` |
AD-11, AD-12."

`grep -n "AD-11" epics.md` returns **five** hits: lines 193, 216, 265, 289, 290 —
all in the retired-register / FR-map / risk-table prose. **No story cites AD-11
in a Dependencies or Governing-invariants line.** The 28 UX-PB stories carry no
`Governing invariants` field at all; their AD citations live on the
`**Dependencies:**` line, and AD-11 appears on none of them. `epics.md`'s
explicit escape hatch — "**Four invariants bind every story and are cited by none
of them** … AD-1 … AD-2 … AD-20 … the Determinism convention" — does **not**
include AD-11.

So:

- **UX-PB.1a** adds the package-row plan-membership checkbox. Its Dependencies:
  "D27-D30; AD-16; AD-17; finalized UX spines". Nothing routes its builder to
  AD-11. This is *the exact control* AD-11's rule was written about ("the
  package-row plan-membership checkbox had no visible focus at all").
- **UX-PB.5d** is the accessibility story. Its Dependencies: "UX-PB.5a;
  finalized focus and high-zoom contracts; FR-19". It cites **no AD at all**.

The scope/title mismatch the attack question anticipated is real and it is the
delivery mechanism for the miss: a builder who reads the AD titled "Release
acceptance…" while implementing a checkbox is doing something unusual, and the
spine gives them no reason to.

**Close it.** Split the two new rules out of AD-11 into their own `AD` — bound to
"all frontend work; Epic UX-PB; Stories 3.1, 3.2, 3.4, 3.5" — and add it to
`epics.md`'s "binds every story and is cited by none" list. AD-11 keeps release
acceptance. Add a Capability → Architecture Map row for interactive-affordance
styling.

---

### C-4 — "Proven in WebKit" has no owner, no lane, and the only WebKit that runs is not WKWebView
**Stories: UX-PB.5d and 3.5.**

AD-11: "A focus indicator must be drawn by a mechanism the **shipping engine
actually paints**, and **proven in WebKit rather than Chromium alone**."

No sentence in the spine names who proves it, in which lane, at what gate. It is
an obligation in the passive voice.

Search for a candidate owner:

- **UX-PB.5d** is the accessibility story and the natural owner. It has no test
  level and cites no AD; its criteria are about focus trapping, accessible
  names, high zoom, and focus restoration — nothing about engines.
- **Story 3.5** is the only live story whose test level names a browser:
  "Required test level: Component plus browser E2E". Its criteria are keyboard
  selection and row plan actions; it never mentions focus *painting*.
- No UX-PB story carries a test level at all.

**And the proof surface is a proxy, not the shipping engine.**
`playwright.config.ts` declares `projects: [{ name: "chromium", … }, { name:
"webkit", use: { ...devices["Desktop Safari"] } }]`. `.github/workflows/test.yml`
runs both the `test` and `burn-in` jobs on `runs-on: ubuntu-latest`. Playwright's
`webkit` on Linux is not Apple's WKWebView on macOS, and the property in
question — whether `box-shadow` paints on a control still rendering with its
*native* appearance — is exactly the class of behaviour that differs between
WebKit ports, because the native appearance is supplied by the host platform's
control rendering.

AD-11's own account of the incident is the proof this lane is insufficient: it
attributes the miss to coverage ("the style contract's own focus assertion
targets a `button`"), but a rule that says "proven in WebKit" while the only
WebKit in CI is a Linux port has not fixed the engine problem — it has renamed
it. `docs/RELEASE-CHECKLIST.md` carries "one manual VoiceOver pass and a by-eye
contrast check"; the spine does not add a by-eye focus pass on the packaged app.

**Close it.** State the enforcement point explicitly: either (a) the obligation
discharges on the packaged macOS app via a named `docs/RELEASE-CHECKLIST.md`
step, and AD-11 says so; or (b) it discharges in the native harness AD-26
governs, which makes it a Story 6.5 dependency and moves the whole rule behind
an OPEN row; or (c) it discharges in the Playwright `webkit` project and the
spine states — and defends — that Linux WebKit is an adequate proxy for
WKWebView's native-appearance painting. Any of the three is fine. None being
stated is the defect.

---

### C-5 — Retry from a replay during a live attempt has nowhere to go
**Stories: UX-PB.4c and UX-PB.4d.**

AD-17: "Its visibility is a four-way union: a non-terminal attempt, a **derived
intent under review** (AD-24), undismissed Results, or a non-empty draft — **in
that content precedence, highest first**. … Higher precedence hides lower
content, never destroys it."

AD-16: "Exactly one confirmed attempt may be active. A second confirmation fails
closed with a typed already-active result."

Now walk the two stories:

- **UX-PB.4c** exists *precisely* for this situation: "**Given** a confirmed plan
  attempt is running when I open a History replay". It requires "the live sidecar
  stays visibly live" and `Back to live activity`.
- **UX-PB.4b**'s read-only carve-out is **unconditional**: "exactly one
  carve-out: the non-executing `Retry` affordance UX-PB.4d offers from a History
  entry." No clause suppresses it during a live attempt.
- **UX-PB.4d**: "`Create new plan` composes a derived `RetryIntent` in Rust …
  and takes that separate reviewable object **straight to preview and
  confirmation**."

So the user opens a History replay while an upgrade runs (UX-PB.4c's own Given),
invokes Retry inside it (UX-PB.4b permits it), and chooses `Create new plan`. The
derived intent is now "under review" — precedence rank 2 — while the non-terminal
attempt holds rank 1. **AD-17 hides the review surface behind the live attempt.**
And even if the user could see it, AD-16 fails its confirmation closed.

Both builders are compliant. UX-PB.4c's builder implemented every criterion they
were given; UX-PB.4d's builder implemented every criterion they were given.
Neither story's criteria mention the other's precondition. `EXPERIENCE.md:231`
compounds it: "`Create new plan` deliberately **replaces the sidecar** with a new
reviewable draft" — replacing a live attempt's sidecar is the one thing AD-17's
precedence forbids, and `EXPERIENCE.md` is a normative Dependency of UX-PB.3a and
UX-PB.5a. (It also calls the derived intent a "draft", which AD-24 spent a whole
`AD` separating.)

**Close it.** Add a rule to AD-24: while a confirmed attempt is non-terminal, the
Retry affordance in a History replay is either disabled with a stated reason, or
it may reveal the scope but `Create new plan` is refused with the same typed
already-active result AD-16 uses. State which. And correct or supersede
`EXPERIENCE.md:231`'s "replaces the sidecar with a new reviewable draft".

---

### C-6 — AD-18 fixes the journal's *home* but not its *writer* or its record cardinality
**Stories: UX-PB.3d and UX-PB.4a.** (UX-PB.2c is the third writer.)

AD-18's `Prevents` is exact about what it solves: "the writer, the History
reader, and the diagnostics exporter each choosing a **different home** for
attempt records". Its rules fix location, durability, compaction, retention,
disclosure, and correlation. They never fix **who appends**, **how many records
one attempt produces**, or **how the reader folds them into one row** — and the
journal is "**append-only** NDJSON", so a terminal outcome cannot be an in-place
update of the admission record.

Three live stories each own a durable write and each has its own failure
criterion for it:

- **UX-PB.2c**: "the append-only record stores the reviewed Manager/Package
  scope … and result/verification state as immutable plan-admission metadata" —
  and "**When** persisting the reviewed intent or command snapshot fails **Then**
  … no partial attempt record is left behind".
- **UX-PB.3d**: "**When** the transformed persistent Results / terminal outcome
  **cannot be written** **Then** the failure to persist is surfaced honestly".
- **UX-PB.4a**: "**When** its **single immutable History row cannot be
  persisted** **Then** the write failure is surfaced honestly" — and the happy
  path: "exactly one immutable History row is created for that `planAttemptId`
  … **And** no attempt ever yields more than one row".

AD-18 itself says "A story that adds a field to **the attempt record** owns its
disclosure review" — singular. So the spine's own prose assumes one record per
attempt, while its append-only rule guarantees at least two (admission at mint,
terminal at completion) and its story set has three writers.

**The divergence.** UX-PB.4a's builder, reading "exactly one immutable History
row", implements History as a **separate durable row written at terminal** —
which double-records against UX-PB.3d's terminal write. UX-PB.3d's builder,
reading "the transformed persistent Results … cannot be written", implements the
terminal append and treats History as a **read** — at which point UX-PB.4a's
failure criterion has no code to attach to. Both are compliant. One produces two
terminal records per attempt; the other produces one and a story with an
untestable criterion.

AD-19 sharpens the consequence: "an unparseable line is skipped and counted, the
surrounding records stay readable". If History is a fold, a skipped terminal line
silently turns a completed attempt into an interrupted one. UX-PB.4a forbids the
opposite direction only ("no completed outcome is fabricated for work that did
not finish"); nothing forbids a completed attempt reading as interrupted.

**Close it.** Extend AD-18: name the single writer (the Rust plan-attempt store),
state that an attempt is **a fold over its records** and History and diagnostics
are readers not writers, fix the record kinds (`admitted`, `terminal`, and
whether `verifying` is journaled), and state the fold rule when a record is
missing or unparseable — including which way an attempt with an admission record
and no terminal record must read.

---

## HIGH

### H-1 — AD-21 assigns its own narrowing to "whichever lands first", and one of the two never reads it
**Stories: UX-PB.5b and 3.4.**

AD-21's closing rule: "The shipping call site bumps unconditionally for every key
(`src-tauri/src/commands.rs` `set_settings_core`). Narrowing it is product work
owned by **whichever of UX-PB.5b or Story 3.4 lands first**, not a test concern
(AD-1)."

"Whichever lands first" is a scheduling accident, not an owner. And the two
stories are not symmetrically informed:

- **UX-PB.5b** cites it: "AD-21 (`skipUpgradePlanConfirmation` is declared
  plan-inert)". (Its *criteria* still never state the substance — the spine
  records this itself in the `epics.md` residuals row.)
- **Story 3.4** does not. "Governing invariants: AD-4, AD-5, AD-19". AD-21 is
  absent — even though AD-21's own `Binds` line names it: "**Binds:** UX-PB.2b,
  UX-PB.5b, UX-PB.5c; **Story 3.4**; all settings work".

And Story 3.4 introduces exactly the keys AD-21 classifies as plan-determining:
"the retained editable **stall threshold, hard cap**, and log level plus
`skipUpgradePlanConfirmation`". AD-21: "any execution parameter the reviewed
snapshot records, **timeouts and stall thresholds among them**, because the user
reviewed a plan that runs under them."

If Story 3.4 lands first its builder never learns that classification exists,
adds three keys to `set_settings_core` with no declaration site, and UX-PB.5b
later retrofits classification onto a call site it does not own. AD-21's
fail-closed default limits the blast radius to unnecessary re-reviews rather than
unreviewed execution — which is why this is HIGH not CRITICAL — but the
divergence itself is certain.

**Close it.** Name one owner. Add AD-21 to Story 3.4's Governing invariants and
restate the plan-inert/plan-determining declaration requirement in its criteria.

### H-2 — Bulk scope: "visible rendered rows" vs "every identity matching the filter"
**Stories: UX-PB.1d and 3.5.**

- **UX-PB.1d**: "the bulk header Checkbox scope covers only eligible Packages
  **matching the active filter** and adds no ineligible identity."
- **Story 3.5**: "**When** toggle, shift-range, tri-state, **Cmd+A**, Space,
  Cmd-click, Clear, and Esc interactions execute **Then** the exact selectable
  identities and **visible filter semantics** are preserved."

The sources contradict:

- `docs/SPEC.md` F5: "header tri-state over **visible** selectable rows … and
  **Cmd+A select all visible**." §4.11: "Cmd+A select all **visible** selectable
  rows".
- `EXPERIENCE.md:143`: "The header Checkbox applies to every eligible Package
  identity matching the active filter, **including off-screen virtualized
  rows**".
- `DESIGN.md:252`: "Do not … **treat only rendered virtual rows as a
  bulk-selection scope**".

`docs/SPEC.md` §0.1 "Superseding update-experience contract" lists what it
supersedes; **selection scope is not on that list**, so SPEC F5's "visible" is
live text. AD-16 fixes only that a bulk expansion is *frozen* ("the scope
predicate never runs a second time"); it does not fix the predicate's **domain**.
So a Story 3.5 builder freezes "the 40 rendered rows" and a UX-PB.1d builder
freezes "all 340 matching the filter" — both frozen, both compliant, different
plans from the same click.

**Close it.** One sentence in AD-16 or AD-23: a bulk expansion's domain is every
identity matching the active filter, never the rendered viewport. Then correct
`docs/SPEC.md` F5/§4.11 or add selection scope to §0.1's supersession list.

### H-3 — No stated precedence among the spine, `docs/SPEC.md`, `DESIGN.md`, and `EXPERIENCE.md`
**Stories: 3.5 and UX-PB.1b** (concrete victim below); H-2 and C-5 are also
instances.

`epics.md` states one precedence and only one: "Where this document and the spine
disagree, **the spine is upstream and wins**." Nothing states spine-vs-UX-spines
or spine-vs-SPEC. Meanwhile revision 9 *increased* the number of authorities in
play:

- Styling convention: "The *values* are `DESIGN.md`'s, adopted under
  `docs/DECISIONS.md` D35 — a story proposing different ones is proposing a new
  decision".
- AD-11: "`docs/SPEC.md` §4.1 … plus `EXPERIENCE.md` … are **the floor** it is
  verified against."

Three documents, two of them promoted to authority in the same revision, no
ordering. A concrete live victim outside the focus area: `docs/SPEC.md` §4.11 is
still live text and reads "Cmd+U upgrade selected (**opens sheet**) · … Esc clear
selection / **close sheet** / **close drawer** · **Cmd+L toggle drawer**". AD-17
retires the drawer by name ("The existing `ActivityDrawer` surface retires with
the `autoOpenDrawer` setting"), and §0.1's supersession list covers
"Activity-drawer-only … descriptions" but not the keyboard map. **Story 3.5**
owns keyboard interactions and would wire `Cmd+L` and `Esc` per §4.11;
**UX-PB.1b** owns the sidecar and UX-PB.3a owns Activity-as-destination, neither
of which has a drawer to toggle.

**Close it.** Add a precedence sentence to the spine's front matter or to the
Consistency Conventions: the order of authority is `ARCHITECTURE-SPINE.md` >
`docs/DECISIONS.md` > `DESIGN.md`/`EXPERIENCE.md` > `docs/SPEC.md` (or whatever
the real order is), and a conflict is a spine change, never a story's call.

### H-4 — The single status-announcement channel has no owning story, and `ToastHost` is an unretired second live region
**Stories: UX-PB.3a and UX-PB.3d.**

AD-17: "There is **exactly one** status-announcement channel for plan and attempt
progress, **owned alongside the sidecar region**. … Stories announce through it;
none adds a second live region for the same information, and Brief Notifications
suppress speech the channel already emitted. **Two live regions narrating one
attempt is a defect, not additive coverage.**"

"Owned alongside the sidecar region" is not a story. The sidecar's lifecycle
story is **UX-PB.1b**, whose criteria never mention a live region. The first
story that *needs* one is **UX-PB.3a** ("the status channel announces plan
start"); the second is **UX-PB.3d** ("one atomic outcome summary is announced");
**UX-PB.3f** needs it escalated to assertive for `Interaction required`, which
AD-17 requires but UX-PB.3f's criteria never mention. So whichever of 3a/3d
lands first invents the region, its DOM home, and its default politeness, and the
others mutate a region they do not own.

**And AD-17 retires the wrong surface.** It names `ActivityDrawer` explicitly.
It says nothing about `ToastHost`, which is a live region today —
`src/components/shell/ToastHost.tsx:30` `role="status"`, line 65
`aria-live="polite"` — and which `docs/SPEC.md` §4.10 still specifies as
narrating per-operation outcomes: "success auto-dismiss 4s (\"brew: 1 package
upgraded\"); failure persists with `View log`". No live story retires, mutes, or
reconciles it. An attempt of 12 packages therefore emits 12 toast announcements
plus one atomic summary — precisely the "two live regions narrating one attempt"
AD-17 calls a defect, with no story accountable for preventing it.

**Close it.** Name the owning story for the channel in AD-17 (UX-PB.1b or
UX-PB.3a) and add `ToastHost` to the same sentence that retires `ActivityDrawer`
— retired, or explicitly demoted to non-announcing with the suppression rule
owned by a named story.

### H-5 — Four new `OpStatus` variants, four atomic AD-3 changes, no tie-break — and one variant nobody classified
**Stories: UX-PB.2e and UX-PB.3c.**

AD-16: "`OpStatus` ships seven variants today, so **every addition moves as one
atomic AD-3 change** across the Rust enum, `src/lib/ipc/types.ts`, the guards,
and `dev/fixtures/ipc/*.json`."

Compare AD-23, which faced the same problem and solved it explicitly: "UX-PB.1a
and UX-PB.1c may not land it independently; **whichever runs first lands the
complete shape and the other builds against it**." AD-16 has no such sentence for
`OpStatus`, and four stories add variants: UX-PB.2e (`Skipped`, `Cancelling`),
UX-PB.3c (`Verifying`), UX-PB.3f (`Interaction required`), UX-PB.3g (asserts
`Skipped`/`Cancelling` again). Each independently regenerates all 15 committed
fixtures via `PM_UPDATE_CONTRACT=1 cargo test ipc_contract`.

**The unclassified variant.** UX-PB.3c also requires: "**Then** it shows queued,
**waiting (with the lock or ownership reason)**, running (indeterminate unless
the adapter provides a trustworthy total), verifying, or a terminal state". AD-16
says "**The same answer governs every new operation state the UX-PB stories
introduce, not only those two**" — which makes `waiting` durable. The Frontend
state convention says the opposite for the adjacent concept: "**Per-manager phase
is derived, never stored.**" So UX-PB.3c's builder can model `waiting` as a
durable eighth-plus variant (AD-16's blanket clause) or as a projection of
`queued` + the scheduler's lock set (the convention), while UX-PB.2e's builder
models `Skipped`/`Cancelling` as durable. Replay of a crashed attempt then shows
`queued` where the user saw `waiting` — which is exactly the "replay must
reconstruct what the user saw" failure AD-16 wrote the rule to prevent.

**Close it.** Give AD-16 AD-23's tie-break sentence, enumerate the complete
target `OpStatus` set (including `waiting` or its explicit exclusion with a
reason), and require it to land as one change.

### H-6 — Three independent crash-relaunch reconstructors, and the spine tells each story to build its own
**Stories: UX-PB.2c and UX-PB.4a.**

Decision Status: "Crash/relaunch lifecycle controller | **Deferred (live
consumers)** | UX-PB.1b, UX-PB.2f, UX-PB.4e, and Story 6.5 each assert crash,
force-quit, or relaunch behavior … **until it exists those stories own their own
disposable-root setup**".

That deferral covers the *root*. It does not cover the **reconstruction
semantics**, and two stories not even in that list define them differently for
the same on-disk state:

- **UX-PB.2c**: "**Given** a `planAttemptId` was minted but its durable record
  was lost to a crash or forced quit mid-admission **When** Pack-Manager
  relaunches **Then** it reconstructs the attempt **only from durable
  plan-admission metadata that actually persisted** … and **never resurrects an
  unpersisted attempt** as a completed durable record."
- **UX-PB.4a**: "**Given** a confirmed attempt was admitted but the app crashed
  or relaunched before the attempt reached a terminal row **When** History
  reconciles on the next launch **Then** the in-flight attempt is **reconciled
  from its durable `planAttemptId` records into one honest row**, an attempt that
  never reached terminal is **shown as interrupted**."

Same input, two verdicts: 2c says an attempt with no persisted admission record
does not exist; 4a says an admitted-but-unterminated attempt appears as
interrupted. The boundary case — mint succeeded, journal append failed (AD-18
explicitly permits this: "an append failure is **nonfatal** to package
operations") — falls to whichever builder wrote the loader. AD-5's rule is about
Operations, not attempts: "An unfinished start is reconstructed as **Interrupted**
instead."

**Close it.** State in AD-18 that reconstruction happens once, in Rust, at load,
and that the durable record — not the mint — is what makes an attempt exist.
Extend AD-5's Interrupted rule to attempts explicitly.

### H-7 — AD-25's substance never reaches UX-PB.3d's criteria, and Story 2.2 carries the full text
**Stories: 2.2 and UX-PB.3d.**

The spine records this itself in the `epics.md` residuals row, which is to its
credit; it is listed here because it is a live divergence and the tier is earned
by the consequence.

- **Story 2.2** states AD-25 in full: "the recovered output **merges** into the
  inventory already parsed from the successful refresh outputs **And** the
  snapshot is never replaced by an empty one and never by an outdated-only
  overlay … the merge never un-pins a row, and health and staleness presentation
  read from the snapshot's real timestamp".
- **UX-PB.3d** cites AD-25 on its Dependencies line — "AD-25 (a failed
  verification refresh leaves the Last-good Snapshot in place)" — and its
  criterion says only: "the item does not declare success — it stays `Verifying`
  until it resolves, then reports verification failure with its evidence, and is
  never colored successful on the strength of the exit code alone."

Nothing in UX-PB.3d's criteria mentions the snapshot. AD-25's rule for exactly
this path — "A verification refresh that fails or times out marks the attempt's
verification failed and **leaves the Manager's Last-good Snapshot in place**" —
appears nowhere the UX-PB.3d builder reads. A builder implementing the
verification-refresh failure path from their own criteria can reasonably blank
the manager's inventory (the refresh failed, so there is no data), destroying the
evidence of what the mutation actually did.

**Close it.** Restate AD-25's verification clause in UX-PB.3d's criterion text.
Tracked already; it needs to land, not be re-recorded.

### H-8 — "Is this a port?" has two answers, and two stories will give different ones
**Stories: 3.4 and 6.5.**

AD-4: "**Five ports exist today** and are extended rather than bypassed:
`CommandRunner`, `EventSink`, `UpdateSource`, `PendingRelease`, and
`ManagerAdapter`." Then: "Effects the live build queue **newly introduces** go
through a port from the start — **specifically** the filesystem access AD-18's
attempt journal requires, and the clock any verification or staleness deadline
reads."

That "specifically" is a closed list of two. Two stories introduce effects on
neither list:

- **Story 6.5**: "Dependencies: disposable logs/transcripts/journal" and the
  criterion "**When** native command/opener success and failure are
  **controlled**". The Decision Status row is explicit: "Story 6.5 must introduce
  an opener/reveal seam as a **sixth port** under AD-4".
- **Story 3.4**: "Dependencies: controlled persistence and **clipboard seams**",
  with the criterion "**When** Environment Report opens and **Copy** is used
  **Then** … copy success and failure are visible and actionable."

Clipboard is a macOS effect whose success and failure Story 3.4 must control.
AD-4 does not name it, and the "Runtime effects" convention only says
"Application and domain code depends on typed ports. Direct OS calls live only in
production adapters" — which a builder reading `navigator.clipboard` in React
does not obviously violate, since React is presentation. So Story 6.5 adds a
sixth typed Rust port and Story 3.4 adds a frontend seam, and the product ends
with two different answers to "how do we control a macOS effect".

Also note AD-4's "Five ports exist today" is a rule sentence the spine's own
Decision Status row already contradicts with "a sixth port".

**Close it.** Replace the closed list with a test: any effect a live story must
control for success *and* failure goes behind a typed port in Rust. Update the
port count or stop counting.

---

## MEDIUM

### M-1 — The deliberate `ring-accent` survivor is governed by nothing
**Stories: 3.1 and 3.5.**

The Decision Status row: "One `ring-accent` survives deliberately at
`src/components/manager/PackageRow.tsx` — a cross-manager navigation highlight
with no `focus-visible:` prefix, kept distinct precisely so a navigated-to row
cannot read as a focused control." Verified: `PackageRow.tsx:85`
`highlighted ? "ring-2 ring-inset ring-accent" : ""`, asserted at
`src/components/manager/managerPane.test.tsx:114`.

That reasoning lives in a **Decision Status note**, not in an `AD` or a
convention. The Styling convention says only "Focus resolves
`--color-focus-ring` and never `--color-accent`" — which a builder reads as "no
accent rings", and deleting this one satisfies it. Meanwhile the package row now
carries **three** visual channels and the spine governs two: selection wash
(`--color-accent-subtle`), navigation highlight (`ring-accent`), roving focus
(`--color-focus-ring` outline). **Story 3.1** renders the row and its states;
**Story 3.5** owns roving focus and `DESIGN.md`'s "one visibly focused row". One
can delete the highlight as a stray accent ring, the other can convert it into
focus styling or add a fourth channel — and the test asserting it belongs to
neither story.

**Close it.** Promote the navigation-highlight rule into the Styling convention:
name the three row channels, their tokens, and that a navigation highlight is
never `focus-visible:`-prefixed and never substitutes for focus.

### M-2 — `aria-disabled` controls are focusable but not interactive, and the floor says "every interactive element"
**Stories: UX-PB.1d and UX-PB.1a.**

**UX-PB.1d** requires: "it uses `aria-disabled=\"true\"` rather than native
`disabled`, **keeps focus**, announces its persistent reason as an accessible
description, stays inert on activation". A focusable-but-inert control.

AD-11's floor is "`docs/SPEC.md` §4.1 … **on every interactive element**" and
`EXPERIENCE.md` "**Every interactive element** uses a separate
`{colors.focusRing}` indicator". An `aria-disabled` control is deliberately *not*
interactive — AD-16's domain rules call it "inert": "its control is
non-interactive to pointer and keyboard". So the floor arguably does not reach
it. `DESIGN.md` does — "Explanatory-disabled controls look unavailable but retain
the same 2px `focusRing`" — but the Styling convention adopts `DESIGN.md`'s
*values*, not its component-state table.

`src/components/primitives/Checkbox.tsx` ships `disabled` + `disabled:opacity-40`
today, so UX-PB.1d must rewrite the shared primitive that UX-PB.1a consumes. A
compliant UX-PB.1d ships `aria-disabled` + `opacity-40` and no indicator; a
compliant UX-PB.1a ships the eligible checkbox with one. Keyboard users then tab
into a control that vanishes.

**Close it.** Say in the focus rule that *focusable* — not *interactive* — is the
trigger for an indicator.

### M-3 — Programmatically focused headings do not match `:focus-visible`
**Stories: UX-PB.3a and UX-PB.5a.**

Four stories move focus to a heading: UX-PB.3a "focus moves to its
**programmatically focusable** Upgrade Activity summary heading"; UX-PB.3d
"moves to the Results heading"; UX-PB.4d "Move focus to the retry-scope heading"
(`EXPERIENCE.md:300`); UX-PB.5a "focus moves to the **dialog
heading**/command summary".

These are `tabindex="-1"` targets. `:focus-visible` heuristics generally do not
match a programmatically focused non-input element, so a
`focus-visible:outline-*` treatment — the mechanism AD-11 blesses and every one
of the 31 sites in the tree uses — paints **nothing** on them. AD-11 governs the
*property* (`outline` not `box-shadow`) and is silent on the *selector*. Two
builders diverge: UX-PB.5a uses `:focus` on the dialog heading (visible),
UX-PB.3a uses `:focus-visible` on the Activity heading (invisible). Both obey
AD-11 word for word, and the style-contract lane cannot catch it because it "does
not assert that a given interactive element has one".

**Close it.** Extend the focus rule to the selector: a programmatic focus target
draws its indicator on `:focus`, not `:focus-visible`.

### M-4 — "2px" vs "at least 2px", and the lane asserts exactly `"2px"`
**Stories: UX-PB.1e and UX-PB.5d.**

`DESIGN.md`: "Keyboard focus uses a separated **2px** `focusRing` outline".
`EXPERIENCE.md:318`: "a separate `{colors.focusRing}` indicator that is **at
least 2px wide**". `tests/e2e/browser-style-contract.spec.ts`:
`expect(focusTreatment.outlineWidth).toBe("2px")` — exact.

The epics register makes both stories build from the UX sources: "**Blocks
UX-PB.1e and UX-PB.5d** … Both stories are bound to build from the UX sources".
UX-PB.5d is the high-zoom story; a builder reading "at least 2px" and thickening
the ring at 200% zoom breaks the lane the whole reduced-motion regression surface
depends on. UX-PB.1e's builder reading "2px" does not.

**Close it.** Pick one number in the Styling convention.

### M-5 — Tri-state `mixed` needs a live predicate that AD-16/AD-23 say never runs twice
**Stories: UX-PB.1d and 3.5.**

`EXPERIENCE.md:143`: the header Checkbox "announces the exact count and uses
`mixed` when only some are staged." Computing "only some" requires evaluating
*current* filter matches against *current* membership.

AD-16: "A bulk mutation freezes its expansion into concrete members at the moment
it is made — **the scope predicate never runs a second time**." AD-23: "`scope`
is descriptive. It records which action created the member … and is **never
re-evaluated**."

Read narrowly (provenance is not re-derived) both are fine. Read literally, a
UX-PB.1d builder refuses to run the predicate again and derives `mixed` from
frozen `Bulk { scope }` membership — which goes stale the moment the filter
changes. A Story 3.5 builder, owning "tri-state" and "visible filter semantics
are preserved", evaluates live. Two header checkboxes, two truths.

**Close it.** One clause in AD-23: the freeze applies to *membership creation*;
display-time predicates for counts and tri-state are permitted and are never
authority.

### M-6 — `epics.md` still shows the design-token row as OPEN and blocking, with a line pointer that now resolves elsewhere
**Stories: UX-PB.1e and UX-PB.5d.**

The spine's Decision Status says "Canonical design-token set | **RESOLVED** …
**UX-PB.1e and UX-PB.5d are unblocked.**" `epics.md:308` still says: "Canonical
design-token set | `OPEN` — needs an owner decision | UX decides; Development
implements | **Blocks UX-PB.1e and UX-PB.5d**
(`ARCHITECTURE-SPINE.md:944`)."

Two problems. The row is stale — recoverable via "the spine is upstream and
wins", but a builder opening `epics.md` first sees BLOCKED. And the pointer is
stale in a way nothing recovers: `sed -n '944p'` on the spine returns
`| Tauri opener plugin | 2.5.4 |`. The Styling convention it meant to cite is
~20 lines away and moves every revision.

This is not in the spine's `epics.md` residuals row, which lists only UX-PB.3d's
AD-25 gap and AD-21's missing criterion text.

**Close it.** Add it to the residuals row for the next `bmad-correct-course` run.
And stop citing the spine by line number — `epics.md`'s own convention already
says "Cite AD ids by subject, never by rule ordinal"; extend it to line numbers.

### M-7 — The 720px responsive behaviour of the one sidecar region has two owners
**Stories: UX-PB.1b and UX-PB.5d.**

AD-17: "Below 720 usable CSS pixels the region stops being a fixed sidecar and
**the same single instance** is presented as a full-workspace or stacked surface.
Viewport is a placement driver, **never a second mount point**."

**UX-PB.1b** owns sidecar lifecycle and visibility ("when hidden the main
workspace reclaims its width with no reserved empty column") and never mentions
720px. **UX-PB.5d** owns "**Given** the 900 x 600 minimum window at 100%, 150%,
and 200% zoom **Then** below 720 usable CSS pixels the layout enters high-zoom
mode … Plan/Confirmation/Activity/Results present as a full-workspace or stacked
surface" — and its Dependencies are only "UX-PB.5a; finalized focus and high-zoom
contracts; FR-19", not UX-PB.1b.

So UX-PB.1b builds the fixed sidecar and UX-PB.5d builds the stacked surface,
with no dependency edge between them and AD-17's "never a second mount point"
being exactly what two unconnected builders produce.

**Close it.** Add the dependency edge, or name UX-PB.1b as the owner of the
region's placement across all viewports with UX-PB.5d verifying it.

### M-8 — Contrast is "an obligation on whichever story adds it", and no story adds it
**Stories: UX-PB.1e and UX-PB.5d.**

AD-11: "**Automated 4.5:1 text contrast does not exist** … Contrast is therefore
an obligation on whichever story adds it". NFR-6 requires "at least 4.5:1 text
contrast"; `docs/SPEC.md` §4.11 requires "text contrast ≥4.5:1 on its surface";
FR-19 maps to "Release checklist".

No live story's criteria mention contrast. **UX-PB.1e** introduces the coloured
surfaces (status badges, self-update deltas, "update availability is never
colored as a system-health problem") and **UX-PB.5d** is the accessibility story
— and neither owns measurement. "Whichever story adds it" is a way of writing
"nobody". The divergence form is weak (a gap, not a clash), which is why this is
MEDIUM rather than HIGH.

**Close it.** Either name the owner or state plainly that contrast discharges
only on the release checklist's by-eye pass and that no automated check is
planned — which AD-11 half-says already and should say fully.

### M-9 — AD-16 requires a preview `planId`; UX-PB.5c's bypass path never issues one
**Stories: UX-PB.5c and UX-PB.2b.**

AD-16: "`execute_plan` returns a newly generated durable `planAttemptId`" and
"Execution must **match the issued preview** and a fresh coherent rebuild".
**UX-PB.2b**'s Given is "a reviewed plan **authorized by a one-use preview
`planId`**".

**UX-PB.5c** is the confirmation-off path: "**When** I choose `Run N Updates`
**Then** Rust rebuilds the exact commands from canonical intent and runs the
stale-plan check before the plan is atomically admitted, so the bypass removes
only the final dialog and never the persistent plan, native rebuild, stale check,
or explicit user action." No `planId` is mentioned. AD-16 says the opt-out "skips
only the final modal — never draft review, the Rust rebuild, stale detection, or
the explicit confirmation action" — again silent on whether a preview capability
is minted and consumed on this path.

So UX-PB.5c's builder can rebuild-and-admit directly, while UX-PB.2b's admission
code path expects an issued `planId` to match against. One of them changes
`execute_plan`'s signature.

**Close it.** State in AD-16 that the confirmation-off path issues and consumes a
preview `planId` exactly as the dialog path does; only the modal is skipped.

---

## LOW

### L-1 — AD-11 quotes `docs/SPEC.md` §4.1 with a string that is not in it
AD-11: "`docs/SPEC.md` §4.1 (\"**offset against surface, on every interactive
element**\") … [is] the floor it is verified against."

`grep -rn "offset against surface" docs/` returns one hit and it is not SPEC:
`docs/DECISIONS.md:446`. `docs/SPEC.md` §4.1's actual sentence is "Focus: a real
2px `outline` in `--color-focus-ring` with `outline-offset`, **on every
interactive element** — a dedicated indicator, never `--color-accent`."

A rule that names a quotation as the floor should quote it correctly, because the
next builder will grep for the quoted string and not find it.

### L-2 — AD-4's "Five ports exist today" is contradicted by the spine's own Decision Status
AD-4 rule: "**Five ports exist today**". Decision Status: "Story 6.5 must
introduce an opener/reveal seam as a **sixth port** under AD-4". The count is a
brownfield observation living inside a rule sentence; it will be wrong the moment
Story 6.5 lands. See H-8.

### L-3 — AD-20 pins the capability set to exactly three permissions; AD-26's harness needs a fourth
AD-20: "One capability file grants the `main` window **exactly** `core:default`,
`opener:default`, and `core:window:allow-start-dragging`. Adding a permission, a
plugin, a window, or a second capability is a security-sensitive change reviewed
on its own terms and **never folded into a feature story as a side effect**."

AD-26 makes Story 6.5 buildable by registering `tauri-plugin-wdio-webdriver`
under `#[cfg(debug_assertions)]`. `cfg` gates Rust registration; **capability
files are JSON and are not `cfg`-gated**, so a debug-only plugin still needs its
permissions declared in a file AD-20 pins to three entries. AD-26's
compile-time-exclusion rule does not reach the capability file, and AD-20 forbids
folding the change into Story 6.5 — which is a feature story (diagnostics
export). Tiered LOW only because both ADs already route this to a named
security review and the Decision Status row keeps it OPEN; it is a real
AD-vs-AD contradiction that will surface the day Story 6.5 starts.

### L-4 — `Cancel` label reservation is stated in one story and contradicted in a live source
UX-PB.3g: "it is the only place labeled `Cancel operation`, while generic
`Cancel` is reserved for closing a dialog or retry-scope editor without mutating
running work." `docs/SPEC.md` §4.10 still specifies `UpgradePlanSheet` footer as
"**Cancel** / `Upgrade` (primary)" and `QuitGuardDialog` as "\"Cancel operations
and quit\" (danger)". §0.1 supersedes the sheet's *existence* but not its labels.
Cosmetic; listed for completeness.

---

## What revision 9 got right

Recorded so the tally is not read as a rejection of the revision:

- The focus-paint rule is a genuinely good invariant and the `appearance`-based
  discriminator added mid-review is the correct generalization — the problem is
  its **filing** (C-3) and its **permissiveness** (C-2), not its content.
- Stating what the style-contract lane does **not** assert is the single most
  valuable sentence added this revision. It closes the "green run as evidence"
  hole for focus the same way AD-3 closed it for fixtures. Findings C-4, M-2,
  M-3, and M-8 all exist *because* that sentence is now true and unowned — the
  spine surfaced them rather than hiding them.
- The `epics.md` residuals row is an honest self-report: it names UX-PB.3d's
  AD-25 gap (H-7) and AD-21's missing criterion text (H-1) before a reviewer
  did. Both are tiered here for consequence, not for being undisclosed.
- Closing the five Open rows against the committed tree rather than the report of
  it is verifiable and verified: `--color-focus-ring: #F4F7FB` and
  `--color-accent: #65A7FF` are in `src/styles/theme.css:19,27`; no `runs-on` in
  `.github/workflows/` names `macos-14`.

---

## Proposed changes, in priority order

1. **New `AD` for interactive-affordance styling**, bound to all frontend work
   and Epic UX-PB, carrying the focus mechanism (one property, one offset value,
   one width, `:focus` for programmatic targets, focusable-not-interactive as the
   trigger), the row's three visual channels, and the source-precedence sentence.
   Add it to `epics.md`'s "binds every story, cited by none" list. — closes C-2,
   C-3, M-1, M-2, M-3, M-4.
2. **New `AD` (or AD-16 extension) for selection**, deciding whether transient
   selection exists and what `Esc`/`Clear` do to tombstones. — closes C-1, and
   H-2/M-5 fall out of it.
3. **AD-18 extension**: one writer, attempt-as-fold, record kinds, missing-record
   read direction. — closes C-6, H-6.
4. **AD-11 rewrite of the WebKit clause** to name its enforcement point. —
   closes C-4.
5. **AD-24 extension**: Retry availability while an attempt is non-terminal. —
   closes C-5.
6. **AD-16 tie-break sentence for `OpStatus`**, modelled on AD-23's. — closes
   H-5.
7. **AD-17**: name the status-channel owner and retire `ToastHost`'s narration.
   — closes H-4.
8. **AD-21**: replace "whichever lands first" with a named owner; add AD-21 to
   Story 3.4. — closes H-1.
9. **AD-4**: replace the closed effect list with a test; drop the port count. —
   closes H-8, L-2.
10. **Residuals row**: add `epics.md:308` and the `ARCHITECTURE-SPINE.md:944`
    pointer. — closes M-6.
