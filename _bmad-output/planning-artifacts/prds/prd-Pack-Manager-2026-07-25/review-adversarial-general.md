# Adversarial review — PRD Pack-Manager 2026-07-25

**Reviewer lens:** adversarial-general
**Target:** `_bmad-output/planning-artifacts/prds/prd-Pack-Manager-2026-07-25/prd.md` (659 lines) + `addendum.md` (69 lines)
**Baseline:** `HEAD` = `5972109` (`git log --oneline -1`)
**Excluded by instruction:** D33-retired gate apparatus, D37-removed keyboard/VoiceOver criteria, commercial-scale rigor, Planned-status D27–D30 absences, mechanism deliberately mapped to `addendum.md` §1, and the four already-known items (FR-14 quit guard, §7.3 health fixes, §7.1 contrast guard, FR-6 self-update call sites).

---

## What survived the attack

I tried to break the numbers first, because a PRD that invents constants is worthless downstream. It does not invent them. Every quantitative claim I checked is exact:

| PRD claim | Verified against |
| --- | --- |
| 20 IPC commands (§0.1) | `src-tauri/src/lib.rs:232-253` — 20 entries in `generate_handler!` |
| 6 events (§0.1) | `src-tauri/src/events.rs:77-82` — 6 `EVENT_*` consts |
| 8 settings fields (§0.1, FR-17) | `src-tauri/src/settings.rs:28-39` — 8 fields |
| 64 unconsumed capabilities (FR-8) | `src-tauri/src/state.rs:25` `"pub const ISSUED_PLAN_LIMIT: usize = 64;"` |
| Concurrency cap 4 (FR-9) | `src-tauri/src/queue.rs:48` `"pub const MAX_CONCURRENCY: usize = 4;"` |
| 120s stall / 30min cap (FR-14) | `src-tauri/src/settings.rs:46-47` |
| 14d / 200 files / 90d / 1,000 records (FR-15) | `src-tauri/src/logging.rs:26-28`, `src-tauri/src/journal.rs:19` |
| 50 ms / 64 lines / 8 KiB (NFR-3) | `src-tauri/src/events.rs:183,185,187` |
| 5,000 live lines (NFR-3) | `src/store/operations.ts:17` `"export const LOG_CAP = 5000;"` |
| 3 logs / 25 transcripts (FR-18) | `src-tauri/src/diagnostics.rs:23` + test at `:251` |
| Six-hour app check (FR-20) | `src-tauri/src/app_update.rs:27` |
| 31 focus sites (FR-19) | `git grep -c "focus-visible:outline" HEAD -- src/` summed = 31 (excl. one test file) |
| 28 `epic-ux-pb` stories (§7.2) | `grep -c "^  ux-pb-" sprint-status.yaml` = 28 |
| 9 of 17 SPEC features lack acceptance, incl. 4 P0s (§0.1) | `grep -c "^\*\*Acceptance:\*\*" docs/SPEC.md` = 8; F9–F12 are P0 and have none |
| `docs/SPEC.md` in second commit `f395db3` (§0.1) | `git log --reverse` — `d857cf5`, then `f395db3` |
| Every positional citation I spot-checked | `epics.md:89`, `epics.md:113`, `ARCHITECTURE-SPINE.md:1050`, `docs/SPEC.md:19,108,128`, `ux .memlog.md:75`, `review-reconcile-epics.md:166` — all resolve to the quoted text |

That is a genuinely high grounding floor. The findings below are what is left after that floor.

---

## THE SINGLE WORST THING

### FR-5 mandates the one mechanism that destroys the one behavior D37 explicitly protected — and it contradicts itself inside two consecutive bullets

`prd.md:198`:

> "Pinned formulae cannot enter the Upgrade Plan. The row keeps a visible disabled checkbox for table alignment and remains an explanatory interaction target: activating it never changes membership, and it states how to unpin and refresh."

`prd.md:200`, the very next bullet:

