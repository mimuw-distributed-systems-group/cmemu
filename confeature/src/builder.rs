use crate::deser::{ConfigSpec, Configurable, Mode, Namespace};
use crate::optional;
use convert_case::{Boundary, Case, Casing};
use quote::{ToTokens, format_ident, quote};
use serde::Deserialize;
use std::cell::Cell;
use std::cmp::min;
use std::fmt::{Display, Write};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use syn;

/// While max works naturally for options, ``min`` will prefer None. This is to prefer a set value.
fn min_not_none<T: Ord>(a: Option<T>, b: Option<T>) -> Option<T> {
    if a.is_some() && b.is_some() {
        min(a, b)
    } else {
        a.or(b)
    }
}

#[derive(Deserialize, Default, Debug)]
#[non_exhaustive]
pub enum DebugMode {
    #[serde(alias = "never")]
    Never,
    #[serde(alias = "debug_assertions")]
    #[default]
    DebugAssertions,
    #[serde(alias = "always")]
    Always,
}

#[derive(Default, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum Formatter {
    // None,
    #[default]
    PrettyPlease,
    Rustfmt,
}

#[derive(Default, Debug)]
pub struct SelfContainedOpts {
    pub crate_name: String,
    pub config_path: PathBuf,
    pub out_dir: PathBuf,
    pub debug_mode: DebugMode,
    pub mode_cap: Option<Mode>,
    pub formatter: Formatter,
    pub split_top_matter: bool,

    pub __track: Cell<u16>,
}
pub fn self_contained_from_build_script() -> PathBuf {
    self_contained_from_env(Formatter::PrettyPlease, None)
}

pub fn self_contained_from_env(
    formatter: Formatter,
    nondefault_location: Option<PathBuf>,
) -> PathBuf {
    let manifest_dir = std::env::var_os("CARGO_MANIFEST_DIR").unwrap();
    let config_path = Path::new(&manifest_dir).join("confeature.yml");
    let crate_name = std::env::var("CARGO_PKG_NAME").unwrap();
    // XXX: remove this hardcoding
    let is_debug_build = std::env::var("DEBUG").is_ok_and(|x| x.parse().unwrap());
    let opt_level = std::env::var("OPT_LEVEL").is_ok_and(|x| x.parse::<u32>().unwrap() > 1);
    // let out_dir = Path::new(&manifest_dir).join("src");
    let out_dir = nondefault_location.unwrap_or(std::env::var_os("OUT_DIR").unwrap().into());
    println!("cargo:rerun-if-env-changed=DEBUG");
    println!("cargo:rerun-if-env-changed=OPT_LEVEL");

    let split_top_matter = out_dir.extension().is_none_or(|e| e != "rs");
    self_contained(SelfContainedOpts {
        crate_name,
        config_path,
        out_dir,
        debug_mode: if is_debug_build && !opt_level {
            DebugMode::Always
        } else {
            DebugMode::Never
        },
        mode_cap: None,
        formatter,
        split_top_matter,
        ..SelfContainedOpts::default()
    })
}

