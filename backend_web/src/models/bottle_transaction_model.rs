use sqlx::types::BigDecimal;
use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx::{FromRow, Row};
use uuid::Uuid;
use bigdecimal::ToPrimitive;

#[derive(Debug, Serialize)]
pub struct BottleTransactions {
    pub id: i64,
    pub user_id: Uuid,
    pub bottle_type: i64,
    pub quantity: i64,
    pub created_at: DateTime<Utc>,
    pub transaction_group_id: i64,
    pub points: i64
}

#[derive(Debug, Serialize)]
pub struct BottleCount{
    pub bottle_type: i64,
    pub quantity: i64
}

impl<'r> FromRow<'r, sqlx::postgres::PgRow> for BottleTransactions {
    fn from_row(row: &'r sqlx::postgres::PgRow) -> Result<Self, sqlx::Error> {
        let id: i64 = row.try_get("id")?;
        let user_id: Uuid = row.try_get("user_id")?;
        let bottle_type: i64 = row.try_get("bottle_type")?;
        let quantity: i64 = row.try_get("quantity")?;
        let created_at: DateTime<Utc> = row.try_get("created_at")?;
        let transaction_group_id: i64 = row.try_get("transaction_group_id")?;
        let points: i64 = row.try_get("points")?;

        Ok(BottleTransactions {
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


impl<'r> FromRow<'r, sqlx::postgres::PgRow> for BottleCount {
    fn from_row(row: &'r sqlx::postgres::PgRow) -> Result<Self, sqlx::Error> {
        let bottle_type: i64 = row.try_get("bottle_type")?;
        let quantity: BigDecimal = row.try_get("count")?;
        let qty = match quantity.to_i64() {
            Some(x) => x,
            None => 0,
        };

        Ok(BottleCount {
            bottle_type,
            quantity:qty,
        })
    }
}