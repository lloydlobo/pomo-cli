#[cfg(target_os = "macos")]
use std::process::Command;
use std::{
    process,
    sync::Arc,
};

use clap::Parser;
use miette::{
    Context,
    IntoDiagnostic,
};
#[cfg(target_os = "linux")]
use notify_rust::Hint;
use notify_rust::{
    Notification as NR_Notification,
    Timeout as NR_Timeout,
};
pub use pompom::{
    self,
    error::{
        NotificationError,
        NotifyResult,
    },
    notification::notify::*,
    PompomConfig,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let summary_message: &'static str = "lorem";
    let body_message: &'static str = "ipsum";
    notify_desktop(summary_message, body_message).await.unwrap();

    notify_rust::Notification::new()
        .summary("Work session over")
        .body("This will almost look like a real firefox notification.")
        .icon("firefox")
        .show()?;

    let cfg = match pompom::PompomConfig::try_parse()
        .into_diagnostic()
        .wrap_err("Failed to parse command line arguments in main")
    {
        Ok(t) => t,
        Err(e) => {
            eprintln!("oops: {:?}", e);
            process::exit(1);
        }
    };

    match pompom::run(Ok(cfg)) {
        Err(e) => {
            eprintln!("Application error: {}", e);
            log::error!("Application error: {}", e);
            process::exit(1);
        }
        Ok(_) => {
            log::info!("Application finished successfully")
        }
    }
    Ok(())
}

// pub async fn notify_desktop_simple() -> NotifyResult {
//     let mut notification = notify_rust::Notification::new();
//     notification
//         .summary("Work session over")
//         .body("This will almost look like a real firefox notification.")
//         .icon("firefox")
//         .appname("pompom")
//         .timeout(NR_Timeout::Milliseconds(5000));

//     #[cfg(target_os = "linux")]
//     notification.hint(Hint::Category("im.received".to_owned())).sound_name("message-new-instant"
// );

//     notification.show().map(|_| ()).map_err(NotificationError::Desktop)
// }
