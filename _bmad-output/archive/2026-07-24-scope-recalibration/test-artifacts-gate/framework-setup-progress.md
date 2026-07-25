---
stepsCompleted:
  [
    "step-01-preflight",
    "step-02-select-framework",
    "step-03-scaffold-framework",
    "step-04-docs-and-scripts",
    "step-05-validate-and-summary",
  ]
lastStep: "step-05-validate-and-summary"
lastSaved: "2026-07-22"
workflowType: "testarch-framework"
frameworkSetupStatus: "complete"
validationStatus: "pass-with-webkit-install-pending"
detectedStack: "fullstack"
packageManager: "npm"
existingE2EFramework: "none"
selectedBrowserFramework: "playwright"
selectedBackendFramework: "cargo-test"
executionMode: "agent-team"
inputDocuments:
  - "_bmad/tea/config.yaml"
  - "_bmad-output/project-context.md"
  - "package.json"
  - "package-lock.json"
  - "vite.config.ts"
  - "src-tauri/Cargo.toml"
  - "docs/architecture.md"
---

# Test Framework Setup Progress - Pack-Manager

## Step 1: Preflight Checks

### Stack Detection

- **Detected stack:** Full stack
- **Application:** One macOS Tauri bundle with a React/TypeScript WebView and Rust native core
- **Frontend:** React 19, TypeScript 5.8, Vite 7 on fixed development port 1420
- **Native backend:** Rust 2021, Tauri 2, Tokio
- **Package manager:** npm, established by the root `package-lock.json` (lockfile version 3)
- **Existing test suites:** Vitest/React Testing Library/jsdom and Cargo tests

### Prerequisite Validation

- Root `package.json` and Vite configuration are present.
- `src-tauri/Cargo.toml` provides the backend manifest.
- No Playwright or Cypress package, executable, configuration, script, or CI job exists.
- The empty `.playwright/` directory is not an installed framework and creates no conflict.
- Existing Vitest helpers and ignored Rust live-smoke tests do not conflict with adding a browser E2E framework.
- The local Node, npm, Rust, Cargo, and Xcode development environment is available.

**Preflight result:** PASS. The project is ready for a new E2E framework scaffold.

### Architecture Context

- Pack-Manager has no application login/session system, HTTP API, remote service, or database.
- The frontend/native interface is typed local Tauri IPC: 20 commands and six events.
- `src/lib/ipc/bridge.ts` is the only frontend Tauri import seam; browser tests will need a deterministic IPC/event replacement.
- Default tests must remain offline and deterministic. They must not invoke live package managers, external networks, sleeps, or machine-specific state.
- A browser framework can validate React journeys, but it cannot by itself validate native Tauri processes, Finder actions, signing, notarization, or real updater installation.
- Existing CSP and native capability permissions must not be broadened for test setup.

### Environment Notes

- CI uses Node 24 and stable Rust, while local toolchains are not repository-pinned. This version drift is non-blocking but should be considered in framework configuration.
- npm reports a deprecated unknown `http-proxy` environment setting during inspection. It does not block installation or test execution.

### Step 1 Outcome

All framework-scaffolding prerequisites are satisfied. Framework selection is deferred to the next workflow step.

## Step 2: Framework Selection

### Selected Frameworks

- **Browser journey framework:** Playwright
- **Native backend framework:** Existing Rust `cargo test`

### Decision Rationale

Playwright is the default browser choice for a full-stack project and is the better fit for Pack-Manager's shell-level workflows, deterministic parallel execution, multi-browser capability, and future CI integration. Its built-in web-server lifecycle can target the existing Vite development server on port 1420.

Cypress was not selected because its strongest differentiator—component-test developer experience—is already served by the established Vitest and React Testing Library suite. Replacing or duplicating that layer would add maintenance without closing the traceability gaps.

The existing Cargo test framework remains the native backend choice. Adding another Rust test framework would conflict with established fixture, fake-runner, paused-time, and ignored live-smoke patterns.

### Coverage Boundary

