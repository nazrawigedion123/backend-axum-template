// src/initiator.rs
use crate::internal;

use crate::config::AppConfig;
use crate::docs::ApiDoc;
use crate::internal::handler::user_handler::UserHandler;
use crate::internal::module::UserService;
use crate::internal::module::user_service::DefaultUserService;
use crate::internal::storage::user_storage::PostgresUserRepository;
use axum::Router;
use axum_governor::{GovernorConfigBuilder, GovernorLayer, Quota, extractor::PeerIp, nz};
use colored::Colorize;

use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::TcpListener;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use sqlx::postgres::PgPoolOptions;

pub async fn init_db(database_url: &str) -> sqlx::PgPool {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await
        .expect("Failed to connect to Postgres via SQLx");

    // Optional: Automatically run pending migrations on startup
    

    pool
}

/// Layer 1: Storage Layer Container
#[derive(Clone)]
pub struct Storage {
    pub user_repo: Arc<PostgresUserRepository>,
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
        // tracing::info!("Initializing asynchronous connection pool adapters...");
        println!("  {} Connecting to database...", "⏳".yellow());
        println!("  {} Initializing connection pool...", "⏳".yellow());
        println!(
            "  {} Initializing asynchronous connection pool adapters...",
            "⏳".yellow()
        );
      
        // Use .await here to resolve the Future into the actual PgPool
        let pool = init_db(&cfg.database_url).await;
        
        // Replace tracing::info with stylized println
        println!(
            "  {} Database: {}",
            "✓".green().bold(),
            "PostgreSQL connected".green()
        );
        println!("  {} Connection pool established successfully", "✓".green());
        
        // --- Step 1: Initialize Storage Tier ---
        // Now pool is a valid PgPool instance and can be passed to your repository
        let user_repo = Arc::new(PostgresUserRepository::new(pool));

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
        let handlers = Handlers { user_handler };

        Self {
            storage,
            modules,
            handlers,
        }
    }

    pub async fn initiate() -> std::io::Result<()> {
        internal::platform::logger::init_logger();
        println!(
            "{}",
            "════════════════════════════════════════════════════".dimmed()
        );
        println!("{}", "  🔧 I N I T I A L I Z I N G".white().bold());
        println!(
            "{}",
            "════════════════════════════════════════════════════".dimmed()
        );
        println!("");

        // 2. Load system configurations safely via Envy environment layers
        let cfg = AppConfig::load_from_env();
        // 3. Complete structural dependency tree orchestration pipelines (Storage -> Modules -> Handlers)
        let initiator = AppInitiator::init(cfg).await;

        // Cache port configuration for runtime closures
        let bind_port = cfg.server_port;

        // tracing::info!("Starting axum production server on port {}...", bind_port);
        println!(
            "{}",
            "════════════════════════════════════════════════════".dimmed()
        );
        println!("{}", "  🚀 S E R V E R   R U N N I N G".white().bold());
        println!(
            "{}",
            "════════════════════════════════════════════════════".dimmed()
        );

        // set up cors
        let cors = CorsLayer::new()
            .allow_origin(Any) // Allow requests from any domain
            .allow_methods(Any) // Allow any HTTP method (GET, POST, etc.)
            .allow_headers(Any);
        //set up ratelimiter
        let config_rate_limiter = GovernorConfigBuilder::default()
            .with_extractor(PeerIp::default())
            .expect_connect_info()
            .quota_default(Quota::requests_per_second(nz!(50u32)))
            .finish()
            .unwrap();

        let app: Router = internal::routes::configure_routes(&initiator.handlers)
            .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
            .layer(TraceLayer::new_for_http())
            .layer(cors)
            .layer(GovernorLayer::new(config_rate_limiter));
        let addr = SocketAddr::from(([127, 0, 0, 1], bind_port));
        let listener = TcpListener::bind(addr).await?;

        println!("");
        println!(
            "  ✓ Server listening on: {}",
            format!("http://localhost:{}", bind_port)
                .green()
                .underline()
        );
        println!(
            "  ✓ Swagger UI: {}",
            format!("http://localhost:{}/swagger-ui", bind_port)
                .green()
                .underline()
        );
        println!("  ✓ Environment: {}", cfg.env.green());
        println!("  ✓ PID: {}", std::process::id().to_string().cyan());
        println!("");
        println!(
            "{}",
            "════════════════════════════════════════════════════".dimmed()
        );
        println!("{}", "  Press Ctrl+C to stop".dimmed());
        println!(
            "{}",
            "════════════════════════════════════════════════════".dimmed()
        );

        axum::serve(listener, app.into_make_service_with_connect_info::<SocketAddr>()).await
    }
}
