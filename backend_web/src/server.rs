use std::sync::Arc;

use anyhow::Context;
use axum::routing::get;
use axum::Router;
use sqlx::PgPool;
use tokio::net::TcpListener;
use tower::ServiceBuilder;
use tower_http::cors::{self, CorsLayer};
use tracing::log::info;

use crate::config::AppConfig;
use crate::routes::bottle_routes;
use crate::routes::manufacturer_routes;
use crate::routes::bottle_transaction_routes;
use crate::routes::points_routes;
use crate::routes::rvm_routes;
use crate::routes::user_routes;

#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
}

/// Server entry point where we register the services and start the server
pub async fn serve(config: Arc<AppConfig>, pool: PgPool) -> anyhow::Result<()> {
    let app = Router::new()
        .route("/", get(|| async { "Hello, world!" }))
        .nest("/", user_routes::user_routes())
        .nest("/", points_routes::points_routes())
        .nest("/", bottle_routes::bottle_routes())
        .nest("/", manufacturer_routes::manufacturer_routes())
        .nest("/", bottle_transaction_routes::bottle_transaction_routes())
        .nest("/", rvm_routes::rvm_routes())
        .with_state(AppState { pool })
        .layer(
            // Use ServiceBuilder to apply multiple middleware
            // This will ensure that the middleware is applied in the order from top to bottom
            // Read https://docs.rs/axum/latest/axum/middleware/index.html#ordering for more info
            ServiceBuilder::new().layer(
                CorsLayer::new()
                    // .allow_credentials(true)
                    // .allow_methods([
                    //     Method::GET,
                    //     Method::POST,
                    //     Method::OPTIONS,
                    //     Method::DELETE,
                    //     Method::PUT,
                    // ])
                    // .allow_headers([AUTHORIZATION, ACCEPT, COOKIE, CONTENT_TYPE]),
                    .allow_origin(cors::Any), // In a real application, you should validate the `Origin` header.
            ),
        );

    info!("Starting server at {}", config.server_address);
    let listener = TcpListener::bind(&config.server_address)
        .await
        .context("Failed to bind to server address")?;
    axum::serve(listener, app.into_make_service())
        .await
        .context("Error starting server")
}
