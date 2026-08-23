// §21.4: the seed book — Foundry's own constitution. Genesis is vendored in
// Genesis.lean; it appears in the ledger for completeness.
import fs from "node:fs";
import { writeBook, renderBook, renderExamples, LEDGER, type BookEntry } from "./book.js";
import { justice, leanFile } from "../paths.js";

/**
 * This jurisdiction's founding statutes.
 *
 * A court governs its own record and nothing else. There is no source tree here to protect
 * and no build to gate: what must not be quietly altered is the judgments, the prose mirror
 * of the law, and the hook that enforces both. Every operative rule ships with a deny and an
 * allow vector, so unfalsifiable law is refused before Lean is ever consulted (Charter Art. 8).
 */
export const SEED: BookEntry[] = [
  { year: 2026, ordinal: 1, slug: "genesisInstrument", kind: "charter", rule: { type: "free" },
    entrenched: true, supersedes: null, authority: { type: "sovereign" },
    title: "Genesis", summary: "The root of every authority chain in this jurisdiction. Pinned to the sha256 of law/genesis.md." },

  { year: 2026, ordinal: 2, slug: "actJudgmentIntegrity", kind: "statute",
    rule: { type: "recordRequired", scope: ".justice/judgments/" },
    entrenched: true, supersedes: null, authority: { type: "derived", parent: { year: 2026, ordinal: 1 } },
    title: "Judgment Integrity Act",
    summary: "A judgment may not be altered without a record entry explaining why. Rulings filed by the court write their own record and pass without ceremony; only hand edits are denied.",
    vectors: {
      deny:  { pathsChanged: [".justice/judgments/first-instance/001-x.md"], recordsAdded: 0 },
      allow: { pathsChanged: [".justice/judgments/first-instance/001-x.md", "record/0002.md"], recordsAdded: 1 } } },

  { year: 2026, ordinal: 3, slug: "actCitatorIntegrity", kind: "statute",
    rule: { type: "recordRequired", scope: ".justice/INDEX.md" },
    entrenched: true, supersedes: null, authority: { type: "derived", parent: { year: 2026, ordinal: 1 } },
    title: "Citator Integrity Act",
    summary: "The citator is the memo table res judicata reads. A ruling missing from it is a question open to being decided twice, so changing it by hand requires saying why. Enacted after a silent corruption of exactly this kind on 2026-08-23.",
    vectors: {
      deny:  { pathsChanged: [".justice/INDEX.md"], recordsAdded: 0 },
      allow: { pathsChanged: [".justice/INDEX.md", "record/0003.md"], recordsAdded: 1 } } },

  { year: 2026, ordinal: 4, slug: "actRecordDiscipline", kind: "statute",
    rule: { type: "recordRequired", scope: "law/" },
    entrenched: false, supersedes: null, authority: { type: "derived", parent: { year: 2026, ordinal: 1 } },
    title: "Record Discipline Act",
    summary: "The prose mirror of the law changes only alongside a record entry.",
    vectors: {
      deny:  { pathsChanged: ["law/2026-vjs-4.md"], recordsAdded: 0 },
      allow: { pathsChanged: ["law/2026-vjs-4.md", "record/0004.md"], recordsAdded: 1 } } },

  { year: 2026, ordinal: 5, slug: "actGateIntegrity", kind: "statute",
    rule: { type: "pathForbidden", scope: "gate/" },
    entrenched: false, supersedes: null, authority: { type: "derived", parent: { year: 2026, ordinal: 1 } },
    title: "Gate Integrity Act",
    summary: "The gate protects itself: its hook changes only by a lawful superseding enactment shipping with the change.",
    vectors: {
      deny:  { pathsChanged: ["gate/pre-commit"], recordsAdded: 1 },
      allow: { pathsChanged: ["README.md"], recordsAdded: 0 } } }
];

export async function seedBook(): Promise<void> {
  if (fs.existsSync(LEDGER())) return; // idempotent
  fs.mkdirSync(justice(), { recursive: true });
  writeBook(SEED);
  fs.writeFileSync(leanFile("Vjs/Book.lean"), renderBook(SEED));
  fs.writeFileSync(leanFile("Vjs/Examples.lean"), renderExamples(SEED));
}
