---
name: Sprint Change Proposal — reconcile epics.md with ARCHITECTURE-SPINE revision 6
date: "2026-07-25"
project: Pack-Manager
workflow: bmad-correct-course
mode: Batch
status: awaiting-approval
trigger_type: upstream-authority-revision
scope_classification: Minor
requirements_authority_used: docs/SPEC.md + the FR/NFR inventory inside epics.md
prd_status: archived — not authoritative (project-context.md:135)
closes: "ARCHITECTURE-SPINE.md Decision Status row `epics.md` retired register — **Open**"
sources_read_this_session:
  - _bmad-output/planning-artifacts/epics.md
  - _bmad-output/planning-artifacts/architecture/architecture-Pack-Manager-2026-07-23/ARCHITECTURE-SPINE.md
  - _bmad-output/planning-artifacts/architecture/architecture-Pack-Manager-2026-07-23/DRIFT-NOTE.md
  - docs/DECISIONS.md
  - docs/RELEASE-CHECKLIST.md
  - _bmad-output/project-context.md
  - _bmad-output/implementation-artifacts/sprint-status.yaml
  - .claude/skills/bmad-correct-course/checklist.md
---

# Sprint Change Proposal — 2026-07-25

Reconcile `_bmad-output/planning-artifacts/epics.md` with `ARCHITECTURE-SPINE.md`
revision 6 and `docs/DECISIONS.md` D33.

---

## 1. Issue Summary

### Problem statement

`epics.md` still requires a body of scope that its upstream authorities retired.
The two documents now contradict each other, and `epics.md` is the file
`bmad-create-story` reads, so the contradiction propagates into the next story
unless it is reconciled first.

The spine records this against itself. `ARCHITECTURE-SPINE.md:648`:

> \| `epics.md` retired register \| **Open** \| `_bmad-output/planning-artifacts/epics.md` still carries TIR-1..TIR-8, RE-1..RE-11, the 72-criterion controls, and a set-equality requirement against `contracts/tauri-boundary/v1.json`. It contradicts this spine and `docs/DECISIONS.md` D33; reconciling it was out of scope for this run. See `DRIFT-NOTE.md`. \|

`DRIFT-NOTE.md:170-171` states which side wins:

> Until it is reconciled, `epics.md` and the spine disagree, and the spine
> is the correct one.

### Issue type

Not a technical limitation, a new requirement, or a failed approach. This is
**downstream propagation of a strategic pivot already ratified upstream**. D33
retired the gate on 2026-07-24; the spine caught up over revisions 4–6 on
2026-07-25; `epics.md` did not. `DRIFT-NOTE.md:169-170` says the omission was
deliberate and scoped:

> **This run was scoped to the spine plus this note, so the upstream
> reconciliation was recorded, not performed.**

### How it was discovered

Recorded by the architecture workflow as an Open item in the spine's own decision
table, not found during story execution. No story revealed it, because no story
has started — all 38 `sprint-status.yaml` entries are `backlog`.

### Evidence

Every line reference below was read this session.

| # | `epics.md` says | Verified reality |
| --- | --- | --- |
| 1 | `:150` — "AD-3 / ASR-01: … Accept by Batch 4 exit through exact set equality across the versioned `contracts/tauri-boundary/v1.json` catalog" | `ls -d contracts` → `No such file or directory`. `docs/DECISIONS.md:326` — the retired gate ran "against a `contracts/` directory that does not exist and that no story creates". `project-context.md:132` — "no scenario contract, no evidence manifest, and no `contracts/` directory — do not build one". `ARCHITECTURE-SPINE.md:637` records the catalog as **RETIRED**; `:161-162` replaces it: "There is no separate versioned boundary-catalog file and none is to be created." |
| 2 | `:104` — "Preserve exactly 72 P0 criteria from `readiness-coverage-map.md`, whose status remains `final-pending-approval`." | `docs/DECISIONS.md:331` — "Retired: the 72-criterion P0 gate, all P0/P1/overall coverage percentages, the 55 scenario contracts, the evidence-manifest and candidate-freeze machinery, the multi-host environment requirements, and Epics 7–8." The map file is archived: `_bmad-output/archive/2026-07-24-scope-recalibration/planning/prds/prd-Pack-Manager-2026-07-22/` — and `project-context.md:135` states "Nothing under `_bmad-output/archive/2026-07-24-scope-recalibration/` is authoritative". |
| 3 | `:123-130` — TIR-1 through TIR-8 | Specify the `forced-offline` / `provisioned-target-mac` / `candidate-release` evidence lanes retired by D33. `grep -c 'TIR-'` = 9 across the whole file: `:106` plus the eight definitions. **No surviving story cites a TIR id.** |
| 4 | `:134-144` — RE-1 through RE-11 | The Candidate Identity Manifest, Evidence Index, and Evidence Registrar apparatus. `grep -c 'RE-[0-9]'` = 11 — all eleven are the definitions themselves. **No surviving story cites an RE id.** `:137`, `:140`, `:141` additionally require "physical Intel", dropped by D32 (`docs/DECISIONS.md:294` — "What is dropped is the obligation to physically verify on Intel hardware"). |
| 5 | `:148-168` — AD-1 through AD-15 plus ASR-01..ASR-05 | This is the **retired gate's own AD numbering, which collides with the spine's**. `epics.md:148` AD-1 is a readiness-concern rule; `ARCHITECTURE-SPINE.md:112` AD-1 is "Dependencies point inward". `epics.md:162` AD-11 ends packaged acceptance "at the installed exact candidate"; `ARCHITECTURE-SPINE.md:234` AD-11 is "Release acceptance is the checklist plus two automated checks". Same ids, different meanings. `ARCHITECTURE-SPINE.md:636` — "AD-6..AD-10 and AD-13..AD-15 are retired ids and are never reused"; `:638` — the "ASR-01 / ASR-02 / ASR-03 enabler framing" is **RETIRED**. |
| 6 | `:155-159` — Candidate Identity Manifest v1, `contracts/readiness/v1/contract-lock.json`, the Evidence Registrar | All retired by D33 (`docs/DECISIONS.md:331-337`). A second non-existent `contracts/` path. |
| 7 | `:164` — "AD-13: Preserve the exact dependency waves: Batch 1 first; Batches 2/3/4 …" | `epics.md:373-374` already contradicts it in the same file: "The original evidence-batch ordering is void along with the gate that defined it, so the survivors carry no inter-epic dependencies." `grep -ci batch` = 18. |
| 8 | `:188-207` — Implementation-Entry Blocker Register, 16 rows | 11 of 16 rows are gate machinery. `:201` still carries `DEFERRED — BLOCKER` for "Evidence transport and retention duration", and `:203` requires "Apple-silicon and physical Intel hosts" — both retired. |

