use chrono::{DateTime, Utc};
use serde::Serialize;
use utoipa::ToSchema;
use uuid::Uuid;



// Used for SELECT queries
#[derive(Debug, Clone, Serialize, sqlx::FromRow, ToSchema)]
pub struct UserModel {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Clone)]
pub struct NewUserModel {
    pub username: String,
    pub email: String,
}
