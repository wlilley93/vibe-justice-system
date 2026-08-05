use crate::Authority;
use crate::AuthorityGraph;
use crate::error::*;
use crate::remove_superseded;
use crate::sort_by_rank_then_specificity_then_date;
use crate::types::*;

pub fn resolve_authority(
    input: &RouteInput,
    graph: &AuthorityGraph,
) -> Result<AuthoritySet, KernelError> {
    let candidates: Vec<Authority> = graph
        .authorities
        .values()
        .filter(|a| a.status.is_live())
        .filter(|a| {
            a.scope
                .as_ref()
                .map(|s| scope_matches(s, input))
                .unwrap_or(true)
        })
        .cloned()
        .collect();

    let unsuperseded = remove_superseded(candidates, graph);
    let sorted = sort_by_rank_then_specificity_then_date(unsuperseded, graph);

    // ON-POINT AUTHORITY FIRST. Rank alone is the wrong order for a citator.
    //
    // Sorted by rank, the constitutional recitals always win and any County Court order that decides
    // the very issue asked about falls below the truncation limit - so `lookup --issue X` returned
    // the same five ACT-001 sections whatever X was, including for an issue that did not exist. An
    // answer that does not depend on the question cannot inform a decision, and this one was feeding
    // the check that is supposed to stop settled law being re-litigated (S-11(c)).
    //
    // So when the caller names an issue, authorities whose own issue tags match it are hoisted above
    // those that do not. Both groups keep their internal rank order, so a Supreme order on the issue
    // still outranks a County one on the same issue, and nothing that used to be returned is dropped
    // - it moves down. Where no issue is named the order is exactly as before.
    let sorted = if input.issue_tags.is_empty() {
        sorted
    } else {
        let (on_point, rest): (Vec<Authority>, Vec<Authority>) =
            sorted.into_iter().partition(|a| {
                a.issue_tags.iter().any(|t| {
                    input
                        .issue_tags
                        .iter()
                        .any(|q| fold_tag(&q.0) == fold_tag(&t.0))
                })
            });
        on_point.into_iter().chain(rest).collect()
    };

    let pointers: Vec<AuthorityPointer> = sorted
        .into_iter()
        .map(|a| AuthorityPointer {
            id: a.id,
            title: a.title,
            rank: a.rank,
            status: a.status,
            summary: a.summary,
            source_path: a.source_path,
            issue_tags: a.issue_tags,
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

    if let Some(ref action_kinds) = scope.action_kinds
        && !action_kinds.contains(&input.action_kind)
    {
        return false;
    }

    if let Some(ref issue_tags) = scope.issue_tags
        && !input.issue_tags.iter().any(|it| issue_tags.contains(it))
    {
        return false;
    }

    true
}

#[cfg(test)]
mod on_point_tests {
    use super::*;
    use crate::{AuthorityGraph, AuthorityKind};

    fn auth(id: &str, rank: AuthorityRank, tags: Vec<&str>) -> Authority {
        Authority {
            id: AuthorityId(id.into()),
            kind: AuthorityKind::Order,
            rank,
            status: AuthorityStatus::Binding,
            jurisdiction: None,
            title: id.into(),
            summary: id.into(),
            source_path: None,
            issue_tags: tags.into_iter().map(|t| IssueTag(t.into())).collect(),
            scope: None,
            supersedes: Vec::new(),
        }
    }

    fn input(tags: Vec<&str>) -> RouteInput {
        RouteInput {
            repo_root: None,
            jurisdiction: None,
            actor: "lexby".into(),
            action_kind: ActionKind::ImplementationDecision,
            issue_tags: tags.into_iter().map(|t| IssueTag(t.into())).collect(),
            intent: "t".into(),
            affected_paths: Vec::new(),
            risk: RiskLevel::Low,
            public_target: false,
            external_target: false,
            irreversible: false,
            user_instruction: None,
        }
    }

    fn graph_of(items: Vec<Authority>) -> AuthorityGraph {
        let mut g = AuthorityGraph::new();
        for a in items {
            g.authorities.insert(a.id.clone(), a);
        }
        g
    }

    /// THE DEFECT THIS EXISTS TO REFUSE. Before 2026-07-27 the resolver ignored `issue_tags`
    /// entirely, so `vjs lookup --issue X` returned byte-identical output for a real binding issue
    /// and for one that did not exist. A lookup whose answer does not depend on the question cannot
    /// inform a decision, and this one feeds the check that is meant to stop settled law being
    /// re-litigated (S-11(c)).
    #[test]
    fn an_on_point_order_outranks_a_general_recital() {
        let g = graph_of(vec![
            auth("ACT-001:s1", AuthorityRank::Constitutional, vec![]),
            auth(
                "CC-OPBOX-5",
                AuthorityRank::CountyCourt,
                vec!["credential_return"],
            ),
        ]);
        let set = resolve_authority(&input(vec!["credential_return"]), &g).unwrap();
        assert_eq!(
            set.authorities[0].id.0, "CC-OPBOX-5",
            "the order deciding the very issue asked about must come first, not the recital"
        );
    }

    /// The other half: a question with no authority on it must NOT surface an unrelated order as if
    /// it were on point. Without this, hoisting would be indistinguishable from noise.
    #[test]
    fn an_unrelated_issue_does_not_hoist_anything() {
        let g = graph_of(vec![
            auth("ACT-001:s1", AuthorityRank::Constitutional, vec![]),
            auth(
                "CC-OPBOX-5",
                AuthorityRank::CountyCourt,
                vec!["credential_return"],
            ),
        ]);
        let set = resolve_authority(&input(vec!["something_else_entirely"]), &g).unwrap();
        assert_eq!(
            set.authorities[0].id.0, "ACT-001:s1",
            "an issue with no authority on it must fall back to rank order"
        );
    }

    /// With no issue named at all the order is exactly as it was before this change, so nothing that
    /// depended on plain rank ordering is disturbed.
    #[test]
    fn no_issue_named_leaves_rank_order_untouched() {
        let g = graph_of(vec![
            auth("ACT-001:s1", AuthorityRank::Constitutional, vec![]),
            auth(
                "CC-OPBOX-5",
                AuthorityRank::CountyCourt,
                vec!["credential_return"],
            ),
        ]);
        let set = resolve_authority(&input(vec![]), &g).unwrap();
        assert_eq!(set.authorities[0].id.0, "ACT-001:s1");
    }
}
