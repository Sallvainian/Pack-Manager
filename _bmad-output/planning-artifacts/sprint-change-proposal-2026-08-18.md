# Sprint Change Proposal — 2026-08-18

Produced by `bmad-correct-course`. Trigger: `docs/DECISIONS.md` **D40**. Approved by the owner (Sallvain) 2026-08-18 — the owner directed this run after accepting D39–D42, so approval was granted in advance for exactly this scope; mode: batch.

## 1. Issue summary

D40 (2026-08-18, the owner-modified closure of `prd.md` §9 Q2) creates a small piece of product scope that no story owns: **every absent Manager card carries a copyable install hint** — extending mas's existing treatment to all six Managers — plus **all-absent first-run guidance copy** ("No package managers found. Install one yourself — Homebrew is the usual first — then click Refresh All."). D40 explicitly **rejects an executing Install button** (installer non-goal, the no-sudo promise SM-3, no shell surface, FR-23's closed immediate-execution set).

Evidence, verified against `HEAD` this run:

- `src-tauri/src/detect.rs:272-277` — `install_hint()` returns `Some` only for `Mas`; its own comment says "SPEC F1 names only mas's hint".
- The rendering pipeline is already universal: `Sidebar.tsx:130-132`, `ManagerCard.tsx:78-80`, and `ManagerPane.tsx:187` each render `info.installHint` through `CopyableCommand` whenever the field is present, and `ManagerInfo.installHint` already exists in the IPC contract (`src/lib/ipc/types.ts:114`, optional field).
- `DashboardView.tsx` maps Manager cards with no all-absent state panel.
- `prd.md:738` (§9 Q2) states the all-absent case is undefined; D40 now defines it.

Discovery context: the four §9 open questions were closed as D39–D42 on 2026-08-18 after an adversarially verified recommendation pass; the owner modified Q2's closure to add the hints.

## 2. Impact analysis

- **Epic impact:** Epic 2 ("Make Detection and Refresh Fail Independently and Recover Usefully") is the natural home — its outcome line names absence behavior, and FR-1 (detection/absence) is already one of its cross-cutting FRs. One new story, **2.5**; Story 2.2 is unaffected. No other epic is touched; Epic UX-PB is untouched (the redesign's System Summary Card / State Panel consume D40's copy when they are built, but this story targets the shipping UI).
- **Story impact:** add Story 2.5 (below). No existing story text changes.
- **Artifact conflicts:**
  - `epics.md` — add Story 2.5; update Epic 2's retained-stories lines; add a "partly revived" note to the FR Coverage Map's FR-1 entry, mirroring FR-14's precedent wording.
  - `sprint-status.yaml` — add the `2-5` key **by hand** (the generate step drops all Epic UX-PB keys; never regenerate — 6-6 precedent).
  - `prd.md` — FR-1's "install hint where one is known" limb and the §9 closures; routed to the `bmad-prd` Update run the owner has already directed, citing D39–D42. Not edited here.
  - `_bmad-output/specs/spec-shipped-behavior-gaps/` — gains a capability for Story 2.5 via a `bmad-spec` re-derive so the two spec kernels keep covering the whole queue.
  - Architecture — **no impact**: no new invariant, no IPC schema change (`installHint` exists; populating more values may require regenerating the representative contract fixture with `PM_UPDATE_CONTRACT=1`, an implementation detail).
  - UX documents — **no required change**: `DESIGN.md` already reserves the "no Managers" states; the copy is recorded in D40. An optional future `bmad-ux` Update may inline the copy.
- **Technical impact:** small — a static hint table in `detect.rs` plus tests, an all-absent state panel in `DashboardView`, possible contract-fixture regeneration.

## 3. Recommended approach

**Direct Adjustment** (checklist option 1): one new story inside the existing Epic 2 structure. Effort: Low. Risk: Low. Rollback and MVP review are not applicable — nothing is being undone and MVP scope is unchanged. The story is independent, buildable now, and joins the six-survivor queue without resequencing anything.

## 4. Detailed change proposals

### 4.1 `epics.md` — new Story 2.5 (inserted after Story 2.2)

Full story text as applied:

