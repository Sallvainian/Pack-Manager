# Focused Accessibility Review — Pack-Manager

## Overall verdict

**Adequate foundation, but not yet implementation-ready for accessibility.** The spine pair makes accessibility a real product contract rather than an afterthought: it covers keyboard activation, modal containment, non-color status, reduced motion, completion announcements, copyable evidence, and minimum-window capacity. Four high-impact details remain underspecified or internally inconsistent, however, and downstream implementation could satisfy the prose while still producing lost keyboard focus, inaccessible virtualized rows, or unusable 200% zoom behavior.

No critical issue was found. Resolve the high findings before marking the spines final; the medium findings should be resolved in the same pass because they affect VoiceOver clarity and the pinned-Package interaction.

## Strengths

- The contracts require keyboard operation, visible focus, meaningful VoiceOver roles/states, non-color cues, and reduced motion across every primary surface (`EXPERIENCE.md:223–257`).
- Status colors consistently travel with text, counts, icons, or exact values; the Health Meter repeats current/total counts instead of relying on hue or length (`DESIGN.md:131–138`; `EXPERIENCE.md:130–131, 248–249`).
- Required text combinations are strong. Calculated examples include `textMuted` on `overlay` at **4.99:1**, `textSecondary` on `surface` at **8.51:1**, `onAccent` on `accent` at **7.74:1**, and status colors on `surface` at **7.42:1 or higher** (`DESIGN.md:14–35, 107–129`).
- The Confirmation Dialog is modal, focus-contained, dismissible before execution, and command-visible; opening the Upgrade Sidecar intentionally preserves source focus (`EXPERIENCE.md:132–133, 157–172, 227–229`).
- Live execution avoids fabricated progress and explicitly suppresses output-line chatter; Results remains persistent and is announced (`EXPERIENCE.md:174–193, 236–251`).
- Tooltip/Popover access is not hover-only, and the pinned state has visible text plus an explanatory interaction path (`EXPERIENCE.md:123, 137, 252–253`).

## Findings

### High

#### 1. The designated focused-boundary color is not strong enough for a reliable focus indicator

`borderStrong` is described as the token for “selected, focused, or emphasized boundaries,” while every interactive component is required to have visible focus (`DESIGN.md:22, 116, 193–215`; `EXPERIENCE.md:245–247`). Its calculated contrast is only **2.67:1 against `background`** and **2.33:1 against `surface`**, so an implementation that follows the token description can produce a weak focus boundary. The documents do not name a different focus-ring token, width, or offset.

**Concrete fix:** Define one explicit focus-ring contract in `DESIGN.md`: use `accent` (at least 5.85:1 against all listed dark surfaces) or add a dedicated high-contrast focus token; require a minimum 2px ring with separation from the component boundary; and specify that selected state and focus state remain visually distinguishable. Reference that token from every `focus-visible` component state.

#### 2. The dense Package-table keyboard and VoiceOver model is incomplete

The contracts require more than 100 Package rows and recommend virtualization, but the keyboard model only says Tab follows the table and Enter/Space activates controls (`DESIGN.md:168–170, 196, 202`; `EXPERIENCE.md:123, 225–230, 247, 255`). They do not define whether every row Checkbox is a Tab stop, how arrow/range/bulk navigation behaves, how the mixed header Checkbox is announced, or how focus survives filter changes and recycled virtual rows. A literal implementation could require tabbing through 101 rows, remove the focused row from the accessibility tree, or expose only the rendered window to VoiceOver. This also leaves the accepted source behavior for exact keyboard selection and final-row reachability without a UX-level mapping.

**Concrete fix:** Add one normative Package-table interaction pattern covering:

- the chosen table/grid semantics and column headers;
- a bounded Tab sequence with documented arrow navigation inside the row set;
- Space, range selection, select-all/clear, and mixed-header behavior as direct Upgrade Plan membership for eligible visible Package identities;
- stable Package identity, focus, and membership across filtering and virtualization;
- accessible total/position metadata for virtual rows, scrolling the focused row into view, and guaranteed reachability of the final row/action;
- a short keyboard path from the Package table to the visible Upgrade Sidecar.

Require packaged VoiceOver verification with 101 rows, not only DOM-level assertions.

#### 3. Focus destinations are undefined when confirmation becomes Activity and Activity becomes Results

