# Focused UX Validation Report — Pack-Manager

- **DESIGN.md:** `_bmad-output/planning-artifacts/ux-designs/ux-Pack-Manager-2026-07-23/DESIGN.md`
- **EXPERIENCE.md:** `_bmad-output/planning-artifacts/ux-designs/ux-Pack-Manager-2026-07-23/EXPERIENCE.md`
- **Run at:** 2026-07-24T02:30:58Z
- **Selected lenses:** Usability and accessibility
- **Resolution status:** All findings applied to the final spines; mock coverage confirmed; four authoritative key-screen mockups validated and promoted on 2026-07-24.

## Overall verdict

The contracts now provide a coherent implementation foundation: the Upgrade Plan lifecycle is understandable, terminal evidence remains secondary but available, failures are handled honestly, and accessibility is treated as a product requirement.

No critical blocker was found. The reviewers reported **9 high**, **7 medium**, and **2 low** findings in total; one high finding overlaps across both lenses around virtualized Package selection. Those findings are resolved in the final spine pair, and the affected interactions are represented in the authoritative key-screen mockups.

The broader rubric/adversarial review was not run because the user selected focused usability and accessibility validation.

> This report preserves the original review findings. `DESIGN.md` and `EXPERIENCE.md` now contain their approved resolutions.

## Original lens verdicts

- **Usability — needs clarification.** Coherent mental model, but five high-impact workflow decisions need to be made explicit.
- **Accessibility — adequate foundation.** Strong direction, but four high-impact implementation contracts are incomplete.

## Findings by severity

### Critical (0)

No critical findings.

### High (9)

#### Usability — Plan items cannot be edited directly in the sidecar

After Update Everything, no direct method is defined for removing an individual Package or Manager self-update. `Change Plan` returns to an information-only plan, and `In Plan` has no defined removal behavior.

**Location:** `EXPERIENCE.md` Information Architecture, Upgrade Sidecar, Manager Header, Confirmation, and AJ-2.

**Fix:** Make every staged Package and Manager self-update removable from the Upgrade Sidecar. Define `In Plan` as a toggle or pair it with `Remove from plan`, update counts immediately, and return focus to this editable plan from `Change Plan`.

#### Usability — Header Checkbox scope is ambiguous

“Eligible visible rows” could mean only Package rows currently rendered on screen or the complete result set under the active filter.

**Location:** `EXPERIENCE.md` Checkbox, Package Filter, and AJ-3; `DESIGN.md` virtualized Package-table guidance.

**Fix:** Define it as every eligible Package matching the active filter, including off-screen virtualized rows. Use a mixed state for partial membership, label the exact consequence such as `Add all 8 updates`, and announce the resulting count.

#### Usability — Confirmation-disabled action has an unclear consequence

If the dialog is disabled, `Confirm # Updates` implies another confirmation even though activation executes immediately.

**Location:** `EXPERIENCE.md` Draft Upgrade Plan, Confirmation, Settings, and AJ-2.

**Fix:** Show a persistent `Confirmation is off` safety message with a Settings link, expand commands before activation, and label the immediate action `Run # updates`. Restore `Confirm # updates` when the dialog is enabled.

#### Usability — Abnormal Activity controls and interactive-command handling are undefined

Cancel, skip, stall, timeout, and waiting states exist, but the controls and consequences that produce them are not specified. Unexpected command prompts have no visible contract.

**Location:** `EXPERIENCE.md` Live Activity and AJ-4.

**Fix:** Define exactly when `Keep waiting`, `Cancel plan`, and any approved `Skip` action appear and what each affects. State that plans run non-interactively; an unexpected prompt becomes an explicit blocked `Interaction required` state with only safe actions.

#### Usability — Retry conflicts with one-entry History semantics

The lifecycle text could imply that retries stay inside the original History entry, while Results correctly says Retry creates a new plan.

**Location:** `EXPERIENCE.md` Lifecycle model, Results and recovery, and AJ-4.

**Fix:** State that each confirmed execution attempt creates a new History entry. Link a retry to its source, for example `Retry of plan from 10:42 AM`, without changing the original failed result.

#### Accessibility — Focus boundary token is too weak

`borderStrong` is described as a focused boundary but measures only 2.33:1 against `surface`; no separate focus-ring token, width, or offset is defined.

**Location:** `DESIGN.md` Colors and Components; `EXPERIENCE.md` Accessibility Floor.

**Fix:** Use `accent` or a dedicated high-contrast focus token, require at least a 2px separated ring, and keep selected and focused states visually distinct.

#### Accessibility — Dense virtualized Package-table operation is incomplete

The contract does not define table/grid semantics, bounded Tab behavior, arrow navigation, mixed-header announcements, stable focus across virtual-row recycling, final-row reachability, or accessible total/position metadata.

