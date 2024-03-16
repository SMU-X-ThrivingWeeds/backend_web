use dotenv::dotenv;
use sqlx::{postgres::PgPoolOptions, PgPool};
use std::env;

pub async fn connect() -> PgPool {
    // Load environment variables from .env file
    dotenv().ok();

    // Read the database URL from the environment
    let database_url = env::var("SUPABASE_URL").expect("SUPABASE_URL must be set");

    // Create a connection pool
    let pool = PgPoolOptions::new()
        .max_connections(5) // Adjust the maximum number of connections as needed
        .connect(&database_url)
        .await
        .expect("can't connect to database");

    return pool;
}
