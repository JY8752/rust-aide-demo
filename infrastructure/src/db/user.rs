use async_trait::async_trait;
use domain::user::{User, UserId};
use sqlx::PgPool;
use uuid::Uuid;

pub struct UserRepository {
    pool: PgPool,
}

impl UserRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl domain::user::repository::UserRepository for UserRepository {
    async fn save(&self, user: &User) -> anyhow::Result<()> {
        let id: Uuid = user.id().clone().into();
        let name: String = user.name().clone().into();
        let email: String = user.email().clone().into();
        sqlx::query!(
            "INSERT INTO users (id, name, email) VALUES ($1, $2, $3)",
            id,
            name,
            email,
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }
    async fn find_by_id(&self, id: &UserId) -> anyhow::Result<Option<User>> {
        let id: Uuid = id.clone().into();
        let row = sqlx::query!("SELECT id, name, email FROM users WHERE id = $1", id)
            .fetch_optional(&self.pool)
            .await?;
        if let Some(row) = row {
            return Ok(Some(User::reconstruct(row.id, row.name, row.email)));
        }
        Ok(None)
    }
}
