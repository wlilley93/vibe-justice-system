'use strict';

const assert = require('assert');
const fs = require('fs');
const os = require('os');
const path = require('path');
const { spawnSync } = require('child_process');
const {
  scanJudgmentProvenance,
  hasCourtWorkflowProvenance,
  hasRegistrarProvenance,
} = require('./judgment-provenance-scan');

let pass = 0;
function t(name, fn) { fn(); pass++; process.stdout.write(`ok - ${name}\n`); }

function tmpRepo() {
  const root = fs.mkdtempSync(path.join(os.tmpdir(), 'vjs-judgment-provenance-'));
  spawnSync('git', ['init', '-q'], { cwd: root, stdio: 'ignore' });
  spawnSync('git', ['config', 'user.email', 'test@example.invalid'], { cwd: root, stdio: 'ignore' });
  spawnSync('git', ['config', 'user.name', 'Test User'], { cwd: root, stdio: 'ignore' });
  return root;
}

function writeFile(root, rel, text) {
  const full = path.join(root, rel);
  fs.mkdirSync(path.dirname(full), { recursive: true });
  fs.writeFileSync(full, text);
  return full;
}

t('court workflow provenance requires source, workflow, and run id', () => {
  assert.strictEqual(hasCourtWorkflowProvenance([
    '---',
    'adjudication_provenance: court-workflow',
    'workflow: Judicature/court/workflows/first-instance.js',
    'workflow_run_id: local-run-123',
    '---',
  ].join('\n')), true);
  assert.strictEqual(hasCourtWorkflowProvenance('adjudication_provenance: court-workflow\nworkflow: first-instance.js\n'), false);
});

t('authorised registrar provenance requires authority and registrar note', () => {
  assert.strictEqual(hasRegistrarProvenance([
    '---',
    'adjudication_provenance: authorised-registrar',
    'registrar_authority: "[2026] REALM-SC 8"',
    'registrar_note: "Reduced to record by authorised registrar."',
    '---',
  ].join('\n')), true);
  assert.strictEqual(hasRegistrarProvenance('adjudication_provenance: authorised-registrar\nregistrar_note: ok\n'), false);
});

t('new central judgment without provenance fails', () => {
  const root = tmpRepo();
  writeFile(root, 'Judicature/.justice/INDEX.md', '# index\n');
  writeFile(root, 'Judicature/.justice/judgments/privy-council/2026-realm-pc-21.md', [
    '---',
    'citation_id: "[2026] REALM-PC 21"',
    'registrar_note: "Authored by the bench."',
    '---',
  ].join('\n'));
  const res = scanJudgmentProvenance(root);
  assert.strictEqual(res.ok, false);
  assert.strictEqual(res.findings[0].type, 'missing-judgment-provenance');
});

t('new central judgment with workflow provenance passes', () => {
  const root = tmpRepo();
  writeFile(root, 'Judicature/.justice/INDEX.md', '# index\n');
  writeFile(root, 'Judicature/.justice/judgments/privy-council/2026-realm-pc-21.md', [
    '---',
    'citation_id: "[2026] REALM-PC 21"',
    'adjudication_provenance: court-workflow',
    'workflow: Judicature/court/workflows/first-instance.js',
    'workflow_run_id: local-run-123',
    '---',
  ].join('\n'));
  const res = scanJudgmentProvenance(root);
  assert.strictEqual(res.ok, true);
  assert.strictEqual(res.scanned.length, 1);
});

t('existing committed central judgments are not re-litigated by this guard', () => {
  const root = tmpRepo();
  writeFile(root, 'Judicature/.justice/INDEX.md', '# index\n');
  writeFile(root, 'Judicature/.justice/judgments/privy-council/2026-realm-pc-20.md', 'legacy record\n');
  spawnSync('git', ['add', '.'], { cwd: root, stdio: 'ignore' });
  spawnSync('git', ['commit', '-m', 'seed', '-q'], { cwd: root, stdio: 'ignore' });
  const res = scanJudgmentProvenance(root);
  assert.strictEqual(res.ok, true);
  assert.deepStrictEqual(res.scanned, []);
});

process.stdout.write(`\n${pass} passing\n`);
