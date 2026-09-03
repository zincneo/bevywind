use crate::{Property, StyleError, StyleRule, Value};

pub(crate) fn parse(class: &str, offset: usize) -> Option<Result<StyleRule, StyleError>> {
    if let Some(value) = class.strip_prefix("gz_") {
        return Some(parse_value(value, class, offset, Property::GlobalZIndex));
    }
    if let Some(value) = class.strip_prefix("z_") {
        return Some(parse_value(value, class, offset, Property::ZIndex));
    }
    None
}

fn parse_value(
    value: &str,
    class: &str,
    offset: usize,
    property: Property,
) -> Result<StyleRule, StyleError> {
    let (negative, value) = value
        .strip_prefix("n_")
        .map_or((false, value), |value| (true, value));
    let value = crate::units::parse_number(value, class, offset, "z-index")? as i32;
    let value = if negative { -value } else { value };
    Ok(StyleRule {
        property,
        value: match property {
            Property::ZIndex => Value::ZIndex(value),
            Property::GlobalZIndex => Value::GlobalZIndex(value),
            _ => unreachable!(),
        },
    })
}
