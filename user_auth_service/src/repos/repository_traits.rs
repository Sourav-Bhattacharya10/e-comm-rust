use async_trait::async_trait;
use std::error::Error;

// Core Traits for CRUD operations
#[async_trait]
pub trait Create<T> {
    async fn create(&self, entity: T) -> Result<T, Box<dyn Error>>;
}

#[async_trait]
pub trait Read<T, ID> {
    async fn read(&self, id: ID) -> Result<Option<T>, Box<dyn Error>>;
    async fn read_all(&self) -> Result<Vec<T>, Box<dyn Error>>;
}

#[async_trait]
pub trait Update<T, ID> {
    async fn update(&self, id: ID, entity: T) -> Result<T, Box<dyn Error>>;
}

#[async_trait]
pub trait Delete<ID> {
    async fn delete(&self, id: ID) -> Result<(), Box<dyn Error>>;
}

// Composite Repository Trait
#[async_trait]
pub trait Repository<T, ID>: Read<T, ID> {} // Create<T> +  + Update<T, ID> + Delete<ID>
