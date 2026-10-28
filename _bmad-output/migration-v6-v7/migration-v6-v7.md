---
type: migration
title: Pack-Manager BMad v6 to v7 migration
status: approved
created: 2026-10-28
---

# Pack-Manager BMad v6 to v7 migration

Migration `method` 6 → 7 (`.claude/skills/bmod-method/migration-1.toml`), run through `bmad migrate`.
`<root>` is `_bmad-output/`. Nothing below has been applied yet; this plan waits for approval.

## Matching and scope

| Migration | Applies | Evidence |
| --- | --- | --- |
| method 6 → 7 | yes | `planning-artifacts/epics.md`, `implementation-artifacts/sprint-status.yaml`, dated `prds/`, `architecture/`, `ux-designs/` run folders, two `specs/spec-*/SPEC.md` + `stories.yaml`; no `active_initiative`, no `initiative-*/` |
| method 7-preview → 7 | no | no `initiative-*/` or `epic-*/` folder; re-check after this one |
| tea 6 → 7 | no | no `_bmad/tea/config.yaml` or `module-help.csv`; `[modules.tea]` holds only the 13 keys `bmad setup tea` asks |

## Questions and answers

| Question | Answer | Reason |
| --- | --- | --- |
| Planning content is `_bmad-output/planning-artifacts` (PRD, spine, UX run, 5 sprint-change proposals, readiness report, story triage, `epics.md`) and implementation content is `_bmad-output/implementation-artifacts` (`sprint-status.yaml`, `deferred-work.md`). Right, and any elsewhere? | Yes; plus the two spec kernels under `_bmad-output/specs/` | Folders read from `_bmad/config.toml` in the setup backup (`modules.bmm.*` already removed from the live config). `docs/` is hand-written and not BMad content. |
| Workspace / separate git history? | No — the store stays in this repo under `_bmad-output/` | User choice. Every move is a `git mv`. |
| `epics.md` and both spec kernels describe the same 36 stories; which becomes the epics? | The two spec kernels become the two epics | User choice. Avoids duplicating every story. The kernels are the newest planning (2026-08-18), cover the whole queue, and their `stories.yaml` order was topologically verified. `epics.md` acceptance criteria are merged into each entry; the Epic 2/3/6 grouping survives in `v6_key` and the envelope. |
| Five `sprint-change-proposal-*` files share a name apart from date/suffix | Keep all five live | User choice. Each records a separate applied change and is cited. |
| Rewrite the 9 path references inside the spine? | Yes, mechanical path-only substitution | User choice. Content is untouched; `bmad-architecture` Validate can confirm afterwards. |
| Stories in progress or in review — finish in v6 first? | Not asked | Every story and epic is `backlog`. |
| Fold unstarted stories' build-record files into entries? | Not asked | No v6 story files exist in the implementation folder. |

## Backup

`.v6-v7-migration-backup-20261028-1754/` (made by `bmad setup`, already holds `_bmad/`; its `.gitignore` is `*`).
`_bmad-output/` was copied into it as `_bmad-output/`: 218 files, `diff -rq` identical to the source.

## Inventory

218 files under `_bmad-output/`. 137 are git-tracked; 81 are git-ignored, all under
`planning-artifacts/ux-designs/ux-Pack-Manager-2026-07-23/.working/` (mockup working files and PNGs).

