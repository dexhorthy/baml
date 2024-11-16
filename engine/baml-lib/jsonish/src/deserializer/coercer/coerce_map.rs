use std::collections::VecDeque;

use anyhow::Result;

use crate::deserializer::{
    deserialize_flags::{DeserializerConditions, Flag},
    types::BamlValueWithFlags,
};
use baml_types::{BamlMap, FieldType, LiteralValue, TypeValue};

use super::{ParsingContext, ParsingError, TypeCoercer};

pub(super) fn coerce_map(
    ctx: &ParsingContext,
    map_target: &FieldType,
    value: Option<&crate::jsonish::Value>,
) -> Result<BamlValueWithFlags, ParsingError> {
    log::debug!(
        "scope: {scope} :: coercing to: {name} (current: {current})",
        name = map_target.to_string(),
        scope = ctx.display_scope(),
        current = value.map(|v| v.r#type()).unwrap_or("<null>".into())
    );

    let Some(value) = value else {
        return Err(ctx.error_unexpected_null(map_target));
    };

    let FieldType::Map(key_type, value_type) = map_target else {
        return Err(ctx.error_unexpected_type(map_target, value));
    };

    if !matches!(
        **key_type,
        FieldType::Primitive(TypeValue::String) | FieldType::Enum(_) | FieldType::Union(_)
    ) {
        return Err(ctx.error_map_must_have_supported_key(key_type));
    }

    if let FieldType::Union(items) = &**key_type {
        let mut queue = VecDeque::from_iter(items.iter());
        while let Some(item) = queue.pop_front() {
            match item {
                FieldType::Literal(LiteralValue::String(_)) => continue,
                FieldType::Union(nested) => queue.extend(nested.iter()),
                other => return Err(ctx.error_map_must_have_supported_key(other)),
            }
        }
    }

    let mut flags = DeserializerConditions::new();
    flags.add_flag(Flag::ObjectToMap(value.clone()));

    match &value {
        crate::jsonish::Value::Object(obj) => {
            let mut items = BamlMap::new();
            for (key, value) in obj.iter() {
                match value_type.coerce(&ctx.enter_scope(key), value_type, Some(value)) {
                    Ok(v) => {
                        items.insert(key.clone(), (DeserializerConditions::new(), v));
                    }
                    Err(e) => flags.add_flag(Flag::MapValueParseError(key.clone(), e)),
                }
            }
            Ok(BamlValueWithFlags::Map(flags, items))
        }
        // TODO: first map in an array that matches
        _ => Err(ctx.error_unexpected_type(map_target, value)),
    }
}
