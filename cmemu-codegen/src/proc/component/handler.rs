use super::validate_field_name;
use proc_macro2::TokenStream;

/// Handlers are normal methods, but they're registered
#[must_use]
pub fn handler(attr: TokenStream, item: TokenStream) -> TokenStream {
    let field_name = attr.to_string();
    if !validate_field_name(&field_name) {
        return syn::Error::new_spanned(attr, format!("Invalid attr field name: {field_name}"))
            .to_compile_error();
    }

    item
}
