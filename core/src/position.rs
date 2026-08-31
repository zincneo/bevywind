use crate::{Property, StyleError, StyleRule, Value};

pub(crate) fn parse(class: &str, offset: usize) -> Option<Result<StyleRule, StyleError>> {
    let (property, value) = match class {
        "relative" => return Some(Ok(rule(Property::PositionType, Value::PositionRelative))),
        "absolute" => return Some(Ok(rule(Property::PositionType, Value::PositionAbsolute))),
        _ => {
            let prefixes = [
                ("left_", Property::Left),
                ("right_", Property::Right),
                ("top_", Property::Top),
                ("bottom_", Property::Bottom),
            ];
            let Some((prefix, property)) = prefixes
                .iter()
                .find(|(prefix, _)| class.starts_with(prefix))
            else {
                return None;
            };
            (*property, &class[prefix.len()..])
        }
    };

    let value = if let Some(value) = value.strip_prefix("n_") {
        crate::units::parse(value, class, offset).map(negative_value)
    } else {
        crate::units::parse(value, class, offset)
    };
    Some(value.map(|value| rule(property, value)))
}

fn negative_value(value: Value) -> Value {
    match value {
        Value::Pixels(value) => Value::NegativePixels(value),
        Value::Percent(value) => Value::NegativePercent(value),
        Value::ViewportWidth(value) => Value::NegativeViewportWidth(value),
        Value::ViewportHeight(value) => Value::NegativeViewportHeight(value),
        _ => unreachable!("dimension parser returned an unsupported position value"),
    }
}

fn rule(property: Property, value: Value) -> StyleRule {
    StyleRule { property, value }
}
