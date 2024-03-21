use crate::models::{
    points_model::Points,
    transaction_model::{Transactions, UserTransactions},
    user_bottles_model::UserBottles,
    user_model::User,
};
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
  bt.user_id = $1
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

pub async fn get_user_transactions(
    pool: &PgPool,
    id: Uuid,
) -> Result<Vec<UserTransactions>, sqlx::Error> {
    // Example query to retrieve a user's transactions from the database
    let user_transactions = sqlx::query_as::<_, UserTransactions>(
        "
SELECT
  bt.user_id,
  bt.transaction_group_id,
  bt.quantity AS bottle_quantity,
  bt2.drink_name,
  m.name AS manufacturer_name,
  bt.points,
  bt.created_at
FROM
  bottle_transactions bt
  JOIN bottle_types bt2 ON bt.bottle_type = bt2.id
  JOIN manufacturers m ON bt2.manufacturer_id = m.id
WHERE
  user_id = $1
GROUP BY
  bt.id,
  bt.transaction_group_id,
  bt2.drink_name,
  manufacturer_name
ORDER BY
  bt.transaction_group_id DESC;
",
    )
    .bind(id)
    .fetch_all(pool)
    .await?;
    Ok(user_transactions)
}

pub async fn get_latest_user_transaction(
    pool: &PgPool,
    id: Uuid,
) -> Result<UserTransactions, sqlx::Error> {
    // Example query to retrieve a user's transactions from the database
    let user_transaction = sqlx::query_as::<_, UserTransactions>(
        "
SELECT
  bt.user_id,
  bt.transaction_group_id,
  bt.quantity AS bottle_quantity,
  bt2.drink_name,
  m.name AS manufacturer_name,
  bt.points,
  bt.created_at
FROM
  bottle_transactions bt
  JOIN bottle_types bt2 ON bt.bottle_type = bt2.id
  JOIN manufacturers m ON bt2.manufacturer_id = m.id
WHERE
  user_id = $1
GROUP BY
  bt.id,
  bt.transaction_group_id,
  bt2.drink_name,
  manufacturer_name
ORDER BY
  bt.transaction_group_id DESC
LIMIT 1;
",
    )
    .bind(id)
    .fetch_one(pool)
    .await?;
    Ok(user_transaction)
}

pub async fn create_user_transaction(
    pool: &PgPool,
    user_id: Uuid,
    bottle_type: i64,
    quantity: i64,
    transaction_group_id: i64,
    points: i64,
) -> Result<Transactions, sqlx::Error> {
    // Example query to insert a user's transaction into the database
    let user_transaction = sqlx::query_as::<_, Transactions>(
        "INSERT INTO
          bottle_transactions (
            user_id,
            bottle_type,
            quantity,
            transaction_group_id,
            points
          )
        VALUES
          ( $1, $2, $3, $4, $5 )
        RETURNING *;",
    )
    .bind(user_id)
    .bind(bottle_type)
    .bind(quantity)
    .bind(transaction_group_id)
    .bind(points)
    .fetch_one(pool)
    .await?;
    Ok(user_transaction)
}

pub async fn update_user_points(
    pool: &PgPool,
    user_id: Uuid,
    points: i64,
) -> Result<Points, sqlx::Error> {
    // Example query to update a user's points in the database
    let points =
        sqlx::query_as::<_, Points>("UPDATE points SET points = $1 WHERE user_id = $2 RETURNING *")
            .bind(points)
            .bind(user_id)
            .fetch_one(pool)
            .await?;
    Ok(points)
}
