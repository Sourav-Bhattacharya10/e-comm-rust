use bigdecimal::BigDecimal;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CreateProductDto {
    pub name: String,
    pub description: Option<String>,
    pub price: BigDecimal,
    pub image_url: Option<String>,
}