**One live, correctly-patched island inside the affected range:** `:175-178`
(DR-1 CLOSED by D31, DR-2 RESTATED by D33, DR-3 NARROWED by D32, DR-4 DISSOLVED
by D33). These are accurate and are the model this proposal copies.

---

## 2. Impact Analysis

### Epic impact — none

No epic is added, removed, renumbered, redefined, or resequenced.

- **Epic UX-PB (28 stories)** — the primary build queue. Substance untouched.
- **Epics 2, 3, 6 (6 stories: 2.2, 3.1, 3.2, 3.4, 3.5, 6.5)** — untouched. Read
  in full this session: none carries a `Primary concern` line, a criterion id, a
  batch reference, or an ASR reference. Their `Story Contract` blocks carry only
  `FR and requirement links`, `Required test level`, and `Dependencies`.
- **Epics 1, 4, 5, 7, 8** — already removed before this run (D33; applied
  2026-07-25).

`ARCHITECTURE-SPINE.md:639-640` is already consistent with this: Epic UX-PB is
**IN BUILD**, Epics 1–6 are **RESCOPED**.

### Story impact — no story's acceptance criteria change

The retired register is confined to `epics.md:100-207`. It is a requirements
preamble, not story text. Confirmed by grep: `TIR-`, `RE-[0-9]`, `Registrar`,
`Evidence Index`, `criterion`, and `Batch` appear **only** inside that range plus
seven incidental lines outside it (Change Set B below).

Two secondary story-adjacent findings, neither of which alters acceptance
criteria — see Change Sets C and D.

### Artifact conflicts

| Artifact | Impact | Action |
| --- | --- | --- |
| `epics.md` | The conflict. Six sections. | **Change Set A** — this proposal |
| `ARCHITECTURE-SPINE.md` | None. It is the correct side and is already `status: final`, revision 6, lint-clean. Its `:648` Open row becomes closable. | Close the Open row **via bmad-architecture Validate → Update**, resuming from `.memlog.md`. Not by hand. |
| `docs/DECISIONS.md` | None. D31/D32/D33 are the authority being applied. No new decision is needed — D33 already decided this; only propagation was outstanding. | No change |
| `docs/RELEASE-CHECKLIST.md` | None. It is what replaced the gate and is already written that way (`:3-5`). | No change |
| `DESIGN.md` / `EXPERIENCE.md` / `validation-report.md` | None. No UX component, flow, wireframe, interaction pattern, or accessibility requirement is touched. `epics.md:209-227` "UX Design Requirements" is outside the affected range and stays verbatim. | No change |
| `_bmad-output/project-context.md` | None. `:132` and `:135` already state the post-D33 position and were used as verification sources here. | No change |
| `sprint-status.yaml` | **None.** Read in full: 38 entries, no reference to any retired identifier. No epic or story is added, removed, or renumbered, so checklist item 6.4 is genuinely N/A. | **No change** |
| `story-triage-2026-07-24.md` | None. It is the per-story triage record D33 points at (`docs/DECISIONS.md:348-351`) and remains accurate. | No change |

### Technical impact — none

No code, test, config, workflow, or dependency change. One live test is named in
the affected file's neighbourhood (`AUT-003`, Change Set D) and its rewrite is
**already owned** by an existing acceptance criterion, `epics.md:411-413`.

---

## 3. Recommended Approach

### Selected path: Option 1 — Direct Adjustment

| Option | Verdict | Reasoning |
| --- | --- | --- |
| **1. Direct Adjustment** | **Viable — selected** | The contradiction is confined to a requirements preamble in one document. The correct replacement text already exists upstream and needs only to be pointed at. Effort **Low**, risk **Low**, timeline impact **none** — no story is blocked while this lands. |
| 2. Potential Rollback | **Not viable** | Nothing to roll back. All 38 `sprint-status.yaml` entries are `backlog`; no story file exists; no code was written against the retired register. |
| 3. PRD MVP Review | **Not viable — already performed** | D33 *was* the MVP review (`docs/DECISIONS.md:310-365`), approved 2026-07-24. Re-opening it would relitigate a settled decision. This proposal propagates that review's outcome; it does not revisit it. |

### Rationale

The spine wins by construction — it is upstream of `epics.md`, and
`DRIFT-NOTE.md:171` says so explicitly. So this is not a design question with
trade-offs; it is a propagation task with one correct answer. The only real
choice is **how much of the retired text to leave behind as a record**, and the
selected form (short retired-record stubs) matches how `epics.md:175-178`
already handles DR-1 through DR-4 — the one part of the affected range that was
patched correctly after D33.

