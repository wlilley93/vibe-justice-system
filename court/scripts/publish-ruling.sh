#!/usr/bin/env bash
# publish-ruling.sh - VPR 8 publication of a Supreme Court ruling to the Community Record.
#
# This is the deterministic publication step the runnable courts must invoke (or the one
# writer must run) immediately after any Supreme Court judgment. It closes the gap where a
# ruling was decided but never reached the Community Record / the sovereign statute.
#
# Usage:  court/scripts/publish-ruling.sh <citation-slug> [--enacts]
#   <citation-slug>  e.g. 2026-lexby-sc-2  (maps to community/caselaw/<year>/<slug>.md)
#   --enacts         the ruling enacts new SPEC-LAW article(s) already appended to SPEC-LAW.md (VPR 6)
#
# Preconditions (Lexby prepares; the script enforces + publishes, failing closed):
#   - community/caselaw/<year>/<slug>.md exists and is ANONYMISED per VPR 8.
#   - if --enacts: SPEC-LAW.md already carries the new article(s).
# Effect: branch -> commit -> push -> open a PR to the canonical VJS repo (gh).
set -euo pipefail
SLUG="${1:?usage: publish-ruling.sh <citation-slug> [--enacts]}"
ENACTS="${2:-}"
YEAR="$(printf '%s' "$SLUG" | grep -oE '[0-9]{4}' | head -1)"
RULING="community/caselaw/${YEAR}/${SLUG}.md"
[ -f "$RULING" ] || { echo "FAIL (VPR 8): anonymised ruling $RULING not found; prepare it before publishing."; exit 1; }
grep -qiE 'anonymised|<project>' "$RULING" || { echo "FAIL (VPR 8): $RULING shows no anonymisation; project identifiers must be stripped."; exit 1; }
command -v gh >/dev/null || { echo "FAIL: gh CLI not available."; exit 1; }
BRANCH="publish/${SLUG}"
git checkout -b "$BRANCH" 2>/dev/null || git checkout "$BRANCH"
FILES=("$RULING")
[ "$ENACTS" = "--enacts" ] && FILES+=("SPEC-LAW.md")
git add "${FILES[@]}"
git commit -m "VPR 8: publish ${SLUG} to the Community Record${ENACTS:+ + statute enactment (VPR 6)}" >/dev/null 2>&1 || echo "(nothing new to commit)"
git push -u origin "$BRANCH"
gh pr create \
  --title "VPR 8: ${SLUG} to the Community Record${ENACTS:+ (+ statute)}" \
  --body "Automated VPR 8 publication of Supreme Court ruling ${SLUG}, anonymised per VPR 8.${ENACTS:+ Enacts new SPEC-LAW article(s) per VPR 6; clerk to review constitutional compliance before merge.}" \
  --base main --head "$BRANCH" 2>&1 | tail -2
echo "Published: PR opened for ${SLUG}."
