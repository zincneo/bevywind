use crate::{Property, StyleError, StyleRule, Value, error};

pub(crate) fn parse(class: &str, offset: usize) -> Result<StyleRule, StyleError> {
    if class == "b_r"
        || class.starts_with("b_r_")
        || class.starts_with("btl_r_")
        || class.starts_with("btr_r_")
        || class.starts_with("bbl_r_")
        || class.starts_with("bbr_r_")
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

    if let Ok((red, green, blue, alpha)) = crate::parse_color(prefix, class, offset) {
        return Ok(StyleRule {
            property: color_property,
            value: Value::BorderColor(red, green, blue, alpha),
        });
    }

    let value = crate::units::parse(prefix, class, offset)?;
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
    if class.starts_with("btl_r_") {
        return expand_radius(class, offset, &[Property::BorderRadiusTopLeft]);
    }
    if class.starts_with("btr_r_") {
        return expand_radius(class, offset, &[Property::BorderRadiusTopRight]);
    }
    if class.starts_with("bbl_r_") {
        return expand_radius(class, offset, &[Property::BorderRadiusBottomLeft]);
    }
    if class.starts_with("bbr_r_") {
        return expand_radius(class, offset, &[Property::BorderRadiusBottomRight]);
    }
    let (prefix, properties) = if class.starts_with("b_") {
        if crate::parse_color(&class[2..], class, offset).is_ok() {
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
    let (prefix, property) = if class.starts_with("btl_r_") {
        ("btl_r_", Property::BorderRadiusTopLeft)
    } else if class.starts_with("btr_r_") {
        ("btr_r_", Property::BorderRadiusTopRight)
    } else if class.starts_with("bbl_r_") {
        ("bbl_r_", Property::BorderRadiusBottomLeft)
    } else if class.starts_with("bbr_r_") {
        ("bbr_r_", Property::BorderRadiusBottomRight)
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
    let value = match value {
        "sm" => Value::RadiusPixels(2),
        "" => Value::RadiusPixels(4),
        "md" => Value::RadiusPixels(6),
        "lg" => Value::RadiusPixels(8),
        "xl" => Value::RadiusPixels(12),
        "2xl" => Value::RadiusPixels(16),
        "3xl" => Value::RadiusPixels(24),
        _ => return map_radius_value(crate::units::parse(value, class, offset)?, class, offset),
    };
    Ok(value)
}

fn map_radius_value(value: Value, class: &str, offset: usize) -> Result<Value, StyleError> {
    Ok(match value {
        Value::Pixels(value) => Value::RadiusPixels(value),
        Value::Percent(value) => Value::RadiusPercent(value),
        Value::ViewportWidth(value) => Value::RadiusViewportWidth(value),
        Value::ViewportHeight(value) => Value::RadiusViewportHeight(value),
        _ => return Err(error(class, offset, "invalid border radius value")),
    })
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
        6
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
