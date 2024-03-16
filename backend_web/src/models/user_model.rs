use uuid::Uuid;

#[derive(Debug)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub created_at: String,
}
