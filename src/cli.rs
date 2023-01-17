use clap::{
    self,
    arg,
    command,
    Parser,
};
use miette::{
    Context,
    Diagnostic,
    ErrReport,
    IntoDiagnostic,
    NamedSource,
    Report,
    SourceSpan,
};
use thiserror::Error;

#[derive(Default, Parser, Debug, Clone)]
#[command(author,version,about,long_about = None)]
pub struct CliArgs {
    /// Name of task session.
    #[arg(short, long)]
    pub task: String,

    /// Number of interval cycles to run pomodoro.
    #[arg(short, long, default_value_t = 1)]
    pub intervals: u8,
}

/// Parse from `std::env::args_os()`, return Err on error.
///
/// Calling this on a type implementing [`Diagnostic`] will reduce it to the common denominator
/// of [`std::error::Error`]. Meaning all extra information provided by [`Diagnostic`] will be
/// inaccessible. If you have a type implementing [`Diagnostic`] consider simply returning it or
/// using [`Into`] or the [`Try`](std::ops::Try) operator (`?`).
///
/// Wrap the error value with a new adhoc error
pub fn get_user_args() -> miette::Result<CliArgs, miette::Error> {
    Ok(CliArgs::try_parse().into_diagnostic().wrap_err("Failed to parse user args")?)
}
// Now let's define a function!
// Use this `Result` type (or its expanded version) as the return type throughout your app (but NOT
// your libraries! Those should always return concrete types!).
//
pub fn get_user_args_fails() -> miette::Result<CliArgs, CliParseError> {
    match CliArgs::try_parse().into_diagnostic() {
        Ok(it) => Ok(it),
        Err(err) => {
            return Err(CliParseError {
                src: NamedSource::new("cli.rs", format!("{:#?}", CliArgs::try_parse().as_ref())),
                bad_bit: (9, 4).into(),
                advice: Some(format!(
                    "{}",
                    r#"Try: `pomo_cli --task Work --interval 4`"#.to_string()
                )),
                related_error: Some(err),
            })?;
        }
    }
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
    /// Snippets and highlights can be included in the diagnostic!
    #[label("This bit here from CliParseError")]
    pub bad_bit: SourceSpan,
    #[help]
    pub advice: Option<String>,
    #[related]
    related_error: Option<Report>,
}

/* #[proc_macro_derive(
    Diagnostic,
    attributes(diagnostic, source_code, label, related, help, diagnostic_source)
)] */
