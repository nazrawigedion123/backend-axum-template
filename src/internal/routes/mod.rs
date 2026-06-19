pub mod user_routes;
use colored::*;

use crate::initiator::Handlers;
use axum::{Router, routing::method_routing::MethodRouter};

pub struct Route {
    pub path: &'static str,
    pub method: HttpMethod,
    pub method_router: MethodRouter,
}

pub enum HttpMethod {
    GET,
    POST,
    DELETE,
    PUT,
    PATCH,
    HEAD,
    OPTIONS,
    ANY,
}
impl HttpMethod {
    pub fn as_str(&self) -> &'static str {
        match self {
            HttpMethod::GET => "GET",
            HttpMethod::POST => "POST",
            HttpMethod::PUT => "PUT",
            HttpMethod::DELETE => "DELETE",
            HttpMethod::PATCH => "PATCH",
            HttpMethod::HEAD => "HEAD",
            HttpMethod::OPTIONS => "OPTIONS",
            HttpMethod::ANY => "ANY",
        }
    }
}

pub fn configure_routes(handlers: &Handlers) -> Router {
    register_routes(user_routes::routes::register_user_routes(handlers))
}

// pub fn register_routes(routes: Vec<Route>) -> Router {
//     routes.into_iter().fold(Router::new(),
//      |router, route| {
//         router.route(route.path, route.method_router)
//     })
// }

pub fn register_routes(routes: Vec<Route>) -> Router {

     println!("{}", "════════════════════════════════════════════════════".dimmed());
    println!("{}", "  📡 R O U T E S".white().bold());
    println!("{}", "════════════════════════════════════════════════════".dimmed());
    println!("");
    println!("");
   
    let t=routes.len();
    let r=routes.into_iter().fold(Router::new(), |router, route| {
        println!(
            "✅ Route registered: {} {}",
            route.method.as_str(),
            route.path
        );
        router.route(route.path, route.method_router)
    });
    println!("════════════════════════════════════════════════════");
    println!("  Total routes: {}", t);
    println!("════════════════════════════════════════════════════");
    println!("");
    r

    
}
