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
    Ok(())
}
