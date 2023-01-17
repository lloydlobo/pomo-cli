use pomo_cli::pomodoro;
use pretty_env_logger::env_logger::Builder;

fn main() -> miette::Result<()> {
    let mut _builder = enable_logging();

    pomodoro()?;

    Ok(())
}

fn enable_logging() -> Builder {
    let mut builder = Builder::from_env("RUST_LOG"); // or for basic: Builder::new();
    builder
        .filter(None, log::LevelFilter::Info)
        .filter_level(log::LevelFilter::Info)
        .format_indent(Some(4)) // Multiline log records indentation.
        .format_module_path(true)
        .write_style(pretty_env_logger::env_logger::WriteStyle::Always)
        .init();

    builder
}
