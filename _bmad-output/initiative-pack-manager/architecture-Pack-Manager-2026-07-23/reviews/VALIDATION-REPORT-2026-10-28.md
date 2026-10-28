# Validation report — architecture spine revision 12 (2026-10-28)

**Spine:** `architecture-Pack-Manager-2026-07-23.md` (`artifact_revision: 12`, 22 ADs, 1,633 lines), validated after the BMad v7 migration.
**Gate:** `lint_spine.py` + rubric walker + the two configured floor reviewers (currency, adversary), each in an independent context. The spine was not changed.

## Verdict

The migration left the spine intact. Its Upgrade Plan core and its claims about the existing code still hold, but it is **not safe to build past wave 1 without an Update pass (revision 13)**. After merging duplicates, the reviewers found **2 critical, 6 high, 9 medium and 8 low** issues. No AD needs retiring; every fix is a new or tightened AD, or a frontmatter refresh.

## Migration integrity (deterministic) — PASS

- Compared with the pre-migration spine at `3f64e2b`, the only changes are **9 path tokens** and the added `title:` line. No content changed.
- All 9 new targets resolve.
- `lint_spine.py`: 0 findings.

## Critical

| # | Finding | Lens | Where |
|---|---|---|---|
| C1 | **`Run fix` (FR-23) has no invariant and no owner.** The Upgrade Plan epic covers R23, and its R1 says all four immediate-execution call sites (including `Run fix`) go through the draft. The spine names "three call sites", gives `PlanIntent` no health-fix member, and mentions `Run fix`/FR-23 zero times. No ticket mentions it. Shipping code still executes immediately (`HealthBanner.tsx:43` → `run_health_fix`). Verified by the parent. | rubric C1, adversary H-5 | AD-16 :467–477 |
| C2 | **No ticket owns how a normally completing attempt ends, from 2b until 3d.** AD-29 gives the terminal append to 2e, but 2e covers only cancellation. Until 3d lands, a compliant build has three choices and all are wrong: never end the attempt (every later confirmation is refused), write success before verification, or write no end record (History shows `Interrupted`). | adversary C-1 | AD-16 :504–506; AD-29 :1371–1375, :1442–1444 |

## High

| # | Finding | Lens | Where |
|---|---|---|---|
| H1 | **The quit and app-update guard checks a closed list, `Queued ∪ Running`, but AD-16 adds non-terminal `Verifying` and `Interaction required` states**, and the second one holds a live child process. Ticket 66 locks in the two-state set and is buildable now; 3f adds the new state. Quitting at a prompt would orphan the child. `commands.rs:772–780` is verified. | rubric H1, adversary H-1 | AD-30 :1495–1500 vs AD-16 :522–535 |
| H2 | **AD-26's "a compliant harness composition exists" was never checked against the WebdriverIO service's own setup guide.** That guide requires `tauri-plugin-wdio`, `withGlobalTauri: true`, capability grants and a frontend plugin import. None of these can be removed at compile time, and each conflicts with AD-20, AD-26's "no capability toggle", or AD-3's "`bridge.ts` is the only Tauri importer". | currency HIGH-1 | :1099–1107, :1608 |
| H3 | **The native harness has two consumers but the spine names only one.** Story 6.6 (ticket 66) needs real native E2E and lands first; the spine names only 6.5 as owner and "only live consumer". | rubric H2 | AD-3 :283–286; AD-26 :1082; :1608 |
| H4 | **The attempt fold has no rule for a terminal record without an admission record.** Two existing mechanisms produce one: the start-record write is allowed to fail, and count-based compaction of `operations.jsonl` (`journal.rs:185`) can drop it. 2c, 4a and 4e would each read that case differently. | adversary H-2 | AD-29 :1438–1446; AD-18 :851–855 |
| H5 | **"Durable, journaled" `Verifying` and `Interaction required` have nowhere to be recorded.** The Operation journal writes one start line and one finish line, and the last line wins. One plausible encoding brings a crashed verifying Operation back as `Verifying` instead of `Interrupted`. | adversary H-3 | AD-16 :526–527; AD-29 :1474 |
| H6 | **AD-28's snapshot token has no identity, owner or wave.** The obvious candidate, `PlanCoordinator.revision`, advances on every queue change, so it would reject every batch staged during a live attempt. Rule :1263 also contradicts :1334. | adversary H-4 | AD-28 :1263, :1331–1334 |

