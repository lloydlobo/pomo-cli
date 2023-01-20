#![doc(
    html_logo_url = "https://raw.githubusercontent.com/lloydlobo/pompom/main/assets/pompom_logo_dark.png"
)]
#![doc = include_str!("../README.md")]
#![forbid(unsafe_code)]
#![allow(unused)]

mod error;

use std::{
    error::Error,
    f32::consts::E,
};

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
    PomodoroError,
};
use miette::Diagnostic;
use notify_rust::{
    Hint,
    Notification,
};
use termcolor::{
    BufferWriter,
    Color,
    ColorChoice,
    ColorSpec,
    StandardStream,
    WriteColor,
};
use xshell::{
    cmd,
    Shell,
};

#[derive(Debug)]
pub struct App {
    pub cli: PomoFocusCli,
    state_manager: StateManager,
}

impl App {
    pub fn new(cli: PomoFocusCli) -> Self {
        let mut state_manager = StateManager::new();
        state_manager.max_count = Some(cli.cycles);

        Self { cli, state_manager }
    }

    pub async fn run(&mut self) -> miette::Result<()> {
        let notification_mangaer = match (&mut self.cli.command) {
            Some(cmd) => match cmd {
                CliCommands::Interactive | CliCommands::I => Some(dialoguer_main(&self.cli)?),
            },
            None => None,
        };
        if let Some(arg) = notification_mangaer {
            self.cli.work_time = arg.work_time as u64;
            self.cli.short_break_time = arg.short_break_time as u64;
        };

        if let Err(eee) =
            self.run_timer_sequence().await.map(|_| ()).map_err(|_| NotificationError::Desktop)
        {
            unimplemented!()
        }

        Ok(())
    }
    async fn run_timer_sequence(&mut self) -> NotifyResult {
        // let m = &mut self.state_manager;
        let cycles_requested: u16 = self.cli.cycles;
        dbg!(&cycles_requested);
        let mut counter = 0;
        for i in (1..=cycles_requested) {
            // dbg!(&m.get_state());
            // dbg!(m.check_next_state());
            // let curr_state = m.get_state();
            &mut self.state_manager.set_next_state();
            &mut self.state_manager.next_counter();

            match &mut &mut self.state_manager.get_state() {
                PomofocusState::Work => {
                    let work_time = self.cli.work_time;
                    Self::prog(work_time);
                }
                PomofocusState::ShortBreak => {
                    let work_time = self.cli.short_break_time;
                    Self::prog(work_time);
                }
                PomofocusState::LongBreak => {
                    let work_time = self.cli.long_break_time;
                    Self::prog(work_time);
                }
                PomofocusState::None => {
                    if self.state_manager.state == PomofocusState::LongBreak {
                        &mut self.state_manager.reset();
                        &mut self.state_manager.reset_counter();
                        // break;
                    }
                }
            }
            counter += i;
            dbg!(&self.cli);
            dbg!(&mut self.state_manager);
        }
        dbg!(&counter);
        dbg!(&mut self.state_manager);
        dbg!(&self.cli);

        Ok(())
    }

