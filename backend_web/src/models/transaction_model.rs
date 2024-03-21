use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Row};
use uuid::Uuid;

#[derive(Debug, Serialize)]
pub struct Transactions {
    pub id: i64,
    pub user_id: Uuid,
    pub bottle_type: i64,
    pub quantity: i64,
    pub created_at: DateTime<Utc>,
    pub transaction_group_id: i64,
    pub points: i64,
}

#[derive(Debug, Serialize)]
pub struct UserTransactions {
    pub user_id: Uuid,
    pub transaction_group: i64,
    pub bottle_quantity: i64,
    pub drink_name: String,
    pub manufacturer_name: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewUserTransactions {
    pub bottle_quantity: i64,
    pub drink_name: String,
    pub manufacturer_name: String,
    pub barcode: String,
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
        let created_at: DateTime<Utc> = row.try_get("created_at")?;
        let user_transactions = UserTransactions {
            user_id,
            transaction_group,
            bottle_quantity,
            drink_name,
            manufacturer_name,
            created_at,
        };
        Ok(user_transactions)
    }
}

impl<'r> FromRow<'r, sqlx::postgres::PgRow> for Transactions {
    fn from_row(row: &'r sqlx::postgres::PgRow) -> Result<Self, sqlx::Error> {
        let id: i64 = row.try_get("id")?;
        let user_id: Uuid = row.try_get("user_id")?;
        let bottle_type: i64 = row.try_get("bottle_type")?;
        let quantity: i64 = row.try_get("quantity")?;
        let created_at: DateTime<Utc> = row.try_get("created_at")?;
        let transaction_group_id: i64 = row.try_get("transaction_group_id")?;
        let points: i64 = row.try_get("points")?;

        Ok(Transactions {
            id,
            user_id,
            bottle_type,
            quantity,
            created_at: created_at.into(),
            transaction_group_id,
            points,
        })
    }
}
