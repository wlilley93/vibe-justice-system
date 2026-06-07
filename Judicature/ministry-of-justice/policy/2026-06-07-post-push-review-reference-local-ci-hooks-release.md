# Post-Push Review Reference: Local-CI Hooks and Gazette Release

**Date:** 2026-06-07  
**Owner:** Ministry of Justice policy arm  
**Status:** public system-data reference; not law, not a judgment, and not the Privy Council review itself

## Reference

The Ministry of Justice places the following completed public VJS pushes before the Privy Council
for post-push review under [2026] REALM-SI 7:

| Public act | Remote ref | Previous public SHA | New public SHA | Result |
|---|---|---:|---:|---|
| Publish completed agent-hooks/local-CI/Gazette branch tip | `refs/heads/cli-and-deterministic-citations` | `3f5776a8bce491eed41a9ab15e587c1ac626466f` | `cff7344d218117cd59f9e4ea19afaec30471c7a3` | accepted |
| Merge public master into the VJS canonical line by non-destructive `ours` merge | local merge commit | `830048a` parent retained | `6feaab44d03292b0829a4b8756d5cdff592c467a` | local merge complete |
| Publish merged canonical branch | `refs/heads/master` | `830048a` | `6feaab44d03292b0829a4b8756d5cdff592c467a` | accepted; host reported branch-rule bypass |
| Align public feature branch with merged canonical SHA | `refs/heads/cli-and-deterministic-citations` | `cff7344d218117cd59f9e4ea19afaec30471c7a3` | `6feaab44d03292b0829a4b8756d5cdff592c467a` | accepted |

## Checks Recorded

- `cdd local-ci --json`: passed at `cff7344d218117cd59f9e4ea19afaec30471c7a3`.
- `cdd local-ci --json`: passed at `6feaab44d03292b0829a4b8756d5cdff592c467a`.
- Public pre-push gate: matched scoped [2026] REALM-SI 7 release-warrant checkpoint for each
  public ref before publication.
- Gazette latest-ordering regression: fixed before publication; same-day court and SI records now
  sort ahead of Acts, and local CI fails if `Bill 30` would present as latest while same-day court
  or SI records exist.
- Gazette colourisation: SC gold, CA blue, first-instance lighter blue, Acts red, SIs darker red,
  and PC pink/distinct presentation classes were retained.
- Gazette graph visualisation: per-card lineage display, node chips, edge badges, graph JSON access,
  and `cdd graph` retrieval were retained.
- Public-law index consistency: local CI checked repeated citations, slugs, source paths, and graph
  validation status.

## Questions for Review

1. Did each completed public push match a scoped [2026] REALM-SI 7 release warrant?
2. Was the non-destructive `ours` merge of `upstream/master` into the VJS canonical line lawful and
   adequate to avoid reintroducing duplicate old-layout paths?
3. Was the branch-rule bypass notice on `refs/heads/master` acceptable in light of the local release
   warrant and deterministic local CI gate?
4. Did the public-data boundary and repository-integrity chain hold?
5. Is any remediation required?

## Remaining Todo

| Item | Status | Owner route | Note |
|---|---|---|---|
| Privy Council post-push review | pending | Court / MoJ reference route | This file places the matter for review; it is not itself the review. |
| Semantic legal contradiction and redundant-law analysis | open | Court / MoJ policy / MBES tooling | `cdd local-ci` catches deterministic duplicates and graph-validation failures, not semantic contradiction across all law. See ALH-19. |
| Full graph map view, if wanted beyond per-card lineage | open unless directed | MBES UI route | Current release includes lineage visualisation on cards, not a separate force-directed graph canvas. |