**Location:** `DESIGN.md` Layout & Spacing and Components; `EXPERIENCE.md` Checkbox, Package Row, keyboard behavior, and Accessibility Floor.

**Fix:** Add one normative Package-table interaction pattern covering semantics, keyboard navigation, direct plan membership, range/bulk selection, stable identity/focus, virtual-row accessibility metadata, final-row reachability, and a short path to the Upgrade Sidecar. Verify the packaged app with VoiceOver and 101 rows.

#### Accessibility — Focus transitions are undefined across confirmation, Activity, and Results

Dialog dismissal is covered, but initial dialog focus, final-confirm focus, Activity updates, Activity-to-Results transformation, and Results dismissal are not.

**Location:** `EXPERIENCE.md` Confirmation Dialog, Confirmation state, Keyboard, and Accessibility Floor.

**Fix:** Add a focus-transition matrix. Initial focus must not make final confirmation the accidental default; final confirmation moves focus to an Activity summary; row updates never move focus; Results preserves viable focus or moves to its heading only when necessary; closing Results restores focus to a relevant surviving control.

#### Accessibility — Fixed panes conflict with the 200% zoom requirement

A fixed 190px sidebar plus 340–380px sidecar cannot remain usable at the effective 450 CSS-pixel width created by 200% zoom in a 900px-wide window.

**Location:** `DESIGN.md` Layout & Spacing; `EXPERIENCE.md` Accessibility Floor and Responsive & Platform.

**Fix:** Define a high-zoom/narrow-layout mode that collapses navigation and presents Plan/Activity/Results as a full-workspace or stacked surface. Test 100%, 150%, and 200% for overlap, reachable actions, and no two-dimensional scrolling for primary work.

### Medium (7)

#### Usability — Sidecar Activity, full Activity, and History replay can conflict

Define one shared live state and what happens when a replay is opened during an active plan, including a clear `Back to live activity` path.

#### Usability — Retry may displace an unread Results Summary

Define the transition from failure diagnosis to a new draft. Preserve the completed result in History and provide a clear return link.

#### Usability — Excluded, unavailable, and delegated ownership lack standard explanations

Give every nonselectable state a reason and recovery path. Use plain language such as `Managed through mise`.

#### Usability — Application-update discovery has no canonical location

Choose Settings → Pack-Manager updates as the detailed location and provide one restrained application-level notice or badge that links there.

#### Usability — Settings persistence model is undefined

Choose immediate save with visible saved/failure rollback, or one consistent Apply/Cancel model.

#### Accessibility — Live-announcement policy could create silence or chatter

Define one atomic channel for plan start, changed waiting reason, action-required failure, Manager completion, and final outcome. Suppress duplicate notification speech and never announce output lines or progress ticks.

#### Accessibility — Explanatory-disabled Checkbox semantics are contradictory

A native disabled Checkbox cannot receive focus or activation. Use an unavailable semantic state such as `aria-disabled="true"` without the native disabled attribute, attach a persistent accessible description, keep activation inert, and treat the Tooltip/Popover as supplemental.

### Low (2)

#### Usability — Results dismissal has two names

Choose one label. `Done` is clearer for completing the workflow.

#### Usability — History’s `updated/total` wording can overstate success

Use verified outcome language such as `10 of 12 verified · 2 failed`.

## Applied-resolution verification

- All reported usability and accessibility findings are represented in `DESIGN.md` and `EXPERIENCE.md`.
- The component tables match across the spine pair, and every referenced color token resolves.
- Focus-ring contrast, all six annotated user journeys, YAML frontmatter, formatting, and whitespace checks pass.
- Manager workspace, live Activity and Results, History replay, and Settings/app-update mockups were validated at 1440 × 900 and 900 × 600.
- The validated pages have no document or app-window horizontal overflow and produced no browser-console errors or warnings.
- The completed Results canvas was also inspected independently at both validation sizes.
- The required structure-and-prose review completed; all ten material prose refinements were applied. Mandatory machine-readable frontmatter, canonical section order, full journeys, and deliberate cross-checks were preserved.

## Recommended resolution order

1. Define editable plan removal, bulk-selection scope, confirmation-off behavior, abnormal Activity actions, and retry/History identity.
2. Define focus-ring styling, the virtualized Package-table interaction model, lifecycle focus transitions, and high-zoom reflow.
3. Resolve Activity/replay coexistence, Results-to-Retry transition, unavailable-state explanations, app-update discovery, Settings persistence, announcements, and explanatory-disabled semantics.
4. Standardize `Done` and verified History wording.
5. Update the remaining mockups only after the high findings are committed to the spine pair.

Steps 1–5 and finalization are complete.

## Reviewer files

- `review-usability.md`
- `review-accessibility.md`
