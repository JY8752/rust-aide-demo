use anyhow::Context;
use domain::user::{User, UserId, repository::UserRepository};
use std::sync::Arc;

use crate::error::ApplicationError;

pub struct UserUseCase {
    repository: Arc<dyn UserRepository + Send + Sync>,
}

impl UserUseCase {
    pub fn new(repository: Arc<dyn UserRepository + Send + Sync>) -> Self {
        Self { repository }
    }

    pub async fn create_user(&self, name: &str, email: &str) -> Result<User, ApplicationError> {
        let user = User::new(name, email)?;

        self.repository
            .save(&user)
            .await
            .context("failed to save user")?;

        Ok(user)
    }

    pub async fn find_user_by_id(&self, id: &UserId) -> Result<Option<User>, ApplicationError> {
        Ok(self
            .repository
            .find_by_id(id)
            .await
            .context("failed to find user by id")?)
    }
}
