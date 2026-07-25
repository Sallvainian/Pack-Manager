# Review — retired-scope residue

**Target:** `_bmad-output/planning-artifacts/prds/prd-Pack-Manager-2026-07-25/prd.md` + `addendum.md`
**Lens:** anything the PRD or addendum reintroduces — directly or by ambiguous restatement — that `docs/DECISIONS.md` D33 or D37 retired.
**Date:** 2026-07-25 · **HEAD:** `5972109`

---

## Verdict

The PRD's *prose* is clean. Nothing in it asks for a coverage percentage, a criterion count, a versioned scenario contract, an evidence manifest, a candidate freeze, a multi-host environment, a `contracts/` directory, keyboard navigation, VoiceOver operability, live-region announcements, or focus restoration. `prd.md:30` states the exclusion explicitly and `prd.md:430` states the D37 removal explicitly. The negative space I checked is recorded in §"Checked and clean" below.

The residue is not in what the PRD *says*. It is in what the PRD **points at** and how it **sizes the cleanup**. Six findings, all in that class:

- The PRD elevates one hand-written file to release-readiness authority; at `HEAD` that file still carries both D37-removed criteria verbatim, and it appears in no reconciliation list.
- The reconciliation queue is sized by a number that counts VoiceOver only while being labeled "keyboard/VoiceOver". Every keyboard obligation in all three downstream artifacts falls outside the scope the addendum hands to the workflows.
- The PRD names a fourth artifact — `sprint-status.yaml` — as "the live build queue" without noticing that two of its 28 story keys still encode D37-removed scope, and routes it nowhere.

The failure mode this lens exists to catch is retired apparatus returning through artifact globbing. It did not return through globbing this time. It is positioned to return through **under-scoped reconciliation instructions**, which is the same outcome by a different route.

---

## Findings

### 1. `docs/RELEASE-CHECKLIST.md` is made the release-readiness authority while it still carries both D37-removed criteria, and it is in no reconciliation list — HIGH

`prd.md:30`:

> Release readiness is `docs/RELEASE-CHECKLIST.md` plus the two release-blocking checks in `release.yml`.

At `HEAD` (`git show HEAD:docs/RELEASE-CHECKLIST.md`), that file reads:

- line 86: `9. **Keyboard and accessibility pass.** Tab and arrow navigation reach every control.`
- line 89: `   One VoiceOver pass over the Upgrade Plan announces state changes and completion.`

Those are, word for word, the two things D37 removed. `docs/DECISIONS.md:527`:

> Removed as release criteria: the `docs/RELEASE-CHECKLIST.md` step requiring

`docs/DECISIONS.md:528`:

> "Tab and arrow navigation reach every control", and the manual VoiceOver pass

`docs/DECISIONS.md:529`:

> over the Upgrade Plan. Both were roughly 3–5 minutes of every release, verifying

So the PRD's single named release-readiness authority is the one artifact D37 named **first**, and it is unfixed at `HEAD`.

The bookkeeping compounds it. `prd.md:440` enumerates where the removed obligations survive:

> `epics.md`, `ARCHITECTURE-SPINE.md`, and `EXPERIENCE.md` still carry the removed obligations (10, 3, and 4 mentions respectively).

`addendum.md:48` repeats the same three:

> This PRD is now the requirements authority. Three downstream artifacts contain statements it supersedes. All three are workflow-owned — **none of them may be hand-edited.**

`docs/RELEASE-CHECKLIST.md` is in neither list. The addendum has the right structural slot for it — `addendum.md:56` handles the other hand-written, workflow-unowned file:

> `docs/SPEC.md` is hand-written and owned by no workflow. Its defects are recorded in `prd.md` §0.1 rather than fixed here, because editing it is a maintainer decision about a file the BMAD chain does not own.

`RELEASE-CHECKLIST.md` gets no equivalent paragraph, and unlike `SPEC.md` it is not demoted — `prd.md:30` and `prd.md:491` ("They are validated through `docs/RELEASE-CHECKLIST.md`") make it normative.

**Note on working-tree state, so this is not read as a duplicate of the known contrast-guard finding.** `git diff docs/RELEASE-CHECKLIST.md` shows the a11y block is being rewritten in the working tree (lines 102–104 of the modified file now read `**Keyboard navigation and screen-reader support are explicitly not release criteria.**`). That work is uncommitted and absent from `HEAD` — the same *class* of problem as the §7.1 contrast-guard claim, but a different location and a different defect: the contrast-guard issue is an over-claim about shipping state, this one is a normative delegation plus a missing reconciliation entry. Even after the working-tree edit lands, `prd.md:440` and `addendum.md:48` remain wrong about where the residue lived.

