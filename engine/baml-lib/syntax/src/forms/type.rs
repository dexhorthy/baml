/// The syntax of a type.
///
/// For example:
/// ```
/// int                    <- bare builtin type
/// string                 <- bare builtin type
/// MyType                 <- bare user type
/// 1                      <- literal type
/// int | string           <- choice type (union)
/// int[] | MyType?        <- choice of list or optional
/// (int | bool[])[]       <- list of (int or list of bool)
/// int @attr(arg) | bool  <- int with attr or bool
/// ```
///
/// These types are modeled in the syntax
use internal_baml_diagnostics::Span;

use crate::forms::identifier::Identifier;
use crate::forms::attribute::Attribute;
use crate::pos::WithPos;

use std::str::FromStr;

pub enum Type<T> {
    Builtin{ builtin_type: BuiltinType, meta: T },
    Option{ base_type: Box<Type<T>>, meta: T },
    List{ base_type: Box<Type<T>>, meta: T },
    Union{ variants: Vec<Type<T>>, meta: T },
    WithAttributes{ base_type: Box<Type<T>>, attributes: Vec<Attribute<T>>, meta:T },
}

#[derive(Clone, Debug, PartialEq)]
pub enum BuiltinType {
    Int,
    String,
    Float,
    Bool,
    Image,
    Audio,
}

impl FromStr for BuiltinType {
    fn from_str(s: &str) -> Result<Self, Error> {
        use BuiltinType::*;
        match s {
            "int" => Ok(Int),
            "float" => Ok(Float),
            "bool" => Ok(Bool),
            "string" => Ok(String),
            "image" => Ok(Image),
            "audio" => Ok(Audio),
            _ => Err(todo!())
        }
    }
}

#[cfg(test)]
mod tests {
    use internal_baml_diagnostics::SourceFile;
    use super::*;
    use crate::grammar::TypeParser;

    #[test]
    fn test_parse_type() {
        let source_file = SourceFile::new_static("tmp.baml".into(), "");
        let ty = TypeParser::new().parse(&source_file, "int").unwrap();
        if let Type::Builtin{ builtin_type, meta } = ty {
            assert_eq!(builtin_type, BuiltinType::Int);
        } else {
            panic!("Expected builtin type int");
        }
    }
}
