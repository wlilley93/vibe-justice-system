# Request for Ruling: Public Release, Merge Route, and Post-Push Review

**Date:** 2026-06-07  
**Filed by:** Lexby, as registrar/engineer placing the matter before the Court  
**Status:** request for ruling; not a judgment, not law, and not a completed post-push review

## Question

Should the Court approve, qualify, or require remediation for the 2026-06-07 public release route
covering:

1. the local-CI hooks and Gazette release;
2. the non-destructive `ours` merge of moved `upstream/master` into the VJS canonical line;
3. the branch-rule bypass notice on public `refs/heads/master`;
4. the publication status of the MoJ post-push review reference;
5. the private typo-path ignore fix for `engineing-department/projects/`; and
6. whether current Gazette graph lineage visualisation is sufficient or a separate graph-map view is
   required before further public release?

## Facts Placed Before the Court

| Item | Fact |
|---|---|
| Development branch | `origin/cli-and-deterministic-citations` at `2c0d58ac535ff0b76b4c5a590b3f7f7b82e22f3c` |
| Public canonical branch | `upstream/master` at `6feaab44d03292b0829a4b8756d5cdff592c467a` |
| Public feature branch | `upstream/cli-and-deterministic-citations` at `6feaab44d03292b0829a4b8756d5cdff592c467a` |
| Public merge route | `upstream/master` had moved to `830048a`; a non-destructive `ours` merge retained that parent while keeping the four-branch VJS tree |
| Public master push | Accepted after local CI and scoped SI7 warrant; GitHub reported branch-rule bypass for requiring pull requests |
| Local CI | Passed before public pushes, including syntax, JSON, public-law index consistency, CLI tests, deterministic aggregate checks, law/graph smoke checks, and whitespace checks |
| Gazette latest-ordering issue | Fixed before the final public merge SHA: same-day court and SI records sort ahead of Acts, preventing `Bill 30` from presenting as latest where same-day court/SI records exist |
| Gazette colourisation | Present: Supreme Court gold, Court of Appeal blue, first instance lighter blue, Acts red, SIs darker red, Privy Council pink/distinct |
| Gazette graph visualisation | Present as per-card lineage with node chips and edge badges backed by `citator-graph.json`; not a separate force-directed graph canvas |
| Private typo-path issue | A local untracked private ADR exists under misspelled `engineing-department/projects/`; a `.gitignore` fix is committed on the development branch but not yet publicly released |

## Authority Cited

- [2026] REALM-SI 7, public-push release warrant and post-push review route.
- [2026] REALM-PC 19, superrepo change-control order requirement.
- [2026] REALM-PC 20, prior post-push review pattern and publication-loop warning.
- Bill 22 and Bill 27, public/private system-data boundary.

## Requested Determination

The Court is asked to determine:

1. whether the completed public pushes to `6feaab44d03292b0829a4b8756d5cdff592c467a` were lawful;
2. whether the `ours` merge route was a valid non-destructive integration of the moved public master;
3. whether the GitHub branch-rule bypass requires remediation or only recording;
4. whether the development-only commits after `6feaab4`, especially the MoJ reference and typo-path
   ignore fix at `2c0d58a`, should be published now, held pending review, or handled by a further
   warrant after review; and
5. whether per-card graph lineage satisfies the Gazette graph visualisation requirement or whether
   MBES must add a dedicated graph-map view.

## Workflow Invocation

The CLI referral surface produced:

```text
Workflow({
  scriptPath: 'Judicature/court/workflows/first-instance.js',
  args: {
    kind: 'request_for_ruling',
    question: 'Post-push review and release-route reference: determine whether the local-CI hooks/Gazette release, non-destructive ours merge of upstream/master, branch-rule bypass on public master, publication of the MoJ post-push reference, private typo-path ignore fix, and remaining graph-visualisation scope comply with [2026] REALM-SI 7, [2026] REALM-PC 19, and the public/private boundary.'
  }
})
```

