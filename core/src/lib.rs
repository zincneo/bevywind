//! Shared Bevywind style parsing and utility metadata.

use std::collections::HashSet;

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
        "h-full",
        "w-full",
        "h-100px",
        "w-100px",
        "h-100%",
        "w-100%",
        "h-100w",
        "w-100w",
        "h-100h",
        "w-100h",
        "min-h-100px",
        "min-w-100px",
        "max-h-100px",
        "max-w-100px",
    ]
}
