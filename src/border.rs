use bevy::ui::Node;
use bevywind_core::{Property, Value};

pub(crate) fn apply(node: &mut Node, property: Property, value: Value) {
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
