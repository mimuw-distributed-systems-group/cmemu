// This file is to workaround dependency on cmemu binary (Cargo's -Z bindeps are unstable)

use clap::Parser;
use std::process::ExitCode;

use cmemu::{App, PrettyTermination, run};

fn main() -> PrettyTermination {
    // This is hack for FromResidual being unstable
    mainer().into()
}

fn mainer() -> Result<ExitCode, Box<dyn std::error::Error>> {
    let args = App::parse();

    // This is different for errors catched by main cmemu for now!
    run(args, None)
}
