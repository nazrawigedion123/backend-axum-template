use crate::initiator::Handlers;
use crate::internal::handler::middleware::{append_trace_header_middleware, dummy_auth_middleware};
use crate::internal::handler::user_handler::UserHandler;
use axum::{
    Router, middleware,
    routing::{get, post},
};
use std::sync::Arc;

/// Central configuration controller entry point invoked by src/main.rs
pub fn configure_routes(handlers: &Handlers) -> Router {
    let user_handler = Arc::new(UserHandler::new(handlers.user_handler.clone()));

    Router::new()
        .route(
            "/api/v1/users",
            post(UserHandler::create_user)
                .route_layer(middleware::from_fn(append_trace_header_middleware)),
        )
        .route(
            "/api/v1/users/{id}",
            get(UserHandler::get_user)
                .route_layer(middleware::from_fn(dummy_auth_middleware))
                .route_layer(middleware::from_fn(append_trace_header_middleware)),
        )
        .with_state(user_handler)
}
