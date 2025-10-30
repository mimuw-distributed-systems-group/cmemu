use proc_macro2::TokenStream;

/// Mark some imports to be pushed to proxies.
///
/// Idea:
/// build script reuses exactly same imports for proxy
#[must_use]
pub fn proxy_use(attr: TokenStream, item: TokenStream) -> TokenStream {
    if attr.is_empty() {
        item
    } else if attr.to_string() == "proxy_only" {
        // TODO: consider instead: #[proxy_use(as crate::component::ABC::Type)] on item definition
        TokenStream::new()
    } else {
        syn::Error::new_spanned(
            attr,
            "Only #[proxy_use] and #[proxy_use(proxy_only)] are valid",
        )
        .to_compile_error()
    }
}
