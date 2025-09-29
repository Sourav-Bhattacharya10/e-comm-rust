use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{dtos::user_dto::UserDto, traits::into_dto::IntoDto};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub role: String,
    pub is_active: bool,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

impl IntoDto<UserDto> for User {
    fn into_dto(&self) -> UserDto {
        UserDto {
            id: self.id,
            username: self.username.clone(),
            email: self.email.clone(),
            role: self.role.clone(),
            is_active: self.is_active,
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }
}
