// §21.6: from M11 the citation allocator IS the kernel book — filing a ruling enacts
// its spine. Pre-M11 (no book.json yet) the interim max+1 allocator serves.
import fs from "node:fs";
import { interimAllocator, type Allocator } from "../court/store.js";
import { loadConfig } from "../llm/provider.js";
import { justice } from "../paths.js";

export async function kernelAllocator(): Promise<Allocator> {
  const cfg = loadConfig();
  if (!fs.existsSync(justice("book.json"))) return interimAllocator(cfg.citationCourtCode);
  const { readBook, nextOrdinal } = await import("./book.js");
  const { enact } = await import("./enact.js");
  return {
    async allocate(court, questionKey, caseId) {
      const entries = readBook();
      const year = new Date().getFullYear();
      const ordinal = nextOrdinal(entries);
      const slug = `ruling${year}N${ordinal}`;
      const res = await enact({
        year, ordinal, slug, kind: "ruling", rule: { type: "free" },
        entrenched: false, supersedes: null,
        authority: { type: "derived", parent: { year: 2026, ordinal: 1 } },
        title: `Ruling on ${questionKey}`, summary: `Filed from case ${caseId} (${court}). Payload in .justice/judgments/.`
      });
      if (!res.ok) throw Object.assign(new Error(`enactment failed for ${slug}: ${res.diagnostics}`), { code: 4 });
      // The court code comes from config, never a literal. `interimAllocator` already
      // read it; this path did not, so any second jurisdiction would have minted VJS
      // citations regardless of its own configuration.
      return `[${year}] ${cfg.citationCourtCode} ${ordinal}`;
    }
  };
}

// appeal = supersession (§21.6)
export async function enactOverturn(originalCitation: string, questionKey: string): Promise<string> {
  const { readBook, nextOrdinal } = await import("./book.js");
  const { enact } = await import("./enact.js");
  const m = originalCitation.match(/^\[(\d+)\]\s+\S+\s+(\d+)$/);
  if (!m) throw new Error("bad citation: " + originalCitation);
  const entries = readBook();
  const year = new Date().getFullYear();
  const ordinal = nextOrdinal(entries);
  const res = await enact({
    year, ordinal, slug: `ruling${year}N${ordinal}`, kind: "ruling", rule: { type: "free" },
    entrenched: false, supersedes: { year: Number(m[1]), ordinal: Number(m[2]) },
    authority: { type: "derived", parent: { year: 2026, ordinal: 1 } },
    title: `Appeal ruling on ${questionKey}`, summary: `Supersedes ${originalCitation}.`
  });
  if (!res.ok) throw Object.assign(new Error(`appeal enactment failed: ${res.diagnostics}`), { code: 4 });
  return `[${year}] ${loadConfig().citationCourtCode} ${ordinal}`;
}
