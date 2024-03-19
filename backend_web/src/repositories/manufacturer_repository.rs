use crate::models::manufacturer_model::Manufacturers;
use sqlx::PgPool;

pub async fn get_all_manufacturers(pool: &PgPool) -> Result<Vec<Manufacturers>, sqlx::Error> {
    let manufacturers = sqlx::query_as::<_, Manufacturers>("SELECT * FROM manufacturers")
        .fetch_all(pool)
        .await?;
    Ok(manufacturers)
}