#!/usr/bin/env bash
# The court system renders EVERY central judgment to a full-cream A4 PDF (always-render + backfill).
# Idempotent: it (re)renders a judgment whose PDF is missing or older than its source markdown, and
# skips one whose PDF is already current. Run from anywhere in the repo.
#
# Usage:  Judicature/court/scripts/render-all-judgments.sh
# Wired into the pre-commit gate so a newly filed/changed judgment is always rendered and staged.
set -uo pipefail
ROOT="$(git rev-parse --show-toplevel 2>/dev/null || pwd)"
cd "$ROOT"

JDIR="Judicature/.justice/judgments"
PDIR="Judicature/.justice/pdfs"
CONV="Judicature/court/scripts/md_to_ruling_json.py"
REND="Judicature/court/renderer/index.js"
mkdir -p "$PDIR"

command -v node >/dev/null 2>&1 || { echo "render-all-judgments: node not found; skipping." >&2; exit 0; }
command -v python3 >/dev/null 2>&1 || { echo "render-all-judgments: python3 not found; skipping." >&2; exit 0; }
[ -f "$REND" ] || { echo "render-all-judgments: renderer not found ($REND); skipping." >&2; exit 0; }

n=0
for f in "$JDIR"/*/*.md; do
  [ -f "$f" ] || continue
  slug="$(basename "$f" .md)"
  pdf="$PDIR/$slug.pdf"
  # up to date? skip.
  if [ -f "$pdf" ] && [ "$pdf" -nt "$f" ]; then continue; fi
  # derive tier (frontmatter, else the court-dir name) + citation (the H1 title) from the source.
  tier="$(grep -m1 -E '^tier:' "$f" | sed -E 's/^tier:[[:space:]]*//; s/"//g')"
  [ -z "$tier" ] && tier="$(basename "$(dirname "$f")")"
  cite="$(grep -m1 -E '^# \[' "$f" | sed -E 's/^#[[:space:]]*//')"
  # the renderer only accepts a JSON arg whose name ends .json (index.js arg-parse), so the temp
  # file MUST carry a .json suffix - a bare mktemp path is silently rejected with the usage banner.
  json="$(mktemp --suffix=.json 2>/dev/null || mktemp)"
  case "$json" in *.json) ;; *) mv "$json" "$json.json" 2>/dev/null && json="$json.json" ;; esac
  if ! python3 "$CONV" "$f" --tier "$tier" --citation "$cite" --date '6 June 2026' > "$json" 2>/dev/null; then
    echo "render-all-judgments: WARNING - failed to convert $slug to ruling JSON" >&2
    rm -f "$json"; continue
  fi
  # Render with one retry: back-to-back puppeteer launches occasionally flake on a fresh-process
  # collision (a different judgment each run); a single re-attempt with a new browser clears it.
  if node "$REND" "$json" "$pdf" >/dev/null 2>&1 || node "$REND" "$json" "$pdf" >/dev/null 2>&1; then
    echo "rendered $slug"; n=$((n+1))
  else
    echo "render-all-judgments: WARNING - failed to render $slug (after retry)" >&2
  fi
  rm -f "$json"
done
echo "render-all-judgments: $n rendered (the rest were already current)"
