# Reconciliation — archived PRD product substance vs. `prd.md` (2026-07-25)

**Reviewer lane:** archived PRD product substance.

## Inputs

| Role | Path |
| --- | --- |
| Under review | `_bmad-output/planning-artifacts/prds/prd-Pack-Manager-2026-07-25/prd.md` (659 lines) |
| Companion | `_bmad-output/planning-artifacts/prds/prd-Pack-Manager-2026-07-25/addendum.md` (69 lines) |
| Archived source extraction (READ-ONLY) | `_bmad-output/archive/2026-07-24-scope-recalibration/planning/prds/prd-Pack-Manager-2026-07-22/extract-product-intent.md` (616 lines) |
| Archived reconciliation (READ-ONLY) | `_bmad-output/archive/2026-07-24-scope-recalibration/planning/prds/prd-Pack-Manager-2026-07-22/reconcile-spec.md` (93 lines) |

Line counts from `wc -l` on the four files.

**Nothing in the archive is proposed for copying into `planning-artifacts/`.** Every recommendation below is a statement to *author* in the new PRD, in the new PRD's own voice. No recommendation reinstates the readiness apparatus D33 retired, and none reinstates the criteria D37 removed.

## Scope boundaries honored

- No finding concerns a P0 criterion count, a coverage percentage, a versioned scenario contract, an evidence manifest, a candidate freeze, a multi-host environment, or a `contracts/` directory (D33).
- No finding concerns keyboard operability, VoiceOver, live-region announcements, or focus restoration (D37). Where the archived source carried those obligations — `extract-product-intent.md:263`, `:390–398`, `:462–470` — their absence from the new PRD is treated as correct and is not reported.
- No finding applies commercial-launch expectations. `extract-product-intent.md:601` ("Release cadence/rollback") and `:595` ("Success measures") are deliberately not carried; §8 of the new PRD answers the latter at the right scale.
- No finding treats a **Planned — D27–D30** tag as a defect.

---

## Part 1 — the five gaps `reconcile-spec.md` named, retested against the new PRD

`reconcile-spec.md:13` — "Five material gaps remain." Each was raised against the *old* `prd.md`. The test is whether the new PRD fixed it or reinherited it.

### Gap 1 — the one permitted cross-Manager deduplication rule — **FIXED**

Archived complaint, `reconcile-spec.md:17`: "### 1. The one permitted cross-Manager deduplication rule is missing".

New PRD, `prd.md:203`:

> "The one permitted cross-Manager deduplication is the Rust rule (D10): a single Upgrade Plan never contains both mise's `tool:rust` and rustup toolchains; the mise entry is excluded with a visible reason. No broader cross-Manager deduplication is performed."

Reinforced at `prd.md:574`: "**A cross-Manager deduplicator.** The single Rust rule (D10) is the only exception, and it is scoped to one plan."

Both limbs the archive asked for — the exception *and* the continued exclusion of broader dedup — are present. Closed.

### Gap 2 — D25's automatic download weakened to optional — **FIXED**

Archived complaint, `reconcile-spec.md:31`: "### 2. D25's automatic download behavior is weakened to optional", against "PRD FR-20 says the app "may download" in the background" (`reconcile-spec.md:36`).

New PRD, `prd.md:458`:

> "A newer authorized release downloads automatically in the background. Automatic **download** is required behavior, not an optional outcome — installation is the machine mutation, and that stays user-gated."

The gate the archive asked to retain is at `prd.md:470`: "A downloaded update installs only after the user chooses Restart to update. Never a silent install, never a silent restart." And the manual-install fallback at `prd.md:473`: "A non-writable install location produces manual-install-required, never an administrator prompt."

Closed, and closed emphatically — the new text pre-empts the exact softening the archive caught.

### Gap 3 — decision-precedence repairs not fully carried — **PARTIALLY REINHERITED**

The gap had three bullets. Two are fixed; one is reinherited.

**Bullet 2 — D26's transcript rule — FIXED.** `reconcile-spec.md:53`: "PRD FR-15/NFR-4 do not state D26's product rule". New PRD `prd.md:362`:

