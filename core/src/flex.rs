use crate::{Property, StyleError, StyleRule, Value};

pub(crate) fn parse(class: &str, offset: usize) -> Option<Result<StyleRule, StyleError>> {
    if class == "grow" {
        return Some(Ok(rule(Property::FlexGrow, Value::FlexGrow(1))));
    }
    if class.starts_with("grow_") {
        return Some(
            crate::units::parse_number(&class[5..], class, offset, "flex grow")
                .map(|value| rule(Property::FlexGrow, Value::FlexGrow(value))),
        );
    }
    if class == "shrink" {
        return Some(Ok(rule(Property::FlexShrink, Value::FlexShrink(1))));
    }
    if class.starts_with("shrink_") {
        return Some(
            crate::units::parse_number(&class[7..], class, offset, "flex shrink")
                .map(|value| rule(Property::FlexShrink, Value::FlexShrink(value))),
        );
    }
    if let Some(value) = class.strip_prefix("basis_") {
        let value = if value == "auto" {
            Ok(Value::FlexBasisAuto)
        } else if value == "0" {
            Ok(Value::Pixels(0))
        } else {
            crate::units::parse(value, class, offset)
        };
        return Some(value.map(|value| rule(Property::FlexBasis, value)));
    }
    if let Some(value) = class.strip_prefix("gap_x_") {
        return Some(parse_gap(value, class, offset).map(|value| rule(Property::ColumnGap, value)));
    }
    if let Some(value) = class.strip_prefix("gap_y_") {
        return Some(parse_gap(value, class, offset).map(|value| rule(Property::RowGap, value)));
    }
    if let Some(value) = class.strip_prefix("gap_") {
        return Some(parse_gap(value, class, offset).map(|value| rule(Property::RowGap, value)));
    }
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
        "self_auto" => Value::AlignSelfAuto,
        "self_start" => Value::AlignSelfStart,
        "self_end" => Value::AlignSelfEnd,
        "self_center" => Value::AlignSelfCenter,
        "self_baseline" => Value::AlignSelfBaseline,
        "self_stretch" => Value::AlignSelfStretch,
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
        Value::AlignSelfAuto
        | Value::AlignSelfStart
        | Value::AlignSelfEnd
        | Value::AlignSelfCenter
        | Value::AlignSelfBaseline
        | Value::AlignSelfStretch => Property::AlignSelf,
        _ => Property::AlignContent,
    };
    Some(Ok(StyleRule { property, value }))
}

pub(crate) fn expansion(class: &str, offset: usize) -> Option<Result<Vec<StyleRule>, StyleError>> {
    if let Some(value) = class.strip_prefix("gap_") {
        if value.starts_with("x_") || value.starts_with("y_") {
            return None;
        }
        let value = parse_gap(value, class, offset);
        return Some(value.map(|value| {
            vec![
                rule(Property::RowGap, value),
                rule(Property::ColumnGap, value),
            ]
        }));
    }
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

fn parse_gap(value: &str, class: &str, offset: usize) -> Result<Value, StyleError> {
    if value == "0" {
        return Ok(Value::Pixels(0));
    }
    crate::units::parse(value, class, offset)
}

fn rule(property: Property, value: Value) -> StyleRule {
    StyleRule { property, value }
}
