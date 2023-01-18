//! # pompom
//!
//! This crate is a simple pomodoro cli based terminal timer.
//!
//! # Dependencies
//!
//! * Works on Linux and MacOS. Currently unimplemented for Windows except for WSL.
// TODO: Install `libdbus-1` to integrate pompom with linux's notification system.
//! # Installation
//!
//! Works both on stable and nightly rust.
//!
//! Install the application with the command:
//!
//! *Stable*
//! ```terminal
//! $ cargo install pompom
//! ```
//!
//! *Nightly*
//! ```terminal
//! $ cargo +nightly install pompom
//! ```
//! # Using pompom
//!
//! Run pompom directly in your terminal. It currently defaults to:
//! * 25 minutes of work time.
//! * 5 minutes of break.
//! * 20 minutes of long break.
//!
//! ```terminal
//! $ pompom
//! ```
//!
//! # Flags
//!
//! To customize the pompom settings, you can pass flags into the terminal.
//! * `-w` | `--work` - sets the work time.
//! * `-s` | `--shortbreak` - sets the short break time.
//! * `-l` | `--longbreak` - sets the long break time.
//!
//! # Examples
//!
//! ```terminal
//! $ pompom -w 45 -s 15 -l 25
//! ```
//!
//! ```terminal
//! $ pompom --work 45 --shortbreak 15 --longbreak 25
//! ```

use std::{
    ffi::OsString,
    io::{
        self,
        stdin,
        Write,
    },
    time::Instant,
};

use clap::{
    command,
    Parser,
    Subcommand,
};
use crossterm::{
    cursor,
    event::{
        self,
        DisableMouseCapture,
        EnableMouseCapture,
        Event,
        KeyCode,
        KeyEvent,
        KeyModifiers,
    },
    execute,
    queue,
    style,
    style::Print,
    terminal::{
        self,
        ClearType,
        EnterAlternateScreen,
    },
};
use log::{
    debug,
    info,
};
use miette::{
    Context,
    Diagnostic,
    NamedSource,
    Report,
    SourceSpan,
};
use pretty_env_logger::env_logger::Builder;
use spinners::{
    Spinner,
    Spinners,
};
use thiserror::Error;

//------------------------------------------------------

pub fn run(config: miette::Result<PompomConfig>) -> miette::Result<()> {
    // ! Disable pretty_env_log Builder this when in production!!!!
    let mut builder = Builder::from_env("RUST_LOG");
    builder
        .filter(None, log::LevelFilter::Info)
        .filter_level(log::LevelFilter::Debug)
        .format_indent(Some(4))
        .format_module_path(true)
        .write_style(pretty_env_logger::env_logger::WriteStyle::Always)
        .init();

    load_spinner(None)?;

    let instant_now: Instant = Instant::now();
    let cmd = config.as_ref().unwrap();

    if cmd.command == Some(Commands::Interactive) || cmd.command == Some(Commands::I) {
        run_tui(config)?;
    } else {
        run_cli(config)?;
    }

    println!("\nDone in {:?}!\nGoodbye!", instant_now.elapsed());
    Ok(())
}

fn load_spinner(time: Option<std::time::Duration>) -> miette::Result<()> {
    let time = if let Some(t) = time { t } else { std::time::Duration::from_millis(2000) };
    let message = format!("pompom starting in {} seconds", time.as_millis() / 1000);

    let mut sp = Spinner::new(Spinners::Clock, message);
    std::thread::sleep(time);
    Ok::<(), Report>(sp.stop()).wrap_err("Failed to stop spinner")
}

fn run_cli(config: Result<PompomConfig, Report>) -> Result<(), Report> {
    log::info!("{:#?}", config.unwrap());
    Ok(())
}

fn run_tui(config: Result<PompomConfig, Report>) -> Result<(), Report> {
    let mut stdout = io::stdout();
    let (w, h) = buffer_size()?;
    init(&mut stdout, w, h, config?).wrap_err("Failed to initialize the terminal.")?;
    std::thread::sleep(std::time::Duration::from_millis(450));
    Ok(())
}

#[derive(Debug, Subcommand, PartialEq)]
pub(crate) enum Commands {
    /// Usage: $ pompom interactive
    #[command(arg_required_else_help = false)]
    Interactive,

    /// Usage: $ pompom interactive
    #[command(arg_required_else_help = false)]
    I,
    // /// Interactive OsString Vector
    // #[command(external_subcommand)]
    // Interactive(Vec<OsString>),
}

