#!/usr/bin/env bash
# vjs-pre-commit.sh - the VJS hard gate.
#
# The watchdog (vjs-watchdog.sh) is the soft, model-based backstop for behaviour. THIS is the
# hard, deterministic gate for the RECORD: it fails closed, with no model in the loop, on the two
# ways a jurisdiction silently corrupts itself:
#   - citation collisions (the same [YEAR] REALM-<CODE> N issued twice), and
#   - filing breaks (a ruling file with no citator row, or a citator row with no ruling file).
#
# It then keeps the DERIVED PROJECTIONS in lockstep: when a commit touches the law sources
# (.justice/INDEX.md, .justice/judgments/, legislature/bills/), it regenerates the pointer-only
# law-site corpus + search index and the rulings ledger and stages them, so the index can never
# silently drift from the law ([2026] REALM-PC 4; Bill 16 s. 12(2)). The citator audit is
# fail-CLOSED (it guards the legal record); the projection rebuild is fail-OPEN (a convenience
# layer, so a build hiccup warns rather than blocks). The git-history-based reasons ledger
# (ministry-of-justice/reasons-ledger/) is rebuilt at milestones, not per-commit (by design it
# lags the in-progress commit by one).
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
  if [ -f Executive/cli/bin/cdd.js ] && command -v node >/dev/null 2>&1; then node Executive/cli/bin/cdd.js check-citator; return $?; fi
  if [ -f cli/bin/cdd.js ] && command -v node >/dev/null 2>&1; then node cli/bin/cdd.js check-citator; return $?; fi
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

# --- Derived-projection lockstep (REALM-PC 4 / Bill 16 s. 12(2)) --------------------------------
# When this commit touches the law sources, regenerate the derived, pointer-only projections and
# stage them so the index can never silently drift from the law, nor be forgotten. Fail-OPEN: the
# citator hard gate above already protects the legal record; the projections are a convenience
# layer, so a build hiccup warns and the commit proceeds (rebuild manually if so). Errors are
# trapped so they can never abort the commit under `set -e`.
staged="$(git diff --cached --name-only 2>/dev/null || true)"
if printf '%s\n' "$staged" | grep -qE '^Judicature/\.justice/INDEX\.md$|^Judicature/\.justice/judgments/|^Legislature/legislature/bills/'; then
  # REALM-SI 2 (the Judgment Rendering and Lodgement Instrument): invoke the first-class deterministic
  # render-and-lodge verb that the Instrument mandates. It renders judgment PDFs (idempotent), rebuilds
  # the law-site corpus + search index + rulings-ledger projections in lockstep, and verifies the
  # citation layer. Fail-OPEN here (the hard fail-closed citation gate already ran above via run_check;
  # the verb's render + projections are the convenience layer, so a hiccup warns and the commit proceeds).
  if [ -f Executive/cli/bin/cdd.js ] && command -v node >/dev/null 2>&1; then
    if node Executive/cli/bin/cdd.js lodge-judgment >/dev/null 2>&1; then
      echo "VJS pre-commit: render-and-lodge ran (cdd lodge-judgment, REALM-SI 2): PDFs + projections rebuilt in lockstep." >&2
    else
      echo "VJS pre-commit: WARNING - render-and-lodge (cdd lodge-judgment) reported an issue; committing the convenience layer fail-open (rebuild manually)." >&2
    fi
    git add Judicature/.justice/pdfs Judicature/law-reports/corpus.json Judicature/law-reports/site/search-index.json Judicature/ministry-of-justice/ledger/INDEX.md >/dev/null 2>&1 || true
  else
    echo "VJS pre-commit: WARNING - cdd CLI or node missing; render-and-lodge NOT run (REALM-SI 2 / REALM-PC 4 lockstep not enforced this commit)." >&2
  fi
fi
exit 0
