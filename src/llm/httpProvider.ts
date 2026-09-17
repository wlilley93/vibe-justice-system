// §8.4: a completion endpoint the jurisdiction's host runs, reached over HTTP.
//
// WHY THIS EXISTS. The two live providers are one vendor's CLI and one vendor's API. An
// organisation that runs its bench on a different model - Balmoral's court sits on the
// Codex identity of its own justice cell (Will, 17 September 2026: "Use codex instead") -
// had no way in that did not mean editing the court, and the court is pinned and shared.
// This provider is deliberately vendor-blind: it posts the prompt to a URL the host
// names and takes back text. Who answers, with what credential, on which model, is the
// host's business and stays out of the pinned code.
//
// THE CONTRACT, so a host can implement it in an afternoon:
//   POST $VJS_HTTP_PROVIDER_URL
//     Authorization: Bearer $VJS_HTTP_PROVIDER_TOKEN     (sent only when set)
//     Content-Type: application/json
//     {"promptName","matter","system","user","maxTokens"}
//   200 {"text": "<the completion>"}
// Anything else - a non-2xx, a body that is not JSON, a JSON with no string `text` -
// is a refusal with the status and the first line of the body in the message, never an
// empty completion. An empty completion would reach the bench as a ruling of nothing and
// be repaired into something, which is the one outcome worse than an error.
import type { LLMProvider } from "./provider.js";

export function httpProvider(): LLMProvider {
  const url = process.env.VJS_HTTP_PROVIDER_URL;
  if (!url) throw Object.assign(new Error("provider \"http\" needs VJS_HTTP_PROVIDER_URL"), { code: 5 });
  const token = process.env.VJS_HTTP_PROVIDER_TOKEN;
  const timeoutMs = Number(process.env.VJS_HTTP_PROVIDER_TIMEOUT_MS || 600000);
  return {
    async complete(req) {
      const headers: Record<string, string> = { "content-type": "application/json", accept: "application/json" };
      if (token) headers.authorization = "Bearer " + token;
      const body = JSON.stringify({
        promptName: req.promptName, matter: req.caseId, system: req.system, user: req.user,
        maxTokens: req.maxTokens ?? 4096,
      });
      const r = await fetch(url, { method: "POST", headers, body, signal: AbortSignal.timeout(timeoutMs) });
      const raw = await r.text();
      if (!r.ok) {
        throw Object.assign(new Error(`http provider refused (${r.status}): ${firstLine(raw)}`), { code: 5 });
      }
      let parsed: unknown;
      try { parsed = JSON.parse(raw); } catch {
        throw Object.assign(new Error(`http provider answered something that is not JSON: ${firstLine(raw)}`), { code: 5 });
      }
      const text = (parsed as { text?: unknown })?.text;
      if (typeof text !== "string" || !text.trim()) {
        throw Object.assign(new Error("http provider answered with no text"), { code: 5 });
      }
      return text;
    }
  };
}

function firstLine(s: string): string {
  return (s || "").split("\n")[0].slice(0, 200);
}
