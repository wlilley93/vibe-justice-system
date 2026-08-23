# Cage mode (opt-in capability separation)

Improvement #16. The default VJS posture is **defence in depth**: the MCP server is
a convenient front door, the host commit hook is the wall, and the agent has normal
filesystem access. That is right for a trusted developer. **Cage mode** is the opt-in
hardening for an *untrusted* agent or a production context, where the goal is that the
agent *cannot* write law except through the validated front door.

As [2026] VJS-PC 14 found, the MCP server is only a real wall when paired with
capability separation. Cage mode is that pairing. It changes nothing by default - it
is composed from three opt-in pieces:

## 1. Deny the agent direct write access (the sandbox)

Run the agent sandboxed (container, seccomp, or a restricted user) with **no write
access to `lawpack/` or `.vjs/`**. Now "just write a YAML file" is impossible: the
agent has no hands. This is an OS/container control, outside the kernel.

## 2. The authenticated sole-writer server (#17)

The MCP server (the only process with write capability) runs with a token:

```bash
VJS_MCP_TOKEN=$(openssl rand -hex 32) docker compose run --rm -i vjs-mcp
```

Every call must then carry a matching `_token` param, or it is refused
(`unauthenticated`). Every call - allowed or denied - is appended to the append-only
audit trail at `.vjs/audit/mcp-audit.log`. With the token held only by the
orchestrator (not the agent's prompt), the agent reaches the kernel **only** through
authenticated verbs, each of which validates.

## 3. Server-side enforcement on push (#1)

The host commit hook still bites locally, but a sandboxed CI/remote pipeline that
re-runs `vjs local-ci` on every push/PR (improvement #1, `.github/workflows/`) is the
un-bypassable wall for the shared canon - no `--no-verify` reaches it.

## What you get

With all three: the agent has no filesystem to write law with; its only path to a
record is an authenticated, validated, audited verb; and the canon's remote
re-validates independently. The server boundary has become a real cage - not because
of the boundary itself, but because the agent's write capability was removed and given
solely to the validating server.

Use the default (hook + door) for trusted work; reach for cage mode only when the
agent is untrusted. Nothing here is mandatory - PC-14 deliberately did not require it.
