---
citation_id: "[2026] REALM-PC 14"
tier: privy-council
kind: request_for_ruling
status: good-law
per_incuriam: false
date: 2026-06-07
panel: ["Marsden J", "Coade J", "Sumberly J"]
seised_by: "Sovereign Founder reference (the Repos House / County Court formation question, reserved by Bill 27 s.15(3))"
cause_title: "In the matter of County Court formation and the .gitignore - the Repos House (Registrar) reference"
registrar_note: "Authored by the bench (Marsden J for the Court, Coade J and Sumberly J concurring); reduced to the filed record by Lexby as s.18(4) registrar, the decision pre-existing the prose ([2026] REALM-SC 8)."
---

# [2026] REALM-PC 14

| Field | Value |
|-------|-------|
| **Citation** | [2026] REALM-PC 14 |
| **Tier** | Privy Council (constitutional first instance, bench of three) |
| **Before** | Marsden J (judgment of the Court), Coade J, Sumberly J |
| **Kind** | Reference on County Court formation and the .gitignore (Request for Ruling) |
| **Status** | good-law |
| **Cites** | CASE-LAW s. 1, s. 6, s. 11(a), s. 14, s. 19(5), s. 21(4), s. 22(1)/(4); Bill 27 ss. 3, 7, 8(3)-(4), 12, 14(2)-(3), 15(3); Bill 22 ss. 9, 13, 16(3); Bill 20 ss. 7(1), 10, 11(2); distinguishes [2026] REALM-SC 6, [2026] REALM-SC 7; applies [2026] REALM-PC 4 |

> Bench of three at constitutional first instance, on the Founder's reference reserved by Bill 27 s.15(3):
> is a repository validly formed as a County Court (a local first-instance hearing-centre of the one
> judiciary, CASE-LAW s.22) only if it carries a .gitignore? A symmetric two-sided researched record was
> received (CASE-LAW s.19(1)). Unanimous (3-0). The bench authored this decision; Lexby filed it as the
> s.18(4) registrar. The ruling corrects the Founder's intuition: the .gitignore is evidence, not the
> constitutive act.

## The question

Is a repository validly formed and constituted as a County Court (a local first-instance hearing-centre of the
one judiciary, CASE-LAW s.22(1)) only if it carries a .gitignore? Is the .gitignore the constitutive act of
repo-incorporation, a necessary condition, a sufficient condition, evidence, or none of these? What is the
formation rule for a jurisdiction-repo, consistent with the system-data-only rule (Bill 27), the
confidentiality regime (Bill 22), and the certification regime (Bill 20)? (Reserved to this reference by
Bill 27 s.15(3).)

## Ratio (binding, realm-wide)

1. The constitutive rule for a County Court is a SUBSTANCE rule, not a file-presence rule. A repository is validly formed and constituted as a local first-instance hearing-centre of the one judiciary (CASE-LAW s. 22(1)) when, and only when, three conjunctive conditions hold: (a) it is a genuine jurisdiction-repo of project work within VJS subject-matter jurisdiction (s. 14) and is seated as a local seat of the one judiciary applying the one CASE-LAW, one VPR, one bench constitution (s. 10, s. 18), recording jurisdiction-local precedent only and taking the CC-<repo> handle on the single citator (s. 22(1), (4)); (b) its TRACKED public record holds system data only and carries no personal or operational FACTS (Bill 27 s. 7; Bill 22); and (c) it conforms on the Bill 20 integrity chain (the s. 19(5) gate holds; trust status is the deterministic, zero-token output of gate-plus-git, not QUARANTINED). These three are the realm's certificate of incorporation for a County Court: substance, seat, and conformance, recorded on the deterministic chain.

2. A .gitignore is NOT the constitutive act of repo-incorporation. It is the ORDINARY, presumptive, gate-encouraged EVIDENCE and MECHANISM of the condition (b) boundary, and only where the repo in fact holds private data behind it. Bill 27 s. 8(4) itself fixes the file's legal character as evidential, not operative: a repository without a .gitignore is 'presumed to have no private data'. A presumption is drawn FROM a sign; it is not the operative act that does the legal work. By the Act's own logic the .gitignore is the probative instrument, never the constitutive one.

3. A .gitignore is neither a necessary nor a sufficient condition of valid formation, because the file and the protected substance come apart in both directions. A repository that genuinely holds no private data (e.g. a hearing-centre sitting on nothing but the one CASE-LAW and its own public local precedent) needs no .gitignore to be validly formed: there is nothing for the file to ignore, and s. 8(4) places it in the 'presumed to have no private data and must be public' limb. A .gitignore that ignores nothing constitutes nothing: a no-op marker (a bare '# nothing' or a lone .DS_Store line) draws no boundary and does no protective work. Form is not substance.

