use bevy::ui::Node;
use bevywind_core::{Property, Value};

pub(crate) fn apply(node: &mut Node, property: Property, value: Value) {
    match property {
        Property::PositionType => {
            node.position_type = match value {
                Value::PositionRelative => bevy::ui::PositionType::Relative,
                Value::PositionAbsolute => bevy::ui::PositionType::Absolute,
                _ => return,
            }
        }
        Property::Left | Property::Right | Property::Top | Property::Bottom => {
            let Some(value) = crate::units::to_val(value) else {
                return;
            };
            match property {
                Property::Left => node.left = value,
                Property::Right => node.right = value,
                Property::Top => node.top = value,
                Property::Bottom => node.bottom = value,
                _ => unreachable!(),
            }
        }
        _ => {}
    }
}
