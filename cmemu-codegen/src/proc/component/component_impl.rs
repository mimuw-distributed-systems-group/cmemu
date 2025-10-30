use super::validate_field_name;
use proc_macro2::TokenStream;
use quote::{ToTokens, format_ident, quote};

/// Mark impl blocks to make magic on them
///
/// Idea:
/// 1) remap `#[handler]` on methods to `#[handler({field_name})]`
/// 2) add implicit `self.tick_flops()` at the beginning of `tick()` body
#[must_use]
pub fn component_impl(attr: TokenStream, item: syn::ItemImpl) -> TokenStream {
    let field_name = attr.to_string();
    if !validate_field_name(&field_name) {
        return syn::Error::new_spanned(attr, format!("Invalid field name: {field_name}"))
            .to_compile_error();
    }

    component_impl_expand(&field_name, item).unwrap_or_else(|e| e.to_compile_error())
}

fn component_impl_expand(field_name: &str, mut item: syn::ItemImpl) -> syn::Result<TokenStream> {
    // TODO: we don't need the large attribute anymore
    for item in &mut item.items {
        if let syn::ImplItem::Fn(method) = item {
            for attr in &mut method.attrs {
                if attr.path().is_ident("handler") {
                    attr.meta.require_path_only().map_err(|e| {
                        syn::Error::new(e.span(), "You mustn't specify any tokens for #[handler]")
                    })?;
                    attr.meta = syn::Meta::List(syn::MetaList {
                        path: attr.path().clone(),
                        delimiter: syn::MacroDelimiter::Paren(Default::default()),
                        tokens: format_ident!("{}", field_name).to_token_stream(),
                    });
                }
            }
        }
    }
    Ok(quote! { #item })
}