> "Transcript content is faithful to child output, with exactly one exception: D26's closed, literal list of unterminated `mas` notices, after which one readability newline may be inserted. No general heuristic rewriting."

That is the archived rule verbatim in substance, in FR-15, where the archive asked for it.

**Bullet 3 — the D23a truth correction misplaced in a test lane — moot.** The TIR lane the archive referenced no longer exists; D33 retired it. Not reported.

**Bullet 1 — superseded SPEC assertions unmarked — REINHERITED.** `reconcile-spec.md:50–52`:

> "- `SPEC.md` still calls itself authoritative while retaining superseded
>   `mas`-absent/unverified and ad-hoc/notarization-out-of-scope statements in
>   F1, F10, P2, target-machine facts, adapter contracts, and tests."

The new PRD fixed the *authority* half and the *notarization* half:

- `prd.md:34` — "**It is no longer the requirements authority.**"
- `prd.md:44` — the §0.1 defect table row: "| §1 P2 (line 128) | `notarized DMG` is out of scope | Line 108 of the same file says delivery is "signed, notarized, and stapled"; D25/D25a superseded D20 and the pipeline has notarized since |"

It did **not** fix the `mas` half. `grep -c 'D23' prd.md` returns **0**; the §0.1 table has no `mas` row. Meanwhile `prd.md:34` grants SPEC continuing authority over exactly the sections that carry the stale text:

> "It remains valuable and remains authoritative for UI specification, architecture detail, parser contracts, and the test plan."

The stale text is still live at `HEAD`:

- `docs/SPEC.md:138` — "…rustup standalone in `~/.cargo/bin`; mas absent."
- `docs/SPEC.md:402` — the adapter table's mas row ends "— (UNVERIFIED live; labeled)"
- `docs/SPEC.md:418` — "**mas**: list `^(\d+)\s+(.+?)\s+\((\S+)\)$`; outdated `^(\d+)\s+(.+?)\s+\((\S+) -> (\S+)\)$` — from docs, UNVERIFIED live (mas absent), synthetic fixtures labeled"
- `docs/SPEC.md:757` — "`mas_outdated_synthetic_parses`; `mas_shell_error_never_reaches_parser` (documents detection gating)."

Superseded by `docs/DECISIONS.md:97`: "### D23a. Resolved 2026-07-22 — mas is verified live", and `docs/DECISIONS.md:101–102`: "The UNVERIFIED label is withdrawn and the README limitation deleted."

This is a product-truth defect, not a test-plan defect: the new PRD's Glossary closes the Manager set at six including `mas` (`prd.md:102`) and FR-1 tags detection **Shipping** (`prd.md:137`), while the source it delegates parser authority to says that Manager is absent from the machine and its parsing unverified. See Finding 4.

### Gap 4 — the running-operation quit guard — **FIXED**

Archived complaint, `reconcile-spec.md:65`: "### 4. The running-operation quit guard is omitted from product behavior".

New PRD, `prd.md:349` (FR-14 consequence): "Quitting with work in flight presents an explicit choice and does not silently discard it."

And it is explicitly separated from crash recovery, which the archive asked for (`reconcile-spec.md:74–75`, "leaving crash/forced-quit recovery as a separate acceptance path") — crash reconstruction lives in FR-15 (`prd.md:360`: "Work with a start record and no finish record surfaces as Interrupted on the next launch."). The residual — queued-but-not-running work — is correctly parked rather than invented, at `prd.md:629`: "**What happens on quit with work *queued* but not running?** The running-Operation quit guard is defined."

The new statement is outcome-level rather than naming the two labels SPEC §4.10 named. That is the right altitude for a PRD. Closed.

### Gap 5 — the control-plane experience flattened into generic UI quality — **MOSTLY FIXED, ONE LIMB REINHERITED**

Archived complaint, `reconcile-spec.md:77–78`: "### 5. The distinctive control-plane experience is flattened into generic UI / quality", with the specific ask at `reconcile-spec.md:89–92`: "preserve the outcome-level information architecture and recognizable package/update identity in FR-19, including the app icon".

