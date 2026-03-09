#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Invalid user ID. {0}")]
    InvalidUserId(&'static str),
    #[error("Invalid user name. {0}")]
    InvalidUserName(&'static str),
    #[error("Invalid user email. {0}")]
    InvalidUserEmail(&'static str),
}
