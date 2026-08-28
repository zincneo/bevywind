use bevywind::bstyle;

mod common;
use common::accepts_scene;

#[test]
fn supports_full_and_pixel_dimensions() {
    accepts_scene(bstyle!(h_full));
    accepts_scene(bstyle!(w_full));
    accepts_scene(bstyle!(h_10px));
    accepts_scene(bstyle!(w_20px));
}

#[test]
fn supports_percent_dimensions() {
    accepts_scene(bstyle!(h_0per));
    accepts_scene(bstyle!(w_100per));
    accepts_scene(bstyle!(h_37per w_63per));
}

#[test]
fn supports_viewport_dimensions() {
    accepts_scene(bstyle!(h_10w));
    accepts_scene(bstyle!(w_20w));
    accepts_scene(bstyle!(h_30h));
    accepts_scene(bstyle!(w_40h));
}

#[test]
fn supports_min_and_max_dimensions() {
    accepts_scene(bstyle!(min_h_10px));
    accepts_scene(bstyle!(min_w_20per));
    accepts_scene(bstyle!(max_h_30w));
    accepts_scene(bstyle!(max_w_40h));
}
