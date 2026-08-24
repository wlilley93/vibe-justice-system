// Property tests for the fact extractor -- the one trusted component that can lie to a
// correct kernel.
//
// Every theorem in the kernel is a statement about a Facts record. Nothing proves the
// Facts record describes the commit. If the extractor drops a path, the gate reasons
// flawlessly about a change that is not the one being made, and it does so silently and
// in the PERMISSIVE direction: an instrument that should have fired does not, and there
// is no denial to notice the absence of.
//
// These drive real git, not a mock, because the defects here are all in git's output
// format rather than in the parsing idea: renames carry three fields, and paths outside
// the portable character set come back C-quoted with octal escapes unless -z is used.
// A mock would have encoded the same wrong assumption the parser did.
import { describe, it, expect, beforeEach, afterEach } from "vitest";
import fs from "node:fs";
import os from "node:os";
import path from "node:path";
import { execa } from "execa";
import { setRoot } from "../src/paths.js";
import { stagedFacts } from "../src/kernel/gate.js";

let repo: string;

async function git(...args: string[]) {
  return execa("git", args, { cwd: repo, reject: false });
}

beforeEach(async () => {
  repo = fs.mkdtempSync(path.join(os.tmpdir(), "vjs-facts-"));
  await git("init", "-q", ".");
  await git("config", "user.email", "t@t.t");
  await git("config", "user.name", "t");
  fs.mkdirSync(path.join(repo, "lean", "Vjs"), { recursive: true });
  fs.mkdirSync(path.join(repo, "record"), { recursive: true });
  fs.writeFileSync(path.join(repo, "lean", "Vjs", "Book.lean"), "x\n");
  fs.writeFileSync(path.join(repo, "README.md"), "y\n");
  await git("add", "-A");
  await git("commit", "-qm", "base");
  setRoot(repo);
});

afterEach(() => { fs.rmSync(repo, { recursive: true, force: true }); });

describe("stagedFacts", () => {
  // The headline defect. `git diff --name-status` prints a rename as three tab-separated
  // fields (R100, source, destination). Taking the last field keeps only the destination,
  // so moving a file OUT of a protected directory never puts the protected path in front
  // of the gate. record/0034 learned that renaming a protected path removes protection
  // rather than moving it; this is the same lesson one level lower, and it does not even
  // need the directory to be renamed.
  it("reports BOTH paths of a rename, so moving a file out of a protected scope is visible", async () => {
    await git("mv", "lean/Vjs/Book.lean", "escaped.lean");
    const f = await stagedFacts();
    expect(f.pathsChanged).toContain("lean/Vjs/Book.lean");
    expect(f.pathsChanged).toContain("escaped.lean");
  });

  // Without -z, git C-quotes any path outside the portable character set: the extracted
  // string literally begins with a double quote and carries \303\251 rather than the
  // bytes. `String.isPrefixOf "lean/Vjs/"` is then false, and the protection is gone.
  it("reports a unicode path raw, not C-quoted, so prefix scopes still match", async () => {
    fs.writeFileSync(path.join(repo, "lean", "Vjs", "café.lean"), "z\n");
    await git("add", "-A");
    const f = await stagedFacts();
    expect(f.pathsChanged).toContain("lean/Vjs/café.lean");
    for (const p of f.pathsChanged) expect(p.startsWith('"')).toBe(false);
    expect(f.pathsChanged.some(p => p.startsWith("lean/Vjs/"))).toBe(true);
  });

  // A quote or backslash in a filename is the same class of defect as unicode, and is the
  // one an adversary picks: it is legal on every filesystem the estate runs on.
  it("does not mangle a path containing a quote or a backslash", async () => {
    const weird = 'lean/Vjs/a"b\\c.lean';
    fs.writeFileSync(path.join(repo, weird), "z\n");
    await git("add", "-A");
    const f = await stagedFacts();
    expect(f.pathsChanged).toContain(weird);
  });

  // Line-splitting a newline-bearing filename turns one path into two, neither of which is
  // the real one. -z removes the question by not using lines at all.
  it("treats a newline inside a filename as one path, not two", async () => {
    const nl = "lean/Vjs/two\nlines.lean";
    fs.writeFileSync(path.join(repo, nl), "z\n");
    await git("add", "-A");
    const f = await stagedFacts();
    expect(f.pathsChanged).toContain(nl);
    expect(f.pathsChanged.filter(p => p.includes("lines.lean"))).toHaveLength(1);
  });

  // The ordinary path must keep working, and recordsAdded must count only genuine additions
  // under record/ -- it is what satisfies every recordRequired instrument.
  it("counts added record entries and reports plain adds, edits and deletes", async () => {
    fs.writeFileSync(path.join(repo, "record", "0001.md"), "r\n");
    fs.writeFileSync(path.join(repo, "lean", "Vjs", "Book.lean"), "edited\n");
    fs.rmSync(path.join(repo, "README.md"));
    await git("add", "-A");
    const f = await stagedFacts();
    expect(f.pathsChanged.sort()).toEqual(["README.md", "lean/Vjs/Book.lean", "record/0001.md"]);
    expect(f.recordsAdded).toBe(1);
  });

  // Prefix scoping is the rule language's only matching primitive, so the extractor must
  // not normalise a path in a way that makes a sibling directory look like the scope.
  // "gateX/" must never arrive looking like "gate/".
  it("keeps sibling directories distinct under prefix scoping", async () => {
    fs.mkdirSync(path.join(repo, "gateX"), { recursive: true });
    fs.writeFileSync(path.join(repo, "gateX", "not-the-hook"), "z\n");
    await git("add", "-A");
    const f = await stagedFacts();
    expect(f.pathsChanged).toContain("gateX/not-the-hook");
    expect(f.pathsChanged.some(p => p.startsWith("gate/"))).toBe(false);
  });
});