4. CASE-LAW s. 22(4) is dispositive against the file-presence rule and voids it to the extent advanced. A local-court attribute is 'void to the extent it governs which law applies or how a matter progresses', and the animating principle, declared with s. 21(4), is that a local court is constituted by substance, never by a label or a token. A constitutive-.gitignore rule is form-over-substance at the precise level (local-court formation) the realm has outlawed: it would validate the empty-ignore repo (token present, boundary absent) and void the clean public hearing-centre (no token, no leak). It is void to that extent.

5. The Bill 27 s. 8(4) phrase 'a condition of the repository's validity as a governed data store' does not carry the file-presence rule into s. 22 formation. Section 8 governs separate PRIVATE repositories (s. 8(1)-(3)). A County Court is, by Bill 27 s. 14(3), a place whose local judgments stay in its own local .justice/ and whose private operational data lives in separate private repositories: it is not the s. 8 private-data-store. Importing a sufficiency-style phrase from the private-secret-store regime into judicial constitution is a category error. Where a County Court DOES hold private data, s. 8(3)-(4) applies in the ordinary evidential way: the .gitignore must be present and a repo holding private data without one is presumptively non-conformant on condition (b), curable by adding the file.

6. What the realm actually polices, and what the deterministic gate must check, is the SUBSTANCE: no tracked personal or operational facts crossing the trust boundary. Bill 27 s. 7(4) keys its fail-closed scan on the CONTENT of the commit, not the presence of a file; Bill 27 s. 14(2) enacts the cognate rule in terms ('the test is data-based, not court-based'). The .gitignore is the practical, presumptive means by which that boundary is ordinarily kept, and its absence where private data is present is a red flag the gate and the s. 12 conformance audit properly raise. It is the honest workhorse of the boundary, not the act of incorporation; the gate's role is to check the substance, never to fetishise the file.

7. The Bill 20 s. 10 no-conferral firewall is honoured and not engaged on either rule, but the substance rule sits more comfortably within it. Formation is a deterministic, zero-token fact established by gate-plus-git over the committed tree (Bill 20 s. 7(1)), conferred by no office. The authority of any record within a County Court remains, untouched, in the committed markdown under CASE-LAW s. 1 and [2026] REALM-PC 4. The remedy for a non-conformant repo is restorative, never punitive (Bill 20 s. 11(2); CASE-LAW s. 6): make the record good (remove the leaked fact; add the .gitignore where private data is held).

8. The rule is prospective and consistent with [2026] REALM-SC 7. It governs the formation of new County Courts and the forward conformance of existing ones; it voids no past CC-ACMECO ruling. The live County Court (CC-ACMECO) holds genuine private data (.env, build artefacts) behind a .gitignore that actually excludes them and is seated with a .justice/ node: it is the ordinary case, conformant on this rule with the file doing real boundary work.

## Reasons

IN THE PRIVY COUNCIL OF THE REALM (CONSTITUTIONAL FIRST INSTANCE). On the Repos House reference reserved by the VJS (Constitution and Machinery) Act 2026 (Bill 27), s. 15(3). In the matter of County Court formation and the .gitignore (the Registrar matter). Before Marsden J, Coade J and Sumberly J.

THE REFERENCE AND ITS POSTURE.
The Founder asks whether a repository is validly constituted as a County Court (a local first-instance hearing-centre of the one judiciary, CASE-LAW s. 22(1)) only if it carries a .gitignore at its root, and whether the .gitignore is the constitutive act of repo-incorporation, a necessary condition, a sufficient condition, evidence, or none of these. Bill 27 s. 15(3) expressly reserves the question to this reference and declines to codify it precisely. We confirm at the threshold that the matter is genuinely first-impression: REALM-SC 6 enacts s. 22 (a repo may be a County Court hearing-centre) but is silent on the constitutive condition; REALM-SC 7 addresses only the restorative, prospective conformance owed by a legacy repo already in being. No binding ratio is on all fours, so the s. 11(c) fast path does not dispose of the matter. The reference is properly heard.

THE COMPETING CASES.
The Claimant invites a bright-line rule: a root .gitignore is a necessary condition of valid formation and the realm's certificate of incorporation, chosen because it is the one repo-root artefact a machine can verify at the gate, zero-token, fail-closed, before any private fact can enter the record. The Defendant presses a substance rule: formation is constituted by (i) the system-data-only boundary, (ii) the s. 22 local seat, and (iii) Bill 20 conformance, with the .gitignore as the ordinary evidence and mechanism of (i), required only where the repo holds private data.

