# The Agent Lawfulness Hooks (Best-Efforts Trigger) Amendment Instrument 2026

**Citation:** [2026] REALM-SI 11 (under Bill 5, Bill 13, Bill 18, Bill 21, Bill 22, Bill 26, Bill 27 and Bill 29)

**Amends:** [2026] REALM-SI 8 and [2026] REALM-SI 10

**Made by:** the Standing Committee of the Legislature, in exercise of the operational-detail statutory-instrument power conferred by section 18 of the Ministries and Offices Act 2026 (Bill 5) as inserted by the Statutory Instruments (Framework) Act 2026 (Bill 26); the parent authority being the Ministry of Justice for court-routing and public-law process, read with the Enforcement, Sanctions and Compliance Act 2026 (Bill 13), the Autonomous Execution and Safety Act 2026 (Bill 18), the Security and Integrity Act 2026 (Bill 21), the Data, Disclosure and Confidentiality Act 2026 (Bill 22), the VJS (Constitution and Machinery) Act 2026 (Bill 27), and the Ministerial Policy Arm Act 2026 (Bill 29).

**Status:** made

**Procedure:** negative (Bill 14 s.14 objection window)

**Made:** 2026-06-07

**Coming into force:** on expiry of the Bill 14 s.14 objection window without valid objection.

---

## Recitals

[2026] REALM-SI 10 clarifies that the agent lawfulness hooks are an agent-agnostic workflow contract and that product-specific bindings are adapters only.

That clarification is incomplete unless each agent bears a positive best-efforts duty to make the hook capable of triggering in the runtime actually being used.

The duty must be realistic. An agent cannot invent a hook surface the runtime does not expose, but it can use the best available adapter, instruction file, wrapper, script, delegated workflow, deterministic check, or explicit manual checklist and record the limitation.

The Standing Committee therefore makes the following clarifying amendment.

---

## PART 1 - AMENDMENT

### 1. Best-efforts trigger duty

After section 1A of [2026] REALM-SI 8, as inserted by [2026] REALM-SI 10, insert:

> ### 1B. Best-efforts duty to make the hooks trigger
>
> (1) Every agent undertaking governed load-bearing work must make good, on a best-efforts basis, the ability for the agent lawfulness hooks to trigger in the runtime, interface, repository, or workflow it is actually using.
>
> (2) The duty in subsection (1) includes, proportionately to the work and to the runtime's technical capability:
>
> - checking whether the repository provides a VJS adapter, hook directory, instruction file, wrapper, plugin, skill, or command surface;
> - enabling or using the available adapter where it can be done without exposing private facts or defeating urgency;
> - using delegable workflows, subagents, reviewers, workers, background tasks, separated tools, or equivalent independent-check mechanisms where the runtime supports them and the work calls for separation;
> - running deterministic checks supplied by the repository where they bear on the proposed act;
> - using an explicit manual hook checklist where automatic triggering is unavailable; and
> - recording, briefly, any material inability to make an automatic hook trigger.
>
> (3) A runtime-specific absence of automatic hooks is not a defence to ignoring the workflow contract. The agent must use the best available substitute check.
>
> (4) A technical inability to trigger an automatic hook does not itself create breach, invalidity, sanction, or punishment. Breach and remedy remain for the single judiciary, and remedy remains restorative only.
>
> (5) The duty is strongest for public-record changes, legal routing, court or legislative filings, data-boundary decisions, irreversible external acts, security-sensitive acts, production operations, and any act that may expose private facts.

### 2. Adapter record to state trigger quality

In section 12A of [2026] REALM-SI 8, as inserted by [2026] REALM-SI 10, after subsection (1), insert:

> (1A) The adapter record must also state the trigger quality for each runtime or workflow class, using a plain public description such as automatic, advisory, manual, wrapper-mediated, deterministic-only, unavailable, or unknown.
>
> (1B) Where trigger quality is unavailable or unknown, the record must state the current best-efforts substitute check.

### 3. Agent obligation saved alongside MBES obligation

Nothing in [2026] REALM-SI 10 or this Instrument makes the best-efforts trigger duty solely an MBES engineering duty. MBES owns implementation surfaces, but each agent remains responsible for using or making good the available hook route during governed load-bearing work.

---

## PART 2 - LIMITS

### 4. No impossible duty

(1) The duty imposed by this Instrument is a best-efforts duty, not an impossible-duty rule.

(2) An agent is not required to fabricate a runtime capability, bypass security controls, expose secrets, or alter private operational facts in the public record.

(3) Where the hook cannot be made automatic, the agent must use the next-best route: manual checklist, deterministic check, delegable workflow if available, explicit role-labelled review, or referral.

### 5. No adjudication or sanction

This Instrument creates no new court, sanction, punishment, validity score, or automatic invalidity rule.

---

## PART 3 - COMMENCEMENT

### 6. Commencement

This Instrument comes into force on expiry of the Bill 14 s.14 objection window without valid objection.

---

## Made by the Standing Committee

**Counsel Aldous (Restraint):** Best efforts is the right standard. It creates a real duty without pretending every runtime exposes the same hook surface.

**Counsel Verity (Codifier):** The adapter record now has to say trigger quality. That is the public answer to "why did I not see the hook fire?"

**Counsel Marlowe (Guardrail):** Technical absence is not a loophole. The substitute check must be used and recorded, but breach and remedy remain judicial and restorative.

**Counsel Drummond (Pragmatist):** Agents can actually comply: check the adapter, enable what exists, delegate where possible, run deterministic checks, and say what could not be triggered.

**Clerk's Note:** Made 2026-06-07. Commencement on lapse of the Bill 14 s.14 objection window without valid objection.

---

**END OF INSTRUMENT**
