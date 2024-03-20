use crate::models::manufacturer_model::Manufacturers;
use crate::repositories::manufacturer_repository;
use sqlx::{Error, PgPool};

pub async fn fetch_all_manufacturers(pool: &PgPool) -> Result<Vec<Manufacturers>, Error> {
    manufacturer_repository::get_all_manufacturers(pool).await
}


pub async fn create_manufacturer_if_not_exists(
    pool: &PgPool,
    name: &str,
) -> Result<Manufacturers, sqlx::Error> {
    match manufacturer_repository::get_manufacturer_by_name(pool, name).await {
        Ok(Some(manufacturer)) => {
            // A matching manufacturer was found
            Ok(manufacturer)
        }
        Ok(None) => {
            // No matching manufacturer was found
            manufacturer_repository::create_manufacturer(pool, name).await
        }
        Err(err) => {
            // Error occurred during database operation
            Err(err)
        }
    }
}
