//! Shared Bevywind style parsing and utility metadata.

use std::collections::HashSet;

mod color;
mod dimension;
mod flex;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Property {
    Display,
    FlexDirection,
    FlexWrap,
    JustifyContent,
    AlignItems,
    AlignContent,
    Height,
    Width,
    MinHeight,
    MinWidth,
    MaxHeight,
    MaxWidth,
    BackgroundColor,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Value {
    DisplayFlex,
    FlexDirectionRow,
    FlexDirectionRowReverse,
    FlexDirectionColumn,
    FlexDirectionColumnReverse,
    FlexWrapNoWrap,
    FlexWrap,
    FlexWrapReverse,
    JustifyStart,
    JustifyEnd,
    JustifyCenter,
    JustifyBetween,
    JustifyAround,
    JustifyEvenly,
    JustifyStretch,
    AlignStart,
    AlignEnd,
    AlignCenter,
    AlignBaseline,
    AlignStretch,
    ContentStart,
    ContentEnd,
    ContentCenter,
    ContentBetween,
    ContentAround,
    ContentEvenly,
    ContentStretch,
    Pixels(u16),
    Percent(u16),
    ViewportWidth(u16),
    ViewportHeight(u16),
    Background(u8, u8, u8, u8),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct StyleRule {
    pub property: Property,
    pub value: Value,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StyleError {
    pub offset: usize,
    pub length: usize,
    pub message: String,
}

impl std::fmt::Display for StyleError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&self.message)
    }
}

impl std::error::Error for StyleError {}

/// Parses a whitespace-separated style class string.
pub fn parse_classes(input: &str) -> Result<Vec<StyleRule>, StyleError> {
    let mut rules = Vec::new();
    let mut properties = HashSet::new();

    let mut search_from = 0;
    for class in input.split_whitespace() {
        let offset = search_from + input[search_from..].find(class).unwrap_or(0);
        search_from = offset + class.len();
        let rules_for_class = match flex::expansion(class, offset) {
            Some(rules) => rules?,
            None => vec![parse_class(class, offset)?],
        };

        for rule in rules_for_class {
            if !properties.insert(rule.property) {
                return Err(StyleError {
                    offset,
                    length: class.len(),
                    message: format!("duplicate style property in `{class}`"),
                });
            }
            rules.push(rule);
        }
    }

    Ok(rules)
}

pub fn parse_class(class: &str, offset: usize) -> Result<StyleRule, StyleError> {
    if let Some(rule) = flex::parse(class, offset) {
        return rule;
    }
    if class.starts_with("bg_") {
        return color::parse(class, offset);
    }
    dimension::parse(class, offset)
}

pub(crate) fn error(class: &str, offset: usize, message: &str) -> StyleError {
    StyleError {
        offset,
        length: class.len(),
        message: format!("{message}: `{class}`"),
    }
}

pub fn completion_items() -> &'static [&'static str] {
    &[
        "h_full",
        "w_full",
        "h_100px",
        "w_100px",
        "h_100per",
        "w_100per",
        "h_100w",
        "w_100w",
        "h_100h",
        "w_100h",
        "min_h_100px",
        "min_w_100px",
        "max_h_100px",
        "max_w_100px",
        "bg_red",
        "bg_red_500",
        "bg_rrggbb",
        "bg_rrggbbaa",
    ]
}
