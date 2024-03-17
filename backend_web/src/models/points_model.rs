use sqlx::
    types::chrono::{DateTime, Utc};

use uuid::Uuid;

#[derive(Debug)]
pub struct Points {
    pub id: i64,
    pub user_id: Uuid,
    pub points: i64,
    pub created_at: DateTime<Utc>,
}
