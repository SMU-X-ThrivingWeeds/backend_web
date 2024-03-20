use crate::models::bottle_transaction_model::BottleTransactions;
use crate::repositories::bottle_transaction_repository;
use sqlx::{Error, PgPool};

pub async fn fetch_all_transactions(pool: &PgPool) -> Result<Vec<BottleTransactions>, Error> {
    bottle_transaction_repository::get_all_transactions(pool).await
}

pub async fn get_drink_counts(pool: &PgPool) -> Result<Vec<BottleTransactions>, Error> {
    bottle_transaction_repository::get_drink_counts(pool).await
}

