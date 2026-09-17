// The http provider's contract, exercised against a real local server.
//
// The point of a vendor-blind provider is that a host can implement the other side in an
// afternoon, so the contract is asserted byte-for-byte here rather than described: what
// is posted, which header carries the token, what comes back, and - the half that
// matters - that every failure is a refusal with a reason and never an empty completion.
import { describe, it, expect, beforeAll, afterAll, beforeEach } from "vitest";
import http from "node:http";
import type { AddressInfo } from "node:net";
import fs from "node:fs";
import os from "node:os";
import path from "node:path";

// A scratch root BEFORE the provider module loads: withLogging writes the bench log and
// records each completion under the jurisdiction root, and paths.ts memoises that root
// at first import. Without this the test files its opinions into the real repository.
process.env.VJS_ROOT = fs.mkdtempSync(path.join(os.tmpdir(), "vjs-http-"));

let server: http.Server;
let base = "";
let received: { headers: http.IncomingHttpHeaders; body: unknown }[] = [];
let answer: (req: http.IncomingMessage, res: http.ServerResponse) => void;

beforeAll(async () => {
  server = http.createServer((req, res) => {
    let raw = "";
    req.on("data", (c) => (raw += c));
    req.on("end", () => {
      received.push({ headers: req.headers, body: raw ? JSON.parse(raw) : null });
      answer(req, res);
    });
  });
  await new Promise<void>((r) => server.listen(0, "127.0.0.1", r));
  base = `http://127.0.0.1:${(server.address() as AddressInfo).port}/bench`;
});
afterAll(() => server.close());
beforeEach(() => {
  received = [];
  process.env.VJS_HTTP_PROVIDER_URL = base;
  process.env.VJS_HTTP_PROVIDER_TOKEN = "t0ken";
  answer = (_req, res) => { res.setHeader("content-type", "application/json"); res.end(JSON.stringify({ text: "RULING." })); };
});

const req = { promptName: "judge-first-instance", caseId: "policy:probe", system: "S", user: "U", maxTokens: 512 };

describe("the http provider", () => {
  it("posts the prompt with the bearer and takes the text back", async () => {
    const { httpProvider } = await import("../src/llm/httpProvider.js");
    const out = await httpProvider().complete(req);
    expect(out).toBe("RULING.");
    expect(received).toHaveLength(1);
    expect(received[0].headers.authorization).toBe("Bearer t0ken");
    expect(received[0].body).toEqual({ promptName: "judge-first-instance", matter: "policy:probe", system: "S", user: "U", maxTokens: 512 });
  });

  it("sends no authorization header when no token is set", async () => {
    delete process.env.VJS_HTTP_PROVIDER_TOKEN;
    const { httpProvider } = await import("../src/llm/httpProvider.js");
    await httpProvider().complete(req);
    expect(received[0].headers.authorization).toBeUndefined();
  });

  it("refuses a non-2xx with the status and the first line of the body", async () => {
    answer = (_req, res) => { res.statusCode = 503; res.end("the court cell is not answering\nsecond line"); };
    const { httpProvider } = await import("../src/llm/httpProvider.js");
    await expect(httpProvider().complete(req)).rejects.toThrow(/refused \(503\): the court cell is not answering$/);
  });

  it("refuses a body that is not JSON", async () => {
    answer = (_req, res) => { res.end("<html>oops</html>"); };
    const { httpProvider } = await import("../src/llm/httpProvider.js");
    await expect(httpProvider().complete(req)).rejects.toThrow(/not JSON: <html>oops<\/html>/);
  });

  it("refuses an answer with no text rather than returning an empty completion", async () => {
    answer = (_req, res) => { res.end(JSON.stringify({ text: "   " })); };
    const { httpProvider } = await import("../src/llm/httpProvider.js");
    await expect(httpProvider().complete(req)).rejects.toThrow(/no text/);
  });

  it("refuses to start without a URL", async () => {
    delete process.env.VJS_HTTP_PROVIDER_URL;
    const { httpProvider } = await import("../src/llm/httpProvider.js");
    expect(() => httpProvider()).toThrow(/VJS_HTTP_PROVIDER_URL/);
  });

  it("is selected by provider: http in the config", async () => {
    const { getProvider } = await import("../src/llm/provider.js");
    const provider = await getProvider({ provider: "http" } as any);
    // withLogging wraps it; the wrapper still completes through the endpoint.
    await expect(provider.complete(req)).resolves.toBe("RULING.");
  });
});
