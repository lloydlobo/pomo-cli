//
// Code copied and slightly modified from [24seconds/rust-cli-pomodoro](https://github.com/24seconds/rust-cli-pomodoro).
//

#[cfg(target_os = "macos")]
use std::process::Command;
use std::sync::Arc;

#[cfg(target_os = "linux")]
use notify_rust::Hint;
use notify_rust::{
    Notification as NR_Notification,
    Timeout as NR_Timeout,
};

use crate::error::{
    NotificationError,
    NotifyResult,
};

pub async fn notify_desktop(
    summary_message: &'static str,
    body_message: &'static str,
) -> NotifyResult {
    let mut notification = NR_Notification::new();
    let notification = notification
        .summary(summary_message)
        .body(body_message)
        .appname("pompom")
        .timeout(NR_Timeout::Milliseconds(5000));

    #[cfg(target_os = "linux")]
    notification.hint(Hint::Category("im.received".to_owned())).sound_name("message-new-instant");

    notification.show().map(|_| ()).map_err(NotificationError::Desktop)
}
