---
citation_id: "[2026] REALM-PC 20"
tier: privy-council
kind: request_for_ruling
status: good-law
per_incuriam: false
date: 2026-06-07
panel: ["Sumberly J", "Coade J", "Marsden J"]
seised_by: "Ministry of Justice reference under [2026] REALM-SI 7: post-push review of the agent-lawfulness and Gazette graph publication"
cause_title: "In the matter of the public push of agent lawfulness hooks, Gazette graph lineage, private working areas, and bench-name conformance"
registrar_note: "Authored by the bench (Sumberly J for the Court, Coade J and Marsden J concurring); reduced to the filed record by Lexby as s.18(4) registrar, the decision pre-existing the prose ([2026] REALM-SC 8)."
---

# [2026] REALM-PC 20

| Field | Value |
|-------|-------|
| **Citation** | [2026] REALM-PC 20 |
| **Tier** | Privy Council (constitutional first instance, bench of three) |
| **Before** | Sumberly J (judgment of the Court), Coade J, Marsden J |
| **Kind** | Post-push review under [2026] REALM-SI 7 |
| **Status** | good-law |
| **Cites** | CASE-LAW s. 1; s. 3; s. 5; s. 6; s. 8; s. 13; s. 18(4)-(5); s. 19(1)/(5); Bill 8; Bill 16; Bill 18 ss. 2(e), 3, 7; Bill 20; Bill 22; Bill 27; Bill 29; [2026] REALM-SC 8; [2026] REALM-PC 16; [2026] REALM-PC 17; [2026] REALM-PC 19; [2026] REALM-SI 7; [2026] REALM-SI 8; [2026] REALM-SI 9; [2026] REALM-SI 10; [2026] REALM-SI 11 |

> The Court reviews the completed public push of commit `3f5776a8bce491eed41a9ab15e587c1ac626466f` to `refs/heads/cli-and-deterministic-citations` in the public VJS super-repo. Unanimous (3-0).

## The questions

1. Did the public push match the release warrant required by [2026] REALM-SI 7?
2. Were the pre-push checks adequate for the nature of the push?
3. Did the public-data boundary and repository-integrity chain hold?
4. Is any remediation required?

## Ratio (binding, realm-wide)

1. The completed public push of commit `3f5776a8bce491eed41a9ab15e587c1ac626466f` to `refs/heads/cli-and-deterministic-citations` was lawful under [2026] REALM-SI 7. The deterministic pre-push gate found a scoped release warrant before publication and the public remote fast-forwarded from `2b68bc196434fd5762a1a37811792d28834b2516` to `3f5776a8bce491eed41a9ab15e587c1ac626466f`.

2. The release warrant adequately identified the outward act, authorising office, timestamp, public remote, public ref, exact local SHA, intended effect, legal authority, public-data boundary check, private backup state, and checks run.

3. The pushed content was authorised by the cited legal instruments and judgments: [2026] REALM-SI 8 to [2026] REALM-SI 11 for the agent-lawfulness hooks and best-efforts trigger duty; [2026] REALM-SI 9 for the Gazette graph; [2026] REALM-PC 19 for superrepo change control; [2026] REALM-SC 8 for source-of-force discipline; and [2026] REALM-SI 7 for the release process itself.

4. The public-data boundary held. The public commit contains system data, law, public policy records, public adapter records, public derived graph records, redacted court-route records, and tracked placeholders for private working areas. It does not publish the unredacted local evidence retained in the gitignored private working areas.

5. The repository-integrity chain held. The citator check passed; the bench-name scanner passed; the Gazette graph validation passed; law-report build and CLI tests passed; whitespace checking passed; and the pre-commit render-and-lodge gate rebuilt derived projections in lockstep before commit.

6. The post-push review itself is a public legal record required after the completed push. Publishing this review to the public super-repo is a further public push and must occur only under a later [2026] REALM-SI 7 release warrant. Filing the review locally and backing it up to the development remote satisfies the immediate judicial review step without creating an infinite same-push publication loop.

## Reasons

