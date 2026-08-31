//! Tailwind-inspired styling utilities for Bevy UI.

use bevy::scene::{ResolveContext, ResolvedScene, Scene, SceneFunction};
use bevy::ui::Node;
use bevywind_core::Property;

mod border;
mod color;
mod dimension;
mod flex;
mod overflow;
mod position;
mod typography;
mod units;

pub use bevywind_macros::bstyle;

/// Parses a dynamic style class string at runtime and returns a scene patch.
///
/// The input is copied into the returned scene, so borrowed values such as a
/// local `&String` or `&str` can be passed safely.
/// Invalid or conflicting style classes panic when the scene is resolved.
pub fn bstyle_r<S: AsRef<str>>(classes: S) -> impl Scene {
    let classes = classes.as_ref().to_owned();

    SceneFunction(
        move |context: &mut ResolveContext, scene: &mut ResolvedScene| {
            let rules = bevywind_core::parse_classes(&classes)
                .unwrap_or_else(|error| panic!("failed to expand runtime styles: {error}"));

            if rules.is_empty() {
                return;
            }

            scene.get_or_insert_template::<Node>(context);

            let typography_rules: Vec<_> = rules
                .iter()
                .copied()
                .filter(|rule| {
                    matches!(
                        rule.property,
                        Property::TextColor
                            | Property::FontSize
                            | Property::TextJustify
                            | Property::LineHeight
                            | Property::LineBreak
                            | Property::FontWeight
                            | Property::FontStyle
                    )
                })
                .collect();
            typography::apply(scene, &typography_rules);

            for rule in rules {
                match rule.property {
                    Property::BackgroundColor
                    | Property::BorderColorLeft
                    | Property::BorderColorRight
                    | Property::BorderColorTop
                    | Property::BorderColorBottom => {
                        color::apply(scene, context, rule.property, rule.value)
                    }
                    Property::TextColor
                    | Property::FontSize
                    | Property::TextJustify
                    | Property::LineHeight
                    | Property::LineBreak
                    | Property::FontWeight
                    | Property::FontStyle => {}
                    Property::Display
                    | Property::FlexDirection
                    | Property::FlexWrap
                    | Property::JustifyContent
                    | Property::AlignItems
                    | Property::AlignContent
                    | Property::FlexGrow
                    | Property::FlexShrink
                    | Property::FlexBasis
                    | Property::AlignSelf
                    | Property::RowGap
                    | Property::ColumnGap => {
                        let node = scene.get_or_insert_template::<Node>(context);
                        flex::apply(node, rule.property, rule.value);
                    }
                    Property::PositionType
                    | Property::Left
                    | Property::Right
                    | Property::Top
                    | Property::Bottom => {
                        let node = scene.get_or_insert_template::<Node>(context);
                        position::apply(node, rule.property, rule.value);
                    }
                    Property::OverflowX | Property::OverflowY => {
                        let node = scene.get_or_insert_template::<Node>(context);
                        overflow::apply(node, rule.property, rule.value);
                    }
                    Property::BorderLeft
                    | Property::BorderRight
                    | Property::BorderTop
                    | Property::BorderBottom
                    | Property::BorderRadiusTopLeft
                    | Property::BorderRadiusTopRight
                    | Property::BorderRadiusBottomRight
                    | Property::BorderRadiusBottomLeft => {
                        let node = scene.get_or_insert_template::<Node>(context);
                        border::apply(node, rule.property, rule.value);
                    }
                    _ => {
                        let node = scene.get_or_insert_template::<Node>(context);
                        dimension::apply(node, rule.property, rule.value);
                    }
                }
            }
        },
    )
}
