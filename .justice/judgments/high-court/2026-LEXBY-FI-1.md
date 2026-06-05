---
citation: "[2026] LEXBY-FI 1"
uuid: 86b154cf-52d9-44a1-b472-289948da3403
tier: first-instance
judge: Bowan J
kind: request_for_ruling
status: good-law
date: 2026-06-05
---

# [2026] LEXBY-FI 1

**IN THE FIRST INSTANCE COURT OF THE VIBE JUSTICE SYSTEM**

**Before: Bowan J**

---

## Matter

**Kind:** Request for Ruling (forward-looking)

**Question as filed:** Is the Vibe Justice System (VJS), as currently built (commit 3ff820a on wlilley93/vibe-justice-system, 2026-06-05), fit for release as an alpha? The artefacts in place are: SPEC-LAW.md (s. 1 through s. 14, constitutional and ordinary), VPR.md (VPR 1 through 8), three runnable court workflow scripts (first-instance.js, court-of-appeal.js, supreme-council.js) each with a Law Load phase and a Community PR phase, a clerk GitHub Actions workflow, CDD.md methodology manifesto, plugin/CLAUDE.md binding injection block, caselaw/INDEX.md citator, founding case [2026] LEXBY-SC 1, community/ directory structure, court/README.md, and docs/DESIGN-NOTES.md. What is not yet built: the cdd CLI init command, submit-request-to-court and submit-breach-to-court user-facing commands, the ruling card renderer, the npm/PyPI package, the lexby cite command, and deterministic citation numbering. The applicable standard is reasonable skill and care (s. 4 through s. 8). Is the current state coherent, functional, and documented to an alpha standard? What should be completed before a v1 (non-alpha) release?

---

## Intake

**Standing:** Established. The principal files a genuine forward-looking request concerning a real release decision with real consequences for a named project at a specific commit. There is a real party, a real question, and real stakes. Standing is not in doubt.

**Fast-path:** No binding ratio on all fours governs this question. The only precedent in the citator is [2026] LEXBY-SC 1, which establishes the negligence model, unitary sovereignty, and court structure. That ratio does not govern the question of alpha-readiness of the VJS itself. The matter proceeds to full deliberation.

---

## Judgment

**Jurisdiction (s. 14):** The question concerns a decision in the conduct of an AI-assisted software engineering project. It falls squarely within the VJS subject-matter jurisdiction. No jurisdictional objection arises.

**Governing standard (s. 4, s. 5):** The filing invokes s. 4 through s. 8. The applicable standard is reasonable skill and care (s. 5, default rung). The question is not whether the system is perfect or feature-complete but whether a responsible body of competent practice would endorse release under the label "alpha" at the current state of completeness.

**What alpha means under this standard:** In responsible software practice, "alpha" connotes: (a) the core concept is demonstrably instantiated; (b) the system is coherent - its constituent parts work together without contradiction; (c) it is documented well enough that an informed user can understand its shape and navigate it; (d) known gaps are disclosed and labelled; and (e) it is not held out as production-complete. Alpha does not require feature-completeness. It is an invitation to scrutiny, not a commitment to finality.

**Assessment of artefacts present:** The statute book is complete and internally consistent (s. 1 through s. 14, constitutional and ordinary, with the founding Supreme Council case properly enacted). The VPR is complete (VPR 1 through 8). The court workflows are present and runnable. The CDD methodology manifesto exists. The plugin injection block binds practitioners. The citator and founding caselaw are committed. The community directory structure is in place. The clerk automation enforces constitutional compliance on pull requests. Documentation exists in the README and DESIGN-NOTES. The concept - a justice system for AI decisions - is demonstrably instantiated. The constituent parts are internally coherent: the statute is supreme, the VPR aligns with it, the workflows implement the VPR phases, the clerk enforces constitutional compliance.

**Assessment of gaps:** The missing artefacts (cdd CLI init, submit-request-to-court and submit-breach-to-court user commands, ruling card renderer, npm/PyPI packaging, lexby cite command, deterministic citation numbering) are, with one exception addressed below, convenience-layer items. They make the system easier to invoke but do not alter the legal logic it operates. A user today can invoke the court by running the workflow scripts directly. The system is navigable by an informed technical audience. The gaps are acknowledged and do not contradict any part of the law or procedure already in place.

