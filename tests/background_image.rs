use bevywind::bstyle;
use bevywind_core::{Property, Value, parse_classes};

mod common;
use common::accepts_scene;

#[test]
fn supports_background_image_styles() {
    accepts_scene(bstyle!(
        w_320px h_200px
        bgi_url("images/panel.png")
        bgi_stretch
        bgi_flip_x
        bgi_flip_y
    ));
    accepts_scene(bstyle!(bgi_auto));
    accepts_scene(bstyle!(bgi_repeat));
    accepts_scene(bstyle!(bgi_repeat_x));
    accepts_scene(bstyle!(bgi_repeat_y));
    accepts_scene(bstyle!(bgi_no_repeat));
}

#[test]
fn parses_background_image_resource_and_modes() {
    assert_eq!(
        parse_classes(r#"bgi_url("images/panel.png") bgi_repeat_x bgi_flip_y"#).unwrap(),
        vec![
            bevywind_core::StyleRule {
                property: Property::Image,
                value: Value::ImageUrl("images/panel.png".to_owned()),
            },
            bevywind_core::StyleRule {
                property: Property::ImageMode,
                value: Value::ImageModeRepeatX,
            },
            bevywind_core::StyleRule {
                property: Property::ImageFlipY,
                value: Value::ImageFlipY,
            },
        ]
    );
}

#[test]
fn supports_spaces_inside_image_path() {
    let rules = parse_classes(r#"bgi_url("images/panel backgrounds/panel.png")"#).unwrap();
    assert_eq!(
        rules[0].value,
        Value::ImageUrl("images/panel backgrounds/panel.png".to_owned())
    );
}

#[test]
fn rejects_conflicting_background_image_fields() {
    assert!(parse_classes(r#"bgi_url("one.png") bgi_url("two.png")"#).is_err());
    assert!(parse_classes("bgi_stretch bgi_repeat").is_err());
    assert!(parse_classes("bgi_flip_x bgi_flip_x").is_err());
}

#[test]
fn rejects_invalid_background_image_urls() {
    assert!(parse_classes(r#"bgi_url("one.png")"#).is_ok());
    assert!(parse_classes("bgi_url(one.png)").is_err());
    assert!(parse_classes("bgi_url(\"\")").is_err());
    assert!(parse_classes("bgi_url(\"one.png\"").is_err());
}
