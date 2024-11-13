mod forms;
use lalrpop_util::lalrpop_mod;
pub mod pos;

lalrpop_mod!(pub grammar);

#[cfg(test)]
mod tests {
    use internal_baml_diagnostics::SourceFile;

    use super::*;
    use crate::grammar::*;

    #[test]
    fn test_parse_identifier() {
        let source_file = SourceFile::new_static("tmp.baml".into(), "");
        let identifier = IdentifierParser::new().parse(&source_file, "foo").unwrap();
        assert_eq!(identifier.name, "foo");
    }

}

// use internal_baml_diagnostics::{DatamodelError, DatamodelWarning, Diagnostics, Span};
// use crate::lexer::BAMLParser;
//  use crate::forms::{Function, Enum};
// 
// /// The abstract syntax of the BAML language, paired with an arbitrary
// /// type of metadata. Metadata will typically be a `Span`.
// pub struct Ast<T> {
//     pub top_levels: Vec<AstTopLevel<T>>,
//     pub meta: T
// }
// 
// /// The different elements that can be found an the top level of a BAML
// /// source file.
// pub enum AstTopLevel<T> {
//     // Class(Class<T>),
//     Enum (Enum<T>),
//     Function (Function<T>),
//     // TemplateString (TemplateString<T>),
//     // Test (Test<T>)
// }
// 
// /// Parsing at the top level always returns an `Ast`. It is
// /// infallible so that any errors encountered during parsing
// /// must be handled as diagnostics.
// /// 
// /// When encountering some kind of error during parsing,
// /// we prefer to add an error to the `Diagnostics` context
// /// and return a partial AST element.
// pub fn parse(_diagnostics: &mut Diagnostics) -> Ast<Span> {
//     unimplemented!()
// }