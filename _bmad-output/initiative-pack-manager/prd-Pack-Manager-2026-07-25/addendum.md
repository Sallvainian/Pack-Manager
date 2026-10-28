# PRD Addendum — Pack-Manager

Companion to `prd.md` (2026-07-25). Holds material that is real and load-bearing but belongs to architecture, UX, or epics rather than to a requirements document. Nothing here is optional context — it is placed here so the PRD stays a statement of *what*, and so the downstream owner of each item can find it.

---

## 1. Mechanisms deliberately excluded from the PRD narrative

The PRD states outcomes. These are the mechanisms that produce them. Each is already specified in a named artifact; this section exists so nobody re-derives them into the PRD later.

| Mechanism | Lives in | Why it is not a requirement |
| --- | --- | --- |
| Search-path construction order, login-shell probe form, sentinel strings, constructed child environment | `docs/SPEC.md` §5.2 | The requirement is "detection works when launched from Finder" (FR-1). How the path is built is architecture. |
| Raw-path-before-canonicalization ownership classification | `docs/SPEC.md` §5.3, D3 | The requirement is accurate discovered ownership with inspectable evidence (FR-4). The algorithm is the means. |
| Adapter trait signatures, per-Manager command tables, parser regexes, recovery-parser wiring, fixture mechanics | `docs/SPEC.md` §§5.4–5.5, D7–D9 | FR-2's "fail visibly rather than invent state" is the requirement; parser shape is implementation. |
| Lock-set data structures, scheduler task design, semaphore, aging guard, process-group signalling | `docs/SPEC.md` §5.7, D4 | FR-9 requires conflict serialization, concurrency, and understandable queue states. The scheduler is how. |
| IPC command signatures, wire casing, payload structs, event subscription architecture, frontend store shape | `docs/SPEC.md` §5.9, `ARCHITECTURE-SPINE.md` AD-3 | The contract is architecture's to own. The PRD only records that the surface changes atomically. |
| Transcript header syntax, log library configuration, span names | `docs/SPEC.md` §6 | FR-15 requires durable, faithful, correlated evidence with stated retention. Format is implementation. |
| Test seams (`CommandRunner`/`FakeRunner`, `EventSink`/`VecSink`, `bridge.ts`/`fakeIpc`, paused time) and individual test names | `docs/SPEC.md` §7, `_bmad-output/project-context.md` | Determinism is a project standard, not a product requirement. |
| Updater API choice, Rust update-state module, release metadata transport, signing environment | D25, D25a, `.github/workflows/release.yml` | FR-20–FR-22 state the user-visible contract; the transport is architecture. |

**Technical detail that *did* earn a place in the PRD**, because it directly constrains observable trust rather than describing implementation: exact preview/execution agreement; one-use bounded Plan Capability with fail-closed eviction; stale-plan reconfirmation with zero enqueue; no shell strings and no privilege path; atomic all-or-none admission; Last-good Snapshot retention; interruption durability and the never-signal-recorded-PIDs rule; diagnostics privacy and symlink resistance; signed/notarized/stapled universal delivery with both architecture keys; and explicit restart-gated installation.

---

## 2. Rejected alternatives worth not relitigating

Condensed from `docs/DECISIONS.md`. Each of these gets proposed again by someone who does not know why it was refused.

- **Local version comparison** to decide outdatedness (D2). Real data includes `2.0.14-1`, `1.6.2.dev0`, `stable`, and commit hashes. Semver math produces confidently wrong verdicts.
- **Canonicalize-then-classify** for ownership (D3). Mise shims are symlinks to the mise binary, so this misroutes uv and npm to Homebrew. Verified live.
- **An in-JSON heuristic for self-updating casks** (D7). Provably wrong — captured cask fixtures carry concrete versions, not a `latest` marker. It is a two-call set difference or it is broken.
- **Blind retry on Homebrew lock contention** (D22). Our serialization prevents *self*-contention only; a user's terminal can hold the lock, and a retry loop fights a human.
- **`-ilc` for the login-shell probe** (D11). Interactive rc files can block on a TTY. Non-interactive `-l` with sentinels only.
- **Auto-killing orphan process groups at startup** (D12). PID reuse makes this unsafe. Journaled PIDs are evidence, never targets.
- **A confirmation dialog on cancel** (D13). Cancel is already a deliberate click on a specific thing, and it is time-critical.
- **Forcing or unpinning pinned formulae** (D15). It silently defeats an explicit user decision.
- **`ring-*` for focus** (D35). Tailwind's `ring-*` compiles to `box-shadow`, and WebKit does not paint `box-shadow` on native-appearance form controls. The app ships in WKWebView, so every checkbox had an invisible focus state while `:focus-visible` still matched `true`. `appearance: none` makes the ring paint and destroys the native checkmark, so it is not the fix. One mechanism — `outline` + `outline-offset` — everywhere.
- **Darkening `--color-accent` so white text passes contrast** (D36). The accent is the approved palette value and carries selection, running state, and navigation. Moving it to rescue one text pair moves every other surface. The dark-ink token is the fix.
- **`macos-latest` for CI and release runners** (D34). A floating label would move the signing and notarization environment without a commit.
- **Dropping the Intel architecture key from update metadata** (D32). The updater resolves its target from the compiled architecture, so removing the key strands installed Intel users with no signal at all.
- **Completing the readiness gate rather than retiring it** (D33). Its P1 rule was unsatisfiable by construction — 5 of 8 rows declared out of scope caps achievable coverage at 37.5% against an 80% minimum. Its P0 rule was achievable but required evidence infrastructure that was never built and is disproportionate to a tool with 3 lifetime installs.

