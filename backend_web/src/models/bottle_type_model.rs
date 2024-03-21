use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Row};

#[derive(Debug, Serialize)]
pub struct BottleType {
    pub manufacturer_id: i64,
    pub drink_name: String,
    pub points: i64,
    pub created_at: DateTime<Utc>,
    pub barcode: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewBottleType {
    pub drink_name: String,
    pub barcode: String,
}

impl<'r> FromRow<'r, sqlx::postgres::PgRow> for BottleType {
    fn from_row(row: &'r sqlx::postgres::PgRow) -> Result<Self, sqlx::Error> {
        // Implement the conversion logic here
        // For example:
        let manufacturer_id: i64 = row.try_get("manufacturer_id")?;
        let drink_name: String = row.try_get("drink_name")?;
        let points: i64 = row.try_get("points")?;
        let created_at: DateTime<Utc> = row.try_get("created_at")?;
        let barcode: String = row.try_get("barcode")?;
        let bottle_type = BottleType {
            manufacturer_id,
            drink_name,
            points,
            created_at,
            barcode,
        };
        Ok(bottle_type)
    }
}
