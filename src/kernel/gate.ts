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

// There is deliberately NO TypeScript mirror of the gate here any more.
//
// `evalGateTs` reimplemented `Vps.decideVerdict` by hand and ruled whenever Lean was absent,
// with nothing proving the two agreed. ASSESSMENT.md names that exact object as the disease
// it refused: "a separate formal model must be kept in correspondence with the implementation
// — which is precisely a new watcher watching a watcher." It was also fail-OPEN dressed as a
// fallback: an unproved copy quietly deciding constitutional questions.
//
// Without Lean the gate now fails CLOSED (LEARNINGS #17). A jurisdiction whose kernel cannot
// be consulted has not allowed the change; it has failed to judge it, and those differ.

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
  throw Object.assign(new Error(
    "the kernel is unavailable, so this change cannot be judged.\n" +
    "  The gate fails closed: an unjudged change is not an allowed change.\n" +
    "  Install the pinned toolchain (elan, leanprover/lean4:v4.15.0) and retry.\n" +
    "  The requested evaluation is queued in UNVERIFIED-LEAN.md."
  ), { code: 4 });
}
