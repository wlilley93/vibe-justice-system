/**
 * `@vjs/client` — how a client asks the court something.
 *
 * DELIBERATELY THIN. This spawns the `vjs` CLI and parses its envelope. It contains no
 * lookup logic, no ranking, no book handling, and no knowledge of how a ruling is stored.
 * Everything it knows is the shape of the answer.
 *
 * Why a subprocess rather than a library:
 *
 *  - **One writer.** `rule` takes the jurisdiction lock, regenerates the book and runs
 *    `lake build` under a keep-or-rollback. Three clients importing that as a library means
 *    three processes racing one lockfile and, worse, three *versions* of the enactment code
 *    the moment anyone pins an older release. The court must be the only thing that writes
 *    the docket.
 *  - **Clients are not all TypeScript.** boltrig is Python and Swift. A TS library is
 *    unusable there; a CLI is not.
 *  - **The duplication already cost something.** Two copies of the court diverged in eight of
 *    twelve modules, and the one that drifted in a safety-relevant direction was the gate —
 *    one copy reported a Lean compile failure as a constitutional denial for hours after the
 *    other was fixed. That is the argument against sharing by copy, realised.
 *
 * Why not a daemon: a long-lived process with write access to the record, reachable by
 * anything on localhost, adds availability as a failure mode to a system whose entire value
 * is determinism — and it is one config flag from a process that decides on its own.
 */
import { execFile } from "node:child_process";
import { promisify } from "node:util";

const exec = promisify(execFile);

/** The envelope version this client understands. A mismatch is refused rather than parsed
 *  optimistically — a client guessing at an unfamiliar shape is how silent drift starts. */
export const ENVELOPE_VERSION = 1;

export interface Ruling {
  citation: string;
  court: string;
  questionKey: string;
  caseId: string;
  date: string;
  status: string;
  file: string;
  question?: string;
  facts?: string;
  ruling?: string;
  reasoning?: string;
  lawApplied?: string[];
}

export interface AskResult {
  key: string;
  /** The standing ruling, or null if the question is open. */
  standing: Ruling | null;
}

export interface RuleResult {
  key: string;
  /** True when a standing ruling already answered it and no bench sat. */
  reused: boolean;
  ruling: Ruling;
}

export interface ClientOptions {
  /** The jurisdiction to address. Becomes VJS_ROOT for the subprocess. */
  root: string;
  /** Path to the vjs CLI entry. Defaults to the `vjs` on PATH. */
  bin?: string;
  /** Seconds to wait for the jurisdiction lock before failing. */
  waitSeconds?: number;
  timeoutMs?: number;
}

export class CourtError extends Error {
  constructor(message: string, readonly code: number, readonly command: string) {
    super(message);
  }
}

function parseEnvelope(stdout: string, command: string): any {
  let env: any;
  try { env = JSON.parse(stdout); }
  catch { throw new CourtError(`the court returned unparseable output for \`${command}\``, 1, command); }
  if (env.vjs !== ENVELOPE_VERSION) {
    throw new CourtError(
      `envelope version ${env.vjs} is not ${ENVELOPE_VERSION}; this client and that court disagree about the shape of an answer`,
      1, command
    );
  }
  if (!env.ok && env.error) throw new CourtError(env.error.message, env.error.code ?? 1, command);
  return env;
}

export class Court {
  constructor(private readonly opts: ClientOptions) {}

  private async run(args: string[]): Promise<any> {
    const bin = this.opts.bin ?? "vjs";
    const argv = [...args, "--json", "--root", this.opts.root];
    if (this.opts.waitSeconds) argv.push("--wait", String(this.opts.waitSeconds));
    try {
      const { stdout } = await exec(bin, argv, {
        env: { ...process.env, VJS_ROOT: this.opts.root },
        timeout: this.opts.timeoutMs ?? 600_000,
        maxBuffer: 32 * 1024 * 1024
      });
      return parseEnvelope(stdout, args[0]);
    } catch (e: any) {
      // A non-zero exit still carries an envelope on stdout — the court emits one on
      // failure precisely so a caller never has to distinguish "refused" from "killed".
      if (typeof e?.stdout === "string" && e.stdout.trim().startsWith("{")) {
        return parseEnvelope(e.stdout, args[0]);
      }
      throw new CourtError(String(e?.message ?? e), typeof e?.code === "number" ? e.code : 1, args[0]);
    }
  }

  /**
   * Has this question been decided?
   *
   * EXACT match on the key. This is res judicata, and its exactness is the whole property:
   * a ranking function anywhere near this call would let the same question be answered two
   * ways while `res_judicata` continued to compile. Never substitute `search` for a miss.
   *
   * Cheap — two file reads, no model, no Lean, no lock — so it is safe to call inline.
   */
  async ask(key: string): Promise<AskResult> {
    const env = await this.run(["ask", key]);
    return env.result as AskResult;
  }

  /**
   * Put a question to the bench and file the ruling.
   *
   * Expensive and non-deterministic: it may sit a live bench (~25s) and it takes the
   * jurisdiction lock. Never call this on a hot path or inside a gate — a gate that can
   * answer differently on retry is not a gate. It returns `reused: true` without sitting
   * when a standing ruling already answers the key.
   */
  async rule(input: {
    key: string; question: string; facts: string; matter?: string;
    court?: "first-instance" | "appeals-court" | "supreme-court";
  }): Promise<RuleResult> {
    const args = ["rule", input.key, "--question", input.question, "--facts", input.facts];
    if (input.matter) args.push("--matter", input.matter);
    if (input.court) args.push("--court", input.court);
    return (await this.run(args)).result as RuleResult;
  }
}