Deleting the sections outright was rejected: a reader who encounters `TIR-3` or
`ASR-01` in git history, in the archive, or in the spine's own retired-id list
needs an in-document record of why it is gone. `docs/DECISIONS.md:363-365`
warns about the opposite failure — leaving retired documents in place "as
aspirational, since BMAD skill runs glob them back into the plan" — and a stub
that says *retired, do not rebuild* satisfies both concerns.

### Effort, risk, timeline

- **Effort:** Low. Six section replacements in one file, net reduction of roughly
  85 lines.
- **Risk:** Low. No acceptance criterion, dependency edge, or `sprint-status.yaml`
  entry changes in Change Set A.
- **Timeline:** No impact. Epic UX-PB is unblocked before and after.
- **Residual risk if declined:** the next `bmad-create-story` run reads
  `epics.md:150` and hands a story an instruction to build
  `contracts/tauri-boundary/v1.json` — a directory three separate authorities say
  must not exist.

---

## 4. Detailed Change Proposals

Four change sets, ordered by how directly the trigger forces them. **Change Set A
alone closes the spine's Open item.** B, C, and D are the same class of residue
found outside the six named sections; each can be approved or declined
independently.

Two global conventions applied throughout:

- **Retired AD/ASR/TIR/RE ids are named, never reused.** Consistent with
  `ARCHITECTURE-SPINE.md:636`.
- **All 28 `**Primary concern:** Product Behavior` story lines stay exactly as
  they are.** Only the retired three-way taxonomy that defined the *alternatives*
  is removed. The label remains true — every live story is product-behavior work.
  Zero edits inside the 28 stories in Change Set A.

---

### CHANGE SET A — the six named sections (`epics.md:102-207`)

`#### Product Acceptance Journeys` (`:112-120`, AJ-1..AJ-6) sits between sections
A1 and A2 and is **not** part of this change set. It is retained verbatim.

---

#### A1 — `#### Readiness and Scope Controls` (`:102-110`)

**Rationale:** the 72-criterion denominator, the coverage-map oracle, and the
three-way concern taxonomy are all retired by D33. One habit genuinely survives
(`docs/DECISIONS.md:344`) and must be carried forward, not dropped with the rest.

**OLD** (`:102-110`)

```markdown
#### Readiness and Scope Controls

- Preserve exactly 72 P0 criteria from `readiness-coverage-map.md`, whose status remains `final-pending-approval`.
- Preserve the historical planning baseline as FAIL with 14/72 FULL. Plan closure for all 58 non-FULL criteria and candidate-era revalidation of all 14 historical-FULL criteria at their mapped evidence depth; never carry a historical status forward automatically.
- Preserve the 58-row provisional concern split of 1 Product Behavior, 52 Reusable Test Infrastructure, and 5 Candidate-Specific Release Evidence, subject to TIR-1 behavior-present reclassification.
- Keep RP-1 and RP-2 mandatory but outside the denominator, baseline totals, concern totals, and batch counts.
- Do not promote criteria, approve or revise the coverage map, regenerate traceability, configure the gate, execute evidence, or claim product-and-release readiness in this planning artifact.
- Treat Product Behavior, Reusable Test Infrastructure, and Candidate-Specific Release Evidence as separate primary concerns. Every criterion-bearing story declares exactly one.
- For each of the 24 `BP` rows, check that required behavior is present before accepting regression work. Missing or incorrect behavior creates Product Behavior work and requires a reviewed map revision before regression evidence can receive credit.
```

**NEW**

```markdown
#### Scope Controls

The 72-criterion P0 readiness gate this section carried is retired by
`docs/DECISIONS.md` **D33**. There is no P0 denominator, no coverage percentage,
no `readiness-coverage-map.md` oracle, no criterion promotion, and no three-way
primary-concern taxonomy. Release readiness is `docs/RELEASE-CHECKLIST.md` plus
the two publication-blocking checks in `release.yml`. The retired artifacts are
archived under `_bmad-output/archive/2026-07-24-scope-recalibration/` and are
not authoritative.

One habit survives the gate, and it binds every story below: **before scheduling
work described as a test gap, verify whether the behavior is already present in
the shipping code.** `docs/DECISIONS.md` D33 records why — an adversarial pass
over the Epics 1-6 triage overturned 14 of 20 initial keep verdicts for exactly
that reason. `ARCHITECTURE-SPINE.md` AD-1 carries it as a rule.

Every live story is product-behavior work. The `Primary concern` label retained
on the 28 Epic UX-PB stories records that; the retired Reusable Test
Infrastructure and Candidate-Specific Release Evidence alternatives no longer
exist.

RP-1 and RP-2 remain mandatory requirements, validated through the release
checklist per the FR Coverage Map below. They no longer sit inside or outside any
denominator.
```

**Note:** heading renamed from `Readiness and Scope Controls` to `Scope Controls`
— "readiness" was the gate's word. Nothing in the repo links to the old anchor
(verified). Say the word and I will keep the original heading instead.

---

#### A2 — `#### Test Infrastructure Readiness` (`:121-130`)

**Rationale:** TIR-1..TIR-8 specify the three evidence lanes D33 retired. Three
pieces of substance survive, all of them already owned elsewhere — so the stub
points rather than restates, which is what keeps `epics.md` from drifting from
the spine again.

**OLD** (`:121-130`) — the heading plus all eight `- TIR-n: …` bullets, quoted in
full in the Evidence table above.

**NEW**

