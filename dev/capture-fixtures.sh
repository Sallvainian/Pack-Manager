#!/usr/bin/env bash
#
# dev/capture-fixtures.sh — re-capture live package-manager outputs as
# date-stamped fixtures for the parser test suite.
#
# Provenance:  SPEC §5.1 (repo layout), §5.4/§5.5 (command surface),
#              DECISIONS D9 (capture inventory fixtures live BEFORE writing
#              the parsers), IMPL_PLAN U3.
#
# Default run captures only OFFLINE-SAFE commands: pure inventory / listing
# that read local state and never touch the network. These are the fixtures
# the parsers are grounded in. Every file is written to dev/fixtures/ with a
# YYYY-MM-DD stamp and NEVER overwrites an existing capture (a same-day rerun
# refuses to clobber; delete by hand if you really mean to re-take one).
#
# The NETWORK-dependent "outdated" probes (brew outdated, mise outdated,
# npm outdated, uv tool list --outdated, rustup check) are gated behind
# PM_CAPTURE_ONLINE=1 because their results vary with the network and the
# machine's current update state; populated captures of those feed the
# "capture backlog" documented in dev/fixtures/README.md.
#
# Nothing in this script mutates the system: no install, upgrade, or update.
#
set -uo pipefail

FIXDIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/fixtures" && pwd)"
STAMP="$(date +%Y-%m-%d)"

# Constructed child env mirrors the app's spawn environment (SPEC §5.2) so the
# captured bytes match what the running app will parse: no color, no emoji,
# no pager, no auto-update side effects.
export NO_COLOR=1 TERM=dumb LANG=en_US.UTF-8 GIT_TERMINAL_PROMPT=0 \
       HOMEBREW_COLOR=0 HOMEBREW_NO_EMOJI=1 HOMEBREW_NO_ENV_HINTS=1 \
       HOMEBREW_NO_AUTO_UPDATE=1 HOMEBREW_NO_INSTALL_CLEANUP=1

# capture <outfile-basename> <cmd> [args...]
capture() {
  local base="$1"; shift
  local out="$FIXDIR/$base"
  local bin="$1"
  if ! command -v "$bin" >/dev/null 2>&1; then
    printf '  SKIP   %-46s (%s not installed)\n' "$*" "$bin"
    return 0
  fi
  if [ -e "$out" ]; then
    printf '  KEEP   %-46s (exists: %s)\n' "$*" "$base"
    return 0
  fi
  "$@" >"$out" 2>/dev/null
  local rc=$?
  printf '  WROTE  %-46s -> %s (%s bytes, exit %s)\n' \
    "$*" "$base" "$(wc -c <"$out" | tr -d ' ')" "$rc"
}

echo "Capturing offline-safe inventory fixtures into $FIXDIR (stamp $STAMP)"
capture "brew_list_versions_${STAMP}.txt"       brew   list --versions
capture "brew_list_cask_versions_${STAMP}.txt"  brew   list --cask --versions
capture "mise_ls_${STAMP}.json"                 mise   ls --json
capture "npm_ls_g_${STAMP}.json"                npm    ls -g --depth=0 --json
capture "rustup_toolchain_list_${STAMP}.txt"    rustup toolchain list
capture "uv_tool_list_${STAMP}.txt"             uv     tool list

if [ "${PM_CAPTURE_ONLINE:-0}" = "1" ]; then
  echo "PM_CAPTURE_ONLINE=1 — capturing network-dependent outdated fixtures"
  capture "brew_outdated_${STAMP}.json"         brew   outdated --json=v2
  capture "brew_outdated_greedy_${STAMP}.json"  brew   outdated --json=v2 --greedy
  capture "mise_outdated_${STAMP}.json"         mise   outdated --json
  capture "npm_outdated_g_${STAMP}.json"        npm    outdated -g --json
  capture "uv_tool_list_outdated_${STAMP}.txt"  uv     tool list --outdated
  capture "rustup_check_${STAMP}.txt"           rustup check
else
  echo "(skipping network-dependent outdated probes; set PM_CAPTURE_ONLINE=1 to include them)"
fi

echo "Done. Review new files, then commit. Update dev/fixtures/README.md provenance."
