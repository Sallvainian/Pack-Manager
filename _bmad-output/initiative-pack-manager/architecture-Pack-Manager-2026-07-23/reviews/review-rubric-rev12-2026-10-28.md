# Rubric-walker review: architecture spine revision 12

- **Spine:** `architecture-Pack-Manager-2026-07-23.md`, `artifact_revision: 12`, `updated: 2026-08-11`, 22 `### AD-` entries (counted with `grep -c '^### AD-'`).
- **Reviewed against:** the v7 initiative (`initiative-pack-manager.md`, R1–R25), epic `epic-upgrade-plan-redesign` (28 entries, covers R8 and R23), and epic `epic-shipped-behavior-gaps` (8 entries: 66, 22, 25, 35, 31, 32, 34, 65; covers R1, R3, R5–R7, R10, R14–R18). Also `docs/DECISIONS.md` (42 `## D` entries; D38–D42 are later than revision 12 or the same day), the PRD, AGENTS.md, and the code in `src/` and `src-tauri/src/`.
- **Date:** 2026-10-28. Review only; the spine was not modified.
- **Out of scope by instruction:** keyboard navigation, VoiceOver and live-region criteria (D37), and the retired D33 readiness gate. Neither is reported as a gap.

## Verdict

**Revision 12 does its main job: it covers the Upgrade Plan redesign's core divergence points, and its brownfield claims still match the code. It is not yet fit for the v7 ticket set.** It has one critical coverage gap: FR-23's `Run fix` is a fourth immediate-execution call site, and the epic the spine binds says it must be staged. It also has two high-severity divergence points that the later tickets expose: the quit-guard status set against the new durable `Interaction required` state, and native-harness ownership now that Story 6.6 exists. Several medium findings come from the v7 migration and from D38–D42. An Update pass (revision 13) is warranted. No AD needs to be retired.

Tally: **1 critical, 2 high, 5 medium, 3 low.**

---

## Findings

### C1 — CRITICAL — FR-23's `Run fix` has no invariant, no `PlanIntent` shape, and no owning ticket, yet the bound epic says it must be staged

- **Spine refs:** AD-16 rule "No entry point executes" (L467–477) says "**Three shipping call sites are retired by this rule, not one**" and lists the row action plus two self-update paths. The normative domain minimum (L589–595) has only `packageUpdates` and `managerUpdates` members, with no health-fix member. The Capability → Architecture map (L1576–1592) has no Health or `Run fix` row. `grep -c` for `Run fix` and for `FR-23` in the spine each returns **0**. `HealthBanner` appears once, inside the maintainer-edits row (L1630), and only as a checklist citation.
- **Verified facts:**
  - `epic-upgrade-plan-redesign.md` frontmatter has `covers: [R8, R23]`. R23 is FR-23.
  - Its R1 success criterion says "all **four** immediate-execution call sites (row action, both self-update paths, `Run fix`) are gone".
  - Its Done-when says "every Package, Manager, and `Run fix` update is staged in the draft first (SM-2)".
  - PRD FR-23 (L356–368) says Run fix "routes through the persistent Upgrade Plan and the confirmation gate like every other mutation, at which point the set above collapses to zero". PRD FR-6 (L236) names `Run fix` as the fourth call site and puts it explicitly outside FR-6's scope.
  - The shipping code has a separate registered command, `commands::run_health_fix` (`src-tauri/src/lib.rs` handler list), called from `src/components/manager/HealthBanner.tsx:45`.
  - No entry in `epic-upgrade-plan-redesign/tickets/tickets.toml` mentions `Run fix`, `HealthFix`, or a health fix: `grep -i 'run fix|healthfix|health fix'` returns nothing. Entry 1a's note says only "Target is zero immediate-execution kinds".
