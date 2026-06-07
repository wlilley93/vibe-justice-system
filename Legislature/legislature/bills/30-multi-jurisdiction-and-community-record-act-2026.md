<!-- Bill 30 of the Order Paper of the Realm. Drafted by the Standing Committee after the Privy Council reference in [2026] REALM-PC 17. -->
<!-- status: enacted | royal-assent: 2026-06-07 | outcome: passed-round-1 | ayes: 4/4 | drafting rounds: 1 -->
<!-- express amendment of CASE-LAW s. 9 [constitutional] and VPR 8 -->

# The Multi-Jurisdiction and Community Record Act 2026

**Bill 30 of the Order Paper of the Realm**

---

## Long title

**An Act** to provide that installing or forking the Vibe Justice System creates a local jurisdiction subscribed by default to canonical VJS law; to provide that default subscription gives immediate governing law but does not prevent the local Principal, acting as local sovereign, from amending local law, becoming independent, or joining or creating a different multilateral community; to amend CASE-LAW s. 9 expressly so unity governs canonical VJS and subscribed jurisdictions but does not bind independent forks forever; to amend VPR 8 so Community Record publication is opt-in unless local law provides otherwise; to require anonymisation and maintainer review for contributions to the canonical Community Record; to require tooling and documentation to distinguish the exact canonical public remote from forks and independent communities; and for connected purposes.

---

## Short title

1. This Act may be cited as the **Multi-Jurisdiction and Community Record Act 2026** (Bill 30).

## Commencement

2. (1) This Act comes into force on Royal Assent by the Sovereign Founder (2026-06-07).

(2) The amendment to CASE-LAW s. 9 made by section 4 is an express deliberate amendment of the entrenched constitutional article CASE-LAW s. 9.

(3) The amendment to VPR 8 made by section 5 takes effect on Royal Assent and supersedes inconsistent prior VPR text from that date forward.

## Constitutional basis

3. (1) This Act gives effect to [2026] REALM-PC 17.

(2) The Act preserves canonical VJS sovereignty inside canonical VJS. It does not create a second apex within canonical VJS, a second citation authority for canonical law, or a power in another repo to alter canonical VJS by unilateral act.

(3) The Act recognises a separate question: what happens when another person downloads, forks, or installs VJS. That repo begins by subscribing to canonical VJS law, but its Principal may later choose local independence or another federation by express local law.

---

## Part 1 - Amendment of CASE-LAW s. 9

### 4. Subscription, independence, and federation

In CASE-LAW s. 9, after the existing text, insert:

"**s. 9(2)** [constitutional] **Canonical unity and local subscription.** Within canonical VJS and any jurisdiction that remains subscribed to it, CASE-LAW is the shared statute book and the canonical Supreme Court remains the apex for canonical VJS law. Installing or forking VJS creates a local jurisdiction subscribed by default to the canonical VJS law it vendors at install or fork time.

**s. 9(3)** [constitutional] **Default subscription is not perpetual subjection.** A subscribed local jurisdiction may, by express local sovereign act, amend its local law, create or remove local courts or ministries, alter local procedure, become independent of canonical VJS, or join or create another multilateral community. That act binds the local jurisdiction only, unless accepted into canonical VJS or another community by that community's own rules.

**s. 9(4)** [constitutional] **Canonical acceptance rule.** Canonical VJS changes only by canonical VJS law: statute, judgment, accepted pull request, or other lawful canonical mechanism. A fork, local court, independent jurisdiction, or external community cannot amend canonical VJS merely by acting locally.

**s. 9(5)** [constitutional] **Community records.** A community record is a shared persuasive-law repository. Canonical VJS maintains one such record. Other communities may maintain their own. Community rulings are persuasive unless the receiving jurisdiction expressly makes them binding.

**s. 9(6)** [constitutional] **No automatic upstream duty.** No local jurisdiction is automatically required to publish its case law to canonical VJS. Publication to any community record is opt-in unless the local jurisdiction's own law provides otherwise, and any contribution must be anonymised before submission."

## Part 2 - Amendment of VPR 8

### 5. Community Record publication

(1) VPR 8 is amended so that Community Record publication is opt-in unless local law provides otherwise.

(2) A local ruling remains in the local repo's `.justice/judgments/` unless the local Principal chooses to submit it to a community record.

