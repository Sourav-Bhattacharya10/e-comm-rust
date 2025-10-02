use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateUserDto {
    pub username: String,
    pub email: String,
    pub role: String,
    pub password_hash: String,
}
