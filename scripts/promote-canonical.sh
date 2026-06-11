#!/usr/bin/env bash
# Promote master to vjs-canonical: the deliberate publication act.
#
# vjs-canonical is the published surface (the default branch; GitHub Pages
# serves the Gazette from it). It moves ONLY through this script: a boundary
# pass over everything master would newly publish, then a fast-forward push.
# Never force-push canonical; if the push is not a fast-forward, something is
# wrong - stop and look.
#
# Precedent: the publication-boundary line (BREACH-2026-06-10-client-data-
# published and its cure; the vjs gazette publication gate; the hashed
# denylist .vjs/publication-denylist.txt; the boundary-aware permit globs).
# Publication is constitutively inert - but exposure is real, so the boundary
# duty applies at the moment of publishing, which is this script.
set -euo pipefail
cd "$(git rev-parse --show-toplevel)"

echo "== promote-canonical: master -> vjs-canonical =="

[ -z "$(git status --porcelain)" ] || { echo "FAIL: working tree dirty"; exit 1; }

git fetch -q origin master vjs-canonical
MASTER=$(git rev-parse origin/master)
CANON=$(git rev-parse origin/vjs-canonical)
echo "master:        $MASTER"
echo "vjs-canonical: $CANON"

if [ "$MASTER" = "$CANON" ]; then
  echo "OK: canonical is already at master; nothing to promote."
  exit 0
fi

git merge-base --is-ancestor "$CANON" "$MASTER" \
  || { echo "FAIL: master does not fast-forward from canonical - resolve on master first, never force canonical"; exit 1; }

RANGE="$CANON..$MASTER"
echo "promoting range: $RANGE ($(git rev-list --count "$RANGE") commits)"

# --- the boundary pass: everything the range would newly publish ---

echo "-- 1/4 private-record paths"
BAD_PATHS=$(git diff --name-only "$RANGE" | grep -E '^\.vjs/private/|(^|/)\.env($|\.)|\.pem$|(^|/)id_rsa' || true)
[ -z "$BAD_PATHS" ] || { echo "FAIL: private paths in the range:"; echo "$BAD_PATHS"; exit 1; }

echo "-- 2/4 secrets (high-confidence patterns over the added lines)"
ADDED=$(git diff "$RANGE" | grep '^+' || true)
echo "$ADDED" | grep -nE "sk-[a-zA-Z0-9]{48}|gh[pousr]_[A-Za-z0-9_]{36,}|AKIA[0-9A-Z]{16}|-----BEGIN (RSA |EC |OPENSSH )?PRIVATE KEY" \
  | grep -v "store_boundary.rs\|synthetic\|0123456789abcdefghijklmnopqrstuvwxyz" \
  && { echo "FAIL: secret-shaped content in the range"; exit 1; } || echo "   clean"

echo "-- 3/4 denylisted private terms (hashed; the SC-11 class)"
# via a temp file: piping into a heredoc-fed python is a silent no-op (the
# heredoc overrides stdin) and pipefail turns the writer's SIGPIPE into a
# spurious failure - both found by the first dogfood run
DIFF_ADDED=$(mktemp)
git diff "$RANGE" | grep '^+' > "$DIFF_ADDED" || true
python3 - "$DIFF_ADDED" <<'PYEOF' || exit 1
import hashlib, sys
deny = set()
try:
    for line in open(".vjs/publication-denylist.txt"):
        line = line.strip()
        if line and not line.startswith("#"):
            deny.add(line)
except FileNotFoundError:
    sys.exit(0)
text = open(sys.argv[1], encoding="utf-8", errors="replace").read()
if not text.strip():
    print("FAIL: empty diff input - the sweep saw nothing, refusing to pass vacuously")
    sys.exit(1)
token = ""
hit = False
def check(t):
    global hit
    if len(t) >= 3 and hashlib.sha256(t.lower().encode()).hexdigest() in deny:
        hit = True
for ch in text:
    if ch.isalnum() or ch == "-":
        token += ch
    else:
        check(token); token = ""
check(token)
if hit:
    print("FAIL: a denylisted private term would be published"); sys.exit(1)
print("   clean")
PYEOF
rm -f "$DIFF_ADDED"

echo "-- 4/4 dev-machine paths in added content"
echo "$ADDED" | grep -nE "/home/[a-z]+/" | grep -v "promote-canonical.sh" \
  && { echo "FAIL: a dev-machine absolute path would be published"; exit 1; } || echo "   clean"

echo "-- validate (the lawpack gate)"
VJS=${VJS:-target/release/vjs}; [ -x "$VJS" ] || VJS=target/debug/vjs
"$VJS" validate >/dev/null && echo "   Validation: OK"

# --- the promotion: fast-forward only ---
git push origin "$MASTER:refs/heads/vjs-canonical"
echo "promoted: vjs-canonical -> $MASTER"

if command -v gh >/dev/null; then
  gh api -X POST repos/wlilley93/vibe-justice-system/pages/builds --jq '.status' 2>/dev/null \
    && echo "Pages rebuild queued" || true
fi
echo "== done =="
