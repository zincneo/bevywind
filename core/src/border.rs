use crate::{Property, StyleError, StyleRule, Value, error};

pub(crate) fn parse(class: &str, offset: usize) -> Result<StyleRule, StyleError> {
    if class == "b_r"
        || class.starts_with("b_r_")
        || class.starts_with("bl_r_")
        || class.starts_with("br_r_")
        || class.starts_with("bt_r_")
        || class.starts_with("bb_r_")
    {
        return parse_radius(class, offset);
    }
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
    if class == "b_r" || class.starts_with("b_r_") {
        return expand_radius(
            class,
            offset,
            &[
                Property::BorderRadiusTopLeft,
                Property::BorderRadiusTopRight,
                Property::BorderRadiusBottomRight,
                Property::BorderRadiusBottomLeft,
            ],
        );
    }
    if class.starts_with("bl_r_") {
        return expand_radius(
            class,
            offset,
            &[
                Property::BorderRadiusTopLeft,
                Property::BorderRadiusBottomLeft,
            ],
        );
    }
    if class.starts_with("br_r_") {
        return expand_radius(
            class,
            offset,
            &[
                Property::BorderRadiusTopRight,
                Property::BorderRadiusBottomRight,
            ],
        );
    }
    if class.starts_with("bt_r_") {
        return expand_radius(
            class,
            offset,
            &[
                Property::BorderRadiusTopLeft,
                Property::BorderRadiusTopRight,
            ],
        );
    }
    if class.starts_with("bb_r_") {
        return expand_radius(
            class,
            offset,
            &[
                Property::BorderRadiusBottomLeft,
                Property::BorderRadiusBottomRight,
            ],
        );
    }
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

fn parse_radius(class: &str, offset: usize) -> Result<StyleRule, StyleError> {
    let (prefix, property) = if class.starts_with("bl_r_") {
        ("bl_r_", Property::BorderRadiusTopLeft)
    } else if class.starts_with("br_r_") {
        ("br_r_", Property::BorderRadiusTopRight)
    } else if class.starts_with("bt_r_") {
        ("bt_r_", Property::BorderRadiusTopLeft)
    } else if class.starts_with("bb_r_") {
        ("bb_r_", Property::BorderRadiusBottomLeft)
    } else if class == "b_r" {
        ("b_r", Property::BorderRadiusTopLeft)
    } else {
        ("b_r_", Property::BorderRadiusTopLeft)
    };
    Ok(StyleRule {
        property,
        value: parse_radius_value(&class[prefix.len()..], class, offset)?,
    })
}

fn parse_radius_value(value: &str, class: &str, offset: usize) -> Result<Value, StyleError> {
    if value == "none" {
        return Ok(Value::RadiusPixels(0));
    }
    if value == "full" {
        return Ok(Value::RadiusFull);
    }
    let (number, value): (&str, fn(u16) -> Value) = if let Some(number) = value.strip_suffix("px") {
        (number, Value::RadiusPixels)
    } else if let Some(number) = value.strip_suffix("per") {
        (number, Value::RadiusPercent)
    } else if let Some(number) = value.strip_suffix('w') {
        (number, Value::RadiusViewportWidth)
    } else if let Some(number) = value.strip_suffix('h') {
        (number, Value::RadiusViewportHeight)
    } else {
        return match value {
            "sm" => Ok(Value::RadiusPixels(2)),
            "" => Ok(Value::RadiusPixels(4)),
            "md" => Ok(Value::RadiusPixels(6)),
            "lg" => Ok(Value::RadiusPixels(8)),
            "xl" => Ok(Value::RadiusPixels(12)),
            "2xl" => Ok(Value::RadiusPixels(16)),
            "3xl" => Ok(Value::RadiusPixels(24)),
            _ => Err(error(class, offset, "invalid border radius value")),
        };
    };
    if number.is_empty() || !number.bytes().all(|byte| byte.is_ascii_digit()) {
        return Err(error(
            class,
            offset,
            "border radius value must be a non-negative integer",
        ));
    }
    let number = number
        .parse()
        .map_err(|_| error(class, offset, "border radius value does not fit in u16"))?;
    Ok(value(number))
}

fn expand_radius(
    class: &str,
    offset: usize,
    properties: &[Property],
) -> Option<Result<Vec<StyleRule>, StyleError>> {
    let prefix_len = if class == "b_r" {
        3
    } else if class.starts_with("b_r_") {
        4
    } else {
        5
    };
    let value = match parse_radius_value(&class[prefix_len..], class, offset) {
        Ok(value) => value,
        Err(error) => return Some(Err(error)),
    };
    Some(Ok(properties
        .iter()
        .copied()
        .map(|property| StyleRule { property, value })
        .collect()))
}
