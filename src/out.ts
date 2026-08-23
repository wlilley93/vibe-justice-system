// Machine-readable output (§15.2's `--json`, specified but never implemented).
//
// Four rules make this usable by a caller that is not a human:
//
//  1. In json mode stdout carries the envelope and NOTHING else. Every line a human would
//     read goes to stderr. This is why `say()` has to replace `console.log` in command
//     bodies — one stray log corrupts the caller's JSON.parse.
//  2. `--json` never changes exit codes. A gate stop is still exit 2; the envelope just
//     also says why. `envelope.exitCode === process exit status` is an invariant.
//  3. Failures emit too. A caller must never see empty stdout on error, because empty
//     stdout is indistinguishable from "the process was killed".
//  4. Emission happens in one place, so no command can forget.

export type OutMode = "text" | "json";
let mode: OutMode = "text";

export const setOutMode = (m: OutMode) => { mode = m; };
export const isJson = () => mode === "json";

/** Human-facing line. Suppressed entirely in json mode. */
export const say = (...a: unknown[]) => { if (mode === "text") console.log(...a); };
/** Diagnostics. Always stderr, both modes — never pollutes the envelope. */
export const warn = (...a: unknown[]) => console.error(...a);

export interface Next { reason: string; command: string[]; human: string }

export interface GateInfo {
  awaiting: boolean;
  kind: "human-signoff" | null;
  since: string | null;
  irSha: string | null;
  signoff: { by: string; at: string; irSha: string; rulingCitation?: string; stale: boolean } | null;
  actions: string[];
}

export interface Envelope {
  vjs: 1;
  ok: boolean;
  command: string;
  /** What the command acted on — a question key, a citation, a matter. */
  subject: string | null;
  exitCode: number;
  status: string | null;
  stage: string | null;
  result: unknown;
  gate: GateInfo | null;
  next: Next | null;
  error: { code: number; message: string } | null;
  at: string;
}

export interface Result {
  /** What the command acted on — a question key, a citation, a matter. */
  subject: string | null;
  exitCode: number;
  status: string | null;
  stage: string | null;
  result: unknown;
  gate: GateInfo | null;
  next: Next | null;
}


function emit(env: Envelope): never {
  if (mode === "json") {
    // `process.exit()` does NOT flush stdout when stdout is a pipe or a file — writes are
    // asynchronous there and synchronous only to a TTY. Calling exit straight after write
    // therefore truncates the envelope in exactly the case that matters: a headless caller
    // redirecting to a file or reading a pipe. (Observed 2026-08-23: a failing live run
    // wrote a 0-byte envelope to disk while the same command printed fine to a terminal.)
    // Set exitCode and let the event loop drain instead.
    process.exitCode = env.exitCode;
    process.stdout.write(JSON.stringify(env, null, 2) + "\n", () => { process.exit(env.exitCode); });
    // If the callback never fires (stream already destroyed), still terminate.
    setTimeout(() => process.exit(env.exitCode), 2000).unref();
    // Unreachable as a value, but the signature promises `never`.
    return undefined as never;
  }
  process.exit(env.exitCode);
}

/** Terminal success/known-stop path. Exits with r.exitCode (default 0). */
export function finish(command: string, r: Partial<Result> = {}): never {
  const subject = r.subject ?? null;
  emit({
    vjs: 1,
    ok: (r.exitCode ?? 0) === 0,
    command,
    subject,
    exitCode: r.exitCode ?? 0,
    status: r.status ?? null,
    stage: r.stage ?? null,
    result: r.result ?? null,
    gate: r.gate ?? null,
    next: r.next ?? null,
    error: null,
    at: new Date().toISOString()
  });
}

/** Terminal failure path. Prints the message for humans and still emits an envelope. */
export function fail(command: string, e: any, subject: string | null = null): never {
  const code = typeof e?.code === "number" ? e.code : 1;
  const message = String(e?.message ?? e);
  if (mode === "text") console.error(message);
  emit({
    vjs: 1,
    ok: false,
    command,
    subject,
    exitCode: code,
    status: null,
    stage: null,
    result: null,
    gate: null,
    next: null,
    error: { code, message },
    at: new Date().toISOString()
  });
}
