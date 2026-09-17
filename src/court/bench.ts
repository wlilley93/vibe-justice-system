// §11.5: First Instance (automated) + Appeal (three personas + synthesis).
import fs from "node:fs";
import { z } from "zod";
import type { Config, LLMProvider } from "../llm/provider.js";
import { completeJson } from "../llm/json.js";
import { renderPrompt } from "../prompts.js";
import { justice } from "../paths.js";

// `classification` is the bench's own statement of what KIND of question it just
// decided, required by [2026] VJS 15. The bench is asked because it is the party
// that knows: it has read the question, the facts and the law in force, and it is
// already writing the record. Asking the filer instead would let machinery be
// filed as work by whoever wanted a quieter count, and rule 15 exists because
// that drift is invisible.
const FIRuling = z.object({
  ruling: z.string(), reasoning: z.string(), lawApplied: z.array(z.string()),
  classification: z.enum(["machinery", "work"]),
});
export type FIRulingT = z.infer<typeof FIRuling>;

export async function firstInstance(
  provider: LLMProvider, cfg: Config, caseId: string,
  q: { key: string; text: string; facts: string }, priorRulings: string
): Promise<FIRulingT> {
  const specLaw = fs.existsSync(justice("SPEC-LAW.md")) ? fs.readFileSync(justice("SPEC-LAW.md"), "utf8") : "";
  const { system, user } = renderPrompt("judge-first-instance", {
    question: q.text, facts: q.facts, specLaw, priorRulings
  });
  return completeJson(provider, FIRuling, {
    promptName: `judge-first-instance.${q.key.replace(/[^a-z0-9]+/gi, "-")}`,
    caseId, system, user
  }, cfg.maxJsonRepairs);
}

const AppealOut = z.object({ upheld: z.boolean(), ruling: z.string(), reasoning: z.string() });
export type AppealOutT = z.infer<typeof AppealOut>;

export async function appeal(
  provider: LLMProvider, cfg: Config, caseId: string,
  original: { citation: string; question: string; ruling: string }, grounds: string
): Promise<AppealOutT> {
  const personas = ["textualist", "purposivist", "pragmatist"];
  const opinions: string[] = [];
  for (const persona of personas) {
    const { system, user } = renderPrompt("judge-appeal", {
      persona, citation: original.citation, question: original.question,
      ruling: original.ruling, challenge: grounds, opinions: ""
    });
    const text = await provider.complete({ promptName: `judge-appeal.${persona}`, caseId, system, user });
    opinions.push(`### ${persona}\n${text}`);
  }
  const { system, user } = renderPrompt("judge-appeal", {
    persona: "synthesis", citation: original.citation, question: original.question,
    ruling: original.ruling, challenge: grounds, opinions: opinions.join("\n\n")
  });
  return completeJson(provider, AppealOut, { promptName: "judge-appeal.synthesis", caseId, system, user }, cfg.maxJsonRepairs);
}
