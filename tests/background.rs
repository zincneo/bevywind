use bevywind::bstyle;
use bevywind_core::parse_classes;

mod common;
use common::accepts_scene;

#[test]
fn supports_special_background_colors() {
    accepts_scene(bstyle!(bg_transparent));
    accepts_scene(bstyle!(bg_black));
    accepts_scene(bstyle!(bg_white));
}

#[test]
fn supports_hex_background_colors() {
    accepts_scene(bstyle!(bg_112233));
    accepts_scene(bstyle!(bg_11223380));
}

#[test]
fn supports_every_named_palette() {
    accepts_scene(bstyle!(bg_slate));
    accepts_scene(bstyle!(bg_gray));
    accepts_scene(bstyle!(bg_zinc));
    accepts_scene(bstyle!(bg_neutral));
    accepts_scene(bstyle!(bg_stone));
    accepts_scene(bstyle!(bg_red));
    accepts_scene(bstyle!(bg_orange));
    accepts_scene(bstyle!(bg_amber));
    accepts_scene(bstyle!(bg_yellow));
    accepts_scene(bstyle!(bg_lime));
    accepts_scene(bstyle!(bg_green));
    accepts_scene(bstyle!(bg_emerald));
    accepts_scene(bstyle!(bg_teal));
    accepts_scene(bstyle!(bg_cyan));
    accepts_scene(bstyle!(bg_sky));
    accepts_scene(bstyle!(bg_blue));
    accepts_scene(bstyle!(bg_indigo));
    accepts_scene(bstyle!(bg_violet));
    accepts_scene(bstyle!(bg_purple));
    accepts_scene(bstyle!(bg_fuchsia));
    accepts_scene(bstyle!(bg_pink));
    accepts_scene(bstyle!(bg_rose));
}

#[test]
fn supports_every_documented_shade_for_every_palette() {
    let palettes = [
        "slate", "gray", "zinc", "neutral", "stone", "red", "orange", "amber", "yellow", "lime",
        "green", "emerald", "teal", "cyan", "sky", "blue", "indigo", "violet", "purple", "fuchsia",
        "pink", "rose",
    ];
    let shades = [50, 100, 200, 300, 400, 500, 600, 700, 800, 900, 950];

    for palette in palettes {
        for shade in shades {
            let class = format!("bg_{palette}_{shade}");
            assert!(parse_classes(&class).is_ok(), "failed to parse {class}");
        }
    }
}
