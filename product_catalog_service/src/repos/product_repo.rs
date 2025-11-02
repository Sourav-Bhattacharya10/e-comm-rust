use async_trait::async_trait;
use sqlx::PgPool;

use crate::{
    dtos::{create_product_dto::CreateProductDto, update_product_dto::UpdateProductDto},
    models::{
        app_error::AppError, paginated_response::PaginatedResponse, pagination::Pagination,
        product::Product,
    },
    repos::repository_traits::Repository,
};

pub struct ProductRepo {
    pub pool: PgPool,
}

#[async_trait]
impl Repository<Product, CreateProductDto, UpdateProductDto> for ProductRepo {
    async fn create(&self, data: &CreateProductDto) -> Result<Product, AppError> {
        let product = sqlx::query_as!(
            Product,
            r#"
            INSERT INTO products (name, description, price, image_url)
            VALUES ($1, $2, $3, $4)
            RETURNING *
            "#,
            data.name,
            data.description,
            data.price,
            data.image_url,
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|_| AppError::InternalServerError)?;

        Ok(product)
    }

    async fn get_all(
        &self,
        pagination: &Pagination,
    ) -> Result<PaginatedResponse<Product>, AppError> {
        let page = pagination.page.unwrap_or(1);
        let per_page = pagination.per_page.unwrap_or(10);

        let offset = (page - 1) * per_page;

        let products = sqlx::query_as!(
            Product,
            r#"
            SELECT * FROM products
            ORDER BY created_at DESC
            LIMIT $1
            OFFSET $2
            "#,
            per_page,
            offset
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|_| AppError::InternalServerError)?;

        let total: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM products")
            .fetch_one(&self.pool)
            .await
            .map_err(|_| AppError::InternalServerError)?;

        let total_pages = (total as f64 / per_page as f64).ceil() as i64;

        Ok(PaginatedResponse {
            page,
            per_page,
            total,
            total_pages,
            data: products,
        })
    }

    async fn get_by_id(&self, id: uuid::Uuid) -> Result<Product, AppError> {
        let product = sqlx::query_as!(
            Product,
            r#"
            SELECT * FROM products
            WHERE id = $1
            "#,
            id
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|_| AppError::NotFound("Product not found".to_string()))?;

        Ok(product)
    }

    async fn update(&self, id: uuid::Uuid, data: &UpdateProductDto) -> Result<Product, AppError> {
        let product = sqlx::query_as!(
            Product,
            r#"
            UPDATE products
            SET
                name = COALESCE($1, name),
                description = COALESCE($2, description),
                price = COALESCE($3, price),
                image_url = COALESCE($4, image_url),
                updated_at = NOW()
            WHERE id = $5
            RETURNING *
            "#,
            data.name,
            data.description,
            data.price,
            data.image_url,
            id
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|_| AppError::NotFound("Product not found".to_string()))?;

        Ok(product)
    }

    async fn delete(&self, id: uuid::Uuid) -> Result<(), AppError> {
        sqlx::query!(
            r#"
            DELETE FROM products
            WHERE id = $1
            "#,
            id
        )
        .execute(&self.pool)
        .await
        .map_err(|_| AppError::NotFound("Product not found".to_string()))?;

        Ok(())
    }
}