Fixed:

- **The icon is now a product requirement.** `prd.md:421`: "The visual identity is the approved "Aurora Control Deck" palette in `DESIGN.md`, adopted by D35, with a recognizable package/update application icon." The archive's specific objection — "The icon appears only as release attestation in RE-4" (`reconcile-spec.md:86`) — no longer holds.
- **The surfaces are enumerated rather than reduced to "consistent state language."** `prd.md:420`: "One coherent dark-only interface spans the Dashboard, Manager navigation and workspaces, the Upgrade Plan, Activity, History, Settings, status surfaces, and application menus."
- **VersionDelta survives as a named product term** — Glossary `prd.md:115`, FR-2 `prd.md:158`, FR-19 `prd.md:426`.
- **Routing explanation and phase labels survive** — `prd.md:186` ("A routed action names both subject and executor in plain language") and `prd.md:170` ("Each Manager shows its own loading, phase, timeout, and error state").

Reinherited: the **Dashboard's one-glance cross-Manager awareness** is named as a surface but never stated as an outcome. See Finding 5.

---

## Part 2 — product substance the new PRD lost beyond the five gaps

Checked systematically: principles PI-1…PI-11 (`extract-product-intent.md:56–131`), P0 capabilities F1–F12 and F18 (`:137–306`), journeys A–H (`:310–375`), product terms (`:534–552`), and the ambiguity register (`:582–606`).

### PI-1…PI-11 coverage

| Principle | Carried into | Verdict |
| --- | --- | --- |
| PI-1 Manager truth authoritative | FR-2 (`prd.md:149–160`) | Carried |
| PI-2 Ownership/routing discovered | FR-4 (`prd.md:176–188`) | Carried |
| PI-3 Conflicting mutations serialize | FR-9 (`prd.md:265–275`) | Carried |
| PI-4 No privilege prompt path | FR-12 (`prd.md:312–323`), FR-14 (`prd.md:345–346`) | Carried |
| PI-5 Failures isolated, data retained | FR-3 (`prd.md:162–174`), NFR-2 (`prd.md:515–519`) | Carried **except the per-Manager retry path** — Finding 6 |
| PI-6 Bulk work reviewable and exact | FR-7, FR-8, FR-9 | Carried **except the explicit-package-names rule** — Finding 2 |
| PI-7 Exclusions preserve user intent | FR-5 (`prd.md:198–203`) | Carried, including the Rust rule |
| PI-8 Mutations require user action | §6 (`prd.md:567`), FR-21 | Carried |
| PI-9 Operations observable/reconstructible | FR-13, FR-15, NFR-4 | Carried |
| PI-10 Cancellation immediate and honest | FR-14 (`prd.md:348`) | Carried |
| PI-11 Settings/trusted state do not partially apply | FR-17 (`prd.md:393`), FR-8 (`prd.md:263`) | Carried |

Nine of eleven carried whole. The `PI-` numbering itself was deliberately not carried — but one reference to it survived, and now dangles. See Finding 7.

### Product-term coverage

Every term in `extract-product-intent.md:534–552` appears in the new PRD's Glossary (`prd.md:102–121`) except one:

- `extract-product-intent.md:546` — "**All-outdated intent:** A request to build a plan from all currently outdated, eligible items rather than a frozen explicit selection."

`grep -ci 'all-outdated' prd.md` returns **0**. This is not a cosmetic drop — it is the concept that told an implementer whether `Update Everything` freezes members at invocation or re-expands at confirm time. See Finding 1.

### Journey coverage

Archived journeys A–H (`extract-product-intent.md:310–375`) map onto AJ-1…AJ-6 (`prd.md:87–92`) without loss of *substance*: A→AJ-1, C→AJ-2, D→AJ-2/AJ-3, E→AJ-3, F→AJ-4, G→AJ-5, H→AJ-6. Journey B ("Review one manager", `:319`) has no AJ counterpart, but its content is carried by FR-5 and FR-11 and the PRD's decision to mirror `EXPERIENCE.md`'s IDs rather than mint a fourth namespace (`prd.md:83`) is sound. **Not reported.**

