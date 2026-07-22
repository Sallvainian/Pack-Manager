# Fixture provenance

Every file in `dev/fixtures/` is either a **real capture** from the target
machine or a **labeled synthetic** (`*_synthetic.*`). The pure parsers in
`src-tauri/src/managers/parse/` and the §7.1 test suite are grounded in these
bytes; nothing is parsed against guessed shapes without a `_synthetic` label
and a retirement condition documented below.

- **Target machine:** macOS 27.0 (build `26A5388g`), arm64 (Apple M4) — the
  machine described in SPEC §2.
- **Re-capture:** `dev/capture-fixtures.sh` re-captures the offline-safe
  inventory/list commands (date-stamped, never clobbering an existing file).
  Network-dependent `outdated` probes are gated behind `PM_CAPTURE_ONLINE=1`
  (see the backlog at the end).
- `dev/fixtures/ipc/` is **not** covered here — it is owned by U1 (the IPC
  contract fixtures, guarded by the byte-equality contract test).

## Naming

- `*_YYYY-MM-DD.*` — real capture; the date in the filename is the capture date.
- `*_synthetic.*` — hand-written from a real capture's verbatim values because
  the real shape for that exact command is not yet available (see backlog).
- Undated real captures (e.g. `brew_outdated.json`) are earlier design-recon
  captures from the same target machine; their content was re-verified on
  2026-07-22. Their dated `_2026-07-21` siblings are the dated reference.

## Real captures

### Inventory / listing — captured 2026-07-22 by `capture-fixtures.sh` (this machine, offline)

| File | Command | Verified facts |
|---|---|---|
| `brew_list_versions_2026-07-22.txt` | `brew list --versions` | 258 lines. `brew list --versions` **includes casks** — all 15 casks below appear here too, so the formula inventory is 258 − 15 = **243 formulae** after deduping against the cask list. One `name version` pair per line. |
| `brew_list_cask_versions_2026-07-22.txt` | `brew list --cask --versions` | 15 casks. `ngrok 3.39.9,6pfVfGALLzX,a` proves a version token may contain commas (no spaces); parse the last whitespace-separated token as the version. |
| `mise_ls_2026-07-22.json` | `mise ls --json` | **11 tools** keyed by name; each value is an array of installed versions. `node` has 2 entries (`24.18.0` active, `25.9.0` inactive) → the parser picks the `active` one. `npm:prettier` is a key (name kept verbatim). Confirms the `mise ls --json` flag used by the refresh plan (SPEC §5.4, DECISIONS D9). |
| `npm_ls_g_2026-07-22.json` | `npm ls -g --depth=0 --json` | `{ name, dependencies: { pkg: { version, overridden, [resolved] } } }`. **15 global deps** incl. the `npm` self row (→ 14 after the self-hoist). `class-points` carries an extra `resolved` field (tolerated/ignored). |
| `rustup_toolchain_list_2026-07-22.txt` | `rustup toolchain list` | 1 toolchain: `stable-aarch64-apple-darwin (active, default)`. Real marker is `(active, default)`, not the bare `(default)` in SPEC §5.5 — the parser takes the first token and ignores the parenthetical. |
| `uv_tool_list_2026-07-22.txt` | `uv tool list` | Fresh re-capture; byte-identical to `uv_tool_list.txt` (uv state unchanged). 12 tools. |
| `mas_list_2026-07-22.txt` | `mas list` | **12 apps**, lifted verbatim from the 14:18:09 operation transcript. Retires `mas_list_synthetic.txt`, whose guessed `id Name (version)` shape was wrong in two ways real output proves: mas **right-aligns the app id**, so 9-digit ids carry a **leading space** (` 640199958`), and the name column is **space-padded** to the widest entry rather than single-spaced. `FireShot - Full web page screenshots` proves a name may contain both hyphens and spaces. |
| `mas_outdated_2026-07-22.txt` | `mas outdated` | **3 apps**, from the same 14:18:09 refresh as the list above — so the pair is self-consistent and exercises the overlay honestly (list says `5.20.0`; outdated says `5.20.0 -> 5.21.0`). Retires `mas_outdated_synthetic.txt`. The shape no guess would produce: mas pads the version column **inside the parens**, so `(4.2.2  -> 4.3.0)` leaves trailing spaces on the installed version that the parser must trim. |

### Outdated / status — earlier recon captures (this machine)

