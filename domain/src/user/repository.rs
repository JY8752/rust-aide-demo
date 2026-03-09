use crate::user::{User, UserId};
use async_trait::async_trait;

#[async_trait]
pub trait UserRepository {
    async fn save(&self, user: &User) -> anyhow::Result<()>;
    async fn find_by_id(&self, id: &UserId) -> anyhow::Result<Option<User>>;
}
