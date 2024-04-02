use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Row};

#[derive(Debug, Serialize)]
pub struct RVM {
    pub id: i64,
    pub location: String,
    pub longitude: String,
    pub latitude: String,
    pub check_full: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewRVM {
    pub location: String,
    pub longitude: String,
    pub latitude: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateRVM {
    pub status: bool,
}

impl<'r> FromRow<'r, sqlx::postgres::PgRow> for RVM {
    fn from_row(row: &'r sqlx::postgres::PgRow) -> Result<Self, sqlx::Error> {
        // Implement the conversion logic here
        // For example:
        let id: i64 = row.try_get("id")?;
        let location: String = row.try_get("location")?;
        let longitude: String = row.try_get("longitude")?;
        let latitude: String = row.try_get("latitude")?;
        let check_full: bool = row.try_get("full_check")?;
        let created_at: DateTime<Utc> = row.try_get("created_at")?;
        let user = RVM {
            id,
            location,
            longitude,
            latitude,
            check_full,
            created_at,
        };
        Ok(user)
    }
}
