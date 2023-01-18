//! pomodoro is a pomodoro cli application.
//!
//! Inspired by [bashbunni: Source](https://gist.github.com/bashbunni/3880e4194e3f800c4c494de286ebc1d7)
//!
//! * Requires  spd-say should ship with your distro or install it.
//! `$ echo "hello world" | spd-say -e -t male3`
//! * (Optional) Requires [caarlos0/timer](https://github.com/caarlos0/timer).
//!
//! # Development
//!
//! ```bash
//! $ cargo run -- --help
//! pomo_cli is a terminal pomodoro CLI app that vocally reminds you
//! while running gracefully in the background.
//!
//! Usage: pomo_cli [OPTIONS] --task <TASK>
//!
//! Options:
//!   -t, --task <TASK>            Name of task session
//!   -i, --intervals <INTERVALS>  Number of interval cycles to run pomodoro [default: 1]
//!   -h, --help                   Print help
//!   -V, --version                Print version
//! ```
//!
//! ```bash
//! $ cargo watch -x 'run -- --task Study --intervals 4'
//! ```

// -------------------------------------------------------------------------

mod cli;
mod error;
mod utils;

#[macro_use]
extern crate lazy_static;

use std::{
    thread,
    time::Duration,
};

use error::PomoLibError;
use indicatif::ProgressBar;
use log;
use miette::{
    Context,
    Diagnostic,
};
use thiserror::Error;
pub use utils::*;
use xshell::{
    cmd,
    Shell,
};

use crate::error::{
    handle_invalid_user_args,
    CliParseError,
};

// -------------------------------------------------------------------------

// -------------------------------------------------------------------------

lazy_static! {
    pub static ref REPEAT_INTERVALS_NOTIFY: u64 = 5u64;
}

pub const POMO_OPTIONS: [&str; 2] = ["work", "break"];
pub const MILLIS_IN_ONE_SECOND: u64 = 10; // Default 1000 ms.
pub const SECONDS_IN_ONE_MINUTE: u64 = 60;
// static REPEAT_INTERVALS_NOTIFY: u64 = 5;

// -------------------------------------------------------------------------

#[derive(Error, Debug, Diagnostic)]
#[error("Invalid pomodoro option")]
pub enum PomoOptions {
    Work,
    Break,
}

impl PomoOptions {
    fn new(self) -> String {
        match self {
            PomoOptions::Work => String::from("45"),
            PomoOptions::Break => String::from("10"),
        }
    }
}

// -------------------------------------------------------------------------

// TODO: Handle error when user passed arg is None, or not one of opts;
pub fn pomodoro() -> miette::Result<(), PomoLibError> {
    log::info!(
        "pomodoro: Tty(..).are_you_tty(): {:#?}",
        utils::Tty(atty::Stream::Stdout).are_you_tty()
    );

    let cli_args: cli::CliArgs = match cli::get_user_stdin_args() {
        // let cli_args: cli::CliArgs = match cli::get_user_args() {
        Ok(it) => it,
        Err(err) => {
            log::error!("cli::get_user_args():\n{:#?}Severity: {:?}", err.src, err.severity());
            return Err(err).wrap_err("Could not get arguments passed in the terminal").unwrap();
        }
    };
    log::debug!("{:#?}", &cli_args);
    handle_invalid_user_args(&cli_args);

    // error_app::some_tool().wrap_err("pomodoro")?;

    let sh = Shell::new().expect("should create a new shell");
    // let args: Vec<String> = std::env::args().collect();
    let args: Vec<String> = vec![cli_args.intervals.to_string(), cli_args.task.to_lowercase()];
    log::debug!("args: {:#?}", &args);

    let pomo_user: Option<PomoOptions> = match Some(&args[1]) {
        Some(arg) if arg == POMO_OPTIONS[0] || arg == "wo" => Some(PomoOptions::Work),
        Some(arg) if arg == POMO_OPTIONS[1] || arg == "br" => Some(PomoOptions::Break),
        _ => None,
    };
    let arg_duration_min: Option<String> = match pomo_user {
        Some(x) => match x {
            PomoOptions::Work => Some(PomoOptions::new(PomoOptions::Work)),
            PomoOptions::Break => Some(PomoOptions::new(PomoOptions::Break)),
        },
        None => None,
    };

    let progress_duration: u64 = 60 * (&arg_duration_min.as_ref().unwrap()).parse::<u64>().unwrap();

    let arg_duration_secs: Option<String> = Some(progress_duration.to_string());
    cmd!(sh, "echo {arg_duration_min...} minutes or {arg_duration_secs...} seconds").run().unwrap(); // `echo 45 min`

    let pb = ProgressBar::new(progress_duration);
    (0..progress_duration).for_each(|i: u64| {
        pb.inc(1);
        notify_intervals(i, intervals(&progress_duration), SECONDS_IN_ONE_MINUTE, &sh);
        thread::sleep(Duration::from_millis(MILLIS_IN_ONE_SECOND));
        // 1 second = 1000 milliseconds
    });
    pb.finish_with_message("Time up");

    let arg_user_session_done: Option<String> = Some(format!("{} session done", &args[1]));
    cmd!(sh, "spd-say -t female1 {arg_user_session_done...}").run().unwrap(); // `$ spd-say "'$val' session done"`

    Ok(())
}

