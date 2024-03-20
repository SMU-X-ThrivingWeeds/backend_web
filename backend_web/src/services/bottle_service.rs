use crate::{models::bottle_type_model::BottleType, repositories::bottle_type_repository};
use sqlx::PgPool;

pub async fn add_bottle_type_if_not_exists(
    pool: &PgPool,
    manufacturer_id: i64,
    drink_name: &str,
    points: i64,
) -> Result<BottleType, sqlx::Error> {
    match bottle_type_repository::get_bottle_type_by_drink_name(pool, drink_name).await {
        Ok(Some(bottle_type)) => {
            // A matching bottle type was found
            Ok(bottle_type)
        }
        Ok(None) => {
            // No matching bottle type was found
            bottle_type_repository::add_bottle_type(pool, manufacturer_id, drink_name, points).await
        }
        Err(err) => {
            // Error occurred during database operation
            Err(err)
        }
    }
}
