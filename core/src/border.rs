use crate::{Property, StyleError, StyleRule, Value, error};

pub(crate) fn parse(class: &str, offset: usize) -> Result<StyleRule, StyleError> {
    let (prefix, width_property, color_property) = if let Some(value) = class.strip_prefix("bl_") {
        (value, Property::BorderLeft, Property::BorderColorLeft)
    } else if let Some(value) = class.strip_prefix("br_") {
        (value, Property::BorderRight, Property::BorderColorRight)
    } else if let Some(value) = class.strip_prefix("bt_") {
        (value, Property::BorderTop, Property::BorderColorTop)
    } else if let Some(value) = class.strip_prefix("bb_") {
        (value, Property::BorderBottom, Property::BorderColorBottom)
    } else if let Some(value) = class.strip_prefix("b_") {
        (value, Property::BorderLeft, Property::BorderColorLeft)
    } else {
        return Err(error(class, offset, "unknown border utility"));
    };

    if let Ok((red, green, blue, alpha)) = crate::color::parse_value(prefix, class, offset) {
        return Ok(StyleRule {
            property: color_property,
            value: Value::BorderColor(red, green, blue, alpha),
        });
    }

    let value = crate::dimension::parse_value(prefix, class, offset)?;
    Ok(StyleRule {
        property: width_property,
        value,
    })
}

pub(crate) fn expansion(class: &str, offset: usize) -> Option<Result<Vec<StyleRule>, StyleError>> {
    let (prefix, properties) = if class.starts_with("b_") {
        if crate::color::parse_value(&class[2..], class, offset).is_ok() {
            (
                "b_",
                [
                    Property::BorderColorLeft,
                    Property::BorderColorRight,
                    Property::BorderColorTop,
                    Property::BorderColorBottom,
                ],
            )
        } else {
            (
                "b_",
                [
                    Property::BorderLeft,
                    Property::BorderRight,
                    Property::BorderTop,
                    Property::BorderBottom,
                ],
            )
        }
    } else {
        return None;
    };

    let rule = match parse(&format!("bl_{}", &class[prefix.len()..]), offset) {
        Ok(rule) => rule,
        Err(error) => return Some(Err(error)),
    };
    Some(Ok(properties
        .iter()
        .copied()
        .map(|property| StyleRule {
            property,
            value: rule.value,
        })
        .collect()))
}
