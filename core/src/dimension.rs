use crate::{Property, StyleError, StyleRule, error};

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

    let value = crate::units::parse(value, class, offset)?;

    Ok(StyleRule { property, value })
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

    let value = match crate::units::parse(&class[prefix.len()..], class, offset) {
        Ok(value) => value,
        Err(error) => return Some(Err(error)),
    };
    Some(Ok(properties
        .iter()
        .copied()
        .map(|property| StyleRule { property, value })
        .collect()))
}
