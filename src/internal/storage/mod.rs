pub mod user_storage;

use crate::internal::constant::{NewUserModel, UserModel};
use uuid::Uuid;

#[async_trait::async_trait]
pub trait UserRepository: Send + Sync {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<UserModel>, sqlx::Error>;
    async fn create_user(&self, new_user: NewUserModel) -> Result<UserModel, sqlx::Error>;
    async fn find_by_username(&self, username: String) -> Result<Option<UserModel>, sqlx::Error>;
}
