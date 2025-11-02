use crate::models::{
    app_error::AppError, paginated_response::PaginatedResponse, pagination::Pagination,
};
use async_trait::async_trait;

#[async_trait]
pub trait Repository<T, C, U>: Send + Sync {
    async fn create(&self, data: &C) -> Result<T, AppError>;
    async fn get_all(&self, pagination: &Pagination) -> Result<PaginatedResponse<T>, AppError>;
    async fn get_by_id(&self, id: uuid::Uuid) -> Result<T, AppError>;
    async fn update(&self, id: uuid::Uuid, data: &U) -> Result<T, AppError>;
    async fn delete(&self, id: uuid::Uuid) -> Result<(), AppError>;
}
