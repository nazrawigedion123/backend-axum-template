use crate::initiator::Handlers;
use crate::internal::handler::middleware::{append_trace_header_middleware, dummy_auth_middleware};
use crate::internal::handler::user_handler::UserHandler;
use crate::internal::routes::{HttpMethod, Route};
use axum::{
    handler::Handler,
    middleware::from_fn,
    routing::{get_service, post_service},
};

pub fn register_user_routes(handlers: &Handlers) -> Vec<Route> {
    let user_state = handlers.user_handler.clone();

    vec![
        Route {
            path: "/api/v1/users",
            method: HttpMethod::POST,
            method_router: post_service(UserHandler::create_user.with_state(user_state.clone()))
                .route_layer(from_fn(append_trace_header_middleware)),
        },
        Route {
            path: "/api/v1/users/{id}",
            method: HttpMethod::GET,
            method_router: get_service(UserHandler::get_user.with_state(user_state))
                .route_layer(from_fn(dummy_auth_middleware))
                .route_layer(from_fn(append_trace_header_middleware)),
        },
    ]
}
