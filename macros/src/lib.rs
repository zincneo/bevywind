use bevywind_core::{Property, Value, parse_classes};
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;

/// Creates a style scene for the current Bevy entity.
#[proc_macro]
pub fn bstyle(input: TokenStream) -> TokenStream {
    let tokens: TokenStream2 = input.into();
    if syn::parse2::<syn::LitStr>(tokens.clone()).is_ok() {
        return compile_error(
            "bstyle! accepts style tokens, not strings; use bstyle_r for runtime strings",
        );
    }
    let classes = tokens.to_string();
    if classes.trim().is_empty() {
        return compile_error("bstyle! expects at least one style utility");
    }

    let rules = match parse_classes(&classes) {
        Ok(rules) => rules,
        Err(error) => return compile_error(error.to_string()),
    };

    let fields: Vec<_> = rules
        .iter()
        .filter_map(|rule| {
            let field = match rule.property {
                Property::Display => quote! { display },
                Property::FlexDirection => quote! { flex_direction },
                Property::FlexWrap => quote! { flex_wrap },
                Property::JustifyContent => quote! { justify_content },
                Property::AlignItems => quote! { align_items },
                Property::AlignContent => quote! { align_content },
                Property::Height => quote! { height },
                Property::Width => quote! { width },
                Property::MinHeight => quote! { min_height },
                Property::MinWidth => quote! { min_width },
                Property::MaxHeight => quote! { max_height },
                Property::MaxWidth => quote! { max_width },
                Property::PositionType => quote! { position_type },
                Property::Left => quote! { left },
                Property::Right => quote! { right },
                Property::Top => quote! { top },
                Property::Bottom => quote! { bottom },
                Property::MarginLeft
                | Property::MarginRight
                | Property::MarginTop
                | Property::MarginBottom
                | Property::PaddingLeft
                | Property::PaddingRight
                | Property::PaddingTop
                | Property::PaddingBottom => return None,
                Property::BorderLeft
                | Property::BorderRight
                | Property::BorderTop
                | Property::BorderBottom
                | Property::BorderRadiusTopLeft
                | Property::BorderRadiusTopRight
                | Property::BorderRadiusBottomRight
                | Property::BorderRadiusBottomLeft
                | Property::BorderColorLeft
                | Property::BorderColorRight
                | Property::BorderColorTop
                | Property::BorderColorBottom => return None,
                Property::BackgroundColor => return None,
                Property::TextColor
                | Property::FontSize
                | Property::TextJustify
                | Property::LineHeight
                | Property::LineBreak
                | Property::FontWeight
                | Property::FontStyle => return None,
            };
            let value = value_tokens(rule.value);
            Some(quote! { #field: #value })
        })
        .collect();
    let margin = rect_tokens(
        &rules,
        [
            Property::MarginLeft,
            Property::MarginRight,
            Property::MarginTop,
            Property::MarginBottom,
        ],
        quote! { margin },
    );
    let padding = rect_tokens(
        &rules,
        [
            Property::PaddingLeft,
            Property::PaddingRight,
            Property::PaddingTop,
            Property::PaddingBottom,
        ],
        quote! { padding },
    );
    let border = rect_tokens(
        &rules,
        [
            Property::BorderLeft,
            Property::BorderRight,
            Property::BorderTop,
            Property::BorderBottom,
        ],
        quote! { border },
    );
    let border_radius = radius_tokens(&rules);
    let node = quote! {
        Node {
            #(#fields,)*
            #margin
            #padding
            #border
            #border_radius
        }
    };
    let backgrounds = rules.iter().filter_map(|rule| {
        let Value::Background(red, green, blue, alpha) = rule.value else {
            return None;
        };
        let red = red as f32 / 255.0;
        let green = green as f32 / 255.0;
        let blue = blue as f32 / 255.0;
        let alpha = alpha as f32 / 255.0;
        Some(quote! {
            BackgroundColor({
                Color::srgba(#red, #green, #blue, #alpha)
            })
        })
    });
    let border_colors = color_rect_tokens(
        &rules,
        [
            Property::BorderColorLeft,
            Property::BorderColorRight,
            Property::BorderColorTop,
            Property::BorderColorBottom,
        ],
    );
    let text_colors = rules.iter().filter_map(|rule| {
        let Value::TextColor(red, green, blue, alpha) = rule.value else {
            return None;
        };
        let red = red as f32 / 255.0;
        let green = green as f32 / 255.0;
        let blue = blue as f32 / 255.0;
        let alpha = alpha as f32 / 255.0;
        Some(quote! {
            ::bevy::text::TextColor({ ::bevy::color::Color::srgba(
                #red, #green, #blue, #alpha,
            ) })
        })
    });
    let text_font_fields: Vec<_> = rules
        .iter()
        .filter_map(|rule| match rule.value {
            Value::FontSize(value) => {
                Some(quote! { font_size: { ::bevy::text::FontSize::Px(#value as f32) } })
            }
            Value::FontWeight(value) => {
                Some(quote! { weight: { ::bevy::text::FontWeight(#value) } })
            }
            Value::FontStyleNormal => Some(quote! { style: { ::bevy::text::FontStyle::Normal } }),
            Value::FontStyleItalic => Some(quote! { style: { ::bevy::text::FontStyle::Italic } }),
            _ => None,
        })
        .collect();
    let text_font = (!text_font_fields.is_empty()).then(|| {
        quote! {
            ::bevy::text::TextFont { #(#text_font_fields,)* }
        }
    });
    let text_layout_fields: Vec<_> = rules
        .iter()
        .filter_map(|rule| match rule.value {
            Value::TextJustifyLeft => Some(quote! { justify: { ::bevy::text::Justify::Left } }),
            Value::TextJustifyCenter => Some(quote! { justify: { ::bevy::text::Justify::Center } }),
            Value::TextJustifyRight => Some(quote! { justify: { ::bevy::text::Justify::Right } }),
            Value::TextJustify => Some(quote! { justify: { ::bevy::text::Justify::Justified } }),
            Value::TextJustifyStart => Some(quote! { justify: { ::bevy::text::Justify::Start } }),
            Value::TextJustifyEnd => Some(quote! { justify: { ::bevy::text::Justify::End } }),
            Value::LineBreakWordBoundary => {
                Some(quote! { linebreak: { ::bevy::text::LineBreak::WordBoundary } })
            }
            Value::LineBreakNoWrap => {
                Some(quote! { linebreak: { ::bevy::text::LineBreak::NoWrap } })
            }
            Value::LineBreakAnyCharacter => {
                Some(quote! { linebreak: { ::bevy::text::LineBreak::AnyCharacter } })
            }
            Value::LineBreakWordOrCharacter => {
                Some(quote! { linebreak: { ::bevy::text::LineBreak::WordOrCharacter } })
            }
            _ => None,
        })
        .collect();
    let text_layout = (!text_layout_fields.is_empty()).then(|| {
        quote! {
            ::bevy::text::TextLayout { #(#text_layout_fields,)* }
        }
    });
    let line_heights = rules.iter().filter_map(|rule| match rule.value {
        Value::LineHeightRelative(value) => Some(quote! {
            { ::bevy::ecs::template::FnTemplate(|_| Ok(::bevy::text::LineHeight::RelativeToFont(#value as f32 / 1000.0))) }
        }),
        Value::LineHeightPixels(value) => Some(quote! {
            { ::bevy::ecs::template::FnTemplate(|_| Ok(::bevy::text::LineHeight::Px(#value as f32))) }
        }),
        _ => None,
    });

    quote! {
        {
            use ::bevy::color::Color;
            use ::bevy::ui::{BackgroundColor, Node};
            ::bevy::scene::bsn! {
            #node
            #(#backgrounds)*
            #border_colors
            #(#text_colors)*
            #text_font
            #text_layout
            #(#line_heights)*
            }
        }
    }
    .into()
}

fn value_tokens(value: Value) -> TokenStream2 {
    match value {
        Value::DisplayFlex => quote! { { ::bevy::ui::Display::Flex } },
        Value::FlexDirectionRow => quote! { { ::bevy::ui::FlexDirection::Row } },
        Value::FlexDirectionRowReverse => quote! { { ::bevy::ui::FlexDirection::RowReverse } },
        Value::FlexDirectionColumn => quote! { { ::bevy::ui::FlexDirection::Column } },
        Value::FlexDirectionColumnReverse => {
            quote! { { ::bevy::ui::FlexDirection::ColumnReverse } }
        }
        Value::FlexWrapNoWrap => quote! { { ::bevy::ui::FlexWrap::NoWrap } },
        Value::FlexWrap => quote! { { ::bevy::ui::FlexWrap::Wrap } },
        Value::FlexWrapReverse => quote! { { ::bevy::ui::FlexWrap::WrapReverse } },
        Value::JustifyStart => quote! { { ::bevy::ui::JustifyContent::Start } },
        Value::JustifyEnd => quote! { { ::bevy::ui::JustifyContent::End } },
        Value::JustifyCenter => quote! { { ::bevy::ui::JustifyContent::Center } },
        Value::JustifyBetween => quote! { { ::bevy::ui::JustifyContent::SpaceBetween } },
        Value::JustifyAround => quote! { { ::bevy::ui::JustifyContent::SpaceAround } },
        Value::JustifyEvenly => quote! { { ::bevy::ui::JustifyContent::SpaceEvenly } },
        Value::JustifyStretch => quote! { { ::bevy::ui::JustifyContent::Stretch } },
        Value::AlignStart => quote! { { ::bevy::ui::AlignItems::Start } },
        Value::AlignEnd => quote! { { ::bevy::ui::AlignItems::End } },
        Value::AlignCenter => quote! { { ::bevy::ui::AlignItems::Center } },
        Value::AlignBaseline => quote! { { ::bevy::ui::AlignItems::Baseline } },
        Value::AlignStretch => quote! { { ::bevy::ui::AlignItems::Stretch } },
        Value::ContentStart => quote! { { ::bevy::ui::AlignContent::Start } },
        Value::ContentEnd => quote! { { ::bevy::ui::AlignContent::End } },
        Value::ContentCenter => quote! { { ::bevy::ui::AlignContent::Center } },
        Value::ContentBetween => quote! { { ::bevy::ui::AlignContent::SpaceBetween } },
        Value::ContentAround => quote! { { ::bevy::ui::AlignContent::SpaceAround } },
        Value::ContentEvenly => quote! { { ::bevy::ui::AlignContent::SpaceEvenly } },
        Value::ContentStretch => quote! { { ::bevy::ui::AlignContent::Stretch } },
        Value::Pixels(value) => quote! { { ::bevy::ui::px(#value) } },
        Value::Percent(value) => quote! { { ::bevy::ui::percent(#value) } },
        Value::ViewportWidth(value) => quote! { { ::bevy::ui::vw(#value) } },
        Value::ViewportHeight(value) => quote! { { ::bevy::ui::vh(#value) } },
        Value::PositionRelative => quote! { { ::bevy::ui::PositionType::Relative } },
        Value::PositionAbsolute => quote! { { ::bevy::ui::PositionType::Absolute } },
        Value::NegativePixels(value) => quote! { { ::bevy::ui::px(-(#value as f32)) } },
        Value::NegativePercent(value) => quote! { { ::bevy::ui::percent(-(#value as f32)) } },
        Value::NegativeViewportWidth(value) => quote! { { ::bevy::ui::vw(-(#value as f32)) } },
        Value::NegativeViewportHeight(value) => {
            quote! { { ::bevy::ui::vh(-(#value as f32)) } }
        }
        Value::RadiusPixels(value) => quote! { { ::bevy::ui::px(#value) } },
        Value::RadiusPercent(value) => quote! { { ::bevy::ui::percent(#value) } },
        Value::RadiusViewportWidth(value) => quote! { { ::bevy::ui::vw(#value) } },
        Value::RadiusViewportHeight(value) => quote! { { ::bevy::ui::vh(#value) } },
        Value::Background(..) | Value::BorderColor(..) => {
            unreachable!("colors are emitted separately")
        }
        Value::TextColor(..)
        | Value::FontSize(_)
        | Value::TextJustifyLeft
        | Value::TextJustifyCenter
        | Value::TextJustifyRight
        | Value::TextJustify
        | Value::TextJustifyStart
        | Value::TextJustifyEnd
        | Value::LineHeightRelative(_)
        | Value::LineHeightPixels(_)
        | Value::LineBreakWordBoundary
        | Value::LineBreakNoWrap
        | Value::LineBreakAnyCharacter
        | Value::LineBreakWordOrCharacter
        | Value::FontWeight(_)
        | Value::FontStyleNormal
        | Value::FontStyleItalic
        | Value::RadiusFull => unreachable!("value is emitted separately"),
    }
}

fn radius_tokens(rules: &[bevywind_core::StyleRule]) -> Option<TokenStream2> {
    let properties = [
        Property::BorderRadiusTopLeft,
        Property::BorderRadiusTopRight,
        Property::BorderRadiusBottomRight,
        Property::BorderRadiusBottomLeft,
    ];
    if !rules.iter().any(|rule| properties.contains(&rule.property)) {
        return None;
    }
    let values = properties.map(|property| {
        rules
            .iter()
            .find(|rule| rule.property == property)
            .map_or_else(
                || quote! { ::bevy::ui::CornerRadius::ZERO },
                |rule| radius_value_tokens(rule.value),
            )
    });
    let [top_left, top_right, bottom_right, bottom_left] = values;
    Some(quote! {
        border_radius: {
            ::bevy::ui::BorderRadius::new(
                #top_left,
                #top_right,
                #bottom_right,
                #bottom_left,
            )
        },
    })
}

fn radius_value_tokens(value: Value) -> TokenStream2 {
    match value {
        Value::RadiusFull => quote! { ::bevy::ui::CornerRadius::MAX },
        Value::RadiusPixels(value) => {
            quote! { ::bevy::ui::CornerRadius::circular(::bevy::ui::px(#value)) }
        }
        Value::RadiusPercent(value) => {
            quote! { ::bevy::ui::CornerRadius::circular(::bevy::ui::percent(#value)) }
        }
        Value::RadiusViewportWidth(value) => {
            quote! { ::bevy::ui::CornerRadius::circular(::bevy::ui::vw(#value)) }
        }
        Value::RadiusViewportHeight(value) => {
            quote! { ::bevy::ui::CornerRadius::circular(::bevy::ui::vh(#value)) }
        }
        _ => unreachable!("only radius values are emitted here"),
    }
}

fn rect_tokens(
    rules: &[bevywind_core::StyleRule],
    properties: [Property; 4],
    field: TokenStream2,
) -> Option<TokenStream2> {
    if !rules.iter().any(|rule| properties.contains(&rule.property)) {
        return None;
    }

    let values = properties.map(|property| {
        rules
            .iter()
            .find(|rule| rule.property == property)
            .map_or_else(
                || quote! { ::bevy::ui::Val::ZERO },
                |rule| value_tokens(rule.value),
            )
    });
    let [left, right, top, bottom] = values;
    Some(quote! {
        #field: { ::bevy::ui::UiRect::new(#left, #right, #top, #bottom) },
    })
}

fn color_tokens(value: Value) -> TokenStream2 {
    let Value::BorderColor(red, green, blue, alpha) = value else {
        unreachable!("only border colors are emitted here")
    };
    let red = red as f32 / 255.0;
    let green = green as f32 / 255.0;
    let blue = blue as f32 / 255.0;
    let alpha = alpha as f32 / 255.0;
    quote! { Color::srgba(#red, #green, #blue, #alpha) }
}

fn color_rect_tokens(
    rules: &[bevywind_core::StyleRule],
    properties: [Property; 4],
) -> Option<TokenStream2> {
    if !rules.iter().any(|rule| properties.contains(&rule.property)) {
        return None;
    }
    let values = properties.map(|property| {
        rules
            .iter()
            .find(|rule| rule.property == property)
            .map_or_else(
                || quote! { ::bevy::color::Color::NONE },
                |rule| color_tokens(rule.value),
            )
    });
    let [left, right, top, bottom] = values;
    Some(quote! {
        ::bevy::ui::BorderColor {
            left: #left,
            right: #right,
            top: #top,
            bottom: #bottom,
        }
    })
}

fn compile_error(message: impl Into<String>) -> TokenStream {
    let message = syn::LitStr::new(&message.into(), proc_macro2::Span::call_site());
    quote! {
        compile_error!(#message);
    }
    .into()
}
