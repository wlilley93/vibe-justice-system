//! The kernel-context helpers shared across commands, re-exported from the ENGINE.
//!
//! This file HELD `build_kernel_context` and `overlay_filed_orders` until [2026] VJS-CC-VJS
//! 16 C4. They were correct; the defect was that they were only the CLI's, so the MCP door
//! answered from a graph that could not see this repository's own filed orders. Deleted here
//! rather than shared from here, because a helper module in a bin crate is not somewhere a
//! second front door can reach. See `vjs-engine/src/context.rs` for what having two cost.
//!
//! Lawpack resolution is likewise the ENGINE's; it was duplicated here until [2026]
//! VJS-CC-VJS 12.

//! `compute_digest` was re-exported here until [2026] VJS-CC-VJS 16 C4. Its only caller in
//! this crate was `build_kernel_context`, which has moved to the engine and calls it there.
pub(crate) use vjs_engine::{
    build_kernel_context, digest_of_lawpack_dir, is_invoked_jurisdiction, load_lawpack,
    resolve_invocation_lawpack, resolve_lawpack, resolve_lawpack_dir,
};
