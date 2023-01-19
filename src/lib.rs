#![doc(
    html_logo_url = "https://raw.githubusercontent.com/lloydlobo/pompom/main/assets/pompom_logo_dark.png"
)]
#![doc = include_str!("../README.md")]
#![forbid(unsafe_code)]
//
// Code copied and slightly modified from [PrismaPhonic/Pomodoro](https://github.com/PrismaPhonic/Pomodoro/blob/master/src/lib.rs).
use std::{
    ffi::OsString,
    io::{
        self,
        stdin,
        Read,
        Stdin,
        Write,
    },
    time::Instant,
};

use clap::{
    command,
    Parser,
    Subcommand,
};
use clap_verbosity_flag::{
    InfoLevel,
    LogLevel,
    Verbosity,
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
    Result,
    SourceSpan,
};
use pretty_env_logger::env_logger::Builder;
use spinners::{
    Spinner,
    Spinners,
};
use thiserror::Error;

//------------------------------------------------------

pub fn run(mut config: Result<PompomConfig>) -> Result<()> {
    // ! Disable pretty_env_log Builder this when in production!!!!
    build_env_logger(&mut config.as_mut().unwrap())?;

    load_spinner(None)?;

    let instant_now: Instant = Instant::now();
    let cmd = config.as_ref().unwrap();

    if cmd.command == Some(CliCommands::Interactive) || cmd.command == Some(CliCommands::I) {
        run_tui(config)?;
    } else {
        run_cli(config)?;
    }

    println!("\nDone in {:?}!\nGoodbye!", instant_now.elapsed());
    Ok(())
}

