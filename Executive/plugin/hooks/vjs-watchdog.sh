#!/usr/bin/env bash
# vjs-watchdog.sh - the VJS turn watchdog. A token-light post-answer hook.
#
# Lexby is meant to catch himself: self-file a breach, convene on a real fork, seek
# permission to appeal. But an agent's job is to produce value the way it sees best,
# not to hold the whole statute book in its head every turn. This hook is the backstop
# for the turns where Lexby was heads-down and missed one.
#
# Every turn, it asks the active runtime CLI THREE yes/no questions about the
# agent's last turn (and only the last turn - that is what keeps it token-light):
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
# In the bundled adapters this is wired as a Stop or nearest post-answer hook. It uses the CLI
# for the runtime that fired the hook, such as codex, opencode, gemini, or claude. Where the
# runtime exposes a named agent/subagent selector, the hook uses it: Claude gets an inline
# vjs-watchdog agent by default, and opencode can use VJS_WATCHDOG_AGENT when configured. Where the
# runtime exposes structured output, the hook captures JSON/JSONL and schema-constrains the final
# verdict. It never calls a vendor API endpoint directly and never requires a vendor API key env
# var in this script. Inert by design unless a root .justice/ or Judicature/.justice/ exists and a
# runtime CLI is available. It never blocks a non-VJS repo and never blocks if it cannot reach the
# CLI - it fails OPEN, because a watchdog that wedges your session is itself a breach of the duty
# of care.
#
# Env:
#   VJS_WATCHDOG=off             disable entirely
#   VJS_WATCHDOG_MODE=warn       surface the finding but do not block the turn (default: block)
#   VJS_AGENT_RUNTIME=codex      adapter-provided runtime: codex, opencode, gemini, or claude
#   VJS_WATCHDOG_RUNTIME=codex   override runtime detection
#   VJS_WATCHDOG_AGENT=reviewer  optional runtime agent/subagent selector where supported
#   VJS_WATCHDOG_MODEL=...       optional runtime-specific model override
#   VJS_WATCHDOG_MAXCHARS=N      cap on last-turn chars sent to the model (default: 6000)
#   VJS_WATCHDOG_TIMEOUT=N       CLI timeout seconds (default: 45)
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
[ -n "${TRANSCRIPT:-}" ] && [ -f "$TRANSCRIPT" ] || exit 0
command -v python3 >/dev/null 2>&1 || exit 0

MODE="${VJS_WATCHDOG_MODE:-block}"
MAXCHARS="${VJS_WATCHDOG_MAXCHARS:-6000}"
WATCHDOG_TIMEOUT="${VJS_WATCHDOG_TIMEOUT:-45}"

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
PROMPT_FILE="$(mktemp "${TMPDIR:-/tmp}/vjs-watchdog-prompt.XXXXXX")" || exit 0
OUTPUT_FILE="$(mktemp "${TMPDIR:-/tmp}/vjs-watchdog-output.XXXXXX")" || exit 0
JSONL_FILE="$(mktemp "${TMPDIR:-/tmp}/vjs-watchdog-events.XXXXXX")" || exit 0
SCHEMA_FILE="$(mktemp "${TMPDIR:-/tmp}/vjs-watchdog-schema.XXXXXX")" || exit 0
chmod 600 "$PROMPT_FILE" "$OUTPUT_FILE" "$JSONL_FILE" "$SCHEMA_FILE" 2>/dev/null || true
trap 'rm -f "$PROMPT_FILE" "$OUTPUT_FILE" "$JSONL_FILE" "$SCHEMA_FILE"' EXIT

SCHEMA_JSON='{"type":"object","additionalProperties":false,"properties":{"breach":{"type":"boolean"},"fork":{"type":"boolean"},"appeal":{"type":"boolean"},"why":{"type":"string","maxLength":180}},"required":["breach","fork","appeal","why"]}'
CLAUDE_AGENTS_JSON='{"vjs-watchdog":{"description":"Reviews one VJS turn for breach, fork, and appeal backstops.","prompt":"You are the VJS watchdog reviewer. Review only the supplied turn. Return only the requested schema JSON."}}'
printf '%s\n' "$SCHEMA_JSON" > "$SCHEMA_FILE"

{
  printf '%s\n\n' "$SYSTEM"
  printf 'AGENT TURN UNDER REVIEW:\n\n%s\n' "$LAST_TURN"
} > "$PROMPT_FILE"

detect_runtime() {
  local configured="${VJS_WATCHDOG_RUNTIME:-${VJS_AGENT_RUNTIME:-auto}}"
  case "$configured" in
    codex|opencode|gemini|claude) printf '%s\n' "$configured"; return 0 ;;
    auto|"") ;;
    *) return 1 ;;
  esac
  for candidate in codex opencode gemini claude; do
    if command -v "$candidate" >/dev/null 2>&1; then
      printf '%s\n' "$candidate"
      return 0
    fi
  done
  return 1
}

run_cli() {
  if command -v timeout >/dev/null 2>&1; then
    VJS_WATCHDOG=off VJS_LAWFULNESS_HOOKS=off timeout "$WATCHDOG_TIMEOUT" "$@"
  else
    VJS_WATCHDOG=off VJS_LAWFULNESS_HOOKS=off "$@"
  fi
}

