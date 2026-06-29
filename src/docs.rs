use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        create_user,
        get_user,
        get_user_by_username,
    ),
    components(
        schemas(
            crate::internal::constant::dto::CreateUserRequest,
            crate::internal::constant::dto::UserResponse,
            crate::internal::constant::UserModel,
            crate::internal::constant::errors::ErrorResponse
        )
    ),
    tags(
        (name = "users", description = "User management endpoints")
    )
)]
pub struct ApiDoc;

#[utoipa::path(
    post,
    path = "/api/v1/users",
    request_body = crate::internal::constant::dto::CreateUserRequest,
    responses(
        (status = 201, description = "User created successfully", body = crate::internal::constant::dto::UserResponse),
        (status = 400, description = "Invalid request payload", body = crate::internal::constant::errors::ErrorResponse),
        (status = 500, description = "Internal database error", body = crate::internal::constant::errors::ErrorResponse)
    ),
    tag = "users"
)]
pub async fn create_user() {}

#[utoipa::path(
    get,
    path = "/api/v1/users/{id}",
    params(
        ("id" = uuid::Uuid, Path, description = "User ID")
    ),
    responses(
        (status = 200, description = "User fetched successfully", body = crate::internal::constant::dto::UserResponse),
        (status = 400, description = "Invalid UUID format", body = crate::internal::constant::errors::ErrorResponse),
        (status = 404, description = "User not found", body = crate::internal::constant::errors::ErrorResponse),
        (status = 500, description = "Internal database error", body = crate::internal::constant::errors::ErrorResponse)
    ),
    tag = "users"
)]
pub async fn get_user() {}

#[utoipa::path(
    get,
    path = "/api/v1/users/get-by-username/{username}",
    params(
        ("username" = String, Path, description = "User Name")
    ),
    responses(
        (status = 200, description = "User fetched successfully", body = crate::internal::constant::dto::UserResponse),
        (status = 400, description = "Invalid UUID format", body = crate::internal::constant::errors::ErrorResponse),
        (status = 404, description = "User not found", body = crate::internal::constant::errors::ErrorResponse),
        (status = 500, description = "Internal database error", body = crate::internal::constant::errors::ErrorResponse)
    ),
    tag = "users"
)]
pub async fn get_user_by_username() {}
