use crate::{
    models::{
        points_model::Points,
        transaction_model::{Transactions, UserTransactions},
        user_bottles_model::UserBottles,
        user_model::User,
    },
    repositories::user_repository,
};
use sqlx::PgPool;
use uuid::Uuid;

pub async fn fetch_users(pool: &PgPool) -> Result<Vec<User>, sqlx::Error> {
    user_repository::get_users(pool).await
}

pub async fn fetch_user(pool: &PgPool, id: Uuid) -> Result<User, sqlx::Error> {
    user_repository::get_user(pool, id).await
}

pub async fn fetch_user_points(pool: &PgPool, id: Uuid) -> Result<Points, sqlx::Error> {
    user_repository::get_user_points(pool, id).await
}

pub async fn fetch_user_bottles(pool: &PgPool, id: Uuid) -> Result<Vec<UserBottles>, sqlx::Error> {
    user_repository::get_user_bottles(pool, id).await
}

pub async fn fetch_user_transactions(
    pool: &PgPool,
    id: Uuid,
) -> Result<Vec<UserTransactions>, sqlx::Error> {
    user_repository::get_user_transactions(pool, id).await
}

pub async fn fetch_latest_user_transaction(
    pool: &PgPool,
    id: Uuid,
) -> Result<UserTransactions, sqlx::Error> {
    user_repository::get_latest_user_transaction(pool, id).await
}

pub async fn create_user_transaction(
    pool: &PgPool,
    user_id: Uuid,
    bottle_type: i64,
    quantity: i64,
    transaction_group_id: i64,
    points: i64,
) -> Result<Transactions, sqlx::Error> {
    user_repository::create_user_transaction(
        pool,
        user_id,
        bottle_type,
        quantity,
        transaction_group_id,
        points,
    )
    .await
}

pub async fn update_user_points(
    pool: &PgPool,
    user_id: Uuid,
    points: i64,
) -> Result<Points, sqlx::Error> {
    user_repository::update_user_points(pool, user_id, points).await
}
