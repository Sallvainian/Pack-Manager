# Continuous Integration Guide

Pack-Manager uses two ordinary verification workflows:

- `.github/workflows/ci.yml` checks Rust, the React/Vite application, and a main-branch Tauri bundle smoke build.
- `.github/workflows/test.yml` checks Playwright browser journeys without changing the established Rust/web pipeline.

Release automation remains separate. See [`deployment-guide.md`](./deployment-guide.md) for release-please, signing, notarization, and updater artifacts.

## Playwright Triggers

The Playwright workflow runs for:

- pushes to `main`;
- pull requests targeting `main`; and
- a weekly Sunday 02:00 UTC schedule.

A workflow-specific concurrency group cancels an older run for the same event and Git ref without cancelling `.github/workflows/ci.yml`.

## Playwright Stages

1. **E2E validation** installs locked npm dependencies, type-checks the Playwright files, and verifies test discovery.
2. **E2E shards** run in parallel with `fail-fast: false`. The two shards partition the full suite and collectively cover both configured browser projects, Chromium and WebKit.
3. **Burn-in** runs after the shards on pull requests and the weekly schedule. Every browser test is repeated ten times with retries disabled; one failure blocks the workflow.
4. **Test report** merges shard blob reports into one HTML report and one JUnit XML file, publishes a GitHub run summary, and rechecks upstream results so reporting cannot hide a failed test.

Normal CI runs retain two retries for transient browser failures, but `--fail-on-flaky-tests` still makes a retrying test fail the quality gate. The burn-in stage uses zero retries so it exposes intermittent behavior immediately.

## Local Parity

Use Node 24 from `.nvmrc`, then run the same project scripts used by CI:

```sh
nvm install
nvm use
npm ci
npm run test:e2e:typecheck
npm run test:e2e:install
npm run test:e2e
```

If Node is managed through mise, prefix npm commands with `mise exec node@24 --`. Do not run `npm run test:e2e:install:ci` on the Mac; that command installs browsers plus Linux packages on GitHub's Ubuntu runners.

The workflow itself uses `npm run test:e2e:install:ci` and starts Vite through Playwright because `BASE_URL` is intentionally unset.

## Quality Gate

- Every current browser test must pass; the workflow uses an all-green quality gate rather than a percentage threshold.
- A failed TypeScript/discovery check, browser shard, flaky retry, or required burn-in fails the workflow.
- Pull-request branch protection can make the stable `Playwright / Test report` check required after the first remote run creates it.
- Point-in-time BMAD traceability and NFR documents are not used as live CI inputs because they would become stale unless regenerated automatically.

## Artifacts and Notifications

- Mergeable shard blobs and raw failure evidence are retained for 7 days.
- The merged HTML/JUnit report is retained for 30 days.
- Playwright traces, screenshots, and videos are included when a test fails.
- GitHub's native workflow notifications report failures according to each collaborator's GitHub notification settings.
- The run summary and error annotation link directly to the workflow run and its artifacts.

## Secrets

The Playwright workflow requires no repository secrets. See [`ci-secrets-checklist.md`](./ci-secrets-checklist.md) for the explicit boundary. Never add credential values to `.env`, test fixtures, source files, or workflow text; project secrets remain fnox-managed.

## First Remote Validation

Local validation cannot prove GitHub-hosted runner behavior. After these files are committed and pushed on a branch:

1. Open a pull request targeting `main`.
2. Confirm the `Playwright` workflow shows E2E validation, two shards, burn-in, and the report job.
3. Open the merged report artifact and confirm both Chromium and WebKit results are present.
4. Check the browser-cache step on a later run for a cache restore.
5. If desired, require `Playwright / Test report` in branch protection after its first successful run.

## Troubleshooting

- **Browser installation fails:** inspect the `Install Playwright browsers and system dependencies` step. Keep `npm run test:e2e:install:ci` after cache restoration because the cache does not contain Linux system packages.
- **Vite does not become ready:** inspect the Playwright web-server output for port 1420. CI should not set `BASE_URL` unless it intentionally targets a controlled test deployment.
- **Only one shard reports:** inspect both uniquely named `playwright-blob-*` artifacts. The report job downloads only those blob artifacts before merging.
- **A test passes after retry but CI fails:** this is intentional. `--fail-on-flaky-tests` turns a retrying result into a failed quality gate.
- **Burn-in is too slow as coverage grows:** reduce its trigger to the weekly schedule or introduce safe changed-test selection; do not silently enable retries.
