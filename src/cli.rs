mod fast_log_lib;
mod humantime_serde_lib;

use clap::{
    self,
    arg,
    command,
    Parser,
};
use miette::{
    IntoDiagnostic,
    NamedSource,
};

use crate::error::CliParseError;

// -------------------------------------------------------------------------

/// Cli program to init a audible pomodoro timer.
#[derive(Parser, Debug, Clone)] // requires clap `derive` feature
#[command(author, version, about, long_about = None, term_width = 0)] // term_width just to make testing across clap feature easy
pub struct CliArgs {
    /// Name of task session
    #[arg(short= 't', long, default_value_t = String::from("Work"))]
    pub task: String,

    /// Number of interval cycles to run pomodoro
    #[arg(short = 'i', long, default_value_t = 1)]
    pub intervals: u8,

    /// Start pomo after a delay of time e.g. 5sec or 5ms
    #[arg(short = 's', long, default_value = None)]
    pub sleep: Option<humantime::Duration>,
    // Allow invalid UTF-8 paths
    // #[arg(short = 'I', value_name = "DIR", value_hint = clap::ValueHint::DirPath)]
    // pub include: Option<std::path::PathBuf>,
}

// -------------------------------------------------------------------------

/*
 * * If you have a type implementing [`Diagnostic`] consider simply returning it or using
 *   [`Into`] or the [`Try`](std::ops::Try) operator (`?`).
 * * Wrap the error value with a new adhoc error
 * * Now let's define a function! Use this `Result` type (or its expanded version) as the return
 *   type throughout your app (but NOT your libraries! Those should always return concrete
 *   types!).
 * * Calling this on a type implementing [`Diagnostic`] will reduce it to the common denominator
 *   of [`std::error::Error`]. All extra information provided by [`Diagnostic`] will be
 *   inaccessible.
 */

/// Parse from `std::env::args_os()`, return Err on error.
pub fn get_user_stdin_args() -> miette::Result<CliArgs, CliParseError> {
    match CliArgs::try_parse().into_diagnostic() {
        Ok(it) => Ok(it),
        Err(err) => {
            let mut err_help = err.help();
            err_help.get_or_insert(Box::new(
                "try `pomo_cli --task work --interval 4`\nor `pomo_cli -t break -i 1`",
            ));

            Err(CliParseError {
                src: NamedSource::new("cli.rs", format!("{:#?}", CliArgs::try_parse().as_ref())),
                // src: NamedSource::new("cli.rs", format!("{:#?}", "dfs")),
                bad_bit: (7..11).into(), // (9, 4).into()
                advice: Some(format!("{}", err_help.unwrap())),
                related_error: Some(err),
            })?
        }
    }
}

// -------------------------------------------------------------------------
