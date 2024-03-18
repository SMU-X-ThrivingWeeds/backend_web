use crate::models::points_model::Points;
use sqlx::PgPool;

pub async fn get_all_points(pool: &PgPool) -> Result<Vec<Points>, sqlx::Error> {
    // Example query to retrieve all users from the database
    let points = sqlx::query_as::<_, Points>("SELECT * FROM points")
        .fetch_all(pool)
        .await?;
    Ok(points)
}

// pub async fn get_all_points(pool: &PgPool) -> Result<Vec<i32>, sqlx::Error> {
//     let points: Vec<i32> = sqlx::query_scalar!("SELECT points FROM points")
//         .fetch_all(pool)
//         .await?;
//     Ok(points)
// }
