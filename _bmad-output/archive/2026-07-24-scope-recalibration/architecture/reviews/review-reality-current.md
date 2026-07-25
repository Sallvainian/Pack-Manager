# Independent Architecture Review — Reality and Currentness

## Verdict

**FAIL pending two high-severity contract corrections and two
medium-severity precision corrections.**

Reviewed artifact:
`ARCHITECTURE-SPINE.md`, snapshot SHA-256
`2860f460c5fcfe1de11a90d964e7c0bcacd33272b207b5dd476d8625f3623c5c`.

Finding count:

- **HIGH:** 2
- **MEDIUM:** 2
- **LOW:** 0

This verdict is about the architecture artifact only. It is not a product,
test, candidate, or release-readiness verdict.

## Review Method

The review mechanically checked the spine against:

- the finalized PRD, addendum, and pending-approval 72-row coverage map;
- `docs/SPEC.md`, `docs/DECISIONS.md`, and project context;
- current Tauri registration, event definitions, frontend bridge/wrappers,
  startup ordering, process runner, settings, journal, updater, and
  application composition;
- Tauri configuration and capabilities, package/Cargo manifests and lockfiles,
  `.nvmrc`, and all relevant GitHub workflows;
- the traceability and test-design artifacts as gap evidence only; and
- primary specifications for JSON Schema Draft 2020-12, RFC 8785 JCS, and
  FIPS 180-4 SHA-256.

Current-state claims were distinguished from future architectural mandates.
The current command/event surface was independently enumerated: 20 registered
Rust commands have matching TypeScript invoke wrappers, and six Rust event
names match six TypeScript event constants/subscriptions. The numbers were
treated as the current set, not as a permanent invariant.

## Findings

### HIGH R-1 — The named v1 schemas are not yet exact enough to prevent incompatible implementations

**Evidence**

- AD-7 names `pack-manager.candidate-identity/v1` and describes required
  content, but does not define exact nested member names, primitive types,
  required sets, patterns, bounds, or `additionalProperties: false` at every
  object boundary (`ARCHITECTURE-SPINE.md` lines 246–271 and 480–488).
- The same problem exists for the evidence envelope and payload. Grouped
  entries such as `source`, `build`, `signing`, `coverageMap`, `gatePolicy`,
  `producer`, and `attempt` allow multiple schema-valid interpretations unless
  their exact shapes are fixed (lines 278–299 and 494–516).
- The planned schema files appear only in the logical future layout; the spine
  supplies neither their exact schema content nor immutable schema-document
  digests (lines 518–527).
- JSON Schema Draft 2020-12 distinguishes the instance discriminator from the
  schema document's `$schema` dialect URI and absolute `$id`. The spine fixes
  the former but not the latter.
- Repository identity normalization is unspecified. The same repository can
  otherwise appear as an SSH remote, HTTPS URL, or owner/repository slug,
  producing different candidate digests despite identical intended identity.

**Why this fails the gate**

The user explicitly required a versioned evidence schema that produces
reproducible identities across machines and runs. RFC 8785 makes one parsed
value deterministic; it cannot make two differently shaped or differently
normalized values equivalent. Independent Release, QA, and CI implementations
can still serialize different valid-looking manifests or records.

**Exact fix**

Within the spine, make both v1 contracts mechanically unambiguous:

1. Declare the schema documents' exact Draft 2020-12 `$schema` URI and immutable
   absolute `$id` values, while retaining the existing instance `schema`
   discriminator literals.
2. Define every nested key, JSON type, required set, enum, pattern, length/bound,
   nullability rule, sort key, and `additionalProperties: false` boundary for
   Candidate Manifest v1, Evidence Record payload v1, and the record envelope.
3. Choose and state one normalized repository-identity syntax.
4. State how each schema document and the canonicalization-vector file is
   frozen and checksum-bound before any producer or Registrar is admitted.
5. Make conformance to those exact schemas and shared positive/negative vectors
   an implementation-entry acceptance check.

The existing JCS byte boundary, SHA-256 boundary, newline rule, append
authority, and candidate invalidation rules can remain.

### HIGH R-2 — The current coverage map cannot supply the lane/depth decision that AD-10 says it supplies

**Evidence**

- AD-10 says each coverage-map row declares a minimum `bindingLevel` and
  permitted execution lane, and the Registrar/aggregator relies on that
  declaration (`ARCHITECTURE-SPINE.md` lines 338–345 and 449–452).
- The actual pending-approval coverage map contains criterion ID, consequence,
  P0 priority, baseline status, provisional remediation lane, PRD references,
  closure batch, and prose notes. It has no explicit evidence-binding-depth or
  permitted-execution-lane fields
  (`readiness-coverage-map.md` lines 18–28 and 54–58).
- Its `Product/source correction`, `Test infrastructure/coverage`, and
  `Release evidence` values are remediation ownership lanes, not the three
  execution lanes defined by AD-6. The map itself says provenance remains
  governed by TIR-8 and RE-10.

**Why this fails the gate**

