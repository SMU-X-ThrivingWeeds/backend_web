use crate::{
    models::bottle_type_model::{BottleType, CheckBottles},
    repositories::bottle_type_repository,
};
use sqlx::PgPool;

pub async fn add_bottle_type_if_not_exists(
    pool: &PgPool,
    manufacturer_id: i64,
    drink_name: &str,
    barcode: &str,
) -> Result<BottleType, sqlx::Error> {
    match bottle_type_repository::get_bottle_type_by_barcode(pool, barcode).await {
        Ok(Some(bottle_type)) => {
            // A matching bottle type was found
            Ok(bottle_type)
        }
        Ok(None) => {
            // No matching bottle type was found
            bottle_type_repository::add_bottle_type(pool, manufacturer_id, drink_name, barcode)
                .await
        }
        Err(err) => {
            // Error occurred during database operation
            Err(err)
        }
    }
}

pub async fn check_bottles(
    pool: &PgPool,
    barcode_arr: Vec<String>,
) -> Result<CheckBottles, sqlx::Error> {
    let existing_bottles = bottle_type_repository::check_bottles(pool, &barcode_arr).await?;
    let bottles_not_exists: Vec<String> = barcode_arr
        .iter()
        .filter(|barcode| {
            !existing_bottles
                .iter()
                .any(|bottle| bottle.barcode == **barcode)
        })
        .map(|barcode| barcode.to_string())
        .collect();
    Ok(CheckBottles {
        barcode_exists: existing_bottles,
        barcode_not_exists: bottles_not_exists,
    })
}