/// PERF: Prompt the user for intervals. Prompt user to increase total cycles or time of duration.
/// Handle cases when `repeat * minute` exceeds `progress_duration` doesn't make sense.
/// Will have no interal notifications.
fn intervals(&progress_duration: &u64) -> u64 {
    let mut repeat = *REPEAT_INTERVALS_NOTIFY;
    while (repeat * SECONDS_IN_ONE_MINUTE) > progress_duration {
        repeat -= 1;
    }
    repeat
}

fn notify_intervals(i: u64, repeat: u64, minute: u64, sh: &Shell) {
    if i % (repeat * minute) == 0 && i != 0 {
        if i <= 60 {
            notify_elapsed_time(sh, Some(format!("{} minute over", i / 60)));
        } else {
            notify_elapsed_time(sh, Some(format!("{} minutes over", i / 60)));
        }
    }
}

/// `$ spd-say "'$val' session done"`
fn notify_elapsed_time(sh: &Shell, arg_curr_progress: Option<String>) {
    cmd!(sh, "spd-say {arg_curr_progress...}").run().unwrap();
}

pub struct Cli {
    pomo: String,
    break_short: String,
    break_long: String,
    duration_pomo: u32,
    duration_break: u32,
    task: String,
}

// -------------------------------------------------------------------------

// -------------------------------------------------------------------------

// -------------------------------------------------------------------------

mod error_app {
    use miette::{
        IntoDiagnostic,
        Result,
        WrapErr,
    };
    use semver::Version;

    /// * Application code tends to work a little differently than libraries. You don't always need
    ///   or care to define dedicated error wrappers for errors coming from external libraries and
    ///   tools.
    /// * For this situation, miette includes two tools: Report and IntoDiagnostic. They work in
    ///   tandem to make it easy to convert regular std::error::Errors into Diagnostics.
    ///   Additionally, there's a Result type alias that you can use to be more terse.
    /// * When dealing with non-Diagnostic types, you'll want to .into_diagnostic() them: miette
    ///   also includes an anyhow/eyre-style Context/WrapErr traits that you can import to add
    ///   ad-hoc context messages to your Diagnostics, as well, though you'll still need to use
    ///   .into_diagnostic() to make use of it:
    pub fn some_tool() -> miette::Result<Version> {
        Ok("1.2.x"
            .parse()
            .into_diagnostic()
            .wrap_err("Parsing this tool's semver version failed.")?)
    }
}

// -------------------------------------------------------------------------
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_matches() {
        let sh = Shell::new().expect("should create a new shell");
        let greeting = "hello world";
        let c = cmd!(sh, "echo {greeting}!");
        assert_eq!(c.to_string(), r#"echo "hello world!""#);
    }

    #[test]
    fn it_echo_args_variables() {
        let sh = Shell::new().expect("should create a new shell");
        let arg_work = POMO_OPTIONS[0].trim();
        let c = cmd!(sh, "echo {arg_work}");
        assert_eq!(c.to_string(), r#"echo work"#);
    }

    #[test]
    fn it_echo_arg_in_vec() {
        let sh = Shell::new().expect("should create a new shell");
        let args = ["hello", "world"];
        let c = cmd!(sh, "echo {args...}");
        assert_eq!(c.to_string(), r#"echo hello world"#);
    }

    #[test]
    fn it_echo_pomo_opts() {
        let sh = Shell::new().expect("should create a new shell");
        let arg_work = POMO_OPTIONS[0].trim();
        let c = cmd!(sh, "echo '{arg_work}'");
        assert_eq!(c.to_string(), "echo {arg_work}");
        let c = cmd!(sh, "echo {arg_work}");
        assert_eq!(c.to_string(), "echo work");
        let c = cmd!(sh, "echo '{arg_work...}'");
        assert_eq!(c.to_string(), "echo {arg_work...}");
    }

    #[test]
    #[should_panic]
    fn it_panics_if_not_raw_str() {
        let sh = Shell::new().expect("should create a new shell");
        let arg_work = POMO_OPTIONS[0].trim();
        let c = cmd!(sh, "echo {arg_work}");
        assert_eq!(c.to_string(), r#"echo "work""#);
    }

    #[test]
    fn it_echo_option_iter() {
        let sh = Shell::new().expect("should create a new shell");
        let args = [Some("work"), Some("wo"), Some("break"), Some("br"), None];

        // Can iter individual args from a vector if it is an option.
        args.iter().for_each(|arg| {
            let c = cmd!(sh, "echo {arg...}");
            match arg {
                Some(a) => {
                    assert_eq!(c.to_string(), format!("echo {}", a))
                }
                None => {
                    assert_eq!(c.to_string(), format!("echo{}", arg.unwrap_or("")))
                }
            }
        })
    }
}
