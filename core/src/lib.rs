//! Shared Bevywind style parsing and utility metadata.

use std::collections::HashSet;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Property {
    Height,
    Width,
    MinHeight,
    MinWidth,
    MaxHeight,
    MaxWidth,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Value {
    Pixels(u16),
    Percent(u16),
    ViewportWidth(u16),
    ViewportHeight(u16),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct StyleRule {
    pub property: Property,
    pub value: Value,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StyleError {
    pub offset: usize,
    pub length: usize,
    pub message: String,
}

impl std::fmt::Display for StyleError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&self.message)
    }
}

impl std::error::Error for StyleError {}

/// Parses a whitespace-separated style class string.
pub fn parse_classes(input: &str) -> Result<Vec<StyleRule>, StyleError> {
    let mut rules = Vec::new();
    let mut properties = HashSet::new();

    let mut search_from = 0;
    for class in input.split_whitespace() {
        let offset = search_from + input[search_from..].find(class).unwrap_or(0);
        search_from = offset + class.len();
        let rule = parse_class(class, offset)?;
        if !properties.insert(rule.property) {
            return Err(StyleError {
                offset,
                length: class.len(),
                message: format!("duplicate style property in `{class}`"),
            });
        }
        rules.push(rule);
    }

    Ok(rules)
}

pub fn parse_class(class: &str, offset: usize) -> Result<StyleRule, StyleError> {
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
        return Err(error(class, offset, "unknown dimension utility"));
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
        return Err(error(
            class,
            offset,
            "expected `full`, a `px` value, a `%` value, a `w` value, or an `h` value",
        ));
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

fn error(class: &str, offset: usize, message: &str) -> StyleError {
    StyleError {
        offset,
        length: class.len(),
        message: format!("{message}: `{class}`"),
    }
}

pub fn completion_items() -> &'static [&'static str] {
    &[
        "h-full",
        "w-full",
        "h-100px",
        "w-100px",
        "h-100%",
        "w-100%",
        "h-100w",
        "w-100w",
        "h-100h",
        "w-100h",
        "min-h-100px",
        "min-w-100px",
        "max-h-100px",
        "max-w-100px",
    ]
}
