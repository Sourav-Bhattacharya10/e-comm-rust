use bigdecimal::BigDecimal;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct UpdateProductDto {
    pub name: Option<String>,
    pub description: Option<String>,
    pub price: Option<BigDecimal>,
    pub image_url: Option<String>,
}