| File | Command | Verified facts |
|---|---|---|
| `brew_outdated.json` | `brew outdated --json=v2` | Line 1 is junk (`✔︎ JSON API packages.arm64_golden_gate.jws.json`) before the `{` — skip until the first line whose trim starts with `{`. `formulae`: `dolt` `2.2.1 → 2.2.2`, `pinned: false`. `casks: []`. |
| `brew_outdated_greedy.json` | `brew outdated --json=v2 --greedy` | Same `dolt` formula, **no junk line** (starts with `{`). `casks: []`. |
| `brew_outdated_greedy_text_2026-07-21.txt` | `brew outdated --greedy` (text) | 3 greedy casks: `openusage (0.6.20) != 0.7.6`, `syncthing-app (2.0.14-1) != 2.1.2-1`, `transmission (4.1.1) != 4.1.3`. Wired as the **recovery** parser for the unverified cask JSON shape. `2.0.14-1` proves non-semver versions. |
| `mise_outdated.json` | `mise outdated --json` | `{}` — clean means empty object (verified). |
| `mise_outdated_text_2026-07-21.txt` | `mise outdated` (text) | No header row. 7 whitespace columns `tool requested current latest source`. **7 rows, 6 outdated**: `rust stable stable stable` has `current == latest` → NOT outdated (dropped). `npm:prettier` kept verbatim. `uv 0.11.26 → 0.11.30`. Source path → `meta.source`. Wired as the **recovery** parser for the unverified populated JSON shape. |
| `npm_outdated_g.json` | `npm outdated -g --json` | `{}` — clean (verified). (Exits 0 when clean; exits 1 when populated — the exit-1 rule lives in the adapter, not the parser.) |
| `npm_outdated_g_text_2026-07-21.txt` | `npm outdated -g` (text) | Header `Package Current Wanted Latest Location Depended by` + **5 rows**; the `npm` row (`11.16.0 → 12.0.1`) hoists to the self-update card → **4 package rows**. `wanted`/`location`/`dependent` → meta. Wired as the **recovery** parser for the unverified populated JSON shape. |
| `uv_tool_list.txt` | `uv tool list` | **12 tools**; `claude-code-tools` has **17 executables**; `serena-agent v1.6.2.dev0` proves non-semver versions. `- exe` lines accumulate into `meta.executables`. |
| `uv_tool_list_2026-07-21.txt` | `uv tool list` | First line is a `warning:` for a broken tool env: `` warning: Tool `aider-chat` environment not found (run `uv tool install aider-chat --reinstall` to reinstall) `` → HealthIssue + fix command. 12 tools. |
| `uv_tool_list_outdated.txt` | `uv tool list --outdated` | **0 bytes** — empty output means clean, not an error. |
| `rustup_check.txt` | `rustup check` | Both toolchain and self up to date, with **both colon spacings** in one file: `up to date: 1.97.1 (…)` and `up to date : 1.29.0`. The `\s*:\s*` in the regex is load-bearing. |
| `rustup_check_2026-07-21.txt` | `rustup check` | Outdated: toolchain `stable-aarch64-apple-darwin 1.94.0 → 1.97.1`; self `rustup 1.28.2 → 1.29.0`. Commit hashes/dates → meta. |
| `mas_outdated.txt` | `mas outdated` | `zsh: command not found: mas` — captured **while mas was absent** (superseded 2026-07-22 by the real captures above; kept deliberately). Detection gates an absent manager so this never reaches a parser, but a defensive test documents that feeding it to one yields `ParseFailed`, never a panic — the value is the shell-error shape, which no longer depends on mas being missing. |

## Synthetic fixtures (labeled, value-grounded — DECISIONS D8/D23)

| File | Stands in for | Values copied from | Retirement condition |
|---|---|---|---|
| `mise_outdated_synthetic.json` | populated `mise outdated --json` (only a `{}` clean capture exists; JSON shape UNVERIFIED) | `mise_outdated_text_2026-07-21.txt` (verbatim) | Retire once a real populated `mise outdated --json` is captured (`PM_CAPTURE_ONLINE=1`). The text parser is the wired recovery in the meantime. |
| `npm_outdated_g_synthetic.json` | populated `npm outdated -g --json` (only a `{}` clean capture exists; JSON shape UNVERIFIED) | `npm_outdated_g_text_2026-07-21.txt` (verbatim) | Retire once a real populated `npm outdated -g --json` is captured. |

Both mas synthetic fixtures were **retired on 2026-07-22**, their stated
condition ("once mas is installed and … is captured live") having been met:
mas 7.0.0 is installed and `mas_list_2026-07-22.txt` /
`mas_outdated_2026-07-22.txt` above are real captures. They are a worked
example of why the suffix exists — both guessed the column format wrong, and
the parser only tolerated real output by accident of being regex-lenient.

## Capture backlog (real fixtures still missing)

Run `PM_CAPTURE_ONLINE=1 dev/capture-fixtures.sh` when the machine has the
relevant outdated state; each capture retires the matching synthetic above or
firms up an under-verified parser branch.

- **Populated `mise outdated --json`** — the machine was fully up to date on
  2026-07-22 (`mise outdated --json` → `{}`), so no populated capture was
  possible this run. It had 6 outdated tools on 2026-07-21 (see the text
  capture), so the opportunity recurs whenever tools drift.
- **Populated `npm outdated -g --json`** — same: `{}` on 2026-07-22 (clean),
  5 outdated on 2026-07-21.
- **Populated `uv tool list --outdated`** — only the 0-byte clean capture
  exists; the populated line format is unknown. The parser captures a
  `(vX available)`-style suffix leniently as `latest` and degrades any unknown
  suffix to `latest: null` (UI shows "update available", never a fabricated
  delta — Judge 2's mandate). No `_synthetic` fixture is invented for a format
  we have not seen; the degradation branch is covered by an inline test string.
- **Populated brew cask JSON** — both `brew outdated --json=v2` captures have
  `"casks": []`, so the cask JSON shape is UNVERIFIED. The `BrewCask` serde
  struct is default-tolerant, and the greedy **text** parser
  (`openusage (0.6.20) != 0.7.6`) is the authoritative wired recovery until a
  populated cask JSON is captured.
- **All mas** — mas is not installed here; every mas fixture is synthetic.