Journey G step 3 ("The user reveals the full log in Finder", `:366`) is *not* carried — see Finding 3.

---

## Findings

### Finding 1 — HIGH — `Update Everything` is the product's headline action and has no FR

`Update Everything` was a P0 capability in the archive with its own required behavior, `extract-product-intent.md:176–189` (F4), opening at `:182`:

> "- "Update Everything" and per-manager "Upgrade all" open a plan sheet."

In the new PRD, `grep -c 'Update Everything' prd.md` returns **2**, and neither is a requirement:

- `prd.md:88` — the AJ-2 journey row: "**AJ-2** | Review and authorize Update Everything — eligible work populates the Upgrade Plan with exclusions and reasons…" — a journey narrative, realized by "FR-6, FR-7, FR-8, FR-9, FR-11".
- `prd.md:212` — the §4.2 group description: "Every path to a mutation — a Package row, a Manager header, a Manager-wide action, `Update Everything` — converges on one reviewable Upgrade Plan".

None of the five FRs AJ-2 cites states what `Update Everything` does. FR-6 is scoped to a Manager's filter (`prd.md:226`: "The header checkbox adds or removes every eligible Package matching the active filter"); FR-10 is one Package; FR-11 is one Manager's self-update. There is no requirement for the cross-Manager, cross-Package global action, and no requirement for `Upgrade all` at Manager scope either (`grep -c 'Upgrade all' prd.md` = 0).

The unstated semantics are load-bearing. `docs/SPEC.md:454` records that the plan builder treats "`null` remains all-outdated" as distinct from an explicit selection, and `epics.md:587` already resolved the question the PRD never asks:

> "**Given** a draft seeded by `Update Everything`, whose expansion was frozen into concrete members at the moment I invoked it — each carrying `Bulk { scope: Everything }` provenance that is never re-evaluated"

with the behavior at `epics.md:581`: "`Update Everything` seeds all eligible work while remaining editable".

That substance sits only in the Phase 3 artifact, and `addendum.md:52` routes that artifact's FR/NFR block to "cite this PRD rather than restate requirements". Executing that route deletes the only written statement of the product's primary action.

**Fix:** add an FR to §4.2 for the global and Manager-wide bulk staging actions, stating at minimum: they stage every currently eligible item into the persistent draft, membership is frozen into concrete identities at invocation and not re-expanded later, the result remains individually editable, and every exclusion (Pinned, self-updating, Current, Rust-dedup) applies. Restore **All-outdated intent** to the Glossary or explicitly retire it in favor of the frozen-membership model.

### Finding 2 — HIGH — PI-6's "no bare, indiscriminate update command" rule is gone

`extract-product-intent.md:95`:

> "- Bulk commands contain explicit package names; a bare, indiscriminate update command is not accepted as the plan."

`grep -c 'explicit package names' prd.md` = **0**; `grep -c 'indiscriminate' prd.md` = **0**.

What the new PRD does say, FR-7 `prd.md:245–246`:

> "- Every staged Package and Manager update appears in the plan before execution, grouped by Manager, showing `installed → latest`.
> - The exact commands are revealable on demand and are byte-equal to the commands actually spawned."

Both are one-directional: everything staged must appear, and what is previewed must equal what is spawned. Neither states the converse — that the spawned argv affects *nothing outside* the plan. A bare `brew upgrade` satisfies FR-7 exactly: it is previewable, it is byte-equal to what runs, and every staged Package genuinely appears in the plan. It also upgrades every Package the user deliberately unchecked.

That converse is the whole content of the §4.2 promise at `prd.md:212` — "*nothing runs that was not staged and shown*" — and of SM-2 at `prd.md:612`: "**SM-2: Zero unreviewed mutations.** No Package or Manager update ever runs that the user did not see staged first. A single violation is a P0 defect". The PRD asserts the outcome in two prose slogans and never makes it testable. NFR-1 (`prd.md:513`, "User exclusions and Manager protections remain authoritative in every path") gestures at it but constrains the plan, not the command vector.

