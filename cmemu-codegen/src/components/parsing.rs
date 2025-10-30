use crate::components::SKIP_ARGS_IN_HANDLERS;
use crate::components::conf::{ComponentConf, ComponentDesc, HandlerDesc, HandlerKind};
use quote::quote;
use std::fs;

pub(super) fn parse_component(c: &ComponentConf) -> ComponentDesc {
    let src = fs::read_to_string(&c.file_path)
        .unwrap_or_else(|_| panic!("Unable to read file: {}", c.file_path));
    match syn::parse_file(&src) {
        Ok(syntax) => ComponentDesc {
            config: (*c).clone(),
            is_special: false,
            imports: parse_imports(&syntax),
            handlers: parse_handlers(c, &syntax),
        },
        // HAXXX: syn crate doesn't provide error span, so we let the rustc
        //  \o/   to parse the code again and provide better error message
        //  /o\   https://github.com/dtolnay/syn/issues/641
        // note: by emitting empty file we make sure that other components can't use this one
        Err(_) => ComponentDesc {
            config: (*c).clone(),
            is_special: false,
            imports: Vec::new(),
            handlers: Vec::new(),
        },
    }
}

fn parse_imports(syntax: &syn::File) -> Vec<String> {
    syntax
        .items
        .iter()
        .filter_map(|e| match e {
            syn::Item::Use(item_use) => {
                let mut item_use_clone = item_use.clone();
                item_use_clone.attrs = item_use
                    .attrs
                    .iter()
                    .filter(|a| !a.path().is_ident("proxy_use"))
                    .cloned()
                    .collect();
                if item_use.attrs.len() == item_use_clone.attrs.len() {
                    None
                } else {
                    Some(quote! { #item_use_clone }.to_string())
                }
            }
            _ => None,
        })
        .collect()
}

// todo optional check for #[handler] outside #[component_impl(comp)];
//      if not intentional, it won't compile anyway (the latter propagates comp into the former)
fn parse_handlers(comp_conf: &ComponentConf, syntax: &syn::File) -> Vec<HandlerDesc> {
    syntax
    .items
    .iter()
    .filter_map(|e| match e {
        syn::Item::Impl(item_impl) => {
            // check if this is #[component_impl(...)]
            let mut found = false;
            for attr in &item_impl.attrs {
                if attr.path().is_ident("component_impl") {
                    assert!(
                        !found,
                        "Only one component_impl allowed per impl item, in file {}",
                        comp_conf.file_path,
                    );
                    let field_name: syn::Ident = syn::parse_str(&comp_conf.field_name)
                        .unwrap_or_else(|err| panic!(
                            "Invalid component identifier \"{}\", error msg {}, in file {}",
                            comp_conf.field_name,
                            err,
                            comp_conf.file_path,
                        ));
                    // note: to_string() on TokenStream might change output,
                    //       so we don't want to compare it to our string literal
                    //       instead, we create corresponding TokenStream and stringify it
                    let file_arg: syn::Ident = attr.parse_args().unwrap();
                    assert_eq!(
                        file_arg,
                        field_name,
                        "Impl for invalid component, in file {}",
                        comp_conf.file_path,
                    );
                    found = true;
                }
            }

            let self_ty = &item_impl.self_ty;
            let self_ty = quote! { #self_ty }.to_string();
            let comp_ty = comp_conf.mod_path.split("::").last().unwrap();

            match (found, self_ty == comp_ty) {
                (true, true) => {}
                (true, false) => panic!(
                    "Only one component per file is allowed! Found {:?} and {:?} in file {}",
                    self_ty, comp_ty, comp_conf.file_path,
                ), // poor man's type validation; should be good enough
                (false, false) => return None,
                (false, true) => panic!(
                    "Every impl block for the component must be annotated with #[component_impl(...)], in file {}",
                    comp_conf.file_path,
                ),
            }

            Some(item_impl.items.iter().filter_map(move |e| match e {
                syn::ImplItem::Fn(method) => {
                    parse_handlers_filter_map_method(method, &self_ty)
                }
                _ => None,
            }))
        }
        _ => None,
    })
    .flatten()
    .collect()
}

fn parse_handlers_filter_map_method(
    method: &syn::ImplItemFn,
    self_ty: &str,
) -> Option<HandlerDesc> {
    // note: earlier attributes can alter this via proc macro, so compiler will see something else, but we assume everyone plays nicely
    if !method.attrs.iter().any(|a| a.path().is_ident("handler")) {
        return None;
    }

    let method_name = &method.sig.ident;
    assert_ne!(
        method_name, "tick",
        "tick() is a special method and mustn't be a #[handler], found in type {self_ty}",
    );
    assert!(
        method.sig.inputs.len() >= SKIP_ARGS_IN_HANDLERS,
        "Handler should take at least self and context! In {self_ty}::{method_name}",
    );
    Some(HandlerDesc {
            name: method_name.to_string(),
            allow_dead_code: method.attrs.iter().any(|a| {
                // poor man's check; in theory this shouldn't be even needed
                a.path().is_ident("allow")
                  && a.parse_args::<syn::Ident>().is_ok_and(|ident| ident == "dead_code")
            }),
            explicit_delay: false,
            pass_components: false,
            generator: HandlerKind::Auto,
            fields: method.sig.inputs.iter().skip(SKIP_ARGS_IN_HANDLERS).enumerate().map(|(n, arg)| match arg {
                syn::FnArg::Typed(syn::PatType { pat, ty, .. }) => {
                    match &**if let syn::Type::Paren(syn::TypeParen{elem, ..}) | syn::Type::Group(syn::TypeGroup{elem, ..}) = &**ty {elem} else {ty} {
                        syn::Type::Array{..} | syn::Type::BareFn{..} | syn::Type::Path{..} | syn::Type::Tuple{..} |
                        syn::Type::Reference(syn::TypeReference{mutability: None, lifetime: Some(syn::Lifetime{..}), ..}) => {
                            let field_name = if let syn::Pat::Ident(pat_ident) = &**pat {
                                pat_ident.ident.to_string()
                            } else {
                                format!("pattern_arg{n}")
                            };
                            (field_name, quote!{ #ty }.to_string())
                        }
                        syn::Type::Reference(..) => panic!("Invalid handler argument in {self_ty}::{method_name}: \
                            extra arguments should be received by moved value, not references as previously"),
                        _ => panic!("Invalid handler argument in {self_ty}::{method_name}: extra arguments should be received by moved value")
                    }
                }
                syn::FnArg::Receiver(_) => panic!("Invalid untyped argument in {self_ty}::{method_name}"),
            }).collect()
        })
}
