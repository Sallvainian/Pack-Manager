---
review: downstream-divergence
artifact: ../ARCHITECTURE-SPINE.md
date: '2026-07-23'
verdict: FAIL
critical: 3
high: 4
medium: 0
re_review_date: '2026-07-23'
re_review_verdict: PASS
remaining_critical: 0
remaining_high: 0
---

# Independent Reviewer Gate — Downstream Divergence

## Verdict

**FAIL.** The spine establishes the right high-level boundaries, but seven
committed contracts still permit independently reasonable downstream units to
produce incompatible wire coverage, candidate identities, evidence records,
lane qualifications, or readiness results. The most consequential gaps are
that the two versioned evidence schemas are described but not actually fixed,
candidate evidence has no unambiguous field for the artifact under test, and
the final aggregation/retry algorithm is not defined.

This review does not dispute the 72-row denominator, promote a baseline row,
approve the pending coverage map, resolve DR-1, approve DR-4, or claim release
readiness.

## Divergence cases

### DIV-01 — [CRITICAL] Candidate Manifest v1 is not one exact schema and its component-hash boundaries are incomplete

**Committed text:** AD-7 calls the manifest a strict Draft 2020-12 schema, but
the normative table supplies only category-level fields. It does not fix the
complete nested property names, object/array shapes, scalar encodings, null
rules, repository representation, or exact bytes hashed for several component
fingerprints.

**Compliant unit A:** A Release story defines `source.lockfiles` as an object
whose keys are file names, records the repository as an HTTPS URL, hashes the
literal updater-public-key text, and obtains the certificate fingerprint by
hashing the leaf certificate's DER bytes.

**Compliant unit B:** An independent verifier defines `source.lockfiles` as a
sorted array, records the repository as an SSH URL, hashes the decoded updater
key, and hashes a textual certificate export. It also enforces
`additionalProperties: false`, NFC, I-JSON, RFC 8785 JCS, and SHA-256.

Both implementations satisfy the prose categories and canonicalization rule.
They reject each other's documents or assign different identities to the same
candidate. The same ambiguity applies to resolved Rust/Node/Xcode identity,
workflow identity, tag syntax, and build-workflow hashing.

**Exact binding language needed:**

> Candidate Manifest v1 has one normative, complete Draft 2020-12 schema. The
> architecture handoff MUST inline that schema or name one exact
> version-controlled schema path and accepted SHA-256 before any producer or
> validator is implemented. The schema fixes every property path, required and
> optional member, scalar type, null rule, enum, pattern, array identity key,
> and ordering precondition; independently authored v1 schema variants are
> non-conforming. File hashes, including lockfiles, workflow source, metadata,
> and release artifacts, are SHA-256 over exact raw file bytes. The Developer
> ID fingerprint is SHA-256 over the DER-encoded leaf signing certificate. The
> updater-key fingerprint is SHA-256 over the exact UTF-8 bytes of the public
> key string embedded in the packaged application, with no whitespace or
> newline normalization. The repository identity and toolchain fields use
> schema-fixed canonical literals/structured fields rather than producer-local
> command output. The accepted schema and canonicalization-vector hashes are
> bound into the contract bundle and validated before manifest hashing.

### DIV-02 — [CRITICAL] Evidence Record v1 cannot unambiguously name the candidate artifact actually tested

**Committed text:** AD-9 requires the Registrar to revalidate the “tested
artifact checksum,” and AD-10 requires candidate-bound evidence to add “every
exact tested artifact checksum.” The Evidence Record table defines only
`artifacts[]`, whose entries are expressly result artifacts stored in the
evidence object store. It provides no candidate-subject field.

**Compliant unit A:** A Batch 7 producer records the Candidate Manifest digest
and puts only screenshots, logs, and JSON results in `artifacts[]`. It assumes
the manifest digest is sufficient to identify the tested DMG or updater
archive.

**Compliant unit B:** A Batch 8 producer puts the tested DMG and `latest.json`
in `artifacts[]`, or adds `testedArtifactChecksum`. The first choice confuses
candidate subjects with retained evidence objects; the second violates the
unknown-field rejection rule.

The two records cannot be interpreted consistently, and unit A can pass while
testing a different local copy whose checksum was never named.

**Exact binding language needed:**

