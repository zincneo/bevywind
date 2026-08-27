use crate::{Property, StyleError, StyleRule, Value, error};

pub(crate) fn parse(class: &str, offset: usize) -> Result<StyleRule, StyleError> {
    let (property, value) = if let Some(value) = class.strip_prefix("max-h-") {
        (Property::MaxHeight, value)
    } else if let Some(value) = class.strip_prefix("max-w-") {
        (Property::MaxWidth, value)
    } else if let Some(value) = class.strip_prefix("min-h-") {
        (Property::MinHeight, value)
    } else if let Some(value) = class.strip_prefix("min-w-") {
        (Property::MinWidth, value)
    } else if let Some(value) = class.strip_prefix("h-") {
        (Property::Height, value)
    } else if let Some(value) = class.strip_prefix("w-") {
        (Property::Width, value)
    } else {
        return Err(error(class, offset, "unknown style utility"));
    };

    let value = if value == "full" {
        Value::Percent(100)
    } else if let Some(number) = value.strip_suffix("px") {
        Value::Pixels(parse_number(number, class, offset)?)
    } else if let Some(number) = value.strip_suffix('%') {
        Value::Percent(parse_number(number, class, offset)?)
    } else if let Some(number) = value.strip_suffix('w') {
        Value::ViewportWidth(parse_number(number, class, offset)?)
    } else if let Some(number) = value.strip_suffix('h') {
        Value::ViewportHeight(parse_number(number, class, offset)?)
    } else {
        return Err(error(class, offset, "invalid dimension value"));
    };

    Ok(StyleRule { property, value })
}

fn parse_number(number: &str, class: &str, offset: usize) -> Result<u16, StyleError> {
    if number.is_empty() || !number.bytes().all(|byte| byte.is_ascii_digit()) {
        return Err(error(
            class,
            offset,
            "dimension value must be a non-negative integer",
        ));
    }
    number
        .parse::<u16>()
        .map_err(|_| error(class, offset, "dimension value does not fit in u16"))
}