The rule is real and observable in the source contract: `docs/SPEC.md:397` specifies "`brew upgrade <formulae…>`; `brew upgrade --cask <tokens…>`" and `:398` "`mise upgrade <tool…>`" — enumerated identities, never a no-arg form.

**Fix:** add to FR-7's consequences: every spawned command enumerates exactly the canonical identities staged in the confirmed plan; no Manager's no-argument bulk-upgrade form is an acceptable plan command. This is the assertion that makes SM-2 falsifiable.

### Finding 3 — HIGH — History must be preserved, but is never required to be inspectable

Archived F8, `extract-product-intent.md:237` and `:239`:

> "- Show session and prior-history records with manager, status, search, and filters."
> "- Let the user inspect the full command and transcript tail and reveal the transcript in Finder."

New PRD FR-15 (`prd.md:352–364`) states the requirement as "The user can answer "what ran, what happened, where is the evidence" after the fact" (`prd.md:356`), then lists six consequences that are **entirely about durability**: transcripts are written, crash records survive, Interrupted is reconstructed, PIDs are never signaled, D26's repair is the only exception, and retention is bounded to "History compacted to the newest 1,000 records" (`prd.md:363`).

Nothing requires the user to be able to reach a record. Verification:

- `grep -c 'History.*\(search\|filter\)' prd.md` = **0**
- `grep -ci 'reveal' prd.md` = **1**, and the single hit is `prd.md:246`, "The exact commands are revealable on demand" — a plan-preview affordance, not a Finder reveal.
- The four `Finder` hits (`prd.md:70`, `:145`, `:428`, `:551`) are all about launching from Finder, none about revealing a transcript.

This is a retention floor of 1,000 records with no stated way to find one. It also strands two downstream artifacts that assume the requirement exists:

- `epics.md:152` — "AJ-5: Prove searchable History, command/outcome/transcript reconstruction, Finder reveal, and privacy-preserving diagnostics export."
- `EXPERIENCE.md:438` — "1. Sallvain opens History and searches or filters for a prior execution."

And it undercuts SM-4 (`prd.md:617`): "When something fails, the transcript plus History answer "what ran and what happened" without reproducing the failure."

Note this is **not** the removed-accessibility lane. It is browse/search/filter/detail/reveal as product capability, exactly as the archive stated it.

**Fix:** add to FR-15's consequences that History is browsable and filterable by Manager, status, and free-text search, that a record exposes its full command and transcript tail, and that the on-disk transcript is revealable in Finder. State it at the Shipping/Planned altitude the rest of FR-15 uses.

### Finding 4 — MEDIUM — the superseded `mas`-absent/unverified SPEC assertions are unmarked, while the PRD hands SPEC authority over exactly those sections

Full grounding in Part 1, Gap 3 above. Summary of the collision:

- `prd.md:34` — "It remains valuable and remains authoritative for UI specification, architecture detail, parser contracts, and the test plan."
- `prd.md:36` — "A 2026-07-25 validation, re-verified against `HEAD` for this PRD, found it materially out of date on the following. These are recorded so no future reader reconciles back to a stale source:" — followed by seven rows, none about `mas`.
- `docs/SPEC.md:402` — "— (UNVERIFIED live; labeled)"; `docs/SPEC.md:418` — "UNVERIFIED live (mas absent), synthetic fixtures labeled"; `docs/SPEC.md:138` — "mas absent".
- `docs/DECISIONS.md:97` — "### D23a. Resolved 2026-07-22 — mas is verified live".
- `grep -c 'D23' prd.md` = **0**.

The §0.1 table exists precisely to stop a reader reconciling back to stale SPEC text. It omits the one stale claim that contradicts a **Shipping**-tagged FR: the Manager set is closed at six including `mas` (`prd.md:102`) and FR-1 is Shipping (`prd.md:137`).

