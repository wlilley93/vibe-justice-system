'use strict';

const assert = require('assert');
const fs = require('fs');
const os = require('os');
const path = require('path');
const {
  isPublicVjsRemote,
  parseWarrantText,
  releaseWarrantReport,
} = require('./release-warrant');

function tmpRoot() {
  return fs.mkdtempSync(path.join(os.tmpdir(), 'vjs-release-warrant-'));
}

assert.ok(isPublicVjsRemote('https://github.com/wlilley93/vibe-justice-system.git'));
assert.ok(!isPublicVjsRemote('https://github.com/wlilley93/agent-universe.git'));

const fields = parseWarrantText(`
AUTHORISED_OUTWARD_ACT=public-vjs-publish
AUTHORISED_BY=Sovereign Founder
AUTHORISED_AT=2026-06-08T00:00:00Z
AUTHORISED_REMOTE_URL=https://github.com/wlilley93/vibe-justice-system.git
IGNORED_SECRET=do-not-return
`);
assert.strictEqual(fields.AUTHORISED_OUTWARD_ACT, 'public-vjs-publish');
assert.strictEqual(fields.IGNORED_SECRET, undefined);

{
  const root = tmpRoot();
  const dir = path.join(root, '_private', 'release-warrants');
  fs.mkdirSync(dir, { recursive: true });
  fs.writeFileSync(path.join(dir, 'public.env'), [
    'AUTHORISED_OUTWARD_ACT=public-vjs-publish',
    'AUTHORISED_BY=Sovereign Founder',
    'AUTHORISED_AT=2026-06-08T00:00:00Z',
    'AUTHORISED_REMOTE_URL=https://github.com/wlilley93/vibe-justice-system.git',
    'AUTHORISED_REMOTE_REF=refs/heads/main',
    'AUTHORISED_LOCAL_SHA=abc123',
  ].join('\n'));
  const report = releaseWarrantReport(root, {
    remoteUrl: 'https://github.com/wlilley93/vibe-justice-system.git',
    remoteRef: 'refs/heads/main',
    localSha: 'abc123',
  });
  assert.strictEqual(report.ok, true);
  assert.strictEqual(report.requiresWarrant, true);
  assert.deepStrictEqual(report.matchingRecords, ['_private/release-warrants/public.env']);
}

{
  const report = releaseWarrantReport(tmpRoot(), {
    remoteUrl: 'https://github.com/wlilley93/agent-universe.git',
  });
  assert.strictEqual(report.ok, true);
  assert.strictEqual(report.requiresWarrant, false);
}

console.log('release-warrant tests OK');
