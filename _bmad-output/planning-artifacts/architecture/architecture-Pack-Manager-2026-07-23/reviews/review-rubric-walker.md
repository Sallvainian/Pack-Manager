# Independent Architecture Gate — Rubric Walker

**Reviewed artifact:** `ARCHITECTURE-SPINE.md`  
**Reviewed SHA-256:** `2860f460c5fcfe1de11a90d964e7c0bcacd33272b207b5dd476d8625f3623c5c`  
**Deterministic lint:** PASS, zero findings  
**Verdict:** **FAIL**

The spine covers the initiative's real divergence points unusually well, but
its purportedly normative evidence schemas are still descriptions of schemas,
not exact schemas; independent implementations can therefore produce different
canonical values while claiming conformance to the same `/v1` contract.

## Findings

### HIGH — H1: The `/v1` evidence schemas do not define one exact JSON value shape

**Disposition:** Autofix before finalization.

AD-7 and AD-8 correctly fix JCS, UTF-8, SHA-256, byte boundaries, newline
handling, append authority, and validation intent
(`ARCHITECTURE-SPINE.md:241-306`). The field tables still define only top-level
containers such as `source`, `build`, `signing`, `producer`, and `attempt`,
however (`ARCHITECTURE-SPINE.md:478-510`). They do not define:

- the exact nested property names and tree;
- the type, requiredness, literal, pattern, and length rule for every property;
- whether `additionalProperties: false` applies recursively;
- the exact canonical repository and tag representation, so Git SSH/HTTPS
  aliases or `v0.2.7` versus `refs/tags/v0.2.7` cannot diverge;
- the exact array identity key for every array;
- the byte format of `candidate-identity.sha256`; or
- an immutable binding from the `/v1` identifier to the exact schema and
  canonicalization-vector bytes stored under `schemas/`.

Consequently, two teams can both satisfy the prose while choosing, for example,
`source.repository` versus `source.repositoryId`, nested versus flat lockfile
objects, or different repository/tag spellings. JCS would deterministically
hash each choice, but it would not make those choices the same. That defeats
AD-7's stated prevention and the user's explicit cross-machine/cross-run
reproducibility requirement.

**Exact correction:** Add one normative property-path/type table or compact
JSON shape for both `pack-manager.candidate-identity/v1` and
`pack-manager.evidence-record/v1`. It must enumerate every property and nested
property, required set, closed-object rule, enum/literal, lexical pattern,
array uniqueness and sort key, and repository/tag normalization. Bind the
exact two schema files and canonicalization-vector set immutably—either by
their SHA-256 values in the contract/candidate identity or by a single
`schemaSetDigest` whose byte construction is defined. Define the digest-sidecar
bytes as well. Keep the already-correct JCS and SHA-256 boundaries unchanged.

### MEDIUM — M1: AD-10 assigns new lane/depth fields to the pending coverage map

**Disposition:** Autofix before finalization.

AD-10 says each coverage-map row declares a minimum binding depth and permitted
execution lane and later says the frozen map permits association
(`ARCHITECTURE-SPINE.md:338-345`). AD-14 simultaneously says Architecture does
not alter the map (`ARCHITECTURE-SPINE.md:442-457`). The current normative map
is still `final-pending-approval` and contains provisional closure ownership,
PRD IDs, and batches, but no binding-level or execution-lane fields
(`readiness-coverage-map.md:1-47`).

That ambiguity permits one downstream unit to edit the map and another to put
the mapping in DR-4's gate policy. The first path would also silently violate
the instruction to preserve and not alter the current map.

**Exact correction:** State that the approved P0 gate-policy artifact—not the
current coverage map—must key all 72 frozen criterion IDs to minimum
`bindingLevel` and permitted `executionLane` values. The Evidence Index already
binds both map and gate-policy digests, so this preserves the denominator and
current map byte-for-byte. Keep candidate validation blocked while DR-4 is
PROPOSED and the policy mapping does not exist.

### MEDIUM — M2: ASR-02 no longer preserves the approved staged delivery timing

**Disposition:** Autofix before finalization.

The accountability row now requires the entire ASR-02 capability—including
filesystem, opener, restart, and updater controls—before Batch 5
(`ARCHITECTURE-SPINE.md:468`). The approved timing was staged: core
process/operating-system controls before Batch 5, with the relevant
filesystem/lifecycle extensions before Batch 6 and updater/installed-app
extensions before Batch 7. The addendum likewise makes Batch 5 depend on
controllable process/OS boundaries, Batch 6 on the lifecycle environment, and
Batch 7 on the updater/candidate prerequisites (`addendum.md:58-62`).

