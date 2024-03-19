use clap::Parser;
use std::sync::Arc;

mod config;
use config::AppConfig;
mod controllers;
mod db;
mod models;
mod repositories;
mod routes;
mod server;
mod services;

#[tokio::main]
async fn main() {
    // Initialize logger
    tracing_subscriber::fmt::init();

    // Initialize environment
    let app_config = get_app_config();

    // Establish database connection
    let pool = db::connect().await;

    // Start the server
    server::serve(app_config, pool).await.unwrap();
}

// Separating this so we can reuse it in tests
pub fn get_app_config() -> Arc<AppConfig> {
    dotenv::dotenv().ok();
    Arc::new(AppConfig::parse())
}
