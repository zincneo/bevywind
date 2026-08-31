use bevy::ui::Node;
use bevywind_core::{Property, Value};

pub(crate) fn apply(node: &mut Node, property: Property, value: Value) {
    let value = match value {
        Value::OverflowVisible => bevy::ui::OverflowAxis::Visible,
        Value::OverflowClip => bevy::ui::OverflowAxis::Clip,
        Value::OverflowHidden => bevy::ui::OverflowAxis::Clip,
        Value::OverflowScroll => bevy::ui::OverflowAxis::Scroll,
        _ => return,
    };
    match property {
        Property::OverflowX => node.overflow.x = value,
        Property::OverflowY => node.overflow.y = value,
        _ => {}
    }
}
