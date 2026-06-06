#!/usr/bin/env node
'use strict';
// Vibe Justice System CLI. Commands: init, next-citation, submit-request, submit-breach.
// No runtime dependencies (pure Node).

const fs = require('fs');
const path = require('path');
const { nextCitation, tierCode } = require('../lib/citation');
const { auditCitator } = require('../lib/citator-audit');

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

function cmdCheckCitator() {
  const res = auditCitator(process.cwd());
  if (res.ok) { process.stdout.write('citator OK: no collisions, every ruling filed and indexed.\n'); return; }
  for (const p of res.problems) process.stderr.write('FAIL [' + p.type + ']: ' + p.message + '\n');
  process.stderr.write(`\n${res.problems.length} problem(s). The record is inconsistent; fix the citator/ruling files.\n`);
  process.exit(1);
}

// Install the VJS hooks into a target repo: the watchdog Stop hook, the settings wiring, and the
// deterministic pre-commit hard gate. Idempotent.
function installHooks(target) {
  const srcHooks = path.join(PKG_ROOT, 'plugin', 'hooks');
  if (!fs.existsSync(srcHooks)) { process.stderr.write('skip hooks: plugin/hooks not found in package\n'); return; }
  const dstHooks = path.join(target, '.claude', 'hooks');
  fs.mkdirSync(dstHooks, { recursive: true });
  for (const f of fs.readdirSync(srcHooks)) {
    if (!f.endsWith('.sh')) continue;
    const dst = path.join(dstHooks, f);
    fs.copyFileSync(path.join(srcHooks, f), dst);
    try { fs.chmodSync(dst, 0o755); } catch (_) {}
    process.stdout.write('installed hook .claude/hooks/' + f + '\n');
  }
  // Merge the Stop-hook wiring into .claude/settings.json, idempotently.
  const settingsSrc = path.join(PKG_ROOT, 'plugin', 'settings.json');
  if (fs.existsSync(settingsSrc)) {
    let incoming = {};
    try { incoming = JSON.parse(fs.readFileSync(settingsSrc, 'utf8')); } catch (_) { incoming = {}; }
    const dstSettings = path.join(target, '.claude', 'settings.json');
    let cur = {};
    if (fs.existsSync(dstSettings)) { try { cur = JSON.parse(fs.readFileSync(dstSettings, 'utf8')); } catch (_) { cur = {}; } }
    cur.hooks = cur.hooks || {};
    let added = false;
    for (const event of Object.keys(incoming.hooks || {})) {
      const arr = Array.isArray(cur.hooks[event]) ? cur.hooks[event] : (cur.hooks[event] = []);
      if (!JSON.stringify(arr).includes('vjs-watchdog')) { arr.push(...incoming.hooks[event]); added = true; }
    }
    fs.writeFileSync(dstSettings, JSON.stringify(cur, null, 2) + '\n');
    process.stdout.write(added ? 'merged VJS hooks into .claude/settings.json\n' : '.claude/settings.json already has VJS hooks, left as-is\n');
  }
  // Lay down the deterministic pre-commit hard gate.
  const gitDir = path.join(target, '.git');
  if (fs.existsSync(gitDir)) {
    const ghooks = path.join(gitDir, 'hooks');
    fs.mkdirSync(ghooks, { recursive: true });
    const dst = path.join(ghooks, 'pre-commit');
    if (fs.existsSync(dst)) {
      process.stdout.write('note: .git/hooks/pre-commit exists; chain it to .claude/hooks/vjs-pre-commit.sh manually\n');
    } else {
      const rel = path.relative(ghooks, path.join(dstHooks, 'vjs-pre-commit.sh'));
      try { fs.symlinkSync(rel, dst); } catch (_) { fs.copyFileSync(path.join(dstHooks, 'vjs-pre-commit.sh'), dst); }
      try { fs.chmodSync(dst, 0o755); } catch (_) {}
      process.stdout.write('installed git pre-commit hard gate\n');
    }
  }
}

function cmdInit(args) {
  const target = path.resolve(args._[0] || process.cwd());
  const copy = ['CASE-LAW.md', 'VPR.md', 'CDD.md'];
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
    // Seed an EMPTY citator template - a fresh jurisdiction starts with no rulings of its own.
    fs.copyFileSync(path.join(__dirname, '..', 'templates', 'INDEX.md'), indexPath);
    process.stdout.write('created .justice/INDEX.md (empty citator)\n');
  } else process.stdout.write('.justice/INDEX.md already present, left as-is\n');
  // Append the binding plugin block to CLAUDE.md, idempotently.
  const MARK = '<!-- vjs:plugin -->';
  const block = `\n${MARK}\n` + fs.readFileSync(path.join(PKG_ROOT, 'plugin', 'CLAUDE.md'), 'utf8') + `\n<!-- /vjs:plugin -->\n`;
  const claudePath = path.join(target, 'CLAUDE.md');
  const existing = fs.existsSync(claudePath) ? fs.readFileSync(claudePath, 'utf8') : '';
  if (existing.includes(MARK)) process.stdout.write('CLAUDE.md already has the VJS plugin block, left as-is\n');
  else { fs.writeFileSync(claudePath, existing + block); process.stdout.write('appended the VJS plugin block to CLAUDE.md\n'); }
  // Install the watchdog Stop hook + the deterministic pre-commit hard gate.
  installHooks(target);
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
    case 'check-citator': return cmdCheckCitator();
    case 'init': return cmdInit(args);
    case 'submit-request': return process.stdout.write(workflowInvocation('first-instance.js', 'request_for_ruling', args._[0]));
    case 'submit-breach': return process.stdout.write(workflowInvocation('first-instance.js', 'breach', args._[0]));
    case undefined:
    case '--help':
    case '-h':
      return process.stdout.write(
`vjs / cdd - Vibe Justice System CLI (v${VERSION})

Commands:
  init [dir]                       Install VJS into a repo (vendor CASE-LAW/VPR/CDD, scaffold .justice/, inject plugin block into CLAUDE.md)
  next-citation <tier> [--year Y]  Deterministic next neutral citation from the citator (.justice/INDEX.md). tier = first-instance|court-of-appeal|supreme-court. --json for full object.
  check-citator                    Deterministic citator audit (the hard gate): fails closed on citation collisions and on ruling-file/citator-row mismatches. Exit 1 on any problem.
  submit-request "<question>"      Print the Workflow invocation to file a Request for Ruling
  submit-breach "<charge>"         Print the Workflow invocation to file a Breach
  --version                        Print version

Spec is law. Rulings are precedent. Lexby is your lawyer.\n`);
    default: die(`unknown command: ${cmd}\nrun 'cdd --help'`);
  }
}

main();