```markdown
#### Test Infrastructure

TIR-1 through TIR-8 are retired by `docs/DECISIONS.md` **D33**, together with the
evidence lanes they specified: the `forced-offline`, `provisioned-target-mac`,
and `candidate-release` lane separation, the source/environment/candidate
provenance depths, and the first-attempt/zero-automatic-retry admission terms.

Three obligations survive. Each is owned elsewhere and is deliberately not
restated here:

- **Determinism and offline defaults** — `ARCHITECTURE-SPINE.md`'s Determinism
  convention, and the testing rules in `_bmad-output/project-context.md`.
- **Real-versus-simulated honesty** — `ARCHITECTURE-SPINE.md` AD-3: the committed
  fixtures in `dev/fixtures/ipc/` prove payload shape on both sides and never
  dispatch anything through Tauri, so no story may claim event-delivery coverage
  from a fixture or from the browser double. The native Tauri harness is Deferred
  there, with Story 6.5 as its only live consumer.
- **Behavior-present verification** before scheduling a test gap — see Scope
  Controls above.
```

---

#### A3 — `#### Release Evidence Requirements` (`:132-144`)

**Rationale:** RE-1..RE-11 are the candidate-freeze and evidence-ledger
apparatus, retired wholesale. Three of them additionally require physical Intel
hardware, separately dropped by D32.

**OLD** (`:132-144`) — the heading plus all eleven `- RE-n: …` bullets.

**NEW**

```markdown
#### Release Acceptance

RE-1 through RE-11 are retired by `docs/DECISIONS.md` **D33**, together with the
Candidate Identity Manifest, the append-only Evidence Index, the Evidence
Registrar, `contracts/readiness/v1/contract-lock.json`, and the candidate-freeze
machinery. No `contracts/` directory exists and none is to be created.

Release acceptance is `docs/RELEASE-CHECKLIST.md` — a manual pass, not a computed
verdict or gate decision — plus two checks in `release.yml` that block
publication: the detached updater signature is base64-decoded and verified with
`minisign` against the public key the shipping app embeds, and the published
`latest.json` is asserted reachable and coherent after upload. Their failure
modes are silent and simultaneous across every installed client, which is why
they are automated rather than manual. `ARCHITECTURE-SPINE.md` AD-11 and AD-12
are the architectural statement of this.

The physical-Intel obligation in the former RE-4, RE-7, and RE-8 is separately
dropped by **D32**: the build stays universal and `latest.json` keeps publishing
both `darwin-aarch64` and `darwin-x86_64`, but verification is Apple silicon
only and Intel is best-effort and unverified.
```

---

#### A4 — `#### Architecture Invariants and ASR Enablers` (`:146-168`)

**This is the highest-risk section and the reason the reconciliation cannot
wait.** It publishes a *competing* AD numbering under the same ids the spine
uses. A story author who reads `epics.md` AD-3 and a story author who reads spine
AD-3 receive contradictory instructions, and one of the two is told to build a
directory that must not exist.

**OLD** (`:146-168`) — the heading plus all 21 bullets: AD-1, AD-2, AD-3/ASR-01,
AD-4/ASR-02, AD-5/ASR-03, AD-6/ASR-05, AD-7/ASR-04, the Candidate Identity
Manifest v1 shape, the `/v1` contract-lock freeze, the manifest artifact list,
AD-8, the Registrar enforcement list, AD-9, AD-10, AD-11, AD-12, AD-13, AD-14,
AD-15 (already marked VOID), the attempt-ordinal rule, and the PASS-admission
rule.

**NEW**

```markdown
#### Architecture Invariants

`_bmad-output/planning-artifacts/architecture/architecture-Pack-Manager-2026-07-23/ARCHITECTURE-SPINE.md`
is the **single authority** for architecture invariants. Stories cite its AD ids
and no others. This section does not restate them, so that the two documents
cannot drift apart again.

The AD-1..AD-15 list this section previously carried was the **retired gate's own
numbering, and it is not the spine's.** It is removed rather than renumbered,
because the two schemes collide on the same ids with different meanings:

| Retired id here | Said | `ARCHITECTURE-SPINE.md` says under that id |
| --- | --- | --- |
| AD-1 | Each work item has one primary readiness concern | Dependencies point inward; test and release tooling are never product dependencies |
| AD-3 / ASR-01 | Exact set equality across the versioned `contracts/tauri-boundary/v1.json` catalog | The IPC surface changes atomically, proven by the committed fixtures in `dev/fixtures/ipc/`. "There is no separate versioned boundary-catalog file and none is to be created." |
| AD-11 | Packaged acceptance ends at the installed exact candidate | Release acceptance is the checklist plus two automated checks |
| AD-13 | Preserve the exact Batch 1-8 dependency waves | (no such rule; the batch ordering is void) |

`ARCHITECTURE-SPINE.md` records **AD-6..AD-10 and AD-13..AD-15 as retired ids
that are never reused.** The `ASR-01` / `ASR-02` / `ASR-03` enabler framing is
retired with the register that defined it; the spine states that the surviving
obligations are its **AD-2, AD-3, AD-4, and AD-5**.

The Candidate Identity Manifest v1 JCS shape, the `contracts/readiness/v1/contract-lock.json`
freeze, the Evidence Registrar append rules, the attempt-ordinal terms, and the
PASS-admission counters are retired by `docs/DECISIONS.md` **D33**.
```

**Note:** heading renamed to `Architecture Invariants` — "ASR Enablers" is the
retired framing (`ARCHITECTURE-SPINE.md:638`).

---

#### A5 — `#### Governance, Risks, and Entry Blockers` (`:170-186`)

