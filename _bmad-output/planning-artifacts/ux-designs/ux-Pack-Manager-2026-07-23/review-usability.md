# Focused Usability Review — Pack-Manager

## Overall verdict

The UX direction is coherent and well suited to a non-developer macOS user: it replaces command-line reasoning with one understandable Upgrade Plan lifecycle, keeps evidence available without making it primary, and treats failures honestly. It is **not yet handoff-safe without targeted clarification**, because several high-impact actions still have ambiguous scope or consequences. Resolve the high findings before the remaining mockups establish those behaviors visually.

## Strengths

- The lifecycle—draft Upgrade Plan → confirmation → live Activity → Results → one History entry—creates a strong mental model.
- Manager self-update status, managed-Package health, and refresh reliability are separated clearly.
- Package rows stage work instead of executing it, preventing accidental one-click updates.
- Confirmation shows exact commands while preserving a human-readable plan.
- Activity avoids fabricated progress and declares success only after refresh verification.
- Results prioritizes diagnosis and safe next steps before Retry.
- History is plan-centric, which matches how the user thinks about an update session.
- Pack-Manager application updates remain distinct from Package updates.
- Keyboard, VoiceOver, focus, reduced-motion, and non-color status requirements are unusually complete.

## Findings

### High

1. **The Upgrade Plan does not define a direct way to remove or change individual items from the sidecar.** `Change Plan` returns to the plan, but the plan is only described as grouped information; this is especially unclear after `Update Everything`, when the originating Dashboard has no Package rows to uncheck. The Manager header also changes to `In Plan` without saying whether that control removes the Manager self-update.  
   **Location:** `EXPERIENCE.md` Information Architecture → Surface map and Manager workspace structure (lines 56–71); Component Patterns → Upgrade Sidecar and Manager Header (lines 127, 132); Confirmation (line 170); AJ-2 (lines 318–323).  
   **Fix:** Make every staged Package and Manager self-update removable from the Upgrade Sidecar, with an explicit control and immediate count updates. Define `In Plan` as a removable/toggle state or pair it with a clear `Remove from plan` action. `Change Plan` should return focus to this editable plan.

2. **The header Checkbox scope is ambiguous in a virtualized Package list.** “Eligible visible rows” could mean only rows currently on screen or every row in the active filter. That ambiguity can change the number of Packages added without the user realizing it.  
   **Location:** `EXPERIENCE.md` Component Patterns → Checkbox and Package Filter (lines 123, 128); AJ-3 (line 336); `DESIGN.md` Layout & Spacing notes that Package tables may virtualize more than 100 rows.  
   **Fix:** Define “visible” as the complete filtered result set, never only the rendered viewport. Show the exact consequence, such as `Add all 8 updates`, use the mixed Checkbox state when only some are in the plan, and announce the resulting count.

3. **Disabling future confirmation leaves the primary action’s consequence unclear.** The contract says commands automatically expand and the action executes immediately, but the button is still described as `Confirm # Updates`, which implies another confirmation will follow.  
   **Location:** `EXPERIENCE.md` Draft Upgrade Plan (line 162), Confirmation (lines 167–171), Settings (line 206), and AJ-2 (lines 322–325).  
   **Fix:** When confirmation is disabled, show a persistent `Confirmation is off` safety message with a Settings link, keep commands expanded before activation, and change the button to an immediate-consequence label such as `Run 12 updates`. Restore `Confirm 12 updates` when the dialog is enabled.

4. **Activity names abnormal outcomes without defining the controls that lead to them or what happens if a command unexpectedly requests interaction.** `Cancel`, `skip`, stall choices, and timeout exist as states, but their availability, consequences, and confirmation behavior are absent. This could leave a non-developer watching an apparently stuck operation with no safe decision path.  
   **Location:** `EXPERIENCE.md` Live Activity (lines 174–183) and AJ-4 (lines 348–352).  
   **Fix:** Add an Activity action contract that lists exactly when `Keep waiting`, `Cancel plan`, or any approved `Skip` action appears and what each affects. Explicitly state that planned commands run non-interactively; if an unexpected prompt is detected, show `Interaction required` or an equivalent blocked state with a plain-language explanation and only the safe actions allowed by the execution contract.

