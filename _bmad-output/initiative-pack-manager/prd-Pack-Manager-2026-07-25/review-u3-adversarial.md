# Review U3 — Lens 2: Adversarial Consistency

**Target:** the 2026-08-18 changes to `_bmad-output/planning-artifacts/prds/prd-Pack-Manager-2026-07-25/prd.md` only — the §9 closure pointer (line 737), the new §9.3 (lines 773–783), and FR-1's retag + Planned — D40 limb (lines 139, 146). Verified against the uncommitted working tree on top of `5c28dcb`.

**Method:** every attacked claim defaulted to refuted; it stands only where the cited file, read this session, supports it verbatim. Each verdict below quotes its evidence as `path:line "text"`.

**Verdict: today's changes survive all four primary refutation attacks.** Every closure in §9.3 and every clause of the FR-1 limb is supported by the file it cites. Four secondary precision defects were found — one medium (a genuine internal contradiction the new Q1 text creates with FR-20's unconditional consequence), three low. None blocks.

---

## Attack (a) — Q1 "process-scoped, no persistence" vs RP-1 and FR-20/FR-21

**Attack:** if the pending download is process-scoped with no restore path, RP-1's continuity consequences or FR-20/FR-21's update-state consequences should break.

**Primary attack REFUTED — the closure stands.**

- RP-1's supposedly contradicted consequence is `prd.md:575` "In-process update state survives supported UI recreation." *In-process* is the process scope D39 decides — UI recreation is not process death. The state lives in the Rust backend: `src-tauri/src/app_update.rs:65` `downloaded: Mutex<Option<Downloaded>>`, and a recreated webview re-reads it (the `get_app_update_state` command is registered — `prd.md` §0.1 table names it among the 20). The two statements reinforce each other; they do not collide.
- The 9.3 code claims check out exactly: `app_update.rs:76` `state: AppUpdateState::Idle,` and `app_update.rs:81` `downloaded: Mutex::new(None),` at construction — a fresh process really does construct at `Idle` with no restore path, and nothing in `app_update.rs` reads a persisted payload. `app_update.rs:35` "A downloaded, signature-verified release paired with its archive bytes." — bytes in memory, never on disk.
- RP-1's fail-closed consequence is *strengthened*, not contradicted: `prd.md:577` "A failed or interrupted download never presents as Ready." A `Ready` that is always re-derived by the live process cannot present a stale artifact.
- 9.3's "the saved trigger policy still survives a normal relaunch" matches RP-1 (`prd.md:576`) and `src-tauri/src/settings.rs:38/:51` (`auto_check_for_updates`, default `true`, persisted).
- FR-21 is untouched: `prd.md:539` "A downloaded update installs only after the user chooses Restart to update." Nothing in D39 installs anything.

**Collateral hit CONFIRMED (Finding 1, medium).** The new Q1 text asserts a gate FR-20 does not carry: `prd.md:777` "Recovery is the launch check and six-hour heartbeat **while `autoCheckForUpdates` (default on) holds**; with it off, the manual menu check". The code agrees with 9.3 — `src-tauri/src/lib.rs:135` "Launch check + a 6h heartbeat, both gated on `autoCheckForUpdates`." But FR-20's Shipping consequences are unconditional: `prd.md:527` "Checks run at launch, every six hours, and on demand from the application menu." and `prd.md:528` "A newer authorized release downloads automatically in the background. Automatic **download** is required behavior". With the setting off, the first two limbs of :527 and all of :528 are false in the shipped build — and now the PRD's own §9.3 says so while FR-20 still doesn't. This is the FR-18-shaped trap §9.2 itself names (`prd.md:765` "a Shipping tag over an unbuilt consequence is exactly the trap FR-18 set" — here it is a Shipping tag over a *conditional* consequence stated unconditionally). The closure is right and matches code; FR-20 is the document's own counter-example. Fix: qualify FR-20's automatic-check/download consequences with `autoCheckForUpdates` (default on), the way 9.3 already does.

---

## Attack (b) — FR-1 limb's "never reads as `Warning`" vs DESIGN.md / EXPERIENCE.md

**Attack:** the system-summary rules should force `Warning` (or forbid the claimed state) on an all-absent machine.

