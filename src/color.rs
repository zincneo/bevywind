use bevy::scene::{ResolveContext, ResolvedScene};
use bevy::ui::BackgroundColor;
use bevy::ui::BorderColor;
use bevywind_core::Property;
use bevywind_core::Value;

pub(crate) fn apply(
    scene: &mut ResolvedScene,
    context: &mut ResolveContext,
    property: Property,
    value: Value,
) {
    let (red, green, blue, alpha) = match value {
        Value::Background(red, green, blue, alpha) => (red, green, blue, alpha),
        Value::BorderColor(red, green, blue, alpha) => (red, green, blue, alpha),
        _ => return,
    };
    let color = bevy::color::Color::srgba(
        red as f32 / 255.0,
        green as f32 / 255.0,
        blue as f32 / 255.0,
        alpha as f32 / 255.0,
    );
    if property == Property::BackgroundColor {
        let background = scene.get_or_insert_template::<BackgroundColor>(context);
        background.0 = color;
        return;
    }
    let border = scene.get_or_insert_template::<BorderColor>(context);
    match property {
        Property::BorderColorLeft => border.left = color,
        Property::BorderColorRight => border.right = color,
        Property::BorderColorTop => border.top = color,
        Property::BorderColorBottom => border.bottom = color,
        _ => {}
    }
}
