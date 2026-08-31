use bevy::math::{Rot2, Vec2};
use bevy::scene::{ResolveContext, ResolvedScene};
use bevy::ui::UiTransform;
use bevywind_core::{Property, Value};

pub(crate) fn apply(
    scene: &mut ResolvedScene,
    context: &mut ResolveContext,
    property: Property,
    value: Value,
) {
    let transform = scene.get_or_insert_template::<UiTransform>(context);
    match property {
        Property::TransformX => {
            if let Some(value) = crate::units::to_val(value) {
                transform.translation.x = value;
            }
        }
        Property::TransformY => {
            if let Some(value) = crate::units::to_val(value) {
                transform.translation.y = value;
            }
        }
        Property::Scale | Property::ScaleX | Property::ScaleY => {
            let Value::Percent(value) = value else { return };
            let value = value as f32 / 100.0;
            match property {
                Property::Scale => transform.scale = Vec2::splat(value),
                Property::ScaleX => transform.scale.x = value,
                Property::ScaleY => transform.scale.y = value,
                _ => unreachable!(),
            }
        }
        Property::Rotation => {
            transform.rotation = match value {
                Value::Rotation(value) => Rot2::degrees(value as f32),
                Value::NegativeRotation(value) => Rot2::degrees(-(value as f32)),
                _ => return,
            };
        }
        _ => {}
    }
}
