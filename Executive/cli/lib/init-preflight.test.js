'use strict';

const assert = require('assert');
const fs = require('fs');
const os = require('os');
const path = require('path');
const { spawnSync } = require('child_process');

const BIN = path.resolve(__dirname, '..', 'bin', 'cdd.js');

function tmpRepo() {
  const root = fs.mkdtempSync(path.join(os.tmpdir(), 'vjs-init-'));
  const git = spawnSync('git', ['init', '-q'], { cwd: root, encoding: 'utf8' });
  assert.strictEqual(git.status, 0, git.stderr);
  return root;
}

function run(args, cwd) {
  return spawnSync(process.execPath, [BIN, ...args], { cwd, encoding: 'utf8' });
}

{
  const root = tmpRepo();
  const res = run(['init'], root);
  assert.notStrictEqual(res.status, 0);
  assert.match(res.stderr, /has not declared itself as a VJS system repository/);
}

{
  const root = tmpRepo();
  const res = run(['init', '--declare-system-repo'], root);
  assert.strictEqual(res.status, 0, res.stderr);
  const declaration = JSON.parse(fs.readFileSync(path.join(root, '.vjs', 'system.json'), 'utf8'));
  assert.strictEqual(declaration.system, 'vjs');
  assert.strictEqual(declaration.included, true);
  assert.strictEqual(declaration.basis, 'local-sovereign-act');
  assert.ok(fs.existsSync(path.join(root, '.justice', 'INDEX.md')));
  assert.ok(fs.existsSync(path.join(root, '.codex', 'hooks.json')));
  assert.ok(fs.existsSync(path.join(root, '.gemini', 'settings.json')));
  assert.ok(fs.existsSync(path.join(root, '.opencode', 'plugins', 'vjs-lawfulness.js')));
}

{
  const root = tmpRepo();
  const subdir = path.join(root, 'subdir');
  fs.mkdirSync(subdir);
  const res = run(['init', '--declare-system-repo'], subdir);
  assert.notStrictEqual(res.status, 0);
  assert.match(res.stderr, /target must be the git worktree root/);
}

console.log('init-preflight tests OK');