/// `pompom` CLI terminal flags with settings.
// [See](https://github.com/clap-rs/clap/blob/master/examples/git-derive.rs)
#[derive(Parser, Debug)] // requires `derive` feature
#[command(name = "pompom")]
#[command(author, version, about, long_about = None, term_width=0)]
pub struct PompomConfig {
    #[command(subcommand)]
    command: Option<Commands>,

    /// Sets the length of work time period in minutes.
    #[arg(short = 'w', long = "work", default_value = "25")]
    work_time: u64,

    /// Sets the length of short break in minutes after each work period elapses.
    #[arg(short = 's', long = "shortbreak", default_value = "5")]
    short_break_time: u64,

    /// Sets the length of long break in minutes after all work period completes.
    #[arg(short = 'l', long = "longbreak", default_value = "20")]
    long_break_time: u64,
}

//------------------------------------------------------

// Code copied and slightly modified from [PrismaPhonic/Pomodoro](https://github.com/PrismaPhonic/Pomodoro/blob/master/src/lib.rs).
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

//------------------------------------------------------

/// Menu of `pompom`.
const POMPOM_MENU: &'static str = "
╔═════════════════╗
║───┬ pompom──────║
║ s ┆ start next  ║
║ q ┆ quit        ║
╚═══╧═════════════╝";

/// Welcome menu at the start of pompom.
pub const POMPOM_PROMPT_START: &'static str = "
╔══════════════════════════════╗
║───Start your first pompom!──-║
║──────────────────────────────║
║ s ┆ start    Press s         ║
║ q ┆ quit     to start!       ║
║ r ┆ reset                    ║
╚═══╧══════════════════════════╝";

/// Ever present layout controls while the clock is running.
pub const CONTROLS: &'static str = "
------controls------
 q    ~ quit current
 r    ~ reset all
";

const TEST_MENU: &str = r#"Crossterm interactive test
Controls:
 - 'q' - quit interactive test (or return to this menu)
 - any other key - continue with next step
Available tests:
1. cursor
2. color (foreground, background)
3. attributes (bold, italic, ...)
4. input
Select test to run ('1', '2', ...) or hit 'q' to quit.
"#;

/// Active clock ping sound.
#[cfg(target_os = "macos")]
static SOUND: &'static str = "Ping";

/// Active clock ping sound.
#[cfg(all(unix, not(target_os = "macos")))]
static SOUND: &'static str = "alarm-clock-elapsed";

const DEFAULT_WORK_TIME: u64 = 15;
const DEFAULT_SHORT_BREAK_TIME: u64 = 5;
const DEFAULT_LONG_BREAK_TIME: u64 = 25;

//------------------------------------------------------

fn init<W>(
    stdout: &mut W,
    width: u16,
    height: u16,
    config: PompomConfig,
) -> miette::Result<(), PompomError>
where
    W: Write,
{
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

    terminal::enable_raw_mode()?;

    loop {
        // * Top left cell is represented as `0,0`.
        // * Commands must be executed/queued for execution otherwise they do nothing.
        queue!(
            stdout,
            style::ResetColor,
            terminal::Clear(ClearType::All),
            cursor::Hide,
            cursor::MoveTo(1u16, 1u16)
        )?;

        for line in POMPOM_PROMPT_START.lines() {
            queue!(stdout, style::Print(line), cursor::MoveToNextLine(1))?;
        }

        if let Ok(key_press) = event::read() {
            if let Event::Key(key) = key_press {
                // A command that moves the terminal cursor down the given number of lines, and
                // moves it to the first column.
                // * This command is 1 based, meaning `MoveToNextLine(1)` moves to the next line.
                // * Most terminals default 0 argument to 1.
                // * Commands must be executed/queued for execution otherwise they do nothing.
                // queue!(stdout, cursor::MoveToNextLine(0))?;
                let code_value = match key.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Char('w') => Some(DEFAULT_WORK_TIME),
                    KeyCode::Char('s') => Some(DEFAULT_SHORT_BREAK_TIME),
                    KeyCode::Char('l') => Some(DEFAULT_LONG_BREAK_TIME),
                    _ => None,
                };
                match code_value {
                    Some(val) => {
                        queue!(stdout, style::Print(val), cursor::MoveToNextLine(2))?;
                        stdout.flush()?;
                    }

                    None => {}
                }
            };
        } else {
            continue;
        }
        stdout.flush()?;
        // stdout.flush()?;
    }

    // execute!(stdout, style::ResetColor, cursor::Show, terminal::LeaveAlternateScreen)?;
    execute!(
        stdout,
        style::ResetColor,
        cursor::MoveTo(1u16, 1u16),
        terminal::Clear(ClearType::All),
        cursor::Show,
        DisableMouseCapture,
        terminal::LeaveAlternateScreen
    )?;

    terminal::disable_raw_mode()?;

    Ok(())
}

