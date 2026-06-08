import { spawnSync } from "node:child_process";
import { existsSync } from "node:fs";
import { join } from "node:path";

function projectRoot(ctx) {
  return ctx.worktree || ctx.directory || process.cwd();
}

function scriptPath(root, script) {
  const installed = join(root, ".vjs", "hooks", script);
  if (existsSync(installed)) return installed;
  return join(root, "Executive", "plugin", "hooks", script);
}

function runVjsHook(ctx, script, event, input, extraEnv = {}) {
  const root = projectRoot(ctx);
  const payload = JSON.stringify({
    cwd: root,
    hook_event_name: event,
    opencode: input || {},
  });
  const result = spawnSync("bash", [scriptPath(root, script)], {
    cwd: root,
    input: payload,
    encoding: "utf8",
    env: {
      ...process.env,
      VJS_AGENT_RUNTIME: "opencode",
      ...extraEnv,
    },
  });
  if (result.status && result.stderr) throw new Error(result.stderr.trim());
}

export const VjsLawfulnessPlugin = async (ctx) => {
  return {
    event: async (input) => {
      const type = input?.event?.type || input?.event?.name || input?.event?.event || "event";
      if (/session\.(created|idle|completed|stopped)|app\.(started|exiting)/i.test(type)) {
        runVjsHook(ctx, "vjs-post-answer.sh", type, input);
      }
    },
    "chat.message": async (input, output) => {
      runVjsHook(ctx, "vjs-pre-answer.sh", "chat.message", { input, output }, {
        VJS_PRE_ANSWER_REMINDER: "on",
        VJS_HOOK_EVENT_NAME: "chat.message",
      });
    },
    "tool.execute.before": async (input, output) => {
      runVjsHook(ctx, "vjs-pre-answer.sh", "tool.execute.before", { input, output }, {
        VJS_HOOK_EVENT_NAME: "tool.execute.before",
      });
    },
    "tool.execute.after": async (input, output) => {
      runVjsHook(ctx, "vjs-post-answer.sh", "tool.execute.after", { input, output });
    },
    "experimental.session.compacting": async (input, output) => {
      runVjsHook(ctx, "vjs-post-answer.sh", "experimental.session.compacting", { input, output });
    },
  };
};

export const server = VjsLawfulnessPlugin;
export default VjsLawfulnessPlugin;
