#!/usr/bin/env node
/**
 * VJS — the court, standing on its own.
 *
 * v1 was a prose realm: real doctrine, nothing enforced. v2 was a Rust kernel that needed
 * ever more law to trust itself. v3 is neither: the court is a small TypeScript program over
 * a Lean statute book it does not own, consumed as a pinned dependency from the Vibe Proof
 * System. What is proved stays proved upstream; what is judged stays judged here.
 *
 * WHAT THIS IS FOR, and why it is separate.
 *
 * A court answers one question — "has this been decided, and what was decided" — for
 * arbitrary questions, and remembers the answer. Nothing about that requires a repository,
 * a specification, or a build. Rulings carry `rule: free`: they have a citation, a standing,
 * an authority chain and a payload, and they bind nothing by themselves. That is precisely
 * what makes the court reusable — an operational decision ("may this deploy divert egress?")
 * is a question with a key, exactly like a modelling decision.
 *
 * THE ONE BOUNDARY. `lookup(questionKey)` is an EXACT match on a hashed key. That exactness
 * IS res judicata. `search` is a separate, human-facing affordance and is never wired into
 * lookup — a ranking function anywhere near the memo table would let the same question be
 * answered two ways while the theorem still compiled.
 *
 * Exit codes: 0 ok · 2 human gate · 4 lean · 5 provider · 6 constitutional denial · 1 error.
 */
import { Command } from "commander";
import path from "node:path";
import { R, root, setRoot, resolveRoot, invocationCwd } from "./paths.js";
import { setOutMode, say, warn, finish, fail } from "./out.js";
import { withLock } from "./lock.js";

const program = new Command().name("vjs")
  .description("The court: precedent, rulings, appeals and res judicata over a proved statute book.");

const die = (e: any): never => fail("vjs", e);

program
  .option("--root <path>", "jurisdiction root (default: $VJS_ROOT, else nearest ancestor with vjs.config.json)")
  .option("--config <path>", "path to vjs.config.json")
  .option("--provider <p>", "llm provider: cli | api | mock")
  .option("--wait <seconds>", "wait for the jurisdiction lock instead of failing fast", "0")
  .option("--json", "emit one machine-readable envelope on stdout");

program.hook("preSubcommand", (cmd) => {
  const o = cmd.opts();
  if (o.config) process.env.VJS_CONFIG = path.resolve(invocationCwd, o.config);
  if (o.provider) process.env.VJS_PROVIDER = o.provider;
  if (o.json) setOutMode("json");
  setRoot(resolveRoot(o.root));
});

const mutating = <A extends unknown[]>(name: string, fn: (...a: A) => Promise<void> | void) =>
  async (...a: A) => {
    const waitMs = Number(program.opts().wait ?? 0) * 1000;
    try { await withLock(name, async () => { await fn(...a); }, { waitMs }); }
    catch (e) { die(e); }
  };

// ---------------------------------------------------------------- the record

program.command("ask <key>")
  .description("has this question been decided? (exact key — this is res judicata, not search)")
  .action(async (key: string) => {
    const { lookup } = await import("./court/store.js");
    const r = lookup(key);
    if (!r) { say(`open: no standing ruling on ${key}`); finish("ask", { result: { key, standing: null } }); }
    say(`${r.citation}  ${r.court}  ${r.status}`);
    say(`  ${r.question}`);
    say(`  ${r.ruling}`);
    finish("ask", { result: { key, standing: r } });
  });

program.command("rule <key>")
  .description("put a question to the bench and file the ruling")
  .requiredOption("--question <q>", "the question, in words")
  .requiredOption("--facts <f>", "the facts the ruling is about")
  .option("--matter <m>", "what this question arose from", "adhoc")
  .option("--court <c>", "first-instance | appeals-court | supreme-court", "first-instance")
  .action(mutating("rule", async (key: string, o: any) => {
    const { lookup, fileRuling } = await import("./court/store.js");
    const { firstInstance } = await import("./court/bench.js");
    const { getProvider, loadConfig } = await import("./llm/provider.js");
    const { kernelAllocator } = await import("./kernel/bridge.js");

    // Res judicata, enforced before anything is spent: a settled question is answered from
    // the record, not re-litigated. This is the fast path the theorem is about.
    const standing = lookup(key);
    if (standing) {
      say(`already decided: ${standing.citation}`);
      say(`  ${standing.ruling}`);
      finish("rule", { result: { key, reused: true, ruling: standing } });
    }

    const cfg = loadConfig();
    const provider = await getProvider(cfg);
    const opinion = await firstInstance(provider, cfg, o.matter, { key, text: o.question, facts: o.facts }, "");
    const alloc = await kernelAllocator();
    const filed = await fileRuling(alloc, o.court, key, o.matter, {
      question: o.question, facts: o.facts,
      ruling: opinion.ruling, reasoning: opinion.reasoning, lawApplied: opinion.lawApplied
    });
    say(`${filed.citation} filed -> ${filed.file}`);
    finish("rule", { result: { key, reused: false, ruling: filed } });
  }));

