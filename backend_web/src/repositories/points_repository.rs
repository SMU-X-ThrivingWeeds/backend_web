use crate::models::points_model::Points; // Assuming your model module is named points_models
use sqlx::{
    types::chrono::{DateTime, Utc},
    FromRow, PgPool, Row,
};
use uuid::Uuid;

impl<'r> FromRow<'r, sqlx::postgres::PgRow> for Points {
    fn from_row(row: &'r sqlx::postgres::PgRow) -> Result<Self, sqlx::Error> {
        let id: i8 = row.try_get("id")?; // Assuming id is of type integer
        let user_id: Uuid = row.try_get("user_id")?;
        let points: i8 = row.try_get("points")?; // Assuming points is of type smallint or integer
        let created_at: DateTime<Utc> = row.try_get("created_at")?;
        
        Ok(Points {
            id,
            user_id,
            points,
            created_at: created_at.into(), // Ensure created_at is compatible with your Points model
        })
    }
}

pub async fn get_all_points(pool: &PgPool) -> Result<Vec<i32>, sqlx::Error> {
    let points: Vec<i32> = sqlx::query_scalar!("SELECT points FROM points")
        .fetch_all(pool)
        .await?;
    Ok(points)
}
