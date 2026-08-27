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
            Property::Height => quote! { height },
            Property::Width => quote! { width },
            Property::MinHeight => quote! { min_height },
            Property::MinWidth => quote! { min_width },
            Property::MaxHeight => quote! { max_height },
            Property::MaxWidth => quote! { max_width },
        };
        let value = match rule.value {
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

fn compile_error(message: impl Into<String>) -> TokenStream {
    let message = syn::LitStr::new(&message.into(), proc_macro2::Span::call_site());
    quote! {
        compile_error!(#message);
    }
    .into()
}
