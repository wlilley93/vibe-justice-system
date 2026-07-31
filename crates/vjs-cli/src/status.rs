//! The `vjs status` command: what this repository is, and - since
//! [2026] VJS-CC-VJS 12 - whether it can actually see any law.
//!
//! Split out of `lifecycle.rs` when D6 pushed that file past the 600-line
//! structural ceiling. Behaviour-preserving: the function is moved verbatim
//! apart from this header.

use super::*;

pub(crate) fn cmd_status(repo: &Path, json: bool) -> Result<(), KernelError> {
    let git_root = GitIntegration::find_repo_root(repo)?;
    let is_git = git_root.is_some();
    let is_public = if is_git {
        GitIntegration::is_public_remote(git_root.as_deref().unwrap_or(repo)).unwrap_or(false)
    } else {
        false
    };

    let vjs_dir = repo.join(".vjs");
    let vjs_installed = vjs_dir.exists();

    let lock = Store::read_lawpack_lock(repo)?;
    let lawpack_info = lock.map(|l| format!("{}@{}", l.lawpack_id, l.lawpack_version));

    let logs = if vjs_installed {
        Store::read_logs(repo)?.len()
    } else {
        0
    };

    let orders = if vjs_installed {
        Store::read_orders(repo)?.len()
    } else {
        0
    };

    let permits = if vjs_installed {
        Store::read_permits(repo)?
    } else {
        Vec::new()
    };

    let proofs = if vjs_installed {
        Store::read_proofs(repo)?.len()
    } else {
        0
    };

    let active_permits = permits
        .iter()
        .filter(|p| matches!(p.status, PermitStatus::Active))
        .count();
    let closed_permits = permits
        .iter()
        .filter(|p| matches!(p.status, PermitStatus::Closed))
        .count();

    let status = StatusInfo {
        repo: repo.display().to_string(),
        git_repo: is_git,
        public_remote: is_public,
        vjs_installed,
        lawpack: lawpack_info,
        logs_count: logs,
        orders_count: orders,
        permits_count: permits.len(),
        active_permits_count: active_permits,
        closed_permits_count: closed_permits,
        proofs_count: proofs,
    };

    if json {
        println!("{}", serde_json::to_string_pretty(&status).unwrap());
    } else {
        println!("Repo: {}", status.repo);
        println!("Git: {}", status.git_repo);
        println!("Public remote: {}", status.public_remote);
        println!("VJS installed: {}", status.vjs_installed);
        // D6 of [2026] VJS-CC-VJS 12. This line used to name a lawpack whether or not one
        // could be resolved, so `vibe-design-system` reported "VJS installed: true" and
        // "Lawpack: vjs-v2@0.1.0" while `vjs lookup` returned nothing at all. An operator
        // had no way, short of reading the kernel source, to learn that their jurisdiction
        // had no law. A silent failure is one thing; a silent failure that prints a
        // reassurance is the aggravating fact the court named.
        //
        // IT IS KEYED ON BEING A JURISDICTION, NOT ON HAVING A LAWPACK LABEL. The first
        // version sat inside `if let Some(lp) = status.lawpack`, which reads the LOCK - so a
        // jurisdiction with a config and no lock printed no lawpack line at all, and the
        // warning was unreachable in one of the two states it exists for. Caught by the
        // test, not by reading it back.
        let repo_path = Path::new(&status.repo);
        if crate::context::is_invoked_jurisdiction(repo_path) {
            let label = status.lawpack.as_deref().unwrap_or("none recorded");
            match crate::context::resolve_lawpack_dir(repo_path) {
                Some(dir) => println!("Lawpack: {} ({})", label, dir.display()),
                None => println!(
                    "Lawpack: {} - UNRESOLVED. No canon is loaded and every issue would \
                     present as first impression. Re-run `vjs invoke --lawpack <path>`.",
                    label
                ),
            }
        } else if let Some(ref lp) = status.lawpack {
            println!("Lawpack: {}", lp);
        }
        println!("Logs: {}", status.logs_count);
        println!("Orders: {}", status.orders_count);
        println!(
            "Permits: {} total, {} active, {} closed",
            status.permits_count, status.active_permits_count, status.closed_permits_count
        );
        println!("Proofs: {}", status.proofs_count);
    }

    Ok(())
}
