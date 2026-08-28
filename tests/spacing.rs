use bevywind::bstyle;
use bevywind_core::{Property, Value, parse_classes};

mod common;
use common::accepts_scene;

#[test]
fn supports_margin_styles() {
    accepts_scene(bstyle!(m_10px));
    accepts_scene(bstyle!(ml_10per));
    accepts_scene(bstyle!(mr_20w));
    accepts_scene(bstyle!(mt_30h));
    accepts_scene(bstyle!(mb_40px));
}

#[test]
fn supports_padding_styles() {
    accepts_scene(bstyle!(p_10px));
    accepts_scene(bstyle!(pl_10per));
    accepts_scene(bstyle!(pr_20w));
    accepts_scene(bstyle!(pt_30h));
    accepts_scene(bstyle!(pb_40px));
}

#[test]
fn expands_all_sides_and_preserves_values() {
    assert_eq!(
        parse_classes("m_10px").unwrap(),
        vec![
            bevywind_core::StyleRule {
                property: Property::MarginLeft,
                value: Value::Pixels(10)
            },
            bevywind_core::StyleRule {
                property: Property::MarginRight,
                value: Value::Pixels(10)
            },
            bevywind_core::StyleRule {
                property: Property::MarginTop,
                value: Value::Pixels(10)
            },
            bevywind_core::StyleRule {
                property: Property::MarginBottom,
                value: Value::Pixels(10)
            },
        ]
    );
    assert_eq!(
        parse_classes("p_20per").unwrap(),
        vec![
            bevywind_core::StyleRule {
                property: Property::PaddingLeft,
                value: Value::Percent(20)
            },
            bevywind_core::StyleRule {
                property: Property::PaddingRight,
                value: Value::Percent(20)
            },
            bevywind_core::StyleRule {
                property: Property::PaddingTop,
                value: Value::Percent(20)
            },
            bevywind_core::StyleRule {
                property: Property::PaddingBottom,
                value: Value::Percent(20)
            },
        ]
    );
}

#[test]
fn combines_non_conflicting_spacing_styles() {
    accepts_scene(bstyle!(mt_10px mb_20px pl_30per pr_40w));
}

#[test]
fn combines_all_node_styles_into_one_scene_node() {
    accepts_scene(bstyle!(
        w_full h_100px flex_col items_center
        ml_20per mr_10px
        pr_40h pl_30w
    ));
}

#[test]
fn combines_node_styles_with_background_color() {
    accepts_scene(bstyle!(w_full p_10px bg_blue_500));
}

#[test]
fn rejects_overlapping_spacing_styles() {
    assert!(parse_classes("m_10px ml_20px").is_err());
    assert!(parse_classes("pl_10px pl_20px").is_err());
}
