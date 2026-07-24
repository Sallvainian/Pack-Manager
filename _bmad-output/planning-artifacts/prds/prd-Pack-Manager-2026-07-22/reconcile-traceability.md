# Reconciliation: Traceability Matrix vs. P0 Readiness PRD

**Inputs**

- `_bmad-output/test-artifacts/traceability-matrix.md`
- `_bmad-output/planning-artifacts/prds/prd-Pack-Manager-2026-07-22/prd.md`

**Result:** The PRD faithfully carries forward the matrix's current `FAIL`
decision and coverage arithmetic. No material wrong-lane classification was
found. Three material gaps remain in the handoff from the old oracle to the new
PRD oracle.

## Reconciled baseline

| Check | Matrix | PRD | Reconciliation |
| --- | --- | --- | --- |
| Current gate | `FAIL` | `FAIL` | Match |
| Criteria | 80 total: 72 P0, 8 P1 | Uses the same 72-P0/8-P1 baseline | Match |
| P0 FULL | 14/72 (19.4%) | 14/72 (19.4%) | Match |
| Open P0 | 58 | 58 | Match |
| P1 FULL | 0/8 | 0/8 | Match |
| Matrix status distribution | P0: 14 FULL, 35 PARTIAL, 16 UNIT-ONLY, 2 INTEGRATION-ONLY, 5 NONE | Not repeated in full | No contradiction |
| Lane distribution | Not encoded in the matrix itself | A = 1, B = 52, C = 5 | Arithmetically valid |

The lane semantics are also sound:

- **A — product behavior/correctness:** only `D23a-AC4`, the one confirmed
  repository-truth defect.
- **B — test infrastructure/coverage:** 52 criteria whose required behavior is
  not yet proven at the material boundary. The PRD correctly requires a
  behavior-present check and reclassification to A if that check finds a
  defect.
- **C — release evidence/operational proof:** `F10-AC2`, `F10-AC3`,
  `F10-AC4`, `D25A-AC2`, and `F12-AC1`.

The five Category C rows must not be confused with the matrix's five
coverage-`NONE` P0 blockers. The latter are `D23a-AC4` (A), `F1-AC7` (B), and
`F10-AC2` through `F10-AC4` (C). All 58 non-FULL P0 rows block a strict
72/72-P0 decision; the five `NONE` rows are merely the most acute subset.

The PRD's P1 caveat is accurate. The matrix independently requires at least
80% strict-FULL P1 coverage, while the current result is 0/8. Therefore
72/72 P0 FULL alone cannot make the unchanged matrix gate pass. The PRD
correctly requires either satisfaction of that P1 policy or an explicit policy
change, without silently adding P1 product features to this P0 closure.

## Material gaps

### 1. The new PRD has not yet become the traced oracle

The matrix's recorded inputs are `docs/SPEC.md`, `docs/DECISIONS.md`, project
context, the command-hardening spec, and the automation summary. It explicitly
says that no separate PRD was found. The PRD now gives itself highest source
authority, but no trace run has consumed it.

Consequently, the current matrix is a valid planning baseline, not evidence
that the new FR/TIR/RE/NFR model has been traced. A new trace must use the PRD
as the formal oracle before any candidate-bound decision.

### 2. The PRD lacks an auditable criterion crosswalk

The matrix gates 72 atomic P0 IDs, while the PRD reorganizes the scope into 22
FRs, 8 TIRs, 12 REs, 8 NFRs, and 6 acceptance journeys. The PRD states that
72/72 validates this model but does not map each old atomic ID to the new
requirement or identify which PRD statements remain rollups rather than new
counted criteria.

Without a normative crosswalk, a future trace can legitimately decompose the
PRD into a different denominator, and the 1/52/5 lane totals cannot be audited
from the two principal documents alone. Preserve an ID-level mapping as part
of the PRD gate package or make the existing gap-classification extract an
explicit normative companion.

### 3. The 14 inherited FULL rows do not yet satisfy the PRD's stronger evidence contract

The matrix assigned FULL from executable mappings at an appropriate level, but
its source snapshot is a commit plus a dirty working tree; discovery in the
trace run did not execute the suites; green results came from a prior
automation record; and no immutable release candidate was evaluated. The PRD
additionally requires source/candidate provenance, every material boundary,
exclusion of ignored or merely collected checks, and preservation of the first
failure across retries.

Accordingly, 14/72 is the honest inherited baseline, not 14 candidate-bound
closures under the new definition of FULL. The final trace must reassess all
72 rows—including the currently FULL rows—against the exact clean candidate
and retained Evidence Set.

## Release disposition

The reconciliation does not soften the result: release readiness remains
`FAIL`. Closure requires all 58 open P0 rows, revalidation of the 14 inherited
FULL rows under the PRD evidence rules, resolution of the independent P1 gate
policy, and a regenerated trace tied to one immutable candidate. Green
Vitest/Rust/Playwright records remain supporting evidence only: Playwright uses
fake Tauri IPC, 11 live/environment tests were ignored, the real JS-to-Rust and
packaged WKWebView boundaries were not crossed, and release assets,
installation, relaunch, signing, notarization, stapling, and updater integrity
were not candidate-verified.
