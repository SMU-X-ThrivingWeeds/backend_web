use bigdecimal::BigDecimal as RustBigDecimal;
use serde::Serialize;
use sqlx::{types::BigDecimal as SqlxBigDecimal, FromRow, Row};
use std::str::FromStr;
use uuid::Uuid;

#[derive(Debug, Serialize)]
pub struct UserBottles {
    pub user_id: Uuid,
    pub manufacturer_name: String,
    pub drink_name: String,
    pub bottle_quantity: RustBigDecimal,
}

impl<'r> FromRow<'r, sqlx::postgres::PgRow> for UserBottles {
    fn from_row(row: &'r sqlx::postgres::PgRow) -> Result<Self, sqlx::Error> {
        // Implement the conversion logic here
        // For example:
        let user_id: Uuid = row.try_get("user_id")?;
        let manufacturer_name: String = row.try_get("manufacturer_name")?;
        let drink_name: String = row.try_get("drink_name")?;
        let sqlx_bottle_quantity: SqlxBigDecimal = row.try_get("total_bottles_transacted")?;
        let bottle_quantity: RustBigDecimal =
            RustBigDecimal::from_str(&sqlx_bottle_quantity.to_string()).unwrap();
        let user_bottles = UserBottles {
            user_id,
            manufacturer_name,
            drink_name,
            bottle_quantity,
        };
        Ok(user_bottles)
    }
}
