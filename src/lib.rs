//! Tailwind-inspired styling utilities for Bevy UI.

use bevy::scene::{ResolveContext, ResolvedScene, Scene, SceneFunction};
use bevy::ui::{Node, Val, percent, px, vh, vw};

pub use bevywind_macros::style;

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
                let (field, value) = if let Some(value) = class.strip_prefix("max-h-") {
                    ("max_height", value)
                } else if let Some(value) = class.strip_prefix("max-w-") {
                    ("max_width", value)
                } else if let Some(value) = class.strip_prefix("min-h-") {
                    ("min_height", value)
                } else if let Some(value) = class.strip_prefix("min-w-") {
                    ("min_width", value)
                } else if let Some(value) = class.strip_prefix("h-") {
                    ("height", value)
                } else if let Some(value) = class.strip_prefix("w-") {
                    ("width", value)
                } else {
                    continue;
                };

                let value: Option<Val> = match value {
                    "full" => Some(percent(100u16)),
                    value if value.ends_with("px") => value
                        .strip_suffix("px")
                        .and_then(|value| value.parse::<u16>().ok())
                        .map(px),
                    value if value.ends_with('%') => value
                        .strip_suffix('%')
                        .and_then(|value| value.parse::<u16>().ok())
                        .map(percent),
                    value if value.ends_with('w') => value
                        .strip_suffix('w')
                        .and_then(|value| value.parse::<u16>().ok())
                        .map(vw),
                    value if value.ends_with('h') => value
                        .strip_suffix('h')
                        .and_then(|value| value.parse::<u16>().ok())
                        .map(vh),
                    _ => None,
                };

                match (field, value) {
                    ("height", Some(value)) => node.height = value,
                    ("width", Some(value)) => node.width = value,
                    ("min_height", Some(value)) => node.min_height = value,
                    ("min_width", Some(value)) => node.min_width = value,
                    ("max_height", Some(value)) => node.max_height = value,
                    ("max_width", Some(value)) => node.max_width = value,
                    _ => {}
                }
            }
        },
    )
}
