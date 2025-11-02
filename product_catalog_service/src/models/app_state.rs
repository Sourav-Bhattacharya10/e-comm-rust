use std::sync::Arc;

use sqlx::PgPool;

use crate::repos::product_repo::ProductRepo;

pub struct AppState {
    pub db_pool: PgPool,
    pub product_repo: Arc<ProductRepo>,
}