> "Up-to-date and otherwise ineligible Packages cannot enter the Upgrade Plan and expose a plain-language reason on pointer interaction. Ineligibility never relies on gray styling alone."

A native `disabled` form control is not an activation target and not a pointer-interaction target. "Keeps a visible **disabled** checkbox" and "remains an explanatory interaction target: **activating it**" cannot both be built. One sentence, two halves, mutually exclusive.

This is not a nitpick, because three other documents converge on it:

1. **`EXPERIENCE.md:143`** — the line the addendum declares survives — says the opposite in so many words:
   > "An explanatory-disabled control never uses native `disabled`: expose `aria-disabled=\"true\"`, attach its persistent reason as an accessible description, keep activation inert, and preserve focus for its supplemental Tooltip/Popover."

   And `addendum.md:54` says of that exact line:
   > "Its membership model (line 143) is **correct and survives** — only the accessibility limbs change."

   The PRD declares line 143 correct and then writes a requirement that violates the half of line 143 it did not carve out. `EXPERIENCE.md:326` repeats the rule independently: `"Explanatory-disabled Package Checkboxes do not use native \`disabled\`."`

2. **D37 protects this behavior by name.** `docs/DECISIONS.md:560-562`:
   > "**Story UX-PB.1d is not to be deleted**: its pointer-hover explanation of why a Package is ineligible is mouse-facing behavior, and only its keyboard and VoiceOver limbs are in scope here."

   The PRD agrees at `prd.md:438` — `"Pointer-facing explanations of *why* a Package is ineligible also stay"` — and again at `prd.md:543` (NFR-6, `"pointer-accessible ineligibility reasons"`). So the PRD retains the requirement and then specifies the mechanism that suppresses it.

3. **The shipping code already has the defect.** `src/components/manager/PackageRow.tsx:92` `"disabled={checkboxDisabled}"`, `:95` `"title={checkboxTitle}"`, `:100` `"\"disabled:cursor-not-allowed disabled:opacity-40\","`. The reason string is delivered by a `title` tooltip on a `disabled` input, and the only other cue is 40% opacity — which is precisely what `prd.md:200` forbids: `"Ineligibility never relies on gray styling alone."`

**Why this bites in six weeks.** `sprint-status.yaml:75` has `ux-pb-1d-...-pointer-and-voiceover-explanation: backlog`. Whoever builds it reads FR-5, sees "keeps a visible disabled checkbox", ships `disabled`, and the pointer explanation D37 spent a paragraph protecting silently never renders — for the *only user this product has*. Nothing tests it. The PRD stripped `aria-disabled` (correctly, per D37) and in doing so deleted the one word that made the requirement buildable.

**Fix:** rewrite `prd.md:198` to state the outcome without the broken mechanism, e.g. *"The row keeps a visibly inert checkbox for table alignment. It is not rendered with native `disabled`, because a natively disabled control cannot surface its own reason on pointer interaction; activation changes nothing and the persistent reason stays reachable by pointer."* Add the surviving `EXPERIENCE.md:143`/`:326` constraint to the addendum's list of what survives, so the `bmad-ux` Update does not strip it as an accessibility limb.

---

## Critical

### C1. The reconciliation queue omits the live build queue, and simultaneously orders the record of the FR-6 conflict destroyed

`addendum.md:48-54` names exactly three artifacts to reconcile: `epics.md`, `ARCHITECTURE-SPINE.md`, `EXPERIENCE.md`. `sprint-status.yaml` is not among them — yet `prd.md:595` cites it as authoritative:

> "This is the live build queue — 28 stories under `epic-ux-pb` in `_bmad-output/implementation-artifacts/sprint-status.yaml`"

Three live entries in that file encode behavior this PRD abolishes or that D37 removed:

