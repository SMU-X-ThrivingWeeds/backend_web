use crate::models::points_model::Points;
use sqlx::{
    types::chrono::{DateTime, Utc},
    FromRow, PgPool, Row,
};
use uuid::Uuid;

impl<'r> FromRow<'r, sqlx::postgres::PgRow> for Points {
    fn from_row(row: &'r sqlx::postgres::PgRow) -> Result<Self, sqlx::Error> {
        let id: i64 = row.try_get("id")?;
        let user_id: Uuid = row.try_get("user_id")?;
        let points: i64 = row.try_get("points")?;
        let created_at: DateTime<Utc> = row.try_get("created_at")?;
        
        Ok(Points {
            id,
            user_id,
            points,
            created_at: created_at.into(),
        })
    }
}

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

