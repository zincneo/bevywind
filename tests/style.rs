use bevy::prelude::*;
use bevywind::{style, style_runtime};

fn accepts_scene<S: Scene>(_: S) {}

#[test]
fn size_style_is_a_node_scene() {
    accepts_scene(style!("h-full w-full"));
}

#[test]
fn pixel_size_style_is_a_node_scene() {
    accepts_scene(style!("h-10px w-20px"));
}

#[test]
fn percent_size_style_is_a_node_scene() {
    accepts_scene(style!("h-10% w-20%"));
}

#[test]
fn viewport_size_style_is_a_node_scene() {
    accepts_scene(style!("h-10w w-20w h-30h w-40h"));
}

#[test]
fn runtime_style_is_a_scene() {
    let classes = String::from("h-10px w-20%");
    accepts_scene(style_runtime(&classes));
}
