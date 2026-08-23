// §21.3: .justice/book.json ledger -> generated Book.lean + Examples.lean.
// Regenerate-don't-patch; unfalsifiable law refused in TS before Lean is consulted.
import fs from "node:fs";
import path from "node:path";
import { justice, R } from "../paths.js";
import crypto from "node:crypto";

/** The sha256 of this jurisdiction's signed genesis text. Computed, never typed: a digest
 *  that can drift from the text it pins is not a pin. Falls back to the placeholder until
 *  law/genesis.md exists, so a fresh repo still builds. */
export function genesisDigest(): string {
  const p = R("law", "genesis.md");
  if (!fs.existsSync(p)) return "sha256:GENESIS-PLACEHOLDER-PIN-ME";
  return "sha256:" + crypto.createHash("sha256").update(fs.readFileSync(p)).digest("hex");
}

export interface Vec { pathsChanged: string[]; recordsAdded: number }
export interface BookEntry {
  year: number; ordinal: number; slug: string;
  kind: "charter" | "statute" | "ruling" | "note";
  rule: { type: "pathForbidden" | "recordRequired"; scope: string } | { type: "free" };
  entrenched: boolean;
  supersedes: { year: number; ordinal: number } | null;
  authority: { type: "sovereign" } | { type: "derived"; parent: { year: number; ordinal: number } };
  title: string; summary: string;
  vectors?: { deny: Vec; allow: Vec };
}

// Function, not a module-level const: a const here is evaluated at import time, before
// any --root is resolved, and captures the wrong directory.
export const LEDGER = () => justice("book.json");

export function readBook(): BookEntry[] {
  return JSON.parse(fs.readFileSync(LEDGER(), "utf8"));
}
export function writeBook(entries: BookEntry[]): void {
  fs.writeFileSync(LEDGER(), JSON.stringify(entries, null, 2));
}
export const citeOf = (e: { year: number; ordinal: number }) => `[${e.year}] VJS ${e.ordinal}`;

const q = (s: string) => JSON.stringify(s);

function ruleToLean(r: BookEntry["rule"]): string {
  if (r.type === "free") return ".free";
  if (r.type === "pathForbidden") return `.pathForbidden ${q(r.scope)}`;
  return `.recordRequired ${q(r.scope)}`;
}
function authorityToLean(a: BookEntry["authority"]): string {
  return a.type === "sovereign" ? ".sovereign digest" : `.derived ⟨${a.parent.year}, ${a.parent.ordinal}⟩`;
}
function supersedesToLean(s: BookEntry["supersedes"]): string {
  return s ? `some ⟨${s.year}, ${s.ordinal}⟩` : "none";
}

export function renderBook(entries: BookEntry[]): string {
  // entries newest first; genesis (ordinal 1, charter) is vendored in Genesis.lean and
  // appears in the ledger for completeness — it renders as the list tail, not a def.
  const nonGenesis = entries.filter(e => !(e.kind === "charter" && e.ordinal === 1));
  const L: string[] = [];
  L.push("/-");
  L.push("GENERATED from .justice/book.json by src/kernel/book.ts — do not edit (CLAUDE.md 5e).");
  L.push("The build is the enactment (§21.5): if book_lawful fails to compile, the enactment");
  L.push("never happened.");
  L.push("-/");
  L.push("import Vps.Legitimacy");
  L.push("import Vps.Proofs");
  L.push("");
  L.push("namespace Vjs");
  L.push("open Vps");
  L.push("");
  for (const e of [...nonGenesis].reverse()) { // oldest first for readable defs
    L.push(`/-- ${citeOf(e)} — ${e.title}. ${e.summary} -/`);
    L.push(`def ${e.slug} : Instrument :=`);
    L.push(`  { cite := ⟨${e.year}, ${e.ordinal}⟩`);
    L.push(`  , kind := .${e.kind}`);
    L.push(`  , rule := ${ruleToLean(e.rule)}`);
    L.push(`  , entrenched := ${e.entrenched}`);
    L.push(`  , supersedes := ${supersedesToLean(e.supersedes)}`);
    L.push(`  , authority := ${authorityToLean(e.authority)} }`);
    L.push("");
  }
  L.push("/-- **This jurisdiction's sovereign digest** -- the sha256 of law/genesis.md.");
  L.push("    The engine is a pinned dependency and contributes no digest of its own; this is");
  L.push("    the only thing this repository adds to legitimacy. Changing it is an Article 10");
  L.push("    amendment, which forces every proof below to be re-established. -/");
  L.push(`def digest : String := ${JSON.stringify(genesisDigest())}`);
  L.push("");
  L.push("/-- The book, newest first. -/");
  L.push("def theBook : List Instrument :=");
  L.push("  [" + [...nonGenesis.map(e => e.slug), "genesisInstrument digest"].join(", ") + "]");
  L.push("");
  L.push("/-- The book's legitimacy is a compile-time theorem (§21.5). -/");
  L.push("theorem book_lawful : Lawful digest theBook :=");
  let term = "Lawful.genesis";
  for (let i = 0; i < nonGenesis.length; i++)
    term = `(Lawful.enact ${term} (by decide) (by decide))`;
  L.push("  " + term.replace(/^\(|\)$/g, "").trim());
  L.push("");
  L.push("/-- The gate as deployed: the compiled book applied to facts. -/");
  L.push("def gate (f : Facts) : Verdict :=");
  L.push("  decideVerdict theBook f");
  L.push("");
  L.push("end Vjs");
  L.push("");
  return L.join("\n");
}

export function renderExamples(entries: BookEntry[]): string {
  const L: string[] = [];
  L.push("/-");
  L.push("GENERATED example vectors (Art. 8): every operative rule demonstrates one world");
  L.push("it denies and one it allows. native_decide: String.isPrefixOf does not");
  L.push("kernel-reduce (PROVENANCE); the compiler joins the trusted base for these");
  L.push("vectors only (§21.9).");
  L.push("-/");
  L.push("import Vjs.Book");
  L.push("");
  L.push("namespace Vjs");
  L.push("open Vps");
  L.push("");
  for (const e of entries) {
    if (e.rule.type === "free") continue;
    if (!e.vectors) throw new Error(`unfalsifiable law refused: ${citeOf(e)} (${e.slug}) has an operative rule but no vectors (Art. 8)`);
    const fmt = (v: Vec) =>
      `{ pathsChanged := [${v.pathsChanged.map(q).join(", ")}], recordsAdded := ${v.recordsAdded} }`;
    L.push(`-- ${citeOf(e)} (${e.title}): deny vector`);
    L.push(`example : gate ${fmt(e.vectors.deny)}`);
    L.push(`    = .deny [${e.slug}.cite] := by native_decide`);
    L.push("");
    L.push(`-- ${citeOf(e)}: allow vector`);
    L.push(`example : gate ${fmt(e.vectors.allow)}`);
    L.push(`    = .allow := by native_decide`);
    L.push("");
  }
  L.push("end Vjs");
  L.push("");
  return L.join("\n");
}

export function nextOrdinal(entries: BookEntry[]): number {
  return entries.reduce((a, e) => Math.max(a, e.ordinal), 0) + 1;
}
