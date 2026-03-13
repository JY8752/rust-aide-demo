use std::sync::Arc;

use anyhow::Context;
use domain::user::repository::UserRepository as UserRepositoryPort;
use infrastructure::db::user::UserRepository;
use sqlx::PgPool;
use usecase::user::UserUseCase;

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
        let user_usecase = UserUseCase::new(user_repository);

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
