/-
GENERATED example vectors (Art. 8): every operative rule demonstrates one world
it denies and one it allows. native_decide: String.isPrefixOf does not
kernel-reduce (PROVENANCE); the compiler joins the trusted base for these
vectors only (§21.9).
-/
import Vjs.Book

namespace Vjs
open Vps

-- [2026] VJS 2 (Judgment Integrity Act): deny vector
example : gate { pathsChanged := [".justice/judgments/first-instance/001-x.md"], recordsAdded := 0 }
    = .deny [actJudgmentIntegrity.cite] := by native_decide

-- [2026] VJS 2: allow vector
example : gate { pathsChanged := [".justice/judgments/first-instance/001-x.md", "record/0002.md"], recordsAdded := 1 }
    = .allow := by native_decide

-- [2026] VJS 3 (Citator Integrity Act): deny vector
example : gate { pathsChanged := [".justice/INDEX.md"], recordsAdded := 0 }
    = .deny [actCitatorIntegrity.cite] := by native_decide

-- [2026] VJS 3: allow vector
example : gate { pathsChanged := [".justice/INDEX.md", "record/0003.md"], recordsAdded := 1 }
    = .allow := by native_decide

-- [2026] VJS 4 (Record Discipline Act): deny vector
example : gate { pathsChanged := ["law/2026-vjs-4.md"], recordsAdded := 0 }
    = .deny [actRecordDiscipline.cite] := by native_decide

-- [2026] VJS 4: allow vector
example : gate { pathsChanged := ["law/2026-vjs-4.md", "record/0004.md"], recordsAdded := 1 }
    = .allow := by native_decide

-- [2026] VJS 5 (Gate Integrity Act): deny vector
example : gate { pathsChanged := ["gate/pre-commit"], recordsAdded := 1 }
    = .deny [actGateIntegrity.cite] := by native_decide

-- [2026] VJS 5: allow vector
example : gate { pathsChanged := ["README.md"], recordsAdded := 0 }
    = .allow := by native_decide

end Vjs
