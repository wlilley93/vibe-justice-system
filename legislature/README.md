# The Legislature of the Realm

The realm's law-making organ, at the Sovereign level (Agent Universe). Renamed from the former `parliament/`:
**there is no Parliament.** The **Sovereign Founder** (the Principal) is both sovereign and legislature.
Law is enacted by the Founder; it is *drafted* by the **Standing Committee on the Laws of the Realm**
(four members, distinct slants) and, in future, by **automated legislative agents** under delegated authority.

## What lives here

```
legislature/
├── committee/        the Standing Committee (4 members) that drafts the bills - see CHARTER.md
├── bills/            bills in progress; ORDER-PAPER.md lists the first 25 and their status
└── (passed Acts are recorded in ../statutes/ and are the supreme enacted law of the realm)
```

## The hierarchy of law (as it now stands)

The **Acts** drafted here are the realm's **supreme enacted statute**. They post-date and **supersede the founding
settlement** - the doctrine formerly styled SPEC-LAW, now reframed as **case law** (`CASE-LAW.md`: the duty-of-care
doctrine and the precedent the courts built). On the model the Founder settled:

```
Acts of Union 2026                 the constitutional root (supreme)
   -> Acts of the Realm            enacted statute, supreme over case law
        -> Case law (CASE-LAW.md + the REALM/LEXBY precedent)   subordinate to statute
             -> delegated rules, codes, practice directions
```

Statute beats case law where they conflict; the courts interpret statute and develop case law in the gaps.

> **Constitutional caveat (raised by the Standing Committee on Privy Council guidance, 2026).** This
> Acts-over-case-law tier is **intended law, not yet law in force**, until it is brought into being by an
> **express amendment** of the entrenched CASE-LAW ss. 1, 2 and 9 (s. 9 forbids a competing statute book; ss. 1-2
> vest one legislative power in the Founder). The **Acts of Union 2026** (Bill 1) is the instrument that effects
> that express amendment; the hierarchy above takes force on its Royal Assent (or on a Supreme Court ruling the
> Sovereign affirms). Pending that, the entrenched case law governs. This is a matter for **Sovereign consultation**.

## The drafting ladder (how a bill is made, and how doubt is resolved)

1. **The Standing Committee drafts.** Four members, each with a different slant (restraint, codification,
   guardrails, operability), contribute to every bill and record their positions in a Committee note.
2. **Doubt -> the Privy Council.** Where the committee is unsure whether a bill coheres with the constitutional
   settlement or the case law, it refers a question to the **Privy Council** (the realm's constitutional court of
   first instance) for guidance before reporting the bill.
3. **Constitutional tension -> the Supreme Court.** Where a bill raises genuine constitutional tension, the
   **Supreme Court** may rule on it.
4. **The Sovereign is consulted.** The **Founder** may then be consulted to **overturn or affirm** the Supreme
   Court's ruling. The Sovereign is the final word (lack of democratic electors; the reserved authority of s. 2).

## Future automation

The Legislature is designed to be automated: delegated legislative agents will propose, codify, consolidate, and
revise subordinate law under the Delegated Legislative Authority Act, never able to amend the Acts of Union or to
become sovereign. The sovereign remains the constitutional system itself: Acts of Union -> Founder -> Legislature
-> Courts -> Delegated Institutions -> Agents.

**UP:** the realm - [`../README.md`](../README.md). **Committee:** [`committee/CHARTER.md`](committee/CHARTER.md).
**Bills:** [`bills/ORDER-PAPER.md`](bills/ORDER-PAPER.md). **Enacting archive:** [`../statutes/`](../statutes/).