Requiring every later extension before Batch 5 changes a deadline the user
explicitly told Architecture to keep and unnecessarily blocks Batch 5 on
Batch-7 capability.

**Exact correction:** Restore the staged delivery boundary in the ASR-02 row:
core process controls accepted before Batch 5; the relevant filesystem and
lifecycle controls before Batch 6; updater and installed-app controls before
Batch 7. Development remains the sole accountable role and Platform remains
the capability area.

## Good-Spine Checklist

| Rubric item | Result | Notes |
| --- | --- | --- |
| Fixes the initiative-level divergence points and misses none | **Conditional fail** | ASR boundaries, three lanes, lifecycle, candidate invalidation, packaged acceptance, release hold point, and the eight-batch DAG are covered. H1 leaves the evidence contract itself divergent. |
| Every AD is enforceable and prevents its stated divergence | **Conditional fail** | AD-1 through AD-6 and AD-9 through AD-14 are enforceable after M1/M2. AD-7/AD-8 cannot enforce one `/v1` representation until H1 is fixed. |
| Deferred items cannot silently split downstream work | **Pass** | Harness, helper language, evidence transport/retention, secrets, hosts, people, and dates all have owners or explicit unassigned blockers plus revisit boundaries. |
| Named technology is verified-current | **Pass** | Lockfiles/manifests match the listed Tauri, Tokio, React, TypeScript, Vite, Playwright, and Node seed; workflows match release-please v5. The external standards are official JSON Schema 2020-12, RFC 8785, and FIPS 180-4 sources. |
| Ratifies rather than contradicts the brownfield system | **Pass** | The 20-command/six-event surface is correctly treated as a derived baseline, existing ports and release framework are retained, and necessary new seams are constrained to acceptance boundaries. |
| Covers source requirements and capabilities | **Pass after findings** | PRD/addendum ASRs, TIRs, REs, provenance, first-failure retention, ignored-test exclusion, DR statuses, physical Intel, packaged WKWebView, signing/notarization/updater, and 72-row governance are represented. |
| Inherited parent spine is preserved | **Not applicable** | No parent architecture spine was supplied. |
| Every initiative-owned dimension is decided, blocked, or deferred | **Pass** | Paradigm, boundaries, state mutation, ownership, evidence data, deployment/execution environments, provider constraints, operational lanes, release sequencing, and excluded scope are all addressed. |

## Status-Constraint Audit

- Coverage denominator remains exactly 72; no criterion is promoted.
- The 14 baseline-FULL rows remain planning history, not candidate proof.
- The coverage map remains `final-pending-approval`.
- DR-1 remains OPEN and blocks the affected handoff.
- DR-2 and DR-3 are APPROVED without claiming their evidence exists.
- Physical Intel remains a hard 100% requirement.
- DR-4 remains PROPOSED and blocked on Product/QA governance.
- Exactly one accountable role is assigned to each ASR.
- Product behavior, reusable test infrastructure, and candidate-specific
  release evidence remain non-substitutable.
- No new updater provider, distribution channel, or release framework is
  introduced.
- The spine makes no release-readiness claim.

## Finding Counts

| Severity | Count |
| --- | ---: |
| Critical | 0 |
| High | 1 |
| Medium | 2 |
| Low | 0 |

## Re-review — 2026-07-23

**Reviewed artifact SHA-256:** `4049cf0cd1e133f34f59f5f809bb8144fc54b7b1e5a979d63550db5ca0a1ffce`  
**Deterministic lint:** PASS, zero findings  
**Verdict:** **FAIL**

The remediation closes the original schema-shape and ASR-02 timing findings,
and it moves criterion lane/depth ownership into a separately hashed Criterion
Acceptance Profile. Two newly introduced contradictions in the exact evidence
contract must be fixed before the gate can pass.

### Original finding closure

| Original finding | Re-review result | Evidence |
| --- | --- | --- |
| H1 — `/v1` schemas did not define one exact JSON shape | **CLOSED** | The exact locked contract paths, immutable schema IDs, recursive closed-object rule, nested property-path/type tables, schema/vector digests, normalization rules, and validation stages now define and bind Candidate Manifest, Criterion Acceptance Profile, and Evidence Record v1 (`ARCHITECTURE-SPINE.md:550-738`). |
| M1 — lane/depth fields were assigned to the pending coverage map | **PARTIALLY CLOSED — MEDIUM remains** | AD-15 and the profile schema correctly place slot/lane/depth policy outside the untouched map (`ARCHITECTURE-SPINE.md:496-532`, `631-659`). One stale AD-10 sentence still says “the frozen map permits that depth and lane” (`ARCHITECTURE-SPINE.md:377-380`). Replace `map` with `Criterion Acceptance Profile` so no implementer reads the map as the policy authority. |
| M2 — ASR-02 staged timing was lost | **CLOSED** | The row again requires core controls before Batch 5 and relevant filesystem/updater extensions before Batches 6–7, with Development solely accountable and Platform the capability area (`ARCHITECTURE-SPINE.md:539-545`). |

