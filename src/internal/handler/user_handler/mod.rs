use crate::internal::constant::dto::{ApiResponse, CreateUserRequest};
use crate::internal::constant::errors::AppError;
use crate::internal::module::UserService;
use crate::internal::constant::UserModel;
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use std::sync::Arc;
use uuid::Uuid;

// --- Axum Invariant Routing Functions (No self context) ---

// FIXED: State parameter must expect Arc<dyn UserHandlerTrait>
pub async fn create_user_route(
    State(handler): State<Arc<dyn UserHandlerTrait>>,
    Json(payload): Json<CreateUserRequest>,
) -> Result<(StatusCode, Json<ApiResponse<UserModel>>), AppError> {
    handler.create_user(payload).await
}

// FIXED: State parameter must expect Arc<dyn UserHandlerTrait>
pub async fn get_user_route(
    Path(raw_id): Path<String>,
    State(handler): State<Arc<dyn UserHandlerTrait>>,
) -> Result<Json<ApiResponse<UserModel>>, AppError> {
    let target_uuid = uuid::Uuid::parse_str(&raw_id)
        .map_err(|_| AppError::ValidationError("Invalid UUID format".to_string()))?;
    handler.get_user(target_uuid).await
}
// --- Trait and Concrete Implementations ---
// 
// 
 pub async fn get_user_by_username_route(
    Path(username): Path<String>,
    State(handler): State<Arc<dyn UserHandlerTrait>>,
) -> Result<Json<ApiResponse<UserModel>>, AppError> {
  
    handler.get_user_by_username(username).await
}

#[async_trait::async_trait]
pub trait UserHandlerTrait: Send + Sync {
    async fn create_user(
        &self,
        payload: CreateUserRequest,
    ) -> Result<(StatusCode, Json<ApiResponse<UserModel>>), AppError>;

    async fn get_user(&self, id: Uuid) -> Result<Json<ApiResponse<UserModel>>, AppError>;
     async fn get_user_by_username(&self, username:String) -> Result<Json<ApiResponse<UserModel>>, AppError> ;
}

pub struct UserHandler {
    user_service: Arc<dyn UserService>,
}

impl UserHandler {
    pub fn new(user_service: Arc<dyn UserService>) -> Self {
        Self { user_service }
    }
}

#[async_trait::async_trait]
impl UserHandlerTrait for UserHandler {
    async fn create_user(
        &self,
        payload: CreateUserRequest,
    ) -> Result<(StatusCode, Json<ApiResponse<UserModel>>), AppError> {
        tracing::info!(username = %payload.username, "HTTP Request received: Create User");

        let user = self
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

    async fn get_user(&self, id: Uuid) -> Result<Json<ApiResponse<UserModel>>, AppError> {
        tracing::info!(%id, "HTTP Request received: Get User by ID");

        let user = self.user_service.get_user_by_id(id).await?;

        Ok(Json(ApiResponse {
            success: true,
            data: user,
        }))
    }
    
    async fn get_user_by_username(&self, username:String) -> Result<Json<ApiResponse<UserModel>>, AppError> {
        tracing::info!(%username, "HTTP Request received: Get User by USER NAME");

        let user = self.user_service.get_user_by_username(username).await?;

        Ok(Json(ApiResponse {
            success: true,
            data: user,
        }))
    }
}