> Evidence Record v1 separates `subjectArtifacts[]` from
> `resultArtifacts[]`. Each candidate-bound `subjectArtifacts[]` entry contains
> an exact Candidate Manifest artifact logical ID, raw-byte length, and digest;
> entries are unique and ASCII-sorted by logical ID. The Registrar rejects a
> subject absent from the manifest, a length/digest mismatch, or a record whose
> scenario-required subject set is incomplete. `resultArtifacts[]` contains
> only immutable evidence outputs and retains the existing
> content-addressed-path rules. Source- and environment-bound records always
> use an empty subject set; later association with a same-source candidate does
> not claim that those records tested candidate bytes.

The complete Evidence Record envelope and payload schema must be fixed with the
same single-authority rule as Candidate Manifest v1; the current field summary
is not an exact versioned schema.

### DIV-03 — [CRITICAL] The final aggregator has no deterministic rule for required evidence slots, retries, or conflicting outcomes

**Committed text:** AD-10 retains the first failure and allows a manually
authorized retry; AD-14 requires every row to be satisfied. It does not define
which attempt is decisive, how one row combines multiple required scenarios or
architectures, or what it means when one record names several criteria.

**Compliant unit A:** An aggregator marks a criterion FULL when any qualified
PASS exists, provided earlier failures remain visible and the PASS links to the
failure as a retry.

**Compliant unit B:** Another aggregator refuses FULL whenever any qualified
FAIL exists, because the retry “never replaces” the first result. A third
reasonable implementation applies one outcome to every ID in a multi-criterion
record even when only part of the scenario passed.

All retain the first failure and can claim to satisfy the present prose, but
they issue different 72-row decisions for the same index.

**Exact binding language needed:**

> A frozen acceptance profile defines the required evidence slots for every
> criterion and separate RP-1/RP-2 prerequisite. A slot fixes its scenario,
> subject set, lane, minimum binding depth, environment/architecture matrix,
> and permitted concern. Every attempt names exactly one slot; one attempt may
> reference multiple criterion IDs only when that same indivisible slot and
> outcome applies to all of them. A slot is satisfied only when its first
> attempt is PASS or the terminal attempt in one valid, gap-free,
> explicitly-authorized retry chain is PASS. A later PASS may satisfy the slot
> but never removes or changes prior failures, which remain visible in both
> machine- and human-readable decisions. FAIL, ERROR, NOT-RUN, missing,
> cancelled, skipped, ignored-but-unexecuted, a forked retry, or an
> unauthorized retry leaves the slot unsatisfied. A criterion is FULL only
> when every required slot is satisfied; RP-1/RP-2 are evaluated separately.
> The aggregator algorithm, including duplicate/fork rejection and terminal
> attempt selection, is versioned and covered by shared decision vectors.

### DIV-04 — [HIGH] Per-criterion lane and provenance requirements have no frozen authority

**Committed text:** AD-10 says each coverage-map row declares a minimum binding
depth and permitted execution lane. The current authoritative
`readiness-coverage-map.md` does not contain either field. It contains the
provisional *readiness concern* lane (`Product/source correction`, `Test
infrastructure/coverage`, or `Release evidence`), which AD-10 correctly treats
as different from an execution lane.

**Compliant unit A:** QA maps `F10-AC1` to a candidate-release slot because its
source consequence says “packaged UI.”

**Compliant unit B:** another QA epic maps automated contrast and reduced
motion to forced-offline evidence and only VoiceOver/focus to
candidate-release evidence, relying on the row's secondary-overlap note.

Both preserve the same criterion and closure batch, but their aggregators
accept different evidence depth. Similar divergence can omit either Apple
silicon or physical Intel when a row has a multi-environment consequence.

**Exact binding language needed:**

> The coverage map remains unchanged and pending approval. Before any result
> aggregation, Product and QA must approve one versioned
> `criterion-acceptance-profile/v1` companion that maps all 72 unchanged IDs
> and RP-1/RP-2 to their complete required evidence slots. Each slot declares
> readiness concern, permitted execution lane, minimum binding level,
> scenario/command, candidate subject set, required operating-system and
> architecture matrix, and retry policy. The profile cannot alter denominator
> membership, priority, baseline status, or map approval state. Its revision
> and raw-file SHA-256 are frozen with the coverage-map and gate-policy
> digests, carried in every record, and validated by the Registrar and final
> aggregator. Until that profile is accepted, lane/depth aggregation is an
> explicit implementation-handoff blocker.

