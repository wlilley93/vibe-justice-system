//! The resource vocabulary the capability primitive is generic over (K-9).
//!
//! The primitive never learns a subscriber's resource SEMANTICS (PC-19 K2): a vocabulary only answers
//! two deterministic questions. `covers` - does this capability PATTERN match a concrete REQUEST.
//! `within` - is this child pattern an attenuation of (no wider than) the parent, used for delegation.
//! `TypedResource` is the built-in typed vocabulary; the VJS permit gate supplies a path-glob
//! `PathScope` in governance.rs.

use super::CapError;

/// The resource vocabulary the capability primitive is generic over. The primitive never learns a
/// subscriber's resource SEMANTICS (PC-19 K2): it only asks a vocabulary two deterministic
/// questions. `covers` - does this capability PATTERN match a concrete REQUEST. `within` - is this
/// child pattern an attenuation of (no wider than) the parent, used for delegation. `TypedResource`
/// is the built-in typed vocabulary; the VJS permit gate supplies a path-glob `PathScope`.
pub trait Resource: Clone + std::fmt::Debug {
    fn covers(&self, req: &Self) -> bool;
    fn within(&self, parent: &Self) -> bool;
}

/// A typed, canonical resource `kind:body`. As a capability PATTERN, `body` may be `*`
/// (whole kind) or end `/*` (a subtree); a concrete REQUEST is always exact.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypedResource {
    pub kind: String,
    pub body: String,
}

impl TypedResource {
    pub fn parse(s: &str) -> Result<Self, CapError> {
        if s == "*" {
            return Err(CapError::BareWildcard); // K-9: bare global wildcard rejected
        }
        let (kind, body) = s.split_once(':').ok_or(CapError::Untyped)?;
        if kind.is_empty() || body.is_empty() {
            return Err(CapError::Untyped);
        }
        // K-9: wildcards are terminal-only.
        if body.contains('*') && body != "*" && !body.ends_with("/*") {
            return Err(CapError::NonTerminalWildcard);
        }
        Ok(Self {
            kind: kind.to_string(),
            body: body.to_string(),
        })
    }
}

impl Resource for TypedResource {
    /// Does this pattern COVER a concrete requested resource? Prefix-collision-safe:
    /// `k:src/*` covers `k:src/main` but NOT `k:src2/main`.
    fn covers(&self, req: &TypedResource) -> bool {
        if self.kind != req.kind {
            return false;
        }
        if self.body == "*" {
            return true;
        }
        if let Some(prefix) = self.body.strip_suffix("/*") {
            return req.body == prefix || req.body.starts_with(&format!("{prefix}/"));
        }
        self.body == req.body
    }

    /// Is `self` (a child pattern) within `parent` (no widening)? Used for delegation attenuation.
    fn within(&self, parent: &TypedResource) -> bool {
        if self.kind != parent.kind {
            return false;
        }
        match (parent.body.as_str(), self.body.strip_suffix("/*")) {
            ("*", _) => true,
            (_, Some(child_prefix)) => parent.covers(&TypedResource {
                kind: self.kind.clone(),
                body: child_prefix.to_string(),
            }),
            (_, None) => parent.covers(self),
        }
    }
}
