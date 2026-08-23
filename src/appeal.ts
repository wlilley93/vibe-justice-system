// §11.5 appeal: three personas + synthesis; if not upheld, supersede via the kernel.
import fs from "node:fs";
import { readIndex, readJudgment, fileRuling, markOverturned } from "./court/store.js";
import { appeal } from "./court/bench.js";
import { getProvider, loadConfig } from "./llm/provider.js";
import { kernelAllocator, enactOverturn } from "./kernel/bridge.js";
import { justice } from "./paths.js";

export async function runAppeal(citation: string, grounds: string): Promise<void> {
  const cfg = loadConfig();
  const provider = await getProvider(cfg);
  const row = readIndex().find(r => r.citation === citation);
  if (!row) throw new Error("citation not found in citator: " + citation);
  const j = readJudgment(row.file);
  if (!j) throw new Error("judgment file unreadable: " + row.file);
  const out = await appeal(provider, cfg, j.caseId || "appeal", {
    citation, question: j.question ?? "", ruling: j.ruling ?? ""
  }, grounds);
  if (out.upheld) { console.log("UPHELD — the original ruling stands.\n" + out.reasoning); return; }
  let newCitation: string;
  if (fs.existsSync(justice("book.json"))) newCitation = await enactOverturn(citation, j.questionKey);
  else {
    const alloc = await kernelAllocator();
    newCitation = (await fileRuling(alloc, "appeals-court", j.questionKey, j.caseId, {
      question: j.question ?? "", facts: j.facts ?? "", ruling: out.ruling, reasoning: out.reasoning,
      lawApplied: ["appeal of " + citation]
    })).citation;
  }
  if (fs.existsSync(justice("book.json"))) {
    // payload judgment for the superseding ruling
    const alloc = { async allocate() { return newCitation; } };
    await fileRuling(alloc as any, "appeals-court", j.questionKey, j.caseId, {
      question: j.question ?? "", facts: j.facts ?? "", ruling: out.ruling, reasoning: out.reasoning,
      lawApplied: ["appeal of " + citation]
    });
  }
  markOverturned(citation, newCitation);
  console.log(`OVERTURNED — ${citation} superseded by ${newCitation}`);
}
