# The Super-Repo Public Push and Post-Push Review Instrument 2026

**Citation:** [2026] REALM-SI 7 (under Bill 18, Bill 20, Bill 27 and Bill 29)

**Made by:** the Standing Committee of the Legislature, in exercise of the operational-detail statutory-instrument power conferred by section 18 of the Ministries and Offices Act 2026 (Bill 5) as inserted by the Statutory Instruments (Framework) Act 2026 (Bill 26); the parent authority being the Ministry of Justice for court-routing and public-law record process, read with the Autonomous Execution and Safety Act 2026 (Bill 18), the Repositories and Records Certification Act 2026 (Bill 20), the VJS (Constitution and Machinery) Act 2026 (Bill 27), and the Ministerial Policy Arm Act 2026 (Bill 29); and made by the Standing Committee under section 5C of Bill 27.

**Status:** made

**Procedure:** negative (Bill 14 s.14 objection window)

**Made:** 2026-06-07

**Coming into force:** on expiry of the Bill 14 s.14 objection window without valid objection.

---

## Recitals

In exercise of the powers conferred by the public-mechanics and public-repository provisions of the VJS (Constitution and Machinery) Act 2026 (Bill 27), by the outward-act and safety provisions of the Autonomous Execution and Safety Act 2026 (Bill 18), by the repository-integrity provisions of the Repositories and Records Certification Act 2026 (Bill 20), and by the policy-arm route codified in the Ministerial Policy Arm Act 2026 (Bill 29), the Standing Committee makes the following Regulations.

The Ministry of Justice has proposed, as public-law machinery, that every public push to the VJS super-repo should be authorised before it leaves the private repository and reviewed after it lands. The purpose is not to slow ordinary work. The purpose is to make irreversible public acts legible: who authorised the push, what exact commit moved, what checks ran, and whether the public record remains system data only.

---

## PART 1 - INTERPRETATION

### 1. Definitions

In these Regulations:

**"super-repo"** means the public VJS repository holding the realm-as-state public record described in [2026] REALM-PC 15 and Bill 27.

**"public push"** means any outward git act that publishes, updates, replaces, retargets, or makes default a branch or commit in the public VJS super-repo, including an additive branch push, a default-branch change, and a protected-branch replacement attempt.

**"release warrant"** means a recorded pre-push authority for a public push, identifying the actor, the remote, the ref, the exact local SHA, the intended effect, the public-data boundary checked, and the private backup state.

**"post-push review"** means a Privy Council review after a public push, recording whether the push complied with the release warrant, whether the public-data boundary held, whether the repository integrity chain held, and whether any remediation is required.

**"public-data boundary"** means the Bill 27 and Bill 22 rule that the public realm holds system data only: law, central judgments, procedure, public mechanics, registers, and derived records; not personal, secret, or operational facts.

---

## PART 2 - PRE-PUSH AUTHORITY

### 2. Release warrant required

(1) A public push to the super-repo must not be attempted unless a release warrant exists before the push.

(2) The release warrant must record:

- the intended remote and remote ref;
- the exact local SHA to be pushed;
- whether the push is additive, fast-forward, default-branch retargeting, or history replacement;
- the private backup branch and SHA;
- the checks run for citator integrity, judgment lodgement, whitespace integrity, and public-data boundary scanning;
- the authorising person or office; and
- the timestamp of authorisation.

(3) A deterministic pre-push gate may enforce the warrant by refusing any public push whose remote, ref, or SHA does not match the warrant.

(4) An attempted public push blocked by the deterministic gate, by GitHub branch protection, or by a lease check is not itself a completed publication. It must nevertheless be recorded in the later review if it formed part of the release sequence.

### 3. Law-changing pushes

(1) Where a proposed public push changes the law, adds a judgment, adds an instrument, changes the public-record boundary, or changes the enforcement machinery, the release warrant must identify the legal instrument or judgment that authorises or records that change.

