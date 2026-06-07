# Policy Request: CLI Law Search and Gazette Graph Retrieval

**Type:** Ministry policy request (MoJ public-law process -> competent implementation route)  
**To:** Ministry of Business, Engineering and Skills (MBES), Engineering department  
**From:** Ministry of Justice (MoJ), public-law record and Gazette process  
**Subject:** agent-facing CLI access to law search and Gazette graph nodes  
**Date:** 2026-06-07  
**Status:** implemented by MBES Engineering; see `Executive/cli/LAW-RETRIEVAL-SPEC.md`

> This is a policy request, not a judgment, statutory instrument, or source of law. MoJ states the
> public-law retrieval need and boundary. MBES owns the technical specification and implementation
> under `[2026] REALM-SI 9` Part 6. Lexby may coordinate the route but is not the sovereign drafter,
> bench, Legislature, MoJ, MBES, MDS, or source of legal force.

## 1. Problem

Agents working under VJS should not rely on memory of the record when answering or acting on
load-bearing legal, routing, or implementation questions. The public Gazette already contains:

- a public corpus projection;
- a lexical search index;
- a pointer-only Gazette graph; and
- a graph validation report.

Those projections are useful for humans in the web Gazette, but agents also need a small,
deterministic command surface that retrieves only the relevant records, graph nodes, and adjacent
lineage edges. Without such a surface, agents are tempted to load large files or reason from stale
context.

## 2. Existing Authority

The existing route is sufficient for a narrow implementation request:

- `[2026] REALM-SI 8` requires proportionate pre-answer law research and post-answer validity review
  for governed load-bearing work.
- `[2026] REALM-SI 9` creates the Gazette graph and states that the graph may be used by agents as a
  retrieval and orientation tool.
- `[2026] REALM-SI 9` Part 6 gives MoJ ownership of the public-law taxonomy, disputed-edge route,
  public/private boundary, and backfill policy.
- `[2026] REALM-SI 9` Part 6 gives MBES ownership of the technical specification and execution,
  including build scripts, API design, validation gates, derived artefact regeneration,
  performance, and maintainability.
- `[2026] REALM-PC 19` requires superrepo changes to proceed under a court order or existing
  authority. This request relies on the existing SI 8 and SI 9 route and creates no new law.

No Privy Council or Supreme Court referral is requested at this stage. Referral becomes necessary
only if implementation would decide disputed legal meaning, alter source of force, change the
public/private boundary, or reveal a genuine conflict in the governing authorities.

## 3. MoJ Request

MoJ requests that MBES decide and execute an implementation specification for agent-facing CLI
retrieval over the public Gazette projections.

The implementation should make available, at minimum:

1. **Law search.** A command that searches public judgments, Acts, and statutory instruments and
   returns compact pointer records suitable for agents.
2. **Record lookup.** A command that resolves a known citation or stable identifier to a pointer
   record, source path, PDF path, status, date, and short public summary where available.
3. **Graph node lookup.** A command that resolves a Gazette graph node by stable id or citation.
4. **Graph edge lookup.** A command that returns bounded incoming, outgoing, or both-direction
   adjacent edges with brief-why text and neighbouring node summaries.
5. **Machine-readable output.** JSON output for agent use, with text output available for humans.
6. **Bounded retrieval.** Limits and filters so an agent retrieves the smallest adequate slice of the
   record rather than loading the whole corpus.

## 4. Boundary and Legal Effect

The implementation must preserve these limits:

- The canonical law remains the committed judgment, Act, statutory instrument, and citator record.
- Search results are retrieval aids, not law.
- Graph nodes and edges are public relationship evidence and orientation aids, not legal force.
- A graph edge does not replace ratio, statutory text, remedy, or a competent organ's decision.
- Local/private judgment trees, private evidence, operational facts, secrets, hostnames, credentials,
  logs, screenshots, and repo-specific personal data must not be scanned or exposed by the public
  command surface.
- Ambiguous legal relationships should be routed to MoJ, the Court, or the Legislature as appropriate
  rather than silently classified by implementation code.

## 5. Requested MBES Deliverables

MBES should provide:

1. a public implementation specification describing command names, schemas, limits, and validation;
2. the CLI implementation;
3. tests for search, node lookup, edge lookup, and boundary behaviour;
4. documentation for agents explaining when to use the commands before governed answers or acts; and
5. an update to the outstanding-work register marking the route and implementation status.

Any draft code prepared before this request should be treated as unratified implementation material
until MBES adopts, rejects, or supersedes it through the proper route.
