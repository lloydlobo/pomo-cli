#![doc(
    html_logo_url = "https://raw.githubusercontent.com/lloydlobo/pompom/main/assets/pompom_logo_dark.png"
)]
#![doc = include_str!("../README.md")]
#![forbid(unsafe_code)]

mod error;

use chrono::{
    DateTime,
    Duration,
    Utc,
};
use clap::{
    command,
    Parser,
    Subcommand,
};
use clap_verbosity_flag::Verbosity;
use dialoguer::{
    console::Style,
    theme::ColorfulTheme,
    Confirm,
    Input,
};
use error::{
    NotificationError,
    NotifyResult,
};
use notify_rust::{
    Hint,
    Notification,
};
use xshell::{
    cmd,
    Shell,
};

pub async fn run(mut cli: PomoFocusCli) -> miette::Result<()> {
    if let Some(arg) = match &cli.command {
        Some(cmd) => match cmd {
            CliCommands::Interactive | CliCommands::I => Some(dialoguer_main(&cli)?),
        },
        None => None,
    } {
        cli.work_time = arg.work_time as u64;
        cli.short_break_time = arg.short_break_time as u64;
    }
    let _res = run_timer(cli).await.map(|_| ()).map_err(|_| NotificationError::Desktop);
    Ok(())
}

async fn run_timer(cli: PomoFocusCli) -> NotifyResult {
    let sh = Shell::new().expect("Shell::new() failed");
    let len_duration: u64 = cli.work_time * 60;
    let pb = indicatif::ProgressBar::new(len_duration);

    let created_at: DateTime<Utc> = Utc::now();
    let interval = std::time::Duration::from_millis(1000); // default to 1000ms as 1sec.
                                                           //
    let arg_duration_work = Some(cli.work_time.to_string());
    cmd!(sh, "echo {arg_duration_work...} minutes").run().unwrap();

    let every_n_minute = |m: u64| m * 60;
    let if_elapsed_spd_say = |i: &u64| match (i) % every_n_minute(5) == 0 && *i != 0 {
        //TODO: Instead of spd-say, use rust_notify::Notification.
        true => Some(format!("{} minutes over", i / 60)),
        false => None,
    };
    (0..len_duration).for_each(|i: u64| {
        pb.inc(1);
        // TODO: Move conditional here to avoid invoking notification for `None` cases.
        notify_elapsed_time(&sh, if_elapsed_spd_say(&i));
        std::thread::sleep(interval);
    });
    pb.finish_with_message("Pomodoro finished! Take a break!");
    {
        let args = vec!["5", "5"];
        let arg_user_session_done: Option<String> = Some(format!("{} session done", &args[1]));
        cmd!(sh, "spd-say -t female1 {arg_user_session_done...}").run().unwrap();
        // `$ spd-say "'$val' session done"`
    }
    {
        let work_expired_at = created_at + Duration::minutes(cli.work_time as i64);
        let break_expired_at = work_expired_at + Duration::minutes(cli.short_break_time as i64);
        let id = 1;
        let args = NotificationManager {
            id,
            description: String::from("Work session over"),
            work_time: cli.work_time as u16,
            short_break_time: cli.short_break_time as u16,
            long_break_time: cli.long_break_time as u16,
            created_at,
            work_expired_at,
            break_expired_at,
            body: format!("{:#?},{}", id, work_expired_at),
            icon: "alarm",
            timeout: 2000,
            appname: "pompom",
        };

        notify_desktop(args)
    }
}

fn notify_desktop(args: NotificationManager) -> Result<(), NotificationError> {
    let mut notification = Notification::new();
    let notification = notification
        .summary(&args.description)
        .body(&args.body)
        .icon(args.icon)
        .appname(args.appname)
        .hint(Hint::Category("timer".to_owned())) // remove?
        .hint(Hint::Resident(true)) // this is not supported by all implementations
        .timeout(notify_rust::Timeout::Milliseconds(args.timeout));
    // this however is

    #[cfg(target_os = "linux")]
    notification.hint(Hint::Category("im.received".to_owned())).sound_name("message-new-instant");
    notification.show().map(|_| ()).map_err(NotificationError::Desktop)
}

#[derive(Debug, Subcommand, PartialEq, Clone)]
pub enum CliCommands {
    /// Usage: $ pompom interactive
    #[command(arg_required_else_help = false)]
    Interactive,

    /// Usage: $ pompom interactive
    #[command(arg_required_else_help = false)]
    I,
}

const DEFAULT_WORK_TIME: u64 = 15;
const DEFAULT_SHORT_BREAK_TIME: u64 = 5;
const DEFAULT_LONG_BREAK_TIME: u64 = 25;

