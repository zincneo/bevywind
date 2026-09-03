use bevy::scene::{ResolveContext, ResolvedScene};
use bevy::ui::{GlobalZIndex, ZIndex};
use bevywind_core::{Property, Value};

pub(crate) fn apply(
    scene: &mut ResolvedScene,
    context: &mut ResolveContext,
    property: Property,
    value: Value,
) {
    match (property, value) {
        (Property::ZIndex, Value::ZIndex(value)) => {
            scene.get_or_insert_template::<ZIndex>(context).0 = value;
        }
        (Property::GlobalZIndex, Value::GlobalZIndex(value)) => {
            scene.get_or_insert_template::<GlobalZIndex>(context).0 = value;
        }
        _ => {}
    }
}