**REFUTED — the claim stands.** Both UX authorities trigger `Warning` on exactly one condition, refresh failure, and both reserve a distinct no-Managers state:

- `DESIGN.md:247` "Do not call the system `Ready` when any Manager refresh has failed; use `Warning` with the exact failure count." — the *only* `Warning` mandate in DESIGN.md, conditioned on refresh failure.
- `EXPERIENCE.md:145` "System health becomes `Warning` when any Manager refresh has failed even if a last-good snapshot keeps the app usable." — same single trigger.
- An absent Manager has no refresh to fail: FR-1 (`prd.md:146`) "Absence is a normal state — rendered as 'Not installed' … never an error", and `refresh_all` re-detects and fans out only from the fresh result (AGENTS.md pitfall; FR-1 consequence "All six Managers are probed at launch and on an explicit Re-detect action", `prd.md:145`).
- The reserved state D40 composes exists by name in both component tables: `DESIGN.md:205` System Summary Card key states "Ready, Warning, **no Managers**, loading" and `DESIGN.md:222` State Panel key states "Loading, empty, offline, **no Managers**, no active upgrade, interrupted, fatal". So the all-absent Dashboard has a designed home that is neither `Ready` nor `Warning`, and "never reads as `Warning`, because absence is not failure" (`prd.md:146`) contradicts nothing.
- A stricter sub-attack — "six muted `Not installed` cards" vs `DESIGN.md:242` "Do not show a permanent Manager list on the Dashboard" — also fails: Manager Cards are the *designed* Dashboard surface (`DESIGN.md:77` "manager-card: Dashboard overview of one Manager", `DESIGN.md:175` "Use two Manager Card columns when space permits"), with "unavailable" among their key states (`DESIGN.md:206`). The Don't forbids duplicating the sidebar's Manager *list* (`DESIGN.md:75`), not the card grid.
- Adjacent observation, not a finding against today's change: `EXPERIENCE.md:386` lets AJ-1 settle into "`Ready` when all detected Managers are trustworthy" — vacuously satisfiable at zero detected Managers, which would collide with the reserved no-Managers state. That ambiguity is EXPERIENCE.md's, predates today, and in no reading produces `Warning`, so the attacked claim survives either way.

---

## Attack (c) — copyable hints for all six vs FR-23's closed set, §6 non-goals, SM-C2

**Attack:** extending install hints to all six Managers should grow FR-23's closed immediate-execution set, cross the §6 installer non-goal, or breach SM-C2.

**REFUTED — the decision stands.**