(3) A submission to the canonical VJS Community Record must be anonymised. Repo names where they identify private work, file paths, function names, variable names, class names, personal facts, tokens, hostnames, infrastructure facts, and operational details must be removed or generalised. The legal question, necessary facts, ratio, law applied, and outcome may remain.

(4) Canonical VJS maintainers may accept, reject, edit, or request changes to a contribution under canonical VJS law and repository rules. Acceptance into the canonical Community Record makes the contribution part of the canonical public record with persuasive weight unless canonical law gives it greater weight.

(5) An independent VJS community may set different contribution, anonymisation, and weight rules for its own record, provided it does not present those rules as canonical VJS law unless canonical VJS accepts them.

## Part 3 - Tooling and documentation conformance

### 6. Exact canonical remote only

(1) A public-push checkpoint gate may fail closed only for the exact canonical public VJS remote:

`wlilley93/vibe-justice-system`.

(2) A fork, mirror, private remote, or independent community using the name or pattern `vibe-justice-system` is not the canonical public-push act merely because its URL contains those words.

### 7. Local paths by default

(1) Installer and plugin documentation must describe installed repos in local terms: `.justice/INDEX.md`, `.justice/judgments/`, and `.justice/suites/`.

(2) References to `Judicature/.justice/` are reserved for the canonical VJS source repo or for documentation expressly explaining the canonical source layout.

### 8. Public-data boundary

(1) Canonical VJS remains system-data-only in public. Personal facts, operational facts, secrets, private project work, and internal handover material are not public product data.

(2) A public file whose only public function is private provenance, local path handover, or internal release continuity must be removed from the public product tree or replaced by a public-safe system-data document.

---

## Part 4 - Savings

### 9. Savings

(1) Nothing in this Act weakens the binding force of canonical VJS law inside canonical VJS.

(2) Nothing in this Act requires a local jurisdiction to remain subscribed, to publish upstream, or to accept future canonical amendments after it has lawfully become independent.

(3) Nothing in this Act makes canonical VJS responsible for the acts, laws, or disclosures of independent forks or other communities.

(4) Earlier references to "no competing sovereigns", including the anti-federalism recital retained by the Acts of Union 2026, are read after this Act as applying inside canonical VJS and jurisdictions that remain subscribed to it. They do not prevent a downloaded, forked, or installed repo from becoming locally independent by the express local sovereign act recognised in CASE-LAW s. 9(3).

---

**END OF ACT**

---

## Committee note

The Bill implements [2026] REALM-PC 17. It solves the multiplayer problem by separating three ideas:

1. **Default subscription:** an installed repo gets working law immediately.
2. **Local sovereignty:** the local Principal may later amend, leave, or federate elsewhere by express local law.
3. **Optional community:** useful rulings may be shared, anonymised, by PR or another community route, but there is no automatic upstream duty.

The Act amends CASE-LAW s. 9 expressly because the old "no competing sovereigns" shorthand was too strong for forks. It remains true inside canonical VJS. It is no longer read to bind every downloader forever.

## Vote record

- **Counsel Aldous (Restraint): AYE** - The Act is short and does the minimum necessary constitutional work. It does not create a new canonical apex or a second legislature. It states the subscription default, the independence right, and the opt-in community rule in one place.

- **Counsel Verity (Codifier): AYE** - The insertion into CASE-LAW s. 9 is express, deliberate, and correctly numbered. VPR 8 is amended by a machinery section rather than left as a contradictory procedure rule. The canonical acceptance rule prevents local forks from changing canonical VJS by accident.

- **Counsel Marlowe (Guardrail): AYE** - The anonymisation and public-data boundary are necessary. No user should be forced to publish local case law upstream, and canonical VJS should not receive private operational material by design.

- **Counsel Drummond (Pragmatist): AYE** - This is the rule the tooling can actually enforce. The hook checks one exact canonical remote; the installer lays down local paths; contributions become pull requests, not surprise pushes.

**Vote: 4 ayes, 0 nays. The Bill passes; no second round required.**

## Royal Assent

*Royal Assent granted by the Sovereign Founder on 2026-06-07 ("proceed"). This Act is now in force. Canonical VJS law remains the default subscription law for installed repos; local independence and federation are lawful by express local sovereign act; Community Record publication is opt-in unless local law says otherwise; and tooling must distinguish the exact canonical public remote from forks and independent communities.*
