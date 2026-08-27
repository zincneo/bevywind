use bevywind_core::{Property, Value, parse_classes};
use proc_macro::TokenStream;
use proc_macro2::{TokenStream as TokenStream2, TokenTree};
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
    if has_spaced_hyphen(&tokens) {
        return compile_error("style utilities must use a continuous hyphenated form, such as `flex-center` or `h-10px`");
    }
    let classes = tokens
        .to_string()
        .replace(" - ", "-")
        .replace("- ", "-")
        .replace(" -", "-");
    if classes.trim().is_empty() {
        return compile_error("bstyle! expects at least one style utility");
    }

    let rules = match parse_classes(&classes) {
        Ok(rules) => rules,
        Err(error) => return compile_error(error.to_string()),
    };

    let fields = rules.iter().map(|rule| {
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
        };
        let value = match rule.value {
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
        };
        quote! { #field: #value }
    });

    quote! {
        ::bevy::scene::bsn! {
            Node {
                #(#fields),*
            }
        }
    }
    .into()
}

fn has_spaced_hyphen(tokens: &TokenStream2) -> bool {
    let tokens: Vec<_> = tokens.clone().into_iter().collect();
    tokens.windows(3).any(|window| {
        let TokenTree::Punct(punct) = &window[1] else {
            return false;
        };
        if punct.as_char() != '-' {
            return false;
        }
        let before = window[0].span().end();
        let hyphen_start = punct.span().start();
        let hyphen_end = punct.span().end();
        let after = window[2].span().start();
        before.line != hyphen_start.line
            || before.column != hyphen_start.column
            || hyphen_end.line != after.line
            || hyphen_end.column != after.column
    })
}

fn compile_error(message: impl Into<String>) -> TokenStream {
    let message = syn::LitStr::new(&message.into(), proc_macro2::Span::call_site());
    quote! {
        compile_error!(#message);
    }
    .into()
}
