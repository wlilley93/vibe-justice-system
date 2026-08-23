// §11.4: PrecedentStore over .justice/. From M11, ordinal allocation and standing
// go through the kernel book (§21.6); pre-M11 max+1 scan retained as interim.
import fs from "node:fs";
import path from "node:path";
import { justice } from "../paths.js";

export interface Ruling {
  citation: string; court: string; questionKey: string; caseId: string;
  date: string; status: string; file: string;
  question?: string; facts?: string; ruling?: string; reasoning?: string;
  /** Written by fileRuling; recovered by precedent/search.ts, which is the only reader. */
  lawApplied?: string[];
}

// NB the filename. An earlier mechanical const->function refactor rewrote every bare
// `INDEX` token to `INDEX()`, including the one INSIDE this string literal, so reader and
// writer both used ".justice/INDEX().md" — consistently, and therefore silently. The
// citator is the memo table res judicata is built on; a silent second copy of it is the
// worst possible thing to get wrong quietly. test/citator.test.ts now asserts the book and
// the index agree, which would have caught this the moment it happened.
const INDEX = () => justice("INDEX.md");

export function readIndex(): Ruling[] {
  if (!fs.existsSync(INDEX())) return [];
  const rows: Ruling[] = [];
  for (const line of fs.readFileSync(INDEX(), "utf8").split("\n")) {
    const m = line.match(/^\|\s*\[([^\]]+)\]\s+([A-Z]+)\s+(\d+)\s*\|\s*([^|]+)\|\s*([^|]+)\|\s*([^|]+)\|/);
    if (!m) continue;
    rows.push({
      citation: `[${m[1]}] ${m[2]} ${m[3]}`, court: m[4].trim(), questionKey: m[5].trim(),
      caseId: "", date: "", status: "standing", file: m[6].trim()
    });
  }
  return rows;
}

export function lookup(questionKey: string): Ruling | null {
  const rows = readIndex().filter(r => r.questionKey === questionKey);
  for (const r of rows.reverse()) {
    const j = readJudgment(r.file);
    if (j && j.status.startsWith("standing")) return { ...r, ...j };
  }
  return null;
}

export function readJudgment(file: string): Ruling | null {
  const p = justice(file);
  if (!fs.existsSync(p)) return null;
  const src = fs.readFileSync(p, "utf8");
  const fm = src.match(/^---\n([\s\S]*?)\n---/);
  if (!fm) return null;
  const get = (k: string) => fm[1].match(new RegExp(`^${k}:\\s*"?([^"\\n]+)"?`, "m"))?.[1]?.trim() ?? "";
  const section = (h: string) => src.match(new RegExp(`## ${h}\\n([\\s\\S]*?)(?=\\n## |$)`))?.[1]?.trim() ?? "";
  return {
    citation: get("citation"), court: get("court"), questionKey: get("questionKey"),
    caseId: get("caseId"), date: get("date"), status: get("status"), file,
    question: section("Question"), facts: section("Facts"), ruling: section("Ruling"),
    reasoning: section("Reasoning")
  };
}

export interface FileRulingInput {
  question: string; facts: string; ruling: string; reasoning: string; lawApplied: string[];
}

export interface Allocator {
  // returns citation like "[2026] VPS 7"; from M11 this is kernel enactment (§21.6)
  allocate(court: string, questionKey: string, caseId: string): Promise<string>;
}

export function interimAllocator(courtCode: string): Allocator {
  return {
    async allocate() {
      const rows = readIndex();
      const year = new Date().getFullYear();
      const max = rows
        .map(r => r.citation.match(/^\[(\d+)\]\s+\S+\s+(\d+)$/))
        .filter((m): m is RegExpMatchArray => !!m)
        .reduce((a, m) => Math.max(a, Number(m[2])), 0);
      return `[${year}] ${courtCode} ${max + 1}`;
    }
  };
}

export async function fileRuling(
  alloc: Allocator, court: "first-instance" | "appeals-court" | "supreme-court",
  questionKey: string, caseId: string, input: FileRulingInput
): Promise<Ruling> {
  const citation = await alloc.allocate(court, questionKey, caseId);
  const nnn = citation.match(/(\d+)$/)![1].padStart(3, "0");
  const slug = questionKey.replace(/[^a-z0-9]+/gi, "-").replace(/^-|-$/g, "").slice(0, 48);
  const rel = path.join("judgments", court, `${nnn}-${slug}.md`);
  const date = new Date().toISOString().slice(0, 10);
  const body = `---
citation: "${citation}"
court: ${court}
questionKey: "${questionKey}"
caseId: ${caseId}
date: ${date}
status: standing
---
## Question
${input.question}

## Facts
${input.facts}

## Ruling
${input.ruling}

## Reasoning
${input.reasoning}

## Law applied
${input.lawApplied.map(l => "- " + l).join("\n")}
`;
  fs.mkdirSync(path.dirname(justice(rel)), { recursive: true });
  fs.writeFileSync(justice(rel), body);
  if (!fs.existsSync(INDEX()))
    fs.writeFileSync(INDEX(), "# Citator\n\n| Citation | Court | Question key | File |\n|---|---|---|---|\n");
  fs.appendFileSync(INDEX(), `| ${citation} | ${court} | ${questionKey} | ${rel} |\n`);
  return { citation, court, questionKey, caseId, date, status: "standing", file: rel, ...input };
}

export function markOverturned(citation: string, byCitation: string): void {
  const row = readIndex().find(r => r.citation === citation);
  if (!row) throw new Error(`citation not found: ${citation}`);
  const p = justice(row.file);
  const src = fs.readFileSync(p, "utf8");
  fs.writeFileSync(p, src.replace(/^status: standing$/m, `status: overturned:${byCitation}`));
}
