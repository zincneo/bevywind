use crate::{Property, StyleError, StyleRule, Value};

pub(crate) fn parse(class: &str, offset: usize) -> Option<Result<StyleRule, StyleError>> {
    if let Some(value) = class.strip_prefix("tr_x_") {
        return Some(
            parse_translation(value, class, offset).map(|value| rule(Property::TransformX, value)),
        );
    }
    if let Some(value) = class.strip_prefix("tr_y_") {
        return Some(
            parse_translation(value, class, offset).map(|value| rule(Property::TransformY, value)),
        );
    }
    if let Some(value) = class.strip_prefix("sc_x_") {
        return Some(parse_scale(value, class, offset).map(|value| rule(Property::ScaleX, value)));
    }
    if let Some(value) = class.strip_prefix("sc_y_") {
        return Some(parse_scale(value, class, offset).map(|value| rule(Property::ScaleY, value)));
    }
    if let Some(value) = class.strip_prefix("sc_") {
        return Some(parse_scale(value, class, offset).map(|value| rule(Property::Scale, value)));
    }
    if let Some(value) = class.strip_prefix("rt_") {
        return Some(
            parse_rotation(value, class, offset).map(|value| rule(Property::Rotation, value)),
        );
    }
    if let Some(value) = class.strip_prefix("tr_") {
        return Some(
            parse_translation(value, class, offset).map(|value| rule(Property::Transform, value)),
        );
    }
    None
}

pub(crate) fn expansion(class: &str, offset: usize) -> Option<Result<Vec<StyleRule>, StyleError>> {
    if let Some(value) = class.strip_prefix("tr_") {
        if value.starts_with("x_") || value.starts_with("y_") {
            return None;
        }
        let value = match parse_translation(value, class, offset) {
            Ok(value) => value,
            Err(error) => return Some(Err(error)),
        };
        return Some(Ok(vec![
            rule(Property::TransformX, value.clone()),
            rule(Property::TransformY, value),
        ]));
    }

    let value = class.strip_prefix("sc_")?;
    if value.starts_with("x_") || value.starts_with("y_") {
        return None;
    }
    let value = match parse_scale(value, class, offset) {
        Ok(value) => value,
        Err(error) => return Some(Err(error)),
    };
    Some(Ok(vec![
        rule(Property::ScaleX, value.clone()),
        rule(Property::ScaleY, value),
    ]))
}

fn parse_translation(value: &str, class: &str, offset: usize) -> Result<Value, StyleError> {
    if let Some(value) = value.strip_prefix("n_") {
        return crate::units::parse(value, class, offset).map(negative);
    }
    crate::units::parse(value, class, offset)
}

fn parse_scale(value: &str, class: &str, offset: usize) -> Result<Value, StyleError> {
    if !value.ends_with("per") {
        return Err(crate::error(
            class,
            offset,
            "scale value must use the per unit",
        ));
    }
    let value = crate::units::parse(value, class, offset)?;
    if matches!(value, Value::Percent(_)) {
        Ok(value)
    } else {
        Err(crate::error(
            class,
            offset,
            "scale value must use the per unit",
        ))
    }
}

fn parse_rotation(value: &str, class: &str, offset: usize) -> Result<Value, StyleError> {
    let (negative, value) = value
        .strip_prefix("n_")
        .map_or((false, value), |value| (true, value));
    let Some(value) = value.strip_suffix("deg") else {
        return Err(crate::error(
            class,
            offset,
            "rotation value must use the deg unit",
        ));
    };
    let value = crate::units::parse_number(value, class, offset, "rotation")?;
    if negative {
        Ok(Value::NegativeRotation(value))
    } else {
        Ok(Value::Rotation(value))
    }
}

fn negative(value: Value) -> Value {
    match value {
        Value::Pixels(value) => Value::NegativePixels(value),
        Value::Percent(value) => Value::NegativePercent(value),
        Value::ViewportWidth(value) => Value::NegativeViewportWidth(value),
        Value::ViewportHeight(value) => Value::NegativeViewportHeight(value),
        _ => unreachable!("translation parser returned an unsupported value"),
    }
}

fn rule(property: Property, value: Value) -> StyleRule {
    StyleRule { property, value }
}
