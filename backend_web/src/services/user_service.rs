use crate::{models::user_model::User, repositories::user_repository};
use sqlx::PgPool;

pub async fn fetch_users(pool: &PgPool) -> Result<Vec<User>, sqlx::Error> {
    user_repository::get_users(pool).await
}