**Rationale:** mixed section. Most is gate governance; but **R-001..R-008 are
real open risks that D33 did not retire** — `docs/DECISIONS.md:331-337` lists
what was retired and risks are not in it. They are preserved verbatim and
explicitly marked as not-retired, so a later reader does not assume they went
with the gate. `:175-178` (DR-1..DR-4) is already correct and is preserved
verbatim.

**OLD** (`:170-186`) — the heading, PC-1, GP-1, GP-2, the four DR lines, the
R-001..R-008 line, and the six trailing governance bullets (`:180-186`).

**NEW**

```markdown
#### Governance and Risks

GP-1, GP-2, PC-1, the coverage-map approval requirement, the evidence transport
and retention choice, the provisioned-target-Mac and multi-host environment
requirements, the per-story criterion-authoring rules, and the candidate-bound
story rules are retired by `docs/DECISIONS.md` **D33**.

PC-1's substance is closed independently of the gate: **D23a** withdrew the `mas`
UNVERIFIED label and recorded `mas` as verified live, and the current production
surface is 20 commands and six events, so the obsolete five-event invariant no
longer absorbs application-update state. One documented residual remains —
`src-tauri/tests/live_smoke.rs` still declares this machine as "mas absent".
`_bmad-output/project-context.md` records that precondition as stale rather than
as evidence of a code defect.

DR-1 is CLOSED by `docs/DECISIONS.md` **D31**: the minimum supported macOS version is 15.0, declared as `bundle.macOS.minimumSystemVersion` in `src-tauri/tauri.conf.json` and shipped in v1.0.0. Nothing is blocked on it.

DR-2 is RESTATED by **D33**: its substance survives without the gate framing. Automated 4.5:1 contrast and reduced-motion checks belong in the existing Playwright/Vitest lane; one manual VoiceOver pass joins `docs/RELEASE-CHECKLIST.md`. Accessibility here is product quality, not evidence ceremony. Neither automated check exists yet, so this is an obligation on whichever story adds them, not a description of current coverage.

DR-3 is NARROWED by **D32**: the release still builds universal, but the obligation to verify on physical Intel hardware is dropped. Verification is Apple silicon only; Intel remains best-effort and unverified.

DR-4 is DISSOLVED by **D33** along with the gate that defined it. There is no P0/P1 threshold, no Acceptance Profile, and no gate decision. Release readiness is `docs/RELEASE-CHECKLIST.md` plus the automated updater-signature and published-endpoint checks in `release.yml`.

**R-001 through R-008 are NOT retired.** D33 retired the gate, not the risks.
They remain open high risks: source/oracle drift (6), fake/native boundary gap
(6), misleading UI state (6), process lifecycle uncertainty (6),
persistence/diagnostics failure (6), updater integrity failure (6), invalid
shipped artifact (9), and environmental dependency/contamination (6). No
mitigation is complete, waived, or accepted.

Secrets stay in fnox locally and GitHub Secrets in CI and never enter build
artifacts, manifests, or documentation. Apple Developer ID signing and
notarization are required for a published release, and updater signing is
required by the build; `ARCHITECTURE-SPINE.md` AD-12 owns this.
```

---

#### A6 — `#### Implementation-Entry Blocker Register` (`:188-207`)

**Rationale:** 11 of the 16 rows are gate machinery. The `Deadline boundary`
column is dropped entirely because every value in it was a batch boundary, and
`epics.md:373` already declares the batch ordering void. Five rows survive in
substance, reworded to the spine's framing.

**OLD** (`:188-207`) — the heading plus the five-column, 16-row table.

**NEW**

```markdown
#### Implementation-Entry Register

The `Deadline boundary` column this table carried is removed: every value in it
was a Batch 1-8 boundary, and the evidence-batch ordering is void along with the
gate that defined it. The six surviving Epic 1-6 stories carry no inter-epic
dependencies.

| Decision or dependency | Current state | Accountable role | Effect on implementation entry |
| --- | --- | --- | --- |
| Product Behavior Prerequisite UX-PB.1..UX-PB.5 | `APPROVED TARGET — NOT IMPLEMENTED` | Product/UX/Architecture accept; Development implements | Nothing is blocked from starting — Epic UX-PB is the primary build queue and runs first. Any story or test text authored against immediate row execution, direct self-update execution, the Activity drawer, Operation-row History, or active `autoOpenDrawer` behavior is superseded by D27-D30. |
| DR-1 — minimum supported macOS | `CLOSED` — D31 | Resolved 2026-07-24 | None. 15.0 declared and shipped in v1.0.0. Whether `notarytool` accepts `minos 15.0` against the CI SDK is OPEN and is settled by a manual Release run, never by assertion. |
| DR-2 — packaged accessibility method | `RESTATED` — D33 | Existing Playwright/Vitest lane + release checklist | None. An obligation on whichever story adds the two automated checks, which do not exist yet. |
| DR-3 — physical Intel requirement | `NARROWED` — D32 | Resolved 2026-07-24 | None. Universal build retained; verification Apple silicon only. |
| DR-4 — P0 gate/retry policy | `DISSOLVED` — D33 | Retired with the gate | None. |
| Named assignees and calendar dates | `REMOVED` — D33 | n/a | None. The `Assignee` and `Calendar date` fields were removed from every surviving story on 2026-07-25. |
| Native Tauri E2E harness and runner | `DEFERRED` | Architecture accepts; Development implements | Story 6.5's "Real native Tauri E2E plus artifact inspection" test level is its only live consumer. Any choice must satisfy `ARCHITECTURE-SPINE.md` AD-2 and AD-3. |
| Controlled child-helper language | `DEFERRED` | Development | No live story requires one. Any choice must satisfy AD-4 and cannot add a production shell-command surface. |
| Plan-attempt journal filename and serde shape | `DEFERRED` | Development | Owned by Story UX-PB.2c. AD-18 fixes ownership, location, durability, and failure mode; the exact filename and field list are the story's. |

Every other row this register carried is retired by `docs/DECISIONS.md` **D33**:
normative coverage-map approval, evidence transport and retention duration, the
provisioned target Mac and versioned profile, Apple-silicon and physical Intel
hosts, the actually-installed prior public version as a gate dependency, candidate
credentials as a freeze precondition, evidence/profile approval records with
versioned scenario digests, and the one immutable candidate with all required
artifacts. The prior-version update check and the signing credentials survive as
release-checklist steps and AD-12 obligations respectively — not as entry
blockers.
```