/// `pompom` CLI terminal flags with settings.
/// By default, this will only report errors.
/// Press `Crl+z` to pause & go to background.
/// `$ fg` to return back to foreground.
/// `verbose: Verbosity::new(1, 0),` -> show warnings , output not silenced.
// [See](https://github.com/clap-rs/clap/blob/master/examples/git-derive.rs)
#[derive(Parser, Debug, Clone)] // requires `derive` feature
#[command(name = "pompom")]
#[command(author, version, about, long_about = None, term_width=0)]
pub struct PomoFocusCli {
    #[command(subcommand)]
    command: Option<CliCommands>,

    /// - `-q` silences output
    /// - `-v` show warnings - `-vv` show info - `-vvv` show debug - `-vvvv` show trace
    #[clap(flatten)]
    verbose: clap_verbosity_flag::Verbosity,

    /// Sets the length of work time period in minutes.
    #[arg(short = 'w', long = "work", default_value_t = DEFAULT_WORK_TIME)]
    work_time: u64,

    /// Sets the length of short break in minutes after each work period elapses.
    #[arg(short = 's', long = "shortbreak", default_value_t = DEFAULT_SHORT_BREAK_TIME)]
    short_break_time: u64,

    /// Sets the length of long break in minutes after all work period completes.
    #[arg(short = 'l', long = "longbreak", default_value_t = DEFAULT_LONG_BREAK_TIME)]
    long_break_time: u64,
}

impl Default for PomoFocusCli {
    fn default() -> Self {
        Self::new()
    }
}
impl PomoFocusCli {
    pub fn new() -> Self {
        Self {
            command: None,
            work_time: DEFAULT_WORK_TIME,
            short_break_time: DEFAULT_SHORT_BREAK_TIME,
            long_break_time: DEFAULT_LONG_BREAK_TIME,
            verbose: Verbosity::new(1, 0),
        }
    }
}

#[derive(Debug)]
pub struct NotificationManager {
    id: u16,
    description: String,
    work_time: u16,
    short_break_time: u16,
    long_break_time: u16,
    created_at: DateTime<Utc>,
    work_expired_at: DateTime<Utc>,
    break_expired_at: DateTime<Utc>,
    body: String,
    icon: &'static str,
    /// In milliseconds
    timeout: u32,
    appname: &'static str,
}

// TODO: Return or modify or parse Cli clap args.
pub fn dialoguer_main(cli: &PomoFocusCli) -> miette::Result<NotificationManager> {
    let created_at: DateTime<Utc> = Utc::now();

    let theme =
        ColorfulTheme { values_style: Style::new().yellow().dim(), ..ColorfulTheme::default() };
    println!("Welcome to the setup wizard");

    let id = 1;
    let work_expired_at = created_at + Duration::minutes(cli.work_time as i64);
    let break_expired_at = work_expired_at + Duration::minutes(cli.short_break_time as i64);

    let mut args = NotificationManager {
        id,
        description: String::from("Work session over"),
        work_time: cli.work_time as u16,
        short_break_time: cli.short_break_time as u16,
        long_break_time: cli.long_break_time as u16,
        created_at,
        work_expired_at,
        break_expired_at,
        body: format!("{:#?},{}", id, work_expired_at),
        icon: "alarm",
        timeout: 2000,
        appname: "pompom",
    };

    args.work_time = Input::with_theme(&theme).with_prompt("Enter work time").interact().unwrap();

    if Confirm::with_theme(&theme)
        .with_prompt("Do you want to edit break times?")
        .interact()
        .unwrap()
    {
        args.short_break_time = Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Enter short break time")
            .interact()
            .unwrap();
        args.long_break_time = Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Enter long break time")
            .interact()
            .unwrap();
        // return Ok(None);
    }

    Ok(args)
}

/// `$ spd-say "'$val' session done"`
fn notify_elapsed_time(sh: &Shell, arg_curr_progress: Option<String>) {
    match arg_curr_progress {
        Some(arg) => {
            let arg = Some(arg);
            cmd!(sh, "spd-say {arg...}").run().unwrap();
        }
        None => (),
    }
}

//-----------------------------------------------------------------------------

#[allow(unused)]
mod tbd {
    fn spinner() {
        let spinner = indicatif::ProgressBar::new_spinner();
        let interval = std::time::Duration::from_millis(1000);
        spinner.enable_steady_tick(interval);
        println!("Feedback: How was your work session?");
        std::thread::sleep(std::time::Duration::from_millis(2000));
        spinner.finish();
        log::debug!("{:?}", spinner.elapsed());
    }

    fn get_feedback() {
        if let Some(rv) = dialoguer::Editor::new().edit("The work session was ...").unwrap() {
            println!("{}", rv);
        } else {
            println!("Abort!");
        }
    }

    fn get_default_editor() -> std::ffi::OsString {
        if let Some(prog) = std::env::var_os("VISUAL") {
            return prog;
        }
        if let Some(prog) = std::env::var_os("EDITOR") {
            return prog;
        }
        if cfg!(windows) {
            "notepad.exe".into()
        } else {
            "vi".into()
        }
    }
}
