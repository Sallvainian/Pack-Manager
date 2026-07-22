# Pack-Manager

macOS GUI for package managers (Tauri 2 + React 19 + TypeScript + Vite;
Rust backend in `src-tauri/`).

## Releases & versioning

Releases are fully automated by release-please. The flow:

1. Push conventional commits to `main`.
2. release-please opens/updates a `chore(main): release X.Y.Z` PR
   (bumps `package.json`, `package-lock.json`, `src-tauri/tauri.conf.json`,
   `src-tauri/Cargo.toml` + `Cargo.lock`, and writes `CHANGELOG.md`).
3. Merging that PR **is** the release: it creates tag `vX.Y.Z`, publishes the
   GitHub Release, and `.github/workflows/release.yml` builds a universal
   (arm64 + x86_64), signed, notarized app and attaches
   `Pack-Manager-X.Y.Z.dmg` / `.zip`.

### Commit messages: conventional commits, always

release-please computes the next version from commit messages on `main`:

| Prefix                                | Effect        |
| ------------------------------------- | ------------- |
| `fix: …`                              | patch bump    |
| `feat: …`                             | minor bump    |
| `feat!: …` / `BREAKING CHANGE:` footer| major bump    |
| `chore:`, `docs:`, `ci:`, `refactor:`, `test:` | no release |

To force a specific version, add a `Release-As: X.Y.Z` footer (own paragraph)
to any commit.

### Never edit versions by hand

- The app version lives in **five** files that release-please keeps in
  lockstep: `package.json`, `package-lock.json`, `src-tauri/tauri.conf.json`,
  `src-tauri/Cargo.toml`, `src-tauri/Cargo.lock`. Only the release PR bumps
  them.
- The version the bundle ships with comes from `src-tauri/tauri.conf.json`;
  the Rust code reports `env!("CARGO_PKG_VERSION")` (Cargo.toml) in
  diagnostics — that's why Cargo.toml/Cargo.lock must stay in lockstep
  (`cargo test --locked` in CI also depends on it).
- `CHANGELOG.md` and `.release-please-manifest.json` are release-please-owned;
  don't edit them manually.

### Testing the build pipeline

Actions → Release → Run workflow: builds/signs/notarizes and uploads the
artifacts to the run only — it never touches a GitHub Release.
