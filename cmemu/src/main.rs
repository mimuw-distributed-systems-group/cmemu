use clap::Parser;
use cmemu::{App, PrettyTermination, TimeoutError, run};
use std::process::ExitCode;

fn main() -> PrettyTermination {
    // This is hack for FromResidual being unstable
    mainer().into()
}

fn mainer() -> Result<ExitCode, Box<dyn std::error::Error>> {
    let args = App::parse();

    // prepare environment, data, etc
    let handle = flexi_logger::Logger::try_with_env()?
        .adaptive_format_for_stderr(pretty_flexi_logger::ADAPTIVE_PRETTY_FORMAT)
        .start()?;

    run(args, Some(handle)).map_or_else(
        |err| {
            if let Some(&TimeoutError { .. }) = err.downcast_ref() {
                Ok(ExitCode::SUCCESS)
            } else {
                Err(err)
            }
        },
        Ok,
    )
}
