use cmemu_codegen::proc::{component, decode_instr};
use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn proxy_use(attr: TokenStream, item: TokenStream) -> TokenStream {
    component::proxy_use(attr.into(), item.into()).into()
}

#[proc_macro_attribute]
pub fn component_impl(attr: TokenStream, item: TokenStream) -> TokenStream {
    let item = syn::parse_macro_input!(item as syn::ItemImpl);
    component::component_impl(attr.into(), item).into()
}

#[proc_macro_attribute]
pub fn handler(attr: TokenStream, item: TokenStream) -> TokenStream {
    component::handler(attr.into(), item.into()).into()
}

#[proc_macro_derive(TickComponent, attributes(flop, subcomponent, subcomponent_1to1))]
pub fn derive_tick_component(item: TokenStream) -> TokenStream {
    let item = syn::parse_macro_input!(item as syn::DeriveInput);
    component::derive_tick_component(item).into()
}

#[proc_macro_derive(TickComponentExtra)]
pub fn derive_tick_component_extra(item: TokenStream) -> TokenStream {
    let item = syn::parse_macro_input!(item as syn::DeriveInput);
    component::derive_tick_component_extra(item).into()
}
#[proc_macro_derive(Subcomponent, attributes(flop, subcomponent, subcomponent_1to1))]
pub fn derive_subomponent(item: TokenStream) -> TokenStream {
    let item = syn::parse_macro_input!(item as syn::DeriveInput);
    component::derive_subcomponent(item).into()
}

#[proc_macro_derive(MainComponent)]
pub fn derive_component(item: TokenStream) -> TokenStream {
    let item = syn::parse_macro_input!(item as syn::DeriveInput);
    component::derive_component(item).into()
}

#[proc_macro_derive(SkippableClockTreeNode, attributes(skippable_if_disableable))]
pub fn derive_skippable_clock_tree_node(item: TokenStream) -> TokenStream {
    let item = syn::parse_macro_input!(item as syn::DeriveInput);
    component::derive_skippable_clock_tree_node(item).into()
}

// TODO: remove disableable (and its uses (also in derive_disableable_component()))
#[proc_macro_derive(DisableableComponent, attributes(disableable, disableable_ignore))]
pub fn derive_disableable_component(item: TokenStream) -> TokenStream {
    let item = syn::parse_macro_input!(item as syn::DeriveInput);
    component::derive_disableable_component(item).into()
}

#[proc_macro_attribute]
pub fn decode_instr(attr: TokenStream, item: TokenStream) -> TokenStream {
    let item = syn::parse_macro_input!(item as syn::ItemFn);
    decode_instr::decode_instr(attr.into(), item).into()
}
