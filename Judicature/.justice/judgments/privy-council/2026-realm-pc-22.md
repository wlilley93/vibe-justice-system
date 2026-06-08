---
citation_id: "[2026] REALM-PC 22"
tier: privy-council
kind: request_for_ruling
status: good-law
per_incuriam: false
date: 2026-06-08
panel: ["Sumberly J", "Coade J", "Marsden J"]
seised_by: "Agent Loop posthook self-referral: development-origin push, draft PR #1, CLI action-spine clarification, and push-licence retrieval"
cause_title: "In the matter of development remotes, draft pull requests, merge readiness, CLI-first governed actions, and release-warrant retrieval"
adjudication_provenance: authorised-registrar
registrar_authority: "[2026] REALM-SC 8; [2026] REALM-PC 19; Bill 31 ss. 10, 14-17"
registrar_note: "Authored by the bench (Sumberly J for the Court, Coade J and Marsden J concurring); reduced to the filed record by Lexby as s.18(4) registrar, the decision pre-existing the prose ([2026] REALM-SC 8)."
---

# [2026] REALM-PC 22

| Field | Value |
|-------|-------|
| **Citation** | [2026] REALM-PC 22 |
| **Tier** | Privy Council (constitutional first instance, bench of three) |
| **Before** | Sumberly J (judgment of the Court), Coade J, Marsden J |
| **Kind** | Request for ruling |
| **Status** | good-law |
| **Cites** | CASE-LAW s. 1; s. 3; s. 5; s. 6; s. 8; s. 13; s. 18(4)-(5); s. 19(1)/(5); Bill 6; Bill 16; Bill 20; Bill 22; Bill 27; Bill 31; [2026] REALM-SC 8; [2026] REALM-PC 19; [2026] REALM-PC 20; [2026] REALM-PC 21; [2026] REALM-SI 7; [2026] REALM-SI 8; [2026] REALM-SI 10; [2026] REALM-SI 11 |

> The Court answers the posthook self-referral. The development-remote push and draft PR state are lawful preparation acts on this record. The Agent Loop must be read and conformed so governed actions flow through an available CLI action route, and so relevant push licences or release warrants are retrieved through the CLI before movement, unless a recorded exemption applies. Unanimous (3-0).

## Questions

1. Was pushing the development branch to `origin` and opening draft PR #1 lawful?
2. Does the GitHub connector route used for the draft PR invalidate the PR or require retraction?
3. How should the Principal's direction that basically every action should flow through the CLI be expressed?
4. Must the CLI retrieve the relevant licences, release warrants, or route authority before governed push and merge movement?

## Ratio (binding, realm-wide)

1. The push to `origin` was a development-remote act, not a canonical public VJS publication. On the record before the Court, the VJS pre-push gate identified `origin` as the non-canonical development remote and allowed the push. [2026] REALM-SI 7 was not triggered as a canonical public VJS release warrant because the push was not to the public VJS remote.

2. Draft PR #1 was a review and merge-preparation surface, not a merge, publication to the canonical VJS public remote, or final public-law enactment step. It may remain open in draft while checks, review, and legal-route questions are resolved.

3. The local VJS gates were adequate for development push preparation on this record: `cdd local-ci` passed, the pre-commit provenance check passed, the citator passed, the bench-name scan passed, render-and-lodge rebuilt projections, and the pre-push hook allowed only the development remote route actually used.

4. The use of a non-CLI GitHub connector to create the draft PR is not invalidating on this record. It did not merge code, bypass a failing local VJS gate, publish to the canonical public remote, or alter the source of legal force. It is, however, a recorded procedural defect against the now-clarified action-spine discipline because a safe `gh` route existed or likely existed for the same PR act.

5. The Principal's direction is accepted as a clarification of Bill 31, not as a new sanction rule: governed actions should flow through an available CLI action route. The default is:

   - use `cdd` for VJS law, filing, citation, graph, lodge, local CI, release-gate, release-warrant retrieval, and deterministic validation work;
   - use `git` for repository state and branch movement;
   - use `gh` for GitHub PR, check, review, readiness, and merge-preparation state where available;
   - use runtime CLIs for Codex, Claude, Gemini, opencode, and equivalent agent-runtime probes where available;
   - use build and test CLIs for validation.

