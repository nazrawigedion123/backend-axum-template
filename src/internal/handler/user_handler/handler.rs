// src/internal/handler/user_handler.rs
use crate::internal::constant::dto::{ApiResponse, CreateUserRequest};
use crate::internal::module::UserService;
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use std::sync::Arc;
use uuid::Uuid;

/// Concrete implementation wrapper holding references to our business modules
pub struct UserHandler {
    user_service: Arc<dyn UserService>,
}

impl UserHandler {
    pub fn new(user_service: Arc<dyn UserService>) -> Self {
        Self { user_service }
    }

    /// POST /api/v1/users
    pub async fn create_user(
        State(handler): State<Arc<Self>>,
        Json(payload): Json<CreateUserRequest>,
    ) -> Result<
        (
            StatusCode,
            Json<ApiResponse<crate::internal::storage::user_storage::UserModel>>,
        ),
        crate::internal::constant::errors::AppError,
    > {
        // Logging context automatically captures the trace IDs inherited from middleware
        tracing::info!(username = %payload.username, "HTTP Request received: Create User");

        let user = handler
            .user_service
            .register_new_user(payload.username.clone(), payload.email.clone())
            .await?;

        Ok((
            StatusCode::CREATED,
            Json(ApiResponse {
                success: true,
                data: user,
            }),
        ))
    }

    /// GET /api/v1/users/{id}
    pub async fn get_user(
        Path(raw_id): Path<String>,
        State(handler): State<Arc<Self>>,
    ) -> Result<
        Json<ApiResponse<crate::internal::storage::user_storage::UserModel>>,
        crate::internal::constant::errors::AppError,
    > {
        let target_uuid = Uuid::parse_str(&raw_id).map_err(|_| {
            crate::internal::constant::errors::AppError::ValidationError(
                "Invalid UUID format".to_string(),
            )
        })?;

        tracing::info!(%target_uuid, "HTTP Request received: Get User by ID");

        let user = handler.user_service.get_user_by_id(target_uuid).await?;

        Ok(Json(ApiResponse {
            success: true,
            data: user,
        }))
    }
}
