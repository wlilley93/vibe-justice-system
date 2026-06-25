#!/usr/bin/env bash
#
# VJS one-command bootstrap (improvement #22). Clone -> wired, in a single step:
# installs the host wall (the bypass-proof commit hooks), locks the install surface,
# and prints the MCP client config for the front door. Idempotent.
#
#   scripts/bootstrap.sh [jurisdiction] [principal]
#
set -euo pipefail

ROOT="$(git rev-parse --show-toplevel)"
JUR="${1:-vjs}"
PRIN="${2:-principal}"

echo "==> VJS bootstrap for $ROOT (jurisdiction: $JUR)"

# 1. Get a kernel binary on the host: prefer an existing build, else Docker export,
#    else cargo. No hard dependency on a Rust toolchain.
BIN=""
for c in "target/release/vjs" "target/debug/vjs" "bin/vjs"; do
  [ -x "$ROOT/$c" ] && BIN="$ROOT/$c" && break
done
if [ -z "$BIN" ]; then
  if command -v docker >/dev/null 2>&1; then
    echo "==> exporting the kernel binary from the Docker image (no Rust needed)"
    docker build --target export --output "type=local,dest=$ROOT/bin" "$ROOT"
    BIN="$ROOT/bin/vjs"; chmod +x "$BIN"
  elif command -v cargo >/dev/null 2>&1; then
    echo "==> building the kernel with cargo"
    (cd "$ROOT" && cargo build --release --bin vjs)
    BIN="$ROOT/target/release/vjs"
  else
    echo "ERROR: need an existing vjs binary, Docker, or cargo." >&2; exit 1
  fi
fi

# 2. Install the wall (hooks) + lock the surface. invoke is create-if-absent.
echo "==> installing the host wall (commit hooks) and locking the surface"
"$BIN" invoke --jurisdiction "$JUR" --principal "$PRIN" --install-hooks
"$BIN" install-lock

# 3. The front door (optional, defence in depth) - print the MCP client config.
cat <<EOF

==> Bootstrap complete. The wall is active: every commit runs vjs validate --staged.

Optional front door (the MCP server_of_law). Register with your MCP client:

  { "command": "docker", "args": ["compose", "run", "--rm", "-i", "vjs-mcp"] }

For an untrusted/prod agent, see docs/cage-mode.md (set VJS_MCP_TOKEN).
EOF
