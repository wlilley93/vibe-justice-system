#!/usr/bin/env bash
# vjs-pre-push.sh - deterministic checkpoint gate for irreversible outward pushes.
#
# The pre-commit hook protects the legal record before it is committed. This hook protects the
# outward act before it leaves the machine. In particular, a push to the public
# wlilley93/vibe-justice-system repository is an irreversible outward act (Bill 18 s. 2(e), s. 7;
# Bill 27 s. 6(3)). It therefore fails closed unless the matter records express authorisation.
#
# Private/dev pushes are allowed. So are forks and locally independent VJS jurisdictions. Only
# the exact canonical public VJS remote is the public VJS publication checkpoint.
#
# Accepted public-publish authorisation records:
#   1. Tracked system record:
#      Judicature/ministry-of-justice/reasons-ledger/outward-act-authorisations/public-vjs-publish.md
#   2. Local operational checkpoint (may stay untracked):
#      .vjs/checkpoints/public-vjs-publish-authorisation.env
#
# Either record must contain:
#   AUTHORISED_OUTWARD_ACT=public-vjs-publish
#   AUTHORISED_BY=<human/founder name or office>
#   AUTHORISED_AT=<timestamp>
#
# Optional scoping fields:
#   AUTHORISED_REMOTE_URL=<exact remote URL>
#   AUTHORISED_REMOTE_REF=<exact remote ref, e.g. refs/heads/main>
#   AUTHORISED_LOCAL_SHA=<exact local sha being pushed>
#
# If an optional field is present it must match the attempted push.
#
# Agents can inspect the same authority evidence before pushing with:
#   cdd release-warrant --remote-url <url> --remote-ref <ref> --local-sha <sha>
set -euo pipefail

ROOT="$(git rev-parse --show-toplevel 2>/dev/null || pwd)"
cd "$ROOT"

REMOTE_NAME="${1:-}"
REMOTE_URL="${2:-}"

is_public_vjs_remote() {
  case "$REMOTE_URL" in
    https://github.com/wlilley93/vibe-justice-system|\
https://github.com/wlilley93/vibe-justice-system.git|\
git@github.com:wlilley93/vibe-justice-system.git|\
ssh://git@github.com/wlilley93/vibe-justice-system.git) return 0 ;;
    *) return 1 ;;
  esac
}

run_local_ci() {
  if command -v cdd >/dev/null 2>&1; then cdd local-ci; return $?; fi
  if command -v vjs >/dev/null 2>&1; then vjs local-ci; return $?; fi
  if [ -f Executive/cli/bin/cdd.js ] && command -v node >/dev/null 2>&1; then
    node Executive/cli/bin/cdd.js local-ci
    return $?
  fi
  if [ -f cli/bin/cdd.js ] && command -v node >/dev/null 2>&1; then
    node cli/bin/cdd.js local-ci
    return $?
  fi
  echo "VJS pre-push: cdd CLI not found; cannot run local CI for public VJS push." >&2
  return 1
}

require_field() {
  local file="$1"
  local key="$2"
  if ! grep -Eq "^${key}=.+" "$file" && ! grep -Eq "^${key}:.+" "$file"; then
    echo "VJS pre-push: authorisation record missing ${key}: $file" >&2
    return 1
  fi
}

field_value() {
  local file="$1"
  local key="$2"
  awk -v key="$key" '
    index($0, key "=") == 1 {
      value = substr($0, length(key) + 2)
      sub(/^[[:space:]]+/, "", value)
      sub(/[[:space:]]+$/, "", value)
      print value
      exit
    }
    index($0, key ":") == 1 {
      value = substr($0, length(key) + 2)
      sub(/^[[:space:]]+/, "", value)
      sub(/[[:space:]]+$/, "", value)
      print value
      exit
    }
  ' "$file"
}

record_authorises_push() {
  local file="$1"
  local local_sha="$2"
  local remote_ref="$3"

  [ -f "$file" ] || return 1
  require_field "$file" "AUTHORISED_OUTWARD_ACT" || return 1
  require_field "$file" "AUTHORISED_BY" || return 1
  require_field "$file" "AUTHORISED_AT" || return 1

  local act
  act="$(field_value "$file" "AUTHORISED_OUTWARD_ACT")"
  [ "$act" = "public-vjs-publish" ] || {
    echo "VJS pre-push: authorisation record has wrong act '${act}' in $file" >&2
    return 1
  }

  local scoped_remote scoped_ref scoped_sha
  scoped_remote="$(field_value "$file" "AUTHORISED_REMOTE_URL" || true)"
  scoped_ref="$(field_value "$file" "AUTHORISED_REMOTE_REF" || true)"
  scoped_sha="$(field_value "$file" "AUTHORISED_LOCAL_SHA" || true)"

  if [ -n "$scoped_remote" ] && [ "$scoped_remote" != "$REMOTE_URL" ]; then
    echo "VJS pre-push: authorisation remote mismatch: expected $scoped_remote, got $REMOTE_URL" >&2
    return 1
  fi
  if [ -n "$scoped_ref" ] && [ "$scoped_ref" != "$remote_ref" ]; then
    echo "VJS pre-push: authorisation ref mismatch: expected $scoped_ref, got $remote_ref" >&2
    return 1
  fi
  if [ -n "$scoped_sha" ] && [ "$scoped_sha" != "$local_sha" ]; then
    echo "VJS pre-push: authorisation sha mismatch: expected $scoped_sha, got $local_sha" >&2
    return 1
  fi

  echo "VJS pre-push: public VJS checkpoint authorisation found in $file" >&2
  return 0
}

if ! is_public_vjs_remote; then
  echo "VJS pre-push: non-canonical/dev remote '${REMOTE_NAME:-?}' allowed ($REMOTE_URL)." >&2
  exit 0
fi

if ! run_local_ci; then
  echo "" >&2
  echo "VJS pre-push: BLOCKED public VJS push because local CI failed." >&2
  echo "This repository does not rely on hosted CI for the public checkpoint." >&2
  exit 1
fi

AUTH_RECORDS=(
  "Judicature/ministry-of-justice/reasons-ledger/outward-act-authorisations/public-vjs-publish.md"
  ".vjs/checkpoints/public-vjs-publish-authorisation.env"
)

blocked=0
while read -r local_ref local_sha remote_ref remote_sha; do
  # Deleted refs do not publish new public VJS content; leave destructive remote deletion to host perms.
  [ "$local_sha" = "0000000000000000000000000000000000000000" ] && continue

  ok=1
  for record in "${AUTH_RECORDS[@]}"; do
    if record_authorises_push "$record" "$local_sha" "$remote_ref"; then
      ok=0
      break
    fi
  done

  if [ "$ok" -ne 0 ]; then
    blocked=1
    echo "" >&2
    echo "VJS pre-push: BLOCKED public VJS push." >&2
    echo "Remote: $REMOTE_NAME $REMOTE_URL" >&2
    echo "Ref:    $local_ref ($local_sha) -> $remote_ref" >&2
    echo "" >&2
    echo "This is an irreversible outward act. Record the Founder checkpoint first, then retry." >&2
    echo "Inspect the available licence/warrant evidence with:" >&2
    echo "  cdd release-warrant --remote-url '$REMOTE_URL' --remote-ref '$remote_ref' --local-sha '$local_sha'" >&2
    echo "Accepted records:" >&2
    for record in "${AUTH_RECORDS[@]}"; do echo "  - $record" >&2; done
  fi
done

exit "$blocked"
