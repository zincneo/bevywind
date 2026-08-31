use bevywind::bstyle;
use bevywind_core::{Property, Value, parse_classes};

mod common;
use common::accepts_scene;

#[test]
fn supports_border_widths() {
    accepts_scene(bstyle!(b_1px));
    accepts_scene(bstyle!(bl_2per br_3w bt_4h bb_5px));
}

#[test]
fn supports_border_radius_styles() {
    accepts_scene(bstyle!(b_r_none));
    accepts_scene(bstyle!(b_r_sm));
    accepts_scene(bstyle!(b_r));
    accepts_scene(bstyle!(b_r_md));
    accepts_scene(bstyle!(b_r_lg));
    accepts_scene(bstyle!(b_r_xl));
    accepts_scene(bstyle!(b_r_2xl));
    accepts_scene(bstyle!(b_r_3xl));
    accepts_scene(bstyle!(b_r_full));
    accepts_scene(bstyle!(b_r_10px));
    accepts_scene(bstyle!(bl_r_10px br_r_20per));
    accepts_scene(bstyle!(bt_r_3w bb_r_4h));
}

#[test]
fn supports_border_colors() {
    accepts_scene(bstyle!(b_112233));
    accepts_scene(bstyle!(b_red_50));
    accepts_scene(bstyle!(bl_red_500 br_aabbcc bt_ffffff bb_00000000));
}

#[test]
fn supports_every_named_palette_and_shade() {
    let palettes = [
        "slate", "gray", "zinc", "neutral", "stone", "red", "orange", "amber", "yellow", "lime",
        "green", "emerald", "teal", "cyan", "sky", "blue", "indigo", "violet", "purple", "fuchsia",
        "pink", "rose",
    ];
    let shades = [50, 100, 200, 300, 400, 500, 600, 700, 800, 900, 950];

    for palette in palettes {
        for shade in shades {
            let class = format!("b_{palette}_{shade}");
            assert!(parse_classes(&class).is_ok(), "failed to parse {class}");
            let class = format!("bl_{palette}_{shade}");
            assert!(parse_classes(&class).is_ok(), "failed to parse {class}");
        }
    }
}

#[test]
fn expands_all_border_widths_and_colors() {
    let widths = parse_classes("b_10px").unwrap();
    assert_eq!(widths.len(), 4);
    assert!(widths.iter().all(|rule| {
        matches!(
            rule.property,
            Property::BorderLeft
                | Property::BorderRight
                | Property::BorderTop
                | Property::BorderBottom
        ) && rule.value == Value::Pixels(10)
    }));

    let colors = parse_classes("b_11223380").unwrap();
    assert_eq!(colors.len(), 4);
    assert!(colors.iter().all(|rule| {
        matches!(
            rule.property,
            Property::BorderColorLeft
                | Property::BorderColorRight
                | Property::BorderColorTop
                | Property::BorderColorBottom
        ) && rule.value == Value::BorderColor(0x11, 0x22, 0x33, 0x80)
    }));
}

#[test]
fn rejects_overlapping_border_sides() {
    assert!(parse_classes("b_1px bl_2px").is_err());
    assert!(parse_classes("b_ffffff bl_112233").is_err());
    assert!(parse_classes("b_r_lg bl_r_2px").is_err());
    assert!(parse_classes("bl_r_2px bt_r_3px").is_err());
}

#[test]
fn expands_border_radius_to_the_expected_corners() {
    let rules = parse_classes("bl_r_10px").unwrap();
    assert_eq!(rules.len(), 2);
    assert!(rules.iter().all(|rule| {
        matches!(
            rule.property,
            Property::BorderRadiusTopLeft | Property::BorderRadiusBottomLeft
        ) && rule.value == Value::RadiusPixels(10)
    }));
}

#[test]
fn combines_border_with_other_node_styles() {
    accepts_scene(bstyle!(w_full flex_row p_10px b_1px b_ffffff));
}
