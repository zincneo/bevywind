use bevy::prelude::*;
use bevywind::{bstyle, style_runtime};

fn accepts_scene<S: Scene>(_: S) {}

#[test]
fn size_style_is_a_node_scene() {
    accepts_scene(bstyle!(h-full w-full));
}

#[test]
fn pixel_size_style_is_a_node_scene() {
    accepts_scene(bstyle!(h-10px w-20px));
}

#[test]
fn token_style_is_a_node_scene() {
    accepts_scene(bstyle!(h-10px w-20px min-h-50%));
}

#[test]
fn percent_size_style_is_a_node_scene() {
    accepts_scene(bstyle!(h-10% w-20%));
}

#[test]
fn viewport_size_style_is_a_node_scene() {
    accepts_scene(bstyle!(h-10w w-20w));
    accepts_scene(bstyle!(h-30h w-40h));
}

#[test]
fn constrained_size_style_is_a_node_scene() {
    accepts_scene(bstyle!(min-h-10px min-w-20% max-h-30w max-w-40h));
}

#[test]
fn runtime_style_is_a_scene() {
    let classes = String::from("h-10px w-20%");
    accepts_scene(style_runtime(&classes));
}
