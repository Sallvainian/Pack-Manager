# Archive — 2026-07-25 completed one-shot specs

Five `bmad-quick-dev` one-shot specs whose work is **finished and merged to `main`**.
They are archived because they are done, not because they were wrong.

That makes this archive different from
`_bmad-output/archive/2026-07-24-scope-recalibration/`, where nothing is authoritative
because the whole apparatus was retired as unbuildable. **Here the content is accurate
history**: each file describes work that actually shipped, and each carries
`status: done`. Read them for why something was built the way it was.

What they are *not* is open work. Do not feed them back into planning or sprint
tracking as pending items.

## What is here

| File | Shipped in | Notes |
|---|---|---|
| `spec-harden-command-trust-boundaries.md` | `1c45834` (#18) | 2026-07-22 |
| `spec-configure-dependabot.md` | `0cef7c4` | 2026-07-22 |
| `spec-fix-uv-outdated-parser.md` | `3a4b8b4` | 2026-07-23 |
| `spec-fix-window-drag-region.md` | `03e03fa` (#33) | 2026-07-25. Carries `route: 'one-shot'`. |
| `spec-adopt-design-tokens-and-focus-ring.md` | `c8c1f9a` (#35) | 2026-07-25. Recorded as `docs/DECISIONS.md` **D35**. Its `status` was still `in-progress` at archive time and was corrected to `done` — the work was on `main`. |

## Why they moved out of `implementation-artifacts/`

`_bmad-output/implementation-artifacts/sprint-status.yaml` declares
`story_location: {project-root}/_bmad-output/implementation-artifacts`, but none of these
five was ever tracked as a row in it. They arrived through `bmad-quick-dev` rather than
through `epics.md`, so two lanes overlapped in one directory with nothing on disk to tell
them apart — and the tracker read `0 in-progress` while five pieces of finished work sat
in its own declared story folder.

Moving them resolves that by making `story_location` mean one thing. What remains there
is `sprint-status.yaml` itself, `deferred-work.md`, and — once
`bmad-create-story` runs — the Epic UX-PB story files it tracks.

`sprint-status.yaml` needs no edit for this: it never referenced these files.

## What was deliberately NOT archived

- **`deferred-work.md`** — live. It records work consciously deferred, which is open, not
  finished.
- **`sprint-status.yaml`** — live, and the tracker for the 28-story Epic UX-PB build
  queue.