5. **Retry conflicts with the one-entry History rule.** One statement says a confirmed plan creates one History entry “even if it contains … retries,” while Results says Retry creates a fresh reviewable Upgrade Plan. If that new plan is confirmed, it is a new execution attempt and should not be folded silently into the old entry.  
   **Location:** `EXPERIENCE.md` Lifecycle model (line 93), Results and recovery (line 193), and AJ-4 (line 354).  
   **Fix:** State that every confirmed execution attempt creates its own History entry. Link a retry entry to its source with plain language such as `Retry of plan from 10:42 AM`, while leaving the original failed result unchanged.

### Medium

1. **The relationship between sidecar Activity, full Activity, and History replay is underspecified.** The contract can produce two Activity presentations at once, and it does not say what happens if a prior replay is opened while a live plan is running.  
   **Location:** `EXPERIENCE.md` Primary navigation and Surface map (lines 46–63); Upgrade Sidecar (line 132); AJ-4 and AJ-5 (lines 348, 364).  
   **Fix:** Define one shared live state model and explicit presentation rules: what the sidecar shows while the full Activity screen is open, how `Back to live activity` works, and whether replay is blocked or clearly secondary during an active run.

2. **Retry may displace the Results Summary that is supposed to remain visible until closed.** The contract does not explain how a new draft coexists with or replaces the diagnosis the user is reading.  
   **Location:** `EXPERIENCE.md` Results Summary (line 135) and Results and recovery (lines 187–193).  
   **Fix:** Define the transition. For example, Retry first shows the failed-item scope, then intentionally replaces the sidecar with a new draft while the completed result remains available in History with a clear return link.

3. **Pinned Packages receive exact explanatory copy, but excluded and unavailable Packages do not.** The approved delegated-owner information also lacks a standardized plain-language pattern, despite the rule against unexplained route/owner jargon.  
   **Location:** `EXPERIENCE.md` Manager workspace structure (lines 68–70), Package Row (line 129), Draft Upgrade Plan (line 160), and Voice and Tone (lines 102–111).  
   **Fix:** Give every unavailable selection state a short reason and recovery path. Standardize delegated ownership as plain language such as `Managed through mise`, with an accessible explanation of where its update will be grouped and executed.

4. **Application update discovery has no canonical location.** `Application Update Status` is described as living in a “Settings/status area,” while the IA only names Settings. A background-ready update could therefore be easy to miss.  
   **Location:** `DESIGN.md` Components → Application Update Status; `EXPERIENCE.md` Component Patterns (line 139), Settings and application updates (lines 203–212), and AJ-6 (lines 376–382).  
   **Fix:** Choose one canonical detailed location, such as Settings → Pack-Manager updates, and one restrained application-level notice or badge that links there. Keep both explicitly outside Upgrade Plans, Activity, Results, and History.

5. **Settings does not establish whether changes save immediately or require an Apply action.** “Saved-state feedback” is required, but the interaction model and failure rollback are not.  
   **Location:** `EXPERIENCE.md` Component Patterns → Settings Section (line 138) and Settings and application updates (lines 203–212).  
   **Fix:** Choose one persistence model for the entire page. If controls save immediately, show `Saved` near the changed control and revert it on failure; if changes are staged, provide one predictable Apply/Cancel pattern.

### Low

1. **The Results dismissal action has two names.** `Done/Close` leaves downstream UI copy unresolved.  
   **Location:** `EXPERIENCE.md` Results and recovery (line 187).  
   **Fix:** Choose one label and use it everywhere; `Done` is clearer for a completed workflow, while the standard window close control may remain available.

2. **History’s “updated/total” summary can overstate success when verification fails.** The rest of the contract carefully distinguishes process completion from verified state.  
   **Location:** `EXPERIENCE.md` AJ-5 (line 363) and `DESIGN.md` Components → History Plan Row.  
   **Fix:** Use explicit outcome wording such as `10 of 12 verified` or `2 failed` rather than a generic `updated/total`.

## Compact summary

| Severity | Count | Meaning                                                      |
| -------- | ----: | ------------------------------------------------------------ |
| Critical |     0 | No foundational workflow failure found                       |
| High     |     5 | Resolve before rendering the remaining authoritative mockups |
| Medium   |     5 | Resolve during mockup refinement and contract polish         |
| Low      |     2 | Copy and consistency cleanup                                 |

The strongest next move is to settle the five high findings in `EXPERIENCE.md`, then render the Manager workspace, Activity/Results, History replay, and Settings against those clarified behaviors.
