#!/usr/bin/env bash
# Install the local fail-closed publication-boundary hooks (defense-in-depth
# under the canon-enforce CI trust root). Run once per clone. --no-verify can
# bypass these locally, which is exactly why the SAME scan also runs in CI as a
# required check - the remote verdict, not the committer's, admits a change.
set -euo pipefail
root="$(git rev-parse --show-toplevel)"

# THIS SCRIPT WAS A NO-OP FOR AS LONG AS core.hooksPath HAS BEEN SET
# ([2026] VJS-CC-VJS 20 D21). It writes to .git/hooks, and git ignores .git/hooks
# ENTIRELY once core.hooksPath points elsewhere. So the boundary gate was installed,
# executable, correct, and never once executed - which is why a registered private term
# sat in a published register filename for weeks. The scan is now called directly from
# the hook git actually runs, so this installer's only remaining job is to REFUSE to
# create the impression that it did something.
#
# It must not simply write to the hooksPath instead: the loop below is `cat >`, which
# would overwrite the kernel's own pre-commit and delete the validate gate. Curing an
# unrun gate by silently removing a different one is not a cure.
hooks_path="$(git config core.hooksPath || true)"
if [ -n "$hooks_path" ]; then
  echo "core.hooksPath is set to '$hooks_path', so git ignores .git/hooks." >&2
  # core.hooksPath may be absolute or repo-relative; resolve both without assuming.
  case "$hooks_path" in /*) hooks_dir="$hooks_path" ;; *) hooks_dir="$root/$hooks_path" ;; esac
  if grep -q "boundary-scan.sh" "$hooks_dir/pre-commit" 2>/dev/null; then
    echo "The boundary scan is already wired into $hooks_path/pre-commit. Nothing to do." >&2
    exit 0
  fi
  echo "REFUSING to install into .git/hooks, where nothing would run it." >&2
  echo "Add this line to $hooks_dir/pre-commit instead:" >&2
  echo '  bash "$(git rev-parse --show-toplevel)/scripts/boundary-scan.sh" --cached || exit 1' >&2
  exit 1
fi

hooks="$root/.git/hooks"
mkdir -p "$hooks"
for h in pre-commit pre-push; do
  cat > "$hooks/$h" <<'HOOK'
#!/usr/bin/env bash
# fail-closed publication boundary (installed by scripts/install-boundary-hooks.sh)
exec bash "$(git rev-parse --show-toplevel)/scripts/boundary-scan.sh"
HOOK
  chmod +x "$hooks/$h"
  echo "installed $h"
done
echo "boundary hooks active. CI (canon-enforce) is the non-bypassable backstop."
