---
title: "Adopt the approved DESIGN.md palette and a dedicated focus ring"
type: "feature"
created: "2026-07-25"
status: "in-progress"
baseline_commit: "db92db7"
review_loop_iteration: 0
context:
  - "{project-root}/_bmad-output/planning-artifacts/ux-designs/ux-Pack-Manager-2026-07-23/DESIGN.md"
---

<frozen-after-approval reason="human-owned intent — do not modify unless human renegotiates">

## Intent

**Problem:** `src/styles/theme.css` still carries the 2026-07-22 foundation-stub palette (commit `69f37b0`), which predates the 2026-07-23 UX design and was never a decision anyone made; `tests/e2e/browser-style-contract.spec.ts` then froze those stub values on 2026-07-24, so placeholder colours are now held in place by CI. Separately, `theme.css:21` comments accent as "primary actions, focus, running" and all 22 focus-visible sites draw focus in accent blue, which `EXPERIENCE.md:318` forbids.

**Approach:** Replace every `@theme` colour value in `theme.css` with its `DESIGN.md` counterpart, add the five tokens `DESIGN.md` defines that the theme has no equivalent for (`focusRing`, `shell`, `onAccent`, `onSuccess`, `violet`), repoint all 22 `focus-visible` rings at the new `--color-focus-ring`, and move the three pinned CI assertions to the adopted values in the same change.

## Boundaries & Constraints

**Always:**
- Values come verbatim from the `DESIGN.md` frontmatter `colors:` block. Do not invent, round, or split the difference.
- Keep the existing `--color-*` variable names. Consumers across `src/` use them; renaming is not in scope.
- Keep all three CI assertions in `browser-style-contract.spec.ts` — retarget them, never weaken or delete. `ARCHITECTURE-SPINE.md` AD-11 relies on this lane for reduced-motion coverage.
- Focus and selection must stay visually distinct (`EXPERIENCE.md:318`).

**Ask First:**
- Any DESIGN.md value that cannot be mapped onto an existing token without renaming a consumed variable.
- Any change to a `ring-*` site that is not a `focus-visible` state.

**Never:**
- Do not edit `DESIGN.md` or `EXPERIENCE.md` — they are `bmad-ux` output and are the source here, not a target.
- Do not edit `ARCHITECTURE-SPINE.md` or `epics.md`.
- Do not commit, stage, create, or switch branches — two other sessions hold uncommitted work in this tree.
- Do not add `ring-offset-*` to sites that lack one; that is deferred, not done here.
- Do not repoint `PackageRow.tsx:85` — it is a navigation highlight, not focus.

## I/O & Edge-Case Matrix

| Scenario | Input / State | Expected Output / Behavior | Error Handling |
|----------|--------------|---------------------------|----------------|
| Body surface | App shell rendered in browser | `background-color: rgb(9, 12, 19)`, `color: rgb(244, 247, 251)` | N/A |
| Keyboard focus | `Refresh All` focused via keyboard | `:focus-visible` true; `box-shadow` contains `rgb(244, 247, 251)`, not `rgb(101, 167, 255)` | N/A |
| Focus vs selection | A row is selected and a control in it is focused | Selection reads as `accentSubtle` wash / `borderStrong`; focus reads as the near-white ring — distinguishable | N/A |
| Navigation highlight | Cross-manager join targets a row | Row keeps the accent `ring-inset`, distinct from the focus ring | N/A |
| Reduced motion | `prefers-reduced-motion: reduce` | Transitions and animations remain fully suppressed | N/A |

</frozen-after-approval>

## Code Map

- `src/styles/theme.css` -- the single `@theme` token block; the only file to carry colour values.
- `tests/e2e/browser-style-contract.spec.ts` -- pins body background (`:66`), body text (`:67`), and focus ring (`:90`).
- `docs/SPEC.md:157-196` -- verbatim mirror of the whole `@theme` block; `:198` asserts focus is a `--color-accent` ring.
- `docs/DECISIONS.md` -- D33 at `:310` was the highest at planning time; the team lead landed D34 (`macos-15` runners) concurrently, so this change took **D35** by their assignment.
- 16 component files -- 22 `focus-visible:ring-accent` sites. 6 already carry `ring-offset-*` (`Button.tsx:38`, `Checkbox.tsx:38`, `Chip.tsx:40`, `CopyableCommand.tsx:36`, `PackageRow.tsx:99`, `PackageTable.tsx:110`); 16 do not.
- `src/components/manager/PackageRow.tsx:85` -- `highlighted ? "ring-2 ring-inset ring-accent"`, sourced from `ui.ts:45` "Cross-manager join navigation target". **Not** a focus ring; stays accent.
- `src/components/manager/VersionDelta.tsx:19-21` -- sole consumer of the three `--color-sev-*` tokens.