**Fix:** add a fourth row to `addendum.md` §3, or an explicit paragraph beside the `docs/SPEC.md` one, recording that `docs/RELEASE-CHECKLIST.md` §9 carries the two D37-removed steps at `HEAD` and that removing them is a maintainer edit (the file is workflow-unowned). Amend `prd.md:440` to say four artifacts, not three.

---

### 2. "10, 3, and 4 mentions" are VoiceOver-only counts labeled as keyboard/VoiceOver counts — every keyboard obligation falls outside the reconciliation scope — HIGH

`prd.md:440`:

> `epics.md`, `ARCHITECTURE-SPINE.md`, and `EXPERIENCE.md` still carry the removed obligations (10, 3, and 4 mentions respectively).

`addendum.md:52`:

> FR-19 (line 89) and NFR-6 (line 113) still carry the D37-removed keyboard/VoiceOver and announcement obligations; 10 mentions total.

`addendum.md:53`:

> `ARCHITECTURE-SPINE.md` (rev 9) | 3 keyboard/VoiceOver mentions, including the manual pass referenced at lines 332 and 941.

`addendum.md:54`:

> `EXPERIENCE.md` | 4 keyboard/VoiceOver mentions, plus the `Accessibility Floor` section (lines 313–332)

Measured. Command: `grep -c -i voiceover <file>` and `grep -c -i keyboard <file>`:

| Artifact | lines w/ VoiceOver | lines w/ keyboard | lines w/ keyboard and **no** VoiceOver |
| --- | --- | --- | --- |
| `epics.md` | **10** | 11 | 3 |
| `ARCHITECTURE-SPINE.md` | **3** | 6 | 5 |
| `EXPERIENCE.md` | **4** | 9 | 7 |

10, 3, 4 are the VoiceOver column exactly. The label "keyboard/VoiceOver mentions" is false: 15 additional lines carry keyboard obligations with no VoiceOver token on them, so a `bmad-correct-course` / `bmad-architecture` / `bmad-ux` run scoped to the stated numbers strips the screen-reader limbs and leaves keyboard navigation fully intact.

The PRD inherited the number rather than inventing it — `docs/DECISIONS.md:556` says `**Not yet applied:** `epics.md` (10 mentions, including FR-19, NFR-6, and Story` — but D37 says "mentions" without saying of what. The addendum **sharpened an ambiguous number into a specific false claim**, which is worse than repeating it, because a downstream run can now believe it is complete at 10.

The largest omission is `EXPERIENCE.md`, where the addendum scopes the `bmad-ux` Update to "4 keyboard/VoiceOver mentions, plus the `Accessibility Floor` section (lines 313–332)". Outside that scope sit three entire sections:

- `EXPERIENCE.md:272` `## Keyboard`
- `EXPERIENCE.md:281` `## Package Grid keyboard model`
- `EXPERIENCE.md:290` `## Focus transitions`

Sample obligations left standing:

`EXPERIENCE.md:274`:

> - Tab order follows visual reading order: sidebar → page header/actions → filters → table/Manager cards → Upgrade Sidecar.

`EXPERIENCE.md:277`:

> - F6 cycles the primary navigation, main Package Grid/workspace, and Upgrade Sidecar regions without changing selection.

`EXPERIENCE.md:283`:

> - The grid exposes persistent column headers, one roving row Tab stop, total filtered row count, and each virtual row’s stable position.

`EXPERIENCE.md:284`:

> - Up/Down moves one Package; Page Up/Down moves one rendered viewport; Home/End reaches the first/final Package and scrolls it into view.

`EXPERIENCE.md:294`:

> | Open Confirmation Dialog              | Move focus to the programmatically focusable dialog heading/command summary. `Change Plan` is the first actionable control; final confirmation is never the accidental default for an unfocused Enter press. |

That last table is *precisely* the item `prd.md:430` says was dropped:

> Specifically dropped: keyboard operability of primary actions, VoiceOver operability, live-region announcements of plan progress/verification/cancellation/failure/completion, and NFR-6's deterministic dialog/sidecar focus restoration.

and `prd.md:659` re-confirms it stays dropped. Yet the addendum's `bmad-ux` scope does not reach the section that defines it. Worse, `EXPERIENCE.md:327` inside the in-scope Accessibility Floor reads `- Opening/closing dialogs and transformations follow the Focus transitions matrix; Results receives one accessible summary announcement.` — so deleting the in-scope bullet removes the *pointer* to the matrix while leaving the matrix itself live and unreferenced. That is the ambiguity a downstream run rebuilds from.

