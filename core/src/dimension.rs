use crate::{Property, StyleError, StyleRule, Value, error};

pub(crate) fn parse(class: &str, offset: usize) -> Result<StyleRule, StyleError> {
    let (property, value) = if let Some(value) = class.strip_prefix("max_h_") {
        (Property::MaxHeight, value)
    } else if let Some(value) = class.strip_prefix("max_w_") {
        (Property::MaxWidth, value)
    } else if let Some(value) = class.strip_prefix("min_h_") {
        (Property::MinHeight, value)
    } else if let Some(value) = class.strip_prefix("min_w_") {
        (Property::MinWidth, value)
    } else if let Some(value) = class.strip_prefix("h_") {
        (Property::Height, value)
    } else if let Some(value) = class.strip_prefix("w_") {
        (Property::Width, value)
    } else if let Some(value) = class.strip_prefix("ml_") {
        (Property::MarginLeft, value)
    } else if let Some(value) = class.strip_prefix("mr_") {
        (Property::MarginRight, value)
    } else if let Some(value) = class.strip_prefix("mt_") {
        (Property::MarginTop, value)
    } else if let Some(value) = class.strip_prefix("mb_") {
        (Property::MarginBottom, value)
    } else if let Some(value) = class.strip_prefix("pl_") {
        (Property::PaddingLeft, value)
    } else if let Some(value) = class.strip_prefix("pr_") {
        (Property::PaddingRight, value)
    } else if let Some(value) = class.strip_prefix("pt_") {
        (Property::PaddingTop, value)
    } else if let Some(value) = class.strip_prefix("pb_") {
        (Property::PaddingBottom, value)
    } else {
        return Err(error(class, offset, "unknown style utility"));
    };

    let value = parse_value(value, class, offset)?;

    Ok(StyleRule { property, value })
}

fn parse_value(value: &str, class: &str, offset: usize) -> Result<Value, StyleError> {
    let value = if value == "full" {
        Value::Percent(100)
    } else if let Some(number) = value.strip_suffix("px") {
        Value::Pixels(parse_number(number, class, offset)?)
    } else if let Some(number) = value.strip_suffix("per") {
        Value::Percent(parse_number(number, class, offset)?)
    } else if let Some(number) = value.strip_suffix('w') {
        Value::ViewportWidth(parse_number(number, class, offset)?)
    } else if let Some(number) = value.strip_suffix('h') {
        Value::ViewportHeight(parse_number(number, class, offset)?)
    } else {
        return Err(error(class, offset, "invalid dimension value"));
    };

    Ok(value)
}

pub(crate) fn expansion(class: &str, offset: usize) -> Option<Result<Vec<StyleRule>, StyleError>> {
    let (prefix, properties): (&str, &[Property]) = if class.starts_with("m_") {
        (
            "m_",
            &[
                Property::MarginLeft,
                Property::MarginRight,
                Property::MarginTop,
                Property::MarginBottom,
            ],
        )
    } else if class.starts_with("p_") {
        (
            "p_",
            &[
                Property::PaddingLeft,
                Property::PaddingRight,
                Property::PaddingTop,
                Property::PaddingBottom,
            ],
        )
    } else {
        return None;
    };

    let value = match parse_value(&class[prefix.len()..], class, offset) {
        Ok(value) => value,
        Err(error) => return Some(Err(error)),
    };
    Some(Ok(properties
        .iter()
        .copied()
        .map(|property| StyleRule { property, value })
        .collect()))
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
