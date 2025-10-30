use proc_macro2::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::parse_quote;
use syn::visit_mut::{self, VisitMut};

enum DeriveAfterParse {
    TickComponent,
    Subcomponent,
}

/// Derives `TickComponent`
///
/// Idea:
/// - generates `trait TickComponent` implementation for automatic ticking subcomponents and flops
///   and need Subcomponent
#[must_use]
pub fn derive_tick_component(item: syn::DeriveInput) -> TokenStream {
    derive_expand(&item, DeriveAfterParse::TickComponent).unwrap_or_else(|e| e.to_compile_error())
}

/// Derives `Subcomponent`
///
/// Idea:
/// - generates `trait Subcomponent` implementation for subcomponents if struct name is given
#[must_use]
pub fn derive_subcomponent(item: syn::DeriveInput) -> TokenStream {
    derive_expand(&item, DeriveAfterParse::Subcomponent).unwrap_or_else(|e| e.to_compile_error())
}

// We use a single large method and an enum,
// because it is simpler to have the parsing code wrote once,
// and then auto-extract a subprocedure.
#[allow(clippy::items_after_statements, clippy::too_many_lines)]
fn derive_expand(
    item: &syn::DeriveInput,
    to_generate: DeriveAfterParse,
) -> syn::Result<TokenStream> {
    let struct_name = &item.ident;
    let fields = match &item.data {
        syn::Data::Struct(struct_data) => &struct_data.fields,
        _ => {
            return Err(syn::Error::new_spanned(
                item,
                "derive(TickComponent) is implemented only for structs now (does anything else make sense?)",
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
    let mut flop_field_names_str = vec![];
    let mut subcomp_field_names = vec![];
    let mut subcomp_field_types = vec![];
    let mut subcomp_impl_trait_struct_names = vec![];

    args_to_names(
        args,
        &mut flop_field_names,
        &mut flop_field_names_str,
        &mut subcomp_field_names,
        &mut subcomp_field_types,
        &mut subcomp_impl_trait_struct_names,
    )?;

    let (impl_generics, ty_generics, where_clause) = item.generics.split_for_impl();
    let type_path: syn::Path = parse_quote! {#struct_name #ty_generics};
    struct SelfFinder(syn::Path);
    impl VisitMut for SelfFinder {
        fn visit_path_mut(&mut self, p: &mut syn::Path) {
            if p.is_ident("Self") {
                *p = self.0.clone();
                return;
            }
            visit_mut::visit_path_mut(self, p);
        }
    }

    let where_clause = where_clause.cloned().map(|mut w| {
        SelfFinder(type_path).visit_where_clause_mut(&mut w);
        w
    });

    Ok(match to_generate {
        DeriveAfterParse::TickComponent => generate_tick_component(
            struct_name,
            &mut flop_field_names,
            &mut flop_field_names_str,
            &mut subcomp_field_names,
            impl_generics,
            ty_generics,
            where_clause,
        ),
        DeriveAfterParse::Subcomponent => generate_subcomponent(
            item,
            struct_name,
            &mut subcomp_field_names,
            &mut subcomp_field_types,
            &mut subcomp_impl_trait_struct_names,
            impl_generics,
            ty_generics,
            where_clause,
        ),
    })
}

#[allow(clippy::too_many_arguments)]
fn generate_subcomponent(
    item: &syn::DeriveInput,
    struct_name: &syn::Ident,
    subcomp_field_names: &mut Vec<&syn::Ident>,
    subcomp_field_types: &mut Vec<&syn::Type>,
    subcomp_impl_trait_struct_names: &mut Vec<Option<IdentWithVis>>,
    impl_generics: syn::ImplGenerics,
    ty_generics: syn::TypeGenerics,
    where_clause: Option<syn::WhereClause>,
) -> TokenStream {
    let mut result = quote! {};
    let phantom_generics: Vec<_> = item
        .generics
        .type_params()
        .map(|tp| tp.ident.clone())
        .collect();
    let is_generic = !phantom_generics.is_empty();
    let has_sub_1to1 = item
        .attrs
        .iter()
        .any(|a| a.path().is_ident("subcomponent_1to1"));
    let is_toplevel = !is_generic && !has_sub_1to1;

    // Make sure main types implement Subcomponent:
    // - root type is implemented below
    // - for generics over <SC> (i.e. T<SC>) do impl<SC: Subcomponent> Subcomponent for T<SC>
    // - otherwise, child subcomponents are either generic (will get the above) or 1-1 (will get one here)
    if is_toplevel {
        result.extend(quote! {
        impl #impl_generics crate::engine::Subcomponent for #struct_name #ty_generics #where_clause {
            type Component = Self;
            type Member = Self;

            fn component_to_member(component: &Self::Component) -> &Self::Member {
                component
            }

            fn component_to_member_mut(component: &mut Self::Component) -> &mut Self::Member {
                component
            }
        }
        });
    } else if is_generic && phantom_generics.first().unwrap() == "SC" {
        let sc = phantom_generics.first();
        result.extend(quote! {
        impl #impl_generics crate::engine::Subcomponent for #struct_name #ty_generics #where_clause {
            type Component = <#sc as crate::engine::Subcomponent>::Component ;
            type Member = Self;

            fn component_to_member(component: &Self::Component) -> &Self::Member {
            <#sc as crate::engine::Subcomponent>::component_to_member(component)
            }

            fn component_to_member_mut(component: &mut Self::Component) -> &mut Self::Member {
            <#sc as crate::engine::Subcomponent>::component_to_member_mut(component)
            }
        }
        });
    }

    for ((f_name, f_ty), its_def) in subcomp_field_names
        .iter()
        .zip(subcomp_field_types)
        .zip(subcomp_impl_trait_struct_names)
    {
        if let Some(IdentWithVis {
            vis: its_vis,
            ident: its_name,
        }) = its_def
        {
            if match *f_ty {
                syn::Type::Path(p) => p.path.is_ident(its_name),
                _ => false,
            } {
                // Special case for 1-to-1 subcomponents -> just don't generate the marker type,
                // and the rest will work!
                // TODO: consider impling full Path-ed types
            } else if is_generic {
                // We implement default here, to make it possible to derive Default
                result.extend(quote! {
                #its_vis struct #its_name #ty_generics(#(::std::marker::PhantomData<#phantom_generics>),*);
                impl #impl_generics Default for #its_name #ty_generics {
                        fn default() -> Self {Self(#(::std::marker::PhantomData::<#phantom_generics>),*)}
                    }
                impl #impl_generics crate::engine::PureSubcomponentMarker for #its_name #ty_generics {}
                    });
            } else {
                result.extend(quote! {
                #[derive(Default)]
                #its_vis struct #its_name;
                impl crate::engine::PureSubcomponentMarker for #its_name {}
                    });
            }
            let sc = if is_generic && phantom_generics.first().unwrap() == "SC" {
                let id = phantom_generics.first().unwrap();
                quote! {#id}
            } else {
                quote! { #struct_name #ty_generics}
            };
            result.extend(quote! {
                impl #impl_generics crate::engine::Subcomponent for #its_name #ty_generics #where_clause {
                    type Component = <#sc as crate::engine::Subcomponent>::Component ;
                    type Member = #f_ty;

                    fn component_to_member(component: &Self::Component) -> &Self::Member {
                        &<#sc as crate::engine::Subcomponent>::component_to_member(component).#f_name
                    }

                    fn component_to_member_mut(component: &mut Self::Component) -> &mut Self::Member {
                        &mut <#sc as crate::engine::Subcomponent>::component_to_member_mut(component).#f_name
                    }
                }
            });
        }
    }
    result
}

fn generate_tick_component(
    struct_name: &syn::Ident,
    flop_field_names: &mut Vec<&syn::Ident>,
    flop_field_names_str: &mut Vec<String>,
    subcomp_field_names: &mut Vec<&syn::Ident>,
    impl_generics: syn::ImplGenerics,
    ty_generics: syn::TypeGenerics,
    where_clause: Option<syn::WhereClause>,
) -> TokenStream {
    let result = quote! {
        impl #impl_generics TickComponent for #struct_name #ty_generics #where_clause {
            #[cfg(debug_assertions)]
            fn tick_assertions_traverse(&self) {
                // TODO: Maybe inform a user, that there is a chance that the following messages
                //       might be misleading. Currently, however, the misleading part is
                //       failing on a random assertion of a complex components and hiding
                //       the broader picture.
                // We assert unwind safe, because we're going to panic anyway and at worst the
                // messages would be wrong. There is no safety issues (see the reference).
                let err = [
                    #(
                    std::panic::catch_unwind(std::panic::AssertUnwindSafe(||
                        TickComponent::tick_assertions_traverse(&self.#subcomp_field_names)
                    )).err(),
                    )*

                    std::panic::catch_unwind(std::panic::AssertUnwindSafe(||
                        TickComponentExtra::tick_assertions(self)
                    )).err()
                ].into_iter().flatten().last();
                if let Some(err) = err {
                    std::panic::resume_unwind(err);
                }
            }

            fn tick_flops_and_extra_traverse(&mut self) {
                #(TickComponent::tick_flops_and_extra_traverse(&mut self.#subcomp_field_names);)*

                TickComponent::tick_flops(self);
                TickComponentExtra::tick_extra(self);
            }

            fn tick_flops(&mut self) {
                #(self.#flop_field_names.tick(#[cfg(debug_assertions)] #flop_field_names_str);)*
            }
        }
    };
    result
}

fn args_to_names<'a>(
    args: impl Iterator<Item = &'a syn::Field>,
    flop_field_names: &mut Vec<&'a syn::Ident>,
    flop_field_names_str: &mut Vec<String>,
    subcomp_field_names: &mut Vec<&'a syn::Ident>,
    subcomp_field_types: &mut Vec<&'a syn::Type>,
    subcomp_impl_trait_struct_names: &mut Vec<Option<IdentWithVis>>,
) -> syn::Result<()> {
    // error handling combined with filtering doesn't look like too nice
    for f in args {
        // full scan for checking validity of every attribute
        let mut found = false;
        let mut mark_found_flag = |span_tokens| {
            if found {
                Err(syn::Error::new_spanned(
                    span_tokens,
                    "there should be at most one #[flop] or #[subcomponent(...)] per field (they are mutually exclusive)",
                ))
            } else {
                found = true;
                Ok(())
            }
        };
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
                flop_field_names_str.push(field_ident.to_string());
            } else if attr.path().is_ident("subcomponent") {
                mark_found_flag(attr)?;

                let field_ident = f
                    .ident
                    .as_ref()
                    .expect("Structure with named fields must have named fields");
                let struct_name = if let syn::Meta::Path(_) = attr.meta {
                    None
                } else {
                    Some(attr.parse_args()?)
                };

                subcomp_field_names.push(field_ident);
                let unwrapped_type = match &f.ty {
                    syn::Type::Group(syn::TypeGroup { elem, .. }) => elem,
                    x => x,
                };
                subcomp_field_types.push(unwrapped_type);
                subcomp_impl_trait_struct_names.push(struct_name);
            }
        }
    }

    Ok(())
}

struct IdentWithVis {
    vis: syn::Visibility,
    ident: syn::Ident,
}

impl Parse for IdentWithVis {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(IdentWithVis {
            vis: input.parse()?,
            ident: input.parse()?,
        })
    }
}
