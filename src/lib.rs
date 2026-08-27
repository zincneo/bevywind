//! Tailwind-inspired styling utilities for Bevy UI.

use bevy::scene::{ResolveContext, ResolvedScene, Scene, SceneFunction};
use bevy::ui::Node;
use bevywind_core::Property;

mod dimension;
mod flex;

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

            let Ok(rules) = bevywind_core::parse_classes(&classes) else {
                return;
            };

            for rule in rules {
                match rule.property {
                    Property::Display
                    | Property::FlexDirection
                    | Property::FlexWrap
                    | Property::JustifyContent
                    | Property::AlignItems
                    | Property::AlignContent => flex::apply(node, rule.property, rule.value),
                    _ => dimension::apply(node, rule.property, rule.value),
                }
            }
        },
    )
}
