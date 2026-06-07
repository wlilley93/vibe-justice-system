---
citation_id: "[2026] REALM-PC 17"
tier: privy-council
kind: request_for_ruling
status: good-law
per_incuriam: false
date: 2026-06-07
panel: ["Coade J", "Goffe J", "Sumberly J"]
seised_by: "Sovereign Founder reference via the Ministry of Justice: multiplayer VJS, local sovereignty, and community records"
cause_title: "In the matter of local VJS jurisdictions, default subscription, independence, and community federation"
registrar_note: "Authored by the bench (Coade J for the Court, Goffe J and Sumberly J concurring); reduced to the filed record by Lexby as s.18(4) registrar, the decision pre-existing the prose ([2026] REALM-SC 8). Bench-name conformance only: invented VJS names replace non-VJS real jurist labels; no change to ratio, status, citation, vote, or legal force."
---

# [2026] REALM-PC 17

| Field | Value |
|-------|-------|
| **Citation** | [2026] REALM-PC 17 |
| **Tier** | Privy Council (constitutional first instance, bench of three) |
| **Before** | Coade J (judgment of the Court), Goffe J, Sumberly J |
| **Kind** | Request for ruling |
| **Status** | good-law |
| **Cites** | CASE-LAW s. 1; s. 2; s. 3; s. 5; s. 6; s. 9; s. 10; s. 11(c)-(d); s. 13; s. 14; s. 18(4)-(5); s. 19(5); s. 22; Bill 16; Bill 20; Bill 22; Bill 27 ss. 5A, 7, 14; Bill 29; [2026] REALM-SC 6; [2026] REALM-SC 8; [2026] REALM-PC 14; [2026] REALM-PC 15; VPR 8 |

> The Court answers the Founder's multiplayer reference. Installing or forking VJS creates a local
> jurisdiction subscribed by default to canonical VJS law, but not trapped inside canonical VJS governance.
> A local sovereign may remain subscribed, become independent, or federate elsewhere. Unanimous (3-0).

## The questions

1. Does a person downloading or forking VJS expose personal or operational material from the public repo?
2. Does installing VJS automatically require that person's repo to push future case law to canonical VJS?
3. What is the legal effect of branching, creating or deleting courts, changing ministries, or enacting new local law?
4. How should multiplayer VJS jurisdictions share value without making canonical VJS an overlord of every copy?

## Ratio (binding, realm-wide)

1. The canonical public VJS repository must contain system data only. Personal facts, operational facts, secrets, private project work, and internal handover material are not part of the public product record. A public file serving only private provenance or operational handover is to be removed from the public product tree.

2. Installing or forking VJS creates a local VJS jurisdiction. By default that jurisdiction subscribes to the canonical VJS law vendored at install time, including CASE-LAW, VPR, the plugin rules, the deterministic citator method, and the safety hooks. Subscription is the starting law because it gives immediate value and shared meaning.

3. Default subscription is not subjection forever. The local Principal, acting as local sovereign for that repo, may by express local law amend the local statute book, create or remove local courts, alter ministries, change local procedure, become independent of canonical VJS, or join/create another multilateral community. Those local acts bind that repo only unless accepted into canonical VJS or another community by that community's own rules.

4. No local jurisdiction is automatically required to publish its case law to canonical VJS. Community-record publication is opt-in unless the local sovereign's own law makes it mandatory. A contribution to canonical VJS is made by anonymised pull request and is reviewed by canonical maintainers before it forms part of the canonical record.

5. Community rulings are persuasive across participating jurisdictions unless a jurisdiction expressly subscribes to them as binding. Canonical VJS remains free to accept, reject, edit, or decline community submissions under its own law; a fork or independent community is equally free to adopt its own acceptance rules.

6. A pre-push gate that blocks every remote merely because its URL or name contains `vibe-justice-system` is overbroad. The public-push checkpoint gate may fail closed only for the exact canonical public remote `wlilley93/vibe-justice-system`. Forks, mirrors, private remotes, and independent VJS communities are not the canonical public-push act.

7. The installer and plugin text must speak in local paths by default: `.justice/INDEX.md`, `.justice/judgments/`, and `.justice/suites/`. References to `Judicature/.justice/` are correct only for the canonical source repo.

## Reasons

The first point is privacy and publication. Bill 27 draws the line between public system data and private personal or operational data. The public system needs enough law, procedure, judgments, hooks, and docs to be useful, inspectable, and forkable. It does not need a private handover note, local machine paths, host names, secrets, or project-specific facts. Their presence would add no public value and would weaken trust in the public boundary.

The second point is value. A downloaded VJS repo should work immediately. That requires a default law: the user receives the constitutional settlement, the procedure rules, the plugin instructions, the citator method, and the enforcement hooks. Without that default, every installation starts in a constitutional vacuum and the agent cannot know what correctness means.

The third point is sovereignty. The old shorthand in CASE-LAW s. 9 that there are "no competing sovereigns" was correct inside one realm, but too wide when read against forks and installations. Canonical VJS can say what counts as canonical VJS law. It cannot make every downloader forever subject to future canonical governance. The better construction is subscription: the local repo begins by adopting canonical law, and its local sovereign may later keep it, change it, leave it, or federate elsewhere.

That construction preserves the useful part of unity. A repo that remains subscribed gets the benefit of shared precedent and common tooling. A repo that becomes independent remains coherent because it does so by an express local act and keeps its own citator. A group of repos can form a multilateral community record by agreeing the rules of contribution and weight. None of those acts corrupts canonical VJS because canonical VJS changes only by its own lawful process.

The fourth point is community. Mandatory upstream publication would be both impractical and wrong. It would disclose too much unless every ruling were perfectly anonymised; it would impose a burden on people using VJS privately; and it would turn the canonical repository into an involuntary receiver of every local experiment. The correct rule is opt-in contribution. The shared record grows because people choose to contribute rulings that are useful beyond one repo.

The fifth point is tooling. Hooks should enforce the law they are written for and no more. The canonical public-push gate protects an irreversible outward act by the canonical maintainer. It should not block a user's fork named `vibe-justice-system`, a private mirror, or a new community that uses the VJS pattern under its own sovereignty.

## Disposal

The reference is answered as follows.

1. The public VJS repo is to remain system-data-only. Internal handover material is not part of the public product tree.
2. Installing or forking VJS creates a local jurisdiction subscribed by default to canonical VJS law.
3. The local sovereign may remain subscribed, become independent, or federate elsewhere by express local law.
4. No automatic upstream publication duty exists. Community contribution is opt-in and anonymised.
5. The pre-push gate, plugin text, installer, README, VPR, and community documentation are to be conformed to this ruling.
6. The Standing Committee should enact a short Multi-Jurisdiction and Community Record Act to amend CASE-LAW s. 9 and VPR 8 expressly.

The matter does not climb. It clarifies the reach of canonical sovereignty over copies and requires express statutory amendment only because CASE-LAW s. 9 is entrenched.
