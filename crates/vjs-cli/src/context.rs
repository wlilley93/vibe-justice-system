//! The kernel-context helpers shared across commands: build the KernelContext, load the lawpack,

use super::*;

pub(crate) fn build_kernel_context(repo: &Path) -> Result<KernelContext, KernelError> {
    let lawpack = load_lawpack(repo)?;
    let mut graph = lawpack.build_authority_graph()?;
    overlay_filed_orders(repo, &mut graph)?;
    let digest = compute_digest(repo)?;

    Ok(KernelContext {
        authority_graph: graph,
        limits: ContextLimits::default(),
        lawpack_digest: digest,
    })
}

/// Publish the Gazette data file: every law object of the V2 canon (walked
/// from the lawpack so nothing is hand-curated out of the record) plus the
/// curated V1 archive estate, with editorial summaries and citation edges
/// overlaid where the provenance file carries them. Edges to ids that do not
/// resolve to an item are dropped, so the graph can never link to a non-item.
pub(crate) fn load_lawpack(repo: &Path) -> Result<Lawpack, KernelError> {
    let lawpack_dir = repo.join("lawpack/v2");
    if lawpack_dir.exists() {
        LawpackLoader::load(&lawpack_dir)
    } else {
        // Fallback: use empty lawpack
        Ok(Lawpack {
            statutes: Vec::new(),
            regulations: Vec::new(),
            rules: Vec::new(),
            orders: Vec::new(),
            specs: Vec::new(),
            invariants: Vec::new(),
            decisions: Vec::new(),
            obligations: Vec::new(),
        })
    }
}

pub(crate) fn compute_digest(repo: &Path) -> Result<String, KernelError> {
    use sha2::Digest;
    let mut hasher = sha2::Sha256::new();
    let manifest = repo.join("lawpack/v2/manifest.toml");
    if manifest.exists() {
        let content = std::fs::read(&manifest).map_err(|e| KernelError::Io(e.to_string()))?;
        hasher.update(&content);
    }
    Ok(format!("sha256:{}", hex::encode(hasher.finalize())))
}

/// Add the repo's FILED ORDERS to the authority graph.
///
/// Until 2026-07-27 the graph was built from `lawpack/v2` alone, so `.vjs/orders/` was write-only:
/// orders were validated, committed, and then read back by nothing. `vjs lookup --issue X` returned
/// byte-identical output for a real binding issue and for one that did not exist, and `vjs route` on
/// a decided issue answered `court_required: false` while omitting the order that decided it.
///
/// That is not a missing convenience. `ACT-001:s3` already ranks "County Court orders" in the
/// authority hierarchy, above local decision logs - the resolver simply was not implementing its own
/// statute. The consequence is that every issue presents as first-impression, because the check for
/// existing law returns the same answer whether or not any exists, so the S-11(c) prohibition on
/// re-litigating settled law could only ever be honoured from memory.
///
/// Orders are inserted with their `issue` as an issue tag and their court as the rank, so
/// `resolve_authority` can match them. A missing or unreadable orders directory is NOT an error: a
/// fresh subscriber repo has no orders yet, and refusing to route in that state would be worse than
/// the defect being fixed.
fn overlay_filed_orders(repo: &Path, graph: &mut AuthorityGraph) -> Result<(), KernelError> {
    // PER FILE, not all-or-nothing, and NEVER silently.
    //
    // Store::read_orders returns Err if ANY single order fails to parse. The first version of this
    // function swallowed that with `Err(_) => return Ok(())`, so one malformed file silently emptied
    // the entire citator - and the symptom was indistinguishable from the bug being fixed: lookup
    // still answered the same thing for every issue. A fail-open in the thing built to close a
    // fail-open. Reading each file separately means one bad order costs that order, and says so.
    let orders_dir = repo.join(".vjs/orders");
    if !orders_dir.exists() {
        return Ok(());
    }
    let mut orders: Vec<Order> = Vec::new();
    let entries = match std::fs::read_dir(&orders_dir) {
        Ok(e) => e,
        Err(e) => {
            eprintln!("warning: cannot read {}: {e}", orders_dir.display());
            return Ok(());
        }
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) != Some("yaml") {
            continue;
        }
        let content = match std::fs::read_to_string(&path) {
            Ok(c) => c,
            Err(e) => {
                eprintln!("warning: unreadable order {}: {e}", path.display());
                continue;
            }
        };
        match serde_yaml::from_str::<Order>(&content) {
            Ok(o) => orders.push(o),
            Err(e) => eprintln!(
                "warning: order {} does not parse and is NOT in the citator: {e}",
                path.display()
            ),
        }
    }
    for order in orders {
        if !order.status.is_live() {
            continue;
        }
        let rank = match order.court {
            Court::SupremeCourt => AuthorityRank::SupremeCourt,
            Court::PrivyCouncil => AuthorityRank::PrivyCouncil,
            Court::CourtOfAppeal => AuthorityRank::CourtOfAppeal,
            Court::County => AuthorityRank::CountyCourt,
        };
        // The citation is what a human cites ("[2026] VJS-CC-OPBOX 5"); the id is what the store
        // keys on. Show the citation when there is one, because an answer nobody can cite is not
        // usable as precedent.
        let title = order
            .citation
            .clone()
            .unwrap_or_else(|| order.id.clone());
        let authority = Authority {
            id: AuthorityId(order.id.clone()),
            kind: AuthorityKind::Order,
            rank,
            status: order.status.clone(),
            jurisdiction: Some(order.jurisdiction.clone()),
            title,
            summary: order.runtime_summary.clone(),
            source_path: None,
            issue_tags: vec![order.issue.clone()],
            scope: None,
            supersedes: order.supersedes.clone(),
        };
        graph.authorities.insert(authority.id.clone(), authority);
    }
    Ok(())
}
