use bevy::ui::Node;
use bevywind_core::{Property, Value};

pub(crate) fn apply(node: &mut Node, property: Property, value: Value) {
    match property {
        Property::Display => {
            if let Value::DisplayFlex = value {
                node.display = bevy::ui::Display::Flex
            }
        }
        Property::FlexDirection => {
            node.flex_direction = match value {
                Value::FlexDirectionRow => bevy::ui::FlexDirection::Row,
                Value::FlexDirectionRowReverse => bevy::ui::FlexDirection::RowReverse,
                Value::FlexDirectionColumn => bevy::ui::FlexDirection::Column,
                Value::FlexDirectionColumnReverse => bevy::ui::FlexDirection::ColumnReverse,
                _ => return,
            }
        }
        Property::FlexWrap => {
            node.flex_wrap = match value {
                Value::FlexWrapNoWrap => bevy::ui::FlexWrap::NoWrap,
                Value::FlexWrap => bevy::ui::FlexWrap::Wrap,
                Value::FlexWrapReverse => bevy::ui::FlexWrap::WrapReverse,
                _ => return,
            }
        }
        Property::JustifyContent => node.justify_content = justify(value),
        Property::AlignItems => node.align_items = items(value),
        Property::AlignContent => node.align_content = content(value),
        _ => return,
    }
}

fn justify(value: Value) -> bevy::ui::JustifyContent {
    match value {
        Value::JustifyStart => bevy::ui::JustifyContent::Start,
        Value::JustifyEnd => bevy::ui::JustifyContent::End,
        Value::JustifyCenter => bevy::ui::JustifyContent::Center,
        Value::JustifyBetween => bevy::ui::JustifyContent::SpaceBetween,
        Value::JustifyAround => bevy::ui::JustifyContent::SpaceAround,
        Value::JustifyEvenly => bevy::ui::JustifyContent::SpaceEvenly,
        Value::JustifyStretch => bevy::ui::JustifyContent::Stretch,
        _ => return bevy::ui::JustifyContent::Default,
    }
}

fn items(value: Value) -> bevy::ui::AlignItems {
    match value {
        Value::AlignStart => bevy::ui::AlignItems::Start,
        Value::AlignEnd => bevy::ui::AlignItems::End,
        Value::AlignCenter => bevy::ui::AlignItems::Center,
        Value::AlignBaseline => bevy::ui::AlignItems::Baseline,
        Value::AlignStretch => bevy::ui::AlignItems::Stretch,
        _ => return bevy::ui::AlignItems::Default,
    }
}

fn content(value: Value) -> bevy::ui::AlignContent {
    match value {
        Value::ContentStart => bevy::ui::AlignContent::Start,
        Value::ContentEnd => bevy::ui::AlignContent::End,
        Value::ContentCenter => bevy::ui::AlignContent::Center,
        Value::ContentBetween => bevy::ui::AlignContent::SpaceBetween,
        Value::ContentAround => bevy::ui::AlignContent::SpaceAround,
        Value::ContentEvenly => bevy::ui::AlignContent::SpaceEvenly,
        Value::ContentStretch => bevy::ui::AlignContent::Stretch,
        _ => return bevy::ui::AlignContent::Default,
    }
}
