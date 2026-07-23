# Browser Journey Tests

This directory contains Playwright tests for Pack-Manager's React interface. The tests run the real frontend in a browser, but replace Tauri's native command and event transport with a deterministic in-browser fixture.

That boundary is important: these tests can prove browser-visible workflows and frontend IPC behavior. They do not prove real package-manager processes, Finder integration, native Tauri behavior, signing, notarization, or updater installation.

## Setup

1. Install and use Node 24, matching `.nvmrc` and CI:

   ```sh
   nvm install
   nvm use
   ```

   If you use mise instead of nvm, run `mise install node@24` and prefix the commands below with `mise exec node@24 --` (for example, `mise exec node@24 -- npm ci`).

2. Install the locked npm dependencies:

   ```sh
   npm ci
   ```

3. Install the configured Playwright browsers:

   ```sh
   npm run test:e2e:install
   ```

The checked-in defaults use `http://127.0.0.1:1420`. A local `.env` is optional; copy `.env.example` only when you need to change an environment value. `.env` files are ignored by Git. Do not put secrets in the template or source files; Pack-Manager secrets are managed through fnox.

## Running Tests

| Command                                  | Purpose                                             |
| ---------------------------------------- | --------------------------------------------------- |
| `npm run test:e2e`                       | Run every browser journey in Chromium and WebKit.   |
| `npm run test:e2e:typecheck`             | Type-check the Playwright config and test code.     |
| `npm run test:e2e -- --project=chromium` | Run only Chromium.                                  |
| `npm run test:e2e:headed`                | Run with browser windows visible.                   |
| `npm run test:e2e:debug`                 | Open Playwright Inspector and pause through a test. |
| `npm run test:e2e:ui`                    | Open Playwright's interactive test runner.          |
| `npm run test:e2e:report`                | Open the most recent HTML report.                   |
| `npm run test:e2e:install`               | Install local Chromium and WebKit browser binaries. |
| `npm run test:e2e:install:ci`            | Install browsers and Linux packages on CI.          |
| `npx playwright test --list`             | Check discovery without running browsers.           |

The existing suites remain separate:

```sh
npm test       # Vitest frontend unit and integration tests
npm run test:rust  # Rust backend tests
```

Playwright starts the Vite server automatically for the default local URL. A non-local `BASE_URL` is rejected unless `ALLOW_REMOTE_E2E=1` explicitly opts into a deliberate remote run; Playwright then expects that target to be running already. The default fixture blocks browser requests outside the configured target origin. `TEST_ENV` is a reserved, currently informational label; the test configuration does not branch on it. `API_URL` is reserved for a future HTTP boundary and is also unused today.

## Architecture

```text
tests/
├── e2e/                         # User-visible browser journeys
├── support/
│   ├── fixtures/
│   │   ├── index.ts             # Shared Playwright test/expect exports
│   │   ├── tauri-ipc.ts         # Browser-local Tauri command/event double
│   │   └── factories/           # Seeded Pack-Manager domain data
│   ├── helpers/                 # Reusable arrangement/navigation helpers
│   └── page-objects/            # Small, role-first UI interaction helpers
└── README.md
```

Tests import `test` and `expect` from `tests/support/fixtures`, not directly from `@playwright/test`. The shared test object composes:

- deterministic Faker-backed domain factories with per-test cleanup;
- a fake Tauri runtime installed with `page.addInitScript` before application code;
- automatic cross-origin request blocking; and
- local HTTP error monitoring with JSON failure evidence.

The Tauri fixture supplies schema-complete startup responses for `get_state` and `get_app_update_state`. It can configure command responses or failures, emit Tauri-shaped events, and inspect calls without launching native processes or reading machine state.

## Writing a Journey

Use the existing package-search journey as the smallest working example.

