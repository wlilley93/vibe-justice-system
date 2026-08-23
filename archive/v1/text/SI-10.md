# The Agent Lawfulness Hooks (Agent-Agnostic Workflow) Amendment Instrument 2026

**Citation:** [2026] REALM-SI 10 (under Bill 5, Bill 13, Bill 18, Bill 21, Bill 22, Bill 26, Bill 27 and Bill 29)

**Amends:** [2026] REALM-SI 8

**Made by:** the Standing Committee of the Legislature, in exercise of the operational-detail statutory-instrument power conferred by section 18 of the Ministries and Offices Act 2026 (Bill 5) as inserted by the Statutory Instruments (Framework) Act 2026 (Bill 26); the parent authority being the Ministry of Justice for court-routing and public-law process, read with the Enforcement, Sanctions and Compliance Act 2026 (Bill 13), the Autonomous Execution and Safety Act 2026 (Bill 18), the Security and Integrity Act 2026 (Bill 21), the Data, Disclosure and Confidentiality Act 2026 (Bill 22), the VJS (Constitution and Machinery) Act 2026 (Bill 27), and the Ministerial Policy Arm Act 2026 (Bill 29).

**Status:** made

**Procedure:** negative (Bill 14 s.14 objection window)

**Made:** 2026-06-07

**Coming into force:** on expiry of the Bill 14 s.14 objection window without valid objection.

---

## Recitals

The Agent Lawfulness Hooks Instrument 2026 ([2026] REALM-SI 8) states a generic duty for agents carrying out governed load-bearing work.

The public hook must not become a product-specific Claude hook, a Codex-only instruction file, or a runtime-specific convention. A product-specific hook is only an adapter for the public agent-agnostic contract.

The Principal has directed that the hook must support any agent that supports delegable workflows. The Standing Committee therefore makes the following clarifying amendment.

---

## PART 1 - AMENDMENT

### 1. Agent-agnostic hook contract

After section 1 of [2026] REALM-SI 8, insert:

> ### 1A. Agent-agnostic hook contract
>
> (1) The agent lawfulness hooks are an agent-agnostic workflow contract.
>
> (2) A runtime-specific hook, settings file, prompt wrapper, CLI adapter, plugin, skill, or instruction document is only an implementation adapter for that contract.
>
> (3) No implementation adapter may narrow the contract to a single model provider, product, shell, IDE, chat interface, or agent framework.
>
> (4) Where an agent runtime supports delegable workflows, subagents, reviewers, workers, background tasks, separated tools, or an equivalent mechanism for splitting work, the adapter must expose or instruct that mechanism for governed load-bearing work where separation would materially improve lawfulness, independence, verification, or record integrity.
>
> (5) Where an agent runtime does not support delegable workflows, the adapter must state the absence of that capability and fall back to deterministic retrieval, citator checking, explicit role-labelled review, or referral to the competent organ.
>
> (6) A runtime-specific adapter may add local ergonomics, but it must preserve the public contract: Lexby invocation, retrieval-first record checking, role separation where available, pre-answer law research, post-answer validity review, previous-answer review, data-boundary checking, and source-of-authority recording.

### 2. Adapter implementation record

After section 12 of [2026] REALM-SI 8, insert:

> ### 12A. Adapter implementation record
>
> (1) The Ministry of Business, Engineering and Skills must maintain a public adapter record identifying, for each supported agent runtime or workflow class:
>
> - the adapter surface used;
> - whether the adapter is automatic, advisory, manual, or unavailable;
> - whether the runtime supports delegable workflows;
> - the substitute check used where delegation is unavailable;
> - the public command, instruction file, or configuration entry by which the adapter can be verified; and
> - any private operational registry that holds concrete prompts, thresholds, credentials, tenant facts, or volatile implementation details.
>
> (2) The adapter record is system data only.
>
> (3) The adapter record does not need to expose private prompt text, credentials, hostnames, logs, model configuration, tenant lists, or local repository facts.

---

## PART 2 - SAVINGS AND LIMITS

### 3. Existing adapters saved

(1) Existing Claude-style hook bindings are saved as one adapter for the agent-agnostic contract.

(2) Existing git gates are saved as deterministic repository adapters for the parts of the contract that concern commit and push acts.

(3) A Codex instruction file, wrapper, or future runtime hook is a separate adapter. Its absence does not narrow the contract.

### 4. No new adjudication

This Instrument does not create a court, sanction, validity score, or automatic invalidity rule. Delegable workflow separation is a method of review and verification, not a decision on legal force.

---

## PART 3 - COMMENCEMENT

### 5. Commencement

This Instrument comes into force on expiry of the Bill 14 s.14 objection window without valid objection.

---

## Made by the Standing Committee

**Counsel Aldous (Restraint):** The public law should say the hook contract, not bless one product's event names. This amendment keeps the rule portable.

**Counsel Verity (Codifier):** The adapter record solves the evidence problem. Each runtime shows how it binds, whether it delegates, and what substitute check is used.

**Counsel Marlowe (Guardrail):** Delegation is not adjudication. It helps keep Lexby out of the bench's chair, but the competent organ still supplies legal force.

**Counsel Drummond (Pragmatist):** Claude can use Claude hooks, Codex can use instructions or a future hook surface, and other agents can use their own adapters. The user sees one VJS contract.

**Clerk's Note:** Made 2026-06-07. Commencement on lapse of the Bill 14 s.14 objection window without valid objection.

---

**END OF INSTRUMENT**
