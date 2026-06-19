use crate::initiator::Handlers;
use crate::internal::handler::middleware::{append_trace_header_middleware, dummy_auth_middleware};
use crate::internal::handler::user_handler::UserHandler;
use axum::{
    Router, middleware,
    routing::{get, post},
};

pub fn configure_routes(handlers: &Handlers) -> Router {
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
        .with_state(handlers.user_handler.clone())
}
