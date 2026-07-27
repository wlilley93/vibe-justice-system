#!/usr/bin/env bash
# Fail-closed publication-boundary scan. The single scanner shared by the local
# pre-commit / pre-push hooks, the canon-enforce CI trust root, and
# promote-canonical. Scans added content (a range) or the whole HEAD tree for:
#   - denylisted private terms (hashed, .vjs/publication-denylist.txt)
#   - secret-shaped tokens (keys, GitHub tokens, private keys)
#   - dev-machine home paths
# Records ARE in scope (the 2026-06-10 leak came from records that the old gate
# skipped). Only vendored/minified assets are excluded (they carry no authored
# private data and would false-positive on library internals).
set -euo pipefail
cd "$(git rev-parse --show-toplevel)"
RANGE="${1:-}"
EXCL=(":(exclude)*/vendor/*" ":(exclude)*.min.*")
TMP=$(mktemp)
# SCOPE names what was actually scanned. A no-argument run reads the whole HEAD
# tree, so reporting those hits as "added content" made a STANDING tree condition
# read as a fresh introduction on every push.
if [ -n "$RANGE" ]; then
  git diff "$RANGE" -- . "${EXCL[@]}" | grep "^+" > "$TMP" || true
  SCOPE="in added content"
else
  git grep -I --no-color -h "" -- . "${EXCL[@]}" > "$TMP" 2>/dev/null || true
  SCOPE="in the HEAD tree"
fi
python3 - "$TMP" "$SCOPE" <<'PY'
import hashlib, re, sys
text = open(sys.argv[1], encoding="utf-8", errors="replace").read()
scope = sys.argv[2]
fail = False
# Synthetic fixtures that legitimately carry secret/path SHAPES (they test the
# scanner itself). Precedent: promote-canonical.sh allowlisted this sequence.
ALLOW = ["0123456789abcdefghij", "/home/other/"]
def allowed(m): return any(a in m for a in ALLOW)
deny = set()
try:
    for ln in open(".vjs/publication-denylist.txt"):
        ln = ln.strip()
        if ln and not ln.startswith("#"): deny.add(ln)
except FileNotFoundError: pass
tok = ""
for ch in text + "\n":
    if ch.isalnum() or ch == "-": tok += ch
    else:
        if len(tok) >= 3 and hashlib.sha256(tok.lower().encode()).hexdigest() in deny:
            print(f"FAIL: denylisted private term (hash match) {scope}"); fail = True
        tok = ""
for pat,label in [(r"sk-[A-Za-z0-9]{48}","openai key"),(r"gh[pousr]_[A-Za-z0-9_]{36,}","github token"),
                  (r"AKIA[0-9A-Z]{16}","aws key"),(r"-----BEGIN (RSA |EC |OPENSSH )?PRIVATE KEY","private key"),
                  (r"/home/[a-z][a-z0-9_-]+/","dev-machine home path")]:
    for m in re.finditer(pat, text):
        if allowed(m.group(0)): continue
        print(f"FAIL: {label} {scope}: {m.group(0)[:24]}"); fail = True
sys.exit(1 if fail else 0)
PY
rc=$?; rm -f "$TMP"
[ $rc -eq 0 ] && echo "boundary-scan: clean" || echo "boundary-scan: BLOCKED"
exit $rc
