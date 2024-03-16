use crate::models::user_model::User;
use sqlx::{
    types::chrono::{DateTime, Utc},
    FromRow, PgPool, Row,
};
use uuid::Uuid;

impl<'r> FromRow<'r, sqlx::postgres::PgRow> for User {
    fn from_row(row: &'r sqlx::postgres::PgRow) -> Result<Self, sqlx::Error> {
        // Implement the conversion logic here
        // For example:
        let id: Uuid = row.try_get("id")?;
        let email: String = row.try_get("email")?;
        let created_at: DateTime<Utc> = row.try_get("created_at")?;
        let user = User {
            id,
            email,
            created_at: created_at.format("%Y-%m-%d %H:%M:%S").to_string(), // Format the date to a string
        };
        Ok(user)
    }
}

pub async fn get_users(pool: &PgPool) -> Result<Vec<User>, sqlx::Error> {
    // Example query to retrieve all users from the database
    let users = sqlx::query_as::<_, User>("SELECT * FROM auth.users")
        .fetch_all(pool)
        .await?;
    Ok(users)
}
