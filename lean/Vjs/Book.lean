/-
GENERATED from .justice/book.json by src/kernel/book.ts — do not edit (CLAUDE.md 5e).
The build is the enactment (§21.5): if book_lawful fails to compile, the enactment
never happened.
-/
import Vps.Legitimacy
import Vps.Proofs

namespace Vjs
open Vps

/-- [2026] VJS 5 — Gate Integrity Act. The gate protects itself: its hook changes only by a lawful superseding enactment shipping with the change. -/
def actGateIntegrity : Instrument :=
  { cite := ⟨2026, 5⟩
  , kind := .statute
  , rule := .pathForbidden "gate/"
  , entrenched := false
  , supersedes := none
  , authority := .derived ⟨2026, 1⟩ }

/-- [2026] VJS 4 — Record Discipline Act. The prose mirror of the law changes only alongside a record entry. -/
def actRecordDiscipline : Instrument :=
  { cite := ⟨2026, 4⟩
  , kind := .statute
  , rule := .recordRequired "law/"
  , entrenched := false
  , supersedes := none
  , authority := .derived ⟨2026, 1⟩ }

/-- [2026] VJS 3 — Citator Integrity Act. The citator is the memo table res judicata reads. A ruling missing from it is a question open to being decided twice, so changing it by hand requires saying why. Enacted after a silent corruption of exactly this kind on 2026-08-23. -/
def actCitatorIntegrity : Instrument :=
  { cite := ⟨2026, 3⟩
  , kind := .statute
  , rule := .recordRequired ".justice/INDEX.md"
  , entrenched := true
  , supersedes := none
  , authority := .derived ⟨2026, 1⟩ }

/-- [2026] VJS 2 — Judgment Integrity Act. A judgment may not be altered without a record entry explaining why. Rulings filed by the court write their own record and pass without ceremony; only hand edits are denied. -/
def actJudgmentIntegrity : Instrument :=
  { cite := ⟨2026, 2⟩
  , kind := .statute
  , rule := .recordRequired ".justice/judgments/"
  , entrenched := true
  , supersedes := none
  , authority := .derived ⟨2026, 1⟩ }

/-- [2026] VJS 6 — Ruling on op:boltrig:dev-egress-loopback. Filed from case boltrig (first-instance). Payload in .justice/judgments/. -/
def ruling2026N6 : Instrument :=
  { cite := ⟨2026, 6⟩
  , kind := .ruling
  , rule := .free
  , entrenched := false
  , supersedes := none
  , authority := .derived ⟨2026, 1⟩ }

/-- [2026] VJS 7 — Ruling on model:2026-08-23-court-client-proof:reviewer:shape. Filed from case 2026-08-23-court-client-proof (first-instance). Payload in .justice/judgments/. -/
def ruling2026N7 : Instrument :=
  { cite := ⟨2026, 7⟩
  , kind := .ruling
  , rule := .free
  , entrenched := false
  , supersedes := none
  , authority := .derived ⟨2026, 1⟩ }

/-- [2026] VJS 8 — Ruling on model:2026-08-23-court-client-proof:auditrequirement:shape. Filed from case 2026-08-23-court-client-proof (first-instance). Payload in .justice/judgments/. -/
def ruling2026N8 : Instrument :=
  { cite := ⟨2026, 8⟩
  , kind := .ruling
  , rule := .free
  , entrenched := false
  , supersedes := none
  , authority := .derived ⟨2026, 1⟩ }

/-- [2026] VJS 9 — Ruling on model:2026-08-23-court-client-proof:predicate:meetsAuditRequirement. Filed from case 2026-08-23-court-client-proof (first-instance). Payload in .justice/judgments/. -/
def ruling2026N9 : Instrument :=
  { cite := ⟨2026, 9⟩
  , kind := .ruling
  , rule := .free
  , entrenched := false
  , supersedes := none
  , authority := .derived ⟨2026, 1⟩ }

/-- [2026] VJS 10 — Ruling on interpret:09dcf6bd. Filed from case 2026-08-23-court-client-proof (first-instance). Payload in .justice/judgments/. -/
def ruling2026N10 : Instrument :=
  { cite := ⟨2026, 10⟩
  , kind := .ruling
  , rule := .free
  , entrenched := false
  , supersedes := none
  , authority := .derived ⟨2026, 1⟩ }

/-- **This jurisdiction's sovereign digest** -- the sha256 of law/genesis.md.
    The engine is a pinned dependency and contributes no digest of its own; this is
    the only thing this repository adds to legitimacy. Changing it is an Article 10
    amendment, which forces every proof below to be re-established. -/
def digest : String := "sha256:c07a29e3b65699dd19a71f9a9c5b98fb67008d9f5b7eef38ce0a0d4558e846bf"

/-- The book, newest first. -/
def theBook : List Instrument :=
  [ruling2026N10, ruling2026N9, ruling2026N8, ruling2026N7, ruling2026N6, actJudgmentIntegrity, actCitatorIntegrity, actRecordDiscipline, actGateIntegrity, genesisInstrument digest]

/-- The book's legitimacy is a compile-time theorem (§21.5). -/
theorem book_lawful : Lawful digest theBook :=
  Lawful.enact (Lawful.enact (Lawful.enact (Lawful.enact (Lawful.enact (Lawful.enact (Lawful.enact (Lawful.enact (Lawful.enact Lawful.genesis (by decide) (by decide)) (by decide) (by decide)) (by decide) (by decide)) (by decide) (by decide)) (by decide) (by decide)) (by decide) (by decide)) (by decide) (by decide)) (by decide) (by decide)) (by decide) (by decide)

/-- The gate as deployed: the compiled book applied to facts. -/
def gate (f : Facts) : Verdict :=
  decideVerdict theBook f

end Vjs
