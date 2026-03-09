use anyhow::Context;
use domain::user::{User, UserId, repository::UserRepository};

use crate::error::ApplicationError;

pub struct UserUseCase<R>
where
    R: UserRepository,
{
    repository: R,
}

impl<R> UserUseCase<R>
where
    R: UserRepository,
{
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn create_user(&self, name: &str, email: &str) -> Result<User, ApplicationError> {
        let user = User::new(name, email).map_err(anyhow::Error::from)?;

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
