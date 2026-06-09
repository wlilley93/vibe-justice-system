use crate::types::*;
use crate::error::*;
use crate::AuthorityGraph;
use crate::Authority;
use crate::remove_superseded;
use crate::sort_by_rank_then_specificity_then_date;

pub fn resolve_authority(
    input: &RouteInput,
    graph: &AuthorityGraph,
) -> Result<AuthoritySet, KernelError> {
    let candidates: Vec<Authority> = graph
        .authorities
        .values()
        .filter(|a| a.status.is_live())
        .filter(|a| a.scope.as_ref().map(|s| scope_matches(s, input)).unwrap_or(true))
        .cloned()
        .collect();

    let unsuperseded = remove_superseded(candidates, graph);
    let sorted = sort_by_rank_then_specificity_then_date(unsuperseded, graph);

    let pointers: Vec<AuthorityPointer> = sorted
        .into_iter()
        .map(|a| AuthorityPointer {
            id: a.id,
            title: a.title,
            rank: a.rank,
            status: a.status,
            summary: a.summary,
            source_path: a.source_path,
        })
        .collect();

    Ok(AuthoritySet {
        authorities: pointers,
    })
}

fn scope_matches(scope: &Scope, input: &RouteInput) -> bool {
    if let Some(ref jurisdictions) = scope.jurisdictions {
        let wildcard = jurisdictions.iter().any(|j| j.0 == "*");
        match input.jurisdiction {
            // A scope that names jurisdictions is a restriction; an input with
            // no jurisdiction must not slip past it (only "*" lets it through).
            Some(ref input_j) => {
                if !jurisdictions.contains(input_j) && !wildcard {
                    return false;
                }
            }
            None => {
                if !wildcard {
                    return false;
                }
            }
        }
    }

    if let Some(ref action_kinds) = scope.action_kinds {
        if !action_kinds.contains(&input.action_kind) {
            return false;
        }
    }

    if let Some(ref issue_tags) = scope.issue_tags {
        if !input.issue_tags.iter().any(|it| issue_tags.contains(it)) {
            return false;
        }
    }

    true
}
