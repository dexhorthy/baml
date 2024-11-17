
use internal_baml_diagnostics::Span;

use crate::forms::identifier::Identifier;
use crate::forms::expression::Expression;
use crate::pos::WithPos;


pub struct Attribute<T> {
    pub name: Identifier<T>,
    pub args: Vec<AttributeArgument<T>>,
}

pub enum AttributeArgument<T> {
    Positional{ expression: Expression<T> },
    Named{ name: Identifier<T>, expression: Expression<T> }
}
