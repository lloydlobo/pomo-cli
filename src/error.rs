//! Error types.
//!
//! You can derive a `Diagnostic` from any `std::error::Error` type.
//!`thiserror` is a great way to define them, and plays nicely with `miette`!

use miette::{
    Diagnostic,
    NamedSource,
    SourceSpan,
};
use thiserror::Error;

// -------------------------------------------------------------------------

// * We highly recommend using something like thiserror to define unique error types and error
//   wrappers for your library.
// * While miette integrates smoothly with thiserror, it is not required. * If you don't want to use
//   the Diagnostic derive macro, you can implement the trait directly, just like with
//   std::error::Error. Then, return this error type from all your fallible public APIs.
// * It's a best practice to wrap any "external" error types in your error enum instead of using
//   something like Report in a library.
#[derive(Error, Diagnostic, Debug)]
pub enum PomoLibError {
    #[error(transparent)]
    #[diagnostic(code(pomo_cli::io_error))]
    IoError(#[from] std::io::Error),

    #[error("Parse from std::env::args_os(), return Err on error.")]
    #[diagnostic(code(pomo_cli::error::PomoLibError))]
    ClapError(#[from] clap::error::Error),

    #[error("Could not parse arguments from stdin")]
    #[diagnostic(code(pomo_cli::cli::CliParseError))]
    BadThingHappened,
}

// -------------------------------------------------------------------------

/* /// `Result` from miette, with the error types defaulting to `pomodoro`'s ['Error`].
pub type Result<T, E = Error> = miette::Result<T, E>;

/// An error returned by an `pomodoro` operation.
pub struct Error {
    // kind: Box<ErrorKind>,
} */

// -------------------------------------------------------------------------

// -------------------------------------------------------------------------

// TODO: add test to error.rs
// #[test]
// fn should_error_send_sync() {
// fn f<T: Send + Sync>() {}
// f::<Error>();
// }

// -------------------------------------------------------------------------

// Code used and modified from [matklad/xshell](https://github.com/matklad/xshell/blob/master/src/error.rs)
/* /// `ErrorKind` enumerates `kind: Box<_>` for `Error`
/// Note: this is intentionally not public.
//
// TODO: Implement ErrorKind enum variants.
//
// CurrentDir { err: io::Error },
// Var { err: env::VarError, var: OsString },
// ReadFile { err: io::Error, path: PathBuf },
// ReadDir { err: io::Error, path: PathBuf },
// WriteFile { err: io::Error, path: PathBuf },
// CopyFile { err: io::Error, src: PathBuf, dst: PathBuf },
// HardLink { err: io::Error, src: PathBuf, dst: PathBuf },
// CreateDir { err: io::Error, path: PathBuf },
// RemovePath { err: io::Error, path: PathBuf },
enum ErrorKind {
    CmdStatus { cmd: CmdData, status: ExitStatus },
    CmdIo { err: io::Error, cmd: CmdData },
    CmdUtf8 { err: FromUtf8Error, cmd: CmdData },
    CmdStdin { err: io::Error, cmd: CmdData },
}

impl From<ErrorKind> for Error {
    fn from(kind: ErrorKind) -> Self {
        let kind = Box::new(kind);
        Error { kind }
    }
}
// #[macro_export]
// macro_rules! cmd {
//     ($sh:expr, $cmd:literal) => {{
//         #[cfg(trick_rust_analyzer_into_highlighting_interpolated_bits)]
//         format_args!($cmd);
//         let f = |prog| $sh.cmd(prog);
//         let cmd: $crate::Cmd = $crate::__cmd!(f $cmd);
//         cmd
//     }};
// } */
//
//
/* impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

impl std::error::Error for Error {} */

/* /// `pub(crate)` constructors, visible only in this crate.
impl Error {
    pub(crate) fn new_cmd_status(cmd: &Cmd<'_>, status: ExitStatus) -> Error {
        let cmd = cmd.data.clone();
        ErrorKind::CmdStatus { cmd, status }.into()
    }

    pub(crate) fn new_cmd_io(cmd: &Cmd<'_>, err: io::Error) -> Error {
        let cmd = cmd.data.clone();
        ErrorKind::CmdIo { err, cmd }.into()
    }

    pub(crate) fn new_cmd_utf8(cmd: &Cmd<'_>, err: io::Error) -> Error {
        let cmd = cmd.data.clone();
        ErrorKind::CmdIo { err, cmd }.into()
    }
} */
