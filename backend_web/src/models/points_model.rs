use sqlx::
    types::chrono::{DateTime, Utc};

use uuid::Uuid;

#[derive(Debug)]
pub struct Points {
    pub id: i8,
    pub user_id: Uuid,
    pub points: i8,
    pub created_at: DateTime<Utc>,
}
