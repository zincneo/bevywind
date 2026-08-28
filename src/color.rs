use bevy::color::Color;
use bevy::scene::{ResolveContext, ResolvedScene};
use bevy::ui::BackgroundColor;
use bevywind_core::Value;

pub(crate) fn apply(scene: &mut ResolvedScene, context: &mut ResolveContext, value: Value) {
    let Value::Background(red, green, blue, alpha) = value else {
        return;
    };
    let background = scene.get_or_insert_template::<BackgroundColor>(context);
    background.0 = Color::srgba(
        red as f32 / 255.0,
        green as f32 / 255.0,
        blue as f32 / 255.0,
        alpha as f32 / 255.0,
    );
}
