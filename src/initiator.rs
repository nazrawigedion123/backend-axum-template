// src/initiator.rs
use crate::internal;

use crate::config::AppConfig;
use crate::internal::handler::user_handler::UserHandler;
use crate::internal::module::UserService;
use crate::internal::module::user_service::DefaultUserService;
use crate::internal::storage::user_storage::DieselUserRepository;
use axum::Router;
use diesel_async::AsyncPgConnection;
use diesel_async::pooled_connection::bb8::Pool;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::TcpListener;
use tower_http::trace::TraceLayer;

/// Layer 1: Storage Layer Container
#[derive(Clone)]
pub struct Storage {
    pub user_repo: Arc<DieselUserRepository>,
}

/// Layer 2: Business Domain Modules Layer Container
#[derive(Clone)]
pub struct Modules {
    pub user_service: Arc<dyn UserService>,
}

/// Layer 3: HTTP Handler Controllers Layer Container
#[derive(Clone)]
pub struct Handlers {
    pub user_handler: Arc<UserHandler>,
}

/// Composition Root Orchestrating all layered inversions of control dependencies
#[derive(Clone)]
pub struct AppInitiator {
    pub storage: Storage,
    pub modules: Modules,
    pub handlers: Handlers,
}

impl AppInitiator {
    pub async fn init(cfg: &AppConfig) -> Self {
        tracing::info!("Initializing asynchronous connection pool adapters...");

        let manager = diesel_async::pooled_connection::AsyncDieselConnectionManager::<
            AsyncPgConnection,
        >::new(&cfg.database_url);

        let pool = Pool::builder()
            .max_size(10)
            .build(manager)
            .await
            .unwrap_or_else(|err| {
                panic!("CRITICAL INFRASTRUCTURE FAILURE: Failed to create database connection pool: {err}");
            });

        tracing::info!("Database connection pool established successfully");

        // --- Step 1: Initialize Storage Tier ---
        let user_repo = Arc::new(DieselUserRepository::new(pool));
        let storage = Storage {
            user_repo: user_repo.clone(),
        };

        // --- Step 2: Initialize Modules Tier (Injecting Storage) ---
        let user_service = Arc::new(DefaultUserService::new(user_repo));
        let modules = Modules {
            user_service: user_service.clone(),
        };

        // --- Step 3: Initialize Handlers Tier (Injecting Modules) ---
        let user_handler = Arc::new(UserHandler::new(user_service));
        let handlers = Handlers {
            user_handler,
        };

        Self {
            storage,
            modules,
            handlers,
        }
    }

    pub async fn initiate() -> std::io::Result<()> {
        internal::platform::logger::init_logger();

        // 2. Load system configurations safely via Envy environment layers
        let cfg = AppConfig::load_from_env();
        // 3. Complete structural dependency tree orchestration pipelines (Storage -> Modules -> Handlers)
        let initiator = AppInitiator::init(cfg).await;

        // Cache port configuration for runtime closures
        let bind_port = cfg.server_port;

        tracing::info!("Starting axum production server on port {}...", bind_port);

        let app: Router = internal::routes::configure_routes(&initiator.handlers)
            .layer(TraceLayer::new_for_http());
        let addr = SocketAddr::from(([127, 0, 0, 1], bind_port));
        let listener = TcpListener::bind(addr).await?;

        axum::serve(listener, app).await
    }
}
