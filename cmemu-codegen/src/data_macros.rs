//! Callback-macro API
//!
//! Because Rust macros are expanded from the outtermost if one macro wants to postprocess output
//! of another, the only way is to invert the order and make the inner macro evaluate first
//! to an invocation of the outter macro: i.e., to implement `postprocess_inner!(inner!(...))`, we
//! need to have `inner!(<postprocess_inner_callback>, ...)` to expand to `postprocess_inner_callback!(result)`.
//!
//! To make an interoperable ecosystem, we propose function-like macros (decl and proc) to accept a prefix:
//! `@callback($($cb:ident)::+$(, $extra:tt)?); ` before the rest of arguments, which will cause
//! the macro to wrap its output with `$($cb)::+!($($extra,)?` and `)`. For example, calling
//! `revert_tokens!(@callback(stringify, ()); tokens to reverse)` would expand to:
//! `stringify!((), reverse to tokens)`.
//!
//! The *extra* argument part is an opaque optional argument, which enables various kinds of composition:
//! for instance, a trampoline macro could evaluate multiple macros in order.

use proc_macro2::TokenStream;
use quote::{ToTokens, format_ident, quote, quote_spanned};
use syn::Ident;
use syn::spanned::Spanned;

pub(crate) const INJECTED_MACROS: &str = include_str!("data_macros/injected_macros.rs");

pub(crate) fn build_export_tokens_macro(ident: Ident, tokens: impl ToTokens) -> TokenStream {
    quote_spanned! {tokens.span()=>
        macro_rules! #ident {
            (@callback($($cb:ident)::+$(, $extra:tt)?); ) => {
                $($cb)::+!($($extra,)?
                    #tokens
                )
            };
        }
        pub(crate) use #ident;
    }
}

pub(crate) fn export_paths_list(
    name: impl AsRef<str>,
    paths: impl Iterator<Item = impl ToTokens>,
) -> TokenStream {
    let name = format_ident!("{}", name.as_ref());
    let tokens = quote! {
        #(#paths),*
    };
    build_export_tokens_macro(name, tokens)
}
