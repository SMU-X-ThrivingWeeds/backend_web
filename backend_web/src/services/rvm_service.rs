use crate::models::rvm_model::RVM;
use crate::repositories::rvm_repository;
use sqlx::PgPool;

pub async fn fetch_all_rvms(pool: &PgPool) -> Result<Vec<RVM>, sqlx::Error> {
    rvm_repository::get_all_rvms(pool).await
}

pub async fn create_rvm_if_not_exists(
    pool: &PgPool,
    location: &str,
    latitude: &str,
    longitude: &str,
) -> Result<RVM, sqlx::Error> {
    match rvm_repository::get_rvm_by_location(pool, location).await {
        Ok(Some(rvm)) => {
            // A matching rvm was found
            Ok(rvm)
        }
        Ok(None) => {
            // No matching rvm was found
            rvm_repository::add_rvm(pool, location, longitude, latitude).await
        }
        Err(err) => {
            // Error occurred during database operation
            Err(err)
        }
    }
}

pub async fn get_rvm_by_id(pool: &PgPool, id: i64) -> Result<Option<RVM>, sqlx::Error> {
    rvm_repository::get_rvm_by_id(pool, id).await
}

pub async fn update_rvm_status(pool: &PgPool, id: i64, status: bool) -> Result<RVM, sqlx::Error> {
    // check if rvm exists else return error
    match rvm_repository::get_rvm_by_id(pool, id).await {
        Ok(Some(_rvm)) => {
            // Update the status of the rvm
            rvm_repository::update_rvm_status(pool, id, status).await
        }
        Ok(None) => Err(sqlx::Error::RowNotFound),
        Err(err) => Err(err),
    }
}
