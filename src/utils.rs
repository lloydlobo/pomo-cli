use atty::Stream;

use crate::error::PomoLibError;

/// [See also](https://crates.io/crates/atty)
pub struct Tty(pub Stream);

impl Tty {
    pub fn are_you_tty(self) -> bool {
        if atty::is(self.0) {
            println!("I'm a terminal");
            true
        } else {
            println!("I'm not a terminal");
            false
        }
    }
}

#[cfg(target_os = "linux")]
pub fn clear_terminal() -> Result<std::process::ExitStatus, PomoLibError> {
    match std::process::Command::new("clear").status() {
        Ok(it) => Ok(it),
        Err(err) => Err(PomoLibError::IoError(err)),
    }
}

#[cfg(target_os = "macos")]
pub fn clear_terminal() -> Result<std::process::ExitStatus, PomoLibError> {
    match std::process::Command::new("clear").status() {
        Ok(it) => Ok(it),
        Err(err) => Err(PomoLibError::IoError(err)),
    }
}

#[cfg(target_os = "windows")]
pub fn clear_terminal() -> Result<std::process::ExitStatus, PomoLibError> {
    match std::process::Command::new("cls").status() {
        Ok(it) => Ok(it),
        Err(err) => Err(PomoLibError::IoError(err)),
    }
}
