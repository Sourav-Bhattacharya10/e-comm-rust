use bigdecimal::BigDecimal;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

use crate::dtos::product_dto::ProductDto;
use crate::traits::to_dto::ToDto;

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct Product {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub price: BigDecimal,
    pub image_url: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl ToDto<ProductDto> for Product {
    fn to_dto(&self) -> ProductDto {
        ProductDto {
            id: self.id,
            name: self.name.clone(),
            description: self.description.clone(),
            price: self.price.clone(),
            image_url: self.image_url.clone(),
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }
}