## Tasks & Acceptance

**Execution:**
- [x] `src/styles/theme.css` -- replace all 17 mapped colour values with their `DESIGN.md` counterparts; repoint `--color-sev-major/minor/patch` at the new danger/warning/success; add `--color-focus-ring`, `--color-bg-shell`, `--color-on-accent`, `--color-on-success`, `--color-violet`; drop "focus" from the accent comment -- adopt the approved palette in the one file that owns it.
- [x] 16 component files -- swap `focus-visible:ring-accent` → `focus-visible:ring-focus-ring` at all 22 focus sites, leaving `ring-2` and any existing `ring-offset-*` untouched -- give focus its own indicator.
- [x] `tests/e2e/browser-style-contract.spec.ts` -- retarget `:66` to `rgb(9, 12, 19)`, `:67` to `rgb(244, 247, 251)`, `:90` to `rgb(244, 247, 251)`; update the step title from "accent ring treatment" -- keep the lane green and honest.
- [x] `docs/SPEC.md` -- regenerate the `§4.1` block to match `theme.css` exactly and rewrite `:198` so focus is the dedicated ring -- stop the spec contradicting the code.
- [x] `docs/DECISIONS.md` -- append **D35** in house format (decision → rejected alternatives → why) -- record why the stub palette and accent focus were replaced.
- [x] `_bmad-output/implementation-artifacts/deferred-work.md` -- append the 16 offset-less focus sites as one entry -- pre-existing `SPEC.md:198` violation, out of scope here.
- [x] `_bmad-output/implementation-artifacts/deferred-work.md` -- append a second entry for three focusable controls that render no focus indicator at all, found during visual verification -- pre-existing `EXPERIENCE.md:318` violation no test catches.

**Acceptance Criteria:**
- Given the adopted theme, when `npm run test:e2e` runs, then `browser-style-contract.spec.ts` passes with the three retargeted assertions and reduced-motion coverage intact.
- Given the adopted theme, when `npm test`, `npx tsc --noEmit`, and `npm run build` run, then all three pass.
- Given `grep -c 'ring-accent' src/`, when the change is complete, then exactly 1 match remains (`PackageRow.tsx:85`).
- Given `docs/SPEC.md §4.1`, when compared to `src/styles/theme.css`, then every token name and value agrees.

## Design Notes

Mapping is 1:1 by role, not by name (`DESIGN.md:110-132` supplies the role column):
`background`→`--color-bg-base`, `surface`→`--color-bg-surface`, `raised`→`--color-bg-raised`, `overlay`→`--color-bg-overlay`, `inset`→`--color-bg-inset`, plus `border`/`borderStrong`, the three text tokens, and accent/status.

Two consequences worth stating up front:
- `focusRing` and `textPrimary` are the same value, `#F4F7FB`. The body-text and focus-ring assertions therefore both expect `rgb(244, 247, 251)`. This is the design's intent, not a copy error.
- `--color-accent-subtle` goes from `#4F8CFF1F` (8-digit, 12% alpha) to opaque `#172A46`, so selected rows (`PackageRow.tsx:84`) and accent chips (`Chip.tsx:7`) stop being translucent.

`--color-sev-*` have no `DESIGN.md` equivalent, but their current values are byte-identical to the danger/warning/success stubs, so "severity mirrors status colour" is a relationship the code already encodes. Preserving it under the new palette is adoption; leaving them stubbed would put two palettes in one table row.

## Verification

**Commands:**
- `npm run test:e2e` -- expected: pass, `browser-style-contract.spec.ts` included, on both Chromium and WebKit.
- `npm test` -- expected: pass.
- `npx tsc --noEmit` -- expected: clean.
- `npm run build` -- expected: succeeds.
- `grep -rc 'ring-accent' src/` -- expected: only `PackageRow.tsx` matches, count 1.

**Manual checks (if no CLI):**
- Run the app and tab through the shell. Focus must be clearly visible on buttons, checkboxes, sidebar items, and the search input against every surface (`bg-base`, `bg-surface`, `bg-raised`, `bg-overlay`).
- Confirm a selected row and a focused control inside it remain distinguishable.
- Note: package rows carry no `tabIndex` anywhere in `src/components/manager/`, so a row is not itself focusable — the focusable elements inside it are its checkbox and expand button. Verify those.
