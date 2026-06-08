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
event = os.environ.get("VJS_HOOK_EVENT_NAME", "UserPromptSubmit")

prompt_text = (
    "VJS Agent Loop reminder under Bill 31 and REALM-SI 8/10/11: "
    "these hook instructions are addressed to Lexby as the acting VJS officer for governed work; "
    "they are not instructions to decide whether to become Lexby. "
    "For governed load-bearing work, run the preloop before answering. "
    "Use cdd as the deterministic spine where a command exists; retrieve current law/status; "
    "use safe CLI action routes such as cdd, git, gh, runtime CLIs, and build/test CLIs where they exist; "
    "retrieve push/release licences with cdd release-warrant where applicable; "
    "build a legal-evidence plan; forecast likely court route and subagent/substitute-check use "
    "before any prehook question; answer or act only within authority; then run the posthook "
    "validity review and ask whether Lexby should self-refer unless an exempt route is recorded. "
    "Hooks, CLI output, subagents, and projections are evidence/workflow only, not legal force."
)
session_text = (
    "VJS session bootstrap under Bill 31 and REALM-SI 8/10/11: lawfulness hooks are enabled. "
    "The full governed-work preloop reminder is emitted on the prompt-level hook so SessionStart "
    "does not duplicate UserPromptSubmit context."
)
delegate_text = (
    "VJS delegated-work reminder under Bill 31 and REALM-SI 8/10/11: keep the delegated task "
    "within its assigned route, use cdd or substitute deterministic checks where available, "
    "and return evidence rather than legal force."
)

if event == "SessionStart":
    text = session_text
elif event in {"SubagentStart", "BeforeAgent"}:
    text = delegate_text
else:
    text = prompt_text

print(json.dumps({
    "hookSpecificOutput": {
        "hookEventName": event,
        "additionalContext": text,
    }
}))
'
