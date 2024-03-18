use serde::Serialize;
use sqlx::{FromRow, Row};
use uuid::Uuid;

#[derive(Debug, Serialize)]
pub struct UserTransactions {
    pub user_id: Uuid,
    pub transaction_group: i64,
    pub bottle_quantity: i64,
    pub drink_name: String,
    pub manufacturer_name: String,
}

impl<'r> FromRow<'r, sqlx::postgres::PgRow> for UserTransactions {
    fn from_row(row: &'r sqlx::postgres::PgRow) -> Result<Self, sqlx::Error> {
        // Implement the conversion logic here
        // For example:
        let user_id: Uuid = row.try_get("user_id")?;
        let transaction_group: i64 = row.try_get("transaction_group_id")?;
        let bottle_quantity: i64 = row.try_get("bottle_quantity")?;
        let drink_name: String = row.try_get("drink_name")?;
        let manufacturer_name: String = row.try_get("manufacturer_name")?;
        let user_transactions = UserTransactions {
            user_id,
            transaction_group,
            bottle_quantity,
            drink_name,
            manufacturer_name,
        };
        Ok(user_transactions)
    }
}
