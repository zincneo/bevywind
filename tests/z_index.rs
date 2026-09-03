use bevywind::bstyle;
use bevywind_core::{Property, Value, parse_classes};

mod common;
use common::accepts_scene;

#[test]
fn supports_z_index_styles() {
    accepts_scene(bstyle!(z_10));
    accepts_scene(bstyle!(z_n_10 gz_100));
}

#[test]
fn parses_z_index_components() {
    assert_eq!(
        parse_classes("z_n_20 gz_100").unwrap(),
        vec![
            bevywind_core::StyleRule {
                property: Property::ZIndex,
                value: Value::ZIndex(-20),
            },
            bevywind_core::StyleRule {
                property: Property::GlobalZIndex,
                value: Value::GlobalZIndex(100),
            },
        ]
    );
    assert_eq!(parse_classes("z_10").unwrap()[0].value, Value::ZIndex(10));
    assert_eq!(
        parse_classes("gz_n_5").unwrap()[0].value,
        Value::GlobalZIndex(-5)
    );
}

#[test]
fn rejects_invalid_or_duplicate_z_index_styles() {
    assert!(parse_classes("z_auto").is_err());
    assert!(parse_classes("z_10px").is_err());
    assert!(parse_classes("z_10 z_n_20").is_err());
    assert!(parse_classes("gz_10 gz_20").is_err());
    assert!(parse_classes("z_10 gz_20").is_ok());
}