### HIGH — H2: The common decimal rule makes every valid sequence invalid

**Disposition:** Autofix before finalization.

The common encoding rule forbids leading zeroes in “all quantities and sequence
numbers” (`ARCHITECTURE-SPINE.md:585-587`), while AD-8 and the Evidence Record
schema require an eight-digit sequence (`ARCHITECTURE-SPINE.md:318-320`,
`671`). The first valid sequence must be `00000001`, so it violates the common
rule. No first record can satisfy both normative statements.

**Exact correction:** Exempt `sequence` from the ordinary decimal-string rule.
Define it as exactly eight ASCII digits, starting at `"00000001"` and
incrementing by one without gaps through `"99999999"`. Keep the no-leading-zero
rule for byte lengths, run IDs, attempts, ordinals, and counts.

### HIGH — H3: The PASS count equation allows collected-but-unexecuted work

**Disposition:** Autofix before finalization.

The Evidence Record says PASS permits
`collected >= executed = passed > 0` (`ARCHITECTURE-SPINE.md:697-698`).
Therefore a record with 10 collected, 8 executed, and 8 passed is schema-valid
as PASS even though two collected checks were not executed. That contradicts
AD-10's exclusion of collected-but-unexecuted checks and validation stage 5's
promise to reject any PASS with unexecuted work
(`ARCHITECTURE-SPINE.md:382-388`, `729-731`).

**Exact correction:** Require
`collected = executed = passed > 0` for PASS, with every failed, errored,
skipped, ignored, and cancelled count equal to `"0"`. If discovery may include
irrelevant checks, record them outside the slot attempt; they cannot enter its
PASS counts.

### New-issue sweep

No critical issue was introduced. Apart from H2 and H3 and the residual M1
wording, the locked schema/profile arrangement is internally coherent,
provider-neutral, candidate/profile-bound, fail-closed, and consistent with the
approved DR and ASR status constraints.

### Remaining finding counts

| Severity | Count |
| --- | ---: |
| Critical | 0 |
| High | 2 |
| Medium | 1 |
| Low | 0 |

## Final targeted re-check — 2026-07-23

**Reviewed artifact SHA-256:** `a91bb7502f0fa188cdce57f19acec79b96461aba7beead20d2f4f36916da32d0`  
**Deterministic lint:** PASS, zero findings  
**Critical/high verdict:** **PASS**

- H2 is closed: `sequence` is now the explicit sole exception to the ordinary
  decimal rule and is exactly `"00000001"` through `"99999999"`, gaplessly
  incremented (`ARCHITECTURE-SPINE.md:585-588`).
- H3 is closed: slot counts now cover exactly profile-required checks and PASS
  requires `collected = executed = passed > 0` with every non-passed count
  `"0"` (`ARCHITECTURE-SPINE.md:699-700`).
- No remaining CRITICAL or HIGH rubric finding was found.
- The previously recorded MEDIUM wording cleanup remains: AD-10 line 379 says
  the frozen `map` permits lane/depth association, while the normative
  authority is the frozen Criterion Acceptance Profile.

### Final targeted counts

| Severity | Count |
| --- | ---: |
| Critical | 0 |
| High | 0 |
| Medium | 1 |
| Low | 0 |

## Final closure confirmation — 2026-07-23

**Reviewed artifact SHA-256:** `0a5f6247d484b284dad4434a26463f6504d43518370363e4cde3d2d75c51208b`  
**Deterministic lint:** PASS, zero findings  
**Final verdict:** **PASS**

M1 is closed: AD-10 now assigns lane/depth permission to the frozen Criterion
Acceptance Profile and leaves the pending coverage map unchanged
(`ARCHITECTURE-SPINE.md:372-381`). H1, H2, H3, M1, and M2 are all closed, and
the final targeted sweep found no remaining rubric finding.

| Severity | Count |
| --- | ---: |
| Critical | 0 |
| High | 0 |
| Medium | 0 |
| Low | 0 |
