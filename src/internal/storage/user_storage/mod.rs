
use uuid::Uuid;
use sqlx::PgPool;
use crate::internal::storage::UserRepository;
use crate::internal::constant::{UserModel,NewUserModel};




pub struct PostgresUserRepository {
    pool: PgPool,
}

impl PostgresUserRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl UserRepository for PostgresUserRepository {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<UserModel>, sqlx::Error> {
        let user = sqlx::query_as::<_, UserModel>(
            "SELECT id, username, email, created_at, updated_at FROM users WHERE id = $1"
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(user)
    }
    
    
    async fn find_by_username(&self, username :String) -> Result<Option<UserModel>, sqlx::Error> {
        let user = sqlx::query_as::<_, UserModel>(
            "SELECT id, username, email, created_at, updated_at FROM users WHERE username = $1"
        )
        .bind(username)
        .fetch_optional(&self.pool)
        .await?;

        Ok(user)
    }

    async fn create_user(&self, new_user: NewUserModel) -> Result<UserModel, sqlx::Error> {
        let user = sqlx::query_as::<_, UserModel>(
            "INSERT INTO users (username, email) 
             VALUES ($1, $2) 
             RETURNING id, username, email, created_at, updated_at"
        )
        .bind(new_user.username)
        .bind(new_user.email)
        .fetch_one(&self.pool)
        .await?;

        Ok(user)
    }
}