program.command("appeal <citation>")
  .description("challenge a standing ruling; three benches and a synthesis")
  .requiredOption("--grounds <g>")
  .action(mutating("appeal", async (cit: string, o: any) => {
    const { runAppeal } = await import("./appeal.js");
    await runAppeal(cit, o.grounds);
    finish("appeal", { result: { citation: cit } });
  }));

program.command("search <query>")
  .description("full text across every judgment (NOT res judicata — see `ask`)")
  .option("--standing").option("--court <c>").option("--limit <n>")
  .action(async (q: string, o: any) => {
    const { searchPrecedents } = await import("./court/search.js");
    const hits = searchPrecedents(q, {
      standingOnly: !!o.standing, court: o.court, limit: o.limit ? Number(o.limit) : undefined
    });
    for (const h of hits) {
      const mark = h.ruling.status.startsWith("standing") ? "" : `  [${h.ruling.status}]`;
      say(`${h.ruling.citation}  ${h.ruling.court}  ${h.ruling.questionKey}${mark}`);
      say(`    ${h.ruling.question}`);
      for (const s of h.snippets) say(`    ${s.field}: ${s.text}`);
      say("");
    }
    say(`${hits.length} ruling${hits.length === 1 ? "" : "s"} matched.`);
    finish("search", { result: { query: q, hits } });
  });

program.command("docket")
  .description("every ruling on the record")
  .option("--grep <g>")
  .action(async (o: any) => {
    const { readIndex } = await import("./court/store.js");
    const rows = readIndex().filter(r => !o.grep || `${r.citation} ${r.court} ${r.questionKey}`.includes(o.grep));
    for (const r of rows) say(`${r.citation}  ${r.court}  ${r.questionKey}`);
    finish("docket", { result: { rulings: rows } });
  });

// ---------------------------------------------------------------- the book

program.command("book")
  .description("the statute book in force")
  .action(async () => {
    const { readBook, citeOf } = await import("./kernel/book.js");
    const entries = readBook();
    for (const e of entries) {
      const rule = e.rule.type === "free" ? "free" : `${e.rule.type} ${(e.rule as any).scope}`;
      const superseded = entries.some(x => x.supersedes && x.supersedes.year === e.year && x.supersedes.ordinal === e.ordinal);
      say(`${citeOf(e)}  ${e.kind}  ${rule}  ${e.entrenched ? "ENTRENCHED" : ""}  ${superseded ? "superseded" : "standing"}  — ${e.title}`);
    }
    finish("book", { result: entries });
  });

program.command("gate")
  .description("evaluate a change against the book; exit 6 on denial, each denial naming its law")
  .option("--staged").option("--paths <p>").option("--records <n>")
  .action(async (o: any) => {
    const { stagedFacts, evaluateGate } = await import("./kernel/gate.js");
    let facts;
    if (o.staged) facts = await stagedFacts();
    else if (o.paths) {
      const paths = String(o.paths).split(",");
      facts = { pathsChanged: paths, recordsAdded: o.records !== undefined ? Number(o.records) : paths.filter(p => p.startsWith("record/")).length };
    } else die(new Error("--staged or --paths required"));
    const v = await evaluateGate(facts!);
    if (v.allow) { say(`ALLOW (${v.via})`); finish("gate", { result: { allow: true, via: v.via, cites: [] } }); }
    warn(`DENY (${v.via}) — the following instruments forbid this change:`);
    for (const c of v.cites) warn(`  ${c.citation} — ${c.title}: ${c.summary}`);
    finish("gate", { exitCode: 6, result: { allow: false, via: v.via, cites: v.cites } });
  });

program.command("doctor").action(async () => {
  const { leanAvailable } = await import("./lean/runner.js");
  const lean = await leanAvailable();
  say(`root        : ${root()}`);
  say(`lean        : ${lean ? "available" : "UNAVAILABLE — verdicts will be deferred"}`);
  say(`kernel dep  : ${R("lean", "lake-manifest.json")}`);
  finish("doctor", { result: { root: root(), lean } });
});

program.parseAsync().catch(die);
