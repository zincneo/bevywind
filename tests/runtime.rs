use bevy::{
    app::{App, TaskPoolPlugin},
    asset::{AssetApp, AssetPlugin, Assets},
    image::Image,
    scene::{ScenePatch, WorldSceneExt},
};
use bevywind::{bstyle, bstyle_r};

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
        "w_full flex_row h_10px ml_20px p_30per bg_red_500 absolute top_n_10px",
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
fn parses_runtime_overflow_styles() {
    accepts_scene(bstyle_r("overflow_x_clip overflow_y_scroll"));
}

#[test]
fn parses_runtime_flex_item_and_gap_styles() {
    accepts_scene(bstyle_r(
        "grow_2 shrink_0 basis_50per self_center gap_x_8px gap_y_12px",
    ));
}

#[test]
fn parses_runtime_background_image_styles() {
    accepts_scene(bstyle_r(
        r#"w_320px h_200px bgi_url("images/panel.png") bgi_repeat_x bgi_flip_y"#,
    ));
}

#[test]
fn parses_runtime_ui_transform_styles() {
    accepts_scene(bstyle_r("tr_x_10px tr_y_n_5px sc_110per rt_3deg"));
}

#[test]
fn parses_runtime_z_index_styles() {
    accepts_scene(bstyle_r("z_n_10 gz_100"));
}

#[test]
fn resolves_runtime_ui_transform_scene() {
    let mut app = App::new();
    app.add_plugins(AssetPlugin::default());
    app.init_resource::<Assets<ScenePatch>>();
    app.world_mut()
        .spawn_scene(bstyle_r("tr_10px sc_110per rt_n_3deg"))
        .unwrap();
}

#[test]
fn resolves_runtime_background_image_scene() {
    let mut app = App::new();
    app.add_plugins(TaskPoolPlugin::default());
    app.add_plugins(AssetPlugin::default());
    app.init_asset::<Image>();
    app.init_resource::<Assets<ScenePatch>>();
    app.world_mut()
        .spawn_scene(bstyle_r(r#"bgi_url("images/panel.png") bgi_stretch"#))
        .unwrap();
}

#[test]
fn resolves_compile_time_background_image_scene() {
    let mut app = App::new();
    app.add_plugins(TaskPoolPlugin::default());
    app.add_plugins(AssetPlugin::default());
    app.init_asset::<Image>();
    app.init_resource::<Assets<ScenePatch>>();
    app.world_mut()
        .spawn_scene(bstyle!(bgi_url("images/panel.png") bgi_stretch))
        .unwrap();
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
