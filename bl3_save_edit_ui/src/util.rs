use anyhow::{bail, Result};
use clipboard::{ClipboardContext, ClipboardProvider};
use tracing::error;

use crate::widgets::notification::{Notification, NotificationSentiment};

pub trait ErrorExt {
    fn handle_ui_error(&self, message: &str, notification: &mut Option<Notification>);
}

impl<T> ErrorExt for anyhow::Result<T> {
    fn handle_ui_error(&self, message: &str, notification: &mut Option<Notification>) {
        if let Err(e) = self {
            let message = format!("{}: {}", message, e.to_string());

            error!("{}", message);

            *notification = Some(Notification::new(message, NotificationSentiment::Negative));
        }
    }
}

impl ErrorExt for anyhow::Error {
    fn handle_ui_error(&self, message: &str, notification: &mut Option<Notification>) {
        let message = format!("{}: {}", message, self.to_string());

        error!("{}", message);

        *notification = Some(Notification::new(message, NotificationSentiment::Negative));
    }
}

pub fn get_clipboard_contents() -> Result<String> {
    match ClipboardProvider::new().and_then(|mut ctx: ClipboardContext| ctx.get_contents()) {
        Ok(contents) => Ok(contents),
        Err(e) => bail!("{}", e.to_string()),
    }
}

pub fn set_clipboard_contents(contents: String) -> Result<()> {
    match ClipboardProvider::new().and_then(|mut ctx: ClipboardContext| ctx.set_contents(contents))
    {
        Ok(_) => Ok(()),
        Err(e) => bail!("{}", e.to_string()),
    }
}
