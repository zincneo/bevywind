//! Shared Bevywind style parsing and utility metadata.

use std::collections::HashSet;

mod border;
mod color;
mod dimension;
mod flex;
mod typography;

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
    MarginLeft,
    MarginRight,
    MarginTop,
    MarginBottom,
    PaddingLeft,
    PaddingRight,
    PaddingTop,
    PaddingBottom,
    BorderLeft,
    BorderRight,
    BorderTop,
    BorderBottom,
    BorderColorLeft,
    BorderColorRight,
    BorderColorTop,
    BorderColorBottom,
    BackgroundColor,
    TextColor,
    FontSize,
    TextJustify,
    LineHeight,
    LineBreak,
    FontWeight,
    FontStyle,
    BorderRadiusTopLeft,
    BorderRadiusTopRight,
    BorderRadiusBottomRight,
    BorderRadiusBottomLeft,
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
    BorderColor(u8, u8, u8, u8),
    TextColor(u8, u8, u8, u8),
    FontSize(u16),
    TextJustifyLeft,
    TextJustifyCenter,
    TextJustifyRight,
    TextJustify,
    TextJustifyStart,
    TextJustifyEnd,
    LineHeightRelative(u16),
    LineHeightPixels(u16),
    LineBreakWordBoundary,
    LineBreakNoWrap,
    LineBreakAnyCharacter,
    LineBreakWordOrCharacter,
    FontWeight(u16),
    FontStyleNormal,
    FontStyleItalic,
    RadiusPixels(u16),
    RadiusPercent(u16),
    RadiusViewportWidth(u16),
    RadiusViewportHeight(u16),
    RadiusFull,
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
            None => match dimension::expansion(class, offset) {
                Some(rules) => rules?,
                None => match border::expansion(class, offset) {
                    Some(rules) => rules?,
                    None => vec![parse_class(class, offset)?],
                },
            },
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
    if class.starts_with("t_") {
        return typography::parse(class, offset);
    }
    if class.starts_with("b_")
        || class.starts_with("bl_")
        || class.starts_with("br_")
        || class.starts_with("bt_")
        || class.starts_with("bb_")
        || class.starts_with("btl_r_")
        || class.starts_with("btr_r_")
        || class.starts_with("bbl_r_")
        || class.starts_with("bbr_r_")
    {
        return border::parse(class, offset);
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
        "m_10px",
        "ml_10px",
        "mr_10px",
        "mt_10px",
        "mb_10px",
        "p_10px",
        "pl_10px",
        "pr_10px",
        "pt_10px",
        "pb_10px",
        "b_1px",
        "bl_1px",
        "br_1px",
        "bt_1px",
        "bb_1px",
        "b_r_none",
        "b_r_sm",
        "b_r",
        "b_r_md",
        "b_r_lg",
        "b_r_xl",
        "b_r_2xl",
        "b_r_3xl",
        "b_r_full",
        "b_r_10px",
        "btl_r_10px",
        "btr_r_10px",
        "bbl_r_10px",
        "bbr_r_10px",
        "b_rrggbb",
        "b_rrggbbaa",
        "b_red",
        "b_red_500",
        "bg_red",
        "bg_red_500",
        "bg_rrggbb",
        "bg_rrggbbaa",
        "t_red",
        "t_red_500",
        "t_rrggbb",
        "t_rrggbbaa",
        "t_xs",
        "t_lg",
        "t_center",
        "t_bold",
        "t_italic",
    ]
}
