use internal_baml_diagnostics::Span;

use crate::forms::identifier::Identifier;
use crate::pos::WithPos;

// TODO: Fill this out.
pub enum Expression<T> {
    Variable{ identifier: Identifier<T> }
}