**Net effect of Change Set A:** 39 retired identifiers stop being requirements —
TIR-1..TIR-8 (8), RE-1..RE-11 (11), and the register's own AD-1..AD-15 plus
ASR-01..ASR-05 (20) — along with the 72-criterion controls and 11 of the 16
register rows. `epics.md` gains no requirement that any other authority
contradicts. Zero story edits.

Line count goes slightly **up**, not down: the six sections are 92 lines today,
because the retired bullets are dense single-line paragraphs (`:150` alone is one
504-character line), and the replacement stubs are wrapped prose. The reduction
is in obligations, not bytes.

---

### CHANGE SET B — forced residuals outside the six sections

Same retirement, seven places it leaked past the section boundaries. These are
factual defects, not stylistic ones.

**B1 — frontmatter `inputDocuments` (`:8-19`): all eight paths are stale.**
Verified by existence check — every one is `MISSING` at the path given, because
all eight moved into the archive:

| `:line` | Path as written | Status |
| --- | --- | --- |
| 8 | `_bmad-output/planning-artifacts/prds/prd-Pack-Manager-2026-07-22/prd.md` | MISSING → archived |
| 9 | `…/prd-Pack-Manager-2026-07-22/addendum.md` | MISSING → archived |
| 11 | `_bmad-output/test-artifacts/test-design-architecture.md` | MISSING → archived |
| 12 | `_bmad-output/test-artifacts/test-design-qa.md` | MISSING → archived |
| 13 | `_bmad-output/test-artifacts/test-design-progress.md` | MISSING → archived |
| 14 | `_bmad-output/test-artifacts/test-design/Pack-Manager-handoff.md` | MISSING → archived |
| 15 | `…/prd-Pack-Manager-2026-07-22/readiness-coverage-map.md` | MISSING → archived (and retired) |
| 19 | `_bmad-output/planning-artifacts/sprint-change-proposal-2026-07-24.md` | MISSING → archived |

**Proposed:** repoint each to its actual `_bmad-output/archive/2026-07-24-scope-recalibration/…`
location and add a comment marking them historical inputs, non-authoritative per
`project-context.md:135`. Add `docs/RELEASE-CHECKLIST.md` and this proposal.
**Do not** move any file back — `project-context.md:135` warns that BMAD skills
glob `*prd*.md` and `*epic*/*.md` from `planning-artifacts` and would silently
reload the retired gate.

**B2 — `:28` Overview** cites "reconciled System-Level Test Design, normative
readiness mapping". Both retired. **Proposed:** drop those two clauses; keep the
rest of the sentence and the 2026-07-24 amendment note verbatim.

**B3 — `:78` (RP-1) and `:80` (RP-2)** both end "outside the 72-row P0
denominator". **Proposed:** replace with "as a mandatory prerequisite validated
through `docs/RELEASE-CHECKLIST.md`". These are FR-inventory lines, so the
requirement itself is untouched — only the dead denominator reference goes.

**B4 — `:258`** "do not create a ninth readiness batch". **Proposed:** delete the
clause; there are no batches.

**B5 — `:274`** "Exact P0 criterion ownership is governed separately by the
72-row story allocation." **Proposed:** delete the sentence.

**B6 — `:331`** "No epic, infrastructure result, or evidence plan changes a
criterion status." **Proposed:** delete the sentence.

**B7 — `:295` (FR-8 coverage)** cites "Stories 3.3, 3.6" and "Story 5.8" as the
mechanism. All three are archived. The clause it quotes — "stale, altered,
replayed, evicted, or conflicting plans enqueue nothing" — is live and is carried
by UX-PB.2a, which the same line already names. **Proposed:** reattribute to
UX-PB.2a and UX-PB.2b and drop the three archived story numbers.

---

### CHANGE SET C — dangling `Blocks:` edges inside the 28 UX-PB stories

**This is the only change set that edits the 28 stories, and only their
`**Blocks:**` metadata lines — never a heading, a user story, or an acceptance
criterion.** Flagged separately for exactly that reason; decline it and Change
Set A still stands on its own.

**Why it matters:** `bmad-create-story` reads these lines. Eleven of them name
stories that no longer exist in `epics.md` or in `sprint-status.yaml`, and
thirteen use the phrase "and its affected evidence" — evidence stories died with
D33.

