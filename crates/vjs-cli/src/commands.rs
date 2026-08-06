//! The nested subcommand groups.
//!
//! Split out of `main.rs` under the 600-line structural ceiling when the publication
//! gate ([2026] VJS-CC-VJS 21 D3) was added. These six enums are one cohesive thing -
//! the second level of the command tree - and they carry no dispatch logic, so moving
//! them changes nothing but where they are read.

use clap::Subcommand;
use std::path::PathBuf;

#[derive(Subcommand)]
pub(crate) enum OrderCommands {
    Validate { path: PathBuf },
    Apply { path: PathBuf },
}

#[derive(Subcommand)]
pub(crate) enum PermitCommands {
    List,
    Close {
        #[arg(long)]
        id: String,
        #[arg(long)]
        proof: Option<String>,
    },
}

#[derive(Subcommand)]
pub(crate) enum CourtCommands {
    /// List the docket: filed submissions and issued orders, grouped by issue.
    Docket,
    /// Record a convening: pin the sha256 of a filed submission (the symmetric
    /// case file) and the bench that decided it.
    Record {
        #[arg(long)]
        court: String,
        /// The filed submission id whose bytes are the case file.
        #[arg(long)]
        submission: String,
        /// A deciding seat (repeat for each bench member).
        #[arg(long = "seat")]
        bench: Vec<String>,
        #[arg(long)]
        issue: Option<String>,
    },
}

#[derive(Subcommand)]
pub(crate) enum BundleCommands {
    /// Verify a bundle.lock: schema completeness, sha256 well-formedness, and the
    /// AGPL/MIT licence firewall. Fails closed on the first violation.
    Verify {
        /// Path to the bundle.lock manifest.
        path: PathBuf,
    },
}

#[derive(Subcommand)]
pub(crate) enum LogCommands {
    Decision {
        #[arg(long)]
        kind: String,
        #[arg(long)]
        issue: String,
        #[arg(long)]
        decision: String,
        #[arg(long)]
        basis: Vec<String>,
        #[arg(long)]
        risk: String,
        #[arg(long)]
        why: String,
    },
    FromPermit {
        #[arg(long)]
        permit_id: String,
        #[arg(long)]
        decision: String,
        #[arg(long)]
        why: String,
    },
}

#[derive(Subcommand)]
pub(crate) enum ProofCommands {
    Add {
        #[arg(long)]
        permit_id: String,
        #[arg(long)]
        kind: Option<String>,
        #[arg(long)]
        status: Option<String>,
    },
}
