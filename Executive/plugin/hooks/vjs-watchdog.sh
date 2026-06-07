#!/usr/bin/env bash
# vjs-watchdog.sh - the VJS turn watchdog. A token-light post-answer hook.
#
# Lexby is meant to catch himself: self-file a breach, convene on a real fork, seek
# permission to appeal. But an agent's job is to produce value the way it sees best,
# not to hold the whole statute book in its head every turn. This hook is the backstop
# for the turns where Lexby was heads-down and missed one.
#
# Every turn, it asks one small Haiku call THREE yes/no questions about the agent's
# last turn (and only the last turn - that is what keeps it token-light):
#
#   1. BREACH  - did the work fall below the duty of reasonable skill and care (s. 4/s. 5)
#                without being self-reported to court?
#   2. FORK    - did a load-bearing decision hit a convening trigger (first-impression,
#                genuine distinction, overruling, or principal-vs-law conflict) but get
#                neither disposed on citation nor sent to the court?
#   3. APPEAL  - is there an arguable ground to appeal an existing ruling, or to take a
#                point up to the Supreme Court (arguable point of law / binding-precedent
#                conflict / constitutional-or-foundational question)?
#
# If any answer is yes, the hook hands Lexby the reason and (in block mode) refuses to let
# the turn end until he disposes of it by the law: file the breach, convene, or seek leave.
#
# In the bundled Claude adapter this is wired as a Stop hook. Other adapters may bind it at their
# nearest post-answer / post-act event. Inert by design unless ALL of: root .justice/ or
# Judicature/.justice/ exists (VJS is installed in this repo) AND ANTHROPIC_API_KEY is set. It
# never blocks a non-VJS repo and never blocks if it cannot reach the model - it fails OPEN,
# because a watchdog that wedges your session is itself a breach of the duty of care.
#
# Env:
#   VJS_WATCHDOG=off          disable entirely
#   VJS_WATCHDOG_MODE=warn    surface the finding but do not block the turn (default: block)
#   VJS_WATCHDOG_MODEL=...     override the model (default: claude-haiku-4-5)
#   VJS_WATCHDOG_MAXCHARS=N    cap on last-turn chars sent to the model (default: 6000)
set -euo pipefail

INPUT="$(cat)"

eval "$(printf '%s' "$INPUT" | python3 -c '
import sys, json, shlex
d = json.load(sys.stdin)
def emit(k, v): print(f"{k}={shlex.quote(str(v))}")
emit("STOP_ACTIVE", d.get("stop_hook_active", False))
emit("TRANSCRIPT", d.get("transcript_path", ""))
emit("CWD", d.get("cwd", "."))
' 2>/dev/null || true)"

# Guards. Any failure here = fail open (exit 0): a watchdog must never wedge the session.
[ "${STOP_ACTIVE:-False}" = "True" ] && exit 0       # already a stop-hook continuation; do not re-fire (loop guard)
[ "${VJS_WATCHDOG:-on}" = "off" ] && exit 0
cd "${CWD:-.}" 2>/dev/null || exit 0
if [ -d .justice ]; then
  VJS_JUSTICE_DIR=".justice"
elif [ -d Judicature/.justice ]; then
  VJS_JUSTICE_DIR="Judicature/.justice"
else
  exit 0                                              # not a VJS jurisdiction
fi
[ -n "${ANTHROPIC_API_KEY:-}" ] || exit 0            # advisory feature; silent without a key
[ -n "${TRANSCRIPT:-}" ] && [ -f "$TRANSCRIPT" ] || exit 0
command -v curl >/dev/null 2>&1 || exit 0
command -v python3 >/dev/null 2>&1 || exit 0

MODE="${VJS_WATCHDOG_MODE:-block}"
MODEL="${VJS_WATCHDOG_MODEL:-claude-haiku-4-5}"
MAXCHARS="${VJS_WATCHDOG_MAXCHARS:-6000}"

# --- Extract ONLY the last assistant turn's text, tail-capped. This is the whole point of
#     "token-light": we never send the transcript, just the turn under review. ---
LAST_TURN="$(python3 - "$TRANSCRIPT" "$MAXCHARS" <<'PY'
import sys, json
path, maxchars = sys.argv[1], int(sys.argv[2])
texts = []
try:
    with open(path) as f:
        for line in f:
            line = line.strip()
            if not line:
                continue
            try:
                rec = json.loads(line)
            except Exception:
                continue
            if rec.get("type") != "assistant":
                continue
            parts = [b.get("text", "") for b in rec.get("message", {}).get("content", [])
                     if isinstance(b, dict) and b.get("type") == "text"]
            if parts:
                texts = parts          # keep only the most recent assistant turn
