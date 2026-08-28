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
