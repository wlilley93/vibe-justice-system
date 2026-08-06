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
#
# EVERY FINDING NAMES ITS FILE AND LINE, as of 2026-08-06. It did not before, and
# that made the gate unusable for the one job it exists for. The tree scan ran
# `git grep -h`, which SUPPRESSES filenames, so the whole repository arrived as one
# concatenated blob and all 610 standing findings printed the identical string "in
# the HEAD tree". A release blocker you cannot locate is a release blocker you
# cannot clear: the number could only ever go up or down, never be worked. The
# scan was accurate and useless at the same time.
#
# The finding still NEVER prints the term. A denylisted term is denylisted because
# writing it down publishes it, and this output lands in CI logs and commit
# records. `file:line` is the whole disclosure, which is enough to fix it and not
# enough to leak it.
set -euo pipefail
cd "$(git rev-parse --show-toplevel)"
RANGE="${1:-}"
EXCL=(":(exclude)*/vendor/*" ":(exclude)*.min.*")
TMP=$(mktemp)
# SCOPE names what was actually scanned. A no-argument run reads the whole HEAD
# tree, so reporting those hits as "added content" made a STANDING tree condition
# read as a fresh introduction on every push.
if [ -n "$RANGE" ]; then
  # `--unified=0` so every kept line sits in a hunk whose header gives its new-file
  # line number; the parser below reconstructs file:line from `+++ b/<path>` and
  # `@@ ... +start,count @@`, which is the only way a diff can address a finding.
  git diff --unified=0 "$RANGE" -- . "${EXCL[@]}" > "$TMP" || true
  SCOPE="in added content"
else
  # -n gives the line number; dropping -h keeps the filename. Both are load-bearing.
  git grep -I --no-color -n "" -- . "${EXCL[@]}" > "$TMP" 2>/dev/null || true
  SCOPE="in the HEAD tree"
fi
python3 - "$TMP" "$SCOPE" "$RANGE" <<'PY'
import hashlib, re, sys
scope = sys.argv[2]
is_range = bool(sys.argv[3])
fail = False
# Synthetic fixtures that legitimately carry secret/path SHAPES (they test the
# scanner itself). Precedent: promote-canonical.sh allowlisted this sequence.
ALLOW = ["0123456789abcdefghij", "/home/other/"]
def allowed(m): return any(a in m for a in ALLOW)
sys.path.insert(0, "scripts/lib")
import denylist
deny = denylist.load()

def rows(path, is_range):
    """Yield (file, lineno, content) for every scannable line.

    The content is yielded WITHOUT its address prefix. Scanning the prefix too
    would let a path spell a denylisted term into every finding under it, turning
    one badly-named file into hundreds of phantom hits."""
    if not is_range:
        for raw in open(path, encoding="utf-8", errors="replace"):
            # git grep -n emits `path:lineno:content`; a path may contain ':'.
            m = re.match(r"^(.*?):(\d+):(.*)$", raw.rstrip("\n"))
            if m:
                yield m.group(1), int(m.group(2)), m.group(3)
        return
    cur, lineno = "?", 0
    for raw in open(path, encoding="utf-8", errors="replace"):
        raw = raw.rstrip("\n")
        if raw.startswith("+++ "):
            cur = raw[4:]
            cur = cur[2:] if cur.startswith("b/") else cur
        elif raw.startswith("@@"):
            m = re.search(r"\+(\d+)", raw)
            lineno = int(m.group(1)) if m else 0
        elif raw.startswith("+"):
            yield cur, lineno, raw[1:]
            lineno += 1

PATTERNS = [(r"sk-[A-Za-z0-9]{48}", "openai key"),
            (r"gh[pousr]_[A-Za-z0-9_]{36,}", "github token"),
            (r"AKIA[0-9A-Z]{16}", "aws key"),
            (r"-----BEGIN (RSA |EC |OPENSSH )?PRIVATE KEY", "private key"),
            (r"/home/[a-z][a-z0-9_-]+/", "dev-machine home path")]

# Deduplicate by (file, line, label): one line carrying a term three times is one
# place to go and fix, and a 610-line report whose length is driven by repetition
# hides how much work there actually is.
seen = set()
def report(where, label, detail=""):
    global fail
    key = (where, label)
    if key in seen:
        return
    seen.add(key)
    print(f"FAIL: {label} {scope}: {where}{detail}")
    fail = True

for fname, lineno, content in rows(sys.argv[1], is_range):
    where = f"{fname}:{lineno}"
    tok = ""
    for ch in content + "\n":
        if ch.isalnum() or ch == "-":
            tok += ch
        else:
            if len(tok) >= 3 and hashlib.sha256(tok.lower().encode()).hexdigest() in deny:
                # The term itself is NEVER printed. See the header.
                report(where, "denylisted private term (hash match)")
            tok = ""
    for pat, label in PATTERNS:
        for m in re.finditer(pat, content):
            if allowed(m.group(0)):
                continue
            report(where, label, f" -> {m.group(0)[:24]}")

if fail:
    files = sorted({w.rsplit(":", 1)[0] for w, _ in seen})
    print(f"\n{len(seen)} finding(s) across {len(files)} file(s).", file=sys.stderr)
sys.exit(1 if fail else 0)
PY
rc=$?; rm -f "$TMP"
[ $rc -eq 0 ] && echo "boundary-scan: clean" || echo "boundary-scan: BLOCKED"
exit $rc
