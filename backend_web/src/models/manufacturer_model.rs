use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx::{FromRow, Row};

#[derive(Debug, Serialize)]
pub struct Manufacturers {
    pub id: i64,
    pub name: String,
    pub created_at: DateTime<Utc>,
}

impl<'r> FromRow<'r, sqlx::postgres::PgRow> for Manufacturers {
    fn from_row(row: &'r sqlx::postgres::PgRow) -> Result<Self, sqlx::Error> {
        let id: i64 = row.try_get("id")?;
        let name: String = row.try_get("name")?;
        let created_at: DateTime<Utc> = row.try_get("created_at")?;

        Ok(Manufacturers {
            id,
            name,
            created_at: created_at.into(),
        })
    }
}