- **Why it is a divergence point:** a builder following AD-16 retires three call sites and leaves `run_health_fix` running immediately, because the AD's own enumeration says "three, not one". Another builder following the epic's R1 must make a HealthFix a plan member, and the spine gives no shape for that. Several questions are left open:
  - Is it a third `PlanMember<HealthIssueRef>` list?
  - What provenance and scope does it carry under AD-23?
  - Does it belong in AD-21's plan-determining set?
  - Does it count toward `Confirm N Updates`?
  - Does verification (AD-16) refresh the affected Manager?
  - Does the closed exact-recognition gate from FR-23 move into the Rust rebuild?

  Two compliant builds would produce incompatible `PlanIntent` wire shapes, which is exactly what AD-23's last rule (L1011–1014) forbids. SM-2 rates a single unstaged mutation as P0.
- **Remedy direction (for the Update, not applied here):** extend AD-16's call-site rule to four, naming `run_health_fix`. Give the domain minimum a health-fix member, or state that a HealthFix is a Manager-scoped member with a defined ref. Bind FR-23's exact-recognition gate as a rebuild-time eligibility check. Name an owning ticket; today there is none, so this is also a gap in the ticket tree to route to `bmad-agent-po`.

### H1 — HIGH — AD-30's `Queued ∪ Running` quit and app-update set has no rule for the new durable states, and `Interaction required` holds a live child

- **Spine refs:**
  - AD-30 (L1495–1500): "The guard's active set is `Queued` ∪ `Running`, identical to the app-update guard's, and the two may not drift apart."
  - AD-16 (L528–535) makes `Interaction required` "a durable wire-level state on `OpStatus`", and L522–527 does the same for `Verifying` and `Skipped`.
  - The "App-update safety guard" row (L1620) fixes the Rust predicate as `Queued || Running`.
- **Verified facts:**
  - `refuse_app_update_while_busy` (`src-tauri/src/commands.rs:772`) matches only `OpStatus::Queued | OpStatus::Running`.
  - `OpStatus` (`src-tauri/src/ipc.rs:99`) has seven variants today.
  - Ticket 66's criteria hard-code "the guard's active set is `Queued` ∪ `Running`" and say it is "buildable now".
  - Neither ticket 66 nor ticket 3f (which adds the classifier) mentions the other's predicate.
- **Inference (strong):** an Operation in `Interaction required` is a child that is still alive and blocked on a prompt with null stdin. AD-16 L568–572 describes it as a reclassification of a live silence, not a terminal state. If the variant is added as AD-16 requires, an Operation in that state is neither `Queued` nor `Running`. A quit would then bypass the guard, and an app update would be allowed with a live child. That breaks AD-30's own invariant, "children never outlive the app" (L1507), and the AGENTS.md pitfall about mirroring `activeOps`. `Verifying` needs an explicit ruling too: the attempt is not finished while verification refreshes run.
- **Why it is a divergence point:** 66 lands first with a two-variant set, then 3f adds a variant. Nothing in either AD forces 3f to touch both guards. The spine states the predicate as a *list* rather than a *property* ("any Operation whose process group may be alive, plus admitted-unstarted work").
- **Remedy direction:** restate AD-30's set as a closed property-defined set. Require every new `OpStatus` variant to declare its membership in it, in the same atomic AD-3 change that adds the variant. Bind 3f and 2e/3c to AD-30.

### H2 — HIGH — The native harness has two consumers now, and the spine names only the later one as owner

- **Spine refs:**
  - AD-3 (L283–286): "proving delivery itself waits on the native harness, whose **only live consumer is Story 6.5**".
  - AD-26 Binds (L1082): "Story 6.5; any native E2E harness adoption".
  - The harness row (L1608): "**OPEN — owner Story 6.5**".
  - The opener/reveal port row (L1612) also says owner 6.5.
- **Verified facts:**
  - Ticket 66 (Story 6.6, added to `epics.md` in commit `5a62a01`, 2026-08-11 02:30, twenty minutes *after* the revision-12 commit `87d0669`) requires "Real native Tauri E2E for the window-close and `⌘Q` paths … (**AD-26** governs the harness)".
  - Ticket 66 has `after = []` and is "buildable now".
  - Ticket 65 (Story 6.5) waits on 3d and 4d.
  - So the first story to adopt the harness, and the AD-20 security-reviewed plugin, is 6.6, which the spine never names. `grep -c '6\.6'` on the spine returns **0**.