| `:line` | Story | `Blocks:` as written | Problem |
| --- | --- | --- | --- |
| 388 | UX-PB.1a | `UX-PB.1b, UX-PB.1c; Story 3.5 and its affected evidence` | 3.5 survives; "affected evidence" is retired |
| 445 | UX-PB.1c | `UX-PB.1d, UX-PB.1e; Stories 3.3 and 3.6 and their affected evidence` | **3.3 and 3.6 both archived** |
| 471 | UX-PB.1d | `Story 3.2 and its affected evidence` | 3.2 survives; phrase retired |
| 494 | UX-PB.1e | `Stories 3.1 and 5.2 and their affected evidence` | **5.2 archived**; 3.1 survives |
| 517 | UX-PB.2a | `UX-PB.2b, UX-PB.2f; Story 4.1` | **4.1 archived** |
| 536 | UX-PB.2b | `UX-PB.2c, UX-PB.2d, UX-PB.2e; Story 4.6` | **4.6 archived** |
| 583 | UX-PB.2d | `UX-PB.2e; Story 6.3` | **6.3 archived** |
| 623 | UX-PB.2f | `Story 6.4` | **6.4 archived** |
| 699 | UX-PB.3d | `UX-PB.3e, UX-PB.3g; Stories 5.4, 6.5` | **5.4 archived**; 6.5 survives |
| 725 | UX-PB.3e | `UX-PB.4 and its affected evidence` | `UX-PB.4` is not a story id — the stories are UX-PB.4a-4e |
| 743 | UX-PB.3f | `UX-PB.4 and its affected evidence` | same |
| 765 | UX-PB.3g | `Story 5.5 and its affected evidence; UX-PB.4` | **5.5 archived**; plus both problems above |
| 787 | UX-PB.4a | `UX-PB.4b, UX-PB.4e; Story 6.3 and its affected evidence` | **6.3 archived** |
| 810 | UX-PB.4b | `UX-PB.4c, UX-PB.4d; Story 6.4 and its affected evidence` | **6.4 archived** |
| 847 | UX-PB.4d | `Story 6.5 and its affected evidence` | 6.5 survives; phrase retired |
| 917 | UX-PB.5b | `UX-PB.5c; Stories 3.4 and 6.7 and their affected evidence` | **6.7 archived**; 3.4 survives |

**Proposed rule, applied mechanically — three operations, nothing else:**

1. Drop every archived story number: 3.3, 3.6, 4.1, 4.6, 5.2, 5.4, 5.5, 6.3, 6.4,
   6.7.
2. Delete the phrase "and its/their affected evidence" (13 occurrences).
3. Expand bare `UX-PB.4` to `UX-PB.4a`, and `UX-PB.3`/`UX-PB.2` similarly where
   they appear bare in `Dependencies:` lines.

Surviving story numbers (3.1, 3.2, 3.4, 3.5, 6.5) are **kept**. Where a line
would become empty, it reads `None`. Example:

```
OLD  **Blocks:** UX-PB.1d, UX-PB.1e; Stories 3.3 and 3.6 and their affected evidence
NEW  **Blocks:** UX-PB.1d, UX-PB.1e

OLD  **Blocks:** UX-PB.3e, UX-PB.3g; Stories 5.4, 6.5
NEW  **Blocks:** UX-PB.3e, UX-PB.3g; Story 6.5
```

---

### CHANGE SET D — the 2026-07-24 amendment tables (`:229-270`)

`:234-242` is a supersession table whose seven rows point at story areas: rows
for **Stories 4.1/4.6, 5.2/5.4/5.5, 6.3-6.5/6.7, 7.6/7.7/7.10, and 8.7** — every
one of those except 6.5 is archived or belongs to a removed epic.

`:265-266` already labels the table correctly: "The amendment table above is
retained as the revision record of the prior wording." So it is honest history,
not a live instruction.

**Proposed (minimal):** add one sentence marking it explicitly historical and
noting that the rows naming Epics 4, 5, 7, 8 and Stories 3.3/3.6/6.3/6.4/6.7
address stories archived on 2026-07-25. **Do not rewrite the table** — it is a
revision record and rewriting it would destroy what it records.

**One live item found inside it, deliberately left alone.** `:244-245`:

> `AUT-003` is retained as historical evidence of superseded behavior and must
> not support revised `F5-AC3`.

`AUT-003` is a **real, currently-committed test**:
`tests/e2e/upgrade-journeys.spec.ts:169` — `test("[P0] AUT-003 executes a one-row
Upgrade immediately without a plan dialog"`. Its rewrite is already owned by an
existing acceptance criterion, `epics.md:411-413` (UX-PB.1a): "that assertion is
rewritten to expect draft membership with nothing executing, because Decision D27
supersedes the behavior it encodes." `F5-AC3` returns zero matches anywhere in
the repo outside this line. **No change proposed** — the live half is correctly
owned by a story, and the dead half is inside a historical record.

---

## 5. Implementation Handoff

### Scope classification: **Minor**

No code, no test, no config. No epic or story added, removed, renumbered, or
resequenced. No `sprint-status.yaml` change. No backlog reorganization, so this
is not Moderate; no replan, since D33 already decided the substance, so it is not
Major.

### Handoff

| Deliverable | Recipient | Notes |
| --- | --- | --- |
| Change Sets A-D against `epics.md` | Developer agent — direct implementation | Single-file edit. Approved sets only. |
| Close `ARCHITECTURE-SPINE.md:648` — the `epics.md` retired register **Open** row | **`bmad-architecture` Validate → Update** | **Not a hand edit.** The spine is `status: final`, revision 6, lint-clean. Resume from the run folder's append-only `.memlog.md` (84 entries) and let the workflow write revision 7 and extend `DRIFT-NOTE.md`. |
| `sprint-status.yaml` | No one | Verified unchanged. Checklist item 6.4 is N/A. |

### Success criteria

1. `grep -nE 'TIR-[0-9]|RE-[0-9]|ASR-0|Registrar|Evidence Index|contracts/' epics.md`
   returns only lines that explicitly mark those identifiers **retired**.
2. `grep -n 'contracts/tauri-boundary' epics.md` returns **zero** matches.
3. No `AD-n` id in `epics.md` asserts a rule that differs from the spine's rule
   under that same id.