### DIV-05 — [HIGH] Controlled-adapter selection can be either non-shipping or a hidden runtime mode

**Committed text:** AD-2 requires one composition root and two adapter sets,
while AD-5 says packaged candidate checks must not enable a hidden test path.
The spine does not prohibit compiling a runtime selector for controlled
adapters into the distributed application.

**Compliant unit A:** Development creates a non-distributable native harness
target that calls the shared production builder/registration source with
controlled adapters at compile-time.

**Compliant unit B:** Development lets the ordinary app select controlled
adapters when an environment variable or hidden launch argument is present.
Native acceptance crosses the same bridge, registration, handlers, and events,
so it appears compliant with AD-2/AD-3. The shipped app now contains a hidden
way to bypass production filesystem/process/updater behavior.

The two implementations have materially different production safety and
candidate-acceptance boundaries.

**Exact binding language needed:**

> Adapter-set selection is a construction-time capability of a separate,
> non-distributable native acceptance target. The release application target
> compiles and constructs only production adapters and exposes no environment
> variable, launch argument, preference, deep link, command, event, feature
> flag, or runtime API that selects controlled adapters or disposable-root
> behavior. The native target may share the exact production builder,
> registration source, handlers, models, and dispatcher, but it is excluded
> from every candidate bundle and release manifest. Candidate packaged tests
> use external OS isolation around unchanged release bits.

### DIV-06 — [HIGH] “Representative command families” and set equality do not fix the native wire-coverage contract

**Committed text:** AD-3 requires representative success/failure paths for
each command family and set equality for names. No authoritative family
taxonomy or per-command request/success/error/event payload contract is
identified.

**Compliant unit A:** Architecture groups the baseline commands into four
broad families and crosses the real transport for two commands per family.
Set-equality inventory accounts for the remaining names without invoking them.

**Compliant unit B:** QA defines ten narrow families and crosses every command
at least once. It uses different assumptions about omitted fields, `null`,
unknown properties, enum spellings, and error serialization.

Both can satisfy “representative” and name equality while producing
incompatible native-coverage exits and allowing an unexercised command's wire
shape to drift.

**Exact binding language needed:**

> Architecture owns one versioned boundary-contract catalog derived from the
> production registration source. For every registered command and event it
> records the exact production name; request, success, error, and event payload
> schemas; required/optional/null/unknown-field behavior; owning command
> family; and required native vectors. Every registered command must complete
> at least one real frontend-to-Tauri-to-Rust round trip, every event must cross
> one real subscription/dispatch path, and the catalog—not a locally invented
> grouping—defines representative family success/failure vectors. The
> inventory proves set equality among production registration,
> wrappers/subscriptions, catalog entries, and executed native coverage. A
> deliberate surface change updates all four in one change.

The catalog is a contract inventory, not a permanent 20-command/six-event
count.

### DIV-07 — [HIGH] Registrar authority and compare-and-append are not verifiable or transport-atomic

**Committed text:** AD-8 names one Release-owned automation identity and says
the Registrar performs compare-and-append, while allowing provider-neutral
immutable snapshots. It does not define the provider-enforced serialization
boundary, idempotency behavior, or how a validator proves that the claimed
Registrar—not a self-authored local process—accepted a record.

**Compliant unit A:** A GitHub workflow reads the latest snapshot, appends, and
uploads the next numbered snapshot under a workflow concurrency group.

**Compliant unit B:** A local Release script using the same automation token
checks the predecessor before upload. Both are controlled by Release and both
perform a logical comparison. Concurrent/replayed runs can nevertheless
publish competing sequence numbers, and either unit can self-assert producer
and Registrar identity in the payload.

Validators can accept different forks or assurance levels.

**Exact binding language needed:**

> The Registrar has one candidate-scoped, provider-enforced serialization
> domain covering predecessor read, validation, sequence allocation, append,
> and publication. Under that lock it re-reads the single accepted predecessor,
> requires exact sequence/digest match, applies a unique idempotency key, and
> publishes one immutable next snapshot; a competing sequence, stale
> predecessor, replay with different bytes, or second successful publication
> is rejected. Every accepted append is bound to a provider-verifiable
> Registrar identity containing repository, protected workflow path and source
> commit, run/attempt, and protected environment or equivalent immutable
> attestation. Self-asserted payload identity or possession of a general token
> is insufficient. The validation contract names the sole accepted-head
> mechanism and how an offline verifier resolves exactly one chain before
> release preparation implementation begins.

