use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Row};
use uuid::Uuid;

#[derive(Debug, Serialize)]
pub struct RVM {
    pub id: i64,
    pub location: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewRVM {
    pub location: String,
}

impl<'r> FromRow<'r, sqlx::postgres::PgRow> for RVM {
    fn from_row(row: &'r sqlx::postgres::PgRow) -> Result<Self, sqlx::Error> {
        // Implement the conversion logic here
        // For example:
        let id: i64 = row.try_get("id")?;
        let location: String = row.try_get("location")?;
        let created_at: DateTime<Utc> = row.try_get("created_at")?;
        let user = RVM {
            id,
            location,
            created_at,
        };
        Ok(user)
    }
}
