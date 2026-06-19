// src/internal/constant/errors.rs
use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;
use thiserror::Error;
use utoipa::ToSchema;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Internal database tracking anomaly encountered")]
    DatabaseError(#[from] diesel::result::Error),

    #[error("Requested resource for entity could not be verified")]
    NotFound,

    #[error("Validation failed: {0}")]
    ValidationError(String),
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ErrorResponse {
    pub success: bool,
    pub error: String,
}

// Implement the Axum response trait so errors turn into clean HTTP status codes later
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let status_code = match &self {
            AppError::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::NotFound => StatusCode::NOT_FOUND,
            AppError::ValidationError(_) => StatusCode::BAD_REQUEST,
        };

        let body = Json(ErrorResponse {
            success: false,
            error: self.to_string(),
        });

        (status_code, body).into_response()
    }
}

impl AppError {
    pub fn status_code(&self) -> StatusCode {
        match self {
            AppError::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::NotFound => StatusCode::NOT_FOUND,
            AppError::ValidationError(_) => StatusCode::BAD_REQUEST,
        }
    }
}
