use clap::{
    command,
    Parser,
};

pub fn run(config: miette::Result<PomodoroConfig>) -> miette::Result<()> {
    dbg!(&config?);
    Ok(())
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about=None, term_width=0)]
pub struct PomodoroConfig {
    #[arg(short = 'w', long = "work", default_value = "25")]
    /// Sets the length of work time period in minutes.
    work_time: u64,

    /// Sets the length of short break in minutes after each work period elapses.
    #[arg(short = 's', long = "shortbreak", default_value = "5")]
    short_break_time: u64,

    /// Sets the length of long break in minutes after all work period completes.
    #[arg(short = 'l', long = "longbreak", default_value = "20")]
    long_break_time: u64,
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