> ### Story 2.5: Offer Copyable Install Guidance for Absent Managers
>
> Added 2026-08-18 by this proposal, implementing `docs/DECISIONS.md` **D40**. Like Story 6.6, this postdates the D33 rescope: it is new scope decided by the owner, not a resurrected triage story.
>
> As a Pack-Manager user,
> I want every absent Manager to show a copyable install command, and an all-absent machine to tell me where to start,
> So that a Manager I lack — or a machine with none at all — hands me the terminal command instead of a dead end.
>
> **Story Contract:**
>
> - FR and requirement links: FR-1 (the install-hint limb, extended by D40 from "where one is known" to all six)
> - Required test level: Unit plus component
> - Governing invariants: AD-4 (hints are static data through existing typed surfaces; no new process effect), AD-27 (focus is a 2px `outline` in `--color-focus-ring` on any added control; never a `ring-*`/box-shadow)
> - Governing decisions: **D40** (copyable hints, never an executing Install button — installer non-goal, SM-3 no-privilege, no shell surface, FR-23's closed immediate-execution set may not grow); D14 (copy-to-terminal is the product's handoff ethos)
> - Dependencies: none — buildable now; no Epic UX-PB surface is involved. The existing `installHint` render paths (`Sidebar`, `ManagerCard`, `ManagerPane`) are reused, not rebuilt.
>
> **Acceptance Criteria:**
>
> **Given** any of the six Managers is detected absent
> **When** its Dashboard card, sidebar entry, and Manager workspace absent state render
> **Then** each shows that Manager's copyable install command through the existing `CopyableCommand` treatment (extending the mas behavior to all six), the command is copy-only, nothing in the app can execute it, and no `Install` button or other execution affordance exists (D40)
> **And** the absent presentation still explains that Refresh All / Re-detect picks the Manager up after installation.
>
> **Given** the indicative commands recorded in D40 (brew → the official Homebrew installer one-liner; mise → `brew install mise`; npm → `mise use -g node@lts`; uv → `mise use -g uv`; rustup → the official rustup installer one-liner; mas → `brew install mas`)
> **When** the hints are implemented
> **Then** each shipped hint is verified against that Manager's current official installation documentation, and hints are static per-Manager strings — no context-aware suppression or rewriting based on which other Managers are present, per D40's rejected-alternatives record.
>
> **Given** a machine where all six Managers are absent
> **When** detection completes
> **Then** the Dashboard presents a state panel carrying D40's guidance — no package managers were found, the user installs one themselves (Homebrew is the usual first), and `Refresh All` re-detects afterward — the system never reads as an error or `Warning` for absence alone, and `Update Everything` stays disabled with a reason.

### 4.2 `epics.md` — Epic 2 retained-stories lines (two places)

OLD: `**Retained stories:** 2.2. The other three were triaged out on 2026-07-25 (D33); …`
NEW: same, plus: `Story 2.5 was added 2026-08-18 by sprint-change-proposal-2026-08-18.md, implementing D40.`

### 4.3 `epics.md` — FR Coverage Map, FR-1 entry

Append, mirroring FR-14's "partly revived" precedent: the install-hint limb of FR-1 is **Epic 2 — Story 2.5** per D40 (2026-08-18), which postdates the D33 rescope; the detection-proving limbs stay as the triage left them.

### 4.4 `sprint-status.yaml` — hand-added key

`2-5-offer-copyable-install-guidance-for-absent-managers: backlog` under the epic-2 block, with a NOTE comment mirroring 6-6's: added by hand under this proposal; do not regenerate the file.

### 4.5 `spec-shipped-behavior-gaps` — kernel update (via `bmad-spec` derivation)

New CAP-8 for Story 2.5; the "seven stories" framing becomes eight; success signal extended. Applied through the spec's `.memlog.md` and re-derive, per that spec's discipline.

## 5. Implementation handoff

- **Scope classification: Minor** — direct implementation by the Developer agent when the story is picked up from the backlog; no backlog reorganization, no replan.
- **This proposal's own edits** (4.1–4.5) are planning-artifact changes applied by this run.
- **PRD reconciliation** (§9 closure + FR-1 wording) — `bmad-prd` Update citing D39–D42, already directed by the owner as the companion action to this run.
- **Success criteria:** Story 2.5 reaches done with its three ACs green; `sprint-status.yaml` carries the key; the FR Coverage Map explains FR-1's revived limb; the spec kernel covers the story.

## Checklist record

1.1 Done (trigger is a decision, not a story — D40; 6-6/D30 precedent) · 1.2 Done (new owner requirement) · 1.3 Done (code citations above) · 2.1–2.2 Done (Epic 2 gains Story 2.5) · 2.3–2.5 Done/N-A (no other epic affected, no resequencing) · 3.1 Done (PRD FR-1 + §9, routed to bmad-prd) · 3.2 Done (no architecture impact) · 3.3 Done (no required UX-doc change; optional bmad-ux copy inline) · 3.4 Done (spec kernel + sprint-status; no CI/deploy impact) · 4.1 Viable-selected (Direct Adjustment, Low/Low) · 4.2 Not viable (nothing to roll back) · 4.3 N/A (MVP unchanged) · 4.4 Done · 5.1–5.5 Done (this document) · 6.1–6.2 Done · 6.3 Done (owner pre-approval, 2026-08-18) · 6.4 Done (key added by hand) · 6.5 Done (handoff above)
