//! The kernel-context helpers shared across commands: build the KernelContext, load the lawpack,

use super::*;

pub(crate) fn build_kernel_context(repo: &Path) -> Result<KernelContext, KernelError> {
    let lawpack = load_lawpack(repo)?;
    let graph = lawpack.build_authority_graph()?;
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
