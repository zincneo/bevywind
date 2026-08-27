use proc_macro2::TokenStream;
use quote::quote;

/// Parses width and height utilities from the style class string.
pub(super) fn parse(class: &str) -> Result<TokenStream, String> {
    let Some((name, value)) = class.split_once('-') else {
        return Err(format!("invalid dimension utility `{class}`"));
    };

    let field_value = match value {
        "full" => quote! { { ::bevy::ui::percent(100) } },
        value if let Some(number) = value.strip_suffix('%') => {
            let percent = parse_number(number, value)?;
            quote! { { ::bevy::ui::percent(#percent) } }
        }
        value if let Some(number) = value.strip_suffix("px") => {
            let pixels = parse_number(number, value)?;
            quote! { { ::bevy::ui::px(#pixels) } }
        }
        value if let Some(number) = value.strip_suffix('w') => {
            let viewport_width = parse_number(number, value)?;
            quote! { { ::bevy::ui::vw(#viewport_width) } }
        }
        value if let Some(number) = value.strip_suffix('h') => {
            let viewport_height = parse_number(number, value)?;
            quote! { { ::bevy::ui::vh(#viewport_height) } }
        }
        _ => {
            return Err(format!(
                "unsupported dimension utility `{class}`; expected `h-full`, `h-10px`, `h-10%`, `h-10w`, or `h-10h`"
            ));
        }
    };

    match name {
        "h" => Ok(quote! { height: #field_value }),
        "w" => Ok(quote! { width: #field_value }),
        _ => Err(format!("unsupported dimension utility `{class}`")),
    }
}

fn parse_number(number: &str, value: &str) -> Result<u16, String> {
    if number.is_empty() || !number.bytes().all(|byte| byte.is_ascii_digit()) {
        return Err(format!(
            "expected a non-negative integer dimension value, got `{value}`"
        ));
    }

    number
        .parse::<u16>()
        .map_err(|_| format!("dimension value `{value}` does not fit in u16"))
}