- FR-23's closed set counts *execution* kinds: `prd.md:366` "Three *kinds* of affordance **run** one known command against one named target without staging… What the set may not do is **grow**: a fourth kind is a new decision". A copyable hint runs nothing. D40 refuses the only variant that would grow the set, on exactly this ground: `docs/DECISIONS.md:658-660` (D40) "the button would be a fourth immediate-execution kind, which FR-23 forbids growing." The FR-1 limb repeats the refusal: `prd.md:146` "never an executing Install button". Consistent, not colliding.
- §6: `prd.md:658` "**A package installer or uninstaller.** It updates what is already installed. Nothing else." The non-goal bars the *product* installing. A hint is copy text handed to the terminal — the shipped pattern already: `src/components/shell/Sidebar.tsx:132` `<CopyableCommand command={m.installHint} label="Install" />` and `src/components/settings/SettingsView.tsx:275` `<CopyableCommand command="brew install mas" …/>`, grounded in D14 ("copy-to-terminal handoff", cited at `docs/DECISIONS.md:648` and Story 2.5's contract). D40 additionally cites SM-3 and the no-shell boundary against execution (`docs/DECISIONS.md:653-657`), matching `prd.md:721` (SM-3) and `prd.md:661` ("**A terminal.** No general shell surface"). The FR-1 limb's factual anchor is verbatim true: `src-tauri/src/detect.rs:273-277` — `install_hint` matches `ManagerId::Mas => Some("brew install mas".into()), _ => None` — "returns one only for mas" confirmed.
- SM-C2 is a counter-metric, not a prohibition: `prd.md:731` "Counterbalances the temptation in §7.3… §6 exists to make the boundary expensive to cross by accident." Six static strings through an existing component cross no §6 boundary, and D40's rejected-alternatives record shows the counter-metric was actually applied (rejects context-aware suppression as "cleverness"; D39 and D41 reject machinery on SM-C1/SM-C2 grounds explicitly). Per `prd.md:16`, the 2026-08-18 owner decision supersedes anyway.
- Story 2.5 exists as claimed: `epics.md:1272` "### Story 2.5: Offer Copyable Install Guidance for Absent Managers", `sprint-status.yaml:122` `2-5-offer-copyable-install-guidance-for-absent-managers: backlog`, provenance note at `sprint-status.yaml:67`; `epics.md:463` records the FR-1 revival. The proposal file exists and carries the verification claim 9.3's preamble repeats (`sprint-change-proposal-2026-08-18.md:16` "closed as D39–D42 on 2026-08-18 after an adversarially verified recommendation pass").

**Collateral hit (Finding 3, low).** D40's closure has three components — hints, guidance panel, and "`Update Everything` disabled with a reason" (`docs/DECISIONS.md:636-637`). §9.3 Q2 carries all three (`prd.md:779` "plus a disabled `Update Everything`"), and Story 2.5's AC tests the third (`epics.md` Story 2.5: "`Update Everything` stays disabled with a reason"). But the FR-1 limb (`prd.md:139,146`) names only the first two, and no FR consequence anywhere in §4 homes a disabled `Update Everything` for the nothing-eligible case. The requirements authority (`prd.md:16`) thus under-specifies a behavior its own §9.3 and the story's AC both assert. Fix: add the disabled-with-reason clause to the FR-1 limb (or home it at the FR that owns `Update Everything` staging) so the AC traces to a requirement, not only to D40.

---

## Attack (d) — Q3/Q4 wording vs FR-15, FR-18, AD-18, AD-29, D29

**Attack:** the History and diagnostics closures should contradict the evidence-preservation or export requirements.

**Primary attack REFUTED — both closures stand on their citations.**

- Q3 vs FR-15: `prd.md:781` "automatic compaction to the newest 1,000 records stays the only pruning" matches FR-15's retention consequence `prd.md:410` "History compacted to the newest 1,000 records" (transcript/log pruning in the same line is separate retention for separate stores, which Q3's History-scoped sentence does not deny). "History rows are immutable evidence (D29…)" is a faithful cite: `docs/DECISIONS.md:209-210` (D29) "History contains one immutable entry per confirmed plan attempt", ":213-214" "it never overwrites the first failure" — mirrored in FR-15's Planned limb (`prd.md:412`). SM-4 cite checks: `prd.md:725` "SM-4: Failure legibility… Validates FR-15, FR-16, FR-18." Refusing user deletion *protects* FR-15/SM-4; nothing collides.
- Q4 vs FR-18/NFR-5: every construction-time clause is verbatim in the FRs — `prd.md:479` "It contains **only** environment values Pack-Manager itself set. The inherited environment is never dumped.", `prd.md:480` "Symlink substitution is rejected both when selecting files and when streaming them.", NFR-5 (`prd.md` §5) "Diagnostic export resists symlink substitution." No-transmit-path checks against `prd.md:663` "**A telemetry client.** Nothing is reported anywhere." The Story 6.5 claim is exact: `epics.md:1445-1447` "**Given** documented default destination, alternate permission outcomes, and invocation from Settings and History … **Then** the timestamped ZIP path and visible success/failure match the contract." — the "both invocation points" are Settings and History, and it is indeed an existing acceptance criterion, not new work. AD-18 names the residue as claimed: spine `:846-848` "never ambient environment or user paths beyond what the existing entries already disclose."
- D42's rejected-alternatives grounds (in DECISIONS.md, mirrored but not cited by the PRD text) also verify: automatic redaction vs the raw-lines mandate is spine AD-29 `:1428` "The diagnostics archive carries the **raw journal lines**, not a synthesized record", matching FR-18's Planned limb (`prd.md:482` "raw lines — never a synthesized record").

**Two precision hits survive:**