**Fix:** add one row to the §0.1 table — SPEC §1 target-machine facts, the §5.5 adapter table, the mas parser paragraph, and the mas test names still describe `mas` as absent and its parsing UNVERIFIED; D23a withdrew that label on 2026-07-22 and retired both `_synthetic` fixtures. One row, no SPEC edit, consistent with `addendum.md:56`'s stance that SPEC defects are recorded rather than fixed.

### Finding 5 — MEDIUM — the Dashboard's one-glance cross-Manager awareness is named as a surface, never stated as an outcome

Archived desired outcome 1, `extract-product-intent.md:45`:

> "1. **One-glance awareness:** The user can quickly see manager availability, ownership, health, package counts, and pending updates."

`reconcile-spec.md:82–85` flagged the loss of this specific identity: "The source's qualitative identity is a one-glance Manager-card Dashboard, per-Manager panes, a persistent Activity Drawer, version-delta treatment, routing chips, phase labels, and an approved package/update app icon".

The new PRD fixed most of that list (see Gap 5 above) but the Dashboard remains a name on a list. `grep -c 'Dashboard' prd.md` = **2**:

- `prd.md:420` — inside FR-19's surface enumeration.
- `prd.md:229` — "The draft persists while the user navigates between Managers and the Dashboard".

No FR states what a user learns from it. FR-1's consequences route detection detail elsewhere — `prd.md:147`: "Detection details appear in the Environment Report and in the diagnostics export." FR-5 is scoped inside one Manager. The aggregate question the product exists to answer — how much is outdated across this machine right now — has no requirement, despite the Vision promising it at `prd.md:54`: "It detects what is installed, shows what each Manager says is outdated".

**Fix:** add a consequence to FR-19 (or FR-1) stating the cross-Manager awareness outcome: without navigating into a Manager, the user can see each Manager's availability, ownership, installed version, pending-update count, and error/absent/busy state. Outcome only — card geometry stays in `DESIGN.md`.

### Finding 6 — MEDIUM — what happens to the persistent draft after a confirmed plan is admitted is unstated

Archived F5, `extract-product-intent.md:200`: "- Clear selection after successful enqueue." Journey D step 4, `:343`: "4. On successful enqueue, selection clears."

D27 replaces transient selection with a persistent draft, so the *mechanism* is correctly obsolete — but the *behavior* has no successor. FR-6's persistence clause, `prd.md:229`:

> "The draft persists while the user navigates between Managers and the Dashboard, and every staged item is individually removable from the Upgrade Plan."

Persistence across navigation is stated. Persistence across a **confirmed execution** is not stated in either direction. FR-7's Planned clause (`prd.md:249`) covers only the empty case: "the sidecar is hidden when empty, appears on first addition, persists across navigation". FR-8 and FR-9 describe admission without saying what becomes of the draft that produced it.

Downstream does not recover it either. `epics.md:562` covers only removal of the last item — "**Then** the sidecar closes, the draft returns to empty, and nothing lingers in Activity or History" — and `epics.md:565–566` covers relaunch. The nearest statement is `EXPERIENCE.md:256`, "the sidecar always transforms automatically after confirmation", which is a surface transition, not a membership rule.

Left unstated, a build can retain post-admission membership containing Packages that were just upgraded; the user's next confirm then re-submits completed work, and FR-8's stale-plan machinery (`prd.md:260`) only catches it *after* the user has reviewed and confirmed a plan that should never have been offered.

**Fix:** state in FR-6 or FR-7 what the draft holds after a plan is confirmed and atomically admitted — cleared, or retained-minus-admitted-members with a stated reason.

### Finding 7 — LOW — `prd.md:104` cites `PI-1`, which exists nowhere in the new planning artifacts

Glossary entry, `prd.md:104`:

> "- **Outdated** — a Manager's verdict that an update exists. Pack-Manager never infers it. See PI-1 in FR-2."

`grep -c 'PI-' prd.md` returns **1** — that reference itself. `sed -n '149,161p' prd.md | grep -c 'PI-1'` returns **0**: FR-2 carries no `PI-1` label.

