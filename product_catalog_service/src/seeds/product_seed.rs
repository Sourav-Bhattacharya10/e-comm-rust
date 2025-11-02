use bigdecimal::{BigDecimal, FromPrimitive};
use sqlx::PgPool;

use crate::dtos::create_product_dto::CreateProductDto;
use crate::repos::product_repo::ProductRepo;
use crate::repos::repository_traits::Repository;

pub async fn seeding_products_data(pool: &PgPool) -> Result<(), Box<dyn std::error::Error>> {
    let product_repo = ProductRepo { pool: pool.clone() };

    let products_to_seed = vec![
        CreateProductDto {
            name: "Laptop".to_string(),
            description: Some("A powerful laptop".to_string()),
            price: BigDecimal::from_f64(1200.00).unwrap(),
            image_url: Some("https://example.com/laptop.jpg".to_string()),
        },
        CreateProductDto {
            name: "Mouse".to_string(),
            description: Some("A wireless mouse".to_string()),
            price: BigDecimal::from_f64(25.00).unwrap(),
            image_url: Some("https://example.com/mouse.jpg".to_string()),
        },
        CreateProductDto {
            name: "Keyboard".to_string(),
            description: Some("A mechanical keyboard".to_string()),
            price: BigDecimal::from_f64(75.00).unwrap(),
            image_url: Some("https://example.com/keyboard.jpg".to_string()),
        },
    ];

    for product_dto in products_to_seed {
        let _ = product_repo.create(&product_dto).await;
    }

    Ok(())
}