(2) A routine public update need not mint a fresh Act or statutory instrument merely because it is pushed. The per-push legal act is the release warrant plus the post-push review required by these Regulations.

(3) A new Act, statutory instrument, or judgment is required where the content of the push itself changes the law or creates a new legal rule.

---

## PART 3 - POST-PUSH REVIEW

### 4. Privy Council review required

(1) After every completed public push to the super-repo, the Ministry of Justice must place a review reference before the Privy Council.

(2) The Privy Council must determine:

- whether the push matched the release warrant;
- whether the pre-push checks were adequate for the nature of the push;
- whether the public-data boundary held;
- whether the repository integrity chain held;
- whether any failed, blocked, or rejected attempt in the release sequence disclosed a defect needing remedy;
- whether any branch-protection or host-governance issue remains; and
- whether the release was lawful, lawful with remediation, or unlawful.

(3) The review must append the relevant commits and refs, including private source commits and public clean-history commits where both exist.

(4) The review is restorative. If a defect is found, the remedy is to correct the public record, improve the gate or warrant process, and file any necessary legislation or judgment. No punitive consequence is available.

### 5. Timing

(1) The review should be filed promptly after the public push and before the next unrelated public push where practicable.

(2) If urgent publication makes immediate review impossible, the release warrant must state the urgency and the later review must address it.

---

## PART 4 - IMPLEMENTATION

### 6. Ministry of Justice mandate

(1) The Ministry of Justice owns the public-law process for release warrants and post-push review references.

(2) The Ministry of Justice must maintain a public mechanics record describing the warrant fields and the review checklist.

(3) Operational facts supporting the checks may remain in the appropriate private registry. The public record must state the checks and outcomes, not the secrets or concrete operational facts.

### 7. Deterministic gates

(1) The Executive may implement deterministic gates that enforce the release warrant.

(2) A gate must fail closed for a public push lacking a matching warrant.

(3) A gate must allow private/dev backup pushes that are not a public VJS publication, unless another law forbids them.

(4) A gate defect discovered during a release sequence must be fixed before retrying the outward act where the defect bears on the authorisation check.

---

## PART 5 - COMMENCEMENT AND SAVINGS

### 8. Commencement

This Instrument comes into force on expiry of the Bill 14 s.14 objection window without valid objection.

### 9. Savings

(1) These Regulations are prospective.

(2) A release made before commencement is not unlawful merely because the post-push review rule was not yet in force.

(3) Where such a release is voluntarily reviewed by the Privy Council before commencement, that review satisfies the spirit of these Regulations and may be used as the model for the first release-warrant public mechanics record.

---

## Made by the Standing Committee

**Counsel Aldous (Restraint):** A narrow instrument is enough. It does not require a new Act for every ordinary push, which would turn release management into a statute factory. It requires a warrant, a matching gate, and a review. That is the minimum machinery that makes the outward act legible without stopping the realm from moving.

**Counsel Verity (Codifier):** The chain is exact: Bill 18 supplies the outward-act safety frame, Bill 20 supplies repository integrity, Bill 27 supplies the super-repo and public-data boundary, and Bill 29 supplies the Ministry policy route. The instrument distinguishes the per-push release warrant from law-changing content. Where the push changes law, the law must exist; where it only publishes already lawful material, the warrant and review suffice.

**Counsel Marlowe (Guardrail):** The rule is fail-closed at the edge and restorative after the edge. It records blocked attempts, lease failures, branch-protection failures, and gate defects because each can show a real risk. It does not punish. It fixes the record, the gate, or the warrant.

**Counsel Drummond (Pragmatist):** This will work. The push pipeline already has the ingredients: exact SHA, ref, remote, private backup, scans, and a hook. The instrument turns those into a short repeatable checklist and makes the Privy Council review the public close-out.

**Clerk's Note:** Made 2026-06-07. Commencement on lapse of the Bill 14 s.14 objection window without valid objection.

---

**END OF INSTRUMENT**