The Playwright scaffold will exercise frontend browser journeys through a deterministic Tauri IPC/event substitute. It must not be described as proof of native process execution, real Tauri transport, Finder actions, application signing/notarization, or updater installation. Those boundaries require Cargo, macOS integration, packaged-application, or release-pipeline evidence.

### Step 2 Outcome

Playwright and Cargo test are selected for the full-stack architecture. Framework scaffolding is deferred to the next workflow step.

## Step 3: Scaffold Framework

### Execution Mode

The scaffold ran in agent-team mode. Independent workers produced the Playwright configuration, deterministic fixture layer, and representative browser journey; the outputs were then integrated and checked together.

### Installed Development Dependencies

- `@playwright/test` 1.61.1
- `@seontechnologies/playwright-utils` 4.4.0
- `@faker-js/faker` 10.5.0

The user completed the npm installation after the managed agent network returned HTTP 403. npm recorded the packages and their resolved dependency graph in `package.json` and `package-lock.json`.

### Framework Configuration

- `playwright.config.ts` targets `tests/e2e`, starts the fixed Vite server at `127.0.0.1:1420` for local runs, and supports an external `BASE_URL`.
- Test, action, navigation, and expectation timeouts are explicit.
- Chromium and WebKit projects are configured.
- Trace, screenshot, and video retention preserve failure evidence.
- Console, HTML, and JUnit reporting are enabled, with CI-specific retries, worker count, and `forbidOnly` protection.
- `.nvmrc` pins Node 24 to the version used by CI.
- `.env.example` documents `TEST_ENV`, `BASE_URL`, and `API_URL` without secrets; local environment files and Playwright output are ignored by Git.
- `vitest.config.ts` excludes `tests/e2e/**` so the established Vitest suite and the Playwright suite remain isolated.

### Fixture Architecture

- `tests/support/fixtures/index.ts` composes Playwright with the relevant logging and network-error-monitor utilities through `mergeTests`.
- `tests/support/fixtures/factories/pack-manager.ts` provides deterministic, per-test Faker factories for the typed Pack-Manager domain model, explicit overrides, entity tracking, and cleanup.
- `tests/support/fixtures/tauri-ipc.ts` installs a browser-local Tauri command/event substitute before navigation. It supports configured responses, rejection paths, response sequences, event delivery, command-call inspection, listener inspection, reset, and cleanup.
- Authentication, HTTP API, database, and Pact fixtures are intentionally absent because Pack-Manager has none of those boundaries.

### Representative Journey

`tests/e2e/package-search.spec.ts` uses shared helpers and a small page object to verify that a user can open a detected npm manager and filter its package list by visible package name. The test follows explicit Given/When/Then steps and uses accessible roles plus the existing stable package-row test identifier.

### Verification

- Strict TypeScript checking passed for the complete Playwright scaffold.
- Playwright discovery passed: one journey is listed for Chromium and WebKit.
- Browser-independent probes passed for factory determinism/cleanup and fake Tauri invoke/listen/emit/call-recording behavior.
- The existing Vitest suite passed: 22 files and 120 tests.
- `git diff --check` passed.
- Initial browser attempts encountered intermittent managed-sandbox listener/process restrictions. Final execution evidence is recorded in Step 5.

### Coverage Boundary

The browser suite proves React behavior against a deterministic Tauri transport substitute. It does not prove native Tauri process execution, real package-manager commands, Finder integration, application signing/notarization, or updater installation.

### Step 3 Outcome

The Playwright framework scaffold is complete and statically validated. Documentation and package scripts are deferred to the next workflow step.

## Step 4: Documentation and Scripts

### Test Documentation

Created `tests/README.md` with:

- Node, npm, and browser setup instructions;
- local, project-specific, headed, debug, UI, report, and discovery commands;
- the fixture/factory/helper/page-object architecture;
- the deterministic browser-local Tauri boundary and its coverage limits;
- selector, isolation, cleanup, event-timing, and failure-evidence practices;
- current and future CI integration guidance;
- relevant BMAD Test Architecture pattern references and official Playwright documentation; and
- troubleshooting for missing browsers, port conflicts, sandbox listener restrictions, and missing fake command responses.

### Package Scripts

Added the following scripts to `package.json`:

