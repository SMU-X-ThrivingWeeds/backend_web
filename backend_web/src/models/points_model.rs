use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx::{FromRow, Row};
use uuid::Uuid;

#[derive(Debug, Serialize)]
pub struct Points {
    pub id: i64,
    pub user_id: Uuid,
    pub points: i64,
    pub created_at: DateTime<Utc>,
}

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
