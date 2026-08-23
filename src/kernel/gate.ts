// §21.7: the meta-gate — Facts extraction (trusted, dumb) + gate evaluation.
import fs from "node:fs";
import path from "node:path";
import { execa } from "execa";
import { readBook, citeOf, type BookEntry } from "./book.js";
import { leanAvailable } from "../lean/runner.js";
import { leanDir, leanFile, unverifiedLedger, root } from "../paths.js";

export interface Facts { pathsChanged: string[]; recordsAdded: number }
export interface GateVerdict { allow: boolean; cites: { citation: string; title: string; summary: string }[]; via: "lean" | "ts-mirror" }

export async function stagedFacts(): Promise<Facts> {
  // cwd must be the repo, not the caller's: the pre-commit hook runs from the git
  // root but `vjs gate` may be invoked from anywhere once --root exists.
  const r = await execa("git", ["diff", "--cached", "--name-status"], { reject: false, cwd: root() });
  const lines = r.stdout.split("\n").filter(Boolean);
  const paths = lines.map(l => l.split("\t").pop()!).filter(Boolean);
  const records = lines.filter(l => l.startsWith("A") && (l.split("\t").pop() ?? "").startsWith("record/")).length;
  return { pathsChanged: paths, recordsAdded: records };
}

// TS mirror of the kernel gate semantics (Legitimacy/Gate.lean). Used when Lean is
// unavailable; the Lean evaluation is queued so the mirror never silently rules alone.
export function evalGateTs(entries: BookEntry[], f: Facts): { allow: boolean; cites: BookEntry[] } {
  const effective = (e: BookEntry) =>
    !entries.some(j => j.supersedes && j.supersedes.year === e.year && j.supersedes.ordinal === e.ordinal);
  const violated = (e: BookEntry): boolean => {
    if (e.rule.type === "free") return false;
    const hit = f.pathsChanged.some(p => p.startsWith((e.rule as any).scope));
    if (e.rule.type === "pathForbidden") return hit;
    return hit && f.recordsAdded === 0;
  };
  const cites = entries.filter(e => effective(e) && violated(e));
  return { allow: cites.length === 0, cites };
}

export async function evaluateGate(f: Facts): Promise<GateVerdict> {
  const entries = readBook();
  if (await leanAvailable()) {
    const eval1 = `import Vjs.Book\n#eval Vjs.gate { pathsChanged := [${f.pathsChanged.map(p => JSON.stringify(p)).join(", ")}], recordsAdded := ${f.recordsAdded} }\n`;
    const rel = path.join("Vjs", "GateEval.lean");
    fs.writeFileSync(leanFile(rel), eval1);
    const r = await execa("lake", ["env", "lean", rel], { cwd: leanDir(), timeout: 120000, reject: false });
    const out = r.stdout + r.stderr;
    if (out.includes("Vps.Verdict.allow")) return { allow: true, cites: [], via: "lean" };
    const ords = [...out.matchAll(/ordinal\s*:=\s*(\d+)/g)].map(m => Number(m[1]));
    const cited = entries.filter(e => ords.includes(e.ordinal));
    // A denial that names no instrument is not a denial -- `every_deny_names_its_law` is a
    // theorem, so if we got here with nothing cited the eval itself broke and we are about
    // to report breakage as law. Fail as an internal error (exit 1), loudly, rather than
    // let a compile failure wear the costume of a constitutional refusal.
    if (!cited.length) {
      throw Object.assign(new Error(
        `gate evaluation produced no verdict — the kernel eval failed rather than denying.\n` +
        out.split("\n").filter(Boolean).slice(0, 6).map(l => "  " + l).join("\n")
      ), { code: 1 });
    }
    return { allow: false, cites: cited.map(e => ({ citation: citeOf(e), title: e.title, summary: e.summary })), via: "lean" };
  }
  fs.appendFileSync(unverifiedLedger(), `- [ ] gate eval via Lean for facts ${JSON.stringify(f)}\n`);
  const { allow, cites } = evalGateTs(entries, f);
  return { allow, cites: cites.map(e => ({ citation: citeOf(e), title: e.title, summary: e.summary })), via: "ts-mirror" };
}
