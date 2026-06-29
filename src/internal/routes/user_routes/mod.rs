use crate::initiator::Handlers;
use crate::internal::handler::middleware::{append_trace_header_middleware, dummy_auth_middleware};
use crate::internal::handler::{UserHandlerTrait, create_user_route, get_user_route,get_user_by_username_route};
use crate::internal::routes::{HttpMethod, Route};
use axum::{
    handler::Handler,
    middleware::from_fn,
    routing::{get_service, post_service},
};
use std::sync::Arc;

pub fn register_user_routes(handlers: &Handlers) -> Vec<Route> {
    let user_state: Arc<dyn UserHandlerTrait> = handlers.user_handler.clone();

    vec![
        Route {
            path: "/api/v1/users",
            method: HttpMethod::POST,
            // FIXED: Swapped post_service for post
            method_router: post_service(create_user_route.with_state(user_state.clone()))
                .route_layer(from_fn(append_trace_header_middleware)),
        },
        Route {
            // FIXED: Changed "{id}" to ":id" to match Axum's routing engine specification
            path: "/api/v1/users/{id}",
            method: HttpMethod::GET,
            // FIXED: Swapped get_service for get
            method_router: get_service(get_user_route.with_state(user_state.clone()))
                // .route_layer(from_fn(dummy_auth_middleware)),
                .route_layer(from_fn(append_trace_header_middleware)),
        },
        Route {
            // FIXED: Changed "{id}" to ":id" to match Axum's routing engine specification
            path: "/api/v1/users/get-by-username/{username}",
            method: HttpMethod::GET,
            // FIXED: Swapped get_service for get
            method_router: get_service(get_user_by_username_route.with_state(user_state))
                // .route_layer(from_fn(dummy_auth_middleware)),
                .route_layer(from_fn(append_trace_header_middleware)),
        },
    ]
}
