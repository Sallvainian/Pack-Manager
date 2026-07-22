/**
 * test/fixtures.ts — typed frontend fixtures hand-derived from the real command
 * captures in `dev/fixtures/` (SPEC §7.5). Version strings are copied verbatim
 * from those captures so the UI is exercised against realistic, non-semver data
 * (`2.0.14-1`, `1.6.2.dev0`, `stable`, rustup commit hashes).
 */
import type {
  DetectionReport,
  ManagerInfo,
  ManagerSnapshot,
  OperationRecord,
  Settings,
} from "../lib/ipc/types";

const HOME = "/Users/sallvain";

export const defaultSettings: Settings = {
  runBrewUpdateOnRefresh: true,
  autoRefreshOnLaunch: true,
  stallAfterSecs: 120,
  upgradeHardCapMins: 30,
  logLevel: "debug",
  autoOpenDrawer: true,
  includeGreedyByDefault: false,
};

// --- Manager infos (detection results on this machine) ----------------------

export const brewInfo: ManagerInfo = {
  id: "brew",
  displayName: "Homebrew",
  status: "present",
  binaryPath: "/opt/homebrew/bin/brew",
  canonicalPath: "/opt/homebrew/bin/brew",
  version: "4.5.2",
  managedBy: "standalone",
  evidence: "resolved at /opt/homebrew/bin/brew — Homebrew's own tree",
  selfUpdate: { kind: "viaRefresh", note: "brew update runs as part of every refresh" },
};

export const miseInfo: ManagerInfo = {
  id: "mise",
  displayName: "mise",
  status: "present",
  binaryPath: "/opt/homebrew/bin/mise",
  canonicalPath: "/opt/homebrew/bin/mise",
  version: "2026.7.1",
  managedBy: "brew",
  evidence: "resolved at /opt/homebrew/bin/mise — Homebrew's own tree",
  selfUpdate: {
    kind: "routed",
    executor: "brew",
    commandPreview: "brew upgrade mise",
    why: "mise is managed by Homebrew",
  },
};

export const npmInfo: ManagerInfo = {
  id: "npm",
  displayName: "npm",
  status: "present",
  binaryPath: `${HOME}/.local/share/mise/shims/npm`,
  canonicalPath: "/opt/homebrew/bin/mise",
  version: "11.16.0",
  managedBy: "mise",
  evidence: "resolved at ~/.local/share/mise/shims/npm",
  selfUpdate: {
    kind: "inBand",
    commandPreview: "npm install -g npm@latest",
    note: "npm and all global packages live inside the mise-managed node — upgrading node via mise resets them.",
  },
};

export const uvInfo: ManagerInfo = {
  id: "uv",
  displayName: "uv",
  status: "present",
  binaryPath: `${HOME}/.local/share/mise/shims/uv`,
  canonicalPath: "/opt/homebrew/bin/mise",
  version: "0.11.26",
  managedBy: "mise",
  evidence: "resolved at ~/.local/share/mise/shims/uv",
  selfUpdate: {
    kind: "routed",
    executor: "mise",
    commandPreview: "mise upgrade uv",
    why: "uv is managed by mise",
  },
};

export const rustupInfo: ManagerInfo = {
  id: "rustup",
  displayName: "rustup",
  status: "present",
  binaryPath: `${HOME}/.cargo/bin/rustup`,
  canonicalPath: `${HOME}/.cargo/bin/rustup`,
  version: "1.28.2",
  managedBy: "rustup",
  evidence: "resolved at ~/.cargo/bin/rustup — rustup's own tree",
  selfUpdate: { kind: "inBand", commandPreview: "rustup self update" },
};

export const masInfo: ManagerInfo = {
  id: "mas",
  displayName: "mas",
  status: "absent",
  managedBy: "standalone",
  selfUpdate: { kind: "unavailable", reason: "mas is not installed" },
  installHint: "brew install mas",
};

export const detectionReport: DetectionReport = {
  managers: [brewInfo, miseInfo, npmInfo, uvInfo, rustupInfo, masInfo],
  env: {
    path: `${HOME}/.local/share/mise/shims:/opt/homebrew/bin:/opt/homebrew/sbin:${HOME}/.cargo/bin:/usr/local/bin:/usr/bin:/bin:/usr/sbin:/sbin`,
    entries: [
      `${HOME}/.local/share/mise/shims`,
      "/opt/homebrew/bin",
      "/opt/homebrew/sbin",
      `${HOME}/.cargo/bin`,
      "/usr/local/bin",
      "/usr/bin",
      "/bin",
      "/usr/sbin",
      "/sbin",
    ],
    source: "merged",
    home: HOME,
  },
};

