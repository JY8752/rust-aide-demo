// dummy implementation
use usecase::{error::ApplicationError, notify::Notifier};

pub struct SlackClient {
    webhook_url: String,
}

impl SlackClient {
    pub fn new(webhook_url: String) -> Self {
        Self { webhook_url }
    }
}

impl Notifier for SlackClient {
    fn notify(&self, message: &str) -> Result<(), ApplicationError> {
        tracing::info!(webhook_url = %self.webhook_url, message = %message, "notifying to slack");
        Ok(())
    }
}