- **Why it matters:** 6.6 can adopt the harness under its own composition, and 6.5 later adopts it as the declared owner, possibly differently. Possible differences include the embedded vs `crabnebula` provider, debug-profile shape, and the opener/reveal seam. AD-26 forbids a second composition root but does not say who decides the one composition. The AD-30 Binds line (L1484–1485) also lists "Story 6.5" and omits 6.6, the story whose entire scope is AD-30.
- **Currency check (verified 2026-10-28):** `tauri.app/develop/tests/webdriver/` still states that `tauri-driver` direct supports only Windows and Linux, that `@wdio/tauri-service` covers macOS through the embedded server (`tauri-plugin-wdio-webdriver`), and that CrabNebula needs "a paid API key … for macOS". AD-26's factual basis (L1086–1102) is still current.
- **Remedy direction:** make the harness row's owner "whichever of 6.6 or 6.5 lands first, with 6.6 expected first". Add 6.6 to AD-3's consumer sentence and to the Binds of AD-26 and AD-30.

### M1 — MEDIUM — Frontmatter and body still describe the v6 story universe

- **`binds:` (L13–15):** lists "Epic UX-PB (28 stories)" and "Stories 2.2, 3.1, 3.2, 3.4, 3.5, 6.5". It omits **Story 2.5** (ticket 25, D40, added 2026-08-18) and **Story 6.6** (ticket 66). It uses v6 ids rather than the v7 entry ids (`1a…5e`; `66, 22, 25, 35, 31, 32, 34, 65`). The epics carry the mapping, but the spine never states it.
- **`sources:` (L16–31):**
  - `docs/architecture.md` (L22) and `_bmad-output/project-context.md` (L23) **do not exist** (verified with `test -e`).
  - L24 names `archive-v6/epics.md`, which is now archive. The live story authorities are the initiative envelope and the two epic envelopes plus their `tickets.toml`, and none of them is listed.
  - D38–D42 postdate or coincide with revision 12. `grep -c` returns 1 for D38 and 0 for each of D39, D40, D41 and D42.
- **Body:**
  - "Epics 1–6 | RESCOPED | **Six** stories survive — 2.2, 3.1, 3.2, 3.4, 3.5, 6.5" (L1607). The epic now says eight.
  - The "Requirements authority" row (L1605) says "`epics.md` stays the **story** authority"; under v7 that role belongs to the tickets.
  - The Capability map (L1576–1592) has no row for Story 2.5's absent-Manager presentation. The quit row (L1591) names no story.
  - The 2026-10-28 migration commit `6bbfdc0` rewrote paths inside the spine without bumping `artifact_revision` or `updated`. That is defensible as a path-only edit, but it means `updated: 2026-08-11` (L11) no longer dates the file's content.
- **Remedy direction:** rewrite `binds`/`sources` in the Update, and add a v6→v7 id crosswalk line or point to the one in the epics.

### M2 — MEDIUM — AD-27's closed binding list omits three stories that render controls, and the spine's own rule makes the list decisive

- **Spine ref:** AD-27 Binds (L1128–1133): "of the surviving Epic 1–6 stories, **3.1, 3.2, 3.4, 3.5 and 6.5**". The same bullet records that "a prose preamble followed by a closed list resolves as the list", which is how 3.4 and 6.5 were omitted before.
- **Verified facts:**
  - Ticket 22 (Story 2.2) renders `Retry refresh` (tickets.toml L91).
  - Ticket 25 (Story 2.5) renders `CopyableCommand` copy controls on all six absent Managers plus a Dashboard guidance panel with `Refresh All` guidance.
  - Ticket 66 (Story 6.6) wires the `QuitGuardDialog` choice buttons.
  - All three render interactive controls, and none is in the list.
- **Remedy direction:** add 2.2, 2.5 and 6.6 (or 22, 25, 66). Alternatively, rephrase the binding as a property, "every story that renders an interactive control", with no enumeration.

### M3 — MEDIUM — D40 / Story 2.5 install hints have no owning rule, and a second source already exists

