// §21.5: enactment protocol — snapshot -> regen -> build -> keep/rollback.
import fs from "node:fs";
import path from "node:path";
import { execa } from "execa";
import { readBook, writeBook, renderBook, renderExamples, citeOf, type BookEntry, LEDGER } from "./book.js";
import { leanAvailable } from "../lean/runner.js";
import { leanDir, leanFile, recordDir, lawDir, unverifiedLedger } from "../paths.js";

// Functions, not module-level consts (evaluated at import time, before --root resolves).
const BOOK_LEAN = () => leanFile("Vjs/Book.lean");
const EXAMPLES_LEAN = () => leanFile("Vjs/Examples.lean");

export interface EnactResult { ok: boolean; citation: string; deferred?: boolean; diagnostics?: string }

function nextRecordNumber(): string {
  const files = fs.existsSync(recordDir()) ? fs.readdirSync(recordDir()).filter(f => /^\d{4}\.md$/.test(f)) : [];
  const max = files.reduce((a, f) => Math.max(a, Number(f.slice(0, 4))), 0);
  return String(max + 1).padStart(4, "0");
}

export async function enact(entry: BookEntry): Promise<EnactResult> {
  const before = readBook();
  // TS pre-checks (a courtesy; the kernel is the authority)
  if (before.some(e => e.slug === entry.slug)) throw new Error(`slug not fresh: ${entry.slug}`);
  if (before.some(e => e.year === entry.year && e.ordinal === entry.ordinal))
    throw new Error(`citation not fresh: ${citeOf(entry)}`);
  if (entry.rule.type !== "free" && !entry.vectors)
    throw new Error(`unfalsifiable law refused: ${citeOf(entry)} needs deny+allow vectors (Art. 8)`);

  const snapshot = fs.readFileSync(LEDGER(), "utf8");
  const bookPrev = fs.existsSync(BOOK_LEAN()) ? fs.readFileSync(BOOK_LEAN(), "utf8") : null;
  const exPrev = fs.existsSync(EXAMPLES_LEAN()) ? fs.readFileSync(EXAMPLES_LEAN(), "utf8") : null;

  const after = [entry, ...before];
  writeBook(after);
  fs.writeFileSync(BOOK_LEAN(), renderBook(after));
  fs.writeFileSync(EXAMPLES_LEAN(), renderExamples(after));

  if (!(await leanAvailable())) {
    // Deferred environment: the enactment is provisional until the ledger runs (README deviations).
    fs.appendFileSync(unverifiedLedger(), `- [ ] \`lake build Vps\` -- enactment ${citeOf(entry)} (${entry.slug})\n`);
    writeSideEffects(entry);
    return { ok: true, citation: citeOf(entry), deferred: true };
  }

  const r = await execa("lake", ["build", "Vjs"], { cwd: leanDir(), timeout: 120000, reject: false });
  if (r.exitCode === 0) {
    writeSideEffects(entry);
    return { ok: true, citation: citeOf(entry) };
  }
  // rollback: the enactment never happened
  fs.writeFileSync(LEDGER(), snapshot);
  if (bookPrev !== null) fs.writeFileSync(BOOK_LEAN(), bookPrev); else fs.rmSync(BOOK_LEAN(), { force: true });
  if (exPrev !== null) fs.writeFileSync(EXAMPLES_LEAN(), exPrev); else fs.rmSync(EXAMPLES_LEAN(), { force: true });
  return { ok: false, citation: citeOf(entry), diagnostics: (r.stdout + "\n" + r.stderr).slice(-4000) };
}

function writeSideEffects(entry: BookEntry): void {
  fs.mkdirSync(lawDir(), { recursive: true });
  fs.writeFileSync(path.join(lawDir(), `${entry.year}-vps-${entry.ordinal}.md`),
    `# ${citeOf(entry)} — ${entry.title}\n\n${entry.summary}\n\n` +
    "```json\n" + JSON.stringify(entry, null, 2) + "\n```\n");
  fs.mkdirSync(recordDir(), { recursive: true });
  const n = nextRecordNumber();
  fs.writeFileSync(path.join(recordDir(), `${n}.md`),
    `# Record ${n}\n\n${citeOf(entry)} enacted: ${entry.title}.\n`);
}