| Location | Files | Treatment |
| --- | --- | --- |
| `planning-artifacts/prds/prd-Pack-Manager-2026-07-25/` | 24 | move whole |
| `planning-artifacts/architecture/architecture-Pack-Manager-2026-07-23/` | 35 | move whole |
| `planning-artifacts/ux-designs/ux-Pack-Manager-2026-07-23/` | 14 tracked + 81 ignored | move whole (ignored files with plain `mv`) |
| `planning-artifacts/` flat files | 8 (`epics.md`, 5 proposals, readiness report, story triage) | `epics.md` → `archive-v6/`; others into folders by stem |
| `implementation-artifacts/` | 2 | `sprint-status.yaml` → `archive-v6/`; `deferred-work.md` → initiative |
| `specs/spec-upgrade-plan-redesign/` | 4 | becomes an epic |
| `specs/spec-shipped-behavior-gaps/` | 3 | becomes an epic |
| `archive/` | 47 | stays in place (already the v7 `archive/`) |
| `test-artifacts/` | 0 (empty) | stays in place (TEA's `test_artifacts`) |

No story files, retrospectives, action items, `epic-N-context.md` caches, pre-release `spec-<slug>/spec-<slug>.md` folders, brainstorming, research or party-mode folders exist.

## Repository

Work happens on a local branch `chore/bmad-v7-migration` cut from `main` (clean at `3f64e2b`). One commit per step 7–14, message `chore(bmad): v7 migration step <n> — <name>`, staged by explicit path (never `git add -A`), backup never staged. No commit triggers a release (`chore:`). Nothing is pushed; merging to `main` is the user's call.

## Target tree

```
_bmad-output/
  initiative-pack-manager/                         # git mv planning-artifacts
    initiative-pack-manager.md                     # NEW envelope
    prd-Pack-Manager-2026-07-25/                   # from prds/, all 24 files
      prd-Pack-Manager-2026-07-25.md               # renamed from prd.md; gains `type: prd`
    architecture-Pack-Manager-2026-07-23/          # from architecture/, all 35 files
      architecture-Pack-Manager-2026-07-23.md      # renamed from ARCHITECTURE-SPINE.md; gains `title`
    ux-Pack-Manager-2026-07-23/                    # from ux-designs/, all 95 files incl. .working/
      ux-Pack-Manager-2026-07-23.md                # NEW router: DESIGN.md, EXPERIENCE.md, validation-report, mockups, reviews
    sprint-change-proposal-2026-07-25/sprint-change-proposal-2026-07-25.md
    sprint-change-proposal-2026-07-25-spine-rev8/sprint-change-proposal-2026-07-25-spine-rev8.md
    sprint-change-proposal-2026-07-25-spine-rev10-residuals/sprint-change-proposal-2026-07-25-spine-rev10-residuals.md
    sprint-change-proposal-2026-08-11/sprint-change-proposal-2026-08-11.md
    sprint-change-proposal-2026-08-18/sprint-change-proposal-2026-08-18.md
    implementation-readiness-report-2026-07-25/implementation-readiness-report-2026-07-25.md
    story-triage-2026-07-24/story-triage-2026-07-24.md
    deferred-work.md                               # from implementation-artifacts/, unchanged
    tickets/
      tickets.toml                                 # [[epic]] 1 upgrade-plan-redesign, 2 shipped-behavior-gaps
      epic-upgrade-plan-redesign/
        epic-upgrade-plan-redesign.md              # NEW, rendered from SPEC.md (16 CAPs)
        .memlog.md                                 # moved unchanged from the spec folder
        story-map.md                               # companion, moved; References line
        tickets/tickets.toml                       # 28 [[entry]]
      epic-shipped-behavior-gaps/
        epic-shipped-behavior-gaps.md              # NEW, rendered from SPEC.md (8 CAPs)
        .memlog.md                                 # moved unchanged
        tickets/tickets.toml                       # 8 [[entry]]
    archive-v6/
      epics.md
      sprint-status.yaml
      spec-upgrade-plan-redesign/SPEC.md, stories.yaml
      spec-shipped-behavior-gaps/SPEC.md, stories.yaml
  backlog/.gitkeep                                 # NEW, empty
  archive/                                         # unchanged
  test-artifacts/                                  # unchanged (TEA)
  migration-v6-v7/migration-v6-v7.md               # this file
```

Removed once empty: `initiative-pack-manager/prds/`, `architecture/`, `ux-designs/`; `implementation-artifacts/`; `specs/`.
Every moved main file without `type`, `title`, `created` gains them; nothing else in it changes except path rewrites.

## Initiative envelope (`initiative-pack-manager.md`)

From the ticket template. Title from `core.project_name`; Description and Outcome from PRD §0–§1; Done when from PRD §8 Success Metrics; Constraints NFR-1…NFR-8 plus the PRD's binding non-negotiables; Boundaries from §6 Non-Goals.
Requirements, one line per PRD requirement, numbered so `R<n>` = `FR-n`: R1–R23 (FR-1…FR-23), R24 (RP-1), R25 (RP-2), each the PRD's normative line.
`sources:` the PRD; `companions:` the spine and the UX DESIGN/EXPERIENCE files.

Epic `covers` from the `epics.md` FR Coverage Map, with Epics 2/3/6 mapped onto `epic-shipped-behavior-gaps`:

| Epic | covers | Basis |
| --- | --- | --- |
| 1 `epic-upgrade-plan-redesign` | R8, R23 | map: FR-23 → UX-PB; FR-8 "Epic UX-PB now owns the whole requirement" |
| 2 `epic-shipped-behavior-gaps` | R1, R3, R5, R6, R7, R10, R14, R15, R16, R17, R18 | map: Epics 2/3/6 (FR-1 and FR-14 as their revived limbs) |

No epic: R2, R4, R9, R11, R12, R13 (triaged out by D33 — shipped) and R19–R22, R24, R25 (release checklist). Each becomes a `Parked:` Notes line with that reason.
No epic-level `after`: neither epic waits on the other as a whole; the cross-epic gates are entry-level.

## Epic envelopes

Each `SPEC.md` renders into its epic file: each `CAP-N` → `R<n> (CAP-N): <intent>; success: <success>` (same number), Constraints → Constraints, Non-goals → Boundaries, Assumptions → Notes, Why → Description, Success signal → Outcome. `parent: initiative-pack-manager`. No `status` line (both `backlog`). References: the parent's Requirements, the spine, DESIGN.md/EXPERIENCE.md, and (epic 1) `story-map.md`.

## Story entries

All 36 stories are `backlog` with no build record: entry only, no plan, no status. Each entry carries the `stories.yaml` title and description, then the story's full `epics.md` acceptance criteria and Dependencies line; `verify` is distilled from those criteria; `invoke_dev_with` goes into `notes`; `spec_checkpoint` → `plan_checkpoint`, `done_checkpoint` → `done_checkpoint`; `hitl = false`; `risk = "low"` except 5a (P0, SM-2) and 65 (AD-20 security-reviewed change) at `"high"`. `v6_key` is the `sprint-status.yaml` key, which contains the original `stories.yaml` id. `after` comes only from the `Dependencies:`/`Blocks:` lines `epics.md` declares, never from list order.

Epic 1 `epic-upgrade-plan-redesign` (list order = `stories.yaml` order):

| id | v6_key prefix | covers | after | checkpoints |
| --- | --- | --- | --- | --- |
| "1a" | ux-pb-1a-… | R1 | — | plan |
| "1b" | ux-pb-1b-… | R2 | 1a | |
| "1c" | ux-pb-1c-… | R1 | 1a | |
| "1d" | ux-pb-1d-… | R3 | 1a, 1c | |
| "1e" | ux-pb-1e-… | R4 | 1c | done |
| "2a" | ux-pb-2a-… | R5 | 1a–1e | plan |
| "2b" | ux-pb-2b-… | R5 | 2a | |
| "2c" | ux-pb-2c-… | R6 | 2b | |
| "2d" | ux-pb-2d-… | R6 | 2b | |
| "2e" | ux-pb-2e-… | R7 | 2b, 2d | |
| "2f" | ux-pb-2f-… | R8 | 2a | done |
| "3a" | ux-pb-3a-… | R9 | 2a–2f | plan |
| "3b" | ux-pb-3b-… | R9 | 3a | |
| "3c" | ux-pb-3c-… | R9 | 3b, 2d | |
| "3d" | ux-pb-3d-… | R10 | 3c | |
| "3e" | ux-pb-3e-… | R10 | 3d | |
| "3f" | ux-pb-3f-… | R11 | 3c | |
| "3g" | ux-pb-3g-… | R7 | 3d, 2e | done |
| "4a" | ux-pb-4a-… | R12 | 3a–3g | plan |
| "4b" | ux-pb-4b-… | R12 | 4a | |
| "4c" | ux-pb-4c-… | R12 | 4b | |
| "4d" | ux-pb-4d-… | R13 | 4b, 2b | |
| "4e" | ux-pb-4e-… | R8 | 4a, 2f | done |
| "5a" | ux-pb-5a-… | R14 | 1a–1e, 2a–2f | plan |
| "5b" | ux-pb-5b-… | R14 | 5a | |
| "5c" | ux-pb-5c-… | R14 | 5b | |
| "5d" | ux-pb-5d-… | R15 | 5a | |
| "5e" | ux-pb-5e-… | R16 | 4a–4e | done |

Epic 2 `epic-shipped-behavior-gaps` (ids lose their dash per the spec rule):

| id | v6_key prefix | covers | after | checkpoints |
| --- | --- | --- | --- | --- |
| 66 | 6-6-… | R7 | — | plan |
| 22 | 2-2-… | R1 | — | |
| 25 | 2-5-… | R8 | — | done |
| 35 | 3-5-… | R5 | "1.1a" | plan |
| 31 | 3-1-… | R2 | "1.1e" | |
| 32 | 3-2-… | R3 | "1.1d" | |
| 34 | 3-4-… | R4 | "1.5b" | |
| 65 | 6-5-… | R6 | "1.3d", "1.4d" | plan, done |

Counts: 36 entries = 36 non-epic, non-retrospective keys in `sprint-status.yaml` (28 + 2 + 4 + 2), all backlog.

## What stays in place

- `_bmad-output/archive/` — already the v7 `archive/`; AGENTS.md forbids moving files out of `2026-07-24-scope-recalibration/`. Its files keep their original paths.
- `_bmad-output/test-artifacts/` — TEA's `test_artifacts` config names it.
- Nothing else at the store root is unrecognized.

## Path rewrites

Inside the store: every old path (e.g. `_bmad-output/planning-artifacts/prds/prd-Pack-Manager-2026-07-25/prd.md`, `../../planning-artifacts/...`, `_bmad-output/specs/...`, `_bmad-output/implementation-artifacts/...`) is rewritten to its new path in every moved file outside `archive-v6/` and `archive/`, frontmatter lists, References, and prose alike, with relative links recomputed from each file's new folder. This covers the spine (user-approved, path-only).

Deliberate limits, recorded so they are not read as misses:
- **The four `.memlog.md` files are moved byte-identical and never rewritten** — AGENTS.md: a memlog is append-only and is never rewritten. Their old paths are historical.
- **Bare-name prose mentions** such as "`ARCHITECTURE-SPINE.md` AD-27" (no directory component) are document names, not paths, and are left as written. The spine file itself is renamed to `architecture-Pack-Manager-2026-07-23.md`, so these names now identify the document by its v6 name.
- `_bmad-output/project-context.md` is cited 52 times in the store but was deleted on 2026-08-18: already dead in v6, listed apart, not rewritten.

Outside the store (never edited here; the user updates them):

| File:line | Reference |
| --- | --- |
| `AGENTS.md:17` | full spine path `_bmad-output/planning-artifacts/architecture/architecture-Pack-Manager-2026-07-23/ARCHITECTURE-SPINE.md` |
| `docs/DECISIONS.md:349` | `_bmad-output/planning-artifacts/story-triage-2026-07-24.md` |
| `docs/DECISIONS.md:569`, `:592`; `docs/SPEC.md:49`, `:297`; `docs/RELEASE-CHECKLIST.md:56`, `:125` | bare name `ARCHITECTURE-SPINE.md` |

## Configuration

- `_bmad/custom/config.user.toml` (new; gitignored by `_bmad/custom/.gitignore`): `[core] active_initiative = "initiative-pack-manager"`.
- `core.output_folder` unchanged (`{project-root}/_bmad-output`); no `_bmad/custom/config.toml` needed.
- No `_bmad/custom/` overrides exist, so nothing names a moved path. `migrate_v6.py --clean` runs to confirm there are no retired `bmad-agent-po` customization keys.
- `planning_artifacts` / `implementation_artifacts` left alone (already absent from the live config).

## Side effects worth knowing

- bmad-loop 0.13.2 switches to tickets mode (v7 tree + active initiative). Its v6 `validate` ignored all 30 `ux-pb` keys; the ticket tree carries those stories as ordinary entries.
- `deferred-work.md` in the initiative folder is where v7 `bmad-build` and bmad-loop's sweep expect the ledger.
- `bmad-sprint-planning`, `bmad-create-epics-and-stories`, `bmad-spec` and `bmad-correct-course` are replaced by Sofia (`bmad-agent-po`).

## Checklist results

(filled in at step 15)
