//
// Code copied and slightly modified from [24seconds/rust-cli-pomodoro](https://github.com/24seconds/rust-cli-pomodoro).
//

use std::{
    error::Error,
    fmt,
    io,
    result,
};

use bincode::error::{
    DecodeError,
    EncodeError,
};
use notify_rust::error::Error as NotifyRustError;
use reqwest::Error as ReqwestError;
use serde_json::error::Error as SerdeJsonError;

pub type NotifyResult = result::Result<(), NotificationError>;

pub enum PomodoroError {
    NotificationError,
    ConfigurationError,
    UdsHandlerError,
    UserInputHandlerError,
    ParseError,
}

/// NotificationError enumerates all errors related to notification.
#[derive(Debug)]
pub enum NotificationError {
    Desktop(NotifyRustError),
    EmptyConfiguration,
    NewNotification(ParseError),
    DeletionFail(String),
}

impl From<NotifyRustError> for NotificationError {
    fn from(v: NotifyRustError) -> Self {
        Self::Desktop(v)
    }
}

impl fmt::Display for NotificationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NotificationError::Desktop(_) => write!(f, "NotificationError::Desktop"),
            NotificationError::EmptyConfiguration => write!(f, "configuration is empty"),
            NotificationError::NewNotification(err) => {
                write!(f, "failed to get new notification: {}", err)
            }
            NotificationError::DeletionFail(msg) => write!(f, "{}", msg),
        }
    }
}

impl std::error::Error for NotificationError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            NotificationError::Desktop(ref e) => Some(e),
            NotificationError::EmptyConfiguration => None,
            NotificationError::NewNotification(ref e) => Some(e),
            NotificationError::DeletionFail(_) => None,
        }
    }
}

#[derive(Debug)]
pub struct ParseError {
    pub message: Option<String>,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.message {
            Some(msg) => write!(f, "error occurred while parsing: {}", msg),
            None => write!(f, "error occurred while parsing"),
        }
    }
}

impl std::error::Error for ParseError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

impl ParseError {
    fn new(message: String) -> Self {
        Self { message: Some(message) }
    }
}
