# Review — Lens 1: internal discipline and citation accuracy

- **Target:** today's (2026-08-18) changes to `prd.md` — the §9 open-question replacement, new §9.3, and the FR-1 retag/limb. Verified against the working-tree diff (`git diff` over `5c28dcb`); everything else in the PRD was out of scope and untouched.
- **Reviewer:** u3 (discipline lens), 2026-08-18.
- **Verdict: PASS.** Every factual claim in the new §9.3 and the FR-1 limb grounded against its cited source on the first attempt. No citation-convention violation, no status-tag violation, no renumbering. Two low findings, both about untouched sentences elsewhere in the PRD that today's change quietly made stale.

## 1. Checks against the PRD's own rules

### 1.1 §0 status-tag definitions — FR-1's retag conforms

§0 defines: "**Partial** — some limbs ship; the rest is named inline." FR-1 now reads "**Status:** Partial. Detection, ownership evidence, coherent replacement, and Finder/Dock launch all ship. The D40 install-hint extension … is Planned, owned by Story 2.5 … Named inline below." — and the limb *is* inline, inside the absence consequence, labeled "**Planned — D40 (Story 2.5)**". The `Planned — <decision>` limb-label form has in-document precedent (§9.2: FR-18 "now carries a Planned — D29 limb"; RP-2's "Planned — D27" limbs), so no new tag vocabulary was invented.

### 1.2 §9.2's Partial-never-Shipping rule — applied, and cited

§9.2 states: "A requirement whose consequences include a **Planned** limb is **Partial**, never Shipping — §0's definition admits no third reading." FR-1 gained a Planned consequence and was retagged Shipping → Partial in the same edit, and §9.3 Q2 names the mechanism explicitly: "FR-1 now carries the Planned limb and is retagged Partial under §9.2's rule." Conforms. (See finding F2 for the FR-9/FR-19 residue this rule leaves.)

### 1.3 Citation convention — no positional citations into the spine

§9.2's own rule: AD references are "cited by subject rather than by line, because this document's own line-number citations into the spine have already rotted once and the spine's Citations convention forbids them." Today's text contains exactly one spine reference — §9.3 Q4's "AD-18's per-field disclosure review" — cited by subject. Verified against the spine (`### AD-18 — [ADOPTED] Confirmed plan attempts have their own durable store`): "A story that adds a field to the attempt record owns its disclosure review" and "never ambient environment or user paths beyond what the existing entries already disclose." Accurate subject citation. The line-number citations that do appear today (`detect.rs`, and the proposal's `detect.rs:272-277`) point into code, which the convention does not forbid.

### 1.4 Requirement IDs never renumbered

FR-1 keeps its ID; no FR/NFR/RP was renumbered. Q1–Q4 keep their identities and map in order to D39–D42, and each of D39–D42 quotes back the same question sentence the old §9 list carried, verbatim ("Closes `prd.md` §9 Q1 ('Does a downloaded application update survive a relaunch that was not the update restart?')" etc.). Story 2.5 is a story ID, owned by `epics.md`, not a PRD requirement ID.

### 1.5 §9.1/§9.2 closure-recording form

§9.3 follows the established form: a dated, source-named section heading ("Closed by `docs/DECISIONS.md` D39–D42 (owner decisions, 2026-08-18)"), each entry opening with the bolded question + `CLOSED by <decision>: <answer>`, followed by grounding and where the consequences landed (FR-1/Story 2.5 for Q2, RP-1 unchanged for Q1, Story 6.5 for Q4). The §9 intro's replacement sentence correctly re-asserts the standing claim ("no phase-blockers") rather than dropping it. One deliberate asymmetry, correct as written: §9.3's preamble grounds its own authority in §0 verbatim — "a decision later than 2026-07-25 supersedes anything here" appears word-for-word in §0's Authority paragraph.

## 2. Factual grounding of every claim in the new text

| Claim (today's text) | Source | Ground |
| --- | --- | --- |
| Q1: payload "held in `AppUpdater.downloaded`, never on disk" | `src-tauri/src/app_update.rs:65` | `downloaded: Mutex<Option<Downloaded>>` where `type Downloaded = (Arc<dyn PendingRelease>, Vec<u8>)` (line 36); no filesystem write of the payload anywhere in the file |
| Q1: "a fresh process constructs at `Idle` with no restore path" | `app_update.rs:71–83` | `new()` sets `state: AppUpdateState::Idle`, `downloaded: Mutex::new(None)`; no load/restore code exists |
| Q1: "launch check and six-hour heartbeat while `autoCheckForUpdates` (default on) holds" | `app_update.rs:27`, `lib.rs:135`, `settings.rs:51` | `AUTO_CHECK_INTERVAL … from_secs(6 * 60 * 60)`; "Launch check + a 6h heartbeat, both gated on `autoCheckForUpdates`"; default `true` |
| Q1: "RP-1 is unchanged: the saved trigger policy still survives a normal relaunch" | `prd.md` RP-1 | RP-1 consequence "The saved trigger policy survives a normal relaunch" is untouched; "Ready" is RP-1's own vocabulary ("never presents as Ready") |
| Q2 / FR-1 limb: "today `detect.rs`'s `install_hint` returns one only for mas" | `src-tauri/src/detect.rs:273–278` | `match id { ManagerId::Mas => Some("brew install mas".into()), _ => None }` |
| Q2: six-card composition, disabled Update Everything, guidance panel never `Warning`, hints extended to all six, Install button rejected on four grounds | `docs/DECISIONS.md` D40 | All present, including the owner-modification paragraph and the four rejection grounds (installer non-goal §6, SM-3/FR-12, no shell surface, FR-23's set "may not grow") |
| Q2 / FR-1 limb: `CopyableCommand` treatment exists | `src/components/primitives/CopyableCommand.tsx` | exists; rendered from `Sidebar.tsx`, `ManagerCard.tsx`, `ManagerPane.tsx` per the proposal's evidence |
| Q2: "Realized by Story 2.5, added to Epic 2 by `sprint-change-proposal-2026-08-18.md`" | `epics.md:1272`, `epics.md:530`, proposal §4.1 | `### Story 2.5: Offer Copyable Install Guidance for Absent Managers`; retained-stories line updated; proposal exists and matches |
| Q2: `sprint-status.yaml` carries the key | `sprint-status.yaml:122` (+ note at :67) | `2-5-offer-copyable-install-guidance-for-absent-managers: backlog` |
| FR-1 limb / Q2: "FR-23's closed immediate-execution set" | `prd.md` FR-23 | "this is a member of a closed set … What the set may not do is **grow**" |
| FR-1 limb: "no-shell boundary", "SM-3's no-privilege promise", "installer non-goal" | `prd.md` §6, §8 | §6 "A package installer or uninstaller … Nothing else."; "No general shell surface"; SM-3 "Zero privilege prompts" |
| Q3: "automatic compaction to the newest 1,000 records" | `src-tauri/src/journal.rs:19` | `pub const COMPACT_KEEP: usize = 1000;` ("Compacted to the newest 1000 operations", line 6) |
| Q3: D41's no-delete answer, `operations.jsonl` out-of-band caveat, D29 immutability, SM-4 | `docs/DECISIONS.md` D41; `journal.rs:1` | all verbatim in D41; the journal file is named `operations.jsonl` |
| Q4: construction-time guarantee (closed allowlist, inherited env excluded, symlink substitution rejected) | `prd.md` FR-18, NFR-5; D42 | FR-18 "contains **only** environment values Pack-Manager itself set"; "Symlink substitution is rejected both when selecting files and when streaming them"; NFR-5 matches |
| Q4: "no transmit path (§6)" | `prd.md` §6 | "A telemetry client. Nothing is reported anywhere." |
| Q4: visible timestamped path + success/failure at both invocation points "is Story 6.5's acceptance criterion" | `epics.md` Story 6.5 AC-1 | "invocation from Settings and History … Then the timestamped ZIP path and visible success/failure match the contract" |
| §9.3 preamble: "adversarially verified against `HEAD` before the owner accepted" | proposal §1 | "closed as D39–D42 on 2026-08-18 after an adversarially verified recommendation pass"; "Evidence, verified against `HEAD` this run" |

No claim in today's text failed grounding. The FR-1 limb's three-ground summary of D40's Install-button rejection omits D40's fourth ground (FR-23) — a summarization, not an inaccuracy, and §9.3 Q2 carries all four.

## 3. Findings

### F1 (LOW) — §10's traceability sentence is now stale and today's edit did not touch it

`prd.md` §10 still opens: "Every requirement in this document traces to a named source — `docs/SPEC.md`, `docs/DECISIONS.md` **D1–D37**, …" and stamps code claims at `5972109`/`1ac959e`. As of today, FR-1's Planned limb traces to **D40** and §9.3 rests on D39–D42 — outside the stated range — and the new `detect.rs`/`app_update.rs` claims were verified against today's `HEAD`, not `1ac959e`. The claims themselves are true (verified above); the meta-claim about their provenance is what aged. Fix is one sentence: extend the decision range and add the 2026-08-18 verification baseline to §10.

### F2 (LOW) — §9.2's standing instruction to "the next Update" went unexecuted and unacknowledged

§9.2 records: "**FR-9 and FR-19 have the same shape and are not yet reconciled** … the next Update should make it deliberately." Today's pass is an Update (frontmatter `updated: 2026-08-18`), it applied that exact rule to FR-1, and it left FR-9 (Shipping over a `Planned — D30` consequence) and FR-19 untouched with no note that the deferral was renewed. The scope defense is real — the proposal routes this run as "`bmad-prd` Update citing D39–D42", and touching FR-9/FR-19 would exceed the directed scope — but the PRD now reads as if the promised "next Update" has not happened while its own frontmatter says it has. A one-line renewal in §9.2 or §9.3 (deferred again, still owed) would close the gap.

## 4. Explicitly checked and clean

- No positional line-number citation into `ARCHITECTURE-SPINE.md` anywhere in today's text.
- No requirement renumbering; no new ID namespace.
- FR-1's normative sentence correctly moved to target state ("with a copyable install hint", dropping "where one is known") with the gap carried by the status tag, exactly the Current-vs-Target discipline §0 defines.
- §9.3's four entries match D39–D42 one-to-one, including the owner-modification attribution on Q2 and the rejected-alternatives substance on all four.
- Story 2.5's contract text in `epics.md` matches the proposal's §4.1 blockquote, and the PRD's description of it (copy-only, existing `CopyableCommand` paths, all-absent panel, never `Warning`) matches the story's three ACs.
