use bevywind::bstyle;
use bevywind_core::{Property, Value, parse_classes};

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
fn supports_flex_item_and_gap_styles() {
    accepts_scene(bstyle!(grow));
    accepts_scene(bstyle!(grow_2));
    accepts_scene(bstyle!(shrink));
    accepts_scene(bstyle!(shrink_0));
    accepts_scene(bstyle!(basis_auto self_center));
    accepts_scene(bstyle!(basis_100px));
    accepts_scene(bstyle!(gap_10px));
    accepts_scene(bstyle!(gap_x_8px gap_y_12px));
}

#[test]
fn parses_flex_item_and_gap_styles() {
    assert_eq!(
        parse_classes("grow_2 shrink_0 basis_auto self_end gap_10px").unwrap(),
        vec![
            bevywind_core::StyleRule {
                property: Property::FlexGrow,
                value: Value::FlexGrow(2),
            },
            bevywind_core::StyleRule {
                property: Property::FlexShrink,
                value: Value::FlexShrink(0),
            },
            bevywind_core::StyleRule {
                property: Property::FlexBasis,
                value: Value::FlexBasisAuto,
            },
            bevywind_core::StyleRule {
                property: Property::AlignSelf,
                value: Value::AlignSelfEnd,
            },
            bevywind_core::StyleRule {
                property: Property::RowGap,
                value: Value::Pixels(10),
            },
            bevywind_core::StyleRule {
                property: Property::ColumnGap,
                value: Value::Pixels(10),
            },
        ]
    );
}

#[test]
fn rejects_overlapping_flex_gap_styles() {
    assert!(parse_classes("gap_10px gap_x_20px").is_err());
    assert!(parse_classes("gap_x_10px gap_y_20px").is_ok());
}

#[test]
fn combines_non_conflicting_flex_styles() {
    accepts_scene(bstyle!(flex_col items_start justify_between content_center flex_wrap));
}
