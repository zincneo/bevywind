use bevy::{
    app::App,
    asset::{AssetPlugin, Assets},
    scene::{ScenePatch, WorldSceneExt},
};
use bevywind::bstyle_r;

mod common;
use common::accepts_scene;

#[test]
fn parses_runtime_dimensions_and_flex_styles() {
    let classes = String::from("h_10px w_20per flex_row items_center");
    accepts_scene(bstyle_r(&classes));
}

#[test]
fn parses_runtime_background_styles() {
    accepts_scene(bstyle_r("bg_red_500"));
    accepts_scene(bstyle_r("bg_11223380"));
}

#[test]
fn parses_runtime_spacing_styles() {
    let classes = String::from("m_10px mt_20per p_30w pb_40h");
    accepts_scene(bstyle_r(&classes));
}

#[test]
fn combines_all_runtime_node_styles() {
    accepts_scene(bstyle_r(
        "w_full flex_row h_10px ml_20px p_30per bg_red_500",
    ));
}

#[test]
fn parses_runtime_border_styles() {
    accepts_scene(bstyle_r("w_full b_1px bl_11223380 p_10px b_ffffff b_r_lg"));
}

#[test]
fn parses_runtime_typography_styles() {
    accepts_scene(bstyle_r(
        "t_blue_500 t_lg t_bold t_italic t_center t_leading_relaxed t_whitespace_normal",
    ));
}

#[test]
#[should_panic(expected = "failed to expand runtime styles")]
fn panics_when_runtime_styles_fail_to_expand() {
    let mut app = App::new();
    app.add_plugins(AssetPlugin::default());
    app.init_resource::<Assets<ScenePatch>>();
    app.world_mut()
        .spawn_scene(bstyle_r("not_a_style"))
        .unwrap();
}
