//! The kernel spec layer: the authority data model (`model`), the deterministic invariant evaluator
//! (`evaluate`), and the permit lifecycle verbs (`lifecycle`). The public surface is re-exported
//! unchanged so `crate::spec::*` (and the crate-root `pub use spec::*`) resolve as before.

mod evaluate;
mod lifecycle;
mod model;

pub use evaluate::*;
pub use lifecycle::*;
pub use model::*;