/// Generate a guest module that doesn't need to be changed when changing compilation environment.
/// I.e., parsing is done inside the module or decl macros.
pub fn self_contained(opts: SelfContainedOpts) -> PathBuf {
    let config_path_str = opts.config_path.to_str().unwrap().to_owned();
    println!("cargo:rerun-if-changed={config_path_str}");

    let config =
        String::from_utf8_lossy(&fs::read(opts.config_path.as_path()).unwrap()).to_string();
    let input: ConfigSpec = serde_yaml::from_str(&config).unwrap();
    // eprintln!("Using YAML: \n{}", serde_yaml::to_string(&input).unwrap());

    let dest_path = if opts.out_dir.is_dir() {
        opts.out_dir.join("confeature.rs")
    } else {
        opts.out_dir.clone()
    };
    // XXX: fix this relative paths in in-repo mode
    let config_path_relative = if dest_path.is_relative()
        || opts
            .out_dir
            .starts_with(opts.config_path.parent().unwrap().join("src"))
    {
        "../confeature.yml".to_owned()
    } else {
        config_path_str
    };
    let conf = input.conf.as_ref();

    // This env is compilation-global.
    println!("cargo:rerun-if-env-changed=CONFEATURE_MODE_CAP");
    let mode_cap = [
        std::env::var_os("CONFEATURE_MODE_CAP")
            .map(|x| serde_plain::from_str::<Mode>(x.to_str().unwrap()).unwrap()),
        optional!(conf?.scope.mode_cap?),
        opts.mode_cap,
    ]
    .into_iter()
    .reduce(min_not_none)
    .flatten();

    let default_mode = optional!(conf?.scope.default_mode?).unwrap_or(Mode::Mixed);

    assert!(
        conf.is_none_or(|c| c.scope.default_mode.is_none_or(|m| m == Mode::Mixed)),
        "Changing the default mode is not yet implemented"
    );

    let mods = input.ns.into_iter().map(|(n, ns)| {
        generate_namespace(
            &opts,
            &Scope {
                mode_cap,
                default_mode,
                for_struct: false,
                scope: vec![n.clone()],
                depth: 0,
            },
            n.clone(),
            ns,
        )
    });

    let section_name = format!(".note.{}", opts.crate_name);
    let mut top_matter = String::new();
    let top_matter = &mut top_matter;
    if let Some(extra_doc) = optional!(&conf?.scope.doc) {
        writeln!(top_matter, "\n{extra_doc}").unwrap();
    }
    writeln!(
        top_matter,
        "This is an autogenerated module by ``confeature`` from {}!",
        opts.config_path
            .strip_prefix(
                opts.config_path
                    .parent()
                    .and_then(|p| p.parent())
                    .unwrap_or(Path::new("."))
            )
            .unwrap()
            .display(),
    )
    .unwrap();
    if let Some(cap) = mode_cap {
        writeln!(
            top_matter,
            "\nNOTE: The configuration mode was capped at \"{cap:?}\""
        )
        .unwrap();
    }
    // We overuse FDO_PACKAGING_METADATA (0xcafe1a7e) for readelf to display this as string.
    let mut output = quote! {
        #(#mods)*

        #[cfg(target_os = "linux")]
        confeature::elf_note::put_note! {
            var: _SOURCE,
            section: #section_name,
            vendor: b"FDO_CONFEATURE",
            type: 0xcafe_1a7e,
            note: include_bytes!(#config_path_relative),
        }
    };
    if opts.split_top_matter {
        fs::write(dest_path.with_extension("md"), top_matter).unwrap();
    } else {
        output = quote! {
            #![doc=#top_matter]
            #output
        };
    }
    // eprintln!("{}", output);
    let syntax_tree = syn::parse2(output).unwrap();

    let mut formatted =
        "// This is an autogenerated module by ``confeature``. Do not manually update this file!\n"
            .to_owned();
    formatted.push_str(&prettyplease::unparse(&syntax_tree));

    eprintln!("Generated file in {}", dest_path.display());
    fs::write(dest_path.as_path(), formatted).unwrap();
    if opts.formatter == Formatter::Rustfmt {
        Command::new("rustfmt")
            .arg(dest_path.as_path())
            .status()
            .expect("Rustfmt failed");
    }
    dest_path
}

#[derive(Default, Debug, Clone)]
struct Scope {
    mode_cap: Option<Mode>,
    default_mode: Mode,
    depth: u16,
    for_struct: bool,
    scope: Vec<String>,
}

fn generate_namespace(
    opts: &SelfContainedOpts,
    scope: &Scope,
    namespace: String,
    inner: Namespace,
) -> impl ToTokens + use<> {
    assert!(
        !namespace.starts_with('.'),
        "Namespaces starting with `.` are reserved"
    );
    let namespace_str = namespace;
    let namespace_qual_str = scope.scope.iter().join_qual_str();
    let namespace = namespace_str.as_ident().mod_name();

    let mut field_names = Vec::<syn::Ident>::new();
    let mut debug_entries = Vec::<_>::new();
    let mode_cap = min_not_none(scope.mode_cap, optional!(inner.scope.as_ref()?.mode_cap?));
    let default_mode = optional!(inner.scope.as_ref()?.default_mode?).unwrap_or(scope.default_mode);
    let mut module_doc = String::new();

    let module_doc = &mut module_doc;
    if let Some(extra_doc) = optional!(&inner.scope.as_ref()?.doc) {
        writeln!(module_doc, "\n{extra_doc}").unwrap();
    }
    writeln!(
        module_doc,
        "\nModule representing the **{namespace_qual_str}** scope defined in ``<{} root>/confeature.yml``",
        opts.crate_name,
    ).unwrap();

    let extra_code = optional!(inner.scope.as_ref()?.extra_code.as_ref()?)
        .map_or(quote!(), |c| c.parse().expect("Cannot parse code"));
    let _order = {
        let x = opts.__track.get();
        opts.__track.set(x + 1);
        x
    };

    let fields = inner.into_iter().map(|(name, conf)| {
            assert!(!name.starts_with('.'), "All keys starting with `.` are reserved");

            let local_scope = Scope {
                mode_cap,
                default_mode,
                for_struct: scope.for_struct,
                depth: scope.depth+1,
                scope: scope.scope.iter().chain([&name.mod_name()]).cloned().collect(),
            };
            let debug_indent = "│ ".repeat(scope.depth as usize) + "├─";

            if let Configurable::Namespace {ns, ..} = conf {
                let mod_name = name.mod_name().as_ident();
                debug_entries.push(quote! {
                    #mod_name::debug_internal_();
                });
                return generate_namespace(opts, &local_scope, name, ns);
            }
            let name_orig = name;
            let name_qual = local_scope.scope.iter().join_qual_str();
            let name_str = name_orig.const_name();
            let name = name_str.as_ident();
            let env_name = local_scope.scope.iter().join_env_var();
            field_names.push(name.clone());

            let inherited_mode = conf.get_mode(default_mode);
            let effective_mode = min_not_none(mode_cap, Some(inherited_mode)).unwrap();

            let env_mode = effective_mode.as_ident();

            let mut var_doc = String::new();
            let var_doc = &mut var_doc;
            if let Some(ref extra_doc) = conf.get_attrs().doc {
                writeln!(var_doc, "\n{extra_doc}").unwrap();
            }
            write!(var_doc, "\nConfigurable ``{name_qual}``").unwrap();

            let type_name = build_type_name(&conf, &name_str);
            let type_def = build_type_def(&conf, &type_name);
            let wrap_in_option = conf.is_optional();
            if wrap_in_option {
                write!(var_doc, ": `Option<{}>`", type_name.segments.iter().join_qual_str()).unwrap();
            } else {
                write!(var_doc, ": `{}`", type_name.segments.iter().join_qual_str()).unwrap();
            }

            assert!(!matches!(conf,
                    Configurable::Int { optional: false, default:None, .. }
                    | Configurable::Enum { optional: false, default: None, .. }
                ), "Field {name_qual} is required, but has no default");
            let mut default = match (&conf, effective_mode) {
                (Configurable::Feature { .. }, _) => quote!(cfg!(feature = #name_orig)),
                (Configurable::Cfg { expr, .. }, _) => {
                    let meta: syn::Meta = syn::parse_str(expr).expect("Cannot parse expr as cfg! meta");
                    quote!(cfg!(#meta))
                }
                (Configurable::Bool { default, .. }, Mode::Fixed) => {
                    quote! {#default}
                }
                (Configurable::Bool { default, .. }, _) => {
                    write!(var_doc, " default = {default}").unwrap();
                    quote!(konst::option::unwrap_or!(
                        konst::option::map!(
                            ::confeature::from_env!(#env_mode, #env_name),
                            ::confeature::parse_bool
                        ),
                        #default
                    ))
                }
                (Configurable::Int { default: None, .. }, Mode::Fixed) => {
                    quote!(None)
                },
                (Configurable::Int { default: Some(default), .. }, Mode::Fixed) => {
                    quote! {#default}
                },
                (Configurable::Int { default, range, .. }, _) => {
                    let mut code =
                        quote!(konst::option::map!(
                        ::confeature::from_env!(#env_mode, #env_name),
                        |x| konst::result::unwrap_ctx!(konst::primitive::parse_i64(x))
                    ));
                    if let Some(range) = range {
                        let start = range.start;
                        let end = range.end;
                        write!(var_doc, " range = {start}..={end}").unwrap();
                        code = quote!(konst::option::map!(
                            #code,
                            |x| konst::max!(konst::min!(x, #end), #start)
                        ));
                    }
                    if let Some(default) = default {
                        write!(var_doc, " default = {default}").unwrap();
                        code = quote!(
                           konst::option::unwrap_or!(
                             #code,
                            #default
                        ));
                    }
                    code
                }
                (Configurable::Enum { default: None, .. }, Mode::Fixed) => {
                    quote!(None)
                },
                (Configurable::Enum { default: Some(default), .. }, Mode::Fixed) => {
                    let variant = default.as_ident();
                    quote! {#type_name::#variant}
                },
                (Configurable::Enum { default: None, .. }, _) => quote!(konst::option::and_then!(
                    ::confeature::from_env!(#env_mode, #env_name),
                    // TODO: return Result here
                    #type_name::parse_str
                )),
                (Configurable::Enum {
                    default: Some(default),
                    ..
                }, _) => {
                    write!(var_doc, " default = {default}").unwrap();
                    quote!(konst::option::unwrap_or!(
                    konst::option::and_then!(
                        ::confeature::from_env!(#env_mode, #env_name),
                        // TODO: return Result here
                        #type_name::parse_str
                    ),
                    konst::option::unwrap!(
                        #type_name::parse_str(#default)
                    )
                ))
                },
                (Configurable::Str { default: None, .. }, Mode::Fixed) => {
                    quote!(None)
                },
                (Configurable::Str { default: Some(default), .. }, Mode::Fixed) => {
                    quote! {#default}
                },
                // The only option to get a &'static initialized at runtime is to leak...
                (Configurable::Str { default: None, .. }, m) if m >= Mode::Mixed =>   quote!(
                    konst::option::map!(
                        ::confeature::from_env!(#env_mode, #env_name),
                        |x| x.to_string().leak::<'static>()
                    )
                ),
                // XXX: Patchable &'static str symbol points to [str] slice,
                //      but the pointer is inlined and only size is read.
                (Configurable::Str { default: None, .. }, _) => quote!(
                    ::confeature::from_env!(#env_mode, #env_name)
                ),
                (Configurable::Str {
                    default: Some(default),
                    ..
                }, m) if m >= Mode::Mixed => {
                    write!(var_doc, " default = {default}").unwrap();
                    quote!(konst::option::unwrap_or!(
                        konst::option::map!(
                            ::confeature::from_env!(#env_mode, #env_name),
                            |x| x.to_string().leak::<'static>()
                        ),
                        #default,
                    )
                )
                },
                (Configurable::Str {
                    default: Some(default),
                    ..
                }, _) => {
                    write!(var_doc, " default = {default}").unwrap();
                    quote!(konst::option::unwrap_or!(
                        ::confeature::from_env!(#env_mode, #env_name),
                        #default,
                    )
                )
                },
                (Configurable::Bitfield { .. }, _) => quote!(todo!()),
                (Configurable::Struct { fields, .. }, _) => {
                    assert!(conf.has_default(), "Right now structs don't support optional fields! In: {}", &name_qual);
                    let (defaults, names) : (Vec<_>, Vec<_>)  = fields.iter().map(|(fname, fconf)| {
                        assert!(!matches!(fconf, Configurable::Namespace {..}), "Struct {} cannot have a nested namespace", &name_qual);
                        let fname = fname.field_name().as_ident();
                        let value_path = fname.const_name().qual_in(name.mod_name());
                        (quote!(*#value_path), fname)
                    }).unzip();

                    // Instantiate
                    quote! {
                        #type_name {
                            #(
                               #names: #defaults,
                            )*
                        }
                    }

                },
                (Configurable::Namespace { .. }, _) => unimplemented!(),
            };
            append_doc_hints(var_doc, &conf, inherited_mode, effective_mode, &env_name);
            let mut final_type =
                build_type(&conf, &type_name);
            if wrap_in_option {
                if conf.has_default() {
                    default = quote! {Some(#default)};
                }
                final_type = syn::parse_quote!(Option<#final_type>);
            }
            debug_entries.push(quote! {
                ::confeature::debug_env!(
                    #debug_indent, #env_mode, #name, #name_qual, #final_type, #env_name
                );
            });
            let var_def = match effective_mode {
                Mode::Comptime | Mode::Fixed => quote! {
                    pub const #name: &#final_type = &#default;
                },
                Mode::Patchable => quote! {
                    #[unsafe(export_name = #env_name)]
                    pub static #name: ::confeature::BlackBox<#final_type> = ::confeature::BlackBox::new(#default);
                },
                Mode::Mixed | Mode::Anytime | Mode::Runtime => quote! {
                    #[dynamic]
                    pub static #name: #final_type = #default;
                },
            };

            let extra_defs = if let Configurable::Struct {mode, fields, ..} = conf {
                // For now, let's define a nested namespace
                let mode = mode.unwrap_or(default_mode);
                // Mode of the struct is defined here, so we need to cap these below.
                let mut mode_cap = min_not_none(mode_cap, Some(mode));
                // Nested patchable not implemented
                if let Some(Mode::Patchable) = mode_cap {
                    mode_cap = Some(Mode::Comptime);
                    writeln!(var_doc, "\nNOTE: structs are patchable as a whole: the nested namespace is capped to `comptime`.").unwrap();
                }
                let mod_name = name_orig.mod_name().as_ident();
                debug_entries.push(quote! {
                    #mod_name::debug_internal_();
                });
                let phony_scope = Scope{
                    default_mode: mode,
                    mode_cap,
                    for_struct: true,
                    ..local_scope.clone()
                };
                generate_namespace(opts, &phony_scope, name_orig, fields)
            } else {quote!()};
            quote! {
                #extra_defs

                #type_def
                #[doc = #var_doc]
                #var_def
            }
        });
    let debug_cond = opts.debug_mode.as_cfg();
    let debug_indent = "│ ".repeat(scope.depth.saturating_sub(1) as usize)
        + if scope.depth > 0 { "├─" } else { "" };
    let debug_prefix = if scope.depth == 0 {
        // Top level
        quote! {
            #[constructor]
            extern "C"
        }
    } else {
        quote! {pub(super)}
    };
    let silenced_lints = silence_lints(opts, scope);
    let vis: syn::Visibility = if scope.for_struct {
        syn::parse_quote!()
    } else {
        syn::parse_quote!(pub)
    };
    let output = quote! {
        #[doc = #module_doc]
        #silenced_lints
        #vis mod #namespace {
            use static_init::{dynamic, constructor};
            use confeature::konst;
            #(#fields)*

            #debug_cond
            #debug_prefix fn debug_internal_() {
                let Some(_) = std::env::var_os("CONFEATURE_DEBUG") else {return};
                ::confeature::debug_scope!(
                    #debug_indent,
                    configurable,
                    #namespace_qual_str);
                #(
                    #debug_entries
                )*
            }

            #extra_code
        }
    };
    output
}

fn silence_lints(_opts: &SelfContainedOpts, scope: &Scope) -> impl ToTokens + use<> {
    if scope.depth == 0 {
        // List taken by something generated by syn or something
        // Manually comment them to evaluate if the generated code needs refinements
        quote! {
            #[allow(
                dead_code,
                unreachable_code,
                unused_variables,
                unused_braces,
                unused_imports,
                unused_qualifications,
                unreachable_pub,
            )]
            #[allow(
                clippy::style,
                clippy::pedantic,
                clippy::restriction,
                clippy::perf,
                clippy::nursery,
                clippy::complexity,
                clippy::cargo,
                clippy::suspicious_else_formatting,
                clippy::almost_swapped,
                clippy::redundant_locals,
                clippy::print_stderr,
            )]
        }
    } else {
        quote! {}
    }
}

fn append_doc_hints(
    var_doc: &mut String,
    conf: &Configurable,
    inherited_mode: Mode,
    effective_mode: Mode,
    env_name: &String,
) {
    {
        writeln!(var_doc).unwrap();
        if let Configurable::Struct { .. } = conf {
            // struct has special impl right now

            writeln!(
                var_doc,
                "Configurable on a field-by-field basis in a nested namespace."
            )
            .unwrap();
            return;
        }
        if effective_mode == Mode::Fixed {
            write!(
                var_doc,
                "Configurable only by changing the specification file"
            )
            .unwrap();
        }
        if effective_mode > Mode::Fixed {
            write!(
                var_doc,
                "Configurable with \"{env_name}\" environment variable"
            )
            .unwrap();
        }
        match effective_mode {
            Mode::Comptime | Mode::Patchable => write!(var_doc, " (only at comptime)"),
            Mode::Mixed => write!(var_doc, " (may upgrade to a constant)"),
            Mode::Runtime => write!(var_doc, " (only at runtime)"),
            _ => Ok(()),
        }
        .unwrap();
        if effective_mode == Mode::Patchable {
            write!(
                var_doc,
                ",\nor by binary patching the \"{env_name}\" symbol value in the result binary"
            )
            .unwrap();
        }
        writeln!(var_doc, ".").unwrap();
        if inherited_mode > effective_mode {
            writeln!(
                var_doc,
                "\nNOTE: The inherited mode \"{inherited_mode}\" was capped at \"{effective_mode}\""
            )
            .unwrap();
        }
    }
}

impl DebugMode {
    fn as_cfg(&self) -> impl ToTokens {
        match self {
            DebugMode::Never => quote!(#[cfg(false)]),
            DebugMode::DebugAssertions => quote!(#[cfg(debug_assertions)]),
            DebugMode::Always => quote!(#[cfg(true)]),
        }
    }
}

impl Mode {
    fn as_str(self) -> &'static str {
        match self {
            Mode::Fixed => "fixed",
            Mode::Comptime => "comptime",
            Mode::Patchable => "patchable",
            Mode::Mixed => "mixed",
            Mode::Anytime => "anytime",
            Mode::Runtime => "runtime",
        }
    }
    fn as_ident(self) -> syn::Ident {
        self.as_str().as_ident()
    }
}

impl Display for Mode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

fn build_type_name(conf: &Configurable, name_str: &String) -> syn::Path {
    match &conf {
        Configurable::Bool { .. } => "bool".as_path(),
        Configurable::Feature { .. } | Configurable::Cfg { .. } => "bool".as_path(),
        Configurable::Int { .. } => "i64".as_path(),
        Configurable::Enum { .. } => format!("{name_str}Values").item_name().as_path(),
        Configurable::Str { .. } => "str".as_path(),
        Configurable::Bitfield { .. } => name_str.item_name().as_path(),
        Configurable::Struct {
            instance: Some(s), ..
        } => syn::parse_str(s).expect("invalid path to instantiate a struct"),
        Configurable::Struct { .. } => name_str.item_name().as_path(),
        Configurable::Namespace { .. } => unimplemented!(),
    }
}

fn build_type(conf: &Configurable, type_name: &syn::Path) -> syn::Type {
    match &conf {
        Configurable::Str { .. } => syn::Type::Reference(syn::parse_quote!(&'static #type_name)),
        _ => syn::Type::Path(syn::parse_quote!(#type_name)),
    }
}

fn build_type_def(conf: &Configurable, type_name: &syn::Path) -> impl ToTokens + use<> {
    match &conf {
        Configurable::Enum {
            variants: items, ..
        } => {
            let items = items.iter().map(|i| i.as_ident()).collect::<Vec<_>>();
            quote!(
                #[derive(Debug, PartialEq, Eq, Clone, Copy)]
                #[repr(C)]
                pub enum #type_name {
                    #(#items),*
                }
                impl #type_name {
                    pub const fn parse_str(s: &str) -> Option<#type_name> {
                        if konst::eq_str(s, "") {None}
                        #(
                        else if konst::eq_str(s, stringify!(#items)) {
                                Some(#type_name::#items)
                            }
                        )*
                        else {
                            panic!(concat!("Invalid variant for ", stringify!(#type_name)));
                        }
                    }
                }
                // impl std::str::FromStr for #type_name {
                //     type Err = String;
                //     fn from_str(s: &str) -> Result<Self, Self::Err> {
                //         match s {
                //             // #( stringify!(#items) => Ok(#) ,)*
                //             _ => Err(format!("'{}' is not a valid value for {}", s, #name_str))
                //         }
                //         }
                //     }
            )
        }
        Configurable::Bitfield { .. } => quote!(
            pub struct #type_name {

            }
        ),
        Configurable::Struct {
            instance: Some(_), ..
        } => {
            quote!()
        }
        Configurable::Struct { fields, .. } => {
            let type_name = type_name
                .get_ident()
                .expect("We cannot generate a struct with a complex name!");
            let (field_names, field_types): (Vec<_>, Vec<_>) = fields
                .iter()
                .map(|(n, f)| {
                    let name_ident = n.as_ident();
                    let ftype_name = build_type_name(f, n);
                    let ftype_name = match f {
                        Configurable::Struct { instance: None, .. } | Configurable::Enum { .. } => {
                            let nested_mod_name = type_name.mod_name();
                            let path = if let Some(ftype_name) = ftype_name.get_ident() {
                                ftype_name.qual_in(nested_mod_name)
                            } else {
                                ftype_name
                            };
                            syn::Type::Path(syn::parse_quote!(#path))
                        }
                        _ => build_type(f, &ftype_name),
                    };

                    (name_ident, ftype_name)
                })
                .unzip();
            // TODO: Struct in struct is hard (use Copy for now)
            quote! {
            #[derive(Debug, Clone, Copy)]
            #[repr(C)]
            pub struct #type_name {
                #(
                    pub #field_names: #field_types,
                )*
            }
            }
        }
        _ => quote!(),
    }
}

trait IdentOps<T: IdentOps<T>>: Sized {
    fn as_string(&self) -> String;
    fn as_ident(&self) -> syn::Ident;
    #[allow(unused)]
    fn from_ident(ident: syn::Ident) -> T;
    fn from_string(str: String) -> T;

    fn as_path(&self) -> syn::Path {
        let x = self.as_ident();
        syn::parse_quote! {#x}
    }

    #[inline(always)]
    fn _convert(&self, case: Case) -> String {
        self.as_string()
            .without_boundaries(&Boundary::digits())
            .to_case(case)
    }
    fn const_name(&self) -> T {
        Self::from_string(self._convert(Case::Constant))
    }
    fn mod_name(&self) -> T {
        Self::from_string(self._convert(Case::Snake))
    }
    fn field_name(&self) -> T {
        Self::from_string(self._convert(Case::Snake))
    }
    fn item_name(&self) -> T {
        Self::from_string(self._convert(Case::Pascal))
    }
    fn qual_in(&self, scope: impl ToTokens) -> syn::Path {
        let id = self.as_ident();
        syn::parse_quote!(#scope :: #id)
    }
}

trait QualOps<'a, T>
where
    Self: 'a,
    Self: Sized,
    Self: Iterator,
    <Self as Iterator>::Item: IdentOps<T> + 'a,
    T: IdentOps<T>,
{
    fn __mapped<B>(self, f: impl FnMut(<Self as Iterator>::Item) -> B) -> Vec<B> {
        self.map(f).collect::<Vec<_>>()
    }
    fn join_env_var(self) -> String {
        self.__mapped(|x| x._convert(Case::Constant)).join("__")
    }

    fn join_qual_str(self) -> String {
        self.__mapped(|x| x.as_string()).join("::")
    }

    #[allow(unused)]
    fn join_qual(self) -> syn::Path {
        let ids = self.__mapped(|x| x.as_ident());
        syn::parse_quote! {#(#ids)::*}
    }
}

impl<'a, I, T> QualOps<'a, T> for I
where
    I: 'a,
    I: Sized,
    I: Iterator,
    <I as Iterator>::Item: IdentOps<T> + 'a,
    T: IdentOps<T>,
{
}

impl<T: AsRef<str>> IdentOps<String> for T {
    fn as_string(&self) -> String {
        <T as AsRef<str>>::as_ref(self).to_owned()
    }

    fn as_ident(&self) -> syn::Ident {
        format_ident!("{}", self.as_ref())
    }

    fn from_ident(ident: syn::Ident) -> String {
        ident.to_string()
    }

    fn from_string(str: String) -> String {
        str
    }
}

impl IdentOps<syn::Ident> for syn::Ident {
    fn as_string(&self) -> String {
        self.to_string()
    }

    fn as_ident(&self) -> syn::Ident {
        self.clone()
    }

    fn from_ident(ident: syn::Ident) -> syn::Ident {
        ident
    }

    fn from_string(str: String) -> syn::Ident {
        format_ident!("{}", str)
    }
}

impl IdentOps<syn::Ident> for &syn::PathSegment {
    fn as_string(&self) -> String {
        self.ident.to_string()
    }

    fn as_ident(&self) -> syn::Ident {
        self.ident.clone()
    }

    fn from_ident(ident: syn::Ident) -> syn::Ident {
        ident
    }
    fn from_string(str: String) -> syn::Ident {
        format_ident!("{}", str)
    }
}
