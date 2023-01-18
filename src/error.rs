//! Error types.
//!
//! You can derive a `Diagnostic` from any `std::error::Error` type.
//!`thiserror` is a great way to define them, and plays nicely with `miette`!

use miette::{
    Context,
    Diagnostic,
    NamedSource,
    Report,
    SourceSpan,
};
use thiserror::Error;

use crate::cli;

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

    #[error("Invalid args passed in stdin")]
    #[diagnostic(code(pomo_cli::cli::CliParseError))]
    InvalidArgs(#[from] self::CliParseError),
}

// -------------------------------------------------------------------------

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

// -------------------------------------------------------------------------

/* /// `Result` from miette, with the error types defaulting to `pomodoro`'s ['Error`].
pub type Result<T, E = Error> = miette::Result<T, E>;

/// An error returned by an `pomodoro` operation.
pub struct Error {
    // kind: Box<ErrorKind>,
} */

// -------------------------------------------------------------------------

pub fn handle_invalid_user_args(cli_args: &cli::CliArgs) {
    let advice = "try `pomo_cli --task work --interval 4`\nor `pomo_cli -t break -i 1`";
    if let Some(t) = Some(&cli_args.task) {
        match t.as_str() {
            "work" | "wo" | "break" | "br" => (),
            _ => {
                Err(PomoLibError::InvalidArgs(CliParseError {
                    src: NamedSource::new("cli.rs", format!("{:#?}", cli_args)),
                    bad_bit: (7..11).into(), // (9, 4).into()
                    advice: Some(format!("{}", advice)),
                    related_error: Some(miette::Report::msg(" Invalid pomodoro option")),
                }))
                .wrap_err(advice)
                .unwrap()
            }
        }
    }
}
