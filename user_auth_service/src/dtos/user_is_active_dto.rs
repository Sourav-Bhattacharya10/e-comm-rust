use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct UserIsActiveDto {
    pub is_active: bool,
}
