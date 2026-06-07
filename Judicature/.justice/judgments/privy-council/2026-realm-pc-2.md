---
citation_id: "[2026] REALM-PC 2"
uuid: 3f7a2b91-d4e8-4c10-a1f5-8e6b0c93d517
tier: privy-council
judge: Sumberly J
kind: request_for_ruling
date: 2026-06-05
status: good-law
---

# [2026] REALM-PC 2

**FIRST INSTANCE COURT**
**Before: Sumberly J**
**5 June 2026**

---

## Matter

Is the Vibe Justice System (VJS), as currently built at commit ed156a8 on wlilley93/vibe-justice-system (2026-06-05), ready for public release and active outreach - specifically, to be shared with the developer community via X (Twitter), Hacker News, and direct contact with high-amplification figures such as @karpathy?

---

## Ratio decidendi (binding)

VJS at commit ed156a8 is ready for public release and active outreach under the standard of reasonable skill and care (s. 4, s. 5), subject to one necessary precondition: the README must be amended to add an explicit, prominently placed known-limitations section disclosing the three material gaps (absent submit commands as standalone executables, absent deterministic citation numbering, absent CLI and packaging) before any outreach act is performed.

The governing standard for public outreach is materially distinct from the alpha-release standard applied in [2026] REALM-PC 1: outreach invites a mass, unsolicited, technically varied audience and engages the credibility of the system's authors, not merely the curiosity of invited reviewers. Under that heightened standard, coherence and honest disclosure are necessary and sufficient; outreach is not contingent on v1 feature completeness. An alpha badge, accurate disclaimers, and a clearly labelled limitations section together discharge the duty of reasonable skill and care for this act. The precondition is remediable in a single editing pass and does not itself constitute a breach of any prior duty.

---

## Obiter dicta (persuasive)

**On sequencing.** This court strongly recommends that outreach be directed first to Hacker News (a technically literate, high-context audience accustomed to evaluating alpha-stage tooling on its merits) rather than to high-amplification individuals before that audience reaction is known. An uninstructed high-amplification share amplifies both praise and adverse criticism symmetrically and without moderation. A Hacker News landing first provides a calibrating signal about how a developer audience actually encounters the system, which is of substantial practical value before seeking wider amplification. The court does not order this sequencing - it is beyond the scope of the ruling - but records it as a consideration of reasonable professional judgment.

**On the README install prompt.** The court read the current install prompt with care. The prompt directs the user's AI to fetch plugin/CLAUDE.md from GitHub and append it to the local CLAUDE.md, and to fetch CASE-LAW.md and save it as .justice/CASE-LAW.md. This approach is workable for a technically literate user but presupposes that the AI agent can reliably execute a multi-step file-fetch-and-write sequence against an external GitHub URL without error. In practice, LLM agents vary in their ability to do this faithfully in a single turn. The README does not currently flag this execution variability. This is not a breach and not a blocker for release, but before outreach reaches a less-technical audience, a fallback install path - for example, a manual two-file copy - would be a prudent addition.

**On the prior alpha ruling.** [2026] REALM-PC 1 (Bowan J) recorded a forward duty arising from the citation-numbering gap (s. 8), and identified submit-request-to-court and deterministic citation numbering as necessary conditions for v1. Nothing in this ruling disturbs that finding. This ruling does not adjudicate v1 readiness; it adjudges only whether the current state of the system is fit, with appropriate disclosure, for the distinct act of active public outreach. The v1 closure set identified in [2026] REALM-PC 1 remains unaffected and continues to bind.

---

## Remedy

No remediation order is made. This is a request_for_ruling, not a breach proceeding. The court imposes one precondition on the outreach act: the README must receive a disclosure pass adding an explicit known-limitations section before any outreach message is published or sent. Outreach conducted without compliance with this precondition would constitute a failure of disclosure on a material point and would, on a fresh filing, be liable to be found in breach of the duty of care under s. 4 and s. 5.

---

## Lexby translation

**In plain English:** The court was asked whether VJS is ready to be shared publicly - posted on Hacker News, sent to Karpathy, put out into the world. Standing was found. The court said yes, ready, but with one firm condition: the README must have an honest known-limitations section naming the three gaps before a single message goes out. The court drew a line between releasing an alpha (the earlier ruling) and doing public outreach: outreach puts the authors' credibility on the line with strangers, so honest disclosure is required. Coherent, honestly labelled work with accurate disclaimers is enough; it does not have to be feature-complete.

**What it means in practice:** Add the known-limitations section to the README. Once done, outreach can proceed. The court also strongly (non-bindingly) recommends posting to Hacker News before reaching out to high-amplification individuals like Karpathy, so you get calibrated developer feedback before the megaphone is turned up.

**Can it be appealed?** Yes. Permission to appeal to the Court of Appeal may be sought on an arguable point of law or on a conflict with binding precedent, per VPR 3.
