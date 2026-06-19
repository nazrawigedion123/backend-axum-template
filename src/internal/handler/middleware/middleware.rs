use axum::{extract::Request, http::StatusCode, middleware::Next, response::Response};
/// Example Middleware 1: Appends server header to trace requests
pub async fn append_trace_header_middleware(
    req: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    tracing::debug!("Route Middleware 1: Processing tracing headers intercept");
    Ok(next.run(req).await)
}

/// Example Middleware 2: Basic security validation gate simulation
pub async fn dummy_auth_middleware(req: Request, next: Next) -> Result<Response, StatusCode> {
    tracing::debug!("Route Middleware 2: Validating client security permissions");

    if req.headers().contains_key("x-api-token") {
        Ok(next.run(req).await)
    } else {
        Err(StatusCode::UNAUTHORIZED)
    }
}
