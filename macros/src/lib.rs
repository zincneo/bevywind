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
            "bstyle! accepts style tokens, not strings; use style_runtime for runtime strings",
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
                Property::MarginLeft
                | Property::MarginRight
                | Property::MarginTop
                | Property::MarginBottom
                | Property::PaddingLeft
                | Property::PaddingRight
                | Property::PaddingTop
                | Property::PaddingBottom => return None,
                Property::BackgroundColor => return None,
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
    let node = quote! {
        Node {
            #(#fields,)*
            #margin
            #padding
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

    quote! {
        {
            use ::bevy::color::Color;
            use ::bevy::ui::{BackgroundColor, Node};
            ::bevy::scene::bsn! {
            #node
            #(#backgrounds)*
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
        Value::Background(..) => unreachable!("background colors are emitted separately"),
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

fn compile_error(message: impl Into<String>) -> TokenStream {
    let message = syn::LitStr::new(&message.into(), proc_macro2::Span::call_site());
    quote! {
        compile_error!(#message);
    }
    .into()
}
