#!/usr/bin/env bash
#
# VJS host setup - THE WALL (REG-FRONT-DOOR-DELIVERY-001, [2026] VJS-PC 14 D6).
#
# The dockerized MCP server is the front door (the well-lit, easy path). THIS script
# installs the bypass-proof, absolute-path commit hooks on the host - the sole seat of
# the enforcement guarantee, which never depends on the container being up. It needs
# no Rust toolchain: it exports the vjs binary straight out of the Docker image.
#
#   scripts/vjs-host-setup.sh [jurisdiction] [principal]
#
set -euo pipefail

ROOT="$(git rev-parse --show-toplevel)"
JUR="${1:-vjs}"
PRIN="${2:-principal}"

echo "Exporting the host kernel binary from the Docker image (no Rust needed)..."
docker build --target export --output "type=local,dest=$ROOT/bin" "$ROOT"
chmod +x "$ROOT/bin/vjs"

echo "Installing the bypass-proof commit hooks (the wall)..."
# invoke writes the repo-root resolver hooks; with bin/vjs present they bind to it.
"$ROOT/bin/vjs" invoke --jurisdiction "$JUR" --principal "$PRIN" --install-hooks
"$ROOT/bin/vjs" install-lock

cat <<EOF

Host wall installed:
  - $ROOT/bin/vjs            (the kernel binary the hooks call)
  - .vjs/hooks/pre-commit    (validate --staged on every commit)
  - .vjs/install.lock        (the atomic install manifest)

The commit gate is now active and is the enforcement guarantee. The MCP server
(the front door) is optional defence in depth:

  docker compose run --rm -i vjs-mcp

Register it with your MCP client as: command "docker",
args ["compose","run","--rm","-i","vjs-mcp"] (or a plain docker run -i).
EOF
