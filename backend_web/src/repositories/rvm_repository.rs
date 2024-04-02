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

pub async fn get_rvm_by_id(pool: &PgPool, id: i64) -> Result<Option<RVM>, sqlx::Error> {
    // Example query to retrieve a rvm by id from the database
    let rvm = sqlx::query_as::<_, RVM>("SELECT * FROM rvms WHERE id = $1")
        .bind(id)
        .fetch_optional(pool)
        .await?;
    Ok(rvm)
}

pub async fn add_rvm(
    pool: &PgPool,
    location: &str,
    longitude: &str,
    latitude: &str,
) -> Result<RVM, sqlx::Error> {
    // Example query to add a new rvm to the database
    let rvm = sqlx::query_as::<_, RVM>(
        "INSERT INTO rvms (location, longitude, latitude) VALUES ($1, $2, $3) RETURNING *",
    )
    .bind(location)
    .bind(longitude)
    .bind(latitude)
    .fetch_one(pool)
    .await?;
    Ok(rvm)
}

pub async fn update_rvm_status(pool: &PgPool, id: i64, status: bool) -> Result<RVM, sqlx::Error> {
    // Example query to update the status of a rvm in the database
    let rvm = sqlx::query_as::<_, RVM>("UPDATE rvms SET full_check = $1 WHERE id = $2 RETURNING *")
        .bind(status)
        .bind(id)
        .fetch_one(pool)
        .await?;
    Ok(rvm)
}
