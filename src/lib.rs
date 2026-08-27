//! Tailwind-inspired styling utilities for Bevy UI.

use bevy::scene::{ResolveContext, ResolvedScene, Scene, SceneFunction};
use bevy::ui::{Node, percent, px, vh, vw};
use bevywind_core::{Property, Value, parse_class};

pub use bevywind_macros::bstyle;

/// Parses a dynamic style class string at runtime and returns a scene patch.
///
/// The input is copied into the returned scene, so borrowed values such as a
/// local `&String` or `&str` can be passed safely.
pub fn style_runtime<S: AsRef<str>>(classes: S) -> impl Scene {
    let classes = classes.as_ref().to_owned();

    SceneFunction(
        move |context: &mut ResolveContext, scene: &mut ResolvedScene| {
            let node = scene.get_or_insert_template::<Node>(context);

            for class in classes.split_whitespace() {
                let Ok(rule) = parse_class(class, 0) else {
                    continue;
                };

                let value = match rule.value {
                    Value::Pixels(value) => px(value),
                    Value::Percent(value) => percent(value),
                    Value::ViewportWidth(value) => vw(value),
                    Value::ViewportHeight(value) => vh(value),
                };

                match rule.property {
                    Property::Height => node.height = value,
                    Property::Width => node.width = value,
                    Property::MinHeight => node.min_height = value,
                    Property::MinWidth => node.min_width = value,
                    Property::MaxHeight => node.max_height = value,
                    Property::MaxWidth => node.max_width = value,
                }
            }
        },
    )
}
