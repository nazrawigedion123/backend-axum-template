pub mod middleware;
pub mod user_handler;

use crate::internal::constant::dto::{ApiResponse, CreateUserRequest};
use crate::internal::constant::errors::AppError;
use crate::internal::storage::user_storage::UserModel;
use axum::{Json, http::StatusCode};
use uuid::Uuid;

#[async_trait::async_trait]
pub trait UserHandlerTrait: Send + Sync {
    async fn create_user(
        &self,
        payload: CreateUserRequest,
    ) -> Result<(StatusCode, Json<ApiResponse<UserModel>>), AppError>;

    async fn get_user(&self, id: Uuid) -> Result<Json<ApiResponse<UserModel>>, AppError>;
}
