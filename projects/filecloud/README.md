# filecloud

A cleanroom personal cloud file browser to replace Nextcloud, styled with Onyx design primitives and served locally over Tailscale.

## What it is

Filecloud serves `/home/jellytot/Documents/The-Atrophied-Mind/` on Beelink as a dark-themed, fast personal file browser. Accessible from Mac and iPhone over Tailscale. No third-party branding, no sync client required - just a browser.

## What it replaces

Nextcloud at `http://100.113.51.76:8181`. Nextcloud is heavy, slow on ARM, and visually mismatched with the rest of the stack. Filecloud is a focused replacement for its file-browsing and sharing functions only - no calendar, no contacts, no office suite.

## Where things live

| Path | Purpose |
|------|---------|
| `projects/filecloud/` | This project root |
| `projects/filecloud/REQUIREMENTS.md` | Functional and non-functional requirements |
| `projects/filecloud/DESIGN-SPEC.md` | UI/UX design spec with Onyx primitives |
| `projects/filecloud/METACOGNITION.md` | Open questions, unknowns, assumptions |
| `projects/filecloud/DECISION-LEDGER.md` | Founding decisions with rationale |
| `lawpack/v2/specs/filecloud.yaml` | VJS spec (invariants, obligations, review triggers) |
| `.vjs/logs/decisions/LOG-2026-06-26-filecloud-inception.yaml` | Founding decision log |

## Stack (decided)

- Runtime: Node.js with Hono (lightweight, fast, TypeScript-native)
- Frontend: React + Vite (no Next.js overhead for a local tool)
- Styling: Tailwind CSS with Onyx token mapping
- File ops: Node `fs` + `chokidar` for watch
- Preview: pdf.js (PDF), Prism (code/markdown), native video/audio tags
- Port: 3000 (Tailscale-accessible)

## Status

Inception - founding documents written 2026-06-26. Build not started.