**The citation numbering exception (s. 8, s. 11(d)):** Deterministic citation numbering is a gap of greater legal consequence than the others. s. 11(d) mandates neutral citation in the form [YEAR] LEXBY n. Without a reliable global counter, two simultaneous matters could receive the same citation identifier, compromising the citator's reliability as the authoritative record (s. 1, s. 11). However, under s. 8, a genuinely novel, named hazard at first occurrence triggers a forward duty to spec and remediate, not an immediate breach finding. The gap is disclosed in the filing itself. This court therefore records a forward duty: the citation numbering mechanism must be specified and implemented before v1. Its absence is tolerated at alpha on the grounds that the alpha audience is small, the hazard is named and known, and simultaneous parallel sittings are unlikely in practice at this stage.

**Bowan J's test applied:** I test the proposition "this is alpha-ready" to destruction. The hardest challenge is this: the system governs future AI decisions, but the submit commands that a practitioner would actually use are absent. Can a project using this as a plugin invoke a court today without reading internal workflow code? Marginally, no for a casual user - the CLAUDE.md names the commands but they are not yet wired as executable artefacts. However, the primary audience at alpha is the builder and a small group of technically literate early reviewers who can and will read the workflow scripts. For that audience, the practitioner surface is thin but not zero. The proposition survives destruction at the alpha standard. It would not survive destruction at the v1 standard.

---

## Ratio

The VJS at commit 3ff820a on wlilley93/vibe-justice-system, 2026-06-05, is fit for release as an alpha under the standard of reasonable skill and care (s. 4, s. 5). A governance or justice system is fit for alpha release where: (a) the core legal model is demonstrably instantiated and internally coherent; (b) the governing rules, procedure, and founding caselaw are committed and self-consistent; (c) constitutional enforcement automation is in place; and (d) the known gaps are disclosed, do not undermine legal coherence, and are appropriate to remediate before v1. Convenience-layer tooling (CLI commands, packaging, renderers) is not a prerequisite for alpha. The known gaps at this commit are not a breach of the duty of reasonable skill and care (s. 8: novel, named hazard, forward duty triggered).

---

## Obiter

Before v1 (non-alpha), the following constitute the minimum closure set, in order of legal priority:

1. **Deterministic citation numbering** (highest priority: s. 11(d) citator integrity). A reliable counter mechanism is a necessary condition for v1.
2. **submit-request-to-court and submit-breach-to-court as executable commands** (practitioner surface). The practitioner surface named in plugin/CLAUDE.md must be wired before v1; without it the plugin injection block makes promises the system does not yet keep.
3. **The lexby cite command** (citator tooling: practitioner lookup of ratio by citation). Strongly advisable before v1.
4. **Ruling card renderer** (output legibility). Strongly advisable before v1.
5. **cdd CLI init command** (onboarding). Advisable before v1.
6. **npm/PyPI packaging** (distribution). Advisable before v1.

Items 1 and 2 are in this court's view necessary conditions for v1 readiness. Items 3 through 6 are strongly advisable but their absence alone would not defeat a v1 claim.

This court further observes that the community/caselaw directory structure, the clerk automation, and the citator-first convention (VPR 2 fast-path) are among the strongest design choices in the current artefact set: they instantiate the legal model operationally, not merely as documentation.

---

## Remedy

None ordered. This is a request_for_ruling, not a breach. No prior deviation has been found. The forward duty arising from the citation-numbering gap (s. 8) is noted and recorded but requires no immediate remediation order at this stage.

---

## Per Incuriam

False. This ruling was made with the governing statute (SPEC-LAW.md, s. 1 through s. 14) and all available precedent (caselaw/INDEX.md, [2026] LEXBY-SC 1) before the court.

---

**Status:** good-law
**Citation:** [2026] LEXBY-FI 1

---

*Bowan J*
*2026-06-05*
*First Instance Court of the Vibe Justice System*