4. `grep -c 'Primary concern' epics.md` still returns **28**.
5. The six surviving Epic 1-6 stories (2.2, 3.1, 3.2, 3.4, 3.5, 6.5) and all 28
   UX-PB acceptance criteria are byte-identical to their pre-change text, except
   for the `Blocks:` lines in Change Set C if approved.
6. `sprint-status.yaml` is byte-identical.
7. Every `Blocks:`/`Dependencies:` story reference in `epics.md` resolves to a
   story that exists in `sprint-status.yaml` (Change Set C only).
8. `ARCHITECTURE-SPINE.md`'s Decision Status table no longer carries an Open row
   for the `epics.md` register.

---

## 6. Change Navigation Checklist — recorded status

### Section 1 — Understand the Trigger and Context

- **1.1 Triggering story** — **[N/A]** No story revealed this. The trigger is an
  upstream document revision, self-recorded at `ARCHITECTURE-SPINE.md:648` as an
  Open item. All 38 `sprint-status.yaml` entries are `backlog`; no story file
  exists yet.
- **1.2 Core problem defined** — **[x] Done** Type: downstream propagation of an
  already-ratified strategic pivot (D33, 2026-07-24). Not a technical limitation,
  new requirement, misunderstanding, or failed approach.
- **1.3 Impact and evidence** — **[x] Done** Eight line-cited contradictions, §1.
  Independently verified against `ls -d contracts`, `docs/DECISIONS.md`,
  `project-context.md`, and existence checks on all eight frontmatter paths.

### Section 2 — Epic Impact Assessment

- **2.1 Current epic completable as planned** — **[x] Done** Yes. Epic UX-PB is
  unaffected in substance and stays the primary build queue.
- **2.2 Epic-level changes required** — **[x] Done** **None.** No epic modified,
  added, removed, or redefined by this proposal.
- **2.3 Remaining epics reviewed** — **[x] Done** Epics 2, 3, 6 read in full. All
  six surviving stories are clean of retired framing.
- **2.4 Invalidates or necessitates epics** — **[N/A]** Epics 1, 4, 5, 7, 8 were
  already removed before this run.
- **2.5 Order or priority change** — **[N/A]** UX-PB stays first; survivors carry
  no inter-epic dependencies (`epics.md:374`).

### Section 3 — Artifact Conflict and Impact Analysis

- **3.1 PRD conflicts** — **[!] Action-needed** The PRD is archived and
  non-authoritative. Per your decision this run, `docs/SPEC.md` plus the FR/NFR
  inventory inside `epics.md` served as the requirements authority. **No FR or
  NFR changes.** MVP unaffected — D33 already performed the MVP review. Standing
  item: this workflow's HALT condition will recur on every future
  `bmad-correct-course` run until a live PRD exists or the skill is customized.
- **3.2 Architecture conflicts** — **[x] Done** The spine is the correct side and
  needs no content change. Only its Open row closes, and via Validate → Update.
- **3.3 UI/UX conflicts** — **[x] Done** **None.** No component, flow, wireframe,
  interaction pattern, or accessibility requirement touched. `epics.md:209-227`
  retained verbatim.
- **3.4 Other artifacts** — **[x] Done** `sprint-status.yaml` unchanged;
  `DECISIONS.md` unchanged; `RELEASE-CHECKLIST.md` unchanged; CI/CD untouched.
  One live test (`AUT-003`) identified, already owned by `epics.md:411-413`.

### Section 4 — Path Forward Evaluation

- **4.1 Direct Adjustment** — **[x] Viable** Effort Low, risk Low.
- **4.2 Potential Rollback** — **[ ] Not viable** Nothing implemented to revert.
- **4.3 PRD MVP Review** — **[ ] Not viable** Already performed as D33.
- **4.4 Selected path** — **[x] Done** Option 1, Direct Adjustment. Rationale §3.

### Section 5 — Sprint Change Proposal Components

- **5.1** Issue summary — **[x] Done** (§1)
- **5.2** Epic and artifact impact — **[x] Done** (§2)
- **5.3** Recommended path with rationale — **[x] Done** (§3)
- **5.4** MVP impact and action plan — **[x] Done** MVP unaffected; §4 is the
  action plan, sequenced A → B → C → D → spine closure.
- **5.5** Agent handoff plan — **[x] Done** (§5)

### Section 6 — Final Review and Handoff

- **6.1** Checklist completion reviewed — **[x] Done**
- **6.2** Proposal accuracy verified — **[x] Done** Every factual claim carries a
  `path:line` citation to a file read this session; counts come from `grep -c`,
  `wc -l`, and existence checks.
- **6.3** Explicit user approval — **[ ] Pending** — §7
- **6.4** `sprint-status.yaml` updated — **[N/A]** No epic or story added,
  removed, or renumbered. Verified: 38 entries, no retired-identifier reference.
- **6.5** Next steps and handoff confirmed — **[ ] Pending approval**

---

## 7. Approval

Awaiting explicit approval. Change Set A is the core and closes the spine's Open
item on its own; B, C, and D can each be approved or declined independently.

| Set | Scope | Touches the 28 UX-PB stories? | Recommendation |
| --- | --- | --- | --- |
| **A** | The six named sections, `:102-207` | No | **Approve** — this is the ask |
| **B** | 7 forced residuals outside those sections | No | **Approve** — factual defects, incl. 8 broken paths |
| **C** | 16 `Blocks:` metadata lines | Metadata lines only, no criteria | **Approve** — `bmad-create-story` reads these |
| **D** | 1 sentence marking the amendment table historical | No | **Approve** — minimal; table preserved |