Without one frozen criterion-to-depth/lane policy, the Registrar cannot
deterministically decide whether a source-, environment-, or candidate-bound
record may satisfy a criterion. Different batches could accept different
depths while all claiming conformance. Inferring this from prose would defeat
the fail-closed contract.

**Exact fix**

Do not alter or approve the 72-row map. Instead:

1. Define a separate versioned criterion-evidence policy, or explicitly make
   it a required portion of the still-unapproved P0 gate policy.
2. Key it one-to-one by all 72 frozen criterion IDs and declare each row's
   minimum binding level and allowed execution lane(s), without changing the
   denominator, baseline status, or remediation lane.
3. Record its revision and SHA-256 in every Evidence Record and in final
   aggregation input.
4. Change AD-10 from “the frozen map permits” to “the frozen map plus the
   approved criterion-evidence policy permit.”
5. Add Product/QA approval of this policy as an explicit blocker before
   candidate validation or gate configuration, consistent with DR-4.

### MEDIUM R-3 — “Signing is optional today” is too broad and is false for updater signing

**Evidence**

- The baseline says the release workflow can finish without “signing/
  notarization secrets,” and AD-12 calls the current signing/notarization
  fallback optional (`ARCHITECTURE-SPINE.md` lines 106–109 and 399–403).
- The current workflow does allow an Apple-unsigned/unnotarized build when the
  Developer ID or Apple API credentials are absent
  (`release.yml` lines 112–165).
- However, the same workflow explicitly states that the updater archive and
  detached signature require `TAURI_SIGNING_PRIVATE_KEY`, and that the build
  fails without it because updater artifacts are enabled (`release.yml` lines
  172–180). The CI debug smoke avoids both signing systems only by using
  `--no-sign`.

**Consequence**

“Signing” can be read to include updater signing, which would misstate current
mechanics at the exact release-integrity boundary this spine governs.

**Exact fix**

Qualify both statements as **Apple Developer ID signing and Apple
notarization** being optional in the current release workflow. State
separately that the current release build already requires updater signing
credentials, while the future candidate lane requires both updater signing and
the complete Developer ID/notarization credential set and fails closed if
either is absent.

### MEDIUM R-4 — ASR-02 no longer preserves the approved staged delivery boundary

**Evidence**

- The approved proposed timing was: core process controls before Batch 5, with
  relevant filesystem/updater extensions before their Batch 6–7 consumers.
- The current ASR table instead requires the complete process, filesystem, and
  updater capability before Batch 5
  (`ARCHITECTURE-SPINE.md` line 468).
- The addendum makes Batch 5 depend on controllable process/OS boundaries,
  Batch 6 depend on lifecycle/filesystem capability, and Batch 7 consume
  packaged updater capability. It does not require every later extension to be
  complete at the Batch 5 entry point.

**Consequence**

The stricter wording is technically safe, but it silently changes the delivery
timing the user explicitly said to keep and can create an unintended
front-loaded implementation blocker.

**Exact fix**

Restore the approved staged wording:
“Core controls accepted before Batch 5; relevant filesystem/updater extensions
accepted before Batches 6–7.” Keep Development as the single accountable role
and Platform as the capability area.

## Decision-by-Decision Reality Audit

| Decision | Result | Basis |
| --- | --- | --- |
| AD-1 | PASS | Directly matches PRD/addendum separation of product behavior, reusable infrastructure, and candidate evidence. |
| AD-2 | PASS as future invariant | Existing `CommandRunner`, `EventSink`, updater seams, and current hard-wired composition make the boundary necessary; it is not represented as already implemented. |
| AD-3 | PASS | Current 20-command/six-event set, wrappers, subscriptions, bridge-only import, and subscribe-before-hydrate order were mechanically verified. |
| AD-4 | PASS | Current runner safety mechanics and remaining direct OS/path/updater effects were verified; settled no-shell/no-sudo/update behavior is preserved. |
| AD-5 | PASS | Atomic settings, journal reconstruction, historical-PGID rule, real-path binding, and missing window-close host wiring were verified. |
| AD-6 | PASS | Current workflows lack host-wide denial and qualified target/candidate lanes; the three future lanes match TIR-2/TIR-6/TIR-7. |
| AD-7 | FAIL pending R-1 | RFC 8785, I-JSON, UTF-8, no-normalization, and SHA-256 rules are current and correctly described, but the data schema is not exact enough. |
| AD-8 | FAIL pending R-1 | Hashing and append authority are explicit; exact envelope/payload schemas still need closure. |
| AD-9 | PASS | Candidate invalidation matches RE-1 and the addendum; rebuild run identity is explicitly future policy. |
| AD-10 | FAIL pending R-2 | Provenance and first-failure rules match TIR-8/RE-10/RE-11, but the required row-level lane/depth oracle does not yet exist. |
| AD-11 | PASS | Packaged WKWebView, updater, accessibility, Apple silicon, and physical Intel boundaries match the PRD plus approved DR-2/DR-3. DR-1 remains open. |
| AD-12 | PASS after R-3 wording fix | Current release-please/GitHub Actions mechanics, automatic publication, optional Apple trust credentials, and `--clobber` were verified. |
| AD-13 | PASS | The dependency wave matches the addendum: 1; parallel 2/3/4; 5/6 after 4; freeze; 7; then 8. |
| AD-14 | PASS | The map is still `final-pending-approval`; mechanical inspection confirms exactly 72 unique rows and 14 baseline FULL. No readiness promotion is claimed. |

