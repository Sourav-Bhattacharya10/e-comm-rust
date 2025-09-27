use sqlx::{Pool, Postgres};

pub mod user;

#[derive(Clone)]
pub struct AppState {
    pub db_pool: Pool<Postgres>,
}
