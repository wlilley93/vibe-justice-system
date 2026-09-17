// Rule 15: the docket should be mostly about work.
//
// The rule has always said "it is measured, not sensed: a test counts it", and until now no
// such test existed. [2026] VJS 15 held that this made rule 15 UNENACTABLE as an operative
// bar under rule 6 — its only stated mechanism did not exist, so the condition could not be
// checked — and that until a test exists nobody may declare the mix breached, expressly
// including the bench that said so. It kept rule 15 alive as the SPECIFICATION any
// conforming test must satisfy, in five conditions. This is that test.
//
//   (a) count every ruling in the record, standing or superseded, once each
//   (b) each classified IN THE RECORD — declared, not inferred at run time — into exactly
//       one of machinery or work, with no unclassified remainder
//   (c) excluding the closed founding set named BY IDENTITY, being the genesis charter and
//       the four entrenched integrity statutes, and never a category answering to a
//       description
//   (d) failing when machinery exceeds work
//   (e) shipping one passing and one failing fixture
//
// CONDITION (b) IS WHY THIS TEST FAILS TODAY, AND THE FAILURE IS THE POINT. No record
// declares a classification. An earlier counter, written before VJS 15, inferred "machinery"
// from the string `constitution:` appearing in a questionKey — a category answering to a
// description, which (c) forbids precisely because a description quietly changes what it
// catches: rename a matter and the count moves without anyone deciding it should.
//
// The declaration cannot be added to the existing records by hand. [2026] VJS 2 denies hand
// edits to the record in terms, and it denied exactly that twice on 17 September. So a
// conforming count needs `class:` written by the court AT FILING, and the records filed
// before that mechanism exists are unclassified and stay unclassified. This test names them
// rather than guessing at them.
import { describe, it, expect } from "vitest";
import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

const repo = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "..");

// (c) THE FOUNDING SET, BY IDENTITY. Not "charters and statutes", not "anything entrenched"
// — those are descriptions, and a description admits whatever later matches it. These five
// ordinals and no others: a court cannot be constituted without rules about its own record,
// and they arrived in one commit before any work existed to govern.
const FOUNDING = new Set([1, 2, 3, 4, 5]);
const CLASSES = new Set(["machinery", "work"]);

interface Record { ordinal: number; declared: string | null; file: string }

function records(root: string): Record[] {
  const dir = path.join(root, ".justice", "judgments");
  if (!fs.existsSync(dir)) return [];
  const out: Record[] = [];
  for (const court of fs.readdirSync(dir)) {
    const courtDir = path.join(dir, court);
    if (!fs.statSync(courtDir).isDirectory()) continue;
    for (const name of fs.readdirSync(courtDir)) {
      if (!name.endsWith(".md")) continue;
      const text = fs.readFileSync(path.join(courtDir, name), "utf8");
      const front = /^---\n([\s\S]*?)\n---/.exec(text)?.[1] ?? "";
      // (a) ONCE EACH, STANDING OR SUPERSEDED. A superseded ruling is still a ruling the
      // court sat for; excluding it would let a docket of machinery be cleaned by appealing
      // it away. Nothing is deleted (r.3), so nothing is uncounted.
      const ordinal = Number(/^\s*citation:\s*"?\[\d{4}\]\s+\S+\s+(\d+)"?/m.exec(front)?.[1]);
      if (!Number.isFinite(ordinal)) continue;
      const declared = /^\s*class:\s*(\S+)\s*$/m.exec(front)?.[1] ?? null;
      out.push({ ordinal, declared, file: `${court}/${name}` });
    }
  }
  return out;
}

function count(all: Record[]) {
  const counted = all.filter((r) => !FOUNDING.has(r.ordinal));
  const unclassified = counted.filter((r) => r.declared === null || !CLASSES.has(r.declared));
  const machinery = counted.filter((r) => r.declared === "machinery").length;
  const work = counted.filter((r) => r.declared === "work").length;
  return { counted, unclassified, machinery, work };
}

describe("rule 15 — the docket should be mostly about work", () => {
  it("counts every ruling once, classified in the record, and fails when machinery leads", () => {
    const all = records(repo);
    expect(all.length, "the walk found no records at all").toBeGreaterThan(0);
    const { unclassified, machinery, work } = count(all);

    // (b) NO UNCLASSIFIED REMAINDER. Reported before the ratio, because a ratio computed
    // over a partial denominator is the failure this whole session kept finding: a number
    // that looks like a measurement and is a measurement of something else.
    expect(
      unclassified.map((r) => r.file),
      "these records declare no `class: machinery|work`, so the mix cannot be counted. " +
        "The declaration is written by the court at filing; it may NOT be added to an " +
        "existing record by hand — [2026] VJS 2 denies that in terms.",
    ).toEqual([]);

    // (d) FAILS WHEN MACHINERY EXCEEDS WORK.
    expect(
      machinery,
      `machinery ${machinery} vs work ${work}: rulings about this court's own machinery ` +
        "outnumber rulings about the work it governs, which is the v2 failure returning",
    ).toBeLessThanOrEqual(work);
  });

  // (e) ONE PASSING AND ONE FAILING FIXTURE. Rule 6: unfalsifiable law is unenactable, and
  // a test nobody has watched fail is a test nobody knows can.
  it("passes on a docket that is mostly work", () => {
    const fixture = [
      { ordinal: 1, declared: null, file: "founding" },
      { ordinal: 6, declared: "work", file: "a" },
      { ordinal: 7, declared: "work", file: "b" },
      { ordinal: 8, declared: "machinery", file: "c" },
    ];
    const { unclassified, machinery, work } = count(fixture);
    expect(unclassified).toEqual([]);
    expect(machinery).toBeLessThanOrEqual(work);
  });

  it("fails on a docket that is mostly machinery", () => {
    const fixture = [
      { ordinal: 6, declared: "machinery", file: "a" },
      { ordinal: 7, declared: "machinery", file: "b" },
      { ordinal: 8, declared: "work", file: "c" },
    ];
    const { machinery, work } = count(fixture);
    expect(machinery).toBeGreaterThan(work);
  });

  it("does not let the founding set be recognised by a description", () => {
    // A charter or an entrenched statute filed LATER is not founding. If this test excluded
    // "anything entrenched", a future entrenched machinery ruling would exclude itself from
    // the count it is meant to be in.
    const fixture = [
      { ordinal: 20, declared: "machinery", file: "late-entrenched-statute" },
      { ordinal: 21, declared: "work", file: "ordinary" },
    ];
    const { counted } = count(fixture);
    expect(counted.map((r) => r.ordinal)).toEqual([20, 21]);
  });
});