## Verified Current Facts

- The application remains one local React/Tauri/Rust product with no HTTP API
  or database.
- Current registration and wrappers agree on 20 commands; Rust/TypeScript
  agree on six events. These are correctly described as a mutable baseline.
- Browser E2E replaces `__TAURI_INTERNALS__`; no existing test crosses the
  complete production JavaScript-to-Tauri-to-Rust transport.
- `RealRunner` uses structured argv, `env_clear`, explicit environment, null
  stdin, a new process group, bounded output, timeout, and
  SIGTERM-to-SIGKILL escalation.
- Settings replacement is atomic; journal start-without-finish becomes
  Interrupted; journaled PGIDs are not signaled on startup.
- No minimum macOS deployment target is declared.
- The release workflow checks out the tag, verifies the three application
  versions, builds `universal-apple-darwin`, permits missing Apple trust
  credentials, requires updater signing for normal release artifacts, and
  uploads release assets with `--clobber`.
- No Candidate Manifest, Evidence Index, host-wide forced-offline gate,
  provisioned target-Mac lane, or installed-candidate lane exists today.
- Lockfile-derived versions in the stack table are current for the reviewed
  workspace.

## Primary-Standard Check

- **RFC 8785 JCS:** The spine correctly uses I-JSON input, deterministic
  property ordering, ECMAScript-compatible primitive serialization, no
  inter-token whitespace, preserved Unicode code points, and UTF-8 output.
  Its added NFC-rejection rule is an allowed ecosystem constraint and does not
  claim that JCS performs normalization.
- **JSON Schema Draft 2020-12:** The selected dialect is current and usable.
  R-1 is required because the dialect URI and schema-resource identity are not
  yet fully bound in the contract.
- **FIPS 180-4:** SHA-256 remains specified by the current final Secure Hash
  Standard. NIST's published plan to revise FIPS 180-4 does not invalidate the
  selected SHA-256 boundary.

## Gate Exit Conditions

The reality/currentness reviewer can pass the spine after R-1 through R-4 are
applied and the final artifact is re-linted. No product code, tests, CI,
release workflow, or configuration change is required to close these document
findings.

## Re-review

### Verdict

**PASS at the independent reality/currentness gate.**

Reviewed remediated artifact snapshot SHA-256:
`4049cf0cd1e133f34f59f5f809bb8144fc54b7b1e5a979d63550db5ca0a1ffce`.

Remaining severity:

- **CRITICAL:** 0
- **HIGH:** 0
- **MEDIUM:** 2 editorial precision fixes
- **LOW:** 0

### Original finding closure

| Finding | Re-review result | Evidence |
| --- | --- | --- |
| R-1 — exact schemas | **CLOSED** | The spine now fixes Draft 2020-12 `$schema` and immutable `$id` values, target paths, a JCS-locked contract file, exact nested property paths/types/rules, closed-object behavior, normalization, raw-byte boundaries, fixed arrays, canonical repository identity, and validation/vector admission. |
| R-2 — criterion lane/depth oracle | **CLOSED** | AD-15 and Criterion Acceptance Profile v1 now bind all 72 criterion IDs plus RP-1/RP-2 to explicit slots, lanes, depths, scenarios, subjects, environments, physical-host requirements, and retry policy without changing or approving the coverage map. The profile/map/policy digests are bound into each record and the aggregator. |
| R-3 — signing wording | **CLOSED** | The baseline and AD-12 now distinguish currently optional Apple Developer ID signing/notarization from currently required updater signing, and the future candidate lane fails closed on either credential class. |
| R-4 — ASR-02 timing | **CLOSED** | The ASR table restores the approved staged boundary: core process controls before Batch 5 and relevant filesystem/updater extensions before Batches 6–7. Development remains the sole accountable role and Platform the capability area. |

### Non-blocking precision fixes

1. AD-10 still says association is allowed when “the frozen map permits that
   depth and lane.” The map has no such fields; the newly frozen **Criterion
   Acceptance Profile** permits them. Replace “map” with “Criterion Acceptance
   Profile” to align the sentence with AD-15 and the normative schema.
2. `contracts/readiness/v1/*` does not exist in the current brownfield tree,
   which is correct because this turn creates architecture only. To avoid
   reading target-state language as a current implementation claim, change
   “The `/v1` authority is the exact byte set committed at these paths” to
   “Before ASR-04 implementation acceptance, the `/v1` authority shall be
   committed at these paths and locked as follows.” This preserves the
   architecture-only scope and explicitly blocks use until the target contract
   files exist.

Neither precision issue permits cross-batch divergence: the surrounding
AD-15 rules and normative property tables resolve both meanings
deterministically. No new critical or high-severity issue was found.
