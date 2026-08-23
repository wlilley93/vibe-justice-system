// The citator (.justice/INDEX.md) is the memo table res judicata is built on: `lookup()`
// reads it to decide whether a question has already been answered. If a ruling is in the
// book but missing from the index, the question it settled becomes litigable again — the
// system would re-decide something already decided, and could decide it differently, while
// `res_judicata` still compiles because the theorem is about the table's consistency with
// the gate, not about the table being complete.
//
// This exact failure happened on 2026-08-23 and was silent for a whole session: a
// mechanical const->function refactor rewrote `justice("INDEX.md")` to
// `justice("INDEX().md")`, so reader and writer agreed on the wrong path and eight standing
// rulings were invisible to lookup. Consistency between two wrong things is not detectable
// from inside either of them; only this invariant catches it.
import { describe, it, expect } from "vitest";
import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

const repo = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "..");
const read = (p: string) => fs.readFileSync(path.join(repo, p), "utf8");

const bookRulings = (): number[] =>
  JSON.parse(read(".justice/book.json"))
    .filter((e: any) => e.kind === "ruling").map((e: any) => e.ordinal).sort((a: number, b: number) => a - b);

const indexRows = (): number[] =>
  [...read(".justice/INDEX.md").matchAll(/^\|\s*\[\d{4}\]\s+[A-Z]+\s+(\d+)\s*\|/gm)]
    .map(m => Number(m[1])).sort((a, b) => a - b);

describe("the citator", () => {
  it("holds a row for every ruling in the book", () => {
    const missing = bookRulings().filter(o => !indexRows().includes(o));
    expect(missing).toEqual([]);
  });

  it("holds no row for a ruling that is not in the book", () => {
    const orphans = indexRows().filter(o => !bookRulings().includes(o));
    expect(orphans).toEqual([]);
  });

  it("names a judgment file that exists, for every row", () => {
    const missing: string[] = [];
    // Anchor on a citation so the header row and the |---| separator are skipped.
    for (const m of read(".justice/INDEX.md").matchAll(/^\|\s*\[\d{4}\][^|]*\|[^|]+\|[^|]+\|\s*([^|]+?)\s*\|/gm)) {
      const f = path.join(repo, ".justice", m[1]);
      if (!fs.existsSync(f)) missing.push(m[1]);
    }
    expect(missing).toEqual([]);
  });

  it("has exactly one citator file", () => {
    // The bug produced a second, plausibly-named index alongside the real one.
    const strays = fs.readdirSync(path.join(repo, ".justice"))
      .filter(f => /INDEX/i.test(f) && f !== "INDEX.md");
    expect(strays).toEqual([]);
  });
});
