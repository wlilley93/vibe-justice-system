# Request for Ruling: Branch Consolidation and Public Release Simplification

**Date:** 2026-06-07  
**Filed by:** Lexby, as registrar/engineer placing the matter before the Court  
**Status:** request for ruling; not a judgment, not law, and not a completed post-push review

## Question

Should the Court approve, qualify, or require remediation for the final 2026-06-07 branch
consolidation release, and should it order a simpler public-push system so future releases flow
through one deterministic `cdd` route rather than manual branch and warrant handling?

## Facts Placed Before the Court

| Item | Fact |
|---|---|
| Branch-consolidation base SHA under review | `9f6666ad217979493a6b60ed57352e5f77013e9e` |
| Public repository | `wlilley93/vibe-justice-system` |
| Development repository | `wlilley93/agent-universe` |
| Remaining public branches | `master` and `public-vjs-canonical-preview`, kept in lockstep after branch cleanup and later request-publication pushes |
| Remaining development branches | `master` and `public-vjs-canonical-preview`, kept in lockstep after branch cleanup and later request-publication pushes |
| Deleted stale public branches | `cli-and-deterministic-citations`, `community/2026-lexby-fi-1`, and `publish/2026-lexby-sc-2` |
| Deleted stale development branches | `cli-and-deterministic-citations` and `publish/2026-lexby-sc-2` |
| Default branches | Both public and development repositories now default to `public-vjs-canonical-preview` |
| GitHub Pages source | Public Pages serves `public-vjs-canonical-preview` at path `/` |
| Gazette stale-site issue | Resolved by fast-forwarding the Pages source branch to the branch-consolidation base SHA |
| Gazette latest-ordering issue | Live Pages assets include the same-day ordering fix that prevents `Bill 30` presenting above same-day court or SI records |
| Gazette colourisation | Live Pages assets include the MoJ taxonomy: Supreme Court gold, Court of Appeal blue, first instance lighter blue, Acts red, SIs darker red, and Privy Council pink/distinct |
| Gazette graph visualisation | Live Pages assets include per-card lineage display, node chips, edge badges, graph JSON access, and CLI graph retrieval support |
| Local CI | `node Executive/cli/bin/cdd.js local-ci --json` passed after branch consolidation and before publication of this reference |
| Operational checkpoint state | No `.vjs/checkpoints` files remained after release handling |
| Host governance signal | Public `master` accepted a push while reporting a branch-rule bypass requiring pull requests |

## Why the Matter Is Referred

The final branch state is tidy, but the route was not. The release required manual warrant creation,
multiple public refs, a linked Pages worktree, branch deletion, default-branch correction, live-site
verification, and post-push explanation. That is too easy for a future agent to get wrong.

The system already says the deterministic CLI is the required spine where a command exists. Public
release does not yet have one first-class deterministic command that owns the whole sequence.

## Authority Cited

- [2026] REALM-SI 7, public-push release warrant and post-push review route.
- [2026] REALM-PC 16, clean public branch and public-push legality review.
- [2026] REALM-PC 19, superrepo change-control order requirement.
- [2026] REALM-PC 20, post-push review pattern and publication-loop warning.
- [2026] REALM-SI 8, [2026] REALM-SI 10, and [2026] REALM-SI 11, agent hook and deterministic workflow duties.
- Bill 20, Bill 22, and Bill 27, repository integrity and public/private system-data boundaries.

## Requested Determination

The Court is asked to determine:

1. whether the branch consolidation represented by `9f6666ad217979493a6b60ed57352e5f77013e9e` was lawful;
2. whether retaining both `master` and `public-vjs-canonical-preview` in lockstep is sufficient, or whether
   the public repository should migrate to literal `main` plus canonical branch terminology;
3. whether branch-rule bypass on `master` requires remediation, branch-protection amendment, or only recording;
4. whether the deletion of stale public/development branches was a lawful cleanup of superseded publication refs;
5. whether per-card Gazette lineage satisfies the present graph-visualisation requirement;
6. whether future public release should be required to flow through a single deterministic CLI route, provisionally
   described as `cdd release`, that:
   - runs `cdd local-ci`;
   - verifies the public-data boundary;
   - creates or validates the [2026] REALM-SI 7 release warrant;
   - verifies exact remote, ref, SHA, branch default, and Pages source;
   - pushes only the authorised refs;
   - deletes or refuses stale publication branches unless expressly warranted;
   - records the post-push review reference; and
   - reports all branch-rule bypasses or host-governance notices;
7. whether MBES should be ordered to implement that CLI route, with MoJ owning the public-law checklist and
   the Court retaining post-push review.

## Workflow Invocation

The intended CLI referral surface is:

```text
Workflow({
  scriptPath: 'Judicature/court/workflows/first-instance.js',
  args: {
    kind: 'request_for_ruling',
    question: 'Branch consolidation and release simplification: determine whether the final 2026-06-07 public/development branch consolidation to 9f6666ad217979493a6b60ed57352e5f77013e9e complied with [2026] REALM-SI 7, [2026] REALM-PC 19, and the public/private boundary; determine whether master plus public-vjs-canonical-preview may remain in lockstep or should migrate to main plus canonical naming; and determine whether MBES must implement a single deterministic cdd release route for future public pushes.'
  }
})
```
