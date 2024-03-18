use crate::models::{points_model::Points, user_bottles_model::UserBottles, user_model::User};
use sqlx::PgPool;
use uuid::Uuid;

pub async fn get_users(pool: &PgPool) -> Result<Vec<User>, sqlx::Error> {
    // Example query to retrieve all users from the database
    let users = sqlx::query_as::<_, User>("SELECT * FROM auth.users")
        .fetch_all(pool)
        .await?;
    Ok(users)
}

pub async fn get_user(pool: &PgPool, id: Uuid) -> Result<User, sqlx::Error> {
    // Example query to retrieve a user from the database
    let user = sqlx::query_as::<_, User>("SELECT * FROM auth.users WHERE id = $1")
        .bind(id)
        .fetch_one(pool)
        .await?;
    Ok(user)
}

pub async fn get_user_points(pool: &PgPool, id: Uuid) -> Result<Points, sqlx::Error> {
    // Example query to retrieve a user's points from the database
    let points = sqlx::query_as::<_, Points>("SELECT * FROM points WHERE user_id = $1")
        .bind(id)
        .fetch_one(pool)
        .await?;
    Ok(points)
}

pub async fn get_user_bottles(pool: &PgPool, id: Uuid) -> Result<Vec<UserBottles>, sqlx::Error> {
    // Example query to retrieve a user's bottles from the database
    let user_bottles = sqlx::query_as::<_, UserBottles>(
        "
SELECT
  user_id,
  m.name AS manufacturer_name,
  bt2.drink_name,
  SUM(bt.quantity) AS total_bottles_transacted
FROM
  bottle_transactions bt
  JOIN bottle_types bt2 ON bt.bottle_type = bt2.id
  JOIN manufacturers m ON bt2.manufacturer_id = m.id
WHERE
  bt.user_id = 'a08d24a9-e979-48d5-b464-4fe3305d95e0'
GROUP BY
  user_id,
  m.name,
  bt2.drink_name;
    ",
    )
    .bind(id)
    .fetch_all(pool)
    .await?;
    Ok(user_bottles)
}
