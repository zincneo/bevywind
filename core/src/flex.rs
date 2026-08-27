use crate::{Property, StyleError, StyleRule, Value};

pub(crate) fn parse(class: &str, _offset: usize) -> Option<Result<StyleRule, StyleError>> {
    let value = match class {
        "flex-row" => Value::FlexDirectionRow,
        "flex-row-reverse" => Value::FlexDirectionRowReverse,
        "flex-col" => Value::FlexDirectionColumn,
        "flex-col-reverse" => Value::FlexDirectionColumnReverse,
        "flex-nowrap" => Value::FlexWrapNoWrap,
        "flex-wrap" => Value::FlexWrap,
        "flex-wrap-reverse" => Value::FlexWrapReverse,
        "justify-start" => Value::JustifyStart,
        "justify-end" => Value::JustifyEnd,
        "justify-center" => Value::JustifyCenter,
        "justify-between" => Value::JustifyBetween,
        "justify-around" => Value::JustifyAround,
        "justify-evenly" => Value::JustifyEvenly,
        "justify-stretch" => Value::JustifyStretch,
        "items-start" => Value::AlignStart,
        "items-end" => Value::AlignEnd,
        "items-center" => Value::AlignCenter,
        "items-baseline" => Value::AlignBaseline,
        "items-stretch" => Value::AlignStretch,
        "content-start" => Value::ContentStart,
        "content-end" => Value::ContentEnd,
        "content-center" => Value::ContentCenter,
        "content-between" => Value::ContentBetween,
        "content-around" => Value::ContentAround,
        "content-evenly" => Value::ContentEvenly,
        "content-stretch" => Value::ContentStretch,
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
    if class == "flex-center" {
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
        "flex-row" | "flex-row-reverse" | "flex-col" | "flex-col-reverse"
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
