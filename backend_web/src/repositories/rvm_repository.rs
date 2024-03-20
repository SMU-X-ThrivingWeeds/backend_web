use crate::models::rvm_model::RVM;
use sqlx::PgPool;

pub async fn get_all_rvms(pool: &PgPool) -> Result<Vec<RVM>, sqlx::Error> {
    // Example query to retrieve all rvms from the database
    let rvms = sqlx::query_as::<_, RVM>("SELECT * FROM rvms")
        .fetch_all(pool)
        .await?;
    Ok(rvms)
}

pub async fn get_rvm_by_location(
    pool: &PgPool,
    location: &str,
) -> Result<Option<RVM>, sqlx::Error> {
    // Example query to retrieve a rvm by location from the database
    let rvm = sqlx::query_as::<_, RVM>("SELECT * FROM rvms WHERE location = $1")
        .bind(location)
        .fetch_optional(pool)
        .await?;
    Ok(rvm)
}

pub async fn add_rvm(pool: &PgPool, location: &str) -> Result<RVM, sqlx::Error> {
    // Example query to add a new rvm to the database
    let rvm = sqlx::query_as::<_, RVM>("INSERT INTO rvms (location) VALUES ($1) RETURNING *")
        .bind(location)
        .fetch_one(pool)
        .await?;
    Ok(rvm)
}