---

## 3. Reconciliation queue created by this PRD

This PRD is now the requirements authority. Four artifacts contain statements it supersedes. The first three are workflow-owned — **none of those may be hand-edited.**

**Scope every run by the named sections below, never by a mention count.** An earlier draft of this table carried "10, 3, and 4 mentions", copied from `docs/DECISIONS.md` D37 rather than measured. Re-measured with `grep -ciE "voiceover|keyboard"`, the real figures are **13, 8, and 11** lines (21, 9, and 13 raw occurrences). Counts below are indicative only and will drift again; a run that stops when it has fixed *n* mentions will leave the file half-converted.

| Artifact | What is stale | Route |
| --- | --- | --- |
| `_bmad-output/planning-artifacts/epics.md` | Its FR/NFR block (lines 53–450) is Phase 2 content embedded in a Phase 3 artifact — it should now cite this PRD rather than restate requirements. FR-19 (line 89) and NFR-6 (line 113) still carry the D37-removed keyboard/VoiceOver and announcement obligations, and Story UX-PB.1d does too; ~13 lines in all. FR-17 also still describes `skipUpgradePlanConfirmation` and `autoOpenDrawer` without status qualification. **UX-PB.1d is not to be deleted** — D37 protects its pointer-hover ineligibility explanation by name; only its keyboard and VoiceOver limbs are in scope. | `bmad-correct-course` |
| `ARCHITECTURE-SPINE.md` (rev 9) | ~8 lines, including the manual pass at 332 and 941. Separately, the OPEN row at 1050 (transient selection vs plan membership) is **now closed** — see `prd.md` §9.1. Retiring it is not enough on its own: the row exists because no invariant models the relationship, and FR-6's batch membership operation still needs one. Write the invariant, then retire the row. | `bmad-architecture` (Update intent), resuming from the run folder's `.memlog.md` |
| `EXPERIENCE.md` | ~11 lines, across **four** sections, not one: `## Keyboard` (272), `## Package Grid keyboard model` (281), `## Focus transitions` (290–303), and `Accessibility Floor` (313–332). The Focus transitions matrix is line-by-line the "deterministic dialog/sidecar focus restoration" this PRD records as dropped — e.g. `EXPERIENCE.md:296` "Escape or backdrop dismissal \| Restore focus to the originating `Confirm # updates` action." A run that removes only the Accessibility Floor leaves three sections mandating what the PRD deleted. Its membership model (line 143) and its explanatory-disabled rule are **correct and survive** — only the accessibility limbs change. | `bmad-ux` (Update intent) |
| `docs/RELEASE-CHECKLIST.md` | The PRD makes this the release-readiness authority (§0) and the validation route for RP-1/RP-2 (§4.6), but at `HEAD` it still carries both criteria D37 removed **by name**: `:86` "**Keyboard and accessibility pass.** Tab and arrow navigation reach every control" and `:89` "One VoiceOver pass over the Upgrade Plan announces state changes and completion." D37 names this file first among the things it retires. | **Maintainer edit** — workflow-unowned, like `docs/SPEC.md` |

`docs/SPEC.md` is hand-written and owned by no workflow. Its defects are recorded in `prd.md` §0.1 rather than fixed here, because editing it is a maintainer decision about a file the BMAD chain does not own. Two changes would carry most of the value: a header pointing requirements authority at this PRD, and **adding F5 to the §0.1 supersession list**, whose omission is the single mechanical reason the membership-model conflict stayed live long enough to reach a spine OPEN row.

---

## 4. Shipping defects surfaced while writing this PRD

Found by reconciling requirements against code. None is a PRD defect; all four are real and none had a ticket. Recorded here so they are not lost — the PRD states the requirement, this list states what the build actually does.

1. **⌘A is swallowed on three views.** The handler calls `preventDefault()` before the select-all helper early-returns on views with no Package list, so on the Dashboard, History and Settings the shortcut blocks native select-all and puts nothing in its place. The search field escapes it, because the handler bails on editable targets first. Not an accessibility item — ⌘A is an Edit-menu action, so D37 does not excuse it. FR-6 must not inherit this when ⌘A is re-pointed.
2. **The quit guard is unwired.** `QuitGuardDialog` exists and the dialog host renders it, but no close-requested handler exists in either process. Its only caller is the application-update path. Quitting with work in flight is therefore unguarded — the *restart* case is covered, the *quit* case is not.
3. **The npm-inside-mise warning does not render on the routed path.** The note is computed for mise-owned npm and then discarded on the routed branch, because the routed Route type carries no note field. Mise-managed npm that is not itself outdated — the ordinary state — shows ownership without the D21 consequence.
4. **The contrast guard and its on-fill ink tokens are uncommitted.** The 4.5:1 assertion and the `text-white` replacements exist only in the working tree, absent from `HEAD` `5972109`. Until they land, D36's guarantee is not enforced by CI.

## 5. Scale evidence

Recorded because it is the basis for D33 and D37, and because it will be questioned by anyone who reads the requirement count without it.

- 1 star, 0 forks.
- 12 releases inside a 48-hour window.
- 3 lifetime `.dmg` downloads. Of 30 total recorded downloads, 27 are the app's own updater traffic — 17 metadata polls and 10 payload fetches, both automatic under D25.
- One user, one machine, mouse-operated.

The prior planning stack sized for this was 83 stories: 28 real product stories (the D27–D30 redesign) and 55 evidence-production stories, each carrying one versioned scenario contract, all unassigned, against a contracts directory that did not exist and that no story created. That is the apparatus D33 retired and that this PRD does not reintroduce.
