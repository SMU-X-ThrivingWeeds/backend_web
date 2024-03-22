use crate::models::bottle_transaction_model::{BottleCount, BottleTransactions};
use sqlx::PgPool;


pub async fn get_all_transactions(pool: &PgPool) -> Result<Vec<BottleTransactions>, sqlx::Error> {
    let transactions =
        sqlx::query_as::<_, BottleTransactions>("SELECT * FROM bottle_transactions")
            .fetch_all(pool)
            .await?;
    Ok(transactions)
}

pub async fn get_drink_counts(pool: &PgPool) -> Result<Vec<BottleCount>, sqlx::Error> {
    let counts = sqlx::query_as::<_, BottleCount>(
        "SELECT bottle_type, SUM(quantity) as count FROM bottle_transactions GROUP BY bottle_type",
    )
    .fetch_all(pool)
    .await?;
    Ok(counts)
}
