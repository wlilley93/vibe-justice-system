//! The three dispositions of a REMOVED governed record ([2026] VJS-CC-VJS 20 D1,
//! amended by D18).
//!
//! Split out of `staged.rs` under the 600-line structural ceiling in the same commit
//! that wrote it. It carries its entrenchment with it: `staged.rs` is on
//! ENFORCEMENT_SURFACE and so is this, per the rule that file states in terms -
//! "entrenchment follows the code across a split; it never waits for one, and a split
//! is never a de-entrenchment event."

use std::path::{Path, PathBuf};

use vjs_core::front_door;
use vjs_core::report::Finding;
use vjs_core::types::Severity;
use vjs_git::GitIntegration;

use super::f;

/// THE THREE DISPOSITIONS OF A REMOVED GOVERNED RECORD
/// ([2026] VJS-CC-VJS 20 D1, amended by D18).
///
/// The gate this replaces asked one question of a PATH: is a governed record file
/// being deleted? Everything that answered yes got the same destructive-delete
/// warning. CC-VJS 20 held that question is the wrong one, because three quite
/// different acts answer yes to it and only one of them is destructive:
///
///   * UNPUBLISHED - the file leaves the index and stays on disk inside a REGISTERED
///     store. "Untracking is NOT deletion. Publication is constitutively inert, so
///     ceasing to publish cannot remove a force publication never conferred." Lawful,
///     and reported as an Info so the act is visible without being an alarm.
///   * RENAMED - the record's ID survives at another path. D18: "a rename reports
///     RECORD_RENAMED and never a deletion code." A record that moved has not gone
///     anywhere; treating a rename as a deletion was how the projection work of the
///     last two days would have read.
///   * DELETED - the id survives nowhere the register knows about. This is the
///     destructive act, and the court was precise about why the register is the test:
///     "a record untracked out of every register, or held in a store on no register,
///     is DELETED IN LAW though no byte is erased."
///
/// KEYED ON THE RECORD ID, NOT THE PATH, which is the whole of D1. A path answers
/// "did this file go away". Only an id answers "did this RECORD go away", and the
/// second is the question the Act asks.
///
/// The occasion was 626 untrackings in one commit on 2026-08-06. Every one of them
/// would have raised a destructive-delete warning on a record that had not been
/// deleted, and a real deletion in that commit would have been the 627th line of an
/// alarm nobody could read.
pub(crate) fn record_removal_findings(repo: &Path) -> Vec<Finding> {
    let Ok(deletions) = GitIntegration::read_staged_deletions(repo) else {
        return Vec::new();
    };
    let removed: Vec<&String> = deletions
        .iter()
        .filter(|d| front_door::is_governed_record(d))
        .collect();
    if removed.is_empty() {
        return Vec::new();
    }

    let stores = crate::store_register::registered_stores(repo);
    let in_a_registered_store = |rel: &str| {
        stores
            .iter()
            .any(|s| rel == s || rel.starts_with(&format!("{s}/")))
    };

    // The id -> path index over every registered store, built ONCE. A per-deletion
    // scan would be 626 tree walks on the commit that motivated this.
    let surviving = surviving_record_ids(repo, &stores);

    let mut out = Vec::new();
    for rel in removed {
        // Still on disk, inside a store the register knows about: unpublished.
        if repo.join(rel).exists() && in_a_registered_store(rel) {
            out.push(
                f(
                    Severity::Info,
                    "RECORD_UNPUBLISHED",
                    format!(
                        "'{rel}' leaves version tracking and remains on disk in a registered \
                         store. Untracking is not deletion ([2026] VJS-CC-VJS 20): publication \
                         is constitutively inert, so ceasing to publish removes no force. \
                         Route it as a publication decision, never as irreversible."
                    ),
                )
                .at(PathBuf::from(rel)),
            );
            continue;
        }

        // The id survives elsewhere: a rename or a re-projection, never a deletion.
        let id = record_id_at_head(repo, rel);
        let moved_to = id
            .as_deref()
            .and_then(|i| surviving.get(i))
            .filter(|now_at| *now_at != rel);
        if let Some(now_at) = moved_to {
            let id = id.as_deref().unwrap_or(rel);
            out.push(
                f(
                    Severity::Info,
                    "RECORD_RENAMED",
                    format!(
                        "record '{id}' moved from '{rel}' to '{now_at}'. A rename is not \
                                 a deletion and is never reported as one ([2026] VJS-CC-VJS 20 \
                                 D18); the record is where it always was, under another name."
                    ),
                )
                .at(PathBuf::from(rel)),
            );
            continue;
        }

        // Nowhere the register knows about. Deleted in law, whatever the disk says.
        let named = id.as_deref().unwrap_or(rel);
        out.push(
            f(
                Severity::Warning,
                "DESTRUCTIVE_RECORD_DELETE",
                format!(
                    "governed record '{named}' (at '{rel}') survives in NO registered store - \
                     this is a destructive act (ACT-006:s4; ACT-004:s9). A record untracked out \
                     of every register, or held in a store on no register, is deleted in law \
                     though no byte is erased ([2026] VJS-CC-VJS 20). Confirm it is \
                     human-approved and authorised."
                ),
            )
            .at(PathBuf::from(rel))
            .fix("Route with --irreversible and record the authority, or keep the record inside a registered store."),
        );
    }
    out
}

/// The top-level `id:` of a record as it stood at HEAD - the only place to read it,
/// because the working-tree file may be gone.
fn record_id_at_head(repo: &Path, rel: &str) -> Option<String> {
    let text = GitIntegration::read_blob_at_head(repo, rel).ok()??;
    text.lines().find_map(|l| {
        let v = l.strip_prefix("id:")?.trim();
        let v = v.trim_matches('"').trim_matches('\'').trim();
        (!v.is_empty()).then(|| v.to_string())
    })
}

/// Every record id currently on disk in a registered store, mapped to where it lives.
fn surviving_record_ids(
    repo: &Path,
    stores: &[String],
) -> std::collections::HashMap<String, String> {
    let mut out = std::collections::HashMap::new();
    for store in stores {
        let root = repo.join(store);
        if !root.is_dir() {
            continue;
        }
        for entry in walkdir::WalkDir::new(&root).into_iter().flatten() {
            let path = entry.path();
            match path.extension().and_then(|e| e.to_str()) {
                Some("yaml") | Some("yml") => {}
                _ => continue,
            }
            let Ok(text) = std::fs::read_to_string(path) else {
                continue;
            };
            let Some(id) = text.lines().find_map(|l| {
                let v = l.strip_prefix("id:")?.trim();
                let v = v.trim_matches('"').trim_matches('\'').trim();
                (!v.is_empty()).then(|| v.to_string())
            }) else {
                continue;
            };
            let rel = path
                .strip_prefix(repo)
                .unwrap_or(path)
                .to_string_lossy()
                .to_string();
            out.entry(id).or_insert(rel);
        }
    }
    out
}