1. Build only the state the scenario needs with `factories` and explicit overrides.
2. Configure `tauriIpc` before `page.goto()`.
3. Group the scenario with `test.step()` using Given/When/Then language.
4. Select by accessible role, label, or visible name first. Use a stable `data-testid` only when a dynamic row has no reliable user-facing selector.
5. Assert visible outcomes and, when the contract matters, inspect `await tauriIpc.callsFor("command_name")`.

For event-driven scenarios, wait for the relevant listener with `tauriIpc.listenerCount(event)` or for a manager-specific hydrated element before emitting. The always-visible **Packages** heading does not prove startup subscriptions are ready, and React Strict Mode can run development effects more than once.

## Isolation and Cleanup

- Never call live package managers, external services, the filesystem, or the native Tauri runtime from the default browser suite.
- Do not add fixed sleeps. Rely on Playwright's locator assertions, explicit UI state, or deterministic events.
- Keep tests independent. Each test receives a new factory stream, page, call ledger, and Tauri runtime controller.
- Prefer factory overrides to hand-written object literals so fixtures stay aligned with TypeScript IPC types.
- Let the shared fixtures clean up. If a test creates another resource, release it in a fixture `finally` block.
- Keep authentication, HTTP API, database, and Pact fixtures out until Pack-Manager actually gains one of those boundaries.

## Failure Evidence

On failure, Playwright retains a trace, screenshot, and video. It also writes:

- `playwright-report/` for the HTML report;
- `test-results/results.xml` for JUnit consumers; and
- per-test artifacts under `test-results/`.

These paths are ignored by Git. Use `npm run test:e2e:report` or open a trace from the HTML report to inspect the page, actions, console, and network timeline.

## CI Integration Notes

`.github/workflows/test.yml` runs on pushes and pull requests to `main`, plus a weekly schedule. It uses Node 24 from `.nvmrc`, validates the Playwright TypeScript and test discovery, then runs two parallel shards that collectively cover the configured Chromium and WebKit projects. The Linux jobs run `npm run test:e2e:install:ci`; developers should continue to use `npm run test:e2e:install` locally.

Pull requests and the weekly schedule also repeat the complete browser suite ten times with retries disabled. A failed validation, shard, flaky retry, or burn-in blocks the workflow. Successful shard reports are merged into one HTML/JUnit artifact retained for 30 days; raw failure traces, screenshots, and videos are retained for 7 days. GitHub's run summary and failure annotation link directly to those artifacts.

The Playwright workflow requires no repository secrets. It uses only GitHub-provided `GITHUB_*` variables, and `BASE_URL` stays unset so Playwright starts the local Vite server. Do not add credentials to `.env`, tests, or workflow files; Pack-Manager secrets remain fnox-managed.

A deliberate remote CI run may set `BASE_URL` together with `ALLOW_REMOTE_E2E=1`; otherwise Playwright starts Vite itself. Do not point the suite at a developer machine or any environment containing real user/package-manager state.

## Pattern References

This scaffold applies the BMAD Test Architecture knowledge-base patterns for fixture composition, deterministic data factories, origin enforcement, network-error monitoring, resilient selectors, and interception at the system boundary. Authentication-session, API-request, recursive polling, and external service/Pact contract patterns were reviewed but intentionally omitted because they do not match the current architecture.

For framework behavior, see the official Playwright documentation for [installation](https://playwright.dev/docs/intro), [configuration](https://playwright.dev/docs/test-configuration), [fixtures](https://playwright.dev/docs/test-fixtures), [locators](https://playwright.dev/docs/locators), and [trace viewing](https://playwright.dev/docs/trace-viewer).

## Troubleshooting

- **Browser executable is missing:** run `npm run test:e2e:install`.
- **Port 1420 is already in use:** stop the other Vite/Tauri development server or set `BASE_URL` to the intended running server.
- **`listen EPERM` while starting Vite:** the execution environment is blocking local listeners. Run the test from a normal local terminal or an appropriately configured CI runner.
- **A startup command has no fake response:** add a response through `tauriIpc` before navigation; do not weaken the fixture to silently accept unknown commands.