run_reviewer() {
  local runtime="$1"
  local prompt_text
  prompt_text="$(cat "$PROMPT_FILE")" || return 1
  case "$runtime" in
    codex)
      command -v codex >/dev/null 2>&1 || return 1
      if [ -n "${VJS_WATCHDOG_MODEL:-}" ]; then
        run_cli codex exec -m "$VJS_WATCHDOG_MODEL" -C "$CWD" -s read-only --ephemeral --json --output-schema "$SCHEMA_FILE" -o "$OUTPUT_FILE" - < "$PROMPT_FILE" > "$JSONL_FILE"
      else
        run_cli codex exec -C "$CWD" -s read-only --ephemeral --json --output-schema "$SCHEMA_FILE" -o "$OUTPUT_FILE" - < "$PROMPT_FILE" > "$JSONL_FILE"
      fi
      cat "$OUTPUT_FILE" "$JSONL_FILE" 2>/dev/null
      ;;
    opencode)
      command -v opencode >/dev/null 2>&1 || return 1
      local opencode_args=(opencode run --dir "$CWD" --format json)
      if [ -n "${VJS_WATCHDOG_AGENT:-}" ]; then
        opencode_args+=(--agent "$VJS_WATCHDOG_AGENT")
      fi
      if [ -n "${VJS_WATCHDOG_MODEL:-}" ]; then
        opencode_args+=(-m "$VJS_WATCHDOG_MODEL")
      fi
      run_cli "${opencode_args[@]}" "$prompt_text" > "$JSONL_FILE"
      cat "$JSONL_FILE" 2>/dev/null
      ;;
    gemini)
      command -v gemini >/dev/null 2>&1 || return 1
      if [ -n "${VJS_WATCHDOG_MODEL:-}" ]; then
        run_cli gemini -m "$VJS_WATCHDOG_MODEL" -p "$prompt_text"
      else
        run_cli gemini -p "$prompt_text"
      fi
      ;;
    claude)
      command -v claude >/dev/null 2>&1 || return 1
      local claude_args=(claude -p --no-session-persistence --permission-mode plan --max-turns 1 --tools "" --output-format stream-json --json-schema "$SCHEMA_JSON")
      if [ -n "${VJS_WATCHDOG_AGENT:-}" ]; then
        claude_args+=(--agent "$VJS_WATCHDOG_AGENT")
      else
        claude_args+=(--agents "$CLAUDE_AGENTS_JSON" --agent vjs-watchdog)
      fi
      if [ -n "${VJS_WATCHDOG_MODEL:-}" ]; then
        claude_args+=(--model "$VJS_WATCHDOG_MODEL")
      fi
      run_cli "${claude_args[@]}" "$prompt_text" > "$JSONL_FILE"
      cat "$JSONL_FILE" 2>/dev/null
      ;;
    *) return 1 ;;
  esac
}

RUNTIME="$(detect_runtime)" || exit 0
RESPONSE="$(run_reviewer "$RUNTIME" 2>/dev/null)" || exit 0
[ -n "$RESPONSE" ] || exit 0

VERDICT="$(python3 -c '
import json
import sys

text = sys.stdin.read()


def truthy(value):
    if value is True:
        return True
    if isinstance(value, str):
        return value.strip().lower() == "true"
    return False


def verdict_from_obj(obj):
    if not isinstance(obj, dict):
        return None
    if not any(k in obj for k in ("breach", "fork", "appeal")):
        return None
    flags = [k for k in ("breach", "fork", "appeal") if truthy(obj.get(k))]
    if not flags:
        return "CLEAR"
    why = str(obj.get("why", "")).strip().replace("\n", " ")[:300]
    return "FLAG|" + ",".join(flags) + "|" + why


def balanced_json_objects(raw):
    stack = []
    start = None
    in_string = False
    escape = False
    spans = []
    for idx, ch in enumerate(raw):
        if in_string:
            if escape:
                escape = False
            elif ch == "\\":
                escape = True
            elif ch == "\"":
                in_string = False
            continue
        if ch == "\"":
            in_string = True
        elif ch == "{":
            if not stack:
                start = idx
            stack.append(ch)
        elif ch == "}" and stack:
            stack.pop()
            if not stack and start is not None:
                spans.append((start, idx + 1))
                start = None
    for start, end in reversed(spans):
        candidate = raw[start:end]
        try:
            yield json.loads(candidate)
        except Exception:
            continue


def strings_from_obj(obj):
    if isinstance(obj, str):
        yield obj
    elif isinstance(obj, list):
        for item in obj:
            yield from strings_from_obj(item)
    elif isinstance(obj, dict):
        preferred = ("result", "output", "text", "delta", "content", "message", "data")
        seen = set()
        for key in preferred:
            if key in obj:
                seen.add(key)
                yield from strings_from_obj(obj[key])
        for key, value in obj.items():
            if key not in seen:
                yield from strings_from_obj(value)


def parse_candidate(raw):
    raw = str(raw).strip()
    if not raw:
        return None
    try:
        obj = json.loads(raw)
    except Exception:
        obj = None
    if obj is not None:
        verdict = verdict_from_obj(obj)
        if verdict:
            return verdict
        for nested in reversed(list(strings_from_obj(obj))):
            verdict = parse_candidate(nested)
            if verdict:
                return verdict
    for obj in balanced_json_objects(raw):
        verdict = verdict_from_obj(obj)
        if verdict:
            return verdict
    return None


lines = [line for line in text.splitlines() if line.strip()]
for line in reversed(lines):
    verdict = parse_candidate(line)
    if verdict:
        print(verdict)
        sys.exit(0)

verdict = parse_candidate(text)
print(verdict or "CLEAR")
' <<< "$RESPONSE" 2>/dev/null || echo CLEAR)"

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
