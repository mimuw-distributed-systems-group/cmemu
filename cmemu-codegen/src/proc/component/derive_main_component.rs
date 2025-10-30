// TODO: decide on the name
use proc_macro2::TokenStream;
use quote::quote;

/// Used by `#[derive(MainComponent)]` to generate multiple impls
///
/// Idea:
/// implement `ClockTreeNode` by forwarding to user-provided impl of `MainComponent`
/// This is the location that triggers the `TickComponent` recursive traversal!
#[must_use]
pub fn derive_component(item: syn::DeriveInput) -> TokenStream {
    let main_comp = derive_clock_tree_node_expand(&item);
    let sub_of_main = super::derive_subcomponent(item);
    // TODO: Require TickComponent here right away?
    quote! {
        #sub_of_main
        #main_comp
    }
}

#[must_use]
pub fn derive_skippable_clock_tree_node(item: syn::DeriveInput) -> TokenStream {
    let has_skippable_if_disableable = item
        .attrs
        .iter()
        .any(|a| a.path().is_ident("skippable_if_disableable"));
    if has_skippable_if_disableable {
        derive_simply_skippable_tick_node(item)
    } else {
        derive_not_skippable_tick_node(item)
    }
}

/// A `SkippableClockTreeNode` impl that always returns it cannot be skipped
fn derive_not_skippable_tick_node(item: syn::DeriveInput) -> TokenStream {
    let struct_name = &item.ident;

    let (impl_generics, ty_generics, where_clause) = item.generics.split_for_impl();
    quote! {
        impl #impl_generics crate::engine::SkippableClockTreeNode for #struct_name #ty_generics #where_clause {
        }
    }
}

/// A `SkippableClockTreeNode` impl that returns INF if `DisableableComponent::can_be_disabled_now`,
/// and doesn't do anything to emulate.
fn derive_simply_skippable_tick_node(item: syn::DeriveInput) -> TokenStream {
    let struct_name = &item.ident;

    // TODO: better error on missing derives?
    // Should we condition it on ctx.get_energy_state_of(Self::id())?
    // Without it, we can handle events on disabled components... if that makes any sense.
    let (impl_generics, ty_generics, where_clause) = item.generics.split_for_impl();
    quote! {
        impl #impl_generics crate::engine::SkippableClockTreeNode for #struct_name #ty_generics #where_clause {
            fn max_cycles_to_skip(
                comp: &mut <Self as crate::engine::Subcomponent>::Component,
                _ctx: &mut crate::engine::Context,
                _parent: <Self as crate::engine::EnergyNode>::IdSpace,
                _extra: &mut <Self as crate::engine::EnergyNode>::Extra,
            ) -> u64 {
                if <Self as crate::engine::DisableableComponent>::can_be_disabled_now(comp) {
                    u64::MAX
                } else {
                    0
                }
            }

            /// Move from post-Tock to a post-Tock state `skipped_cycles` later.
            fn emulate_skipped_cycles(
                _comp: &mut <Self as crate::engine::Subcomponent>::Component,
                _ctx: &mut crate::engine::Context,
                _parent: <Self as crate::engine::EnergyNode>::IdSpace,
                _extra: &mut <Self as crate::engine::EnergyNode>::Extra,
                skipped_cycles: u64,
            ) {
                log::trace!(
                    "Component {} skipping {skipped_cycles:?} cycles by ignoring them.",
                    <Self as crate::engine::EnergyNode>::NAME
                );
            }
        }
    }
}

fn derive_clock_tree_node_expand(item: &syn::DeriveInput) -> TokenStream {
    let struct_name = &item.ident;

    // TODO: rewrite direct uses to a trait impl
    let mut generics = item.generics.clone();
    generics
        .make_where_clause()
        .predicates
        .push(syn::parse_quote! {Self: crate::engine::Subcomponent::<Component=Self> + crate::engine::TickComponent});
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    let clock_tree_id = quote! {crate::build_data::EnergyEntity::Component(crate::build_data::Components::#struct_name)};
    quote! {
        impl #impl_generics crate::engine::MainComponent for #struct_name #ty_generics #where_clause {
        }
        impl #impl_generics crate::engine::EnergyNode for #struct_name #ty_generics #where_clause {
            type Extra = ();
            type IdSpace = crate::build_data::EnergyEntity;
            const NAME: &'static str = stringify!(#struct_name);
            fn id() -> Self::IdSpace {
                #clock_tree_id
            }
        }
        impl #impl_generics crate::engine::ClockTreeNode for #struct_name #ty_generics #where_clause {
            fn tick(comp: &mut Self::Component, ctx: &mut Context, parent: Self::IdSpace, extra: &mut Self::Extra) {
                #[cfg(debug_assertions)]
                TickComponent::tick_assertions_traverse(comp);
                TickComponent::tick_flops_and_extra_traverse(comp);

                comp.tick(ctx);
            }
            fn tock(comp: &mut Self::Component, ctx: &mut Context, parent: Self::IdSpace, extra: &mut Self::Extra) {
                comp.tock(ctx);
            }
        }
        impl #impl_generics crate::engine::PowerNode for #struct_name #ty_generics #where_clause {
            fn get_power_state(
                comp: &<Self as crate::engine::Subcomponent>::Component,
                ctx: &crate::engine::Context,) -> crate::engine::PowerMode
            {
                ctx.get_energy_state_of(#clock_tree_id)
            }

            fn prepare_to_disable(
                comp: &mut <Self as crate::engine::Subcomponent>::Component,
                ctx: &mut crate::engine::Context,
                parent: <Self as crate::engine::EnergyNode>::IdSpace,
                extra: &mut <Self as crate::engine::EnergyNode>::Extra,
                mode: crate::engine::PowerMode,
            ) -> crate::engine::PowerMode {
                if <Self as crate::engine::DisableableComponent>::can_be_disabled_now(comp) {
                    mode
                } else {
                    crate::engine::PowerMode::Active
                }
            }
            fn set_power_state(
                _comp: &mut <Self as crate::engine::Subcomponent>::Component,
                ctx: &mut crate::engine::Context,
                _parent: <Self as crate::engine::EnergyNode>::IdSpace,
                _extra: &mut <Self as crate::engine::EnergyNode>::Extra,
                mode: crate::engine::PowerMode,
            ) {
                ctx.set_energy_state_of(#clock_tree_id, mode);
            }
        }
    }
}
