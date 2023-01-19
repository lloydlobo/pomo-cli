use std::process;

use clap::Parser;
use miette::{
    Context,
    IntoDiagnostic,
};
pub use pompom::{
    self,
    PompomConfig,
};

fn main() {
    let cfg = match pompom::PompomConfig::try_parse()
        .into_diagnostic()
        .wrap_err("Failed to parse command line arguments in main")
    {
        Ok(t) => t,
        Err(e) => {
            eprintln!("oops: {:?}", e);
            process::exit(1);
        }
    };

    match pompom::run(Ok(cfg)) {
        Err(e) => {
            eprintln!("Application error: {}", e);
            log::error!("Application error: {}", e);
            process::exit(1);
        }
        Ok(_) => {
            log::info!("Application finished successfully")
        }
    }
}
