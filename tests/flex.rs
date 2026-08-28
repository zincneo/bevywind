use bevywind::bstyle;

mod common;
use common::accepts_scene;

#[test]
fn supports_direction_styles() {
    accepts_scene(bstyle!(flex_row));
    accepts_scene(bstyle!(flex_row_reverse));
    accepts_scene(bstyle!(flex_col));
    accepts_scene(bstyle!(flex_col_reverse));
    accepts_scene(bstyle!(flex_center));
}

#[test]
fn supports_wrap_styles() {
    accepts_scene(bstyle!(flex_nowrap));
    accepts_scene(bstyle!(flex_wrap));
    accepts_scene(bstyle!(flex_wrap_reverse));
}

#[test]
fn supports_justify_styles() {
    accepts_scene(bstyle!(justify_start));
    accepts_scene(bstyle!(justify_end));
    accepts_scene(bstyle!(justify_center));
    accepts_scene(bstyle!(justify_between));
    accepts_scene(bstyle!(justify_around));
    accepts_scene(bstyle!(justify_evenly));
    accepts_scene(bstyle!(justify_stretch));
}

#[test]
fn supports_item_alignment_styles() {
    accepts_scene(bstyle!(items_start));
    accepts_scene(bstyle!(items_end));
    accepts_scene(bstyle!(items_center));
    accepts_scene(bstyle!(items_baseline));
    accepts_scene(bstyle!(items_stretch));
}

#[test]
fn supports_content_alignment_styles() {
    accepts_scene(bstyle!(content_start));
    accepts_scene(bstyle!(content_end));
    accepts_scene(bstyle!(content_center));
    accepts_scene(bstyle!(content_between));
    accepts_scene(bstyle!(content_around));
    accepts_scene(bstyle!(content_evenly));
    accepts_scene(bstyle!(content_stretch));
}

#[test]
fn combines_non_conflicting_flex_styles() {
    accepts_scene(bstyle!(flex_col items_start justify_between content_center flex_wrap));
}
