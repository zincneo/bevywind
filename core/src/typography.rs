use crate::{Property, StyleError, StyleRule, Value, color, error};

pub(crate) fn parse(class: &str, offset: usize) -> Result<StyleRule, StyleError> {
    let Some(value) = class.strip_prefix("t_") else {
        return Err(error(class, offset, "unknown style utility"));
    };

    if let Ok((red, green, blue, alpha)) = color::parse_value(value, class, offset) {
        return Ok(StyleRule {
            property: Property::TextColor,
            value: Value::TextColor(red, green, blue, alpha),
        });
    }

    let value = match value {
        "xs" => Value::FontSize(12),
        "sm" => Value::FontSize(14),
        "base" => Value::FontSize(16),
        "lg" => Value::FontSize(18),
        "xl" => Value::FontSize(20),
        "2xl" => Value::FontSize(24),
        "3xl" => Value::FontSize(30),
        "4xl" => Value::FontSize(36),
        "5xl" => Value::FontSize(48),
        "6xl" => Value::FontSize(60),
        "7xl" => Value::FontSize(72),
        "8xl" => Value::FontSize(96),
        "9xl" => Value::FontSize(128),
        "left" => Value::TextJustifyLeft,
        "center" => Value::TextJustifyCenter,
        "right" => Value::TextJustifyRight,
        "justify" => Value::TextJustify,
        "start" => Value::TextJustifyStart,
        "end" => Value::TextJustifyEnd,
        "leading_none" => Value::LineHeightRelative(1000),
        "leading_tight" => Value::LineHeightRelative(1250),
        "leading_snug" => Value::LineHeightRelative(1375),
        "leading_normal" => Value::LineHeightRelative(1500),
        "leading_relaxed" => Value::LineHeightRelative(1625),
        "leading_loose" => Value::LineHeightRelative(2000),
        "whitespace_normal" => Value::LineBreakWordBoundary,
        "whitespace_nowrap" => Value::LineBreakNoWrap,
        "break_words" => Value::LineBreakAnyCharacter,
        "break_word" => Value::LineBreakWordOrCharacter,
        "thin" => Value::FontWeight(100),
        "extralight" => Value::FontWeight(200),
        "light" => Value::FontWeight(300),
        "normal" => Value::FontWeight(400),
        "medium" => Value::FontWeight(500),
        "semibold" => Value::FontWeight(600),
        "bold" => Value::FontWeight(700),
        "extrabold" => Value::FontWeight(800),
        "w_black" => Value::FontWeight(900),
        "italic" => Value::FontStyleItalic,
        "not_italic" => Value::FontStyleNormal,
        value
            if value
                .strip_prefix("leading_")
                .and_then(|v| v.strip_suffix("px"))
                .is_some() =>
        {
            let number = value
                .strip_prefix("leading_")
                .unwrap()
                .strip_suffix("px")
                .unwrap();
            Value::LineHeightPixels(parse_number(number, class, offset)?)
        }
        value
            if value
                .strip_prefix("leading_")
                .and_then(|v| v.strip_suffix("rel"))
                .is_some() =>
        {
            let number = value
                .strip_prefix("leading_")
                .unwrap()
                .strip_suffix("rel")
                .unwrap();
            Value::LineHeightRelative(parse_number(number, class, offset)?.saturating_mul(10))
        }
        value if value.strip_suffix("px").is_some() => {
            let number = value.strip_suffix("px").unwrap();
            Value::FontSize(parse_number(number, class, offset)?)
        }
        _ => return Err(error(class, offset, "unknown typography style")),
    };

    let property = match value {
        Value::FontSize(_) => Property::FontSize,
        Value::TextJustifyLeft
        | Value::TextJustifyCenter
        | Value::TextJustifyRight
        | Value::TextJustify
        | Value::TextJustifyStart
        | Value::TextJustifyEnd => Property::TextJustify,
        Value::LineHeightRelative(_) | Value::LineHeightPixels(_) => Property::LineHeight,
        Value::LineBreakWordBoundary
        | Value::LineBreakNoWrap
        | Value::LineBreakAnyCharacter
        | Value::LineBreakWordOrCharacter => Property::LineBreak,
        Value::FontWeight(_) => Property::FontWeight,
        Value::FontStyleNormal | Value::FontStyleItalic => Property::FontStyle,
        _ => unreachable!(),
    };
    Ok(StyleRule { property, value })
}

fn parse_number(number: &str, class: &str, offset: usize) -> Result<u16, StyleError> {
    if number.is_empty() || !number.bytes().all(|byte| byte.is_ascii_digit()) {
        return Err(error(
            class,
            offset,
            "typography value must be a non-negative integer",
        ));
    }
    number
        .parse::<u16>()
        .map_err(|_| error(class, offset, "typography value does not fit in u16"))
}
