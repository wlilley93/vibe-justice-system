use thiserror::Error;

use crate::types::*;

#[derive(Debug, Error)]
pub enum KernelError {
    #[error("invalid input: {0}")]
    InvalidInput(String),

    #[error("lawpack invalid: {0}")]
    LawpackInvalid(String),

    #[error("authority not found: {0}")]
    AuthorityNotFound(String),

    #[error("citation collision: {0}")]
    CitationCollision(String),

    #[error("word limit exceeded: {actual}/{limit}")]
    WordLimitExceeded { actual: usize, limit: usize },

    #[error("private boundary violation: {0}")]
    PrivateBoundaryViolation(String),

    #[error("court required: {0:?}")]
    CourtRequired(CourtTrigger),

    #[error("storage error: {0}")]
    Store(String),

    #[error("invalid path: {0}")]
    InvalidPath(String),

    #[error("permit not found: {0}")]
    PermitNotFound(String),

    #[error("permit expired: {0}")]
    PermitExpired(String),

    #[error("missing proof: {0}")]
    MissingProof(String),

    #[error("invariant failed: {0}")]
    InvariantFailed(String),

    #[error("spec not found: {0}")]
    SpecNotFound(String),

    #[error("validation failed: {0}")]
    ValidationFailed(String),

    #[error("serialization error: {0}")]
    Serialization(String),

    #[error("io error: {0}")]
    Io(String),
}
