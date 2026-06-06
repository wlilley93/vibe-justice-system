# The Standing Committee on the Laws of the Realm

The four-member drafting committee of the Legislature. It contributes to **every bill** of the realm: each member
brings a distinct, standing slant, and a bill is reported only once all four have been heard and reconciled. The
committee drafts; the **Sovereign Founder** enacts.

## The four members (each a standing slant)

| Member | Slant | The question they always ask | Characteristic move |
|---|---|---|---|
| **Counsel Aldous** | **Restraint / Minimalist** | "Does this clause earn its place, or is it bloat?" | Strikes anything not strictly necessary; repeal before adding; the shortest workable statute. |
| **Counsel Verity** | **Codifier / Completeness** | "Where is this defined, and what does it collide with?" | Comprehensive structure, consistent definitions, sound cross-references, no gaps, consolidation. |
| **Counsel Marlowe** | **Guardrail / Rights** | "Who is protected against this power, and what is the review route?" | Rights, due process, judicial independence, checks on executive and Founder overreach, protection of agents and memory. |
| **Counsel Drummond** | **Pragmatist / Operability** | "What happens on a real turn, and who actually enforces this?" | Enforceability, token/compute cost, real agent workflows, no ceremony; tests every clause against operation. |

The four slants are deliberately in tension: Aldous prunes what Verity would add; Marlowe constrains what Drummond
would streamline. A bill that survives all four is lean, complete, safe, and operable.

## The legislative procedure (declared topic -> drafting -> vote -> Royal Assent)

Every bill follows the same course. **Royal Assent is required for every Act, but it arrives only after the drafting
rounds are complete** - the Sovereign is not troubled with a bill the committee has not yet settled.

```
1. DECLARED TOPIC      the bill's subject is declared (its long title + intended function)
2. DRAFTING ROUND 1    the four members memo the topic; the Clerk-Drafter reconciles a draft Act
3. VOTE                each of the four members votes aye or nay on the draft
        |                                                         |
     passes (>= 3 ayes)                                    DEADLOCK (2-2, or < 3 ayes)
        |                                                         |
        v                                                4. DRAFTING ROUND 2
5. ROYAL ASSENT  <-------------------------------------  the members revise to break the deadlock,
   the bill comes to the Sovereign, who assents              the Clerk re-drafts, then it proceeds
   (or, on a flagged bill, consults before assenting)        to Royal Assent
```

- A bill **passes** on a clear majority of the four members (at least 3 ayes). A **deadlock** (2-2, or no clear
  majority) sends the bill back for **one** further drafting round addressing the dissents; after the second round
  it proceeds to Royal Assent. The second round is the tie-breaker, not an endless loop.
- **Royal Assent** is the Sovereign Founder's act and is required for every Act. A bill that carries a Privy Council
  referral, a Supreme Court note, or a pending Sovereign consultation is flagged so the Founder sees exactly what to
  weigh before assenting.
- Every bill carries a **Committee note** (each member's stance and where they divided) and a **vote record**.

## How the committee works on a bill

1. Each member files a short **memo** on the declared topic (their slant's position + concerns).
2. The memos are **reconciled** by the Clerk-Drafter into a draft Act, with a **Committee note**.
3. The members **vote**; on a deadlock the bill returns for a second drafting round.
4. The settled bill is presented to the Founder for **Royal Assent**, carrying its Committee note and vote record.

## The escalation ladder (resolving doubt and tension)

The committee does not resolve constitutional questions itself. It escalates:

1. **Committee doubt -> the Privy Council.** Where the committee is unsure whether a bill coheres with the
   constitutional settlement or the case law, it **refers a question to the Privy Council** (the realm's
   constitutional court of first instance) for guidance, and waits for the answer before reporting the bill.
2. **Constitutional tension -> the Supreme Court.** Where a bill raises genuine constitutional tension (a clash
   with the Acts of Union, a separation-of-powers strain, an entrenched-article question), the **Supreme Court may
   rule** on it. The Privy Council leapfrogs such a matter straight to the Supreme Court.
3. **The Sovereign is consulted -> overturn or affirm.** The **Founder** may then be consulted to **overturn or
   affirm** the Supreme Court's ruling. The Sovereign is the final word; a Supreme Court ruling on constitutional
   tension stands unless the Founder, on consultation, overturns it.

A bill that triggered a Privy Council referral, a Supreme Court ruling, or a pending Sovereign consultation carries
that fact on its face, so the record shows exactly how each doubt was resolved.

## Future automation

This committee is the human-shaped seed of an automated legislature. Once the first body of Acts exists, the four
slants become four standing review agents; the escalation ladder (Privy Council -> Supreme Court -> Sovereign) is
the safety rail that lets drafting be automated without letting agents become sovereign.

**UP:** [`../README.md`](../README.md). **Bills:** [`../bills/ORDER-PAPER.md`](../bills/ORDER-PAPER.md).
