use bevywind::bstyle;
use bevywind_core::{Property, Value, parse_classes};

mod common;
use common::accepts_scene;

#[test]
fn supports_overflow_styles() {
    accepts_scene(bstyle!(overflow_visible));
    accepts_scene(bstyle!(overflow_clip));
    accepts_scene(bstyle!(overflow_hidden));
    accepts_scene(bstyle!(overflow_scroll));
    accepts_scene(bstyle!(overflow_x_clip overflow_y_scroll));
}

#[test]
fn expands_two_axis_overflow_styles() {
    assert_eq!(
        parse_classes("overflow_hidden").unwrap(),
        vec![
            bevywind_core::StyleRule {
                property: Property::OverflowX,
                value: Value::OverflowHidden,
            },
            bevywind_core::StyleRule {
                property: Property::OverflowY,
                value: Value::OverflowHidden,
            },
        ]
    );
}

#[test]
fn rejects_overflow_axis_conflicts() {
    assert!(parse_classes("overflow_clip overflow_x_visible").is_err());
    assert!(parse_classes("overflow_x_clip overflow_y_visible").is_ok());
}