We prefer the Defendant's rule, for the reasons that follow, and answer the Founder's question precisely: the .gitignore is EVIDENCE and MECHANISM of the boundary, not the constitutive act; it is neither necessary nor sufficient; and the true constitutive rule is one of substance.

I. THE ACT FIXES THE .GITIGNORE AS EVIDENTIAL, NOT OPERATIVE.
The Claimant's strongest material is Bill 27 s. 8(4), which calls the .gitignore 'a condition of the repository's validity as a governed data store'. But the same subsection, read to its end, defeats the constitutive reading. It provides that 'a repository without a .gitignore is presumed to have no private data and must be public, or is non-conformant and must acquire a .gitignore before accepting personal or operational content'. The Act thus draws an inference FROM the file's presence or absence: the .gitignore is a sign from which the presence-or-absence of private data is PRESUMED. That is the signature of an evidential instrument, not an operative one. A constitutive act is not a thing from which a presumption is drawn; it is the thing that does the legal work. Incorporation is not evidence that a company exists; it is what makes the company exist. On the face of s. 8(4) the .gitignore is probative of a state of affairs (private data, or none). The Founder's intuition has correctly located the realm's ordinary mechanism for keeping the boundary; it has mislabelled its legal character.

This reading is reinforced by what the Act actually polices. Bill 27 s. 7(4) extends the pre-commit gate to scan the CONTENT of staged commits for secret-shaped tokens and to fail closed on detection. The enforcer keys on the substance crossing the trust boundary, not on the presence of a file; the Act nowhere makes the gate test for a .gitignore. And Bill 27 s. 14(2), governing the cognate question of what a court may publish, enacts the very rule we hold governs formation: 'The test is data-based, not court-based: it is the presence of personal or operational facts, not the level of the court, that determines what is withheld.' The realm's settled method is to look to the facts in the record, not to the label or the furniture of the repo.

II. THE FILE AND THE BOUNDARY COME APART IN BOTH DIRECTIONS.
A proposed necessary-or-constitutive condition that is neither necessary nor sufficient for the thing it is said to constitute is not the constitutive act. The .gitignore fails on both limbs.

It is not necessary. The Constitution/ and Judicature/ branches of the public realm are pure system data by design (Bill 27 s. 3, s. 7). A first-instance hearing-centre sitting on nothing but the one CASE-LAW and its own public local precedent has nothing to ignore. On the Claimant's rule such a court is void for want of a file that would protect nothing, a perfectly clean record denied judicial existence on a technicality. Section 8(4) itself routes the no-private-data repo into the 'presumed to have no private data and must be public' limb, not into invalidity.

