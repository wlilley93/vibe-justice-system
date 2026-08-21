#!/usr/bin/env bash
#
# install-resource-guards.sh - put the box-survival pair into a host repo.
#
#   scripts/install-resource-guards.sh /path/to/host-repo
#
# THE PAIR
#   heavy-job.sh       prevention: one machine-wide lock + a MemAvailable floor,
#                      in front of every memory-hungry job on the box.
#   session-restore.sh recovery:   find Claude sessions that DIED rather than
#                      ended, and bring them back into free tmux slots.
#
# Both come from the same incident. On 2026-08-19 a `next build --webpack` reached
# 9.3GB beside a pre-push gate at 1.9GB on an 18GB box; the kernel OOM-killer fired,
# systemd tore down the tmux-spawn scope it died in, and two Claude sessions went
# with the panes. Prevention alone was not enough of an answer, because the chats
# were recoverable the whole time and nobody could tell.
#
# WHAT THIS INSTALLER WILL NOT DO, AND WHY
# It copies the scripts and then STOPS. It does not edit the host's pre-push, and
# it does not touch core.hooksPath. That restraint is inherited from this repo's
# own install-boundary-hooks.sh, which spent weeks installing a gate into
# .git/hooks that git had been ignoring the entire time because core.hooksPath was
# set ([2026] VJS-CC-VJS 20 D21). The lesson recorded there was not "fix the path" -
# it was that an installer which cannot verify the gate runs must not imply it does.
#
# So this prints the exact line to add and leaves the adding to a human, who can
# see the surrounding hook. An unrun gate banks debt; an unrun gate that reported
# success banks debt AND spends the trust that would have found it.
#
set -euo pipefail

HOST="${1:-}"
if [ -z "$HOST" ]; then
  echo "usage: scripts/install-resource-guards.sh /path/to/host-repo" >&2
  exit 2
fi
[ -d "$HOST" ] || { echo "install-resource-guards: no such directory: $HOST" >&2; exit 1; }

HOST_ROOT="$(cd "$HOST" && git rev-parse --show-toplevel 2>/dev/null || true)"
[ -n "$HOST_ROOT" ] || { echo "install-resource-guards: $HOST is not a git repository." >&2; exit 1; }

SRC="$(cd "$(dirname "$0")" && pwd)"
DEST="$HOST_ROOT/.vjs/bin"
mkdir -p "$DEST"

for f in heavy-job.sh session-restore.sh; do
  [ -f "$SRC/$f" ] || { echo "install-resource-guards: missing $SRC/$f" >&2; exit 1; }
  install -m 0755 "$SRC/$f" "$DEST/$f"
  echo "installed  $DEST/$f"
done

# DOES VERSION CONTROL ACTUALLY SEE WHAT WE JUST INSTALLED?
# Found the hard way on the first real run: opbox's .gitignore carries a bare
# `bin/`, which matches .vjs/bin/ at any depth. Both scripts installed, both
# reported success, and both were invisible to git - so a fresh clone would have a
# hook referencing guards that do not exist, and nothing would have said so.
# An installer that cannot see its own output is the same defect as a gate nobody
# runs, one step earlier in the chain.
ignored=0
for f in heavy-job.sh session-restore.sh; do
  if rule="$(git -C "$HOST_ROOT" check-ignore -v ".vjs/bin/$f" 2>/dev/null)"; then
    [ "$ignored" -eq 0 ] && {
      echo ""
      echo "WARNING: this repo's git ignores what was just installed." >&2
      ignored=1
    }
    echo "  .vjs/bin/$f  <- ignored by  ${rule%%	*}" >&2
  fi
done
if [ "$ignored" -eq 1 ]; then
  cat >&2 <<'WARN'

  These files exist on THIS machine only. A fresh clone gets a hook that calls
  guards which are not there. Fix the ignore before wiring anything:

      echo '!.vjs/bin/' >> .gitignore     # un-ignore just this directory

  Then confirm with:  git check-ignore -v .vjs/bin/heavy-job.sh   (expect no output)
WARN
fi

LOCK_DEFAULT="\$HOME/.cache/vjs/heavy.lock"

cat <<EOF

Copied, and NOTHING is wired yet. Two edits make them real; both are yours to make.

1. THE FLOOR AND THE LOCK, in $HOST_ROOT's pre-push, around each heavy gate:

     .vjs/bin/heavy-job.sh --label tsc    -- <your tsc command>
     .vjs/bin/heavy-job.sh --label lint   -- <your lint command>
     .vjs/bin/heavy-job.sh --label tests  -- <your test command>

   Wrap the gates that actually cost gigabytes. Wrapping a cheap one buys nothing
   and teaches people the guard is noise, which is how a guard gets disabled.

2. ONE LOCK, NOT TWO. If this repo has its own build lock, point it at the same
   file or you have two locks, which is no lock. For opbox specifically:

     export OPBOX_BUILD_LOCK="\${VJS_HEAVY_LOCK:-$LOCK_DEFAULT}"

   near the top of the hook, so scripts/opbox-build-lock and heavy-job.sh contend
   for one inode instead of politely ignoring each other.

TUNING
  VJS_HEAVY_FLOOR_MB   MemAvailable a job needs to start   (default 4000)
  VJS_HEAVY_WAIT_SEC   how long it waits before REFUSING   (default 1800)
  VJS_HEAVY_LOCK       the one lock path for this machine  (default $LOCK_DEFAULT)

WHAT IT STILL DOES NOT DO
  It refuses to START a job without headroom. It cannot CAP one that grows: the
  process that caused this reached 9.3GB RSS while carrying an 8GB V8 heap cap,
  because a heap cap is not an RSS cap. Bounding growth needs a cgroup
  (systemd-run --scope -p MemoryMax=), which is not installed here and is not
  pretended to be.

RECOVERY, when it happens anyway
  .vjs/bin/session-restore.sh --list          report dead sessions, touch nothing
  .vjs/bin/session-restore.sh                 reopen them in free tmux slots 1-10
EOF
