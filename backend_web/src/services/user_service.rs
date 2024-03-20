use crate::{
    models::{
        points_model::Points, transaction_model::UserTransactions, user_bottles_model::UserBottles,
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
