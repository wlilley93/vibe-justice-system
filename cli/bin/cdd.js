#!/usr/bin/env node
'use strict';
// Vibe Justice System CLI. Commands: init, next-citation, submit-request, submit-breach.
// No runtime dependencies (pure Node).

const fs = require('fs');
const path = require('path');
const { nextCitation, tierCode } = require('../lib/citation');

const PKG_ROOT = path.resolve(__dirname, '..', '..'); // the vibe-justice-system repo/package root
const VERSION = require('../package.json').version;

function findCitator(dir) {
  for (const rel of ['.justice/INDEX.md', 'caselaw/INDEX.md']) {
    const p = path.join(dir, rel);
    if (fs.existsSync(p)) return p;
  }
  return null;
}

function die(msg, code = 1) { process.stderr.write(msg + '\n'); process.exit(code); }

function cmdNextCitation(args) {
  const tier = args._[0];
  if (!tier) die('usage: cdd next-citation <first-instance|court-of-appeal|supreme-court> [--year YYYY] [--citator PATH] [--json]');
  tierCode(tier); // validate early
  const citatorPath = args.citator || findCitator(process.cwd());
  const text = citatorPath && fs.existsSync(citatorPath) ? fs.readFileSync(citatorPath, 'utf8') : '';
  if (!citatorPath) process.stderr.write('note: no .justice/INDEX.md found; numbering from an empty citator (this will be N=1)\n');
  const year = args.year ? parseInt(args.year, 10) : undefined;
  const r = nextCitation(text, tier, year);
  if (args.json) process.stdout.write(JSON.stringify(r) + '\n');
  else process.stdout.write(r.citation + '\n');
}

function cmdInit(args) {
  const target = path.resolve(args._[0] || process.cwd());
  const copy = ['SPEC-LAW.md', 'VPR.md', 'CDD.md'];
  for (const f of copy) {
    const src = path.join(PKG_ROOT, f);
    if (!fs.existsSync(src)) { process.stderr.write(`skip (missing in package): ${f}\n`); continue; }
    fs.copyFileSync(src, path.join(target, f));
    process.stdout.write(`vendored ${f}\n`);
  }
  // .justice scaffold
  const jdir = path.join(target, '.justice');
  for (const d of ['', 'caselaw', 'pdfs']) fs.mkdirSync(path.join(jdir, d), { recursive: true });
  const indexPath = path.join(jdir, 'INDEX.md');
  if (!fs.existsSync(indexPath)) {
    fs.copyFileSync(path.join(PKG_ROOT, '.justice', 'INDEX.md'), indexPath);
    process.stdout.write('created .justice/INDEX.md (citator)\n');
  } else process.stdout.write('.justice/INDEX.md already present, left as-is\n');
  // Append the binding plugin block to CLAUDE.md, idempotently.
  const MARK = '<!-- vjs:plugin -->';
  const block = `\n${MARK}\n` + fs.readFileSync(path.join(PKG_ROOT, 'plugin', 'CLAUDE.md'), 'utf8') + `\n<!-- /vjs:plugin -->\n`;
  const claudePath = path.join(target, 'CLAUDE.md');
  const existing = fs.existsSync(claudePath) ? fs.readFileSync(claudePath, 'utf8') : '';
  if (existing.includes(MARK)) process.stdout.write('CLAUDE.md already has the VJS plugin block, left as-is\n');
  else { fs.writeFileSync(claudePath, existing + block); process.stdout.write('appended the VJS plugin block to CLAUDE.md\n'); }
  process.stdout.write('\nVJS installed. The court is in session.\n');
}

function workflowInvocation(script, kind, text) {
  const q = String(text || '').replace(/'/g, "\\'");
  return `Run the court in Claude Code via the Workflow tool:\n\n` +
    `Workflow({\n  scriptPath: 'court/workflows/${script}',\n  args: { kind: '${kind}', ${kind === 'breach' ? 'charge' : 'question'}: '${q}' }\n})\n`;
}

function main() {
  const argv = process.argv.slice(2);
  if (argv.includes('--version') || argv.includes('-v')) return process.stdout.write(VERSION + '\n');
  const cmd = argv[0];
  // tiny flag parser
  const args = { _: [] };
  for (let i = 1; i < argv.length; i++) {
    const a = argv[i];
    if (a === '--json') args.json = true;
    else if (a.startsWith('--')) { args[a.slice(2)] = argv[i + 1]; i++; }
    else args._.push(a);
  }
  switch (cmd) {
    case 'next-citation': return cmdNextCitation(args);
    case 'init': return cmdInit(args);
    case 'submit-request': return process.stdout.write(workflowInvocation('first-instance.js', 'request_for_ruling', args._[0]));
    case 'submit-breach': return process.stdout.write(workflowInvocation('first-instance.js', 'breach', args._[0]));
    case undefined:
    case '--help':
    case '-h':
      return process.stdout.write(
`vjs / cdd - Vibe Justice System CLI (v${VERSION})

Commands:
  init [dir]                       Install VJS into a repo (vendor SPEC-LAW/VPR/CDD, scaffold .justice/, inject plugin block into CLAUDE.md)
  next-citation <tier> [--year Y]  Deterministic next neutral citation from the citator (.justice/INDEX.md). tier = first-instance|court-of-appeal|supreme-court. --json for full object.
  submit-request "<question>"      Print the Workflow invocation to file a Request for Ruling
  submit-breach "<charge>"         Print the Workflow invocation to file a Breach
  --version                        Print version

Spec is law. Rulings are precedent. Lexby is your lawyer.\n`);
    default: die(`unknown command: ${cmd}\nrun 'cdd --help'`);
  }
}

main();
