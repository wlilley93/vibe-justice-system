use std::io::{self, BufRead, Write};
use vjs_mcp::*;

/// VJS MCP stdio transport
/// Reads JSON-RPC requests from stdin, writes responses to stdout
fn main() {
    let repo_root = std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from("."));
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
