# Request for Privy Council Ruling: Development Push, Draft PR, and CLI Action Spine

**Date:** 2026-06-08
**Filed by:** Lexby, as registrar/engineer following the Agent Loop posthook self-referral instruction
**Proposed court:** Privy Council of the Vibe Justice System
**Route requested:** constitutional/governance first-instance ruling on development remotes, draft PRs, CLI-first action routing, and push-licence retrieval
**Status:** public request; system-data only; not a judgment, order, or law

## Question

Was the development-remote push of branch `codex/agent-loop-hooks` and the creation of draft PR #1 lawful, and how should the Agent Loop express the Principal's direction that basically every governed action should flow through the CLI, including retrieval of relevant push licences and warrants?

## Proposed Answer

The requested ruling should confirm that:

1. the push to `origin` was a development-remote act, not a canonical public VJS publication;
2. the draft PR was a preparation/review surface, not a merge or canonical publication;
3. the local VJS checks passed before the push and the VJS pre-push gate allowed `origin` as a non-canonical development remote;
4. using a non-CLI GitHub connector to create the draft PR was not invalidating on this record, but it revealed a procedural gap;
5. future GitHub PR, check, review, readiness, and merge movements should use the GitHub CLI (`gh`) or another source-equivalent CLI route where safe and available;
6. before a governed push, release, publication, PR readiness step, or merge, the agent should retrieve the relevant licence, release warrant, or route authority through the CLI where a CLI command exists;
7. a non-CLI route is an exemption route and should record why the CLI was unavailable, insufficient, unsafe, unauthorised, or unsupported.

## Legal Sources

- Bill 31, especially the CLI-first, posthook, exemption, and self-referral provisions.
- [2026] REALM-SI 7, on public release and post-push review.
- [2026] REALM-SI 8, [2026] REALM-SI 10, and [2026] REALM-SI 11, on hooks, agent-agnostic adapters, and best-efforts triggering.
- [2026] REALM-PC 19, on superrepo edit control.
- [2026] REALM-PC 20, on development backup, public push review, and deterministic release checks.
- [2026] REALM-PC 21, on honest adapter equivalence and recorded limitations.
- [2026] REALM-SC 8, on source of force and Lexby's limits.

## Evidence on the Record

The filing record includes:

1. `cdd local-ci --json` passing before commit;
2. pre-commit provenance, citator, bench-name scan, and render-and-lodge gates passing;
3. `git push -u origin codex/agent-loop-hooks` accepted by the VJS pre-push hook as a non-canonical development remote;
4. draft PR #1 opened against `master` for branch `codex/agent-loop-hooks`;
5. `gh pr view` recording the PR as draft, open, and `UNSTABLE` while GitHub Actions remained queued;
6. `cdd release-warrant` reporting that `origin` requires no public VJS release warrant, and retrieving the earlier matching private release warrant for the canonical public VJS push when supplied with its remote/ref/SHA;
7. no direct merge and no canonical public VJS push.

## Requested Direction

The Privy Council is asked to:

1. rule on the development push and draft PR state;
2. direct conformance of Bill 31, the CLI, and the agent instructions so CLI action routing and push-licence retrieval are explicit;
3. confirm that the draft PR should remain unmerged until local checks and any required remote or legal gates are satisfied;
4. confirm that this clarification adds no new sanction or source of law, and only consolidates the already-existing deterministic-action discipline.

## Data Boundary

This request is system-data only. It records branch names, commit/PR posture, public CLI classes, warrant-route fields, and law references. It does not reproduce secrets, credentials, private logs, local evidence, or protected operational material.
