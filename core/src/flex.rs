use crate::{Property, StyleError, StyleRule, Value};

pub(crate) fn parse(class: &str, _offset: usize) -> Option<Result<StyleRule, StyleError>> {
    let value = match class {
        "flex_row" => Value::FlexDirectionRow,
        "flex_row_reverse" => Value::FlexDirectionRowReverse,
        "flex_col" => Value::FlexDirectionColumn,
        "flex_col_reverse" => Value::FlexDirectionColumnReverse,
        "flex_nowrap" => Value::FlexWrapNoWrap,
        "flex_wrap" => Value::FlexWrap,
        "flex_wrap_reverse" => Value::FlexWrapReverse,
        "justify_start" => Value::JustifyStart,
        "justify_end" => Value::JustifyEnd,
        "justify_center" => Value::JustifyCenter,
        "justify_between" => Value::JustifyBetween,
        "justify_around" => Value::JustifyAround,
        "justify_evenly" => Value::JustifyEvenly,
        "justify_stretch" => Value::JustifyStretch,
        "items_start" => Value::AlignStart,
        "items_end" => Value::AlignEnd,
        "items_center" => Value::AlignCenter,
        "items_baseline" => Value::AlignBaseline,
        "items_stretch" => Value::AlignStretch,
        "content_start" => Value::ContentStart,
        "content_end" => Value::ContentEnd,
        "content_center" => Value::ContentCenter,
        "content_between" => Value::ContentBetween,
        "content_around" => Value::ContentAround,
        "content_evenly" => Value::ContentEvenly,
        "content_stretch" => Value::ContentStretch,
        _ => return None,
    };
    let property = match value {
        Value::FlexDirectionRow
        | Value::FlexDirectionRowReverse
        | Value::FlexDirectionColumn
        | Value::FlexDirectionColumnReverse => Property::FlexDirection,
        Value::FlexWrapNoWrap | Value::FlexWrap | Value::FlexWrapReverse => Property::FlexWrap,
        Value::JustifyStart
        | Value::JustifyEnd
        | Value::JustifyCenter
        | Value::JustifyBetween
        | Value::JustifyAround
        | Value::JustifyEvenly
        | Value::JustifyStretch => Property::JustifyContent,
        Value::AlignStart
        | Value::AlignEnd
        | Value::AlignCenter
        | Value::AlignBaseline
        | Value::AlignStretch => Property::AlignItems,
        _ => Property::AlignContent,
    };
    Some(Ok(StyleRule { property, value }))
}

pub(crate) fn expansion(class: &str, offset: usize) -> Option<Result<Vec<StyleRule>, StyleError>> {
    if class == "flex_center" {
        return Some(Ok(vec![
            StyleRule {
                property: Property::Display,
                value: Value::DisplayFlex,
            },
            StyleRule {
                property: Property::FlexDirection,
                value: Value::FlexDirectionRow,
            },
            StyleRule {
                property: Property::JustifyContent,
                value: Value::JustifyCenter,
            },
            StyleRule {
                property: Property::AlignItems,
                value: Value::AlignCenter,
            },
        ]));
    }
    if matches!(
        class,
        "flex_row" | "flex_row_reverse" | "flex_col" | "flex_col_reverse"
    ) {
        return Some(Ok(vec![
            StyleRule {
                property: Property::Display,
                value: Value::DisplayFlex,
            },
            parse(class, offset)
                .expect("flex direction was matched")
                .unwrap(),
        ]));
    }
    None
}
