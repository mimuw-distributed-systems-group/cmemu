use cmemu_codegen::components;
use std::process::Termination;

const RECOMMENDED_STABLE_RUSTC_VERSION: &str = "1.89.0"; // Update also "rust-version" field in cmemu-lib/Cargo.toml.

fn main() -> impl Termination {
    check_rustc_version();
    // println!("cargo:rustc-cfg=i_dont_care_about_warnings_in=\"rfc\"");
    // Alternatively, switch to manual `cargo run --manifest-path <playground>/mm319369/confeature/Cargo.toml -F binary -- --cap comptime -d never`
    confeature::builder::self_contained_from_env(confeature::builder::Formatter::Rustfmt, None);
    println!("cargo:rustc-check-cfg=cfg(i_dont_care_about_warnings_in, values(any()))");
    println!("cargo:rustc-cfg=i_dont_care_about_warnings_in=\"rfc\"");
    components::analyze_components_generate_boilerplate()
}

//////////////////////////////////////////////
//          Check rustc version
//////////////////////////////////////////////
const CHECK_VAR_NAME: &str = "CMEMU_FAIL_IF_RUSTC_VERSION_MISMATCHED";
fn check_rustc_version() {
    // kinda hack, we probably shouldn't have this check in long term

    // `version_check::triple` is not used, because some of our developers use
    // builds of Rust that don't have the build date recorded and it blows up.
    let (rustc_version, channel) = version_check::Version::read()
        .zip(version_check::Channel::read())
        .expect("cannot read rustc version");
    // Set a cfg (`rustc_stable`, `rustc_nightly`, ...) to allow us to gate lints
    // behind rustc channel.
    println!("cargo:rustc-cfg=rustc_{channel}");

    // Version check is invoked in CI to make sure the latest stable version as
    // installed by CI matches what we know to be the latest stable version.
    // Outside of CI, the MSRV is enforced by rust-version in cmemu-lib's Cargo.toml.
    let version_check_needed = std::env::var_os(CHECK_VAR_NAME).is_some();
    println!("cargo:rerun-if-env-changed={CHECK_VAR_NAME}");
    if version_check_needed {
        if !channel.is_stable() {
            // We're not keeping bounds for versions other than stable, so expecting
            // nightly to be exactly the same version we expect stable to be is
            // quite unreasonable.
            eprintln!("attempted to run rustc version check on non-stable channel");
            std::process::exit(1);
        }
        if !rustc_version.exactly(RECOMMENDED_STABLE_RUSTC_VERSION) {
            let msg = format!(
                "Using rustc version {rustc_version} ({channel}), recommended is {RECOMMENDED_STABLE_RUSTC_VERSION}.",
            );
            println!("cargo:warning={msg}");
            eprintln!("rustc version check failed");
            std::process::exit(1);
        }
    }
}