- `test:e2e`
- `test:e2e:typecheck`
- `test:e2e:headed`
- `test:e2e:debug`
- `test:e2e:ui`
- `test:e2e:report`
- `test:e2e:install`
- `test:e2e:install:ci`
- `test:rust`

The established `npm test` Vitest command remains unchanged.

### CI Note

The existing CI workflow remains unchanged in this setup step. The test guide explicitly records that Playwright is not yet a CI gate and describes the separate Node 24 browser job and failure-artifact handling needed when it is integrated.

### Step 4 Outcome

The framework now has a project-specific operating guide and discoverable npm commands for browser and Rust test execution.

## Step 5: Validate and Summarize

### Checklist Result

The framework setup passes the BMAD validation checklist for project structure, configuration, fixture architecture, factories, representative tests, helpers, documentation, scripts, formatting, security, and downstream workflow compatibility.

The Pact-specific checklist is not applicable because Pact support is disabled and Pack-Manager has no HTTP service contract. Authentication, API, database, feature-flag, and network-mocking helpers are likewise not applicable to the current local Tauri architecture.

### Framework and Artifacts

- **Browser framework:** Playwright 1.61.1
- **Native backend framework:** existing Cargo test suite
- **Browser projects:** Chromium and WebKit
- **Configuration:** `playwright.config.ts`, `.env.example`, `.nvmrc`, and `tests/tsconfig.json`
- **Runner isolation:** `vitest.config.ts` excludes Playwright journeys
- **Shared support:** merged fixtures, deterministic Faker factories, browser-local Tauri IPC/events, helpers, and page objects
- **Representative journey:** `tests/e2e/package-search.spec.ts`
- **Operating guide:** `tests/README.md`
- **Commands:** local/headed/debug/UI/report/install/CI-install/typecheck E2E scripts and the Cargo test script

### Validation Evidence

- `npm run test:e2e:typecheck`: PASS
- Node 24.18.0 Playwright discovery: PASS, two projects listed
- `npm run test:e2e -- --project=chromium`: PASS, one real browser journey
- `npm test`: PASS, 22 Vitest files and 120 tests
- `npm run build`: PASS
- `npm run test:rust`: PASS, 216 Rust tests passed, nine unit tests ignored, and two live smoke tests ignored by design
- Framework dependencies resolve to one Playwright version with no peer conflict
- Prettier check and `git diff --check`: PASS
- Required directories are readable/writable; imports resolve; no placeholders, credentials, secrets, hard-coded waits, or live system calls were found
- HTML and JUnit report generation was observed; failure-trace generation was observed during the missing-WebKit validation path

### Per-Machine Browser Follow-Up

The installed Playwright 1.61.1 cache contains Chromium but not its matching WebKit revision. The managed network denied the WebKit CDN download with HTTP 403, so WebKit execution, screenshots, and video retention remain unverified on this machine.

This is a documented post-workflow environment action rather than a framework defect:

```sh
npm run test:e2e:install
npm run test:e2e
```

The first command installs the exact Chromium and WebKit revisions required by the locked Playwright version; the second runs the complete configured matrix.

### Knowledge Fragments Applied

- overview and framework-selection guidance;
- fixture composition with `mergeTests` and automatic cleanup;
- deterministic, override-friendly data factories;
- browser-first system-boundary interception through the fake Tauri runtime;
- automatic network-error monitoring and structured logging; and
- selector resilience, Given/When/Then steps, artifact retention, and isolation practices.

Authentication-session, API-request, recursive-polling, Pact, and database patterns were reviewed and intentionally omitted because they do not fit Pack-Manager's current boundaries.

### Recommended Next Workflows

1. Run `bmad-testarch-ci` when Playwright should become a CI gate.
2. Re-run `bmad-testarch-trace` so the traceability matrix recognizes the new browser framework and journey.
3. Run `bmad-testarch-atdd` against a specific approved story to generate red-phase acceptance tests on this foundation.

### Step 5 Outcome

The Playwright framework scaffold is complete. Chromium is execution-validated, the full framework is type-checked and regression-safe, and the remaining WebKit action is documented for the local machine.
