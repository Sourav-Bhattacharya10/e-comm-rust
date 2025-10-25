use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct UpdateUserDto {
    pub id: Uuid,
    #[validate(length(min = 1))]
    pub username: String,
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 1))]
    pub role: String,
    pub is_active: bool,
    pub updated_at: Option<DateTime<Utc>>,
}
