//! Shared Bevywind style parsing and utility metadata.

use std::collections::HashSet;

mod border;
mod color;
mod dimension;
mod flex;
mod image;
mod overflow;
mod position;
mod transform;
mod typography;
mod units;

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
    PositionType,
    Left,
    Right,
    Top,
    Bottom,
    OverflowX,
    OverflowY,
    FlexGrow,
    FlexShrink,
    FlexBasis,
    AlignSelf,
    RowGap,
    ColumnGap,
    Image,
    ImageMode,
    ImageFlipX,
    ImageFlipY,
    Transform,
    TransformX,
    TransformY,
    Scale,
    ScaleX,
    ScaleY,
    Rotation,
}

#[derive(Clone, Debug, PartialEq, Eq)]
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
    PositionRelative,
    PositionAbsolute,
    NegativePixels(u16),
    NegativePercent(u16),
    NegativeViewportWidth(u16),
    NegativeViewportHeight(u16),
    OverflowVisible,
    OverflowClip,
    OverflowHidden,
    OverflowScroll,
    FlexGrow(u16),
    FlexShrink(u16),
    FlexBasisAuto,
    AlignSelfAuto,
    AlignSelfStart,
    AlignSelfEnd,
    AlignSelfCenter,
    AlignSelfBaseline,
    AlignSelfStretch,
    ImageUrl(String),
    ImageModeAuto,
    ImageModeStretch,
    ImageModeRepeat,
    ImageModeRepeatX,
    ImageModeRepeatY,
    ImageModeNoRepeat,
    ImageFlipX,
    ImageFlipY,
    Rotation(u16),
    NegativeRotation(u16),
}

#[derive(Clone, Debug, PartialEq, Eq)]
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

    for (offset, class) in split_classes(input) {
        let rules_for_class = match flex::expansion(class, offset) {
            Some(rules) => rules?,
            None => match overflow::expansion(class, offset) {
                Some(rules) => rules?,
                None => match transform::expansion(class, offset) {
                    Some(rules) => rules?,
                    None => match dimension::expansion(class, offset) {
                        Some(rules) => rules?,
                        None => match border::expansion(class, offset) {
                            Some(rules) => rules?,
                            None => vec![parse_class(class, offset)?],
                        },
                    },
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

fn split_classes(input: &str) -> Vec<(usize, &str)> {
    let mut classes = Vec::new();
    let mut start = None;
    let mut quote = false;
    let mut escaped = false;
    let mut depth: usize = 0;

    for (index, character) in input.char_indices() {
        if escaped {
            escaped = false;
            continue;
        }
        if quote && character == '\\' {
            escaped = true;
            continue;
        }
        if character == '"' {
            quote = !quote;
            if start.is_none() {
                start = Some(index);
            }
            continue;
        }
        if !quote && character == '(' {
            depth += 1;
        } else if !quote && character == ')' {
            depth = depth.saturating_sub(1);
        }
        if !quote && depth == 0 && character.is_whitespace() {
            if let Some(start) = start.take() {
                classes.push((start, &input[start..index]));
            }
        } else if start.is_none() {
            start = Some(index);
        }
    }
    if let Some(start) = start {
        classes.push((start, &input[start..]));
    }
    classes
}

pub fn parse_class(class: &str, offset: usize) -> Result<StyleRule, StyleError> {
    if let Some(rule) = flex::parse(class, offset) {
        return rule;
    }
    if let Some(rule) = position::parse(class, offset) {
        return rule;
    }
    if let Some(rule) = overflow::parse(class, offset) {
        return rule;
    }
    if let Some(rule) = image::parse(class, offset) {
        return rule;
    }
    if let Some(rule) = transform::parse(class, offset) {
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

pub(crate) fn parse_color(
    value: &str,
    class: &str,
    offset: usize,
) -> Result<(u8, u8, u8, u8), StyleError> {
    color::parse_value(value, class, offset)
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
        "relative",
        "absolute",
        "top_10px",
        "right_10px",
        "bottom_10px",
        "left_10px",
        "top_n_10px",
        "right_n_10px",
        "bottom_n_10px",
        "left_n_10px",
        "overflow_visible",
        "overflow_clip",
        "overflow_hidden",
        "overflow_scroll",
        "overflow_x_visible",
        "overflow_x_clip",
        "overflow_x_hidden",
        "overflow_x_scroll",
        "overflow_y_visible",
        "overflow_y_clip",
        "overflow_y_hidden",
        "overflow_y_scroll",
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
        "bgi_url(\"images/panel.png\")",
        "bgi_auto",
        "bgi_stretch",
        "bgi_repeat",
        "bgi_repeat_x",
        "bgi_repeat_y",
        "bgi_no_repeat",
        "bgi_flip_x",
        "bgi_flip_y",
    ]
}
