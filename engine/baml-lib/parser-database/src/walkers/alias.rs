use std::collections::HashSet;

use super::TypeWalker;
use internal_baml_schema_ast::ast::{self, FieldType, Identifier, WithName};

pub type TypeAliasWalker<'db> = super::Walker<'db, ast::TypeAliasId>;

impl<'db> TypeAliasWalker<'db> {
    /// Name of the type alias.
    pub fn name(&self) -> &str {
        &self.db.ast[self.id].identifier.name()
    }

    /// Returns a set containing the names of the types aliased by `self`.
    ///
    /// Note that the parser DB must be populated for this method to work.
    pub fn targets(&self) -> &'db HashSet<String> {
        &self.db.types.type_aliases[&self.id]
    }

    /// Returns a set containing the final resolution of the type alias.
    ///
    /// If the set is empty it just means that the type resolves to primitives
    /// instead of symbols.
    ///
    /// Parser DB must be populated before calling this method.
    pub fn resolved(&self) -> &'db HashSet<String> {
        &self.db.types.resolved_type_aliases[&self.id]
    }

    /// Returns the field type of the aliased type.
    pub fn ast_field_type(&self) -> &'db FieldType {
        &self.db.ast[self.id].value
    }
}
