use proc_macro2::TokenStream;
use quote::quote;

/// Derives `TickComponentExtra`
///
/// Idea:
/// generates empty (default) implementation of `trait TickComponentExtra`, so there's no need to do it manually if it's not useful
#[must_use]
pub fn derive_tick_component_extra(item: syn::DeriveInput) -> TokenStream {
    derive_tick_component_extra_expand(&item)
}

fn derive_tick_component_extra_expand(item: &syn::DeriveInput) -> TokenStream {
    let struct_name = &item.ident;

    let (impl_generics, ty_generics, where_clause) = item.generics.split_for_impl();
    quote! {
        impl #impl_generics TickComponentExtra for #struct_name #ty_generics #where_clause {}
    }
}