**Fix:** replace the three counts with the measured keyboard+VoiceOver totals (21 / 9 / 13 occurrences; 13 / 8 / 11 lines) or drop the counts entirely and enumerate the sections by heading. For `EXPERIENCE.md`, name `## Keyboard` (272), `## Package Grid keyboard model` (281), and `## Focus transitions` (290) explicitly in the `bmad-ux` row.

---

### 3. `sprint-status.yaml` is declared "the live build queue" but is routed nowhere, and two of its 28 story keys still encode D37-removed scope — HIGH

`prd.md:595`:

> This is the live build queue — 28 stories under `epic-ux-pb` in `_bmad-output/implementation-artifacts/sprint-status.yaml`, which D33 called "the real product stories".

The count is right — `sprint-status.yaml` lines 72–99 are 28 story keys. Two of them:

`_bmad-output/implementation-artifacts/sprint-status.yaml:75`:

> `  ux-pb-1d-ineligible-control-inertness-with-keyboard-pointer-and-voiceover-explanation: backlog`

`_bmad-output/implementation-artifacts/sprint-status.yaml:98`:

> `  ux-pb-5d-accessibility-and-responsiveness-of-the-confirmation-and-safety-surfaces: backlog`

and in the surviving epic-3 block:

`_bmad-output/implementation-artifacts/sprint-status.yaml:112`:

> `  3-5-preserve-exact-keyboard-selection-and-row-plan-actions: backlog`

The file was regenerated at `last_updated: 2026-07-25T00:04:01` and carries a detailed D33 rescope note (lines 48–60) — but no D37 note anywhere. `grep -i d37 sprint-status.yaml` returns nothing.

The binding text behind those keys is in `epics.md`, which the addendum *does* route — but the addendum narrowed D37's own instruction. `docs/DECISIONS.md:556`:

> **Not yet applied:** `epics.md` (10 mentions, including FR-19, NFR-6, and Story

`docs/DECISIONS.md:557`:

> UX-PB.1d), `ARCHITECTURE-SPINE.md` (3), and `EXPERIENCE.md` (4) still carry the

D37 names three sites in `epics.md`: FR-19, NFR-6, **and Story UX-PB.1d**. `addendum.md:52` names only two — FR-19 and NFR-6. UX-PB.1d was dropped from the handoff. What that leaves live:

`epics.md:596`:

> ### Story UX-PB.1d: Ineligible-control inertness with keyboard, pointer, and VoiceOver explanation

`epics.md:612`:

> **When** a keyboard or VoiceOver user reaches it

`epics.md:613`:

> **Then** it uses `aria-disabled="true"` rather than native `disabled`, keeps focus, announces its persistent reason as an accessible description, stays inert on activation, and retains focus when Escape closes its supplemental Tooltip/Popover.

And UX-PB.5d, which D37 does not name and the addendum does not either, is a *wholly* removed-scope story:

`epics.md:1106`:

> ### Story UX-PB.5d: Accessibility and responsiveness of the confirmation and safety surfaces

`epics.md:1111`:

> As a keyboard and VoiceOver user at high zoom, I want every safety action reachable and announced so that the confirmation gate protects everyone at the 900 x 600 minimum and at 150-200% zoom.

Its persona *is* the user D37 says the project does not have. `prd.md:79` states the position:

> - **Assistive-technology users.** Per D37 this is a single-user utility operated with a mouse, and keyboard navigation and screen-reader support are explicitly not release criteria.

D37 protects UX-PB.1d from deletion but not from narrowing — `docs/DECISIONS.md:560`: `**Story UX-PB.1d is not to be deleted**: its` / `:561` `pointer-hover explanation of why a Package is ineligible is mouse-facing` / `:562` `behavior, and only its keyboard and VoiceOver limbs are in scope here.` It says nothing about UX-PB.5d, whose entire subject is the removed scope.

**Fix:** add `sprint-status.yaml` as a fourth row in `addendum.md` §3. Restore D37's naming of Story UX-PB.1d in the `epics.md` row, and add UX-PB.5d and Story 3.5 as sites the correct-course run must decide on — narrow, retire, or keep with a stated reason.

---

### 4. The addendum's `EXPERIENCE.md` scope preserves DR-2's packaged-evidence framing, which D33 retired — MEDIUM

`addendum.md:54` scopes the `bmad-ux` Update to:

> plus the `Accessibility Floor` section (lines 313–332) whose announcement and 101-row VoiceOver-reachability obligations D37 removes.

Line range verified: `EXPERIENCE.md:313` is `# Accessibility Floor`, and the section's last bullet is line 332. But the two lines that carry the *gate* framing are not "announcement" or "101-row reachability" obligations, so the stated scope does not reach them:

