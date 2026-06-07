#!/usr/bin/env bash
# vjs-pre-answer.sh - VJS pre-answer lawfulness hook binding stub.
#
# The contract-level event is "before a governed load-bearing answer or act". Claude Code maps
# that to UserPromptSubmit: after the user submits a prompt, before Claude processes it. Other
# adapters may bind this same script at their nearest equivalent point. This stub keeps that
# binding visible and safe while REALM-SI 8 awaits commencement.
#
# Default: no-op, fail-open. Set VJS_PRE_ANSWER_REMINDER=on to inject a short advisory context
# reminder in VJS jurisdictions.
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
[ "${VJS_PRE_ANSWER_REMINDER:-off}" = "on" ] || exit 0

python3 -c '
import json, os
print(json.dumps({
    "hookSpecificOutput": {
        "hookEventName": os.environ.get("VJS_HOOK_EVENT_NAME", "UserPromptSubmit"),
        "additionalContext": "VJS pre-answer lawfulness hook is installed under REALM-SI 8. For governed, load-bearing work, retrieve applicable law, separate delegable review where useful, and route before answering; this advisory stub does not adjudicate.",
    }
}))
'