- **Finding 2 (low) — AD-18 attribution in Q4.** `prd.md:783` says the residue "is named and bounded by AD-18's per-field disclosure review". AD-18's review clause is scoped to *new attempt-record fields*: spine `:848-849` "A story that **adds a field to the attempt record** owns its disclosure review." The existing `report.json`/transcript residue is bounded by FR-18's closed allowlist (`prd.md:477-479`); AD-18's contribution is that widening the export may not widen disclosure. The substance survives (AD-18 does name and cap the residue), but the mechanism attribution is loose. Fix: "bounded by FR-18's closed allowlist, with AD-18's widening rule and per-field disclosure review keeping it from growing."
- **Finding 4 (low) — Q3's "carries no promised behavior" over-disclaims.** `prd.md:781` "Out-of-band deletion of `operations.jsonl` remains possible, unsupported, and carries no promised behavior." Read literally, that excuses any behavior — but NFR-2 (Shipping) promises "persistence failures are contained" (`prd.md` §5) and AD-19 promises reads "never fail the application" (spine AD-19: "Reading a persisted file tolerates unknown and retired fields and never fails the application."). An app that crashed on a deleted journal would still be a defect. What is genuinely unpromised is the *History semantics* of a hand-pruned journal, not the app's survival. Fix: "no promised History semantics" (or "no promised behavior beyond the containment NFR-2 already requires").

---

## Findings (ranked)

| # | Severity | Where | Defect |
| --- | --- | --- | --- |
| 1 | Medium | `prd.md:527-528` vs `prd.md:777` | New Q1 text conditions the launch/6-hour checks and auto-download on `autoCheckForUpdates` (correct per `lib.rs:135`); FR-20's Shipping consequences state them unconditionally. The document now contradicts itself, and FR-20's tag sits over a consequence that is false with the setting off — the §9.2 trap. Qualify FR-20. |
| 2 | Low | `prd.md:783` | "bounded by AD-18's per-field disclosure review" over-attributes: that review governs new attempt-record fields (spine :848-849); the existing residue is bounded by FR-18's allowlist, AD-18 prevents widening. |
| 3 | Low | `prd.md:139,146` | FR-1's Planned limb omits D40's third component — "`Update Everything` disabled with a reason" — which §9.3 Q2 and Story 2.5's AC both carry; the behavior has no FR home anywhere in §4. |
| 4 | Low | `prd.md:781` | "carries no promised behavior" for out-of-band journal deletion over-disclaims against NFR-2's containment and AD-19's never-fails-the-app rule; only History semantics are unpromised. |

## Refuted attacks (for the record)

1. Q1 vs RP-1: "In-process update state survives supported UI recreation" (`prd.md:575`) *is* process scope; backend-held state survives UI recreation by construction. No contradiction.
2. FR-1 limb vs UX system-summary rules: `Warning` triggers only on refresh failure (`DESIGN.md:247`, `EXPERIENCE.md:145`); "no Managers" is a reserved state in both component tables (`DESIGN.md:205,222`). "Never reads as Warning" is supported, not contradicted.
3. Hints vs FR-23/§6/SM-C2: copy-only hints run nothing, so the closed execution set (`prd.md:366`) does not grow — D40 refuses the one variant that would; §6 bars product-executed installs, not copy text (D14 precedent, shipped at `Sidebar.tsx:132`); SM-C2 is a counter-metric the decision record demonstrably applied.
4. "Six Not installed cards" vs `DESIGN.md:242`: Manager Cards are the designed Dashboard surface (`DESIGN.md:77,175,206`); the Don't targets a duplicate Manager *list*, not the card grid.
5. Q3's deletion-motive hand-off to Q4: both closures deliberately refuse an in-product mechanism; the answer is that the ZIP is local, inspectable at a visible path, and never transmitted by the product (`prd.md:663`, `epics.md:1445-1447`). A rhetorical hand-off, not an inconsistency.
6. §9.3 preamble claims: "per §0, a decision later than 2026-07-25 supersedes anything here" is verbatim §0 (`prd.md:16`); "adversarially verified against `HEAD`" matches `sprint-change-proposal-2026-08-18.md:16`. Story 2.5 (`epics.md:1272`), the `2-5` key (`sprint-status.yaml:122`), and D39–D42 (`docs/DECISIONS.md:609,630,664,682`) all exist as cited; `grep -c '^## D' docs/DECISIONS.md` = 42.