`EXPERIENCE.md:315`:

> Pack-Manager must meet the packaged-app accessibility method approved by the Architecture Spine.

`EXPERIENCE.md:330`:

> - Packaged acceptance verifies focus, final-row reachability, selection scope, completion announcements, and no overlap at 100%, 150%, and 200% zoom with VoiceOver and 101 Package rows.

"Packaged acceptance verifies …" is an evidence-production obligation with a named verification surface. That is DR-2's gate framing. D33 retired exactly the framing while keeping the substance — `docs/DECISIONS.md:339`:

> DR-2's substance survives without its gate framing. Automated 4.5:1 contrast

A `bmad-ux` Update that deletes only what the addendum names removes the announcement bullets and leaves a section whose opening sentence still asserts a mandatory "packaged-app accessibility method approved by the Architecture Spine" with a "Packaged acceptance verifies" clause underneath it. That reads as a live acceptance gate to the next reader.

**Fix:** name lines 315 and 330 in the `EXPERIENCE.md` row, and say the retirement is of the *packaged-acceptance framing* (D33/DR-2), distinct from the announcement removal (D37). The contrast, focus-indicator, non-color-cue, hit-target, and zoom bullets in that section survive and should be named as surviving so the run does not over-cut.

---

### 5. NFR-7 keeps candidate-acceptance vocabulary — MEDIUM

`prd.md:551`:

> **Minimum supported macOS is 15.0**, declared in `src-tauri/tauri.conf.json` — D31 closed this; it is no longer an open prerequisite for candidate acceptance.

"Candidate acceptance" is the retired process. `docs/DECISIONS.md:331`:

> Retired: the 72-criterion P0 gate, all P0/P1/overall coverage percentages,

`docs/DECISIONS.md:332`:

> the 55 scenario contracts, the evidence-manifest and candidate-freeze

`docs/DECISIONS.md:333`:

> machinery, the multi-host environment requirements, and Epics 7–8. They are

