use proc_macro2::TokenStream;
use quote::quote;

/// Generates `trait DisableableComponent` implementation
///
/// Idea:
/// generates `trait DisableableComponent` implementation making it easier to manage disabling nodes in clock tree without breaking the invariants.
/// Mark a field with `#[disableable_ignore]` to not call it recursively.
#[must_use]
pub fn derive_disableable_component(item: syn::DeriveInput) -> TokenStream {
    derive_disableable_component_expand(&item).unwrap_or_else(|e| e.to_compile_error())
}

// TODO: make it sensible (that is use a conjunction of all members implementing DisableableComponent)
fn derive_disableable_component_expand(item: &syn::DeriveInput) -> syn::Result<TokenStream> {
    let struct_name = &item.ident;
    let fields = match &item.data {
        syn::Data::Struct(struct_data) => &struct_data.fields,
        _ => {
            return Err(syn::Error::new_spanned(
                item,
                "derive(DisableableComponent) is implemented only for structs now (does anything else make sense?)",
            ));
        }
    };

    let args = match fields {
        syn::Fields::Named(v) => v.named.iter(),
        syn::Fields::Unnamed(_v) => {
            return Err(syn::Error::new_spanned(
                item,
                "derive(TickComponent) is not implemented for tuple structs (does it make sense?)",
            ));
        }
        syn::Fields::Unit => return Ok(quote! {}), // weird, but okay...
    };

    let mut flop_field_names = vec![];
    let mut subcomp_field_names = vec![];
    let mut disableable_field_names = vec![];

    args_to_names(
        args,
        &mut flop_field_names,
        &mut subcomp_field_names,
        &mut disableable_field_names,
    )?;

    // assert!(!flop_field_names.is_empty() || !subcomp_field_names.is_empty() || !disableable_field_names.is_empty());

    let (impl_generics, ty_generics, where_clause) = item.generics.split_for_impl();
    Ok(quote! {
        impl #impl_generics DisableableComponent for #struct_name #ty_generics #where_clause {
            fn can_be_disabled_now(&self) -> bool {
                let result = true
                    #(
                        && self.#flop_field_names.is_empty()
                    )*
                    #(
                        && self.#subcomp_field_names.can_be_disabled_now()
                    )*
                    #(
                        && self.#disableable_field_names.can_be_disabled_now()
                    )*;
                if *crate::confeature::cm_logs::DISABLEABLE_IMPL && !result {
                    log::warn!("{} cannot be disabled now!", stringify!(#struct_name));
                }
                result
            }
        }
    })
}

// TODO: reduce copypaste from derive_tick_component.rs
fn args_to_names<'a>(
    args: impl Iterator<Item = &'a syn::Field>,
    flop_field_names: &mut Vec<&'a syn::Ident>,
    subcomp_field_names: &mut Vec<&'a syn::Ident>,
    disableable_field_names: &mut Vec<&'a syn::Ident>,
) -> syn::Result<()> {
    // error handling combined with filtering doesn't look like too nice
    for f in args {
        // full scan for checking validity of every attribute
        let mut found = false;
        let mut mark_found_flag = |span_tokens| {
            if found {
                Err(syn::Error::new_spanned(
                    span_tokens,
                    "there should be at most one #[flop] or #[subcomponent(...)] or #[disableable] per field (they are mutually exclusive)",
                ))
            } else {
                found = true;
                Ok(())
            }
        };
        if f.attrs
            .iter()
            .any(|a| a.path().is_ident("disableable_ignore"))
        {
            continue;
        }
        for attr in &f.attrs {
            if attr.path().is_ident("flop") {
                attr.meta
                    .require_path_only()
                    .map_err(|e| syn::Error::new(e.span(), "#[flop] doesn't take any arguments"))?;
                mark_found_flag(attr)?;

                let field_ident = f
                    .ident
                    .as_ref()
                    .expect("Structure with named fields must have named fields");
                flop_field_names.push(field_ident);
            } else if attr.path().is_ident("subcomponent") {
                mark_found_flag(attr)?;

                let field_ident = f
                    .ident
                    .as_ref()
                    .expect("Structure with named fields must have named fields");
                subcomp_field_names.push(field_ident);
            } else if attr.path().is_ident("disableable") {
                attr.meta.require_path_only().map_err(|e| {
                    syn::Error::new(e.span(), "#[disableable] doesn't take any arguments")
                })?;
                mark_found_flag(attr)?;

                let field_ident = f
                    .ident
                    .as_ref()
                    .expect("Structure with named fields must have named fields");
                disableable_field_names.push(field_ident);
            }
        }
    }

    Ok(())
}
