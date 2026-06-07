#!/usr/bin/env bash
# vjs-post-answer.sh - VJS post-answer validity hook binding stub.
#
# The contract-level event is "after a governed load-bearing answer or act". Claude Code maps that
# to Stop: when Claude finishes responding. Other adapters may bind this same script at their
# nearest equivalent point. The active post-answer reviewer remains vjs-watchdog.sh; this stub is
# intentionally non-blocking so it cannot interfere with the watchdog or create a legal consequence
# while REALM-SI 8 awaits commencement.
set -euo pipefail

INPUT="$(cat)"

eval "$(printf '%s' "$INPUT" | python3 -c '
import sys, json, shlex
d = json.load(sys.stdin)
print("CWD=" + shlex.quote(str(d.get("cwd", "."))))
' 2>/dev/null || true)"

cd "${CWD:-.}" 2>/dev/null || exit 0
[ "${VJS_LAWFULNESS_HOOKS:-on}" = "off" ] && exit 0
[ -d .justice ] || [ -d Judicature/.justice ] || exit 0

# Reserved for a deterministic post-answer validity review. The watchdog is the current
# non-adjudicating Stop-hook review and may block independently.
exit 0
