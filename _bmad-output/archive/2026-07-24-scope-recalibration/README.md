# Archive — 2026-07-24 scope recalibration

These artifacts were retired by the scope recalibration recorded in
`docs/DECISIONS.md` **D33**. They are kept, not deleted, because they contain real
reasoning that may be worth reading again. **Nothing here is authoritative.**

Do not feed these files back into planning. They live outside
`_bmad-output/planning-artifacts/` deliberately: the BMAD skills glob patterns like
`{planning_artifacts}/*prd*.md`, `*epic*/*.md`, and `*architecture*/*.md`, so anything
left inside that directory gets silently reloaded into future planning runs — which is
the exact re-contamination this archive exists to prevent.

## Why these were retired

Pack-Manager is a personal, open-source macOS utility: 1 star, 0 forks, and 3 lifetime
`.dmg` downloads. It had been planned with an enterprise readiness apparatus — a
72-criterion P0 gate, 55 versioned scenario contracts, an append-only hash-chained
evidence ledger, a candidate-freeze process, and multi-host hardware requirements.

Three things made that apparatus unbuildable rather than merely oversized:

1. **All 55 scenario contracts were unassigned**, against a `contracts/` directory that
   does not exist and that no story creates.
2. **The evidence schema bound even manual scenarios to GitHub Actions run provenance**,
   which nothing in the plan provisioned.
3. **The legacy P1 rule was arithmetically impossible** — 5 of its 8 rows were declared
   out of scope, capping achievable coverage at 3/8 = 37.5% against an 80% minimum.

The 72-criterion P0 rule was *not* impossible — 100% of criteria you intend to deliver is
just work. It required the evidence infrastructure above, which was never built. Those
two failures are different and D33 keeps them separate.

## What is here

| Path | What it is |
|---|---|
| `planning/prds/` | The full PRD family (16 files). `prd.md` is the gate document — it is not a product PRD with gate machinery attached. Includes `readiness-coverage-map.md`, the 72-row map. |
| `planning/implementation-readiness-report-*.md` | Three readiness reports asserting a FAIL baseline computed from the retired gate. |
| `planning/epics-7-8-retired.md` | The 18 evidence stories of Epics 7–8, 789 lines. Replaced by `docs/RELEASE-CHECKLIST.md`. |
| `planning/sprint-change-proposal-2026-07-24.md` | The approved change proposal for the *previous* correction, earlier the same day — the one that finalized the D27–D30 UX contract. Its planning changes were applied and its substance survives in D27–D30 and Epic UX-PB. It is archived because its readiness claims (`planning baseline: FAIL`, `14/72 P0 criteria FULL`) are computed from the gate D33 retires, and are now void. Superseded by D33. |
| `architecture/ARCHITECTURE-SPINE-original.md` | The 969-line spine before the split. The live spine keeps its product architecture; this is the complete original including the retired evidence apparatus. |
| `architecture/reviews/` | Three architecture reviews. Every finding in all three concerns the evidence contract that D33 dissolves; none is a product-architecture finding. |
| `test-artifacts-gate/` | Gate paperwork: the 149 KB traceability matrix, the trace summary, the gate decision, the test-design set, and progress logs. |

## What was deliberately NOT archived

- **`docs/SPEC.md` and `docs/DECISIONS.md`** — preserved untouched and still
  authoritative. Verified clean of gate references before this correction.
- **The UX design artifacts** (`planning-artifacts/ux-designs/`) — preserved. `DESIGN.md`
  and `EXPERIENCE.md` are the specification the promoted build queue works from.
- **The test and CI infrastructure** — all 30 Rust test files, 27 TypeScript test files,
  Vitest, Playwright, and every workflow. The gate paperwork *about* the tests was
  retired; the tests themselves were not touched.
- **`epics.md`** — retained live, with the UX-PB stories promoted and Epics 7–8 removed.
- **The live Architecture Spine** — trimmed from 969 to 455 lines. AD-16, the Upgrade Plan
  domain model that the primary build queue depends on, is preserved verbatim and
  extended with five rules its stories already required.

## What replaced it

- `docs/RELEASE-CHECKLIST.md` — a ~15-minute manual pass, plus two automated checks in
  `release.yml` that close failure modes which are silent across every installed client.
- `docs/DECISIONS.md` D31, D32, D33 — the platform floor, the architecture-support
  posture, and the retirement itself.
- `epics.md` Epic UX-PB — the 28 stories implementing Decisions D27–D30, now the primary
  build queue.

## A note on completeness

This archive holds exactly one memlog:
`planning/prds/prd-Pack-Manager-2026-07-22/.memlog.md`, the PRD run's working memory.

It **is** tracked by git and will appear in a fresh clone. An earlier version of this
note said the opposite — that `**/.memlog.md` was gitignored and needed force-adding.
That ignore rule was removed on 2026-07-25 and all three memlogs in the repo are now
tracked (`git ls-files '*.memlog.md'`).

The rule was removed because a memlog is a workflow run's append-only working memory —
`_bmad/scripts/memlog.py` has no edit or delete subcommand by design — and the `bmad-*`
Update intents resume from it as the authority on what was decided, not from the rendered
artifact. It is not a deliverable, but it is not regenerable either: lose it and the
artifact can never be re-distilled. See `.gitignore:51-55`.

That is also why the architecture run's memlog is **not** here. It was moved back to
`_bmad-output/planning-artifacts/architecture/architecture-Pack-Manager-2026-07-23/`,
its live run folder, so the spine's Update intent has something to resume from.

## One habit worth keeping

Before scheduling any work described here as a gap, check whether the behavior already
exists in the shipping code. The Epics 1–6 triage overturned 14 of 20 initial keeps for
exactly that reason.
