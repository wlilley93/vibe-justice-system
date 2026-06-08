# MBES Specification: Agent Law and Graph Retrieval CLI

**Status:** implemented  
**Authority:** MoJ request `Judicature/ministry-of-justice/policy/2026-06-07-cli-law-search-graph-agent-retrieval-mbes-request.md`; `[2026] REALM-SI 8`; `[2026] REALM-SI 9` Part 6  
**Owner:** MBES Engineering

## Purpose

Agents should retrieve the smallest adequate public-law slice before governed answers or acts. The
CLI therefore exposes pointer-first commands over the derived Gazette projections:

- `Judicature/law-reports/site/search-index.json` for lexical law search; and
- `Judicature/law-reports/site/citator-graph.json` for graph node and edge lookup.

The commands do not load `corpus.json` by default. Search results, graph nodes, and graph edges are
retrieval aids only. They are not legal force and do not replace the committed judgment, Act,
statutory instrument, or citator record.

## Commands

```bash
cdd law search "public push warrant" --limit 10 --json
cdd law search "agent lawfulness hooks" --kind si --json
cdd law search "Agent Loop" --kind submission --json
cdd law get "[2026] REALM-PC 19" --json
cdd law get "si:7" --include-source --max-chars 2000 --json
cdd graph node "case:[2026] REALM-PC 19" --json
cdd graph edges "si:7" --dir both --limit 20 --json
cdd graph edges "bill:30" --dir in --type cites --json
cdd gazette privacy --json
cdd gazette live-check --json
```

## Output Contract

Default output is compact text for humans. `--json` returns machine-readable records for agents.

Law records include:

- `id`, `kind`, `citation`, `title`, `series`, `court`, `status`, `date`;
- a short `summary` from the stored pointer payload where available;
- `sourcePath`, `pdfPath`, and `slug`; and
- a note that the result is retrieval only.

Default search ranks judgments, statutory instruments, and Acts ahead of public submission records.
Use `--kind submission` when the retrieval target is a request, referral, or policy filing rather
than law.

Graph node records include the resolved node, incoming/outgoing edge counts, graph validation
metadata, and the same retrieval-only note.

Graph edge records include bounded adjacent edges plus compact source and target node summaries,
`type`, `briefWhy`, edge provenance, evidence token, and source/target paths.

## Limits and Boundary

- `cdd law search` defaults to `--limit 10`.
- `cdd graph edges` defaults to `--limit 20`.
- Source text is omitted unless `cdd law get --include-source` is explicit.
- Included source text is bounded by `--max-chars`, defaulting to 4000 characters.
- The command surface reads only the public central Gazette projections and linked public source
  files when explicitly requested. It does not scan local/private judgment trees, logs, operational
  facts, secrets, hostnames, screenshots, or repo-specific personal data.

## Gazette Release Invariant

The public Gazette is live only when the generated local Pages artifacts match the deployed GitHub
Pages artifacts. The release route is:

1. rebuild the Gazette projection;
2. run `cdd gazette privacy` and `cdd local-ci`;
3. retrieve the relevant publication authority with `cdd release-warrant` where the public VJS
   remote is targeted;
4. publish through the authorised CLI route; and
5. run `cdd gazette live-check` against the deployed Pages URL.

`cdd gazette live-check` compares `index.html`, `app.js`, `corpus.json`, and `search-index.json`
by content hash and also compares the local and live corpus item set. A mismatch means the public
reading room is stale even if the local build is correct.

## Validation

Focused validation lives in `Executive/cli/lib/law-lookup.test.js` and
`Executive/cli/lib/gazette-audit.test.js` and is included in
`npm test` for the CLI package.
