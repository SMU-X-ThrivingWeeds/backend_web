use crate::models::points_model::Points;
use sqlx::PgPool;

pub async fn get_all_points(pool: &PgPool) -> Result<Vec<Points>, sqlx::Error> {
    // Example query to retrieve all users from the database
    let points = sqlx::query_as::<_, Points>("SELECT * FROM points ORDER BY points DESC LIMIT 50")
        .fetch_all(pool)
        .await?;
    Ok(points)
}