6. Before a governed push, pull-request readiness step, merge, release, publication, or equivalent outward act, the agent must retrieve the relevant licence, release warrant, or route authority through the CLI where a CLI command exists. For public VJS pushes, `cdd release-warrant` and its aliases `cdd push-licence` and `cdd push-license` are the default retrieval route. The CLI output is authority evidence; it does not itself create authority.

7. A non-CLI connector, web UI, app connector, or direct API path is an exemption route for governed actions where a CLI route exists. It may be used only if no safe CLI route exists, the CLI lacks the needed permission or capability, CLI use would expose private facts, credentials, secrets, or protected operational material, a competent platform route requires the non-CLI surface, or urgency requires a reversible protective act. The reason and substitute check must be recorded.

8. The remedy for the connector use here is curative, not destructive. The record has now been self-referred; Bill 31, the CLI, and the agent instructions may be conformed; future PR updates, check inspection, readiness changes, pushes, and merge movements on this branch should use `gh`, `git`, `cdd`, or another source-equivalent CLI route unless an exemption is recorded.

9. This ruling does not authorise merging PR #1. Merge requires the ordinary repository authority, clean local VJS checks, any required remote checks or review posture, and any separate public-release warrant if the merge or later publication targets the canonical public VJS remote.

## Reasons

The Agent Loop is a determinism discipline. The CLI requirement is not aesthetic. It creates auditable commands, reproducible output, and a narrow path for law lookup, branch movement, release preparation, licence retrieval, and review state.

The earlier record already moved strongly in this direction. Bill 31 required `cdd` where available. [2026] REALM-PC 20 approved a release sequence with deterministic local checks and a release warrant. [2026] REALM-PC 21 required honest adapter records and substitute checks rather than capability overclaims. The Principal's present instruction identifies the missing generality: service actions and repository movements should also flow through CLI routes where safe, and the CLI should surface the relevant licence before the act moves.

The Court does not undo a draft PR merely because a connector was used once. That would confuse a record defect with invalidity. The draft PR is reversible, visible, and unmerged. The correct remedy is to file the referral, state the rule, add the CLI retrieval command, update the agent-facing law and instructions, and use CLI routes going forward.

The same answer protects against ceremony. If the CLI is absent, insufficient, unsafe, unauthorised, or cannot perform the act, an exemption route is allowed. The record must say so. The loop remains proportionate.

## Disposal

1. The development-remote push of the development branch to `origin` is approved on the present record.
2. Draft PR #1 is approved as a draft review and merge-preparation surface on the present record.
3. The non-CLI connector used to create the draft PR is recorded as a curable process defect, not an invalidating act.
4. Bill 31, the CLI, and the agent-facing instruction files may be conformed so CLI action routing and push-licence retrieval are express.
5. Future PR/check/review/readiness/merge movements should use `gh` or a source-equivalent CLI route unless an exemption is recorded.
6. Before governed push/release/publication movement, the agent should retrieve the relevant licence or release warrant with `cdd release-warrant` or a source-equivalent CLI route where available.
7. PR #1 must not be merged merely by this ruling. Merge remains subject to the ordinary local, remote, repository, and public-release gates.
8. The matter does not climb. No conflict with Supreme Court authority appears.

## Appendix A - record accepted

| Item | Finding |
|---|---|
| Branch | `[development branch]` |
| Development remote | `origin`, `[development repository]` |
| Public VJS remote | `upstream`, `https://github.com/wlilley93/vibe-justice-system.git` |
| PR | Draft PR #1 against `master` in `[development repository]` |
| Local gate | `cdd local-ci --json` passed before the first push |
| Pre-push gate | allowed `origin` as non-canonical/development remote |
| Licence retrieval | `cdd release-warrant` reports no public VJS warrant required for `origin`; it retrieves the matching prior public VJS release warrant when supplied with that public remote, ref, and SHA |
| Remote check posture | GitHub Actions constitutional review was queued at self-referral |
| Merge posture | no merge performed |
