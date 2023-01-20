use std::{
    env,
    ffi::OsString,
};

use clap::Parser;
use miette::{
    Context,
    IntoDiagnostic,
};
use pompom::*;

#[tokio::main]
async fn main() -> miette::Result<()> {
    let cli = PomoFocusCli::try_parse()
        .into_diagnostic()
        .wrap_err("Failed to parse command line arguments");
    run(cli?).await?;

    let spinner = indicatif::ProgressBar::new_spinner();
    let interval = std::time::Duration::from_millis(1000);
    spinner.enable_steady_tick(interval);
    println!("Feedback: How was your work session?");
    std::thread::sleep(std::time::Duration::from_millis(2000));
    spinner.finish();
    if let Some(rv) = dialoguer::Editor::new().edit("The work session was ...").unwrap() {
        println!("{}", rv);
    } else {
        println!("Abort!");
    }
    log::debug!("{:?}", spinner.elapsed());
    Ok(())
}

fn get_default_editor() -> OsString {
    if let Some(prog) = env::var_os("VISUAL") {
        return prog;
    }
    if let Some(prog) = env::var_os("EDITOR") {
        return prog;
    }
    if cfg!(windows) {
        "notepad.exe".into()
    } else {
        "vi".into()
    }
}