- `sprint-status.yaml:112` — `"3-5-preserve-exact-keyboard-selection-and-row-plan-actions: backlog"`. Its acceptance criteria at `epics.md:1278` and `:1280` still require the transient selection layer:
  > "**When** toggle, shift-range, tri-state, Cmd+A, Space, Cmd-click, Clear, and Esc interactions execute"
  > "**And** excluded rows never enter selection."

  `prd.md:234` (FR-6 Out of Scope) says: `"A transient selection distinct from draft membership, and any \`Add Selected\` submit step. Both are eliminated."` Story 3.5 is `backlog` — buildable today, against a model FR-6 deletes. The addendum's `epics.md` row (`addendum.md:52`) names only FR-19, NFR-6, and FR-17 as stale. Story 3.5 is not mentioned anywhere in the PRD or addendum.
- `sprint-status.yaml:75` and `:98` carry `voiceover` and `accessibility` in the story slugs — the strings that become story filenames.

Worse, `prd.md:648` and `addendum.md:53` order the deletion of the only artifact that names this collision:

> `prd.md:648`: "`ARCHITECTURE-SPINE.md:1050` still records this as OPEN and routes it to the owner. It is now answered, and that row should be **retired** in the `bmad-architecture` Update that follows this PRD."

`ARCHITECTURE-SPINE.md:1050` is the row that states the concrete risk:

> "Story 3.5 (keyboard selection) and UX-PB.1a (staging) can each obey every existing `AD` and still build opposite models."

Retiring that row while leaving Story 3.5 live and unreconciled removes the warning and keeps the hazard.

**Compounding:** `addendum.md:53` gives the architecture run two incompatible instructions in one sentence — `"must be **retired**, not re-answered. Closing it writes the new invariant the row was waiting for: membership mutation is batch-capable and Rust-authored."` Retiring a row deletes it; writing an invariant creates an `AD`. If the run does the first, no `AD` ever models selection-vs-membership and the spine's own stated failure mode returns unguarded.

**Fix:** add `sprint-status.yaml` + Story 3.5 to the reconciliation queue with `bmad-correct-course` as the route; split `addendum.md:53` into two explicit actions ("write `AD-nn`: membership mutation is batch-capable and Rust-authored" **then** "retire row 1050 citing that `AD`").

---

## High

### H2. FR-20 declares automatic download non-optional; FR-17 ships the off switch

`prd.md:457-458`:

> "Checks run at launch, every six hours, and on demand from the application menu."
> "A newer authorized release downloads automatically in the background. Automatic **download** is required behavior, not an optional outcome — installation is the machine mutation, and that stays user-gated."

`prd.md:394` lists among the eight shipping settings:

> "and **check for application updates automatically (on)**."

`src-tauri/src/lib.rs:135-136` is unambiguous:

> "/// Launch check + a 6h heartbeat, both gated on `autoCheckForUpdates`. The
> /// setting is re-read every tick so toggling it in Settings takes effect"

and `:154-157`:

> "                .auto_check_for_updates;
>             if !enabled {
>                 continue;
>             }"

There is a second, unstated gate at `lib.rs:141-144`: `"if cfg!(debug_assertions) { ... return; }"`.

Both FR-20 consequences are stated unconditionally and both are false when a shipping, user-facing preference is off. `RP-1` (`prd.md:497`) repeats the unconditional form — `"Launch, six-hour, and app-menu update checks are preserved"` — while also referring to `"the saved trigger policy"`, so RP-1 half-knows about the setting and FR-20 does not.

This is exactly the "reads as testable but is not" class: an acceptance test written from FR-20 passes or fails depending on a setting FR-20 never mentions. And "required behavior, not an optional outcome" is a direct denial of a shipped user preference the same document enumerates two pages earlier.

**Fix:** qualify FR-20 — *"When `autoCheckForUpdates` is enabled (the default), checks run at launch and every six hours; manual checks from the application menu run regardless. When enabled, download is automatic and not an optional outcome."* Delete or requalify the "required behavior" sentence.

### H3. FR-19's focus requirement is universally quantified; its cited guard covers 2 of 31 sites, and only under keyboard focus

`prd.md:425`:

> "Every interactive element carries a visible focus indicator, drawn as a real `outline` — never a `ring-*` box-shadow, which WebKit does not paint on native-appearance form controls, and never `outline-none`."

