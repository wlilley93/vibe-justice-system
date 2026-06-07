# Agent instructions

This repository runs under the Vibe Justice System. The authoritative operational directive is
`Constitution/AGENTS.md`; read it before load-bearing work.

The hook and workflow contract is agent-agnostic. Claude Code is one adapter. Any agent with a
delegable workflow, task, subagent, reviewer, or equivalent independent-check surface must use that
separation where it materially improves lawfulness, verification, routing, or record integrity.
Every agent must make good, on a best-efforts basis, the ability for the hook to trigger in the
runtime it is actually using: enable the available adapter, use portable `.vjs/hooks/`, follow this
instruction file, run deterministic checks, delegate where supported, or record the substitute check
where automatic triggering is unavailable.

Before governed load-bearing work, retrieve the current law and record first, then identify the
source of authority and route. After governed load-bearing work, review authority, candour,
final/provisional status, public/private boundary, and whether correction, supersession, appeal, or
referral is required.

Do not edit Gazette graph implementation files or bench-name scanner files unless the user
explicitly assigns that work.
