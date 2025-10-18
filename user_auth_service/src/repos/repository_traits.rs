use async_trait::async_trait;
use std::error::Error;

// Core Traits for CRUD operations
#[async_trait]
pub trait Create<T> {
    async fn create(&self, entity: T) -> Result<T, Box<dyn Error + Send + Sync>>;
}

#[async_trait]
pub trait Read<T, ID> {
    async fn read(&self, id: ID) -> Result<Option<T>, Box<dyn Error + Send + Sync>>;
    async fn read_all(
        &self,
        name: Option<String>,
        limit: u32,
        offset: u32,
        order_by: &str,
    ) -> Result<Vec<T>, Box<dyn Error + Send + Sync>>;
    async fn count_total(&self) -> Result<u64, Box<dyn Error + Send + Sync>>;
}

#[async_trait]
pub trait Update<T, ID> {
    async fn update(&self, id: ID, entity: T) -> Result<T, Box<dyn Error + Send + Sync>>;
    async fn update_is_active(
        &self,
        id: ID,
        is_active: bool,
    ) -> Result<T, Box<dyn Error + Send + Sync>>;
}

#[async_trait]
pub trait Delete<T, ID> {
    async fn delete(&self, id: ID) -> Result<T, Box<dyn Error + Send + Sync>>;
}

// Composite Repository Trait
#[async_trait]
pub trait Repository<T, ID>: Read<T, ID> + Create<T> + Update<T, ID> + Delete<T, ID> {}
