mod dimension;

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;

/// Creates a style scene for the current Bevy entity.
///
/// The style string is parsed at compile time and expanded into fields on a
/// `bevy::ui::Node`.
#[proc_macro]
pub fn style(input: TokenStream) -> TokenStream {
    match parse_input(input) {
        Ok(classes) => match parse_classes(&classes) {
            Ok(fields) => quote! {
                ::bevy::scene::bsn! {
                    Node {
                        #(#fields),*
                    }
                }
            }
            .into(),
            Err(message) => compile_error(message),
        },
        Err(message) => compile_error(message),
    }
}

fn parse_input(input: TokenStream) -> Result<String, String> {
    let tokens: TokenStream2 = input.into();
    let literal = syn::parse2::<syn::LitStr>(tokens).map_err(|_| {
        String::from(
            "style! expects a non-empty string literal, for example `style!(\"h-10px\")`",
        )
    })?;
    let classes = literal.value();

    if classes.trim().is_empty() {
        return Err("style! expects at least one style utility".into());
    }

    Ok(classes)
}

fn parse_classes(classes: &str) -> Result<Vec<TokenStream2>, String> {
    classes
        .split_whitespace()
        .map(dimension::parse)
        .collect()
}

fn compile_error(message: String) -> TokenStream {
    let message = syn::LitStr::new(&message, proc_macro2::Span::call_site());
    quote! {
        compile_error!(#message);
    }
    .into()
}