This does not require a replacement release framework; the mechanism must be
implemented within the approved GitHub Actions/GitHub transport boundary.

## Gate conditions for a re-review

The reviewer gate can pass when:

1. Candidate Manifest v1 and Evidence Record v1 each have one exact schema
   authority and every subordinate hash boundary is byte-exact.
2. Candidate subject artifacts and evidence result artifacts are distinct in
   the record contract.
3. Required evidence slots, retry resolution, and final aggregation are
   deterministic.
4. The pending coverage map is paired—without denominator/status mutation—with
   an explicitly blocked, approval-bound lane/depth profile.
5. Controlled adapters cannot be selected by shipped bits.
6. The production boundary catalog fixes family taxonomy, wire schemas, and
   minimum actual transport execution while preserving dynamic surface counts.
7. Registrar serialization and authority are provider-verifiable and
   single-headed.

## Re-review

**Verdict: FAIL — 1 critical, 0 high remaining.**

The remediated spine closes the seven original divergence cases:

- **DIV-01 closed:** one locked `/v1` contract byte set, normative exact
  property paths, and raw-file/DER/embedded-key hashing boundaries remove
  independently authored Candidate Manifest variants.
- **DIV-02 closed:** `subjectArtifacts[]` and `resultArtifacts[]` now have
  distinct, closed schemas and validation rules.
- **DIV-03 closed:** one slot per record, gap-free retry chains, a frozen retry
  disposition, and deterministic profile replay define the decisive result.
  DR-4 remains explicitly blocked rather than silently selecting a policy.
- **DIV-04 closed:** the Criterion Acceptance Profile fixes every criterion and
  RP-1/RP-2 slot's lane, depth, scenario, subject, environment matrix, and retry
  rule without altering the coverage map; map, DR-1, and DR-4 blockers prevent
  premature freeze.
- **DIV-05 closed:** controlled adapters are construction-time dependencies of
  a non-distributable target, with no shipped runtime selector.
- **DIV-06 closed:** the authoritative boundary catalog fixes command families,
  wire-schema digests, scenarios, set equality, and minimum real transport
  execution while retaining dynamic command/event counts.
- **DIV-07 closed:** the protected Registrar identity, candidate/profile lock
  or CAS, exact idempotency key, stale/fork rejection, atomic single head, and
  explicit transport handoff blocker prevent competing compliant append
  semantics.

### DIV-08 — [CRITICAL] PASS currently permits collected-but-unexecuted checks

**Current rule:** Evidence Record v1 says PASS requires
`collected >= executed = passed > 0` with all non-passed counts zero.

**Compliant validator A:** Accepts `collected = 10`, `executed = 8`, and
`passed = 8`, because the written inequality is true and every failure/skip
counter is zero.

**Compliant validator B:** Rejects the same record because two collected checks
were unexecuted and AD-10/TIR-8 say collected-but-unexecuted required checks
cannot produce PASS.

The same index can therefore produce different slot and 72-row outcomes, and
validator A can issue a false PASS.

**Minimum exact binding rule needed:**

> Outcome counts cover exactly the checks required by the profile-fixed slot
> and scenario contract. PASS requires
> `collected = executed = passed > 0` and
> `failed = errored = skipped = ignored = cancelled = 0`. An unselected,
> filtered, collected-but-unexecuted, missing, or optional check may not be
> hidden outside those counts; it must be excluded by the versioned scenario
> contract or represented by a separate slot/result. Any count mismatch fails
> closed.

No other new critical or high divergence was found.

### Final targeted re-check

**Verdict: PASS — 0 critical, 0 high remaining.**

**DIV-08 closed:** Evidence Record v1 now states that outcome counts cover
exactly the profile-fixed slot/scenario requirements and that PASS requires
`collected = executed = passed > 0` with every non-passed count zero. Registrar
validation separately rejects ignored, skipped, unexecuted, failed, errored,
cancelled, filtered, or unreported required checks and requires optional checks
to be excluded by the versioned scenario contract or represented separately.
The former validator divergence is no longer possible.

The final targeted scan found no remaining critical or high downstream
divergence.
