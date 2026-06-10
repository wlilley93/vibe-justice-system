# [2026] VJS-CC-AGENT-UNIVERSE-V2 14 - Order-schema court-record extension (operational limb)

**County Court of agent-universe-v2, single judge (odd bench of 1) per 2026-VJS-COURTS-CONSTITUTION-001 D1. Convened on the registrar's own motion. Before: Ledgerward CCJ. Disposition: operational limb disposed; the schema change is machinery; permission to appeal GRANTED.**

I sit as Ledgerward CCJ, single judge of the County Court of agent-universe-v2, convened on the registrar's own motion. The registrar's preference is not before me; I decide on the symmetric file alone.

The s.7 power lets the Standing Committee make subordinate law that is "strictly below" the Act, the Acts of Union, the constitution, and the primary Acts. It may not amend, disapply, or expand any of those, nor the assent rule (anti-Henry-VIII; s.25 entrenchment). So the dividing line is plain: an SI is good if it operates wholly within the space the constitution leaves to administration, and void to the extent it reaches a reserved matter.

The courts constitution (2026-VJS-COURTS-CONSTITUTION-001) fixes who sits, the odd bench sizes, the tiers, and each court's jurisdiction. REG-COURT-RECORD-001 touches none of these. It adds a recording duty: that a ruling record its deciding bench and the sha256 of the symmetric case file it decided. All new Order fields are optional, serde-default, and skip-if-empty, so the 22 existing orders load unchanged. The duty binds prospectively and expressly disclaims altering bench sizes, jurisdiction, or the assent rule.

Recording what a court did is the administration of the court, not its constitution. The schema describes the audit trail of the constitutive output; it does not re-define what a court constitutionally IS. I am alert to the contrary reading: a required element of a valid ruling could, pushed far enough, become constitutive. The Regulation guards against that by making the fields a recording duty, not a validity gate that voids a ruling for their absence. Read so, it stays machinery, and I uphold it.

That contrary reading is a genuine point of general constitutional importance on the dividing line between machinery and constitution, fit for the constitutional first instance. I therefore GRANT PERMISSION TO APPEAL to the Privy Council (bench of 3), having disposed of the operational limb myself.

*Order entered at lawpack/v2/orders/2026-VJS-CC-ORDER-SCHEMA-001.yaml. Appealable to the Privy Council per [2026] VJS-PC 6 D3.*
