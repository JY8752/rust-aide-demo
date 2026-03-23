use anyhow::Context;
use domain::user::{User, UserId, repository::UserRepository};
use std::sync::Arc;

use crate::{error::ApplicationError, notify::Notifier};

pub struct UserUseCase {
    repository: Arc<dyn UserRepository + Send + Sync>,
    notifier: Arc<dyn Notifier + Send + Sync>,
}

impl UserUseCase {
    pub fn new(
        repository: Arc<dyn UserRepository + Send + Sync>,
        notifier: Arc<dyn Notifier + Send + Sync>,
    ) -> Self {
        Self {
            repository,
            notifier,
        }
    }

    pub async fn create_user(&self, name: &str, email: &str) -> Result<User, ApplicationError> {
        let user = User::new(name, email)?;

        self.repository
            .save(&user)
            .await
            .context("failed to save user")?;

        let user_email: String = user.email().clone().into();
        let message = format!("user created: id={:?}, email={user_email}", user.id());

        self.notifier
            .notify(&message)
            .context("failed to notify user creation")?;

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
