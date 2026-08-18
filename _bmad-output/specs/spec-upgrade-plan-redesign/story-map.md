# Story Map — Upgrade Plan Redesign (Epic UX-PB)

Maps this spec's capabilities to the 28 `epics.md` stories (the binding acceptance-criteria authority), their wave order, and the decisions/invariants that govern them. Status lives in `sprint-status.yaml`; ids there use the `ux-pb-Na-…` slug form of the story numbers below.

| CAP | Stories | Wave | Governed by |
| --- | --- | --- | --- |
| CAP-1 | UX-PB.1a, UX-PB.1c | 1 | D27; AD-16, AD-17, AD-23, AD-24, AD-28 |
| CAP-2 | UX-PB.1b | 1 | D27; AD-17, AD-28, AD-30 |
| CAP-3 | UX-PB.1d | 1 | D38; AD-16, AD-28; D37 (restated, not deleted) |
| CAP-4 | UX-PB.1e | 1 | D27; AD-23, AD-25, AD-28 |
| CAP-5 | UX-PB.2a, UX-PB.2b | 2 | D29, D30; AD-3, AD-16, AD-18 |
| CAP-6 | UX-PB.2c, UX-PB.2d | 2 | D29; AD-18, AD-29 |
| CAP-7 | UX-PB.2e (wave 2), UX-PB.3g (wave 3) | 2–3 | D30; AD-16, AD-29; no `Cancelling` (FR-13) |
| CAP-8 | UX-PB.2f (wave 2), UX-PB.4e (wave 4) | 2–4 | D29; AD-18, AD-29, AD-30 |
| CAP-9 | UX-PB.3a, UX-PB.3b, UX-PB.3c | 3 | D29, D30; AD-16, AD-17 |
| CAP-10 | UX-PB.3d, UX-PB.3e | 3 | D29, D30; AD-16, AD-25, AD-29 |
| CAP-11 | UX-PB.3f | 3 | D30; AD-16 |
| CAP-12 | UX-PB.4a, UX-PB.4b, UX-PB.4c | 4 | D29, D30; AD-18, AD-24, AD-29 |
| CAP-13 | UX-PB.4d | 4 | D29; AD-16, AD-24 |
| CAP-14 | UX-PB.5a, UX-PB.5b, UX-PB.5c | 5 | D27, D28; AD-16, AD-19, AD-21, AD-22; D37 (5a restated) |
| CAP-15 | UX-PB.5d | 5 | NFR-3; AD-17, AD-27; D37 (restated) |
| CAP-16 | UX-PB.5e | 5 | D25/D25a boundary; AD-17 |

Every story additionally carries AD-27 (focus outline mechanism).

## Ordering that is load-bearing, not stylistic

- Waves gate on completion of the prior UX-PB sub-epic: UX-PB.2a needs 1a–1e complete; 3a needs 2a–2f; 4a needs 3a–3g; 5a needs waves 1–2; 5e needs wave 4 (History must exist to assert separation).
- **UX-PB.2e ships in wave 2** because the `Cancelling` correction is wire surface (AD-3 atomic change) — it cannot be retrofitted after later stories build on the enum.
- **UX-PB.3a does not wait on 5a**: its trigger is atomic admission (2b). Keying it to the dialog leaves it undefined on the confirmation-off path (5c).
- **The terminal-record append belongs to UX-PB.2e**, not the History story that reads it (AD-29) — 4a lands two waves later, and giving it the write would read every intervening terminal attempt as `Interrupted`.
- Cross-spec: UX-PB.1a blocks Story 3.5, 1d blocks 3.2, 1e blocks 3.1, 5b blocks 3.4, 3d and 4d block 6.5 (see `spec-shipped-behavior-gaps`). Story 6.6 is independent; UX-PB.1b/2f/4e presume its guard and never build a second one (AD-30).