- **Spine refs:** no AD or capability row covers install hints (`grep -ci 'install hint'` returns 0). AD-16's "frontend display text is never authority" (L657) is scoped to plan text.
- **Verified facts:**
  - `ManagerInfo.install_hint: Option<String>` is a Rust-owned IPC field (`src-tauri/src/ipc.rs:188–190`). `Sidebar.tsx:130`, `ManagerCard.tsx:78` and `ManagerPane.tsx:187` render it.
  - `src/components/settings/SettingsView.tsx:275` **hard-codes** `brew install mas` independently.
  - D40 extends hints to all six Managers. Its commands are "indicative … to be verified against current official docs when built". It explicitly refuses an executing Install button as "a fourth immediate-execution kind".
- **Why it is a divergence point:** ticket 25 touches three named surfaces. A builder can add the five new strings in Rust (`install_hint`) or as frontend constants, as SettingsView already does. The spine says nothing about which, so a hint can drift between surfaces. Neither AD-16 nor AD-4 states the copy-only, never-executable property of a hint. It is an SM-2/SM-3 property and belongs beside C1's closed-set rule.
- **Remedy direction:** add one rule stating that install hints are Rust-owned detection data carried in `install_hint`, rendered only through `CopyableCommand`, and never an action. Then the Settings hard-code is recorded brownfield drift for 2.5 to fold in.

### M4 — MEDIUM — AD-18's "shared retention policy" constrains compaction but does not fix a policy

- **Spine refs:** AD-18 (L851–855): "The two journals share a retention policy. Compacting the Operation journal may not orphan an attempt … A record that loses its counterpart reads as legacy, never as corrupt." Structural seed (L1567–1569).
- **Verified facts:** the shipping Operation journal compacts to `COMPACT_KEEP = 1000` *operations* (`src-tauri/src/journal.rs:19`). One attempt spans several Operations, so a count-of-operations cut necessarily splits attempts at the boundary. D41 (2026-08-18) makes automatic retention the *only* pruning.
- **Gap:** the rule both forbids orphaning and dispositions it ("reads as legacy"). It never states the retention key: newest N attempts, newest N operations with whole-attempt rounding, or one coordinated compaction. No story owns attempt-journal compaction. The deferred filename/serde row (L1611) gives 2c only the filename and fields. The 2c writer and the shipping `journal.rs` compaction can therefore pick incompatible cuts.
- **Remedy direction:** name the key (for example "retain whole attempts; compact both journals in one pass keyed by attempt"), and assign an owner (2c or 4a).

### M5 — MEDIUM — The operational envelope has no rule for dependency automation, and the Stack table is three months stale

- **Spine refs:**
  - AD-12 (L437–439): "work stays off `main` until it is ready to ship".
  - AD-11 (L367–372) bases the x86_64-key rule on `tauri-plugin-updater` behaviour.
  - AD-26 (L1109–1114) depends on the absence of a `[profile.release]` section.
  - Stack (L1531–1557): "Verified against `package-lock.json` and `src-tauri/Cargo.lock` on 2026-07-25".
- **Verified facts:**
  - `.github/workflows/dependabot-auto-merge.yml` (commits `03b9939` and `eb7391c`, 2026-09-16) auto-squash-merges **every Dependabot PR, including majors**, into `main`.
  - `.github/dependabot.yml` covers npm, cargo and github-actions weekly.
  - `grep -ci dependabot` on the spine returns 0.
  - In the Stack table, **12 of the 16 version rows have drifted**:
    - Tauri crate 2.11.5→2.12.2
    - JS API 2.11.1→2.12.4
    - CLI 2.11.4→2.12.4
    - updater plugin 2.10.1→**2.14.0**
    - opener 2.5.4→2.7.2
    - Tokio 1.53.1→1.54.1
    - React 19.2.8→19.3.1
    - Vite 8.1.5→8.3.5
    - Zustand 5.0.14→5.0.16
    - TanStack Virtual 3.14.8→3.14.22
    - **Vitest 4.1.10→5.0.5 (major)**
    - Playwright 1.61.1→1.64.1

    TypeScript 7.0.2, Tailwind 4.3.3, Node 24 and edition 2021 still match. The app version is 1.0.2; the table records "1.0.1 on 2026-07-25", which is explicitly disclaimed, so that one is not a defect.
