use crate::models::rvm_model::RVM;
use crate::repositories::rvm_repository;
use sqlx::{Error, PgPool};

pub async fn fetch_all_rvms(pool: &PgPool) -> Result<Vec<RVM>, sqlx::Error> {
    rvm_repository::get_all_rvms(pool).await
}

pub async fn create_rvm_if_not_exists(pool: &PgPool, location: &str) -> Result<RVM, sqlx::Error> {
    match rvm_repository::get_rvm_by_location(pool, location).await {
        Ok(Some(manufacturer)) => {
            // A matching rvm was found
            Ok(manufacturer)
        }
        Ok(None) => {
            // No matching rvm was found
            rvm_repository::add_rvm(pool, location).await
        }
        Err(err) => {
            // Error occurred during database operation
            Err(err)
        }
    }
}
