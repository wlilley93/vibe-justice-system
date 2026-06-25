# syntax=docker/dockerfile:1
#
# VJS delivery (REG-FRONT-DOOR-DELIVERY-001, giving effect to [2026] VJS-PC 14 D5/D6).
# The kernel ships two ways, and the split is load-bearing:
#   - the SERVER image (target: server) runs the MCP server_of_law - the FRONT DOOR;
#   - the EXPORT stage hands the host a `vjs` binary for the commit hooks - the WALL.
# No enforcement guarantee rests on this image: the bypass-proof, absolute-path commit
# hook on the host is the sole guarantee and never depends on the container being up.

# cargo-chef caches the dependency build as its own layer (#21), so only a source
# change rebuilds the workspace, not every crate from crates.io.
FROM rust:1-slim-bookworm AS chef
RUN cargo install cargo-chef --locked
WORKDIR /build

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /build/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
RUN cargo build --release --bin vjs --bin vjs-mcp

# --- The server_of_law: the MCP stdio front door. ---
# An MCP client spawns this with the repo bind-mounted at /repo and talks JSON-RPC
# over stdio: `docker run --rm -i -v "$PWD:/repo" vjs-mcp:local`.
FROM debian:bookworm-slim AS server
RUN useradd -m vjs
COPY --from=builder /build/target/release/vjs /usr/local/bin/vjs
COPY --from=builder /build/target/release/vjs-mcp /usr/local/bin/vjs-mcp
WORKDIR /repo
USER vjs
ENTRYPOINT ["vjs-mcp"]

# --- The wall's binary: export the host `vjs` without running the server. ---
#   docker build --target export --output "type=local,dest=./bin" .
# yields ./bin/vjs, which the commit hook resolves from the repo root (so a
# subscriber needs no Rust toolchain to get the bypass-proof gate).
FROM scratch AS export
COPY --from=builder /build/target/release/vjs /vjs