// --- Snapshots (merged inventory + outdated overlay) ------------------------

const AT = "2026-07-22T14:05:00Z";

/** Homebrew: dolt outdated formula; three self-updating (greedy) casks. */
export const brewSnapshot: ManagerSnapshot = {
  managerId: "brew",
  refreshedAt: AT,
  packages: [
    {
      id: "formula:dolt",
      name: "dolt",
      kind: "formula",
      installed: "2.2.1",
      latest: "2.2.2",
      outdated: true,
      pinned: false,
    },
    {
      id: "formula:deno",
      name: "deno",
      kind: "formula",
      installed: "2.9.0",
      latest: "2.9.3",
      outdated: true,
      pinned: true,
      meta: { pinnedVersion: "2.9.0" },
    },
    {
      id: "formula:jq",
      name: "jq",
      kind: "formula",
      installed: "1.7.1",
      latest: "1.7.1",
      outdated: false,
      pinned: false,
    },
    {
      id: "caskGreedy:openusage",
      name: "openusage",
      kind: "caskGreedy",
      installed: "0.6.20",
      latest: "0.7.6",
      outdated: true,
      pinned: false,
    },
    {
      id: "caskGreedy:syncthing-app",
      name: "syncthing-app",
      kind: "caskGreedy",
      installed: "2.0.14-1",
      latest: "2.1.2-1",
      outdated: true,
      pinned: false,
    },
    {
      id: "caskGreedy:transmission",
      name: "transmission",
      kind: "caskGreedy",
      installed: "4.1.1",
      latest: "4.1.3",
      outdated: true,
      pinned: false,
    },
  ],
  health: [],
};

/** mise: six outdated tools; `rust stable stable stable` is NOT outdated. */
export const miseSnapshot: ManagerSnapshot = {
  managerId: "mise",
  refreshedAt: AT,
  packages: [
    {
      id: "tool:rust",
      name: "rust",
      kind: "tool",
      installed: "stable",
      latest: "stable",
      outdated: false,
      pinned: false,
      meta: { source: "~/.config/mise/config.toml" },
    },
    {
      id: "tool:deno",
      name: "deno",
      kind: "tool",
      installed: "2.9.0",
      latest: "2.9.3",
      outdated: true,
      pinned: false,
      meta: { requested: "latest", source: "~/.config/mise/config.toml" },
    },
    {
      id: "tool:ruby",
      name: "ruby",
      kind: "tool",
      installed: "4.0.5",
      latest: "4.0.6",
      outdated: true,
      pinned: false,
      meta: { requested: "latest", source: "~/.config/mise/config.toml" },
    },
    {
      id: "tool:fnox",
      name: "fnox",
      kind: "tool",
      installed: "1.28.0",
      latest: "1.31.0",
      outdated: true,
      pinned: false,
      meta: { requested: "latest", source: "~/.config/mise/config.toml" },
    },
    {
      id: "tool:ruff",
      name: "ruff",
      kind: "tool",
      installed: "0.15.20",
      latest: "0.15.22",
      outdated: true,
      pinned: false,
      meta: { requested: "latest", source: "~/.config/mise/config.toml" },
    },
    {
      id: "tool:npm:prettier",
      name: "npm:prettier",
      kind: "tool",
      installed: "3.9.4",
      latest: "3.9.5",
      outdated: true,
      pinned: false,
      meta: { requested: "latest", source: "~/.config/mise/config.toml" },
    },
    {
      id: "tool:uv",
      name: "uv",
      kind: "tool",
      installed: "0.11.26",
      latest: "0.11.30",
      outdated: true,
      pinned: false,
      meta: { requested: "latest", source: "~/.config/mise/config.toml" },
    },
  ],
  selfStatus: { installed: "2026.7.1", latest: "2026.7.4", updateAvailable: true },
  health: [],
};

