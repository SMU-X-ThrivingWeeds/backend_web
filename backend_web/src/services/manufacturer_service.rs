use sqlx::{Error, PgPool};
use crate::repositories::manufacturer_repository;
use crate::models::manufacturer_model::Manufacturers;


pub async fn fetch_all_manufacturers(pool: &PgPool) -> Result<Vec<Manufacturers>, Error> {
    manufacturer_repository::get_all_manufacturers(pool).await
}
