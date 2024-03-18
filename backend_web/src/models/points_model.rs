use chrono::{DateTime, Utc};
use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Serialize)]
pub struct Points {
    pub id: i64,
    pub user_id: Uuid,
    pub points: i64,
    pub created_at: DateTime<Utc>,
}
