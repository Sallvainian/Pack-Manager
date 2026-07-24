# Decision Reconciliation — `DECISIONS.md` vs `prd.md`

## Inputs

- `docs/DECISIONS.md`
- `_bmad-output/planning-artifacts/prds/prd-Pack-Manager-2026-07-22/prd.md`

## Result

Five material reconciliation gaps remain. The PRD correctly states the headline
precedence rules—D23a over D23, D25/D25a over D20 and stale ad-hoc delivery
language, and D26 as the sole transcript-repair exception—and it preserves the
signed/notarized/stapled release, explicit Restart gate, standard macOS menu
behavior, and no-administrator-prompt outcomes. The gaps below are places where
the normative requirements still permit a result that the decision log rejects.

## Material gaps

### 1. D10's one allowed cross-Manager deduplication rule is missing

**Decision:** D10 requires both mise and rustup truth to remain visible, but when
one plan contains both Rust targets, the plan must drop mise's `tool:rust`, show
the `rustDedup` exclusion, and update through rustup. It rejects broader automatic
cross-Manager deduplication.

**PRD distortion:** §10.2 correctly excludes “broader cross-Manager
deduplication,” but FR-6 through FR-9 never state the positive, safety-critical
Rust exception. A plan that schedules both mutations, or silently hides one
Manager's row, can therefore satisfy the PRD as written.

**Required reconciliation:** Add a planning consequence that preserves both
Manager rows, removes only mise's Rust target when the same plan contains the
rustup target, and shows the exclusion reason. Keep broader cross-Manager
deduplication out of scope.

### 2. The D23a/D25 stale-truth correction is collapsed into the wrong lane

**Decision:** D23a withdraws the user-facing `UNVERIFIED` status and retires the
synthetic-fixture limitation as a correctness oracle. D25/D25a likewise make
“notarization is out of scope” obsolete.

**PRD distortion:** The executive table calls stale `mas` claims one Product
Behavior defect, while TIR-1 owns their correction. This combines two different
things: obsolete user-visible product copy and repository-oracle drift in SPEC
text, comments, fixture documentation, or live-smoke expectations. It also
names only the stale `mas` half, not stale D20 delivery copy.

**Required reconciliation:** Put the user-visible outcome in Product Behavior:
no current UI may label `mas` unverified or notarization out of scope. Put
SPEC/comment/fixture/live-oracle cleanup and recurrence guards in Test
Infrastructure. Keep signed/notarized candidate proof in Release Evidence.
Update the executive count/description so documentation cleanup is not counted
as product behavior and a UI contradiction cannot be closed by editing prose.

### 3. D25's automatic download is weakened to optional behavior

**Decision:** D25 says a discovered application update downloads automatically
in the background; only installation and relaunch require the user's click.

**PRD distortion:** FR-20 says the app “may download” an available update. That
wording allows no automatic download, or a separate manual-download step, while
still satisfying the requirement.

**Required reconciliation:** Make background download after discovery normative
and automatic. Retain the already-correct rule that checking/downloading does
not install or restart and that explicit Restart is the mutation gate.

### 4. Two accepted D25a consequences are absent from their proper lanes

**Decision:** D25a accepts (a) `appUpdate:status` as a sixth event specifically
because application updates must not become package `Operation`s in
`op:status`, History, or the scheduler, and (b) `--no-sign` for the secretless CI
bundle smoke because the configured updater public key otherwise requires the
matching private key.

**PRD distortion:** FR-20/FR-21 cover update states, the menu, and privilege
handling, but no requirement preserves the sixth event's separation from
package-operation state. The PRD also does not distinguish the unsigned
`--no-sign` smoke artifact from the signed/notarized Release Candidate; RE-2's
“production builds” wording leaves that boundary ambiguous.

**Required reconciliation:** In Test Infrastructure, require the native
application-update event contract and prove it does not enter `op:status`,
History, or scheduling. Also state that the routine CI bundle smoke uses
`--no-sign` and is capability evidence only. In Release Evidence, explicitly
exclude that unsigned smoke artifact from candidate attestation; the candidate
must still satisfy RE-3 through RE-9.

### 5. D26 is acknowledged but not made testable or normative downstream

**Decision:** D26 permits a newline only after a verbatim member of the closed
`UNTERMINATED_NOTICES` list when output is glued behind it. A notice already at
the end of a buffer is left alone. Regexes, `==>` heuristics, and rewriting
unrelated Manager output are rejected; the accepted cost is that those repaired
transcript lines are not byte-identical to the child stream.

**PRD distortion:** §0.1 mentions the exception, but FR-15, NFR-4, and TIR-4 do
not carry its boundaries or acceptance consequence. A heuristic splitter, an
over-broad allowlist, or a gate with no negative evidence can still pass.

**Required reconciliation:** Add the faithful-except-D26 rule to FR-15/NFR-4 and
require TIR-4 evidence for exact glued literals, literals already terminated,
repeated literals, near-matches, and unrelated output. State explicitly that
only the allowlisted repair may make retained transcript text differ from the
child stream.

## No other material reconciliation gap

The remaining decisions are either represented by the PRD's observable
requirements and non-goals or are implementation choices appropriately left to
the authoritative decision/specification layer rather than repeated in this
readiness PRD.
