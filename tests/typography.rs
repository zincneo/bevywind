use bevy::{
    app::App,
    asset::{AssetPlugin, Assets},
    scene::{ScenePatch, WorldSceneExt},
};
use bevywind::bstyle;
use bevywind_core::{Property, Value, parse_classes};

mod common;
use common::accepts_scene;

#[test]
fn supports_text_colors() {
    accepts_scene(bstyle!(t_transparent));
    accepts_scene(bstyle!(t_black));
    accepts_scene(bstyle!(t_white));
    accepts_scene(bstyle!(t_blue_500));
    accepts_scene(bstyle!(t_11223380));
}

#[test]
fn supports_font_sizes() {
    accepts_scene(bstyle!(t_xs));
    accepts_scene(bstyle!(t_10px));
}

#[test]
fn supports_text_layout_styles() {
    accepts_scene(bstyle!(t_center t_leading_relaxed t_whitespace_normal));
    accepts_scene(bstyle!(t_break_words));
}

#[test]
fn supports_font_effects() {
    accepts_scene(bstyle!(t_bold t_italic));
    accepts_scene(bstyle!(t_semibold t_not_italic));
    accepts_scene(bstyle!(t_w_black));
}

#[test]
fn parses_text_rules_and_values() {
    assert_eq!(
        parse_classes("t_red_500 t_24px t_bold t_italic").unwrap(),
        vec![
            bevywind_core::StyleRule {
                property: Property::TextColor,
                value: Value::TextColor(239, 68, 68, 255),
            },
            bevywind_core::StyleRule {
                property: Property::FontSize,
                value: Value::FontSize(24),
            },
            bevywind_core::StyleRule {
                property: Property::FontWeight,
                value: Value::FontWeight(700),
            },
            bevywind_core::StyleRule {
                property: Property::FontStyle,
                value: Value::FontStyleItalic,
            },
        ]
    );
}

#[test]
fn rejects_duplicate_text_properties() {
    assert!(parse_classes("t_red t_blue").is_err());
    assert!(parse_classes("t_bold t_italic t_medium").is_err());
    assert!(parse_classes("t_center t_right").is_err());
}

#[test]
fn macro_and_runtime_produce_the_same_scene_components() {
    let classes = "w_320px flex_col p_10px bg_red_500 b_2px b_blue_400 b_r_lg \
        t_blue_500 t_20px t_bold t_italic t_center t_leading_relaxed t_whitespace_normal";

    let mut app = App::new();
    app.add_plugins(AssetPlugin::default());
    app.init_resource::<Assets<ScenePatch>>();
    let macro_entity = app
        .world_mut()
        .spawn_scene(bstyle!(
        w_320px flex_col p_10px bg_red_500 b_2px b_blue_400 b_r_lg
            t_blue_500 t_20px t_bold t_italic t_center t_leading_relaxed t_whitespace_normal
        ))
        .unwrap()
        .id();

    let runtime_entity = app
        .world_mut()
        .spawn_scene(bevywind::style_runtime(classes))
        .unwrap()
        .id();

    let world = app.world();
    assert_eq!(
        world.get::<bevy::ui::Node>(macro_entity),
        world.get::<bevy::ui::Node>(runtime_entity)
    );
    assert_eq!(
        world.get::<bevy::ui::BackgroundColor>(macro_entity),
        world.get::<bevy::ui::BackgroundColor>(runtime_entity)
    );
    assert_eq!(
        world.get::<bevy::ui::BorderColor>(macro_entity),
        world.get::<bevy::ui::BorderColor>(runtime_entity)
    );
    assert_eq!(
        world.get::<bevy::text::TextColor>(macro_entity),
        world.get::<bevy::text::TextColor>(runtime_entity)
    );
    assert_eq!(
        world.get::<bevy::text::TextFont>(macro_entity),
        world.get::<bevy::text::TextFont>(runtime_entity)
    );
    assert_eq!(
        world
            .get::<bevy::text::TextLayout>(macro_entity)
            .map(|layout| (layout.justify, layout.linebreak)),
        world
            .get::<bevy::text::TextLayout>(runtime_entity)
            .map(|layout| (layout.justify, layout.linebreak))
    );
    assert_eq!(
        world.get::<bevy::text::LineHeight>(macro_entity),
        world.get::<bevy::text::LineHeight>(runtime_entity)
    );
}
