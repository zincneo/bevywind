use bevywind::bstyle;
use bevywind_core::{Property, Value, parse_classes};

mod common;
use common::accepts_scene;

#[test]
fn supports_position_styles() {
    accepts_scene(bstyle!(relative));
    accepts_scene(bstyle!(absolute));
    accepts_scene(bstyle!(absolute top_10px right_20per bottom_30w left_40h));
    accepts_scene(bstyle!(relative top_full left_n_10px));
}

#[test]
fn parses_position_values() {
    assert_eq!(
        parse_classes("absolute top_10px right_n_20per").unwrap(),
        vec![
            bevywind_core::StyleRule {
                property: Property::PositionType,
                value: Value::PositionAbsolute,
            },
            bevywind_core::StyleRule {
                property: Property::Top,
                value: Value::Pixels(10),
            },
            bevywind_core::StyleRule {
                property: Property::Right,
                value: Value::NegativePercent(20),
            },
        ]
    );
}
