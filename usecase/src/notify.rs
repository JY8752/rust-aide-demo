use crate::error::ApplicationError;

pub trait Notifier {
    fn notify(&self, message: &str) -> Result<(), ApplicationError>;
}
