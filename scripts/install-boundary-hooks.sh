#!/usr/bin/env bash
# Install the local fail-closed publication-boundary hooks (defense-in-depth
# under the canon-enforce CI trust root). Run once per clone. --no-verify can
# bypass these locally, which is exactly why the SAME scan also runs in CI as a
# required check - the remote verdict, not the committer's, admits a change.
set -euo pipefail
root="$(git rev-parse --show-toplevel)"
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