pub fn buffer_size() -> miette::Result<(u16, u16), PompomError> {
    Ok(terminal::size()?)
}

//------------------------------------------------------

#[derive(Error, Diagnostic, Debug)]
pub enum PompomError {
    #[error(transparent)]
    #[diagnostic(code(pomo_cli::io_error))]
    IoError(#[from] std::io::Error),

    #[error("Parse from std::env::args_os(), return Err on error.")]
    #[diagnostic(code(pomo_cli::error::PomoLibError))]
    ClapError(#[from] clap::error::Error),

    #[error("Could not parse arguments from stdin")]
    #[diagnostic(code(pomo_cli::cli::CliParseError))]
    BadThingHappened,

    #[error("Invalid args passed in stdin")]
    #[diagnostic(code(pomo_cli::cli::CliParseError))]
    InvalidArgs(#[from] self::CliParseError),
}

/// `CLIParseError` checks for errors while parsing terminal stdin inputs from user.
///
/// * miette supports collecting multiple errors into a single diagnostic, and printing them all
///   together nicely. Use the #[related] tag on any IntoIter field in your Diagnostic type:
/// * delayed source code Sometimes it makes sense to add source code to the error message later.
///   One option is to use with_source_code() method for that:
#[derive(Error, Debug, Diagnostic)]
#[error("Failed to parse input from stdin terminal")]
#[diagnostic(
    code(pomo_cli::cli::CliParseError), // Source code
    url(docsrs),                        // A link to rustdoc generated docs
)]
pub struct CliParseError {
    /// Source snippets printed to stdout. Can use String if you don't have/care about file names.
    #[source_code]
    pub src: NamedSource, // miette will use this.
    //
    /// Snippets and highlights can be included in the diagnostic!
    #[label("This bit here from CliParseError")]
    pub bad_bit: SourceSpan,

    #[help]
    pub advice: Option<String>,

    #[related]
    pub related_error: Option<Report>,
}

/// Demonstrates how to match on modifiers like: Control, alt, shift.
///
/// `cargo run --example event-match-modifiers`
//
// Event::FocusGained | Event::FocusLost | Event::Mouse(_) | Event::Paste(_) | Event::Resize(..)
fn match_event(read_event: Event) {
    match read_event {
        // Match on one modifier:
        Event::Key(KeyEvent { modifiers: KeyModifiers::CONTROL, code, .. }) => {
            info!("Control + {:?}", code);
        }
        Event::Key(KeyEvent { modifiers: KeyModifiers::SHIFT, code, .. }) => {
            info!("Shift + {:?}", code);
        }
        Event::Key(KeyEvent { modifiers: KeyModifiers::ALT, code, .. }) => {
            info!("Alt + {:?}", code);
        }
        // Match on multiple modifiers:
        Event::Key(KeyEvent { modifiers, code, .. }) => {
            if modifiers == (KeyModifiers::ALT | KeyModifiers::SHIFT) {
                info!("Alt + Shift {:?}", code);
            } else {
                info!("({:?}) with key: {:?}", modifiers, code);
            }
        }
        _ => (),
    }
}

//------------------------------------------------------

// let mut work_time = config.work_time;
// queue!(
//     stdout,
//     cursor::MoveTo(1u16, TEST_MENU.lines().count() as u16 + 1u16),
//     style::Print("Enter new work time (minutes): "),
//     cursor::MoveToNextLine(1u16),
//     style::Print("> "),
//     cursor::MoveToNextLine(1u16),
//     style::Print("> "),
//     cursor::MoveToNextLine(1u16),
//     style::Print("> "),
//     cursor::MoveToNextLine(1u16),
//     style::Print("> "),
// )?;

//------------------------------------------------------

fn print_new_work_time_queue<W>(stdout: &mut W) -> Result<(), PompomError>
where
    W: Write,
{
    queue!(
        stdout,
        cursor::MoveTo(1u16, TEST_MENU.lines().count() as u16 + 1u16),
        style::Print("Enter new work time (minutes): "),
        cursor::MoveToNextLine(1u16),
        style::Print("> "),
        cursor::MoveToNextLine(1u16),
        style::Print("> "),
        cursor::MoveToNextLine(1u16),
        style::Print("> "),
        cursor::MoveToNextLine(1u16),
        style::Print("> "),
    )?;
    // PERF: Can use stdin to read user input.line
    Ok(())
}
