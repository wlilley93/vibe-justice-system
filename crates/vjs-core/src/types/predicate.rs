//! The predicate AST: RawPredicate (wire form) + PredicateExpr (typed) + the parse.
//! Split out of types.rs (behavior-preserving; re-exported via `pub use`).
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RawPredicate {
    pub kind: String,
    pub items: Option<Vec<RawPredicate>>,
    pub item: Option<Box<RawPredicate>>,
    pub condition: Option<Box<RawPredicate>>,
    pub then: Option<Box<RawPredicate>>,
    pub glob: Option<String>,
    pub pattern: Option<String>,
    pub value: Option<String>,
    pub name: Option<String>,
    pub issue: Option<String>,
    pub id: Option<String>,
    pub field: Option<String>,
    pub max: Option<usize>,
    pub fields: Option<Vec<String>>,
    pub allowed: Option<Vec<String>>,
    /// The proof kind a `proof_exists` predicate requires. `kind` is taken by
    /// the predicate discriminator itself, so this gets its own field.
    pub proof_kind: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PredicateExpr {
    All {
        items: Vec<PredicateExpr>,
    },
    Any {
        items: Vec<PredicateExpr>,
    },
    None {
        items: Vec<PredicateExpr>,
    },
    Not {
        item: Box<PredicateExpr>,
    },
    If {
        condition: Box<PredicateExpr>,
        then: Box<PredicateExpr>,
    },
    PathChanged {
        glob: String,
    },
    FileAdded {
        pattern: String,
    },
    FileModified {
        pattern: String,
    },
    FileDeleted {
        pattern: String,
    },
    StringContains {
        value: String,
    },
    ImportContains {
        value: String,
    },
    DependencyAdded {
        name: String,
    },
    DependencyRemoved {
        name: String,
    },
    DecisionLogExists {
        issue: Option<String>,
    },
    PermitExists {
        id: Option<String>,
    },
    ProofExists {
        kind: Option<String>,
    },
    OrderExists {
        issue: Option<String>,
    },
    WordCountLte {
        field: String,
        max: usize,
    },
    FileWordsLte {
        glob: String,
        max: usize,
    },
    CitationUnique,
    RequiredFields {
        fields: Vec<String>,
    },
    FieldEquals {
        field: String,
        value: String,
    },
    IncludedInRuntimeAuthorityGraph,
    PublicNoPrivateFacts,
    CoreNoModelCalls,
    CoreNoNetwork,
    GovernedWritesRequirePermit,
    ProofsExistBeforeClose,
    LogsStayShort,
    LawpackValidates,
    NoDuplicateIds,
    NoDuplicateCitations,
    OrdersHaveDirectives,
    McpLocalFirst,
    DirectoryRolesResolve,
    V1NotLoadedByDefault,
    /// Affirmative, fail-closed allow-list enforcement of CASE-LAW s. 23(5)
    /// ([2026] REALM-SC 10): a record that claims runtime force carries it ONLY
    /// if it declares an `assent_source` resolving to one of `allowed` (e.g. a
    /// specific Sovereign-assent event, or a standing-bounded route tracing to
    /// specific assent). Absence, emptiness, an unrecognised form, or an
    /// unresolved trace each cause rejection. This is NOT a deny-list: a record
    /// that merely omits `assent_source` is rejected, never passed.
    AssentSourceValid {
        allowed: Vec<String>,
    },
}

impl RawPredicate {
    pub fn to_predicate(&self) -> Result<PredicateExpr, String> {
        match self.kind.as_str() {
            "all" => {
                let items = self
                    .items
                    .as_ref()
                    .ok_or("all requires items")?
                    .iter()
                    .map(|i| i.to_predicate())
                    .collect::<Result<Vec<_>, _>>()?;
                Ok(PredicateExpr::All { items })
            }
            "any" => {
                let items = self
                    .items
                    .as_ref()
                    .ok_or("any requires items")?
                    .iter()
                    .map(|i| i.to_predicate())
                    .collect::<Result<Vec<_>, _>>()?;
                Ok(PredicateExpr::Any { items })
            }
            "none" => {
                let items = self
                    .items
                    .as_ref()
                    .ok_or("none requires items")?
                    .iter()
                    .map(|i| i.to_predicate())
                    .collect::<Result<Vec<_>, _>>()?;
                Ok(PredicateExpr::None { items })
            }
            "not" => {
                let item = self
                    .item
                    .as_ref()
                    .ok_or("not requires item")?
                    .to_predicate()?;
                Ok(PredicateExpr::Not {
                    item: Box::new(item),
                })
            }
            "if" => {
                let condition = self
                    .condition
                    .as_ref()
                    .ok_or("if requires condition")?
                    .to_predicate()?;
                let then = self
                    .then
                    .as_ref()
                    .ok_or("if requires then")?
                    .to_predicate()?;
                Ok(PredicateExpr::If {
                    condition: Box::new(condition),
                    then: Box::new(then),
                })
            }
            "path_changed" => {
                let glob = self
                    .glob
                    .as_ref()
                    .ok_or("path_changed requires glob")?
                    .clone();
                Ok(PredicateExpr::PathChanged { glob })
            }
            "file_added" => {
                let pattern = self
                    .pattern
                    .as_ref()
                    .ok_or("file_added requires pattern")?
                    .clone();
                Ok(PredicateExpr::FileAdded { pattern })
            }
            "file_modified" => {
                let pattern = self
                    .pattern
                    .as_ref()
                    .ok_or("file_modified requires pattern")?
                    .clone();
                Ok(PredicateExpr::FileModified { pattern })
            }
            "file_deleted" => {
                let pattern = self
                    .pattern
                    .as_ref()
                    .ok_or("file_deleted requires pattern")?
                    .clone();
                Ok(PredicateExpr::FileDeleted { pattern })
            }
            "string_contains" => {
                let value = self
                    .value
                    .as_ref()
                    .ok_or("string_contains requires value")?
                    .clone();
                Ok(PredicateExpr::StringContains { value })
            }
            "import_contains" => {
                let value = self
                    .value
                    .as_ref()
                    .ok_or("import_contains requires value")?
                    .clone();
                Ok(PredicateExpr::ImportContains { value })
            }
            "dependency_added" => {
                let name = self
                    .name
                    .as_ref()
                    .ok_or("dependency_added requires name")?
                    .clone();
                Ok(PredicateExpr::DependencyAdded { name })
            }
            "dependency_removed" => {
                let name = self
                    .name
                    .as_ref()
                    .ok_or("dependency_removed requires name")?
                    .clone();
                Ok(PredicateExpr::DependencyRemoved { name })
            }
            "decision_log_exists" => Ok(PredicateExpr::DecisionLogExists {
                issue: self.issue.clone(),
            }),
            "permit_exists" => Ok(PredicateExpr::PermitExists {
                id: self.id.clone(),
            }),
            "proof_exists" => Ok(PredicateExpr::ProofExists {
                kind: self.proof_kind.clone(),
            }),
            "order_exists" => Ok(PredicateExpr::OrderExists {
                issue: self.issue.clone(),
            }),
            "word_count_lte" => {
                let field = self
                    .field
                    .as_ref()
                    .ok_or("word_count_lte requires field")?
                    .clone();
                let max = self.max.ok_or("word_count_lte requires max")?;
                Ok(PredicateExpr::WordCountLte { field, max })
            }
            "file_words_lte" => {
                let glob = self
                    .glob
                    .as_ref()
                    .ok_or("file_words_lte requires glob")?
                    .clone();
                let max = self.max.ok_or("file_words_lte requires max")?;
                Ok(PredicateExpr::FileWordsLte { glob, max })
            }
            "citation_unique" => Ok(PredicateExpr::CitationUnique),
            "required_fields" => {
                let fields = self
                    .fields
                    .as_ref()
                    .ok_or("required_fields requires fields")?
                    .clone();
                Ok(PredicateExpr::RequiredFields { fields })
            }
            "field_equals" => {
                let field = self
                    .field
                    .as_ref()
                    .ok_or("field_equals requires field")?
                    .clone();
                let value = self
                    .value
                    .as_ref()
                    .ok_or("field_equals requires value")?
                    .clone();
                Ok(PredicateExpr::FieldEquals { field, value })
            }
            "included_in_runtime_authority_graph" => {
                Ok(PredicateExpr::IncludedInRuntimeAuthorityGraph)
            }
            "public_no_private_facts" => Ok(PredicateExpr::PublicNoPrivateFacts),
            "core_no_model_calls" => Ok(PredicateExpr::CoreNoModelCalls),
            "core_no_network" => Ok(PredicateExpr::CoreNoNetwork),
            "governed_writes_require_permit" => Ok(PredicateExpr::GovernedWritesRequirePermit),
            "proofs_exist_before_close" => Ok(PredicateExpr::ProofsExistBeforeClose),
            "logs_stay_short" => Ok(PredicateExpr::LogsStayShort),
            "lawpack_validates" => Ok(PredicateExpr::LawpackValidates),
            "no_duplicate_ids" => Ok(PredicateExpr::NoDuplicateIds),
            "no_duplicate_citations" => Ok(PredicateExpr::NoDuplicateCitations),
            "orders_have_directives" => Ok(PredicateExpr::OrdersHaveDirectives),
            "mcp_local_first" => Ok(PredicateExpr::McpLocalFirst),
            "directory_roles_resolve" => Ok(PredicateExpr::DirectoryRolesResolve),
            "v1_not_loaded_by_default" => Ok(PredicateExpr::V1NotLoadedByDefault),
            "assent_source_valid" => {
                let allowed = self
                    .allowed
                    .as_ref()
                    .ok_or("assent_source_valid requires allowed")?
                    .clone();
                if allowed.is_empty() {
                    return Err("assent_source_valid requires a non-empty allowed list".to_string());
                }
                Ok(PredicateExpr::AssentSourceValid { allowed })
            }
            other => Err(format!("Unknown predicate kind: {}", other)),
        }
    }
}
