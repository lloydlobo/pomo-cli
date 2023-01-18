use std::process;

use clap::{
    Arg,
    Parser,
};
use log::info;
use miette::{
    Context,
    IntoDiagnostic,
};
use pompom;

fn main() {
    info!("pompom.rs / main()");
    let config = pompom::PomodoroConfig::try_parse()
        .into_diagnostic()
        .wrap_err("Failed to parse command line arguments in main");

    if let Err(e) = pompom::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
