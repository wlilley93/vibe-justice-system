use std::io::{self, BufRead, Write};
use vjs_mcp::*;

/// VJS MCP stdio transport
/// Reads JSON-RPC requests from stdin, writes responses to stdout
fn main() {
    // #18: resolve the repo root from git, not the raw cwd, so the server works when
    // spawned from a subdirectory; fall back to cwd.
    let repo_root = std::process::Command::new("git")
        .args(["rev-parse", "--show-toplevel"])
        .output()
        .ok()
        .filter(|o| o.status.success())
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .map(|s| std::path::PathBuf::from(s.trim()))
        .filter(|p| p.is_dir())
        .or_else(|| std::env::current_dir().ok())
        .unwrap_or_else(|| std::path::PathBuf::from("."));
    let server = McpServer::new(repo_root);

    let stdin = io::stdin();
    let mut stdout = io::stdout();

    for line in stdin.lock().lines() {
        let request = match line {
            Ok(l) => l,
            Err(_) => continue,
        };

        if request.trim().is_empty() {
            continue;
        }

        let response = match server.handle_request(&request) {
            Ok(r) => r,
            Err(e) => serde_json::json!({
                "jsonrpc": "2.0",
                "error": {
                    "code": -32603,
                    "message": e.to_string()
                }
            })
            .to_string(),
        };

        writeln!(stdout, "{}", response).ok();
        stdout.flush().ok();
    }
}
