# Pack-Manager Deployment Guide

- **Date:** 2026-07-22
- **Target:** Universal macOS application (`arm64` + `x86_64`)

## Release Model

Pack-Manager uses release-please. A release is approved by merging the generated release PR; versions and changelog entries are not edited manually.

```text
Conventional commits on main
  → release-please PR
  → merge release PR
  → tag vX.Y.Z and GitHub Release
  → reusable macOS build workflow
  → signed/notarized DMG, ZIP, and updater assets
```

## Version Ownership

Release-please keeps these five files in lockstep:

- `package.json`
- `package-lock.json`
- `src-tauri/tauri.conf.json`
- `src-tauri/Cargo.toml`
- `src-tauri/Cargo.lock`

It also owns `CHANGELOG.md` and `.release-please-manifest.json`.

Do not edit these versions, the changelog, or the release manifest by hand. A mismatch can make the app bundle, diagnostics version, Cargo lock checks, and GitHub tag disagree.

## Version Selection

Conventional commits determine the release level:

| Commit | Release effect |
| --- | --- |
| `fix: ...` | Patch version |
| `feat: ...` | Minor version |
| `feat!: ...` or `BREAKING CHANGE:` footer | Major version |
| `chore:`, `docs:`, `ci:`, `refactor:`, `test:` | No release by themselves |

To request a specific version, add a separate `Release-As: X.Y.Z` footer to a conventional commit.

## Automated Release Flow

### 1. Release PR

On each push to `main`, `.github/workflows/release-please.yml` reads conventional commits and opens or updates `chore(main): release X.Y.Z`.

Review the PR's synchronized version files and generated changelog. Merging the PR is the human release approval.

### 2. Tag and GitHub Release

After merge, release-please creates `vX.Y.Z` and publishes the GitHub Release. The workflow then calls `.github/workflows/release.yml` directly; it does not depend on a tag-trigger recursion that GitHub blocks for `GITHUB_TOKEN` events.

### 3. Universal Build

The macOS 14 build job:

1. Installs Node 24 and stable Rust.
2. Adds both Apple Rust targets.
3. Verifies version agreement and the release tag.
4. Builds one universal Tauri application.
5. Signs with Developer ID credentials when available.
6. Notarizes and staples the application and DMG when Apple credentials are available.
7. Creates the installer, archive, updater payload/signature, and updater metadata.
8. Runs Gatekeeper/signature checks for signed output.
9. Attaches assets to the existing GitHub Release.

## Release Artifacts

The release workflow produces:

- `Pack-Manager-X.Y.Z.dmg` — user installer.
- `Pack-Manager-X.Y.Z.zip` — zipped application distribution.
- Universal `.app.tar.gz` and `.sig` — Tauri updater payload and signature.
- `latest.json` — updater version, URL, signature, and publication metadata.

The installed app checks the latest GitHub Release endpoint configured in `src-tauri/tauri.conf.json`. A discovered update downloads automatically when enabled, but installation/restart remains user-triggered.

## Signing and Notarization

Two separate signing concerns exist:

- Tauri updater minisign credentials sign updater artifacts.
- Apple Developer ID/App Store Connect credentials sign and notarize the macOS app and installer.

Local updater credentials are age-encrypted in `fnox.toml`; CI credentials are stored as GitHub Secrets. Never place credential values in repository files or command output.

Apple signing/notarization is designed to degrade gracefully when Apple credentials are unavailable. Updater signing is effectively required for the normal configured release build because updater artifacts and a public key are enabled.

The workflow imports the Developer ID certificate into a temporary keychain so both the Tauri app build and later DMG packaging can use the identity. Temporary signing material is cleaned up when the job ends.

## Test the Build Pipeline Without Releasing

Use GitHub Actions → **Release** → **Run workflow**.

A manual run:

- Builds the same universal application path.
- Uploads DMG and ZIP files to the workflow run.
- Does not create or modify a GitHub Release.
- Does not create a version tag.

For a local unsigned smoke build:

```sh
npm install
npm run tauri build -- --no-sign
```

For a local updater-signed build:

```sh
fnox exec -- npm run tauri build
```

## Pre-Release Verification

Before merging a release PR, confirm the current `main` checks are green:

```sh
cd src-tauri
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test --locked

cd ..
npm ci
npx tsc --noEmit
npm test
npm run build
```

For changes that affect detection or real package-manager behavior, also run the explicitly ignored smoke tests on the target Mac:

```sh
cd src-tauri
cargo test -- --ignored
```

## CI Architecture

`.github/workflows/ci.yml` separates verification into:

- Rust on macOS 14: formatting, Clippy with warnings denied, and locked tests.
- Web on Ubuntu: clean npm install, TypeScript, Vitest, and production Vite build.
- Main-only macOS build smoke: unsigned debug application bundle uploaded as an Actions artifact.

`.github/workflows/test.yml` adds browser verification on Ubuntu: Playwright TypeScript/discovery checks, two shards collectively covering the configured Chromium and WebKit projects, pull-request and weekly ten-pass burn-in, and merged HTML/JUnit artifacts. It requires no repository secrets.

The release workflow is intentionally independent from the ordinary CI jobs but should only be reached from a reviewed, green `main` branch.

## Failure Diagnosis

- **Version mismatch:** do not patch individual version files. Correct the conventional-commit/release-please input and regenerate the release PR.
- **Updater private key error:** run the build through `fnox exec` locally or verify the corresponding GitHub Secrets are available to CI.
- **Apple signing unavailable:** inspect the signing/notarization setup steps. Forks or untrusted contexts may intentionally produce unsigned artifacts.
- **Gatekeeper or stapling failure:** inspect the identity selection, notarization response, and final verification steps before distributing the artifact.
- **Updater cannot install in place:** the app detects a non-writable bundle parent and enters a manual-install-required state rather than asking for administrator credentials.

## Rollback

The repository does not define an automated rollback workflow. If a published build must be withdrawn:

1. Stop promoting the affected artifact and update/retract the GitHub Release as appropriate.
2. Fix the issue on `main` with a conventional commit.
3. Merge the resulting release-please PR to publish a newer version.

Do not reuse or overwrite an existing version tag/artifact; ship a new version so the updater and installed application can reason about ordering correctly.