`prd.md:434` offers the guarantee:

> "It ships across 31 sites, is asserted in CI by `tests/e2e/browser-style-contract.spec.ts`, and is governed by D35 and AD-27. Removing the rule would delete no work and would only un-guard working code against the next `ring-*` — the exact trap D35 documents."

The 31 is right. The guard is not what the sentence implies. At `HEAD` the entire spec file (225 lines) calls `.focus()` exactly twice — line 79 (`Refresh All` button) and line 194 (one package-row checkbox). Two of thirty-one. A new `ring-*` on `StatusBar.tsx`, `ToastHost.tsx`, `Sidebar.tsx`, `Chip.tsx`, `CopyableCommand.tsx`, `HistoryView.tsx` or `SettingsView.tsx` ships fully green. The stated reason for keeping the rule — un-guarding working code against the next `ring-*` — is therefore true for 2 sites and false for 29.

There is a second-order problem worth naming precisely, without reopening anything D37 closed. The guard fires only under `:focus-visible`, and the spec says so itself at `HEAD:tests/e2e/browser-style-contract.spec.ts:192`:

> "// Deliberately keyboard-driven: a pointer click sets :focus but not"

`addendum.md:67` describes the user as `"One user, one machine, mouse-operated."` So the retained requirement's only verification path is one a mouse-only user never traverses. That is not an argument to reinstate anything — it is an argument that FR-19's justification ("not an accessibility obligation") does not survive contact with the file it cites, and that a future reader clearing D37 leftovers has a plausible reason to delete the test, at which point FR-19 keeps a universal requirement with zero coverage.

**Fix:** either narrow the claim (*"asserted in CI on the Button and Checkbox primitives; the remaining sites are covered by the AD-27 mechanism rule, not by a test"*) or add a cheap population check (grep gate for `ring-focus-ring` / `ring-offset-` / `outline-none` in `src/`, which the spine at `:1050`-area already used as its evidence). Keep the note that the guard is keyboard-driven by construction, so nobody deletes it as D37 residue.

### H4. §9's Open Question 1 declares open a question FR-21 answers normatively

`prd.md:629`:

> "**What happens on quit with work *queued* but not running?** The running-Operation quit guard is defined. Queued-only work, **application-update installation during Package activity**, and OS-initiated shutdown are not."

`prd.md:471`, FR-21, Status **Shipping**:

> "Installation and relaunch are **refused** while any Package Operation is queued or running. Queued counts as active — admission has already committed to the work, and a restart would drop it unstarted."

FR-21 defines application-update installation during Package activity, including the queued case, and `ARCHITECTURE-SPINE.md` records the enforcement point as closed by commit `7cc7b5f` with the Rust and frontend predicates verified identical. §9 says it is undefined and routes it to "the epic that touches it" — `sprint-status.yaml:98` `ux-pb-5e-application-update-presentation-kept-separate-from-package-plans-and-history`. A story owner resolving Q1 in good faith can re-decide a shipping, two-layer-enforced, already-audited behavior.

`prd.md:635` then asserts `"All five are non-blocking"` on the strength of a question one-third of which is already answered by the same document.

**Fix:** strike the application-update clause from Q1, leaving only *"quit with work queued but not running"* and *"OS-initiated shutdown"*, and cross-reference FR-21 for the resolved third.

---

## Medium

### M5. "10, 3, and 4 mentions respectively" counts VoiceOver only, and is used to size three workflow runs

`prd.md:440`:

> "`epics.md`, `ARCHITECTURE-SPINE.md`, and `EXPERIENCE.md` still carry the removed obligations (10, 3, and 4 mentions respectively)."

Measured (`grep -o -i <term> <file> | wc -l`):

| File | `voiceover` | `keyboard` | lines matching either |
| --- | --- | --- | --- |
| `epics.md` | **10** | 11 | 13 |
| `ARCHITECTURE-SPINE.md` | **3** | 6 | 8 |
| `EXPERIENCE.md` | **4** | 9 | 11 |

