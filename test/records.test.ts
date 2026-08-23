// Every enacted ruling must keep its record stub.
//
// enact() writes `record/<NNNN>.md` itself, auto-numbered. A human writing a record by hand
// picks the next number by eye, and if an enactment has taken it since, the write is a
// silent overwrite: the file still parses as a record, nothing complains, and the only
// evidence that a ruling was enacted is gone. That happened on 2026-08-23 (record/0027.md).
// The failure mode is silent, so it needs an assertion rather than care.
import { describe, it, expect } from "vitest";
import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

const repo = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "..");

describe("the record", () => {
  it("holds an enactment stub for every ruling in the book", () => {
    const book = JSON.parse(fs.readFileSync(path.join(repo, ".justice", "book.json"), "utf8"));
    // Read the court code from config rather than assuming it — this test now guards any
    // jurisdiction, and a second one is coming.
    const code = JSON.parse(fs.readFileSync(path.join(repo, "vjs.config.json"), "utf8")).citationCourtCode ?? "VJS";
    const rulings: string[] = book.filter((e: any) => e.kind === "ruling").map((e: any) => `${code} ${e.ordinal}`);

    const recorded = new Set<string>();
    const dir = path.join(repo, "record");
    for (const f of fs.readdirSync(dir).filter(f => f.endsWith(".md"))) {
      const text = fs.readFileSync(path.join(dir, f), "utf8");
      for (const m of text.matchAll(new RegExp(`\\[\\d{4}\\] (${code} \\d+) enacted`, "g"))) recorded.add(m[1]);
    }

    const missing = rulings.filter(r => !recorded.has(r));
    expect(missing).toEqual([]);
  });

  it("numbers each record file consistently with its name", () => {
    const dir = path.join(repo, "record");
    const bad: string[] = [];
    for (const f of fs.readdirSync(dir).filter(f => /^\d{4}\.md$/.test(f))) {
      const n = f.slice(0, 4);
      const head = fs.readFileSync(path.join(dir, f), "utf8").split("\n")[0];
      // A trailing em-dash title is allowed — "# Record 0001 — v3 constituted" reads better
      // in a directory listing than a bare number. What must match is the number.
      if (!head.trim().startsWith(`# Record ${n}`)) bad.push(`${f}: ${head.trim()}`);
    }
    expect(bad).toEqual([]);
  });
});
