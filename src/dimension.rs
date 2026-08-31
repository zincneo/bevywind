use bevy::ui::{Node, percent, px, vh, vw};
use bevywind_core::{Property, Value};

pub(crate) fn apply(node: &mut Node, property: Property, value: Value) {
    let Some(value) = to_val(value) else { return };
    match property {
        Property::Height => node.height = value,
        Property::Width => node.width = value,
        Property::MinHeight => node.min_height = value,
        Property::MinWidth => node.min_width = value,
        Property::MaxHeight => node.max_height = value,
        Property::MaxWidth => node.max_width = value,
        Property::MarginLeft => node.margin.left = value,
        Property::MarginRight => node.margin.right = value,
        Property::MarginTop => node.margin.top = value,
        Property::MarginBottom => node.margin.bottom = value,
        Property::PaddingLeft => node.padding.left = value,
        Property::PaddingRight => node.padding.right = value,
        Property::PaddingTop => node.padding.top = value,
        Property::PaddingBottom => node.padding.bottom = value,
        _ => {}
    }
}

pub(crate) fn to_val(value: Value) -> Option<bevy::ui::Val> {
    match value {
        Value::Pixels(value) => Some(px(value)),
        Value::Percent(value) => Some(percent(value)),
        Value::ViewportWidth(value) => Some(vw(value)),
        Value::ViewportHeight(value) => Some(vh(value)),
        _ => None,
    }
}
