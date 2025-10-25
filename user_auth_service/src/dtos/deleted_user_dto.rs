use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct DeletedUserDto {
    pub id: Uuid,
    pub username: String,
    pub email: String,
}