The dialog contract says focus moves inside and returns on dismissal, but it does not select the initial focus target or define focus after final confirmation destroys the dialog and transforms the sidecar (`EXPERIENCE.md:133, 165–172, 225–230, 251`). It also says Results receives an announcement without defining whether focus remains stable when Activity becomes Results. This creates a plausible lost-focus path at the product's most consequential interaction.

**Concrete fix:** Add a focus-transition matrix:

- on dialog open, move focus to the dialog heading/command summary or the least-consequential `Change Plan` action; final confirmation must not be the implicit default for an unfocused Enter press;
- on Change Plan, Escape, or backdrop dismissal, restore focus to `Confirm # Updates`;
- on final confirmation, move focus to a programmatically focusable Upgrade Activity summary heading in the transformed sidecar;
- Activity row updates never move focus;
- Activity-to-Results preserves the current viable focus and announces one atomic result summary, or moves to the Results heading only if the focused node was removed;
- closing Results restores focus to the most relevant surviving page control or page heading.

Associate the skip-future checkbox with its safety explanation through an accessible description.

#### 4. The 900 × 600 and 200% zoom contracts cannot both hold with the fixed pane widths

`DESIGN.md` fixes a 190px sidebar and a 340–380px Upgrade Sidecar at the 900 × 600 minimum (`DESIGN.md:161–170`). `EXPERIENCE.md` repeats those fixed widths while also requiring tables to remain usable at 200% zoom within that minimum (`EXPERIENCE.md:255, 283–292`). At 200% zoom, the effective layout width is approximately 450 CSS pixels, less than the sidebar plus sidecar alone. Internal table scrolling cannot prevent pane overlap or make the main workspace usable.

**Concrete fix:** Define a high-zoom/narrow-layout mode. At the equivalent of 900 × 600 at 200%, collapse navigation to an accessible icon rail or temporary navigation panel, and present Upgrade Plan/Activity/Results as a full-workspace or stacked surface rather than a fixed 340–380px sidecar. Preserve headings, status, versions, primary actions, focus order, and a clear path back. Add acceptance checks at 100%, 150%, and 200% for no overlap, reachable actions, and no two-dimensional scrolling for primary tasks.

### Medium

#### 5. Live announcements are directionally correct but not specific enough to prevent silence or chatter

The contracts say to throttle Activity announcements and announce Results, while Brief Notifications may also announce the same event (`EXPERIENCE.md:142, 174–193, 250–251`). They do not define which events are announced, the live-region priority, batching, or duplicate suppression. Implementations could announce every Package transition, repeat completion through both Results and a notification, or fail to announce a waiting/failure state that requires action.

**Concrete fix:** Define one atomic status-announcement channel. Announce plan start, a changed waiting reason, a failure requiring action, each Manager's completion summary, and the final plan outcome; do not announce queued rows, progress ticks, or command-output lines. Use polite priority by default and assertive priority only for an immediate safety action. Suppress duplicate Brief Notification speech when Results already announced the same event, and never move focus as part of a live announcement.

#### 6. “Explanatory-disabled” Checkboxes need an explicit semantic rule

Pinned controls must look disabled while remaining focusable and clickable for an explanation (`DESIGN.md:196, 202, 210`; `EXPERIENCE.md:123, 137, 252–253`). A native disabled Checkbox cannot receive keyboard focus or pointer activation, so the current wording can be implemented in mutually incompatible ways. The PINNED chip supplies the state, but the relationship between the Checkbox and its explanation is not specified.

**Concrete fix:** State that explanatory-disabled controls must not use the native/HTML disabled attribute. Expose the control as unavailable through the approved semantic state (for example, `aria-disabled="true"` in the webview), associate the persistent reason with the control's accessible description, keep activation inert, and treat the Tooltip/Popover as supplemental visual help. Escape closes the popup and focus remains on the Checkbox. Apply the same rule wherever current, excluded, or unavailable Packages expose a non-actionable aligned control.

## Compact summary

| Severity | Count | Main themes                                                                               |
| -------- | ----: | ----------------------------------------------------------------------------------------- |
| Critical |     0 | —                                                                                         |
| High     |     4 | Focus-ring contrast, dense virtualized table operation, lifecycle focus, 200% zoom reflow |
| Medium   |     2 | Live-announcement policy, explanatory-disabled semantics                                  |
| Low      |     0 | —                                                                                         |

**Verdict:** Keep the current accessibility direction. Before finalization, make the six fixes above explicit in the spine pair so implementation and packaged acceptance have one unambiguous contract.
