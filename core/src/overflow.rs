use crate::{Property, StyleError, StyleRule, Value};

pub(crate) fn parse(class: &str, offset: usize) -> Option<Result<StyleRule, StyleError>> {
    let (property, value) = if let Some(value) = class.strip_prefix("overflow_x_") {
        (Property::OverflowX, value)
    } else if let Some(value) = class.strip_prefix("overflow_y_") {
        (Property::OverflowY, value)
    } else if let Some(value) = class.strip_prefix("overflow_") {
        (Property::OverflowX, value)
    } else {
        return None;
    };

    let value = match value {
        "visible" => Value::OverflowVisible,
        "clip" => Value::OverflowClip,
        "hidden" => Value::OverflowHidden,
        "scroll" => Value::OverflowScroll,
        _ => return Some(Err(crate::error(class, offset, "invalid overflow value"))),
    };
    Some(Ok(StyleRule { property, value }))
}

pub(crate) fn expansion(class: &str, offset: usize) -> Option<Result<Vec<StyleRule>, StyleError>> {
    let Some(value) = class.strip_prefix("overflow_") else {
        return None;
    };
    if value.starts_with("x_") || value.starts_with("y_") {
        return None;
    }

    let rule = parse(class, offset)?.ok()?;
    let value = rule.value;
    Some(Ok(vec![
        StyleRule {
            property: Property::OverflowX,
            value: value.clone(),
        },
        StyleRule {
            property: Property::OverflowY,
            value,
        },
    ]))
}
