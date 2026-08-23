// book_lawful is a nested `Lawful.enact` term, one layer per instrument, with two `by decide`
// calls per layer and each traversing the whole book. Its cost therefore grows ~quadratically
// in rulings filed, and every case Foundry runs files about four.
//
// That is a real property and NOT yet a real problem: measured 2026-08-23, a cold
// `lake build Vps` at 29 instruments takes 1.7s against enact()'s 120s timeout — and enact
// treats a timeout as "the enactment never happened", so the failure mode when it does
// arrive is that filing a ruling breaks for reasons unrelated to law.
//
// The fix (a decidable `lawfulB` checker plus a one-time soundness theorem, so book_lawful
// becomes a single application) is constitutional surgery on a verified kernel, which must
// be enacted upstream in VPS and re-proved. Doing that now, for a 1.7-second build, would be
// exactly the meta-work-about-machinery that ASSESSMENT.md identifies as what killed VJS v2.
//
// So instead of remembering to check, this test makes the trigger fire on its own. It is a
// budget, not a benchmark: it fails only when the margin has genuinely eroded.
import { describe, it, expect } from "vitest";
import { execFile } from "node:child_process";
import { promisify } from "node:util";
import fs from "node:fs";
import os from "node:os";
import path from "node:path";
import { fileURLToPath } from "node:url";

const exec = promisify(execFile);
const repo = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "..");
const lake = path.join(os.homedir(), ".elan", "bin", "lake");

// enact() times out at 120_000ms. Fail well before that: at a quarter of the budget there is
// still time to do the lawfulB work deliberately rather than under pressure.
const BUDGET_MS = 30_000;

describe("statute book build budget", () => {
  it("rebuilds well inside enact()'s timeout", async () => {
    if (!fs.existsSync(lake)) return;   // no toolchain here; the CI job covers it
    const t0 = Date.now();
    await exec(lake, ["build", "Vjs"], { cwd: path.join(repo, "lean"), timeout: 180_000 });
    const ms = Date.now() - t0;
    const n = JSON.parse(fs.readFileSync(path.join(repo, ".justice", "book.json"), "utf8")).length;
    if (ms > BUDGET_MS) {
      throw new Error(
        `lake build Vjs took ${ms}ms at ${n} instruments, past the ${BUDGET_MS}ms budget.\n` +
        `book_lawful's nested-enact term has outgrown its shape. Enact a decidable lawfulB\n` +
        `checker plus a soundness theorem UPSTREAM in vibe-proof-system, re-prove, then\n` +
        `re-vendor — before enact()'s 120s timeout starts failing enactments for reasons\n` +
        `that have nothing to do with law.`
      );
    }
    expect(ms).toBeLessThan(BUDGET_MS);
  }, 200_000);
});
