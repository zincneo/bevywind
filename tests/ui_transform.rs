use bevywind::bstyle;
use bevywind_core::{Property, Value, parse_classes};

mod common;
use common::accepts_scene;

#[test]
fn supports_ui_transform_styles() {
    accepts_scene(bstyle!(
        tr_x_10px tr_y_n_5px sc_x_120per sc_y_90per rt_n_3deg
    ));
    accepts_scene(bstyle!(sc_110per));
    accepts_scene(bstyle!(tr_10per));
}

#[test]
fn expands_and_parses_ui_transform_styles() {
    assert_eq!(
        parse_classes("tr_10px sc_150per rt_45deg").unwrap(),
        vec![
            bevywind_core::StyleRule {
                property: Property::TransformX,
                value: Value::Pixels(10),
            },
            bevywind_core::StyleRule {
                property: Property::TransformY,
                value: Value::Pixels(10),
            },
            bevywind_core::StyleRule {
                property: Property::ScaleX,
                value: Value::Percent(150),
            },
            bevywind_core::StyleRule {
                property: Property::ScaleY,
                value: Value::Percent(150),
            },
            bevywind_core::StyleRule {
                property: Property::Rotation,
                value: Value::Rotation(45),
            },
        ]
    );
}

#[test]
fn supports_transform_units_and_negative_values() {
    let rules = parse_classes("tr_x_n_10per tr_y_20w sc_x_0per rt_n_90deg").unwrap();
    assert_eq!(rules[0].value, Value::NegativePercent(10));
    assert_eq!(rules[1].value, Value::ViewportWidth(20));
    assert_eq!(rules[2].value, Value::Percent(0));
    assert_eq!(rules[3].value, Value::NegativeRotation(90));
}

#[test]
fn rejects_conflicting_transform_fields() {
    assert!(parse_classes("tr_10px tr_x_20px").is_err());
    assert!(parse_classes("sc_100per sc_y_120per").is_err());
    assert!(parse_classes("rt_45deg rt_n_45deg").is_err());
    assert!(parse_classes("tr_x_10px tr_y_20px").is_ok());
    assert!(parse_classes("sc_x_100per sc_y_120per").is_ok());
}

#[test]
fn scale_only_accepts_percent_units() {
    assert!(parse_classes("sc_100per").is_ok());
    assert!(parse_classes("sc_x_150per sc_y_80per").is_ok());
    assert!(parse_classes("sc_100px").is_err());
    assert!(parse_classes("sc_x_2w").is_err());
    assert!(parse_classes("sc_y_3h").is_err());
    assert!(parse_classes("sc_full").is_err());
    assert!(parse_classes("sc_100").is_err());
}
