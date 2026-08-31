use bevy::scene::{ResolveContext, ResolvedScene};
use bevy::ui::widget::{ImageNodeTemplate, NodeImageMode};
use bevywind_core::{Property, Value};

pub(crate) fn apply(
    scene: &mut ResolvedScene,
    context: &mut ResolveContext,
    property: Property,
    value: Value,
) {
    let image = scene.get_or_insert_template::<ImageNodeTemplate>(context);
    match (property, value) {
        (Property::Image, Value::ImageUrl(path)) => image.image = path.into(),
        (Property::ImageMode, Value::ImageModeAuto | Value::ImageModeNoRepeat) => {
            image.image_mode = NodeImageMode::Auto;
        }
        (Property::ImageMode, Value::ImageModeStretch) => {
            image.image_mode = NodeImageMode::Stretch;
        }
        (Property::ImageMode, Value::ImageModeRepeat) => {
            image.image_mode = tiled(true, true);
        }
        (Property::ImageMode, Value::ImageModeRepeatX) => {
            image.image_mode = tiled(true, false);
        }
        (Property::ImageMode, Value::ImageModeRepeatY) => {
            image.image_mode = tiled(false, true);
        }
        (Property::ImageFlipX, Value::ImageFlipX) => image.flip_x = true,
        (Property::ImageFlipY, Value::ImageFlipY) => image.flip_y = true,
        _ => {}
    }
}

fn tiled(tile_x: bool, tile_y: bool) -> NodeImageMode {
    NodeImageMode::Tiled {
        tile_x,
        tile_y,
        stretch_value: 1.0,
    }
}
