use sqlx::{Error, PgPool};
use crate::repositories::points_repository;
use crate::models::points_model::Points;


pub async fn fetch_all_points(pool: &PgPool) -> Result<Vec<Points>, Error> {
    points_repository::get_all_points(pool).await
}