D31 itself is careful about tense — `docs/DECISIONS.md:250`: `` `DR-1` ("Minimum supported macOS version — OPEN, implementation-entry`` / `:251` `blocker"), which lived in the retired PRD gate-governance register.` — past tense, "lived in the retired … register."

`prd.md:551` does not carry that tense. "no longer an open prerequisite for candidate acceptance" presupposes a candidate-acceptance process that still exists and still has a prerequisite list; the only thing asserted is that *this item* left it. This is the ambiguous-restatement failure the lens targets: a reader or a workflow can reasonably conclude a candidate-acceptance register is live and go looking for the other prerequisites. It is also the only sentence in the PRD that uses gate vocabulary in the affirmative — `prd.md:30` and `prd.md:430` both use it to negate.

**Fix:** `— D31 closed this; the gate-governance register it was an open item in was retired with D33.` One clause, no live process implied.

---

### 6. The addendum blesses `EXPERIENCE.md:143` as surviving verbatim while it carries announcement and focus-preservation obligations — MEDIUM

`addendum.md:54`:

> Its membership model (line 143) is **correct and survives** — only the accessibility limbs change.

`EXPERIENCE.md:143`:

> | **Checkbox**                  | On eligible Package rows, selection immediately adds/removes Upgrade Plan membership. The header Checkbox applies to every eligible Package identity matching the active filter, including off-screen virtualized rows; it announces the exact count and uses `mixed` when only some are staged. An explanatory-disabled control never uses native `disabled`: expose `aria-disabled="true"`, attach its persistent reason as an accessible description, keep activation inert, and preserve focus for its supplemental Tooltip/Popover.

That single row contains an announcement obligation ("it announces the exact count"), an accessible-description obligation, and a focus-preservation obligation — the three classes D37 removes. The addendum calls the whole line correct and surviving, and separately says "only the accessibility limbs change" without saying that *this line is itself partly accessibility limbs*.

The PRD's own FR-6 already made the call, and made it differently. `prd.md:226`:

> - The header checkbox adds or removes every eligible Package matching the active filter, including off-screen virtualized rows, and reports the exact count it will affect. It shows a mixed state when only some are staged.

"reports" replaced "announces" and the ARIA/focus clauses are gone. So `prd.md:226` and `addendum.md:54` give the `bmad-ux` run opposite instructions about the same line: the PRD de-announced it, the addendum says it survives.

This one is genuinely close to the D37 boundary — `docs/DECISIONS.md:564` explicitly protects shipped ARIA: `**Rejected:** deleting the shipped focus and ARIA affordances from `src/`. They` — so the *code* keeps `aria-disabled`. The question the addendum leaves open is whether the *spec line* keeps it as an obligation. It should answer that rather than blessing the line wholesale.

**Fix:** change `addendum.md:54` to name what survives at line 143 — the direct-membership model and the filter-wide header scope — and state that the announcement, accessible-description, and focus-preservation clauses on that same line are D37-removed, matching `prd.md:226`. Add a note that shipped `aria-disabled` in `src/` is untouched per D37's rejection clause, so the ux run does not read the spec change as licence to strip code.

---

### 7. `prd.md:595` presents a phrase as a D33 quotation that D33 does not contain — LOW

`prd.md:595`:

> This is the live build queue — 28 stories under `epic-ux-pb` in `_bmad-output/implementation-artifacts/sprint-status.yaml`, which D33 called "the real product stories".

`grep -c "the real product stories" docs/DECISIONS.md` → `0`.

The nearest text, `docs/DECISIONS.md:324`:

> stories split into 28 real product stories (the D27–D30 Upgrade Plan

D33 writes "28 real product stories" as a count-and-category, not a name. Quoting it as "the real product stories" turns a descriptive phrase into a definite label. Low impact on its own — the substance is right and the count of 28 checks out against `sprint-status.yaml:72–99` — but it is a quotation mark around text that does not exist, in the exact sentence that hands the build queue to implementation, and it sits next to finding 3's real problem.

**Fix:** drop the quotation marks, or quote D33 accurately: `which D33 counted as the 28 "real product stories"`.

---

## Checked and clean

Recorded so the negative space is not re-searched. Command: `grep -n -i -E "coverage|gate|evidence manifest|scenario contract|candidate|freeze|multi-host|keyboard|voiceover|screen[- ]reader|announce|live[- ]region|focus restoration|P0|P1|readiness|manifest|contracts/" prd.md addendum.md`.

- **No coverage percentage, criterion count, scenario contract, evidence manifest, candidate freeze, multi-host requirement, or `contracts/` directory is requested anywhere.** `prd.md:30` names all of them as excluded and `addendum.md:69` closes with "That is the apparatus D33 retired and that this PRD does not reintroduce."
- **"gate" at `prd.md:214`** ("the separate confirmation gate") **and `prd.md:622`** ("the confirmation gate") is the D28 product feature, not a readiness gate. Correct usage; not residue.
- **"P0"/"P1" at `prd.md:46`, `prd.md:599`, `prd.md:612`** is `docs/SPEC.md`'s feature-priority scheme, defined at `docs/SPEC.md:52`: "Priorities: **P0** = MVP-required. **P1** = ship-with polish, after all P0 tests pass. **P2** = out of MVP scope." Not the retired P0 gate. Not residue.
- **`prd.md:30` naming `readiness-coverage-map.md`** among D33's retirements: D33's retirement sentence (`:331–334`) does not list that filename, but `docs/DECISIONS.md:353` covers it — "Retired planning artifacts are archived under `_bmad-output/archive/2026-07-24-scope-recalibration/`, not deleted" — and the file is in fact at `_bmad-output/archive/2026-07-24-scope-recalibration/planning/prds/prd-Pack-Manager-2026-07-22/readiness-coverage-map.md`. Defensible; not a finding.
- **No archived retired artifact was copied into `planning-artifacts/`.** `find _bmad-output/planning-artifacts -iname "*prd*"` returns only `prds/prd-Pack-Manager-2026-07-25/prd.md` and `reconcile-archived-prd.md`. The latter matches the `*prd*.md` glob `prd.md:30` warns about, but it disclaims the apparatus at its own line 20 — "No finding concerns a P0 criterion count, a coverage percentage, a versioned scenario contract, an evidence manifest, a candidate freeze, a multi-host environment, or a `contracts/` directory (D33)." Clean.
- **`epics.md` lines 344–370's Epic 7/8 references are fenced**, not live: `epics.md:348` reads "**This table is a historical revision record, not a live instruction.**" Not residue.
- **The three D37 survivors are present and correctly framed as non-accessibility**: focus indicator `prd.md:425` and `prd.md:434`; the Cmd-key map at RP-2 `prd.md:503`; the 4.5:1 floor `prd.md:423` and `prd.md:543`. No re-derivation of the removed items around them.
- **`ARCHITECTURE-SPINE.md`'s manual-pass dependency is correctly routed.** `addendum.md:53` names lines 332 and 941; `ARCHITECTURE-SPINE.md:941` reads "the manual VoiceOver-and-keyboard pass on `docs/RELEASE-CHECKLIST.md` until a" — a live rule that makes the removed pass the substitute for real-WKWebView verification. Routing is right; only the count (finding 2) is wrong.