fn build_env_logger(cli: &mut PompomConfig) -> Result<()> {
    let mut builder = Builder::from_env("RUST_LOG");
    builder
        .filter(None, log::LevelFilter::Info)
        .filter_level(cli.verbose.log_level_filter())
        .filter_level(log::LevelFilter::Debug)
        .format_indent(Some(4))
        .format_module_path(true)
        .write_style(pretty_env_logger::env_logger::WriteStyle::Always)
        .init();

    log::error!("Engines exploded");
    log::warn!("Engines smoking");
    log::info!("Engines exist");
    log::debug!("Engine temperature is 200 degrees");
    log::trace!("Engine subsection is 300 degrees");

    Ok(())
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
// [See](https://github.com/clap-rs/clap/blob/master/examples/git-derive.rs)
#[derive(Parser, Debug, Clone)] // requires `derive` feature
#[command(name = "pompom")]
#[command(author, version, about, long_about = None, term_width=0)]
pub struct PompomConfig {
    #[command(subcommand)]
    command: Option<CliCommands>,

    /// By default, this will only report errors.
    /// - `-q` silences output
    /// - `-v` show warnings - `-vv` show info - `-vvv` show debug - `-vvvv` show trace
    /// `verbose: Verbosity::new(1, 0),` -> show warnings , output not silenced.
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
impl Default for PompomConfig {
    fn default() -> Self {
        Self::new()
    }
}
impl PompomConfig {
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

pub struct PomodoroSession<R, W> {
    stdin: R,
    stdout: W,
    width: u16,
    height: u16,
    pompom_tracker: TrackerState,
    clock: Clock,
    config: PompomConfig,
}

// impl<R, W> Fn for PomodoroSession<R, W> {
// }

// [See also](https://doc.rust-lang.org/error_codes/E0277.html)
impl<R, W> Read for PomodoroSession<R, W>
where
    R: Read,
    W: Write,
{
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.stdin.read(buf)
    }
}
/*

fn()->Stdin{stding}:std::io::Read;

error[E0599]: the method `start` exists for struct `PomodoroSession<fn() -> Stdin {stdin}, Stdout>`, but its trait bounds were not satisfied
   --> src/lib.rs:419:19
    |
169 | pub struct PomodoroSession<R, W> {
    | -------------------------------- method `start` not found for this struct
...
419 |     pompom_screen.start();
    |                   ^^^^^ method cannot be called on `PomodoroSession<fn() -> Stdin {stdin}, Stdout>` due to unsatisfied trait bounds
    |
note: trait bound `fn() -> Stdin {stdin}: std::io::Read` was not satisfied
   --> src/lib.rs:202:8
    |
200 | impl<R, W> PomodoroSession<R, W>
    |            ---------------------
201 | where
202 |     R: Read,
    |        ^^^^ unsatisfied trait bound introduced here



*/

// impl<R: std::ops::Deref, W: std::ops::Deref> std::ops::Deref for PomodoroSession<R, W> {
//     type Target = R;

//     fn deref(&self) -> &Self::Target {
//         &self.stdin
//     }
// }
impl<R, W> PomodoroSession<R, W>
where
    R: Read,
    W: Write,
{
    pub fn start(&mut self) {
        write!(self.stdout, "{}", cursor::Hide);
        self.display_menu(Some(POMPOM_PROMPT_START));
    }

    pub fn display_menu(&mut self, menu: Option<&'static str>) {
        let menu = if let Some(menu) = menu { menu } else { POMPOM_MENU };
        match self.wait_for_next_command() {
            Command::Start => self.begin_cycle(),
            Command::Quit => return,
            Command::Reset | Command::None => (),
        }
    }

    /// Wats for the next user command while in a loop.
    /// User command occurs between various `PompomState` pomodoro states.
    // self.stdin.read_exact(&mut buf).unwrap()
    fn wait_for_next_command(&mut self) -> Command {
        let mut command = Command::None;

        while let Command::None = command {
            let mut buf = [0];
            self.stdin.read(&mut buf).unwrap();
            command = match buf[0] {
                b's' => Command::Start,
                b'r' => Command::Reset,
                b'q' => Command::Quit,
                _ => continue,
            }
        }

        command
    }

    fn begin_cycle(&mut self) {
        self.start_work();
        self.display_menu(None);
    }

    fn start_work(&mut self) {
        self.pompom_tracker.set_work_state();
        self.clock.set_time_minutes(self.config.work_time);
        self.countdown();
    }

    fn countdown(&mut self) {
        todo!()
    }
}

/// `Command` enumerates command types matched to user keystrokes.
pub enum Command {
    Start,
    Reset,
    Quit,
    None,
}

/// `PompomState` enumerates `pompom`'s pomodoro state.
#[derive(Debug)]
enum PompomState {
    Working,
    ShortBreak,
    LongBreak,
    None,
}

/// Clock structure that displays minutes and seconds,
/// while drawing border around current time.
#[derive(Debug, Default)]
pub struct Clock {
    minutes: u64,
    seconds: u64,
}

impl Clock {
    /// Creates a new [`Clock`] instance at `00:00`.
    pub fn new() -> Self {
        Self { minutes: 0u64, seconds: 0u64 }
    }

    pub fn get_time(&self) -> String {
        format!("{:02}:{:02}", self.minutes, self.seconds)
    }

    /// # Examples
    ///
    /// ```
    /// use pompom::Clock;
    /// # fn main() {
    /// let clock = Clock::default();
    /// let clock_display = clock.gen_clock("Hello, world!");
    /// let expect = r#"
    /// ╭───────────────────────────────────────╮
    /// │                                       │
    /// │             Hello, world!             │
    /// │                 00:00                 │
    /// │                                       │
    /// ╰───────────────────────────────────────╯
    /// "#;
    /// assert_eq!(clock_display, expect);
    /// let clock_display = clock.gen_clock("Hello");
    /// let expect = r#"
    /// ╭───────────────────────────────────────╮
    /// │                                       │
    /// │             Hello             │
    /// │                 00:00                 │
    /// │                                       │
    /// ╰───────────────────────────────────────╯
    /// "#;
    /// assert_eq!(clock_display, expect);
    /// # }
    /// ```
    pub fn gen_clock(&self, message: &str) -> String {
        let clock_display = format!(
            "
╭───────────────────────────────────────╮
│                                       │
│             {}             │
│                 {}                 │
│                                       │
╰───────────────────────────────────────╯
",
            message,
            self.get_time()
        );

        clock_display
    }

    /// Sets absolute milliseconds for the clock's time.
    fn set_time_ms(&mut self, ms: u64) {
        self.minutes = (ms / (1_000 * 60)) % 60;
        self.seconds = (ms / 1_000) % 60;
    }

    /// Sets absolute minutes for the clock's time.
    fn set_time_minutes(&mut self, minutes: u64) {
        self.set_time_ms(minutes * 60_000_u64)
    }
}

/// * Tracks various [`PompomState`] from `Working` to `None`, ('1' to '4 or None'). `None` state
///   indicates that first pompom timer state hasn't started yer.
/// * Tracks the instant when the current `pompom` was started at. `started_instant` - None between
///   pomodoros.
#[derive(Debug)]
pub struct TrackerState {
    current_order: Option<u32>,
    current_state: PompomState,
    started_instant: Option<Instant>,
}

impl TrackerState {
    pub fn new() -> Self {
        Self { current_order: None, current_state: PompomState::None, started_instant: None }
    }

    fn set_work_state(&self) {
        todo!()
    }
}

//------------------------------------------------------

fn load_spinner(time: Option<std::time::Duration>) -> Result<()> {
    let time = if let Some(t) = time { t } else { std::time::Duration::from_millis(2000) };
    let message = format!("pompom starting in {} seconds", time.as_millis() / 1000);

    let mut sp = Spinner::new(Spinners::Clock, message);
    std::thread::sleep(time);
    Ok::<(), Report>(sp.stop()).wrap_err("Failed to stop spinner")
}

/// Run `pompom`'s pomodoro session from start to finish.
///
/// # Panics
///
/// Panics if .
///
/// # Errors
///
/// This function will return an error if .
fn run_cli(config: Result<PompomConfig, Report>) -> Result<(), PompomError> {
    log::info!("{:#?}", config.as_ref().unwrap());
    let mut stdout = io::stdout();
    let (width, height) = buffer_size()?;
    // let config = config.unwrap();

    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

    terminal::enable_raw_mode()?;
    queue!(
        stdout,
        style::ResetColor,
        terminal::Clear(ClearType::All),
        cursor::Hide,
        cursor::MoveTo(1u16, 1u16)
    )?;
    stdout.flush()?;

    let mut pompom_screen = PomodoroSession {
        stdin,
        stdout,
        width,
        height,
        pompom_tracker: TrackerState::new(),
        clock: Clock::new(),
        config: config.unwrap_or(PompomConfig::default()),
    };

    //? Should this be queue or execute?
    queue!(pompom_screen.stdout, terminal::Clear(ClearType::All), cursor::MoveTo(1u16, 1u16))?;
    pompom_screen.stdout.flush()?;

    // pompom_screen.start();

    // execute!(
    //     stdout,
    //     style::ResetColor,
    //     cursor::MoveTo(1u16, 1u16),
    //     terminal::Clear(ClearType::All),
    //     cursor::Show,
    //     DisableMouseCapture,
    //     terminal::LeaveAlternateScreen
    // )?;

    terminal::disable_raw_mode()?;

    Ok(())
}

fn run_tui(config: Result<PompomConfig, Report>) -> Result<(), Report> {
    let mut stdout = io::stdout();
    let (w, h) = buffer_size()?;
    init(&mut stdout, w, h, config?).wrap_err("Failed to initialize the terminal.")?;
    std::thread::sleep(std::time::Duration::from_millis(450));
    Ok(())
}

//------------------------------------------------------

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

//------------------------------------------------------

fn init<W>(stdout: &mut W, width: u16, height: u16, config: PompomConfig) -> Result<(), PompomError>
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

pub fn buffer_size() -> Result<(u16, u16), PompomError> {
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