The stated figure is the VoiceOver column in all three cases. The label says "keyboard/VoiceOver". Keyboard mentions are roughly double and are not counted anywhere.

This matters because the numbers are the acceptance criterion for three downstream runs. It matters more because some keyboard mentions must *survive*: `ARCHITECTURE-SPINE.md:899` is `"### AD-27 — [ADOPTED] Keyboard focus uses one mechanism, and it is the one WKWebView paints"`, which `prd.md:434` explicitly preserves, and `:1018` is the AD-27 traceability row. A run told "remove 3 keyboard/VoiceOver mentions from the spine" has 8 candidate lines and no rule for which 3 — and one plausible pick is the AD-27 heading.

The figures are inherited verbatim from `docs/DECISIONS.md:566-567`. That is a legitimate provenance, but `prd.md:36` claims this PRD re-verified against `HEAD` — `"A 2026-07-25 validation, re-verified against \`HEAD\` for this PRD"` — and `prd.md:654` claims every requirement traces to a verified source. A count carried forward unchecked is the one class of claim this project's own conventions require a command for.

**Fix:** replace the bare counts with per-file line lists, and mark the keep-list explicitly (`AD-27` heading, `AD-27` traceability row, RP-2's Cmd map, `EXPERIENCE.md:143`/`:326` never-native-`disabled` rule).

### M6. FR-3 asserts Homebrew's self-update Route unconditionally; FR-17 ships the switch that disables it

`prd.md:174` (FR-3, Status **Shipping**):

> "Homebrew's metadata refresh doubles as Homebrew's self-update Route."

`prd.md:394` lists `"run Homebrew metadata update during refresh (on)"` as a user-settable shipping preference. `src-tauri/src/managers/brew.rs:86`:

> "        if settings.run_brew_update_on_refresh {"

— the `brew update` command is only planned when that setting is true. But `brew.rs:288` returns the route unconditionally:

> "        SelfUpdateRoute::ViaRefresh {
>             note: \"brew update runs as part of every refresh\".into(),
>         }"

Turn the setting off and Homebrew's declared self-update route becomes a no-op while FR-11 (`prd.md:298-300`) still requires the identity area to explain "how it updates" and FR-4 (`prd.md:186`) only disables an action `"When the required executor is absent"` — the executor is present, the command simply is not planned. The user sees an accurate-looking route that does nothing.

This is a real interaction the PRD does not acknowledge in either FR. It is also the only self-update route in the product with a user-facing kill switch.

**Fix:** qualify FR-3's bullet (*"…while `runBrewUpdateOnRefresh` is enabled; with it off, Homebrew has no in-app self-update route and the Manager identity area says so"*) or add it to §9 as a sixth open question.

### M7. FR-22's last bullet makes a hardware-testing scope note read as a cryptographic-verification carve-out

`prd.md:484-485`:

> "- Only updater payloads cryptographically authorized for the installed application are accepted.
> - Verification is Apple-silicon only; Intel remains best-effort and unverified (D32)."

Read in sequence, the second bullet reads as: signature verification happens on Apple silicon; Intel accepts payloads unverified. That contradicts the bullet directly above it, contradicts NFR-1 "Fail closed" (`prd.md:513`) and NFR-8 (`prd.md:557`).

D32 means something entirely different. `docs/DECISIONS.md:301-303`:

> "The promise is narrowed to match what is actually verified: the release
> builds universal; verification is Apple silicon only. Intel remains
> best-effort and unverified."

and `:294-295`: `"What is dropped is the obligation to physically verify on Intel hardware."` It is about *release testing on hardware the maintainer does not own*, not about signature checking. Placing it as the closing bullet of a security requirement, immediately after the cryptographic-authorization bullet, with no qualifier, is the worst possible position for it.

**Fix:** `"Release testing is performed on Apple silicon only; the Intel slice is built and signed identically but is not physically verified (D32). Cryptographic authorization is architecture-independent."`

