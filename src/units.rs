use bevy::ui::{Val, percent, px, vh, vw};
use bevywind_core::Value;

pub(crate) fn to_val(value: Value) -> Option<Val> {
    match value {
        Value::Pixels(value) | Value::RadiusPixels(value) => Some(px(value)),
        Value::Percent(value) | Value::RadiusPercent(value) => Some(percent(value)),
        Value::ViewportWidth(value) | Value::RadiusViewportWidth(value) => Some(vw(value)),
        Value::ViewportHeight(value) | Value::RadiusViewportHeight(value) => Some(vh(value)),
        Value::NegativePixels(value) => Some(px(-(value as f32))),
        Value::NegativePercent(value) => Some(percent(-(value as f32))),
        Value::NegativeViewportWidth(value) => Some(vw(-(value as f32))),
        Value::NegativeViewportHeight(value) => Some(vh(-(value as f32))),
        _ => None,
    }
}
