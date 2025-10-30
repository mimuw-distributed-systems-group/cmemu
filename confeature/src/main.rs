use cargo_toml::Manifest;
use serde::Deserialize;
use std::env;
use std::error::Error;
use std::path::PathBuf;

use confeature::builder::{DebugMode, Formatter, SelfContainedOpts, self_contained};
use confeature::deser::Mode;

struct Args {
    cap: Option<Mode>,
    debug_mode: Option<DebugMode>,
    manifest_path: Option<PathBuf>,
}

fn parse_args() -> Result<Args, lexopt::Error> {
    use lexopt::prelude::*;

    let mut args = Args {
        cap: None,
        debug_mode: None,
        manifest_path: None,
    };

    let mut parser = lexopt::Parser::from_env();
    while let Some(arg) = parser.next()? {
        match arg {
            Short('c') | Long("cap") => {
                args.cap = Some(
                    parser
                        .value()?
                        .parse_with(|x| serde_plain::from_str::<Mode>(x))?,
                );
            }
            Short('d') | Long("debug") => {
                args.debug_mode = Some(
                    parser
                        .value()?
                        .parse_with(|x| serde_plain::from_str::<DebugMode>(x))?,
                );
            }
            Long("manifest-path") => {
                args.manifest_path = Some(parser.value()?.parse()?);
            }
            Long("help") => {
                println!(
                    "Usage: confeature [-c|--cap=mode] [-d|--debug=always|never|debug_assertions] [--manifest-path=PATH]"
                );
                std::process::exit(0);
            }
            _ => return Err(arg.unexpected()),
        }
    }
    if args.manifest_path.is_none() {
        let cwd = env::current_dir().map_err(|e| lexopt::Error::Custom(Box::new(e)))?;
        args.manifest_path = Some(cwd);
    }

    Ok(args)
}

#[derive(Deserialize, Debug)]
struct AllMetadata {
    confeature: Option<Metadata>,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
struct Metadata {
    spec_path: Option<PathBuf>,
    generated_file_path: Option<PathBuf>,
    mode_cap: Option<Mode>,
    debug_mode: Option<DebugMode>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut args = parse_args()?;
    let manifest_path = args.manifest_path.unwrap().canonicalize()?;

    let manifest =
        Manifest::<AllMetadata>::from_path_with_metadata(manifest_path.join("Cargo.toml"))?;
    let pkg = manifest
        .package
        .expect("confeature has to be run with manifest of a package, not workspace");
    let crate_name = pkg.name().to_owned();
    let mut config_path = manifest_path.join("confeature.yml");
    let mut out_dir = manifest_path.join("src/");
    if let Some(meta) = pkg.metadata.and_then(|m| m.confeature) {
        args.cap = args.cap.or(meta.mode_cap);
        args.debug_mode = args.debug_mode.or(meta.debug_mode);
        if let Some(spec_path) = meta.spec_path {
            config_path = spec_path;
        }
        if let Some(out_path) = meta.generated_file_path {
            out_dir = manifest_path.join(out_path);
        }
    }
    assert!(
        config_path.exists(),
        "confeature.yml not found in: {}",
        manifest_path.display()
    );
    let _out = self_contained(SelfContainedOpts {
        crate_name,
        config_path,
        out_dir,
        mode_cap: args.cap,
        debug_mode: args.debug_mode.unwrap_or_default(),
        formatter: Formatter::Rustfmt,
        ..SelfContainedOpts::default()
    });
    Ok(())
}
