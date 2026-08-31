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
        Property::FlexGrow => {
            if let Value::FlexGrow(value) = value {
                node.flex_grow = value as f32;
            }
        }
        Property::FlexShrink => {
            if let Value::FlexShrink(value) = value {
                node.flex_shrink = value as f32;
            }
        }
        Property::FlexBasis => {
            node.flex_basis = match value {
                Value::FlexBasisAuto => bevy::ui::Val::Auto,
                value => match crate::units::to_val(value) {
                    Some(value) => value,
                    None => return,
                },
            }
        }
        Property::JustifyContent => node.justify_content = justify(value),
        Property::AlignItems => node.align_items = items(value),
        Property::AlignSelf => node.align_self = self_alignment(value),
        Property::AlignContent => node.align_content = content(value),
        Property::RowGap => {
            if let Some(value) = crate::units::to_val(value) {
                node.row_gap = value;
            }
        }
        Property::ColumnGap => {
            if let Some(value) = crate::units::to_val(value) {
                node.column_gap = value;
            }
        }
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

fn self_alignment(value: Value) -> bevy::ui::AlignSelf {
    match value {
        Value::AlignSelfAuto => bevy::ui::AlignSelf::Auto,
        Value::AlignSelfStart => bevy::ui::AlignSelf::Start,
        Value::AlignSelfEnd => bevy::ui::AlignSelf::End,
        Value::AlignSelfCenter => bevy::ui::AlignSelf::Center,
        Value::AlignSelfBaseline => bevy::ui::AlignSelf::Baseline,
        Value::AlignSelfStretch => bevy::ui::AlignSelf::Stretch,
        _ => bevy::ui::AlignSelf::Auto,
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