## Medium (9)

- **Frontmatter still describes v6.**
  - `binds:` uses v6 ids and leaves out Stories 2.5 and 6.6.
  - `sources:` lists `docs/architecture.md` and `_bmad-output/project-context.md`, which don't exist, plus the archived `epics.md`, and none of the v7 initiative or epic files.
  - The body says "six stories survive"; D39–D42 are never cited.
  - Source: rubric M1 + deterministic check.
- **AD-27's closed binding list leaves out 2.2, 2.5 and 6.6**, all of which render controls (rubric M2).
- **The D40 install hints have no owning rule.** A second source already exists: `SettingsView.tsx:275` hard-codes the mas hint (rubric M3).
- **AD-18's "shared retention policy" constrains compaction but never states the policy** (rubric M4; underlies H4).
- **Dependency automation has no rule, and the Stack table is stale.**
  - 12 of 20 rows have drifted, including Vitest 4 → 5 (major) and the updater plugin 2.10 → 2.14.
  - Dependabot now auto-merges major upgrades, with nothing re-checking the spine.
  - Source: rubric M5, currency MEDIUM-1.
- **Typed, re-serializing compaction**, which 2c may copy, erases what AD-29 needs to stay distinguishable (adversary M-1).
- **Greedy casks fall into two ineligibility classes**, plus an intent-level scalar that AD-23 should have removed (adversary M-2).
- **No ticket retires `ActivityDrawer` or re-points `⌘L`** (adversary M-3).
- **Verification-refresh argv** is in the reviewed snapshot for drift, but is bound from live Settings after admission (adversary M-4).

## Low (8)

- The `macos-14` row tells readers to run deprecated workflows on deleted files.
- "No telemetry" is enforced only on the webview side.
- D39 is not reflected.
- The WebKit focus guard runs on `ubuntu-latest`, which is moving to 26.04 between 2026-10-19 and 2026-11-19.
- The updater plugin changed between 2.12 and 2.14 (`allowDowngrades` config-only, canonical signatures, `requireSignedVersion`), and the deferred post-publish row doesn't track it.
- Node 24 and `macos-15` are now one version behind with no revisit trigger.
- Two tickets each build the below-720px safety indicator.
- The retry preview can be evicted by staging.

## What still holds

- **Quotes:** all five AD-26 quotes are still word-for-word in today's Tauri docs.
- **Tools:** `tauri-driver` still has no macOS support.
- **Tailwind:** AD-27's Tailwind claims compile the same on 4.3.3.
- **CI and release:** the macOS 15 SDK, the `macos-14` cutoff and the release step order all check out.
- **Baseline:** 20 commands, 6 events and 15 fixtures.
- **Brownfield claims:** the rubric walker's code spot-checks all hold.
- **UNVERIFIED:** that WKWebView doesn't paint `box-shadow` on native controls. This needs a real macOS Tauri build and has not been re-checked on macOS 26.

## Recommendation

Run `bmad-architecture` **Update**, resuming from this folder's `.memlog.md`, to fix C1–C2 and H1–H6 as revision 13. Then ask Sofia (`bmad-agent-po`) to add the owning entries the spine names: a `Run fix` staging entry, the normal-completion terminal write, and the harness owner for 66.

| Order | Reason |
|---|---|
| Before **2b** | C2 and H6 bite once 2b lands. |
| Before **66** | H1 and H3 bite in 66, which is buildable now. |
| Before **6.5** | H2 blocks 6.5's harness. |

**22**, **25**, **1a** and **1b** can start in parallel with the Update. **1c** should wait for C1, because `Run fix` is one of its entry points.

## Reviews

- `review-rubric-rev12-2026-10-28.md` — 1 critical, 2 high, 5 medium, 3 low
- `review-adversary-rev12-2026-10-28.md` — 1 critical, 5 high, 4 medium, 2 low; includes proposed AD wording
- `review-currency-rev12-2026-10-28.md` — 1 high, 1 medium, 3 low; includes the claim ledger
