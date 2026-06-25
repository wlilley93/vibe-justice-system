# VJS Docker delivery (the door and the wall)

Governed by `REG-FRONT-DOOR-DELIVERY-001` ([2026] VJS-REG 30), giving effect to
[2026] VJS-PC 14 D5/D6. The kernel ships two ways, and the split is load-bearing.

## The wall (do this first)

The bypass-proof commit hook is the *sole enforcement guarantee*. It runs on the
host, never in the container.

```bash
scripts/vjs-host-setup.sh           # exports bin/vjs from the image, installs the hooks
```

No Rust toolchain needed - the binary is exported straight out of the Docker image.
After this, every commit runs `vjs validate --staged` on the host. This is what
actually holds the line, whether or not the MCP server is ever started.

## The door (optional defence in depth)

The MCP `server_of_law` is a stdio front door, spawned per session by your MCP client:

```bash
docker compose run --rm -i vjs-mcp
```

Register it with your MCP client:

```json
{ "command": "docker", "args": ["compose", "run", "--rm", "-i", "vjs-mcp"] }
```

It exposes nine verbs: `route, lookup, validate, log, file, status` (the lifecycle)
plus `allocate, convene, record` (governed-record creation). It makes the lawful path
the easy path - but it is *not* the wall. An agent can decline it and write a file
directly; the host commit hook catches that anyway. No enforcement guarantee rests on
the container being up, and "the agent used MCP" is never proof of conformance.

## Why the split

Putting the enforcement hook inside the container would make enforcement depend on the
developer choosing to run the container - the exact skip-the-mechanism gap PC-13 and
PC-14 exist to close. So: the container is the door; the host hook is the wall.
