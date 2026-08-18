# CI Secrets Checklist

## Playwright Workflow

`.github/workflows/test.yml` requires **no repository or environment secrets**.

- [x] Workflow token permissions are limited to `contents: read`.
- [x] Checkout credentials are not persisted after checkout.
- [x] Tests use deterministic in-browser Tauri fixtures rather than live accounts or package-manager state.
- [x] `BASE_URL` is unset so Playwright starts the repository's local Vite server.
- [x] Only GitHub-provided `GITHUB_*` runtime variables are used for run and artifact links.
- [x] No pull-request text or other `github.event.*` value is interpolated into a shell command.
- [x] Test reports are reviewed as potentially sensitive diagnostic output and retained for a limited period.

Do not create placeholder secrets for this workflow. If a future browser test needs a credential, manage the value through GitHub Actions secrets, pass it as an environment variable only to the smallest necessary step, and keep its local counterpart in fnox rather than `.env` or source control.

## Release Workflows

Signing, notarization, and updater credentials belong to the release workflows, not the Playwright workflow. Their names and operational checks are documented in [`deployment-guide.md`](./deployment-guide.md). Never copy their values into this checklist, logs, test artifacts, or workflow source.
