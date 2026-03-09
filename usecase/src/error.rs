#[derive(Debug, thiserror::Error)]
pub enum ApplicationError {
    #[error(transparent)]
    DomainError(#[from] domain::Error),
    #[error(transparent)]
    SystemError(#[from] anyhow::Error),
}
