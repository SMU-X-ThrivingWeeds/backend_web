use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx::{FromRow, Row};
use uuid::Uuid;

#[derive(Debug, Serialize)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub created_at: DateTime<Utc>,
}

impl<'r> FromRow<'r, sqlx::postgres::PgRow> for User {
    fn from_row(row: &'r sqlx::postgres::PgRow) -> Result<Self, sqlx::Error> {
        // Implement the conversion logic here
        // For example:
        let id: Uuid = row.try_get("id")?;
        let email: String = row.try_get("email")?;
        let created_at: DateTime<Utc> = row.try_get("created_at")?;
        let user = User {
            id,
            email,
            created_at,
        };
        Ok(user)
    }
}
