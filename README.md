# VJS V2

The second generation of the Vibe Justice System.

## What is VJS V2?

VJS V2 is a deterministic authority resolver and route clerk for AI-assisted software engineering. It replaces long constitutional prose with a compact, machine-checkable lawpack and a Rust kernel that returns bounded instructions.

## Core principle

> The agent reads the smallest binding instruction required to act lawfully.

## V2 vs V1

| | V1 | V2 |
|---|---|---|
| Runtime | Full case law, Acts, SIs | Compact lawpack |
| Context | Long judgments | Short orders, rules, logs |
| Hooks | Long prose | 40-word state checks |
| Courts | 4 tiers + CoA | County, Privy, Supreme |
| Enforcement | Agent discretion | Deterministic kernel |
| Gazette | Runtime source | Archive only |

## Architecture

```
vjs-core       Deterministic authority resolver
vjs-lawpack    Parse, validate, build authority graph
vjs-store      Text-backed records + SQLite cache
vjs-cli        Human and hook commands
vjs-mcp        Thin MCP adapter (6 tools)
vjs-git        Repo detection, diff, hooks
vjs-redact     Boundary scanner
vjs-testkit    Fixtures, golden tests
```

## Installation

```bash
# Clone
git clone https://github.com/wlilley93/agent-universe-v2
cd agent-universe-v2

# Build
cargo build --release

# Install into a repo
./target/release/vjs init
```

## Quick start

```bash
# Route a decision
vjs route --kind implementation-decision --issue dependency_policy --risk low --intent "Add a new crate"

# Write a log
vjs log decision --issue dependency_policy --decision "Keep kernel dependency-free" --basis AGENT-LOOKUP-001 --risk low --why "Kernel determinism requires auditable local execution."

# Validate
vjs validate --staged

# Full CI
vjs local-ci
```

## Lawpack

The V2 lawpack is in `lawpack/v2/`:

- `constitution.yaml` — V2 constitution
- `statutes/` — 7 compact Acts
- `regulations/` — Kernel regulations
- `rules/` — Rule atoms
- `orders/` — Court orders
- `specs/` — Specs (machine-checkable contracts)
- `invariants/` — Deterministic invariants
- `decisions/` — Compact decisions

## Self-governance

This repo is governed by VJS V2. See `AGENTS.md` for the agent contract.

## License

MIT
