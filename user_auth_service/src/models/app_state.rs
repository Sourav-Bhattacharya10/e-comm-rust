use sqlx::{Pool, Postgres};
use std::sync::Arc;
use uuid::Uuid;

use crate::models::user::User;
use crate::repos::repository_traits::Repository;

#[derive(Clone)]
pub struct AppState {
    pub db_pool: Pool<Postgres>,
    pub user_repo: Arc<dyn Repository<User, Uuid> + Send + Sync>,
}