- **Why it matters at this altitude:** the brief's checklist names deployment, environments and operations as owned dimensions. The spine decides CI images, signing and post-publish (deferred), but it is silent on the one channel that changes release bits with no human present. A major `tauri-plugin-updater` or `tauri` bump can land on `main` unreviewed and ride the next `fix:` release, and AD-11's updater reasoning is exactly what such a bump could move. The Stack table disclaims version policy (L1534), so drift alone is low. The finding is the missing decision: whether auto-merged majors of updater, signing or Tauri crates need the release checklist or an AD-11 re-check. It should be decided or explicitly deferred.

### L1 — LOW — The `macos-14` row tells a reader to run deprecated workflows on deleted files

- **Spine ref:** L1599 names `docs/deployment-guide.md`, `docs/architecture.md` and `docs/contribution-guide.md` as needing "`bmad-document-project` / `bmad-generate-project-context` regeneration".
- **Verified facts:** none of the three files exists, and both workflows are deprecated (outputs deleted 2026-08-18). `grep -rln macos-14 docs/` now hits only `docs/DECISIONS.md` (historical) and `docs/.archive/` scan reports. The row's caveat is spent and its instruction is harmful if followed.

### L2 — LOW — "No telemetry / no egress" has only a webview-side invariant

- **Spine ref:** AD-20 (L892–905) covers remote content in the webview and capability grants only.
- **Requirement:** NFR-5 ("no telemetry, no generic shell surface"). D42 rests a privacy guarantee on "the product has no transmit path at all".
- **Gap:** no AD says Rust may not open a network connection other than the updater endpoint. AD-4 constrains processes, not sockets. Only one future story needs to add a Rust HTTP call (for example, checking hint commands online) for the D42 premise to fail silently.

### L3 — LOW — D39 is not reflected

- **Spine ref:** AD-16's app-update rule (L573–579).
- **Gap:** D39 (2026-08-18) decides that a downloaded update is process-scoped and that `Ready` is always re-derived. That directly governs ticket 5e and RP-1 ("in-process update state survives UI recreation"), but the spine is silent. It is low because D39 describes shipping behaviour and a decision record can stand alone; one citation would close it.

---

## Requirement coverage (initiative R1–R25 against the spine)

| Req | Covered by | Status |
| --- | --- | --- |
| R1 FR-1 detection + install hints | AD-4 (path-derived detection) | **Partial.** The D40 hint limb has no rule (M3). |
| R2 FR-2 | AD-4 (manager verdict only) | Covered |
| R3 FR-3 | AD-25 | Covered |
| R4 FR-4 | AD-4 (routing precedence) | Covered |
| R5 FR-5 | AD-16 inertness, AD-28 predicate | Covered |
| R6 FR-6 | AD-28 | Covered |
| R7 FR-7 | AD-16 compensations | Covered |
| R8 FR-8 | AD-16, AD-21, AD-22 | Covered |
| R9 FR-9 | AD-16 all-or-none | Covered |
| R10 FR-10 | AD-16 | Covered |
| R11 FR-11 | presentation; AD-25 (snapshot honesty) | Adequate for altitude |
| R12 FR-12 | AD-4 safety floor | Covered |
| R13 FR-13 | AD-16 (no `Cancelling`), AD-17 | Covered |
| R14 FR-14 | AD-16 interaction, AD-30 | **Gap at the seam** (H1) |
| R15 FR-15 | AD-18, AD-29 | Covered; retention under-specified (M4) |
| R16 FR-16 | AD-16 verification, AD-25 | Covered |
| R17 FR-17 | AD-19 | Covered |
| R18 FR-18 | AD-5, AD-18, AD-29 (consistent with D42) | Covered |
| R19 FR-19 | AD-27, Styling convention | Covered (binding list incomplete, M2) |
| R20 FR-20 | AD-16 app-update rule | Covered |
| R21 FR-21 | AD-30, guard row | Covered |
| R22 FR-22 | AD-11 | Covered |
| **R23 FR-23** | none | **Uncovered** (C1). The epic the spine binds covers R23. |
| R24 RP-1 | AD-16 app-update rule | Partial (L3) |
| R25 RP-2 | AD-28, AD-17, AD-11, map row | Covered; the re-declaration obligation is recorded with no AD, which is acceptable |

