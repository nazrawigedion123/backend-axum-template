// src/internal/constant/dto.rs
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::internal::storage::user_storage::UserModel;

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateUserRequest {
    pub username: String,
    pub email: String,
}

#[derive(Debug, Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: T,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct UserResponse {
    pub success: bool,
    pub data: UserModel,
}