### M8. §3 asserts a vocabulary discipline the document does not keep, and four load-bearing D29/D30 nouns are undefined

`prd.md:100`:

> "Downstream workflows must use these terms exactly. No synonyms appear anywhere else in this document."

The Glossary defines 20 terms. Counted occurrences elsewhere in `prd.md`: `Activity` 9, `Results` 8, `Retry` 5, `sidecar` 4, `Environment Report` 2 — none of them defined. `Activity`, `Results`, and `Retry` are capitalized proper nouns carrying D29/D30 semantics across FR-13, FR-15, FR-19, NFR-3, and §7.2; five `epic-ux-pb` stories are named after them (`ux-pb-3b`, `ux-pb-3d`, `ux-pb-4b`, `ux-pb-4d`, `sprint-status.yaml:88-99`). `sidecar` is a bare synonym for the Upgrade Plan surface — `prd.md:249` `"the sidecar is hidden when empty"` — which falsifies the "no synonyms" sentence outright.

A PRD whose stated job (`prd.md:18`) is `"Vocabulary is Glossary-anchored (§3) and used verbatim throughout"` cannot leave the three nouns that name a fifth of the build queue undefined.

**Fix:** add `Activity`, `Results`, `Retry`, and `Upgrade Plan sidecar` to §3; or soften `prd.md:100` to "Glossary terms are used verbatim; no synonym for a Glossary term appears elsewhere" and then honor it by replacing `sidecar` with the glossed term.

---

## Low

### L9. `epics.md` "lines 53–450" is off by four and truncates RP-1 and RP-2

The range appears three times — `prd.md:12`, `prd.md:654`, `addendum.md:52`. Measured: the FR/NFR block starts at `epics.md:53` (`"FR-1: Detect Homebrew, mise, npm, uv, rustup, and \`mas\`…"`) and the last entry is at `epics.md:454` (`"RP-2: Release checklist — Validate standard macOS Edit/Window menu behavior."`), with `RP-1` at `:452` and `FR-22` at `:450`.

So the cited range ends exactly at `FR-22` and excludes `RP-1` and `RP-2` — the two requirements `prd.md` §4.6 exists to carry forward. A `bmad-correct-course` run told to reconcile "lines 53–450" leaves both untouched.

This is worth one line rather than five because `ARCHITECTURE-SPINE.md` already diagnosed the pattern in the row the PRD reads: `"the same positional-reference failure this run folder has now hit three times (rule ordinals, \`epics.md\` line numbers, and now spine line numbers). The next correct-course run should repoint them to \`AD\` ids and row titles, never line numbers."` The PRD read that row, cited it, and then built its entire reconciliation queue on eight more line-number references.

**Fix:** cite by requirement id — `"epics.md FR-1 through RP-2"` — everywhere the range appears.

### L10. §7.2 puts quotation marks around text D33 does not contain

`prd.md:595`: `"…which D33 called \"the real product stories\"."`
`docs/DECISIONS.md:324`: `"stories split into 28 real product stories (the D27–D30 Upgrade Plan"`

The substance is right; the quoted string is not verbatim. Trivial in isolation, non-trivial in a document whose authority rests on quote fidelity and which is the only Phase 2 input three workflows will trust without rechecking.

**Fix:** `"…which D33 describes as \"28 real product stories\"."`

---

## Bottom line

The numbers are clean and the sourcing discipline is real — I could not find a fabricated constant anywhere. The failures are all of one shape: **the PRD is precise about the past and loose about the seams.** Every finding above sits where two things the document already knows meet — FR-5 vs `EXPERIENCE.md:143`, FR-20 vs FR-17, §9 Q1 vs FR-21, FR-3 vs FR-17, the reconciliation queue vs the file that holds the build queue. Nothing here requires new information; it requires the document to read itself.

The worst of them, FR-5's `disabled` checkbox, is worst precisely because it looks like carefulness. It is the sentence that survived a scope decision by dropping the wrong half.
