# Release Checklist

Pack-Manager is a personal, open-source macOS utility. Release verification is a short
manual pass plus a few automated checks in the release pipeline — not a formal gate.
See `DECISIONS.md` D33 for why the previous 72-criterion gate was retired.

**Budget: about 15 minutes.**

---

## Automated — enforced by `release.yml`, no action required

These fail the release build. They exist because their failure modes are silent,
simultaneous across every installed client, and invisible on the GitHub Release page.

- **The updater's detached signature verifies against the minisign public key the
  shipping app embeds.** If `TAURI_SIGNING_PRIVATE_KEY` ever drifts from
  `tauri.conf.json`'s `pubkey`, the release still builds, signs with Apple, notarizes,
  staples, and publishes green — and every installed client then fails signature
  verification and silently stops updating, because that error is logged at WARN so
  offline laptops don't nag every six hours.
- **The published `latest.json` is reachable, names the version just released, and its
  download URL resolves.** `latest.json` is written before the asset it points at is
  uploaded, so nothing else confirms the endpoint is actually correct.

---

## Before merging the release PR

Everything preventive lives here. Once the release PR merges, publication is automatic
and unattended.

1. **CI is green.** `ci.yml` runs `cargo fmt --check`, `cargo clippy --all-targets -- -D
   warnings`, `cargo test --locked`, `npx tsc --noEmit`, `npx vitest run`, `npm run
   build`, and a `--debug --no-sign` Tauri build smoke test from a clean checkout.
   `test.yml` runs Playwright on Chromium and WebKit.

2. **The app launches from Finder or the Dock.** Not from a terminal — the login-shell
   PATH probe behaves differently, and the terminal path can mask a real failure.

3. **Managers are detected, and absence is honest.** Installed managers appear with their
   real state; uninstalled ones read as genuinely absent, not as errors and not as empty
   successes.

4. **One manager failing does not blank the others.** Break or rename one manager's
   binary and refresh. That manager reports its failure and keeps its prior snapshot; the
   other five are unaffected.

5. **The bulk paths do not execute without explicit confirmation.** Row checkboxes,
   Manager headers, and Update Everything all stage into the draft plan and reach the
   confirmation gate — verify each still does.

   Two paths deliberately **bypass** the gate today and must not be reported as failures:
   a row's own `Update Package` action (`src/components/manager/ManagerPane.tsx:145`,
   commented "Single-package plan executes immediately — no sheet (SPEC §F5)") and a
   Manager's self-update button (`src/components/manager/SelfUpdateCard.tsx:116`). Both
   run one known command against one named target. D27–D30 routes them through the plan
   too, but Epic UX-PB is unbuilt, so that is target state — not something to check here.

6. **An update run shows real output and fails without corrupting state.** Live output
   streams as the process runs. Cancel one mid-flight and relaunch: the interrupted
   operation is reconstructed honestly rather than left claiming success.

---

## After publishing

Items 7–9 are **post-publish smoke tests, not gates**. By the time they can run, the
release is live and installed clients have already begun polling. They catch a bad
release; they cannot prevent one. Prevention lives in the automated checks above.

7. **The release is signed, notarized, and stapled.** The workflow asserts this, but
   confirm the assets attached to the Release are the expected four: `.dmg`, `.zip`,
   `.app.tar.gz` + `.sig`, and `latest.json`.

8. **The updater installs and relaunches.** From an actually installed prior version,
   check for updates, let it download, click Restart. It should install and come back up
   on the new version.

   - **8a. No admin prompt on a non-writable bundle.** *(~60 s)* Run the app from the
     mounted DMG, or `chmod a-w` the parent directory of a copy. Click Restart-to-update.
     It must report manual-install-required, and **no administrator password prompt may
     appear**. `DECISIONS.md` D25a states the promise — the updater plugin's macOS
     installer falls back to AppleScript `with administrator privileges` when the bundle's
     parent isn't writable, and `app_update.rs` pre-flights with `access(2)` to stop
     before that. On a normal install the app almost always sits somewhere writable, so
     this branch is otherwise never exercised.

   - **8b. An update is refused while an operation is running.** *(~45 s)* Start a package
     upgrade, then click Restart-to-update. It must be refused, and the refusal must name
     the running work rather than failing silently. Enforced in both layers: the frontend
     quit guard is the path that explains itself to the user, and `install_app_update`
     refuses independently (`src-tauri/src/commands.rs:810`) so a caller that skips the
     guard cannot destroy in-flight package work. Queued counts as running — admission has
     already committed to the operation, and a restart would drop it unstarted.

9. **Clipboard and menus still work.** ⌘X / ⌘C / ⌘V / ⌘A work in the package search field
   and in every `CopyableCommand`. These die if the Edit and Window submenus aren't
   re-declared, per `DECISIONS.md` D25a — that is a functional regression in copy/paste,
   not an accessibility check, and it is why this step survives.

   **Keyboard navigation and screen-reader support are explicitly not release criteria.**
   Pack-Manager is a personal, single-user, mouse-driven utility. The former "Tab and arrow
   navigation reach every control" pass and the VoiceOver pass over the Upgrade Plan were
   removed deliberately on 2026-07-25 — they are not oversights, and a future
   regeneration or review should not reinstate them. Do not report their absence as a gap.

   This does not license *removing* what already ships. The focus outline is merged, costs
   nothing to keep, and is asserted in CI — see the note below. Deleting that guard would
   not save work, only allow silent regression.

   Reduced motion is covered automatically and needs no manual step — AUT-004 in
   `tests/e2e/browser-style-contract.spec.ts` emulates `prefers-reduced-motion: reduce`
   and asserts transitions and animations are removed. It runs in CI through `test.yml`.
   The same spec pins the focus outline and measures the primary button's real computed
   contrast, failing below 4.5:1. Those are automated and free; nothing to do here.

   That contrast check exists because white on `--color-accent` measured **2.46:1** and
   shipped — a defect visible to anyone looking at the primary button, mouse or not. It is
   kept for that reason, not as an accessibility obligation. No by-eye contrast sweep is
   required at release.

---

## Notes

- The build is **universal (arm64 + x86_64)** and both updater platform keys are
  published, but verification is **Apple silicon only**. Intel is best-effort and
  unverified. See `DECISIONS.md` D32.
- Minimum supported macOS is **15.0**. See `DECISIONS.md` D31.
- To test the pipeline without publishing: **Actions → Release → Run workflow**. It
  builds, signs, and notarizes, and uploads to the workflow run only — it never touches
  a GitHub Release.