    fn prog(work_time: u64) {
        let sh = Shell::new().expect("Shell::new() failed");
        let len_duration: u64 = work_time * 60;
        let duration_sec = std::time::Duration::from_millis(10);
        // default to 1000ms as 1sec.
        let pb = indicatif::ProgressBar::new(len_duration);
        (0..len_duration).for_each(|_| {
            pb.inc(1);
            std::thread::sleep(duration_sec);
        });
        pb.finish_with_message("Pomodoro finished! Take a break!");
    }
}
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
    let duration_sec = std::time::Duration::from_millis(1000); // default to 1000ms as 1sec.
                                                               //
    let arg_duration_work = Some(cli.work_time.to_string());
    cmd!(sh, "echo {arg_duration_work...} minutes").run().unwrap();

    let every_n_minute = |m: u64| m * 60;
    let if_elapsed_spd_say = |i: &u64| match (i) % every_n_minute(5) == 0 && *i != 0 {
        //TODO: Instead of spd-say, use rust_notify::Notification.
        true => Some(format!("{} minutes over", i / 60)),
        false => None,
    };

    // Main pomodoro progress loop!
    (0..len_duration).for_each(|i: u64| {
        pb.inc(1);
        // TODO: Move conditional here to avoid invoking notification for `None` cases.
        notify_elapsed_time(&sh, if_elapsed_spd_say(&i));
        std::thread::sleep(duration_sec);
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

#[derive(Debug, Clone, PartialEq)]
pub enum PomofocusState {
    Work,
    ShortBreak,
    LongBreak,
    /// Default session state between other states.
    None,
}

/// intervals = 3
/// work state 1
/// short break state
/// work state 2
/// short break state
/// work state 3
/// long break state
#[derive(Debug)]
pub struct StateManager {
    pub state: PomofocusState,
    pub counter: Option<u16>,
    pub max_count: Option<u16>,
}

/* pub fn manage_state(&mut self) {
    let state = &mut self.state;
    let counter = &mut self.counter;
    let max_count = self.max_count;
    /*
     * max_count by default = 3 // 3 occurrences of PomofocusState::Work
     * self.new(); // Default state is PomofocusState::None
     *   (counter, state) = (None, PomofocusState:None)
     *
     * If the next state is PomofocusState::Work then increase counter.
     * If the counter is set, then we are in a session.
     * self.next()
     * self.next_counter() -> counter += 1u8
     * PomofocusState::None => PomofocusState::Work,
     *   (counter, state) = (Some(0), PomofocusState:Work)
     *
     * Now work timer elapses
     * self.next()
     * self.next_counter() -> counter += 0u8;
     *   PomofocusState::Work => PomofocusState::ShortBreak,
     *   (counter, state) = (Some(0), PomofocusState:ShortBreak)
     *
     * Now short break timer elapses
     * self.next_counter() -> counter += 1u8
     * self.next();
     *   (counter, state) = (Some(1), PomofocusState:Work)
     *
     * Now work timer elapses
     * self.next()
     * self.next_counter() -> counter += 0u8;
     *   PomofocusState::Work => PomofocusState::ShortBreak,
     *   (counter, state) = (Some(1), PomofocusState:ShortBreak)
     *
     * Now short break timer elapses
     * self.next_counter() -> counter += 1u8
     * self.next();
     *   (counter, state) = (Some(2), PomofocusState:Work)
     *
     * !Max count has been reached [0, 1, 2].count() = 3;
     * So time for LongBreak
     * Now work timer elapses
     * self.next()
     * self.next_counter() -> counter += 0u8;
     *   PomofocusState::Work => PomofocusState::LongBreak, // unpresentable state. have to do manually for now.
     *   (counter, state) = (Some(2), PomofocusState:LongBreak)
     *
     * Reset State
     * self.reset()
     *   (counter, state) = (None, PomofocusState:None)
     *

    */
} */
impl StateManager {
    fn check_next_state(&mut self) -> PomofocusState {
        let next_state = match self.state {
            PomofocusState::Work => PomofocusState::ShortBreak,
            PomofocusState::ShortBreak => PomofocusState::Work,
            PomofocusState::LongBreak => PomofocusState::None,
            PomofocusState::None => PomofocusState::Work,
        };
        next_state
        // self.state = next_state;
    }
    fn get_state(&mut self) -> PomofocusState {
        self.state.to_owned()
    }
    fn is_state_longbreak(&mut self) -> bool {
        match self.get_state() {
            PomofocusState::LongBreak => true,
            _ => false,
            // PomofocusState::LongBreak => { self.state = PomofocusState::None; }
            // _ => {}
            // PomofocusState::Work | PomofocusState::ShortBreak | PomofocusState::None => todo!(),
        }
    }

    fn new() -> Self {
        type Item = PomofocusState;
        Self { state: PomofocusState::None, counter: None, max_count: Some(3) }
    }

    fn next_counter(&mut self) {
        match self.get_state() {
            PomofocusState::Work => {
                self.counter = Some(self.counter.unwrap_or(0) + 1);
            }
            PomofocusState::ShortBreak | PomofocusState::LongBreak => (), /* 1 --> THIS */
            // PomofocusState::None
            // => { self.counter =
            // None; } //1 -->
            // THIS?
            PomofocusState::ShortBreak | PomofocusState::LongBreak | PomofocusState::None => (), /* 2 --> OR THIS? */
        }
        // self.counter = Some(self.counter.unwrap_or(0) + 1);
        // self.next();
    }

    fn reset(&mut self) {
        let is_curr_longbreak = self.is_state_longbreak();
        let is_next_none =
            if let PomofocusState::None = self.check_next_state() { true } else { false };
        assert_eq!(is_curr_longbreak, is_next_none); // then reset.
                                                     //
        self.state = PomofocusState::None;
        self.counter = None;
    }

    fn reset_counter(&mut self) {
        self.counter = None;
    }

    pub fn set_next_state(&mut self) {
        self.state = self.check_next_state();
        self.state_message().expect("StateManager::set_next_state: state_message() failed");
    }

    fn state_message(&self) -> Result<String, Box<dyn std::error::Error>> {
        match self.state {
            PomofocusState::Work => {
                let msg = "💪🏻";
                printer::CliPrinter::new(Some(msg)).write_green()?;
                Ok(String::from(msg))
            }
            PomofocusState::ShortBreak => {
                let msg = "💤🏻";
                printer::CliPrinter::new(Some(msg)).write_yellow()?;
                Ok(String::from(msg))
            }
            PomofocusState::LongBreak => {
                let msg = "💤🏻💤🏻💤🏻";
                printer::CliPrinter::new(Some(msg)).write_red()?;
                Ok(String::from(msg))
            }
            PomofocusState::None => Ok(String::new()),
        }
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
const DEFAULT_WORK_CYCLES: u16 = 3;

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

    /// Sets the count of work cycles before a long break starts. Default: 3.
    #[arg(short = 'c', long = "cycles", default_value_t = DEFAULT_WORK_CYCLES)]
    cycles: u16,
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
            cycles: DEFAULT_WORK_CYCLES,
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

#[allow(unused)]
pub mod printer {
    use std::{
        io,
        io::{
            BufRead,
            BufReader,
            Read,
            Write,
        },
    };

    use termcolor::{
        BufferWriter,
        Color,
        ColorChoice,
        ColorSpec,
        StandardStream,
        WriteColor,
    };

    // TODO: Add stdio stdout feature.
    // pub struct CliPrinter<W:Write,R:Read>(W,R);
    #[derive(Debug, Default)]
    pub struct CliPrinter {
        msg: &'static str,
        stdin: Option<StandardStream>,
        stdout: Option<StandardStream>,
        bufwtr: Option<BufferWriter>,
        /// Write colored text to memory.
        bufmem: Option<termcolor::Buffer>,
    }

    impl CliPrinter {
        pub fn new(msg: Option<&'static str>) -> Self {
            Self {
                msg: if let Some(s) = msg { s } else { "" },
                stdin: None,
                stdout: None,
                bufwtr: None,
                bufmem: None,
            }
        }
        pub fn write_green(&mut self) -> io::Result<()> {
            let mut stdout = StandardStream::stdout(ColorChoice::Always);
            stdout.set_color(ColorSpec::new().set_fg(Some(Color::Green)))?;
            writeln!(&mut stdout, "{0}", self.msg)
        }

        pub fn write_green_buf(&mut self) -> io::Result<()> {
            let mut bufwtr = BufferWriter::stderr(ColorChoice::Always);
            let mut buffer = bufwtr.buffer();
            buffer.set_color(ColorSpec::new().set_fg(Some(Color::Green)))?;
            writeln!(&mut buffer, "{0}", self.msg)?;
            bufwtr.print(&buffer)
        }

        pub fn write_yellow(&mut self) -> io::Result<()> {
            let mut stdout = StandardStream::stdout(ColorChoice::Always);
            stdout.set_color(ColorSpec::new().set_fg(Some(Color::Yellow)))?;
            writeln!(&mut stdout, "{0}", self.msg)
        }
        pub fn write_red(&mut self) -> io::Result<()> {
            let mut stdout = StandardStream::stdout(ColorChoice::Always);
            stdout.set_color(ColorSpec::new().set_fg(Some(Color::Red)))?;
            writeln!(&mut stdout, "{0}", self.msg)
        }
    }
}

/* fn run(&mut self) -> miette::Result<&mut StateManager> {
    // let mut manager = StateManager::new();
    // let mut manager = StateManager::new();
    // let mut shell = Shell::new();
    // let mut notification = Notification::new() .summary("Pomofocus") .body("")
    // .timeout(std::time::Duration::from_secs(5));

    'l: loop {
        let state = self.get_state();
        let _counter = self.counter.unwrap_or(0);
        #[rustfmt::skip]
        let message = match state {
            PomofocusState::Work => {// notification .body(&format!("Work {}", counter)) .timeout(Duration::seconds(5)) .show()?; cmd!("notify-send", "-t", "5000", "Pomofocus Work", "Work") .run()?;
                "Work"
            }
            PomofocusState::ShortBreak => {// notification .body(&format!("Short Break {}", counter)) .timeout(Duration::seconds(5)) .show()?; cmd!("notify-send", "-t", "5000", "Pomofocus Short Break", "Short Break") .run()?;
                "Short Break"
            }
            PomofocusState::LongBreak => {// notification .body(&format!("Long Break {}", counter)) .timeout(Duration::seconds(5)) .show()?; cmd!("notify-send", "-t", "5000", "Pomofocus Long Break", "Long Break") .run()?;
                "Long Break"
            }
            PomofocusState::None => {// notification .body(&format!("No Pomofocus")) .timeout(Duration::seconds(5)) .show()?; cmd!("notify-send", "-t", "5000", "Pomofocus No Pomofocus", "No Pomofocus") .run()?;
                "No Pomofocus"
            }
        };
        self.next_counter();
        println!("{}", message);
        if self.counter == self.max_count {
            break 'l;
        }

        std::thread::sleep(std::time::Duration::from_secs(1));
    }
    Ok(self)
} */