`PI-1` is the archived principle numbering, `extract-product-intent.md:56`: "### PI-1 — Manager truth is authoritative". Dropping the `PI-` namespace is a defensible call and consistent with `prd.md:83`'s stated aversion to a fourth ID namespace. The residual pointer is not: it sends a downstream reader to a label that resolves only inside a read-only archive the PRD elsewhere instructs them not to reload (`prd.md:30`, "No archived file was copied or moved into `planning-artifacts/`").

**Fix:** change to "See FR-2." Single-word edit.

### Finding 8 — LOW — PI-5's per-Manager retry path after a failed refresh is not carried

`extract-product-intent.md:89`: "- Offline and timeout states are shown per manager with a retry path." Journey A step 6, `:317`: "6. Failed managers retain stale data and offer Retry/View log while other managers remain usable."

New PRD FR-3 carries the state and the stale retention — `prd.md:170` ("Each Manager shows its own loading, phase, timeout, and error state") and `prd.md:171` ("A failed refresh retains the Last-good Snapshot, keeps it browsable, and labels it stale") — but no retry affordance. FR-16 keeps only the log link (`prd.md:376`: "`View log` is exposed only when a corresponding log actually exists — the action never dangles.").

All five `Retry` occurrences in the new PRD (`prd.md:90`, `:336`, `:354`, `:364`, `:592`) are the D29 Plan-Attempt Retry, a different mechanism. The archived affordance is visible in the source at `docs/SPEC.md:267`: "error (red left border, one-line summary + ≤2 stderr lines mono + Retry + View log)".

Low because the user can re-invoke Refresh; it is a friction loss, not a capability loss.

**Fix:** add "and offers a direct retry" to FR-3's error-state consequence, or note deliberately that Refresh All is the retry path.

---

## Not reported, and why

| Archived item | Why not a finding |
| --- | --- |
| Keyboard/accessibility contract (`extract:390–398`, `:462–470`), F10's "keyboard operation … completion announcements" (`:263`), accessibility verification ambiguity (`:604`) | Removed by D37. Their absence is the decision. |
| F12 launch quality gates (`:284–293`), release-gate list (`:523–530`) | D33 retired the readiness apparatus; `prd.md:30` states the replacement. |
| Success measures ambiguity (`:595`), release cadence/rollback (`:601`), persona/audience (`:593`) | Answered at the right scale by `prd.md` §8 / §2, or correctly out of scope for a 3-download personal utility. |
| Journey B "Review one manager" has no AJ | Substance carried by FR-5 and FR-11; the mirror-`EXPERIENCE.md`-IDs decision (`prd.md:83`) is sound. |
| Duplicate global actions, Sidebar vs Dashboard header (`:597`) | UX placement, correctly outside a PRD. |
| Single-row trust timing (`:598`) | Resolved by FR-10 (`prd.md:283–287`) — no immediate execution at all. |
| Supported macOS range (`:594`) | Answered: NFR-7, `prd.md:551`, macOS 15.0 per D31. |
| Health fixes / snapshot cache / notifications / detail popover / Rust note (`:404–408`) | Correctly deferred with the runnable-fix rule preserved at `prd.md:599`. |
| P1 promotion verification (`:605`) | Carried as the `[NOTE FOR PM]` at `prd.md:599`. |
| Diagnostics redaction (`:602`), History deletion (`:603`), quit behavior (`:599`) | Correctly parked as Open Questions 5, 4, 1 (`prd.md:629–633`). |

---

## Verdict

The new PRD is a substantial improvement on the archived one against the archive's own test. Three of the five `reconcile-spec.md` gaps are closed outright (1, 2, 4), one is closed except for a single unmarked SPEC row (3), and one is closed except for the Dashboard outcome (5). Nine of eleven product principles carried whole, and the Glossary lost exactly one term.

The three material losses are all in the same lane and were not on the archive's radar because the D27 redesign created them: the product's headline action has no requirement (Finding 1), the argv-level rule that makes "nothing runs that was not staged" testable is gone (Finding 2), and History is required to exist but not to be reachable (Finding 3). All three are additive fixes to existing FRs. None requires a new section, a new ID namespace, or anything D33 or D37 retired.