except Exception:
    pass
print(("\n".join(texts)).strip()[-maxchars:])
PY
)"

[ -n "$LAST_TURN" ] || exit 0

SYSTEM='You are the VJS turn watchdog. The agent (Lexby) runs under the Vibe Justice System: a continuing duty of reasonable skill and care (breach is tortious, remedy is to make the work good, never punishment). You review ONLY the agent turn given to you and answer three yes/no questions. Be conservative: answer true only on a clear, particularised case, never on a hunch, because every true forces extra work. The five convening triggers (a real fork needs one): (1) first-impression question with no governing ratio; (2) a genuine distinction from existing precedent; (3) a precedent that is wrong/outdated and should be overruled; (4) a principal instruction that conflicts with enacted law or precedent; (5) a discovered breach. Questions: BREACH = did this turn fall below the duty of care (e.g. shipped something known-unsound, misrepresented delivered scope, ignored a known material risk) AND not self-report it to court? FORK = did this turn make a load-bearing decision matching trigger 1-4 that was neither disposed on an existing citation nor sent to the court? APPEAL = does this turn reveal an arguable ground to appeal an existing ruling or take a point to the Supreme Court (arguable point of law, binding-precedent conflict, or a constitutional/foundational question)? Reply with ONLY a compact JSON object, no prose: {"breach":bool,"fork":bool,"appeal":bool,"why":"<=25 words, empty if all false"}.'

export LAST_TURN
python3 - "$MODEL" "$SYSTEM" <<'PY' >/dev/null 2>&1 || exit 0
import sys, json, os
model, system = sys.argv[1], sys.argv[2]
last_turn = os.environ.get("LAST_TURN", "")
payload = {
    "model": model,
    "max_tokens": 200,
    "system": system,
    "messages": [{"role": "user", "content": "AGENT TURN UNDER REVIEW:\n\n" + last_turn}],
}
json.dump(payload, open("/tmp/vjs-watchdog-payload.json", "w"))
PY

RESPONSE="$(curl -s --max-time 30 -X POST https://api.anthropic.com/v1/messages \
  -H "x-api-key: $ANTHROPIC_API_KEY" \
  -H "anthropic-version: 2023-06-01" \
  -H "content-type: application/json" \
  -d @/tmp/vjs-watchdog-payload.json 2>/dev/null)" || exit 0

VERDICT="$(printf '%s' "$RESPONSE" | python3 -c '
import sys, json
try:
    data = json.load(sys.stdin)
    text = ""
    for b in data.get("content", []):
        if b.get("type") == "text":
            text = b["text"]; break
    s, e = text.find("{"), text.rfind("}")
    obj = json.loads(text[s:e+1]) if s != -1 and e != -1 else {}
    flags = [k for k in ("breach","fork","appeal") if bool(obj.get(k))]
    if not flags:
        print("CLEAR")
    else:
        why = str(obj.get("why","")).strip()
        print("FLAG|" + ",".join(flags) + "|" + why)
except Exception:
    print("CLEAR")
' 2>/dev/null || echo CLEAR)"

[ "${VERDICT%%|*}" = "FLAG" ] || exit 0

FLAGS="$(printf '%s' "$VERDICT" | cut -d'|' -f2)"
WHY="$(printf '%s' "$VERDICT" | cut -d'|' -f3-)"

MSG="VJS watchdog: this turn may need the court before it closes.
Flagged: ${FLAGS}. Reason: ${WHY}
Dispose of it by the law, do not work around it:
  - breach  -> submit-breach-to-court \"<what fell below standard and why>\"
  - fork    -> check ${VJS_JUSTICE_DIR}/INDEX.md first; cite if covered, else submit-request-to-court \"<the fork>\"
  - appeal  -> seek permission to appeal (arguable point of law / binding-precedent conflict)
If on a fair view none applies, say so in one line and proceed (the watchdog is conservative, not infallible)."

if [ "$MODE" = "warn" ]; then
  printf '%s\n' "$MSG" >&2
  exit 0
fi

# block mode: exit 2 hands stderr back to the agent and refuses to end the turn.
printf '%s\n' "$MSG" >&2
exit 2