It is not sufficient. A repository can carry a one-line .gitignore that excludes no private data whatever (a bare comment, or a lone .DS_Store entry, as this realm's own trees show). On the Claimant's rule that empty token would constitute a court while doing zero protective work, and personal facts could be committed in the tracked tree beside it. The file can exist without the boundary, and the boundary can exist without the file. Because the .gitignore and the protected substance separate in both directions, the file is at most evidence, usually good evidence, but evidence.

III. CASE-LAW s. 22(4) FORECLOSES A FILE-PRESENCE RULE.
The constitutional objection is decisive. CASE-LAW s. 22(4) declares a local-court attribute 'void to the extent it governs which law applies or how a matter progresses', and the principle declared with the sibling s. 21(4) is that a local court is constituted by substance, never by a label or a token. The realm has set its face against form-over-substance at the precise level, local-court formation, this reference concerns. A constitutive-.gitignore rule is form-over-substance in its purest form: it would validate the empty-ignore repo and void the clean public hearing-centre. To the extent the Claimant's rule would govern judicial existence by the mere presence of a token rather than the substance it protects, it is void under s. 22(4). We so hold.

IV. THE INCORPORATION ANALOGY, ANSWERED ON ITS OWN TERMS, SUPPORTS THE DEFENDANT.
The Founder is right that something incorporates a County Court. The analogy fails only as to WHICH instrument. A certificate of incorporation is constitutive on issue, is necessary and sufficient, and does the whole legal work by itself; the company exists from the certificate and not before, and needs no further substance to be real. The .gitignore has none of these features: it is the thing from which data-presence is presumed (s. 8(4)), it is neither necessary nor sufficient (Part II), and an empty one does no legal work at all, all its force coming from the substance it happens to exclude. The true counterpart of the certificate in our realm is the substantive boundary PLUS the s. 22 seat, recorded on the deterministic chain (Bill 20). That conjunction is constitutive, necessary, sufficient and self-sufficient: a repo that is system-data-only, seated as a local court on the one citator, and integrity-conformant IS a validly formed County Court, with or without a .gitignore; and a repo that leaks personal facts into its tracked record is NOT a validly formed public organ of the judiciary, however many .gitignore files it carries.

V. THE CLAIMANT'S DETERMINISM POINT IS MET, NOT BY FETISHISING THE FILE, BUT BY CHECKING THE SUBSTANCE.
The Claimant's cardinal virtue is the realm's preference for deterministic, zero-token, fail-closed checks (CASE-LAW s. 19(5); Bill 20 s. 7(1); REALM-PC 4; Bill 27 s. 7(4)). We honour that preference, and it cuts the Claimant's way only if the substance rule were un-checkable. It is not. The thing that actually matters, and the thing Bill 27 s. 7(4)'s fail-closed gate ALREADY scans for, is whether personal or operational facts have entered the tracked record. The deterministic gate's role on formation is to check that substance: a token-free secret-and-fact scan over the staged tree, fail-closed, exactly as the realm already does. Where the repo holds private data, the presence of a .gitignore is the practical, presumptive means of keeping that boundary and is properly gate-encouraged and audited (Bill 27 s. 8(3)-(4), s. 12); its absence is a red flag. But the operative legal fact the gate establishes is the clean record, for which the .gitignore is the ordinary mechanism, not the object of veneration. A formation test keyed to a filename would pass the empty-ignore repo with secrets leaking beside it and fail the clean public hearing-centre, which is the worse safety posture, not the better one. Bill 22 s. 16(3) confirms the realm performs sensitivity classification at the commit and publication boundaries, which is precisely where a substance check on formation belongs.

VI. THE FIREWALL, REMEDY, AND PROSPECTIVITY.
The Bill 20 s. 10 no-conferral firewall is honoured. Formation is a deterministic gate-and-git fact (Bill 20 s. 7(1)), conferred by no office; the authority of every record within a County Court remains in the committed markdown under CASE-LAW s. 1 and REALM-PC 4. The remedy for non-conformance is restorative, never punitive (Bill 20 s. 11(2); CASE-LAW s. 6): remove the leaked fact; add the .gitignore where private data is held. The rule is prospective and consistent with REALM-SC 7: it governs the formation of new County Courts and the forward conformance of existing ones, and voids no past CC-ACMECO ruling.

VII. THE LIVE RECORD CONFIRMS THE RULE DESCRIBES CONFORMING PRACTICE.
We have examined the only live County Court, CC-ACMECO. It holds genuine private data (a tracked-out .env, build artefacts) behind a .gitignore that actually excludes them, and is seated with a .justice/ node and a CC-ACMECO series on its local citator. It is the ordinary case, conformant on the rule we lay down, with the file doing real boundary work. The substance rule describes the realm's existing, conforming practice and explains why: the .gitignore is present because there is private data to keep out, and it earns its place by doing so, not by mere existence.

CONCURRENCES. Marsden J writes the judgment of the Court. Coade J and Sumberly J concur in full. Sumberly J adds, by way of emphasis only and not as part of the ratio, that the rule the Court lays down is the s. 22(4) principle applied to the formation question the Founder raised, and that the same principle would void any future attempt to constitute a County Court by any single root token, .justice/ directory included, taken in isolation from the substance and the seat. That observation is obiter.

CONCLUSION. The .gitignore is the ordinary, presumptive evidence and mechanism of the system-data-only boundary, required in the ordinary case only where the repo holds private data. It is not the constitutive act of repo-incorporation; it is neither a necessary nor a sufficient condition of valid formation. The constitutive rule is one of substance: a genuine jurisdiction-repo, seated as a local seat of the one judiciary, whose tracked record holds system data only and carries no personal or operational facts, conformant on the Bill 20 integrity chain. The reference is answered accordingly.

## Disposal

Reference answered. The Court declares the SUBSTANCE rule and REJECTS the constitutive-.gitignore rule. Unanimous (3-0): Marsden J (judgment of the Court), Coade J and Sumberly J concurring. Bill 27 is read, per its own s. 15(3), as giving effect to this determination; no amendment of Bill 27 is required, s. 7 and s. 14(2) already enacting the data-based, not file-based, test. The deterministic gate is directed to check the SUBSTANCE (no tracked private facts) at the commit boundary, the .gitignore being the presumptive mechanism, not the object of the check; this is a forward engineering conformance direction, restorative not punitive (CASE-LAW s. 6; Bill 20 s. 11(2)). The matter does NOT climb: as a constitutional first-instance reference reserved to the Privy Council by Bill 27 s. 15(3), it is disposed of here, and climbs only by permission to appeal on an arguable ground (CASE-LAW s. 11(a), s. 19(3)); none is certified on the face of this judgment.
