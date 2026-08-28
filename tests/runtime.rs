use bevywind::style_runtime;

mod common;
use common::accepts_scene;

#[test]
fn parses_runtime_dimensions_and_flex_styles() {
    let classes = String::from("h_10px w_20per flex_row items_center");
    accepts_scene(style_runtime(&classes));
}

#[test]
fn parses_runtime_background_styles() {
    accepts_scene(style_runtime("bg_red_500"));
    accepts_scene(style_runtime("bg_11223380"));
}

#[test]
fn parses_runtime_spacing_styles() {
    let classes = String::from("m_10px mt_20per p_30w pb_40h");
    accepts_scene(style_runtime(&classes));
}

#[test]
fn combines_all_runtime_node_styles() {
    accepts_scene(style_runtime(
        "w_full flex_row h_10px ml_20px p_30per bg_red_500",
    ));
}
