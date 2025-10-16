use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateUserDto {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub role: String,
    pub is_active: bool,
    pub updated_at: Option<DateTime<Utc>>,
}