This push changed public-law machinery. It made and published the Agent Lawfulness Hooks Instrument, the Gazette Graph Database Instrument, and two amending instruments clarifying that the hook contract is agent-agnostic and that each agent must make good hook triggering on a best-efforts basis. It also created a public adapter record, a root agent contract, portable hook installation, Claude adapter binding, a deterministic bench-name scanner, Gazette graph lineage and validation artifacts, a superrepo private working area, redacted local-formation route notes, and conformance edits to bench names in PC17 to PC19.

That was not a routine documentation push. It therefore required both substantive legal authority for the content and a release warrant for the outward act. The substantive authority existed in the newly made statutory instruments and in the earlier court rulings cited above. The outward act was separately warranted under [2026] REALM-SI 7.

The release was scoped. The public remote was `https://github.com/wlilley93/vibe-justice-system.git`; the public ref was `refs/heads/cli-and-deterministic-citations`; the exact local SHA was `3f5776a8bce491eed41a9ab15e587c1ac626466f`; and the development backup was held at the same SHA before the public push.

The pre-push gate did what [2026] REALM-SI 7 expects it to do. It identified the public VJS remote as canonical, looked for an authorisation record, matched the scoped SHA and ref, and allowed the push only after the match. The public host then accepted the fast-forward.

The data-boundary issue was the central risk. The record contains no unredacted local evidence. Private working areas exist as tracked instructions plus ignore rules, while the actual private material remains ignored. The public sweep found no matches for the concrete local terms, local screenshot/path markers, or project/private identifiers checked before publication.

The Court also accepts the system-level concern raised by the Principal: the earlier hook design could have slipped through the cracks if treated as a Claude-only hook. The new instruments and adapter record improve that. They do not make every runtime automatically safe, but they convert the gap into an explicit best-efforts duty: every agent must use the best available trigger or substitute check in its own runtime, and delegable-workflow agents must separate materially independent work where useful.

No punitive or merits-adjudicating consequence arises from the hooks. They are routing and safety machinery. The Court remains the body that decides breach, validity, and remedy.

## Disposal

1. The public push of commit `3f5776a8bce491eed41a9ab15e587c1ac626466f` to `refs/heads/cli-and-deterministic-citations` in the public VJS super-repo was lawful.
2. The push matched the [2026] REALM-SI 7 release warrant.
3. The pre-push checks were adequate for the nature of the push.
4. The public-data boundary and repository-integrity chain held.
5. No immediate remediation is required.
6. The review is filed locally now. Its later public publication must itself be made under a fresh [2026] REALM-SI 7 release warrant.

The matter does not climb. No arguable conflict with Supreme Court authority appears on the face of this review.

## Appendix A - release warrant and refs reviewed

| Field | Value |
|---|---|
| Authorised outward act | `public-vjs-publish` |
| Authorised by | Sovereign Founder direction via Lexby, proceeding under [2026] REALM-SI 7 |
| Authorised at | `2026-06-07T20:46:14Z` |
| Public remote | `https://github.com/wlilley93/vibe-justice-system.git` |
| Public ref | `refs/heads/cli-and-deterministic-citations` |
| Local SHA | `3f5776a8bce491eed41a9ab15e587c1ac626466f` |
| Previous public SHA | `2b68bc196434fd5762a1a37811792d28834b2516` |
| Public post-push SHA | `3f5776a8bce491eed41a9ab15e587c1ac626466f` |
| Development backup | `origin/cli-and-deterministic-citations` at `3f5776a8bce491eed41a9ab15e587c1ac626466f` |

## Appendix B - checks recorded

- `npm run build` in `Judicature/law-reports`: passed.
- `npm test` in `Executive/cli`: passed.
- `node Executive/cli/bin/cdd.js check-citator`: passed.
- `node Executive/cli/bin/cdd.js check-bench-names`: passed.
- Gazette graph validation: passed with no malformed edge errors and no isolated node lacking a no-edge declaration.
- `git diff --check`: passed.
- Public private-data scan for local/private identifiers and path/screenshot markers: no public hits.
- Public pre-push gate: accepted the scoped [2026] REALM-SI 7 release warrant and allowed the fast-forward public push.
