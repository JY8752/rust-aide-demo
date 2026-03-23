use std::sync::Arc;

use anyhow::Context;
use domain::user::repository::UserRepository as UserRepositoryPort;
use infrastructure::{db::user::UserRepository, slack::SlackClient};
use sqlx::PgPool;
use usecase::{notify::Notifier, user::UserUseCase};

#[derive(Clone)]
pub struct AppState {
    user_usecase: Arc<UserUseCase>,
}

impl AppState {
    pub fn new(user_usecase: UserUseCase) -> Self {
        Self {
            user_usecase: Arc::new(user_usecase),
        }
    }

    pub fn from_pool(pool: PgPool) -> Self {
        let user_repository: Arc<dyn UserRepositoryPort + Send + Sync> =
            Arc::new(UserRepository::new(pool));
        let notifier: Arc<dyn Notifier + Send + Sync> = Arc::new(SlackClient::new(
            std::env::var("SLACK_WEBHOOK_URL").unwrap_or_default(),
        ));
        let user_usecase = UserUseCase::new(user_repository, notifier);

        Self::new(user_usecase)
    }

    pub async fn from_database_url(database_url: &str) -> anyhow::Result<Self> {
        let pool = PgPool::connect(database_url)
            .await
            .context("failed to connect to database")?;

        Ok(Self::from_pool(pool))
    }

    pub fn user_usecase(&self) -> &UserUseCase {
        self.user_usecase.as_ref()
    }
}
