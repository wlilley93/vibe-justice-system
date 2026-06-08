# Policy Briefing: Agent Loop, Harness, and Determinism

**Type:** Ministry policy briefing (MoJ policy arm -> Privy Council question definition -> Standing Committee drafting)
**To:** the Ministry of Justice, governance ministry of the Judicature
**From:** the Sovereign Founder, by direction to Lexby as registrar/engineer
**Subject:** Bill 31 correction and consolidation of scattered agent-loop law
**Date:** 2026-06-08
**Status:** referred to the Privy Council for question definition and to the Standing Committee for research and drafting; not itself law, judgment, or order

## 1. Commission

The Sovereign Founder has clarified that a central goal of Bill 31 is to make the agent loop explicit and to surface scattered law governing agent harnesses and deterministic workflow.

The requested output is the **Agent Loop (Harness and Determinism) Act 2026**. The earlier "Consolidation and Clarity Framework" working title is too general and risks concealing the very loop discipline the Founder wants enforced.

The Founder gives assent conditionally: the instrument may clarify, consolidate, source-map, supersede a working title, or retire material only as a source-equivalent consolidation consequence, but must not add a new substantive rule under cover of clarity.

## 2. Question For The Privy Council

The Ministry asks the Privy Council to define the question for the drafting exercise as follows:

> What is the lawful source-equivalent form of Bill 31 as the Agent Loop (Harness and Determinism) Act 2026, consolidating the scattered law that requires agents to retrieve current law, build a preloop plan with legal evidence, use deterministic CLI routes where available, identify likely court and subagent routes in prehook questions, answer or act within authority, run a posthook validity review, and ask whether Lexby should self-refer unless an exempt route is recorded?

The proposed answer is procedural and conservative: [2026] REALM-SI 8, [2026] REALM-SI 10, and [2026] REALM-SI 11 already supply the agent hook contract. Bill 31 should consolidate and surface that contract, require cdd-first legal retrieval where available, and preserve exemptions without creating a new court, sanction, or source of law.

## 3. Record Retrieved Through CLI

The Ministry used the `cdd` CLI as the retrieval spine for the controlling hook law:

| CLI retrieval | Result |
|---|---|
| `cdd law get "[2026] REALM-SI 8"` | Agent Lawfulness Hooks Instrument 2026, status `made`, source path returned |
| `cdd law get "[2026] REALM-SI 10"` | Agent-agnostic workflow amendment, status `made`, source path returned |
| `cdd law get "[2026] REALM-SI 11"` | Best-efforts trigger amendment, status `made`, source path returned |
| `cdd law search "agent lawfulness hooks pre-answer post-answer subagents"` | SI 8, SI 10, SI 11 surfaced as the leading sources |
| `cdd law search "deterministic CLI cdd required route public release hooks"` | [2026] REALM-PC 20, [2026] REALM-PC 16, SI 8, SI 7, SI 10, SI 11 surfaced as relevant deterministic route sources |

Manual source reading followed the CLI retrieval. The CLI output is retrieval evidence only, not legal force.

## 4. Full Record Reviewed

| Source group | Files reviewed | Public-law status |
|---|---|---|
| Acts 1-30 | `Legislature/legislature/bills/01-*.md` through `30-*.md` | enacted Acts of the Realm |
| Statutory instruments 1-11 | `Legislature/statutes/instruments/2026-realm-si-1-*.md` through `2026-realm-si-11-*.md` | made subordinate instruments, with commencement governed by their own objection-window clauses |
| Central judgments | `Judicature/.justice/judgments/privy-council/`, `court-of-appeal/`, `supreme-court/` | committed central case law, with status in `Judicature/.justice/INDEX.md` |
| High Court Division material | central search found no committed High Court Division judgment file as of this briefing | machinery exists in Acts and citation map, but no central High Court judgment is presently on the record |
| Community Record | `Judicature/community/caselaw/2026/` | persuasive/public community material, including full text copies of Supreme Court judgments used for access and context |
| 2026-06-07 and 2026-06-08 policy and requests | MoJ policy, court requests, and committee referrals on the record | policy, request, or referral material unless separately made law or judgment |

## 5. Preliminary Findings

The record already contains the agent loop, but it is scattered and therefore easy for an agent to miss.

The main themes needing consolidation are:

1. preloop discipline: previous-answer review, Lexby invocation, cdd-first retrieval, and a legal-evidence plan before governed load-bearing work;
2. prehook forecast: any prehook question should identify likely court route and subagent or substitute-check use ahead of time;
3. deterministic route: where `cdd` or another deterministic CLI route exists, the agent should use it before ad hoc handling unless an exemption is recorded;
4. answer or act: the agent then answers or acts within the authority, limits, route, and public/private boundary identified;
5. posthook discipline: after the answer or act, the agent reviews validity and asks whether Lexby should self-refer unless a posthook exemption is recorded;
6. source-of-force discipline: Lexby, subagents, adapters, CLI output, and projections are evidence or workflow, not law;
7. public/private boundary: public records carry system data only; operational facts stay private;
8. exemptions: urgent, trivial, private, unavailable, or differently routed cases may depart from the default loop only by a recorded exempt route.

## 6. Holes And Drift To Record, Not Smuggle

The following are not to be silently cured by the Agent Loop Act:

- no High Court Division judgment file was located on the central record, although High Court machinery exists prospectively;
- runtime-specific hook adapters may not yet trigger automatically in every agent product;
- cdd coverage is deterministic where commands exist, but not every possible workflow has a first-class command;
- SIs recorded as `made` must not be treated as already `in-force` unless their commencement condition has occurred or is separately recorded;
- the former Bill 31 working title must be retired as a presentation label only, not as a hidden change in legal meaning.

## 7. Recommended Route

1. File this corrected policy briefing as the Ministry question-definition paper.
2. File a public Privy Council reference defining the legal question and limits of the Agent Loop Act.
3. Place a Standing Committee research record on the committee file.
4. Draft and enact Bill 31 as the Agent Loop (Harness and Determinism) Act 2026, with conditional assent limited to no-new-law consolidation.
5. Render the corrected Act, rebuild Gazette projections, and run deterministic checks through `cdd`.

**UP:** [`../README.md`](../README.md)
