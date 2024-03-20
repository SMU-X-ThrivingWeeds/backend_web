use crate::models::manufacturer_model::Manufacturers;
use sqlx::PgPool;

pub async fn get_all_manufacturers(pool: &PgPool) -> Result<Vec<Manufacturers>, sqlx::Error> {
    let manufacturers = sqlx::query_as::<_, Manufacturers>("SELECT * FROM manufacturers")
        .fetch_all(pool)
        .await?;
    Ok(manufacturers)
}

pub async fn get_manufacturer_by_name(
    pool: &PgPool,
    name: &str,
) -> Result<Option<Manufacturers>, sqlx::Error> {
    let manufacturer =
        sqlx::query_as::<_, Manufacturers>("SELECT * FROM manufacturers WHERE name = $1")
            .bind(name)
            .fetch_optional(pool)
            .await?;
    Ok(manufacturer)
}

pub async fn create_manufacturer(pool: &PgPool, name: &str) -> Result<Manufacturers, sqlx::Error> {
    let manufacturer = sqlx::query_as::<_, Manufacturers>(
        "INSERT INTO manufacturers (name) VALUES ($1) RETURNING *",
    )
    .bind(name)
    .fetch_one(pool)
    .await?;
    Ok(manufacturer)
}


