// Full-text search over the case law.
//
// THE BOUNDARY THIS MUST NOT CROSS, stated first because it is the whole risk:
//
//   `lookup(questionKey)` is an EXACT match on a hashed key. That exactness is what
//   res judicata means and what the kernel's `res_judicata` theorem is about — a memo table
//   that provably cannot disagree with deliberation. Nothing in this file may be wired into
//   that path. If "find me a similar ruling" ever fed the pipeline's precedent lookup, the
//   same question could be answered two ways depending on a ranking function, and the
//   theorem would still compile while meaning nothing.
//
//   So: `lookup` is how the MACHINE addresses a ruling. This is how a PERSON consults the
//   record. They are different operations and stay separate. test/search.test.ts asserts it.
//
// Why it exists at all: `precedents --grep` matched a substring against one index line
// holding citation + court + questionKey + file path, so you could find `interpret:` or a
// case slug but not "attitude clause" or "NPV" — the substance of every ruling was parsed by
// readJudgment and then discarded at the listing boundary. A record the pipeline can hit but
// a person cannot consult is not a body of case law.
import fs from "node:fs";
import path from "node:path";
import { justice } from "../paths.js";
import { readIndex, readJudgment, type Ruling } from "./store.js";

export interface Hit {
  ruling: Ruling;
  score: number;
  /** Which fields matched, for showing the reader why this came back. */
  matched: string[];
  /** One line of context per matching field, with the terms marked. */
  snippets: { field: string; text: string }[];
}

export interface SearchOpts {
  /** Restrict to standing rulings. Overturned law is still law that was, so the default
   *  shows everything and marks it — hiding superseded rulings would misrepresent the
   *  record, which is append-only precisely so that nothing disappears. */
  standingOnly?: boolean;
  court?: string;
  caseId?: string;
  limit?: number;
}

/** `## Law applied` is written by fileRuling and never read back by readJudgment, so it has
 *  been invisible to everything. It is exactly what you want to search on ("which rulings
 *  leaned on entrenchment?"), so recover it here rather than leaving it write-only. */
function lawApplied(file: string): string[] {
  const p = justice(file);
  if (!fs.existsSync(p)) return [];
  const m = fs.readFileSync(p, "utf8").match(/## Law applied\n([\s\S]*?)(?=\n## |$)/);
  if (!m) return [];
  return m[1].split("\n").map(l => l.replace(/^\s*-\s*/, "").trim()).filter(Boolean);
}

// Field weights. The question and the ruling are what a reader is usually looking for; the
// reasoning is where the argument lives; `facts` is a canonical JSON-ish blob and is noisy,
// so it counts but counts least.
const WEIGHTS: Record<string, number> = {
  question: 5, ruling: 5, questionKey: 4, reasoning: 3, lawApplied: 3, citation: 4, facts: 1
};

const terms = (q: string): string[] =>
  q.toLowerCase().split(/\s+/).map(t => t.replace(/^["']|["']$/g, "")).filter(Boolean);

function snippetOf(text: string, ts: string[], width = 160): string {
  const low = text.toLowerCase();
  let at = -1;
  for (const t of ts) { const i = low.indexOf(t); if (i >= 0 && (at < 0 || i < at)) at = i; }
  if (at < 0) return text.slice(0, width).replace(/\s+/g, " ").trim();
  const start = Math.max(0, at - width / 3);
  const raw = text.slice(start, start + width).replace(/\s+/g, " ").trim();
  return (start > 0 ? "…" : "") + raw + (start + width < text.length ? "…" : "");
}

export function searchPrecedents(query: string, opts: SearchOpts = {}): Hit[] {
  const ts = terms(query);
  if (!ts.length) return [];

  const hits: Hit[] = [];
  for (const row of readIndex()) {
    const j = readJudgment(row.file);
    if (!j) continue;
    const r: Ruling = { ...row, ...j };
    if (opts.standingOnly && !r.status.startsWith("standing")) continue;
    if (opts.court && r.court !== opts.court) continue;
    if (opts.caseId && r.caseId !== opts.caseId) continue;

    const fields: Record<string, string> = {
      question: r.question ?? "", ruling: r.ruling ?? "", reasoning: r.reasoning ?? "",
      facts: r.facts ?? "", questionKey: r.questionKey, citation: r.citation,
      lawApplied: lawApplied(r.file).join(" · ")
    };

    // AND semantics: every term must appear somewhere in the document. A ruling that
    // mentions only half of what you asked for is not the ruling you were looking for.
    const doc = Object.values(fields).join(" ").toLowerCase();
    if (!ts.every(t => doc.includes(t))) continue;

    let score = 0;
    const matched: string[] = [];
    const snippets: { field: string; text: string }[] = [];
    for (const [name, text] of Object.entries(fields)) {
      if (!text) continue;
      const low = text.toLowerCase();
      let n = 0;
      for (const t of ts) {
        let i = low.indexOf(t);
        while (i >= 0) { n++; i = low.indexOf(t, i + t.length); }
      }
      if (!n) continue;
      score += n * (WEIGHTS[name] ?? 1);
      matched.push(name);
      if (snippets.length < 3 && name !== "citation" && name !== "questionKey") {
        snippets.push({ field: name, text: snippetOf(text, ts) });
      }
    }
    // Standing law ranks above superseded law, without hiding the latter.
    if (r.status.startsWith("standing")) score *= 1.25;
    hits.push({ ruling: { ...r, lawApplied: lawApplied(r.file) }, score: Math.round(score * 100) / 100, matched, snippets });
  }

  hits.sort((a, b) => b.score - a.score || b.ruling.citation.localeCompare(a.ruling.citation));
  return typeof opts.limit === "number" ? hits.slice(0, opts.limit) : hits;
}
