use bevy::ui::Node;
use bevywind_core::{Property, Value};

pub(crate) fn apply(node: &mut Node, property: Property, value: Value) {
    if matches!(
        property,
        Property::BorderRadiusTopLeft
            | Property::BorderRadiusTopRight
            | Property::BorderRadiusBottomRight
            | Property::BorderRadiusBottomLeft
    ) {
        let radius = match value {
            Value::RadiusFull => bevy::ui::CornerRadius::MAX,
            _ => {
                let Some(value) = crate::dimension::to_val(value) else {
                    return;
                };
                bevy::ui::CornerRadius::circular(value)
            }
        };
        match property {
            Property::BorderRadiusTopLeft => node.border_radius.top_left = radius,
            Property::BorderRadiusTopRight => node.border_radius.top_right = radius,
            Property::BorderRadiusBottomRight => node.border_radius.bottom_right = radius,
            Property::BorderRadiusBottomLeft => node.border_radius.bottom_left = radius,
            _ => unreachable!(),
        }
        return;
    }
    let Some(value) = crate::dimension::to_val(value) else {
        return;
    };
    match property {
        Property::BorderLeft => node.border.left = value,
        Property::BorderRight => node.border.right = value,
        Property::BorderTop => node.border.top = value,
        Property::BorderBottom => node.border.bottom = value,
        _ => {}
    }
}
