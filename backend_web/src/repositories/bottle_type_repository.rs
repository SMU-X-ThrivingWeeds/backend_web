use crate::models::bottle_type_model::BottleType;
use sqlx::PgPool;

pub async fn get_bottle_type_by_drink_name(
    pool: &PgPool,
    drink_name: &str,
) -> Result<Option<BottleType>, sqlx::Error> {
    let bottle_type =
        sqlx::query_as::<_, BottleType>("SELECT * FROM bottle_types WHERE drink_name = $1")
            .bind(drink_name)
            .fetch_optional(pool)
            .await?;
    Ok(bottle_type)
}

pub async fn add_bottle_type(
    pool: &PgPool,
    manufacturer_id: i64,
    drink_name: &str,
    points: i64,
) -> Result<BottleType, sqlx::Error> {
    let bottle_type = sqlx::query_as::<_, BottleType>(
        "INSERT INTO bottle_types (manufacturer_id, drink_name, points) VALUES ($1, $2, $3) RETURNING *",
    )
    .bind(manufacturer_id)
    .bind(drink_name)
    .bind(points)
    .fetch_one(pool)
    .await?;
    Ok(bottle_type)
}