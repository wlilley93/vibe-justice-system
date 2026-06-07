# Redacted Sweep Report: Superrepo Data Boundary

**Date:** 2026-06-07  
**Owner:** Ministry of Justice policy arm  
**Status:** public/system-data sweep report; no secret values reproduced

## Scope

The sweep covered the central public governance trees:

- `Constitution/`
- `Judicature/`
- `Legislature/`
- root public guidance files touched by the private-directory work

The sweep excluded:

- `_private/`
- scoped `_private/` subtrees
- executive/local operational trees
- `.git/`

## Checks Run

The sweep checked for:

- direct project-name references from the local formation matter;
- local machine paths and screenshot references;
- email-address-shaped strings;
- obvious secret/token/password assignments;
- public URLs and hostnames requiring classification;
- central request files containing unredacted local facts;
- git tracking of private request material.

## Findings

| Category | Public result | Classification | Action |
|---|---|---|---|
| Local formation project name | No word-boundary hit in the public central files checked | pass | no public edit required |
| Central request files | Public central request record now contains only redacted route/reference notes | remedied | unredacted material stays private/local |
| Scoped private areas | Ignored by git | pass | retain ignore rules |
| Local `/tmp` paths | Found only renderer/workflow examples using temporary output paths | lawful system-data / tooling examples | no move required |
| Email-shaped strings | Found only public co-author convention examples | lawful system-data | no move required |
| Secret/token/password assignment patterns | No actionable public secret assignment found in authored central governance files | pass | no public edit required |
| Public URLs | Found public law-site, package registry, localhost, and renderer dependency URLs | lawful system-data / dependency metadata | no move required |
| Vendor bundle matches | Generated/vendor code contains generic library terms | dependency artifact | no public data-boundary issue identified |

## Actions Completed

1. Created the root superrepo private working area at `_private/` with tracked instructions and ignored contents.
2. Added scoped ignore rules for private central request, policy, and legislative-referral working papers.
3. Moved the unredacted local formation central request material out of the public request record.
4. Added public redacted route and Supreme Court reference notes.
5. Added SI and MoJ policy wording requiring private implementation facts to remain in private registries or the superrepo private working area.

## Residual Handling

Future sweep outputs that include concrete hits must be written to the private working area or the relevant local jurisdiction evidence record. Public reports should contain only classifications, counts, paths where the path is lawful system-data, and redacted summaries.

This report is not a judgment, order, or instrument of law.