## Deferred and Open items: can any let two units diverge?

- **Controlled child-helper language (L1609):** no. AD-4 constrains it.
- **Crash/relaunch lifecycle controller (L1610):** low risk. Five stories each own their disposable-root setup, but AD-5 constrains each, and the divergence would be test-only.
- **Plan-attempt filename/serde (L1611):** no. There is one owner (2c), and the discriminator requirement is stated.
- **Opener/reveal port (L1612):** yes, a second consumer. It is the same ownership defect as H2.
- **Signing secrets (L1614):** no.
- **Post-publish envelope (L1615):** deliberately deferred with a trigger; acceptable. M5 is a separate, undecided operational question.
- **Native harness (L1608):** yes, see H2.

## Brownfield spot-checks (verified against the tree on 2026-10-28; all hold)

- 20 registered commands (`lib.rs` `generate_handler!`) and 6 event constants (`events.rs:77–82`). 15 files in `dev/fixtures/ipc/`.
- `OpStatus` has 7 variants (`ipc.rs:99–107`). No `Cancelling`, `Verifying`, `Skipped` or `InteractionRequired` variant exists.
- Five port traits: `CommandRunner`, `EventSink`, `UpdateSource`, `PendingRelease`, `ManagerAdapter`.
- `plan_coordinator` is `Arc<std::sync::Mutex<…>>` (`state.rs:8`, `:212`). `execute_issued_plan` carries "No synchronous guard crosses an await" (`commands.rs:353`). `handle_plan_batch` exists (`queue.rs:1003`).
- `set_settings_core` persists, then publishes, then calls `bump_revision()` unconditionally (`commands.rs:636–651`). This confirms both AD-19's ordering and AD-21's stated defect.
- `refuse_app_update_while_busy` matches `Queued | Running` (`commands.rs:772`).
- `selection` and `anchor` are in `src/store/packages.ts:17–19`. `visibleSelectableIds` "mirrors ManagerPane filters" (`useKeyboard.ts:34`). `⌘L` calls `toggleDrawer()` (`useKeyboard.ts:166`).
- `PackageRow.tsx:92` is natively `disabled`, `:100` has `disabled:opacity-40`, `:96` has `aria-label` "Select …", and `:85` carries the `ring-accent` survivor.
- `csp: null`. The capability set is exactly `core:default`, `opener:default`, `core:window:allow-start-dragging`. There is no `[profile.release]` in `Cargo.toml`.
- `journal.rs` has "One line at op start, one at finish, flushed each write", `#[serde(untagged)] enum Line`, and `COMPACT_KEEP = 1000`. `parse_recovery` exists on the mas, npm and rustup adapters.
- `QuitGuardDialog` is rendered by `DialogHost`. Its only non-test caller is the update path (`UpdateStatusItem.tsx:33`), so AD-30's "shipping shape" claim holds.
- In `release.yml`, minisign verification (L317–319) precedes `gh release upload` (L376), and the published `latest.json` check (L382–391) follows it. This matches AD-11's gate-vs-detector split. `minimumSystemVersion: "15.0"`. `macos-15` appears at `ci.yml:28,70` and `release.yml:63`. Node 24. `release-please-action@v5`.

## What is sound and should not be touched

- AD-16 through AD-29 still resolve the Upgrade Plan epic's divergence points. The epic's Constraints section restates their rules faithfully: the two-record journal, the single draft author, batched membership, and no `Cancelling`.
- The claim-boundary phrasing in AD-11 is good. So is the explicit supersession of the stale PRD text in AD-11 L396–405, and the supersession of D15 in AD-16, now discharged by D38.
- AD-26's currency claim was re-verified today.
