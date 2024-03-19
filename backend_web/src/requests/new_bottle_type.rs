use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct NewBottleType {
    pub manufacturer_id: i64,
    pub drink_name: String,
    pub points: i64,
}
