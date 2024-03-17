// services.rs

use sqlx::{Error, PgPool};
use crate::repositories::points_repository;

pub async fn fetch_all_points(pool: &PgPool) -> Result<Vec<i32>, Error> {
    points_repository::get_all_points(pool).await
}