/** npm: four outdated global packages; the `npm` row is hoisted to selfStatus. */
export const npmSnapshot: ManagerSnapshot = {
  managerId: "npm",
  refreshedAt: AT,
  packages: [
    {
      id: "globalPackage:@google/gemini-cli",
      name: "@google/gemini-cli",
      kind: "globalPackage",
      installed: "0.49.0",
      latest: "0.51.0",
      outdated: true,
      pinned: false,
      meta: { wanted: "0.51.0", dependedBy: "global" },
    },
    {
      id: "globalPackage:@just-every/code",
      name: "@just-every/code",
      kind: "globalPackage",
      installed: "0.6.134",
      latest: "0.6.149",
      outdated: true,
      pinned: false,
      meta: { wanted: "0.6.149", dependedBy: "global" },
    },
    {
      id: "globalPackage:dmux",
      name: "dmux",
      kind: "globalPackage",
      installed: "5.9.0",
      latest: "5.10.0",
      outdated: true,
      pinned: false,
      meta: { wanted: "5.10.0", dependedBy: "global" },
    },
    {
      id: "globalPackage:typescript",
      name: "typescript",
      kind: "globalPackage",
      installed: "6.0.3",
      latest: "7.0.2",
      outdated: true,
      pinned: false,
      meta: { wanted: "7.0.2", dependedBy: "global" },
    },
  ],
  selfStatus: { installed: "11.16.0", latest: "12.0.1", updateAvailable: true },
  health: [],
};

/** uv: one outdated tool (unknown latest → null), one non-semver up-to-date
 * tool, a broken-environment health issue, and a routed self-status. */
export const uvSnapshot: ManagerSnapshot = {
  managerId: "uv",
  refreshedAt: AT,
  packages: [
    {
      id: "tool:ruff",
      name: "ruff",
      kind: "tool",
      installed: "0.15.20",
      latest: null,
      outdated: true,
      pinned: false,
      meta: { executables: ["ruff"] },
    },
    {
      id: "tool:serena-agent",
      name: "serena-agent",
      kind: "tool",
      installed: "1.6.2.dev0",
      latest: "1.6.2.dev0",
      outdated: false,
      pinned: false,
      meta: { executables: ["serena", "serena-agent", "serena-hooks"] },
    },
  ],
  selfStatus: { installed: "0.11.26", latest: "0.11.30", updateAvailable: true },
  health: [
    {
      id: "uv:aider-chat",
      managerId: "uv",
      severity: "warning",
      title: "Tool `aider-chat` environment is broken.",
      detail:
        "warning: Tool `aider-chat` environment not found (run `uv tool install aider-chat --reinstall` to reinstall)",
      fixCommand: "uv tool install aider-chat --reinstall",
      fixable: true,
    },
  ],
};

/** rustup: one outdated toolchain; the `rustup` row drives selfStatus. */
export const rustupSnapshot: ManagerSnapshot = {
  managerId: "rustup",
  refreshedAt: AT,
  packages: [
    {
      id: "toolchain:stable-aarch64-apple-darwin",
      name: "stable-aarch64-apple-darwin",
      kind: "toolchain",
      installed: "1.94.0",
      latest: "1.97.1",
      outdated: true,
      pinned: false,
      meta: { source: "default" },
    },
  ],
  selfStatus: { installed: "1.28.2", latest: "1.29.0", updateAvailable: true },
  health: [],
};

/** An empty (all up-to-date) snapshot for the "clean" state. */
export function cleanSnapshot(managerId: ManagerSnapshot["managerId"]): ManagerSnapshot {
  return { managerId, refreshedAt: AT, packages: [], health: [] };
}

// --- Operation records ------------------------------------------------------

export const upgradeRecord: OperationRecord = {
  opId: "01981f2e-6a3b-7c40-9d5e-1f2a3b4c5d6e",
  kind: "upgrade",
  executor: "npm",
  subject: "npm",
  status: "running",
  commandLine: `${HOME}/.local/share/mise/shims/npm install -g typescript@latest`,
  packageIds: ["globalPackage:typescript"],
  queuedAt: "2026-07-22T14:03:11Z",
  startedAt: "2026-07-22T14:03:11Z",
  finishedAt: null,
  exitCode: null,
  error: null,
  logPath: `${HOME}/Library/Logs/Pack-Manager/operations/2026-07-22T14-03-11_01981f2e_npm_upgrade.log`,
};

export const interruptedRecord: OperationRecord = {
  opId: "01981f2e-1111-7000-8000-aaaaaaaaaaaa",
  kind: "refresh",
  executor: "brew",
  subject: "brew",
  status: "interrupted",
  commandLine: "/opt/homebrew/bin/brew update",
  packageIds: [],
  queuedAt: "2026-07-22T13:00:00Z",
  startedAt: "2026-07-22T13:00:00Z",
  finishedAt: null,
  exitCode: null,
  error: null,
  logPath: `${HOME}/Library/Logs/Pack-Manager/operations/2026-07-22T13-00-00_01981f2e_brew_refresh.log`,
};
