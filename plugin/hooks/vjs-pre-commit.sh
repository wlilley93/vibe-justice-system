#!/usr/bin/env bash
# vjs-pre-commit.sh - the VJS hard gate.
#
# The watchdog (vjs-watchdog.sh) is the soft, model-based backstop for behaviour. THIS is the
# hard, deterministic gate for the RECORD: it fails closed, with no model in the loop, on the two
# ways a jurisdiction silently corrupts itself:
#   - citation collisions (the same [YEAR] LEXBY-<TIER> N issued twice), and
#   - filing breaks (a ruling file with no citator row, or a citator row with no ruling file).
#
# Install it as the repo's git pre-commit hook (cdd init does this for you, or symlink it:
#   ln -sf ../../.claude/hooks/vjs-pre-commit.sh .git/hooks/pre-commit).
# It runs `cdd check-citator`; if the citator is sound it is silent and exits 0, otherwise it
# prints the problems and exits 1, blocking the commit.
#
# Bypass for a deliberate, exceptional commit: git commit --no-verify (use sparingly; the gate
# exists precisely so the record never lies).
set -euo pipefail

ROOT="$(git rev-parse --show-toplevel 2>/dev/null || pwd)"
cd "$ROOT"

# Resolve the CLI: prefer an installed `cdd`/`vjs`, else the vendored cli/bin/cdd.js, else skip.
run_check() {
  if command -v cdd >/dev/null 2>&1; then cdd check-citator; return $?; fi
  if command -v vjs >/dev/null 2>&1; then vjs check-citator; return $?; fi
  if [ -f cli/bin/cdd.js ] && command -v node >/dev/null 2>&1; then node cli/bin/cdd.js check-citator; return $?; fi
  if [ -f .justice/cli/bin/cdd.js ] && command -v node >/dev/null 2>&1; then node .justice/cli/bin/cdd.js check-citator; return $?; fi
  echo "VJS pre-commit: cdd CLI not found; skipping citator audit (install the CLI to enforce)." >&2
  return 0
}

if ! run_check; then
  echo "" >&2
  echo "VJS hard gate: the citator audit failed (see above). The commit is blocked because the" >&2
  echo "record would be left inconsistent. Fix the citator/ruling files, or, for a deliberate" >&2
  echo "exception, re-run with: git commit --no-verify" >&2
  exit 1
fi
exit 0
