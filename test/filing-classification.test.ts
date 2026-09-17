// The bench's classification must actually reach the record.
//
// [2026] VJS 15 requires the classification to be DECLARED in the record. The schema asks
// the bench for it and `fileRuling` writes it, and between those two is the failure that
// would be invisible: a field accepted by the schema, dropped on the way to disk, and a
// docket-mix count that reads every new ruling as unclassified for ever while the benches
// dutifully classify. The counting test would go red and the cause would be three files
// away.
//
// So this asserts the frontmatter, not the schema. It writes into a scratch root through
// VJS_ROOT rather than the real jurisdiction, because a test that files a ruling into the
// record is a test that legislates.
import { describe, it, expect, beforeAll, afterAll } from "vitest";
import fs from "node:fs";
import os from "node:os";
import path from "node:path";

let scratch: string;
let previous: string | undefined;

// ONE SCRATCH FOR THE FILE, NOT ONE PER TEST. `paths.ts` memoises the jurisdiction
// root at first import, so a per-test root is read once and ignored afterwards: the
// writes all land in the first scratch while later tests read their own and find
// nothing. Measured - two tests passed by ordering and the third did not, which is
// the worst possible way for this to be wrong.
beforeAll(() => {
  scratch = fs.mkdtempSync(path.join(os.tmpdir(), "vjs-filing-"));
  fs.mkdirSync(path.join(scratch, ".justice", "judgments", "first-instance"), { recursive: true });
  previous = process.env.VJS_ROOT;
  process.env.VJS_ROOT = scratch;
});

afterAll(() => {
  if (previous === undefined) delete process.env.VJS_ROOT;
  else process.env.VJS_ROOT = previous;
  fs.rmSync(scratch, { recursive: true, force: true });
});

const allocator = (citation: string) => ({ allocate: async () => citation });

const input = {
  question: "q", facts: "{}", ruling: "r", reasoning: "because",
  lawApplied: ["the law of this court, II.5"],
};

describe("a filed ruling carries its classification", () => {
  it("writes `class:` into the frontmatter when the bench declares one", async () => {
    const { fileRuling } = await import("../src/court/store.js");
    const filed = await fileRuling(allocator("[2026] VJS 99") as any, "first-instance",
      "work:some-question", "matter", { ...input, classification: "work" });
    const text = fs.readFileSync(path.join(scratch, ".justice", filed.file), "utf8");
    expect(text).toMatch(/^class: work$/m);
  });

  it("round-trips the declaration back out of the record", async () => {
    const store = await import("../src/court/store.js");
    const filed = await store.fileRuling(allocator("[2026] VJS 98") as any, "first-instance",
      "machinery:some-question", "matter", { ...input, classification: "machinery" });
    const read = store.readJudgment(filed.file);
    expect(read?.classification).toBe("machinery");
  });

  it("omits the line entirely rather than writing an empty one", async () => {
    // An `class: ` with nothing after it parses as a declaration and is not one. The
    // counting test treats absent and invalid alike, and this keeps the record from
    // carrying a field that looks answered.
    const { fileRuling } = await import("../src/court/store.js");
    const filed = await fileRuling(allocator("[2026] VJS 97") as any, "first-instance",
      "old:style", "matter", input);
    const text = fs.readFileSync(path.join(scratch, ".justice", filed.file), "utf8");
    expect(text).not.toMatch(/^class:/m);
  });
});
