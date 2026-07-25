# Source reconciliation — test-design input group

## Inputs

- `_bmad-output/test-artifacts/test-design-architecture.md`
- `_bmad-output/test-artifacts/test-design-progress.md`
- `_bmad-output/test-artifacts/test-design-qa.md`
- `_bmad-output/test-artifacts/test-design/pack-manager-handoff.md`
- Reconciled against: `prd.md`

## Result

The PRD correctly preserves the core plan-versus-proof rule: designs and
collectors are not evidence, native claims require native execution, and
release claims require candidate-bound proof. Five material reconciliation
gaps remain.

### 1. The exact eight-batch dependency plan and ASR ownership gate are not
carried into governance

The input group defines eight mutually exhaustive batches covering the 58 open
P0 criteria and an explicit dependency wave:

1. Batch 1 first;
2. Batches 2, 3, and 4 in parallel;
3. Batches 5 and 6 only after the Batch 4 native foundation;
4. Batch 7 against an immutable signed candidate; and
5. Batch 8 only after Batches 1–7, followed by trace regeneration.

It also says owners and delivery batches for ASR-01 through ASR-05 must be
approved before implementation planning proceeds. The PRD compresses this into
a five-step closure sequence and names capabilities, but does not preserve the
58-criterion batch mapping, the parallel/dependent wave contract, or the
architecture-review decision to assign accountable owners. Generic references
to Product, QA, Engineering, and Release ownership do not assign the five
blocking enablers.

**Reconciliation needed:** make the test-design eight-batch handoff the
normative closure decomposition referenced by §9.3, and add approved owners for
the native harness, controlled OS/process boundary, lifecycle harness,
candidate attestation, and split evidence lanes as an implementation-entry
condition. This need not copy all scenarios into the PRD.

### 2. The known D23a defect crosses into the test-infrastructure lane

All four test-design artifacts classify `D23a-AC4` as the single
product/repository defect: current truth must be corrected before a regression
guard can count. PRD TIR-1 places “the stale D23a/`mas` claims are corrected”
inside Test Infrastructure even though the PRD's own lane model says missing or
incorrect behavior belongs in the product lane.

**Reconciliation needed:** place the truth correction and its acceptance in
the product/source-correctness lane. TIR-1 should retain only the
behavior-present classification check and recurrence guard.

### 3. Updater closure is incomplete in both directions

The test-design group requires installation to be refused while Package
Operations are active before the updater may install and relaunch. PRD FR-21
omits that user-safety outcome, so the planned D25 acceptance test has no
corresponding product requirement.

Conversely, PRD RE-8 requires an actually installed prior public version to
discover, download, install, and relaunch into the candidate. The test-design
group asks for a controlled feed and immutable signed candidate but never makes
an installed prior public version a blocker or an explicit Batch 7/8 exit
artifact. A synthetic/current-to-candidate updater run could therefore satisfy
the plan while leaving RE-8 unproven.

**Reconciliation needed:** add the active-Operation install guard to FR-21 and
add a supplied, actually installed prior public version plus retained
before/after evidence to the test-design dependency and release exit contract.

### 4. The configured P1 gate-policy dependency is absent from the test-design
exit model

The test-design documents repeatedly state that 72/72 P0 FULL plus completed
high-risk mitigations is the final release gate; their only P1 rule is a 95%
pass rate for any P1 regression tests that happen to run. PRD RE-12 records a
different live governance dependency: the configured matrix requires 80%
strict-FULL P1 while P1 is currently 0/8, so owners must either satisfy it or
explicitly change the policy before a PASS.

**Reconciliation needed:** add the RE-12 policy decision to Batch 8 entry/exit
evidence and the handoff quality gates. Do not substitute optional P1
regression pass rate for the configured strict-FULL policy.

### 5. The test-design contrast oracle is stale

The test-design group repeatedly labels contrast/WCAG as UNKNOWN and Batch 7
asks stakeholders to define a contrast oracle before FULL. PRD FR-19 and NFR-6
already define the product threshold as text contrast of at least 4.5:1. Only a
broader WCAG level or legal regime remains unspecified.

**Reconciliation needed:** use 4.5:1 as the packaged-app contrast oracle and
keep only the broader WCAG/legal-compliance question open. The current wording
creates an unnecessary blocker and permits the approved threshold to drift.
