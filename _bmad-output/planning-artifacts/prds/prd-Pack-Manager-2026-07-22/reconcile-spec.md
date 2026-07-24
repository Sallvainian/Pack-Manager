# Source reconciliation — `docs/SPEC.md`

## Input

- Source: `docs/SPEC.md`
- Reconciled against: `prd.md`
- Precedence applied: the PRD's stated source order, including D23a over D23,
  D25/D25a over D20 and stale delivery text, and D26 over the unqualified
  byte-faithful-transcript claim.

## Result

Five material gaps remain. Detailed implementation prescriptions, exact file
layouts, IPC shapes, parser mechanics, and individual test names were
intentionally excluded from this reconciliation.

### 1. The one permitted cross-Manager deduplication rule is missing

`SPEC.md` requires one plan containing both mise's Rust entry and rustup
toolchains to exclude the mise entry, explain the exclusion, and leave all
broader cross-Manager deduplication out of scope (`SPEC.md` §§1 F4, 5.7, 7.2).
The PRD preserves pinned and greedy exclusions and says broader deduplication is
out of scope, but never preserves this single Rust exception (FR-5–FR-9,
§10.2). That omission can produce duplicate or topology-conflicting mutation
work while still appearing compliant with the PRD.

**Reconciliation needed:** add the one-plan Rust exclusion and its visible
reason to the product planning requirements; continue to exclude broader
cross-Manager deduplication.

### 2. D25's automatic download behavior is weakened to optional

D25's current product intent is check, then automatically download an available
Pack-Manager update in the background, while installation and relaunch remain
user-controlled. PRD FR-20 says the app “may download” in the background. That
changes a defined user journey into an optional implementation outcome and
weakens the in-app update promise.

**Reconciliation needed:** require automatic background download after an
update is found; retain the explicit Restart-to-update gate and
manual-install-required behavior.

### 3. Decision-precedence repairs are not fully carried into operative
requirements

The PRD correctly declares that `mas` is live-verified, delivery is
signed/notarized/stapled, and D26 permits one narrowly allowlisted transcript
readability repair (§0.1). However:

- `SPEC.md` still calls itself authoritative while retaining superseded
  `mas`-absent/unverified and ad-hoc/notarization-out-of-scope statements in
  F1, F10, P2, target-machine facts, adapter contracts, and tests.
- PRD FR-15/NFR-4 do not state D26's product rule: transcripts remain faithful
  except that a newline may be inserted after a closed literal list of known
  unterminated `mas` notices, with no general heuristic rewriting.
- Correction of stale D23a repository truth appears inside TIR-1 even though
  the PRD classifies it as the one product/correctness defect. The recurrence
  guard belongs in test infrastructure; the truth correction itself does not.

**Reconciliation needed:** carry D26 into the transcript behavior requirement,
place the D23a truth correction in the product/source-correctness lane, and
mark or remove the superseded SPEC assertions so the authority chain is not
dependent on readers finding a later override.

### 4. The running-operation quit guard is omitted from product behavior

`SPEC.md` requires quitting while Operations are running to show the Operations
and offer “Cancel operations and quit” or “Keep running” (§4.10). The PRD
discusses crashes, forced quits, interruption recovery, and lifecycle evidence,
but it never states the normal user-initiated quit behavior. TIR-5 can test a
quit boundary, but it cannot define the missing product outcome.

**Reconciliation needed:** restore the running-operation quit guard as a
product lifecycle requirement, leaving crash/forced-quit recovery as a
separate acceptance path.

### 5. The distinctive control-plane experience is flattened into generic UI
quality

The source's qualitative identity is a one-glance Manager-card Dashboard,
per-Manager panes, a persistent Activity Drawer, version-delta treatment,
routing chips, phase labels, and an approved package/update app icon
(`SPEC.md` §§4.5–4.12). PRD FR-19 names most surfaces and accessibility
outcomes, but reduces their intended relationship and identity to “consistent
state language.” The icon appears only as release attestation in RE-4, so the
release lane is asked to prove a product/brand requirement that the product
lane never defines.

**Reconciliation needed:** preserve the outcome-level information architecture
and recognizable package/update identity in FR-19, including the app icon;
leave pixel values, component filenames, and layout measurements in the UX
specification. RE-4 should verify that defined product requirement rather than
serve as its only statement.
